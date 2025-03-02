global sum_to_n

section .text

sum_to_n:
	mov rax, rdi
	add rdi, 1
	imul rax, rdi 
	shr rax, 1
	ret
