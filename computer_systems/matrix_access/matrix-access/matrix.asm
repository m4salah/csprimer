section .text
global index
index:
	; rdi: matrix
	; esi: rows
	; edx: cols
	; ecx: rindex
	; r8d: cindex
	mov rax, rsi
	mul r8
	add rax, rcx
	imul rax, 4
	add rax, rdi
	mov rax, [rax]
	ret
