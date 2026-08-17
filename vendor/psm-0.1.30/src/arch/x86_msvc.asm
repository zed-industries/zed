; FIXME: this is weird, this works locally but not on appveyor?!??!

.386
.model flat

ASSUME FS:NOTHING

; WTF: PUBLIC conflicts with "SYSCALL" but "SYSCALL" is the only way to stop MASM from manging the
; symbol names?
;
; PUBLIC @rust_psm_stack_direction@0
; PUBLIC @rust_psm_stack_pointer@0
; PUBLIC @rust_psm_replace_stack@12
; PUBLIC @rust_psm_on_stack@16

_TEXT SEGMENT

; extern "fastcall" fn() -> u8 (%al)
@rust_psm_stack_direction@0 PROC SYSCALL
    mov al, 2
    ret
@rust_psm_stack_direction@0 ENDP

; extern "fastcall" fn() -> *mut u8 (%rax)
@rust_psm_stack_pointer@0 PROC SYSCALL
    lea eax, [esp + 4]
    ret
@rust_psm_stack_pointer@0 ENDP

; extern "fastcall" fn(%ecx: usize, %edx: extern "fastcall" fn(usize),
;                      4(%esp): *mut u8, 8(%esp): *mut u8)
@rust_psm_replace_stack@16 PROC SYSCALL
    mov eax, dword ptr [esp + 8]
    mov fs:[08h], eax
    mov esp, dword ptr [esp + 4]
    mov fs:[04h], esp
    jmp edx
@rust_psm_replace_stack@16 ENDP

; extern "fastcall" fn(%ecx: usize, %edx: usize, 4(%esp): extern "fastcall" fn(usize, usize),
;                      8(%esp): *mut u8, 12(%esp): *mut u8)
;
; NB: on Windows for SEH to work at all, the pointers in TIB, thread information block, need to be
; fixed up. Otherwise, it seems that exception mechanism on Windows will not bother looking for
; exception handlers at *all* if they happen to fall outside the are specified in TIB.
;
; This necessitates an API difference from the usual 4-argument signature used elsewhere.
@rust_psm_on_stack@20 PROC SYSCALL
    push ebp
    mov ebp, esp

    push fs:[0E0Ch]
    push fs:[08h]
    mov eax, dword ptr [ebp + 4 + 12]
    mov dword ptr fs:[08h], eax
    mov dword ptr fs:[0E0Ch], eax
    push fs:[04h]
    mov esp, dword ptr [ebp + 4 + 8]
    mov dword ptr fs:[04h], esp
    call dword ptr [ebp + 4 + 4]

    lea esp, [ebp - 12]
    pop fs:[04h]
    pop fs:[08h]
    pop fs:[0E0Ch]
    pop ebp
    ret 12
@rust_psm_on_stack@20 ENDP

END
