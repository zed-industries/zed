/*
 * Copyright © 2020 Loongson Technology Co. Ltd.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the next
 * paragraph) shall be included in all copies or substantial portions of the
 * Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 *
 * Author:  Yin Shiyou (yinshiyou-hf@loongson.cn)
 *          Gu  Xiwei  (guxiwei-hf@loongson.cn)
 */

/*
 * This header file is copied from loongson LSOM project.
 * MSA macros is implemented with msa intrinsics in msa.h,
 * and used for simplifing MSA optimization.
 */

#ifndef _MSA_MACROS_H
#define _MSA_MACROS_H 1
#define MSA_MACROS_VERSION 18
#include <msa.h>

#if (__mips_isa_rev >= 6)
    #define LH(psrc)                              \
    ( {                                           \
        uint16_t val_lh_m = *(uint16_t *)(psrc);  \
        val_lh_m;                                 \
    } )

    #define LW(psrc)                              \
    ( {                                           \
        uint32_t val_lw_m = *(uint32_t *)(psrc);  \
        val_lw_m;                                 \
    } )

    #if (__mips == 64)
        #define LD(psrc)                               \
        ( {                                            \
            uint64_t val_ld_m =  *(uint64_t *)(psrc);  \
            val_ld_m;                                  \
        } )
    #else  // !(__mips == 64)
        #define LD(psrc)                                                    \
        ( {                                                                 \
            uint8_t *psrc_ld_m = (uint8_t *) (psrc);                        \
            uint32_t val0_ld_m, val1_ld_m;                                  \
            uint64_t val_ld_m = 0;                                          \
                                                                            \
            val0_ld_m = LW(psrc_ld_m);                                      \
            val1_ld_m = LW(psrc_ld_m + 4);                                  \
                                                                            \
            val_ld_m = (uint64_t) (val1_ld_m);                              \
            val_ld_m = (uint64_t) ((val_ld_m << 32) & 0xFFFFFFFF00000000);  \
            val_ld_m = (uint64_t) (val_ld_m | (uint64_t) val0_ld_m);        \
                                                                            \
            val_ld_m;                                                       \
        } )
    #endif  // (__mips == 64)

    #define SH(val, pdst)  *(uint16_t *)(pdst) = (val);
    #define SW(val, pdst)  *(uint32_t *)(pdst) = (val);
    #define SD(val, pdst)  *(uint64_t *)(pdst) = (val);

#else  // !(__mips_isa_rev >= 6)
    #define LH(psrc)                                 \
    ( {                                              \
        uint8_t *psrc_lh_m = (uint8_t *) (psrc);     \
        uint16_t val_lh_m;                           \
                                                     \
        __asm__ volatile (                           \
            "ulh  %[val_lh_m],  %[psrc_lh_m]  \n\t"  \
                                                     \
            : [val_lh_m] "=r" (val_lh_m)             \
            : [psrc_lh_m] "m" (*psrc_lh_m)           \
        );                                           \
                                                     \
        val_lh_m;                                    \
    } )

    #define LW(psrc)                                 \
    ( {                                              \
        uint8_t *psrc_lw_m = (uint8_t *) (psrc);     \
        uint32_t val_lw_m;                           \
                                                     \
        __asm__ volatile (                           \
            "ulw  %[val_lw_m],  %[psrc_lw_m]  \n\t"  \
                                                     \
            : [val_lw_m] "=r" (val_lw_m)             \
            : [psrc_lw_m] "m" (*psrc_lw_m)           \
        );                                           \
                                                     \
        val_lw_m;                                    \
    } )

    #if (__mips == 64)
        #define LD(psrc)                                 \
        ( {                                              \
            uint8_t *psrc_ld_m = (uint8_t *) (psrc);     \
            uint64_t val_ld_m = 0;                       \
                                                         \
            __asm__ volatile (                           \
                "uld  %[val_ld_m],  %[psrc_ld_m]  \n\t"  \
                                                         \
                : [val_ld_m] "=r" (val_ld_m)             \
                : [psrc_ld_m] "m" (*psrc_ld_m)           \
            );                                           \
                                                         \
            val_ld_m;                                    \
        } )
    #else  // !(__mips == 64)
        #define LD(psrc)                                                    \
        ( {                                                                 \
            uint8_t *psrc_ld_m = (uint8_t *) (psrc);                        \
            uint32_t val0_ld_m, val1_ld_m;                                  \
            uint64_t val_ld_m = 0;                                          \
                                                                            \
            val0_ld_m = LW(psrc_ld_m);                                      \
            val1_ld_m = LW(psrc_ld_m + 4);                                  \
                                                                            \
            val_ld_m = (uint64_t) (val1_ld_m);                              \
            val_ld_m = (uint64_t) ((val_ld_m << 32) & 0xFFFFFFFF00000000);  \
            val_ld_m = (uint64_t) (val_ld_m | (uint64_t) val0_ld_m);        \
                                                                            \
            val_ld_m;                                                       \
        } )
    #endif  // (__mips == 64)

    #define SH(val, pdst)                            \
    {                                                \
        uint8_t *pdst_sh_m = (uint8_t *) (pdst);     \
        uint16_t val_sh_m = (val);                   \
                                                     \
        __asm__ volatile (                           \
            "ush  %[val_sh_m],  %[pdst_sh_m]  \n\t"  \
                                                     \
            : [pdst_sh_m] "=m" (*pdst_sh_m)          \
            : [val_sh_m] "r" (val_sh_m)              \
        );                                           \
    }

    #define SW(val, pdst)                            \
    {                                                \
        uint8_t *pdst_sw_m = (uint8_t *) (pdst);     \
        uint32_t val_sw_m = (val);                   \
                                                     \
        __asm__ volatile (                           \
            "usw  %[val_sw_m],  %[pdst_sw_m]  \n\t"  \
                                                     \
            : [pdst_sw_m] "=m" (*pdst_sw_m)          \
            : [val_sw_m] "r" (val_sw_m)              \
        );                                           \
    }

    #define SD(val, pdst)                                             \
    {                                                                 \
        uint8_t *pdst_sd_m = (uint8_t *) (pdst);                      \
        uint32_t val0_sd_m, val1_sd_m;                                \
                                                                      \
        val0_sd_m = (uint32_t) ((val) & 0x00000000FFFFFFFF);          \
        val1_sd_m = (uint32_t) (((val) >> 32) & 0x00000000FFFFFFFF);  \
                                                                      \
        SW(val0_sd_m, pdst_sd_m);                                     \
        SW(val1_sd_m, pdst_sd_m + 4);                                 \
    }
#endif // (__mips_isa_rev >= 6)






/* Description : Load vector elements with stride.
 * Arguments   : Inputs  - psrc    (source pointer to load from)
 *                       - stride
 *               Outputs - out0, out1...
 *               Return Type - as per RTYPE
 * Details     : Loads elements in 'out0' from (psrc).
 *               Loads elements in 'out1' from (psrc + stride).
 */
#define MSA_LD_V(RTYPE, psrc, out) (out) = *((RTYPE *)(psrc));

#define MSA_LD_V2(RTYPE, psrc, stride, out0, out1)  \
{                                                   \
    MSA_LD_V(RTYPE, (psrc), out0);                  \
    MSA_LD_V(RTYPE, (psrc) + (stride), out1);       \
}

#define MSA_LD_V4(RTYPE, psrc, stride, out0, out1, out2, out3)     \
{                                                                  \
    MSA_LD_V2(RTYPE, (psrc), stride, out0, out1);                  \
    MSA_LD_V2(RTYPE, (psrc) + 2 * (stride) , stride, out2, out3);  \
}

#define MSA_LD_V8(RTYPE, psrc, stride, out0, out1, out2, out3,                \
                  out4, out5, out6, out7)                                     \
{                                                                             \
    MSA_LD_V4(RTYPE, (psrc), stride, out0, out1, out2, out3);                 \
    MSA_LD_V4(RTYPE, (psrc) + 4 * (stride), stride, out4, out5, out6, out7);  \
}

/* Description : Store vectors with stride.
 * Arguments   : Inputs  - in0, in1...  (source vector to be stored)
 *                       - stride
 *               Outputs - pdst    (destination pointer to store to)
 * Details     : Stores elements from 'in0' to (pdst).
 *               Stores elements from 'in1' to (pdst + stride).
 */
#define MSA_ST_V(RTYPE, in, pdst) *((RTYPE *)(pdst)) = (in);

#define MSA_ST_V2(RTYPE, in0, in1, pdst, stride)  \
{                                                 \
    MSA_ST_V(RTYPE, in0, (pdst));                 \
    MSA_ST_V(RTYPE, in1, (pdst) + (stride));      \
}

#define MSA_ST_V4(RTYPE, in0, in1, in2, in3, pdst, stride)      \
{                                                               \
    MSA_ST_V2(RTYPE, in0, in1, (pdst), stride);                 \
    MSA_ST_V2(RTYPE, in2, in3, (pdst) + 2 * (stride), stride);  \
}

#define MSA_ST_V8(RTYPE, in0, in1, in2, in3, in4, in5, in6, in7, pdst, stride) \
{                                                                              \
    MSA_ST_V4(RTYPE, in0, in1, in2, in3, (pdst), stride);                      \
    MSA_ST_V4(RTYPE, in4, in5, in6, in7, (pdst) + 4 * (stride), stride);       \
}

/* Description : Store half word elements of vector with stride.
 * Arguments   : Inputs  - in      (source vector)
 *                       - pdst    (destination pointer to store to)
 *                       - stride
 * Details     : Stores half word 'idx0' from 'in' to (pdst).
 *               Stores half word 'idx1' from 'in' to (pdst + stride).
 *               Similar for other elements.
 */
#define MSA_ST_H(in, idx, pdst)                          \
{                                                        \
    uint16_t out0_m;                                     \
    out0_m = __msa_copy_u_h((v8i16) in, idx);            \
    SH(out0_m, (pdst));                                  \
}
#define MSA_ST_H2(in, idx0, idx1, pdst, stride)          \
{                                                        \
    uint16_t out0_m, out1_m;                             \
    out0_m = __msa_copy_u_h((v8i16) in, idx0);           \
    out1_m = __msa_copy_u_h((v8i16) in, idx1);           \
    SH(out0_m, (pdst));                                  \
    SH(out1_m, (pdst) + stride);                         \
}
#define MSA_ST_H4(in, idx0, idx1, idx2, idx3, pdst, stride)          \
{                                                                    \
    uint16_t out0_m, out1_m, out2_m, out3_m;                         \
    out0_m = __msa_copy_u_h((v8i16) in, idx0);                       \
    out1_m = __msa_copy_u_h((v8i16) in, idx1);                       \
    out2_m = __msa_copy_u_h((v8i16) in, idx2);                       \
    out3_m = __msa_copy_u_h((v8i16) in, idx3);                       \
    SH(out0_m, (pdst));                                              \
    SH(out1_m, (pdst) + stride);                                     \
    SH(out2_m, (pdst) + 2 * stride);                                 \
    SH(out3_m, (pdst) + 3 * stride);                                 \
}
#define MSA_ST_H8(in, idx0, idx1, idx2, idx3, idx4, idx5,            \
              idx6, idx7, pdst, stride)                              \
{                                                                    \
    MSA_ST_H4(in, idx0, idx1, idx2, idx3, pdst, stride)              \
    MSA_ST_H4(in, idx4, idx5, idx6, idx7, (pdst) + 4*stride, stride) \
}

/* Description : Store word elements of vector with stride.
 * Arguments   : Inputs  - in      (source vector)
 *                       - pdst    (destination pointer to store to)
 *                       - stride
 * Details     : Stores word 'idx0' from 'in' to (pdst).
 *               Stores word 'idx1' from 'in' to (pdst + stride).
 *               Similar for other elements.
 */
#define MSA_ST_W(in, idx, pdst)                          \
{                                                        \
    uint32_t out0_m;                                     \
    out0_m = __msa_copy_u_w((v4i32) in, idx);            \
    SW(out0_m, (pdst));                                  \
}
#define MSA_ST_W2(in, idx0, idx1, pdst, stride)          \
{                                                        \
    uint32_t out0_m, out1_m;                             \
    out0_m = __msa_copy_u_w((v4i32) in, idx0);           \
    out1_m = __msa_copy_u_w((v4i32) in, idx1);           \
    SW(out0_m, (pdst));                                  \
    SW(out1_m, (pdst) + stride);                         \
}
#define MSA_ST_W4(in, idx0, idx1, idx2, idx3, pdst, stride)         \
{                                                                   \
    uint32_t out0_m, out1_m, out2_m, out3_m;                        \
    out0_m = __msa_copy_u_w((v4i32) in, idx0);                      \
    out1_m = __msa_copy_u_w((v4i32) in, idx1);                      \
    out2_m = __msa_copy_u_w((v4i32) in, idx2);                      \
    out3_m = __msa_copy_u_w((v4i32) in, idx3);                      \
    SW(out0_m, (pdst));                                             \
    SW(out1_m, (pdst) + stride);                                    \
    SW(out2_m, (pdst) + 2*stride);                                  \
    SW(out3_m, (pdst) + 3*stride);                                  \
}
#define MSA_ST_W8(in0, in1, idx0, idx1, idx2, idx3,                 \
              idx4, idx5, idx6, idx7, pdst, stride)                 \
{                                                                   \
    MSA_ST_W4(in0, idx0, idx1, idx2, idx3, pdst, stride)            \
    MSA_ST_W4(in1, idx4, idx5, idx6, idx7, pdst + 4*stride, stride) \
}

/* Description : Store double word elements of vector with stride.
 * Arguments   : Inputs  - in      (source vector)
 *                       - pdst    (destination pointer to store to)
 *                       - stride
 * Details     : Stores double word 'idx0' from 'in' to (pdst).
 *               Stores double word 'idx1' from 'in' to (pdst + stride).
 *               Similar for other elements.
 */
