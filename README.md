# SHA256 Compute Shader (Kernel) Written in Rust
... with application to Validating the Bitcoin Blockchain

<p align="center">
  <img src="docs/logo.png"/>
</p>


--------------------------------------------------------------------

## Abstract
The project consists of two primary goals: 1) implement kernels that validate several blocks' SHA256 hash in the Bitcoin Blockchain in parallel, and 2) investigate using the open-source Vulkan standard as an alternative to CUDA. When compared to an equivalent sequential program run on the CPU, the open source tools were approximately 5 times slower than the sequential algorithm, and the proprietary CUDA was  approximately 2 times as fast as the sequential algorithm.

## SHA256 on GPU

SHA256 algorithm itself is primarily a sequential algorithm. However, for a variety of applications being able to hash many items in a parallel fashion can be quite beneficial. As an example, we specifically looked at validating the bitcoin blockchain as a benchmark. We compared the Vulkan/Rust API with a CUDA/C implementation.

Read the [Full Report](docs/bitcoin_gpu.pdf).

### WebGPU, Vulkan, and Rust-GPU

We implemented the SHA256 algorithm in Rust and compiled it to SPIR-V using using [rust-gpu](https://github.com/EmbarkStudios/rust-gpu). The shader was then loaded to a Vulkan backend using [wgpu-rs](https://github.com/gfx-rs/wgpu-rs) with its Rust bindings to `WebGPU`. 

To see more details, visit the [vulkan](vulkan/) directory.

# Compile the code

## Build the C / CPU, CUDA code
`make -C c`

## Build the Rust / Vulkan code
`cd vulkan && cargo build --release && cd ..`

## Generate a plot yourself
`./plotboi.py ./bitcoin/block_data.csv`

`display ./docs/figs/performance_plot.png`



# Licensing Details

This project contains several files directly copied from the OpenSSL project ([https://github.com/openssl/openssl](https://github.com/openssl/openssl)), which is under the Apache 2.0 Licence.

The files in question are:

* `c/include/md32_common.h`

* `c/include/sha256.h`

* `c/src/sha256.c`
