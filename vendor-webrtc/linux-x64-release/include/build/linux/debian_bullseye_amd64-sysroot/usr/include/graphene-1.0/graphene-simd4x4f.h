/* graphene-simd4x4f.h: 4x4 float vector operations
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 */

#pragma once

#include "graphene-simd4f.h"

#include <math.h>
#include <float.h>

GRAPHENE_BEGIN_DECLS

/**
 * graphene_simd4x4f_t:
 *
 * A SIMD-based matrix type that uses four #graphene_simd4f_t vectors.
 *
 * The matrix is treated as row-major, i.e. the x, y, z, and w vectors
 * are rows, and elements of each vector are a column:
 *
 * |[<!-- language="C" -->
 *   graphene_simd4x4f_t = {
 *     x.x, x.y, x.z, x.w,
 *     y.x, y.y, y.z, y.w,
 *     z.x, z.y, z.z, z.w,
 *     w.x, w.y, w.z, w.w
 *   }
 * ]|
 *
 * The contents of the #graphene_simd4x4f_t type are private and
 * cannot be accessed directly; use the provided API instead.
 *
 * Since: 1.0
 */

/**
 * graphene_simd4x4f_init:
 * @x: a #graphene_simd4f_t for the first row
 * @y: a #graphene_simd4f_t for the second row
 * @z: a #graphene_simd4f_t for the third row
 * @w: a #graphene_simd4f_t for the fourth row
 *
 * Creates a new #graphene_simd4x4f_t using the given row vectors
 * to initialize it.
 *
 * Returns: the newly created #graphene_simd4x4f_t
 *
 * Since: 1.0
 */
static inline graphene_simd4x4f_t GRAPHENE_VECTORCALL
graphene_simd4x4f_init (graphene_simd4f_t x,
                        graphene_simd4f_t y,
                        graphene_simd4f_t z,
                        graphene_simd4f_t w)
{
  graphene_simd4x4f_t s;

  s.x = x;
  s.y = y;
  s.z = z;
  s.w = w;

  return s;
}

/**
 * graphene_simd4x4f_init_identity:
 * @m: a #graphene_simd4x4f_t
 *
 * Initializes @m to be the identity matrix.
 *
 * Since: 1.0
 */
static inline void
graphene_simd4x4f_init_identity (graphene_simd4x4f_t *m)
{
  *m = graphene_simd4x4f_init (graphene_simd4f_init (1.0f, 0.0f, 0.0f, 0.0f),
                               graphene_simd4f_init (0.0f, 1.0f, 0.0f, 0.0f),
                               graphene_simd4f_init (0.0f, 0.0f, 1.0f, 0.0f),
                               graphene_simd4f_init (0.0f, 0.0f, 0.0f, 1.0f));
}

/**
 * graphene_simd4x4f_init_from_float:
 * @m: a #graphene_simd4x4f_t
 * @f: (array fixed-size=16): an array of 16 floating point values
 *
 * Initializes a #graphene_simd4x4f_t with the given array
 * of floating point values.
 *
 * Since: 1.0
 */
static inline void
graphene_simd4x4f_init_from_float (graphene_simd4x4f_t *m,
                                   const float         *f)
{
  m->x = graphene_simd4f_init_4f (f +  0);
  m->y = graphene_simd4f_init_4f (f +  4);
  m->z = graphene_simd4f_init_4f (f +  8);
  m->w = graphene_simd4f_init_4f (f + 12);
}

/**
 * graphene_simd4x4f_to_float:
 * @m: a #graphene_sidm4x4f_t
 * @v: (out caller-allocates) (array fixed-size=16): a floating
 *   point values vector capable of holding at least 16 values
 *
 * Copies the content of @m in a float array.
 *
 * Since: 1.0
 */
static inline void
graphene_simd4x4f_to_float (const graphene_simd4x4f_t *m,
                            float                     *v)
{
  graphene_simd4f_dup_4f (m->x, v +  0);
  graphene_simd4f_dup_4f (m->y, v +  4);
  graphene_simd4f_dup_4f (m->z, v +  8);
  graphene_simd4f_dup_4f (m->w, v + 12);
}

GRAPHENE_AVAILABLE_IN_1_0
void    graphene_simd4x4f_transpose_in_place    (graphene_simd4x4f_t *s);

#if defined(GRAPHENE_USE_SSE)

#ifdef __GNUC__
#define graphene_simd4x4f_transpose_in_place(s) \
  (__extension__ ({ \
    _MM_TRANSPOSE4_PS ((s)->x, (s)->y, (s)->z, (s)->w); \
  }))
#elif defined (_MSC_VER)
#define graphene_simd4x4f_transpose_in_place(s) \
  _MM_TRANSPOSE4_PS ((s)->x, (s)->y, (s)->z, (s)->w)
#endif

#elif defined(GRAPHENE_USE_GCC)

#define graphene_simd4x4f_transpose_in_place(s) \
  (__extension__ ({ \
    const graphene_simd4f_t sx = (s)->x; \
    const graphene_simd4f_t sy = (s)->y; \
    const graphene_simd4f_t sz = (s)->z; \
    const graphene_simd4f_t sw = (s)->w; \
    (s)->x = graphene_simd4f_init (sx[0], sy[0], sz[0], sw[0]); \
    (s)->y = graphene_simd4f_init (sx[1], sy[1], sz[1], sw[1]); \
    (s)->z = graphene_simd4f_init (sx[2], sy[2], sz[2], sw[2]); \
    (s)->w = graphene_simd4f_init (sx[3], sy[3], sz[3], sw[3]); \
  }))

#elif defined(GRAPHENE_USE_ARM_NEON)

# ifdef __GNUC__

#define graphene_simd4x4f_transpose_in_place(s) \
  (__extension__ ({ \
    const graphene_simd4f_union_t sx = { (s)->x }; \
    const graphene_simd4f_union_t sy = { (s)->y }; \
    const graphene_simd4f_union_t sz = { (s)->z }; \
    const graphene_simd4f_union_t sw = { (s)->w }; \
    (s)->x = graphene_simd4f_init (sx.f[0], sy.f[0], sz.f[0], sw.f[0]); \
    (s)->y = graphene_simd4f_init (sx.f[1], sy.f[1], sz.f[1], sw.f[1]); \
    (s)->z = graphene_simd4f_init (sx.f[2], sy.f[2], sz.f[2], sw.f[2]); \
    (s)->w = graphene_simd4f_init (sx.f[3], sy.f[3], sz.f[3], sw.f[3]); \
  }))

