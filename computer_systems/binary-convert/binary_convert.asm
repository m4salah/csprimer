section .text
global binary_convert
binary_convert:
	; rdi: pointer to the binary string
	xor eax, eax
loop:
	movzx ebx, byte [rdi]
	cmp ebx, 0
	je end
	inc rdi
	shl eax, 1   ; multiply the eax by 2
	add eax, ebx
	jmp loop
end:
	ret
