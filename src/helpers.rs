#[allow(dead_code)]
pub fn prepare_for_gpu(word: String) -> (Vec<u32>, u32) {
    let init = word.into_bytes();
    return prepare_for_gpu_u8(init);
}

/// Prepare the text data for GPU by padding the bits to multiples of 512
/// - Append 1 as a delimiter
/// - Append 0s
/// - The last 64 bits denote the size of original message
pub fn prepare_for_gpu_u8(mut init: Vec<u8>) -> (Vec<u32>, u32) {

    let msg_size = (init.len() * 8) as u64; // in bits

    let mut desired_size = (msg_size / 512 + 1) * 512;

    // Ensure that desired_size is large enough to accommodate the initial message, 
    // the 0x80 byte, and the 64-bit length field
    while desired_size < msg_size + 8 + 64 {
        desired_size += 512;
    }

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