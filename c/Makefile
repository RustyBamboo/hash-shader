all: test cuda_test

SHA256=-Lbuild -lsha256
SHA256_CU=-Lbuild -lsha256_cu

arch=sm_50

INCLUDES=-Iinclude
FLAGS= -Wall -g $(INCLUDES)

cuda_main: src/main.cu src/sha256.cu include/sha256_cu.h include/test.h
	nvcc -arch=$(arch) -o cuda_main src/main.cu $(INCLUDES)

cuda_test: src/test.cu src/sha256.cu include/sha256_cu.h include/test.h
	nvcc -arch=$(arch) -o cuda_test src/test.cu $(INCLUDES)

test: src/test.c include/test.h build/libsha256.a
	cc -o test src/test.c $(FLAGS) $(SHA256)

build/libsha256.a: include/sha256.h include/md32_common.h src/sha256.c
	cc -c -o build/sha256.o src/sha256.c $(FLAGS)
	ar rcs build/libsha256.a build/sha256.o
