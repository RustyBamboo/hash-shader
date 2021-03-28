#include <stdio.h>
#include <stdlib.h>
#include "sha256.h"
#include "test.h"

static int test_idx = 0;
void test_sha256(unsigned char* d, int d_len,
                 char* answer, int N);

// test cases generated from https://emn178.github.io/online-tools/sha256.html

int main() {
  test(test_sha256, (SHA256_DIGEST_LENGTH * 2) + 1);
  return 0;
}

void test_sha256(unsigned char* d, int d_len,
                 char* answer, int N) {
  unsigned char digest[SHA256_DIGEST_LENGTH] = {};
  SHA256(d, d_len, digest);
  char res[(SHA256_DIGEST_LENGTH * 2) + 1] = "";
  for (int i = 0; i < SHA256_DIGEST_LENGTH; ++i) {
    sprintf(res + i * 2, "%02x", digest[i]);
  }
  if (strcmp(res, answer) != 0) {
    fprintf(
        stderr,
        "sequential sha256 failed test %d!\n got:\n\t%s\n expected:\n\t%s\n",
        test_idx, res, answer);
    exit(1);
  } else {
    fprintf(stdout,
            "sequential sha256 passed test %d\ngot:\n\t%s\n expected:\n\t%s\n\n",
            test_idx, res, answer);
  }
  test_idx += 1;
}