#define MSA_ST_D(in, idx, pdst)                    \
{                                                  \
    uint64_t out0_m;                               \
    out0_m = __msa_copy_u_d((v2i64) in, idx);      \
    SD(out0_m, (pdst));                            \
}
#define MSA_ST_D2(in, idx0, idx1, pdst, stride)    \
{                                                  \
    uint64_t out0_m, out1_m;                       \
    out0_m = __msa_copy_u_d((v2i64) in, idx0);     \
    out1_m = __msa_copy_u_d((v2i64) in, idx1);     \
    SD(out0_m, (pdst));                            \
    SD(out1_m, (pdst) + stride);                   \
}
#define MSA_ST_D4(in0, in1, idx0, idx1, idx2, idx3, pdst, stride)          \
{                                                                          \
    uint64_t out0_m, out1_m, out2_m, out3_m;                               \
    out0_m = __msa_copy_u_d((v2i64) in0, idx0);                            \
    out1_m = __msa_copy_u_d((v2i64) in0, idx1);                            \
    out2_m = __msa_copy_u_d((v2i64) in1, idx2);                            \
    out3_m = __msa_copy_u_d((v2i64) in1, idx3);                            \
    SD(out0_m, (pdst));                                                    \
    SD(out1_m, (pdst) + stride);                                           \
    SD(out2_m, (pdst) + 2 * stride);                                       \
    SD(out3_m, (pdst) + 3 * stride);                                       \
}
#define MSA_ST_D8(in0, in1, in2, in3, idx0, idx1, idx2, idx3,              \
              idx4, idx5, idx6, idx7, pdst, stride)                        \
{                                                                          \
    MSA_ST_D4(in0, in1, idx0, idx1, idx2, idx3, pdst, stride)              \
    MSA_ST_D4(in2, in3, idx4, idx5, idx6, idx7, pdst + 4 * stride, stride) \
}

/* Description : Shuffle byte vector elements as per mask vector.
 * Arguments   : Inputs  - in0, in1  (source vectors)
 *                       - mask      (mask vectors)
 *               Outputs - out       (dstination vectors)
 *               Return Type - as per RTYPE
 * Details     : Selective byte elements from 'in0' & 'in1' are copied to 'out' as
 *               per control vector 'mask'.
 */
#define MSA_VSHF_B(RTYPE, in0, in1, mask, out)                             \
{                                                                          \
    out = (RTYPE) __msa_vshf_b((v16i8) mask, (v16i8) in0, (v16i8) in1);    \
}

#define MSA_VSHF_B2(RTYPE, in0, in1, in2, in3, mask0, mask1, out0, out1)   \
{                                                                          \
    MSA_VSHF_B(RTYPE, in0, in1, mask0, out0)                               \
    MSA_VSHF_B(RTYPE, in2, in3, mask1, out1)                               \
}

#define MSA_VSHF_B4(RTYPE, in0, in1, in2, in3, in4, in5, in6, in7,         \
                    mask0, mask1, mask2, mask3, out0, out1, out2, out3)    \
{                                                                          \
    MSA_VSHF_B2(RTYPE, in0, in1, in2, in3, mask0, mask1, out0, out1);      \
    MSA_VSHF_B2(RTYPE, in4, in5, in6, in7, mask2, mask3, out2, out3);      \
}

/* Description : Shuffle halfword vector elements as per mask vector.
 * Arguments   : Inputs  - in0, in1  (source vectors)
 *                       - mask      (mask vectors)
 *               Outputs - out       (dstination vectors)
 *               Return Type - as per RTYPE
 * Details     : Selective halfword elements from 'in0' & 'in1' are copied to 'out' as
 *               per control vector 'mask'.
 */
#define MSA_VSHF_H(RTYPE, in0, in1, mask, out)                             \
{                                                                          \
    out = (RTYPE) __msa_vshf_h((v8i16) mask, (v8i16) in0, (v8i16) in1);    \
}

#define MSA_VSHF_H2(RTYPE, in0, in1, in2, in3, mask0, mask1, out0, out1)   \
{                                                                          \
    MSA_VSHF_H(RTYPE, in0, in1, mask0, out0)                               \
    MSA_VSHF_H(RTYPE, in2, in3, mask1, out1)                               \
}

#define MSA_VSHF_H4(RTYPE, in0, in1, in2, in3, in4, in5, in6, in7,         \
                    mask0, mask1, mask2, mask3, out0, out1, out2, out3)    \
{                                                                          \
    MSA_VSHF_H2(RTYPE, in0, in1, in2, in3, mask0, mask1, out0, out1);      \
    MSA_VSHF_H2(RTYPE, in4, in5, in6, in7, mask2, mask3, out2, out3);      \
}

/* Description : Shuffle word vector elements as per mask vector.
 * Arguments   : Inputs  - in0, in1  (source vectors)
 *                       - mask      (mask vectors)
 *               Outputs - out       (dstination vectors)
 *               Return Type - as per RTYPE
 * Details     : Selective word elements from 'in0' & 'in1' are copied to 'out' as
 *               per control vector 'mask'.
 */
#define MSA_VSHF_W(RTYPE, in0, in1, mask, out)                             \
{                                                                          \
    out = (RTYPE) __msa_vshf_w((v4i32) mask, (v4i32) in0, (v4i32) in1);    \
}

#define MSA_VSHF_W2(RTYPE, in0, in1, in2, in3, mask0, mask1, out0, out1)   \
{                                                                          \
    MSA_VSHF_W(RTYPE, in0, in1, mask0, out0)                               \
    MSA_VSHF_W(RTYPE, in2, in3, mask1, out1)                               \
}

#define MSA_VSHF_W4(RTYPE, in0, in1, in2, in3, in4, in5, in6, in7,         \
                    mask0, mask1, mask2, mask3, out0, out1, out2, out3)    \
{                                                                          \
    MSA_VSHF_W2(RTYPE, in0, in1, in2, in3, mask0, mask1, out0, out1);      \
    MSA_VSHF_W2(RTYPE, in4, in5, in6, in7, mask2, mask3, out2, out3);      \
}

/* Description : Interleave even byte elements from vectors.
 * Arguments   : Inputs  - in0, in1
 *               Outputs - out
 *               Return Type - as per RTYPE
 * Details     : Even byte elements of 'in0' and even byte
 *               elements of 'in1' are interleaved and copied to 'out'.
 */
#define MSA_ILVEV_B(RTYPE, in0, in1, out)                   \
{                                                           \
    out = (RTYPE) __msa_ilvev_b((v16i8) in0, (v16i8) in1);  \
}

#define MSA_ILVEV_B2(RTYPE, in0, in1, in2, in3, out0, out1)  \
{                                                            \
    MSA_ILVEV_B(RTYPE, in0, in1, out0);                      \
    MSA_ILVEV_B(RTYPE, in2, in3, out1);                      \
}

#define MSA_ILVEV_B4(RTYPE, in0, in1, in2, in3, in4, in5, in6, in7,  \
                     out0, out1, out2, out3)                         \
{                                                                    \
    MSA_ILVEV_B2(RTYPE, in0, in1, in2, in3, out0, out1);             \
    MSA_ILVEV_B2(RTYPE, in4, in5, in6, in7, out2, out3);             \
}

/* Description : Interleave even half word elements from vectors.
 * Arguments   : Inputs  - in0, in1
 *               Outputs - out
 *               Return Type - as per RTYPE
 * Details     : Even half word elements of 'in0' and even half word
 *               elements of 'in1' are interleaved and copied to 'out'.
 */
#define MSA_ILVEV_H(RTYPE, in0, in1, out)                   \
{                                                           \
    out = (RTYPE) __msa_ilvev_h((v8i16) in0, (v8i16) in1);  \
}

#define MSA_ILVEV_H2(RTYPE, in0, in1, in2, in3, out0, out1)  \
{                                                            \
    MSA_ILVEV_H(RTYPE, in0, in1, out0);                      \
    MSA_ILVEV_H(RTYPE, in2, in3, out1);                      \
}

#define MSA_ILVEV_H4(RTYPE, in0, in1, in2, in3, in4, in5, in6, in7,  \
                     out0, out1, out2, out3)                         \
{                                                                    \
    MSA_ILVEV_H2(RTYPE, in0, in1, in2, in3, out0, out1);             \
    MSA_ILVEV_H2(RTYPE, in4, in5, in6, in7, out2, out3);             \
}

/* Description : Interleave even word elements from vectors.
 * Arguments   : Inputs  - in0, in1
 *               Outputs - out
 *               Return Type - as per RTYPE
 * Details     : Even word elements of 'in0' and even word
 *               elements of 'in1' are interleaved and copied to 'out'.
 */
#define MSA_ILVEV_W(RTYPE, in0, in1, out)                   \
{                                                           \
    out = (RTYPE) __msa_ilvev_w((v2i64) in0, (v2i64) in1);  \
}

#define MSA_ILVEV_W2(RTYPE, in0, in1, in2, in3, out0, out1)  \
{                                                            \
    MSA_ILVEV_W(RTYPE, in0, in1, out0);                      \
    MSA_ILVEV_W(RTYPE, in2, in3, out1);                      \
}

#define MSA_ILVEV_W4(RTYPE, in0, in1, in2, in3, in4, in5, in6, in7,  \
                     out0, out1, out2, out3)                         \
{                                                                    \
    MSA_ILVEV_W2(RTYPE, in0, in1, in2, in3, out0, out1);             \
    MSA_ILVEV_W2(RTYPE, in4, in5, in6, in7, out2, out3);             \
}

/* Description : Interleave even double word elements from vectors.
 * Arguments   : Inputs  - in0, in1
 *               Outputs - out
 *               Return Type - as per RTYPE
 * Details     : Even double word elements of 'in0' and even double word
 *               elements of 'in1' are interleaved and copied to 'out'.
 */
#define MSA_ILVEV_D(RTYPE, in0, in1, out)                   \
{                                                           \
    out = (RTYPE) __msa_ilvev_d((v2i64) in0, (v2i64) in1);  \
}

#define MSA_ILVEV_D2(RTYPE, in0, in1, in2, in3, out0, out1)  \
{                                                            \
    MSA_ILVEV_D(RTYPE, in0, in1, out0);                      \
    MSA_ILVEV_D(RTYPE, in2, in3, out1);                      \
}

#define MSA_ILVEV_D4(RTYPE, in0, in1, in2, in3, in4, in5, in6, in7,  \
                     out0, out1, out2, out3)                         \
{                                                                    \
    MSA_ILVEV_D2(RTYPE, in0, in1, in2, in3, out0, out1);             \
    MSA_ILVEV_D2(RTYPE, in4, in5, in6, in7, out2, out3);             \
}

/* Description : Interleave odd byte elements from vectors.
 * Arguments   : Inputs  - in0, in1
 *               Outputs - out
 *               Return Type - as per RTYPE
 * Details     : Odd byte elements of 'in0' and odd byte
 *               elements of 'in1' are interleaved and copied to 'out'.
 */
#define MSA_ILVOD_B(RTYPE, in0, in1, out)                   \
{                                                           \
    out = (RTYPE) __msa_ilvod_b((v16i8) in0, (v16i8) in1);  \
}

#define MSA_ILVOD_B2(RTYPE, in0, in1, in2, in3, out0, out1)  \
{                                                            \
    MSA_ILVOD_B(RTYPE, in0, in1, out0);                      \
    MSA_ILVOD_B(RTYPE, in2, in3, out1);                      \
}

#define MSA_ILVOD_B4(RTYPE, in0, in1, in2, in3, in4, in5, in6, in7,  \
                     out0, out1, out2, out3)                         \
{                                                                    \
    MSA_ILVOD_B2(RTYPE, in0, in1, in2, in3, out0, out1);             \
    MSA_ILVOD_B2(RTYPE, in4, in5, in6, in7, out2, out3);             \
}

/* Description : Interleave odd half word elements from vectors.
 * Arguments   : Inputs  - in0, in1
 *               Outputs - out
 *               Return Type - as per RTYPE
 * Details     : Odd half word elements of 'in0' and odd half word
 *               elements of 'in1' are interleaved and copied to 'out'.
 */
#define MSA_ILVOD_H(RTYPE, in0, in1, out)                   \
{                                                           \
    out = (RTYPE) __msa_ilvod_h((v8i16) in0, (v8i16) in1);  \
}

#define MSA_ILVOD_H2(RTYPE, in0, in1, in2, in3, out0, out1)  \
{                                                            \
    MSA_ILVOD_H(RTYPE, in0, in1, out0);                      \
    MSA_ILVOD_H(RTYPE, in2, in3, out1);                      \
}

#define MSA_ILVOD_H4(RTYPE, in0, in1, in2, in3, in4, in5, in6, in7,  \
                     out0, out1, out2, out3)                         \
{                                                                    \
    MSA_ILVOD_H2(RTYPE, in0, in1, in2, in3, out0, out1);             \
    MSA_ILVOD_H2(RTYPE, in4, in5, in6, in7, out2, out3);             \
}

/* Description : Interleave odd word elements from vectors.
 * Arguments   : Inputs  - in0, in1
 *               Outputs - out
 *               Return Type - as per RTYPE
 * Details     : Odd word elements of 'in0' and odd word
 *               elements of 'in1' are interleaved and copied to 'out'.
 */
#define MSA_ILVOD_W(RTYPE, in0, in1, out)                   \
{                                                           \
    out = (RTYPE) __msa_ilvod_w((v4i32) in0, (v4i32) in1);  \
}

#define MSA_ILVOD_W2(RTYPE, in0, in1, in2, in3, out0, out1)  \
{                                                            \
    MSA_ILVOD_W(RTYPE, in0, in1, out0);                      \
    MSA_ILVOD_W(RTYPE, in2, in3, out1);                      \
}

