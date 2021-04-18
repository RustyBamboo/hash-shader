use alkomp;

use std::env;

fn prepare_for_gpu(word: String) -> (Vec<u32>, u32) {
    let mut init: Vec<u8> = word.into_bytes();

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

fn main() {
    let words: Vec<String> = vec![
        "abc".to_string(),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad12093123".to_string(),
        "abc".to_string(),
    ];
    let count = words.len();

    let (texts, sizes): (Vec<Vec<u32>>, Vec<u32>) =
        words.into_iter().map(|x| prepare_for_gpu(x)).unzip();

    let texts: Vec<u32> = texts.into_iter().flatten().collect();
    println!("{:?}", texts);

    println!("{:?}", sizes);

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
    let hash_res = &hash_res[0..hash.len()];

    let result: String = hash_res.into_iter().map(|x| format!("{:08x}", x)).collect();
    let chunks = result
        .as_bytes()
        .chunks(64)
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap();
    assert_eq!(
        chunks[0],
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
    assert_eq!(
        chunks[1],
        "1c4b99f23a70c71a182da472f4e06406e5e33d0c7cefe34d431f2a3a5900aa90"
    );
    assert_eq!(
        chunks[2],
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}
