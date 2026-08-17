// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

// In this file we will implement elliptic curve point operations for
// NIST curves P-256, P-384, and P-521. The idea is to implement the operations
// in a generic way such that the code can be reused instead of having
// a separate implementation for each of the curves. We implement:
//   1. point addition,
//   2. point doubling,
//   3. scalar multiplication of a base point,
//   4. scalar multiplication of an arbitrary point,
//   5. scalar multiplication of a base and an arbitrary point.
//
// Matrix of what has been done so far:
// 
// | op | P-521 | P-384 | P-256 |
// |----------------------------|
// | 1. |   x   |   x   |   x*  |
// | 2. |   x   |   x   |   x*  |
// | 3. |   x   |   x   |   x*  |
// | 4. |   x   |   x   |   x*  |
// | 5. |   x   |   x   |   x*  |
//  * For P-256, only the Fiat-crypto implementation in p256.c is replaced. 

#include "ec_nistp.h"

// Some of the functions below need temporary field element variables.
// To avoid dynamic allocation we define nistp_felem type to have the maximum
// size possible (which is currently P-521 curve). The values are hard-coded
// for the moment, this will be fixed when we migrate the whole P-521
// implementation to ec_nistp.c.
#if defined(EC_NISTP_USE_64BIT_LIMB)
#define FELEM_MAX_NUM_OF_LIMBS (9)
#else
#define FELEM_MAX_NUM_OF_LIMBS (19)
#endif
typedef ec_nistp_felem_limb ec_nistp_felem[FELEM_MAX_NUM_OF_LIMBS];

// Conditional copy in constant-time (out = t == 0 ? z : nz).
static void cmovznz(ec_nistp_felem_limb *out,
                    size_t num_limbs,
                    ec_nistp_felem_limb t,
                    const ec_nistp_felem_limb *z,
                    const ec_nistp_felem_limb *nz) {
  ec_nistp_felem_limb mask = constant_time_is_zero_w(t);
  for (size_t i = 0; i < num_limbs; i++) {
    out[i] = constant_time_select_w(mask, z[i], nz[i]);
  }
}

// Group operations
// ----------------
//
// Building on top of the field operations we have the operations on the
// elliptic curve group itself. Points on the curve are represented in Jacobian
// coordinates.
//
// ec_nistp_point_double calculates 2*(x_in, y_in, z_in)
//
// The method is based on:
//   http://hyperelliptic.org/EFD/g1p/auto-shortw-jacobian-3.html#doubling-dbl-2001-b
// for which there is a Coq transcription and correctness proof:
//   <https://github.com/mit-plv/fiat-crypto/blob/79f8b5f39ed609339f0233098dee1a3c4e6b3080/src/Curves/Weierstrass/Jacobian.v#L93>
//   <https://github.com/mit-plv/fiat-crypto/blob/79f8b5f39ed609339f0233098dee1a3c4e6b3080/src/Curves/Weierstrass/Jacobian.v#L201>
//
// However, we slighty changed the computation for efficiency (see the full
// explanation within the function body), which makes the Coq proof above
// not applicable to our implementation.
// TODO(awslc): Write a Coq correctness proof for our version of the algorithm.
//
// Outputs can equal corresponding inputs, i.e., x_out == x_in is allowed;
// while x_out == y_in is not (maybe this works, but it's not tested).
void ec_nistp_point_double(const ec_nistp_meth *ctx,
                           ec_nistp_felem_limb *x_out,
                           ec_nistp_felem_limb *y_out,
                           ec_nistp_felem_limb *z_out,
                           const ec_nistp_felem_limb *x_in,
                           const ec_nistp_felem_limb *y_in,
                           const ec_nistp_felem_limb *z_in) {
  ec_nistp_felem delta, gamma, beta, ftmp, ftmp2, tmptmp, alpha, fourbeta;
  // delta = z^2
  ctx->felem_sqr(delta, z_in);
  // gamma = y^2
  ctx->felem_sqr(gamma, y_in);
  // beta = x*gamma
  ctx->felem_mul(beta, x_in, gamma);

  // alpha = 3*(x-delta)*(x+delta)
  ctx->felem_sub(ftmp, x_in, delta);
  ctx->felem_add(ftmp2, x_in, delta);

  ctx->felem_add(tmptmp, ftmp2, ftmp2);
  ctx->felem_add(ftmp2, ftmp2, tmptmp);
  ctx->felem_mul(alpha, ftmp, ftmp2);

  // x' = alpha^2 - 8*beta
  ctx->felem_sqr(x_out, alpha);
  ctx->felem_add(fourbeta, beta, beta);
  ctx->felem_add(fourbeta, fourbeta, fourbeta);
  ctx->felem_add(tmptmp, fourbeta, fourbeta);
  ctx->felem_sub(x_out, x_out, tmptmp);

  // z' = (y + z)^2 - gamma - delta
  // The following calculation differs from the Coq proof cited above.
  // The proof is for:
  //   add(delta, gamma, delta);
  //   add(ftmp, y_in, z_in);
  //   square(z_out, ftmp);
  //   sub(z_out, z_out, delta);
  // Our operations sequence is a bit more efficient because it saves us
  // a certain number of conditional moves.
  ctx->felem_add(ftmp, y_in, z_in);
  ctx->felem_sqr(z_out, ftmp);
  ctx->felem_sub(z_out, z_out, gamma);
  ctx->felem_sub(z_out, z_out, delta);

  // y' = alpha*(4*beta - x') - 8*gamma^2
  ctx->felem_sub(y_out, fourbeta, x_out);
  ctx->felem_add(gamma, gamma, gamma);
  ctx->felem_sqr(gamma, gamma);
  ctx->felem_mul(y_out, alpha, y_out);
  ctx->felem_add(gamma, gamma, gamma);
  ctx->felem_sub(y_out, y_out, gamma);
}

