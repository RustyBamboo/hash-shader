all: test

SHA256=-Lbuild -lsha256

INCLUDES=-Iinclude
FLAGS= -Wall -g $(INCLUDES)


test: src/test.c build/libsha256.a
	cc -o test src/test.c $(FLAGS) $(SHA256)

build/libsha256.a: include/sha256.h include/md32_common.h src/sha256.c
	cc -c -o build/sha256.o src/sha256.c $(FLAGS)
	ar rcs build/libsha256.a build/sha256.o
