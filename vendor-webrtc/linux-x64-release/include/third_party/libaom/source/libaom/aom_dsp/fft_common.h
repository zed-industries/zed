/*
 * Copyright (c) 2018, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AOM_DSP_FFT_COMMON_H_
#define AOM_AOM_DSP_FFT_COMMON_H_

#ifdef __cplusplus
extern "C" {
#endif

/*!\brief A function pointer for computing 1d fft and ifft.
 *
 * The function will point to an implementation for a specific transform size,
 * and may perform the transforms using vectorized instructions.
 *
 * For a non-vectorized forward transforms of size n, the input and output
 * buffers will be size n. The output takes advantage of conjugate symmetry and
 * packs the results as: [r_0, r_1, ..., r_{n/2}, i_1, ..., i_{n/2-1}], where
 * (r_{j}, i_{j}) is the complex output for index j.
 *
 * An inverse transform will assume that the complex "input" is packed
 * similarly. Its output will be real.
 *
 * Non-vectorized transforms (e.g., on a single row) would use a stride = 1.
 *
 * Vectorized implementations are parallelized along the columns so that the fft
 * can be performed on multiple columns at a time. In such cases the data block
 * for input and output is typically square (n x n) and the stride will
 * correspond to the spacing between rows. At minimum, the input size must be
 * n x simd_vector_length.
 *
 * \param[in]  input   Input buffer. See above for size restrictions.
 * \param[out] output  Output buffer. See above for size restrictions.
 * \param[in]  stride  The spacing in number of elements between rows
 *                     (or elements)
 */
typedef void (*aom_fft_1d_func_t)(const float *input, float *output,
                                  int stride);

// Declare some of the forward non-vectorized transforms which are used in some
// of the vectorized implementations
void aom_fft1d_2_float(const float *input, float *output, int stride);
void aom_fft1d_4_float(const float *input, float *output, int stride);
void aom_fft1d_8_float(const float *input, float *output, int stride);
void aom_fft1d_16_float(const float *input, float *output, int stride);
void aom_fft1d_32_float(const float *input, float *output, int stride);

/**\!brief Function pointer for transposing a matrix of floats.
 *
 * \param[in]  input  Input buffer (size n x n)
 * \param[out] output Output buffer (size n x n)
 * \param[in]  n      Extent of one dimension of the square matrix.
 */
typedef void (*aom_fft_transpose_func_t)(const float *input, float *output,
                                         int n);

/**\!brief Function pointer for re-arranging intermediate 2d transform results.
 *
 * After re-arrangement, the real and imaginary components will be packed
 * tightly next to each other.
 *
 * \param[in]  input  Input buffer (size n x n)
 * \param[out] output Output buffer (size 2 x n x n)
 * \param[in]  n      Extent of one dimension of the square matrix.
 */
typedef void (*aom_fft_unpack_func_t)(const float *input, float *output, int n);

/*!\brief Performs a 2d fft with the given functions.
 *
 * This generator function allows for multiple different implementations of 2d
 * fft with different vector operations, without having to redefine the main
 * body multiple times.
 *
 * \param[in]  input     Input buffer to run the transform on (size n x n)
 * \param[out] temp      Working buffer for computing the transform (size n x n)
 * \param[out] output    Output buffer (size 2 x n x n)
 * \param[in]  tform     Forward transform function
 * \param[in]  transpose Transpose function (for n x n matrix)
 * \param[in]  unpack    Unpack function used to massage outputs to correct form
 * \param[in]  vec_size  Vector size (the transform is done vec_size units at
 *                       a time)
 */
void aom_fft_2d_gen(const float *input, float *temp, float *output, int n,
                    aom_fft_1d_func_t tform, aom_fft_transpose_func_t transpose,
                    aom_fft_unpack_func_t unpack, int vec_size);

/*!\brief Perform a 2d inverse fft with the given helper functions
 *
 * \param[in]  input      Input buffer to run the transform on (size 2 x n x n)
 * \param[out] temp       Working buffer for computations (size 2 x n x n)
 * \param[out] output     Output buffer (size n x n)
 * \param[in]  fft_single Forward transform function (non vectorized)
 * \param[in]  fft_multi  Forward transform function (vectorized)
 * \param[in]  ifft_multi Inverse transform function (vectorized)
 * \param[in]  transpose  Transpose function (for n x n matrix)
 * \param[in]  vec_size   Vector size (the transform is done vec_size
 *                        units at a time)
 */
void aom_ifft_2d_gen(const float *input, float *temp, float *output, int n,
                     aom_fft_1d_func_t fft_single, aom_fft_1d_func_t fft_multi,
                     aom_fft_1d_func_t ifft_multi,
                     aom_fft_transpose_func_t transpose, int vec_size);
#ifdef __cplusplus
}
#endif

// The macros below define 1D fft/ifft for different data types and for
// different simd vector intrinsic types.

#define GEN_FFT_2(ret, suffix, T, T_VEC, load, store)               \
  ret aom_fft1d_2_##suffix(const T *input, T *output, int stride) { \
    const T_VEC i0 = load(input + 0 * stride);                      \
    const T_VEC i1 = load(input + 1 * stride);                      \
    store(output + 0 * stride, i0 + i1);                            \
    store(output + 1 * stride, i0 - i1);                            \
  }

#define GEN_FFT_4(ret, suffix, T, T_VEC, load, store, constant, add, sub) \
  ret aom_fft1d_4_##suffix(const T *input, T *output, int stride) {       \
    const T_VEC kWeight0 = constant(0.0f);                                \
    const T_VEC i0 = load(input + 0 * stride);                            \
    const T_VEC i1 = load(input + 1 * stride);                            \
    const T_VEC i2 = load(input + 2 * stride);                            \
    const T_VEC i3 = load(input + 3 * stride);                            \
    const T_VEC w0 = add(i0, i2);                                         \
    const T_VEC w1 = sub(i0, i2);                                         \
    const T_VEC w2 = add(i1, i3);                                         \
    const T_VEC w3 = sub(i1, i3);                                         \
    store(output + 0 * stride, add(w0, w2));                              \
    store(output + 1 * stride, w1);                                       \
    store(output + 2 * stride, sub(w0, w2));                              \
    store(output + 3 * stride, sub(kWeight0, w3));                        \
  }

#define GEN_FFT_8(ret, suffix, T, T_VEC, load, store, constant, add, sub, mul) \
  ret aom_fft1d_8_##suffix(const T *input, T *output, int stride) {            \
    const T_VEC kWeight0 = constant(0.0f);                                     \
    const T_VEC kWeight2 = constant(0.707107f);                                \
    const T_VEC i0 = load(input + 0 * stride);                                 \
    const T_VEC i1 = load(input + 1 * stride);                                 \
    const T_VEC i2 = load(input + 2 * stride);                                 \
    const T_VEC i3 = load(input + 3 * stride);                                 \
    const T_VEC i4 = load(input + 4 * stride);                                 \
    const T_VEC i5 = load(input + 5 * stride);                                 \
    const T_VEC i6 = load(input + 6 * stride);                                 \
    const T_VEC i7 = load(input + 7 * stride);                                 \
    const T_VEC w0 = add(i0, i4);                                              \
    const T_VEC w1 = sub(i0, i4);                                              \
    const T_VEC w2 = add(i2, i6);                                              \
    const T_VEC w3 = sub(i2, i6);                                              \
    const T_VEC w4 = add(w0, w2);                                              \
    const T_VEC w5 = sub(w0, w2);                                              \
    const T_VEC w7 = add(i1, i5);                                              \
    const T_VEC w8 = sub(i1, i5);                                              \
    const T_VEC w9 = add(i3, i7);                                              \
    const T_VEC w10 = sub(i3, i7);                                             \
    const T_VEC w11 = add(w7, w9);                                             \
    const T_VEC w12 = sub(w7, w9);                                             \
    store(output + 0 * stride, add(w4, w11));                                  \
    store(output + 1 * stride, add(w1, mul(kWeight2, sub(w8, w10))));          \
    store(output + 2 * stride, w5);                                            \
    store(output + 3 * stride, sub(w1, mul(kWeight2, sub(w8, w10))));          \
    store(output + 4 * stride, sub(w4, w11));                                  \
    store(output + 5 * stride,                                                 \
          sub(sub(kWeight0, w3), mul(kWeight2, add(w10, w8))));                \
    store(output + 6 * stride, sub(kWeight0, w12));                            \
    store(output + 7 * stride, sub(w3, mul(kWeight2, add(w10, w8))));          \
  }