// ec_nistp_point_add calculates (x1, y1, z1) + (x2, y2, z2)
//
// The method is taken from:
//   http://hyperelliptic.org/EFD/g1p/auto-shortw-jacobian.html#addition-add-2007-bl
// adapted for mixed addition (z2 = 1, or z2 = 0 for the point at infinity).
//
// Coq transcription and correctness proof:
// <https://github.com/davidben/fiat-crypto/blob/c7b95f62b2a54b559522573310e9b487327d219a/src/Curves/Weierstrass/Jacobian.v#L467>
// <https://github.com/davidben/fiat-crypto/blob/c7b95f62b2a54b559522573310e9b487327d219a/src/Curves/Weierstrass/Jacobian.v#L544>
//
// This function includes a branch for checking whether the two input points
// are equal, (while not equal to the point at infinity). This case should
// never happen during single point multiplication, so there is no timing leak
// for ECDH and ECDSA.
void ec_nistp_point_add(const ec_nistp_meth *ctx,
                        ec_nistp_felem_limb *x3,
                        ec_nistp_felem_limb *y3,
                        ec_nistp_felem_limb *z3,
                        const ec_nistp_felem_limb *x1,
                        const ec_nistp_felem_limb *y1,
                        const ec_nistp_felem_limb *z1,
                        const int mixed,
                        const ec_nistp_felem_limb *x2,
                        const ec_nistp_felem_limb *y2,
                        const ec_nistp_felem_limb *z2) {
  ec_nistp_felem x_out, y_out, z_out;

  ec_nistp_felem_limb z1nz = ctx->felem_nz(z1);
  ec_nistp_felem_limb z2nz = ctx->felem_nz(z2);

  // z1z1 = z1**2
  ec_nistp_felem z1z1;
  ctx->felem_sqr(z1z1, z1);

  ec_nistp_felem u1, s1, two_z1z2;
  if (!mixed) {
    // z2z2 = z2**2
    ec_nistp_felem z2z2;
    ctx->felem_sqr(z2z2, z2);

    // u1 = x1*z2z2
    ctx->felem_mul(u1, x1, z2z2);

    // two_z1z2 = (z1 + z2)**2 - (z1z1 + z2z2) = 2z1z2
    ctx->felem_add(two_z1z2, z1, z2);
    ctx->felem_sqr(two_z1z2, two_z1z2);
    ctx->felem_sub(two_z1z2, two_z1z2, z1z1);
    ctx->felem_sub(two_z1z2, two_z1z2, z2z2);

    // s1 = y1 * z2**3
    ctx->felem_mul(s1, z2, z2z2);
    ctx->felem_mul(s1, s1, y1);
  } else {
    // We'll assume z2 = 1 (special case z2 = 0 is handled later).

    // u1 = x1*z2z2
    OPENSSL_memcpy(u1, x1, ctx->felem_num_limbs * sizeof(ec_nistp_felem_limb));
    // two_z1z2 = 2z1z2
    ctx->felem_add(two_z1z2, z1, z1);
    // s1 = y1 * z2**3
    OPENSSL_memcpy(s1, y1, ctx->felem_num_limbs * sizeof(ec_nistp_felem_limb));
  }

  // u2 = x2*z1z1
  ec_nistp_felem u2;
  ctx->felem_mul(u2, x2, z1z1);

  // h = u2 - u1
  ec_nistp_felem h;
  ctx->felem_sub(h, u2, u1);

  ec_nistp_felem_limb xneq = ctx->felem_nz(h);

  // z_out = two_z1z2 * h
  ctx->felem_mul(z_out, h, two_z1z2);

  // z1z1z1 = z1 * z1z1
  ec_nistp_felem z1z1z1;
  ctx->felem_mul(z1z1z1, z1, z1z1);

  // s2 = y2 * z1**3
  ec_nistp_felem s2;
  ctx->felem_mul(s2, y2, z1z1z1);

  // r = (s2 - s1)*2
  ec_nistp_felem r;
  ctx->felem_sub(r, s2, s1);
  ctx->felem_add(r, r, r);

  ec_nistp_felem_limb yneq = ctx->felem_nz(r);

  // This case will never occur in the constant-time |ec_GFp_mont_mul|.
  ec_nistp_felem_limb is_nontrivial_double =
                                     constant_time_is_zero_w(xneq | yneq) &
                                    ~constant_time_is_zero_w(z1nz) &
                                    ~constant_time_is_zero_w(z2nz);
  if (constant_time_declassify_w(is_nontrivial_double)) {
    ec_nistp_point_double(ctx, x3, y3, z3, x1, y1, z1);
    return;
  }

  // I = (2h)**2
  ec_nistp_felem i;
  ctx->felem_add(i, h, h);
  ctx->felem_sqr(i, i);

  // J = h * I
  ec_nistp_felem j;
  ctx->felem_mul(j, h, i);

  // V = U1 * I
  ec_nistp_felem v;
  ctx->felem_mul(v, u1, i);

  // x_out = r**2 - J - 2V
  ctx->felem_sqr(x_out, r);
  ctx->felem_sub(x_out, x_out, j);
  ctx->felem_sub(x_out, x_out, v);
  ctx->felem_sub(x_out, x_out, v);

  // y_out = r(V-x_out) - 2 * s1 * J
  ctx->felem_sub(y_out, v, x_out);
  ctx->felem_mul(y_out, y_out, r);
  ec_nistp_felem s1j;
  ctx->felem_mul(s1j, s1, j);
  ctx->felem_sub(y_out, y_out, s1j);
  ctx->felem_sub(y_out, y_out, s1j);

  cmovznz(x_out, ctx->felem_num_limbs, z1nz, x2, x_out);
  cmovznz(y_out, ctx->felem_num_limbs, z1nz, y2, y_out);
  cmovznz(z_out, ctx->felem_num_limbs, z1nz, z2, z_out);
  cmovznz(x3, ctx->felem_num_limbs, z2nz, x1, x_out);
  cmovznz(y3, ctx->felem_num_limbs, z2nz, y1, y_out);
  cmovznz(z3, ctx->felem_num_limbs, z2nz, z1, z_out);
}

