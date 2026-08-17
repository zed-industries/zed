/* graphene-simd4f.h: SIMD wrappers and operations
 *
 * SPDX-License-Identifier: MIT
 *
 * Copyright 2014  Emmanuele Bassi
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SH1_0 THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 */

#pragma once

#if !defined(GRAPHENE_H_INSIDE) && !defined(GRAPHENE_COMPILATION)
#error "Only graphene.h can be included directly."
#endif

/* needed for memcpy() */
#include <string.h>
#include <math.h>
#include <float.h>

#include "graphene-config.h"
#include "graphene-macros.h"
#include "graphene-version-macros.h"

GRAPHENE_BEGIN_DECLS

/* Platform specific operations */

GRAPHENE_AVAILABLE_IN_1_0
graphene_simd4f_t       graphene_simd4f_init            (float                   x,
                                                         float                   y,
                                                         float                   z,
                                                         float                   w);
GRAPHENE_AVAILABLE_IN_1_0
graphene_simd4f_t       graphene_simd4f_init_zero       (void);
GRAPHENE_AVAILABLE_IN_1_0
graphene_simd4f_t       graphene_simd4f_init_4f         (const float            *v);
GRAPHENE_AVAILABLE_IN_1_0
graphene_simd4f_t       graphene_simd4f_init_3f         (const float            *v);
GRAPHENE_AVAILABLE_IN_1_0
graphene_simd4f_t       graphene_simd4f_init_2f         (const float            *v);

GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_simd4f_dup_4f          (const graphene_simd4f_t s,
                                                         float                  *v);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_simd4f_dup_3f          (const graphene_simd4f_t s,
                                                         float                  *v);
GRAPHENE_AVAILABLE_IN_1_0
void                    graphene_simd4f_dup_2f          (const graphene_simd4f_t s,
                                                         float                  *v);

GRAPHENE_AVAILABLE_IN_1_2
float                   graphene_simd4f_get             (const graphene_simd4f_t s,
                                                         unsigned int            i);
GRAPHENE_AVAILABLE_IN_1_0
float                   graphene_simd4f_get_x           (const graphene_simd4f_t s);
GRAPHENE_AVAILABLE_IN_1_0
float                   graphene_simd4f_get_y           (const graphene_simd4f_t s);
GRAPHENE_AVAILABLE_IN_1_0
float                   graphene_simd4f_get_z           (const graphene_simd4f_t s);
GRAPHENE_AVAILABLE_IN_1_0
float                   graphene_simd4f_get_w           (const graphene_simd4f_t s);

GRAPHENE_AVAILABLE_IN_1_0
graphene_simd4f_t       graphene_simd4f_splat           (float                   v);
GRAPHENE_AVAILABLE_IN_1_0
graphene_simd4f_t       graphene_simd4f_splat_x         (const graphene_simd4f_t s);
GRAPHENE_AVAILABLE_IN_1_0
graphene_simd4f_t       graphene_simd4f_splat_y         (const graphene_simd4f_t s);
GRAPHENE_AVAILABLE_IN_1_0
graphene_simd4f_t       graphene_simd4f_splat_z         (const graphene_simd4f_t s);
GRAPHENE_AVAILABLE_IN_1_0
graphene_simd4f_t       graphene_simd4f_splat_w         (const graphene_simd4f_t s);

GRAPHENE_AVAILABLE_IN_1_0
graphene_simd4f_t       graphene_simd4f_add             (const graphene_simd4f_t a,
                                                         const graphene_simd4f_t b);
GRAPHENE_AVAILABLE_IN_1_0
graphene_simd4f_t       graphene_simd4f_sub             (const graphene_simd4f_t a,
                                                         const graphene_simd4f_t b);
GRAPHENE_AVAILABLE_IN_1_0
graphene_simd4f_t       graphene_simd4f_mul             (const graphene_simd4f_t a,
                                                         const graphene_simd4f_t b);
GRAPHENE_AVAILABLE_IN_1_0
graphene_simd4f_t       graphene_simd4f_div             (const graphene_simd4f_t a,
                                                         const graphene_simd4f_t b);

GRAPHENE_AVAILABLE_IN_1_0
graphene_simd4f_t       graphene_simd4f_sqrt            (const graphene_simd4f_t s);
GRAPHENE_AVAILABLE_IN_1_0
graphene_simd4f_t       graphene_simd4f_reciprocal      (const graphene_simd4f_t s);
GRAPHENE_AVAILABLE_IN_1_0
graphene_simd4f_t       graphene_simd4f_rsqrt           (const graphene_simd4f_t s);

GRAPHENE_AVAILABLE_IN_1_0
graphene_simd4f_t       graphene_simd4f_cross3          (const graphene_simd4f_t a,
                                                         const graphene_simd4f_t b);
GRAPHENE_AVAILABLE_IN_1_0
graphene_simd4f_t       graphene_simd4f_dot3            (const graphene_simd4f_t a,
                                                         const graphene_simd4f_t b);
GRAPHENE_AVAILABLE_IN_1_4
float                   graphene_simd4f_dot3_scalar     (const graphene_simd4f_t a,
                                                         const graphene_simd4f_t b);

GRAPHENE_AVAILABLE_IN_1_0
graphene_simd4f_t       graphene_simd4f_min             (const graphene_simd4f_t a,
                                                         const graphene_simd4f_t b);
GRAPHENE_AVAILABLE_IN_1_0
graphene_simd4f_t       graphene_simd4f_max             (const graphene_simd4f_t a,
                                                         const graphene_simd4f_t b);

GRAPHENE_AVAILABLE_IN_1_0
graphene_simd4f_t       graphene_simd4f_shuffle_wxyz    (const graphene_simd4f_t s);
GRAPHENE_AVAILABLE_IN_1_0
graphene_simd4f_t       graphene_simd4f_shuffle_zwxy    (const graphene_simd4f_t s);
GRAPHENE_AVAILABLE_IN_1_0
graphene_simd4f_t       graphene_simd4f_shuffle_yzwx    (const graphene_simd4f_t s);

GRAPHENE_AVAILABLE_IN_1_0
graphene_simd4f_t       graphene_simd4f_zero_w          (const graphene_simd4f_t s);
GRAPHENE_AVAILABLE_IN_1_0
graphene_simd4f_t       graphene_simd4f_zero_zw         (const graphene_simd4f_t s);

GRAPHENE_AVAILABLE_IN_1_0
graphene_simd4f_t       graphene_simd4f_merge_high      (const graphene_simd4f_t a,
                                                         const graphene_simd4f_t b);
GRAPHENE_AVAILABLE_IN_1_0
graphene_simd4f_t       graphene_simd4f_merge_low       (const graphene_simd4f_t a,
                                                         const graphene_simd4f_t b);
GRAPHENE_AVAILABLE_IN_1_0
graphene_simd4f_t       graphene_simd4f_merge_w         (const graphene_simd4f_t s,
                                                         float                   v);

GRAPHENE_AVAILABLE_IN_1_0
graphene_simd4f_t       graphene_simd4f_flip_sign_0101  (const graphene_simd4f_t s);
GRAPHENE_AVAILABLE_IN_1_0
graphene_simd4f_t       graphene_simd4f_flip_sign_1010  (const graphene_simd4f_t s);

GRAPHENE_AVAILABLE_IN_1_0
bool                    graphene_simd4f_cmp_eq          (const graphene_simd4f_t a,
                                                         const graphene_simd4f_t b);
GRAPHENE_AVAILABLE_IN_1_0
bool                    graphene_simd4f_cmp_neq         (const graphene_simd4f_t a,
                                                         const graphene_simd4f_t b);
GRAPHENE_AVAILABLE_IN_1_2
bool                    graphene_simd4f_cmp_lt          (const graphene_simd4f_t a,
                                                         const graphene_simd4f_t b);
GRAPHENE_AVAILABLE_IN_1_2
bool                    graphene_simd4f_cmp_le          (const graphene_simd4f_t a,
                                                         const graphene_simd4f_t b);
GRAPHENE_AVAILABLE_IN_1_2
bool                    graphene_simd4f_cmp_ge          (const graphene_simd4f_t a,
                                                         const graphene_simd4f_t b);
GRAPHENE_AVAILABLE_IN_1_2
bool                    graphene_simd4f_cmp_gt          (const graphene_simd4f_t a,
                                                         const graphene_simd4f_t b);
GRAPHENE_AVAILABLE_IN_1_0
graphene_simd4f_t       graphene_simd4f_neg             (const graphene_simd4f_t s);

#if !defined(__GI_SCANNER__) && defined(GRAPHENE_USE_SSE)

/* SSE2 implementation of SIMD 4f */

/* Union type used to do single lane reading without memcpy */
typedef union {
  graphene_simd4f_t s;
  float f[4];
} graphene_simd4f_union_t;

/* On GCC, we use __extension__ macros to avoid a static inline */
# if defined(__GNUC__)

/* Use GCC statement __extension__ to inline all these functions */

#  define graphene_simd4f_init(x,y,z,w) \
  (__extension__ ({ \
    (graphene_simd4f_t) { (x), (y), (z), (w) }; \
  }))

#  define graphene_simd4f_init_zero() \
  (__extension__ ({ \
    (graphene_simd4f_t) _mm_setzero_ps(); \
  }))

#  define graphene_simd4f_init_4f(v) \
  (__extension__ ({ \
    (graphene_simd4f_t) _mm_loadu_ps (v); \
  }))

#  define graphene_simd4f_init_3f(v) \
  (__extension__ ({ \
    (graphene_simd4f_t) { (v)[0], (v)[1], (v)[2], 0.f }; \
  }))

#  define graphene_simd4f_init_2f(v) \
  (__extension__ ({ \
    (graphene_simd4f_t) { (v)[0], (v)[1], 0.f, 0.f }; \
  }))

#  define graphene_simd4f_dup_4f(s,v) \
  (__extension__ ({ \
    _mm_storeu_ps ((v), (s)); \
  }))

#  define graphene_simd4f_dup_3f(s,v) \
  (__extension__ ({ \
    memcpy ((v), &(s), sizeof (float) * 3); \
  }))

#  define graphene_simd4f_dup_2f(s,v) \
  (__extension__ ({ \
    memcpy ((v), &(s), sizeof (float) * 2); \
  }))

#  define graphene_simd4f_get(s,i) \
  (__extension__ ({ \
    graphene_simd4f_union_t __u = { (s) }; \
    (float) __u.f[(i)]; \
  }))

#  define graphene_simd4f_get_x(s)      graphene_simd4f_get (s, 0)
#  define graphene_simd4f_get_y(s)      graphene_simd4f_get (s, 1)
#  define graphene_simd4f_get_z(s)      graphene_simd4f_get (s, 2)
#  define graphene_simd4f_get_w(s)      graphene_simd4f_get (s, 3)

#  define graphene_simd4f_splat(v) \
  (__extension__ ({ \
    (graphene_simd4f_t) _mm_set1_ps ((v)); \
  }))

#  define graphene_simd4f_splat_x(v) \
  (__extension__ ({ \
    (graphene_simd4f_t) _mm_shuffle_ps ((v), (v), _MM_SHUFFLE (0, 0, 0, 0)); \
  }))

#  define graphene_simd4f_splat_y(v) \
  (__extension__ ({ \
    (graphene_simd4f_t) _mm_shuffle_ps ((v), (v), _MM_SHUFFLE (1, 1, 1, 1)); \
  }))

