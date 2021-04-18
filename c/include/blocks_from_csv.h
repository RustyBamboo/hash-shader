#include <stdlib.h>

struct Blocks
{
  char* block_buf;
  int* block_starts;
  int num_blocks;
  char** hashes;
};

struct Blocks blocks_from_csv(const char* file_path) {

  int block_count = 0;

  FILE* bc = fopen(file_path, "r");

  // see how many lines
  for (char c = getc(bc); c != EOF; c = getc(bc))
    if (c == '\n')
      block_count += 1;

  fclose(bc);
  bc = fopen(file_path, "r");

  char** blocks = (char**)malloc(sizeof(char*)*block_count);
  char** hashes = (char**)malloc(sizeof(char*)*block_count);

  for (int i = 0; i < block_count; i+=1) {
    char* line = NULL;
    size_t n = 0;
    int l = getline(&line, &n, bc);
    if (l < 0) {
      perror("");
      exit(-1);
    }
    char* s = strtok(line, ",");
    blocks[i] = (char*)malloc(sizeof(char)*strlen(s)+1);
    strcpy(blocks[i], s);
    s = strtok(NULL, ",");
    //printf("%s\n", s);
    hashes[i] = (char*)malloc(sizeof(char)*strlen(s)+1);
    strcpy(hashes[i], s);
    free(line);
  }

  fclose(bc);
  int block_buf_size = 0;

  int* block_starts = (int*)malloc(sizeof(int)*(block_count+1));
  block_starts[0] = 0;

  for (int i = 0; i < block_count; ++i) {
    int block_size = strlen(blocks[i])/2;
    block_starts[i+1] = block_size + block_starts[i];
    block_buf_size += block_size;
    //printf("%d ", block_starts[i]);
  }
  block_starts[block_count] = block_buf_size;
  //printf("%d\n", block_starts[block_count]);

  char* block_buf = (char*)malloc(block_buf_size);

  //printf("block_buf_size: %d\n", block_buf_size);

  for (int i = 0; i < block_count; ++i) {
    unsigned char* byte_array = (unsigned char*)malloc(strlen(blocks[i])/2);
    //printf("%s\n", blocks[i]);
    for (int j = 0; j < strlen(blocks[i]); j+=2) {
      int n;
      sscanf(&blocks[i][j], "%2x", &n);
      if (n < 0 || n > 255) {
        perror("Maformed hex");
        exit(-1);
      }
      byte_array[j/2] = (unsigned char)n;
      //printf("%02x,", byte_array[j/2]);
    }
    //printf("\n");
    memcpy(&block_buf[block_starts[i]], byte_array, block_starts[i+1] - block_starts[i]);
    free(byte_array);
  }
  struct Blocks ret = {block_buf, block_starts, block_count, hashes};
  for (int i = 0; i < block_count; ++i) {
    free(blocks[i]);
  }
  free(blocks);
  return ret;
}