// Returns i-th bit of the scalar (zero or one).
// The caller is responsible for making sure i is within bounds of the scalar. 
static int16_t get_bit(const EC_SCALAR *in, size_t i) {
// |in->words| is an array of BN_ULONGs which can be either 8 or 4 bytes long.
#if defined(OPENSSL_64_BIT)
  OPENSSL_STATIC_ASSERT(sizeof(BN_ULONG) == 8, bn_ulong_not_eight_bytes);
  return (in->words[i >> 6] >> (i & 63)) & 1;
#else
  OPENSSL_STATIC_ASSERT(sizeof(BN_ULONG) == 4, bn_ulong_not_four_bytes);
  return (in->words[i >> 5] >> (i & 31)) & 1;
#endif
}

#define DIV_AND_CEIL(a, b) ((a + b - 1) / b)

// Compute "regular" wNAF representation of a scalar, see
// Joye, Tunstall, "Exponent Recoding and Regular Exponentiation Algorithms",
// AfricaCrypt 2009, Alg 6.
// It forces an odd scalar and outputs digits in
// {\pm 1, \pm 3, \pm 5, \pm 7, \pm 9, ...}
// i.e. signed odd digits with _no zeroes_ -- that makes it "regular".
static void scalar_rwnaf(int16_t *out, size_t window_size,
                         const EC_SCALAR *scalar, size_t scalar_bit_size) {
  assert(window_size < 14);

  // The assert above ensures this works correctly.
  const int16_t window_mask = (1 << (window_size + 1)) - 1;
  int16_t window = (int16_t)(scalar->words[0] & (BN_ULONG)window_mask);
  window |= 1;

  const size_t num_windows = DIV_AND_CEIL(scalar_bit_size, window_size);
  for (size_t i = 0; i < num_windows - 1; i++) {
    int16_t d = (window & window_mask) - (int16_t)(1 << window_size);
    out[i] = d;
    window = (window - d) >> window_size;
    for (size_t j = 1; j <= window_size; j++) {
      size_t idx = (i + 1) * window_size + j;
      if (idx < scalar_bit_size) {
        window |= get_bit(scalar, idx) << j;
      }
    }
  }
  out[num_windows - 1] = window;
}

// The window size for scalar multiplication is hard coded for now.
#define SCALAR_MUL_WINDOW_SIZE (5)
#define SCALAR_MUL_TABLE_NUM_POINTS (1 << (SCALAR_MUL_WINDOW_SIZE - 1))

// To avoid dynamic allocation and freeing of memory in functions below
// we define maximum values of certain variables.
//
// The maximum number of limbs the table in |ec_nistp_scalar_mul| can have.
// Each point in the table has 3 coordinates that are field elements,
// and each field element has a defined maximum number of limbs.
#define SCALAR_MUL_TABLE_MAX_NUM_FELEM_LIMBS \
                (SCALAR_MUL_TABLE_NUM_POINTS * 3 * FELEM_MAX_NUM_OF_LIMBS)

// The maximum number of bits for a scalar.
#define SCALAR_MUL_MAX_SCALAR_BITS (521)

// Maximum number of windows (digits) for a scalar encoding which is
// determined by the maximum scalar bit size -- 521 bits in our case.
#define SCALAR_MUL_MAX_NUM_WINDOWS \
                DIV_AND_CEIL(SCALAR_MUL_MAX_SCALAR_BITS, SCALAR_MUL_WINDOW_SIZE)

