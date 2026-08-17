// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_CPU_MIPS_COMMON_MACROS_MSA_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_CPU_MIPS_COMMON_MACROS_MSA_H_

#include <msa.h>
#include <stdint.h>

#if defined(__clang__)
#define CLANG_BUILD
#endif

typedef union {
  int32_t intVal;
  float floatVal;
} FloatInt;

#ifdef CLANG_BUILD
#define SRLI_B(a, b) __msa_srli_b((v16i8)a, b)
#define SRLI_H(a, b) __msa_srli_h((v8i16)a, b)
#define SLLI_B(a, b) __msa_slli_b((v16i8)a, b)
#define SLLI_H(a, b) __msa_slli_h((v8i16)a, b)
#define CEQI_B(a, b) __msa_ceqi_b((v16i8)a, b)
#define CEQI_H(a, b) __msa_ceqi_h((v8i16)a, b)
#define ANDI_B(a, b) __msa_andi_b((v16u8)a, b)
#else
#define SRLI_B(a, b) ((v16u8)a >> b)
#define SRLI_H(a, b) ((v8u16)a >> b)
#define SLLI_B(a, b) ((v16i8)a << b)
#define SLLI_H(a, b) ((v8i16)a << b)
#define CEQI_B(a, b) (a == b)
#define CEQI_H(a, b) (a == b)
#define ANDI_B(a, b) ((v16u8)a & b)
#endif

#define LD_V(RTYPE, psrc) *((RTYPE*)(psrc))
#define LD_UB(...) LD_V(v16u8, __VA_ARGS__)
#define LD_UH(...) LD_V(v8u16, __VA_ARGS__)
#define LD_SP(...) LD_V(v4f32, __VA_ARGS__)
#define LD_DP(...) LD_V(v2f64, __VA_ARGS__)

#define ST_V(RTYPE, in, pdst) *((RTYPE*)(pdst)) = in
#define ST_UB(...) ST_V(v16u8, __VA_ARGS__)
#define ST_UH(...) ST_V(v8u16, __VA_ARGS__)
#define ST_SP(...) ST_V(v4f32, __VA_ARGS__)
#define ST_DP(...) ST_V(v2f64, __VA_ARGS__)

#ifdef CLANG_BUILD
#define COPY_DOUBLE_TO_VECTOR(a)                \
  ({                                            \
    v2f64 out;                                  \
    out = (v2f64)__msa_fill_d(*(int64_t*)(&a)); \
    out;                                        \
  })
#else
#define COPY_DOUBLE_TO_VECTOR(a)                \
  ({                                            \
    v2f64 out;                                  \
    out = __msa_cast_to_vector_double(a);       \
    out = (v2f64)__msa_splati_d((v2i64)out, 0); \
    out;                                        \
  })
#endif

#define MSA_STORE_FUNC(TYPE, INSTR, FUNCNAME)               \
  static inline void FUNCNAME(TYPE val, void* const pdst) { \
    uint8_t* const pdstm = (uint8_t*)pdst;                  \
    TYPE valm = val;                                        \
    asm volatile(" " #INSTR "  %[valm],  %[pdstm]  \n\t"    \
                 : [pdstm] "=m"(*pdstm)                     \
                 : [valm] "r"(valm));                       \
  }

#define MSA_STORE(val, pdst, FUNCNAME) FUNCNAME(val, pdst)

#ifdef CLANG_BUILD
MSA_STORE_FUNC(uint32_t, sw, msa_sw);
#define SW(val, pdst) MSA_STORE(val, pdst, msa_sw)
#if (__mips == 64)
MSA_STORE_FUNC(uint64_t, sd, msa_sd);
#define SD(val, pdst) MSA_STORE(val, pdst, msa_sd)
#else
#define SD(val, pdst)                                                    \
  {                                                                      \
    uint8_t* const pdstsd = (uint8_t*)(pdst);                            \
    const uint32_t val0m = (uint32_t)(val & 0x00000000FFFFFFFF);         \
    const uint32_t val1m = (uint32_t)((val >> 32) & 0x00000000FFFFFFFF); \
    SW(val0m, pdstsd);                                                   \
    SW(val1m, pdstsd + 4);                                               \
  }
#endif
#else
#if (__mips_isa_rev >= 6)
MSA_STORE_FUNC(uint32_t, sw, msa_sw);
#define SW(val, pdst) MSA_STORE(val, pdst, msa_sw)
MSA_STORE_FUNC(uint64_t, sd, msa_sd);
#define SD(val, pdst) MSA_STORE(val, pdst, msa_sd)
#else  // !(__mips_isa_rev >= 6)
MSA_STORE_FUNC(uint32_t, usw, msa_usw);
#define SW(val, pdst) MSA_STORE(val, pdst, msa_usw)
#define SD(val, pdst)                                                    \
  {                                                                      \
    uint8_t* const pdstsd = (uint8_t*)(pdst);                            \
    const uint32_t val0m = (uint32_t)(val & 0x00000000FFFFFFFF);         \
    const uint32_t val1m = (uint32_t)((val >> 32) & 0x00000000FFFFFFFF); \
    SW(val0m, pdstsd);                                                   \
    SW(val1m, pdstsd + 4);                                               \
  }