# elif defined (_MSC_VER)

#define graphene_simd4x4f_transpose_in_place(s) _simd4x4f_transpose_in_place(s)
static inline void
_simd4x4f_transpose_in_place (graphene_simd4x4f_t *s)
{
  const graphene_simd4f_union_t sx = { (s)->x };
  const graphene_simd4f_union_t sy = { (s)->y };
  const graphene_simd4f_union_t sz = { (s)->z };
  const graphene_simd4f_union_t sw = { (s)->w };
  (s)->x = graphene_simd4f_init (sx.f[0], sy.f[0], sz.f[0], sw.f[0]);
  (s)->y = graphene_simd4f_init (sx.f[1], sy.f[1], sz.f[1], sw.f[1]);
  (s)->z = graphene_simd4f_init (sx.f[2], sy.f[2], sz.f[2], sw.f[2]);
  (s)->w = graphene_simd4f_init (sx.f[3], sy.f[3], sz.f[3], sw.f[3]);
}

# endif

#elif defined(GRAPHENE_USE_SCALAR)

#define graphene_simd4x4f_transpose_in_place(s) \
  (graphene_simd4x4f_transpose_in_place ((graphene_simd4x4f_t *) (s)))

#else
# error "No implementation for graphene_simd4x4f_t defined."
#endif

/**
 * graphene_simd4x4f_sum:
 * @a: a #graphene_simd4f_t
 * @res: (out): return location for the sum vector
 *
 * Adds all the row vectors of @a.
 *
 * Since: 1.0
 */
static inline void
graphene_simd4x4f_sum (const graphene_simd4x4f_t *a,
                       graphene_simd4f_t         *res)
{
  graphene_simd4f_t s = graphene_simd4f_add (a->x, a->y);
  s = graphene_simd4f_add (s, a->z);
  s = graphene_simd4f_add (s, a->w);
  *res = s;
}

/**
 * graphene_simd4x4f_vec4_mul:
 * @a: a #graphene_simd4x4f_t
 * @b: a #graphene_simd4f_t
 * @res: (out): return location for a #graphene_simd4f_t
 *
 * Left multiplies the given #graphene_simd4x4f_t with the given
 * #graphene_simd4f_t row vector using a dot product:
 *
 * |[<!-- language="plain" -->
 * res = b × A
 *
 *     = ⎡x⎤ ⎛ x.x  x.y  x.z  x.w ⎞
 *       ⎜y⎟ ⎜ y.x  y.y  y.z  y.w ⎟
 *       ⎜z⎟ ⎜ z.x  z.y  z.z  z.w ⎟
 *       ⎣w⎦ ⎝ w.x  w.y  w.z  w.w ⎠
 *
 *     = [ x.x × x   x.y × x   x.z × x   x.w × x ]
 *            +         +         +         +
 *       [ y.x × y   y.y × y   y.z × y   y.w × y ]
 *            +         +         +         +
 *       [ z.x × z   z.y × z   z.z × z   z.w × z ]
 *            +         +         +         +
 *       [ w.x × w   w.y × w   w.z × w   w.w × w ]
 *
 *     = ⎡ x.x × x + y.x × y + z.x × z + w.x × w ⎤
 *       ⎜ x.y × x + y.y × y + z.y × z + w.y × w ⎟
 *       ⎜ x.z × x + y.z × y + z.z × z + w.z × w ⎟
 *       ⎣ x.w × x + y.w × y + z.w × z + w.w × w ⎦
 * ]|
 *
 * Since: 1.0
 */
static inline void
graphene_simd4x4f_vec4_mul (const graphene_simd4x4f_t *a,
                            const graphene_simd4f_t   *b,
                            graphene_simd4f_t         *res)
{
  const graphene_simd4f_t v = *b;
  const graphene_simd4f_t v_x = graphene_simd4f_splat_x (v);
  const graphene_simd4f_t v_y = graphene_simd4f_splat_y (v);
  const graphene_simd4f_t v_z = graphene_simd4f_splat_z (v);
  const graphene_simd4f_t v_w = graphene_simd4f_splat_w (v);

  *res = graphene_simd4f_add (graphene_simd4f_add (graphene_simd4f_mul (a->x, v_x),
                                                   graphene_simd4f_mul (a->y, v_y)),
                              graphene_simd4f_add (graphene_simd4f_mul (a->z, v_z),
                                                   graphene_simd4f_mul (a->w, v_w)));
}

/**
 * graphene_simd4x4f_vec3_mul:
 * @m: a #graphene_simd4x4f_t
 * @v: a #graphene_simd4f_t
 * @res: (out): return location for a #graphene_simd4f_t
 *
 * Left multiplies the given #graphene_simd4x4f_t with the given
 * #graphene_simd4f_t, using only the first three row vectors
 * of the matrix, and the first three components of the vector;
 * the W components of the matrix and vector are ignored:
 *
 * |[<!-- language="plain" -->
 * res = b × A
 *
 *     = ⎡x⎤ ⎛ x.x  x.y  x.z ⎞
 *       ⎜y⎟ ⎜ y.x  y.y  y.z ⎟
 *       ⎣z⎦ ⎝ z.x  z.y  z.z ⎠
 *
 *     = [ x.x × x   x.y × x   x.z × x ]
 *            +         +         +
 *       [ y.x × y   y.y × y   y.z × y ]
 *            +         +         +
 *       [ z.x × z   z.y × z   z.z × z ]
 *
 *     = ⎡ x.x × x + y.x × y + z.x × z ⎤
 *       ⎜ x.y × x + y.y × y + z.y × z ⎟
 *       ⎜ x.z × x + y.z × y + z.z × z ⎟
 *       ⎣               0             ⎦
 * ]|
 *
 * See also: graphene_simd4x4f_vec4_mul(), graphene_simd4x4f_point3_mul()
 *
 * Since: 1.0
 */
