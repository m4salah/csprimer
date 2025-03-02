global sum_to_n

section .text

sum_to_n:
	; zero the return value
	; rax is the return value in unix x86 calling convention
	xor rax, rax
; lable the loop
sum_loop:
	; rdi is the first argument in unix x86 calling convention 
	cmp rdi, 0   ; compare it zero
	je done      ; if it's equal to zero then we are done
	add rax, rdi ; add n to rax (the return value)
	sub rdi, 1   ; subtract 1 from n (rdi)
	jmp sum_loop ; loop again until we make rdi 0

done:
	ret
