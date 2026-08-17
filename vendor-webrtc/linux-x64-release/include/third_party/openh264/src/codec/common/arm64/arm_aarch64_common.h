/*!
 *@page License
 *
 * \copy
 *     Copyright (c)  2024, ARM Ltd.
 *     All rights reserved.
 *
 *     Redistribution and use in source and binary forms, with or without
 *     modification, are permitted provided that the following conditions
 *     are met:
 *
 *        * Redistributions of source code must retain the above copyright
 *          notice, this list of conditions and the following disclaimer.
 *
 *        * Redistributions in binary form must reproduce the above copyright
 *          notice, this list of conditions and the following disclaimer in
 *          the documentation and/or other materials provided with the
 *          distribution.
 *
 *     THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 *     "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 *     LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 *     FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 *     COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT,
 *     INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
 *     BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 *     LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 *     CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 *     LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN
 *     ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 *     POSSIBILITY OF SUCH DAMAGE.
 *
 */

#ifndef CODEC_COMMON_ARM64_ARM_AARCH64_COMMON_H_
#define CODEC_COMMON_ARM64_ARM_AARCH64_COMMON_H_

/*
 ; Support macros for
 ;   - Armv8.3-A Pointer Authentication and
 ;   - Armv8.5-A Branch Target Identification
 ; Further documentation can be found at:
 ;  - https://developer.arm.com/documentation/101028/0012/5--Feature-test-macros
 ;
 ; Since openh264 aarch64 assembly code provides functions with no storage of the
 ; LR(x30) on the stack, PAC is not needed as modification of the LR value would
 ; require modification of x30 and not memory. Additionally, no indirect control
 ; flow changes are performed, so bti j instructions are not needed. Thus, just
 ; mark the entry points with bti c landing pads and the ELF files as supporting
 ; BTI and PAC.
 */
#if defined(__ARM_FEATURE_BTI_DEFAULT) && __ARM_FEATURE_BTI_DEFAULT == 1
  /* BTI is enabled */
  #define BTI_C hint 34
  #define GNU_PROPERTY_AARCH64_BTI 0x1 /* Property for notes section in ELF */
#else
  /* BTI is NOT enabled */
  #define BTI_C
  #define GNU_PROPERTY_AARCH64_BTI 0
#endif

#if defined(__ARM_FEATURE_PAC_DEFAULT)
  /* PAC is enabled */
  #define GNU_PROPERTY_AARCH64_POINTER_AUTH 0x2 /* Property for notes section in ELF */
#else
  /* PAC is not enabled */
  #define GNU_PROPERTY_AARCH64_POINTER_AUTH 0
#endif

/* Add the notes section to ELF only */
#if defined(__ELF__)
  .pushsection .note.gnu.property, "a";
  .balign 8;
  .long 4;
  .long 0x10;
  .long 0x5;
  .asciz "GNU";
  .long 0xc0000000; /* GNU_PROPERTY_AARCH64_FEATURE_1_AND */
  .long 4;
  .long(GNU_PROPERTY_AARCH64_POINTER_AUTH | GNU_PROPERTY_AARCH64_BTI);
  .long 0;
  .popsection;
#endif

#endif /* CODEC_COMMON_ARM64_ARM_AARCH64_COMMON_H_ */
