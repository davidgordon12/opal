build:
	mkdir -p bin
	nasm -f elf64 bin.opal.asm -o bin/bin.o
	gcc bin/bin.o -o bin/bin.out

debug: build
	gdb bin/bin.out
