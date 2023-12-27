#[path = "../src/runner.rs"]
mod runner;

use rand::Rng;
use std::env;

use std::time::{Duration, Instant};


fn rand_hash(count: usize) -> Duration {
    // Generate random strings of size 512 bits
    let texts: Vec<Vec<u32>> = (0..count)
        .map(|_| (0..16).map(|_| rand::thread_rng().gen()).collect())
        .collect();
    let texts: Vec<u32> = texts.into_iter().flatten().collect();
    let sizes: Vec<u32> = (0..count).map(|_| 1).collect();
    let hash = vec![0u32; count * 8];

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

    let hash_res = &hash_res[0..hash.len()];

    let result: String = hash_res.into_iter().map(|x| format!("{:08x}", x)).collect();
    let _chunks = result
        .as_bytes()
        .chunks(64)
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap();
    return duration_1;
}

fn run_benchmark(count:usize, f:&dyn Fn(usize)->Duration) -> f64{
    let times: Vec<Duration> = (0..30).map(|_| f(count)).collect();
    let times: Vec<u128> = times.iter().map(|x| x.as_millis()).collect();
    return times.iter().sum::<u128>() as f64 / times.len() as f64;
}

fn main() {
    let count = env::args().nth(1).expect("Enter number of hashes");
    let count = count.parse::<usize>().expect("Not a number");
    println!("{} ms", run_benchmark(count, &rand_hash));
}


