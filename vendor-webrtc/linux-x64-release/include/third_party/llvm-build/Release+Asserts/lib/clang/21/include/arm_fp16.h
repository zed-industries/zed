/*===---- arm_fp16.h - ARM FP16 intrinsics ---------------------------------===
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 *
 *===-----------------------------------------------------------------------===
 */

#ifndef __ARM_FP16_H
#define __ARM_FP16_H

#include <stdint.h>

typedef __fp16 float16_t;
#define __ai static __inline__ __attribute__((__always_inline__, __nodebug__))

#if defined(__aarch64__) || defined(__arm64ec__)
#define vabdh_f16(__p0, __p1) __extension__ ({ \
  float16_t __ret; \
  float16_t __s0 = __p0; \
  float16_t __s1 = __p1; \
  __ret = __builtin_bit_cast(float16_t, __builtin_neon_vabdh_f16(__s0, __s1)); \
  __ret; \
})
#define vabsh_f16(__p0) __extension__ ({ \
  float16_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(float16_t, __builtin_neon_vabsh_f16(__s0)); \
  __ret; \
})
#define vaddh_f16(__p0, __p1) __extension__ ({ \
  float16_t __ret; \
  float16_t __s0 = __p0; \
  float16_t __s1 = __p1; \
  __ret = __builtin_bit_cast(float16_t, __builtin_neon_vaddh_f16(__s0, __s1)); \
  __ret; \
})
#define vcageh_f16(__p0, __p1) __extension__ ({ \
  uint16_t __ret; \
  float16_t __s0 = __p0; \
  float16_t __s1 = __p1; \
  __ret = __builtin_bit_cast(uint16_t, __builtin_neon_vcageh_f16(__s0, __s1)); \
  __ret; \
})
#define vcagth_f16(__p0, __p1) __extension__ ({ \
  uint16_t __ret; \
  float16_t __s0 = __p0; \
  float16_t __s1 = __p1; \
  __ret = __builtin_bit_cast(uint16_t, __builtin_neon_vcagth_f16(__s0, __s1)); \
  __ret; \
})
#define vcaleh_f16(__p0, __p1) __extension__ ({ \
  uint16_t __ret; \
  float16_t __s0 = __p0; \
  float16_t __s1 = __p1; \
  __ret = __builtin_bit_cast(uint16_t, __builtin_neon_vcaleh_f16(__s0, __s1)); \
  __ret; \
})
#define vcalth_f16(__p0, __p1) __extension__ ({ \
  uint16_t __ret; \
  float16_t __s0 = __p0; \
  float16_t __s1 = __p1; \
  __ret = __builtin_bit_cast(uint16_t, __builtin_neon_vcalth_f16(__s0, __s1)); \
  __ret; \
})
#define vceqh_f16(__p0, __p1) __extension__ ({ \
  uint16_t __ret; \
  float16_t __s0 = __p0; \
  float16_t __s1 = __p1; \
  __ret = __builtin_bit_cast(uint16_t, __builtin_neon_vceqh_f16(__s0, __s1)); \
  __ret; \
})
#define vceqzh_f16(__p0) __extension__ ({ \
  uint16_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(uint16_t, __builtin_neon_vceqzh_f16(__s0)); \
  __ret; \
})
#define vcgeh_f16(__p0, __p1) __extension__ ({ \
  uint16_t __ret; \
  float16_t __s0 = __p0; \
  float16_t __s1 = __p1; \
  __ret = __builtin_bit_cast(uint16_t, __builtin_neon_vcgeh_f16(__s0, __s1)); \
  __ret; \
})
#define vcgezh_f16(__p0) __extension__ ({ \
  uint16_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(uint16_t, __builtin_neon_vcgezh_f16(__s0)); \
  __ret; \
})
#define vcgth_f16(__p0, __p1) __extension__ ({ \
  uint16_t __ret; \
  float16_t __s0 = __p0; \
  float16_t __s1 = __p1; \
  __ret = __builtin_bit_cast(uint16_t, __builtin_neon_vcgth_f16(__s0, __s1)); \
  __ret; \
})
#define vcgtzh_f16(__p0) __extension__ ({ \
  uint16_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(uint16_t, __builtin_neon_vcgtzh_f16(__s0)); \
  __ret; \
})
#define vcleh_f16(__p0, __p1) __extension__ ({ \
  uint16_t __ret; \
  float16_t __s0 = __p0; \
  float16_t __s1 = __p1; \
  __ret = __builtin_bit_cast(uint16_t, __builtin_neon_vcleh_f16(__s0, __s1)); \
  __ret; \
})
#define vclezh_f16(__p0) __extension__ ({ \
  uint16_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(uint16_t, __builtin_neon_vclezh_f16(__s0)); \
  __ret; \
})
#define vclth_f16(__p0, __p1) __extension__ ({ \
  uint16_t __ret; \
  float16_t __s0 = __p0; \
  float16_t __s1 = __p1; \
  __ret = __builtin_bit_cast(uint16_t, __builtin_neon_vclth_f16(__s0, __s1)); \
  __ret; \
})
#define vcltzh_f16(__p0) __extension__ ({ \
  uint16_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(uint16_t, __builtin_neon_vcltzh_f16(__s0)); \
  __ret; \
})
#define vcvth_n_s16_f16(__p0, __p1) __extension__ ({ \
  int16_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(int16_t, __builtin_neon_vcvth_n_s16_f16(__s0, __p1)); \
  __ret; \
})
#define vcvth_n_s32_f16(__p0, __p1) __extension__ ({ \
  int32_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(int32_t, __builtin_neon_vcvth_n_s32_f16(__s0, __p1)); \
  __ret; \
})
#define vcvth_n_s64_f16(__p0, __p1) __extension__ ({ \
  int64_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(int64_t, __builtin_neon_vcvth_n_s64_f16(__s0, __p1)); \
  __ret; \
})
#define vcvth_n_u16_f16(__p0, __p1) __extension__ ({ \
  uint16_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(uint16_t, __builtin_neon_vcvth_n_u16_f16(__s0, __p1)); \
  __ret; \
})
#define vcvth_n_u32_f16(__p0, __p1) __extension__ ({ \
  uint32_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(uint32_t, __builtin_neon_vcvth_n_u32_f16(__s0, __p1)); \
  __ret; \
})
#define vcvth_n_u64_f16(__p0, __p1) __extension__ ({ \
  uint64_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(uint64_t, __builtin_neon_vcvth_n_u64_f16(__s0, __p1)); \
  __ret; \
})
#define vcvth_s16_f16(__p0) __extension__ ({ \
  int16_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(int16_t, __builtin_neon_vcvth_s16_f16(__s0)); \
  __ret; \
})
#define vcvth_s32_f16(__p0) __extension__ ({ \
  int32_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(int32_t, __builtin_neon_vcvth_s32_f16(__s0)); \
  __ret; \
})
#define vcvth_s64_f16(__p0) __extension__ ({ \
  int64_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(int64_t, __builtin_neon_vcvth_s64_f16(__s0)); \
  __ret; \
})
#define vcvth_u16_f16(__p0) __extension__ ({ \
  uint16_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(uint16_t, __builtin_neon_vcvth_u16_f16(__s0)); \
  __ret; \
})
#define vcvth_u32_f16(__p0) __extension__ ({ \
  uint32_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(uint32_t, __builtin_neon_vcvth_u32_f16(__s0)); \
  __ret; \
})
#define vcvth_u64_f16(__p0) __extension__ ({ \
  uint64_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(uint64_t, __builtin_neon_vcvth_u64_f16(__s0)); \
  __ret; \
})
#define vcvtah_s16_f16(__p0) __extension__ ({ \
  int16_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(int16_t, __builtin_neon_vcvtah_s16_f16(__s0)); \
  __ret; \
})
#define vcvtah_s32_f16(__p0) __extension__ ({ \
  int32_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(int32_t, __builtin_neon_vcvtah_s32_f16(__s0)); \
  __ret; \
})
#define vcvtah_s64_f16(__p0) __extension__ ({ \
  int64_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(int64_t, __builtin_neon_vcvtah_s64_f16(__s0)); \
  __ret; \
})
#define vcvtah_u16_f16(__p0) __extension__ ({ \
  uint16_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(uint16_t, __builtin_neon_vcvtah_u16_f16(__s0)); \
  __ret; \
})
#define vcvtah_u32_f16(__p0) __extension__ ({ \
  uint32_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(uint32_t, __builtin_neon_vcvtah_u32_f16(__s0)); \
  __ret; \
})
#define vcvtah_u64_f16(__p0) __extension__ ({ \
  uint64_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(uint64_t, __builtin_neon_vcvtah_u64_f16(__s0)); \
  __ret; \
})
#define vcvth_f16_u16(__p0) __extension__ ({ \
  float16_t __ret; \
  uint16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(float16_t, __builtin_neon_vcvth_f16_u16(__s0)); \
  __ret; \
})
#define vcvth_f16_s16(__p0) __extension__ ({ \
  float16_t __ret; \
  int16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(float16_t, __builtin_neon_vcvth_f16_s16(__s0)); \
  __ret; \
})
#define vcvth_f16_u32(__p0) __extension__ ({ \
  float16_t __ret; \
  uint32_t __s0 = __p0; \
  __ret = __builtin_bit_cast(float16_t, __builtin_neon_vcvth_f16_u32(__s0)); \
  __ret; \
})
#define vcvth_f16_s32(__p0) __extension__ ({ \
  float16_t __ret; \
  int32_t __s0 = __p0; \
  __ret = __builtin_bit_cast(float16_t, __builtin_neon_vcvth_f16_s32(__s0)); \
  __ret; \
})
#define vcvth_f16_u64(__p0) __extension__ ({ \
  float16_t __ret; \
  uint64_t __s0 = __p0; \
  __ret = __builtin_bit_cast(float16_t, __builtin_neon_vcvth_f16_u64(__s0)); \
  __ret; \
})
#define vcvth_f16_s64(__p0) __extension__ ({ \
  float16_t __ret; \
  int64_t __s0 = __p0; \
  __ret = __builtin_bit_cast(float16_t, __builtin_neon_vcvth_f16_s64(__s0)); \
  __ret; \
})
#define vcvth_n_f16_u32(__p0, __p1) __extension__ ({ \
  float16_t __ret; \
  uint32_t __s0 = __p0; \
  __ret = __builtin_bit_cast(float16_t, __builtin_neon_vcvth_n_f16_u32(__s0, __p1)); \
  __ret; \
})
#define vcvth_n_f16_s32(__p0, __p1) __extension__ ({ \
  float16_t __ret; \
  int32_t __s0 = __p0; \
  __ret = __builtin_bit_cast(float16_t, __builtin_neon_vcvth_n_f16_s32(__s0, __p1)); \
  __ret; \
})
#define vcvth_n_f16_u64(__p0, __p1) __extension__ ({ \
  float16_t __ret; \
  uint64_t __s0 = __p0; \
  __ret = __builtin_bit_cast(float16_t, __builtin_neon_vcvth_n_f16_u64(__s0, __p1)); \
  __ret; \
})
#define vcvth_n_f16_s64(__p0, __p1) __extension__ ({ \
  float16_t __ret; \
  int64_t __s0 = __p0; \
  __ret = __builtin_bit_cast(float16_t, __builtin_neon_vcvth_n_f16_s64(__s0, __p1)); \
  __ret; \
})
#define vcvth_n_f16_u16(__p0, __p1) __extension__ ({ \
  float16_t __ret; \
  uint16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(float16_t, __builtin_neon_vcvth_n_f16_u16(__s0, __p1)); \
  __ret; \
})
#define vcvth_n_f16_s16(__p0, __p1) __extension__ ({ \
  float16_t __ret; \
  int16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(float16_t, __builtin_neon_vcvth_n_f16_s16(__s0, __p1)); \
  __ret; \
})
#define vcvtmh_s16_f16(__p0) __extension__ ({ \
  int16_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(int16_t, __builtin_neon_vcvtmh_s16_f16(__s0)); \
  __ret; \
})
#define vcvtmh_s32_f16(__p0) __extension__ ({ \
  int32_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(int32_t, __builtin_neon_vcvtmh_s32_f16(__s0)); \
  __ret; \
})
#define vcvtmh_s64_f16(__p0) __extension__ ({ \
  int64_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(int64_t, __builtin_neon_vcvtmh_s64_f16(__s0)); \
  __ret; \
})
#define vcvtmh_u16_f16(__p0) __extension__ ({ \
  uint16_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(uint16_t, __builtin_neon_vcvtmh_u16_f16(__s0)); \
  __ret; \
})
#define vcvtmh_u32_f16(__p0) __extension__ ({ \
  uint32_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(uint32_t, __builtin_neon_vcvtmh_u32_f16(__s0)); \
  __ret; \
})
#define vcvtmh_u64_f16(__p0) __extension__ ({ \
  uint64_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(uint64_t, __builtin_neon_vcvtmh_u64_f16(__s0)); \
  __ret; \
})
#define vcvtnh_s16_f16(__p0) __extension__ ({ \
  int16_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(int16_t, __builtin_neon_vcvtnh_s16_f16(__s0)); \
  __ret; \
})
#define vcvtnh_s32_f16(__p0) __extension__ ({ \
  int32_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(int32_t, __builtin_neon_vcvtnh_s32_f16(__s0)); \
  __ret; \
})
#define vcvtnh_s64_f16(__p0) __extension__ ({ \
  int64_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(int64_t, __builtin_neon_vcvtnh_s64_f16(__s0)); \
  __ret; \
})
#define vcvtnh_u16_f16(__p0) __extension__ ({ \
  uint16_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(uint16_t, __builtin_neon_vcvtnh_u16_f16(__s0)); \
  __ret; \
})
#define vcvtnh_u32_f16(__p0) __extension__ ({ \
  uint32_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(uint32_t, __builtin_neon_vcvtnh_u32_f16(__s0)); \
  __ret; \
})
#define vcvtnh_u64_f16(__p0) __extension__ ({ \
  uint64_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(uint64_t, __builtin_neon_vcvtnh_u64_f16(__s0)); \
  __ret; \
})
#define vcvtph_s16_f16(__p0) __extension__ ({ \
  int16_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(int16_t, __builtin_neon_vcvtph_s16_f16(__s0)); \
  __ret; \
})
#define vcvtph_s32_f16(__p0) __extension__ ({ \
  int32_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(int32_t, __builtin_neon_vcvtph_s32_f16(__s0)); \
  __ret; \
})
#define vcvtph_s64_f16(__p0) __extension__ ({ \
  int64_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(int64_t, __builtin_neon_vcvtph_s64_f16(__s0)); \
  __ret; \
})
#define vcvtph_u16_f16(__p0) __extension__ ({ \
  uint16_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(uint16_t, __builtin_neon_vcvtph_u16_f16(__s0)); \
  __ret; \
})
#define vcvtph_u32_f16(__p0) __extension__ ({ \
  uint32_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(uint32_t, __builtin_neon_vcvtph_u32_f16(__s0)); \
  __ret; \
})
#define vcvtph_u64_f16(__p0) __extension__ ({ \
  uint64_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(uint64_t, __builtin_neon_vcvtph_u64_f16(__s0)); \
  __ret; \
})
#define vdivh_f16(__p0, __p1) __extension__ ({ \
  float16_t __ret; \
  float16_t __s0 = __p0; \
  float16_t __s1 = __p1; \
  __ret = __builtin_bit_cast(float16_t, __builtin_neon_vdivh_f16(__s0, __s1)); \
  __ret; \
})
#define vfmah_f16(__p0, __p1, __p2) __extension__ ({ \
  float16_t __ret; \
  float16_t __s0 = __p0; \
  float16_t __s1 = __p1; \
  float16_t __s2 = __p2; \
  __ret = __builtin_bit_cast(float16_t, __builtin_neon_vfmah_f16(__s0, __s1, __s2)); \
  __ret; \
})
#define vfmsh_f16(__p0, __p1, __p2) __extension__ ({ \
  float16_t __ret; \
  float16_t __s0 = __p0; \
  float16_t __s1 = __p1; \
  float16_t __s2 = __p2; \
  __ret = __builtin_bit_cast(float16_t, __builtin_neon_vfmsh_f16(__s0, __s1, __s2)); \
  __ret; \
})
#define vmaxh_f16(__p0, __p1) __extension__ ({ \
  float16_t __ret; \
  float16_t __s0 = __p0; \
  float16_t __s1 = __p1; \
  __ret = __builtin_bit_cast(float16_t, __builtin_neon_vmaxh_f16(__s0, __s1)); \
  __ret; \
})
#define vmaxnmh_f16(__p0, __p1) __extension__ ({ \
  float16_t __ret; \
  float16_t __s0 = __p0; \
  float16_t __s1 = __p1; \
  __ret = __builtin_bit_cast(float16_t, __builtin_neon_vmaxnmh_f16(__s0, __s1)); \
  __ret; \
})
#define vminh_f16(__p0, __p1) __extension__ ({ \
  float16_t __ret; \
  float16_t __s0 = __p0; \
  float16_t __s1 = __p1; \
  __ret = __builtin_bit_cast(float16_t, __builtin_neon_vminh_f16(__s0, __s1)); \
  __ret; \
})
#define vminnmh_f16(__p0, __p1) __extension__ ({ \
  float16_t __ret; \
  float16_t __s0 = __p0; \
  float16_t __s1 = __p1; \
  __ret = __builtin_bit_cast(float16_t, __builtin_neon_vminnmh_f16(__s0, __s1)); \
  __ret; \
})
#define vmulh_f16(__p0, __p1) __extension__ ({ \
  float16_t __ret; \
  float16_t __s0 = __p0; \
  float16_t __s1 = __p1; \
  __ret = __builtin_bit_cast(float16_t, __builtin_neon_vmulh_f16(__s0, __s1)); \
  __ret; \
})
#define vmulxh_f16(__p0, __p1) __extension__ ({ \
  float16_t __ret; \
  float16_t __s0 = __p0; \
  float16_t __s1 = __p1; \
  __ret = __builtin_bit_cast(float16_t, __builtin_neon_vmulxh_f16(__s0, __s1)); \
  __ret; \
})
#define vnegh_f16(__p0) __extension__ ({ \
  float16_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(float16_t, __builtin_neon_vnegh_f16(__s0)); \
  __ret; \
})
#define vrecpeh_f16(__p0) __extension__ ({ \
  float16_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(float16_t, __builtin_neon_vrecpeh_f16(__s0)); \
  __ret; \
})
#define vrecpsh_f16(__p0, __p1) __extension__ ({ \
  float16_t __ret; \
  float16_t __s0 = __p0; \
  float16_t __s1 = __p1; \
  __ret = __builtin_bit_cast(float16_t, __builtin_neon_vrecpsh_f16(__s0, __s1)); \
  __ret; \
})
#define vrecpxh_f16(__p0) __extension__ ({ \
  float16_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(float16_t, __builtin_neon_vrecpxh_f16(__s0)); \
  __ret; \
})
#define vrndh_f16(__p0) __extension__ ({ \
  float16_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(float16_t, __builtin_neon_vrndh_f16(__s0)); \
  __ret; \
})
#define vrndah_f16(__p0) __extension__ ({ \
  float16_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(float16_t, __builtin_neon_vrndah_f16(__s0)); \
  __ret; \
})
#define vrndih_f16(__p0) __extension__ ({ \
  float16_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(float16_t, __builtin_neon_vrndih_f16(__s0)); \
  __ret; \
})
#define vrndmh_f16(__p0) __extension__ ({ \
  float16_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(float16_t, __builtin_neon_vrndmh_f16(__s0)); \
  __ret; \
})
#define vrndnh_f16(__p0) __extension__ ({ \
  float16_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(float16_t, __builtin_neon_vrndnh_f16(__s0)); \
  __ret; \
})
#define vrndph_f16(__p0) __extension__ ({ \
  float16_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(float16_t, __builtin_neon_vrndph_f16(__s0)); \
  __ret; \
})
#define vrndxh_f16(__p0) __extension__ ({ \
  float16_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(float16_t, __builtin_neon_vrndxh_f16(__s0)); \
  __ret; \
})
#define vrsqrteh_f16(__p0) __extension__ ({ \
  float16_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(float16_t, __builtin_neon_vrsqrteh_f16(__s0)); \
  __ret; \
})
#define vrsqrtsh_f16(__p0, __p1) __extension__ ({ \
  float16_t __ret; \
  float16_t __s0 = __p0; \
  float16_t __s1 = __p1; \
  __ret = __builtin_bit_cast(float16_t, __builtin_neon_vrsqrtsh_f16(__s0, __s1)); \
  __ret; \
})
#define vsqrth_f16(__p0) __extension__ ({ \
  float16_t __ret; \
  float16_t __s0 = __p0; \
  __ret = __builtin_bit_cast(float16_t, __builtin_neon_vsqrth_f16(__s0)); \
  __ret; \
})
#define vsubh_f16(__p0, __p1) __extension__ ({ \
  float16_t __ret; \
  float16_t __s0 = __p0; \
  float16_t __s1 = __p1; \
  __ret = __builtin_bit_cast(float16_t, __builtin_neon_vsubh_f16(__s0, __s1)); \
  __ret; \
})
#endif

#undef __ai

#endif /* __ARM_FP16_H */