#define MSA_ILVOD_W4(RTYPE, in0, in1, in2, in3, in4, in5, in6, in7,  \
                     out0, out1, out2, out3)                         \
{                                                                    \
    MSA_ILVOD_W2(RTYPE, in0, in1, in2, in3, out0, out1);             \
    MSA_ILVOD_W2(RTYPE, in4, in5, in6, in7, out2, out3);             \
}

/* Description : Interleave odd double word elements from vectors.
 * Arguments   : Inputs  - in0, in1
 *               Outputs - out
 *               Return Type - as per RTYPE
 * Details     : Odd double word elements of 'in0' and odd double word
 *               elements of 'in1' are interleaved and copied to 'out'.
 */
#define MSA_ILVOD_D(RTYPE, in0, in1, out)                   \
{                                                           \
    out = (RTYPE) __msa_ilvod_d((v2i64) in0, (v2i64) in1);  \
}

#define MSA_ILVOD_D2(RTYPE, in0, in1, in2, in3, out0, out1)  \
{                                                            \
    MSA_ILVOD_D(RTYPE, in0, in1, out0);                      \
    MSA_ILVOD_D(RTYPE, in2, in3, out1);                      \
}

#define MSA_ILVOD_D4(RTYPE, in0, in1, in2, in3, in4, in5, in6, in7,  \
                     out0, out1, out2, out3)                         \
{                                                                    \
    MSA_ILVOD_D2(RTYPE, in0, in1, in2, in3, out0, out1);             \
    MSA_ILVOD_D2(RTYPE, in4, in5, in6, in7, out2, out3);             \
}

/* Description : Interleave left half of byte elements from vectors.
 * Arguments   : Inputs  - in0, in1
 *               Outputs - out
 *               Return Type - as per RTYPE
 * Details     : Left half of byte elements of 'in0' and left half of byte
 *               elements of 'in1' are interleaved and copied to 'out'.
 */
#define MSA_ILVL_B(RTYPE, in0, in1, out)                   \
{                                                          \
    out = (RTYPE) __msa_ilvl_b((v16i8) in0, (v16i8) in1);  \
}

#define MSA_ILVL_B2(RTYPE, in0, in1, in2, in3, out0, out1)  \
{                                                           \
    MSA_ILVL_B(RTYPE, in0, in1, out0);                      \
    MSA_ILVL_B(RTYPE, in2, in3, out1);                      \
}

#define MSA_ILVL_B4(RTYPE, in0, in1, in2, in3, in4, in5, in6, in7,  \
                    out0, out1, out2, out3)                         \
{                                                                   \
    MSA_ILVL_B2(RTYPE, in0, in1, in2, in3, out0, out1);             \
    MSA_ILVL_B2(RTYPE, in4, in5, in6, in7, out2, out3);             \
}

/* Description : Interleave left half of halfword elements from vectors.
 * Arguments   : Inputs  - in0, in1
 *               Outputs - out
 *               Return Type - as per RTYPE
 * Details     : Left half of halfword elements of 'in0' and left half of halfword
 *               elements of 'in1' are interleaved and copied to 'out'.
 */
#define MSA_ILVL_H(RTYPE, in0, in1, out)                   \
{                                                          \
    out = (RTYPE) __msa_ilvl_h((v8i16) in0, (v8i16) in1);  \
}

#define MSA_ILVL_H2(RTYPE, in0, in1, in2, in3, out0, out1)  \
{                                                           \
    MSA_ILVL_H(RTYPE, in0, in1, out0);                      \
    MSA_ILVL_H(RTYPE, in2, in3, out1);                      \
}

#define MSA_ILVL_H4(RTYPE, in0, in1, in2, in3, in4, in5, in6, in7,  \
                    out0, out1, out2, out3)                         \
{                                                                   \
    MSA_ILVL_H2(RTYPE, in0, in1, in2, in3, out0, out1);             \
    MSA_ILVL_H2(RTYPE, in4, in5, in6, in7, out2, out3);             \
}

/* Description : Interleave left half of word elements from vectors.
 * Arguments   : Inputs  - in0, in1
 *               Outputs - out
 *               Return Type - as per RTYPE
 * Details     : Left half of word elements of 'in0' and left half of word
 *               elements of 'in1' are interleaved and copied to 'out'.
 */
#define MSA_ILVL_W(RTYPE, in0, in1, out)                   \
{                                                          \
    out = (RTYPE) __msa_ilvl_w((v4i32) in0, (v4i32) in1);  \
}

#define MSA_ILVL_W2(RTYPE, in0, in1, in2, in3, out0, out1)  \
{                                                           \
    MSA_ILVL_W(RTYPE, in0, in1, out0);                      \
    MSA_ILVL_W(RTYPE, in2, in3, out1);                      \
}

#define MSA_ILVL_W4(RTYPE, in0, in1, in2, in3, in4, in5, in6, in7,  \
                    out0, out1, out2, out3)                         \
{                                                                   \
    MSA_ILVL_W2(RTYPE, in0, in1, in2, in3, out0, out1);             \
    MSA_ILVL_W2(RTYPE, in4, in5, in6, in7, out2, out3);             \
}

/* Description : Interleave left half of double word elements from vectors.
 * Arguments   : Inputs  - in0, in1
 *               Outputs - out
 *               Return Type - as per RTYPE
 * Details     : Left half of double word elements of 'in0' and left half of
 *               double word elements of 'in1' are interleaved and copied to 'out'.
 */
#define MSA_ILVL_D(RTYPE, in0, in1, out)                   \
{                                                          \
    out = (RTYPE) __msa_ilvl_d((v2i64) in0, (v2i64) in1);  \
}

#define MSA_ILVL_D2(RTYPE, in0, in1, in2, in3, out0, out1)  \
{                                                           \
    MSA_ILVL_D(RTYPE, in0, in1, out0);                      \
    MSA_ILVL_D(RTYPE, in2, in3, out1);                      \
}

#define MSA_ILVL_D4(RTYPE, in0, in1, in2, in3, in4, in5, in6, in7,  \
                    out0, out1, out2, out3)                         \
{                                                                   \
    MSA_ILVL_D2(RTYPE, in0, in1, in2, in3, out0, out1);             \
    MSA_ILVL_D2(RTYPE, in4, in5, in6, in7, out2, out3);             \
}

/* Description : Interleave right half of byte elements from vectors.
 * Arguments   : Inputs  - in0, in1
 *               Outputs - out
 *               Return Type - as per RTYPE
 * Details     : Right half of byte elements of 'in0' and right half of byte
 *               elements of 'in1' are interleaved and copied to 'out'.
 */
#define MSA_ILVR_B(RTYPE, in0, in1, out)                   \
{                                                          \
    out = (RTYPE) __msa_ilvr_b((v16i8) in0, (v16i8) in1);  \
}

#define MSA_ILVR_B2(RTYPE, in0, in1, in2, in3, out0, out1)  \
{                                                           \
    MSA_ILVR_B(RTYPE, in0, in1, out0);                      \
    MSA_ILVR_B(RTYPE, in2, in3, out1);                      \
}

#define MSA_ILVR_B4(RTYPE, in0, in1, in2, in3, in4, in5, in6, in7,  \
                    out0, out1, out2, out3)                         \
{                                                                   \
    MSA_ILVR_B2(RTYPE, in0, in1, in2, in3, out0, out1);             \
    MSA_ILVR_B2(RTYPE, in4, in5, in6, in7, out2, out3);             \
}

/* Description : Interleave right half of halfword elements from vectors.
 * Arguments   : Inputs  - in0, in1
 *               Outputs - out
 *               Return Type - as per RTYPE
 * Details     : Right half of halfword elements of 'in0' and right half of halfword
 *               elements of 'in1' are interleaved and copied to 'out'.
 */
#define MSA_ILVR_H(RTYPE, in0, in1, out)                   \
{                                                          \
    out = (RTYPE) __msa_ilvr_h((v8i16) in0, (v8i16) in1);  \
}

#define MSA_ILVR_H2(RTYPE, in0, in1, in2, in3, out0, out1)  \
{                                                           \
    MSA_ILVR_H(RTYPE, in0, in1, out0);                      \
    MSA_ILVR_H(RTYPE, in2, in3, out1);                      \
}

#define MSA_ILVR_H4(RTYPE, in0, in1, in2, in3, in4, in5, in6, in7,  \
                    out0, out1, out2, out3)                         \
{                                                                   \
    MSA_ILVR_H2(RTYPE, in0, in1, in2, in3, out0, out1);             \
    MSA_ILVR_H2(RTYPE, in4, in5, in6, in7, out2, out3);             \
}

/* Description : Interleave right half of word elements from vectors.
 * Arguments   : Inputs  - in0, in1
 *               Outputs - out
 *               Return Type - as per RTYPE
 * Details     : Right half of word elements of 'in0' and right half of word
 *               elements of 'in1' are interleaved and copied to 'out'.
 */
#define MSA_ILVR_W(RTYPE, in0, in1, out)                   \
{                                                          \
    out = (RTYPE) __msa_ilvr_w((v4i32) in0, (v4i32) in1);  \
}

#define MSA_ILVR_W2(RTYPE, in0, in1, in2, in3, out0, out1)  \
{                                                           \
    MSA_ILVR_W(RTYPE, in0, in1, out0);                      \
    MSA_ILVR_W(RTYPE, in2, in3, out1);                      \
}

#define MSA_ILVR_W4(RTYPE, in0, in1, in2, in3, in4, in5, in6, in7,  \
                    out0, out1, out2, out3)                         \
{                                                                   \
    MSA_ILVR_W2(RTYPE, in0, in1, in2, in3, out0, out1);             \
    MSA_ILVR_W2(RTYPE, in4, in5, in6, in7, out2, out3);             \
}

/* Description : Interleave right half of double word elements from vectors.
 * Arguments   : Inputs  - in0, in1
 *               Outputs - out
 *               Return Type - as per RTYPE
 * Details     : Right half of double word elements of 'in0' and right half of
 *               double word elements of 'in1' are interleaved and copied to 'out'.
 */
#define MSA_ILVR_D(RTYPE, in0, in1, out)                   \
{                                                          \
    out = (RTYPE) __msa_ilvr_d((v2i64) in0, (v2i64) in1);  \
}

#define MSA_ILVR_D2(RTYPE, in0, in1, in2, in3, out0, out1)  \
{                                                           \
    MSA_ILVR_D(RTYPE, in0, in1, out0);                      \
    MSA_ILVR_D(RTYPE, in2, in3, out1);                      \
}

#define MSA_ILVR_D4(RTYPE, in0, in1, in2, in3, in4, in5, in6, in7,  \
                    out0, out1, out2, out3)                         \
{                                                                   \
    MSA_ILVR_D2(RTYPE, in0, in1, in2, in3, out0, out1);             \
    MSA_ILVR_D2(RTYPE, in4, in5, in6, in7, out2, out3);             \
}

/* Description : Interleave both left and right half of input vectors.
 * Arguments   : Inputs  - in0, in1
 *               Outputs - out0, out1
 *               Return Type - as per RTYPE
 * Details     : Right half of byte elements from 'in0' and 'in1' are
 *               interleaved and stored to 'out0'.
 *               Left half of byte elements from 'in0' and 'in1' are
 *               interleaved and stored to 'out1'.
 */
#define MSA_ILVRL_B2(RTYPE, in0, in1, out0, out1)  \
{                                                  \
    MSA_ILVR_B(RTYPE, in0, in1, out0);             \
    MSA_ILVL_B(RTYPE, in0, in1, out1);             \
}

#define MSA_ILVRL_B4(RTYPE, in0, in1, in2, in3,    \
                     out0, out1, out2, out3)       \
{                                                  \
    MSA_ILVRL_B2(RTYPE, in0, in1, out0, out1);     \
    MSA_ILVRL_B2(RTYPE, in2, in3, out2, out3);     \
}

/* Description : Interleave both left and right half of input vectors.
 * Arguments   : Inputs  - in0, in1
 *               Outputs - out0, out1
 *               Return Type - as per RTYPE
 * Details     : Right half of halfword elements from 'in0' and 'in1' are
 *               interleaved and stored to 'out0'.
 *               Left half of halfword elements from 'in0' and 'in1' are
 *               interleaved and stored to 'out1'.
 */
#define MSA_ILVRL_H2(RTYPE, in0, in1, out0, out1)  \
{                                                  \
    MSA_ILVR_H(RTYPE, in0, in1, out0);             \
    MSA_ILVL_H(RTYPE, in0, in1, out1);             \
}

#define MSA_ILVRL_H4(RTYPE, in0, in1, in2, in3,    \
                     out0, out1, out2, out3)       \
{                                                  \
    MSA_ILVRL_H2(RTYPE, in0, in1, out0, out1);     \
    MSA_ILVRL_H2(RTYPE, in2, in3, out2, out3);     \
}

/* Description : Interleave both left and right half of input vectors.
 * Arguments   : Inputs  - in0, in1
 *               Outputs - out0, out1
 *               Return Type - as per RTYPE
 * Details     : Right half of word elements from 'in0' and 'in1' are
 *               interleaved and stored to 'out0'.
 *               Left half of word elements from 'in0' and 'in1' are
 *               interleaved and stored to 'out1'.
 */
#define MSA_ILVRL_W2(RTYPE, in0, in1, out0, out1)  \
{                                                  \
    MSA_ILVR_W(RTYPE, in0, in1, out0);             \
    MSA_ILVL_W(RTYPE, in0, in1, out1);             \
}

#define MSA_ILVRL_W4(RTYPE, in0, in1, in2, in3,    \
                     out0, out1, out2, out3)       \
{                                                  \
    MSA_ILVRL_W2(RTYPE, in0, in1, out0, out1);     \
    MSA_ILVRL_W2(RTYPE, in2, in3, out2, out3);     \
}