#endif  // (__mips_isa_rev >= 6)
#endif

/* Description : Load vectors with elements with stride
 * Arguments   : Inputs  - psrc, stride
 *               Outputs - out0, out1
 *               Return Type - as per RTYPE
 * Details     : Load elements in 'out0' from (psrc)
 *               Load elements in 'out1' from (psrc + stride)
 */
#define LD_V2(RTYPE, psrc, stride, out0, out1) \
  {                                            \
    out0 = LD_V(RTYPE, psrc);                  \
    psrc += stride;                            \
    out1 = LD_V(RTYPE, psrc);                  \
    psrc += stride;                            \
  }
#define LD_UB2(...) LD_V2(v16u8, __VA_ARGS__)
#define LD_UH2(...) LD_V2(v8u16, __VA_ARGS__)
#define LD_SP2(...) LD_V2(v4f32, __VA_ARGS__)

#define LD_V3(RTYPE, psrc, stride, out0, out1, out2) \
  {                                                  \
    LD_V2(RTYPE, psrc, stride, out0, out1);          \
    out2 = LD_V(RTYPE, psrc);                        \
    psrc += stride;                                  \
  }
#define LD_UB3(...) LD_V3(v16u8, __VA_ARGS__)
#define LD_UH3(...) LD_V3(v8u16, __VA_ARGS__)

#define LD_V4(RTYPE, psrc, stride, out0, out1, out2, out3) \
  {                                                        \
    LD_V2(RTYPE, psrc, stride, out0, out1);                \
    LD_V2(RTYPE, psrc, stride, out2, out3);                \
  }
#define LD_UB4(...) LD_V4(v16u8, __VA_ARGS__)
#define LD_UH4(...) LD_V4(v8u16, __VA_ARGS__)
#define LD_SP4(...) LD_V4(v4f32, __VA_ARGS__)

#define LD_V5(RTYPE, psrc, stride, out0, out1, out2, out3, out4) \
  {                                                              \
    LD_V4(RTYPE, psrc, stride, out0, out1, out2, out3);          \
    out4 = LD_V(RTYPE, psrc);                                    \
    psrc += stride;                                              \
  }
#define LD_UB5(...) LD_V5(v16u8, __VA_ARGS__)

#define LD_V6(RTYPE, psrc, stride, out0, out1, out2, out3, out4, out5) \
  {                                                                    \
    LD_V4(RTYPE, psrc, stride, out0, out1, out2, out3);                \
    LD_V2(RTYPE, psrc, stride, out4, out5);                            \
  }
#define LD_UB6(...) LD_V6(v16u8, __VA_ARGS__)
#define LD_UH6(...) LD_V6(v8u16, __VA_ARGS__)
#define LD_SP6(...) LD_V6(v4f32, __VA_ARGS__)

#define LD_V7(RTYPE, psrc, stride, out0, out1, out2, out3, out4, out5, out6) \
  {                                                                          \
    LD_V5(RTYPE, psrc, stride, out0, out1, out2, out3, out4);                \
    LD_V2(RTYPE, psrc, stride, out5, out6);                                  \
  }
#define LD_UB7(...) LD_V7(v16u8, __VA_ARGS__)

#define LD_V8(RTYPE, psrc, stride, out0, out1, out2, out3, out4, out5, out6, \
              out7)                                                          \
  {                                                                          \
    LD_V4(RTYPE, psrc, stride, out0, out1, out2, out3);                      \
    LD_V4(RTYPE, psrc, stride, out4, out5, out6, out7);                      \
  }
#define LD_UB8(...) LD_V8(v16u8, __VA_ARGS__)
#define LD_UH8(...) LD_V8(v8u16, __VA_ARGS__)
#define LD_SP8(...) LD_V8(v4f32, __VA_ARGS__)
#define LD_DP8(...) LD_V8(v2f64, __VA_ARGS__)

/* Description : Store vectors of elements with stride
 * Arguments   : Inputs - in0, in1, pdst, stride
 * Details     : Store elements from 'in0' to (pdst)
 *               Store elements from 'in1' to (pdst + stride)
 */
#define ST_V2(RTYPE, in0, in1, pdst, stride) \
  {                                          \
    ST_V(RTYPE, in0, pdst);                  \
    pdst += stride;                          \
    ST_V(RTYPE, in1, pdst);                  \
    pdst += stride;                          \
  }
#define ST_UB2(...) ST_V2(v16u8, __VA_ARGS__)
#define ST_UH2(...) ST_V2(v8u16, __VA_ARGS__)
#define ST_SP2(...) ST_V2(v4f32, __VA_ARGS__)

#define ST_V3(RTYPE, in0, in1, in2, pdst, stride) \
  {                                               \
    ST_V2(RTYPE, in0, in1, pdst, stride);         \
    ST_V(RTYPE, in2, pdst);                       \
    pdst += stride;                               \
  }
#define ST_UB3(...) ST_V3(v16u8, __VA_ARGS__)
#define ST_UH3(...) ST_V3(v8u16, __VA_ARGS__)

#define ST_V4(RTYPE, in0, in1, in2, in3, pdst, stride) \
  {                                                    \
    ST_V2(RTYPE, in0, in1, pdst, stride);              \
    ST_V2(RTYPE, in2, in3, pdst, stride);              \
  }