#define GEN_FFT_16(ret, suffix, T, T_VEC, load, store, constant, add, sub, \
                   mul)                                                    \
  ret aom_fft1d_16_##suffix(const T *input, T *output, int stride) {       \
    const T_VEC kWeight0 = constant(0.0f);                                 \
    const T_VEC kWeight2 = constant(0.707107f);                            \
    const T_VEC kWeight3 = constant(0.92388f);                             \
    const T_VEC kWeight4 = constant(0.382683f);                            \
    const T_VEC i0 = load(input + 0 * stride);                             \
    const T_VEC i1 = load(input + 1 * stride);                             \
    const T_VEC i2 = load(input + 2 * stride);                             \
    const T_VEC i3 = load(input + 3 * stride);                             \
    const T_VEC i4 = load(input + 4 * stride);                             \
    const T_VEC i5 = load(input + 5 * stride);                             \
    const T_VEC i6 = load(input + 6 * stride);                             \
    const T_VEC i7 = load(input + 7 * stride);                             \
    const T_VEC i8 = load(input + 8 * stride);                             \
    const T_VEC i9 = load(input + 9 * stride);                             \
    const T_VEC i10 = load(input + 10 * stride);                           \
    const T_VEC i11 = load(input + 11 * stride);                           \
    const T_VEC i12 = load(input + 12 * stride);                           \
    const T_VEC i13 = load(input + 13 * stride);                           \
    const T_VEC i14 = load(input + 14 * stride);                           \
    const T_VEC i15 = load(input + 15 * stride);                           \
    const T_VEC w0 = add(i0, i8);                                          \
    const T_VEC w1 = sub(i0, i8);                                          \
    const T_VEC w2 = add(i4, i12);                                         \
    const T_VEC w3 = sub(i4, i12);                                         \
    const T_VEC w4 = add(w0, w2);                                          \
    const T_VEC w5 = sub(w0, w2);                                          \
    const T_VEC w7 = add(i2, i10);                                         \
    const T_VEC w8 = sub(i2, i10);                                         \
    const T_VEC w9 = add(i6, i14);                                         \
    const T_VEC w10 = sub(i6, i14);                                        \
    const T_VEC w11 = add(w7, w9);                                         \
    const T_VEC w12 = sub(w7, w9);                                         \
    const T_VEC w14 = add(w4, w11);                                        \
    const T_VEC w15 = sub(w4, w11);                                        \
    const T_VEC w16[2] = { add(w1, mul(kWeight2, sub(w8, w10))),           \
                           sub(sub(kWeight0, w3),                          \
                               mul(kWeight2, add(w10, w8))) };             \
    const T_VEC w18[2] = { sub(w1, mul(kWeight2, sub(w8, w10))),           \
                           sub(w3, mul(kWeight2, add(w10, w8))) };         \
    const T_VEC w19 = add(i1, i9);                                         \
    const T_VEC w20 = sub(i1, i9);                                         \
    const T_VEC w21 = add(i5, i13);                                        \
    const T_VEC w22 = sub(i5, i13);                                        \
    const T_VEC w23 = add(w19, w21);                                       \
    const T_VEC w24 = sub(w19, w21);                                       \
    const T_VEC w26 = add(i3, i11);                                        \
    const T_VEC w27 = sub(i3, i11);                                        \
    const T_VEC w28 = add(i7, i15);                                        \
    const T_VEC w29 = sub(i7, i15);                                        \
    const T_VEC w30 = add(w26, w28);                                       \
    const T_VEC w31 = sub(w26, w28);                                       \
    const T_VEC w33 = add(w23, w30);                                       \
    const T_VEC w34 = sub(w23, w30);                                       \
    const T_VEC w35[2] = { add(w20, mul(kWeight2, sub(w27, w29))),         \
                           sub(sub(kWeight0, w22),                         \
                               mul(kWeight2, add(w29, w27))) };            \
    const T_VEC w37[2] = { sub(w20, mul(kWeight2, sub(w27, w29))),         \
                           sub(w22, mul(kWeight2, add(w29, w27))) };       \
    store(output + 0 * stride, add(w14, w33));                             \
    store(output + 1 * stride,                                             \
          add(w16[0], add(mul(kWeight3, w35[0]), mul(kWeight4, w35[1])))); \
    store(output + 2 * stride, add(w5, mul(kWeight2, sub(w24, w31))));     \
    store(output + 3 * stride,                                             \
          add(w18[0], add(mul(kWeight4, w37[0]), mul(kWeight3, w37[1])))); \
    store(output + 4 * stride, w15);                                       \
    store(output + 5 * stride,                                             \
          add(w18[0], sub(sub(kWeight0, mul(kWeight4, w37[0])),            \
                          mul(kWeight3, w37[1]))));                        \
    store(output + 6 * stride, sub(w5, mul(kWeight2, sub(w24, w31))));     \
    store(output + 7 * stride,                                             \
          add(w16[0], sub(sub(kWeight0, mul(kWeight3, w35[0])),            \
                          mul(kWeight4, w35[1]))));                        \
    store(output + 8 * stride, sub(w14, w33));                             \
    store(output + 9 * stride,                                             \
          add(w16[1], sub(mul(kWeight3, w35[1]), mul(kWeight4, w35[0])))); \
    store(output + 10 * stride,                                            \
          sub(sub(kWeight0, w12), mul(kWeight2, add(w31, w24))));          \
    store(output + 11 * stride,                                            \
          add(w18[1], sub(mul(kWeight4, w37[1]), mul(kWeight3, w37[0])))); \
    store(output + 12 * stride, sub(kWeight0, w34));                       \
    store(output + 13 * stride,                                            \
          sub(sub(kWeight0, w18[1]),                                       \
              sub(mul(kWeight3, w37[0]), mul(kWeight4, w37[1]))));         \
    store(output + 14 * stride, sub(w12, mul(kWeight2, add(w31, w24))));   \
    store(output + 15 * stride,                                            \
          sub(sub(kWeight0, w16[1]),                                       \
              sub(mul(kWeight4, w35[0]), mul(kWeight3, w35[1]))));         \
  }