static inline void
graphene_simd4x4f_vec3_mul (const graphene_simd4x4f_t *m,
                            const graphene_simd4f_t   *v,
                            graphene_simd4f_t         *res)
{
  const graphene_simd4f_t v_x = graphene_simd4f_splat_x (*v);
  const graphene_simd4f_t v_y = graphene_simd4f_splat_y (*v);
  const graphene_simd4f_t v_z = graphene_simd4f_splat_z (*v);
  graphene_simd4f_t r;

  r = graphene_simd4f_add (graphene_simd4f_add (graphene_simd4f_mul (m->x, v_x),
                                                graphene_simd4f_mul (m->y, v_y)),
                           graphene_simd4f_mul (m->z, v_z));
  *res = graphene_simd4f_zero_w (r);
}

/**
 * graphene_simd4x4f_point3_mul:
 * @m: a #graphene_simd4x4f_t
 * @p: a #graphene_simd4f_t
 * @res: (out): return location for a #graphene_simd4f_t
 *
 * Multiplies the given #graphene_simd4x4f_t with the given
 * #graphene_simd4f_t.
 *
 * Unlike graphene_simd4x4f_vec3_mul(), this function will
 * use the W components of the matrix:
 *
 * |[<!-- language="plain" -->
 * res = b × A
 *
 *     = ⎡x⎤ ⎛ x.x  x.y  x.z  x.w ⎞
 *       ⎜y⎟ ⎜ y.x  y.y  y.z  y.w ⎟
 *       ⎜z⎟ ⎜ z.x  z.y  z.z  z.w ⎟
 *       ⎣w⎦ ⎝ w.x  w.y  w.z  w.w ⎠
 *
 *     = [ x.x × x   x.y × x   x.z × x   x.w × x ]
 *            +         +         +         +
 *       [ y.x × y   y.y × y   y.z × y   y.w × y ]
 *            +         +         +         +
 *       [ z.x × z   z.y × z   z.z × z   z.w × z ]
 *            +         +         +         +
 *       [   w.x       w.y       w.z       w.w   ]
 *
 *     = ⎡ x.x × x + y.x × y + z.x × z + w.x ⎤
 *       ⎜ x.y × x + y.y × y + z.y × z + w.y ⎟
 *       ⎜ x.z × x + y.z × y + z.z × z + w.z ⎟
 *       ⎣ x.w × x + y.w × y + z.w × z + w.w ⎦
 * ]|
 *
 * Since: 1.0
 */
static inline void
graphene_simd4x4f_point3_mul (const graphene_simd4x4f_t *m,
                              const graphene_simd4f_t   *p,
                              graphene_simd4f_t         *res)
{
  const graphene_simd4f_t v = *p;
  const graphene_simd4f_t v_x = graphene_simd4f_splat_x (v);
  const graphene_simd4f_t v_y = graphene_simd4f_splat_y (v);
  const graphene_simd4f_t v_z = graphene_simd4f_splat_z (v);

  *res = graphene_simd4f_add (graphene_simd4f_add (graphene_simd4f_mul (m->x, v_x),
                                                   graphene_simd4f_mul (m->y, v_y)),
                              graphene_simd4f_add (graphene_simd4f_mul (m->z, v_z),
                                                   m->w));
}

/**
 * graphene_simd4x4f_transpose:
 * @s: a #graphene_simd4x4f_t
 * @res: (out): return location for the transposed matrix
 *
 * Transposes the given #graphene_simd4x4f_t.
 *
 * Since: 1.0
 */
static inline void
graphene_simd4x4f_transpose (const graphene_simd4x4f_t *s,
                             graphene_simd4x4f_t       *res)
{
  *res = *s;
  graphene_simd4x4f_transpose_in_place (res);
}

/**
 * graphene_simd4x4f_inv_ortho_vec3_mul:
 * @a: a #graphene_simd4x4f_t
 * @b: a #graphene_simd4f_t
 * @res: (out): return location for the transformed vector
 *
 * Performs the inverse orthographic transformation of the first
 * three components in the given vector, using the first three
 * row vectors of the given SIMD matrix.
 *
 * Since: 1.0
 */
static inline void
graphene_simd4x4f_inv_ortho_vec3_mul (const graphene_simd4x4f_t *a,
                                      const graphene_simd4f_t   *b,
                                      graphene_simd4f_t         *res)
{
  graphene_simd4x4f_t transpose = *a;
  graphene_simd4f_t translation = *b;

  transpose.w = graphene_simd4f_init (0.f, 0.f, 0.f, 0.f);
  graphene_simd4x4f_transpose_in_place (&transpose);

  graphene_simd4x4f_vec3_mul (&transpose, &translation, res);
}

/**
 * graphene_simd4x4f_inv_ortho_point3_mul:
 * @a: a #graphene_simd4x4f_t
 * @b: a #graphene_simd4x4f_t
 * @res: (out): return location for the result vector
 *
 * Performs the inverse orthographic transformation of the first
 * three components in the given vector, using the given SIMD
 * matrix.
 *
 * Unlike graphene_simd4x4f_inv_ortho_vec3_mul(), this function
 * will also use the fourth row vector of the SIMD matrix.
 *
 * Since: 1.0
 */
static inline void
graphene_simd4x4f_inv_ortho_point3_mul (const graphene_simd4x4f_t *a,
                                        const graphene_simd4f_t   *b,
                                        graphene_simd4f_t         *res)
{
  graphene_simd4f_t translation = graphene_simd4f_sub (*b, a->w);
  graphene_simd4x4f_t transpose = *a;

  transpose.w = graphene_simd4f_init (0.f, 0.f, 0.f, 0.f);
  graphene_simd4x4f_transpose_in_place (&transpose);

  graphene_simd4x4f_point3_mul (&transpose, &translation, res);
}

/**
 * graphene_simd4x4f_matrix_mul:
 * @a: a #graphene_simd4x4f_t
 * @b: a #graphene_simd4x4f_t
 * @res: (out): return location for the result
 *
 * Multiplies the two matrices, following the convention:
 *
 * |[<!-- language="plain" -->
 *   res = A × B
 *
 *       = ⎡ A.x × B ⎤
 *         ⎜ A.y × B ⎟
 *         ⎜ A.z × B ⎟
 *         ⎣ A.w × B ⎦
 *
 *       = ⎡ res.x ⎤
 *         ⎜ res.y ⎟
 *         ⎜ res.z ⎟
 *         ⎣ res.w ⎦
 * ]|
 *
 * See also: graphene_simd4x4f_vec4_mul()
 *
 * Since: 1.0
 */
