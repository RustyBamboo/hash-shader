#include <stdio.h>
#include "blocks_from_csv.h"


void test_block_chain(char* csv_file, char*(*run)(unsigned char *block_buf, int *block_starts, int num_blocks)) {
  struct Blocks b =  blocks_from_csv(csv_file);

  printf("num blocks: %d\n", b.num_blocks);
  /*
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

  char* _hashes = run((unsigned char *)b.block_buf, b.block_starts, b.num_blocks);
  free(b.block_starts);
  free(b.block_buf);

  if (strcmp(b.hashes[0], strtok(_hashes, " ")) == 0 ) printf("%d: False\n", 0);
  free(b.hashes[0]);
  for (int i = 1; i < b.num_blocks; ++i) {
    if (strcmp(b.hashes[i], strtok(_hashes, " ")) == 0 ) printf("%d: False\n", i);
    free(b.hashes[i]);
  }
  free(b.hashes);
  free(_hashes);
}