#  define graphene_simd4f_splat_z(v) \
  (__extension__ ({ \
    (graphene_simd4f_t) _mm_shuffle_ps ((v), (v), _MM_SHUFFLE (2, 2, 2, 2)); \
  }))

#  define graphene_simd4f_splat_w(v) \
  (__extension__ ({ \
    (graphene_simd4f_t) _mm_shuffle_ps ((v), (v), _MM_SHUFFLE (3, 3, 3, 3)); \
  }))

#  define graphene_simd4f_add(a,b) \
  (__extension__ ({ \
    (graphene_simd4f_t) _mm_add_ps ((a), (b)); \
  }))

#  define graphene_simd4f_sub(a,b) \
  (__extension__ ({ \
    (graphene_simd4f_t) _mm_sub_ps ((a), (b)); \
  }))

#  define graphene_simd4f_mul(a,b) \
  (__extension__ ({ \
    (graphene_simd4f_t) _mm_mul_ps ((a), (b)); \
  }))

#  define graphene_simd4f_div(a,b) \
  (__extension__ ({ \
    (graphene_simd4f_t) _mm_div_ps ((a), (b)); \
  }))

#  define graphene_simd4f_sqrt(v) \
  (__extension__ ({ \
    (graphene_simd4f_t) _mm_sqrt_ps ((v)); \
  }))

#  define graphene_simd4f_reciprocal(v) \
  (__extension__ ({ \
    const graphene_simd4f_t __two = graphene_simd4f_init (2.0f, 2.0f, 2.0f, 2.0f); \
    graphene_simd4f_t __s = _mm_rcp_ps ((v)); \
    graphene_simd4f_mul (__s, graphene_simd4f_sub (__two, graphene_simd4f_mul ((v), __s))); \
  }))

#  define graphene_simd4f_rsqrt(v) \
  (__extension__ ({ \
    const graphene_simd4f_t __half = graphene_simd4f_init (0.5f, 0.5f, 0.5f, 0.5f); \
    const graphene_simd4f_t __three = graphene_simd4f_init (3.0f, 3.0f, 3.0f, 3.0f); \
    graphene_simd4f_t __s = _mm_rsqrt_ps ((v)); \
    graphene_simd4f_mul (graphene_simd4f_mul (__s, __half), \
                         graphene_simd4f_sub (__three, \
                                              graphene_simd4f_mul (__s, graphene_simd4f_mul ((v), __s)))); \
  }))

#  define graphene_simd4f_cross3(a,b) \
  (__extension__ ({ \
    const graphene_simd4f_t __a_yzx = _mm_shuffle_ps ((a), (a), _MM_SHUFFLE (3, 0, 2, 1)); \
    const graphene_simd4f_t __a_zxy = _mm_shuffle_ps ((a), (a), _MM_SHUFFLE (3, 1, 0, 2)); \
    const graphene_simd4f_t __b_yzx = _mm_shuffle_ps ((b), (b), _MM_SHUFFLE (3, 0, 2, 1)); \
    const graphene_simd4f_t __b_zxy = _mm_shuffle_ps ((b), (b), _MM_SHUFFLE (3, 1, 0, 2)); \
    (graphene_simd4f_t) _mm_sub_ps (_mm_mul_ps (__a_yzx, __b_zxy), _mm_mul_ps (__a_zxy, __b_yzx)); \
  }))

#  if defined(GRAPHENE_USE_SSE4_1)
#   define graphene_simd4f_dot3(a,b) \
  (__extension__ ({ \
    (graphene_simd4f_t) _mm_dp_ps ((a), (b), 0x7f); \
  }))
#  else
#   define graphene_simd4f_dot3(a,b) \
  (__extension__ ({ \
    const unsigned int __mask_bits[] GRAPHENE_ALIGN16 = { 0xffffffff, 0xffffffff, 0xffffffff, 0 }; \
    const graphene_simd4f_t __mask = _mm_load_ps ((const float *) __mask_bits); \
    const graphene_simd4f_t __m = _mm_mul_ps ((a), (b)); \
    const graphene_simd4f_t __s0 = _mm_and_ps (__m, __mask); \
    const graphene_simd4f_t __s1 = _mm_add_ps (__s0, _mm_movehl_ps (__s0, __s0)); \
    const graphene_simd4f_t __s2 = _mm_add_ss (__s1, _mm_shuffle_ps (__s1, __s1, 1)); \
    (graphene_simd4f_t) _mm_shuffle_ps (__s2, __s2, 0); \
  }))
#  endif

#  define graphene_simd4f_dot3_scalar(a,b) \
  (__extension__ ({ \
    float __res; \
    _mm_store_ss (&__res, graphene_simd4f_dot3 (a, b)); \
    __res; \
  }))

#  define graphene_simd4f_min(a,b) \
  (__extension__ ({ \
    (graphene_simd4f_t) _mm_min_ps ((a), (b)); \
  }))

#  define graphene_simd4f_max(a,b) \
  (__extension__ ({ \
    (graphene_simd4f_t) _mm_max_ps ((a), (b)); \
  }))

#  define graphene_simd4f_shuffle_wxyz(v) \
  (__extension__ ({ \
    (graphene_simd4f_t) _mm_shuffle_ps ((v), (v), _MM_SHUFFLE (2, 1, 0, 3)); \
  }))

#  define graphene_simd4f_shuffle_zwxy(v) \
  (__extension__ ({ \
    (graphene_simd4f_t) _mm_shuffle_ps ((v), (v), _MM_SHUFFLE (1, 0, 3, 2)); \
  }))

#  define graphene_simd4f_shuffle_yzwx(v) \
  (__extension__ ({ \
    (graphene_simd4f_t) _mm_shuffle_ps ((v), (v), _MM_SHUFFLE (0, 3, 2, 1)); \
  }))

#  define graphene_simd4f_zero_w(v) \
  (__extension__ ({ \
    graphene_simd4f_t __s = _mm_unpackhi_ps ((v), _mm_setzero_ps ()); \
    (graphene_simd4f_t) _mm_movelh_ps ((v), __s); \
  }))

#  define graphene_simd4f_zero_zw(v) \
  (__extension__ ({ \
    (graphene_simd4f_t) _mm_movelh_ps ((v), _mm_setzero_ps ()); \
  }))

#  define graphene_simd4f_merge_w(s,v) \
  (__extension__ ({ \
    graphene_simd4f_t __s = _mm_unpackhi_ps ((s), _mm_set1_ps ((v))); \
    (graphene_simd4f_t) _mm_movelh_ps ((s), __s); \
  }))

#  define graphene_simd4f_merge_high(a,b) \
  (__extension__ ({ \
    (graphene_simd4f_t) _mm_movehl_ps ((b), (a)); \
  }))

#  define graphene_simd4f_merge_low(a,b) \
  (__extension__ ({ \
    (graphene_simd4f_t) _mm_movelh_ps ((a), (b)); \
  }))

typedef GRAPHENE_ALIGN16 union {
  unsigned int ui[4];
  float f[4];
} graphene_simd4f_uif_t;

#  define graphene_simd4f_flip_sign_0101(v) \
  (__extension__ ({ \
    const graphene_simd4f_uif_t __pnpn = { { \
      0x00000000, \
      0x80000000, \
      0x00000000, \
      0x80000000  \
    } }; \
    (graphene_simd4f_t) _mm_xor_ps ((v), _mm_load_ps (__pnpn.f)); \
  }))

#  define graphene_simd4f_flip_sign_1010(v) \
  (__extension__ ({ \
    const graphene_simd4f_uif_t __npnp = { { \
      0x80000000, \
      0x00000000, \
      0x80000000, \
      0x00000000, \
    } }; \
    (graphene_simd4f_t) _mm_xor_ps ((v), _mm_load_ps (__npnp.f)); \
  }))

#  define graphene_simd4f_cmp_eq(a,b) \
  (__extension__ ({ \
    __m128i __res = (__m128i) _mm_cmpneq_ps ((a), (b)); \
    (bool) (_mm_movemask_epi8 (__res) == 0); \
  }))

#  define graphene_simd4f_cmp_neq(a,b) \
  (__extension__ ({ \
    __m128i __res = (__m128i) _mm_cmpneq_ps ((a), (b)); \
    (bool) (_mm_movemask_epi8 (__res) != 0); \
  }))

#  define graphene_simd4f_cmp_lt(a,b) \
  (__extension__ ({ \
    __m128i __res = (__m128i) _mm_cmplt_ps ((a), (b)); \
    (bool) (_mm_movemask_epi8 (__res) == 0xffff); \
  }))

#  define graphene_simd4f_cmp_le(a,b) \
  (__extension__ ({ \
    __m128i __res = (__m128i) _mm_cmple_ps ((a), (b)); \
    (bool) (_mm_movemask_epi8 (__res) == 0xffff); \
  }))

#  define graphene_simd4f_cmp_ge(a,b) \
  (__extension__ ({ \
    __m128i __res = (__m128i) _mm_cmpge_ps ((a), (b)); \
    (bool) (_mm_movemask_epi8 (__res) == 0xffff); \
  }))

#  define graphene_simd4f_cmp_gt(a,b) \
  (__extension__ ({ \
    __m128i __res = (__m128i) _mm_cmpgt_ps ((a), (b)); \
    (bool) (_mm_movemask_epi8 (__res) == 0xffff); \
  }))

#  define graphene_simd4f_neg(s) \
  (__extension__ ({ \
    const graphene_simd4f_uif_t __mask = { { \
      0x80000000, \
      0x80000000, \
      0x80000000, \
      0x80000000, \
    } }; \
    (graphene_simd4f_t) _mm_xor_ps ((s), _mm_load_ps (__mask.f)); \
  }))

/* On MSVC, we use static inlines */
# elif defined (_MSC_VER) /* Visual Studio SSE intrinsics */

/* Use static inline to inline all these functions */

#define graphene_simd4f_init(x,y,z,w) _simd4f_init(x,y,z,w)

static inline graphene_simd4f_t
_simd4f_init (float x, float y, float z, float w)
{
  graphene_simd4f_t __s = { x, y, z, w };
  return __s;
}

#define graphene_simd4f_init_zero() \
  _mm_setzero_ps()

#define graphene_simd4f_init_4f(v) \
  _mm_loadu_ps(v)

#define graphene_simd4f_init_3f(v) \
  graphene_simd4f_init (v[0], v[1], v[2], 0.f)

#define graphene_simd4f_init_2f(v) \
  graphene_simd4f_init (v[0], v[1], 0.f, 0.f)

#define graphene_simd4f_dup_4f(s,v) \
  _mm_storeu_ps (v, s)

#define graphene_simd4f_dup_3f(s,v) \
  memcpy (v, &s, sizeof (float) * 3)

#define graphene_simd4f_dup_2f(s,v) \
  memcpy (v, &s, sizeof (float) * 2)

#define graphene_simd4f_get(s,i) _simd4f_get_xyzw(s, i)
#define graphene_simd4f_get_x(s) _simd4f_get_xyzw(s, 0)
#define graphene_simd4f_get_y(s) _simd4f_get_xyzw(s, 1)
#define graphene_simd4f_get_z(s) _simd4f_get_xyzw(s, 2)
#define graphene_simd4f_get_w(s) _simd4f_get_xyzw(s, 3)

