.csect .text[PR],2
.file "powerpc64_aix.s"

.globl  rust_psm_stack_direction[DS]
.globl  .rust_psm_stack_direction
.align  4
.csect rust_psm_stack_direction[DS],3
.vbyte 8, .rust_psm_stack_direction
.vbyte 8, TOC[TC0]
.vbyte 8, 0
.csect .text[PR],2
.rust_psm_stack_direction:
# extern "C" fn() -> u8
  li 3, 2
  blr
L..rust_psm_stack_direction_end:
# Following bytes form the traceback table on AIX.
# For specification, see https://www.ibm.com/docs/en/aix/7.2?topic=processor-traceback-tables.
# For implementation, see https://github.com/llvm/llvm-project/blob/main/llvm/lib/Target/PowerPC/PPCAsmPrinter.cpp,
# `PPCAIXAsmPrinter::emitTracebackTable`.
.vbyte 4, 0x00000000 # Traceback table begin, for unwinder to search the table.
.byte 0x00 # Version = 0
.byte 0x09 # Language = CPlusPlus, since rust is using C++-like LSDA.
.byte 0x20 # -IsGlobaLinkage, -IsOutOfLineEpilogOrPrologue
           # +HasTraceBackTableOffset, -IsInternalProcedure
           # -HasControlledStorage, -IsTOCless
           # -IsFloatingPointPresent
           # -IsFloatingPointOperationLogOrAbortEnabled
.byte 0x40 # -IsInterruptHandler, +IsFunctionNamePresent, -IsAllocaUsed
           # OnConditionDirective = 0, -IsCRSaved, -IsLRSaved
.byte 0x80 # +IsBackChainStored, -IsFixup, NumOfFPRsSaved = 0
.byte 0x00 # -HasExtensionTable, -HasVectorInfo, NumOfGPRsSaved = 0
.byte 0x00 # NumberOfFixedParms = 0
.byte 0x01 # NumberOfFPParms = 0, +HasParmsOnStack
.vbyte 4, L..rust_psm_stack_direction_end-.rust_psm_stack_direction #Function size
.vbyte 2, 0x0018 # Function name len = 24
.byte "rust_psm_stack_direction" # Function Name

.globl rust_psm_stack_pointer[DS]
.globl .rust_psm_stack_pointer
.align 4
.csect rust_psm_stack_pointer[DS],3
.vbyte 8, .rust_psm_stack_pointer
.vbyte 8, TOC[TC0]
.vbyte 8, 0
.csect .text[PR],2
.rust_psm_stack_pointer:
# extern "C" fn() -> *mut u8
  mr 3, 1
  blr
L..rust_psm_stack_pointer_end:
.vbyte 4, 0x00000000
.byte 0x00
.byte 0x09
.byte 0x20
.byte 0x40
.byte 0x80
.byte 0x00
.byte 0x00
.byte 0x01
.vbyte 4, L..rust_psm_stack_pointer_end-.rust_psm_stack_pointer
.vbyte 2, 0x0016
.byte "rust_psm_stack_pointer"

.globl rust_psm_replace_stack[DS]
.globl .rust_psm_replace_stack
.align 4
.csect rust_psm_replace_stack[DS],3
.vbyte 8, .rust_psm_replace_stack
.vbyte 8, TOC[TC0]
.vbyte 8, 0
.csect .text[PR],2
.rust_psm_replace_stack:
# extern "C" fn(3: usize, 4: extern "C" fn(usize), 5: *mut u8)
  # Load the function pointer and toc pointer from TOC and make the call.
  ld 2, 8(4)
  ld 4, 0(4)
  addi 5, 5, -48
  mr 1, 5
  mtctr 4
  bctr
L..rust_psm_replace_stack_end:
.vbyte 4, 0x00000000
.byte 0x00
.byte 0x09
.byte 0x20
.byte 0x40
.byte 0x80
.byte 0x00
.byte 0x03
.byte 0x01
.vbyte 4, 0x00000000 # Parameter type = i, i, i
.vbyte 4, L..rust_psm_replace_stack_end-.rust_psm_replace_stack
.vbyte 2, 0x0016
.byte "rust_psm_replace_stack"

.globl rust_psm_on_stack[DS]
.globl .rust_psm_on_stack
.align 4
.csect rust_psm_on_stack[DS],3
.vbyte 8, .rust_psm_on_stack
.vbyte 8, TOC[TC0]
.vbyte 8, 0
.csect .text[PR],2
.rust_psm_on_stack:
# extern "C" fn(3: usize, 4: usize, 5: extern "C" fn(usize, usize), 6: *mut u8)
  mflr 0
  std 2, -72(6)
  std 0, -8(6)
  sub 6, 6, 1
  addi 6, 6, -112
  stdux 1, 1, 6
  ld 2, 8(5)
  ld 5, 0(5)
  mtctr 5
  bctrl
  ld 2, 40(1)
  ld 0, 104(1)
  mtlr 0
  ld 1, 0(1)
  blr
L..rust_psm_on_stack_end:
.vbyte 4, 0x00000000
.byte 0x00
.byte 0x09
.byte 0x20
.byte 0x41
.byte 0x80
.byte 0x00
.byte 0x04
.byte 0x01
.vbyte 4, 0x00000000 # Parameter type = i, i, i, i
.vbyte 4, L..rust_psm_on_stack_end-.rust_psm_on_stack
.vbyte 2, 0x0011
.byte "rust_psm_on_stack"

.toc