#define GEN_FFT_32(ret, suffix, T, T_VEC, load, store, constant, add, sub,   \
                   mul)                                                      \
  ret aom_fft1d_32_##suffix(const T *input, T *output, int stride) {         \
    const T_VEC kWeight0 = constant(0.0f);                                   \
    const T_VEC kWeight2 = constant(0.707107f);                              \
    const T_VEC kWeight3 = constant(0.92388f);                               \
    const T_VEC kWeight4 = constant(0.382683f);                              \
    const T_VEC kWeight5 = constant(0.980785f);                              \
    const T_VEC kWeight6 = constant(0.19509f);                               \
    const T_VEC kWeight7 = constant(0.83147f);                               \
    const T_VEC kWeight8 = constant(0.55557f);                               \
    const T_VEC i0 = load(input + 0 * stride);                               \
    const T_VEC i1 = load(input + 1 * stride);                               \
    const T_VEC i2 = load(input + 2 * stride);                               \
    const T_VEC i3 = load(input + 3 * stride);                               \
    const T_VEC i4 = load(input + 4 * stride);                               \
    const T_VEC i5 = load(input + 5 * stride);                               \
    const T_VEC i6 = load(input + 6 * stride);                               \
    const T_VEC i7 = load(input + 7 * stride);                               \
    const T_VEC i8 = load(input + 8 * stride);                               \
    const T_VEC i9 = load(input + 9 * stride);                               \
    const T_VEC i10 = load(input + 10 * stride);                             \
    const T_VEC i11 = load(input + 11 * stride);                             \
    const T_VEC i12 = load(input + 12 * stride);                             \
    const T_VEC i13 = load(input + 13 * stride);                             \
    const T_VEC i14 = load(input + 14 * stride);                             \
    const T_VEC i15 = load(input + 15 * stride);                             \
    const T_VEC i16 = load(input + 16 * stride);                             \
    const T_VEC i17 = load(input + 17 * stride);                             \
    const T_VEC i18 = load(input + 18 * stride);                             \
    const T_VEC i19 = load(input + 19 * stride);                             \
    const T_VEC i20 = load(input + 20 * stride);                             \
    const T_VEC i21 = load(input + 21 * stride);                             \
    const T_VEC i22 = load(input + 22 * stride);                             \
    const T_VEC i23 = load(input + 23 * stride);                             \
    const T_VEC i24 = load(input + 24 * stride);                             \
    const T_VEC i25 = load(input + 25 * stride);                             \
    const T_VEC i26 = load(input + 26 * stride);                             \
    const T_VEC i27 = load(input + 27 * stride);                             \
    const T_VEC i28 = load(input + 28 * stride);                             \
    const T_VEC i29 = load(input + 29 * stride);                             \
    const T_VEC i30 = load(input + 30 * stride);                             \
    const T_VEC i31 = load(input + 31 * stride);                             \
    const T_VEC w0 = add(i0, i16);                                           \
    const T_VEC w1 = sub(i0, i16);                                           \
    const T_VEC w2 = add(i8, i24);                                           \
    const T_VEC w3 = sub(i8, i24);                                           \
    const T_VEC w4 = add(w0, w2);                                            \
    const T_VEC w5 = sub(w0, w2);                                            \
    const T_VEC w7 = add(i4, i20);                                           \
    const T_VEC w8 = sub(i4, i20);                                           \
    const T_VEC w9 = add(i12, i28);                                          \
    const T_VEC w10 = sub(i12, i28);                                         \
    const T_VEC w11 = add(w7, w9);                                           \
    const T_VEC w12 = sub(w7, w9);                                           \
    const T_VEC w14 = add(w4, w11);                                          \
    const T_VEC w15 = sub(w4, w11);                                          \
    const T_VEC w16[2] = { add(w1, mul(kWeight2, sub(w8, w10))),             \
                           sub(sub(kWeight0, w3),                            \
                               mul(kWeight2, add(w10, w8))) };               \
    const T_VEC w18[2] = { sub(w1, mul(kWeight2, sub(w8, w10))),             \
                           sub(w3, mul(kWeight2, add(w10, w8))) };           \
    const T_VEC w19 = add(i2, i18);                                          \
    const T_VEC w20 = sub(i2, i18);                                          \
    const T_VEC w21 = add(i10, i26);                                         \
    const T_VEC w22 = sub(i10, i26);                                         \
    const T_VEC w23 = add(w19, w21);                                         \
    const T_VEC w24 = sub(w19, w21);                                         \
    const T_VEC w26 = add(i6, i22);                                          \
    const T_VEC w27 = sub(i6, i22);                                          \
    const T_VEC w28 = add(i14, i30);                                         \
    const T_VEC w29 = sub(i14, i30);                                         \
    const T_VEC w30 = add(w26, w28);                                         \
    const T_VEC w31 = sub(w26, w28);                                         \
    const T_VEC w33 = add(w23, w30);                                         \
    const T_VEC w34 = sub(w23, w30);                                         \
    const T_VEC w35[2] = { add(w20, mul(kWeight2, sub(w27, w29))),           \
                           sub(sub(kWeight0, w22),                           \
                               mul(kWeight2, add(w29, w27))) };              \
    const T_VEC w37[2] = { sub(w20, mul(kWeight2, sub(w27, w29))),           \
                           sub(w22, mul(kWeight2, add(w29, w27))) };         \
    const T_VEC w38 = add(w14, w33);                                         \
    const T_VEC w39 = sub(w14, w33);                                         \
    const T_VEC w40[2] = {                                                   \
      add(w16[0], add(mul(kWeight3, w35[0]), mul(kWeight4, w35[1]))),        \
      add(w16[1], sub(mul(kWeight3, w35[1]), mul(kWeight4, w35[0])))         \
    };                                                                       \
    const T_VEC w41[2] = { add(w5, mul(kWeight2, sub(w24, w31))),            \
                           sub(sub(kWeight0, w12),                           \
                               mul(kWeight2, add(w31, w24))) };              \
    const T_VEC w42[2] = {                                                   \
      add(w18[0], add(mul(kWeight4, w37[0]), mul(kWeight3, w37[1]))),        \
      add(w18[1], sub(mul(kWeight4, w37[1]), mul(kWeight3, w37[0])))         \
    };                                                                       \
    const T_VEC w44[2] = {                                                   \
      add(w18[0],                                                            \
          sub(sub(kWeight0, mul(kWeight4, w37[0])), mul(kWeight3, w37[1]))), \
      sub(sub(kWeight0, w18[1]),                                             \
          sub(mul(kWeight3, w37[0]), mul(kWeight4, w37[1])))                 \
    };                                                                       \
    const T_VEC w45[2] = { sub(w5, mul(kWeight2, sub(w24, w31))),            \
                           sub(w12, mul(kWeight2, add(w31, w24))) };         \
    const T_VEC w46[2] = {                                                   \
      add(w16[0],                                                            \
          sub(sub(kWeight0, mul(kWeight3, w35[0])), mul(kWeight4, w35[1]))), \
      sub(sub(kWeight0, w16[1]),                                             \
          sub(mul(kWeight4, w35[0]), mul(kWeight3, w35[1])))                 \
    };                                                                       \
    const T_VEC w47 = add(i1, i17);                                          \
    const T_VEC w48 = sub(i1, i17);                                          \
    const T_VEC w49 = add(i9, i25);                                          \
    const T_VEC w50 = sub(i9, i25);                                          \
    const T_VEC w51 = add(w47, w49);                                         \
    const T_VEC w52 = sub(w47, w49);                                         \
    const T_VEC w54 = add(i5, i21);                                          \
    const T_VEC w55 = sub(i5, i21);                                          \
    const T_VEC w56 = add(i13, i29);                                         \
    const T_VEC w57 = sub(i13, i29);                                         \
    const T_VEC w58 = add(w54, w56);                                         \
    const T_VEC w59 = sub(w54, w56);                                         \
    const T_VEC w61 = add(w51, w58);                                         \
    const T_VEC w62 = sub(w51, w58);                                         \
    const T_VEC w63[2] = { add(w48, mul(kWeight2, sub(w55, w57))),           \
                           sub(sub(kWeight0, w50),                           \
                               mul(kWeight2, add(w57, w55))) };              \
    const T_VEC w65[2] = { sub(w48, mul(kWeight2, sub(w55, w57))),           \
                           sub(w50, mul(kWeight2, add(w57, w55))) };         \
    const T_VEC w66 = add(i3, i19);                                          \
    const T_VEC w67 = sub(i3, i19);                                          \
    const T_VEC w68 = add(i11, i27);                                         \
    const T_VEC w69 = sub(i11, i27);                                         \
    const T_VEC w70 = add(w66, w68);                                         \
    const T_VEC w71 = sub(w66, w68);                                         \
    const T_VEC w73 = add(i7, i23);                                          \
    const T_VEC w74 = sub(i7, i23);                                          \
    const T_VEC w75 = add(i15, i31);                                         \
    const T_VEC w76 = sub(i15, i31);                                         \
    const T_VEC w77 = add(w73, w75);                                         \
    const T_VEC w78 = sub(w73, w75);                                         \
    const T_VEC w80 = add(w70, w77);                                         \
    const T_VEC w81 = sub(w70, w77);                                         \
    const T_VEC w82[2] = { add(w67, mul(kWeight2, sub(w74, w76))),           \
                           sub(sub(kWeight0, w69),                           \
                               mul(kWeight2, add(w76, w74))) };              \
    const T_VEC w84[2] = { sub(w67, mul(kWeight2, sub(w74, w76))),           \
                           sub(w69, mul(kWeight2, add(w76, w74))) };         \
    const T_VEC w85 = add(w61, w80);                                         \
    const T_VEC w86 = sub(w61, w80);                                         \
    const T_VEC w87[2] = {                                                   \
      add(w63[0], add(mul(kWeight3, w82[0]), mul(kWeight4, w82[1]))),        \
      add(w63[1], sub(mul(kWeight3, w82[1]), mul(kWeight4, w82[0])))         \
    };                                                                       \
    const T_VEC w88[2] = { add(w52, mul(kWeight2, sub(w71, w78))),           \
                           sub(sub(kWeight0, w59),                           \
                               mul(kWeight2, add(w78, w71))) };              \
    const T_VEC w89[2] = {                                                   \
      add(w65[0], add(mul(kWeight4, w84[0]), mul(kWeight3, w84[1]))),        \
      add(w65[1], sub(mul(kWeight4, w84[1]), mul(kWeight3, w84[0])))         \
    };                                                                       \
    const T_VEC w91[2] = {                                                   \
      add(w65[0],                                                            \
          sub(sub(kWeight0, mul(kWeight4, w84[0])), mul(kWeight3, w84[1]))), \
      sub(sub(kWeight0, w65[1]),                                             \
          sub(mul(kWeight3, w84[0]), mul(kWeight4, w84[1])))                 \
    };                                                                       \
    const T_VEC w92[2] = { sub(w52, mul(kWeight2, sub(w71, w78))),           \
                           sub(w59, mul(kWeight2, add(w78, w71))) };         \
    const T_VEC w93[2] = {                                                   \
      add(w63[0],                                                            \
          sub(sub(kWeight0, mul(kWeight3, w82[0])), mul(kWeight4, w82[1]))), \
      sub(sub(kWeight0, w63[1]),                                             \
          sub(mul(kWeight4, w82[0]), mul(kWeight3, w82[1])))                 \
    };                                                                       \
    store(output + 0 * stride, add(w38, w85));                               \
    store(output + 1 * stride,                                               \
          add(w40[0], add(mul(kWeight5, w87[0]), mul(kWeight6, w87[1]))));   \
    store(output + 2 * stride,                                               \
          add(w41[0], add(mul(kWeight3, w88[0]), mul(kWeight4, w88[1]))));   \
    store(output + 3 * stride,                                               \
          add(w42[0], add(mul(kWeight7, w89[0]), mul(kWeight8, w89[1]))));   \
    store(output + 4 * stride, add(w15, mul(kWeight2, sub(w62, w81))));      \
    store(output + 5 * stride,                                               \
          add(w44[0], add(mul(kWeight8, w91[0]), mul(kWeight7, w91[1]))));   \
    store(output + 6 * stride,                                               \
          add(w45[0], add(mul(kWeight4, w92[0]), mul(kWeight3, w92[1]))));   \
    store(output + 7 * stride,                                               \
          add(w46[0], add(mul(kWeight6, w93[0]), mul(kWeight5, w93[1]))));   \
    store(output + 8 * stride, w39);                                         \
    store(output + 9 * stride,                                               \
          add(w46[0], sub(sub(kWeight0, mul(kWeight6, w93[0])),              \
                          mul(kWeight5, w93[1]))));                          \
    store(output + 10 * stride,                                              \
          add(w45[0], sub(sub(kWeight0, mul(kWeight4, w92[0])),              \
                          mul(kWeight3, w92[1]))));                          \
    store(output + 11 * stride,                                              \
          add(w44[0], sub(sub(kWeight0, mul(kWeight8, w91[0])),              \
                          mul(kWeight7, w91[1]))));                          \
    store(output + 12 * stride, sub(w15, mul(kWeight2, sub(w62, w81))));     \
    store(output + 13 * stride,                                              \
          add(w42[0], sub(sub(kWeight0, mul(kWeight7, w89[0])),              \
                          mul(kWeight8, w89[1]))));                          \
    store(output + 14 * stride,                                              \
          add(w41[0], sub(sub(kWeight0, mul(kWeight3, w88[0])),              \
                          mul(kWeight4, w88[1]))));                          \
    store(output + 15 * stride,                                              \
          add(w40[0], sub(sub(kWeight0, mul(kWeight5, w87[0])),              \
                          mul(kWeight6, w87[1]))));                          \
    store(output + 16 * stride, sub(w38, w85));                              \
    store(output + 17 * stride,                                              \
          add(w40[1], sub(mul(kWeight5, w87[1]), mul(kWeight6, w87[0]))));   \
    store(output + 18 * stride,                                              \
          add(w41[1], sub(mul(kWeight3, w88[1]), mul(kWeight4, w88[0]))));   \
    store(output + 19 * stride,                                              \
          add(w42[1], sub(mul(kWeight7, w89[1]), mul(kWeight8, w89[0]))));   \
    store(output + 20 * stride,                                              \
          sub(sub(kWeight0, w34), mul(kWeight2, add(w81, w62))));            \
    store(output + 21 * stride,                                              \
          add(w44[1], sub(mul(kWeight8, w91[1]), mul(kWeight7, w91[0]))));   \
    store(output + 22 * stride,                                              \
          add(w45[1], sub(mul(kWeight4, w92[1]), mul(kWeight3, w92[0]))));   \
    store(output + 23 * stride,                                              \
          add(w46[1], sub(mul(kWeight6, w93[1]), mul(kWeight5, w93[0]))));   \
    store(output + 24 * stride, sub(kWeight0, w86));                         \
    store(output + 25 * stride,                                              \
          sub(sub(kWeight0, w46[1]),                                         \
              sub(mul(kWeight5, w93[0]), mul(kWeight6, w93[1]))));           \
    store(output + 26 * stride,                                              \
          sub(sub(kWeight0, w45[1]),                                         \
              sub(mul(kWeight3, w92[0]), mul(kWeight4, w92[1]))));           \
    store(output + 27 * stride,                                              \
          sub(sub(kWeight0, w44[1]),                                         \
              sub(mul(kWeight7, w91[0]), mul(kWeight8, w91[1]))));           \
    store(output + 28 * stride, sub(w34, mul(kWeight2, add(w81, w62))));     \
    store(output + 29 * stride,                                              \
          sub(sub(kWeight0, w42[1]),                                         \
              sub(mul(kWeight8, w89[0]), mul(kWeight7, w89[1]))));           \
    store(output + 30 * stride,                                              \
          sub(sub(kWeight0, w41[1]),                                         \
              sub(mul(kWeight4, w88[0]), mul(kWeight3, w88[1]))));           \
    store(output + 31 * stride,                                              \
          sub(sub(kWeight0, w40[1]),                                         \
              sub(mul(kWeight6, w87[0]), mul(kWeight5, w87[1]))));           \
  }

