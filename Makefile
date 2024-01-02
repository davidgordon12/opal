run:
	mkdir -p bin
	nasm -f elf64 bin.opal.asm -o bin/bin.o
	cp bin.opal.asm bin/bin.opal.asm
	rm bin.opal.asm
	ld bin/bin.o -o bin/bin
