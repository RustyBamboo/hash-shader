#ifndef HASH_SHADER_TEST_H
#define HASH_SHADER_TEST_H

// test cases generated from https://emn178.github.io/online-tools/sha256.html

void test(void (*test_sha)(unsigned char* d, int d_len, char* answer, int N),
          int N) {
  // test 0
  {
  unsigned char d[4] = {0x0, 0x63, 0x62, 0x61};
  char answer[] =
      "4270e117797344c63175638fb462d26559ea1f2208a6821016d8288d690fea6a";
  test_sha(d, 4, answer, N);
  }
  // test 1
  {
  unsigned char d[] =
      "616263646263646563646566646566676566676866676869668696a68696a6b696a6b6c6"
      "a6b6c6d6b6c6d6e6c6d6e6f6d6e6f706e6f7071";
  char answer[] =
      "61f92f340b780498bf1e18dced26e17a53596aa12b09a8310f0f3de0b9559b54";
  test_sha(d, strlen((char*)d), answer, N);
  }
  // test 2
  {
  unsigned char d[] = "One million of61";
  char answer[] =
      "2e62db52ef5ce6df4ecc030e73b095cc19ca2c62c99a7a9b0b123f30eeb20598";
  test_sha(d, strlen((char*)d), answer, N);
  }
  // test 3
  {
  unsigned char d[] = "636261";
  char answer[] =
      "1e342cc50790e3baeb02dcfa23251547b994fc4b0571ee7b4858edd4aac3513d";
  test_sha(d, strlen((char*)d), answer, N);
  }
}
#endif
