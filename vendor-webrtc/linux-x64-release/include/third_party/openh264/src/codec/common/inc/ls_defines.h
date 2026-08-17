/*!
 * \copy
 *     Copyright (c)  2013, Cisco Systems
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

#ifndef ___LD_ST_MACROS___
#define ___LD_ST_MACROS___

#include <string.h>
#include "typedefs.h"

#ifdef __GNUC__

struct tagUnaligned_64 {
  uint64_t l;
} __attribute__ ((packed)) __attribute__ ((may_alias));
struct tagUnaligned_32 {
  uint32_t l;
} __attribute__ ((packed)) __attribute__ ((may_alias));
struct tagUnaligned_16 {
  uint16_t l;
} __attribute__ ((packed)) __attribute__ ((may_alias));

#define LD16(a) (((struct tagUnaligned_16 *) (a))->l)
#define LD32(a) (((struct tagUnaligned_32 *) (a))->l)
#define LD64(a) (((struct tagUnaligned_64 *) (a))->l)

#define STRUCTA(size, align) struct tagUnaligned_##size##_##align {\
    uint##size##_t l; \
} __attribute__ ((aligned(align))) __attribute__ ((may_alias))
STRUCTA (16, 2);
STRUCTA (32, 2);
STRUCTA (32, 4);
STRUCTA (64, 2);
STRUCTA (64, 4);
STRUCTA (64, 8);
//#define _USE_STRUCT_INT_CVT
//#ifdef _USE_STRUCT_INT_CVT
#define ST16(a, b) (((struct tagUnaligned_16 *) (a))->l) = (b)
#define ST32(a, b) (((struct tagUnaligned_32 *) (a))->l) = (b)
#define ST64(a, b) (((struct tagUnaligned_64 *) (a))->l) = (b)

#define LDA(a, size, align) (((struct tagUnaligned_##size##_##align *) (a))->l)
#define STA(a, b, size, align) (((struct tagUnaligned_##size##_##align *) (a))->l) = (b)
#define LD16A2(a) LDA(a, 16, 2)
#define LD32A2(a) LDA(a, 32, 2)
#define LD32A4(a) LDA(a, 32, 4)
#define LD64A2(a) LDA(a, 64, 2)
#define LD64A4(a) LDA(a, 64, 4)
#define LD64A8(a) LDA(a, 64, 8)
#define ST16A2(a, b) STA(a, b, 16, 2)
#define ST32A2(a, b) STA(a, b, 32, 2)
#define ST32A4(a, b) STA(a, b, 32, 4)
#define ST64A2(a, b) STA(a, b, 64, 2)
#define ST64A4(a, b) STA(a, b, 64, 4)
#define ST64A8(a, b) STA(a, b, 64, 8)
//#else
//inline void __ST16(void *dst, uint16_t v) { memcpy(dst, &v, 2); }
//inline void __ST32(void *dst, uint32_t v) { memcpy(dst, &v, 4); }
//inline void __ST64(void *dst, uint64_t v) { memcpy(dst, &v, 8); }
//#endif

#else

//#define INTD16(a) (*((int16_t*)(a)))
//#define INTD32(a) (*((int32_t*)(a)))
//#define INTD64(a) (*((int64_t*)(a)))

#define LD16(a) (*((uint16_t*)(a)))
#define LD32(a) (*((uint32_t*)(a)))
#define LD64(a) (*((uint64_t*)(a)))

#define ST16(a, b) *((uint16_t*)(a)) = (b)
#define ST32(a, b) *((uint32_t*)(a)) = (b)
#define ST64(a, b) *((uint64_t*)(a)) = (b)
#define LD16A2 LD16
#define LD32A2 LD32
#define LD32A4 LD32
#define LD64A2 LD64
#define LD64A4 LD64
#define LD64A8 LD64
#define ST16A2 ST16
#define ST32A2 ST32
#define ST32A4 ST32
#define ST64A2 ST64
#define ST64A4 ST64
#define ST64A8 ST64

#endif /* !__GNUC__ */

#ifndef INTD16
#define INTD16 LD16
#endif//INTD16

#ifndef INTD32
#define INTD32 LD32
#endif//INTD32

#ifndef INTD64
#define INTD64 LD64
#endif//INTD64

#endif//___LD_ST_MACROS___