#define GEN_IFFT_2(ret, suffix, T, T_VEC, load, store)               \
  ret aom_ifft1d_2_##suffix(const T *input, T *output, int stride) { \
    const T_VEC i0 = load(input + 0 * stride);                       \
    const T_VEC i1 = load(input + 1 * stride);                       \
    store(output + 0 * stride, i0 + i1);                             \
    store(output + 1 * stride, i0 - i1);                             \
  }

#define GEN_IFFT_4(ret, suffix, T, T_VEC, load, store, constant, add, sub) \
  ret aom_ifft1d_4_##suffix(const T *input, T *output, int stride) {       \
    const T_VEC kWeight0 = constant(0.0f);                                 \
    const T_VEC i0 = load(input + 0 * stride);                             \
    const T_VEC i1 = load(input + 1 * stride);                             \
    const T_VEC i2 = load(input + 2 * stride);                             \
    const T_VEC i3 = load(input + 3 * stride);                             \
    const T_VEC w2 = add(i0, i2);                                          \
    const T_VEC w3 = sub(i0, i2);                                          \
    const T_VEC w4[2] = { add(i1, i1), sub(i3, i3) };                      \
    const T_VEC w5[2] = { sub(i1, i1), sub(sub(kWeight0, i3), i3) };       \
    store(output + 0 * stride, add(w2, w4[0]));                            \
    store(output + 1 * stride, add(w3, w5[1]));                            \
    store(output + 2 * stride, sub(w2, w4[0]));                            \
    store(output + 3 * stride, sub(w3, w5[1]));                            \
  }

#define GEN_IFFT_8(ret, suffix, T, T_VEC, load, store, constant, add, sub, \
                   mul)                                                    \
  ret aom_ifft1d_8_##suffix(const T *input, T *output, int stride) {       \
    const T_VEC kWeight0 = constant(0.0f);                                 \
    const T_VEC kWeight2 = constant(0.707107f);                            \
    const T_VEC i0 = load(input + 0 * stride);                             \
    const T_VEC i1 = load(input + 1 * stride);                             \
    const T_VEC i2 = load(input + 2 * stride);                             \
    const T_VEC i3 = load(input + 3 * stride);                             \
    const T_VEC i4 = load(input + 4 * stride);                             \
    const T_VEC i5 = load(input + 5 * stride);                             \
    const T_VEC i6 = load(input + 6 * stride);                             \
    const T_VEC i7 = load(input + 7 * stride);                             \
    const T_VEC w6 = add(i0, i4);                                          \
    const T_VEC w7 = sub(i0, i4);                                          \
    const T_VEC w8[2] = { add(i2, i2), sub(i6, i6) };                      \
    const T_VEC w9[2] = { sub(i2, i2), sub(sub(kWeight0, i6), i6) };       \
    const T_VEC w10[2] = { add(w6, w8[0]), w8[1] };                        \
    const T_VEC w11[2] = { sub(w6, w8[0]), sub(kWeight0, w8[1]) };         \
    const T_VEC w12[2] = { add(w7, w9[1]), sub(kWeight0, w9[0]) };         \
    const T_VEC w13[2] = { sub(w7, w9[1]), w9[0] };                        \
    const T_VEC w14[2] = { add(i1, i3), sub(i7, i5) };                     \
    const T_VEC w15[2] = { sub(i1, i3), sub(sub(kWeight0, i5), i7) };      \
    const T_VEC w16[2] = { add(i3, i1), sub(i5, i7) };                     \
    const T_VEC w17[2] = { sub(i3, i1), sub(sub(kWeight0, i7), i5) };      \
    const T_VEC w18[2] = { add(w14[0], w16[0]), add(w14[1], w16[1]) };     \
    const T_VEC w19[2] = { sub(w14[0], w16[0]), sub(w14[1], w16[1]) };     \
    const T_VEC w20[2] = { add(w15[0], w17[1]), sub(w15[1], w17[0]) };     \
    const T_VEC w21[2] = { sub(w15[0], w17[1]), add(w15[1], w17[0]) };     \
    store(output + 0 * stride, add(w10[0], w18[0]));                       \
    store(output + 1 * stride,                                             \
          add(w12[0], mul(kWeight2, add(w20[0], w20[1]))));                \
    store(output + 2 * stride, add(w11[0], w19[1]));                       \
    store(output + 3 * stride,                                             \
          sub(w13[0], mul(kWeight2, sub(w21[0], w21[1]))));                \
    store(output + 4 * stride, sub(w10[0], w18[0]));                       \
    store(output + 5 * stride,                                             \
          add(w12[0], sub(sub(kWeight0, mul(kWeight2, w20[0])),            \
                          mul(kWeight2, w20[1]))));                        \
    store(output + 6 * stride, sub(w11[0], w19[1]));                       \
    store(output + 7 * stride,                                             \
          add(w13[0], mul(kWeight2, sub(w21[0], w21[1]))));                \
  }