static inline float
_simd4f_get_xyzw (graphene_simd4f_t s, int mode)
{
  /* mode: get_x=0
           get_y=1
           get_z=2
           get_w=3 */

  graphene_simd4f_union_t u;
  u.s = s;
  return u.f[mode];
}

#define graphene_simd4f_splat(v) \
  _mm_set1_ps (v)

#define graphene_simd4f_splat_x(v) \
  _mm_shuffle_ps (v, v, _MM_SHUFFLE (0, 0, 0, 0))

#define graphene_simd4f_splat_y(v) \
  _mm_shuffle_ps (v, v, _MM_SHUFFLE (1, 1, 1, 1))

#define graphene_simd4f_splat_z(v) \
  _mm_shuffle_ps (v, v, _MM_SHUFFLE (2, 2, 2, 2))

#define graphene_simd4f_splat_w(v) \
  _mm_shuffle_ps (v, v, _MM_SHUFFLE (3, 3, 3, 3))

#define graphene_simd4f_add(a,b) \
  _mm_add_ps (a, b)

#define graphene_simd4f_sub(a,b) \
  _mm_sub_ps (a, b)

#define graphene_simd4f_mul(a,b) \
  _mm_mul_ps (a, b)

#define graphene_simd4f_div(a,b) \
  _mm_div_ps (a, b)

#define graphene_simd4f_sqrt(v) \
  _mm_sqrt_ps (v)

#define graphene_simd4f_reciprocal(v) _simd4f_reciprocal(v)

static inline graphene_simd4f_t
_simd4f_reciprocal(const graphene_simd4f_t v)
{
  const graphene_simd4f_t __two = graphene_simd4f_init (2.0f, 2.0f, 2.0f, 2.0f);
  graphene_simd4f_t __s = _mm_rcp_ps (v);
  return graphene_simd4f_mul (__s,
                              graphene_simd4f_sub (__two,
                                                   graphene_simd4f_mul (v, __s)));
}

#define graphene_simd4f_rsqrt(v) _simd4f_rsqrt(v)

static inline graphene_simd4f_t
_simd4f_rsqrt(const graphene_simd4f_t v)
{
  const graphene_simd4f_t __half = graphene_simd4f_init (0.5f, 0.5f, 0.5f, 0.5f);
  const graphene_simd4f_t __three = graphene_simd4f_init (3.0f, 3.0f, 3.0f, 3.0f);
  graphene_simd4f_t __s = _mm_rsqrt_ps (v);
  return graphene_simd4f_mul (graphene_simd4f_mul (__s, __half),
                              graphene_simd4f_sub (__three,
                                                   graphene_simd4f_mul (__s, graphene_simd4f_mul (v, __s))));
}

#define graphene_simd4f_cross3(a,b) \
  _simd4f_cross3(a,b)

static inline graphene_simd4f_t
_simd4f_cross3 (const graphene_simd4f_t a,
                const graphene_simd4f_t b)
{
  const graphene_simd4f_t __a_yzx = _mm_shuffle_ps (a, a, _MM_SHUFFLE (3, 0, 2, 1));
  const graphene_simd4f_t __a_zxy = _mm_shuffle_ps (a, a, _MM_SHUFFLE (3, 1, 0, 2));
  const graphene_simd4f_t __b_yzx = _mm_shuffle_ps (b, b, _MM_SHUFFLE (3, 0, 2, 1));
  const graphene_simd4f_t __b_zxy = _mm_shuffle_ps (b, b, _MM_SHUFFLE (3, 1, 0, 2));

  return _mm_sub_ps (_mm_mul_ps (__a_yzx, __b_zxy), _mm_mul_ps (__a_zxy, __b_yzx));
}

#define graphene_simd4f_dot3(a,b) \
  _simd4f_dot3(a,b)

static inline graphene_simd4f_t
_simd4f_dot3 (const graphene_simd4f_t a,
              const graphene_simd4f_t b)
{
#if defined(GRAPHENE_USE_SSE4_1)
  return _mm_dp_ps (a, b, 0x7f);
#else
  GRAPHENE_ALIGN16 const unsigned int __mask_bits[] = { 0xffffffff, 0xffffffff, 0xffffffff, 0 };
  const graphene_simd4f_t __mask = _mm_load_ps ((const float *) __mask_bits);
  const graphene_simd4f_t __m = _mm_mul_ps ((a), (b));
  const graphene_simd4f_t __s0 = _mm_and_ps (__m, __mask);
  const graphene_simd4f_t __s1 = _mm_add_ps (__s0, _mm_movehl_ps (__s0, __s0));
  const graphene_simd4f_t __s2 = _mm_add_ss (__s1, _mm_shuffle_ps (__s1, __s1, 1));

  return _mm_shuffle_ps (__s2, __s2, 0);
#endif
}

#define graphene_simd4f_dot3_scalar(a,b) \
  _simd4f_dot3_scalar(a,b)

static inline float
_simd4f_dot3_scalar (const graphene_simd4f_t a,
                     const graphene_simd4f_t b)
{
  float __res;
  _mm_store_ss (&__res, graphene_simd4f_dot3 (a, b));
  return __res;
}

#define graphene_simd4f_min(a,b) \
  _mm_min_ps (a, b)

#define graphene_simd4f_max(a,b) \
  _mm_max_ps (a, b)


#define graphene_simd4f_shuffle_wxyz(v) \
  _mm_shuffle_ps (v, v, _MM_SHUFFLE (2, 1, 0, 3))

#define graphene_simd4f_shuffle_zwxy(v) \
  _mm_shuffle_ps (v, v, _MM_SHUFFLE (1, 0, 3, 2))

#define graphene_simd4f_shuffle_yzwx(v) \
  _mm_shuffle_ps (v, v, _MM_SHUFFLE (0, 3, 2, 1))

#define graphene_simd4f_zero_w(v) \
  _mm_movelh_ps (v, _mm_unpackhi_ps (v, _mm_setzero_ps ()))

#define graphene_simd4f_zero_zw(v) \
  _mm_movelh_ps (v, _mm_setzero_ps ())

#define graphene_simd4f_merge_w(s,v) \
  _mm_movelh_ps (s, _mm_unpackhi_ps (s, _mm_set1_ps (v)))

#define graphene_simd4f_merge_high(a,b) \
  _mm_movehl_ps (b, a)

#define graphene_simd4f_merge_low(a,b) \
  _mm_movelh_ps (a, b)

typedef GRAPHENE_ALIGN16 union {
  unsigned int ui[4];
  float f[4];
} graphene_simd4f_uif_t;

#define graphene_simd4f_flip_sign_0101(v) _simd4f_flip_sign_0101(v)

static inline graphene_simd4f_t
_simd4f_flip_sign_0101 (const graphene_simd4f_t v)
{
  const graphene_simd4f_uif_t __pnpn = { {
    0x00000000,
    0x80000000,
    0x00000000,
    0x80000000
  } };

  return _mm_xor_ps (v, _mm_load_ps (__pnpn.f));
}

#define graphene_simd4f_flip_sign_1010(v) _simd4f_flip_sign_1010(v)

static inline graphene_simd4f_t
_simd4f_flip_sign_1010(const graphene_simd4f_t v)
{
  const graphene_simd4f_uif_t __npnp = { {
    0x80000000,
    0x00000000,
    0x80000000,
    0x00000000,
  } };

  return _mm_xor_ps (v, _mm_load_ps (__npnp.f));
}

#define graphene_simd4f_cmp_eq(a,b) _simd4f_cmp_eq(a,b)

static inline bool
_simd4f_cmp_eq (const graphene_simd4f_t a,
                        const graphene_simd4f_t b)
{
  __m128i __res = _mm_castps_si128 (_mm_cmpneq_ps (a, b));
  return (_mm_movemask_epi8 (__res) == 0);
}

#define graphene_simd4f_cmp_neq(a,b) _simd4f_cmp_neq(a,b)

static inline bool
_simd4f_cmp_neq (const graphene_simd4f_t a,
                         const graphene_simd4f_t b)
{
  __m128i __res = _mm_castps_si128 (_mm_cmpneq_ps (a, b));
  return (_mm_movemask_epi8 (__res) != 0);
}

#define graphene_simd4f_cmp_lt(a,b) _simd4f_cmp_lt(a,b)

static inline bool
_simd4f_cmp_lt (const graphene_simd4f_t a,
                const graphene_simd4f_t b)
{
  __m128i __res = _mm_castps_si128 (_mm_cmplt_ps (a, b));
  return (_mm_movemask_epi8 (__res) == 0xffff);
}

#define graphene_simd4f_cmp_le(a,b) _simd4f_cmp_le(a,b)

static inline bool
_simd4f_cmp_le (const graphene_simd4f_t a,
                const graphene_simd4f_t b)
{
  __m128i __res = _mm_castps_si128 (_mm_cmple_ps (a, b));
  return (_mm_movemask_epi8 (__res) == 0xffff);
}

#define graphene_simd4f_cmp_ge(a,b) _simd4f_cmp_ge(a,b)

static inline bool
_simd4f_cmp_ge (const graphene_simd4f_t a,
                const graphene_simd4f_t b)
{
  __m128i __res = _mm_castps_si128 (_mm_cmpge_ps (a, b));
  return (_mm_movemask_epi8 (__res) == 0xffff);
}

#define graphene_simd4f_cmp_gt(a,b) _simd4f_cmp_gt(a,b)

static inline bool
_simd4f_cmp_gt (const graphene_simd4f_t a,
                const graphene_simd4f_t b)
{
  __m128i __res = _mm_castps_si128 (_mm_cmpgt_ps (a, b));
  return (_mm_movemask_epi8 (__res) == 0xffff);
}

#define graphene_simd4f_neg(s) _simd4f_neg(s)

static inline graphene_simd4f_t
_simd4f_neg (const graphene_simd4f_t s)
{
  const graphene_simd4f_uif_t __mask = { {
    0x80000000,
    0x80000000,
    0x80000000,
    0x80000000,
  } };

  return _mm_xor_ps (s, _mm_load_ps (__mask.f));
}

#else /* SSE intrinsics-not GCC or Visual Studio */

#  error "Need GCC-compatible or Visual Studio compiler for SSE extensions."

/* Use static inline to inline all these functions */

# endif /* !__GNUC__ && !_MSC_VER */

#elif !defined(__GI_SCANNER__) && defined(GRAPHENE_USE_GCC)

/* GCC vector intrinsic implementation of SIMD 4f */

typedef int graphene_simd4i_t __attribute__((vector_size (16)));

# define graphene_simd4f_init(x,y,z,w) \
  (__extension__ ({ \
    (graphene_simd4f_t) { (x), (y), (z), (w) }; \
  }))

# define graphene_simd4f_init_zero() \
  (__extension__ ({ \
    (graphene_simd4f_t) { 0.f, 0.f, 0.f, 0.f }; \
  }))

# define graphene_simd4f_init_4f(v) \
  (__extension__ ({ \
    (graphene_simd4f_t) { (v)[0], (v)[1], (v)[2], (v)[3] }; \
  }))

# define graphene_simd4f_init_3f(v) \
  (__extension__ ({ \
    (graphene_simd4f_t) { (v)[0], (v)[1], (v)[2], 0.f }; \
  }))

# define graphene_simd4f_init_2f(v) \
  (__extension__ ({ \
    (graphene_simd4f_t) { (v)[0], (v)[1], 0.f, 0.f }; \
  }))

