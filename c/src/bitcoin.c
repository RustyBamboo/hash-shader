#include "test.h"
#include "sha256.h"
#include <stdio.h>
#include <stdlib.h>

#include "test_block_chain.h"

char* run_sha256(unsigned char *block_buf, int *block_starts, int num_blocks);


int main(int argc, char *argv[]) {
  if (argc < 2) {
    printf("Must have at least one argument\n");
    return -1;
  }
  test_block_chain(argv[1], run_sha256);
  return 0;
}

char* run_sha256(unsigned char *block_buf, int *block_starts, int num_blocks) {
  unsigned char* digests = (unsigned char*)malloc(SHA256_DIGEST_LENGTH * num_blocks);
  for (int i = 0; i < num_blocks; ++i) {
    SHA256(&block_buf[block_starts[i]], block_starts[i+1] - block_starts[i], &digests[i*SHA256_DIGEST_LENGTH]);
    SHA256(&digests[i*SHA256_DIGEST_LENGTH], SHA256_DIGEST_LENGTH, &digests[i*SHA256_DIGEST_LENGTH]);
  }
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
  free(digests);
  return res;
}
