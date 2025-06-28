%define mask 0x07fffffe

section .data
msg: db "", 0 ; 10 is for newline

section .text

global pangram
pangram:
	; rdi: source string
	xor ecx, ecx ; the bit set of the result
	start_loop:
	movzx edx, byte [rdi]
	cmp edx, 0
	je exit_loop
	inc rdi
	cmp edx, '@'
	jl start_loop
	and edx, 0x1f
	bts ecx, edx
	jmp start_loop

exit_loop:
	and ecx, mask
	cmp ecx, mask
	jz true
	mov eax, 0
	ret
true:
	mov eax, 1
	ret
