global sum_to_n

section .text

sum_to_n:
	; zero the return value
	; rax is the return value in unix x86 calling convention
	xor rax, rax
; lable the loop
_sum_loop:
	; rdi is the first argument in unix x86 calling convention 
	add rax, rdi ; add n to rax (the return value)
	sub rdi, 1   ; subtract 1 from n (rdi)
	jg _sum_loop ; if the sub is 0 so this will never execute else it jumb back to the loop while is greater than 0
	ret