// Generate table of multiples of the input point P = (x_in, y_in, z_in):
//  table <-- [2i + 1]P for i in [0, SCALAR_MUL_TABLE_NUM_POINTS - 1].
static void generate_table(const ec_nistp_meth *ctx,
                           ec_nistp_felem_limb *table,
                           const ec_nistp_felem_limb *x_in,
                           const ec_nistp_felem_limb *y_in,
                           const ec_nistp_felem_limb *z_in)
{
  const size_t felem_num_limbs = ctx->felem_num_limbs;
  const size_t felem_num_bytes = felem_num_limbs * sizeof(ec_nistp_felem_limb);

  // Helper variables to access individual coordinates of a point.
  const size_t x_idx = 0;
  const size_t y_idx = felem_num_limbs;
  const size_t z_idx = felem_num_limbs * 2;

  // table[0] <-- P.
  OPENSSL_memcpy(&table[x_idx], x_in, felem_num_bytes);
  OPENSSL_memcpy(&table[y_idx], y_in, felem_num_bytes);
  OPENSSL_memcpy(&table[z_idx], z_in, felem_num_bytes);

  // Compute 2P.
  ec_nistp_felem x_in_dbl, y_in_dbl, z_in_dbl;
  ctx->point_dbl(x_in_dbl, y_in_dbl, z_in_dbl,
                 &table[x_idx], &table[y_idx], &table[z_idx]);

  // Compute the rest of the table.
  for (size_t i = 1; i < SCALAR_MUL_TABLE_NUM_POINTS; i++) {
    // Just getting pointers to i-th and (i-1)-th point in the table.
    ec_nistp_felem_limb *point_i = &table[i * 3 * felem_num_limbs];
    ec_nistp_felem_limb *point_im1 = &table[(i - 1) * 3 * felem_num_limbs];

    // table[i] <-- table[i - 1] + 2P
    ctx->point_add(&point_i[x_idx], &point_i[y_idx], &point_i[z_idx],
                   &point_im1[x_idx], &point_im1[y_idx], &point_im1[z_idx],
                   0, x_in_dbl, y_in_dbl, z_in_dbl);
  }
}

// Writes to out the idx-th point from table in constant-time.
static inline void select_point_from_table(const ec_nistp_meth *ctx,
                                           ec_nistp_felem_limb *out,
                                           const ec_nistp_felem_limb *table,
                                           const size_t idx,
                                           const size_t projective) {
  // if projective != 0 then a point is (x, y, z), otherwise (x, y).
  size_t point_num_coord = 2 + (projective != 0 ? 1 : 0);
  size_t point_num_limbs = ctx->felem_num_limbs * point_num_coord;

  // The ifdef branching below is temporary. Using only constant_..._table_8
  // would be best for simplicity, but unfortunatelly, on x86 systems it is
  // significantly slower than constant_..._table_w.
#if defined(EC_NISTP_USE_64BIT_LIMB) && defined(OPENSSL_64_BIT)
  constant_time_select_entry_from_table_w(out, (crypto_word_t*) table, idx,
          SCALAR_MUL_TABLE_NUM_POINTS, point_num_limbs);
#else
  size_t entry_size = point_num_limbs * sizeof(ec_nistp_felem_limb);
  constant_time_select_entry_from_table_8((uint8_t*)out, (uint8_t*)table,
          idx, SCALAR_MUL_TABLE_NUM_POINTS, entry_size);
#endif
}

