; Hello World example
; $ nasm -f bin -o test.bin asm/hello.asm
cpu 8086

mov ax, hello
mov byte [cursor], 0x0

_main:
    jmp helloloop

halt:
    hlt

helloloop:
    ; Print the current char
    call put

    ; Advance to the next character in our string
    inc ax

    ; Halt if we reach a null byte 
    mov bx, ax
    cmp byte[bx], 0x0
    jz halt
    jmp helloloop

put:
    push bx
    push dx
    push di

    ; Set dl to the char
    mov bx, ax
    mov dl, [bx]

    ; Display the char stored in dl
    mov bx, 0x8000
    mov di, [cursor]
    mov [bx+di], dl

    ; Move the cursor forward
    inc di
    mov [cursor], di

    pop di
    pop dx
    pop bx
    ret

.data
cursor dw 0x0
hello dw 'Hello, world!', 0x0