/* Description : Interleave both left and right half of input vectors.
 * Arguments   : Inputs  - in0, in1
 *               Outputs - out0, out1
 *               Return Type - as per RTYPE
 * Details     : Right half of double word elements from 'in0' and 'in1' are
 *               interleaved and stored to 'out0'.
 *               Left half of double word elements from 'in0' and 'in1' are
 *               interleaved and stored to 'out1'.
 */
#define MSA_ILVRL_D2(RTYPE, in0, in1, out0, out1)  \
{                                                  \
    MSA_ILVR_D(RTYPE, in0, in1, out0);             \
    MSA_ILVL_D(RTYPE, in0, in1, out1);             \
}

#define MSA_ILVRL_D4(RTYPE, in0, in1, in2, in3,    \
                     out0, out1, out2, out3)       \
{                                                  \
    MSA_ILVRL_D2(RTYPE, in0, in1, out0, out1);     \
    MSA_ILVRL_D2(RTYPE, in2, in3, out2, out3);     \
}

/* Description : Indexed byte elements are replicated to all elements in
 *               output vector.
 * Arguments   : Inputs  - in, idx
 *               Outputs - out
 *               Return Type - as per RTYPE
 * Details     : 'idx' element value from 'in' vector is replicated to all
 *               elements in 'out' vector.
 *               Valid index range for halfword operation is 0-7.
 */
#define MSA_SPLATI_B(RTYPE, in, idx, out)                 \
{                                                         \
    out = (RTYPE) __msa_splati_b((v16i8) in, idx);        \
}

#define MSA_SPLATI_B2(RTYPE, in, idx0, idx1, out0, out1)  \
{                                                         \
    MSA_SPLATI_B(RTYPE, in, idx0, out0)                   \
    MSA_SPLATI_B(RTYPE, in, idx1, out1)                   \
}

#define MSA_SPLATI_B4(RTYPE, in, idx0, idx1, idx2, idx3,  \
                      out0, out1, out2, out3)             \
{                                                         \
    MSA_SPLATI_B2(RTYPE, in, idx0, idx1, out0, out1)      \
    MSA_SPLATI_B2(RTYPE, in, idx2, idx3, out2, out3)      \
}

/* Description : Indexed halfword elements are replicated to all elements in
 *               output vector.
 * Arguments   : Inputs  - in, idx
 *               Outputs - out
 *               Return Type - as per RTYPE
 * Details     : 'idx' element value from 'in' vector is replicated to all
 *               elements in 'out' vector.
 *               Valid index range for halfword operation is 0-7.
 */
#define MSA_SPLATI_H(RTYPE, in, idx, out)                 \
{                                                         \
    out = (RTYPE) __msa_splati_h((v8i16) in, idx);        \
}

#define MSA_SPLATI_H2(RTYPE, in, idx0, idx1, out0, out1)  \
{                                                         \
    MSA_SPLATI_H(RTYPE, in, idx0, out0)                   \
    MSA_SPLATI_H(RTYPE, in, idx1, out1)                   \
}

#define MSA_SPLATI_H4(RTYPE, in, idx0, idx1, idx2, idx3,  \
                      out0, out1, out2, out3)             \
{                                                         \
    MSA_SPLATI_H2(RTYPE, in, idx0, idx1, out0, out1)      \
    MSA_SPLATI_H2(RTYPE, in, idx2, idx3, out2, out3)      \
}

/* Description : Indexed word elements are replicated to all elements in
 *               output vector.
 * Arguments   : Inputs  - in, idx
 *               Outputs - out
 *               Return Type - as per RTYPE
 * Details     : 'idx' element value from 'in' vector is replicated to all
 *               elements in 'out' vector.
 *               Valid index range for halfword operation is 0-3.
 */
#define MSA_SPLATI_W(RTYPE, in, idx, out)                 \
{                                                         \
    out = (RTYPE) __msa_splati_w((v4i32) in, idx);        \
}

#define MSA_SPLATI_W2(RTYPE, in, idx0, idx1, out0, out1)  \
{                                                         \
    MSA_SPLATI_W(RTYPE, in, idx0, out0)                   \
    MSA_SPLATI_W(RTYPE, in, idx1, out1)                   \
}

#define MSA_SPLATI_W4(RTYPE, in, idx0, idx1, idx2, idx3,  \
                      out0, out1, out2, out3)             \
{                                                         \
    MSA_SPLATI_W2(RTYPE, in, idx0, idx1, out0, out1)      \
    MSA_SPLATI_W2(RTYPE, in, idx2, idx3, out2, out3)      \
}

/* Description : Pack even byte elements of vector pairs.
 * Arguments   : Inputs  - in0, in1
 *               Outputs - out
 *               Return Type - as per RTYPE
 * Details     : Even byte elements of 'in0' are copied to the left half of
 *               'out' & even byte elements of 'in1' are copied to the right
 *               half of 'out'.
 */
#define MSA_PCKEV_B(RTYPE, in0, in1, out)                    \
{                                                            \
    out = (RTYPE) __msa_pckev_b((v16i8) in0, (v16i8) in1);   \
}

#define MSA_PCKEV_B2(RTYPE, in0, in1, in2, in3, out0, out1)  \
{                                                            \
    MSA_PCKEV_B(RTYPE, in0, in1, out0)                       \
    MSA_PCKEV_B(RTYPE, in2, in3, out1)                       \
}

#define MSA_PCKEV_B4(RTYPE, in0, in1, in2, in3, in4, in5,    \
                     in6, in7, out0, out1, out2, out3)       \
{                                                            \
    MSA_PCKEV_B2(RTYPE, in0, in1, in2, in3, out0, out1)      \
    MSA_PCKEV_B2(RTYPE, in4, in5, in6, in7, out2, out3)      \
}

/* Description : Pack even halfword elements of vector pairs.
 * Arguments   : Inputs  - in0, in1
 *               Outputs - out
 *               Return Type - as per RTYPE
 * Details     : Even halfword elements of 'in0' are copied to the left half of
 *               'out' & even halfword elements of 'in1' are copied to the right
 *               half of 'out'.
 */
#define MSA_PCKEV_H(RTYPE, in0, in1, out)                    \
{                                                            \
    out = (RTYPE) __msa_pckev_h((v8i16) in0, (v8i16) in1);   \
}

#define MSA_PCKEV_H2(RTYPE, in0, in1, in2, in3, out0, out1)  \
{                                                            \
    MSA_PCKEV_H(RTYPE, in0, in1, out0)                       \
    MSA_PCKEV_H(RTYPE, in2, in3, out1)                       \
}

#define MSA_PCKEV_H4(RTYPE, in0, in1, in2, in3, in4, in5,    \
                     in6, in7, out0, out1, out2, out3)       \
{                                                            \
    MSA_PCKEV_H2(RTYPE, in0, in1, in2, in3, out0, out1)      \
    MSA_PCKEV_H2(RTYPE, in4, in5, in6, in7, out2, out3)      \
}

/* Description : Pack even word elements of vector pairs.
 * Arguments   : Inputs  - in0, in1
 *               Outputs - out
 *               Return Type - as per RTYPE
 * Details     : Even word elements of 'in0' are copied to the left half of
 *               'out' & even word elements of 'in1' are copied to the right
 *               half of 'out'.
 */
#define MSA_PCKEV_W(RTYPE, in0, in1, out)                    \
{                                                            \
    out = (RTYPE) __msa_pckev_w((v4i32) in0, (v4i32) in1);   \
}

#define MSA_PCKEV_W2(RTYPE, in0, in1, in2, in3, out0, out1)  \
{                                                            \
    MSA_PCKEV_W(RTYPE, in0, in1, out0)                       \
    MSA_PCKEV_W(RTYPE, in2, in3, out1)                       \
}

#define MSA_PCKEV_W4(RTYPE, in0, in1, in2, in3, in4, in5,    \
                     in6, in7, out0, out1, out2, out3)       \
{                                                            \
    MSA_PCKEV_W2(RTYPE, in0, in1, in2, in3, out0, out1)      \
    MSA_PCKEV_W2(RTYPE, in4, in5, in6, in7, out2, out3)      \
}

/* Description : Pack even double word elements of vector pairs.
 * Arguments   : Inputs  - in0, in1
 *               Outputs - out
 *               Return Type - as per RTYPE
 * Details     : Even double word elements of 'in0' are copied to the left
 *               half of 'out' & even double word elements of 'in1' are
 *               copied to the right half of 'out'.
 */
#define MSA_PCKEV_D(RTYPE, in0, in1, out)                    \
{                                                            \
    out = (RTYPE) __msa_pckev_d((v2i64) in0, (v2i64) in1);   \
}

#define MSA_PCKEV_D2(RTYPE, in0, in1, in2, in3, out0, out1)  \
{                                                            \
    MSA_PCKEV_D(RTYPE, in0, in1, out0)                       \
    MSA_PCKEV_D(RTYPE, in2, in3, out1)                       \
}

#define MSA_PCKEV_D4(RTYPE, in0, in1, in2, in3, in4, in5,    \
                     in6, in7, out0, out1, out2, out3)       \
{                                                            \
    MSA_PCKEV_D2(RTYPE, in0, in1, in2, in3, out0, out1)      \
    MSA_PCKEV_D2(RTYPE, in4, in5, in6, in7, out2, out3)      \
}

/* Description : Pack odd byte elements of vector pairs.
 * Arguments   : Inputs  - in0, in1
 *               Outputs - out
 *               Return Type - as per RTYPE
 * Details     : Odd byte elements of 'in0' are copied to the left half of
 *               'out' & odd byte elements of 'in1' are copied to the right
 *               half of 'out'.
 */
#define MSA_PCKOD_B(RTYPE, in0, in1, out)                    \
{                                                            \
    out = (RTYPE) __msa_pckod_b((v16i8) in0, (v16i8) in1);   \
}

#define MSA_PCKOD_B2(RTYPE, in0, in1, in2, in3, out0, out1)  \
{                                                            \
    MSA_PCKOD_B(RTYPE, in0, in1, out0)                       \
    MSA_PCKOD_B(RTYPE, in2, in3, out1)                       \
}

#define MSA_PCKOD_B4(RTYPE, in0, in1, in2, in3, in4, in5,    \
                     in6, in7, out0, out1, out2, out3)       \
{                                                            \
    MSA_PCKOD_B2(RTYPE, in0, in1, in2, in3, out0, out1)      \
    MSA_PCKOD_B2(RTYPE, in4, in5, in6, in7, out2, out3)      \
}

/* Description : Pack odd halfword elements of vector pairs.
 * Arguments   : Inputs  - in0, in1
 *               Outputs - out
 *               Return Type - as per RTYPE
 * Details     : Odd halfword elements of 'in0' are copied to the left half of
 *               'out' & odd halfword elements of 'in1' are copied to the right
 *               half of 'out'.
 */
#define MSA_PCKOD_H(RTYPE, in0, in1, out)                    \
{                                                            \
    out = (RTYPE) __msa_pckod_h((v8i16) in0, (v8i16) in1);   \
}

#define MSA_PCKOD_H2(RTYPE, in0, in1, in2, in3, out0, out1)  \
{                                                            \
    MSA_PCKOD_H(RTYPE, in0, in1, out0)                       \
    MSA_PCKOD_H(RTYPE, in2, in3, out1)                       \
}

#define MSA_PCKOD_H4(RTYPE, in0, in1, in2, in3, in4, in5,    \
                     in6, in7, out0, out1, out2, out3)       \
{                                                            \
    MSA_PCKOD_H2(RTYPE, in0, in1, in2, in3, out0, out1)      \
    MSA_PCKOD_H2(RTYPE, in4, in5, in6, in7, out2, out3)      \
}

/* Description : Pack odd word elements of vector pairs.
 * Arguments   : Inputs  - in0, in1
 *               Outputs - out
 *               Return Type - as per RTYPE
 * Details     : Odd word elements of 'in0' are copied to the left half of
 *               'out' & odd word elements of 'in1' are copied to the right
 *               half of 'out'.
 */
#define MSA_PCKOD_W(RTYPE, in0, in1, out)                    \
{                                                            \
    out = (RTYPE) __msa_pckod_w((v4i32) in0, (v4i32) in1);   \
}

#define MSA_PCKOD_W2(RTYPE, in0, in1, in2, in3, out0, out1)  \
{                                                            \
    MSA_PCKOD_W(RTYPE, in0, in1, out0)                       \
    MSA_PCKOD_W(RTYPE, in2, in3, out1)                       \
}

#define MSA_PCKOD_W4(RTYPE, in0, in1, in2, in3, in4, in5,    \
                     in6, in7, out0, out1, out2, out3)       \
{                                                            \
    MSA_PCKOD_W2(RTYPE, in0, in1, in2, in3, out0, out1)      \
    MSA_PCKOD_W2(RTYPE, in4, in5, in6, in7, out2, out3)      \
}

/* Description : Pack odd double word elements of vector pairs.
 * Arguments   : Inputs  - in0, in1
 *               Outputs - out
 *               Return Type - as per RTYPE
 * Details     : Odd double word elements of 'in0' are copied to the left
 *               half of 'out' & odd double word elements of 'in1' are
 *               copied to the right half of 'out'.
 */
#define MSA_PCKOD_D(RTYPE, in0, in1, out)                    \
{                                                            \
    out = (RTYPE) __msa_pckod_d((v2i64) in0, (v2i64) in1);   \
}

#define MSA_PCKOD_D2(RTYPE, in0, in1, in2, in3, out0, out1)  \
{                                                            \
    MSA_PCKOD_D(RTYPE, in0, in1, out0)                       \
    MSA_PCKOD_D(RTYPE, in2, in3, out1)                       \
}

#define MSA_PCKOD_D4(RTYPE, in0, in1, in2, in3, in4, in5,    \
                     in6, in7, out0, out1, out2, out3)       \
{                                                            \
    MSA_PCKOD_D2(RTYPE, in0, in1, in2, in3, out0, out1)      \
    MSA_PCKOD_D2(RTYPE, in4, in5, in6, in7, out2, out3)      \
}

