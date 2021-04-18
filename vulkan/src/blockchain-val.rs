use alkomp;

use std::env;
use std::time::Instant;

/// Prepare the text data for GPU by padding the bits to multiples of 512
/// - Append 1 as a delimiter
/// - Append 0s
/// - The last 64 bits denote the size of original message
fn prepare_for_gpu(word: String) -> (Vec<u32>, u32) {
    let mut init: Vec<u8> = hex::decode(word).expect("Input was not hex");

    let msg_size = (init.len() * 8) as u64; // in bits

    let desired_size = (msg_size / 512 + 1) * 512;

    // Add a 1 as a delimiter
    init.push(0x80 as u8);
    let size: usize = ((desired_size - 64) as u32 / 8u32 - init.len() as u32) as usize;

    // Pad with zeros
    let remaining = vec![0u8; size];
    init.extend(&remaining);

    // Make the last 64 bits be the size
    let size = (msg_size).to_be_bytes();
    init.extend(&size);

    let mut text = Vec::new();

    use std::convert::TryInto;
    for i in 0..(desired_size / 32) as usize {
        let val = u32::from_be_bytes(init[i * 4..(i + 1) * 4].try_into().unwrap());
        text.push(val);
    }

    (text, (desired_size / 512) as u32)
}

fn sha<'a>(words: Vec<String>) -> Box<[u32]> {
    let count = words.len();

    // A Vec of bit strings, and a vec of "number of iterations"
    let (texts, sizes): (Vec<Vec<u32>>, Vec<u32>) =
        words.into_iter().map(|x| prepare_for_gpu(x)).unzip();

    let texts: Vec<u32> = texts.into_iter().flatten().collect();

    let hash = vec![0u32; count * 8];

    // Check number of bits
    assert_eq!(hash.len() * core::mem::size_of::<u32>() * 8, 8 * 32 * count);

    let mut device = alkomp::Device::new(0);

    let text_gpu = device.to_device(texts.as_slice());
    let hash_gpu = device.to_device(hash.as_slice());
    let size_gpu = device.to_device(sizes.as_slice());

    let shader = wgpu::include_spirv!(env!("kernel.spv"));

    let args = alkomp::ParamsBuilder::new()
        .param(Some(&text_gpu))
        .param(Some(&hash_gpu))
        .param(Some(&size_gpu))
        .build(Some(0));

    let compute = device.compile("main_cs", &shader, &args.0).unwrap();

    device.call(compute, (count as u32, 1, 1), &args.1);

    let hash_res = futures::executor::block_on(device.get(&hash_gpu)).unwrap();
    hash_res
}

fn main() {
    let paths: Vec<String> = env::args().skip(1).collect();
    if paths.len() == 0 {
        println!("Input path to CSV file containing blockchain data");
        return;
    }

    let mut rdr = csv::Reader::from_path(&paths[0]).expect("Failed to open file");

    let mut words = Vec::new();
    let mut expected = Vec::new();

    for record in rdr.records() {
        let record = record.expect("Failed");
        words.push(record[0].to_string());
        expected.push(record[1].to_string());
    }

    // ROUND 1 OF SHA
    let start_1 = Instant::now();

    let hash_res = sha(words);
    let hash_res = &hash_res;

    let duration_1 = start_1.elapsed();

    let result: String = hash_res.into_iter().map(|x| format!("{:08x}", x)).collect();
    let chunks = result
        .as_bytes()
        .chunks(64)
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap();

    let words = chunks.into_iter().map(|s| s.to_string()).collect();

    // ROUND 2 OF SHA

    let start_2 = Instant::now();

    let hash_res = sha(words);
    let hash_res = &hash_res;

    let duration_2 = start_2.elapsed();

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
    println!("Validated {} blocks", chunks.len());
    println!(
        "{:?} + {:?} = {:?}",
        duration_1,
        duration_2,
        duration_1 + duration_2
    );
}
