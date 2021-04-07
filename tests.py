#!/usr/bin/python3

import subprocess
import time

tests = {
    'abc': ['abc'],
    'rust': ['"rust is cool"'],
    'multiple_3': ['1', '2', '3'],
    'multiple_50': [f"{i}XD" for i in range(50)],
    'multiple_250': [f"{i}XD" for i in range(250)],
    'multiple_1000': [f"{i}XD" for i in range(1000)],
    'multiple_10000': [f"{i}XD" for i in range(10000)],
}

def run(func, *args, **kwargs):
    start = time.time()
    out = func(*args, **kwargs)
    end = time.time()
    return end - start, out

if __name__ == '__main__':
    vulkan_runner = './vulkan/target/release/vulkan'
    cuda_runner = './c/cuda_main'

    elapsed_times = []

    for name, params in tests.items():
        print(f"Running {name}")
        elapsed_cuda, _ = run(subprocess.check_output, f"{cuda_runner} {' '.join(params)}", shell=True)
        elapsed_vulkan, _ = run(subprocess.check_output, f"{vulkan_runner} {' '.join(params)}", shell=True)
        elapsed_times.append([elapsed_cuda, elapsed_vulkan])

    print("{:<30}{:<30}".format("CUDA", "VULKAN"))
    for e in elapsed_times:
        print("{:<30}{:<30}".format(*e))

