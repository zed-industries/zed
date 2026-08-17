#!/usr/bin/env perl
# Copyright (c) 2018, Google Inc.
#
# SPDX-License-Identifier: ISC

# This file defines helper functions for crypto/test/abi_test.h on x86. See
# that header for details on how to use this.
#
# For convenience, this file is linked into libcrypto, where consuming builds
# already support architecture-specific sources. The static linker should drop
# this code in non-test binaries. This includes a shared library build of
# libcrypto, provided --gc-sections (ELF), -dead_strip (Mac), or equivalent is
# used.
#
# References:
#
# SysV ABI: https://uclibc.org/docs/psABI-i386.pdf
# Win32 ABI: https://docs.microsoft.com/en-us/cpp/cpp/argument-passing-and-naming-conventions?view=vs-2017

use strict;

$0 =~ m/(.*[\/\\])[^\/\\]+$/;
my $dir = $1;
push(@INC, "${dir}", "${dir}../../perlasm");
require "x86asm.pl";

my $output = $ARGV[1];
open STDOUT, ">$output";

&asm_init($ARGV[0]);

# abi_test_trampoline loads callee-saved registers from |state|, calls |func|
# with |argv|, then saves the callee-saved registers into |state|. It returns
# the result of |func|. |unwind| is ignored.
# uint32_t abi_test_trampoline(void (*func)(...), CallerState *state,
#                              const uint32_t *argv, size_t argc,
#                              int unwind);
&function_begin("abi_test_trampoline")
	# Load registers from |state|. Note |function_begin| (as opposed to
	# |function_begin_B|) automatically saves all callee-saved registers, so we
	# may freely clobber them.
	&mov("ecx", &wparam(1));
	&mov("esi", &DWP(4*0, "ecx"));
	&mov("edi", &DWP(4*1, "ecx"));
	&mov("ebx", &DWP(4*2, "ecx"));
	&mov("ebp", &DWP(4*3, "ecx"));

	# Use a fixed stack allocation so |wparam| continues to work. abi_test.h
	# supports at most 10 arguments. The SysV ABI requires a 16-byte-aligned
	# stack on process entry, so round up to 3 (mod 4).
	&stack_push(11);

	# Copy parameters to stack.
	&mov("eax", &wparam(2));
	&xor("ecx", "ecx");
&set_label("loop");
	&cmp("ecx", &wparam(3));
	&jae(&label("loop_done"));
	&mov("edx", &DWP(0, "eax", "ecx", 4));
	&mov(&DWP(0, "esp", "ecx", 4), "edx");
	&add("ecx", 1);
	&jmp(&label("loop"));

&set_label("loop_done");
	&call_ptr(&wparam(0));

	&stack_pop(11);

	# Save registers back into |state|.
	&mov("ecx", &wparam(1));
	&mov(&DWP(4*0, "ecx"), "esi");
	&mov(&DWP(4*1, "ecx"), "edi");
	&mov(&DWP(4*2, "ecx"), "ebx");
	&mov(&DWP(4*3, "ecx"), "ebp");
&function_end("abi_test_trampoline")

# abi_test_get_and_clear_direction_flag clears the direction flag. If the flag
# was previously set, it returns one. Otherwise, it returns zero.
# int abi_test_get_and_clear_direction_flag(void);
&function_begin_B("abi_test_get_and_clear_direction_flag");
	&pushf();
	&pop("eax");
	&and("eax", 0x400);
	&shr("eax", 10);
	&cld();
	&ret();
&function_end_B("abi_test_get_and_clear_direction_flag");

# abi_test_set_direction_flag sets the direction flag.
# void abi_test_set_direction_flag(void);
&function_begin_B("abi_test_set_direction_flag");
	&std();
	&ret();
&function_end_B("abi_test_set_direction_flag");

# abi_test_clobber_* zeros the corresponding register. These are used to test
# the ABI-testing framework.
foreach ("eax", "ebx", "ecx", "edx", "edi", "esi", "ebp") {
&function_begin_B("abi_test_clobber_$_");
	&xor($_, $_);
	&ret();
&function_end_B("abi_test_clobber_$_");
}
foreach (0..7) {
&function_begin_B("abi_test_clobber_xmm$_");
	&pxor("xmm$_", "xmm$_");
	&ret();
&function_end_B("abi_test_clobber_xmm$_");
}

&asm_finish();

close STDOUT or die "error closing STDOUT: $!";
