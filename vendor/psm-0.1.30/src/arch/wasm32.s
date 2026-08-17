#include "psm.h"
#include "gnu_stack_note.s"

# Note that this function is not compiled when this package is uploaded to
# crates.io, this source is only here as a reference for how the corresponding
# wasm32.o was generated. This file can be compiled with:
#
#    cpp psm/src/arch/wasm32.s | llvm-mc -o psm/src/arch/wasm32.o --arch=wasm32 -filetype=obj
#
# where you'll want to ensure that `llvm-mc` is from a relatively recent
# version of LLVM.

.globaltype __stack_pointer, i32

.globl rust_psm_stack_direction
.type rust_psm_stack_direction,@function
rust_psm_stack_direction:
.functype rust_psm_stack_direction () -> (i32)
    i32.const STACK_DIRECTION_DESCENDING
    end_function

.globl rust_psm_stack_pointer
.type rust_psm_stack_pointer,@function
rust_psm_stack_pointer:
.functype rust_psm_stack_pointer () -> (i32)
    global.get __stack_pointer
    end_function

.globl rust_psm_on_stack
.type rust_psm_on_stack,@function
rust_psm_on_stack:
.functype rust_psm_on_stack (i32, i32, i32, i32) -> ()
    # get our new stack argument, then save the old stack
    # pointer into that local
    local.get 3
    global.get __stack_pointer
    local.set 3
    global.set __stack_pointer

    # Call our indirect function specified
    local.get 0
    local.get 1
    local.get 2
    call_indirect (i32, i32) -> ()

    # restore the stack pointer before returning
    local.get 3
    global.set __stack_pointer
    end_function

.globl rust_psm_replace_stack
.type rust_psm_replace_stack,@function
rust_psm_replace_stack:
.functype rust_psm_replace_stack (i32, i32, i32) -> ()
    local.get 2
    global.set __stack_pointer
    local.get 0
    local.get 1
    call_indirect (i32) -> ()
    unreachable
    end_function
