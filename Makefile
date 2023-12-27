run:
	mkdir -p bin
	nasm -f elf64 tests/bin.opal.asm -o bin/bin.o
	rm tests/bin.opal.asm
	ld bin/bin.o -o bin/bin
	./bin/bin
	echo $?