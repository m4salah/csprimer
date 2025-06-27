section .text
global index
index:
	; rdi: matrix
	; esi: rows
	; edx: cols
	; ecx: rindex
	; r8d: cindex
	imul rsi, r8
	add rsi, rcx
	imul rsi, 4
	mov rax, [rdi + rsi]
	ret
