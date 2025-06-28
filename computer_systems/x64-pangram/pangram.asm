%define mask 0x07fffffe

section .data
msg: db "", 0 ; 10 is for newline

section .text

global pangram
pangram:
	; rdi: source string
	xor ecx, ecx ; uint32_t bs = 0;
	start_loop:
	movzx edx, byte [rdi] ; c = *s
	cmp edx, 0 ; check for null terminator (c = *s) != '\0'
	je exit_loop
	inc rdi ; s++
	cmp edx, '@' ; check if c < '@'
	jl start_loop ; continue
	and edx, 0x1f ; c & 0x1f
	bts ecx, edx  ; bs |= 1 << (c & 0x1f)
	jmp start_loop

exit_loop:
	xor eax, eax  ; clear eax
	and ecx, mask ; bs & MASK
	cmp ecx, mask ; (bs & MASK) == MASK
	je true
	ret
true:
	mov eax, 1
	ret