static inline void
graphene_simd4x4f_matrix_mul (const graphene_simd4x4f_t *a,
                              const graphene_simd4x4f_t *b,
                              graphene_simd4x4f_t       *res)
{
#if 0
  /* this is the classic naive A*B implementation of the row * column
   * matrix product. using a SIMD scalar implementation, it's fairly
   * slow at 329ns per multiplication; the SSE implementation makes it
   * about 10x faster, at 32ns; the GCC vector implementation is only
   * 5x faster, at 66ns. the biggest culprits are the transpose operation
   * and the multiple, one lane reads to compute the scalar sum.
   */
  graphene_simd4x4f_t t;

  graphene_simd4x4f_transpose (b, &t);

  res->x =
    graphene_simd4f_init (graphene_simd4f_sum_scalar (graphene_simd4f_mul (a->x, t.x)),
                          graphene_simd4f_sum_scalar (graphene_simd4f_mul (a->x, t.y)),
                          graphene_simd4f_sum_scalar (graphene_simd4f_mul (a->x, t.z)),
                          graphene_simd4f_sum_scalar (graphene_simd4f_mul (a->x, t.w)));

  res->y =
    graphene_simd4f_init (graphene_simd4f_sum_scalar (graphene_simd4f_mul (a->y, t.x)),
                          graphene_simd4f_sum_scalar (graphene_simd4f_mul (a->y, t.y)),
                          graphene_simd4f_sum_scalar (graphene_simd4f_mul (a->y, t.z)),
                          graphene_simd4f_sum_scalar (graphene_simd4f_mul (a->y, t.w)));

  res->z =
    graphene_simd4f_init (graphene_simd4f_sum_scalar (graphene_simd4f_mul (a->z, t.x)),
                          graphene_simd4f_sum_scalar (graphene_simd4f_mul (a->z, t.y)),
                          graphene_simd4f_sum_scalar (graphene_simd4f_mul (a->z, t.z)),
                          graphene_simd4f_sum_scalar (graphene_simd4f_mul (a->z, t.w)));

  res->w =
    graphene_simd4f_init (graphene_simd4f_sum_scalar (graphene_simd4f_mul (a->w, t.x)),
                          graphene_simd4f_sum_scalar (graphene_simd4f_mul (a->w, t.y)),
                          graphene_simd4f_sum_scalar (graphene_simd4f_mul (a->w, t.z)),
                          graphene_simd4f_sum_scalar (graphene_simd4f_mul (a->w, t.w)));
#else
  /* this is an optimized version of the matrix multiplication, using
   * four dot products for each row vector. this yields drastically
   * better numbers while retaining the same correct results as above:
   * the scalar implementation now clocks at 91ns; the GCC vector
   * implementation is 19ns; and the SSE implementation is 16ns.
   *
   * the order is correct if we want to multiply A with B; remember
   * that matrix multiplication is non-commutative.
   */
  graphene_simd4f_t x, y, z, w;

  graphene_simd4x4f_vec4_mul (b, &a->x, &x);
  graphene_simd4x4f_vec4_mul (b, &a->y, &y);
  graphene_simd4x4f_vec4_mul (b, &a->z, &z);
  graphene_simd4x4f_vec4_mul (b, &a->w, &w);

  *res = graphene_simd4x4f_init (x, y, z, w);
#endif
}

/**
 * graphene_simd4x4f_init_perspective:
 * @m: a #graphene_simd4x4f_t
 * @fovy_rad: the angle of the field of vision, in radians
 * @aspect: the aspect value
 * @z_near: the depth of the near clipping plane
 * @z_far: the depth of the far clipping plane
 *
 * Initializes a #graphene_simd4x4f_t with a perspective projection.
 *
 * Since: 1.0
 */
static inline void
graphene_simd4x4f_init_perspective (graphene_simd4x4f_t *m,
                                    float                fovy_rad,
                                    float                aspect,
                                    float                z_near,
                                    float                z_far)
{
  float delta_z = z_far - z_near;
  float cotangent = tanf (GRAPHENE_PI_2 - fovy_rad * 0.5f);

  float a = cotangent / aspect;
  float b = cotangent;
  float c = -(z_far + z_near) / delta_z;
  float d = -2 * z_near * z_far / delta_z;

  m->x = graphene_simd4f_init (   a, 0.0f, 0.0f,  0.0f);
  m->y = graphene_simd4f_init (0.0f,    b, 0.0f,  0.0f);
  m->z = graphene_simd4f_init (0.0f, 0.0f,    c, -1.0f);
  m->w = graphene_simd4f_init (0.0f, 0.0f,    d,  0.0f);
}

/**
 * graphene_simd4x4f_init_ortho:
 * @m: a #graphene_simd4x4f_t
 * @left: edge of the left clipping plane
 * @right: edge of the right clipping plane
 * @bottom: edge of the bottom clipping plane
 * @top: edge of the top clipping plane
 * @z_near: depth of the near clipping plane
 * @z_far: depth of the far clipping plane
 *
 * Initializes the given SIMD matrix with an orthographic projection.
 *
 * Since: 1.0
 */
static inline void
graphene_simd4x4f_init_ortho (graphene_simd4x4f_t *m,
                              float                left,
                              float                right,
                              float                bottom,
                              float                top,
                              float                z_near,
                              float                z_far)
{
  float delta_x = right - left;
  float delta_y = top - bottom;
  float delta_z = z_far - z_near;

  float a = 2.0f / delta_x;
  float b = -(right + left) / delta_x;
  float c = 2.0f / delta_y;
  float d = -(top + bottom) / delta_y;
  float e = -2.0f / delta_z;
  float f = -(z_far + z_near) / delta_z;

  m->x = graphene_simd4f_init (   a, 0.0f, 0.0f, 0.0f);
  m->y = graphene_simd4f_init (0.0f,    c, 0.0f, 0.0f);
  m->z = graphene_simd4f_init (0.0f, 0.0f,    e, 0.0f);
  m->w = graphene_simd4f_init (   b,    d,    f, 1.0f);
}

/**
 * graphene_simd4x4f_init_look_at:
 * @m: a #graphene_simd4x4f_t
 * @eye: vector for the camera coordinates
 * @center: vector for the object coordinates
 * @up: vector for the upwards direction
 *
 * Initializes a SIMD matrix with the projection necessary for
 * the camera at the @eye coordinates to look at the object at
 * the @center coordinates. The top of the camera is aligned to
 * the @up vector.
 *
 * Since: 1.0
 */