# define graphene_simd4f_dup_4f(s,v) \
  (__extension__ ({ \
    memcpy ((v), &(s), sizeof (float) * 4); \
  }))

# define graphene_simd4f_dup_3f(s,v) \
  (__extension__ ({ \
    memcpy ((v), &(s), sizeof (float) * 3); \
  }))

# define graphene_simd4f_dup_2f(s,v) \
  (__extension__ ({ \
    memcpy ((v), &(s), sizeof (float) * 2); \
  }))

# define graphene_simd4f_get(s,i)       (__extension__ ({ (float) (s)[(i)]; }))
# define graphene_simd4f_get_x(s)       graphene_simd4f_get ((s), 0)
# define graphene_simd4f_get_y(s)       graphene_simd4f_get ((s), 1)
# define graphene_simd4f_get_z(s)       graphene_simd4f_get ((s), 2)
# define graphene_simd4f_get_w(s)       graphene_simd4f_get ((s), 3)

# define graphene_simd4f_splat(v) \
  (__extension__ ({ \
    (graphene_simd4f_t) { (v), (v), (v), (v) }; \
  }))

# define graphene_simd4f_splat_x(v) \
  (__extension__ ({ \
    float __val = graphene_simd4f_get_x ((v)); \
    (graphene_simd4f_t) { __val, __val, __val, __val }; \
  }))

# define graphene_simd4f_splat_y(v) \
  (__extension__ ({ \
    float __val = graphene_simd4f_get_y ((v)); \
    (graphene_simd4f_t) { __val, __val, __val, __val }; \
  }))

# define graphene_simd4f_splat_z(v) \
  (__extension__ ({ \
    float __val = graphene_simd4f_get_z ((v)); \
    (graphene_simd4f_t) { __val, __val, __val, __val }; \
  }))

# define graphene_simd4f_splat_w(v) \
  (__extension__ ({ \
    float __val = graphene_simd4f_get_w ((v)); \
    (graphene_simd4f_t) { __val, __val, __val, __val }; \
  }))

# define graphene_simd4f_reciprocal(v) \
  (__extension__ ({ \
    (graphene_simd4f_t) { \
      (v)[0] != 0.f ? 1.f / (v)[0] : 0.f, \
      (v)[1] != 0.f ? 1.f / (v)[1] : 0.f, \
      (v)[2] != 0.f ? 1.f / (v)[2] : 0.f, \
      (v)[3] != 0.f ? 1.f / (v)[3] : 0.f, \
    }; \
  }))

# define graphene_simd4f_sqrt(v) \
  (__extension__ ({ \
    (graphene_simd4f_t) { \
      sqrtf ((v)[0]), \
      sqrtf ((v)[1]), \
      sqrtf ((v)[2]), \
      sqrtf ((v)[3]), \
    }; \
  }))

# define graphene_simd4f_rsqrt(v) \
  (__extension__ ({ \
    (graphene_simd4f_t) { \
      (v)[0] != 0.f ? 1.f / sqrtf ((v)[0]) : 0.f, \
      (v)[1] != 0.f ? 1.f / sqrtf ((v)[1]) : 0.f, \
      (v)[2] != 0.f ? 1.f / sqrtf ((v)[2]) : 0.f, \
      (v)[3] != 0.f ? 1.f / sqrtf ((v)[3]) : 0.f, \
    }; \
  }))

# define graphene_simd4f_add(a,b)       (__extension__ ({ (graphene_simd4f_t) ((a) + (b)); }))
# define graphene_simd4f_sub(a,b)       (__extension__ ({ (graphene_simd4f_t) ((a) - (b)); }))
# define graphene_simd4f_mul(a,b)       (__extension__ ({ (graphene_simd4f_t) ((a) * (b)); }))
# define graphene_simd4f_div(a,b)       (__extension__ ({ (graphene_simd4f_t) ((a) / (b)); }))

# define graphene_simd4f_cross3(a,b) \
  (__extension__ ({ \
    const graphene_simd4f_t __a = (a); \
    const graphene_simd4f_t __b = (b); \
    graphene_simd4f_init (__a[1] * __b[2] - __a[2] * __b[1], \
                          __a[2] * __b[0] - __a[0] * __b[2], \
                          __a[0] * __b[1] - __a[1] * __b[0], \
                          0.f); \
  }))

# define graphene_simd4f_dot3(a,b) \
  (__extension__ ({ \
    const graphene_simd4f_t __a = (a); \
    const graphene_simd4f_t __b = (b); \
    const float __res = __a[0] * __b[0] + __a[1] * __b[1] + __a[2] * __b[2]; \
    graphene_simd4f_init (__res, __res, __res, __res); \
  }))

# define graphene_simd4f_dot3_scalar(a,b) \
  (__extension__ ({ \
    graphene_simd4f_get_x (graphene_simd4f_dot3 (a, b)); \
  }))

# define graphene_simd4f_min(a,b) \
  (__extension__ ({ \
    const graphene_simd4f_t __a = (a); \
    const graphene_simd4f_t __b = (b); \
    graphene_simd4f_init (__a[0] < __b[0] ? __a[0] : __b[0], \
                          __a[1] < __b[1] ? __a[1] : __b[1], \
                          __a[2] < __b[2] ? __a[2] : __b[2], \
                          __a[3] < __b[3] ? __a[3] : __b[3]); \
  }))

# define graphene_simd4f_max(a,b) \
  (__extension__ ({ \
    const graphene_simd4f_t __a = (a); \
    const graphene_simd4f_t __b = (b); \
    graphene_simd4f_init (__a[0] > __b[0] ? __a[0] : __b[0], \
                          __a[1] > __b[1] ? __a[1] : __b[1], \
                          __a[2] > __b[2] ? __a[2] : __b[2], \
                          __a[3] > __b[3] ? __a[3] : __b[3]); \
  }))

# define graphene_simd4f_shuffle_wxyz(v) \
  (__extension__ ({ \
    const graphene_simd4i_t __mask = { 3, 0, 1, 2 }; \
    (graphene_simd4f_t) __builtin_shuffle ((v), __mask); \
  }))

# define graphene_simd4f_shuffle_zwxy(v) \
  (__extension__ ({ \
    const graphene_simd4i_t __mask = { 2, 3, 0, 1 }; \
    (graphene_simd4f_t) __builtin_shuffle ((v), __mask); \
  }))

# define graphene_simd4f_shuffle_yzwx(v) \
  (__extension__ ({ \
    const graphene_simd4i_t __mask = { 1, 2, 3, 0 }; \
    (graphene_simd4f_t) __builtin_shuffle ((v), __mask); \
  }))

# define graphene_simd4f_zero_w(v) \
  (__extension__ ({ \
    const graphene_simd4i_t __mask = { 0, 1, 2, 4 }; \
    (graphene_simd4f_t) __builtin_shuffle ((v), graphene_simd4f_init_zero (), __mask); \
  }))

# define graphene_simd4f_zero_zw(v) \
  (__extension__ ({ \
    const graphene_simd4i_t __mask = { 0, 1, 4, 4 }; \
    (graphene_simd4f_t) __builtin_shuffle ((v), graphene_simd4f_init_zero (), __mask); \
  }))

# define graphene_simd4f_merge_w(s,v) \
  (__extension__ ({ \
    const graphene_simd4i_t __mask = { 0, 1, 2, 4 }; \
    (graphene_simd4f_t) __builtin_shuffle ((s), graphene_simd4f_splat ((v)), __mask); \
  }))

# define graphene_simd4f_merge_high(a,b) \
  (__extension__ ({ \
    const graphene_simd4i_t __mask = { 2, 3, 6, 7 }; \
    (graphene_simd4f_t) __builtin_shuffle ((a), (b), __mask); \
  }))

# define graphene_simd4f_merge_low(a,b) \
  (__extension__ ({ \
    const graphene_simd4i_t __mask = { 0, 1, 4, 5 }; \
    (graphene_simd4f_t) __builtin_shuffle ((a), (b), __mask); \
  }))

# define graphene_simd4f_flip_sign_0101(v) \
  (__extension__ ({ \
    const graphene_simd4f_t __v = (v); \
    graphene_simd4f_init (__v[0], -__v[1], __v[2], -__v[3]); \
  }))

# define graphene_simd4f_flip_sign_1010(v) \
  (__extension__ ({ \
    const graphene_simd4f_t __v = (v); \
    graphene_simd4f_init (-__v[0], __v[1], -__v[2], __v[3]); \
  }))

# define graphene_simd4f_cmp_eq(a,b) \
  (__extension__ ({ \
    const graphene_simd4i_t __res = (a) == (b); \
    (bool) (__res[0] != 0 && \
            __res[1] != 0 && \
            __res[2] != 0 && \
            __res[3] != 0); \
  }))

# define graphene_simd4f_cmp_neq(a,b) (!graphene_simd4f_cmp_eq (a,b))

# define graphene_simd4f_cmp_lt(a,b) \
  (__extension__ ({ \
    const graphene_simd4i_t __res = (a) < (b); \
    (bool) (__res[0] != 0 && \
            __res[1] != 0 && \
            __res[2] != 0 && \
            __res[3] != 0); \
  }))

# define graphene_simd4f_cmp_le(a,b) \
  (__extension__ ({ \
    const graphene_simd4i_t __res = (a) <= (b); \
    (bool) (__res[0] != 0 && \
            __res[1] != 0 && \
            __res[2] != 0 && \
            __res[3] != 0); \
  }))

# define graphene_simd4f_cmp_ge(a,b) \
  (__extension__ ({ \
    const graphene_simd4i_t __res = (a) >= (b); \
    (bool) (__res[0] != 0 && \
            __res[1] != 0 && \
            __res[2] != 0 && \
            __res[3] != 0); \
  }))

# define graphene_simd4f_cmp_gt(a,b) \
  (__extension__ ({ \
    const graphene_simd4i_t __res = (a) > (b); \
    (bool) (__res[0] != 0 && \
            __res[1] != 0 && \
            __res[2] != 0 && \
            __res[3] != 0); \
  }))

# define graphene_simd4f_neg(s) \
  (__extension__ ({ \
    const graphene_simd4f_t __s = (s); \
    const graphene_simd4f_t __minus_one = graphene_simd4f_splat (-1.f); \
    graphene_simd4f_mul (__s, __minus_one); \
  }))

#elif !defined(__GI_SCANNER__) && defined(GRAPHENE_USE_ARM_NEON)

/* ARM Neon implementation of SIMD4f */

/* Union type used for single lane reading without memcpy */
typedef union {
  graphene_simd4f_t s;
  float f[4];
} graphene_simd4f_union_t;

/* NEON has optimised 2-lanes vectors we can use */
typedef float32x2_t graphene_simd2f_t;

#ifdef __GNUC__
# define graphene_simd4f_init(x,y,z,w) \
  (__extension__ ({ \
    const float32_t __v[4] = { (x), (y), (z), (w) }; \
    (graphene_simd4f_t) vld1q_f32 (__v); \
  }))

# define graphene_simd4f_init_zero() \
  (__extension__ ({ \
    (graphene_simd4f_t) vdupq_n_f32 (0.f); \
  }))

# define graphene_simd4f_init_4f(v) \
  (__extension__ ({ \
    const float32_t *__v32 = (const float32_t *) (v); \
    (graphene_simd4f_t) vld1q_f32 (__v32); \
  }))

# define graphene_simd4f_init_3f(v) \
  (__extension__ ({ \
    graphene_simd4f_init (v[0], v[1], v[2], 0.f); \
  }))