// Multiplication of an arbitrary point by a scalar, r = [scalar]P.
// The product is computed with the use of a small table generated on-the-fly
// and the scalar recoded in the regular-wNAF representation.
//
// The precomputed (on-the-fly) table |table| holds odd multiples of P:
//     [2i + 1]P for i in [0, SCALAR_MUL_TABLE_NUM_POINTS - 1].
// Computing the negation of a point P = (x, y, z) is relatively easy:
//     -P = (x, -y, z),
// so we may assume that for each point we have its negative as well.
//
// The scalar is recoded (regular-wNAF encoding) into signed digits as explained
// in |scalar_rwnaf| function. Namely, for a window size |w| we have:
//     scalar' = s_0 + s_1*2^w + s_2*2^(2*w) + ... + s_{m-1}*2^((m-1)*w),
// where digits s_i are in [\pm 1, \pm 3, ..., \pm (2^w-1)] and
// m = ceil(scalar_bit_size / w). Note that for an odd scalar we have that
// scalar = scalar', while in the case of an even scalar we have that
// scalar = scalar' - 1.
//
// The required product, [scalar]P, is computed by the following algorithm.
//     1. Initialize the accumulator with the point from |table|
//        corresponding to the most significant digit s_{m-1} of the scalar.
//     2. For digits s_i starting from s_{m-2} down to s_0:
//     3.   Double the accumulator w times. (note that doubling a point [a]P
//          w times results in [2^w*a]P).
//     4.   Read from |table| the point corresponding to abs(s_i),
//          negate it if s_i is negative, and add it to the accumulator.
//     5. Subtract P from the result if the scalar is even.
//
// Note: this function is constant-time.
void ec_nistp_scalar_mul(const ec_nistp_meth *ctx,
                         ec_nistp_felem_limb *x_out,
                         ec_nistp_felem_limb *y_out,
                         ec_nistp_felem_limb *z_out,
                         const ec_nistp_felem_limb *x_in,
                         const ec_nistp_felem_limb *y_in,
                         const ec_nistp_felem_limb *z_in,
                         const EC_SCALAR *scalar) {
  // Make sure that the max table size is large enough.
  assert(SCALAR_MUL_TABLE_MAX_NUM_FELEM_LIMBS >=
         SCALAR_MUL_TABLE_NUM_POINTS * ctx->felem_num_limbs * 3);

  // Table of multiples of P = (x_in, y_in, z_in).
  ec_nistp_felem_limb table[SCALAR_MUL_TABLE_MAX_NUM_FELEM_LIMBS];
  generate_table(ctx, table, x_in, y_in, z_in);

  // Regular-wNAF encoding of the scalar.
  int16_t rwnaf[SCALAR_MUL_MAX_NUM_WINDOWS];
  scalar_rwnaf(rwnaf, SCALAR_MUL_WINDOW_SIZE, scalar, ctx->felem_num_bits);

  // We need two point accumulators, so we define them of maximum size
  // to avoid allocation, and just take pointers to individual coordinates.
  // (This cruft will dissapear when we refactor point_add/dbl to work with
  // whole points instead of individual coordinates).
  ec_nistp_felem_limb res[3 * FELEM_MAX_NUM_OF_LIMBS] = {0};
  ec_nistp_felem_limb tmp[3 * FELEM_MAX_NUM_OF_LIMBS] = {0};
  ec_nistp_felem_limb *x_res = &res[0];
  ec_nistp_felem_limb *y_res = &res[ctx->felem_num_limbs];
  ec_nistp_felem_limb *z_res = &res[ctx->felem_num_limbs * 2];
  ec_nistp_felem_limb *x_tmp = &tmp[0];
  ec_nistp_felem_limb *y_tmp = &tmp[ctx->felem_num_limbs];
  ec_nistp_felem_limb *z_tmp = &tmp[ctx->felem_num_limbs * 2];

  // The actual number of windows (digits) of the scalar (denoted by m in the
  // description above the function).
  const size_t num_windows = DIV_AND_CEIL(ctx->felem_num_bits, SCALAR_MUL_WINDOW_SIZE);

  // Step 1. Initialize the accmulator (res) with the input point multiplied by
  // the most significant digit of the scalar s_{m-1} (note that this digit
  // can't be negative).
  int16_t idx = rwnaf[num_windows - 1];
  idx >>= 1;
  select_point_from_table(ctx, res, table, idx, 1);

  // Step 2. Process the remaining digits of the scalar (s_{m-2} to s_0).
  for (int i = num_windows - 2; i >= 0; i--) {
    // Step 3. Double the accumulator w times.
    for (size_t j = 0; j < SCALAR_MUL_WINDOW_SIZE; j++) {
      ctx->point_dbl(x_res, y_res, z_res, x_res, y_res, z_res);
    }

    // Step 4a. Compute abs(s_i).
    int16_t d = rwnaf[i];
    int16_t is_neg = (d >> 15) & 1; // is_neg = (d < 0) ? 1 : 0
    d = (d ^ -is_neg) + is_neg;     // d = abs(d)

    // Step 4b. Select from table the point corresponding to abs(s_i).
    idx = d >> 1;
    select_point_from_table(ctx, tmp, table, idx, 1);

    // Step 4c. Negate the point if s_i < 0.
    ec_nistp_felem ftmp;
    ctx->felem_neg(ftmp, y_tmp);

    cmovznz(y_tmp, ctx->felem_num_limbs, is_neg, y_tmp, ftmp);

    // Step 4d. Add the point to the accumulator.
    ctx->point_add(x_res, y_res, z_res, x_res, y_res, z_res, 0, x_tmp, y_tmp, z_tmp);
  }

  // Step 5a. Negate the input point P (we negate it in-place since we already
  // have it stored as the first entry in the table).
  ec_nistp_felem_limb *x_mp = &table[0];
  ec_nistp_felem_limb *y_mp = &table[ctx->felem_num_limbs];
  ec_nistp_felem_limb *z_mp = &table[ctx->felem_num_limbs * 2];
  ctx->felem_neg(y_mp, y_mp);

  // Step 5b. Subtract P from the accumulator.
  ctx->point_add(x_tmp, y_tmp, z_tmp, x_res, y_res, z_res, 0, x_mp, y_mp, z_mp);

  // Step 5c. Select |res| or |res - P| based on parity of the scalar.
  ec_nistp_felem_limb t = scalar->words[0] & 1;
  cmovznz(x_out, ctx->felem_num_limbs, t, x_tmp, x_res);
  cmovznz(y_out, ctx->felem_num_limbs, t, y_tmp, y_res);
  cmovznz(z_out, ctx->felem_num_limbs, t, z_tmp, z_res);
}

