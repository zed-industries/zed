#! /usr/bin/env perl

# Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0 OR ISC

# RNDR from ARMv8.5-A.
# System register encoding: s3_3_c2_c4_0.
# see https://developer.arm.com/documentation/ddi0601/2024-09/AArch64-Registers/RNDR--Random-Number

# The first two arguments should always be the flavour and output file path.
if ($#ARGV < 1) { die "Not enough arguments provided.
  Two arguments are necessary: the flavour and the output file path."; }

my $flavour = shift;
my $output  = shift;

$0 =~ m/(.*[\/\\])[^\/\\]+$/; $dir=$1;
( $xlate="${dir}arm-xlate.pl" and -f $xlate ) or
( $xlate="${dir}../../../perlasm/arm-xlate.pl" and -f $xlate) or
die "can't locate arm-xlate.pl";

open OUT,qq{| "$^X" "$xlate" "$flavour" "$output"};
*STDOUT=*OUT;

my ($out, $len, $rndr64) = ("x0", "x1", "x2");

$code.=<<___;
#include <openssl/arm_arch.h>

.arch armv8-a
.text

# int CRYPTO_rndr_multiple8(uint8_t *out, const size_t len)
.globl CRYPTO_rndr_multiple8
.type CRYPTO_rndr_multiple8,%function
.align 4
CRYPTO_rndr_multiple8:
  cbz $len, .Lrndr_multiple8_error  // len = 0 is not supported

.Lrndr_multiple8_loop:
  mrs $rndr64, s3_3_c2_c4_0             // rndr instruction https://developer.arm.com/documentation/ddi0601/2024-09/Index-by-Encoding
  cbz $rndr64, .Lrndr_multiple8_error   // Check if rndr failed

  str $rndr64, [$out], #8               // Copy 8 bytes to *out and increment pointer by 8
  sub $len, $len, #8
  cbz $len, .Lrndr_multiple8_done       // If multiple of 8 this will be 0 eventually
  b .Lrndr_multiple8_loop

.Lrndr_multiple8_done:
  mov x0, #1                            // Return value success
  ret

.Lrndr_multiple8_error:
  mov x0, #0                            // Return value error
  ret
.size CRYPTO_rndr_multiple8,.-CRYPTO_rndr_multiple8
___

print $code;
close STDOUT or die "error closing STDOUT: $!"; # enforce flush