# define graphene_simd4f_init_2f(v) \
  (__extension__ ({ \
    const float32_t *__v32 = (const float32_t *) (v); \
    const graphene_simd2f_t __low = vld1_f32 (__v32); \
    const float32_t __zero = 0; \
    const graphene_simd2f_t __high = vld1_dup_f32 (&__zero); \
    (graphene_simd4f_t) vcombine_f32 (__low, __high); \
  }))

# define graphene_simd4f_dup_4f(s,v) \
  (__extension__ ({ \
    vst1q_f32 ((float32_t *) (v), (s)); \
  }))

# define graphene_simd4f_dup_3f(s,v) \
  (__extension__ ({ \
    float *__v = (v); \
    vst1q_lane_f32 (__v++, (s), 0); \
    vst1q_lane_f32 (__v++, (s), 1); \
    vst1q_lane_f32 (__v, (s), 2); \
  }))

# define graphene_simd4f_dup_2f(s,v) \
  (__extension__ ({ \
    const graphene_simd2f_t __low = vget_low_f32 ((s)); \
    vst1_f32 ((float32_t *) (v), __low); \
  }))

# define graphene_simd4f_get(s,i) \
  (__extension__ ({ \
    (float) vgetq_lane_f32 ((s), (i)); \
  }))

# define graphene_simd4f_splat(v) \
  (__extension__ ({ \
    (graphene_simd4f_t) vdupq_n_f32 ((v)); \
  }))

# define graphene_simd4f_splat_x(s) \
  (__extension__ ({ \
    graphene_simd4f_splat (graphene_simd4f_get_x ((s))); \
  }))

# define graphene_simd4f_splat_y(s) \
  (__extension__ ({ \
    graphene_simd4f_splat (graphene_simd4f_get_y ((s))); \
  }))

# define graphene_simd4f_splat_z(s) \
  (__extension__ ({ \
    graphene_simd4f_splat (graphene_simd4f_get_z ((s))); \
  }))

# define graphene_simd4f_splat_w(s) \
  (__extension__ ({ \
    graphene_simd4f_splat (graphene_simd4f_get_w ((s))); \
  }))

# define graphene_simd4f_reciprocal(s) \
  (__extension__ ({ \
    graphene_simd4f_t __est = vrecpeq_f32 ((s)); \
    __est = vmulq_f32 (vrecpsq_f32 (__est, (s)), __est); \
    (graphene_simd4f_t) vmulq_f32 (vrecpsq_f32 (__est, (s)), __est); \
  }))

# define graphene_simd4f_add(a,b) \
  (__extension__ ({ \
    (graphene_simd4f_t) vaddq_f32 ((a), (b)); \
  }))

# define graphene_simd4f_sub(a,b) \
  (__extension__ ({ \
    (graphene_simd4f_t) vsubq_f32 ((a), (b)); \
  }))

# define graphene_simd4f_mul(a,b) \
  (__extension__ ({ \
    (graphene_simd4f_t) vmulq_f32 ((a), (b)); \
  }))

# define graphene_simd4f_div(a,b) \
  (__extension__ ({ \
    graphene_simd4f_t __rec = graphene_simd4f_reciprocal ((b)); \
    (graphene_simd4f_t) vmulq_f32 ((a), __rec); \
  }))

# define _simd4f_rsqrt_iter(v,estimate) \
  (__extension__ ({ \
    const graphene_simd4f_t __est1 = vmulq_f32 ((estimate), (v)); \
    (graphene_simd4f_t) vmulq_f32 ((estimate), vrsqrtsq_f32 (__est1, (estimate))); \
  }))

# define graphene_simd4f_rsqrt(s) \
  (__extension__ ({ \
    graphene_simd4f_t __estimate = vrsqrteq_f32 ((s)); \
    __estimate = _simd4f_rsqrt_iter ((s), __estimate); \
    __estimate = _simd4f_rsqrt_iter ((s), __estimate); \
    _simd4f_rsqrt_iter ((s), __estimate); \
  }))

# define graphene_simd4f_sqrt(s) \
  (__extension__ ({ \
    graphene_simd4f_t __rsq = graphene_simd4f_rsqrt ((s)); \
    graphene_simd4f_t __rrsq = graphene_simd4f_reciprocal (__rsq); \
    uint32x4_t __tmp = vreinterpretq_u32_f32 ((s)); \
    (graphene_simd4f_t) vreinterpretq_f32_u32 (vandq_u32 (vtstq_u32 (__tmp, __tmp), vreinterpretq_u32_f32 (__rrsq))); \
  }))

# define graphene_simd4f_cross3(a,b) \
  (__extension__ ({ \
    const uint32_t __mask_bits[] = { 0xffffffff, 0xffffffff, 0xffffffff, 0 }; \
    const int32x4_t __mask = vld1q_s32 ((const int32_t *) __mask_bits); \
    const graphene_simd4f_t __a = (a), __b = (b); \
    const graphene_simd2f_t __a_low = vget_low_f32 (__a); \
    const graphene_simd2f_t __b_low = vget_low_f32 (__b); \
    const graphene_simd4f_t __a_yzx = vcombine_f32 (vext_f32 (__a_low, vget_high_f32 (__a), 1), __a_low); \
    const graphene_simd4f_t __b_yzx = vcombine_f32 (vext_f32 (__b_low, vget_high_f32 (__b), 1), __b_low); \
    graphene_simd4f_t __s3 = graphene_simd4f_sub (graphene_simd4f_mul (__b_yzx, __a), \
                                                  graphene_simd4f_mul (__a_yzx, __b)); \
    graphene_simd2f_t __s3_low = vget_low_f32 (__s3); \
    __s3 = vcombine_f32 (vext_f32 (__s3_low, vget_high_f32 (__s3), 1), __s3_low); \
    (graphene_simd4f_t) vandq_s32 ((int32x4_t) __s3, __mask); \
  }))

# define graphene_simd4f_dot3(a,b) \
  (__extension__ ({ \
    graphene_simd4f_splat (graphene_simd4f_dot3_scalar (a, b)); \
  }))

# define graphene_simd4f_dot3_scalar(a,b) \
  (__extension__ ({ \
    const graphene_simd4f_t __m = graphene_simd4f_mul (a, b); \
    const graphene_simd2f_t __s1 = vpadd_f32 (vget_low_f32 (__m), vget_low_f32 (__m)); \
    (float) vget_lane_f32 (vadd_f32 (__s1, vget_high_f32 (__m)), 0); \
  }))

# define graphene_simd4f_min(a,b) \
  (__extension__ ({ \
    (graphene_simd4f_t) vminq_f32 ((a), (b)); \
  }))

# define graphene_simd4f_max(a,b) \
  (__extension__ ({ \
    (graphene_simd4f_t) vmaxq_f32 (a, b); \
  }))

# define graphene_simd4f_shuffle_wxyz(v) \
  (__extension__ ({ \
    graphene_simd4f_union_t __u = { (v) }; \
    graphene_simd4f_init (__u.f[3], __u.f[0], __u.f[1], __u.f[2]); \
  }))

# define graphene_simd4f_shuffle_zwxy(v) \
  (__extension__ ({ \
    graphene_simd4f_union_t __u = { (v) }; \
    graphene_simd4f_init (__u.f[2], __u.f[3], __u.f[0], __u.f[1]); \
  }))

# define graphene_simd4f_shuffle_yzwx(v) \
  (__extension__ ({ \
    graphene_simd4f_union_t __u = { (v) }; \
    graphene_simd4f_init (__u.f[1], __u.f[2], __u.f[3], __u.f[0]); \
  }))

# define graphene_simd4f_zero_w(v) \
  (__extension__ ({ \
    graphene_simd4f_union_t __u = { (v) }; \
    graphene_simd4f_init (__u.f[0], __u.f[1], __u.f[2], 0.f); \
  }))

# define graphene_simd4f_zero_zw(v) \
  (__extension__ ({ \
    graphene_simd4f_union_t __u = { (v) }; \
    graphene_simd4f_init (__u.f[0], __u.f[1], 0.f, 0.f); \
  }))

# define graphene_simd4f_merge_w(s,v) \
  (__extension__ ({ \
    graphene_simd4f_union_t __u = { (s) }; \
    graphene_simd4f_init (__u.f[0], __u.f[1], __u.f[2], (v)); \
  }))

# define graphene_simd4f_merge_high(a,b) \
  (__extension__ ({ \
    graphene_simd4f_union_t __u_a = { (a) }; \
    graphene_simd4f_union_t __u_b = { (b) }; \
    graphene_simd4f_init (__u_a.f[2], __u_a.f[3], __u_b.f[2], __u_b.f[3]); \
  }))

# define graphene_simd4f_merge_low(a,b) \
  (__extension__ ({ \
    graphene_simd4f_union_t __u_a = { (a) }; \
    graphene_simd4f_union_t __u_b = { (b) }; \
    graphene_simd4f_init (__u_a.f[0], __u_a.f[1], __u_b.f[0], __u_b.f[1]); \
  }))

# define graphene_simd4f_flip_sign_0101(s) \
  (__extension__ ({ \
    const unsigned int __upnpn[4] = { \
      0x00000000, \
      0x80000000, \
      0x00000000, \
      0x80000000 \
    }; \
    const uint32x4_t __pnpn = vld1q_u32 (__upnpn); \
    (graphene_simd4f_t) vreinterpretq_f32_u32 (veorq_u32 (vreinterpretq_u32_f32 ((s)), __pnpn)); \
  }))

# define graphene_simd4f_flip_sign_1010(s) \
  (__extension__ ({ \
    const unsigned int __unpnp[4] = { \
      0x80000000, \
      0x00000000, \
      0x80000000, \
      0x00000000 \
    }; \
    const uint32x4_t __npnp = vld1q_u32 (__unpnp); \
    (graphene_simd4f_t) vreinterpretq_f32_u32 (veorq_u32 (vreinterpretq_u32_f32 ((s)), __npnp)); \
  }))

# define graphene_simd4f_cmp_eq(a,b) \
  (__extension__ ({ \
    const uint32x4_t __mask = vceqq_f32 ((a), (b)); \
    (bool) (vgetq_lane_u32 (__mask, 0) != 0 && \
            vgetq_lane_u32 (__mask, 1) != 0 && \
            vgetq_lane_u32 (__mask, 2) != 0 && \
            vgetq_lane_u32 (__mask, 3) != 0); \
  }))

# define graphene_simd4f_cmp_neq(a,b) \
  (__extension__ ({ \
    const uint32x4_t __mask = vceqq_f32 ((a), (b)); \
    (bool) (vgetq_lane_u32 (__mask, 0) == 0 || \
            vgetq_lane_u32 (__mask, 1) == 0 || \
            vgetq_lane_u32 (__mask, 2) == 0 || \
            vgetq_lane_u32 (__mask, 3) == 0); \
  }))

# define graphene_simd4f_cmp_lt(a,b) \
  (__extension__ ({ \
    const uint32x4_t __mask = vcltq_f32 ((a), (b)); \
    (bool) (vgetq_lane_u32 (__mask, 0) != 0 && \
            vgetq_lane_u32 (__mask, 1) != 0 && \
            vgetq_lane_u32 (__mask, 2) != 0 && \
            vgetq_lane_u32 (__mask, 3) != 0); \
  }))