static inline void
graphene_simd4x4f_init_look_at (graphene_simd4x4f_t *m,
                                graphene_simd4f_t    eye,
                                graphene_simd4f_t    center,
                                graphene_simd4f_t    up)
{
  const graphene_simd4f_t direction = graphene_simd4f_sub (center, eye);
  graphene_simd4f_t cross;
  graphene_simd4f_t z_axis;
  graphene_simd4f_t x_axis;
  graphene_simd4f_t y_axis;
  float eye_v[4];

  if (graphene_simd4f_get_x (graphene_simd4f_dot3 (direction, direction)) < FLT_EPSILON)
    /* eye and center are in the same position */
    z_axis = graphene_simd4f_init (0, 0, 1, 0);
  else
    z_axis = graphene_simd4f_normalize3 (direction);

  cross = graphene_simd4f_cross3 (z_axis, up);
  if (graphene_simd4f_get_x (graphene_simd4f_dot3 (cross, cross)) < FLT_EPSILON)
    {
      graphene_simd4f_t tweak_z;

      /* up and z_axis are parallel */
      if (fabs (graphene_simd4f_get_z (up) - 1.0) < FLT_EPSILON)
        tweak_z = graphene_simd4f_init (0.0001f, 0, 0, 0);
      else
        tweak_z = graphene_simd4f_init (0, 0, 0.0001f, 0);

      z_axis = graphene_simd4f_add (z_axis, tweak_z);
      z_axis = graphene_simd4f_normalize3 (z_axis);
      cross = graphene_simd4f_cross3 (z_axis, up);
    }

  x_axis = graphene_simd4f_normalize3 (cross);
  y_axis = graphene_simd4f_cross3 (x_axis, z_axis);

  graphene_simd4f_dup_4f (eye, eye_v);

  m->x = x_axis;
  m->y = y_axis;
  m->z = graphene_simd4f_neg (z_axis);
  m->w = graphene_simd4f_init (-eye_v[0], -eye_v[1], -eye_v[2], 1.f);
}

/**
 * graphene_simd4x4f_init_frustum:
 * @m: a #graphene_simd4x4f_t
 * @left: distance of the left clipping plane
 * @right: distance of the right clipping plane
 * @bottom: distance of the bottom clipping plane
 * @top: distance of the top clipping plane
 * @z_near: distance of the near clipping plane
 * @z_far: distance of the far clipping plane
 *
 * Initializes a SIMD matrix with a frustum described by the distances
 * of six clipping planes.
 *
 * Since: 1.2
 */
static inline void
graphene_simd4x4f_init_frustum (graphene_simd4x4f_t *m,
                                float                left,
                                float                right,
                                float                bottom,
                                float                top,
                                float                z_near,
                                float                z_far)
{
  float x = 2.f * z_near / (right - left);
  float y = 2.f * z_near / (top - bottom);

  float a = (right + left) / (right - left);
  float b = (top + bottom) / (top - bottom);
  float c = -1.f * (z_far + z_near) / (z_far - z_near);
  float d = -2.f * z_far * z_near / (z_far - z_near);

  m->x = graphene_simd4f_init (  x, 0.f, 0.f,  0.f);
  m->y = graphene_simd4f_init (0.f,   y, 0.f,  0.f);
  m->z = graphene_simd4f_init (  a,   b,   c, -1.f);
  m->w = graphene_simd4f_init (0.f, 0.f,   d,  0.f);
}

/**
 * graphene_simd4x4f_perspective:
 * @m: a #graphene_simd4x4f_t
 * @depth: depth of the perspective
 *
 * Adds a perspective transformation for the given @depth.
 *
 * Since: 1.0
 */
static inline void
graphene_simd4x4f_perspective (graphene_simd4x4f_t *m,
                               float                depth)
{
#if 1
  const float m_xw = graphene_simd4f_get_w (m->x);
  const float m_yw = graphene_simd4f_get_w (m->y);
  const float m_zw = graphene_simd4f_get_w (m->z);
  const float m_ww = graphene_simd4f_get_w (m->w);

  const float p0 = graphene_simd4f_get_z (m->x) + -1.0f / depth * m_xw;
  const float p1 = graphene_simd4f_get_z (m->y) + -1.0f / depth * m_yw;
  const float p2 = graphene_simd4f_get_z (m->z) + -1.0f / depth * m_zw;
  const float p3 = graphene_simd4f_get_z (m->w) + -1.0f / depth * m_ww;

  const graphene_simd4f_t p_x = graphene_simd4f_merge_w (m->x, m_xw + p0);
  const graphene_simd4f_t p_y = graphene_simd4f_merge_w (m->y, m_yw + p1);
  const graphene_simd4f_t p_z = graphene_simd4f_merge_w (m->z, m_zw + p2);
  const graphene_simd4f_t p_w = graphene_simd4f_merge_w (m->w, m_ww + p3);
#else
  /* this is equivalent to the operations above, but trying to inline
   * them into SIMD registers as much as possible by transposing the
   * original matrix and operating on the resulting column vectors. it
   * should warrant a micro benchmark, because while the above code is
   * dominated by single channel reads, the code below has a transpose
   * operation.
   */
  graphene_simd4x4f_t t;
  const graphene_simd4f_t f, p;
  const graphene_simd4f_t p_x, p_y, p_z, p_w;

  graphene_simd4x4f_transpose (m, &t);

  f = graphene_simd4f_neg (graphene_simd4f_reciprocal (graphene_simd4f_splat (depth)));
  p = graphene_simd4f_sum (t.w, graphene_simd4f_sum (t.z, graphene_simd4f_mul (f, t.w)));
  p_x = graphene_simd4f_merge_w (m->x, graphene_simd4f_get_x (p));
  p_y = graphene_simd4f_merge_w (m->y, graphene_simd4f_get_y (p));
  p_z = graphene_simd4f_merge_w (m->z, graphene_simd4f_get_z (p));
  p_w = graphene_simd4f_merge_w (m->w, graphene_simd4f_get_w (p));
#endif

  *m = graphene_simd4x4f_init (p_x, p_y, p_z, p_w);
}

/**
 * graphene_simd4x4f_translation:
 * @m: a #graphene_simd4x4f_t
 * @x: coordinate of the X translation
 * @y: coordinate of the Y translation
 * @z: coordinate of the Z translation
 *
 * Initializes @m to contain a translation to the given coordinates.
 *
 * Since: 1.0
 */