/* Description : Dot product of unsigned byte vector elements.
 * Arguments   : Inputs  - mult
 *                         cnst
 *               Outputs - out
 *               Return Type - as per RTYPE
 * Details     : Unsigned byte elements from 'mult' are multiplied with
 *               unsigned byte elements from 'cnst' producing a result
 *               twice the size of input i.e. unsigned halfword.
 *               Then this multiplication results of adjacent odd-even elements
 *               are added together and stored to the out vector.
 */
#define MSA_DOTP_UB(RTYPE, mult, cnst, out)                         \
{                                                                   \
    out = (RTYPE) __msa_dotp_u_h((v16u8) mult, (v16u8) cnst);       \
}

#define MSA_DOTP_UB2(RTYPE, mult0, mult1, cnst0, cnst1, out0, out1) \
{                                                                   \
    MSA_DOTP_UB(RTYPE, mult0, cnst0, out0)                          \
    MSA_DOTP_UB(RTYPE, mult1, cnst1, out1)                          \
}

#define MSA_DOTP_UB4(RTYPE, mult0, mult1, mult2, mult3,             \
                     cnst0, cnst1, cnst2, cnst3,                    \
                     out0, out1, out2, out3)                        \
{                                                                   \
    MSA_DOTP_UB2(RTYPE, mult0, mult1, cnst0, cnst1, out0, out1);    \
    MSA_DOTP_UB2(RTYPE, mult2, mult3, cnst2, cnst3, out2, out3);    \
}

/* Description : Dot product of signed byte vector elements.
 * Arguments   : Inputs  - mult
 *                         cnst
 *               Outputs - out
 *               Return Type - as per RTYPE
 * Details     : Signed byte elements from 'mult' are multiplied with
 *               signed byte elements from 'cnst' producing a result
 *               twice the size of input i.e. signed halfword.
 *               Then this multiplication results of adjacent odd-even elements
 *               are added together and stored to the out vector.
 */
#define MSA_DOTP_SB(RTYPE, mult, cnst, out)                         \
{                                                                   \
    out = (RTYPE) __msa_dotp_s_h((v16i8) mult, (v16i8) cnst);       \
}

#define MSA_DOTP_SB2(RTYPE, mult0, mult1, cnst0, cnst1, out0, out1) \
{                                                                   \
    MSA_DOTP_SB(RTYPE, mult0, cnst0, out0)                          \
    MSA_DOTP_SB(RTYPE, mult1, cnst1, out1)                          \
}

#define MSA_DOTP_SB4(RTYPE, mult0, mult1, mult2, mult3,             \
                     cnst0, cnst1, cnst2, cnst3,                    \
                     out0, out1, out2, out3)                        \
{                                                                   \
    MSA_DOTP_SB2(RTYPE, mult0, mult1, cnst0, cnst1, out0, out1);    \
    MSA_DOTP_SB2(RTYPE, mult2, mult3, cnst2, cnst3, out2, out3);    \
}

/* Description : Dot product of unsigned halfword vector elements.
 * Arguments   : Inputs  - mult
 *                         cnst
 *               Outputs - out
 *               Return Type - as per RTYPE
 * Details     : Unsigned halfword elements from 'mult' are multiplied with
 *               unsigned halfword elements from 'cnst' producing a result
 *               twice the size of input i.e. unsigned word.
 *               Then this multiplication results of adjacent odd-even elements
 *               are added together and stored to the out vector.
 */
#define MSA_DOTP_UH(RTYPE, mult, cnst, out)                         \
{                                                                   \
    out = (RTYPE) __msa_dotp_u_w((v8u16) mult, (v8u16) cnst);       \
}

#define MSA_DOTP_UH2(RTYPE, mult0, mult1, cnst0, cnst1, out0, out1) \
{                                                                   \
    MSA_DOTP_UH(RTYPE, mult0, cnst0, out0)                          \
    MSA_DOTP_UH(RTYPE, mult1, cnst1, out1)                          \
}

#define MSA_DOTP_UH4(RTYPE, mult0, mult1, mult2, mult3,             \
                     cnst0, cnst1, cnst2, cnst3,                    \
                     out0, out1, out2, out3)                        \
{                                                                   \
    MSA_DOTP_UH2(RTYPE, mult0, mult1, cnst0, cnst1, out0, out1);    \
    MSA_DOTP_UH2(RTYPE, mult2, mult3, cnst2, cnst3, out2, out3);    \
}

/* Description : Dot product of signed halfword vector elements.
 * Arguments   : Inputs  - mult
 *                         cnst
 *               Outputs - out
 *               Return Type - as per RTYPE
 * Details     : Signed halfword elements from 'mult' are multiplied with
 *               signed halfword elements from 'cnst' producing a result
 *               twice the size of input i.e. signed word.
 *               Then this multiplication results of adjacent odd-even elements
 *               are added together and stored to the out vector.
 */
#define MSA_DOTP_SH(RTYPE, mult, cnst, out)                         \
{                                                                   \
    out = (RTYPE) __msa_dotp_s_w((v8i16) mult, (v8i16) cnst);       \
}

#define MSA_DOTP_SH2(RTYPE, mult0, mult1, cnst0, cnst1, out0, out1) \
{                                                                   \
    MSA_DOTP_SH(RTYPE, mult0, cnst0, out0)                          \
    MSA_DOTP_SH(RTYPE, mult1, cnst1, out1)                          \
}

#define MSA_DOTP_SH4(RTYPE, mult0, mult1, mult2, mult3,             \
                     cnst0, cnst1, cnst2, cnst3,                    \
                     out0, out1, out2, out3)                        \
{                                                                   \
    MSA_DOTP_SH2(RTYPE, mult0, mult1, cnst0, cnst1, out0, out1);    \
    MSA_DOTP_SH2(RTYPE, mult2, mult3, cnst2, cnst3, out2, out3);    \
}

/* Description : Dot product & addition of unsigned byte vector elements.
 * Arguments   : Inputs  - mult
 *                         cnst
 *               Outputs - out
 *               Return Type - as per RTYPE
 * Details     : Unsigned byte elements from 'mult' are multiplied with
 *               unsigned byte elements from 'cnst' producing a result
 *               twice the size of input i.e. unsigned halfword.
 *               Then this multiplication results of adjacent odd-even elements
 *               are added to the out vector.
 */
#define MSA_DPADD_UB(RTYPE, mult, cnst, out)                           \
{                                                                      \
    out = (RTYPE) __msa_dpadd_u_h((v8u16) out,                         \
                                   (v16u8) mult, (v16u8) cnst);        \
}

#define MSA_DPADD_UB2(RTYPE, mult0, mult1, cnst0, cnst1, out0, out1)   \
{                                                                      \
    MSA_DPADD_UB(RTYPE, mult0, cnst0, out0)                            \
    MSA_DPADD_UB(RTYPE, mult1, cnst1, out1)                            \
}

#define MSA_DPADD_UB4(RTYPE, mult0, mult1, mult2, mult3,               \
                  cnst0, cnst1, cnst2, cnst3, out0, out1, out2, out3)  \
{                                                                      \
    MSA_DPADD_UB2(RTYPE, mult0, mult1, cnst0, cnst1, out0, out1);      \
    MSA_DPADD_UB2(RTYPE, mult2, mult3, cnst2, cnst3, out2, out3);      \
}

/* Description : Dot product & addition of signed byte vector elements.
 * Arguments   : Inputs  - mult
 *                         cnst
 *               Outputs - out
 *               Return Type - as per RTYPE
 * Details     : Signed byte elements from 'mult' are multiplied with
 *               signed byte elements from 'cnst' producing a result
 *               twice the size of input i.e. signed halfword.
 *               Then this multiplication results of adjacent odd-even elements
 *               are added to the out vector.
 */
#define MSA_DPADD_SB(RTYPE, mult, cnst, out)                           \
{                                                                      \
    out = (RTYPE) __msa_dpadd_s_h((v8i16) out,                         \
                                   (v16i8) mult, (v16i8) cnst);        \
}

#define MSA_DPADD_SB2(RTYPE, mult0, mult1, cnst0, cnst1, out0, out1)   \
{                                                                      \
    MSA_DPADD_SB(RTYPE, mult0, cnst0, out0)                            \
    MSA_DPADD_SB(RTYPE, mult1, cnst1, out1)                            \
}

#define MSA_DPADD_SB4(RTYPE, mult0, mult1, mult2, mult3,               \
                  cnst0, cnst1, cnst2, cnst3, out0, out1, out2, out3)  \
{                                                                      \
    MSA_DPADD_SB2(RTYPE, mult0, mult1, cnst0, cnst1, out0, out1);      \
    MSA_DPADD_SB2(RTYPE, mult2, mult3, cnst2, cnst3, out2, out3);      \
}

/* Description : Dot product & addition of unsigned halfword vector elements.
 * Arguments   : Inputs  - mult
 *                         cnst
 *               Outputs - out
 *               Return Type - as per RTYPE
 * Details     : Unsigned halfword elements from 'mult' are multiplied with
 *               unsigned halfword elements from 'cnst' producing a result
 *               twice the size of input i.e. unsigned word.
 *               Then this multiplication results of adjacent odd-even elements
 *               are added to the out vector.
 */
#define MSA_DPADD_UH(RTYPE, mult, cnst, out)                           \
{                                                                      \
    out = (RTYPE) __msa_dpadd_u_w((v4u32) out,                         \
                                   (v8u16) mult, (v8u16) cnst);        \
}

#define MSA_DPADD_UH2(RTYPE, mult0, mult1, cnst0, cnst1, out0, out1)   \
{                                                                      \
    MSA_DPADD_UH(RTYPE, mult0, cnst0, out0)                            \
    MSA_DPADD_UH(RTYPE, mult1, cnst1, out1)                            \
}

#define MSA_DPADD_UH4(RTYPE, mult0, mult1, mult2, mult3,               \
                  cnst0, cnst1, cnst2, cnst3, out0, out1, out2, out3)  \
{                                                                      \
    MSA_DPADD_UH2(RTYPE, mult0, mult1, cnst0, cnst1, out0, out1);      \
    MSA_DPADD_UH2(RTYPE, mult2, mult3, cnst2, cnst3, out2, out3);      \
}

/* Description : Dot product & addition of signed halfword vector elements.
 * Arguments   : Inputs  - mult
 *                         cnst
 *               Outputs - out
 *               Return Type - as per RTYPE
 * Details     : Signed halfword elements from 'mult' are multiplied with
 *               signed halfword elements from 'cnst' producing a result
 *               twice the size of input i.e. signed word.
 *               Then this multiplication results of adjacent odd-even elements
 *               are added to the out vector.
 */
#define MSA_DPADD_SH(RTYPE, mult, cnst, out)                           \
{                                                                      \
    out = (RTYPE) __msa_dpadd_s_w((v4i32) out,                         \
                                   (v8i16) mult, (v8i16) cnst);        \
}

#define MSA_DPADD_SH2(RTYPE, mult0, mult1, cnst0, cnst1, out0, out1)   \
{                                                                      \
    MSA_DPADD_SH(RTYPE, mult0, cnst0, out0)                            \
    MSA_DPADD_SH(RTYPE, mult1, cnst1, out1)                            \
}

#define MSA_DPADD_SH4(RTYPE, mult0, mult1, mult2, mult3,               \
                  cnst0, cnst1, cnst2, cnst3, out0, out1, out2, out3)  \
{                                                                      \
    MSA_DPADD_SH2(RTYPE, mult0, mult1, cnst0, cnst1, out0, out1);      \
    MSA_DPADD_SH2(RTYPE, mult2, mult3, cnst2, cnst3, out2, out3);      \
}

/* Description : Clip all signed halfword elements of input vector between min & max.
 *               out = ((in) < (min)) ? (min) : (((in) > (max)) ? (max) : (in)).
 * Arguments   : Inputs  - in    (input vector)
 *                       - min   (min threshold)
 *                       - max   (max threshold)
 *               Outputs - in    (output vector with clipped elements)
 * Note        : type of 'in' must be v8i16.
 */
#define MSA_CLIP_SH(in, min, max)                 \
{                                                 \
    in = __msa_max_s_h((v8i16) min, (v8i16) in);  \
    in = __msa_min_s_h((v8i16) max, (v8i16) in);  \
}

/* Description : Clip all signed halfword elements of input vector between 0 & 255.
 * Arguments   : Inputs  - in    (input vector)
 *               Outputs - in    (output vector with clipped elements)
 * Note        : type of 'in' must be v8i16.
 */
#define MSA_CLIP_SH_0_255(in)                   \
{                                               \
    in = __msa_maxi_s_h((v8i16) in, 0);         \
    in = (v8i16) __msa_sat_u_h((v8u16) in, 7);  \
}

#define MSA_CLIP_SH2_0_255(in0, in1)            \
{                                               \
    MSA_CLIP_SH_0_255(in0);                     \
    MSA_CLIP_SH_0_255(in1);                     \
}

#define MSA_CLIP_SH4_0_255(in0, in1, in2, in3)  \
{                                               \
    MSA_CLIP_SH2_0_255(in0, in1);               \
    MSA_CLIP_SH2_0_255(in2, in3);               \
}

#define MSA_CLIP_SH8_0_255(in0, in1, in2, in3,  \
                       in4, in5, in6, in7)      \
{                                               \
    MSA_CLIP_SH4_0_255(in0, in1, in2, in3);     \
    MSA_CLIP_SH4_0_255(in4, in5, in6, in7);     \
}

/* Description : Clip all signed word elements of input vector between 0 & 255.
 * Arguments   : Inputs  - in    (input vector)
 *               Outputs - in    (output vector with clipped elements)
 * Note        : type of 'in' must be v4i32.
 */
