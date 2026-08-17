/*
 * Copyright © 2019, VideoLAN and dav1d authors
 * Copyright © 2019, Luca Barbato
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this
 *    list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef DAV1D_SRC_PPC_TYPES_H
#define DAV1D_SRC_PPC_TYPES_H

#include <altivec.h>
#undef pixel

#define u8x16 vector unsigned char
#define i8x16 vector signed char
#define b8x16 vector bool char
#define u16x8 vector unsigned short
#define i16x8 vector signed short
#define b16x8 vector bool short
#define u32x4 vector unsigned int
#define i32x4 vector signed int
#define b32x4 vector bool int
#define u64x2 vector unsigned long long
#define i64x2 vector signed long long
#define b64x2 vector bool long long

#define i8h_to_i16(v) ((i16x8) vec_unpackh((i8x16)v))
#define i8l_to_i16(v) ((i16x8) vec_unpackl((i8x16)v))
#define u8h_to_i16(v) ((i16x8) vec_mergeh((u8x16) v, vec_splat_u8(0)))
#define u8l_to_i16(v) ((i16x8) vec_mergel((u8x16) v, vec_splat_u8(0)))
#define u8h_to_u16(v) ((u16x8) vec_mergeh((u8x16) v, vec_splat_u8(0)))
#define u8l_to_u16(v) ((u16x8) vec_mergel((u8x16) v, vec_splat_u8(0)))
#define u16h_to_i32(v) ((i32x4) vec_mergeh((u16x8) v, vec_splat_u16(0)))
#define i16h_to_i32(v) ((i32x4) vec_unpackh((i16x8)v))
#define u16l_to_i32(v) ((i32x4) vec_mergel((u16x8) v, vec_splat_u16(0)))
#define i16l_to_i32(v) ((i32x4) vec_unpackl((i16x8)v))

#endif /* DAV1D_SRC_PPC_TYPES_H */