#define ST_UB4(...) ST_V4(v16u8, __VA_ARGS__)
#define ST_UH4(...) ST_V4(v8u16, __VA_ARGS__)
#define ST_SP4(...) ST_V4(v4f32, __VA_ARGS__)

#define ST_V6(RTYPE, in0, in1, in2, in3, in4, in5, pdst, stride) \
  {                                                              \
    ST_V3(RTYPE, in0, in1, in2, pdst, stride);                   \
    ST_V3(RTYPE, in3, in4, in5, pdst, stride);                   \
  }
#define ST_UB6(...) ST_V6(v16u8, __VA_ARGS__)
#define ST_SP6(...) ST_V6(v4f32, __VA_ARGS__)

#define ST_V8(RTYPE, in0, in1, in2, in3, in4, in5, in6, in7, pdst, stride) \
  {                                                                        \
    ST_V4(RTYPE, in0, in1, in2, in3, pdst, stride);                        \
    ST_V4(RTYPE, in4, in5, in6, in7, pdst, stride);                        \
  }
#define ST_UB8(...) ST_V8(v16u8, __VA_ARGS__)
#define ST_SP8(...) ST_V8(v4f32, __VA_ARGS__)

/* Description : Store 8x1 byte block to destination memory from input vector
   Arguments   : Inputs - in, pdst
   Details     : Index 0 double word element from 'in' vector is copied to the
                 GP register and stored to (pdst)
*/
#define ST8x1_UB(in, pdst)                               \
  {                                                      \
    const uint64_t out0m = __msa_copy_s_d((v2i64)in, 0); \
    SD(out0m, pdst);                                     \
  }

/* Description : Logical and in0 and in1.
   Arguments   : Inputs  - in0, in1, in2, in3,
                 Outputs - out0, out1, out2, out3
                 Return Type - as per RTYPE
   Details     : Each unsigned word element from 'in0' vector is added with
                 each unsigned word element from 'in1' vector. Then the average
                 is calculated and written to 'out0'
*/
#define AND_V2(RTYPE, in0, in1, mask, out0, out1)       \
  {                                                     \
    out0 = (RTYPE)__msa_and_v((v16u8)in0, (v16u8)mask); \
    out1 = (RTYPE)__msa_and_v((v16u8)in1, (v16u8)mask); \
  }
#define AND_V2_UB(...) AND_V2(v16u8, __VA_ARGS__)

#define AND_V4(RTYPE, in0, in1, in2, in3, mask, out0, out1, out2, out3) \
  {                                                                     \
    AND_V2(RTYPE, in0, in1, mask, out0, out1);                          \
    AND_V2(RTYPE, in2, in3, mask, out2, out3);                          \
  }
#define AND_V4_UB(...) AND_V4(v16u8, __VA_ARGS__)

/* Description : Logical equate of input vectors with immediate value
   Arguments   : Inputs  - in0, in1, val
                 Outputs - in place operation
                 Return Type - as per RTYPE
   Details     : Each unsigned byte element from input vector 'in0' & 'in1' is
                 logically and'ed with immediate mask and the result
                 is stored in-place.
*/
#define CEQI_B2(RTYPE, in0, in1, val, out0, out1) \
  {                                               \
    out0 = CEQI_B(in0, val);                      \
    out1 = CEQI_B(in1, val);                      \
  }
#define CEQI_B2_UB(...) CEQI_B2(v16u8, __VA_ARGS__)

#define CEQI_B4(RTYPE, in0, in1, in2, in3, val, out0, out1, out2, out3) \
  {                                                                     \
    CEQI_B2(RTYPE, in0, in1, val, out0, out1);                          \
    CEQI_B2(RTYPE, in2, in3, val, out2, out3);                          \
  }
#define CEQI_B4_UB(...) CEQI_B4(v16u8, __VA_ARGS__)

/* Description : Immediate number of elements to slide
 * Arguments   : Inputs  - in0, in1, slide_val
 *               Outputs - out
 *               Return Type - as per RTYPE
 * Details     : Byte elements from 'in1' vector are slid into 'in0' by
 *               value specified in the 'slide_val'
 */
#define SLDI_B(RTYPE, in0, in1, slide_val) \
  (RTYPE) __msa_sldi_b((v16i8)in0, (v16i8)in1, slide_val)
#define SLDI_UB(...) SLDI_B(v16u8, __VA_ARGS__)
#define SLDI_D(...) SLDI_B(v2f64, __VA_ARGS__)

/* Description : Immediate number of elements to slide
   Arguments   : Inputs  - in0_0, in0_1, in1_0, in1_1, slide_val
                 Outputs - out0, out1
                 Return Type - as per RTYPE
   Details     : Byte elements from 'in0_0' vector are slid into 'in1_0' by
                 value specified in the 'slide_val'
*/
#define SLDI_B2(RTYPE, in0_0, in0_1, in1_0, in1_1, out0, out1, slide_val) \
  {                                                                       \
    out0 = SLDI_B(RTYPE, in0_0, in1_0, slide_val);                        \
    out1 = SLDI_B(RTYPE, in0_1, in1_1, slide_val);                        \
  }
#define SLDI_B2_UB(...) SLDI_B2(v16u8, __VA_ARGS__)

