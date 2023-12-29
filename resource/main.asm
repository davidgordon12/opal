global _start

%include "data.asm"

section .data
	x dd 3.14159

section .text
_start:
	movss xmm0, [b]

	mov rax, 1
	mov rbx, 0
	int 0x80