// Multiplication of the base point G of the curve with the given scalar.
// The product is computed with the Comb method using a precomputed table
// and the regular-wNAF scalar encoding.
//
// While the algorithm is generic and works for different curves, window sizes,
// and scalar sizes, for clarity, we describe it by using the example of P-521.
//
// The precomputed table has 27 sub-tables each holding 16 points:
//
//      0 :       [1]G,       [3]G,  ...,       [31]G
//      1 :  [1*2^20]G,  [3*2^20]G,  ...,  [31*2^20]G
//                         ...
//      i : [1*2^20i]G, [3*2^20i]G,  ..., [31*2^20i]G
//                         ...
//     26 :   [2^520]G, [3*2^520]G,  ..., [31*2^520]G
// Computing the negation of a point P = (x, y) is relatively easy:
//     -P = (x, -y).
// So we may assume that for each sub-table we have 32 points instead of 16:
//     [\pm 1*2^20i]G, [\pm 3*2^20i]G, ..., [\pm 31*2^20i]G.
//
// The 521-bit |scalar| is recoded (regular-wNAF encoding) into 105 signed
// digits, each of length 5 bits, as explained in the
// |p521_felem_mul_scalar_rwnaf| function. Namely,
//     scalar' = s_0 + s_1*2^5 + s_2*2^10 + ... + s_104*2^520,
// where digits s_i are in [\pm 1, \pm 3, ..., \pm 31]. Note that for an odd
// scalar we have that scalar = scalar', while in the case of an even
// scalar we have that scalar = scalar' - 1.
//
// To compute the required product, [scalar]G, we may do the following.
// Group the recoded digits of the scalar in 4 groups:
//                                            |   corresponding multiples in
//                  digits                    |   the recoded representation
//   -------------------------------------------------------------------------
//   (0): {s_0, s_4,  s_8, ..., s_100, s_104} |  { 2^0, 2^20, ..., 2^500, 2^520}
//   (1): {s_1, s_5,  s_9, ..., s_101}        |  { 2^5, 2^25, ..., 2^505}
//   (2): {s_2, s_6, s_10, ..., s_102}        |  {2^10, 2^30, ..., 2^510}
//   (3): {s_3, s_7, s_11, ..., s_103}        |  {2^15, 2^35, ..., 2^515}
//        corresponding sub-table lookup      |  {  T0,   T1, ...,   T25,   T26}
//
// The group (0) digits correspond precisely to the multiples of G that are
// held in the 27 precomputed sub-tables, so we may simply read the appropriate
// points from the sub-tables and sum them all up (negating if needed, i.e., if
// a digit s_i is negative, we read the point corresponding to the abs(s_i) and
// negate it before adding it to the sum).
// The remaining three groups (1), (2), and (3), correspond to the multiples
// of G from the sub-tables multiplied additionally by 2^5, 2^10, and 2^15,
// respectively. Therefore, for these groups we may read the appropriate points
// from the table, double them 5, 10, or 15 times, respectively, and add them
// to the final result.
//
// To minimize the number of required doubling operations we process the digits
// of the scalar from left to right. In other words, the algorithm is:
//   1. For group (i) in this order (3, 2, 1, 0):
//   2.   Double the accumulator 5 times except in the first iteration.
//   3.   Read the points corresponding to the group (i) digits from the tables
//        and add them to an accumulator.
//   4. If the scalar is even subtract G from the accumulator.
//
// Note: this function is designed to be constant-time.
void ec_nistp_scalar_mul_base(const ec_nistp_meth *ctx,
                              ec_nistp_felem_limb *x_out,
                              ec_nistp_felem_limb *y_out,
                              ec_nistp_felem_limb *z_out,
                              const EC_SCALAR *scalar) {
  // Regular-wNAF encoding of the scalar.
  int16_t rwnaf[SCALAR_MUL_MAX_NUM_WINDOWS];
  scalar_rwnaf(rwnaf, SCALAR_MUL_WINDOW_SIZE, scalar, ctx->felem_num_bits);
  size_t num_windows = DIV_AND_CEIL(ctx->felem_num_bits, SCALAR_MUL_WINDOW_SIZE);

  // We need two point accumulators, so we define them of maximum size
  // to avoid allocation, and just take pointers to individual coordinates.
  // (This cruft will disapear when we refactor point_add/dbl to work with
  // whole points instead of individual coordinates).
  ec_nistp_felem_limb res[3 * FELEM_MAX_NUM_OF_LIMBS] = {0};
  ec_nistp_felem_limb tmp[3 * FELEM_MAX_NUM_OF_LIMBS] = {0};
  ec_nistp_felem_limb *x_res = &res[0];
  ec_nistp_felem_limb *y_res = &res[ctx->felem_num_limbs];
  ec_nistp_felem_limb *z_res = &res[ctx->felem_num_limbs * 2];
  ec_nistp_felem_limb *x_tmp = &tmp[0];
  ec_nistp_felem_limb *y_tmp = &tmp[ctx->felem_num_limbs];
  ec_nistp_felem_limb *z_tmp = &tmp[ctx->felem_num_limbs * 2];

  // Process the 4 groups of digits starting from group (3) down to group (0).
  for (int i = 3; i >= 0; i--) {
    // Double |res| 5 times in each iteration, except in the first one.
    for (size_t j = 0; i != 3 && j < SCALAR_MUL_WINDOW_SIZE; j++) {
      ctx->point_dbl(x_res, y_res, z_res, x_res, y_res, z_res);
    }

    // Process the digits in the current group from the most to the least
    // significant one.
    size_t start_idx = ((num_windows - i - 1) / 4) * 4 + i;

    for (int j = start_idx; j >= 0; j -= 4) {
      // For each digit |d| in the current group read the corresponding point
      // from the table and add it to |res|. If |d| is negative, negate
      // the point before adding it to |res|.
      int16_t d = rwnaf[j];
      int16_t is_neg = (d >> 15) & 1; // is_neg = (d < 0) ? 1 : 0
      d = (d ^ -is_neg) + is_neg;     // d = abs(d)

      int16_t idx = d >> 1;

      // Select the point to add, in constant time.
      size_t point_num_limbs = 2 * ctx->felem_num_limbs;  // Affine points.
      size_t subtable_num_limbs = SCALAR_MUL_TABLE_NUM_POINTS * point_num_limbs;
      size_t table_idx = (j / 4) * subtable_num_limbs;
      const ec_nistp_felem_limb *table = &ctx->scalar_mul_base_table[table_idx];
      select_point_from_table(ctx, tmp, table, idx, 0);

      // Negate y coordinate of the point tmp = (x, y); ftmp = -y.
      ec_nistp_felem ftmp;
      ctx->felem_neg(ftmp, y_tmp);

      cmovznz(y_tmp, ctx->felem_num_limbs, is_neg, y_tmp, ftmp);

      // Add the point to the accumulator |res|.
      ctx->point_add(x_res, y_res, z_res, x_res, y_res, z_res, 1,
                     x_tmp, y_tmp, ctx->felem_one);
    }
  }

  // Conditionally subtract G if the scalar is even, in constant-time.
  const ec_nistp_felem_limb *x_mp = &ctx->scalar_mul_base_table[0];
  const ec_nistp_felem_limb *y_mp = &ctx->scalar_mul_base_table[ctx->felem_num_limbs];
  ec_nistp_felem ftmp;
  ctx->felem_neg(ftmp, y_mp);

  // Subtract P from the accumulator.
  ctx->point_add(x_tmp, y_tmp, z_tmp, x_res, y_res, z_res, 1, x_mp, ftmp, ctx->felem_one);

  // Select |res| or |res - P| based on parity of the scalar.
  ec_nistp_felem_limb t = scalar->words[0] & 1;
  cmovznz(x_out, ctx->felem_num_limbs, t, x_tmp, x_res);
  cmovznz(y_out, ctx->felem_num_limbs, t, y_tmp, y_res);
  cmovznz(z_out, ctx->felem_num_limbs, t, z_tmp, z_res);
}

