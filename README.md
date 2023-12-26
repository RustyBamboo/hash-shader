# Vulkan Hash

## To run on GPU:

This will build everything (including the SPIRV kernel) and then search for available GPUs and select the first one possible.

### Hash text
```
cargo run --release --bin vulkan "I like Cheese"
```
Returns:
```
f7e1df1d3eedc8bcbe928e03fedca73afbfe807e2ec147bbb277a8668b47443b
```

### Run blockchain validator
```
cargo run --release --bin blockchain-val ../bitcoin/block_data.csv
```
(this requires path to a csv file with rows as: `block_header_hex,expected_hash`)

Example return:
```
Validated 4998 blocks
57.035164ms + 42.468205ms = 99.503369ms
```


## Tests
This will run GPU tests:
```
cargo test --release

```             

This will run CPU tests of the kernel:
```
cd kernel
cargo test --release
```

