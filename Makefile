INCLUDE_ROOT_PATH := include
INCLUDE_COMPILER_PATH :=include/compiler

INCLUDE := -I$(INCLUDE_ROOT_PATH) -I$(INCLUDE_COMPILER_PATH)
TARGET := target/opalc
FLAGS := -std=c11 --pedantic -g -Wall -Wextra $(INCLUDE)

source_files := src/*.c
compiler_source_files := src/compiler/*.c

test_source_files := tests/*.c

object_files := bin/*.o

file := samples/test.opal

run: build
	./$(TARGET)

build:
	gcc $(FLAGS) $(source_files) $(compiler_source_files) -o $(TARGET)

debug: build
	gdb $(TARGET)

run-file: build
	./$(TARGET) $(file)
