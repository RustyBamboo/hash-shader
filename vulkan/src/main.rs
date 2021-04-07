use alkomp;

use std::env;

fn prepare_for_gpu(word: String) -> Vec<u32> {
    let mut init: Vec<u8> = word.into_bytes();

    let msg_size = (init.len() * 8) as u64; // in bits

    // Add a 1 as a delimiter
    init.push(0x80 as u8);
    let size: usize = (448u32 / 8u32 - init.len() as u32) as usize;

    // Pad with zeros
    let remaining = vec![0u8; size];
    init.extend(&remaining);

    // Make the last 64 bits be the size
    let size = (msg_size).to_be_bytes();
    init.extend(&size);

    let mut text = Vec::new();

    use std::convert::TryInto;
    for i in 0..16 {
        let val = u32::from_be_bytes(init[i * 4..(i + 1) * 4].try_into().unwrap());
        text.push(val);
    }

    text
}

fn main() {
    let words: Vec<String> = env::args().skip(1).collect();
    let count = words.len();
    if count == 0 {
        println!("Input a list of strings to hash");
        return;
    }

    let texts: Vec<u32> = words
        .into_iter()
        .map(|x| prepare_for_gpu(x))
        .collect::<Vec<Vec<u32>>>()
        .into_iter()
        .flatten()
        .collect();

    // Check number of bits
    assert_eq!(texts.len() * core::mem::size_of::<u32>() * 8, 512 * count);

    let hash = vec![0u32; count * 8];

    // Check number of bits
    assert_eq!(hash.len() * core::mem::size_of::<u32>() * 8, 8 * 32 * count);

    let mut device = alkomp::Device::new(0);
    let text_gpu = device.to_device(texts.as_slice());
    let hash_gpu = device.to_device(hash.as_slice());

    let shader = wgpu::include_spirv!(env!("kernel.spv"));

    let args = alkomp::ParamsBuilder::new()
        .param(Some(&text_gpu))
        .param(Some(&hash_gpu))
        .build(Some(0));

    let compute = device.compile("main_cs", &shader, &args.0).unwrap();

    device.call(compute, (count as u32, 1, 1), &args.1);

    let hash_res = futures::executor::block_on(device.get(&hash_gpu)).unwrap();
    let hash_res = &hash_res[0..hash.len()];

    let result: String = hash_res.into_iter().map(|x| format!("{:08x}", x)).collect();
    let chunks = result
        .as_bytes()
        .chunks(64)
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap();
    for c in chunks {
        println!("{}", c);
    }
}
