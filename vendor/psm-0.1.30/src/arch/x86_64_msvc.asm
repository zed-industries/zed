PUBLIC rust_psm_stack_direction
PUBLIC rust_psm_stack_pointer
PUBLIC rust_psm_replace_stack
PUBLIC rust_psm_on_stack

_TEXT SEGMENT

; extern "sysv64" fn() -> u8 (%al)
rust_psm_stack_direction PROC
    mov al, 2
    ret
rust_psm_stack_direction ENDP

; extern "sysv64" fn() -> *mut u8 (%rax)
rust_psm_stack_pointer PROC
    lea rax, [rsp + 8]
    ret
rust_psm_stack_pointer ENDP

; extern "sysv64" fn(%rdi: usize, %rsi: extern "sysv64" fn(usize), %rdx: *mut u8, %rcx: *mut u8)
rust_psm_replace_stack PROC
    mov gs:[08h], rdx
    mov gs:[10h], rcx
    lea rsp, [rdx - 8]
    jmp rsi
rust_psm_replace_stack ENDP

; extern "sysv64" fn(%rdi: usize, %rsi: usize,
;                    %rdx: extern "sysv64" fn(usize, usize), %rcx: *mut u8, %r8: *mut u8)
;
; NB: on Windows for SEH to work at all, the pointers in TIB, thread information block, need to be
; fixed up. Otherwise, it seems that exception mechanism on Windows will not bother looking for
; exception handlers at *all* if they happen to fall outside the are specified in TIB.
;
; This necessitates an API difference from the usual 4-argument signature used elsewhere.
;
; FIXME: this needs a catch-all exception handler that aborts in case somebody unwinds into here.
rust_psm_on_stack PROC FRAME
    push rbp
    .pushreg rbp
    mov rbp, rsp
    .setframe rbp, 0
    .endprolog

    push gs:[08h]
    mov gs:[08h], rcx
    push gs:[10h]
    mov gs:[10h], r8
    mov rsp, rcx
    call rdx
    lea rsp, [rbp - 010h]
    pop gs:[10h]
    pop gs:[08h]

    pop rbp
    ret
rust_psm_on_stack ENDP

_TEXT ENDS

END
