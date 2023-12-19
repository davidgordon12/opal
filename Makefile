INCLUDE_ROOT_PATH := include
INCLUDE_VM_PATH :=include/vm

INCLUDE := -I$(INCLUDE_ROOT_PATH) -I$(INCLUDE_VM_PATH)
TARGET := target/opal.out
FLAGS := -std=c11 --pedantic -g -Wall -Wextra $(INCLUDE)

vm_source_files := src/vm/*.c

test_source_files := tests/*.c

object_files := bin/*.o

file := samples/test.opal

run: build
	./$(TARGET)

build:
	mkdir -p unix
	gcc $(FLAGS) $(vm_source_files) -o $(TARGET)


debug: build
	gdb $(TARGET)

run-file: build
	./$(TARGET) $(file)
