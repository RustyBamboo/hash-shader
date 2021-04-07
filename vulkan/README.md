# Vulkan Hash

## To run on GPU:
```
cargo run --release
```
This will build everything (including the SPIRV kernel) and then search for available GPUs and select the first one possible.


## Tests
This will run CPU tests of the kernel:
```
cd kernel
cargo test --release
```