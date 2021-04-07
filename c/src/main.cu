#include "test.h"
#include <stdio.h>

// linking cuda too hard
#include "sha256.cu"
void run_sha256(unsigned char *d, int d_len, int N);

__global__ void kernel(unsigned char *d, int d_len, unsigned char *out) {
  SHA256(d, d_len, out);
}

int main(int argc, char *argv[]) {
  if (argc < 2) {
	printf("Must have at least one argument\n");
	return -1;
  }
  // TODO: work on several at strings at once
  run_sha256((unsigned char *)argv[1], strlen(argv[1]), (SHA256_DIGEST_LENGTH * 2));
  return 0;
}

void run_sha256(unsigned char *d, int d_len, int N) {
  cudaDeviceSynchronize();
  unsigned char *d_c;
  cudaMallocManaged((void **)&d_c, d_len);
  cudaMemcpy(d_c, d, d_len, cudaMemcpyHostToDevice);

  unsigned char *digest_c;
  cudaMallocManaged((void **)&digest_c, SHA256_DIGEST_LENGTH);
  unsigned char digest[SHA256_DIGEST_LENGTH] = {};

  kernel<<<1, 1>>>(d_c, d_len, digest_c);
  cudaDeviceSynchronize();
  cudaError_t error = cudaGetLastError();
  if (error != cudaSuccess) {
    printf("CUDA error: %s\n", cudaGetErrorString(error));
    exit(-1);
  }

  cudaMemcpy(digest, digest_c, SHA256_DIGEST_LENGTH, cudaMemcpyDeviceToHost);
  char res[N] = "";
  for (int i = 0; i < SHA256_DIGEST_LENGTH; ++i) {
    sprintf(res + i * 2, "%02x", digest_c[i]);
  }
  fprintf(stdout, "%s\n", res);

  cudaFree(d_c);
  cudaFree(digest_c);
}