static inline void
graphene_simd4x4f_translation (graphene_simd4x4f_t *m,
                               float                x,
                               float                y,
                               float                z)
{
  *m = graphene_simd4x4f_init (graphene_simd4f_init (1.0f, 0.0f, 0.0f, 0.0f),
                               graphene_simd4f_init (0.0f, 1.0f, 0.0f, 0.0f),
                               graphene_simd4f_init (0.0f, 0.0f, 1.0f, 0.0f),
                               graphene_simd4f_init (   x,    y,    z, 1.0f));
}

/**
 * graphene_simd4x4f_scale:
 * @m: a #graphene_simd4x4f_t
 * @x: scaling factor on the X axis
 * @y: scaling factor on the Y axis
 * @z: scaling factor on the Z axis
 *
 * Initializes @m to contain a scaling transformation with the
 * given factors.
 *
 * Since: 1.0
 */
static inline void
graphene_simd4x4f_scale (graphene_simd4x4f_t *m,
                         float                x,
                         float                y,
                         float                z)
{
  *m = graphene_simd4x4f_init (graphene_simd4f_init (   x, 0.0f, 0.0f, 0.0f),
                               graphene_simd4f_init (0.0f,    y, 0.0f, 0.0f),
                               graphene_simd4f_init (0.0f, 0.0f,    z, 0.0f),
                               graphene_simd4f_init (0.0f, 0.0f, 0.0f, 1.0f));

}

/**
 * graphene_simd4x4f_rotation:
 * @m: a #graphene_simd4x4f_t
 * @rad: the rotation, in radians
 * @axis: the vector of the axis of rotation
 *
 * Initializes @m to contain a rotation of the given angle
 * along the given axis.
 *
 * Since: 1.0
 */
static inline void
graphene_simd4x4f_rotation (graphene_simd4x4f_t *m,
                            float                rad,
                            graphene_simd4f_t    axis)
{
  float sine, cosine;
  float x, y, z;
  float ab, bc, ca;
  float tx, ty, tz;
  graphene_simd4f_t i, j, k;

  rad = -rad;
  axis = graphene_simd4f_normalize3 (axis);

  /* We cannot use graphene_sincos() because it's a private function, whereas
   * graphene-simd4x4f.h is a public header
   */
  sine = sinf (rad);
  cosine = cosf (rad);

  x = graphene_simd4f_get_x (axis);
  y = graphene_simd4f_get_y (axis);
  z = graphene_simd4f_get_z (axis);

  ab = x * y * (1.0f - cosine);
  bc = y * z * (1.0f - cosine);
  ca = z * x * (1.0f - cosine);

  tx = x * x;
  ty = y * y;
  tz = z * z;

  i = graphene_simd4f_init (tx + cosine * (1.0f - tx), ab - z * sine, ca + y * sine, 0.f);
  j = graphene_simd4f_init (ab + z * sine, ty + cosine * (1.0f - ty), bc - x * sine, 0.f);
  k = graphene_simd4f_init (ca - y * sine, bc + x * sine, tz + cosine * (1.0f - tz), 0.f);

  *m = graphene_simd4x4f_init (i, j, k, graphene_simd4f_init (0.0f, 0.0f, 0.0f, 1.0f));
}

/**
 * graphene_simd4x4f_add:
 * @a: a #graphene_simd4x4f_t
 * @b: a #graphene_simd4x4f_t
 * @res: (out caller-allocates): return location for a #graphene_simd4x4f_t
 *
 * Adds each row vector of @a and @b and places the results in @res.
 *
 * Since: 1.0
 */
static inline void
graphene_simd4x4f_add (const graphene_simd4x4f_t *a,
                       const graphene_simd4x4f_t *b,
                       graphene_simd4x4f_t *res)
{
  res->x = graphene_simd4f_add (a->x, b->x);
  res->y = graphene_simd4f_add (a->y, b->y);
  res->z = graphene_simd4f_add (a->z, b->z);
  res->w = graphene_simd4f_add (a->w, b->w);
}

/**
 * graphene_simd4x4f_sub:
 * @a: a #graphene_simd4x4f_t
 * @b: a #graphene_simd4x4f_t
 * @res: (out caller-allocates): return location for a #graphene_simd4x4f_t
 *
 * Subtracts each row vector of @a and @b and places the results in @res.
 *
 * Since: 1.0
 */
static inline void
graphene_simd4x4f_sub (const graphene_simd4x4f_t *a,
                       const graphene_simd4x4f_t *b,
                       graphene_simd4x4f_t *res)
{
  res->x = graphene_simd4f_sub (a->x, b->x);
  res->y = graphene_simd4f_sub (a->y, b->y);
  res->z = graphene_simd4f_sub (a->z, b->z);
  res->w = graphene_simd4f_sub (a->w, b->w);
}

/**
 * graphene_simd4x4f_mul:
 * @a: a #graphene_simd4x4f_t
 * @b: a #graphene_simd4x4f_t
 * @res: (out caller-allocates): return location for a #graphene_simd4x4f_t
 *
 * Multiplies each row vector of @a and @b and places the results in @res.
 *
 * You most likely want graphene_simd4x4f_matrix_mul() instead.
 *
 * Since: 1.0
 */
static inline void
graphene_simd4x4f_mul (const graphene_simd4x4f_t *a,
                       const graphene_simd4x4f_t *b,
                       graphene_simd4x4f_t *res)
{
  res->x = graphene_simd4f_mul (a->x, b->x);
  res->y = graphene_simd4f_mul (a->y, b->y);
  res->z = graphene_simd4f_mul (a->z, b->z);
  res->w = graphene_simd4f_mul (a->w, b->w);
}

/**
 * graphene_simd4x4f_div:
 * @a: a #graphene_simd4x4f_t
 * @b: a #graphene_simd4x4f_t
 * @res: (out caller-allocates): return location for a #graphene_simd4x4f_t
 *
 * Divides each row vector of @a and @b and places the results in @res.
 *
 * Since: 1.0
 */
static inline void
graphene_simd4x4f_div (const graphene_simd4x4f_t *a,
                       const graphene_simd4x4f_t *b,
                       graphene_simd4x4f_t *res)
{
  res->x = graphene_simd4f_div (a->x, b->x);
  res->y = graphene_simd4f_div (a->y, b->y);
  res->z = graphene_simd4f_div (a->z, b->z);
  res->w = graphene_simd4f_div (a->w, b->w);
}

