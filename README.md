# GPU Based Bitcoin Blockchain Validation

![](docs/logo.png)

## Abstract
The project consists of two primary goals: 1) implement kernels that validate several blocks' hash in the Bitcoin Blockchain in parallel, and 2) investigate using the open-source Vulkan standard as an alternative to CUDA. When compared to an equivalent sequential program run on the CPU, the open source tools were approximately 5 times slower than the sequential algorithm, and the proprietary CUDA was  approximately 2 times as fast as the sequential algorithm.

## Motivation
Block chains are an emerging technology with possible applications in areas of distributed computing. However, it is not hard to imagine a block chain which grows sufficiently quickly such that a machine which is only connected periodically, could not self validate all the blocks in a reasonable time frame with a sequential algorithm. However, the validation of blocks on a proposed block chain is a parallelizable computation.

While the Bitcoin Blockchain only adds a block approximately every 10 minutes, it provides a good testing ground for a proof of concept because it is well known, easy to get the block data, and uses a SHA256 hash as its validation algorithm, which is likely a similar operation to block chains of the future.

We also decided to use this as a testing ground to compare CUDA and Vulkan, as validating block chains should ideally be able to work on as many GPU's as possible.

Read the [Full Report](docs/bitcoin_gpu.pdf).

# Compile the code

## Build the C / CPU, CUDA code
`make -C c`

## Build the Rust / Vulkan code
`cd vulkan && cargo build --release && cd ..`

## Generate a plot yourself
`./plotboi.py ./bitcoin/block_data.csv`

`display ./docs/figs/performance_plot.png`



# Licencing Details

This project contains several files directly copied from the OpenSSL project ([https://github.com/openssl/openssl](https://github.com/openssl/openssl)), which is under the Apache 2.0 Licence.

The files in question are:

* `c/include/md32_common.h`

* `c/include/sha256.h`

* `c/src/sha256.c`
