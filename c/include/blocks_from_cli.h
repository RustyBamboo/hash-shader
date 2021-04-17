#include <stdlib.h>

struct Blocks
{
  char* block_buf;
  int* block_starts;
  int num_blocks;
};

struct Blocks blocks_from_cli(int argc, char** argv) {

  int block_buf_size = 0;

  int* block_starts = (int*)malloc(argc);
  block_starts[0] = 0;

  for (int i = 1; i < argc; ++i) {
    int block_size = strlen(argv[i]);
    block_starts[i] = block_size + block_starts[i-1];
    block_buf_size += block_size;
  }
  block_starts[argc-1] = block_buf_size;

  char* block_buf = (char*)malloc(block_buf_size);

  for (int i = 0; i < argc-1; ++i) {
    memcpy(&block_buf[block_starts[i]], argv[i+1], block_starts[i+1] - block_starts[i]);
  }
  struct Blocks ret = {block_buf, block_starts, argc-1};
  return ret;
}