/* Description : Shuffle byte vector elements as per variable
   Arguments   : Inputs  - in0, in1, shf_val
                 Outputs - out0, out1
                 Return Type - as per RTYPE
   Details     : Byte elements from 'in0' & 'in1' are copied selectively to
                 'out0' as per control variable 'shf_val'.
*/
#define SHF_B2(RTYPE, in0, in1, shf_val)           \
  {                                                \
    in0 = (RTYPE)__msa_shf_b((v16i8)in0, shf_val); \
    in1 = (RTYPE)__msa_shf_b((v16i8)in1, shf_val); \
  }
#define SHF_B2_UB(...) SHF_B2(v16u8, __VA_ARGS__)
#define SHF_B2_UH(...) SHF_B2(v8u16, __VA_ARGS__)

#define SHF_B3(RTYPE, in0, in1, in2, shf_val)      \
  {                                                \
    SHF_B2(RTYPE, in0, in1, shf_val);              \
    in2 = (RTYPE)__msa_shf_b((v16i8)in2, shf_val); \
  }
#define SHF_B3_UB(...) SHF_B3(v16u8, __VA_ARGS__)
#define SHF_B3_UH(...) SHF_B3(v8u16, __VA_ARGS__)

#define SHF_B4(RTYPE, in0, in1, in2, in3, shf_val) \
  {                                                \
    SHF_B2(RTYPE, in0, in1, shf_val);              \
    SHF_B2(RTYPE, in2, in3, shf_val);              \
  }
#define SHF_B4_UB(...) SHF_B4(v16u8, __VA_ARGS__)
#define SHF_B4_UH(...) SHF_B4(v8u16, __VA_ARGS__)

/* Description : Shuffle byte vector elements as per mask vector
   Arguments   : Inputs  - in0, in1, in2, in3, mask0, mask1
                 Outputs - out0, out1
                 Return Type - as per RTYPE
   Details     : Byte elements from 'in0' & 'in1' are copied selectively to
                 'out0' as per control vector 'mask0'
*/
#define VSHF_B(RTYPE, in0, in1, mask) \
  (RTYPE) __msa_vshf_b((v16i8)mask, (v16i8)in1, (v16i8)in0);
#define VSHF_UB(...) VSHF_B(v16u8, __VA_ARGS__)

/* Description : Interleave even byte elements from vectors
   Arguments   : Inputs  - in0, in1, in2, in3
                 Outputs - out0, out1
                 Return Type - as per RTYPE
   Details     : Even byte elements of 'in0' and 'in1' are interleaved
                 and written to 'out0'
*/
#define ILVEV_B2(RTYPE, in0, in1, in2, in3, out0, out1)  \
  {                                                      \
    out0 = (RTYPE)__msa_ilvev_b((v16i8)in1, (v16i8)in0); \
    out1 = (RTYPE)__msa_ilvev_b((v16i8)in3, (v16i8)in2); \
  }
#define ILVEV_B2_UB(...) ILVEV_B2(v16u8, __VA_ARGS__)
#define ILVEV_B2_UH(...) ILVEV_B2(v8u16, __VA_ARGS__)

#define ILVEV_B3(RTYPE, in0, in1, in2, in3, in4, in5, out0, out1, out2) \
  {                                                                     \
    ILVEV_B2(RTYPE, in0, in1, in2, in3, out0, out1)                     \
    out2 = (RTYPE)__msa_ilvev_b((v16i8)in5, (v16i8)in4);                \
  }
#define ILVEV_B3_UH(...) ILVEV_B3(v8u16, __VA_ARGS__)

/* Description : Interleave even halfword elements from vectors
   Arguments   : Inputs  - in0, in1, in2, in3
                 Outputs - out0, out1
                 Return Type - as per RTYPE
   Details     : Even halfword elements of 'in0' and 'in1' are interleaved
                 and written to 'out0'
*/
#define ILVEV_H2(RTYPE, in0, in1, in2, in3, out0, out1)  \
  {                                                      \
    out0 = (RTYPE)__msa_ilvev_h((v8i16)in1, (v8i16)in0); \
    out1 = (RTYPE)__msa_ilvev_h((v8i16)in3, (v8i16)in2); \
  }
#define ILVEV_H2_UB(...) ILVEV_H2(v16u8, __VA_ARGS__)

/* Description : Interleave right half of double word elements from vectors
 * Arguments   : Inputs  - in0, in1, in2, in3
 *               Outputs - out0, out1
 *               Return Type - as per RTYPE
 * Details     : Right half of double word elements of 'in0' and 'in1' are
 *               interleaved and written to 'out0'.
 */
#define ILVR_D2(RTYPE, in0, in1, in2, in3, out0, out1)  \
  {                                                     \
    out0 = (RTYPE)__msa_ilvr_d((v2i64)in0, (v2i64)in1); \
    out1 = (RTYPE)__msa_ilvr_d((v2i64)in2, (v2i64)in3); \
  }
#define ILVR_D2_UB(...) ILVR_D2(v16u8, __VA_ARGS__)

#define ILVR_D3(RTYPE, in0, in1, in2, in3, in4, in5, out0, out1, out2) \
  {                                                                    \
    ILVR_D2(RTYPE, in0, in1, in2, in3, out0, out1);                    \
    out2 = (RTYPE)__msa_ilvr_d((v2i64)in4, (v2i64)in5);                \
  }
