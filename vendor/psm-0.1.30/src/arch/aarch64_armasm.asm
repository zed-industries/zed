    AREA |.text|, CODE, READONLY

    GLOBAL |rust_psm_stack_direction|
    ALIGN 4
|rust_psm_stack_direction| PROC
    orr w0, wzr, #2
    ret
    ENDP


    GLOBAL |rust_psm_stack_pointer|
    ALIGN 4
|rust_psm_stack_pointer| PROC
    mov x0, sp
    ret
    ENDP


    GLOBAL |rust_psm_replace_stack|
    ALIGN 4
|rust_psm_replace_stack| PROC
    mov sp, x2
    br x1
    ENDP

    GLOBAL |rust_psm_on_stack|
    ALIGN 4
|rust_psm_on_stack| PROC
    stp x29, x30, [sp, #-16]!
    mov x29, sp
    mov sp, x3
    blr x2
    mov sp, x29
    ldp x29, x30, [sp], #16
    ret
    ENDP

    END
