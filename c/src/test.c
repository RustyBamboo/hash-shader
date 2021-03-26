#include <stdio.h>
#include <stdlib.h>
#include "sha256.h"

static int test_idx = 0;
void test_sha256(unsigned char* d, int d_len, char answer[SHA256_DIGEST_LENGTH * 2 + 1]);

// test cases generated from https://emn178.github.io/online-tools/sha256.html

int main(int argc, char** argv) {
  // test 0
  unsigned char d0[4] = {0x0, 0x63, 0x62, 0x61};
  char answer0[SHA256_DIGEST_LENGTH * 2 + 1] =
      "4270e117797344c63175638fb462d26559ea1f2208a6821016d8288d690fea6a";
  test_sha256(d0, 4, answer0);

  // test 1
  unsigned char d1[] = "616263646263646563646566646566676566676866676869668696a68696a6b696a6b6c6a6b6c6d6b6c6d6e6c6d6e6f6d6e6f706e6f7071";
  char answer1[(SHA256_DIGEST_LENGTH * 2) + 1] = "61f92f340b780498bf1e18dced26e17a53596aa12b09a8310f0f3de0b9559b54";
  test_sha256(d1, strlen((char*)d1), answer1);

  // test 2
  unsigned char d2[] = "One million of61";
  char answer2[(SHA256_DIGEST_LENGTH * 2) + 1] = "2e62db52ef5ce6df4ecc030e73b095cc19ca2c62c99a7a9b0b123f30eeb20598";
  test_sha256(d2, strlen((char*)d2), answer2);

  return 0;
}

void test_sha256(unsigned char* d, int d_len, char answer[SHA256_DIGEST_LENGTH * 2 + 1])
{
  unsigned char digest[SHA256_DIGEST_LENGTH] = {};
  SHA256(d, d_len, digest);
  char res[(SHA256_DIGEST_LENGTH * 2) + 1] = "";
  for (int i = 0; i < SHA256_DIGEST_LENGTH; ++i) {
    sprintf(res + i * 2, "%02x", digest[i]);
  }
  if (strcmp(res, answer) != 0) {
    fprintf(stderr, "sequencial sha256 failed test %d!\n got:\n\t%s\n expected:\n\t%s\n", test_idx, res, answer);
    exit(1);
  }
  test_idx += 1;
}