/**
 * graphene_simd4x4f_inverse:
 * @m: a #graphene_simd4x4f_t
 * @res: (out): return location for the inverse matrix
 *
 * Inverts the given #graphene_simd4x4f_t.
 *
 * Returns: `true` if the matrix was invertible
 *
 * Since: 1.0
 */
static inline bool
graphene_simd4x4f_inverse (const graphene_simd4x4f_t *m,
                           graphene_simd4x4f_t       *res)
{
  /* split rows */
  const graphene_simd4f_t r0 = m->x;
  const graphene_simd4f_t r1 = m->y;
  const graphene_simd4f_t r2 = m->z;
  const graphene_simd4f_t r3 = m->w;

  /* cofactors */
  const graphene_simd4f_t r0_wxyz = graphene_simd4f_shuffle_wxyz (r0);
  const graphene_simd4f_t r0_zwxy = graphene_simd4f_shuffle_zwxy (r0);
  const graphene_simd4f_t r0_yzwx = graphene_simd4f_shuffle_yzwx (r0);

  const graphene_simd4f_t r1_wxyz = graphene_simd4f_shuffle_wxyz (r1);
  const graphene_simd4f_t r1_zwxy = graphene_simd4f_shuffle_zwxy (r1);
  const graphene_simd4f_t r1_yzwx = graphene_simd4f_shuffle_yzwx (r1);

  const graphene_simd4f_t r2_wxyz = graphene_simd4f_shuffle_wxyz (r2);
  const graphene_simd4f_t r2_zwxy = graphene_simd4f_shuffle_zwxy (r2);
  const graphene_simd4f_t r2_yzwx = graphene_simd4f_shuffle_yzwx (r2);

  const graphene_simd4f_t r3_wxyz = graphene_simd4f_shuffle_wxyz (r3);
  const graphene_simd4f_t r3_zwxy = graphene_simd4f_shuffle_zwxy (r3);
  const graphene_simd4f_t r3_yzwx = graphene_simd4f_shuffle_yzwx (r3);

  const graphene_simd4f_t r0_wxyz_x_r1 = graphene_simd4f_mul (r0_wxyz, r1);
  const graphene_simd4f_t r0_wxyz_x_r1_yzwx = graphene_simd4f_mul (r0_wxyz, r1_yzwx);
  const graphene_simd4f_t r0_wxyz_x_r1_zwxy = graphene_simd4f_mul (r0_wxyz, r1_zwxy);

  const graphene_simd4f_t r2_wxyz_x_r3 = graphene_simd4f_mul (r2_wxyz, r3);
  const graphene_simd4f_t r2_wxyz_x_r3_yzwx = graphene_simd4f_mul (r2_wxyz, r3_yzwx);
  const graphene_simd4f_t r2_wxyz_x_r3_zwxy = graphene_simd4f_mul (r2_wxyz, r3_zwxy);

  const graphene_simd4f_t ar1 = graphene_simd4f_sub (graphene_simd4f_shuffle_wxyz (r2_wxyz_x_r3_zwxy),
                                                     graphene_simd4f_shuffle_zwxy (r2_wxyz_x_r3));
  const graphene_simd4f_t ar2 = graphene_simd4f_sub (graphene_simd4f_shuffle_zwxy (r2_wxyz_x_r3_yzwx),
                                                     r2_wxyz_x_r3_yzwx);
  const graphene_simd4f_t ar3 = graphene_simd4f_sub (r2_wxyz_x_r3_zwxy,
                                                     graphene_simd4f_shuffle_wxyz (r2_wxyz_x_r3));

  const graphene_simd4f_t br1 = graphene_simd4f_sub (graphene_simd4f_shuffle_wxyz (r0_wxyz_x_r1_zwxy),
                                                     graphene_simd4f_shuffle_zwxy (r0_wxyz_x_r1));
  const graphene_simd4f_t br2 = graphene_simd4f_sub (graphene_simd4f_shuffle_zwxy (r0_wxyz_x_r1_yzwx),
                                                     r0_wxyz_x_r1_yzwx);
  const graphene_simd4f_t br3 = graphene_simd4f_sub (r0_wxyz_x_r1_zwxy,
                                                     graphene_simd4f_shuffle_wxyz (r0_wxyz_x_r1));

  const graphene_simd4f_t r0_sum =
    graphene_simd4f_madd (r0_yzwx, ar3,
                          graphene_simd4f_madd (r0_zwxy, ar2,
                                                graphene_simd4f_mul (r0_wxyz, ar1)));
  const graphene_simd4f_t r1_sum =
    graphene_simd4f_madd (r1_wxyz, ar1,
                          graphene_simd4f_madd (r1_zwxy, ar2,
                                                graphene_simd4f_mul (r1_yzwx, ar3)));
  const graphene_simd4f_t r2_sum =
    graphene_simd4f_madd (r2_yzwx, br3,
                          graphene_simd4f_madd (r2_zwxy, br2,
                                                graphene_simd4f_mul (r2_wxyz, br1)));
  const graphene_simd4f_t r3_sum =
    graphene_simd4f_madd (r3_yzwx, br3,
                          graphene_simd4f_madd (r3_zwxy, br2,
                                                graphene_simd4f_mul (r3_wxyz, br1)));

  /* determinant and its inverse */
  const graphene_simd4f_t d0 = graphene_simd4f_mul (r1_sum, r0);
  const graphene_simd4f_t d1 = graphene_simd4f_add (d0, graphene_simd4f_merge_high (d0, d0));
  const graphene_simd4f_t det = graphene_simd4f_sub (d1, graphene_simd4f_splat_y (d1));
  if (fabsf (graphene_simd4f_get_x (det)) >= FLT_EPSILON)
    {
      const graphene_simd4f_t invdet = graphene_simd4f_splat_x (graphene_simd4f_div (graphene_simd4f_splat (1.0f), det));

      const graphene_simd4f_t o0 = graphene_simd4f_mul (graphene_simd4f_flip_sign_0101 (r1_sum), invdet);
      const graphene_simd4f_t o1 = graphene_simd4f_mul (graphene_simd4f_flip_sign_1010 (r0_sum), invdet);
      const graphene_simd4f_t o2 = graphene_simd4f_mul (graphene_simd4f_flip_sign_0101 (r3_sum), invdet);
      const graphene_simd4f_t o3 = graphene_simd4f_mul (graphene_simd4f_flip_sign_1010 (r2_sum), invdet);

      graphene_simd4x4f_t mt = graphene_simd4x4f_init (o0, o1, o2, o3);

      /* transpose the resulting matrix */
      graphene_simd4x4f_transpose (&mt, res);

      return true;
    }

  return false;
}

