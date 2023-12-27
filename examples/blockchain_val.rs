#[path = "../src/runner.rs"]
mod runner;

#[path = "../src/helpers.rs"]
mod helpers;

use std::env;
use std::time::{Duration, Instant};

use helpers::prepare_for_gpu_u8;

fn sha<'a>(words: Vec<String>) -> (Box<[u32]>, Duration) {
    let count = words.len();

    // A Vec of bit strings, and a vec of "number of iterations"
    let (texts, sizes): (Vec<Vec<u32>>, Vec<u32>) =
        words.into_iter().map(|x| prepare_for_gpu_u8(hex::decode(x).expect("Input was not hex"))).unzip();

    let texts: Vec<u32> = texts.into_iter().flatten().collect();

    let hash = vec![0u32; count * 8];

    // Check number of bits
    assert_eq!(hash.len() * core::mem::size_of::<u32>() * 8, 8 * 32 * count);

    let mut device = runner::Device::new(0);

    let text_gpu = device.to_device(texts.as_slice());
    let hash_gpu = device.to_device(hash.as_slice());
    let size_gpu = device.to_device(sizes.as_slice());

    let shader = wgpu::include_spirv!(env!("kernel.spv"));

    let args = runner::ParamsBuilder::new()
        .param(Some(&text_gpu), true)
        .param(Some(&hash_gpu), false)
        .param(Some(&size_gpu), true)
        .build(Some(0));

    let start_1 = Instant::now();
    let compute = device.compile("main_cs", shader, &args.0).unwrap();

    device.call(compute, (count as u32, 1, 1), &args.1);

    let hash_res = futures::executor::block_on(device.get(&hash_gpu)).unwrap();
    let duration_1 = start_1.elapsed();
    (hash_res, duration_1)
}

fn main() {
    let paths: Vec<String> = env::args().skip(1).collect();
    if paths.len() == 0 {
        println!("Input path to CSV file containing blockchain data: [block header],[block hash]
        \nThe following will create a CSV file with this information from the real bitcoin blockchain:
        \n\tpython3 examples/prepare_blockchain.py\n");
        return;
    }
    let max_blocks: i32;
    if paths.len() > 1 {
        max_blocks = paths[1].parse().unwrap()
    } else {
        max_blocks = -1;
    }

    let mut rdr = csv::Reader::from_path(&paths[0]).expect("Failed to open file");

    let mut words = Vec::new();
    let mut expected = Vec::new();

    let mut i = 0;
    for record in rdr.records() {
        if max_blocks != -1 && i == max_blocks {
            break;
        }
        let record = record.expect("Failed");
        words.push(record[0].to_string());
        expected.push(record[1].to_string());
        i += 1;
    }

    // ROUND 1 OF SHA

    let (hash_res, duration_1) = sha(words);
    let hash_res = &hash_res;

    let result: String = hash_res.into_iter().map(|x| format!("{:08x}", x)).collect();
    let chunks = result
        .as_bytes()
        .chunks(64)
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap();

    let words = chunks.into_iter().map(|s| s.to_string()).collect();

    // ROUND 2 OF SHA

    let (hash_res, duration_2) = sha(words);
    let hash_res = &hash_res;

    let result: String = hash_res.into_iter().map(|x| format!("{:08x}", x)).collect();
    let chunks = result
        .as_bytes()
        .chunks(64)
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap();

    for c in 0..chunks.len() {
        assert_eq!(expected[c], chunks[c]);
    }
    print!("{} {}", chunks.len(), (duration_1 + duration_2).as_millis());
}
