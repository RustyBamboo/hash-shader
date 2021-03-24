---
title: Hash Validation and Vulkan Compute
author:
    - "Daniel Volya"
    - "Marshall Rawson"
abstract: "The project consists of two primary goals: 1) implement kernels that validate several hashes (such as to validate the blocks in a blockchain) in parallel, and 2) investigate using the open-source Vulkan standard as an alternative to CUDA"
---

# Validating Hash

# Vulkan Compute and SPIR-V

The state-of-the-art compute library for GPGPUs is CUDA. AMD has their own variant called ROCm. Both are proprietary and closed source. OpenCL was a promising open-source and cross-compatible library, but it lost support. Vulkan is the new-ish, cross-compatible (AMD, Nvidia, Intel, etc) and open-source standard. Vulkan has gained a lot of traction and support, and has been advancing in GPU features such as subgroups, pointers, and even a memory model. But the universality and features come at a cost of requiring developers to be more careful about the details (transferring data, specifying layouts and groups, etc.) 

Vulkan ingests code written in the Standard Portable Intermediate Representation (SPIR-V). This is an intermediate language similar to LLVM but specifically for parallel compute. Compilers for Higher level languages have been made that generate SPIR-V code. For example, one of Google's projects [clspv](https://github.com/google/clspv) is a compiler that converts a subset of OpenCL C into SPIR-V compute shaders. Additionally, KhronosGroup has a bi-directional LLVM/SPIR-V translator ([SPIRV-LLVM-Translator](https://github.com/KhronosGroup/SPIRV-LLVM-Translator)), which can allow for LLVM to be translated into SPIR-V. One idea to generate SPIR-V is to write kernels in Rust/C++/Python, use rustc/clang/numba to generate LLVM, and use bi-directional translator to obtain the SPIR-V code.

Hence, three primary goals for the Vulkan part of the project:

1. Develop a CUDA-like API for interfacing with a GPU, but via the Vulkan API
2. Write the hash validation kernel in a higher level language and obtain SPIR-V
3. Compare the performance between our CUDA-based implementation and our Vulkan-based implementation