// Computes [g_scalar]G + [p_scalar]P, where G is the base point of the curve
// curve, and P is the given point (x_p, y_p, z_p).
//
// Both scalar products are computed by the same "textbook" wNAF method,
// with w = 5 for g_scalar and w = 5 for p_scalar.
// For the base point G product we use the first sub-table of the precomputed
// table, while for P we generate the table on-the-fly. The tables hold the
// first 16 odd multiples of G or P:
//     g_pre_comp = {[1]G, [3]G, ..., [31]G},
//     p_pre_comp = {[1]P, [3]P, ..., [31]P}.
// Computing the negation of a point P = (x, y) is relatively easy:
//     -P = (x, -y).
// So we may assume that we also have the negatives of the points in the tables.
//
// The scalars are recoded by the textbook wNAF method to digits, where a digit
// is either a zero or an odd integer in [-31, 31]. The method guarantees that
// each non-zero digit is followed by at least four zeroes.
//
// The result [g_scalar]G + [p_scalar]P is computed by the following algorithm:
//     1. Initialize the accumulator with the point-at-infinity.
//     2. For i starting from 521 down to 0:
//     3.   Double the accumulator (doubling can be skipped while the
//          accumulator is equal to the point-at-infinity).
//     4.   Read from |p_pre_comp| the point corresponding to the i-th digit of
//          p_scalar, negate it if the digit is negative, and add it to the
//          accumulator.
//     5.   Read from |g_pre_comp| the point corresponding to the i-th digit of
//          g_scalar, negate it if the digit is negative, and add it to the
//          accumulator.
// Note: this function is NOT constant-time.
void ec_nistp_scalar_mul_public(const ec_nistp_meth *ctx,
                                ec_nistp_felem_limb *x_out,
                                ec_nistp_felem_limb *y_out,
                                ec_nistp_felem_limb *z_out,
                                const EC_SCALAR *g_scalar,
                                const ec_nistp_felem_limb *x_p,
                                const ec_nistp_felem_limb *y_p,
                                const ec_nistp_felem_limb *z_p,
                                const EC_SCALAR *p_scalar) {

  const size_t felem_num_bytes = ctx->felem_num_limbs * sizeof(ec_nistp_felem_limb);

  // Table of multiples of P.
  ec_nistp_felem_limb p_table[SCALAR_MUL_TABLE_MAX_NUM_FELEM_LIMBS];
  generate_table(ctx, p_table, x_p, y_p, z_p);
  const size_t p_point_num_limbs = 3 * ctx->felem_num_limbs; // Projective.

  // Table of multiples of G.
  const ec_nistp_felem_limb *g_table = ctx->scalar_mul_base_table;
  const size_t g_point_num_limbs = 2 * ctx->felem_num_limbs; // Affine.

  // Recode the scalars.
  int8_t p_wnaf[SCALAR_MUL_MAX_SCALAR_BITS + 1] = {0};
  int8_t g_wnaf[SCALAR_MUL_MAX_SCALAR_BITS + 1] = {0};
  ec_compute_wNAF(p_wnaf, p_scalar, ctx->felem_num_bits, SCALAR_MUL_WINDOW_SIZE);
  ec_compute_wNAF(g_wnaf, g_scalar, ctx->felem_num_bits, SCALAR_MUL_WINDOW_SIZE);

  // In the beginning res is set to point-at-infinity, so we set the flag.
  int16_t res_is_inf = 1;
  int16_t d, is_neg, idx;
  ec_nistp_felem ftmp;

  for (int i = ctx->felem_num_bits; i >= 0; i--) {

    // If |res| is point-at-infinity there is no point in doubling so skip it.
    if (!res_is_inf) {
      ctx->point_dbl(x_out, y_out, z_out, x_out, y_out, z_out);
    }

    // Process the p_scalar digit.
    d = p_wnaf[i];
    if (d != 0) {
      is_neg = d < 0 ? 1 : 0;
      idx = (is_neg) ? (-d - 1) >> 1 : (d - 1) >> 1;

      if (res_is_inf) {
        // If |res| is point-at-infinity there is no need to add the new point,
        // we can simply copy it.
        const size_t table_idx = idx * p_point_num_limbs;
        OPENSSL_memcpy(x_out, &p_table[table_idx], felem_num_bytes);
        OPENSSL_memcpy(y_out, &p_table[table_idx + ctx->felem_num_limbs], felem_num_bytes);
        OPENSSL_memcpy(z_out, &p_table[table_idx + ctx->felem_num_limbs * 2], felem_num_bytes);
        res_is_inf = 0;
      } else {
        // Otherwise, add to the accumulator either the point at position idx
        // in the table or its negation.
        const ec_nistp_felem_limb *y_tmp;
        y_tmp = &p_table[idx * p_point_num_limbs + ctx->felem_num_limbs];
        if (is_neg) {
          ctx->felem_neg(ftmp, y_tmp);
          y_tmp = ftmp;
        }
        ctx->point_add(x_out, y_out, z_out, x_out, y_out, z_out, 0,
                       &p_table[idx * p_point_num_limbs],
                       y_tmp,
                       &p_table[idx * p_point_num_limbs + ctx->felem_num_limbs * 2]);
      }
    }

    /* // Process the g_scalar digit. */
    d = g_wnaf[i];
    if (d != 0) {
      is_neg = d < 0 ? 1 : 0;
      idx = (is_neg) ? (-d - 1) >> 1 : (d - 1) >> 1;

      if (res_is_inf) {
        // If |res| is point-at-infinity there is no need to add the new point,
        // we can simply copy it.
        const size_t table_idx = idx * g_point_num_limbs;
        OPENSSL_memcpy(x_out, &g_table[table_idx], felem_num_bytes);
        OPENSSL_memcpy(y_out, &g_table[table_idx + ctx->felem_num_limbs], felem_num_bytes);
        OPENSSL_memcpy(z_out, ctx->felem_one, felem_num_bytes);
        res_is_inf = 0;
      } else {
        // Otherwise, add to the accumulator either the point at position idx
        // in the table or its negation.
        const ec_nistp_felem_limb *y_tmp ;
        y_tmp = &g_table[idx * g_point_num_limbs + ctx->felem_num_limbs];
        if (is_neg) {
          ctx->felem_neg(ftmp, &g_table[idx * g_point_num_limbs + ctx->felem_num_limbs]);
          y_tmp = ftmp;
        }
        ctx->point_add(x_out, y_out, z_out, x_out, y_out, z_out, 1,
                       &g_table[idx * g_point_num_limbs], y_tmp, ctx->felem_one);
      }
    }
  }
}

