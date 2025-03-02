global sum_to_n

section .text

sum_to_n:
	; store 2 for the future use
	mov rbx, 2
	; move n (rdi) to rax (the return value)
	mov rax, rdi
	; add 1 to the n (rdi) to be n + 1
	add rdi, 1
	; multiply eax to rdi => n * (n + 1) => rdi * rax
	; mul is taking one op https://www.felixcloutier.com/x86/mul
	; the op is getting muliplied to rax 
	; and the result get stored to RDX:RAX
	; this means that RDX stored the upper ordered 64 bits
	; and the RAX stores the lower ordered 64 bits
	mul rdi
	; we divide the result to 2 => n * (n + 1) / 2 => rdi * rax / 2
	; div is taking also 1 op, and the RDX:RAX is getting divided to this op
	; and the result get stored to the 
	; quotient -> RAX
	; reminder -> RDX
	shr rax, 1
	ret