/**
 * graphene_simd4x4f_determinant:
 * @m: a #graphene_simd4x4f_t
 * @det_r: (out): return location for the matrix determinant
 * @invdet_r: (out): return location for the inverse of the matrix
 *   determinant
 *
 * Computes the determinant (and its inverse) of the given matrix
 *
 * Since: 1.0
 */
static inline void
graphene_simd4x4f_determinant (const graphene_simd4x4f_t *m,
                               graphene_simd4f_t         *det_r,
                               graphene_simd4f_t         *invdet_r)
{
  /* split rows */
  const graphene_simd4f_t r0 = m->x;
  const graphene_simd4f_t r1 = m->y;
  const graphene_simd4f_t r2 = m->z;
  const graphene_simd4f_t r3 = m->w;

  /* cofactors */
  const graphene_simd4f_t r1_wxyz = graphene_simd4f_shuffle_wxyz (r1);
  const graphene_simd4f_t r1_zwxy = graphene_simd4f_shuffle_zwxy (r1);
  const graphene_simd4f_t r1_yzwx = graphene_simd4f_shuffle_yzwx (r1);

  const graphene_simd4f_t r2_wxyz = graphene_simd4f_shuffle_wxyz (r2);

  const graphene_simd4f_t r3_zwxy = graphene_simd4f_shuffle_zwxy (r3);
  const graphene_simd4f_t r3_yzwx = graphene_simd4f_shuffle_yzwx (r3);

  const graphene_simd4f_t r2_wxyz_x_r3 = graphene_simd4f_mul (r2_wxyz, r3);
  const graphene_simd4f_t r2_wxyz_x_r3_yzwx = graphene_simd4f_mul (r2_wxyz, r3_yzwx);
  const graphene_simd4f_t r2_wxyz_x_r3_zwxy = graphene_simd4f_mul (r2_wxyz, r3_zwxy);

  const graphene_simd4f_t ar1 = graphene_simd4f_sub (graphene_simd4f_shuffle_wxyz (r2_wxyz_x_r3_zwxy),
                                                     graphene_simd4f_shuffle_zwxy (r2_wxyz_x_r3));
  const graphene_simd4f_t ar2 = graphene_simd4f_sub (graphene_simd4f_shuffle_zwxy (r2_wxyz_x_r3_yzwx),
                                                     r2_wxyz_x_r3_yzwx);
  const graphene_simd4f_t ar3 = graphene_simd4f_sub (r2_wxyz_x_r3_zwxy,
                                                     graphene_simd4f_shuffle_wxyz (r2_wxyz_x_r3));

  const graphene_simd4f_t r1_sum =
    graphene_simd4f_madd (r1_wxyz, ar1,
                          graphene_simd4f_madd (r1_zwxy, ar2,
                                                graphene_simd4f_mul (r1_yzwx, ar3)));

  /* determinant and its inverse */
  const graphene_simd4f_t d0 = graphene_simd4f_mul (r1_sum, r0);
  const graphene_simd4f_t d1 = graphene_simd4f_add (d0, graphene_simd4f_merge_high (d0, d0));

  const graphene_simd4f_t det = graphene_simd4f_sub (d1, graphene_simd4f_splat_y (d1));

  const graphene_simd4f_t invdet = graphene_simd4f_splat_x (graphene_simd4f_div (graphene_simd4f_splat (1.0f), det));

  if (det_r != NULL)
    *det_r = det;

  if (invdet_r != NULL)
    *invdet_r = invdet;
}

/**
 * graphene_simd4x4f_is_identity:
 * @m: a #graphene_simd4x4f_t
 *
 * Checks whether the given matrix is the identity matrix.
 *
 * Returns: `true` if the matrix is the identity matrix
 *
 * Since: 1.0
 */
static inline bool
graphene_simd4x4f_is_identity (const graphene_simd4x4f_t *m)
{
  const graphene_simd4f_t r0 = graphene_simd4f_init (1.0f, 0.0f, 0.0f, 0.0f);
  const graphene_simd4f_t r1 = graphene_simd4f_init (0.0f, 1.0f, 0.0f, 0.0f);
  const graphene_simd4f_t r2 = graphene_simd4f_init (0.0f, 0.0f, 1.0f, 0.0f);
  const graphene_simd4f_t r3 = graphene_simd4f_init (0.0f, 0.0f, 0.0f, 1.0f);

  return graphene_simd4f_cmp_eq (m->x, r0) &&
         graphene_simd4f_cmp_eq (m->y, r1) &&
         graphene_simd4f_cmp_eq (m->z, r2) &&
         graphene_simd4f_cmp_eq (m->w, r3);
}

/**
 * graphene_simd4x4f_is_2d:
 * @m: a #graphene_simd4x4f_t
 *
 * Checks whether the given matrix is compatible with an affine
 * transformation matrix.
 *
 * Returns: `true` if the matrix is compatible with an affine
 *   transformation matrix
 *
 * Since: 1.0
 */
static inline bool
graphene_simd4x4f_is_2d (const graphene_simd4x4f_t *m)
{
  float f[4];

  if (!(fabsf (graphene_simd4f_get_z (m->x)) < FLT_EPSILON && fabsf (graphene_simd4f_get_w (m->x)) < FLT_EPSILON))
    return false;

  if (!(fabsf (graphene_simd4f_get_z (m->y)) < FLT_EPSILON && fabsf (graphene_simd4f_get_w (m->y)) < FLT_EPSILON))
    return false;

  graphene_simd4f_dup_4f (m->z, f);
  if (!(fabsf (f[0]) < FLT_EPSILON &&
        fabsf (f[1]) < FLT_EPSILON &&
        1.f - fabsf (f[2]) < FLT_EPSILON &&
        fabsf (f[3]) < FLT_EPSILON))
    return false;

  if (!(fabsf (graphene_simd4f_get_z (m->w)) < FLT_EPSILON && 1.f - fabsf (graphene_simd4f_get_w (m->w)) < FLT_EPSILON))
    return false;

  return true;
}

GRAPHENE_END_DECLS