void ec_nistp_point_to_coordinates(ec_nistp_felem_limb *x_out,
                                   ec_nistp_felem_limb *y_out,
                                   ec_nistp_felem_limb *z_out,
                                   const ec_nistp_felem_limb *xyz_in,
                                   size_t num_limbs_per_coord) {
  size_t num_bytes_per_coord = num_limbs_per_coord * sizeof(ec_nistp_felem_limb);
  OPENSSL_memcpy(x_out, xyz_in, num_bytes_per_coord);
  OPENSSL_memcpy(y_out, &xyz_in[num_limbs_per_coord], num_bytes_per_coord);
  OPENSSL_memcpy(z_out, &xyz_in[num_limbs_per_coord * 2], num_bytes_per_coord);
}

void ec_nistp_coordinates_to_point(ec_nistp_felem_limb *xyz_out,
                                   const ec_nistp_felem_limb *x_in,
                                   const ec_nistp_felem_limb *y_in,
                                   const ec_nistp_felem_limb *z_in,
                                   size_t num_limbs_per_coord) {
  size_t num_bytes_per_coord = num_limbs_per_coord * sizeof(ec_nistp_felem_limb);
  OPENSSL_memcpy(xyz_out, x_in, num_bytes_per_coord);
  OPENSSL_memcpy(&xyz_out[num_limbs_per_coord], y_in, num_bytes_per_coord);
  OPENSSL_memcpy(&xyz_out[num_limbs_per_coord * 2], z_in, num_bytes_per_coord);
}
