mod helpers;
mod runner;

use std::{
    env, fs,
    io::{BufRead, BufReader},
};

use clap::Parser;

use crate::helpers::prepare_for_gpu_u8;

const KERNEL: &[u8] = include_bytes!(env!("kernel.spv"));

/// Print or check SHA256 (256-bit) checksums.
/// Computing SHA256 hash is performed on a GPU backend.
#[derive(Parser, Debug)]
#[command(author, version, about, arg_required_else_help(true), long_about = None, after_help = "Example: \n sha256_rgpu MYFILE > hash.sha256sum \n sha256_rgpu -c hash.sha256sum")]
struct Args {
    /// read checksums from file and check them
    #[arg(short, long)]
    check: bool,

    /// files to hash, or checksum files (if check)
    files: Vec<String>,

    /// id of compute device
    #[arg(short, long, default_value_t = 0)]
    device: usize,

    /// verbose mode
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

struct HashAndFile {
    hash: String,
    file: String,
}

///
/// Read lines from a checksum file and extract hash value and file path
///
fn lines_from_file(filename: &str) -> Vec<HashAndFile> {
    let file = fs::File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    let lines = buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .map(|l| {
            let parts: Vec<&str> = l.split("  ").collect();
            if parts.len() != 2 {
                panic!("Line does not contain exactly two parts");
            }
            HashAndFile {
                hash: parts[0].to_string(),
                file: parts[1].to_string(),
            }
        })
        .collect();

    lines
}

fn main() {
    let args = Args::parse();

    // Read all files and obtain a list of hash values and corresponding file paths
    let (files, hashes) = if args.check {
        let hf: Vec<HashAndFile> = args
            .files
            .iter()
            .map(|f| lines_from_file(f))
            .flatten()
            .collect();
        let (files, hashes): (Vec<_>, Vec<_>) = hf.into_iter().map(|x| (x.file, x.hash)).unzip();
        (files, Some(hashes))
    } else {
        (args.files, None)
    };
    let contents: Vec<Vec<u8>> = files
        .iter()
        .map(|f| fs::read(&f).expect("Should have been able to read the file"))
        .collect();

    let count = files.len();

    // A Vec of bit strings, and a vec of "number of iterations"
    let (texts, sizes): (Vec<Vec<u32>>, Vec<u32>) =
        contents.into_iter().map(|x| prepare_for_gpu_u8(x)).unzip();

    for &s in &sizes {
        if s > 10000 {
            todo!("Sorry, the file is too large and will crash your GPU. This will change in the future by sending data in chunks to the GPU.")
        }
    }

    // Flatten the data to send to GPUs
    let texts: Vec<u32> = texts.into_iter().flatten().collect();

    // Prepare buffer to store the results
    let hash = vec![0u32; count * 8];

    // Check number of bits
    assert_eq!(hash.len() * core::mem::size_of::<u32>() * 8, 8 * 32 * count);

    let mut device = runner::Device::new(args.device);
    let text_gpu = device.to_device(texts.as_slice());
    let hash_gpu = device.to_device(hash.as_slice());
    let size_gpu = device.to_device(sizes.as_slice());

    let shader = wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::util::make_spirv(KERNEL),
    };

    let args_gpu = runner::ParamsBuilder::new()
        .param(Some(&text_gpu), true)
        .param(Some(&hash_gpu), false)
        .param(Some(&size_gpu), true)
        .build(Some(0));

    let compute = device.compile("main_cs", shader, &args_gpu.0).unwrap();

    device.call(compute, (count as u32, 1, 1), &args_gpu.1);

    let hash_res = futures::executor::block_on(device.get(&hash_gpu)).unwrap();
    let hash_res = &hash_res[0..hash.len()];

    let result: String = hash_res.into_iter().map(|x| format!("{:08x}", x)).collect();
    let chunks = result
        .as_bytes()
        .chunks(64)
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap();

    if let Some(hashes) = hashes {
        let mut count_bad = 0;
        for i in 0..files.len() {
            let f = &files[i];
            let c = chunks[i];
            let h: &str = hashes[i].as_ref();

            let status = c == h;

            if !status {
                count_bad += 1;
            }

            println!("{}: {}", f, if status { "OK" } else { "FAILURE" });
        }
        if count_bad > 0 {
            println!(
                "sha256_rgpu: WARNING: {} computed checksum did NOT match",
                count_bad
            );
        }
    } else {
        for (f, c) in files.iter().zip(chunks.iter()) {
            println!("{}  {}", c, f);
        }
    }

    if args.verbose {
        let d_info = &device.info.as_ref().unwrap().info;
        let d_name = &d_info.name;
        let d_type = &d_info.device_type;
        let d_driver = &d_info.driver;
        let d_driver_info = &d_info.driver_info;
        let d_backend = &d_info.backend;

        println!(
            "{}\n  Type: {:?}\n  Driver: {} ({})\n  Backend: {:?}",
            d_name, d_type, d_driver, d_driver_info, d_backend
        );
    }
}
