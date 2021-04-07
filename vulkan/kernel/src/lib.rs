#![allow(non_snake_case)]
#![cfg_attr(
    target_arch = "spirv",
    no_std,
    feature(register_attr),
    register_attr(spirv)
)]

#[cfg(not(target_arch = "spirv"))]
#[macro_use]
pub extern crate spirv_std_macros;

use glam::UVec3;

// Rotation right: u32.rotate_right(n: u32)
fn rot_r(x: u32, n: u32) -> u32 {
    x >> n | (x << (32 - n))
}

fn Sigma0(x: u32) -> u32 {
    rot_r(x, 2) ^ rot_r(x, 13) ^ rot_r(x, 22)
    //x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
}
#[test]
fn test_Sigma0() {
    let x: u32 = 0b00000000000000000011111111111111;
    let y: u32 = 0b00111111000001111111001111111110;
    let r = Sigma0(x);
    assert_eq!(
        r, y,
        "Testing choice:\n x:{:#034b}\n y:{:#034b}\n e:{:#034b}",
        x, y, r
    );
}

fn Sigma1(x: u32) -> u32 {
    rot_r(x, 6) ^ rot_r(x, 11) ^ rot_r(x, 25)
    //x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
}
#[test]
fn test_Sigma1() {
    let x: u32 = 0b00000000000000000011111111111111;
    let y: u32 = 0b00000011111111111111111101111000;
    let r = Sigma1(x);
    assert_eq!(
        r, y,
        "Testing choice:\n x:{:#034b}\n y:{:#034b}\n e:{:#034b}",
        x, y, r
    );
}
fn sigma0(x: u32) -> u32 {
    rot_r(x, 7) ^ rot_r(x, 18) ^ (x >> 3)
    //x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3)
}
#[test]
fn test_sigma0() {
    let x: u32 = 0b00000000000000000011111111111111;
    let y: u32 = 0b11110001111111111100011110000000;
    let r = sigma0(x);
    assert_eq!(
        r, y,
        "Testing choice:\n x:{:#034b}\n y:{:#034b}\n e:{:#034b}",
        x, y, r
    );
}

fn sigma1(x: u32) -> u32 {
    rot_r(x, 17) ^ rot_r(x, 19) ^ (x >> 10)
    //x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10)
}
#[test]
fn test_sigma1() {
    let x: u32 = 0b00000000000000000011111111111111;
    let y: u32 = 0b00011000000000000110000000001111;
    let r = sigma1(x);
    assert_eq!(
        r, y,
        "Testing choice:\n x:{:#034b}\n y:{:#034b}\n e:{:#034b}",
        x, y, r
    );

    let x: u32 = 0b00000000000000000000000000011000;
    let y: u32 = 0b00000000000011110000000000000000;
    let r = sigma1(x);
    assert_eq!(
        r, y,
        "Testing choice:\n x:{:#034b}\n y:{:#034b}\n e:{:#034b}",
        x, y, r
    );
}

// Choice operation
fn ch(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (!x & z)
}

#[test]
fn test_ch() {
    let x: u32 = 0b0000000111111110000000011111111;
    let y: u32 = 0b0000000000000001111111111111111;
    let z: u32 = 0b1111111111111110000000000000000;
    let w: u32 = 0b1111111000000000000000011111111;
    assert_eq!(
        ch(x, y, z),
        w,
        "Testing choice:\n x:{:#034b}\n y:{:#034b}\n z:{:#034b}\n w:{:#034b}",
        x,
        y,
        z,
        w
    );
}

// Majority operation
fn maj(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (x & z) ^ (y & z)
}

#[test]
fn test_maj() {
    let x: u32 = 0b0000000111111110000000011111111;
    let y: u32 = 0b0000000000000001111111111111111;
    let z: u32 = 0b1111111111111110000000000000000;
    let w: u32 = 0b0000000111111110000000011111111;
    assert_eq!(
        maj(x, y, z),
        w,
        "Testing choice:\n x:{:#034b}\n y:{:#034b}\n z:{:#034b}\n w:{:#034b}",
        x,
        y,
        z,
        w
    );
}

const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

#[test]
fn test_K() {
    let primes = vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
        191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281,
        283, 293, 307, 311,
    ];
    for (ix, n) in primes.into_iter().enumerate() {
        // Get the fractional part as hex
        let mut fractional = (n as f64).cbrt().fract();
        let mut hex = [0u8; 8];
        for h in 0..hex.len() {
            let product = fractional * 16.;
            let carry = product.floor() as u8;
            fractional = product - product.floor();
            hex[h] = carry;
        }
        // Convert the hex array (4 bits but represented as u8) to a u32
        let mut value: u32 = hex[7] as u32;
        for (i, h) in (0..hex.len() - 1).rev().enumerate() {
            value += hex[h] as u32 * 16_u32.pow(i as u32 + 1);
        }
        assert_eq!(K[ix], value);
    }
}

const INIT_HASH: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

fn hash_fn(text: &[u32], hash: &mut [u32], x: usize) {
    // Offsets for loading and storing in data buffers
    let load_offset = x * 16;
    let store_offset = x * 8;

    let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h, mut t1, mut t2): (
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
    );

    // Need to manually unroll declaration
    let mut m: [u32; 64] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];

    // Create the message schedule
    // The first 16 are assumed to be given
    for i in 0..16 {
        m[i] = text[load_offset + i];
    }

    // Compute the remaining message schedule
    for i in 16..64 {
        m[i] = sigma1(m[i - 2]) + m[i - 7] + sigma0(m[i - 15]) + m[i - 16];
        //println!("{} {:#034b}", i, m[i]);
    }

    // Do compression
    // The initial hash value as sqrt of primes
    a = 0x6a09e667;
    b = 0xbb67ae85;
    c = 0x3c6ef372;
    d = 0xa54ff53a;
    e = 0x510e527f;
    f = 0x9b05688c;
    g = 0x1f83d9ab;
    h = 0x5be0cd19;
    for i in 0..64 {
        t1 = Sigma1(e) + ch(e, f, g) + h + K[i] + m[i];
        t2 = Sigma0(a) + maj(a, b, c);
        h = g;
        g = f;
        f = e;
        e = d + t1;
        d = c;
        c = b;
        b = a;
        a = t1 + t2;
    }

    // Add the original hashed message with initial hash
    a += INIT_HASH[0];
    b += INIT_HASH[1];
    c += INIT_HASH[2];
    d += INIT_HASH[3];
    e += INIT_HASH[4];
    f += INIT_HASH[5];
    g += INIT_HASH[6];
    h += INIT_HASH[7];

    // Store result
    hash[store_offset + 0] = a;
    hash[store_offset + 1] = b;
    hash[store_offset + 2] = c;
    hash[store_offset + 3] = d;
    hash[store_offset + 4] = e;
    hash[store_offset + 5] = f;
    hash[store_offset + 6] = g;
    hash[store_offset + 7] = h;
}

#[test]
fn test_hash_fn() {
    let word: String = String::from("abc");
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

    let mut hash = vec![0u32; 8];

    hash_fn(text.as_slice(), hash.as_mut_slice(), 0);

    let result: String = hash.into_iter().map(|x| format!("{:x}", x)).collect();
    assert_eq!(
        result,
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}

#[allow(unused_attributes)]
#[spirv(compute(threads(1)))]
pub fn main_cs(
    #[spirv(global_invocation_id)] gid: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] text: &[u32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] hash: &mut [u32],
) {
    hash_fn(text, hash, gid.x as usize);
}