#define MSA_CLIP_SW_0_255(in)                   \
{                                               \
    in = __msa_maxi_s_w((v4i32) in, 0);         \
    in = (v4i32) __msa_sat_u_w((v4u32) in, 7);  \
}

#define MSA_CLIP_SW2_0_255(in0, in1)            \
{                                               \
    MSA_CLIP_SW_0_255(in0);                     \
    MSA_CLIP_SW_0_255(in1);                     \
}

#define MSA_CLIP_SW4_0_255(in0, in1, in2, in3)  \
{                                               \
    MSA_CLIP_SW2_0_255(in0, in1);               \
    MSA_CLIP_SW2_0_255(in2, in3);               \
}

#define MSA_CLIP_SW8_0_255(in0, in1, in2, in3,  \
                       in4, in5, in6, in7)      \
{                                               \
    MSA_CLIP_SW4_0_255(in0, in1, in2, in3);     \
    MSA_CLIP_SW4_0_255(in4, in5, in6, in7);     \
}

/* Description : Addition of 16 unsigned byte elements.
 *               16 unsigned byte elements of input vector are added
 *               together and resulted integer sum is returned.
 * Arguments   : Inputs  - in       (unsigned byte vector)
 *               Outputs - sum_m    (u32 sum)
 *               Return Type - unsigned word
 */
#define MSA_HADD_UB_U32(in, sum_m)                       \
{                                                        \
    v8u16 res_m;                                         \
    v4u32 res0_m;                                        \
    v2u64 res1_m, res2_m;                                \
                                                         \
    res_m = __msa_hadd_u_h((v16u8) in, (v16u8) in);      \
    res0_m = __msa_hadd_u_w(res_m, res_m);               \
    res1_m = __msa_hadd_u_d(res0_m, res0_m);             \
    res2_m = (v2u64) __msa_splati_d((v2i64) res1_m, 1);  \
    res1_m += res2_m;                                    \
    sum_m = __msa_copy_u_w((v4i32) res1_m, 0);           \
}

/* Description : Addition of 8 unsigned halfword elements.
 *               8 unsigned halfword elements of input vector are added
 *               together and resulted integer sum is returned.
 * Arguments   : Inputs  - in       (unsigned halfword vector)
 *               Outputs - sum_m    (u32 sum)
 *               Return Type - unsigned word
 */
#define MSA_HADD_UH_U32(in, sum_m)                       \
{                                                        \
    v4u32 res_m;                                         \
    v2u64 res0_m, res1_m;                                \
                                                         \
    res_m = __msa_hadd_u_w((v8u16) in, (v8u16) in);      \
    res0_m = __msa_hadd_u_d(res_m, res_m);               \
    res1_m = (v2u64) __msa_splati_d((v2i64) res0_m, 1);  \
    res0_m += res1_m;                                    \
    sum_m = __msa_copy_u_w((v4i32) res0_m, 0);           \
}

/* Description : Addition of 4 unsigned word elements.
 *               4 unsigned word elements of input vector are added together and
 *               resulted integer sum is returned.
 * Arguments   : Inputs  - in       (unsigned word vector)
 *               Outputs - sum_m    (u32 sum)
 *               Return Type - unsigned word
 */
#define MSA_HADD_UW_U32(in, sum_m)                       \
{                                                        \
    v2u64 res0_m, res1_m;                                \
                                                         \
    res0_m = __msa_hadd_u_d((v4u32) in, (v4u32) in);     \
    res1_m = (v2u64) __msa_splati_d((v2i64) res0_m, 1);  \
    res0_m += res1_m;                                    \
    sum_m = __msa_copy_u_w((v4i32) res0_m, 0);           \
}

/* Description : Addition of 16 signed byte elements.
 *               16 signed byte elements of input vector are added
 *               together and resulted integer sum is returned.
 * Arguments   : Inputs  - in       (signed byte vector)
 *               Outputs - sum_m    (i32 sum)
 *               Return Type - signed word
 */
#define MSA_HADD_SB_S32(in, sum_m)                   \
{                                                    \
    v8i16 res_m;                                     \
    v4i32 res0_m;                                    \
    v2i64 res1_m, res2_m;                            \
                                                     \
    res_m = __msa_hadd_s_h((v16i8) in, (v16i8) in);  \
    res0_m = __msa_hadd_s_w(res_m, res_m);           \
    res1_m = __msa_hadd_s_d(res0_m, res0_m);         \
    res2_m = __msa_splati_d(res1_m, 1);              \
    res1_m += res2_m;                                \
    sum_m = __msa_copy_s_w((v4i32) res1_m, 0);       \
}

/* Description : Addition of 8 signed halfword elements.
 *               8 signed halfword elements of input vector are added
 *               together and resulted integer sum is returned.
 * Arguments   : Inputs  - in       (signed halfword vector)
 *               Outputs - sum_m    (i32 sum)
 *               Return Type - signed word
 */
#define MSA_HADD_SH_S32(in, sum_m)                   \
{                                                    \
    v4i32 res_m;                                     \
    v2i64 res0_m, res1_m;                            \
                                                     \
    res_m = __msa_hadd_s_w((v8i16) in, (v8i16) in);  \
    res0_m = __msa_hadd_s_d(res_m, res_m);           \
    res1_m = __msa_splati_d(res0_m, 1);              \
    res0_m += res1_m;                                \
    sum_m = __msa_copy_s_w((v4i32) res0_m, 0);       \
}

/* Description : Addition of 4 signed word elements.
 *               4 signed word elements of input vector are added together and
 *               resulted integer sum is returned.
 * Arguments   : Inputs  - in       (signed word vector)
 *               Outputs - sum_m    (i32 sum)
 *               Return Type - signed word
 */
#define MSA_HADD_SW_S32(in, sum_m)                    \
{                                                     \
    v2i64 res0_m, res1_m;                             \
                                                      \
    res0_m = __msa_hadd_s_d((v4i32) in, (v4i32) in);  \
    res1_m = __msa_splati_d(res0_m, 1);               \
    res0_m += res1_m;                                 \
    sum_m = __msa_copy_s_w((v4i32) res0_m, 0);        \
}

/* Description : Saturate the unsigned halfword element values to the max
 *               unsigned value of (sat_val+1 bits).
 *               The element data width remains unchanged.
 * Arguments   : Inputs  - in, sat_val
 *               Outputs - in (in place)
 *               Return Type - v8u16
 * Details     : Each unsigned halfword element from 'in' is saturated to the
 *               value generated with (sat_val+1) bit range.
 *               Results are in placed to original vectors.
 */
#define MSA_SAT_UH(in, sat_val)                   \
{                                                 \
    in = __msa_sat_u_h(in, sat_val);              \
}

#define MSA_SAT_UH2(in0, in1, sat_val)            \
{                                                 \
    MSA_SAT_UH(in0, sat_val)                      \
    MSA_SAT_UH(in1, sat_val)                      \
}

#define MSA_SAT_UH4(in0, in1, in2, in3, sat_val)  \
{                                                 \
    MSA_SAT_UH2(in0, in1, sat_val)                \
    MSA_SAT_UH2(in2, in3, sat_val)                \
}

/* Description : Saturate the signed halfword element values to the max
 *               signed value of (sat_val+1 bits).
 *               The element data width remains unchanged.
 * Arguments   : Inputs  - in, sat_val
 *               Outputs - in (in place)
 *               Return Type - v8i16
 * Details     : Each signed halfword element from 'in' is saturated to the
 *               value generated with (sat_val+1) bit range.
 *               Results are in placed to original vectors.
 */
#define MSA_SAT_SH(in, sat_val)                   \
{                                                 \
    in = __msa_sat_s_h(in, sat_val);              \
}

#define MSA_SAT_SH2(in0, in1, sat_val)            \
{                                                 \
    MSA_SAT_SH(in0, sat_val)                      \
    MSA_SAT_SH(in1, sat_val)                      \
}

#define MSA_SAT_SH4(in0, in1, in2, in3, sat_val)  \
{                                                 \
    MSA_SAT_SH2(in0, in1, sat_val)                \
    MSA_SAT_SH2(in2, in3, sat_val)                \
}

/* Description : Saturate the unsigned word element values to the max
 *               unsigned value of (sat_val+1 bits).
 *               The element data width remains unchanged.
 * Arguments   : Inputs  - in, sat_val
 *               Outputs - in (in place)
 *               Return Type - v4u32
 * Details     : Each unsigned word element from 'in' is saturated to the
 *               value generated with (sat_val+1) bit range.
 *               Results are in placed to original vectors.
 */
#define MSA_SAT_UW(in, sat_val)                   \
{                                                 \
    in = __msa_sat_u_w(in, sat_val);              \
}

#define MSA_SAT_UW2(in0, in1, sat_val)            \
{                                                 \
    MSA_SAT_UW(in0, sat_val)                      \
    MSA_SAT_UW(in1, sat_val)                      \
}

#define MSA_SAT_UW4(in0, in1, in2, in3, sat_val)  \
{                                                 \
    MSA_SAT_UW2(in0, in1, sat_val)                \
    MSA_SAT_UW2(in2, in3, sat_val)                \
}

/* Description : Saturate the signed word element values to the max
 *               signed value of (sat_val+1 bits).
 *               The element data width remains unchanged.
 * Arguments   : Inputs  - in, sat_val
 *               Outputs - in (in place)
 *               Return Type - v4i32
 * Details     : Each signed word element from 'in' is saturated to the
 *               value generated with (sat_val+1) bit range.
 *               Results are in placed to original vectors.
 */
#define MSA_SAT_SW(in, sat_val)                   \
{                                                 \
    in = __msa_sat_s_w(in, sat_val);              \
}

#define MSA_SAT_SW2(in0, in1, sat_val)            \
{                                                 \
    MSA_SAT_SW(in0, sat_val)                      \
    MSA_SAT_SW(in1, sat_val)                      \
}

#define MSA_SAT_SW4(in0, in1, in2, in3, sat_val)  \
{                                                 \
    MSA_SAT_SW2(in0, in1, sat_val)                \
    MSA_SAT_SW2(in2, in3, sat_val)                \
}

/* Description : Each byte element is logically xor'ed with immediate 128.
 * Arguments   : Inputs  - in
 *               Outputs - in (in-place)
 *               Return Type - as per RTYPE
 * Details     : Each unsigned byte element from input vector 'in' is
 *               logically xor'ed with 128 and result is in-place stored in
 *               'in' vector.
 */
#define MSA_XORI_B_128(RTYPE, in)                 \
{                                                 \
     in = (RTYPE) __msa_xori_b((v16u8) in, 128);  \
}

#define MSA_XORI_B2_128(RTYPE, in0, in1)  \
{                                         \
    MSA_XORI_B_128(RTYPE, in0);           \
    MSA_XORI_B_128(RTYPE, in1);           \
}

#define MSA_XORI_B4_128(RTYPE, in0, in1, in2, in3)  \
{                                                   \
    MSA_XORI_B2_128(RTYPE, in0, in1);               \
    MSA_XORI_B2_128(RTYPE, in2, in3);               \
}

/* Description : Shift right logical all byte elements of vector.
 * Arguments   : Inputs  - in, shift
 *               Outputs - in (in place)
 *               Return Type - as per RTYPE
 * Details     : Each element of vector 'in' is shifted right logical by
 *               number of bits respective element holds in vector 'shift' and
 *               result is in place written to 'in'.
 *               Here, 'shift' is a vector passed in.
 */
#define MSA_SRL_B(RTYPE, in, shift)                       \
{                                                         \
    in = (RTYPE) __msa_srl_b((v16i8) in, (v16i8) shift);  \
}

#define MSA_SRL_B2(RTYPE, in0, in1, shift)  \
{                                           \
    MSA_SRL_B(RTYPE, in0, shift);           \
    MSA_SRL_B(RTYPE, in1, shift);           \
}

#define MSA_SRL_B4(RTYPE, in0, in1, in2, in3, shift)  \
{                                                     \
    MSA_SRL_B2(RTYPE, in0, in1, shift);               \
    MSA_SRL_B2(RTYPE, in2, in3, shift);               \
}

/* Description : Shift right logical all halfword elements of vector.
 * Arguments   : Inputs  - in, shift
 *               Outputs - in (in place)
 *               Return Type - as per RTYPE
 * Details     : Each element of vector 'in' is shifted right logical by
 *               number of bits respective element holds in vector 'shift' and
 *               result is in place written to 'in'.
 *               Here, 'shift' is a vector passed in.
 */
#define MSA_SRL_H(RTYPE, in, shift)                       \
{                                                         \
    in = (RTYPE) __msa_srl_h((v8i16) in, (v8i16) shift);  \
}

#define MSA_SRL_H2(RTYPE, in0, in1, shift)  \
{                                           \
    MSA_SRL_H(RTYPE, in0, shift);           \
    MSA_SRL_H(RTYPE, in1, shift);           \
}

#define MSA_SRL_H4(RTYPE, in0, in1, in2, in3, shift)  \
{                                                     \
    MSA_SRL_H2(RTYPE, in0, in1, shift);               \
    MSA_SRL_H2(RTYPE, in2, in3, shift);               \
}

/* Description : Shift right logical all word elements of vector.
 * Arguments   : Inputs  - in, shift
 *               Outputs - in (in place)
 *               Return Type - as per RTYPE
 * Details     : Each element of vector 'in' is shifted right logical by
 *               number of bits respective element holds in vector 'shift' and
 *               result is in place written to 'in'.
 *               Here, 'shift' is a vector passed in.
 */
#define MSA_SRL_W(RTYPE, in, shift)                       \
{                                                         \
    in = (RTYPE) __msa_srl_w((v4i32) in, (v4i32) shift);  \
}