#define GEN_IFFT_16(ret, suffix, T, T_VEC, load, store, constant, add, sub,   \
                    mul)                                                      \
  ret aom_ifft1d_16_##suffix(const T *input, T *output, int stride) {         \
    const T_VEC kWeight0 = constant(0.0f);                                    \
    const T_VEC kWeight2 = constant(0.707107f);                               \
    const T_VEC kWeight3 = constant(0.92388f);                                \
    const T_VEC kWeight4 = constant(0.382683f);                               \
    const T_VEC i0 = load(input + 0 * stride);                                \
    const T_VEC i1 = load(input + 1 * stride);                                \
    const T_VEC i2 = load(input + 2 * stride);                                \
    const T_VEC i3 = load(input + 3 * stride);                                \
    const T_VEC i4 = load(input + 4 * stride);                                \
    const T_VEC i5 = load(input + 5 * stride);                                \
    const T_VEC i6 = load(input + 6 * stride);                                \
    const T_VEC i7 = load(input + 7 * stride);                                \
    const T_VEC i8 = load(input + 8 * stride);                                \
    const T_VEC i9 = load(input + 9 * stride);                                \
    const T_VEC i10 = load(input + 10 * stride);                              \
    const T_VEC i11 = load(input + 11 * stride);                              \
    const T_VEC i12 = load(input + 12 * stride);                              \
    const T_VEC i13 = load(input + 13 * stride);                              \
    const T_VEC i14 = load(input + 14 * stride);                              \
    const T_VEC i15 = load(input + 15 * stride);                              \
    const T_VEC w14 = add(i0, i8);                                            \
    const T_VEC w15 = sub(i0, i8);                                            \
    const T_VEC w16[2] = { add(i4, i4), sub(i12, i12) };                      \
    const T_VEC w17[2] = { sub(i4, i4), sub(sub(kWeight0, i12), i12) };       \
    const T_VEC w18[2] = { add(w14, w16[0]), w16[1] };                        \
    const T_VEC w19[2] = { sub(w14, w16[0]), sub(kWeight0, w16[1]) };         \
    const T_VEC w20[2] = { add(w15, w17[1]), sub(kWeight0, w17[0]) };         \
    const T_VEC w21[2] = { sub(w15, w17[1]), w17[0] };                        \
    const T_VEC w22[2] = { add(i2, i6), sub(i14, i10) };                      \
    const T_VEC w23[2] = { sub(i2, i6), sub(sub(kWeight0, i10), i14) };       \
    const T_VEC w24[2] = { add(i6, i2), sub(i10, i14) };                      \
    const T_VEC w25[2] = { sub(i6, i2), sub(sub(kWeight0, i14), i10) };       \
    const T_VEC w26[2] = { add(w22[0], w24[0]), add(w22[1], w24[1]) };        \
    const T_VEC w27[2] = { sub(w22[0], w24[0]), sub(w22[1], w24[1]) };        \
    const T_VEC w28[2] = { add(w23[0], w25[1]), sub(w23[1], w25[0]) };        \
    const T_VEC w29[2] = { sub(w23[0], w25[1]), add(w23[1], w25[0]) };        \
    const T_VEC w30[2] = { add(w18[0], w26[0]), add(w18[1], w26[1]) };        \
    const T_VEC w31[2] = { sub(w18[0], w26[0]), sub(w18[1], w26[1]) };        \
    const T_VEC w32[2] = { add(w20[0], mul(kWeight2, add(w28[0], w28[1]))),   \
                           add(w20[1], mul(kWeight2, sub(w28[1], w28[0]))) }; \
    const T_VEC w33[2] = { add(w20[0],                                        \
                               sub(sub(kWeight0, mul(kWeight2, w28[0])),      \
                                   mul(kWeight2, w28[1]))),                   \
                           add(w20[1], mul(kWeight2, sub(w28[0], w28[1]))) }; \
    const T_VEC w34[2] = { add(w19[0], w27[1]), sub(w19[1], w27[0]) };        \
    const T_VEC w35[2] = { sub(w19[0], w27[1]), add(w19[1], w27[0]) };        \
    const T_VEC w36[2] = { sub(w21[0], mul(kWeight2, sub(w29[0], w29[1]))),   \
                           sub(w21[1], mul(kWeight2, add(w29[1], w29[0]))) }; \
    const T_VEC w37[2] = { add(w21[0], mul(kWeight2, sub(w29[0], w29[1]))),   \
                           add(w21[1], mul(kWeight2, add(w29[1], w29[0]))) }; \
    const T_VEC w38[2] = { add(i1, i7), sub(i15, i9) };                       \
    const T_VEC w39[2] = { sub(i1, i7), sub(sub(kWeight0, i9), i15) };        \
    const T_VEC w40[2] = { add(i5, i3), sub(i11, i13) };                      \
    const T_VEC w41[2] = { sub(i5, i3), sub(sub(kWeight0, i13), i11) };       \
    const T_VEC w42[2] = { add(w38[0], w40[0]), add(w38[1], w40[1]) };        \
    const T_VEC w43[2] = { sub(w38[0], w40[0]), sub(w38[1], w40[1]) };        \
    const T_VEC w44[2] = { add(w39[0], w41[1]), sub(w39[1], w41[0]) };        \
    const T_VEC w45[2] = { sub(w39[0], w41[1]), add(w39[1], w41[0]) };        \
    const T_VEC w46[2] = { add(i3, i5), sub(i13, i11) };                      \
    const T_VEC w47[2] = { sub(i3, i5), sub(sub(kWeight0, i11), i13) };       \
    const T_VEC w48[2] = { add(i7, i1), sub(i9, i15) };                       \
    const T_VEC w49[2] = { sub(i7, i1), sub(sub(kWeight0, i15), i9) };        \
    const T_VEC w50[2] = { add(w46[0], w48[0]), add(w46[1], w48[1]) };        \
    const T_VEC w51[2] = { sub(w46[0], w48[0]), sub(w46[1], w48[1]) };        \
    const T_VEC w52[2] = { add(w47[0], w49[1]), sub(w47[1], w49[0]) };        \
    const T_VEC w53[2] = { sub(w47[0], w49[1]), add(w47[1], w49[0]) };        \
    const T_VEC w54[2] = { add(w42[0], w50[0]), add(w42[1], w50[1]) };        \
    const T_VEC w55[2] = { sub(w42[0], w50[0]), sub(w42[1], w50[1]) };        \
    const T_VEC w56[2] = { add(w44[0], mul(kWeight2, add(w52[0], w52[1]))),   \
                           add(w44[1], mul(kWeight2, sub(w52[1], w52[0]))) }; \
    const T_VEC w57[2] = { add(w44[0],                                        \
                               sub(sub(kWeight0, mul(kWeight2, w52[0])),      \
                                   mul(kWeight2, w52[1]))),                   \
                           add(w44[1], mul(kWeight2, sub(w52[0], w52[1]))) }; \
    const T_VEC w58[2] = { add(w43[0], w51[1]), sub(w43[1], w51[0]) };        \
    const T_VEC w59[2] = { sub(w43[0], w51[1]), add(w43[1], w51[0]) };        \
    const T_VEC w60[2] = { sub(w45[0], mul(kWeight2, sub(w53[0], w53[1]))),   \
                           sub(w45[1], mul(kWeight2, add(w53[1], w53[0]))) }; \
    const T_VEC w61[2] = { add(w45[0], mul(kWeight2, sub(w53[0], w53[1]))),   \
                           add(w45[1], mul(kWeight2, add(w53[1], w53[0]))) }; \
    store(output + 0 * stride, add(w30[0], w54[0]));                          \
    store(output + 1 * stride,                                                \
          add(w32[0], add(mul(kWeight3, w56[0]), mul(kWeight4, w56[1]))));    \
    store(output + 2 * stride,                                                \
          add(w34[0], mul(kWeight2, add(w58[0], w58[1]))));                   \
    store(output + 3 * stride,                                                \
          add(w36[0], add(mul(kWeight4, w60[0]), mul(kWeight3, w60[1]))));    \
    store(output + 4 * stride, add(w31[0], w55[1]));                          \
    store(output + 5 * stride,                                                \
          sub(w33[0], sub(mul(kWeight4, w57[0]), mul(kWeight3, w57[1]))));    \
    store(output + 6 * stride,                                                \
          sub(w35[0], mul(kWeight2, sub(w59[0], w59[1]))));                   \
    store(output + 7 * stride,                                                \
          sub(w37[0], sub(mul(kWeight3, w61[0]), mul(kWeight4, w61[1]))));    \
    store(output + 8 * stride, sub(w30[0], w54[0]));                          \
    store(output + 9 * stride,                                                \
          add(w32[0], sub(sub(kWeight0, mul(kWeight3, w56[0])),               \
                          mul(kWeight4, w56[1]))));                           \
    store(output + 10 * stride,                                               \
          add(w34[0], sub(sub(kWeight0, mul(kWeight2, w58[0])),               \
                          mul(kWeight2, w58[1]))));                           \
    store(output + 11 * stride,                                               \
          add(w36[0], sub(sub(kWeight0, mul(kWeight4, w60[0])),               \
                          mul(kWeight3, w60[1]))));                           \
    store(output + 12 * stride, sub(w31[0], w55[1]));                         \
    store(output + 13 * stride,                                               \
          add(w33[0], sub(mul(kWeight4, w57[0]), mul(kWeight3, w57[1]))));    \
    store(output + 14 * stride,                                               \
          add(w35[0], mul(kWeight2, sub(w59[0], w59[1]))));                   \
    store(output + 15 * stride,                                               \
          add(w37[0], sub(mul(kWeight3, w61[0]), mul(kWeight4, w61[1]))));    \
  }
