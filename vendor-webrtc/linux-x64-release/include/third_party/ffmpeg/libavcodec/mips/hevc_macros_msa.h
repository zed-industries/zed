/*
 * Copyright (c) 2015 - 2017 Manojkumar Bhosale (Manojkumar.Bhosale@imgtec.com)
 *
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with FFmpeg; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

#ifndef AVCODEC_MIPS_HEVC_MACROS_MSA_H
#define AVCODEC_MIPS_HEVC_MACROS_MSA_H

#define HEVC_FILT_8TAP_SH(in0, in1, in2, in3,                    \
                          filt0, filt1, filt2, filt3)            \
( {                                                              \
    v8i16 out_m;                                                 \
                                                                 \
    out_m = __msa_dotp_s_h((v16i8) in0, (v16i8) filt0);          \
    out_m = __msa_dpadd_s_h(out_m, (v16i8) in1, (v16i8) filt1);  \
    DPADD_SB2_SH(in2, in3, filt2, filt3, out_m, out_m);          \
    out_m;                                                       \
} )

#define HEVC_FILT_8TAP(in0, in1, in2, in3,                       \
                       filt0, filt1, filt2, filt3)               \
( {                                                              \
    v4i32 out_m;                                                 \
                                                                 \
    out_m = __msa_dotp_s_w((v8i16) in0, (v8i16) filt0);          \
    out_m = __msa_dpadd_s_w(out_m, (v8i16) in1, (v8i16) filt1);  \
    DPADD_SH2_SW(in2, in3, filt2, filt3, out_m, out_m);          \
    out_m;                                                       \
} )

#define HEVC_FILT_4TAP_SH(in0, in1, filt0, filt1)                \
( {                                                              \
    v8i16 out_m;                                                 \
                                                                 \
    out_m = __msa_dotp_s_h((v16i8) in0, (v16i8) filt0);          \
    out_m = __msa_dpadd_s_h(out_m, (v16i8) in1, (v16i8) filt1);  \
    out_m;                                                       \
} )

#define HEVC_FILT_4TAP_SW(in0, in1, filt0, filt1)                \
( {                                                              \
    v4i32 out_m;                                                 \
                                                                 \
    out_m = __msa_dotp_s_w((v8i16) in0, (v8i16) filt0);          \
    out_m = __msa_dpadd_s_w(out_m, (v8i16) in1, (v8i16) filt1);  \
    out_m;                                                       \
} )

#define HEVC_FILT_4TAP(in0, in1, filt0, filt1)           \
( {                                                      \
    v4i32 out_m;                                         \
                                                         \
    out_m = __msa_dotp_s_w(in0, (v8i16) filt0);          \
    out_m = __msa_dpadd_s_w(out_m, in1, (v8i16) filt1);  \
    out_m;                                               \
} )

#endif  /* AVCODEC_MIPS_HEVC_MACROS_MSA_H */