#define MSA_SRL_W2(RTYPE, in0, in1, shift)  \
{                                           \
    MSA_SRL_W(RTYPE, in0, shift);           \
    MSA_SRL_W(RTYPE, in1, shift);           \
}

#define MSA_SRL_W4(RTYPE, in0, in1, in2, in3, shift)  \
{                                                     \
    MSA_SRL_W2(RTYPE, in0, in1, shift);               \
    MSA_SRL_W2(RTYPE, in2, in3, shift);               \
}

/* Description : Shift right logical all double word elements of vector.
 * Arguments   : Inputs  - in, shift
 *               Outputs - in (in place)
 *               Return Type - as per RTYPE
 * Details     : Each element of vector 'in' is shifted right logical by
 *               number of bits respective element holds in vector 'shift' and
 *               result is in place written to 'in'.
 *               Here, 'shift' is a vector passed in.
 */
#define MSA_SRL_D(RTYPE, in, shift)                       \
{                                                         \
    in = (RTYPE) __msa_srl_d((v2i64) in, (v2i64) shift);  \
}

#define MSA_SRL_D2(RTYPE, in0, in1, shift)  \
{                                           \
    MSA_SRL_D(RTYPE, in0, shift);           \
    MSA_SRL_D(RTYPE, in1, shift);           \
}

#define MSA_SRL_D4(RTYPE, in0, in1, in2, in3, shift)  \
{                                                     \
    MSA_SRL_D2(RTYPE, in0, in1, shift);               \
    MSA_SRL_D2(RTYPE, in2, in3, shift);               \
}

/* Description : Shift right logical rounded all byte elements of vector.
 * Arguments   : Inputs  - in, shift
 *               Outputs - in (in place)
 *               Return Type - as per RTYPE
 * Details     : Each element of vector 'in' is shifted right logical rounded
 *               by number of bits respective element holds in vector 'shift'
 *               and result is in place written to 'in'.
 *               Here, 'shift' is a vector passed in.
 */
#define MSA_SRLR_B(RTYPE, in, shift)                       \
{                                                          \
    in = (RTYPE) __msa_srlr_b((v16i8) in, (v16i8) shift);  \
}

#define MSA_SRLR_B2(RTYPE, in0, in1, shift)  \
{                                            \
    MSA_SRLR_B(RTYPE, in0, shift);           \
    MSA_SRLR_B(RTYPE, in1, shift);           \
}

#define MSA_SRLR_B4(RTYPE, in0, in1, in2, in3, shift)  \
{                                                      \
    MSA_SRLR_B2(RTYPE, in0, in1, shift);               \
    MSA_SRLR_B2(RTYPE, in2, in3, shift);               \
}

/* Description : Shift right logical rounded all halfword elements of vector.
 * Arguments   : Inputs  - in, shift
 *               Outputs - in (in place)
 *               Return Type - as per RTYPE
 * Details     : Each element of vector 'in' is shifted right logical rounded
 *               by number of bits respective element holds in vector 'shift'
 *               and result is in place written to 'in'.
 *               Here, 'shift' is a vector passed in.
 */
#define MSA_SRLR_H(RTYPE, in, shift)                       \
{                                                          \
    in = (RTYPE) __msa_srlr_h((v8i16) in, (v8i16) shift);  \
}

#define MSA_SRLR_H2(RTYPE, in0, in1, shift)  \
{                                            \
    MSA_SRLR_H(RTYPE, in0, shift);           \
    MSA_SRLR_H(RTYPE, in1, shift);           \
}

#define MSA_SRLR_H4(RTYPE, in0, in1, in2, in3, shift)  \
{                                                      \
    MSA_SRLR_H2(RTYPE, in0, in1, shift);               \
    MSA_SRLR_H2(RTYPE, in2, in3, shift);               \
}

/* Description : Shift right logical rounded all word elements of vector.
 * Arguments   : Inputs  - in, shift
 *               Outputs - in (in place)
 *               Return Type - as per RTYPE
 * Details     : Each element of vector 'in' is shifted right logical rounded
 *               by number of bits respective element holds in vector 'shift'
 *               and result is in place written to 'in'.
 *               Here, 'shift' is a vector passed in.
 */
#define MSA_SRLR_W(RTYPE, in, shift)                       \
{                                                          \
    in = (RTYPE) __msa_srlr_w((v4i32) in, (v4i32) shift);  \
}

#define MSA_SRLR_W2(RTYPE, in0, in1, shift)  \
{                                            \
    MSA_SRLR_W(RTYPE, in0, shift);           \
    MSA_SRLR_W(RTYPE, in1, shift);           \
}

#define MSA_SRLR_W4(RTYPE, in0, in1, in2, in3, shift)  \
{                                                      \
    MSA_SRLR_W2(RTYPE, in0, in1, shift);               \
    MSA_SRLR_W2(RTYPE, in2, in3, shift);               \
}

/* Description : Shift right logical rounded all double word elements of vector.
 * Arguments   : Inputs  - in, shift
 *               Outputs - in (in place)
 *               Return Type - as per RTYPE
 * Details     : Each element of vector 'in' is shifted right logical rounded
 *               by number of bits respective element holds in vector 'shift'
 *               and result is in place written to 'in'.
 *               Here, 'shift' is a vector passed in.
 */
#define MSA_SRLR_D(RTYPE, in, shift)                       \
{                                                          \
    in = (RTYPE) __msa_srlr_d((v2i64) in, (v2i64) shift);  \
}

#define MSA_SRLR_D2(RTYPE, in0, in1, shift)  \
{                                            \
    MSA_SRLR_D(RTYPE, in0, shift);           \
    MSA_SRLR_D(RTYPE, in1, shift);           \
}

#define MSA_SRLR_D4(RTYPE, in0, in1, in2, in3, shift)  \
{                                                      \
    MSA_SRLR_D2(RTYPE, in0, in1, shift);               \
    MSA_SRLR_D2(RTYPE, in2, in3, shift);               \
}

/* Description : Shift right arithmetic rounded all byte elements of vector.
 * Arguments   : Inputs  - in, shift
 *               Outputs - in (in place)
 *               Return Type - as per RTYPE
 * Details     : Each element of vector 'in' is shifted right arithmetic
 *               rounded by number of bits respective element holds in
 *               vector 'shift' and result is in place written to 'in'.
 *               Here, 'shift' is a vector passed in.
 */
#define MSA_SRAR_B(RTYPE, in, shift)                       \
{                                                          \
    in = (RTYPE) __msa_srar_b((v16i8) in, (v16i8) shift);  \
}

#define MSA_SRAR_B2(RTYPE, in0, in1, shift)  \
{                                            \
    MSA_SRAR_B(RTYPE, in0, shift);           \
    MSA_SRAR_B(RTYPE, in1, shift);           \
}

#define MSA_SRAR_B4(RTYPE, in0, in1, in2, in3, shift)  \
{                                                      \
    MSA_SRAR_B2(RTYPE, in0, in1, shift);               \
    MSA_SRAR_B2(RTYPE, in2, in3, shift);               \
}

/* Description : Shift right arithmetic rounded all halfword elements of vector.
 * Arguments   : Inputs  - in, shift
 *               Outputs - in (in place)
 *               Return Type - as per RTYPE
 * Details     : Each element of vector 'in' is shifted right arithmetic
 *               rounded by number of bits respective element holds in
 *               vector 'shift' and result is in place written to 'in'.
 *               Here, 'shift' is a vector passed in.
 */
#define MSA_SRAR_H(RTYPE, in, shift)                       \
{                                                          \
    in = (RTYPE) __msa_srar_h((v8i16) in, (v8i16) shift);  \
}

#define MSA_SRAR_H2(RTYPE, in0, in1, shift)  \
{                                            \
    MSA_SRAR_H(RTYPE, in0, shift);           \
    MSA_SRAR_H(RTYPE, in1, shift);           \
}

#define MSA_SRAR_H4(RTYPE, in0, in1, in2, in3, shift)  \
{                                                      \
    MSA_SRAR_H2(RTYPE, in0, in1, shift);               \
    MSA_SRAR_H2(RTYPE, in2, in3, shift);               \
}

/* Description : Shift right arithmetic rounded all word elements of vector.
 * Arguments   : Inputs  - in, shift
 *               Outputs - in (in place)
 *               Return Type - as per RTYPE
 * Details     : Each element of vector 'in' is shifted right arithmetic
 *               rounded by number of bits respective element holds in
 *               vector 'shift' and result is in place written to 'in'.
 *               Here, 'shift' is a vector passed in.
 */
#define MSA_SRAR_W(RTYPE, in, shift)                       \
{                                                          \
    in = (RTYPE) __msa_srar_w((v4i32) in, (v4i32) shift);  \
}

#define MSA_SRAR_W2(RTYPE, in0, in1, shift)  \
{                                            \
    MSA_SRAR_W(RTYPE, in0, shift);           \
    MSA_SRAR_W(RTYPE, in1, shift);           \
}

#define MSA_SRAR_W4(RTYPE, in0, in1, in2, in3, shift)  \
{                                                      \
    MSA_SRAR_W2(RTYPE, in0, in1, shift);               \
    MSA_SRAR_W2(RTYPE, in2, in3, shift);               \
}

/* Description : Shift right arithmetic rounded all double word elements
 *               of vector.
 * Arguments   : Inputs  - in, shift
 *               Outputs - in (in place)
 *               Return Type - as per RTYPE
 * Details     : Each element of vector 'in' is shifted right arithmetic
 *               rounded by number of bits respective element holds in
 *               vector 'shift' and result is in place written to 'in'.
 *               Here, 'shift' is a vector passed in.
 */
#define MSA_SRAR_D(RTYPE, in, shift)                       \
{                                                          \
    in = (RTYPE) __msa_srar_d((v2i64) in, (v2i64) shift);  \
}

#define MSA_SRAR_D2(RTYPE, in0, in1, shift)  \
{                                            \
    MSA_SRAR_D(RTYPE, in0, shift);           \
    MSA_SRAR_D(RTYPE, in1, shift);           \
}

#define MSA_SRAR_D4(RTYPE, in0, in1, in2, in3, shift)  \
{                                                      \
    MSA_SRAR_D2(RTYPE, in0, in1, shift);               \
    MSA_SRAR_D2(RTYPE, in2, in3, shift);               \
}

/* Description : Shift right arithmetic rounded all byte elements of vector.
 * Arguments   : Inputs  - in, shift
 *               Outputs - in (in place)
 *               Return Type - as per RTYPE
 * Details     : Each element of vector 'in' is shifted right arithmetic
 *               rounded by number of bits respective element holds in vector
 *               'shift' and result is in place written to 'in'.
 *               Here, 'shift' is a immediate number passed in.
 */
#define MSA_SRARI_B(RTYPE, in, shift)                       \
{                                                           \
    in = (RTYPE) __msa_srari_b((v16i8) in, (v16i8) shift);  \
}

#define MSA_SRARI_B2(RTYPE, in0, in1, shift)  \
{                                             \
    MSA_SRARI_B(RTYPE, in0, shift);           \
    MSA_SRARI_B(RTYPE, in1, shift);           \
}

#define MSA_SRARI_B4(RTYPE, in0, in1, in2, in3, shift)  \
{                                                       \
    MSA_SRARI_B2(RTYPE, in0, in1, shift);               \
    MSA_SRARI_B2(RTYPE, in2, in3, shift);               \
}

/* Description : Shift right arithmetic rounded all halfword elements of vector.
 * Arguments   : Inputs  - in, shift
 *               Outputs - in (in place)
 *               Return Type - as per RTYPE
 * Details     : Each element of vector 'in' is shifted right arithmetic
 *               rounded by number of bits respective element holds in vector
 *               'shift' and result is in place written to 'in'.
 *               Here, 'shift' is a immediate number passed in.
 */
#define MSA_SRARI_H(RTYPE, in, shift)                       \
{                                                           \
    in = (RTYPE) __msa_srari_h((v8i16) in, (v8i16) shift);  \
}

#define MSA_SRARI_H2(RTYPE, in0, in1, shift)  \
{                                             \
    MSA_SRARI_H(RTYPE, in0, shift);           \
    MSA_SRARI_H(RTYPE, in1, shift);           \
}

#define MSA_SRARI_H4(RTYPE, in0, in1, in2, in3, shift)  \
{                                                       \
    MSA_SRARI_H2(RTYPE, in0, in1, shift);               \
    MSA_SRARI_H2(RTYPE, in2, in3, shift);               \
}

/* Description : Shift right arithmetic rounded all word elements of vector.
 * Arguments   : Inputs  - in, shift
 *               Outputs - in (in place)
 *               Return Type - as per RTYPE
 * Details     : Each element of vector 'in' is shifted right arithmetic
 *               rounded by number of bits respective element holds in vector
 *               'shift' and result is in place written to 'in'.
 *               Here, 'shift' is a immediate number passed in.
 */
#define MSA_SRARI_W(RTYPE, in, shift)                       \
{                                                           \
    in = (RTYPE) __msa_srari_w((v4i32) in, (v4i32) shift);  \
}

#define MSA_SRARI_W2(RTYPE, in0, in1, shift)  \
{                                             \
    MSA_SRARI_W(RTYPE, in0, shift);           \
    MSA_SRARI_W(RTYPE, in1, shift);           \
}

#define MSA_SRARI_W4(RTYPE, in0, in1, in2, in3, shift)  \
{                                                       \
    MSA_SRARI_W2(RTYPE, in0, in1, shift);               \
    MSA_SRARI_W2(RTYPE, in2, in3, shift);               \
}

/* Description : Shift right arithmetic rounded all double word elements
 *               of vector.
 * Arguments   : Inputs  - in, shift
 *               Outputs - in (in place)
 *               Return Type - as per RTYPE
 * Details     : Each element of vector 'in' is shifted right arithmetic
 *               rounded by number of bits respective element holds in
 *               vector 'shift' and result is in place written to 'in'.
 *               Here, 'shift' is a immediate number passed in.
 */
