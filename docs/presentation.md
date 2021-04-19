---
title:
  - SHA256 Validation with CUDA and Vulkan
author:
  - Daniel Volya
  - Marshall Rawson
header-includes:
 - \usepackage{fvextra}
 - \DefineVerbatimEnvironment{Highlighting}{Verbatim}{breaklines,commandchars=\\\{\}}
---

# Summary

1. Introduction
    - Bitcoin Blockchain Validation
    - SHA256 
    - Vulkan
2. Results

# Bitcoin Blockchain

![](figs/blockchain.png)

```
header = version + prevBlockHash + merkleRootHash + time + bits + nonce
bytes = 4 + 32 + 32 + 4 + 4 + 4 = 80
```

hash = SHA256(SHA256(header))

# Example

```
header = 01000000 81cd02ab7e569e8bcd9317e2fe99f2de44d49ab2b8851ba4a308000000000000 e320b6c2fffc8d750423db8b1eb942ae710e951ed797f7affc8892b0f1fc122b c7f5d74d f2b9441a 42a14695
```

SHA256(SHA256(header)) = 

```
1dbd981fe6985776b644b173a4d0385ddc1aa2a829688d1e0000000000000000
```

# SHA256

- Pad a message *m* to closest multiple of *512*
- Perform operations $\Sigma_0$, $\Sigma_1$, $\sigma_0$, $\sigma_1$ to create entries of message block
- Compress the message block into 8, 32bit words. Use *initial_hash* as initialization
- Add *initial_hash* to the compressed result
- Repeat until all message blocks have been hashed

# Vulkan

![](figs/vulkan.png)

Lets us write cross-platform (compute) code using *open source* standards to interface with GPUs.

# Vulkan

![Example of specifying GPU interaction](figs/vulkan-spec.png){width=200px}

# Vulkan SPIR-V

![Example of SPIR-V taken from our SHA256 implementation](figs/vulkan-spirv.png){width=250px}

# Vulkan SPIR-V

![Rust-gpu is used to compile rust into SPIR-V](figs/vulkan-rust-gpu.png)

# Results
