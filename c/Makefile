all: cpu_test cpu_bitcoin cuda_test cuda_bitcoin

SHA256=-Lbuild -lsha256
SHA256_CU=-Lbuild -lsha256_cu

# https://patrickorcl.medium.com/compile-with-nvcc-3566fbdfdbf
arch=sm_75

INCLUDES=-Iinclude
FLAGS= -Wall -g $(INCLUDES)

cuda_bitcoin: src/bitcoin.cu src/sha256.cu include/sha256_cu.h include/test.h include/blocks_from_csv.h
	nvcc -arch=$(arch) -o cuda_bitcoin src/bitcoin.cu $(INCLUDES)

cuda_test: src/test.cu src/sha256.cu include/sha256_cu.h include/test.h
	nvcc -arch=$(arch) -o cuda_test src/test.cu $(INCLUDES)

cpu_bitcoin: src/bitcoin.c build/libsha256.a include/blocks_from_csv.h
	cc -o cpu_bitcoin src/bitcoin.c $(FLAGS) $(SHA256)

cpu_test: src/test.c include/test.h build/libsha256.a
	cc -o cpu_test src/test.c $(FLAGS) $(SHA256)

build/libsha256.a: include/sha256.h include/md32_common.h src/sha256.c
	cc -c -o build/sha256.o src/sha256.c $(FLAGS)
	ar rcs build/libsha256.a build/sha256.o