#define MSA_SRARI_D(RTYPE, in, shift)                       \
{                                                           \
    in = (RTYPE) __msa_srari_d((v2i64) in, (v2i64) shift);  \
}

#define MSA_SRARI_D2(RTYPE, in0, in1, shift)  \
{                                             \
    MSA_SRARI_D(RTYPE, in0, shift);           \
    MSA_SRARI_D(RTYPE, in1, shift);           \
}

#define MSA_SRARI_D4(RTYPE, in0, in1, in2, in3, shift)  \
{                                                       \
    MSA_SRARI_D2(RTYPE, in0, in1, shift);               \
    MSA_SRARI_D2(RTYPE, in2, in3, shift);               \
}

/* Description : Transposes input 4x4 byte block.
 * Arguments   : Inputs  - in0, in1, in2, in3      (input 4x4 byte block)
 *               Outputs - out0, out1, out2, out3  (output 4x4 byte block)
 *               Return Type - RTYPE
 * Details     :
 */
#define MSA_TRANSPOSE4x4_B(RTYPE, in0, in1, in2, in3,         \
                           out0, out1, out2, out3)            \
{                                                             \
    v16i8 zero_m = { 0 };                                     \
                                                              \
    MSA_ILVR_B2(RTYPE, in2, in0, in3, in1, out2, out3);       \
    out0 = (RTYPE) __msa_ilvr_b((v16i8) out3, (v16i8) out2);  \
    out1 = (RTYPE) __msa_sldi_b(zero_m, (v16i8) out0, 4);     \
    out2 = (RTYPE) __msa_sldi_b(zero_m, (v16i8) out1, 4);     \
    out3 = (RTYPE) __msa_sldi_b(zero_m, (v16i8) out2, 4);     \
}

/* Description : Transposes input 8x4 byte block into 4x8.
 * Arguments   : Inputs  - in0, in1, in2 ~ in7     (input 8x4 byte block)
 *               Outputs - out0, out1, out2, out3  (output 4x8 byte block)
 *               Return Type - RTYPE
 * Details     :
 */
#define MSA_TRANSPOSE8x4_B(RTYPE, in0, in1, in2, in3, in4, in5,  \
                           in6, in7, out0, out1, out2, out3)     \
{                                                                \
    v16i8 zero_m = { 0 };                                        \
                                                                 \
    MSA_ILVR_B4(RTYPE, in2, in0, in3, in1, in6, in4, in7, in5,   \
                out0, out1, out2, out3);                         \
    MSA_ILVR_H2(RTYPE, out2, out0, out3, out1, out2, out3);      \
    out0 = (RTYPE) __msa_ilvr_b((v16i8) out3, (v16i8) out2);     \
    out1 = (RTYPE) __msa_sldi_b(zero_m, (v16i8) out0, 8);        \
    out2 = (RTYPE) __msa_ilvl_b((v16i8) out3, (v16i8) out2);     \
    out3 = (RTYPE) __msa_sldi_b(zero_m, (v16i8) out2, 8);        \
}

/* Description : Transposes 16x4 block into 4x16 with byte elements in vectors.
 * Arguments   : Inputs  - in0, in1, in2, in3, in4, in5, in6, in7,
 *                         in8, in9, in10, in11, in12, in13, in14, in15
 *               Outputs - out0, out1, out2, out3
 *               Return Type - RTYPE
 * Details     :
 */
#define MSA_TRANSPOSE16x4_B(RTYPE, in0, in1, in2, in3, in4, in5, in6, in7,  \
                            in8, in9, in10, in11, in12, in13, in14, in15,   \
                            out0, out1, out2, out3)                         \
{                                                                           \
    v2i64 tmp0_m, tmp1_m, tmp2_m, tmp3_m;                                   \
                                                                            \
    MSA_ILVR_B4(RTYPE, in2, in0, in3, in1, in6, in4, in7, in5,              \
                out0, out1, out2, out3);                                    \
    MSA_ILVR_H2(RTYPE, out2, out0, out3, out1, out2, out3);                 \
    MSA_ILVRL_B2(v2i64, out3, out2, tmp0_m, tmp1_m);                        \
                                                                            \
    MSA_ILVR_B4(RTYPE, in10, in8, in11, in9, in14, in12, in15, in13,        \
                out0, out1, out2, out3);                                    \
    MSA_ILVR_H2(RTYPE, out2, out0, out3, out1, out2, out3);                 \
    MSA_ILVRL_B2(v2i64, out3, out2, tmp2_m, tmp3_m);                        \
                                                                            \
    MSA_ILVRL_D4(RTYPE, tmp2_m, tmp0_m, tmp3_m, tmp1_m,                     \
                 out0, out1, out2, out3);                                   \
}

/* Description : Transposes input 8x8 byte block.
 * Arguments   : Inputs  - in0, in1, in2, in3, in4, in5, in6, in7
 *                         (input 8x8 byte block)
 *               Outputs - out0, out1, out2, out3, out4, out5, out6, out7
 *                         (output 8x8 byte block)
 *               Return Type - RTYPE
 * Details     :
 */
#define MSA_TRANSPOSE8x8_B(RTYPE, in0, in1, in2, in3, in4, in5, in6, in7,   \
                           out0, out1, out2, out3, out4, out5, out6, out7)  \
{                                                                           \
    v16i8 zero_m = {0};                                                     \
                                                                            \
    MSA_ILVR_B4(RTYPE, in2, in0, in3, in1, in6, in4, in7, in5,              \
            out0, out1, out2, out3);                                        \
    MSA_ILVRL_B4(RTYPE, out1, out0, out3, out2, out4, out5, out6, out7);    \
    MSA_ILVRL_W4(RTYPE, out6, out4, out7, out5, out0, out2, out4, out6);    \
    out1 = (RTYPE) __msa_sldi_b(zero_m, (v16i8) out0, 8);                   \
    out3 = (RTYPE) __msa_sldi_b(zero_m, (v16i8) out2, 8);                   \
    out5 = (RTYPE) __msa_sldi_b(zero_m, (v16i8) out4, 8);                   \
    out7 = (RTYPE) __msa_sldi_b(zero_m, (v16i8) out6, 8);                   \
}

/* Description : Transposes 16x8 block into 8x16 with byte elements in vectors.
 * Arguments   : Inputs  - in0, in1, in2, in3, in4, in5, in6, in7,
 *                         in8, in9, in10, in11, in12, in13, in14, in15
 *               Outputs - out0, out1, out2, out3, out4, out5, out6, out7
 *               Return Type - RTYPE
 * Details     :
 */
#define MSA_TRANSPOSE16x8_B(RTYPE, in0, in1, in2, in3, in4, in5, in6, in7,   \
                            in8, in9, in10, in11, in12, in13, in14, in15,    \
                            out0, out1, out2, out3, out4, out5, out6, out7)  \
{                                                                            \
    v16i8 tmp0_m, tmp1_m, tmp2_m, tmp3_m;                                    \
                                                                             \
    MSA_ILVEV_D4(RTYPE, in8, in0, in9, in1, in10, in2, in11, in3,            \
                 out7, out6, out5, out4);                                    \
    MSA_ILVEV_D4(RTYPE, in12, in4, in13, in5, in14, in6, in15, in7,          \
                 out3, out2, out1, out0);                                    \
                                                                             \
    tmp0_m =  __msa_ilvev_b((v16i8) out6, (v16i8) out7);                     \
    tmp1_m =  __msa_ilvod_b((v16i8) out6, (v16i8) out7);                     \
    out6 = (RTYPE) __msa_ilvev_b((v16i8) out4, (v16i8) out5);                \
    out5 = (RTYPE) __msa_ilvod_b((v16i8) out4, (v16i8) out5);                \
    tmp2_m = __msa_ilvev_b((v16i8) out2, (v16i8) out3);                      \
    tmp3_m = __msa_ilvod_b((v16i8) out2, (v16i8) out3);                      \
    out2 = (RTYPE) __msa_ilvev_b((v16i8) out0, (v16i8) out1);                \
    out1 = (RTYPE) __msa_ilvod_b((v16i8) out0, (v16i8) out1);                \
                                                                             \
    MSA_ILVEV_H2(RTYPE, out6, tmp0_m, out2, tmp2_m, out3, out7);             \
    out0 = (RTYPE) __msa_ilvev_w((v4i32) out7, (v4i32) out3);                \
    out4 = (RTYPE) __msa_ilvod_w((v4i32) out7, (v4i32) out3);                \
                                                                             \
    MSA_ILVOD_H2(RTYPE, out6, tmp0_m, out2, tmp2_m, out3, out7);             \
    out2 = (RTYPE) __msa_ilvev_w((v4i32) out7, (v4i32) out3);                \
    out6 = (RTYPE) __msa_ilvod_w((v4i32) out7, (v4i32) out3);                \
                                                                             \
    MSA_ILVOD_H2(v16i8, out5, tmp1_m, out1, tmp3_m, tmp0_m, tmp2_m);         \
    out3 = (RTYPE) __msa_ilvev_w((v4i32) tmp2_m, (v4i32) tmp0_m);            \
    out7 = (RTYPE) __msa_ilvod_w((v4i32) tmp2_m, (v4i32) tmp0_m);            \
                                                                             \
    MSA_ILVEV_H2(v16i8, out5, tmp1_m, out1, tmp3_m, tmp0_m, tmp2_m);         \
    out1 = (RTYPE) __msa_ilvev_w((v4i32) tmp2_m, (v4i32) tmp0_m);            \
    out5 = (RTYPE) __msa_ilvod_w((v4i32) tmp2_m, (v4i32) tmp0_m);            \
}

/* Description : Transposes 4x4 block with half word elements in vectors.
 * Arguments   : Inputs  - in0, in1, in2, in3
 *               Outputs - out0, out1, out2, out3
 *               Return Type - RTYPE
 * Details     :
 */
#define MSA_TRANSPOSE4x4_H(RTYPE, in0, in1, in2, in3,         \
                           out0, out1, out2, out3)            \
{                                                             \
    MSA_ILVR_H2(RTYPE, in1, in0, in3, in2, out1, out3);       \
    MSA_ILVRL_W2(RTYPE, out3, out1, out0, out2);              \
    MSA_ILVL_D2(RTYPE, out0, out0, out2, out2, out1, out3);   \
}

/* Description : Transposes 8x4 block with half word elements in vectors.
 * Arguments   : Inputs  - in0, in1, in2, in3, in4, in5, in6, in7
 *               Outputs - out0, out1, out2, out3
 *               Return Type - RTYPE
 * Details     :
 */
#define MSA_TRANSPOSE8x4_H(RTYPE, in0, in1, in2, in3, in4, in5, in6, in7,   \
                           out0, out1, out2, out3)                          \
{                                                                           \
    v8i16 s0_m, s1_m;                                                       \
    v8i16 tmp0_m, tmp1_m, tmp2_m, tmp3_m;                                   \
                                                                            \
    MSA_ILVR_H2(v8i16, in6, in4, in7, in5, s0_m, s1_m);                     \
    MSA_ILVRL_H2(v8i16, s1_m, s0_m, tmp0_m, tmp1_m);                        \
    MSA_ILVR_H2(v8i16, in2, in0, in3, in1, s0_m, s1_m);                     \
    MSA_ILVRL_H2(v8i16, s1_m, s0_m, tmp2_m, tmp3_m);                        \
    MSA_PCKEV_D2(RTYPE, tmp0_m, tmp2_m, tmp1_m, tmp3_m, out0, out2);        \
    MSA_PCKOD_D2(RTYPE, tmp0_m, tmp2_m, tmp1_m, tmp3_m, out1, out3);        \
}

/* Description : Transposes 8x8 block with half word elements in vectors.
 * Arguments   : Inputs  - in0, in1, in2, in3, in4, in5, in6, in7
 *               Outputs - out0, out1, out2, out3, out4, out5, out6, out7
 *               Return Type - RTYPE
 * Details     :
 */
#define MSA_TRANSPOSE8x8_H(RTYPE, in0, in1, in2, in3, in4, in5, in6, in7,   \
                           out0, out1, out2, out3, out4, out5, out6, out7)  \
{                                                                           \
    v8i16 s0_m, s1_m;                                                       \
    v8i16 tmp0_m, tmp1_m, tmp2_m, tmp3_m;                                   \
    v8i16 tmp4_m, tmp5_m, tmp6_m, tmp7_m;                                   \
                                                                            \
    MSA_ILVR_H2(v8i16, in6, in4, in7, in5, s0_m, s1_m);                     \
    MSA_ILVRL_H2(v8i16, s1_m, s0_m, tmp0_m, tmp1_m);                        \
    MSA_ILVL_H2(v8i16, in6, in4, in7, in5, s0_m, s1_m);                     \
    MSA_ILVRL_H2(v8i16, s1_m, s0_m, tmp2_m, tmp3_m);                        \
    MSA_ILVR_H2(v8i16, in2, in0, in3, in1, s0_m, s1_m);                     \
    MSA_ILVRL_H2(v8i16, s1_m, s0_m, tmp4_m, tmp5_m);                        \
    MSA_ILVL_H2(v8i16, in2, in0, in3, in1, s0_m, s1_m);                     \
    MSA_ILVRL_H2(v8i16, s1_m, s0_m, tmp6_m, tmp7_m);                        \
    MSA_PCKEV_D4(RTYPE, tmp0_m, tmp4_m, tmp1_m, tmp5_m, tmp2_m, tmp6_m,     \
             tmp3_m, tmp7_m, out0, out2, out4, out6);                       \
    MSA_PCKOD_D4(RTYPE, tmp0_m, tmp4_m, tmp1_m, tmp5_m, tmp2_m, tmp6_m,     \
             tmp3_m, tmp7_m, out1, out3, out5, out7);                       \
}

#endif /* _MSA_MACROS_H */
