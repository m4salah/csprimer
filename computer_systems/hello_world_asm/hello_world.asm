; nasm -felf64  hello_world.asm && ld hello_world.o -o hello_world && ./hello_world 

; this from https://github.com/hackclub/some-assembly-required/tree/main/code/x86-intel
; I added some comment to indicate my understanding.
global _start

section .text

_start:
	; all of this just to tell the operating system to write to the stdout.
	; https://blog.rchapman.org/posts/Linux_System_Call_Table_for_x86_64/
	mov rax, 1       ; it tells syscall when invoking is to write syscall
	mov rdi, 1       ; 1 is the file descriptor for the stdout
	mov rsi, msg     ; pointer (address) to the string which we will write it to the stdout
	mov rdx, msg.len ; how many bytes we want to write to the stdout it's just 14 bytes
	syscall          ; here we are invoking the syscall with the previuos info which contain what to invoke with all the info.
	
	; we want to exit with status code 0 for success
	mov rax, 60 ; tell syscall we are exiting
	mov rdi, 0  ; the code we are exiting with which is 0 stand for success.
	syscall
	
; here we are defining the data section which include the initialized static variable.
; https://en.wikipedia.org/wiki/Data_segment
section .data

; msg is the label we give.
; db stand for Data Bytes
; saves the ASCII number equivalent of this msg into memory, retrievable later by its label
; https://www.nasm.us/xdoc/2.15.05/html/nasmdoc3.html
msg: db "Hello, World!", 10 ; 10 is for newline
; Define an assemble-time constant, which is calculated during compilation
; Calculate len = string length.  subtract the address of the start of the string from the current position ($)
.len equ $ - msg