#define ILVR_D3_UB(...) ILVR_D3(v16u8, __VA_ARGS__)

#define ILVR_D4(RTYPE, in0, in1, in2, in3, in4, in5, in6, in7, out0, out1, \
                out2, out3)                                                \
  {                                                                        \
    ILVR_D2(RTYPE, in0, in1, in2, in3, out0, out1);                        \
    ILVR_D2(RTYPE, in4, in5, in6, in7, out2, out3);                        \
  }
#define ILVR_D4_UB(...) ILVR_D4(v16u8, __VA_ARGS__)

/* Description : Interleave both left and right half of input vectors
   Arguments   : Inputs  - in0, in1
                 Outputs - out0, out1
                 Return Type - as per RTYPE
   Details     : Right half of byte elements from 'in0' and 'in1' are
                 interleaved and written to 'out0'
*/
#define ILVRL_B2(RTYPE, in0, in1, out0, out1)           \
  {                                                     \
    out0 = (RTYPE)__msa_ilvr_b((v16i8)in0, (v16i8)in1); \
    out1 = (RTYPE)__msa_ilvl_b((v16i8)in0, (v16i8)in1); \
  }
#define ILVRL_B2_UB(...) ILVRL_B2(v16u8, __VA_ARGS__)

#define ILVRL_H2(RTYPE, in0, in1, out0, out1)           \
  {                                                     \
    out0 = (RTYPE)__msa_ilvr_h((v8i16)in0, (v8i16)in1); \
    out1 = (RTYPE)__msa_ilvl_h((v8i16)in0, (v8i16)in1); \
  }
#define ILVRL_H2_UB(...) ILVRL_H2(v16u8, __VA_ARGS__)

/* Description : Interleave both odd and even half of input vectors
   Arguments   : Inputs  - in0, in1
                 Outputs - out0, out1
                 Return Type - as per RTYPE
   Details     : Odd half of byte elements from 'in0' and 'in1' are
                 interleaved and written to 'out0'
*/
#define ILVODEV_B2(RTYPE, in0, in1, out0, out1)          \
  {                                                      \
    out0 = (RTYPE)__msa_ilvod_b((v16i8)in0, (v16i8)in1); \
    out1 = (RTYPE)__msa_ilvev_b((v16i8)in0, (v16i8)in1); \
  }
#define ILVODEV_B2_UB(...) ILVODEV_B2(v16u8, __VA_ARGS__)

/* Description : Pack even byte elements of vector pairs
 *  Arguments   : Inputs  - in0, in1, in2, in3
 *                Outputs - out0, out1
 *                Return Type - as per RTYPE
 *  Details     : Even byte elements of 'in0' are copied to the left half of
 *                'out0' & even byte elements of 'in1' are copied to the right
 *                half of 'out0'.
 */
#define PCKEV_B2(RTYPE, in0, in1, in2, in3, out0, out1)  \
  {                                                      \
    out0 = (RTYPE)__msa_pckev_b((v16i8)in0, (v16i8)in1); \
    out1 = (RTYPE)__msa_pckev_b((v16i8)in2, (v16i8)in3); \
  }
#define PCKEV_B2_UB(...) PCKEV_B2(v16u8, __VA_ARGS__)
#define PCKEV_B2_UH(...) PCKEV_B2(v8u16, __VA_ARGS__)

#define PCKEV_B3(RTYPE, in0, in1, in2, in3, in4, in5, out0, out1, out2) \
  {                                                                     \
    PCKEV_B2(RTYPE, in0, in1, in2, in3, out0, out1);                    \
    out2 = (RTYPE)__msa_pckev_b((v16i8)in4, (v16i8)in5);                \
  }
#define PCKEV_B3_UH(...) PCKEV_B3(v8u16, __VA_ARGS__)

#define PCKEV_B4(RTYPE, in0, in1, in2, in3, in4, in5, in6, in7, out0, out1, \
                 out2, out3)                                                \
  {                                                                         \
    PCKEV_B2(RTYPE, in0, in1, in2, in3, out0, out1);                        \
    PCKEV_B2(RTYPE, in4, in5, in6, in7, out2, out3);                        \
  }
#define PCKEV_B4_UH(...) PCKEV_B4(v8u16, __VA_ARGS__)

/* Description : Pack even halfword elements of vector pairs
   Arguments   : Inputs  - in0, in1, in2, in3
                 Outputs - out0, out1
                 Return Type - as per RTYPE
   Details     : Even halfword elements of 'in0' are copied to the left half of
                 'out0' & even halfword elements of 'in1' are copied to the
                 right half of 'out0'.
*/
#define PCKEV_H2(RTYPE, in0, in1, in2, in3, out0, out1)  \
  {                                                      \
    out0 = (RTYPE)__msa_pckev_h((v8i16)in0, (v8i16)in1); \
    out1 = (RTYPE)__msa_pckev_h((v8i16)in2, (v8i16)in3); \
  }
#define PCKEV_H2_UB(...) PCKEV_H2(v16u8, __VA_ARGS__)