#define GEN_IFFT_32(ret, suffix, T, T_VEC, load, store, constant, add, sub,    \
                    mul)                                                       \
  ret aom_ifft1d_32_##suffix(const T *input, T *output, int stride) {          \
    const T_VEC kWeight0 = constant(0.0f);                                     \
    const T_VEC kWeight2 = constant(0.707107f);                                \
    const T_VEC kWeight3 = constant(0.92388f);                                 \
    const T_VEC kWeight4 = constant(0.382683f);                                \
    const T_VEC kWeight5 = constant(0.980785f);                                \
    const T_VEC kWeight6 = constant(0.19509f);                                 \
    const T_VEC kWeight7 = constant(0.83147f);                                 \
    const T_VEC kWeight8 = constant(0.55557f);                                 \
    const T_VEC i0 = load(input + 0 * stride);                                 \
    const T_VEC i1 = load(input + 1 * stride);                                 \
    const T_VEC i2 = load(input + 2 * stride);                                 \
    const T_VEC i3 = load(input + 3 * stride);                                 \
    const T_VEC i4 = load(input + 4 * stride);                                 \
    const T_VEC i5 = load(input + 5 * stride);                                 \
    const T_VEC i6 = load(input + 6 * stride);                                 \
    const T_VEC i7 = load(input + 7 * stride);                                 \
    const T_VEC i8 = load(input + 8 * stride);                                 \
    const T_VEC i9 = load(input + 9 * stride);                                 \
    const T_VEC i10 = load(input + 10 * stride);                               \
    const T_VEC i11 = load(input + 11 * stride);                               \
    const T_VEC i12 = load(input + 12 * stride);                               \
    const T_VEC i13 = load(input + 13 * stride);                               \
    const T_VEC i14 = load(input + 14 * stride);                               \
    const T_VEC i15 = load(input + 15 * stride);                               \
    const T_VEC i16 = load(input + 16 * stride);                               \
    const T_VEC i17 = load(input + 17 * stride);                               \
    const T_VEC i18 = load(input + 18 * stride);                               \
    const T_VEC i19 = load(input + 19 * stride);                               \
    const T_VEC i20 = load(input + 20 * stride);                               \
    const T_VEC i21 = load(input + 21 * stride);                               \
    const T_VEC i22 = load(input + 22 * stride);                               \
    const T_VEC i23 = load(input + 23 * stride);                               \
    const T_VEC i24 = load(input + 24 * stride);                               \
    const T_VEC i25 = load(input + 25 * stride);                               \
    const T_VEC i26 = load(input + 26 * stride);                               \
    const T_VEC i27 = load(input + 27 * stride);                               \
    const T_VEC i28 = load(input + 28 * stride);                               \
    const T_VEC i29 = load(input + 29 * stride);                               \
    const T_VEC i30 = load(input + 30 * stride);                               \
    const T_VEC i31 = load(input + 31 * stride);                               \
    const T_VEC w30 = add(i0, i16);                                            \
    const T_VEC w31 = sub(i0, i16);                                            \
    const T_VEC w32[2] = { add(i8, i8), sub(i24, i24) };                       \
    const T_VEC w33[2] = { sub(i8, i8), sub(sub(kWeight0, i24), i24) };        \
    const T_VEC w34[2] = { add(w30, w32[0]), w32[1] };                         \
    const T_VEC w35[2] = { sub(w30, w32[0]), sub(kWeight0, w32[1]) };          \
    const T_VEC w36[2] = { add(w31, w33[1]), sub(kWeight0, w33[0]) };          \
    const T_VEC w37[2] = { sub(w31, w33[1]), w33[0] };                         \
    const T_VEC w38[2] = { add(i4, i12), sub(i28, i20) };                      \
    const T_VEC w39[2] = { sub(i4, i12), sub(sub(kWeight0, i20), i28) };       \
    const T_VEC w40[2] = { add(i12, i4), sub(i20, i28) };                      \
    const T_VEC w41[2] = { sub(i12, i4), sub(sub(kWeight0, i28), i20) };       \
    const T_VEC w42[2] = { add(w38[0], w40[0]), add(w38[1], w40[1]) };         \
    const T_VEC w43[2] = { sub(w38[0], w40[0]), sub(w38[1], w40[1]) };         \
    const T_VEC w44[2] = { add(w39[0], w41[1]), sub(w39[1], w41[0]) };         \
    const T_VEC w45[2] = { sub(w39[0], w41[1]), add(w39[1], w41[0]) };         \
    const T_VEC w46[2] = { add(w34[0], w42[0]), add(w34[1], w42[1]) };         \
    const T_VEC w47[2] = { sub(w34[0], w42[0]), sub(w34[1], w42[1]) };         \
    const T_VEC w48[2] = { add(w36[0], mul(kWeight2, add(w44[0], w44[1]))),    \
                           add(w36[1], mul(kWeight2, sub(w44[1], w44[0]))) };  \
    const T_VEC w49[2] = { add(w36[0],                                         \
                               sub(sub(kWeight0, mul(kWeight2, w44[0])),       \
                                   mul(kWeight2, w44[1]))),                    \
                           add(w36[1], mul(kWeight2, sub(w44[0], w44[1]))) };  \
    const T_VEC w50[2] = { add(w35[0], w43[1]), sub(w35[1], w43[0]) };         \
    const T_VEC w51[2] = { sub(w35[0], w43[1]), add(w35[1], w43[0]) };         \
    const T_VEC w52[2] = { sub(w37[0], mul(kWeight2, sub(w45[0], w45[1]))),    \
                           sub(w37[1], mul(kWeight2, add(w45[1], w45[0]))) };  \
    const T_VEC w53[2] = { add(w37[0], mul(kWeight2, sub(w45[0], w45[1]))),    \
                           add(w37[1], mul(kWeight2, add(w45[1], w45[0]))) };  \
    const T_VEC w54[2] = { add(i2, i14), sub(i30, i18) };                      \
    const T_VEC w55[2] = { sub(i2, i14), sub(sub(kWeight0, i18), i30) };       \
    const T_VEC w56[2] = { add(i10, i6), sub(i22, i26) };                      \
    const T_VEC w57[2] = { sub(i10, i6), sub(sub(kWeight0, i26), i22) };       \
    const T_VEC w58[2] = { add(w54[0], w56[0]), add(w54[1], w56[1]) };         \
    const T_VEC w59[2] = { sub(w54[0], w56[0]), sub(w54[1], w56[1]) };         \
    const T_VEC w60[2] = { add(w55[0], w57[1]), sub(w55[1], w57[0]) };         \
    const T_VEC w61[2] = { sub(w55[0], w57[1]), add(w55[1], w57[0]) };         \
    const T_VEC w62[2] = { add(i6, i10), sub(i26, i22) };                      \
    const T_VEC w63[2] = { sub(i6, i10), sub(sub(kWeight0, i22), i26) };       \
    const T_VEC w64[2] = { add(i14, i2), sub(i18, i30) };                      \
    const T_VEC w65[2] = { sub(i14, i2), sub(sub(kWeight0, i30), i18) };       \
    const T_VEC w66[2] = { add(w62[0], w64[0]), add(w62[1], w64[1]) };         \
    const T_VEC w67[2] = { sub(w62[0], w64[0]), sub(w62[1], w64[1]) };         \
    const T_VEC w68[2] = { add(w63[0], w65[1]), sub(w63[1], w65[0]) };         \
    const T_VEC w69[2] = { sub(w63[0], w65[1]), add(w63[1], w65[0]) };         \
    const T_VEC w70[2] = { add(w58[0], w66[0]), add(w58[1], w66[1]) };         \
    const T_VEC w71[2] = { sub(w58[0], w66[0]), sub(w58[1], w66[1]) };         \
    const T_VEC w72[2] = { add(w60[0], mul(kWeight2, add(w68[0], w68[1]))),    \
                           add(w60[1], mul(kWeight2, sub(w68[1], w68[0]))) };  \
    const T_VEC w73[2] = { add(w60[0],                                         \
                               sub(sub(kWeight0, mul(kWeight2, w68[0])),       \
                                   mul(kWeight2, w68[1]))),                    \
                           add(w60[1], mul(kWeight2, sub(w68[0], w68[1]))) };  \
    const T_VEC w74[2] = { add(w59[0], w67[1]), sub(w59[1], w67[0]) };         \
    const T_VEC w75[2] = { sub(w59[0], w67[1]), add(w59[1], w67[0]) };         \
    const T_VEC w76[2] = { sub(w61[0], mul(kWeight2, sub(w69[0], w69[1]))),    \
                           sub(w61[1], mul(kWeight2, add(w69[1], w69[0]))) };  \
    const T_VEC w77[2] = { add(w61[0], mul(kWeight2, sub(w69[0], w69[1]))),    \
                           add(w61[1], mul(kWeight2, add(w69[1], w69[0]))) };  \
    const T_VEC w78[2] = { add(w46[0], w70[0]), add(w46[1], w70[1]) };         \
    const T_VEC w79[2] = { sub(w46[0], w70[0]), sub(w46[1], w70[1]) };         \
    const T_VEC w80[2] = {                                                     \
      add(w48[0], add(mul(kWeight3, w72[0]), mul(kWeight4, w72[1]))),          \
      add(w48[1], sub(mul(kWeight3, w72[1]), mul(kWeight4, w72[0])))           \
    };                                                                         \
    const T_VEC w81[2] = {                                                     \
      add(w48[0],                                                              \
          sub(sub(kWeight0, mul(kWeight3, w72[0])), mul(kWeight4, w72[1]))),   \
      add(w48[1], sub(mul(kWeight4, w72[0]), mul(kWeight3, w72[1])))           \
    };                                                                         \
    const T_VEC w82[2] = { add(w50[0], mul(kWeight2, add(w74[0], w74[1]))),    \
                           add(w50[1], mul(kWeight2, sub(w74[1], w74[0]))) };  \
    const T_VEC w83[2] = { add(w50[0],                                         \
                               sub(sub(kWeight0, mul(kWeight2, w74[0])),       \
                                   mul(kWeight2, w74[1]))),                    \
                           add(w50[1], mul(kWeight2, sub(w74[0], w74[1]))) };  \
    const T_VEC w84[2] = {                                                     \
      add(w52[0], add(mul(kWeight4, w76[0]), mul(kWeight3, w76[1]))),          \
      add(w52[1], sub(mul(kWeight4, w76[1]), mul(kWeight3, w76[0])))           \
    };                                                                         \
    const T_VEC w85[2] = {                                                     \
      add(w52[0],                                                              \
          sub(sub(kWeight0, mul(kWeight4, w76[0])), mul(kWeight3, w76[1]))),   \
      add(w52[1], sub(mul(kWeight3, w76[0]), mul(kWeight4, w76[1])))           \
    };                                                                         \
    const T_VEC w86[2] = { add(w47[0], w71[1]), sub(w47[1], w71[0]) };         \
    const T_VEC w87[2] = { sub(w47[0], w71[1]), add(w47[1], w71[0]) };         \
    const T_VEC w88[2] = {                                                     \
      sub(w49[0], sub(mul(kWeight4, w73[0]), mul(kWeight3, w73[1]))),          \
      add(w49[1],                                                              \
          sub(sub(kWeight0, mul(kWeight4, w73[1])), mul(kWeight3, w73[0])))    \
    };                                                                         \
    const T_VEC w89[2] = {                                                     \
      add(w49[0], sub(mul(kWeight4, w73[0]), mul(kWeight3, w73[1]))),          \
      add(w49[1], add(mul(kWeight4, w73[1]), mul(kWeight3, w73[0])))           \
    };                                                                         \
    const T_VEC w90[2] = { sub(w51[0], mul(kWeight2, sub(w75[0], w75[1]))),    \
                           sub(w51[1], mul(kWeight2, add(w75[1], w75[0]))) };  \
    const T_VEC w91[2] = { add(w51[0], mul(kWeight2, sub(w75[0], w75[1]))),    \
                           add(w51[1], mul(kWeight2, add(w75[1], w75[0]))) };  \
    const T_VEC w92[2] = {                                                     \
      sub(w53[0], sub(mul(kWeight3, w77[0]), mul(kWeight4, w77[1]))),          \
      add(w53[1],                                                              \
          sub(sub(kWeight0, mul(kWeight3, w77[1])), mul(kWeight4, w77[0])))    \
    };                                                                         \
    const T_VEC w93[2] = {                                                     \
      add(w53[0], sub(mul(kWeight3, w77[0]), mul(kWeight4, w77[1]))),          \
      add(w53[1], add(mul(kWeight3, w77[1]), mul(kWeight4, w77[0])))           \
    };                                                                         \
    const T_VEC w94[2] = { add(i1, i15), sub(i31, i17) };                      \
    const T_VEC w95[2] = { sub(i1, i15), sub(sub(kWeight0, i17), i31) };       \
    const T_VEC w96[2] = { add(i9, i7), sub(i23, i25) };                       \
    const T_VEC w97[2] = { sub(i9, i7), sub(sub(kWeight0, i25), i23) };        \
    const T_VEC w98[2] = { add(w94[0], w96[0]), add(w94[1], w96[1]) };         \
    const T_VEC w99[2] = { sub(w94[0], w96[0]), sub(w94[1], w96[1]) };         \
    const T_VEC w100[2] = { add(w95[0], w97[1]), sub(w95[1], w97[0]) };        \
    const T_VEC w101[2] = { sub(w95[0], w97[1]), add(w95[1], w97[0]) };        \
    const T_VEC w102[2] = { add(i5, i11), sub(i27, i21) };                     \
    const T_VEC w103[2] = { sub(i5, i11), sub(sub(kWeight0, i21), i27) };      \
    const T_VEC w104[2] = { add(i13, i3), sub(i19, i29) };                     \
    const T_VEC w105[2] = { sub(i13, i3), sub(sub(kWeight0, i29), i19) };      \
    const T_VEC w106[2] = { add(w102[0], w104[0]), add(w102[1], w104[1]) };    \
    const T_VEC w107[2] = { sub(w102[0], w104[0]), sub(w102[1], w104[1]) };    \
    const T_VEC w108[2] = { add(w103[0], w105[1]), sub(w103[1], w105[0]) };    \
    const T_VEC w109[2] = { sub(w103[0], w105[1]), add(w103[1], w105[0]) };    \
    const T_VEC w110[2] = { add(w98[0], w106[0]), add(w98[1], w106[1]) };      \
    const T_VEC w111[2] = { sub(w98[0], w106[0]), sub(w98[1], w106[1]) };      \
    const T_VEC w112[2] = {                                                    \
      add(w100[0], mul(kWeight2, add(w108[0], w108[1]))),                      \
      add(w100[1], mul(kWeight2, sub(w108[1], w108[0])))                       \
    };                                                                         \
    const T_VEC w113[2] = {                                                    \
      add(w100[0],                                                             \
          sub(sub(kWeight0, mul(kWeight2, w108[0])), mul(kWeight2, w108[1]))), \
      add(w100[1], mul(kWeight2, sub(w108[0], w108[1])))                       \
    };                                                                         \
    const T_VEC w114[2] = { add(w99[0], w107[1]), sub(w99[1], w107[0]) };      \
    const T_VEC w115[2] = { sub(w99[0], w107[1]), add(w99[1], w107[0]) };      \
    const T_VEC w116[2] = {                                                    \
      sub(w101[0], mul(kWeight2, sub(w109[0], w109[1]))),                      \
      sub(w101[1], mul(kWeight2, add(w109[1], w109[0])))                       \
    };                                                                         \
    const T_VEC w117[2] = {                                                    \
      add(w101[0], mul(kWeight2, sub(w109[0], w109[1]))),                      \
      add(w101[1], mul(kWeight2, add(w109[1], w109[0])))                       \
    };                                                                         \
    const T_VEC w118[2] = { add(i3, i13), sub(i29, i19) };                     \
    const T_VEC w119[2] = { sub(i3, i13), sub(sub(kWeight0, i19), i29) };      \
    const T_VEC w120[2] = { add(i11, i5), sub(i21, i27) };                     \
    const T_VEC w121[2] = { sub(i11, i5), sub(sub(kWeight0, i27), i21) };      \
    const T_VEC w122[2] = { add(w118[0], w120[0]), add(w118[1], w120[1]) };    \
    const T_VEC w123[2] = { sub(w118[0], w120[0]), sub(w118[1], w120[1]) };    \
    const T_VEC w124[2] = { add(w119[0], w121[1]), sub(w119[1], w121[0]) };    \
    const T_VEC w125[2] = { sub(w119[0], w121[1]), add(w119[1], w121[0]) };    \
    const T_VEC w126[2] = { add(i7, i9), sub(i25, i23) };                      \
    const T_VEC w127[2] = { sub(i7, i9), sub(sub(kWeight0, i23), i25) };       \
    const T_VEC w128[2] = { add(i15, i1), sub(i17, i31) };                     \
    const T_VEC w129[2] = { sub(i15, i1), sub(sub(kWeight0, i31), i17) };      \
    const T_VEC w130[2] = { add(w126[0], w128[0]), add(w126[1], w128[1]) };    \
    const T_VEC w131[2] = { sub(w126[0], w128[0]), sub(w126[1], w128[1]) };    \
    const T_VEC w132[2] = { add(w127[0], w129[1]), sub(w127[1], w129[0]) };    \
    const T_VEC w133[2] = { sub(w127[0], w129[1]), add(w127[1], w129[0]) };    \
    const T_VEC w134[2] = { add(w122[0], w130[0]), add(w122[1], w130[1]) };    \
    const T_VEC w135[2] = { sub(w122[0], w130[0]), sub(w122[1], w130[1]) };    \
    const T_VEC w136[2] = {                                                    \
      add(w124[0], mul(kWeight2, add(w132[0], w132[1]))),                      \
      add(w124[1], mul(kWeight2, sub(w132[1], w132[0])))                       \
    };                                                                         \
    const T_VEC w137[2] = {                                                    \
      add(w124[0],                                                             \
          sub(sub(kWeight0, mul(kWeight2, w132[0])), mul(kWeight2, w132[1]))), \
      add(w124[1], mul(kWeight2, sub(w132[0], w132[1])))                       \
    };                                                                         \
    const T_VEC w138[2] = { add(w123[0], w131[1]), sub(w123[1], w131[0]) };    \
    const T_VEC w139[2] = { sub(w123[0], w131[1]), add(w123[1], w131[0]) };    \
    const T_VEC w140[2] = {                                                    \
      sub(w125[0], mul(kWeight2, sub(w133[0], w133[1]))),                      \
      sub(w125[1], mul(kWeight2, add(w133[1], w133[0])))                       \
    };                                                                         \
    const T_VEC w141[2] = {                                                    \
      add(w125[0], mul(kWeight2, sub(w133[0], w133[1]))),                      \
      add(w125[1], mul(kWeight2, add(w133[1], w133[0])))                       \
    };                                                                         \
    const T_VEC w142[2] = { add(w110[0], w134[0]), add(w110[1], w134[1]) };    \
    const T_VEC w143[2] = { sub(w110[0], w134[0]), sub(w110[1], w134[1]) };    \
    const T_VEC w144[2] = {                                                    \
      add(w112[0], add(mul(kWeight3, w136[0]), mul(kWeight4, w136[1]))),       \
      add(w112[1], sub(mul(kWeight3, w136[1]), mul(kWeight4, w136[0])))        \
    };                                                                         \
    const T_VEC w145[2] = {                                                    \
      add(w112[0],                                                             \
          sub(sub(kWeight0, mul(kWeight3, w136[0])), mul(kWeight4, w136[1]))), \
      add(w112[1], sub(mul(kWeight4, w136[0]), mul(kWeight3, w136[1])))        \
    };                                                                         \
    const T_VEC w146[2] = {                                                    \
      add(w114[0], mul(kWeight2, add(w138[0], w138[1]))),                      \
      add(w114[1], mul(kWeight2, sub(w138[1], w138[0])))                       \
    };                                                                         \
    const T_VEC w147[2] = {                                                    \
      add(w114[0],                                                             \
          sub(sub(kWeight0, mul(kWeight2, w138[0])), mul(kWeight2, w138[1]))), \
      add(w114[1], mul(kWeight2, sub(w138[0], w138[1])))                       \
    };                                                                         \
    const T_VEC w148[2] = {                                                    \
      add(w116[0], add(mul(kWeight4, w140[0]), mul(kWeight3, w140[1]))),       \
      add(w116[1], sub(mul(kWeight4, w140[1]), mul(kWeight3, w140[0])))        \
    };                                                                         \
    const T_VEC w149[2] = {                                                    \
      add(w116[0],                                                             \
          sub(sub(kWeight0, mul(kWeight4, w140[0])), mul(kWeight3, w140[1]))), \
      add(w116[1], sub(mul(kWeight3, w140[0]), mul(kWeight4, w140[1])))        \
    };                                                                         \
    const T_VEC w150[2] = { add(w111[0], w135[1]), sub(w111[1], w135[0]) };    \
    const T_VEC w151[2] = { sub(w111[0], w135[1]), add(w111[1], w135[0]) };    \
    const T_VEC w152[2] = {                                                    \
      sub(w113[0], sub(mul(kWeight4, w137[0]), mul(kWeight3, w137[1]))),       \
      add(w113[1],                                                             \
          sub(sub(kWeight0, mul(kWeight4, w137[1])), mul(kWeight3, w137[0])))  \
    };                                                                         \
    const T_VEC w153[2] = {                                                    \
      add(w113[0], sub(mul(kWeight4, w137[0]), mul(kWeight3, w137[1]))),       \
      add(w113[1], add(mul(kWeight4, w137[1]), mul(kWeight3, w137[0])))        \
    };                                                                         \
    const T_VEC w154[2] = {                                                    \
      sub(w115[0], mul(kWeight2, sub(w139[0], w139[1]))),                      \
      sub(w115[1], mul(kWeight2, add(w139[1], w139[0])))                       \
    };                                                                         \
    const T_VEC w155[2] = {                                                    \
      add(w115[0], mul(kWeight2, sub(w139[0], w139[1]))),                      \
      add(w115[1], mul(kWeight2, add(w139[1], w139[0])))                       \
    };                                                                         \
    const T_VEC w156[2] = {                                                    \
      sub(w117[0], sub(mul(kWeight3, w141[0]), mul(kWeight4, w141[1]))),       \
      add(w117[1],                                                             \
          sub(sub(kWeight0, mul(kWeight3, w141[1])), mul(kWeight4, w141[0])))  \
    };                                                                         \
    const T_VEC w157[2] = {                                                    \
      add(w117[0], sub(mul(kWeight3, w141[0]), mul(kWeight4, w141[1]))),       \
      add(w117[1], add(mul(kWeight3, w141[1]), mul(kWeight4, w141[0])))        \
    };                                                                         \
    store(output + 0 * stride, add(w78[0], w142[0]));                          \
    store(output + 1 * stride,                                                 \
          add(w80[0], add(mul(kWeight5, w144[0]), mul(kWeight6, w144[1]))));   \
    store(output + 2 * stride,                                                 \
          add(w82[0], add(mul(kWeight3, w146[0]), mul(kWeight4, w146[1]))));   \
    store(output + 3 * stride,                                                 \
          add(w84[0], add(mul(kWeight7, w148[0]), mul(kWeight8, w148[1]))));   \
    store(output + 4 * stride,                                                 \
          add(w86[0], mul(kWeight2, add(w150[0], w150[1]))));                  \
    store(output + 5 * stride,                                                 \
          add(w88[0], add(mul(kWeight8, w152[0]), mul(kWeight7, w152[1]))));   \
    store(output + 6 * stride,                                                 \
          add(w90[0], add(mul(kWeight4, w154[0]), mul(kWeight3, w154[1]))));   \
    store(output + 7 * stride,                                                 \
          add(w92[0], add(mul(kWeight6, w156[0]), mul(kWeight5, w156[1]))));   \
    store(output + 8 * stride, add(w79[0], w143[1]));                          \
    store(output + 9 * stride,                                                 \
          sub(w81[0], sub(mul(kWeight6, w145[0]), mul(kWeight5, w145[1]))));   \
    store(output + 10 * stride,                                                \
          sub(w83[0], sub(mul(kWeight4, w147[0]), mul(kWeight3, w147[1]))));   \
    store(output + 11 * stride,                                                \
          sub(w85[0], sub(mul(kWeight8, w149[0]), mul(kWeight7, w149[1]))));   \
    store(output + 12 * stride,                                                \
          sub(w87[0], mul(kWeight2, sub(w151[0], w151[1]))));                  \
    store(output + 13 * stride,                                                \
          sub(w89[0], sub(mul(kWeight7, w153[0]), mul(kWeight8, w153[1]))));   \
    store(output + 14 * stride,                                                \
          sub(w91[0], sub(mul(kWeight3, w155[0]), mul(kWeight4, w155[1]))));   \
    store(output + 15 * stride,                                                \
          sub(w93[0], sub(mul(kWeight5, w157[0]), mul(kWeight6, w157[1]))));   \
    store(output + 16 * stride, sub(w78[0], w142[0]));                         \
    store(output + 17 * stride,                                                \
          add(w80[0], sub(sub(kWeight0, mul(kWeight5, w144[0])),               \
                          mul(kWeight6, w144[1]))));                           \
    store(output + 18 * stride,                                                \
          add(w82[0], sub(sub(kWeight0, mul(kWeight3, w146[0])),               \
                          mul(kWeight4, w146[1]))));                           \
    store(output + 19 * stride,                                                \
          add(w84[0], sub(sub(kWeight0, mul(kWeight7, w148[0])),               \
                          mul(kWeight8, w148[1]))));                           \
    store(output + 20 * stride,                                                \
          add(w86[0], sub(sub(kWeight0, mul(kWeight2, w150[0])),               \
                          mul(kWeight2, w150[1]))));                           \
    store(output + 21 * stride,                                                \
          add(w88[0], sub(sub(kWeight0, mul(kWeight8, w152[0])),               \
                          mul(kWeight7, w152[1]))));                           \
    store(output + 22 * stride,                                                \
          add(w90[0], sub(sub(kWeight0, mul(kWeight4, w154[0])),               \
                          mul(kWeight3, w154[1]))));                           \
    store(output + 23 * stride,                                                \
          add(w92[0], sub(sub(kWeight0, mul(kWeight6, w156[0])),               \
                          mul(kWeight5, w156[1]))));                           \
    store(output + 24 * stride, sub(w79[0], w143[1]));                         \
    store(output + 25 * stride,                                                \
          add(w81[0], sub(mul(kWeight6, w145[0]), mul(kWeight5, w145[1]))));   \
    store(output + 26 * stride,                                                \
          add(w83[0], sub(mul(kWeight4, w147[0]), mul(kWeight3, w147[1]))));   \
    store(output + 27 * stride,                                                \
          add(w85[0], sub(mul(kWeight8, w149[0]), mul(kWeight7, w149[1]))));   \
    store(output + 28 * stride,                                                \
          add(w87[0], mul(kWeight2, sub(w151[0], w151[1]))));                  \
    store(output + 29 * stride,                                                \
          add(w89[0], sub(mul(kWeight7, w153[0]), mul(kWeight8, w153[1]))));   \
    store(output + 30 * stride,                                                \
          add(w91[0], sub(mul(kWeight3, w155[0]), mul(kWeight4, w155[1]))));   \
    store(output + 31 * stride,                                                \
          add(w93[0], sub(mul(kWeight5, w157[0]), mul(kWeight6, w157[1]))));   \
  }

#endif  // AOM_AOM_DSP_FFT_COMMON_H_