# define graphene_simd4f_cmp_le(a,b) \
  (__extension__ ({ \
    const uint32x4_t __mask = vcleq_f32 ((a), (b)); \
    (bool) (vgetq_lane_u32 (__mask, 0) != 0 && \
            vgetq_lane_u32 (__mask, 1) != 0 && \
            vgetq_lane_u32 (__mask, 2) != 0 && \
            vgetq_lane_u32 (__mask, 3) != 0); \
  }))

# define graphene_simd4f_cmp_ge(a,b) \
  (__extension__ ({ \
    const uint32x4_t __mask = vcgeq_f32 ((a), (b)); \
    (bool) (vgetq_lane_u32 (__mask, 0) != 0 && \
            vgetq_lane_u32 (__mask, 1) != 0 && \
            vgetq_lane_u32 (__mask, 2) != 0 && \
            vgetq_lane_u32 (__mask, 3) != 0); \
  }))

# define graphene_simd4f_cmp_gt(a,b) \
  (__extension__ ({ \
    const uint32x4_t __mask = vcgtq_f32 ((a), (b)); \
    (bool) (vgetq_lane_u32 (__mask, 0) != 0 && \
            vgetq_lane_u32 (__mask, 1) != 0 && \
            vgetq_lane_u32 (__mask, 2) != 0 && \
            vgetq_lane_u32 (__mask, 3) != 0); \
  }))

# define graphene_simd4f_neg(s) \
  (__extension__ ({ \
    const unsigned int __umask[4] = { \
      0x80000000, \
      0x80000000, \
      0x80000000, \
      0x80000000 \
    }; \
    const uint32x4_t __mask = vld1q_u32 (__umask); \
    (graphene_simd4f_t) vreinterpretq_f32_u32 (veorq_u32 (vreinterpretq_u32_f32 ((s)), __mask)); \
  }))

#elif defined _MSC_VER /* Visual Studio ARM */

# define graphene_simd4f_init(x,y,z,w) _simd4f_init(x,y,z,w)
static inline graphene_simd4f_t
_simd4f_init (float x, float y, float z, float w)
{
  const float32_t __v[4] = { (x), (y), (z), (w) };
  return vld1q_f32 (__v);
}

# define graphene_simd4f_init_zero() vdupq_n_f32 (0.f)

# define graphene_simd4f_init_4f(v) vld1q_f32 (v)

# define graphene_simd4f_init_3f(v) graphene_simd4f_init (v[0], v[1], v[2], 0.f)

# define graphene_simd4f_init_2f(v) _simd4f_init_2f(v)
static inline graphene_simd4f_t
_simd4f_init_2f (const float *v)
{
  const float32_t *__v32 = (const float32_t *) (v);
  const graphene_simd2f_t __low = vld1_f32 (__v32);
  const float32_t __zero = 0;
  const graphene_simd2f_t __high = vld1_dup_f32 (&__zero);
  return vcombine_f32 (__low, __high);
}

# define graphene_simd4f_dup_4f(s,v) vst1q_f32 ((float32_t *) (v), (s))

# define graphene_simd4f_dup_3f(s,v) _simd4f_dup_3f(s,v)
static inline
void _simd4f_dup_3f (const graphene_simd4f_t s,
                     float *v)
{
  float *__v = (v);
  vst1q_lane_f32 (__v++, (s), 0);
  vst1q_lane_f32 (__v++, (s), 1);
  vst1q_lane_f32 (__v, (s), 2);
}

# define graphene_simd4f_dup_2f(s,v) vst1_f32 (v, vget_low_f32 (s))

# define graphene_simd4f_get(s,i) vgetq_lane_f32 ((s), (i))

# define graphene_simd4f_splat(v) vdupq_n_f32 ((v))

# define graphene_simd4f_splat_x(s) graphene_simd4f_splat (graphene_simd4f_get_x ((s)))

# define graphene_simd4f_splat_y(s) graphene_simd4f_splat (graphene_simd4f_get_y ((s)))

# define graphene_simd4f_splat_z(s) graphene_simd4f_splat (graphene_simd4f_get_z ((s)))

# define graphene_simd4f_splat_w(s) graphene_simd4f_splat (graphene_simd4f_get_w ((s)))

# define graphene_simd4f_reciprocal(s) _simd4f_reciprocal(s)
static inline graphene_simd4f_t
_simd4f_reciprocal (const graphene_simd4f_t s)
{
  graphene_simd4f_t __est = vrecpeq_f32 ((s));
  __est = vmulq_f32 (vrecpsq_f32 (__est, (s)), __est);
  return vmulq_f32 (vrecpsq_f32 (__est, (s)), __est);
}

# define graphene_simd4f_add(a,b) vaddq_f32 ((a), (b))

# define graphene_simd4f_sub(a,b) vsubq_f32 ((a), (b))

# define graphene_simd4f_mul(a,b) vmulq_f32 ((a), (b))

# define graphene_simd4f_div(a,b) vmulq_f32 (a, graphene_simd4f_reciprocal (b))

static inline graphene_simd4f_t
_simd4f_rsqrt_iter (const graphene_simd4f_t v,
                    const graphene_simd4f_t estimate)
{
  const graphene_simd4f_t __est1 = vmulq_f32 ((estimate), (v));
  return vmulq_f32 ((estimate), vrsqrtsq_f32 (__est1, (estimate)));
}

# define graphene_simd4f_rsqrt(s) _simd4f_rsqrt(s)
static inline graphene_simd4f_t
_simd4f_rsqrt (const graphene_simd4f_t s)
{
  graphene_simd4f_t __estimate = vrsqrteq_f32 ((s));
  __estimate = _simd4f_rsqrt_iter ((s), __estimate);
  __estimate = _simd4f_rsqrt_iter ((s), __estimate);
  return _simd4f_rsqrt_iter ((s), __estimate);
}

# define graphene_simd4f_sqrt(s) _simd4f_sqrt(s)
static inline graphene_simd4f_t
_simd4f_sqrt (const graphene_simd4f_t s)
{
  graphene_simd4f_t __rsq = graphene_simd4f_rsqrt ((s));
  graphene_simd4f_t __rrsq = graphene_simd4f_reciprocal (__rsq);
  uint32x4_t __tmp = vreinterpretq_u32_f32 ((s)); \
  return vreinterpretq_f32_u32 (vandq_u32 (vtstq_u32 (__tmp, __tmp), vreinterpretq_u32_f32 (__rrsq)));
}

# define graphene_simd4f_cross3(a,b) _simd4f_cross3(a,b)
static inline graphene_simd4f_t
_simd4f_cross3 (const graphene_simd4f_t a,
                const graphene_simd4f_t b)
{
  const uint32_t __mask_bits[] = { 0xffffffff, 0xffffffff, 0xffffffff, 0 };
  const int32x4_t __mask = vld1q_s32 ((const int32_t *) __mask_bits);
  const graphene_simd4f_t __a = (a), __b = (b);
  const graphene_simd2f_t __a_low = vget_low_f32 (__a);
  const graphene_simd2f_t __b_low = vget_low_f32 (__b);
  const graphene_simd4f_t __a_yzx = vcombine_f32 (vext_f32 (__a_low, vget_high_f32 (__a), 1), __a_low);
  const graphene_simd4f_t __b_yzx = vcombine_f32 (vext_f32 (__b_low, vget_high_f32 (__b), 1), __b_low);
  graphene_simd4f_t __s3 = graphene_simd4f_sub (graphene_simd4f_mul (__b_yzx, __a),
                                                graphene_simd4f_mul (__a_yzx, __b));
  graphene_simd2f_t __s3_low = vget_low_f32 (__s3);
  __s3 = vcombine_f32 (vext_f32 (__s3_low, vget_high_f32 (__s3), 1), __s3_low);
  return vandq_s32 (__s3, __mask);
}

# define graphene_simd4f_dot3(a,b) graphene_simd4f_splat (graphene_simd4f_dot3_scalar (a, b))

# define graphene_simd4f_dot3_scalar(a,b) _simd4f_dot3_scalar(a,b)
static inline float
_simd4f_dot3_scalar (const graphene_simd4f_t a,
                     const graphene_simd4f_t b)
{
  const graphene_simd4f_t __m = graphene_simd4f_mul (a, b);
  const graphene_simd2f_t __s1 = vpadd_f32 (vget_low_f32 (__m), vget_low_f32 (__m));
  return vget_lane_f32 (vadd_f32 (__s1, vget_high_f32 (__m)), 0);
}

# define graphene_simd4f_min(a,b) vminq_f32 ((a), (b))

# define graphene_simd4f_max(a,b) vmaxq_f32 (a, b)

# define graphene_simd4f_shuffle_wxyz(v) _simd4f_shuffle_wxyz(v)
static inline graphene_simd4f_t
_simd4f_shuffle_wxyz (const graphene_simd4f_t v)
{
  graphene_simd4f_union_t __u = { (v) };
  return graphene_simd4f_init (__u.f[3], __u.f[0], __u.f[1], __u.f[2]);
}

# define graphene_simd4f_shuffle_zwxy(v) _simd4f_shuffle_zwxy(v)
static inline graphene_simd4f_t
_simd4f_shuffle_zwxy (const graphene_simd4f_t v)
{
  graphene_simd4f_union_t __u = { (v) };
  return graphene_simd4f_init (__u.f[2], __u.f[3], __u.f[0], __u.f[1]);
}

# define graphene_simd4f_shuffle_yzwx(v) _simd4f_shuffle_yzwx(v)
static inline graphene_simd4f_t
_simd4f_shuffle_yzwx (const graphene_simd4f_t v)
{
  graphene_simd4f_union_t __u = { (v) };
  return graphene_simd4f_init (__u.f[1], __u.f[2], __u.f[3], __u.f[0]);
}

# define graphene_simd4f_zero_w(v) _simd4f_zero_w(v)
static inline graphene_simd4f_t
_simd4f_zero_w (const graphene_simd4f_t v)
{
  graphene_simd4f_union_t __u = { (v) };
  return graphene_simd4f_init (__u.f[0], __u.f[1], __u.f[2], 0.f);
}

# define graphene_simd4f_zero_zw(v) _simd4f_zero_zw(v)
static inline graphene_simd4f_t
_simd4f_zero_zw (const graphene_simd4f_t v)
{
  graphene_simd4f_union_t __u = { (v) };
  return graphene_simd4f_init (__u.f[0], __u.f[1], 0.f, 0.f);
}

# define graphene_simd4f_merge_w(s,v) _simd4f_merge_w(s,v)
static inline graphene_simd4f_t
_simd4f_merge_w (const graphene_simd4f_t s,
                 float                   v)
{
  graphene_simd4f_union_t __u = { (s) };
  return graphene_simd4f_init (__u.f[0], __u.f[1], __u.f[2], (v));
}

# define graphene_simd4f_merge_high(a,b) _simd4f_merge_high(a,b)
static inline graphene_simd4f_t
_simd4f_merge_high (const graphene_simd4f_t a,
                    const graphene_simd4f_t b)
{
  graphene_simd4f_union_t __u_a = { (a) };
  graphene_simd4f_union_t __u_b = { (b) };
  return graphene_simd4f_init (__u_a.f[2], __u_a.f[3], __u_b.f[2], __u_b.f[3]);
}

