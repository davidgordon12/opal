INCLUDE_ROOT_PATH := include

INCLUDE := -I$(INCLUDE_ROOT_PATH)
TARGET := target/opalc
FLAGS := -std=c11 --pedantic -g -Wall -Wextra $(INCLUDE)

source_files := src/*.c
compiler_source_files := src/compiler/*.c

test_source_files := tests/*.c

object_files := bin/*.o

file := samples/test.opal

run: build
	./$(TARGET)

debug: build
	gdb $(TARGET)

run-file: build
	./$(TARGET) $(file)
