run:
	mkdir -p bin
	nasm -f elf64 bin.opal.asm -o bin/bin.o
	mv bin.opal.asm bin/bin.opal.asm
	ld bin/bin.o -o bin/bin
