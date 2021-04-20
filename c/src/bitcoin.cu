#include "test.h"
#include <stdio.h>

#include "test_block_chain.h"

// linking cuda too hard
#include "sha256.cu"

char* run_sha256(unsigned char *block_buf, int *block_starts, int num_blocks);


void* pinned_alloc(size_t n) {
  void* h_aPinned = NULL;
  cudaError_t status = cudaMallocHost((void**)&h_aPinned, n);
  if (status != cudaSuccess) {
    printf("Error allocating pinned host memory\n");
    exit(-1);
  }
  return h_aPinned;
}

void pinned_free(void* p) {
  cudaFreeHost(p);
}

int main(int argc, char *argv[]) {
  if (argc < 2) {
    printf("Must have at least one argument\n");
    return -1;
  }

  test_block_chain(argv[1], argc > 2 ? atoi(argv[2]) : -1, run_sha256, pinned_alloc, pinned_free);
  return 0;
}

__global__ void kernel(unsigned char *block_buf, int *block_starts, int num_blocks, unsigned char* digests) {
  int i = threadIdx.x + blockIdx.x * blockDim.x;
  if (i < num_blocks) {
    unsigned char intermidiate_digest[SHA256_DIGEST_LENGTH];
    int front = block_starts[i];
    int back = block_starts[i+1];
    SHA256(block_buf + front, back - front, intermidiate_digest);
    __syncthreads();
    SHA256(intermidiate_digest, SHA256_DIGEST_LENGTH, digests+(i*SHA256_DIGEST_LENGTH));
  }
}

char* run_sha256(unsigned char *block_buf, int *block_starts, int num_blocks) {

  cudaDeviceSynchronize();
  unsigned char *dev_block_buf;
  cudaMallocManaged((void **)&dev_block_buf, block_starts[num_blocks]);
  cudaMemcpy(dev_block_buf, block_buf, block_starts[num_blocks], cudaMemcpyHostToDevice);

  int *dev_block_starts;
  cudaMallocManaged((void **)&dev_block_starts, sizeof(int)*(num_blocks+1));
  cudaMemcpy(dev_block_starts, block_starts, sizeof(int)*(num_blocks+1), cudaMemcpyHostToDevice);

  unsigned char *dev_digests;
  cudaMallocManaged((void **)&dev_digests, SHA256_DIGEST_LENGTH * num_blocks);
  unsigned char digests[SHA256_DIGEST_LENGTH * num_blocks] = {};

  int num_thread_blocks = (num_blocks / 256) + 1;
  dim3 threadsPerThreadBlock(256);

  kernel<<<num_thread_blocks, threadsPerThreadBlock>>>(dev_block_buf, dev_block_starts, num_blocks, dev_digests);
  cudaDeviceSynchronize();
  cudaError_t error = cudaGetLastError();
  if (error != cudaSuccess) {
    printf("CUDA error: %s\n", cudaGetErrorString(error));
    exit(-1);
  }

  cudaMemcpy(digests, dev_digests, SHA256_DIGEST_LENGTH*num_blocks, cudaMemcpyDeviceToHost);
  int res_len = (num_blocks * SHA256_DIGEST_LENGTH * 2) + num_blocks;
  char* res = (char*)malloc(res_len);
  int j = 0;
  for (int i = 0; i < num_blocks*SHA256_DIGEST_LENGTH; ++i) {
    if (i % SHA256_DIGEST_LENGTH == 0) {
      sprintf(&res[j], " ");
      j += 1;
    }
    sprintf(&res[j], "%02x", digests[i]);
    j += 2;
  }
  cudaFree(dev_block_buf);
  cudaFree(dev_block_starts);
  cudaFree(dev_digests);
  return res;
}
