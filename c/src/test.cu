#include "test.h"
#include <stdio.h>

// linking cuda too hard
#include "sha256.cu"

static int test_idx = 0;
void test_sha256(unsigned char *d, int d_len, char *answer, int N);

__global__ void kernel(unsigned char *d, int d_len, unsigned char *out) {
  SHA256(d, d_len, out);
}

int main() {
  test(test_sha256, (SHA256_DIGEST_LENGTH * 2) + 1);
  return 0;
}

void test_sha256(unsigned char *d, int d_len, char *answer, int N) {
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
  if (strcmp(res, answer) != 0) {
    fprintf(stderr,
            "cuda sha256 failed test %d\ngot:\n\t%s\n expected:\n\t%s\n\n",
            test_idx, res, answer);
  } else {
    fprintf(stdout,
            "cuda sha256 passed test %d\ngot:\n\t%s\n expected:\n\t%s\n\n",
            test_idx, res, answer);
  }
  test_idx += 1;
  cudaFree(d_c);
  cudaFree(digest_c);
}
