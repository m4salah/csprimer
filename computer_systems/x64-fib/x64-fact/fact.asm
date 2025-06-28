section .text
global fact
fact:
	; rdi: is the input number n

	; this is the base case
	cmp rdi, 0
	je fact_0

	push rdi     ; save the original rdi
	sub rdi, 1   ; subtract 1 to caluclate fact(rdi - 1)
	call fact    ; fact(rdi - 1)
	pop rdi      ; retrieve the original rdi
	mul rdi      ; rax now hase fact(rdi - 1), multiply to rdi
	ret

fact_0:
	mov rax, 1
	ret