#define PCKEV_H3(RTYPE, in0, in1, in2, in3, in4, in5, out0, out1, out2) \
  {                                                                     \
    PCKEV_H2(RTYPE, in0, in1, in2, in3, out0, out1);                    \
    out2 = (RTYPE)__msa_pckev_h((v8i16)in4, (v8i16)in5);                \
  }
#define PCKEV_H3_UB(...) PCKEV_H3(v16u8, __VA_ARGS__)

#define PCKEV_H4(RTYPE, in0, in1, in2, in3, in4, in5, in6, in7, out0, out1, \
                 out2, out3)                                                \
  {                                                                         \
    PCKEV_H2(RTYPE, in0, in1, in2, in3, out0, out1);                        \
    PCKEV_H2(RTYPE, in4, in5, in6, in7, out2, out3);                        \
  }
#define PCKEV_H4_UB(...) PCKEV_H4(v16u8, __VA_ARGS__)

/* Description : Pack odd halfword elements of vector pairs
   Arguments   : Inputs  - in0, in1, in2, in3
                 Outputs - out0, out1
                 Return Type - as per RTYPE
   Details     : Odd halfword elements of 'in0' are copied to the left half of
                 'out0' & odd halfword elements of 'in1' are copied to the
                 right half of 'out0'.
*/
#define PCKOD_H2(RTYPE, in0, in1, in2, in3, out0, out1)  \
  {                                                      \
    out0 = (RTYPE)__msa_pckod_h((v8i16)in0, (v8i16)in1); \
    out1 = (RTYPE)__msa_pckod_h((v8i16)in2, (v8i16)in3); \
  }
#define PCKOD_H2_UB(...) PCKOD_H2(v16u8, __VA_ARGS__)

#define PCKOD_H3(RTYPE, in0, in1, in2, in3, in4, in5, out0, out1, out2) \
  {                                                                     \
    PCKOD_H2(RTYPE, in0, in1, in2, in3, out0, out1);                    \
    out2 = (RTYPE)__msa_pckod_h((v8i16)in4, (v8i16)in5);                \
  }
#define PCKOD_H3_UB(...) PCKOD_H3(v16u8, __VA_ARGS__)

#define PCKOD_H4(RTYPE, in0, in1, in2, in3, in4, in5, in6, in7, out0, out1, \
                 out2, out3)                                                \
  {                                                                         \
    PCKOD_H2(RTYPE, in0, in1, in2, in3, out0, out1);                        \
    PCKOD_H2(RTYPE, in4, in5, in6, in7, out2, out3);                        \
  }
#define PCKOD_H4_UB(...) PCKOD_H4(v16u8, __VA_ARGS__)

/* Description : Logical shift right all elements of half-word vector
   Arguments   : Inputs  - in0, in1, shift
                 Outputs - in place operation
                 Return Type - as per input vector RTYPE
   Details     : Each element of vector 'in0' is right shifted by 'shift' and
                 the result is written in-place. 'shift' is a GP variable.
*/
#define SRLI_B2(RTYPE, in0, in1, shift_val) \
  {                                         \
    in0 = (RTYPE)SRLI_B(in0, shift_val);    \
    in1 = (RTYPE)SRLI_B(in1, shift_val);    \
  }
#define SRLI_B2_UB(...) SRLI_B2(v16u8, __VA_ARGS__)

#define SRLI_B3(RTYPE, in0, in1, in2, shift_val) \
  {                                              \
    SRLI_B2(RTYPE, in0, in1, shift_val);         \
    in2 = (RTYPE)SRLI_B(in2, shift_val);         \
  }
#define SRLI_B3_UB(...) SRLI_B3(v16u8, __VA_ARGS__)

#define SRLI_B4(RTYPE, in0, in1, in2, in3, shift_val) \
  {                                                   \
    SRLI_B2(RTYPE, in0, in1, shift_val);              \
    SRLI_B2(RTYPE, in2, in3, shift_val);              \
  }
#define SRLI_B4_UB(...) SRLI_B4(v16u8, __VA_ARGS__)

/* Description : Logical shift right all elements of vector (immediate)
   Arguments   : Inputs  - in0, in1, in2, in3, shift
                 Outputs - out0, out1, out2, out3
                 Return Type - as per RTYPE
   Details     : Each element of vector 'in0' is right shifted by 'shift' and
                 the result is written in 'out0'. 'shift' is an immediate value.
*/
#define SRLI_H2(RTYPE, in0, in1, out0, out1, shift) \
  {                                                 \
    out0 = (RTYPE)SRLI_H((v8i16)in0, shift);        \
    out1 = (RTYPE)SRLI_H((v8i16)in1, shift);        \
  }
#define SRLI_H2_UB(...) SRLI_H2(v16u8, __VA_ARGS__)

#define SRLI_H4(RTYPE, in0, in1, in2, in3, out0, out1, out2, out3, shift) \
  {                                                                       \
    SRLI_H2(RTYPE, in0, in1, out0, out1, shift);                          \
    SRLI_H2(RTYPE, in2, in3, out2, out3, shift);                          \
  }
#define SRLI_H4_UB(...) SRLI_H4(v16u8, __VA_ARGS__)

