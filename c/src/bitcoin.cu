#include "test.h"
#include <stdio.h>

#include "blocks_from_csv.h"

// linking cuda too hard
#include "sha256.cu"

char* run_sha256(unsigned char *block_buf, int *block_starts, int num_blocks);

int main(int argc, char *argv[]) {
  if (argc < 2) {
    printf("Must have at least one argument\n");
    return -1;
  }
  struct Blocks b =  blocks_from_csv(argv[1]);

  /*
  printf("main:\n");
  for (int i = 0; i < b.block_starts[b.num_blocks]; i+=1) {
    int x = (unsigned char)b.block_buf[i];
    printf("%02x,", x);
  }
  printf("\n");
  for (int i = 0; i < b.num_blocks+1; ++i) {
    printf("%d ", b.block_starts[i]);
  }
  printf("\n");
  */

  char* _hashes = run_sha256((unsigned char *)b.block_buf, b.block_starts, b.num_blocks);
  free(b.block_starts);
  free(b.block_buf);

  strcmp(b.hashes[0], strtok(_hashes, " ")) != 0 ? printf("%d: True\n", 0) : printf("%d: False\n", 0);
  free(b.hashes[0]);
  for (int i = 1; i < b.num_blocks; ++i) {
    strcmp(b.hashes[i], strtok(NULL, " ")) != 0 ? printf("%d: True\n", i) : printf("%d: False\n", i);
    free(b.hashes[i]);
  }
  free(b.hashes);
  free(_hashes);
  return 0;
}

__global__ void kernel(unsigned char *block_buf, int *block_starts, int num_blocks, unsigned char* digests) {
  int i = threadIdx.x + blockIdx.x * blockDim.x;;
  if (i < num_blocks) {
    SHA256(&block_buf[block_starts[i]], block_starts[i+1] - block_starts[i], &digests[i*SHA256_DIGEST_LENGTH]);
    SHA256(&digests[i*SHA256_DIGEST_LENGTH], SHA256_DIGEST_LENGTH, &digests[i*SHA256_DIGEST_LENGTH]);
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

  kernel<<<1, num_blocks>>>(dev_block_buf, dev_block_starts, num_blocks, dev_digests);
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
