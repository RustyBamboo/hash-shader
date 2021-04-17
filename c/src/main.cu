#include "test.h"
#include <stdio.h>

#include "blocks_from_cli.h"

// linking cuda too hard
#include "sha256.cu"

void run_sha256(unsigned char *block_buf, int *block_starts, int num_blocks);

int main(int argc, char *argv[]) {
  if (argc < 2) {
	printf("Must have at least one argument\n");
	return -1;
  }
  struct Blocks b = blocks_from_cli(argc, argv);
  run_sha256((unsigned char *)b.block_buf, b.block_starts, b.num_blocks);
  free(b.block_starts);
  free(b.block_buf);
  return 0;
}

__global__ void kernel(unsigned char *block_buf, int *block_starts, int num_blocks, unsigned char* digests) {
  int i = threadIdx.x + blockIdx.x * blockDim.x;;
  if (i < num_blocks) {
    SHA256(&block_buf[block_starts[i]], block_starts[i+1] - block_starts[i], &digests[i*SHA256_DIGEST_LENGTH]);
  }
}

void run_sha256(unsigned char *block_buf, int *block_starts, int num_blocks) {

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
  char res[num_blocks*SHA256_DIGEST_LENGTH + num_blocks] = "";
  int j = 0;
  for (int i = 0; i < num_blocks*SHA256_DIGEST_LENGTH; ++i) {
    if (i % SHA256_DIGEST_LENGTH == 0) {
      sprintf(&res[j], " ");
      j += 1;
    }
    sprintf(&res[j], "%02x", digests[i]);
    j += 2;
  }
  fprintf(stdout, "%s\n", res);

  cudaFree(dev_block_buf);
  cudaFree(dev_block_starts);
  cudaFree(dev_digests);
}