/* Description : Immediate Bit Insert Left (immediate)
   Arguments   : Inputs  - in0, in1, in2, in3, shift
                 Outputs - out0, out1
                 Return Type - as per RTYPE
   Details     : Copy most significant (left) bits in each element of vector
                 'in1' to elements in vector in0 while preserving the least
                 significant (right) bits. The number of bits to copy is given
                 by the immediate 'shift + 1'.
*/
#define BINSLI_B2(RTYPE, in0, in1, in2, in3, out0, out1, shift)  \
  {                                                              \
    out0 = (RTYPE)__msa_binsli_b((v16u8)in0, (v16u8)in1, shift); \
    out1 = (RTYPE)__msa_binsli_b((v16u8)in2, (v16u8)in3, shift); \
  }
#define BINSLI_B2_UB(...) BINSLI_B2(v16u8, __VA_ARGS__)

/* Description : Immediate Bit Insert Right (immediate)
   Arguments   : Inputs  - in0, in1, in2, in3, shift
                 Outputs - out0, out1
                 Return Type - as per RTYPE
   Details     : Copy least significant (right) bits in each element of vector
                 'in1' to elements in vector in0 while preserving the most
                 significant (left) bits. The number of bits to copy is given
                 by the immediate 'shift + 1'.
*/
#define BINSRI_B2(RTYPE, in0, in1, in2, in3, out0, out1, shift)  \
  {                                                              \
    out0 = (RTYPE)__msa_binsri_b((v16u8)in0, (v16u8)in1, shift); \
    out1 = (RTYPE)__msa_binsri_b((v16u8)in2, (v16u8)in3, shift); \
  }
#define BINSRI_B2_UB(...) BINSRI_B2(v16u8, __VA_ARGS__)

#define BINSRI_B3(RTYPE, in0, in1, in2, in3, in4, in5, out0, out1, out2, \
                  shift)                                                 \
  {                                                                      \
    BINSRI_B2(RTYPE, in0, in1, in2, in3, out0, out1, shift);             \
    out2 = (RTYPE)__msa_binsri_b((v16u8)in4, (v16u8)in5, shift);         \
  }
#define BINSRI_B3_UB(...) BINSRI_B3(v16u8, __VA_ARGS__)

/* Description : Multiplication of pairs of vectors
   Arguments   : Inputs  - in0, in1, in2, in3
                 Outputs - out0, out1
   Details     : Each element from 'in0' is multiplied with elements from 'in1'
                 and the result is written to 'out0'
*/
#define MUL2(in0, in1, in2, in3, out0, out1) \
  {                                          \
    out0 = in0 * in1;                        \
    out1 = in2 * in3;                        \
  }
#define MUL4(in0, in1, in2, in3, in4, in5, in6, in7, out0, out1, out2, out3) \
  {                                                                          \
    MUL2(in0, in1, in2, in3, out0, out1);                                    \
    MUL2(in4, in5, in6, in7, out2, out3);                                    \
  }

/* Description : Division of pairs of vectors
   Arguments   : Inputs  - in0, in1, in2, in3
                 Outputs - out0, out1
   Details     : Each element from 'in0' is divided by elements from 'in1'
                 and the result is written to 'out0'
*/
#define DIV2(in0, in1, in2, in3, out0, out1) \
  {                                          \
    out0 = in0 / in1;                        \
    out1 = in2 / in3;                        \
  }
#define DIV4(in0, in1, in2, in3, in4, in5, in6, in7, out0, out1, out2, out3) \
  {                                                                          \
    DIV2(in0, in1, in2, in3, out0, out1);                                    \
    DIV2(in4, in5, in6, in7, out2, out3);                                    \
  }

/* Description : Logical AND of 4 pairs of vectors with mask
   Arguments   : Inputs  - in0, in1, in2, in3, mask
                 Outputs - in0, in1, in2, in3
   Details     : Each element in 'in0' is logically AND'ed with mask
                 Each element in 'in1' is logically AND'ed with mask
                 Each element in 'in2' is logically AND'ed with mask
                 Each element in 'in3' is logically AND'ed with mask
*/
#define AND_W4(RTYPE, in0, in1, in2, in3, mask) \
  {                                             \
    in0 = (RTYPE)((v16i8)in0 & (v16i8)mask);    \
    in1 = (RTYPE)((v16i8)in1 & (v16i8)mask);    \
    in2 = (RTYPE)((v16i8)in2 & (v16i8)mask);    \
    in3 = (RTYPE)((v16i8)in3 & (v16i8)mask);    \
  }
#define AND_W4_SP(...) AND_W4(v4f32, __VA_ARGS__)

/* Description : Addition of 2 pairs of vectors
   Arguments   : Inputs  - in0, in1, in2, in3
                 Outputs - out0, out1
   Details     : Each element in 'in0' is added to 'in1' and result is written
                 to 'out0'
                 Each element in 'in2' is added to 'in3' and result is written
                 to 'out1'
*/
#define ADD2(in0, in1, in2, in3, out0, out1) \
  {                                          \
    out0 = in0 + in1;                        \
    out1 = in2 + in3;                        \
  }