# define graphene_simd4f_merge_low(a,b) _simd4f_merge_low(a,b)
static inline graphene_simd4f_t
_simd4f_merge_low (const graphene_simd4f_t a,
                   const graphene_simd4f_t b)
{
  graphene_simd4f_union_t __u_a = { (a) };
  graphene_simd4f_union_t __u_b = { (b) };
  return graphene_simd4f_init (__u_a.f[0], __u_a.f[1], __u_b.f[0], __u_b.f[1]);
}


# define graphene_simd4f_flip_sign_0101(s) _simd4f_flip_sign_0101(s)
static inline graphene_simd4f_t
_simd4f_flip_sign_0101 (const graphene_simd4f_t s)
{
  const unsigned int __upnpn[4] = {
    0x00000000,
    0x80000000,
    0x00000000,
    0x80000000
  };
  const uint32x4_t __pnpn = vld1q_u32 (__upnpn);
  return vreinterpretq_f32_u32 (veorq_u32 (vreinterpretq_u32_f32 ((s)), __pnpn));
}

# define graphene_simd4f_flip_sign_1010(s) _simd4f_flip_sign_1010(s)
static inline graphene_simd4f_t
_simd4f_flip_sign_1010 (const graphene_simd4f_t s)
{
  const unsigned int __unpnp[4] = {
    0x80000000,
    0x00000000,
    0x80000000,
    0x00000000
  };

  const uint32x4_t __npnp = vld1q_u32 (__unpnp);
  return vreinterpretq_f32_u32 (veorq_u32 (vreinterpretq_u32_f32 ((s)), __npnp));
}

# define graphene_simd4f_cmp_eq(a,b) _simd4f_cmp_eq(a,b)
static inline bool
_simd4f_cmp_eq (const graphene_simd4f_t a,
                const graphene_simd4f_t b)
{
  const uint32x4_t __mask = vceqq_f32 ((a), (b));
  return  (vgetq_lane_u32 (__mask, 0) != 0 &&
           vgetq_lane_u32 (__mask, 1) != 0 &&
           vgetq_lane_u32 (__mask, 2) != 0 &&
           vgetq_lane_u32 (__mask, 3) != 0);
}

# define graphene_simd4f_cmp_neq(a,b) _simd4f_cmp_neq(a,b)
static inline bool
_simd4f_cmp_neq (const graphene_simd4f_t a,
                 const graphene_simd4f_t b)
{
  const uint32x4_t __mask = vceqq_f32 ((a), (b));
  return (vgetq_lane_u32 (__mask, 0) == 0 ||
          vgetq_lane_u32 (__mask, 1) == 0 ||
          vgetq_lane_u32 (__mask, 2) == 0 ||
          vgetq_lane_u32 (__mask, 3) == 0);
}

# define graphene_simd4f_cmp_lt(a,b) _simd4f_cmp_lt(a,b)
static inline bool
_simd4f_cmp_lt (const graphene_simd4f_t a,
                const graphene_simd4f_t b)
{
  const uint32x4_t __mask = vcltq_f32 ((a), (b));
  return (vgetq_lane_u32 (__mask, 0) != 0 &&
          vgetq_lane_u32 (__mask, 1) != 0 &&
          vgetq_lane_u32 (__mask, 2) != 0 &&
          vgetq_lane_u32 (__mask, 3) != 0);
}

# define graphene_simd4f_cmp_le(a,b) _simd4f_cmp_le(a,b)
static inline bool
_simd4f_cmp_le (const graphene_simd4f_t a,
                const graphene_simd4f_t b)
{
  const uint32x4_t __mask = vcleq_f32 ((a), (b));
  return (vgetq_lane_u32 (__mask, 0) != 0 &&
          vgetq_lane_u32 (__mask, 1) != 0 &&
          vgetq_lane_u32 (__mask, 2) != 0 &&
          vgetq_lane_u32 (__mask, 3) != 0);
}

# define graphene_simd4f_cmp_ge(a,b) _simd4f_cmp_ge(a,b)
static inline bool
_simd4f_cmp_ge (const graphene_simd4f_t a,
                const graphene_simd4f_t b)
{
  const uint32x4_t __mask = vcgeq_f32 ((a), (b));
  return (vgetq_lane_u32 (__mask, 0) != 0 &&
          vgetq_lane_u32 (__mask, 1) != 0 &&
          vgetq_lane_u32 (__mask, 2) != 0 &&
          vgetq_lane_u32 (__mask, 3) != 0);
}

# define graphene_simd4f_cmp_gt(a,b) _simd4f_cmp_gt(a,b)
static inline bool
_simd4f_cmp_gt (const graphene_simd4f_t a,
                const graphene_simd4f_t b)
{
  const uint32x4_t __mask = vcgtq_f32 ((a), (b));
  return (vgetq_lane_u32 (__mask, 0) != 0 &&
          vgetq_lane_u32 (__mask, 1) != 0 &&
          vgetq_lane_u32 (__mask, 2) != 0 &&
          vgetq_lane_u32 (__mask, 3) != 0);
}

# define graphene_simd4f_neg(s) _simd4f_neg(s)
static inline graphene_simd4f_t
_simd4f_neg (const graphene_simd4f_t s)
{
  const unsigned int __umask[4] = {
    0x80000000,
    0x80000000,
    0x80000000,
    0x80000000
  };
  const uint32x4_t __mask = vld1q_u32 (__umask);
  return vreinterpretq_f32_u32 (veorq_u32 (vreinterpretq_u32_f32 ((s)), __mask));
}

#else /* ARM NEON intrinsics-not GCC or Visual Studio */

#  error "Need GCC-compatible or Visual Studio compiler for ARM NEON extensions."

/* Use static inline to inline all these functions */

# endif /* !__GNUC__ && !_MSC_VER */

/* macros that are not compiler-dependent */
# define graphene_simd4f_get_x(s)       graphene_simd4f_get (s, 0)
# define graphene_simd4f_get_y(s)       graphene_simd4f_get (s, 1)
# define graphene_simd4f_get_z(s)       graphene_simd4f_get (s, 2)
# define graphene_simd4f_get_w(s)       graphene_simd4f_get (s, 3)

#elif defined(__GI_SCANNER__) || defined(GRAPHENE_USE_SCALAR)

/* Fallback implementation using scalar types */

#define graphene_simd4f_init(x,y,z,w) \
  (graphene_simd4f_init ((x), (y), (z), (w)))
#define graphene_simd4f_init_zero() \
  (graphene_simd4f_init_zero ())
#define graphene_simd4f_init_4f(v) \
  (graphene_simd4f_init_4f ((const float *) (v)))
#define graphene_simd4f_init_3f(v) \
  (graphene_simd4f_init_3f ((const float *) (v)))
#define graphene_simd4f_init_2f(v) \
  (graphene_simd4f_init_2f ((const float *) (v)))
#define graphene_simd4f_dup_4f(s,v) \
  (graphene_simd4f_dup_4f ((s), (float *) (v)))
#define graphene_simd4f_dup_3f(s,v) \
  (graphene_simd4f_dup_3f ((s), (float *) (v)))
#define graphene_simd4f_dup_2f(s,v) \
  (graphene_simd4f_dup_2f ((s), (float *) (v)))
#define graphene_simd4f_get(s,i) \
  (graphene_simd4f_get ((s), (i)))
#define graphene_simd4f_get_x(s) \
  (graphene_simd4f_get_x ((s)))
#define graphene_simd4f_get_y(s) \
  (graphene_simd4f_get_y ((s)))
#define graphene_simd4f_get_z(s) \
  (graphene_simd4f_get_z ((s)))
#define graphene_simd4f_get_w(s) \
  (graphene_simd4f_get_w ((s)))
#define graphene_simd4f_splat(v) \
  (graphene_simd4f_splat ((v)))
#define graphene_simd4f_splat_x(s) \
  (graphene_simd4f_splat_x ((s)))
#define graphene_simd4f_splat_y(s) \
  (graphene_simd4f_splat_y ((s)))
#define graphene_simd4f_splat_z(s) \
  (graphene_simd4f_splat_z ((s)))
#define graphene_simd4f_splat_w(s) \
  (graphene_simd4f_splat_w ((s)))
#define graphene_simd4f_add(a,b) \
  (graphene_simd4f_add ((a), (b)))
#define graphene_simd4f_sub(a,b) \
  (graphene_simd4f_sub ((a), (b)))
#define graphene_simd4f_mul(a,b) \
  (graphene_simd4f_mul ((a), (b)))
#define graphene_simd4f_div(a,b) \
  (graphene_simd4f_div ((a), (b)))
#define graphene_simd4f_sqrt(s) \
  (graphene_simd4f_sqrt ((s)))
#define graphene_simd4f_rsqrt(s) \
  (graphene_simd4f_rsqrt ((s)))
#define graphene_simd4f_reciprocal(s) \
  (graphene_simd4f_reciprocal ((s)))
#define graphene_simd4f_cross3(a,b) \
  (graphene_simd4f_cross3 ((a), (b)))
#define graphene_simd4f_dot3(a,b) \
  (graphene_simd4f_dot3 ((a), (b)))
#define graphene_simd4f_dot3_scalar(a,b) \
  (graphene_simd4f_dot3_scalar ((a), (b)))
#define graphene_simd4f_min(a,b) \
  (graphene_simd4f_min ((a), (b)))
#define graphene_simd4f_max(a,b) \
  (graphene_simd4f_max ((a), (b)))
#define graphene_simd4f_shuffle_wxyz(s) \
  (graphene_simd4f_shuffle_wxyz ((s)))
#define graphene_simd4f_shuffle_zwxy(s) \
  (graphene_simd4f_shuffle_zwxy ((s)))
#define graphene_simd4f_shuffle_yzwx(s) \
  (graphene_simd4f_shuffle_yzwx ((s)))
#define graphene_simd4f_flip_sign_0101(s) \
  (graphene_simd4f_flip_sign_0101 ((s)))
#define graphene_simd4f_flip_sign_1010(s) \
  (graphene_simd4f_flip_sign_1010 ((s)))
#define graphene_simd4f_zero_w(v) \
  (graphene_simd4f_zero_w ((v)))
#define graphene_simd4f_zero_zw(v) \
  (graphene_simd4f_zero_zw ((v)))
#define graphene_simd4f_merge_w(s,v) \
  (graphene_simd4f_merge_w ((s), (v)))
#define graphene_simd4f_merge_high(a,b) \
  (graphene_simd4f_merge_high ((a), (b)))
#define graphene_simd4f_merge_low(a,b) \
  (graphene_simd4f_merge_low ((a), (b)))
#define graphene_simd4f_cmp_eq(a,b) \
  (graphene_simd4f_cmp_eq ((a), (b)))
#define graphene_simd4f_cmp_neq(a,b) \
  (graphene_simd4f_cmp_neq ((a), (b)))
#define graphene_simd4f_cmp_lt(a,b) \
  (graphene_simd4f_cmp_lt ((a), (b)))
#define graphene_simd4f_cmp_le(a,b) \
  (graphene_simd4f_cmp_le ((a), (b)))
#define graphene_simd4f_cmp_ge(a,b) \
  (graphene_simd4f_cmp_ge ((a), (b)))
#define graphene_simd4f_cmp_gt(a,b) \
  (graphene_simd4f_cmp_gt ((a), (b)))
#define graphene_simd4f_neg(s) \
  (graphene_simd4f_neg ((s)))

#else
# error "Unsupported simd4f implementation."
#endif

/* Generic operations, inlined */

