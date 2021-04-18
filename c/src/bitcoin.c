#include "test.h"
#include "sha256.h"
#include <stdio.h>
#include <stdlib.h>

#include "blocks_from_csv.h"

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
