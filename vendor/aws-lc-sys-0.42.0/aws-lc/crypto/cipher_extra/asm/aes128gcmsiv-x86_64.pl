#!/usr/bin/env perl

# Copyright (c) 2017, Shay Gueron.
# Copyright (c) 2017, Google Inc.
# SPDX-License-Identifier: ISC

use warnings FATAL => 'all';

# The first two arguments should always be the flavour and output file path.
if ($#ARGV < 1) { die "Not enough arguments provided.
  Two arguments are necessary: the flavour and the output file path."; }

$flavour = shift;
$output  = shift;

$win64=0; $win64=1 if ($flavour =~ /[nm]asm|mingw64/ || $output =~ /\.asm$/);

$0 =~ m/(.*[\/\\])[^\/\\]+$/; $dir=$1;
( $xlate="${dir}x86_64-xlate.pl" and -f $xlate ) or
( $xlate="${dir}../../perlasm/x86_64-xlate.pl" and -f $xlate) or
die "can't locate x86_64-xlate.pl";

$avx = 1;
for (@ARGV) { $avx = 0 if (/-DMY_ASSEMBLER_IS_TOO_OLD_FOR_AVX/); }

open OUT,"| \"$^X\" \"$xlate\" $flavour \"$output\"";
*STDOUT=*OUT;

$code.=<<___;
.section .rodata

.align 16
one:
.quad 1,0
two:
.quad 2,0
three:
.quad 3,0
four:
.quad 4,0
five:
.quad 5,0
six:
.quad 6,0
seven:
.quad 7,0
eight:
.quad 8,0

OR_MASK:
.long 0x00000000,0x00000000,0x00000000,0x80000000
poly:
.quad 0x1, 0xc200000000000000
mask:
.long 0x0c0f0e0d,0x0c0f0e0d,0x0c0f0e0d,0x0c0f0e0d
con1:
.long 1,1,1,1
con2:
.long 0x1b,0x1b,0x1b,0x1b
con3:
.byte -1,-1,-1,-1,-1,-1,-1,-1,4,5,6,7,4,5,6,7
and_mask:
.long 0,0xffffffff, 0xffffffff, 0xffffffff
___

$code.=<<___;
.text
___

sub gfmul {
  #########################
  # a = T
  # b = TMP0 - remains unchanged
  # res = T
  # uses also TMP1,TMP2,TMP3,TMP4
  # __m128i GFMUL(__m128i A, __m128i B);

  my $T = "%xmm0";
  my $TMP0 = "%xmm1";
  my $TMP1 = "%xmm2";
  my $TMP2 = "%xmm3";
  my $TMP3 = "%xmm4";
  my $TMP4 = "%xmm5";

  $code.=<<___;
.type GFMUL,\@abi-omnipotent
.align 16
GFMUL:
.cfi_startproc
    vpclmulqdq  \$0x00, $TMP0, $T, $TMP1
    vpclmulqdq  \$0x11, $TMP0, $T, $TMP4
    vpclmulqdq  \$0x10, $TMP0, $T, $TMP2
    vpclmulqdq  \$0x01, $TMP0, $T, $TMP3
    vpxor       $TMP3, $TMP2, $TMP2
    vpslldq     \$8, $TMP2, $TMP3
    vpsrldq     \$8, $TMP2, $TMP2
    vpxor       $TMP3, $TMP1, $TMP1
    vpxor       $TMP2, $TMP4, $TMP4

    vpclmulqdq  \$0x10, poly(%rip), $TMP1, $TMP2
    vpshufd     \$78, $TMP1, $TMP3
    vpxor       $TMP3, $TMP2, $TMP1

    vpclmulqdq  \$0x10, poly(%rip), $TMP1, $TMP2
    vpshufd     \$78, $TMP1, $TMP3
    vpxor       $TMP3, $TMP2, $TMP1

    vpxor       $TMP4, $TMP1, $T
    ret
.cfi_endproc
.size GFMUL, .-GFMUL
___
}
gfmul();

sub aesgcmsiv_htable_init {
  # aesgcmsiv_htable_init writes an eight-entry table of powers of |H| to
  # |out_htable|.
  # void aesgcmsiv_htable_init(uint8_t out_htable[16*8], uint8_t *H);

  my $Htbl = "%rdi";
  my $H = "%rsi";
  my $T = "%xmm0";
  my $TMP0 = "%xmm1";

$code.=<<___;
.globl aesgcmsiv_htable_init
.type aesgcmsiv_htable_init,\@function,2
.align 16
aesgcmsiv_htable_init:
.cfi_startproc
    _CET_ENDBR
    vmovdqa ($H), $T
    vmovdqa $T, $TMP0
    vmovdqa $T, ($Htbl)      # H
    call GFMUL
    vmovdqa $T, 16($Htbl)    # H^2
    call GFMUL
    vmovdqa $T, 32($Htbl)    # H^3
    call GFMUL
    vmovdqa $T, 48($Htbl)    # H^4
    call GFMUL
    vmovdqa $T, 64($Htbl)    # H^5
    call GFMUL
    vmovdqa $T, 80($Htbl)    # H^6
    call GFMUL
    vmovdqa $T, 96($Htbl)    # H^7
    call GFMUL
    vmovdqa $T, 112($Htbl)   # H^8
    ret
.cfi_endproc
.size aesgcmsiv_htable_init, .-aesgcmsiv_htable_init
___
}
aesgcmsiv_htable_init();

sub aesgcmsiv_htable6_init {
  # aesgcmsiv_htable6_init writes a six-entry table of powers of |H| to
  # |out_htable|.
  # void aesgcmsiv_htable6_init(uint8_t out_htable[16*6], uint8_t *H);
  #
  my $Htbl = "%rdi";
  my $H = "%rsi";
  my $T = "%xmm0";
  my $TMP0 = "%xmm1";

  $code.=<<___;
.globl aesgcmsiv_htable6_init
.type aesgcmsiv_htable6_init,\@function,2
.align 16
aesgcmsiv_htable6_init:
.cfi_startproc
    _CET_ENDBR
    vmovdqa ($H), $T
    vmovdqa $T, $TMP0
    vmovdqa $T, ($Htbl)      # H
    call GFMUL
    vmovdqa $T, 16($Htbl)    # H^2
    call GFMUL
    vmovdqa $T, 32($Htbl)    # H^3
    call GFMUL
    vmovdqa $T, 48($Htbl)    # H^4
    call GFMUL
    vmovdqa $T, 64($Htbl)    # H^5
    call GFMUL
    vmovdqa $T, 80($Htbl)    # H^6
    ret
.cfi_endproc
.size aesgcmsiv_htable6_init, .-aesgcmsiv_htable6_init
___
}
aesgcmsiv_htable6_init();

sub aesgcmsiv_htable_polyval {
  # void aesgcmsiv_htable_polyval(uint8_t Htbl[16*8], uint8_t *MSG, uint64_t LEN, uint8_t *T);
  # parameter 1: %rdi     Htable  - pointer to Htable
  # parameter 2: %rsi     INp     - pointer to input
  # parameter 3: %rdx     LEN     - length of BUFFER in bytes
  # parameter 4: %rcx     T       - pointer to POLYVAL output

  my $DATA = "%xmm0";
  my $hlp0 = "%r11";
  my $Htbl = "%rdi";
  my $inp = "%rsi";
  my $len = "%rdx";
  my $TMP0 = "%xmm3";
  my $TMP1 = "%xmm4";
  my $TMP2 = "%xmm5";
  my $TMP3 = "%xmm6";
  my $TMP4 = "%xmm7";
  my $Tp = "%rcx";
  my $T = "%xmm1";
  my $Xhi = "%xmm9";

  my $SCHOOLBOOK_AAD = sub {
    my ($i)=@_;
    return <<___;
    vpclmulqdq \$0x01, ${\eval(16*$i)}($Htbl), $DATA, $TMP3
    vpxor $TMP3, $TMP2, $TMP2
    vpclmulqdq \$0x00, ${\eval(16*$i)}($Htbl), $DATA, $TMP3
    vpxor $TMP3, $TMP0, $TMP0
    vpclmulqdq \$0x11, ${\eval(16*$i)}($Htbl), $DATA, $TMP3
    vpxor $TMP3, $TMP1, $TMP1
    vpclmulqdq \$0x10, ${\eval(16*$i)}($Htbl), $DATA, $TMP3
    vpxor $TMP3, $TMP2, $TMP2
___
  };

  $code.=<<___;
.globl aesgcmsiv_htable_polyval
.type aesgcmsiv_htable_polyval,\@function,4
.align 16
aesgcmsiv_htable_polyval:
.cfi_startproc
    _CET_ENDBR
    test  $len, $len
    jnz   .Lhtable_polyval_start
    ret

.Lhtable_polyval_start:
    vzeroall

    # We hash 8 blocks each iteration. If the total number of blocks is not a
    # multiple of 8, we first hash the leading n%8 blocks.
    movq $len, $hlp0
    andq \$127, $hlp0

    jz .Lhtable_polyval_no_prefix

    vpxor $Xhi, $Xhi, $Xhi
    vmovdqa ($Tp), $T
    sub $hlp0, $len

    sub \$16, $hlp0

    # hash first prefix block
    vmovdqu ($inp), $DATA
    vpxor $T, $DATA, $DATA

    vpclmulqdq \$0x01, ($Htbl,$hlp0), $DATA, $TMP2
    vpclmulqdq \$0x00, ($Htbl,$hlp0), $DATA, $TMP0
    vpclmulqdq \$0x11, ($Htbl,$hlp0), $DATA, $TMP1
    vpclmulqdq \$0x10, ($Htbl,$hlp0), $DATA, $TMP3
    vpxor $TMP3, $TMP2, $TMP2

    lea 16($inp), $inp
    test $hlp0, $hlp0
    jnz .Lhtable_polyval_prefix_loop
    jmp .Lhtable_polyval_prefix_complete

    # hash remaining prefix bocks (up to 7 total prefix blocks)
.align 64
.Lhtable_polyval_prefix_loop:
    sub \$16, $hlp0

    vmovdqu ($inp), $DATA           # next data block

    vpclmulqdq  \$0x00, ($Htbl,$hlp0), $DATA, $TMP3
    vpxor       $TMP3, $TMP0, $TMP0
    vpclmulqdq  \$0x11, ($Htbl,$hlp0), $DATA, $TMP3
    vpxor       $TMP3, $TMP1, $TMP1
    vpclmulqdq  \$0x01, ($Htbl,$hlp0), $DATA, $TMP3
    vpxor       $TMP3, $TMP2, $TMP2
    vpclmulqdq  \$0x10, ($Htbl,$hlp0), $DATA, $TMP3
    vpxor       $TMP3, $TMP2, $TMP2

    test $hlp0, $hlp0

    lea 16($inp), $inp

    jnz .Lhtable_polyval_prefix_loop

.Lhtable_polyval_prefix_complete:
    vpsrldq \$8, $TMP2, $TMP3
    vpslldq \$8, $TMP2, $TMP2

    vpxor $TMP3, $TMP1, $Xhi
    vpxor $TMP2, $TMP0, $T

    jmp .Lhtable_polyval_main_loop

.Lhtable_polyval_no_prefix:
    # At this point we know the number of blocks is a multiple of 8. However,
    # the reduction in the main loop includes a multiplication by x^(-128). In
    # order to counter this, the existing tag needs to be multipled by x^128.
    # In practice, this just means that it is loaded into $Xhi, not $T.
    vpxor $T, $T, $T
    vmovdqa ($Tp), $Xhi

.align 64
.Lhtable_polyval_main_loop:
    sub \$0x80, $len
    jb .Lhtable_polyval_out

    vmovdqu 16*7($inp), $DATA      # Ii

    vpclmulqdq \$0x01, ($Htbl), $DATA, $TMP2
    vpclmulqdq \$0x00, ($Htbl), $DATA, $TMP0
    vpclmulqdq \$0x11, ($Htbl), $DATA, $TMP1
    vpclmulqdq \$0x10, ($Htbl), $DATA, $TMP3
    vpxor $TMP3, $TMP2, $TMP2

    #########################################################
    vmovdqu 16*6($inp), $DATA
    ${\$SCHOOLBOOK_AAD->(1)}

    #########################################################
    vmovdqu 16*5($inp), $DATA

    vpclmulqdq \$0x10, poly(%rip), $T, $TMP4         # reduction stage 1a
    vpalignr \$8, $T, $T, $T

    ${\$SCHOOLBOOK_AAD->(2)}

    vpxor $TMP4, $T, $T                              # reduction stage 1b
    #########################################################
    vmovdqu     16*4($inp), $DATA

    ${\$SCHOOLBOOK_AAD->(3)}
    #########################################################
    vmovdqu     16*3($inp), $DATA

    vpclmulqdq \$0x10, poly(%rip), $T, $TMP4         # reduction stage 2a
    vpalignr \$8, $T, $T, $T

    ${\$SCHOOLBOOK_AAD->(4)}

    vpxor $TMP4, $T, $T                              # reduction stage 2b
    #########################################################
    vmovdqu 16*2($inp), $DATA

    ${\$SCHOOLBOOK_AAD->(5)}

    vpxor $Xhi, $T, $T                               # reduction finalize
    #########################################################
    vmovdqu 16*1($inp), $DATA

    ${\$SCHOOLBOOK_AAD->(6)}
    #########################################################
    vmovdqu 16*0($inp), $DATA
    vpxor $T, $DATA, $DATA

    ${\$SCHOOLBOOK_AAD->(7)}
    #########################################################
    vpsrldq \$8, $TMP2, $TMP3
    vpslldq \$8, $TMP2, $TMP2

    vpxor $TMP3, $TMP1, $Xhi
    vpxor $TMP2, $TMP0, $T

    lea 16*8($inp), $inp
    jmp .Lhtable_polyval_main_loop

    #########################################################

.Lhtable_polyval_out:
    vpclmulqdq  \$0x10, poly(%rip), $T, $TMP3
    vpalignr    \$8, $T, $T, $T
    vpxor       $TMP3, $T, $T

    vpclmulqdq  \$0x10, poly(%rip), $T, $TMP3
    vpalignr    \$8, $T, $T, $T
    vpxor       $TMP3, $T, $T
    vpxor       $Xhi, $T, $T

    vmovdqu $T, ($Tp)
    vzeroupper
    ret
.cfi_endproc
.size aesgcmsiv_htable_polyval,.-aesgcmsiv_htable_polyval
___
}
aesgcmsiv_htable_polyval();

sub aesgcmsiv_polyval_horner {
  #void aesgcmsiv_polyval_horner(unsigned char T[16],  // output
  #      const unsigned char* H, // H
  #      unsigned char* BUF,  // Buffer
  #      unsigned int blocks);  // Len2
  #
  # parameter 1: %rdi T - pointers to POLYVAL output
  # parameter 2: %rsi Hp - pointer to H (user key)
  # parameter 3: %rdx INp - pointer to input
  # parameter 4: %rcx L - total number of blocks in input BUFFER
  #
  my $T = "%rdi";
  my $Hp = "%rsi";
  my $INp = "%rdx";
  my $L = "%rcx";
  my $LOC = "%r10";
  my $LEN = "%eax";
  my $H = "%xmm1";
  my $RES = "%xmm0";

  $code.=<<___;
.globl aesgcmsiv_polyval_horner
.type aesgcmsiv_polyval_horner,\@function,4
.align 16
aesgcmsiv_polyval_horner:
.cfi_startproc
    _CET_ENDBR
    test $L, $L
    jnz .Lpolyval_horner_start
    ret

.Lpolyval_horner_start:
    # We will start with L GFMULS for POLYVAL(BIG_BUFFER)
    # RES = GFMUL(RES, H)

    xorq $LOC, $LOC
    shlq \$4, $L    # L contains number of bytes to process

    vmovdqa ($Hp), $H
    vmovdqa ($T), $RES

.Lpolyval_horner_loop:
    vpxor ($INp,$LOC), $RES, $RES  # RES = RES + Xi
    call GFMUL  # RES = RES * H

    add \$16, $LOC
    cmp $LOC, $L
    jne .Lpolyval_horner_loop

    # calculation of T is complete. RES=T
    vmovdqa $RES, ($T)
    ret
.cfi_endproc
.size aesgcmsiv_polyval_horner,.-aesgcmsiv_polyval_horner
___
}
aesgcmsiv_polyval_horner();

# void aes128gcmsiv_aes_ks(const uint8_t *key, uint8_t *out_expanded_key);
# parameter 1: %rdi
# parameter 2: %rsi
$code.=<<___;
.globl aes128gcmsiv_aes_ks
.type aes128gcmsiv_aes_ks,\@function,2
.align 16
aes128gcmsiv_aes_ks:
.cfi_startproc
    _CET_ENDBR
    vmovdqu (%rdi), %xmm1           # xmm1 = user key
    vmovdqa %xmm1, (%rsi)           # rsi points to output

    vmovdqa con1(%rip), %xmm0
    vmovdqa mask(%rip), %xmm15

    movq \$8, %rax

.Lks128_loop:
    addq \$16, %rsi                 # rsi points for next key
    subq \$1, %rax
    vpshufb %xmm15, %xmm1, %xmm2    # xmm2 = shuffled user key
    vaesenclast %xmm0, %xmm2, %xmm2
    vpslld \$1, %xmm0, %xmm0
    vpslldq \$4, %xmm1, %xmm3
    vpxor %xmm3, %xmm1, %xmm1
    vpslldq \$4, %xmm3, %xmm3
    vpxor %xmm3, %xmm1, %xmm1
    vpslldq \$4, %xmm3, %xmm3
    vpxor %xmm3, %xmm1, %xmm1
    vpxor %xmm2, %xmm1, %xmm1
    vmovdqa %xmm1, (%rsi)
    jne .Lks128_loop

    vmovdqa con2(%rip), %xmm0
    vpshufb %xmm15, %xmm1, %xmm2
    vaesenclast %xmm0, %xmm2, %xmm2
    vpslld \$1, %xmm0, %xmm0
    vpslldq \$4, %xmm1, %xmm3
    vpxor %xmm3, %xmm1, %xmm1
    vpslldq \$4, %xmm3, %xmm3
    vpxor %xmm3, %xmm1, %xmm1
    vpslldq \$4, %xmm3, %xmm3
    vpxor %xmm3, %xmm1, %xmm1
    vpxor %xmm2, %xmm1, %xmm1
    vmovdqa %xmm1, 16(%rsi)

    vpshufb %xmm15, %xmm1, %xmm2
    vaesenclast %xmm0, %xmm2, %xmm2
    vpslldq \$4, %xmm1, %xmm3
    vpxor %xmm3, %xmm1, %xmm1
    vpslldq \$4, %xmm3, %xmm3
    vpxor %xmm3, %xmm1, %xmm1
    vpslldq \$4, %xmm3, %xmm3
    vpxor %xmm3, %xmm1, %xmm1
    vpxor %xmm2, %xmm1, %xmm1
    vmovdqa %xmm1, 32(%rsi)
    ret
.cfi_endproc
.size aes128gcmsiv_aes_ks,.-aes128gcmsiv_aes_ks
___

# void aes256gcmsiv_aes_ks(const uint8_t *key, uint8_t *out_expanded_key);
# parameter 1: %rdi
# parameter 2: %rsi
$code.=<<___;
.globl aes256gcmsiv_aes_ks
.type aes256gcmsiv_aes_ks,\@function,2
.align 16
aes256gcmsiv_aes_ks:
.cfi_startproc
    _CET_ENDBR
    vmovdqu (%rdi), %xmm1
    vmovdqu 16(%rdi), %xmm3
    vmovdqa %xmm1, (%rsi)
    vmovdqa %xmm3, 16(%rsi)
    vmovdqa con1(%rip), %xmm0
    vmovdqa mask(%rip), %xmm15
    vpxor %xmm14, %xmm14, %xmm14
    mov \$6, %rax

.Lks256_loop:
    add \$32, %rsi
    subq \$1, %rax
    vpshufb %xmm15, %xmm3, %xmm2
    vaesenclast %xmm0, %xmm2, %xmm2
    vpslld \$1, %xmm0, %xmm0
    vpsllq \$32, %xmm1, %xmm4
    vpxor %xmm4, %xmm1, %xmm1
    vpshufb con3(%rip), %xmm1,  %xmm4
    vpxor %xmm4, %xmm1, %xmm1
    vpxor %xmm2, %xmm1, %xmm1
    vmovdqa %xmm1, (%rsi)
    vpshufd \$0xff, %xmm1, %xmm2
    vaesenclast %xmm14, %xmm2, %xmm2
    vpsllq \$32, %xmm3, %xmm4
    vpxor %xmm4, %xmm3, %xmm3
    vpshufb con3(%rip), %xmm3,  %xmm4
    vpxor %xmm4, %xmm3, %xmm3
    vpxor %xmm2, %xmm3, %xmm3
    vmovdqa %xmm3, 16(%rsi)
    jne .Lks256_loop

    vpshufb %xmm15, %xmm3, %xmm2
    vaesenclast %xmm0, %xmm2, %xmm2
    vpsllq \$32, %xmm1, %xmm4
    vpxor %xmm4, %xmm1, %xmm1
    vpshufb con3(%rip), %xmm1,  %xmm4
    vpxor %xmm4, %xmm1, %xmm1
    vpxor %xmm2, %xmm1, %xmm1
    vmovdqa %xmm1, 32(%rsi)
    ret
.cfi_endproc
___

sub aes128gcmsiv_aes_ks_enc_x1 {
  my $KS1_REGA = "%xmm1";
  my $KS1_REGB = "%xmm2";
  my $BLOCK1 = "%xmm4";
  my $AUXREG = "%xmm3";

  my $KS_BLOCK = sub {
    my ($reg, $reg2, $auxReg) = @_;
    return <<___;
    vpsllq \$32, $reg, $auxReg         #!!saving mov instruction to xmm3
    vpxor $auxReg, $reg, $reg
    vpshufb con3(%rip), $reg,  $auxReg
    vpxor $auxReg, $reg, $reg
    vpxor $reg2, $reg, $reg
___
  };

  my $round = sub {
    my ($i, $j) = @_;
    return <<___;
    vpshufb %xmm15, %xmm1, %xmm2      #!!saving mov instruction to xmm2
    vaesenclast %xmm0, %xmm2, %xmm2
    vpslld \$1, %xmm0, %xmm0
    ${\$KS_BLOCK->($KS1_REGA, $KS1_REGB, $AUXREG)}
    vaesenc %xmm1, $BLOCK1, $BLOCK1
    vmovdqa %xmm1, ${\eval(16*$i)}($j)
___
  };

  my $roundlast = sub {
    my ($i, $j) = @_;
    return <<___;
    vpshufb %xmm15, %xmm1, %xmm2      #!!saving mov instruction to xmm2
    vaesenclast %xmm0, %xmm2, %xmm2
    ${\$KS_BLOCK->($KS1_REGA, $KS1_REGB, $AUXREG)}
    vaesenclast %xmm1, $BLOCK1, $BLOCK1
    vmovdqa %xmm1, ${\eval(16*$i)}($j)
___
  };

# parameter 1: %rdi                         Pointer to PT
# parameter 2: %rsi                         Pointer to CT
# parameter 4: %rdx                         Pointer to keys
# parameter 5: %rcx                         Pointer to initial key
  $code.=<<___;
.globl aes128gcmsiv_aes_ks_enc_x1
.type aes128gcmsiv_aes_ks_enc_x1,\@function,4
.align 16
aes128gcmsiv_aes_ks_enc_x1:
.cfi_startproc
    _CET_ENDBR
    vmovdqa (%rcx), %xmm1                 # xmm1 = first 16 bytes of random key
    vmovdqa 0*16(%rdi), $BLOCK1

    vmovdqa %xmm1, (%rdx)                 # KEY[0] = first 16 bytes of random key
    vpxor %xmm1, $BLOCK1, $BLOCK1

    vmovdqa con1(%rip), %xmm0             # xmm0  = 1,1,1,1
    vmovdqa mask(%rip), %xmm15            # xmm15 = mask

    ${\$round->(1, "%rdx")}
    ${\$round->(2, "%rdx")}
    ${\$round->(3, "%rdx")}
    ${\$round->(4, "%rdx")}
    ${\$round->(5, "%rdx")}
    ${\$round->(6, "%rdx")}
    ${\$round->(7, "%rdx")}
    ${\$round->(8, "%rdx")}

    vmovdqa con2(%rip), %xmm0

    ${\$round->(9, "%rdx")}
    ${\$roundlast->(10, "%rdx")}

    vmovdqa $BLOCK1, 0*16(%rsi)
    ret
.cfi_endproc
.size aes128gcmsiv_aes_ks_enc_x1,.-aes128gcmsiv_aes_ks_enc_x1
___
}
aes128gcmsiv_aes_ks_enc_x1();

sub aes128gcmsiv_kdf {
  my $BLOCK1 = "%xmm9";
  my $BLOCK2 = "%xmm10";
  my $BLOCK3 = "%xmm11";
  my $BLOCK4 = "%xmm12";
  my $BLOCK5 = "%xmm13";
  my $BLOCK6 = "%xmm14";
  my $ONE = "%xmm13";
  my $KSp = "%rdx";
  my $STATE_1 = "%xmm1";

  my $enc_roundx4 = sub {
    my ($i, $j) = @_;
    return <<___;
    vmovdqa ${\eval($i*16)}(%rdx), $j
    vaesenc $j, $BLOCK1, $BLOCK1
    vaesenc $j, $BLOCK2, $BLOCK2
    vaesenc $j, $BLOCK3, $BLOCK3
    vaesenc $j, $BLOCK4, $BLOCK4
___
  };

  my $enc_roundlastx4 = sub {
    my ($i, $j) = @_;
    return <<___;
    vmovdqa ${\eval($i*16)}(%rdx), $j
    vaesenclast $j, $BLOCK1, $BLOCK1
    vaesenclast $j, $BLOCK2, $BLOCK2
    vaesenclast $j, $BLOCK3, $BLOCK3
    vaesenclast $j, $BLOCK4, $BLOCK4
___
  };

# void aes128gcmsiv_kdf(const uint8_t nonce[16],
#                       uint8_t *out_key_material,
#                       const uint8_t *key_schedule);
  $code.=<<___;
.globl aes128gcmsiv_kdf
.type aes128gcmsiv_kdf,\@function,3
.align 16
aes128gcmsiv_kdf:
.cfi_startproc
    _CET_ENDBR
# parameter 1: %rdi                         Pointer to NONCE
# parameter 2: %rsi                         Pointer to CT
# parameter 4: %rdx                         Pointer to keys

    vmovdqa (%rdx), %xmm1                  # xmm1 = first 16 bytes of random key
    vmovdqa 0*16(%rdi), $BLOCK1
    vmovdqa and_mask(%rip), $BLOCK4
    vmovdqa one(%rip), $ONE
    vpshufd \$0x90, $BLOCK1, $BLOCK1
    vpand $BLOCK4, $BLOCK1, $BLOCK1
    vpaddd $ONE, $BLOCK1, $BLOCK2
    vpaddd $ONE, $BLOCK2, $BLOCK3
    vpaddd $ONE, $BLOCK3, $BLOCK4

    vpxor %xmm1, $BLOCK1, $BLOCK1
    vpxor %xmm1, $BLOCK2, $BLOCK2
    vpxor %xmm1, $BLOCK3, $BLOCK3
    vpxor %xmm1, $BLOCK4, $BLOCK4

    ${\$enc_roundx4->(1, "%xmm1")}
    ${\$enc_roundx4->(2, "%xmm2")}
    ${\$enc_roundx4->(3, "%xmm1")}
    ${\$enc_roundx4->(4, "%xmm2")}
    ${\$enc_roundx4->(5, "%xmm1")}
    ${\$enc_roundx4->(6, "%xmm2")}
    ${\$enc_roundx4->(7, "%xmm1")}
    ${\$enc_roundx4->(8, "%xmm2")}
    ${\$enc_roundx4->(9, "%xmm1")}
    ${\$enc_roundlastx4->(10, "%xmm2")}

    vmovdqa $BLOCK1, 0*16(%rsi)
    vmovdqa $BLOCK2, 1*16(%rsi)
    vmovdqa $BLOCK3, 2*16(%rsi)
    vmovdqa $BLOCK4, 3*16(%rsi)
    ret
.cfi_endproc
.size aes128gcmsiv_kdf,.-aes128gcmsiv_kdf
___
}
aes128gcmsiv_kdf();

sub aes128gcmsiv_enc_msg_x4 {
  my $CTR1 = "%xmm0";
  my $CTR2 = "%xmm1";
  my $CTR3 = "%xmm2";
  my $CTR4 = "%xmm3";
  my $ADDER = "%xmm4";

  my $STATE1 = "%xmm5";
  my $STATE2 = "%xmm6";
  my $STATE3 = "%xmm7";
  my $STATE4 = "%xmm8";

  my $TMP = "%xmm12";
  my $TMP2 = "%xmm13";
  my $TMP3 = "%xmm14";
  my $IV = "%xmm15";

  my $PT = "%rdi";
  my $CT = "%rsi";
  my $TAG = "%rdx";
  my $KS = "%rcx";
  my $LEN = "%r8";

  my $aes_round = sub {
    my ($i) = @_;
    return <<___;
    vmovdqu ${\eval($i*16)}($KS), $TMP
    vaesenc $TMP, $STATE1, $STATE1
    vaesenc $TMP, $STATE2, $STATE2
    vaesenc $TMP, $STATE3, $STATE3
    vaesenc $TMP, $STATE4, $STATE4
___
  };

  my $aes_lastround = sub {
    my ($i) = @_;
    return <<___;
    vmovdqu ${\eval($i*16)}($KS), $TMP
    vaesenclast $TMP, $STATE1, $STATE1
    vaesenclast $TMP, $STATE2, $STATE2
    vaesenclast $TMP, $STATE3, $STATE3
    vaesenclast $TMP, $STATE4, $STATE4
___
  };

# void aes128gcmsiv_enc_msg_x4(unsigned char* PT, unsigned char* CT,
#                              unsigned char* TAG, unsigned char* KS,
#                              size_t byte_len);
# parameter 1: %rdi     #PT
# parameter 2: %rsi     #CT
# parameter 3: %rdx     #TAG  [127 126 ... 0]  IV=[127...32]
# parameter 4: %rcx     #KS
# parameter 5: %r8      #LEN MSG_length in bytes
  $code.=<<___;
.globl aes128gcmsiv_enc_msg_x4
.type aes128gcmsiv_enc_msg_x4,\@function,5
.align 16
aes128gcmsiv_enc_msg_x4:
.cfi_startproc
    _CET_ENDBR
    test $LEN, $LEN
    jnz .L128_enc_msg_x4_start
    ret

.L128_enc_msg_x4_start:
    pushq %r12
.cfi_push %r12
    pushq %r13
.cfi_push %r13

    shrq \$4, $LEN      # LEN = num of blocks
    movq $LEN, %r10
    shlq \$62, %r10
    shrq \$62, %r10

    # make IV from TAG
    vmovdqa ($TAG), $IV
    vpor OR_MASK(%rip), $IV, $IV  #IV = [1]TAG[126...32][00..00]

    vmovdqu four(%rip), $ADDER     # Register to increment counters
    vmovdqa $IV, $CTR1             # CTR1 = TAG[1][127...32][00..00]
    vpaddd one(%rip), $IV, $CTR2   # CTR2 = TAG[1][127...32][00..01]
    vpaddd two(%rip), $IV, $CTR3   # CTR3 = TAG[1][127...32][00..02]
    vpaddd three(%rip), $IV, $CTR4 # CTR4 = TAG[1][127...32][00..03]

    shrq \$2, $LEN
    je .L128_enc_msg_x4_check_remainder

    subq \$64, $CT
    subq \$64, $PT

.L128_enc_msg_x4_loop1:
    addq \$64, $CT
    addq \$64, $PT

    vmovdqa $CTR1, $STATE1
    vmovdqa $CTR2, $STATE2
    vmovdqa $CTR3, $STATE3
    vmovdqa $CTR4, $STATE4

    vpxor ($KS), $STATE1, $STATE1
    vpxor ($KS), $STATE2, $STATE2
    vpxor ($KS), $STATE3, $STATE3
    vpxor ($KS), $STATE4, $STATE4

    ${\$aes_round->(1)}
    vpaddd $ADDER, $CTR1, $CTR1
    ${\$aes_round->(2)}
    vpaddd $ADDER, $CTR2, $CTR2
    ${\$aes_round->(3)}
    vpaddd $ADDER, $CTR3, $CTR3
    ${\$aes_round->(4)}
    vpaddd $ADDER, $CTR4, $CTR4

    ${\$aes_round->(5)}
    ${\$aes_round->(6)}
    ${\$aes_round->(7)}
    ${\$aes_round->(8)}
    ${\$aes_round->(9)}
    ${\$aes_lastround->(10)}

    # XOR with Plaintext
    vpxor 0*16($PT), $STATE1, $STATE1
    vpxor 1*16($PT), $STATE2, $STATE2
    vpxor 2*16($PT), $STATE3, $STATE3
    vpxor 3*16($PT), $STATE4, $STATE4

    subq \$1, $LEN

    vmovdqu $STATE1, 0*16($CT)
    vmovdqu $STATE2, 1*16($CT)
    vmovdqu $STATE3, 2*16($CT)
    vmovdqu $STATE4, 3*16($CT)

    jne .L128_enc_msg_x4_loop1

    addq \$64,$CT
    addq \$64,$PT

.L128_enc_msg_x4_check_remainder:
    cmpq \$0, %r10
    je .L128_enc_msg_x4_out

.L128_enc_msg_x4_loop2:
    # enc each block separately
    # CTR1 is the highest counter (even if no LOOP done)
    vmovdqa $CTR1, $STATE1
    vpaddd one(%rip), $CTR1, $CTR1  # inc counter

    vpxor ($KS), $STATE1, $STATE1
    vaesenc 16($KS), $STATE1, $STATE1
    vaesenc 32($KS), $STATE1, $STATE1
    vaesenc 48($KS), $STATE1, $STATE1
    vaesenc 64($KS), $STATE1, $STATE1
    vaesenc 80($KS), $STATE1, $STATE1
    vaesenc 96($KS), $STATE1, $STATE1
    vaesenc 112($KS), $STATE1, $STATE1
    vaesenc 128($KS), $STATE1, $STATE1
    vaesenc 144($KS), $STATE1, $STATE1
    vaesenclast 160($KS), $STATE1, $STATE1

    # XOR with plaintext
    vpxor ($PT), $STATE1, $STATE1
    vmovdqu $STATE1, ($CT)

    addq \$16, $PT
    addq \$16, $CT

    subq \$1, %r10
    jne .L128_enc_msg_x4_loop2

.L128_enc_msg_x4_out:
    popq %r13
.cfi_pop %r13
    popq %r12
.cfi_pop %r12
    ret
.cfi_endproc
.size aes128gcmsiv_enc_msg_x4,.-aes128gcmsiv_enc_msg_x4
___
}
aes128gcmsiv_enc_msg_x4();

sub aes128gcmsiv_enc_msg_x8 {
  my $STATE1 = "%xmm1";
  my $STATE2 = "%xmm2";
  my $STATE3 = "%xmm3";
  my $STATE4 = "%xmm4";
  my $STATE5 = "%xmm5";
  my $STATE6 = "%xmm6";
  my $STATE7 = "%xmm7";
  my $STATE8 = "%xmm8";

  my $CTR1 = "%xmm0";
  my $CTR2 = "%xmm9";
  my $CTR3 = "%xmm10";
  my $CTR4 = "%xmm11";
  my $CTR5 = "%xmm12";
  my $CTR6 = "%xmm13";
  my $CTR7 = "%xmm14";
  my $SCHED = "%xmm15";

  my $TMP1 = "%xmm1";
  my $TMP2 = "%xmm2";

  my $PT = "%rdi";
  my $CT = "%rsi";
  my $TAG = "%rdx";
  my $KS = "%rcx";
  my $LEN = "%r8";

  my $aes_round8 = sub {
    my ($i) = @_;
    return <<___;
    vmovdqu ${\eval($i*16)}($KS), $SCHED
    vaesenc $SCHED, $STATE1, $STATE1
    vaesenc $SCHED, $STATE2, $STATE2
    vaesenc $SCHED, $STATE3, $STATE3
    vaesenc $SCHED, $STATE4, $STATE4
    vaesenc $SCHED, $STATE5, $STATE5
    vaesenc $SCHED, $STATE6, $STATE6
    vaesenc $SCHED, $STATE7, $STATE7
    vaesenc $SCHED, $STATE8, $STATE8
___
  };

  my $aes_lastround8 = sub {
    my ($i) = @_;
    return <<___;
    vmovdqu ${\eval($i*16)}($KS), $SCHED
    vaesenclast $SCHED, $STATE1, $STATE1
    vaesenclast $SCHED, $STATE2, $STATE2
    vaesenclast $SCHED, $STATE3, $STATE3
    vaesenclast $SCHED, $STATE4, $STATE4
    vaesenclast $SCHED, $STATE5, $STATE5
    vaesenclast $SCHED, $STATE6, $STATE6
    vaesenclast $SCHED, $STATE7, $STATE7
    vaesenclast $SCHED, $STATE8, $STATE8
___
  };

# void ENC_MSG_x8(unsigned char* PT,
#                 unsigned char* CT,
#                 unsigned char* TAG,
#                 unsigned char* KS,
#                 size_t byte_len);
# parameter 1: %rdi     #PT
# parameter 2: %rsi     #CT
# parameter 3: %rdx     #TAG        [127 126 ... 0]  IV=[127...32]
# parameter 4: %rcx     #KS
# parameter 5: %r8      #LEN MSG_length in bytes
  $code.=<<___;
.globl aes128gcmsiv_enc_msg_x8
.type aes128gcmsiv_enc_msg_x8,\@function,5
.align 16
aes128gcmsiv_enc_msg_x8:
.cfi_startproc
    _CET_ENDBR
    test $LEN, $LEN
    jnz .L128_enc_msg_x8_start
    ret

.L128_enc_msg_x8_start:
    pushq %r12
.cfi_push %r12
    pushq %r13
.cfi_push %r13
    pushq %rbp
.cfi_push %rbp
    movq %rsp, %rbp
.cfi_def_cfa_register rbp

    # Place in stack
    subq \$128, %rsp
    andq \$-64, %rsp

    shrq \$4, $LEN  # LEN = num of blocks
    movq $LEN, %r10
    shlq \$61, %r10
    shrq \$61, %r10

    # make IV from TAG
    vmovdqu ($TAG), $TMP1
    vpor OR_MASK(%rip), $TMP1, $TMP1  # TMP1= IV = [1]TAG[126...32][00..00]

    # store counter8 in the stack
    vpaddd seven(%rip), $TMP1, $CTR1
    vmovdqu $CTR1, (%rsp)             # CTR8 = TAG[127...32][00..07]
    vpaddd one(%rip), $TMP1, $CTR2    # CTR2 = TAG[127...32][00..01]
    vpaddd two(%rip), $TMP1, $CTR3    # CTR3 = TAG[127...32][00..02]
    vpaddd three(%rip), $TMP1, $CTR4  # CTR4 = TAG[127...32][00..03]
    vpaddd four(%rip), $TMP1, $CTR5   # CTR5 = TAG[127...32][00..04]
    vpaddd five(%rip), $TMP1, $CTR6   # CTR6 = TAG[127...32][00..05]
    vpaddd six(%rip), $TMP1, $CTR7    # CTR7 = TAG[127...32][00..06]
    vmovdqa $TMP1, $CTR1              # CTR1 = TAG[127...32][00..00]

    shrq \$3, $LEN
    je .L128_enc_msg_x8_check_remainder

    subq \$128, $CT
    subq \$128, $PT

.L128_enc_msg_x8_loop1:
    addq \$128, $CT
    addq \$128, $PT

    vmovdqa $CTR1, $STATE1
    vmovdqa $CTR2, $STATE2
    vmovdqa $CTR3, $STATE3
    vmovdqa $CTR4, $STATE4
    vmovdqa $CTR5, $STATE5
    vmovdqa $CTR6, $STATE6
    vmovdqa $CTR7, $STATE7
    # move from stack
    vmovdqu (%rsp), $STATE8

    vpxor ($KS), $STATE1, $STATE1
    vpxor ($KS), $STATE2, $STATE2
    vpxor ($KS), $STATE3, $STATE3
    vpxor ($KS), $STATE4, $STATE4
    vpxor ($KS), $STATE5, $STATE5
    vpxor ($KS), $STATE6, $STATE6
    vpxor ($KS), $STATE7, $STATE7
    vpxor ($KS), $STATE8, $STATE8

    ${\$aes_round8->(1)}
    vmovdqu (%rsp), $CTR7  # deal with CTR8
    vpaddd eight(%rip), $CTR7, $CTR7
    vmovdqu $CTR7, (%rsp)
    ${\$aes_round8->(2)}
    vpsubd one(%rip), $CTR7, $CTR7
    ${\$aes_round8->(3)}
    vpaddd eight(%rip), $CTR1, $CTR1
    ${\$aes_round8->(4)}
    vpaddd eight(%rip), $CTR2, $CTR2
    ${\$aes_round8->(5)}
    vpaddd eight(%rip), $CTR3, $CTR3
    ${\$aes_round8->(6)}
    vpaddd eight(%rip), $CTR4, $CTR4
    ${\$aes_round8->(7)}
    vpaddd eight(%rip), $CTR5, $CTR5
    ${\$aes_round8->(8)}
    vpaddd eight(%rip), $CTR6, $CTR6
    ${\$aes_round8->(9)}
    ${\$aes_lastround8->(10)}

    # XOR with Plaintext
    vpxor 0*16($PT), $STATE1, $STATE1
    vpxor 1*16($PT), $STATE2, $STATE2
    vpxor 2*16($PT), $STATE3, $STATE3
    vpxor 3*16($PT), $STATE4, $STATE4
    vpxor 4*16($PT), $STATE5, $STATE5
    vpxor 5*16($PT), $STATE6, $STATE6
    vpxor 6*16($PT), $STATE7, $STATE7
    vpxor 7*16($PT), $STATE8, $STATE8

    dec $LEN

    vmovdqu $STATE1, 0*16($CT)
    vmovdqu $STATE2, 1*16($CT)
    vmovdqu $STATE3, 2*16($CT)
    vmovdqu $STATE4, 3*16($CT)
    vmovdqu $STATE5, 4*16($CT)
    vmovdqu $STATE6, 5*16($CT)
    vmovdqu $STATE7, 6*16($CT)
    vmovdqu $STATE8, 7*16($CT)

    jne .L128_enc_msg_x8_loop1

    addq \$128, $CT
    addq \$128, $PT

.L128_enc_msg_x8_check_remainder:
    cmpq \$0, %r10
    je .L128_enc_msg_x8_out

.L128_enc_msg_x8_loop2:
    # enc each block separately
    # CTR1 is the highest counter (even if no LOOP done)
    vmovdqa $CTR1, $STATE1
    vpaddd one(%rip), $CTR1, $CTR1  # inc counter

    vpxor ($KS), $STATE1, $STATE1
    vaesenc 16($KS), $STATE1, $STATE1
    vaesenc 32($KS), $STATE1, $STATE1
    vaesenc 48($KS), $STATE1, $STATE1
    vaesenc 64($KS), $STATE1, $STATE1
    vaesenc 80($KS), $STATE1, $STATE1
    vaesenc 96($KS), $STATE1, $STATE1
    vaesenc 112($KS), $STATE1, $STATE1
    vaesenc 128($KS), $STATE1, $STATE1
    vaesenc 144($KS), $STATE1, $STATE1
    vaesenclast 160($KS), $STATE1, $STATE1

    # XOR with Plaintext
    vpxor ($PT), $STATE1, $STATE1

    vmovdqu $STATE1, ($CT)

    addq \$16, $PT
    addq \$16, $CT

    decq %r10
    jne .L128_enc_msg_x8_loop2

.L128_enc_msg_x8_out:
    movq %rbp, %rsp
.cfi_def_cfa_register %rsp
    popq %rbp
.cfi_pop %rbp
    popq %r13
.cfi_pop %r13
    popq %r12
.cfi_pop %r12
    ret
.cfi_endproc
.size aes128gcmsiv_enc_msg_x8,.-aes128gcmsiv_enc_msg_x8
___
}
aes128gcmsiv_enc_msg_x8();

sub aesgcmsiv_dec {
  my ($aes256) = @_;

  my $T = "%xmm0";
  my $TMP0 = "%xmm1";
  my $TMP1 = "%xmm2";
  my $TMP2 = "%xmm3";
  my $TMP3 = "%xmm4";
  my $TMP4 = "%xmm5";
  my $TMP5 = "%xmm6";
  my $CTR1 = "%xmm7";
  my $CTR2 = "%xmm8";
  my $CTR3 = "%xmm9";
  my $CTR4 = "%xmm10";
  my $CTR5 = "%xmm11";
  my $CTR6 = "%xmm12";
  my $CTR = "%xmm15";
  my $CT = "%rdi";
  my $PT = "%rsi";
  my $POL = "%rdx";
  my $Htbl = "%rcx";
  my $KS = "%r8";
  my $LEN = "%r9";
  my $secureBuffer = "%rax";
  my $HTABLE_ROUNDS = "%xmm13";

  my $labelPrefix = "128";
  if ($aes256) {
    $labelPrefix = "256";
  }

  my $aes_round_dec = sub {
    my ($i) = @_;
    return <<___;
    vmovdqu ${\eval($i*16)}($KS), $TMP3
    vaesenc $TMP3, $CTR1, $CTR1
    vaesenc $TMP3, $CTR2, $CTR2
    vaesenc $TMP3, $CTR3, $CTR3
    vaesenc $TMP3, $CTR4, $CTR4
    vaesenc $TMP3, $CTR5, $CTR5
    vaesenc $TMP3, $CTR6, $CTR6
___
  };

  my $aes_lastround_dec = sub {
    my ($i) = @_;
    return <<___;
    vmovdqu ${\eval($i*16)}($KS), $TMP3
    vaesenclast $TMP3, $CTR1, $CTR1
    vaesenclast $TMP3, $CTR2, $CTR2
    vaesenclast $TMP3, $CTR3, $CTR3
    vaesenclast $TMP3, $CTR4, $CTR4
    vaesenclast $TMP3, $CTR5, $CTR5
    vaesenclast $TMP3, $CTR6, $CTR6
___
  };

  my $schoolbook = sub {
    my ($i) = @_;
    return <<___;
    vmovdqu ${\eval($i*16-32)}($secureBuffer), $TMP5
    vmovdqu ${\eval($i*16-32)}($Htbl), $HTABLE_ROUNDS

    vpclmulqdq \$0x10, $HTABLE_ROUNDS, $TMP5, $TMP3
    vpxor $TMP3, $TMP0, $TMP0
    vpclmulqdq \$0x11, $HTABLE_ROUNDS, $TMP5, $TMP3
    vpxor $TMP3, $TMP1, $TMP1
    vpclmulqdq \$0x00, $HTABLE_ROUNDS, $TMP5, $TMP3
    vpxor $TMP3, $TMP2, $TMP2
    vpclmulqdq \$0x01, $HTABLE_ROUNDS, $TMP5, $TMP3
    vpxor $TMP3, $TMP0, $TMP0
___
  };

  if ($aes256) {
    $code.=<<___;
.globl aes256gcmsiv_dec
.type aes256gcmsiv_dec,\@function,6
.align 16
aes256gcmsiv_dec:
___
  } else {
    $code.=<<___;
.globl aes128gcmsiv_dec
.type aes128gcmsiv_dec,\@function,6
.align 16
aes128gcmsiv_dec:
___
  }

  $code.=<<___;
.cfi_startproc
    _CET_ENDBR
    test \$~15, $LEN
    jnz .L${labelPrefix}_dec_start
    ret

.L${labelPrefix}_dec_start:
    vzeroupper
    vmovdqa ($POL), $T
    # The claimed tag is provided after the current calculated tag value.
    # CTRBLKs is made from it.
    vmovdqu 16($POL), $CTR
    vpor OR_MASK(%rip), $CTR, $CTR      # CTR = [1]TAG[126...32][00..00]
    movq $POL, $secureBuffer

    leaq 32($secureBuffer), $secureBuffer
    leaq 32($Htbl), $Htbl

    andq \$~15, $LEN

    # If less then 6 blocks, make singles
    cmp \$96, $LEN
    jb .L${labelPrefix}_dec_loop2

    # Decrypt the first six blocks
    sub \$96, $LEN
    vmovdqa $CTR, $CTR1
    vpaddd one(%rip), $CTR1, $CTR2
    vpaddd two(%rip), $CTR1, $CTR3
    vpaddd one(%rip), $CTR3, $CTR4
    vpaddd two(%rip), $CTR3, $CTR5
    vpaddd one(%rip), $CTR5, $CTR6
    vpaddd two(%rip), $CTR5, $CTR

    vpxor ($KS), $CTR1, $CTR1
    vpxor ($KS), $CTR2, $CTR2
    vpxor ($KS), $CTR3, $CTR3
    vpxor ($KS), $CTR4, $CTR4
    vpxor ($KS), $CTR5, $CTR5
    vpxor ($KS), $CTR6, $CTR6

    ${\$aes_round_dec->(1)}
    ${\$aes_round_dec->(2)}
    ${\$aes_round_dec->(3)}
    ${\$aes_round_dec->(4)}
    ${\$aes_round_dec->(5)}
    ${\$aes_round_dec->(6)}
    ${\$aes_round_dec->(7)}
    ${\$aes_round_dec->(8)}
    ${\$aes_round_dec->(9)}
___

if ($aes256) {
$code.=<<___;
    ${\$aes_round_dec->(10)}
    ${\$aes_round_dec->(11)}
    ${\$aes_round_dec->(12)}
    ${\$aes_round_dec->(13)}
    ${\$aes_lastround_dec->(14)}
___
} else {
$code.=<<___;
    ${\$aes_lastround_dec->(10)}
___
}

$code.=<<___;
    # XOR with CT
    vpxor 0*16($CT), $CTR1, $CTR1
    vpxor 1*16($CT), $CTR2, $CTR2
    vpxor 2*16($CT), $CTR3, $CTR3
    vpxor 3*16($CT), $CTR4, $CTR4
    vpxor 4*16($CT), $CTR5, $CTR5
    vpxor 5*16($CT), $CTR6, $CTR6

    vmovdqu $CTR1, 0*16($PT)
    vmovdqu $CTR2, 1*16($PT)
    vmovdqu $CTR3, 2*16($PT)
    vmovdqu $CTR4, 3*16($PT)
    vmovdqu $CTR5, 4*16($PT)
    vmovdqu $CTR6, 5*16($PT)

    addq \$96, $CT
    addq \$96, $PT
    jmp .L${labelPrefix}_dec_loop1

# Decrypt 6 blocks each time while hashing previous 6 blocks
.align 64
.L${labelPrefix}_dec_loop1:
    cmp \$96, $LEN
    jb .L${labelPrefix}_dec_finish_96
    sub \$96, $LEN

    vmovdqa $CTR6, $TMP5
    vmovdqa $CTR5, 1*16-32($secureBuffer)
    vmovdqa $CTR4, 2*16-32($secureBuffer)
    vmovdqa $CTR3, 3*16-32($secureBuffer)
    vmovdqa $CTR2, 4*16-32($secureBuffer)
    vmovdqa $CTR1, 5*16-32($secureBuffer)

    vmovdqa $CTR, $CTR1
    vpaddd one(%rip), $CTR1, $CTR2
    vpaddd two(%rip), $CTR1, $CTR3
    vpaddd one(%rip), $CTR3, $CTR4
    vpaddd two(%rip), $CTR3, $CTR5
    vpaddd one(%rip), $CTR5, $CTR6
    vpaddd two(%rip), $CTR5, $CTR

    vmovdqa ($KS), $TMP3
    vpxor $TMP3, $CTR1, $CTR1
    vpxor $TMP3, $CTR2, $CTR2
    vpxor $TMP3, $CTR3, $CTR3
    vpxor $TMP3, $CTR4, $CTR4
    vpxor $TMP3, $CTR5, $CTR5
    vpxor $TMP3, $CTR6, $CTR6

    vmovdqu 0*16-32($Htbl), $TMP3
    vpclmulqdq \$0x11, $TMP3, $TMP5, $TMP1
    vpclmulqdq \$0x00, $TMP3, $TMP5, $TMP2
    vpclmulqdq \$0x01, $TMP3, $TMP5, $TMP0
    vpclmulqdq \$0x10, $TMP3, $TMP5, $TMP3
    vpxor $TMP3, $TMP0, $TMP0

    ${\$aes_round_dec->(1)}
    ${\$schoolbook->(1)}

    ${\$aes_round_dec->(2)}
    ${\$schoolbook->(2)}

    ${\$aes_round_dec->(3)}
    ${\$schoolbook->(3)}

    ${\$aes_round_dec->(4)}
    ${\$schoolbook->(4)}

    ${\$aes_round_dec->(5)}
    ${\$aes_round_dec->(6)}
    ${\$aes_round_dec->(7)}

    vmovdqa 5*16-32($secureBuffer), $TMP5
    vpxor $T, $TMP5, $TMP5
    vmovdqu 5*16-32($Htbl), $TMP4

    vpclmulqdq \$0x01, $TMP4, $TMP5, $TMP3
    vpxor $TMP3, $TMP0, $TMP0
    vpclmulqdq \$0x11, $TMP4, $TMP5, $TMP3
    vpxor $TMP3, $TMP1, $TMP1
    vpclmulqdq \$0x00, $TMP4, $TMP5, $TMP3
    vpxor $TMP3, $TMP2, $TMP2
    vpclmulqdq \$0x10, $TMP4, $TMP5, $TMP3
    vpxor $TMP3, $TMP0, $TMP0

    ${\$aes_round_dec->(8)}

    vpsrldq \$8, $TMP0, $TMP3
    vpxor $TMP3, $TMP1, $TMP4
    vpslldq \$8, $TMP0, $TMP3
    vpxor $TMP3, $TMP2, $T

    vmovdqa poly(%rip), $TMP2

    ${\$aes_round_dec->(9)}
___

if ($aes256) {
$code.=<<___;
    ${\$aes_round_dec->(10)}
    ${\$aes_round_dec->(11)}
    ${\$aes_round_dec->(12)}
    ${\$aes_round_dec->(13)}
    vmovdqu 14*16($KS), $TMP5
___
} else {
$code.=<<___;
    vmovdqu 10*16($KS), $TMP5
___
}

$code.=<<___;
    vpalignr \$8, $T, $T, $TMP1
    vpclmulqdq \$0x10, $TMP2, $T, $T
    vpxor $T, $TMP1, $T

    vpxor 0*16($CT), $TMP5, $TMP3
    vaesenclast $TMP3, $CTR1, $CTR1
    vpxor 1*16($CT), $TMP5, $TMP3
    vaesenclast $TMP3, $CTR2, $CTR2
    vpxor 2*16($CT), $TMP5, $TMP3
    vaesenclast $TMP3, $CTR3, $CTR3
    vpxor 3*16($CT), $TMP5, $TMP3
    vaesenclast $TMP3, $CTR4, $CTR4
    vpxor 4*16($CT), $TMP5, $TMP3
    vaesenclast $TMP3, $CTR5, $CTR5
    vpxor 5*16($CT), $TMP5, $TMP3
    vaesenclast $TMP3, $CTR6, $CTR6

    vpalignr \$8, $T, $T, $TMP1
    vpclmulqdq \$0x10, $TMP2, $T, $T
    vpxor $T, $TMP1, $T

    vmovdqu $CTR1, 0*16($PT)
    vmovdqu $CTR2, 1*16($PT)
    vmovdqu $CTR3, 2*16($PT)
    vmovdqu $CTR4, 3*16($PT)
    vmovdqu $CTR5, 4*16($PT)
    vmovdqu $CTR6, 5*16($PT)

    vpxor $TMP4, $T, $T

    lea 96($CT), $CT
    lea 96($PT), $PT
    jmp .L${labelPrefix}_dec_loop1

.L${labelPrefix}_dec_finish_96:
    vmovdqa $CTR6, $TMP5
    vmovdqa $CTR5, 1*16-32($secureBuffer)
    vmovdqa $CTR4, 2*16-32($secureBuffer)
    vmovdqa $CTR3, 3*16-32($secureBuffer)
    vmovdqa $CTR2, 4*16-32($secureBuffer)
    vmovdqa $CTR1, 5*16-32($secureBuffer)

    vmovdqu 0*16-32($Htbl), $TMP3
    vpclmulqdq \$0x10, $TMP3, $TMP5, $TMP0
    vpclmulqdq \$0x11, $TMP3, $TMP5, $TMP1
    vpclmulqdq \$0x00, $TMP3, $TMP5, $TMP2
    vpclmulqdq \$0x01, $TMP3, $TMP5, $TMP3
    vpxor $TMP3, $TMP0, $TMP0

    ${\$schoolbook->(1)}
    ${\$schoolbook->(2)}
    ${\$schoolbook->(3)}
    ${\$schoolbook->(4)}

    vmovdqu 5*16-32($secureBuffer), $TMP5
    vpxor $T, $TMP5, $TMP5
    vmovdqu 5*16-32($Htbl), $TMP4
    vpclmulqdq \$0x11, $TMP4, $TMP5, $TMP3
    vpxor $TMP3, $TMP1, $TMP1
    vpclmulqdq \$0x00, $TMP4, $TMP5, $TMP3
    vpxor $TMP3, $TMP2, $TMP2
    vpclmulqdq \$0x10, $TMP4, $TMP5, $TMP3
    vpxor $TMP3, $TMP0, $TMP0
    vpclmulqdq \$0x01, $TMP4, $TMP5, $TMP3
    vpxor $TMP3, $TMP0, $TMP0

    vpsrldq \$8, $TMP0, $TMP3
    vpxor $TMP3, $TMP1, $TMP4
    vpslldq \$8, $TMP0, $TMP3
    vpxor $TMP3, $TMP2, $T

    vmovdqa poly(%rip), $TMP2

    vpalignr \$8, $T, $T, $TMP1
    vpclmulqdq \$0x10, $TMP2, $T, $T
    vpxor $T, $TMP1, $T

    vpalignr \$8, $T, $T, $TMP1
    vpclmulqdq \$0x10, $TMP2, $T, $T
    vpxor $T, $TMP1, $T

    vpxor $TMP4, $T, $T

.L${labelPrefix}_dec_loop2:
    # Here we encrypt any remaining whole block

    # if there are no whole blocks
    cmp \$16, $LEN
    jb .L${labelPrefix}_dec_out
    sub \$16, $LEN

    vmovdqa $CTR, $TMP1
    vpaddd one(%rip), $CTR, $CTR

    vpxor 0*16($KS), $TMP1, $TMP1
    vaesenc 1*16($KS), $TMP1, $TMP1
    vaesenc 2*16($KS), $TMP1, $TMP1
    vaesenc 3*16($KS), $TMP1, $TMP1
    vaesenc 4*16($KS), $TMP1, $TMP1
    vaesenc 5*16($KS), $TMP1, $TMP1
    vaesenc 6*16($KS), $TMP1, $TMP1
    vaesenc 7*16($KS), $TMP1, $TMP1
    vaesenc 8*16($KS), $TMP1, $TMP1
    vaesenc 9*16($KS), $TMP1, $TMP1
___
if ($aes256) {
$code.=<<___;
    vaesenc 10*16($KS), $TMP1, $TMP1
    vaesenc 11*16($KS), $TMP1, $TMP1
    vaesenc 12*16($KS), $TMP1, $TMP1
    vaesenc 13*16($KS), $TMP1, $TMP1
    vaesenclast 14*16($KS), $TMP1, $TMP1
___
} else {
$code.=<<___;
    vaesenclast 10*16($KS), $TMP1, $TMP1
___
}

$code.=<<___;
    vpxor ($CT), $TMP1, $TMP1
    vmovdqu $TMP1, ($PT)
    addq \$16, $CT
    addq \$16, $PT

    vpxor $TMP1, $T, $T
    vmovdqa -32($Htbl), $TMP0
    call GFMUL

    jmp .L${labelPrefix}_dec_loop2

.L${labelPrefix}_dec_out:
    vmovdqu $T, ($POL)
    ret
.cfi_endproc
___

  if ($aes256) {
    $code.=<<___;
.size aes256gcmsiv_dec, .-aes256gcmsiv_dec
___
  } else {
    $code.=<<___;
.size aes128gcmsiv_dec, .-aes128gcmsiv_dec
___
  }
}

aesgcmsiv_dec(0);  # emit 128-bit version

sub aes128gcmsiv_ecb_enc_block {
  my $STATE_1 = "%xmm1";
  my $KSp = "%rdx";

  # parameter 1: PT            %rdi    (pointer to 128 bit)
  # parameter 2: CT            %rsi    (pointer to 128 bit)
  # parameter 3: ks            %rdx    (pointer to ks)
  $code.=<<___;
.globl aes128gcmsiv_ecb_enc_block
.type aes128gcmsiv_ecb_enc_block,\@function,3
.align 16
aes128gcmsiv_ecb_enc_block:
.cfi_startproc
    _CET_ENDBR
    vmovdqa (%rdi), $STATE_1

    vpxor       ($KSp), $STATE_1, $STATE_1
    vaesenc 1*16($KSp), $STATE_1, $STATE_1
    vaesenc 2*16($KSp), $STATE_1, $STATE_1
    vaesenc 3*16($KSp), $STATE_1, $STATE_1
    vaesenc 4*16($KSp), $STATE_1, $STATE_1
    vaesenc 5*16($KSp), $STATE_1, $STATE_1
    vaesenc 6*16($KSp), $STATE_1, $STATE_1
    vaesenc 7*16($KSp), $STATE_1, $STATE_1
    vaesenc 8*16($KSp), $STATE_1, $STATE_1
    vaesenc 9*16($KSp), $STATE_1, $STATE_1
    vaesenclast 10*16($KSp), $STATE_1, $STATE_1    # STATE_1 == IV

    vmovdqa $STATE_1, (%rsi)

    ret
.cfi_endproc
.size aes128gcmsiv_ecb_enc_block,.-aes128gcmsiv_ecb_enc_block
___
}
aes128gcmsiv_ecb_enc_block();

sub aes256gcmsiv_aes_ks_enc_x1 {
  my $KS = "%rdx";
  my $KEYp = "%rcx";
  my $CON_MASK = "%xmm0";
  my $MASK_256 = "%xmm15";
  my $KEY_1 = "%xmm1";
  my $KEY_2 = "%xmm3";
  my $BLOCK1 = "%xmm8";
  my $AUX_REG = "%xmm14";
  my $PT = "%rdi";
  my $CT = "%rsi";

  my $round_double = sub {
    my ($i, $j) = @_;
    return <<___;
    vpshufb %xmm15, %xmm3, %xmm2
    vaesenclast %xmm0, %xmm2, %xmm2
    vpslld \$1, %xmm0, %xmm0
    vpslldq \$4, %xmm1, %xmm4
    vpxor %xmm4, %xmm1, %xmm1
    vpslldq \$4, %xmm4, %xmm4
    vpxor %xmm4, %xmm1, %xmm1
    vpslldq \$4, %xmm4, %xmm4
    vpxor %xmm4, %xmm1, %xmm1
    vpxor %xmm2, %xmm1, %xmm1
    vaesenc %xmm1, $BLOCK1, $BLOCK1
    vmovdqu %xmm1, ${\eval(16*$i)}($KS)

    vpshufd \$0xff, %xmm1, %xmm2
    vaesenclast %xmm14, %xmm2, %xmm2
    vpslldq \$4, %xmm3, %xmm4
    vpxor %xmm4, %xmm3, %xmm3
    vpslldq \$4, %xmm4, %xmm4
    vpxor %xmm4, %xmm3, %xmm3
    vpslldq \$4, %xmm4, %xmm4
    vpxor %xmm4, %xmm3, %xmm3
    vpxor %xmm2, %xmm3, %xmm3
    vaesenc %xmm3, $BLOCK1, $BLOCK1
    vmovdqu %xmm3, ${\eval(16*$j)}($KS)
___
  };

  my $round_last = sub {
    my ($i) = @_;
    return <<___;
    vpshufb %xmm15, %xmm3, %xmm2
    vaesenclast %xmm0, %xmm2, %xmm2
    vpslldq \$4, %xmm1, %xmm4
    vpxor %xmm4, %xmm1, %xmm1
    vpslldq \$4, %xmm4, %xmm4
    vpxor %xmm4, %xmm1, %xmm1
    vpslldq \$4, %xmm4, %xmm4
    vpxor %xmm4, %xmm1, %xmm1
    vpxor %xmm2, %xmm1, %xmm1
    vaesenclast %xmm1, $BLOCK1, $BLOCK1
    vmovdqu %xmm1, ${\eval(16*$i)}($KS)
___
  };

  # parameter 1: %rdi         Pointer to PT1
  # parameter 2: %rsi         Pointer to CT1
  # parameter 3: %rdx         Pointer to KS
  # parameter 4: %rcx         Pointer to initial key
  $code.=<<___;
.globl aes256gcmsiv_aes_ks_enc_x1
.type aes256gcmsiv_aes_ks_enc_x1,\@function,4
.align 16
aes256gcmsiv_aes_ks_enc_x1:
.cfi_startproc
    _CET_ENDBR
    vmovdqa con1(%rip), $CON_MASK    # CON_MASK  = 1,1,1,1
    vmovdqa mask(%rip), $MASK_256    # MASK_256
    vmovdqa ($PT), $BLOCK1
    vmovdqa ($KEYp), $KEY_1          # KEY_1 || KEY_2 [0..7] = user key
    vmovdqa 16($KEYp), $KEY_2
    vpxor $KEY_1, $BLOCK1, $BLOCK1
    vaesenc $KEY_2, $BLOCK1, $BLOCK1
    vmovdqu $KEY_1, ($KS)            # First round key
    vmovdqu $KEY_2, 16($KS)
    vpxor $AUX_REG, $AUX_REG, $AUX_REG

    ${\$round_double->(2, 3)}
    ${\$round_double->(4, 5)}
    ${\$round_double->(6, 7)}
    ${\$round_double->(8, 9)}
    ${\$round_double->(10, 11)}
    ${\$round_double->(12, 13)}
    ${\$round_last->(14)}
    vmovdqa $BLOCK1, ($CT)
    ret
.cfi_endproc
.size aes256gcmsiv_aes_ks_enc_x1,.-aes256gcmsiv_aes_ks_enc_x1
___
}
aes256gcmsiv_aes_ks_enc_x1();

sub aes256gcmsiv_ecb_enc_block {
  my $STATE_1 = "%xmm1";
  my $PT = "%rdi";
  my $CT = "%rsi";
  my $KSp = "%rdx";

  # parameter 1: PT            %rdi    (pointer to 128 bit)
  # parameter 2: CT            %rsi    (pointer to 128 bit)
  # parameter 3: ks            %rdx    (pointer to ks)
  $code.=<<___;
.globl aes256gcmsiv_ecb_enc_block
.type aes256gcmsiv_ecb_enc_block,\@function,3
.align 16
aes256gcmsiv_ecb_enc_block:
.cfi_startproc
    _CET_ENDBR
    vmovdqa (%rdi), $STATE_1
    vpxor ($KSp), $STATE_1, $STATE_1
    vaesenc 1*16($KSp), $STATE_1, $STATE_1
    vaesenc 2*16($KSp), $STATE_1, $STATE_1
    vaesenc 3*16($KSp), $STATE_1, $STATE_1
    vaesenc 4*16($KSp), $STATE_1, $STATE_1
    vaesenc 5*16($KSp), $STATE_1, $STATE_1
    vaesenc 6*16($KSp), $STATE_1, $STATE_1
    vaesenc 7*16($KSp), $STATE_1, $STATE_1
    vaesenc 8*16($KSp), $STATE_1, $STATE_1
    vaesenc 9*16($KSp), $STATE_1, $STATE_1
    vaesenc 10*16($KSp), $STATE_1, $STATE_1
    vaesenc 11*16($KSp), $STATE_1, $STATE_1
    vaesenc 12*16($KSp), $STATE_1, $STATE_1
    vaesenc 13*16($KSp), $STATE_1, $STATE_1
    vaesenclast 14*16($KSp), $STATE_1, $STATE_1    # $STATE_1 == IV
    vmovdqa $STATE_1, (%rsi)
    ret
.cfi_endproc
.size aes256gcmsiv_ecb_enc_block,.-aes256gcmsiv_ecb_enc_block
___
}
aes256gcmsiv_ecb_enc_block();

sub aes256gcmsiv_enc_msg_x4 {
  my $CTR1 = "%xmm0";
  my $CTR2 = "%xmm1";
  my $CTR3 = "%xmm2";
  my $CTR4 = "%xmm3";
  my $ADDER = "%xmm4";

  my $STATE1 = "%xmm5";
  my $STATE2 = "%xmm6";
  my $STATE3 = "%xmm7";
  my $STATE4 = "%xmm8";

  my $TMP = "%xmm12";
  my $TMP2 = "%xmm13";
  my $TMP3 = "%xmm14";
  my $IV = "%xmm15";

  my $PT = "%rdi";
  my $CT = "%rsi";
  my $TAG = "%rdx";
  my $KS = "%rcx";
  my $LEN = "%r8";

  my $aes_round = sub {
    my ($i) = @_;
    return <<___;
    vmovdqu ${\eval($i*16)}($KS), $TMP
    vaesenc $TMP, $STATE1, $STATE1
    vaesenc $TMP, $STATE2, $STATE2
    vaesenc $TMP, $STATE3, $STATE3
    vaesenc $TMP, $STATE4, $STATE4
___
  };

  my $aes_lastround = sub {
    my ($i) = @_;
    return <<___;
    vmovdqu ${\eval($i*16)}($KS), $TMP
    vaesenclast $TMP, $STATE1, $STATE1
    vaesenclast $TMP, $STATE2, $STATE2
    vaesenclast $TMP, $STATE3, $STATE3
    vaesenclast $TMP, $STATE4, $STATE4
___
  };

  # void aes256gcmsiv_enc_msg_x4(unsigned char* PT, unsigned char* CT,
  #                              unsigned char* TAG, unsigned char* KS,
  #                              size_t byte_len);
  # parameter 1: %rdi     #PT
  # parameter 2: %rsi     #CT
  # parameter 3: %rdx     #TAG  [127 126 ... 0]  IV=[127...32]
  # parameter 4: %rcx     #KS
  # parameter 5: %r8      #LEN MSG_length in bytes
  $code.=<<___;
.globl aes256gcmsiv_enc_msg_x4
.type aes256gcmsiv_enc_msg_x4,\@function,5
.align 16
aes256gcmsiv_enc_msg_x4:
.cfi_startproc
    _CET_ENDBR
    test $LEN, $LEN
    jnz .L256_enc_msg_x4_start
    ret

.L256_enc_msg_x4_start:
    movq $LEN, %r10
    shrq \$4, $LEN                       # LEN = num of blocks
    shlq \$60, %r10
    jz .L256_enc_msg_x4_start2
    addq \$1, $LEN

.L256_enc_msg_x4_start2:
    movq $LEN, %r10
    shlq \$62, %r10
    shrq \$62, %r10

    # make IV from TAG
    vmovdqa ($TAG), $IV
    vpor OR_MASK(%rip), $IV, $IV        # IV = [1]TAG[126...32][00..00]

    vmovdqa four(%rip), $ADDER          # Register to increment counters
    vmovdqa $IV, $CTR1                  # CTR1 = TAG[1][127...32][00..00]
    vpaddd one(%rip), $IV, $CTR2        # CTR2 = TAG[1][127...32][00..01]
    vpaddd two(%rip), $IV, $CTR3        # CTR3 = TAG[1][127...32][00..02]
    vpaddd three(%rip), $IV, $CTR4      # CTR4 = TAG[1][127...32][00..03]

    shrq \$2, $LEN
    je .L256_enc_msg_x4_check_remainder

    subq \$64, $CT
    subq \$64, $PT

.L256_enc_msg_x4_loop1:
    addq \$64, $CT
    addq \$64, $PT

    vmovdqa $CTR1, $STATE1
    vmovdqa $CTR2, $STATE2
    vmovdqa $CTR3, $STATE3
    vmovdqa $CTR4, $STATE4

    vpxor ($KS), $STATE1, $STATE1
    vpxor ($KS), $STATE2, $STATE2
    vpxor ($KS), $STATE3, $STATE3
    vpxor ($KS), $STATE4, $STATE4

    ${\$aes_round->(1)}
    vpaddd $ADDER, $CTR1, $CTR1
    ${\$aes_round->(2)}
    vpaddd $ADDER, $CTR2, $CTR2
    ${\$aes_round->(3)}
    vpaddd $ADDER, $CTR3, $CTR3
    ${\$aes_round->(4)}
    vpaddd $ADDER, $CTR4, $CTR4

    ${\$aes_round->(5)}
    ${\$aes_round->(6)}
    ${\$aes_round->(7)}
    ${\$aes_round->(8)}
    ${\$aes_round->(9)}
    ${\$aes_round->(10)}
    ${\$aes_round->(11)}
    ${\$aes_round->(12)}
    ${\$aes_round->(13)}
    ${\$aes_lastround->(14)}

    # XOR with Plaintext
    vpxor 0*16($PT), $STATE1, $STATE1
    vpxor 1*16($PT), $STATE2, $STATE2
    vpxor 2*16($PT), $STATE3, $STATE3
    vpxor 3*16($PT), $STATE4, $STATE4

    subq \$1, $LEN

    vmovdqu $STATE1, 0*16($CT)
    vmovdqu $STATE2, 1*16($CT)
    vmovdqu $STATE3, 2*16($CT)
    vmovdqu $STATE4, 3*16($CT)

    jne .L256_enc_msg_x4_loop1

    addq \$64, $CT
    addq \$64, $PT

.L256_enc_msg_x4_check_remainder:
    cmpq \$0, %r10
    je .L256_enc_msg_x4_out

.L256_enc_msg_x4_loop2:
    # encrypt each block separately
    # CTR1 is the highest counter (even if no LOOP done)

    vmovdqa $CTR1, $STATE1
    vpaddd one(%rip), $CTR1, $CTR1      # inc counter
    vpxor ($KS), $STATE1, $STATE1
    vaesenc 16($KS), $STATE1, $STATE1
    vaesenc 32($KS), $STATE1, $STATE1
    vaesenc 48($KS), $STATE1, $STATE1
    vaesenc 64($KS), $STATE1, $STATE1
    vaesenc 80($KS), $STATE1, $STATE1
    vaesenc 96($KS), $STATE1, $STATE1
    vaesenc 112($KS), $STATE1, $STATE1
    vaesenc 128($KS), $STATE1, $STATE1
    vaesenc 144($KS), $STATE1, $STATE1
    vaesenc 160($KS), $STATE1, $STATE1
    vaesenc 176($KS), $STATE1, $STATE1
    vaesenc 192($KS), $STATE1, $STATE1
    vaesenc 208($KS), $STATE1, $STATE1
    vaesenclast 224($KS), $STATE1, $STATE1

    # XOR with Plaintext
    vpxor ($PT), $STATE1, $STATE1

    vmovdqu $STATE1, ($CT)

    addq \$16, $PT
    addq \$16, $CT

    subq \$1, %r10
    jne .L256_enc_msg_x4_loop2

.L256_enc_msg_x4_out:
    ret
.cfi_endproc
.size aes256gcmsiv_enc_msg_x4,.-aes256gcmsiv_enc_msg_x4
___
}
aes256gcmsiv_enc_msg_x4();

sub aes256gcmsiv_enc_msg_x8() {
  my $STATE1 = "%xmm1";
  my $STATE2 = "%xmm2";
  my $STATE3 = "%xmm3";
  my $STATE4 = "%xmm4";
  my $STATE5 = "%xmm5";
  my $STATE6 = "%xmm6";
  my $STATE7 = "%xmm7";
  my $STATE8 = "%xmm8";
  my $CTR1 = "%xmm0";
  my $CTR2 = "%xmm9";
  my $CTR3 = "%xmm10";
  my $CTR4 = "%xmm11";
  my $CTR5 = "%xmm12";
  my $CTR6 = "%xmm13";
  my $CTR7 = "%xmm14";
  my $TMP1 = "%xmm1";
  my $TMP2 = "%xmm2";
  my $KS = "%rcx";
  my $LEN = "%r8";
  my $PT = "%rdi";
  my $CT = "%rsi";
  my $TAG = "%rdx";
  my $SCHED = "%xmm15";

  my $aes_round8 = sub {
    my ($i) = @_;
    return <<___;
    vmovdqu ${\eval($i*16)}($KS), $SCHED
    vaesenc $SCHED, $STATE1, $STATE1
    vaesenc $SCHED, $STATE2, $STATE2
    vaesenc $SCHED, $STATE3, $STATE3
    vaesenc $SCHED, $STATE4, $STATE4
    vaesenc $SCHED, $STATE5, $STATE5
    vaesenc $SCHED, $STATE6, $STATE6
    vaesenc $SCHED, $STATE7, $STATE7
    vaesenc $SCHED, $STATE8, $STATE8
___
  };

  my $aes_lastround8 = sub {
    my ($i) = @_;
    return <<___;
    vmovdqu ${\eval($i*16)}($KS), $SCHED
    vaesenclast $SCHED, $STATE1, $STATE1
    vaesenclast $SCHED, $STATE2, $STATE2
    vaesenclast $SCHED, $STATE3, $STATE3
    vaesenclast $SCHED, $STATE4, $STATE4
    vaesenclast $SCHED, $STATE5, $STATE5
    vaesenclast $SCHED, $STATE6, $STATE6
    vaesenclast $SCHED, $STATE7, $STATE7
    vaesenclast $SCHED, $STATE8, $STATE8
___
  };

  # void ENC_MSG_x8(unsigned char* PT,
  #                 unsigned char* CT,
  #                 unsigned char* TAG,
  #                 unsigned char* KS,
  #                 size_t byte_len);
  # parameter 1: %rdi     #PT
  # parameter 2: %rsi     #CT
  # parameter 3: %rdx     #TAG        [127 126 ... 0]  IV=[127...32]
  # parameter 4: %rcx     #KS
  # parameter 5: %r8      #LEN MSG_length in bytes
  $code.=<<___;
.globl aes256gcmsiv_enc_msg_x8
.type aes256gcmsiv_enc_msg_x8,\@function,5
.align 16
aes256gcmsiv_enc_msg_x8:
.cfi_startproc
    _CET_ENDBR
    test $LEN, $LEN
    jnz .L256_enc_msg_x8_start
    ret

.L256_enc_msg_x8_start:
    # Place in stack
    movq %rsp, %r11
    subq \$16, %r11
    andq \$-64, %r11

    movq $LEN, %r10
    shrq \$4, $LEN                       # LEN = num of blocks
    shlq \$60, %r10
    jz .L256_enc_msg_x8_start2
    addq \$1, $LEN

.L256_enc_msg_x8_start2:
    movq $LEN, %r10
    shlq \$61, %r10
    shrq \$61, %r10

    # Make IV from TAG
    vmovdqa ($TAG), $TMP1
    vpor OR_MASK(%rip), $TMP1, $TMP1    # TMP1= IV = [1]TAG[126...32][00..00]

    # store counter8 on the stack
    vpaddd seven(%rip), $TMP1, $CTR1
    vmovdqa $CTR1, (%r11)                # CTR8 = TAG[127...32][00..07]
    vpaddd one(%rip), $TMP1, $CTR2       # CTR2 = TAG[127...32][00..01]
    vpaddd two(%rip), $TMP1, $CTR3       # CTR3 = TAG[127...32][00..02]
    vpaddd three(%rip), $TMP1, $CTR4     # CTR4 = TAG[127...32][00..03]
    vpaddd four(%rip), $TMP1, $CTR5      # CTR5 = TAG[127...32][00..04]
    vpaddd five(%rip), $TMP1, $CTR6      # CTR6 = TAG[127...32][00..05]
    vpaddd six(%rip), $TMP1, $CTR7       # CTR7 = TAG[127...32][00..06]
    vmovdqa $TMP1, $CTR1                 # CTR1 = TAG[127...32][00..00]

    shrq \$3, $LEN
    jz .L256_enc_msg_x8_check_remainder

    subq \$128, $CT
    subq \$128, $PT

.L256_enc_msg_x8_loop1:
    addq \$128, $CT
    addq \$128, $PT

    vmovdqa $CTR1, $STATE1
    vmovdqa $CTR2, $STATE2
    vmovdqa $CTR3, $STATE3
    vmovdqa $CTR4, $STATE4
    vmovdqa $CTR5, $STATE5
    vmovdqa $CTR6, $STATE6
    vmovdqa $CTR7, $STATE7
    # move from stack
    vmovdqa (%r11), $STATE8

    vpxor ($KS), $STATE1, $STATE1
    vpxor ($KS), $STATE2, $STATE2
    vpxor ($KS), $STATE3, $STATE3
    vpxor ($KS), $STATE4, $STATE4
    vpxor ($KS), $STATE5, $STATE5
    vpxor ($KS), $STATE6, $STATE6
    vpxor ($KS), $STATE7, $STATE7
    vpxor ($KS), $STATE8, $STATE8

    ${\$aes_round8->(1)}
    vmovdqa (%r11), $CTR7                # deal with CTR8
    vpaddd eight(%rip), $CTR7, $CTR7
    vmovdqa $CTR7, (%r11)
    ${\$aes_round8->(2)}
    vpsubd one(%rip), $CTR7, $CTR7
    ${\$aes_round8->(3)}
    vpaddd eight(%rip), $CTR1, $CTR1
    ${\$aes_round8->(4)}
    vpaddd eight(%rip), $CTR2, $CTR2
    ${\$aes_round8->(5)}
    vpaddd eight(%rip), $CTR3, $CTR3
    ${\$aes_round8->(6)}
    vpaddd eight(%rip), $CTR4, $CTR4
    ${\$aes_round8->(7)}
    vpaddd eight(%rip), $CTR5, $CTR5
    ${\$aes_round8->(8)}
    vpaddd eight(%rip), $CTR6, $CTR6
    ${\$aes_round8->(9)}
    ${\$aes_round8->(10)}
    ${\$aes_round8->(11)}
    ${\$aes_round8->(12)}
    ${\$aes_round8->(13)}
    ${\$aes_lastround8->(14)}

    # XOR with Plaintext
    vpxor 0*16($PT), $STATE1, $STATE1
    vpxor 1*16($PT), $STATE2, $STATE2
    vpxor 2*16($PT), $STATE3, $STATE3
    vpxor 3*16($PT), $STATE4, $STATE4
    vpxor 4*16($PT), $STATE5, $STATE5
    vpxor 5*16($PT), $STATE6, $STATE6
    vpxor 6*16($PT), $STATE7, $STATE7
    vpxor 7*16($PT), $STATE8, $STATE8

    subq \$1, $LEN

    vmovdqu $STATE1, 0*16($CT)
    vmovdqu $STATE2, 1*16($CT)
    vmovdqu $STATE3, 2*16($CT)
    vmovdqu $STATE4, 3*16($CT)
    vmovdqu $STATE5, 4*16($CT)
    vmovdqu $STATE6, 5*16($CT)
    vmovdqu $STATE7, 6*16($CT)
    vmovdqu $STATE8, 7*16($CT)

    jne .L256_enc_msg_x8_loop1

    addq \$128, $CT
    addq \$128, $PT

.L256_enc_msg_x8_check_remainder:
   cmpq \$0, %r10
   je .L256_enc_msg_x8_out

.L256_enc_msg_x8_loop2:
    # encrypt each block separately
    # CTR1 is the highest counter (even if no LOOP done)
    vmovdqa $CTR1, $STATE1
    vpaddd one(%rip), $CTR1, $CTR1

    vpxor ($KS), $STATE1, $STATE1
    vaesenc 16($KS), $STATE1, $STATE1
    vaesenc 32($KS), $STATE1, $STATE1
    vaesenc 48($KS), $STATE1, $STATE1
    vaesenc 64($KS), $STATE1, $STATE1
    vaesenc 80($KS), $STATE1, $STATE1
    vaesenc 96($KS), $STATE1, $STATE1
    vaesenc 112($KS), $STATE1, $STATE1
    vaesenc 128($KS), $STATE1, $STATE1
    vaesenc 144($KS), $STATE1, $STATE1
    vaesenc 160($KS), $STATE1, $STATE1
    vaesenc 176($KS), $STATE1, $STATE1
    vaesenc 192($KS), $STATE1, $STATE1
    vaesenc 208($KS), $STATE1, $STATE1
    vaesenclast 224($KS), $STATE1, $STATE1

    # XOR with Plaintext
    vpxor ($PT), $STATE1, $STATE1

    vmovdqu $STATE1, ($CT)

    addq \$16, $PT
    addq \$16, $CT
    subq \$1, %r10
    jnz .L256_enc_msg_x8_loop2

.L256_enc_msg_x8_out:
    ret

.cfi_endproc
.size aes256gcmsiv_enc_msg_x8,.-aes256gcmsiv_enc_msg_x8
___
}
aes256gcmsiv_enc_msg_x8();
aesgcmsiv_dec(1);

sub aes256gcmsiv_kdf {
  my $ONE = "%xmm8";
  my $BLOCK1 = "%xmm4";
  my $BLOCK2 = "%xmm6";
  my $BLOCK3 = "%xmm7";
  my $BLOCK4 = "%xmm11";
  my $BLOCK5 = "%xmm12";
  my $BLOCK6 = "%xmm13";

  my $enc_roundx6 = sub {
    my ($i, $j) = @_;
    return <<___;
    vmovdqa ${\eval($i*16)}(%rdx), $j
    vaesenc $j, $BLOCK1, $BLOCK1
    vaesenc $j, $BLOCK2, $BLOCK2
    vaesenc $j, $BLOCK3, $BLOCK3
    vaesenc $j, $BLOCK4, $BLOCK4
    vaesenc $j, $BLOCK5, $BLOCK5
    vaesenc $j, $BLOCK6, $BLOCK6
___
  };

  my $enc_roundlastx6 = sub {
    my ($i, $j) = @_;
    return <<___;
    vmovdqa ${\eval($i*16)}(%rdx), $j
    vaesenclast $j, $BLOCK1, $BLOCK1
    vaesenclast $j, $BLOCK2, $BLOCK2
    vaesenclast $j, $BLOCK3, $BLOCK3
    vaesenclast $j, $BLOCK4, $BLOCK4
    vaesenclast $j, $BLOCK5, $BLOCK5
    vaesenclast $j, $BLOCK6, $BLOCK6
___
  };

  # void aes256gcmsiv_kdf(const uint8_t nonce[16],
  #                       uint8_t *out_key_material,
  #                       const uint8_t *key_schedule);
  $code.=<<___;
.globl aes256gcmsiv_kdf
.type aes256gcmsiv_kdf,\@function,3
.align 16
aes256gcmsiv_kdf:
.cfi_startproc
    _CET_ENDBR
# parameter 1: %rdi                         Pointer to NONCE
# parameter 2: %rsi                         Pointer to CT
# parameter 4: %rdx                         Pointer to keys

    vmovdqa (%rdx), %xmm1                  # xmm1 = first 16 bytes of random key
    vmovdqa 0*16(%rdi), $BLOCK1
    vmovdqa and_mask(%rip), $BLOCK4
    vmovdqa one(%rip), $ONE
    vpshufd \$0x90, $BLOCK1, $BLOCK1
    vpand $BLOCK4, $BLOCK1, $BLOCK1
    vpaddd $ONE, $BLOCK1, $BLOCK2
    vpaddd $ONE, $BLOCK2, $BLOCK3
    vpaddd $ONE, $BLOCK3, $BLOCK4
    vpaddd $ONE, $BLOCK4, $BLOCK5
    vpaddd $ONE, $BLOCK5, $BLOCK6

    vpxor %xmm1, $BLOCK1, $BLOCK1
    vpxor %xmm1, $BLOCK2, $BLOCK2
    vpxor %xmm1, $BLOCK3, $BLOCK3
    vpxor %xmm1, $BLOCK4, $BLOCK4
    vpxor %xmm1, $BLOCK5, $BLOCK5
    vpxor %xmm1, $BLOCK6, $BLOCK6

    ${\$enc_roundx6->(1, "%xmm1")}
    ${\$enc_roundx6->(2, "%xmm2")}
    ${\$enc_roundx6->(3, "%xmm1")}
    ${\$enc_roundx6->(4, "%xmm2")}
    ${\$enc_roundx6->(5, "%xmm1")}
    ${\$enc_roundx6->(6, "%xmm2")}
    ${\$enc_roundx6->(7, "%xmm1")}
    ${\$enc_roundx6->(8, "%xmm2")}
    ${\$enc_roundx6->(9, "%xmm1")}
    ${\$enc_roundx6->(10, "%xmm2")}
    ${\$enc_roundx6->(11, "%xmm1")}
    ${\$enc_roundx6->(12, "%xmm2")}
    ${\$enc_roundx6->(13, "%xmm1")}
    ${\$enc_roundlastx6->(14, "%xmm2")}

    vmovdqa $BLOCK1, 0*16(%rsi)
    vmovdqa $BLOCK2, 1*16(%rsi)
    vmovdqa $BLOCK3, 2*16(%rsi)
    vmovdqa $BLOCK4, 3*16(%rsi)
    vmovdqa $BLOCK5, 4*16(%rsi)
    vmovdqa $BLOCK6, 5*16(%rsi)
    ret
.cfi_endproc
.size aes256gcmsiv_kdf, .-aes256gcmsiv_kdf
___
}
aes256gcmsiv_kdf();

if ($avx) { print $code; }

close STDOUT or die "error closing STDOUT: $!";