/**
 * graphene_simd4f_madd:
 * @m1: a #graphene_simd4f_t
 * @m2: a #graphene_simd4f_t
 * @a: a #graphene_simd4f_t
 *
 * Adds @a to the product of @m1 and @m2.
 *
 * Returns: the result vector
 *
 * Since: 1.0
 */
static inline graphene_simd4f_t
graphene_simd4f_madd (const graphene_simd4f_t m1,
                      const graphene_simd4f_t m2,
                      const graphene_simd4f_t a)
{
  return graphene_simd4f_add (graphene_simd4f_mul (m1, m2), a);
}

/**
 * graphene_simd4f_sum:
 * @v: a #graphene_simd4f_t
 *
 * Sums all components of the given vector.
 *
 * Returns: a vector with all components set to be the
 *   sum of the passed #graphene_simd4f_t
 *
 * Since: 1.0
 */
static inline graphene_simd4f_t
graphene_simd4f_sum (const graphene_simd4f_t v)
{
  const graphene_simd4f_t x = graphene_simd4f_splat_x (v);
  const graphene_simd4f_t y = graphene_simd4f_splat_y (v);
  const graphene_simd4f_t z = graphene_simd4f_splat_z (v);
  const graphene_simd4f_t w = graphene_simd4f_splat_w (v);

  return graphene_simd4f_add (graphene_simd4f_add (x, y),
                              graphene_simd4f_add (z, w));
}

/**
 * graphene_simd4f_sum_scalar:
 * @v: a #graphene_simd4f_t
 *
 * Sums all the components of the given vector.
 *
 * Returns: a scalar value with the sum of the components
 *   of the given #graphene_simd4f_t
 *
 * Since: 1.0
 */
static inline float
graphene_simd4f_sum_scalar (const graphene_simd4f_t v)
{
  return graphene_simd4f_get_x (graphene_simd4f_sum (v));
}

/**
 * graphene_simd4f_dot4:
 * @a: a #graphene_simd4f_t
 * @b: a #graphene_simd4f_t
 *
 * Computes the dot product of all the components of the two
 * given #graphene_simd4f_t.
 *
 * Returns: a vector whose components are all set to be the
 *   dot product of the components of the two operands
 *
 * Since: 1.0
 */
static inline graphene_simd4f_t
graphene_simd4f_dot4 (const graphene_simd4f_t a,
                      const graphene_simd4f_t b)
{
  return graphene_simd4f_sum (graphene_simd4f_mul (a, b));
}

/**
 * graphene_simd4f_dot2:
 * @a: a #graphene_simd4f_t
 * @b: a #graphene_simd4f_t
 *
 * Computes the dot product of the first two components of the
 * two given #graphene_simd4f_t.
 *
 * Returns: a vector whose components are all set to the
 *   dot product of the components of the two operands
 *
 * Since: 1.0
 */
static inline graphene_simd4f_t
graphene_simd4f_dot2 (const graphene_simd4f_t a,
                      const graphene_simd4f_t b)
{
  const graphene_simd4f_t m = graphene_simd4f_mul (a, b);
  const graphene_simd4f_t x = graphene_simd4f_splat_x (m);
  const graphene_simd4f_t y = graphene_simd4f_splat_y (m);

  return graphene_simd4f_add (x, y);
}

/**
 * graphene_simd4f_length4:
 * @v: a #graphene_simd4f_t
 *
 * Computes the length of the given #graphene_simd4f_t vector,
 * using all four of its components.
 *
 * Returns: the length vector
 *
 * Since: 1.0
 */
static inline graphene_simd4f_t
graphene_simd4f_length4 (const graphene_simd4f_t v)
{
  return graphene_simd4f_sqrt (graphene_simd4f_dot4 (v, v));
}

/**
 * graphene_simd4f_length3:
 * @v: a #graphene_simd4f_t
 *
 * Computes the length of the given #graphene_simd4f_t vector,
 * using the first three of its components.
 *
 * Returns: the length vector
 *
 * Since: 1.0
 */
static inline graphene_simd4f_t
graphene_simd4f_length3 (const graphene_simd4f_t v)
{
  return graphene_simd4f_sqrt (graphene_simd4f_dot3 (v, v));
}

/**
 * graphene_simd4f_length2:
 * @v: a #graphene_simd4f_t
 *
 * Computes the length of the given #graphene_simd4f_t vector,
 * using the first two of its components.
 *
 * Returns: the length vector
 *
 * Since: 1.0
 */
static inline graphene_simd4f_t
graphene_simd4f_length2 (const graphene_simd4f_t v)
{
  return graphene_simd4f_sqrt (graphene_simd4f_dot2 (v, v));
}

/**
 * graphene_simd4f_normalize4:
 * @v: a #graphene_simd4f_t
 *
 * Computes the normalization of the given #graphene_simd4f_t vector,
 * using all of its components.
 *
 * Returns: the normalized vector
 *
 * Since: 1.0
 */
static inline graphene_simd4f_t
graphene_simd4f_normalize4 (const graphene_simd4f_t v)
{
  graphene_simd4f_t invlen = graphene_simd4f_rsqrt (graphene_simd4f_dot4 (v, v));
  return graphene_simd4f_mul (v, invlen);
}

/**
 * graphene_simd4f_normalize3:
 * @v: a #graphene_simd4f_t
 *
 * Computes the normalization of the given #graphene_simd4f_t vector,
 * using the first three of its components.
 *
 * Returns: the normalized vector
 *
 * Since: 1.0
 */
static inline graphene_simd4f_t
graphene_simd4f_normalize3 (const graphene_simd4f_t v)
{
  graphene_simd4f_t invlen = graphene_simd4f_rsqrt (graphene_simd4f_dot3 (v, v));
  return graphene_simd4f_mul (v, invlen);
}

/**
 * graphene_simd4f_normalize2:
 * @v: a #graphene_simd4f_t
 *
 * Computes the normalization of the given #graphene_simd4f_t vector,
 * using the first two of its components.
 *
 * Returns: the normalized vector
 *
 * Since: 1.0
 */
static inline graphene_simd4f_t
graphene_simd4f_normalize2 (const graphene_simd4f_t v)
{
  graphene_simd4f_t invlen = graphene_simd4f_rsqrt (graphene_simd4f_dot2 (v, v));
  return graphene_simd4f_mul (v, invlen);
}

/**
 * graphene_simd4f_is_zero4:
 * @v: a #graphene_simd4f_t
 *
 * Checks whether the given #graphene_simd4f_t has all its components
 * set to 0.
 *
 * Returns: `true` if all the vector components are zero
 *
 * Since: 1.0
 */
static inline bool
graphene_simd4f_is_zero4 (const graphene_simd4f_t v)
{
  graphene_simd4f_t zero = graphene_simd4f_init_zero ();
  return graphene_simd4f_cmp_eq (v, zero);
}

/**
 * graphene_simd4f_is_zero3:
 * @v: a #graphene_simd4f_t
 *
 * Checks whether the given #graphene_simd4f_t has the first three of
 * its components set to 0.
 *
 * Returns: `true` if the vector's components are zero
 *
 * Since: 1.0
 */
static inline bool
graphene_simd4f_is_zero3 (const graphene_simd4f_t v)
{
  return fabsf (graphene_simd4f_get_x (v)) <= FLT_EPSILON &&
         fabsf (graphene_simd4f_get_y (v)) <= FLT_EPSILON &&
         fabsf (graphene_simd4f_get_z (v)) <= FLT_EPSILON;
}

/**
 * graphene_simd4f_is_zero2:
 * @v: a #graphene_simd4f_t
 *
 * Checks whether the given #graphene_simd4f_t has the first two of
 * its components set to 0.
 *
 * Returns: `true` if the vector's components are zero
 *
 * Since: 1.0
 */
static inline bool
graphene_simd4f_is_zero2 (const graphene_simd4f_t v)
{
  return fabsf (graphene_simd4f_get_x (v)) <= FLT_EPSILON &&
         fabsf (graphene_simd4f_get_y (v)) <= FLT_EPSILON;
}

/**
 * graphene_simd4f_interpolate:
 * @a: a #graphene_simd4f_t
 * @b: a #graphene_simd4f_t
 * @f: the interpolation factor
 *
 * Linearly interpolates all components of the two given
 * #graphene_simd4f_t vectors using the given factor @f.
 *
 * Returns: the intrerpolated vector
 *
 * Since: 1.0
 */
static inline graphene_simd4f_t
graphene_simd4f_interpolate (const graphene_simd4f_t a,
                             const graphene_simd4f_t b,
                             float                   f)
{
  const graphene_simd4f_t one_minus_f = graphene_simd4f_sub (graphene_simd4f_splat (1.f),
                                                             graphene_simd4f_splat (f));

  return graphene_simd4f_add (graphene_simd4f_mul (one_minus_f, a),
                              graphene_simd4f_mul (graphene_simd4f_splat (f), b));
}

/**
 * graphene_simd4f_clamp:
 * @v: a #graphene_simd4f_t
 * @min: the lower boundary
 * @max: the upper boundary
 *
 * Ensures that all components of the vector @v are within
 * the components of the @lower and @upper boundaries.
 *
 * Returns: the clamped vector
 *
 * Since: 1.2
 */
static inline graphene_simd4f_t
graphene_simd4f_clamp (const graphene_simd4f_t v,
                       const graphene_simd4f_t min,
                       const graphene_simd4f_t max)
{
  const graphene_simd4f_t tmp = graphene_simd4f_max (min, v);

  return graphene_simd4f_min (tmp, max);
}

/**
 * graphene_simd4f_clamp_scalar:
 * @v: a #graphene_simd4f_t
 * @min: the lower boundary
 * @max: the upper boundary
 *
 * Ensures that all components of the vector @v are within
 * the @lower and @upper boundary scalar values.
 *
 * Returns: the clamped vector
 *
 * Since: 1.2
 */
static inline graphene_simd4f_t
graphene_simd4f_clamp_scalar (const graphene_simd4f_t v,
                              float                   min,
                              float                   max)
{
  return graphene_simd4f_clamp (v,
                                graphene_simd4f_splat (min),
                                graphene_simd4f_splat (max));
}

/**
 * graphene_simd4f_min_val:
 * @v: a #graphene_simd4f_t
 *
 * Computes the minimum value of all the channels in the given vector.
 *
 * Returns: a vector whose components are all set to the
 *   minimum value in the original vector
 *
 * Since: 1.4
 */
static inline graphene_simd4f_t
graphene_simd4f_min_val (const graphene_simd4f_t v)
{
  graphene_simd4f_t s = v;

  s = graphene_simd4f_min (s, graphene_simd4f_shuffle_wxyz (s));
  s = graphene_simd4f_min (s, graphene_simd4f_shuffle_zwxy (s));

  return s;
}

/**
 * graphene_simd4f_max_val:
 * @v: a #graphene_simd4f_t
 *
 * Computes the maximum value of all the channels in the given vector.
 *
 * Returns: a vector whose components are all set to the
 *   maximum value in the original vector
 *
 * Since: 1.4
 */
static inline graphene_simd4f_t
graphene_simd4f_max_val (const graphene_simd4f_t v)
{
  graphene_simd4f_t s = v;

  s = graphene_simd4f_max (s, graphene_simd4f_shuffle_wxyz (s));
  s = graphene_simd4f_max (s, graphene_simd4f_shuffle_zwxy (s));

  return s;
}

GRAPHENE_END_DECLS
