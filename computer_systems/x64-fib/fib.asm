section .text
global fib
fib:
	; rdi: is the input number n

	; this is the base case
	cmp rdi, 0
	je fib_0
	cmp rdi, 1
	je fib_1

	push rdi   ; save the original rdi
	sub rdi, 1 ; subtract 1 to caluclate fib(rdi - 1)
	call fib   ; fib(rdi - 1)
	pop rdi    ; retrieve the original rdi
	push rax   ; save fib(rdi - 1)
	sub rdi, 2 ; rdi - 2
	call fib   ; calculate fib(rdi - 2)
	pop rcx    ; retrieve fib(rdi - 1) -> rcx
	add rax, rcx ; rax now hase fib(rdi - 2), add to it fib(rdi - 1)
	ret

fib_0:
	mov rax, 0
	ret
fib_1:
	mov rax, 1
	ret
