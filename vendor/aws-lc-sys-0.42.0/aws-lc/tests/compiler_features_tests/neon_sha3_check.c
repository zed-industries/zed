// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

// Test for NEON SHA3 extension support in the assembler
// Requires -march=armv8.4-a+sha3 compiler flag
int main(void) {
    __asm__("eor3 v0.16b, v1.16b, v2.16b, v3.16b");
    __asm__("bcax v0.16b, v1.16b, v2.16b, v3.16b");
    __asm__("rax1 v0.2d,  v1.2d,  v2.2d         ");
    __asm__("xar  v0.2d,  v1.2d,  v2.2d,  #0x2a ");
    return 0;
}