/* Description : Addition of 4 pairs of vectors
   Arguments   : Inputs  - in0, in1, in2, in3, in4, in5, in6, in7
                 Outputs - out0, out1
   Details     : Each element in 'in0' is added to 'in1' and result is written
                 to 'out0'
                 Each element in 'in2' is added to 'in3' and result is written
                 to 'out1'
                 Each element in 'in4' is added to 'in5' and result is written
                 to 'out2'
                 Each element in 'in6' is added to 'in7' and result is written
                 to 'out3'
*/
#define ADD4(in0, in1, in2, in3, in4, in5, in6, in7, out0, out1, out2, out3) \
  {                                                                          \
    ADD2(in0, in1, in2, in3, out0, out1);                                    \
    ADD2(in4, in5, in6, in7, out2, out3);                                    \
  }

/* Description : Vector Floating-Point Convert from Unsigned Integer
   Arguments   : Inputs  - in0, in1
                 Outputs - out0, out1
*/
#define FFINTU_W2(RTYPE, in0, in1, out0, out1) \
  {                                            \
    out0 = (RTYPE)__msa_ffint_u_w((v4u32)in0); \
    out1 = (RTYPE)__msa_ffint_u_w((v4u32)in1); \
  }
#define FFINTU_W2_SP(...) FFINTU_W2(v4f32, __VA_ARGS__)

/* Description : Vector Floating-Point Convert from Unsigned Integer
   Arguments   : Inputs  - in0, in1, in2, in3
                 Outputs - out0, out1, out2, out3
*/
#define FFINTU_W4(RTYPE, in0, in1, in2, in3, out0, out1, out2, out3) \
  {                                                                  \
    FFINTU_W2(RTYPE, in0, in1, out0, out1);                          \
    FFINTU_W2(RTYPE, in2, in3, out2, out3);                          \
  }
#define FFINTU_W4_SP(...) FFINTU_W4(v4f32, __VA_ARGS__)

/* Description : Vector Floating-Point Truncate and Convert to Unsigned Integer
   Arguments   : Inputs  - in0, in1
                 Outputs - out0, out1
*/
#define FTRUNCU_W2(RTYPE, in0, in1, out0, out1) \
  {                                             \
    out0 = (RTYPE)__msa_ftrunc_u_w((v4f32)in0); \
    out1 = (RTYPE)__msa_ftrunc_u_w((v4f32)in1); \
  }
#define FTRUNCU_W2_UB(...) FTRUNCU_W2(v16u8, __VA_ARGS__)

/* Description : Vector Floating-Point Truncate and Convert to Unsigned Integer
   Arguments   : Inputs  - in0, in1, in2, in3
                 Outputs - out0, out1, out2, out3
*/
#define FTRUNCU_W4(RTYPE, in0, in1, in2, in3, out0, out1, out2, out3) \
  {                                                                   \
    FTRUNCU_W2(RTYPE, in0, in1, out0, out1);                          \
    FTRUNCU_W2(RTYPE, in2, in3, out2, out3);                          \
  }
#define FTRUNCU_W4_UB(...) FTRUNCU_W4(v16u8, __VA_ARGS__)

/* Description : Vector Floating-Point multiply with scale and accumulate
   Arguments   : Inputs  - in0, in1, in2, in3, out0, out1, out2, out3, scale
                 Outputs - out0, out1, out2, out3
*/
#define VSMA4(in0, in1, in2, in3, out0, out1, out2, out3, scale) \
  {                                                              \
    out0 += in0 * scale;                                         \
    out1 += in1 * scale;                                         \
    out2 += in2 * scale;                                         \
    out3 += in3 * scale;                                         \
  }

/* Description : Vector Floating-Point multiply with scale
   Arguments   : Inputs  - in0, in1, in2, in3, scale
                 Outputs - out0, out1, out2, out3
*/
#define VSMUL4(in0, in1, in2, in3, out0, out1, out2, out3, scale) \
  {                                                               \
    out0 = in0 * scale;                                           \
    out1 = in1 * scale;                                           \
    out2 = in2 * scale;                                           \
    out3 = in3 * scale;                                           \
  }

/* Description : Vector Floating-Point max value
   Arguments   : Inputs - in0, in1, in2, in3, max
                 Output - max
*/
#define VMAX_W4(RTYPE, in0, in1, in2, in3, max)        \
  {                                                    \
    max = (RTYPE)__msa_fmax_w((v4f32)max, (v4f32)in0); \
    max = (RTYPE)__msa_fmax_w((v4f32)max, (v4f32)in1); \
    max = (RTYPE)__msa_fmax_w((v4f32)max, (v4f32)in2); \
    max = (RTYPE)__msa_fmax_w((v4f32)max, (v4f32)in3); \
  }
#define VMAX_W4_SP(...) VMAX_W4(v4f32, __VA_ARGS__)

/* Description : Vector Floating-Point clip to min max
   Arguments   : Inputs  - in0, in1, in2, in3, min, max
                 Outputs - out0, out1, out2, out3
*/
#define VCLIP4(in0, in1, in2, in3, min, max, out0, out1, out2, out3) \
  {                                                                  \
    out0 = __msa_fmax_w(__msa_fmin_w(in0, max), min);                \
    out1 = __msa_fmax_w(__msa_fmin_w(in1, max), min);                \
    out2 = __msa_fmax_w(__msa_fmin_w(in2, max), min);                \
    out3 = __msa_fmax_w(__msa_fmin_w(in3, max), min);                \
  }

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_CPU_MIPS_COMMON_MACROS_MSA_H_
