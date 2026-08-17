/* gmpxx.h -- C++ class wrapper for GMP types.  -*- C++ -*-

Copyright 2001-2003, 2006, 2008, 2011-2015, 2018 Free Software
Foundation, Inc.

This file is part of the GNU MP Library.

The GNU MP Library is free software; you can redistribute it and/or modify
it under the terms of either:

  * the GNU Lesser General Public License as published by the Free
    Software Foundation; either version 3 of the License, or (at your
    option) any later version.

or

  * the GNU General Public License as published by the Free Software
    Foundation; either version 2 of the License, or (at your option) any
    later version.

or both in parallel, as here.

The GNU MP Library is distributed in the hope that it will be useful, but
WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
for more details.

You should have received copies of the GNU General Public License and the
GNU Lesser General Public License along with the GNU MP Library.  If not,
see https://www.gnu.org/licenses/.  */

#ifndef __GMP_PLUSPLUS__
#define __GMP_PLUSPLUS__

#include <iosfwd>

#include <cstring>  /* for strlen */
#include <limits>  /* numeric_limits */
#include <utility>
#include <algorithm>  /* swap */
#include <string>
#include <stdexcept>
#include <cfloat>
#include <gmp.h>

// wrapper for gcc's __builtin_constant_p
// __builtin_constant_p has been in gcc since forever,
// but g++-3.4 miscompiles it.
#if __GMP_GNUC_PREREQ(4, 2)
#define __GMPXX_CONSTANT(X) __builtin_constant_p(X)
#else
#define __GMPXX_CONSTANT(X) false
#endif
#define __GMPXX_CONSTANT_TRUE(X) (__GMPXX_CONSTANT(X) && (X))

// Use C++11 features
#ifndef __GMPXX_USE_CXX11
#if __cplusplus >= 201103L
#define __GMPXX_USE_CXX11 1
#else
#define __GMPXX_USE_CXX11 0
#endif
#endif

#if __GMPXX_USE_CXX11
#define __GMPXX_NOEXCEPT noexcept
#include <type_traits> // for common_type
#else
#define __GMPXX_NOEXCEPT
#endif

// Max allocations for plain types when converted to GMP types
#if GMP_NAIL_BITS != 0 && ! defined _LONG_LONG_LIMB
#define __GMPZ_ULI_LIMBS 2
#else
#define __GMPZ_ULI_LIMBS 1
#endif

#define __GMPXX_BITS_TO_LIMBS(n)  (((n) + (GMP_NUMB_BITS - 1)) / GMP_NUMB_BITS)
#define __GMPZ_DBL_LIMBS __GMPXX_BITS_TO_LIMBS(DBL_MAX_EXP)+1
#define __GMPQ_NUM_DBL_LIMBS __GMPZ_DBL_LIMBS
#define __GMPQ_DEN_DBL_LIMBS __GMPXX_BITS_TO_LIMBS(DBL_MANT_DIG+1-DBL_MIN_EXP)+1
// The final +1s are a security margin. The current implementation of
// mpq_set_d seems to need it for the denominator.

inline void __mpz_set_ui_safe(mpz_ptr p, unsigned long l)
{
  p->_mp_size = (l != 0);
  p->_mp_d[0] = l & GMP_NUMB_MASK;
#if __GMPZ_ULI_LIMBS > 1
  l >>= GMP_NUMB_BITS;
  p->_mp_d[1] = l;
  p->_mp_size += (l != 0);
#endif
}

inline void __mpz_set_si_safe(mpz_ptr p, long l)
{
  if(l < 0)
  {
    __mpz_set_ui_safe(p, -static_cast<unsigned long>(l));
    mpz_neg(p, p);
  }
  else
    __mpz_set_ui_safe(p, l);
    // Note: we know the high bit of l is 0 so we could do slightly better
}

// Fake temporary variables
#define __GMPXX_TMPZ_UI							\
  mpz_t temp;								\
  mp_limb_t limbs[__GMPZ_ULI_LIMBS];					\
  temp->_mp_d = limbs;							\
  __mpz_set_ui_safe (temp, l)
#define __GMPXX_TMPZ_SI							\
  mpz_t temp;								\
  mp_limb_t limbs[__GMPZ_ULI_LIMBS];					\
  temp->_mp_d = limbs;							\
  __mpz_set_si_safe (temp, l)
#define __GMPXX_TMPZ_D							\
  mpz_t temp;								\
  mp_limb_t limbs[__GMPZ_DBL_LIMBS];					\
  temp->_mp_d = limbs;							\
  temp->_mp_alloc = __GMPZ_DBL_LIMBS;					\
  mpz_set_d (temp, d)

#define __GMPXX_TMPQ_UI							\
  mpq_t temp;								\
  mp_limb_t limbs[__GMPZ_ULI_LIMBS+1];					\
  mpq_numref(temp)->_mp_d = limbs;					\
  __mpz_set_ui_safe (mpq_numref(temp), l);				\
  mpq_denref(temp)->_mp_d = limbs + __GMPZ_ULI_LIMBS;			\
  mpq_denref(temp)->_mp_size = 1;					\
  mpq_denref(temp)->_mp_d[0] = 1
#define __GMPXX_TMPQ_SI							\
  mpq_t temp;								\
  mp_limb_t limbs[__GMPZ_ULI_LIMBS+1];					\
  mpq_numref(temp)->_mp_d = limbs;					\
  __mpz_set_si_safe (mpq_numref(temp), l);				\
  mpq_denref(temp)->_mp_d = limbs + __GMPZ_ULI_LIMBS;			\
  mpq_denref(temp)->_mp_size = 1;					\
  mpq_denref(temp)->_mp_d[0] = 1
#define __GMPXX_TMPQ_D							\
  mpq_t temp;								\
  mp_limb_t limbs[__GMPQ_NUM_DBL_LIMBS + __GMPQ_DEN_DBL_LIMBS];		\
  mpq_numref(temp)->_mp_d = limbs;					\
  mpq_numref(temp)->_mp_alloc = __GMPQ_NUM_DBL_LIMBS;			\
  mpq_denref(temp)->_mp_d = limbs + __GMPQ_NUM_DBL_LIMBS;		\
  mpq_denref(temp)->_mp_alloc = __GMPQ_DEN_DBL_LIMBS;			\
  mpq_set_d (temp, d)

inline unsigned long __gmpxx_abs_ui (signed long l)
{
  return l >= 0 ? static_cast<unsigned long>(l)
	  : -static_cast<unsigned long>(l);
}

/**************** Function objects ****************/
/* Any evaluation of a __gmp_expr ends up calling one of these functions
   all intermediate functions being inline, the evaluation should optimize
   to a direct call to the relevant function, thus yielding no overhead
   over the C interface. */

struct __gmp_unary_plus
{
  static void eval(mpz_ptr z, mpz_srcptr w) { mpz_set(z, w); }
  static void eval(mpq_ptr q, mpq_srcptr r) { mpq_set(q, r); }
  static void eval(mpf_ptr f, mpf_srcptr g) { mpf_set(f, g); }
};

struct __gmp_unary_minus
{
  static void eval(mpz_ptr z, mpz_srcptr w) { mpz_neg(z, w); }
  static void eval(mpq_ptr q, mpq_srcptr r) { mpq_neg(q, r); }
  static void eval(mpf_ptr f, mpf_srcptr g) { mpf_neg(f, g); }
};

struct __gmp_unary_com
{
  static void eval(mpz_ptr z, mpz_srcptr w) { mpz_com(z, w); }
};

struct __gmp_binary_plus
{
  static void eval(mpz_ptr z, mpz_srcptr w, mpz_srcptr v)
  { mpz_add(z, w, v); }

  static void eval(mpz_ptr z, mpz_srcptr w, unsigned long int l)
  {
    // Ideally, those checks should happen earlier so that the tree
    // generated for a+0+b would just be sum(a,b).
    if (__GMPXX_CONSTANT(l) && l == 0)
    {
      if (z != w) mpz_set(z, w);
    }
    else
      mpz_add_ui(z, w, l);
  }
  static void eval(mpz_ptr z, unsigned long int l, mpz_srcptr w)
  { eval(z, w, l); }
  static void eval(mpz_ptr z, mpz_srcptr w, signed long int l)
  {
    if (l >= 0)
      eval(z, w, static_cast<unsigned long>(l));
    else
      mpz_sub_ui(z, w, -static_cast<unsigned long>(l));
  }
  static void eval(mpz_ptr z, signed long int l, mpz_srcptr w)
  { eval(z, w, l); }
  static void eval(mpz_ptr z, mpz_srcptr w, double d)
  {  __GMPXX_TMPZ_D;    mpz_add (z, w, temp); }
  static void eval(mpz_ptr z, double d, mpz_srcptr w)
  { eval(z, w, d); }

  static void eval(mpq_ptr q, mpq_srcptr r, mpq_srcptr s)
  { mpq_add(q, r, s); }

  static void eval(mpq_ptr q, mpq_srcptr r, unsigned long int l)
  {
    if (__GMPXX_CONSTANT(l) && l == 0)
    {
      if (q != r) mpq_set(q, r);
    }
    else if (__GMPXX_CONSTANT(l) && l == 1)
    {
      mpz_add (mpq_numref(q), mpq_numref(r), mpq_denref(r));
      if (q != r) mpz_set(mpq_denref(q), mpq_denref(r));
    }
    else
    {
      if (q == r)
        mpz_addmul_ui(mpq_numref(q), mpq_denref(q), l);
      else
      {
        mpz_mul_ui(mpq_numref(q), mpq_denref(r), l);
        mpz_add(mpq_numref(q), mpq_numref(q), mpq_numref(r));
        mpz_set(mpq_denref(q), mpq_denref(r));
      }
    }
  }
  static void eval(mpq_ptr q, unsigned long int l, mpq_srcptr r)
  { eval(q, r, l); }
  static inline void eval(mpq_ptr q, mpq_srcptr r, signed long int l);
  // defined after __gmp_binary_minus
  static void eval(mpq_ptr q, signed long int l, mpq_srcptr r)
  { eval(q, r, l); }
  static void eval(mpq_ptr q, mpq_srcptr r, double d)
  {  __GMPXX_TMPQ_D;    mpq_add (q, r, temp); }
  static void eval(mpq_ptr q, double d, mpq_srcptr r)
  { eval(q, r, d); }

  static void eval(mpq_ptr q, mpq_srcptr r, mpz_srcptr z)
  {
    if (q == r)
      mpz_addmul(mpq_numref(q), mpq_denref(q), z);
    else
    {
      mpz_mul(mpq_numref(q), mpq_denref(r), z);
      mpz_add(mpq_numref(q), mpq_numref(q), mpq_numref(r));
      mpz_set(mpq_denref(q), mpq_denref(r));
    }
  }
  static void eval(mpq_ptr q, mpz_srcptr z, mpq_srcptr r)
  { eval(q, r, z); }

  static void eval(mpf_ptr f, mpf_srcptr g, mpf_srcptr h)
  { mpf_add(f, g, h); }

  static void eval(mpf_ptr f, mpf_srcptr g, unsigned long int l)
  { mpf_add_ui(f, g, l); }
  static void eval(mpf_ptr f, unsigned long int l, mpf_srcptr g)
  { mpf_add_ui(f, g, l); }
  static void eval(mpf_ptr f, mpf_srcptr g, signed long int l)
  {
    if (l >= 0)
      mpf_add_ui(f, g, l);
    else
      mpf_sub_ui(f, g, -static_cast<unsigned long>(l));
  }
  static void eval(mpf_ptr f, signed long int l, mpf_srcptr g)
  { eval(f, g, l); }
  static void eval(mpf_ptr f, mpf_srcptr g, double d)
  {
    mpf_t temp;
    mpf_init2(temp, 8*sizeof(double));
    mpf_set_d(temp, d);
    mpf_add(f, g, temp);
    mpf_clear(temp);
  }
  static void eval(mpf_ptr f, double d, mpf_srcptr g)
  { eval(f, g, d); }
};

struct __gmp_binary_minus
{
  static void eval(mpz_ptr z, mpz_srcptr w, mpz_srcptr v)
  { mpz_sub(z, w, v); }

  static void eval(mpz_ptr z, mpz_srcptr w, unsigned long int l)
  {
    if (__GMPXX_CONSTANT(l) && l == 0)
    {
      if (z != w) mpz_set(z, w);
    }
    else
      mpz_sub_ui(z, w, l);
  }
  static void eval(mpz_ptr z, unsigned long int l, mpz_srcptr w)
  {
    if (__GMPXX_CONSTANT(l) && l == 0)
    {
      mpz_neg(z, w);
    }
    else
      mpz_ui_sub(z, l, w);
  }
  static void eval(mpz_ptr z, mpz_srcptr w, signed long int l)
  {
    if (l >= 0)
      eval(z, w, static_cast<unsigned long>(l));
    else
      mpz_add_ui(z, w, -static_cast<unsigned long>(l));
  }
  static void eval(mpz_ptr z, signed long int l, mpz_srcptr w)
  {
    if (l >= 0)
      eval(z, static_cast<unsigned long>(l), w);
    else
      {
        mpz_add_ui(z, w, -static_cast<unsigned long>(l));
        mpz_neg(z, z);
      }
  }
  static void eval(mpz_ptr z, mpz_srcptr w, double d)
  {  __GMPXX_TMPZ_D;    mpz_sub (z, w, temp); }
  static void eval(mpz_ptr z, double d, mpz_srcptr w)
  {  __GMPXX_TMPZ_D;    mpz_sub (z, temp, w); }

  static void eval(mpq_ptr q, mpq_srcptr r, mpq_srcptr s)
  { mpq_sub(q, r, s); }

  static void eval(mpq_ptr q, mpq_srcptr r, unsigned long int l)
  {
    if (__GMPXX_CONSTANT(l) && l == 0)
    {
      if (q != r) mpq_set(q, r);
    }
    else if (__GMPXX_CONSTANT(l) && l == 1)
    {
      mpz_sub (mpq_numref(q), mpq_numref(r), mpq_denref(r));
      if (q != r) mpz_set(mpq_denref(q), mpq_denref(r));
    }
    else
    {
      if (q == r)
        mpz_submul_ui(mpq_numref(q), mpq_denref(q), l);
      else
      {
        mpz_mul_ui(mpq_numref(q), mpq_denref(r), l);
        mpz_sub(mpq_numref(q), mpq_numref(r), mpq_numref(q));
        mpz_set(mpq_denref(q), mpq_denref(r));
      }
    }
  }
  static void eval(mpq_ptr q, unsigned long int l, mpq_srcptr r)
  { eval(q, r, l); mpq_neg(q, q); }
  static void eval(mpq_ptr q, mpq_srcptr r, signed long int l)
  {
    if (l >= 0)
      eval(q, r, static_cast<unsigned long>(l));
    else
      __gmp_binary_plus::eval(q, r, -static_cast<unsigned long>(l));
  }
  static void eval(mpq_ptr q, signed long int l, mpq_srcptr r)
  { eval(q, r, l); mpq_neg(q, q); }
  static void eval(mpq_ptr q, mpq_srcptr r, double d)
  {  __GMPXX_TMPQ_D;    mpq_sub (q, r, temp); }
  static void eval(mpq_ptr q, double d, mpq_srcptr r)
  {  __GMPXX_TMPQ_D;    mpq_sub (q, temp, r); }

  static void eval(mpq_ptr q, mpq_srcptr r, mpz_srcptr z)
  {
    if (q == r)
      mpz_submul(mpq_numref(q), mpq_denref(q), z);
    else
    {
      mpz_mul(mpq_numref(q), mpq_denref(r), z);
      mpz_sub(mpq_numref(q), mpq_numref(r), mpq_numref(q));
      mpz_set(mpq_denref(q), mpq_denref(r));
    }
  }
  static void eval(mpq_ptr q, mpz_srcptr z, mpq_srcptr r)
  { eval(q, r, z); mpq_neg(q, q); }

  static void eval(mpf_ptr f, mpf_srcptr g, mpf_srcptr h)
  { mpf_sub(f, g, h); }

  static void eval(mpf_ptr f, mpf_srcptr g, unsigned long int l)
  { mpf_sub_ui(f, g, l); }
  static void eval(mpf_ptr f, unsigned long int l, mpf_srcptr g)
  { mpf_ui_sub(f, l, g); }
  static void eval(mpf_ptr f, mpf_srcptr g, signed long int l)
  {
    if (l >= 0)
      mpf_sub_ui(f, g, l);
    else
      mpf_add_ui(f, g, -static_cast<unsigned long>(l));
  }
  static void eval(mpf_ptr f, signed long int l, mpf_srcptr g)
  {
    if (l >= 0)
      mpf_sub_ui(f, g, l);
    else
      mpf_add_ui(f, g, -static_cast<unsigned long>(l));
    mpf_neg(f, f);
  }
  static void eval(mpf_ptr f, mpf_srcptr g, double d)
  {
    mpf_t temp;
    mpf_init2(temp, 8*sizeof(double));
    mpf_set_d(temp, d);
    mpf_sub(f, g, temp);
    mpf_clear(temp);
  }
  static void eval(mpf_ptr f, double d, mpf_srcptr g)
  {
    mpf_t temp;
    mpf_init2(temp, 8*sizeof(double));
    mpf_set_d(temp, d);
    mpf_sub(f, temp, g);
    mpf_clear(temp);
  }
};

// defined here so it can reference __gmp_binary_minus
inline void
__gmp_binary_plus::eval(mpq_ptr q, mpq_srcptr r, signed long int l)
{
  if (l >= 0)
    eval(q, r, static_cast<unsigned long>(l));
  else
    __gmp_binary_minus::eval(q, r, -static_cast<unsigned long>(l));
}

struct __gmp_binary_lshift
{
  static void eval(mpz_ptr z, mpz_srcptr w, mp_bitcnt_t l)
  {
    if (__GMPXX_CONSTANT(l) && (l == 0))
    {
      if (z != w) mpz_set(z, w);
    }
    else
      mpz_mul_2exp(z, w, l);
  }
  static void eval(mpq_ptr q, mpq_srcptr r, mp_bitcnt_t l)
  {
    if (__GMPXX_CONSTANT(l) && (l == 0))
    {
      if (q != r) mpq_set(q, r);
    }
    else
      mpq_mul_2exp(q, r, l);
  }
  static void eval(mpf_ptr f, mpf_srcptr g, mp_bitcnt_t l)
  { mpf_mul_2exp(f, g, l); }
};

struct __gmp_binary_rshift
{
  static void eval(mpz_ptr z, mpz_srcptr w, mp_bitcnt_t l)
  {
    if (__GMPXX_CONSTANT(l) && (l == 0))
    {
      if (z != w) mpz_set(z, w);
    }
    else
      mpz_fdiv_q_2exp(z, w, l);
  }
  static void eval(mpq_ptr q, mpq_srcptr r, mp_bitcnt_t l)
  {
    if (__GMPXX_CONSTANT(l) && (l == 0))
    {
      if (q != r) mpq_set(q, r);
    }
    else
      mpq_div_2exp(q, r, l);
  }
  static void eval(mpf_ptr f, mpf_srcptr g, mp_bitcnt_t l)
  { mpf_div_2exp(f, g, l); }
};

struct __gmp_binary_multiplies
{
  static void eval(mpz_ptr z, mpz_srcptr w, mpz_srcptr v)
  { mpz_mul(z, w, v); }

  static void eval(mpz_ptr z, mpz_srcptr w, unsigned long int l)
  {
// gcc-3.3 doesn't have __builtin_ctzl. Don't bother optimizing for old gcc.
#if __GMP_GNUC_PREREQ(3, 4)
    if (__GMPXX_CONSTANT(l) && (l & (l-1)) == 0)
    {
      if (l == 0)
      {
        z->_mp_size = 0;
      }
      else
      {
        __gmp_binary_lshift::eval(z, w, __builtin_ctzl(l));
      }
    }
    else
#endif
      mpz_mul_ui(z, w, l);
  }
  static void eval(mpz_ptr z, unsigned long int l, mpz_srcptr w)
  { eval(z, w, l); }
  static void eval(mpz_ptr z, mpz_srcptr w, signed long int l)
  {
    if (__GMPXX_CONSTANT_TRUE(l >= 0))
      eval(z, w, static_cast<unsigned long>(l));
    else if (__GMPXX_CONSTANT_TRUE(l <= 0))
      {
        eval(z, w, -static_cast<unsigned long>(l));
	mpz_neg(z, z);
      }
    else
      mpz_mul_si (z, w, l);
  }
  static void eval(mpz_ptr z, signed long int l, mpz_srcptr w)
  { eval(z, w, l); }
  static void eval(mpz_ptr z, mpz_srcptr w, double d)
  {  __GMPXX_TMPZ_D;    mpz_mul (z, w, temp); }
  static void eval(mpz_ptr z, double d, mpz_srcptr w)
  { eval(z, w, d); }

  static void eval(mpq_ptr q, mpq_srcptr r, mpq_srcptr s)
  { mpq_mul(q, r, s); }

  static void eval(mpq_ptr q, mpq_srcptr r, unsigned long int l)
  {
#if __GMP_GNUC_PREREQ(3, 4)
    if (__GMPXX_CONSTANT(l) && (l & (l-1)) == 0)
    {
      if (l == 0)
      {
	mpq_set_ui(q, 0, 1);
      }
      else
      {
        __gmp_binary_lshift::eval(q, r, __builtin_ctzl(l));
      }
    }
    else
#endif
    {
      __GMPXX_TMPQ_UI;
      mpq_mul (q, r, temp);
    }
  }
  static void eval(mpq_ptr q, unsigned long int l, mpq_srcptr r)
  { eval(q, r, l); }
  static void eval(mpq_ptr q, mpq_srcptr r, signed long int l)
  {
    if (__GMPXX_CONSTANT_TRUE(l >= 0))
      eval(q, r, static_cast<unsigned long>(l));
    else if (__GMPXX_CONSTANT_TRUE(l <= 0))
      {
        eval(q, r, -static_cast<unsigned long>(l));
	mpq_neg(q, q);
      }
    else
      {
	__GMPXX_TMPQ_SI;
	mpq_mul (q, r, temp);
      }
  }
  static void eval(mpq_ptr q, signed long int l, mpq_srcptr r)
  { eval(q, r, l); }
  static void eval(mpq_ptr q, mpq_srcptr r, double d)
  {  __GMPXX_TMPQ_D;    mpq_mul (q, r, temp); }
  static void eval(mpq_ptr q, double d, mpq_srcptr r)
  { eval(q, r, d); }

  static void eval(mpf_ptr f, mpf_srcptr g, mpf_srcptr h)
  { mpf_mul(f, g, h); }

  static void eval(mpf_ptr f, mpf_srcptr g, unsigned long int l)
  { mpf_mul_ui(f, g, l); }
  static void eval(mpf_ptr f, unsigned long int l, mpf_srcptr g)
  { mpf_mul_ui(f, g, l); }
  static void eval(mpf_ptr f, mpf_srcptr g, signed long int l)
  {
    if (l >= 0)
      mpf_mul_ui(f, g, l);
    else
      {
	mpf_mul_ui(f, g, -static_cast<unsigned long>(l));
	mpf_neg(f, f);
      }
  }
  static void eval(mpf_ptr f, signed long int l, mpf_srcptr g)
  { eval(f, g, l); }
  static void eval(mpf_ptr f, mpf_srcptr g, double d)
  {
    mpf_t temp;
    mpf_init2(temp, 8*sizeof(double));
    mpf_set_d(temp, d);
    mpf_mul(f, g, temp);
    mpf_clear(temp);
  }
  static void eval(mpf_ptr f, double d, mpf_srcptr g)
  { eval(f, g, d); }
};

struct __gmp_binary_divides
{
  static void eval(mpz_ptr z, mpz_srcptr w, mpz_srcptr v)
  { mpz_tdiv_q(z, w, v); }

  static void eval(mpz_ptr z, mpz_srcptr w, unsigned long int l)
  {
#if __GMP_GNUC_PREREQ(3, 4)
    // Don't optimize division by 0...
    if (__GMPXX_CONSTANT(l) && (l & (l-1)) == 0 && l != 0)
    {
      if (l == 1)
      {
        if (z != w) mpz_set(z, w);
      }
      else
        mpz_tdiv_q_2exp(z, w, __builtin_ctzl(l));
        // warning: do not use rshift (fdiv)
    }
    else
#endif
      mpz_tdiv_q_ui(z, w, l);
  }
  static void eval(mpz_ptr z, unsigned long int l, mpz_srcptr w)
  {
    if (mpz_sgn(w) >= 0)
      {
	if (mpz_fits_ulong_p(w))
	  mpz_set_ui(z, l / mpz_get_ui(w));
	else
	  mpz_set_ui(z, 0);
      }
    else
      {
	mpz_neg(z, w);
	if (mpz_fits_ulong_p(z))
	  {
	    mpz_set_ui(z, l / mpz_get_ui(z));
	    mpz_neg(z, z);
	  }
	else
	  mpz_set_ui(z, 0);
      }
  }
  static void eval(mpz_ptr z, mpz_srcptr w, signed long int l)
  {
    if (l >= 0)
      eval(z, w, static_cast<unsigned long>(l));
    else
      {
	eval(z, w, -static_cast<unsigned long>(l));
	mpz_neg(z, z);
      }
  }
  static void eval(mpz_ptr z, signed long int l, mpz_srcptr w)
  {
    if (mpz_fits_slong_p(w))
      mpz_set_si(z, l / mpz_get_si(w));
    else
      {
        /* if w is bigger than a long then the quotient must be zero, unless
           l==LONG_MIN and w==-LONG_MIN in which case the quotient is -1 */
        mpz_set_si (z, (mpz_cmpabs_ui (w, __gmpxx_abs_ui(l)) == 0 ? -1 : 0));
      }
  }
  static void eval(mpz_ptr z, mpz_srcptr w, double d)
  {  __GMPXX_TMPZ_D;    mpz_tdiv_q (z, w, temp); }
  static void eval(mpz_ptr z, double d, mpz_srcptr w)
  {  __GMPXX_TMPZ_D;    mpz_tdiv_q (z, temp, w); }

  static void eval(mpq_ptr q, mpq_srcptr r, mpq_srcptr s)
  { mpq_div(q, r, s); }

  static void eval(mpq_ptr q, mpq_srcptr r, unsigned long int l)
  {
#if __GMP_GNUC_PREREQ(3, 4)
    if (__GMPXX_CONSTANT(l) && (l & (l-1)) == 0 && l != 0)
      __gmp_binary_rshift::eval(q, r, __builtin_ctzl(l));
    else
#endif
    {
      __GMPXX_TMPQ_UI;
      mpq_div (q, r, temp);
    }
  }
  static void eval(mpq_ptr q, unsigned long int l, mpq_srcptr r)
  {
    if (__GMPXX_CONSTANT_TRUE(l == 0))
      mpq_set_ui(q, 0, 1);
    else if (__GMPXX_CONSTANT_TRUE(l == 1))
      mpq_inv(q, r);
    else
      {
	__GMPXX_TMPQ_UI;
	mpq_div (q, temp, r);
      }
  }
  static void eval(mpq_ptr q, mpq_srcptr r, signed long int l)
  {
    if (__GMPXX_CONSTANT_TRUE(l >= 0))
      eval(q, r, static_cast<unsigned long>(l));
    else if (__GMPXX_CONSTANT_TRUE(l <= 0))
      {
        eval(q, r, -static_cast<unsigned long>(l));
	mpq_neg(q, q);
      }
    else
      {
	__GMPXX_TMPQ_SI;
	mpq_div (q, r, temp);
      }
  }
  static void eval(mpq_ptr q, signed long int l, mpq_srcptr r)
  {
    if (__GMPXX_CONSTANT_TRUE(l == 0))
      mpq_set_ui(q, 0, 1);
    else if (__GMPXX_CONSTANT_TRUE(l == 1))
      mpq_inv(q, r);
    else if (__GMPXX_CONSTANT_TRUE(l == -1))
      {
	mpq_inv(q, r);
	mpq_neg(q, q);
      }
    else
      {
	__GMPXX_TMPQ_SI;
	mpq_div (q, temp, r);
      }
  }
  static void eval(mpq_ptr q, mpq_srcptr r, double d)
  {  __GMPXX_TMPQ_D;    mpq_div (q, r, temp); }
  static void eval(mpq_ptr q, double d, mpq_srcptr r)
  {  __GMPXX_TMPQ_D;    mpq_div (q, temp, r); }

  static void eval(mpf_ptr f, mpf_srcptr g, mpf_srcptr h)
  { mpf_div(f, g, h); }

  static void eval(mpf_ptr f, mpf_srcptr g, unsigned long int l)
  { mpf_div_ui(f, g, l); }
  static void eval(mpf_ptr f, unsigned long int l, mpf_srcptr g)
  { mpf_ui_div(f, l, g); }
  static void eval(mpf_ptr f, mpf_srcptr g, signed long int l)
  {
    if (l >= 0)
      mpf_div_ui(f, g, l);
    else
      {
	mpf_div_ui(f, g, -static_cast<unsigned long>(l));
	mpf_neg(f, f);
      }
  }
  static void eval(mpf_ptr f, signed long int l, mpf_srcptr g)
  {
    if (l >= 0)
      mpf_ui_div(f, l, g);
    else
      {
	mpf_ui_div(f, -static_cast<unsigned long>(l), g);
	mpf_neg(f, f);
      }
  }
  static void eval(mpf_ptr f, mpf_srcptr g, double d)
  {
    mpf_t temp;
    mpf_init2(temp, 8*sizeof(double));
    mpf_set_d(temp, d);
    mpf_div(f, g, temp);
    mpf_clear(temp);
  }
  static void eval(mpf_ptr f, double d, mpf_srcptr g)
  {
    mpf_t temp;
    mpf_init2(temp, 8*sizeof(double));
    mpf_set_d(temp, d);
    mpf_div(f, temp, g);
    mpf_clear(temp);
  }
};

struct __gmp_binary_modulus
{
  static void eval(mpz_ptr z, mpz_srcptr w, mpz_srcptr v)
  { mpz_tdiv_r(z, w, v); }

  static void eval(mpz_ptr z, mpz_srcptr w, unsigned long int l)
  { mpz_tdiv_r_ui(z, w, l); }
  static void eval(mpz_ptr z, unsigned long int l, mpz_srcptr w)
  {
    if (mpz_sgn(w) >= 0)
      {
	if (mpz_fits_ulong_p(w))
	  mpz_set_ui(z, l % mpz_get_ui(w));
	else
	  mpz_set_ui(z, l);
      }
    else
      {
	mpz_neg(z, w);
	if (mpz_fits_ulong_p(z))
	  mpz_set_ui(z, l % mpz_get_ui(z));
	else
	  mpz_set_ui(z, l);
      }
  }
  static void eval(mpz_ptr z, mpz_srcptr w, signed long int l)
  {
    mpz_tdiv_r_ui (z, w, __gmpxx_abs_ui(l));
  }
  static void eval(mpz_ptr z, signed long int l, mpz_srcptr w)
  {
    if (mpz_fits_slong_p(w))
      mpz_set_si(z, l % mpz_get_si(w));
    else
      {
        /* if w is bigger than a long then the remainder is l unchanged,
           unless l==LONG_MIN and w==-LONG_MIN in which case it's 0 */
        mpz_set_si (z, mpz_cmpabs_ui (w, __gmpxx_abs_ui(l)) == 0 ? 0 : l);
      }
  }
  static void eval(mpz_ptr z, mpz_srcptr w, double d)
  {  __GMPXX_TMPZ_D;    mpz_tdiv_r (z, w, temp); }
  static void eval(mpz_ptr z, double d, mpz_srcptr w)
  {  __GMPXX_TMPZ_D;    mpz_tdiv_r (z, temp, w); }
};

struct __gmp_binary_and
{
  static void eval(mpz_ptr z, mpz_srcptr w, mpz_srcptr v)
  { mpz_and(z, w, v); }

  static void eval(mpz_ptr z, mpz_srcptr w, unsigned long int l)
  {  __GMPXX_TMPZ_UI;   mpz_and (z, w, temp);  }
  static void eval(mpz_ptr z, unsigned long int l, mpz_srcptr w)
  { eval(z, w, l);  }
  static void eval(mpz_ptr z, mpz_srcptr w, signed long int l)
  {  __GMPXX_TMPZ_SI;   mpz_and (z, w, temp);  }
  static void eval(mpz_ptr z, signed long int l, mpz_srcptr w)
  { eval(z, w, l);  }
  static void eval(mpz_ptr z, mpz_srcptr w, double d)
  {  __GMPXX_TMPZ_D;    mpz_and (z, w, temp); }
  static void eval(mpz_ptr z, double d, mpz_srcptr w)
  { eval(z, w, d);  }
};

struct __gmp_binary_ior
{
  static void eval(mpz_ptr z, mpz_srcptr w, mpz_srcptr v)
  { mpz_ior(z, w, v); }
  static void eval(mpz_ptr z, mpz_srcptr w, unsigned long int l)
  {  __GMPXX_TMPZ_UI;   mpz_ior (z, w, temp);  }
  static void eval(mpz_ptr z, unsigned long int l, mpz_srcptr w)
  { eval(z, w, l);  }
  static void eval(mpz_ptr z, mpz_srcptr w, signed long int l)
  {  __GMPXX_TMPZ_SI;   mpz_ior (z, w, temp);  }
  static void eval(mpz_ptr z, signed long int l, mpz_srcptr w)
  { eval(z, w, l);  }
  static void eval(mpz_ptr z, mpz_srcptr w, double d)
  {  __GMPXX_TMPZ_D;    mpz_ior (z, w, temp); }
  static void eval(mpz_ptr z, double d, mpz_srcptr w)
  { eval(z, w, d);  }
};

struct __gmp_binary_xor
{
  static void eval(mpz_ptr z, mpz_srcptr w, mpz_srcptr v)
  { mpz_xor(z, w, v); }
  static void eval(mpz_ptr z, mpz_srcptr w, unsigned long int l)
  {  __GMPXX_TMPZ_UI;   mpz_xor (z, w, temp);  }
  static void eval(mpz_ptr z, unsigned long int l, mpz_srcptr w)
  { eval(z, w, l);  }
  static void eval(mpz_ptr z, mpz_srcptr w, signed long int l)
  {  __GMPXX_TMPZ_SI;   mpz_xor (z, w, temp);  }
  static void eval(mpz_ptr z, signed long int l, mpz_srcptr w)
  { eval(z, w, l);  }
  static void eval(mpz_ptr z, mpz_srcptr w, double d)
  {  __GMPXX_TMPZ_D;    mpz_xor (z, w, temp); }
  static void eval(mpz_ptr z, double d, mpz_srcptr w)
  { eval(z, w, d);  }
};

struct __gmp_cmp_function
{
  static int eval(mpz_srcptr z, mpz_srcptr w) { return mpz_cmp(z, w); }

  static int eval(mpz_srcptr z, unsigned long int l)
  { return mpz_cmp_ui(z, l); }
  static int eval(unsigned long int l, mpz_srcptr z)
  { return -mpz_cmp_ui(z, l); }
  static int eval(mpz_srcptr z, signed long int l)
  { return mpz_cmp_si(z, l); }
  static int eval(signed long int l, mpz_srcptr z)
  { return -mpz_cmp_si(z, l); }
  static int eval(mpz_srcptr z, double d)
  { return mpz_cmp_d(z, d); }
  static int eval(double d, mpz_srcptr z)
  { return -mpz_cmp_d(z, d); }

  static int eval(mpq_srcptr q, mpq_srcptr r) { return mpq_cmp(q, r); }

  static int eval(mpq_srcptr q, unsigned long int l)
  { return mpq_cmp_ui(q, l, 1); }
  static int eval(unsigned long int l, mpq_srcptr q)
  { return -mpq_cmp_ui(q, l, 1); }
  static int eval(mpq_srcptr q, signed long int l)
  { return mpq_cmp_si(q, l, 1); }
  static int eval(signed long int l, mpq_srcptr q)
  { return -mpq_cmp_si(q, l, 1); }
  static int eval(mpq_srcptr q, double d)
  {  __GMPXX_TMPQ_D;    return mpq_cmp (q, temp); }
  static int eval(double d, mpq_srcptr q)
  {  __GMPXX_TMPQ_D;    return mpq_cmp (temp, q); }
  static int eval(mpq_srcptr q, mpz_srcptr z)
  { return mpq_cmp_z(q, z); }
  static int eval(mpz_srcptr z, mpq_srcptr q)
  { return -mpq_cmp_z(q, z); }

  static int eval(mpf_srcptr f, mpf_srcptr g) { return mpf_cmp(f, g); }

  static int eval(mpf_srcptr f, unsigned long int l)
  { return mpf_cmp_ui(f, l); }
  static int eval(unsigned long int l, mpf_srcptr f)
  { return -mpf_cmp_ui(f, l); }
  static int eval(mpf_srcptr f, signed long int l)
  { return mpf_cmp_si(f, l); }
  static int eval(signed long int l, mpf_srcptr f)
  { return -mpf_cmp_si(f, l); }
  static int eval(mpf_srcptr f, double d)
  { return mpf_cmp_d(f, d); }
  static int eval(double d, mpf_srcptr f)
  { return -mpf_cmp_d(f, d); }
  static int eval(mpf_srcptr f, mpz_srcptr z)
  { return mpf_cmp_z(f, z); }
  static int eval(mpz_srcptr z, mpf_srcptr f)
  { return -mpf_cmp_z(f, z); }
  static int eval(mpf_srcptr f, mpq_srcptr q)
  {
    mpf_t qf;
    mpf_init(qf); /* Should we use the precision of f?  */
    mpf_set_q(qf, q);
    int ret = eval(f, qf);
    mpf_clear(qf);
    return ret;
  }
  static int eval(mpq_srcptr q, mpf_srcptr f)
  { return -eval(f, q); }
};

struct __gmp_binary_equal
{
  static bool eval(mpz_srcptr z, mpz_srcptr w) { return mpz_cmp(z, w) == 0; }

  static bool eval(mpz_srcptr z, unsigned long int l)
  { return mpz_cmp_ui(z, l) == 0; }
  static bool eval(unsigned long int l, mpz_srcptr z)
  { return eval(z, l); }
  static bool eval(mpz_srcptr z, signed long int l)
  { return mpz_cmp_si(z, l) == 0; }
  static bool eval(signed long int l, mpz_srcptr z)
  { return eval(z, l); }
  static bool eval(mpz_srcptr z, double d)
  { return mpz_cmp_d(z, d) == 0; }
  static bool eval(double d, mpz_srcptr z)
  { return eval(z, d); }

  static bool eval(mpq_srcptr q, mpq_srcptr r)
  { return mpq_equal(q, r) != 0; }

  static bool eval(mpq_srcptr q, unsigned long int l)
  { return ((__GMPXX_CONSTANT(l) && l == 0) ||
	    mpz_cmp_ui(mpq_denref(q), 1) == 0) &&
      mpz_cmp_ui(mpq_numref(q), l) == 0; }
  static bool eval(unsigned long int l, mpq_srcptr q)
  { return eval(q, l); }
  static bool eval(mpq_srcptr q, signed long int l)
  { return ((__GMPXX_CONSTANT(l) && l == 0) ||
	    mpz_cmp_ui(mpq_denref(q), 1) == 0) &&
      mpz_cmp_si(mpq_numref(q), l) == 0; }
  static bool eval(signed long int l, mpq_srcptr q)
  { return eval(q, l); }
  static bool eval(mpq_srcptr q, double d)
  {  __GMPXX_TMPQ_D;    return mpq_equal (q, temp) != 0; }
  static bool eval(double d, mpq_srcptr q)
  { return eval(q, d); }
  static bool eval(mpq_srcptr q, mpz_srcptr z)
  { return mpz_cmp_ui(mpq_denref(q), 1) == 0 && mpz_cmp(mpq_numref(q), z) == 0; }
  static bool eval(mpz_srcptr z, mpq_srcptr q)
  { return eval(q, z); }

  static bool eval(mpf_srcptr f, mpf_srcptr g) { return mpf_cmp(f, g) == 0; }

  static bool eval(mpf_srcptr f, unsigned long int l)
  { return mpf_cmp_ui(f, l) == 0; }
  static bool eval(unsigned long int l, mpf_srcptr f)
  { return eval(f, l); }
  static bool eval(mpf_srcptr f, signed long int l)
  { return mpf_cmp_si(f, l) == 0; }
  static bool eval(signed long int l, mpf_srcptr f)
  { return eval(f, l); }
  static bool eval(mpf_srcptr f, double d)
  { return mpf_cmp_d(f, d) == 0; }
  static bool eval(double d, mpf_srcptr f)
  { return eval(f, d); }
  static bool eval(mpf_srcptr f, mpz_srcptr z)
  { return mpf_cmp_z(f, z) == 0; }
  static bool eval(mpz_srcptr z, mpf_srcptr f)
  { return eval(f, z); }
  static bool eval(mpf_srcptr f, mpq_srcptr q)
  { return __gmp_cmp_function::eval(f, q) == 0; }
  static bool eval(mpq_srcptr q, mpf_srcptr f)
  { return eval(f, q); }
};

struct __gmp_binary_less
{
  static bool eval(mpz_srcptr z, mpz_srcptr w) { return mpz_cmp(z, w) < 0; }

  static bool eval(mpz_srcptr z, unsigned long int l)
  { return mpz_cmp_ui(z, l) < 0; }
  static bool eval(unsigned long int l, mpz_srcptr z)
  { return mpz_cmp_ui(z, l) > 0; }
  static bool eval(mpz_srcptr z, signed long int l)
  { return mpz_cmp_si(z, l) < 0; }
  static bool eval(signed long int l, mpz_srcptr z)
  { return mpz_cmp_si(z, l) > 0; }
  static bool eval(mpz_srcptr z, double d)
  { return mpz_cmp_d(z, d) < 0; }
  static bool eval(double d, mpz_srcptr z)
  { return mpz_cmp_d(z, d) > 0; }

  static bool eval(mpq_srcptr q, mpq_srcptr r) { return mpq_cmp(q, r) < 0; }

  static bool eval(mpq_srcptr q, unsigned long int l)
  { return mpq_cmp_ui(q, l, 1) < 0; }
  static bool eval(unsigned long int l, mpq_srcptr q)
  { return mpq_cmp_ui(q, l, 1) > 0; }
  static bool eval(mpq_srcptr q, signed long int l)
  { return mpq_cmp_si(q, l, 1) < 0; }
  static bool eval(signed long int l, mpq_srcptr q)
  { return mpq_cmp_si(q, l, 1) > 0; }
  static bool eval(mpq_srcptr q, double d)
  {  __GMPXX_TMPQ_D;    return mpq_cmp (q, temp) < 0; }
  static bool eval(double d, mpq_srcptr q)
  {  __GMPXX_TMPQ_D;    return mpq_cmp (temp, q) < 0; }
  static bool eval(mpq_srcptr q, mpz_srcptr z)
  { return mpq_cmp_z(q, z) < 0; }
  static bool eval(mpz_srcptr z, mpq_srcptr q)
  { return mpq_cmp_z(q, z) > 0; }

  static bool eval(mpf_srcptr f, mpf_srcptr g) { return mpf_cmp(f, g) < 0; }

  static bool eval(mpf_srcptr f, unsigned long int l)
  { return mpf_cmp_ui(f, l) < 0; }
  static bool eval(unsigned long int l, mpf_srcptr f)
  { return mpf_cmp_ui(f, l) > 0; }
  static bool eval(mpf_srcptr f, signed long int l)
  { return mpf_cmp_si(f, l) < 0; }
  static bool eval(signed long int l, mpf_srcptr f)
  { return mpf_cmp_si(f, l) > 0; }
  static bool eval(mpf_srcptr f, double d)
  { return mpf_cmp_d(f, d) < 0; }
  static bool eval(double d, mpf_srcptr f)
  { return mpf_cmp_d(f, d) > 0; }
  static bool eval(mpf_srcptr f, mpz_srcptr z)
  { return mpf_cmp_z(f, z) < 0; }
  static bool eval(mpz_srcptr z, mpf_srcptr f)
  { return mpf_cmp_z(f, z) > 0; }
  static bool eval(mpf_srcptr f, mpq_srcptr q)
  { return __gmp_cmp_function::eval(f, q) < 0; }
  static bool eval(mpq_srcptr q, mpf_srcptr f)
  { return __gmp_cmp_function::eval(q, f) < 0; }
};

struct __gmp_binary_greater
{
  template <class T, class U>
  static inline bool eval(T t, U u) { return __gmp_binary_less::eval(u, t); }
};

struct __gmp_unary_increment
{
  static void eval(mpz_ptr z) { mpz_add_ui(z, z, 1); }
  static void eval(mpq_ptr q)
  { mpz_add(mpq_numref(q), mpq_numref(q), mpq_denref(q)); }
  static void eval(mpf_ptr f) { mpf_add_ui(f, f, 1); }
};

struct __gmp_unary_decrement
{
  static void eval(mpz_ptr z) { mpz_sub_ui(z, z, 1); }
  static void eval(mpq_ptr q)
  { mpz_sub(mpq_numref(q), mpq_numref(q), mpq_denref(q)); }
  static void eval(mpf_ptr f) { mpf_sub_ui(f, f, 1); }
};

struct __gmp_abs_function
{
  static void eval(mpz_ptr z, mpz_srcptr w) { mpz_abs(z, w); }
  static void eval(mpq_ptr q, mpq_srcptr r) { mpq_abs(q, r); }
  static void eval(mpf_ptr f, mpf_srcptr g) { mpf_abs(f, g); }
};

struct __gmp_trunc_function
{
  static void eval(mpf_ptr f, mpf_srcptr g) { mpf_trunc(f, g); }
};

struct __gmp_floor_function
{
  static void eval(mpf_ptr f, mpf_srcptr g) { mpf_floor(f, g); }
};

struct __gmp_ceil_function
{
  static void eval(mpf_ptr f, mpf_srcptr g) { mpf_ceil(f, g); }
};

struct __gmp_sqrt_function
{
  static void eval(mpz_ptr z, mpz_srcptr w) { mpz_sqrt(z, w); }
  static void eval(mpf_ptr f, mpf_srcptr g) { mpf_sqrt(f, g); }
};

struct __gmp_hypot_function
{
  static void eval(mpf_ptr f, mpf_srcptr g, mpf_srcptr h)
  {
    mpf_t temp;
    mpf_init2(temp, mpf_get_prec(f));
    mpf_mul(temp, g, g);
    mpf_mul(f, h, h);
    mpf_add(f, f, temp);
    mpf_sqrt(f, f);
    mpf_clear(temp);
  }

  static void eval(mpf_ptr f, mpf_srcptr g, unsigned long int l)
  {
    mpf_t temp;
    mpf_init2(temp, mpf_get_prec(f));
    mpf_mul(temp, g, g);
    mpf_set_ui(f, l);
    mpf_mul_ui(f, f, l);
    mpf_add(f, f, temp);
    mpf_clear(temp);
    mpf_sqrt(f, f);
  }
  static void eval(mpf_ptr f, unsigned long int l, mpf_srcptr g)
  { eval(f, g, l); }
  static void eval(mpf_ptr f, mpf_srcptr g, signed long int l)
  { eval(f, g, __gmpxx_abs_ui(l)); }
  static void eval(mpf_ptr f, signed long int l, mpf_srcptr g)
  { eval(f, g, l); }
  static void eval(mpf_ptr f, mpf_srcptr g, double d)
  {
    mpf_t temp;
    mpf_init2(temp, mpf_get_prec(f));
    mpf_mul(temp, g, g);
    mpf_set_d(f, d);
    mpf_mul(f, f, f);
    mpf_add(f, f, temp);
    mpf_sqrt(f, f);
    mpf_clear(temp);
  }
  static void eval(mpf_ptr f, double d, mpf_srcptr g)
  { eval(f, g, d); }
};

struct __gmp_sgn_function
{
  static int eval(mpz_srcptr z) { return mpz_sgn(z); }
  static int eval(mpq_srcptr q) { return mpq_sgn(q); }
  static int eval(mpf_srcptr f) { return mpf_sgn(f); }
};

struct __gmp_gcd_function
{
  static void eval(mpz_ptr z, mpz_srcptr w, mpz_srcptr v)
  { mpz_gcd(z, w, v); }
  static void eval(mpz_ptr z, mpz_srcptr w, unsigned long int l)
  { mpz_gcd_ui(z, w, l); }
  static void eval(mpz_ptr z, unsigned long int l, mpz_srcptr w)
  { eval(z, w, l); }
  static void eval(mpz_ptr z, mpz_srcptr w, signed long int l)
  { eval(z, w, __gmpxx_abs_ui(l)); }
  static void eval(mpz_ptr z, signed long int l, mpz_srcptr w)
  { eval(z, w, l); }
  static void eval(mpz_ptr z, mpz_srcptr w, double d)
  {  __GMPXX_TMPZ_D;    mpz_gcd (z, w, temp); }
  static void eval(mpz_ptr z, double d, mpz_srcptr w)
  { eval(z, w, d); }
};

struct __gmp_lcm_function
{
  static void eval(mpz_ptr z, mpz_srcptr w, mpz_srcptr v)
  { mpz_lcm(z, w, v); }
  static void eval(mpz_ptr z, mpz_srcptr w, unsigned long int l)
  { mpz_lcm_ui(z, w, l); }
  static void eval(mpz_ptr z, unsigned long int l, mpz_srcptr w)
  { eval(z, w, l); }
  static void eval(mpz_ptr z, mpz_srcptr w, signed long int l)
  { eval(z, w, __gmpxx_abs_ui(l)); }
  static void eval(mpz_ptr z, signed long int l, mpz_srcptr w)
  { eval(z, w, l); }
  static void eval(mpz_ptr z, mpz_srcptr w, double d)
  {  __GMPXX_TMPZ_D;    mpz_lcm (z, w, temp); }
  static void eval(mpz_ptr z, double d, mpz_srcptr w)
  { eval(z, w, d); }
};

struct __gmp_rand_function
{
  static void eval(mpz_ptr z, gmp_randstate_t s, mp_bitcnt_t l)
  { mpz_urandomb(z, s, l); }
  static void eval(mpz_ptr z, gmp_randstate_t s, mpz_srcptr w)
  { mpz_urandomm(z, s, w); }
  static void eval(mpf_ptr f, gmp_randstate_t s, mp_bitcnt_t prec)
  { mpf_urandomb(f, s, prec); }
};

struct __gmp_fac_function
{
  static void eval(mpz_ptr z, unsigned long l) { mpz_fac_ui(z, l); }
  static void eval(mpz_ptr z, signed long l)
  {
    if (l < 0)
      throw std::domain_error ("factorial(negative)");
    eval(z, static_cast<unsigned long>(l));
  }
  static void eval(mpz_ptr z, mpz_srcptr w)
  {
    if (!mpz_fits_ulong_p(w))
      {
	if (mpz_sgn(w) < 0)
	  throw std::domain_error ("factorial(negative)");
	else
	  throw std::bad_alloc(); // or std::overflow_error ("factorial")?
      }
    eval(z, mpz_get_ui(w));
  }
  static void eval(mpz_ptr z, double d)
  {  __GMPXX_TMPZ_D;    eval (z, temp); }
};

struct __gmp_primorial_function
{
  static void eval(mpz_ptr z, unsigned long l) { mpz_primorial_ui(z, l); }
  static void eval(mpz_ptr z, signed long l)
  {
    if (l < 0)
      throw std::domain_error ("primorial(negative)");
    eval(z, static_cast<unsigned long>(l));
  }
  static void eval(mpz_ptr z, mpz_srcptr w)
  {
    if (!mpz_fits_ulong_p(w))
      {
	if (mpz_sgn(w) < 0)
	  throw std::domain_error ("primorial(negative)");
	else
	  throw std::bad_alloc(); // or std::overflow_error ("primorial")?
      }
    eval(z, mpz_get_ui(w));
  }
  static void eval(mpz_ptr z, double d)
  {  __GMPXX_TMPZ_D;    eval (z, temp); }
};

struct __gmp_fib_function
{
  static void eval(mpz_ptr z, unsigned long l) { mpz_fib_ui(z, l); }
  static void eval(mpz_ptr z, signed long l)
  {
    if (l < 0)
      {
	eval(z, -static_cast<unsigned long>(l));
	if ((l & 1) == 0)
	  mpz_neg(z, z);
      }
    else
      eval(z, static_cast<unsigned long>(l));
  }
  static void eval(mpz_ptr z, mpz_srcptr w)
  {
    if (!mpz_fits_slong_p(w))
      throw std::bad_alloc(); // or std::overflow_error ("fibonacci")?
    eval(z, mpz_get_si(w));
  }
  static void eval(mpz_ptr z, double d)
  {  __GMPXX_TMPZ_D;    eval (z, temp); }
};


/**************** Auxiliary classes ****************/

/* this is much the same as gmp_allocated_string in gmp-impl.h
   since gmp-impl.h is not publicly available, I redefine it here
   I use a different name to avoid possible clashes */

extern "C" {
  typedef void (*__gmp_freefunc_t) (void *, size_t);
}
struct __gmp_alloc_cstring
{
  char *str;
  __gmp_alloc_cstring(char *s) { str = s; }
  ~__gmp_alloc_cstring()
  {
    __gmp_freefunc_t freefunc;
    mp_get_memory_functions (NULL, NULL, &freefunc);
    (*freefunc) (str, std::strlen(str)+1);
  }
};


// general expression template class
template <class T, class U>
class __gmp_expr;


// templates for resolving expression types
template <class T>
struct __gmp_resolve_ref
{
  typedef T ref_type;
};

template <class T, class U>
struct __gmp_resolve_ref<__gmp_expr<T, U> >
{
  typedef const __gmp_expr<T, U> & ref_type;
};


template <class T, class U = T>
struct __gmp_resolve_expr;

template <>
struct __gmp_resolve_expr<mpz_t>
{
  typedef mpz_t value_type;
  typedef mpz_ptr ptr_type;
  typedef mpz_srcptr srcptr_type;
};

template <>
struct __gmp_resolve_expr<mpq_t>
{
  typedef mpq_t value_type;
  typedef mpq_ptr ptr_type;
  typedef mpq_srcptr srcptr_type;
};

template <>
struct __gmp_resolve_expr<mpf_t>
{
  typedef mpf_t value_type;
  typedef mpf_ptr ptr_type;
  typedef mpf_srcptr srcptr_type;
};

template <>
struct __gmp_resolve_expr<mpz_t, mpq_t>
{
  typedef mpq_t value_type;
};

template <>
struct __gmp_resolve_expr<mpq_t, mpz_t>
{
  typedef mpq_t value_type;
};

template <>
struct __gmp_resolve_expr<mpz_t, mpf_t>
{
  typedef mpf_t value_type;
};

template <>
struct __gmp_resolve_expr<mpf_t, mpz_t>
{
  typedef mpf_t value_type;
};

template <>
struct __gmp_resolve_expr<mpq_t, mpf_t>
{
  typedef mpf_t value_type;
};

template <>
struct __gmp_resolve_expr<mpf_t, mpq_t>
{
  typedef mpf_t value_type;
};

#if __GMPXX_USE_CXX11
namespace std {
  template <class T, class U, class V, class W>
  struct common_type <__gmp_expr<T, U>, __gmp_expr<V, W> >
  {
  private:
    typedef typename __gmp_resolve_expr<T, V>::value_type X;
  public:
    typedef __gmp_expr<X, X> type;
  };

  template <class T, class U>
  struct common_type <__gmp_expr<T, U> >
  {
    typedef __gmp_expr<T, T> type;
  };

#define __GMPXX_DECLARE_COMMON_TYPE(typ)	\
  template <class T, class U>			\
  struct common_type <__gmp_expr<T, U>, typ >	\
  {						\
    typedef __gmp_expr<T, T> type;		\
  };						\
						\
  template <class T, class U>			\
  struct common_type <typ, __gmp_expr<T, U> >	\
  {						\
    typedef __gmp_expr<T, T> type;		\
  }

  __GMPXX_DECLARE_COMMON_TYPE(signed char);
  __GMPXX_DECLARE_COMMON_TYPE(unsigned char);
  __GMPXX_DECLARE_COMMON_TYPE(signed int);
  __GMPXX_DECLARE_COMMON_TYPE(unsigned int);
  __GMPXX_DECLARE_COMMON_TYPE(signed short int);
  __GMPXX_DECLARE_COMMON_TYPE(unsigned short int);
  __GMPXX_DECLARE_COMMON_TYPE(signed long int);
  __GMPXX_DECLARE_COMMON_TYPE(unsigned long int);
  __GMPXX_DECLARE_COMMON_TYPE(float);
  __GMPXX_DECLARE_COMMON_TYPE(double);
#undef __GMPXX_DECLARE_COMMON_TYPE
}
#endif

// classes for evaluating unary and binary expressions
template <class T, class Op>
struct __gmp_unary_expr
{
  typename __gmp_resolve_ref<T>::ref_type val;

  __gmp_unary_expr(const T &v) : val(v) { }
private:
  __gmp_unary_expr();
};

template <class T, class U, class Op>
struct __gmp_binary_expr
{
  typename __gmp_resolve_ref<T>::ref_type val1;
  typename __gmp_resolve_ref<U>::ref_type val2;

  __gmp_binary_expr(const T &v1, const U &v2) : val1(v1), val2(v2) { }
private:
  __gmp_binary_expr();
};



/**************** Macros for in-class declarations ****************/
/* This is just repetitive code that is easier to maintain if it's written
   only once */

#define __GMPP_DECLARE_COMPOUND_OPERATOR(fun)                         \
  template <class T, class U>                                         \
  __gmp_expr<value_type, value_type> & fun(const __gmp_expr<T, U> &);

#define __GMPN_DECLARE_COMPOUND_OPERATOR(fun) \
  __gmp_expr & fun(signed char);              \
  __gmp_expr & fun(unsigned char);            \
  __gmp_expr & fun(signed int);               \
  __gmp_expr & fun(unsigned int);             \
  __gmp_expr & fun(signed short int);         \
  __gmp_expr & fun(unsigned short int);       \
  __gmp_expr & fun(signed long int);          \
  __gmp_expr & fun(unsigned long int);        \
  __gmp_expr & fun(float);                    \
  __gmp_expr & fun(double);                   \
  /* __gmp_expr & fun(long double); */

#define __GMP_DECLARE_COMPOUND_OPERATOR(fun) \
__GMPP_DECLARE_COMPOUND_OPERATOR(fun)        \
__GMPN_DECLARE_COMPOUND_OPERATOR(fun)

#define __GMP_DECLARE_COMPOUND_OPERATOR_UI(fun) \
  __gmp_expr & fun(mp_bitcnt_t);

#define __GMP_DECLARE_INCREMENT_OPERATOR(fun) \
  inline __gmp_expr & fun();                  \
  inline __gmp_expr fun(int);

#define __GMPXX_DEFINE_ARITHMETIC_CONSTRUCTORS		\
  __gmp_expr(signed char c) { init_si(c); }		\
  __gmp_expr(unsigned char c) { init_ui(c); }		\
  __gmp_expr(signed int i) { init_si(i); }		\
  __gmp_expr(unsigned int i) { init_ui(i); }		\
  __gmp_expr(signed short int s) { init_si(s); }	\
  __gmp_expr(unsigned short int s) { init_ui(s); }	\
  __gmp_expr(signed long int l) { init_si(l); }		\
  __gmp_expr(unsigned long int l) { init_ui(l); }	\
  __gmp_expr(float f) { init_d(f); }			\
  __gmp_expr(double d) { init_d(d); }

#define __GMPXX_DEFINE_ARITHMETIC_ASSIGNMENTS		\
  __gmp_expr & operator=(signed char c) { assign_si(c); return *this; } \
  __gmp_expr & operator=(unsigned char c) { assign_ui(c); return *this; } \
  __gmp_expr & operator=(signed int i) { assign_si(i); return *this; } \
  __gmp_expr & operator=(unsigned int i) { assign_ui(i); return *this; } \
  __gmp_expr & operator=(signed short int s) { assign_si(s); return *this; } \
  __gmp_expr & operator=(unsigned short int s) { assign_ui(s); return *this; } \
  __gmp_expr & operator=(signed long int l) { assign_si(l); return *this; } \
  __gmp_expr & operator=(unsigned long int l) { assign_ui(l); return *this; } \
  __gmp_expr & operator=(float f) { assign_d(f); return *this; } \
  __gmp_expr & operator=(double d) { assign_d(d); return *this; }

#define __GMPP_DECLARE_UNARY_STATIC_MEMFUN(T, fun, eval_fun)                 \
template <class U>                                                           \
static __gmp_expr<T, __gmp_unary_expr<__gmp_expr<T, U>, eval_fun> >          \
fun(const __gmp_expr<T, U> &expr);

#define __GMPNN_DECLARE_UNARY_STATIC_MEMFUN(T, fun, eval_fun, type, bigtype) \
static inline __gmp_expr<T, __gmp_unary_expr<bigtype, eval_fun> >            \
fun(type expr);

#define __GMPNS_DECLARE_UNARY_STATIC_MEMFUN(T, fun, eval_fun, type)  \
__GMPNN_DECLARE_UNARY_STATIC_MEMFUN(T, fun, eval_fun, type, signed long)
#define __GMPNU_DECLARE_UNARY_STATIC_MEMFUN(T, fun, eval_fun, type)  \
__GMPNN_DECLARE_UNARY_STATIC_MEMFUN(T, fun, eval_fun, type, unsigned long)
#define __GMPND_DECLARE_UNARY_STATIC_MEMFUN(T, fun, eval_fun, type)  \
__GMPNN_DECLARE_UNARY_STATIC_MEMFUN(T, fun, eval_fun, type, double)

#define __GMPN_DECLARE_UNARY_STATIC_MEMFUN(T, fun, eval_fun)                 \
__GMPNS_DECLARE_UNARY_STATIC_MEMFUN(T, fun, eval_fun, signed char)           \
__GMPNU_DECLARE_UNARY_STATIC_MEMFUN(T, fun, eval_fun, unsigned char)         \
__GMPNS_DECLARE_UNARY_STATIC_MEMFUN(T, fun, eval_fun, signed int)            \
__GMPNU_DECLARE_UNARY_STATIC_MEMFUN(T, fun, eval_fun, unsigned int)          \
__GMPNS_DECLARE_UNARY_STATIC_MEMFUN(T, fun, eval_fun, signed short int)      \
__GMPNU_DECLARE_UNARY_STATIC_MEMFUN(T, fun, eval_fun, unsigned short int)    \
__GMPNS_DECLARE_UNARY_STATIC_MEMFUN(T, fun, eval_fun, signed long int)       \
__GMPNU_DECLARE_UNARY_STATIC_MEMFUN(T, fun, eval_fun, unsigned long int)     \
__GMPND_DECLARE_UNARY_STATIC_MEMFUN(T, fun, eval_fun, float)                 \
__GMPND_DECLARE_UNARY_STATIC_MEMFUN(T, fun, eval_fun, double)

#define __GMP_DECLARE_UNARY_STATIC_MEMFUN(T, fun, eval_fun)                  \
__GMPP_DECLARE_UNARY_STATIC_MEMFUN(T, fun, eval_fun)                         \
__GMPN_DECLARE_UNARY_STATIC_MEMFUN(T, fun, eval_fun)

/**************** mpz_class -- wrapper for mpz_t ****************/

template <>
class __gmp_expr<mpz_t, mpz_t>
{
private:
  typedef mpz_t value_type;
  value_type mp;

  // Helper functions used for all arithmetic types
  void assign_ui(unsigned long l)
  {
    if (__GMPXX_CONSTANT_TRUE(l == 0))
      mp->_mp_size = 0;
    else
      mpz_set_ui(mp, l);
  }
  void assign_si(signed long l)
  {
    if (__GMPXX_CONSTANT_TRUE(l >= 0))
      assign_ui(l);
    else if (__GMPXX_CONSTANT_TRUE(l <= 0))
      {
	assign_ui(-static_cast<unsigned long>(l));
	mpz_neg(mp, mp);
      }
    else
      mpz_set_si(mp, l);
  }
  void assign_d (double d)
  {
    mpz_set_d (mp, d);
  }

  void init_ui(unsigned long l)
  {
    if (__GMPXX_CONSTANT_TRUE(l == 0))
      mpz_init(mp);
    else
      mpz_init_set_ui(mp, l);
  }
  void init_si(signed long l)
  {
    if (__GMPXX_CONSTANT_TRUE(l >= 0))
      init_ui(l);
    else if (__GMPXX_CONSTANT_TRUE(l <= 0))
      {
	init_ui(-static_cast<unsigned long>(l));
	mpz_neg(mp, mp);
      }
    else
      mpz_init_set_si(mp, l);
  }
  void init_d (double d)
  {
    mpz_init_set_d (mp, d);
  }

public:
  mp_bitcnt_t get_prec() const { return mpf_get_default_prec(); }

  // constructors and destructor
  __gmp_expr() __GMPXX_NOEXCEPT { mpz_init(mp); }

  __gmp_expr(const __gmp_expr &z) { mpz_init_set(mp, z.mp); }
#if __GMPXX_USE_CXX11
  __gmp_expr(__gmp_expr &&z) noexcept
  { *mp = *z.mp; mpz_init(z.mp); }
#endif
  template <class T>
  __gmp_expr(const __gmp_expr<mpz_t, T> &expr)
  { mpz_init(mp); __gmp_set_expr(mp, expr); }
  template <class T, class U>
  explicit __gmp_expr(const __gmp_expr<T, U> &expr)
  { mpz_init(mp); __gmp_set_expr(mp, expr); }

  __GMPXX_DEFINE_ARITHMETIC_CONSTRUCTORS

  explicit __gmp_expr(const char *s, int base = 0)
  {
    if (mpz_init_set_str (mp, s, base) != 0)
      {
        mpz_clear (mp);
        throw std::invalid_argument ("mpz_set_str");
      }
  }
  explicit __gmp_expr(const std::string &s, int base = 0)
  {
    if (mpz_init_set_str(mp, s.c_str(), base) != 0)
      {
        mpz_clear (mp);
        throw std::invalid_argument ("mpz_set_str");
      }
  }

  explicit __gmp_expr(mpz_srcptr z) { mpz_init_set(mp, z); }

  ~__gmp_expr() { mpz_clear(mp); }

  void swap(__gmp_expr& z) __GMPXX_NOEXCEPT { std::swap(*mp, *z.mp); }

  // assignment operators
  __gmp_expr & operator=(const __gmp_expr &z)
  { mpz_set(mp, z.mp); return *this; }
#if __GMPXX_USE_CXX11
  __gmp_expr & operator=(__gmp_expr &&z) noexcept
  { swap(z); return *this; }
#endif
  template <class T, class U>
  __gmp_expr<value_type, value_type> & operator=(const __gmp_expr<T, U> &expr)
  { __gmp_set_expr(mp, expr); return *this; }

  __GMPXX_DEFINE_ARITHMETIC_ASSIGNMENTS

  __gmp_expr & operator=(const char *s)
  {
    if (mpz_set_str (mp, s, 0) != 0)
      throw std::invalid_argument ("mpz_set_str");
    return *this;
  }
  __gmp_expr & operator=(const std::string &s)
  {
    if (mpz_set_str(mp, s.c_str(), 0) != 0)
      throw std::invalid_argument ("mpz_set_str");
    return *this;
  }

  // string input/output functions
  int set_str(const char *s, int base)
  { return mpz_set_str(mp, s, base); }
  int set_str(const std::string &s, int base)
  { return mpz_set_str(mp, s.c_str(), base); }
  std::string get_str(int base = 10) const
  {
    __gmp_alloc_cstring temp(mpz_get_str(0, base, mp));
    return std::string(temp.str);
  }

  // conversion functions
  mpz_srcptr __get_mp() const { return mp; }
  mpz_ptr __get_mp() { return mp; }
  mpz_srcptr get_mpz_t() const { return mp; }
  mpz_ptr get_mpz_t() { return mp; }

  signed long int get_si() const { return mpz_get_si(mp); }
  unsigned long int get_ui() const { return mpz_get_ui(mp); }
  double get_d() const { return mpz_get_d(mp); }

  // bool fits_schar_p() const { return mpz_fits_schar_p(mp); }
  // bool fits_uchar_p() const { return mpz_fits_uchar_p(mp); }
  bool fits_sint_p() const { return mpz_fits_sint_p(mp); }
  bool fits_uint_p() const { return mpz_fits_uint_p(mp); }
  bool fits_sshort_p() const { return mpz_fits_sshort_p(mp); }
  bool fits_ushort_p() const { return mpz_fits_ushort_p(mp); }
  bool fits_slong_p() const { return mpz_fits_slong_p(mp); }
  bool fits_ulong_p() const { return mpz_fits_ulong_p(mp); }
  // bool fits_float_p() const { return mpz_fits_float_p(mp); }
  // bool fits_double_p() const { return mpz_fits_double_p(mp); }
  // bool fits_ldouble_p() const { return mpz_fits_ldouble_p(mp); }

#if __GMPXX_USE_CXX11
  explicit operator bool() const { return mp->_mp_size != 0; }
#endif

  // member operators
  __GMP_DECLARE_COMPOUND_OPERATOR(operator+=)
  __GMP_DECLARE_COMPOUND_OPERATOR(operator-=)
  __GMP_DECLARE_COMPOUND_OPERATOR(operator*=)
  __GMP_DECLARE_COMPOUND_OPERATOR(operator/=)
  __GMP_DECLARE_COMPOUND_OPERATOR(operator%=)

  __GMP_DECLARE_COMPOUND_OPERATOR(operator&=)
  __GMP_DECLARE_COMPOUND_OPERATOR(operator|=)
  __GMP_DECLARE_COMPOUND_OPERATOR(operator^=)

  __GMP_DECLARE_COMPOUND_OPERATOR_UI(operator<<=)
  __GMP_DECLARE_COMPOUND_OPERATOR_UI(operator>>=)

  __GMP_DECLARE_INCREMENT_OPERATOR(operator++)
  __GMP_DECLARE_INCREMENT_OPERATOR(operator--)

  __GMP_DECLARE_UNARY_STATIC_MEMFUN(mpz_t, factorial, __gmp_fac_function)
  __GMP_DECLARE_UNARY_STATIC_MEMFUN(mpz_t, primorial, __gmp_primorial_function)
  __GMP_DECLARE_UNARY_STATIC_MEMFUN(mpz_t, fibonacci, __gmp_fib_function)
};

typedef __gmp_expr<mpz_t, mpz_t> mpz_class;


/**************** mpq_class -- wrapper for mpq_t ****************/

template <>
class __gmp_expr<mpq_t, mpq_t>
{
private:
  typedef mpq_t value_type;
  value_type mp;

  // Helper functions used for all arithmetic types
  void assign_ui(unsigned long l) { mpq_set_ui(mp, l, 1); }
  void assign_si(signed long l)
  {
    if (__GMPXX_CONSTANT_TRUE(l >= 0))
      assign_ui(l);
    else
      mpq_set_si(mp, l, 1);
  }
  void assign_d (double d)        { mpq_set_d (mp, d); }

  void init_ui(unsigned long l)	{ mpq_init(mp); get_num() = l; }
  void init_si(signed long l)	{ mpq_init(mp); get_num() = l; }
  void init_d (double d)	{ mpq_init(mp); assign_d (d); }

public:
  mp_bitcnt_t get_prec() const { return mpf_get_default_prec(); }
  void canonicalize() { mpq_canonicalize(mp); }

  // constructors and destructor
  __gmp_expr() { mpq_init(mp); }

  __gmp_expr(const __gmp_expr &q)
  {
    mpz_init_set(mpq_numref(mp), mpq_numref(q.mp));
    mpz_init_set(mpq_denref(mp), mpq_denref(q.mp));
  }
#if __GMPXX_USE_CXX11
  __gmp_expr(__gmp_expr &&q)
  { *mp = *q.mp; mpq_init(q.mp); }
#endif
  template <class T>
  __gmp_expr(const __gmp_expr<mpz_t, T> &expr)
  { mpq_init(mp); __gmp_set_expr(mp, expr); }
  template <class T>
  __gmp_expr(const __gmp_expr<mpq_t, T> &expr)
  { mpq_init(mp); __gmp_set_expr(mp, expr); }
  template <class T, class U>
  explicit __gmp_expr(const __gmp_expr<T, U> &expr)
  { mpq_init(mp); __gmp_set_expr(mp, expr); }

  __GMPXX_DEFINE_ARITHMETIC_CONSTRUCTORS

  explicit __gmp_expr(const char *s, int base = 0)
  {
    mpq_init (mp);
    // If s is the literal 0, we meant to call another constructor.
    // If s just happens to evaluate to 0, we would crash, so whatever.
    if (s == 0)
      {
	// Don't turn mpq_class(0,0) into 0
	mpz_set_si(mpq_denref(mp), base);
      }
    else if (mpq_set_str(mp, s, base) != 0)
      {
        mpq_clear (mp);
        throw std::invalid_argument ("mpq_set_str");
      }
  }
  explicit __gmp_expr(const std::string &s, int base = 0)
  {
    mpq_init(mp);
    if (mpq_set_str (mp, s.c_str(), base) != 0)
      {
        mpq_clear (mp);
        throw std::invalid_argument ("mpq_set_str");
      }
  }
  explicit __gmp_expr(mpq_srcptr q)
  {
    mpz_init_set(mpq_numref(mp), mpq_numref(q));
    mpz_init_set(mpq_denref(mp), mpq_denref(q));
  }

  __gmp_expr(const mpz_class &num, const mpz_class &den)
  {
    mpz_init_set(mpq_numref(mp), num.get_mpz_t());
    mpz_init_set(mpq_denref(mp), den.get_mpz_t());
  }

  ~__gmp_expr() { mpq_clear(mp); }

  void swap(__gmp_expr& q) __GMPXX_NOEXCEPT { std::swap(*mp, *q.mp); }

  // assignment operators
  __gmp_expr & operator=(const __gmp_expr &q)
  { mpq_set(mp, q.mp); return *this; }
#if __GMPXX_USE_CXX11
  __gmp_expr & operator=(__gmp_expr &&q) noexcept
  { swap(q); return *this; }
  __gmp_expr & operator=(mpz_class &&z) noexcept
  { get_num() = std::move(z); get_den() = 1u; return *this; }
#endif
  template <class T, class U>
  __gmp_expr<value_type, value_type> & operator=(const __gmp_expr<T, U> &expr)
  { __gmp_set_expr(mp, expr); return *this; }

  __GMPXX_DEFINE_ARITHMETIC_ASSIGNMENTS

  __gmp_expr & operator=(const char *s)
  {
    if (mpq_set_str (mp, s, 0) != 0)
      throw std::invalid_argument ("mpq_set_str");
    return *this;
  }
  __gmp_expr & operator=(const std::string &s)
  {
    if (mpq_set_str(mp, s.c_str(), 0) != 0)
      throw std::invalid_argument ("mpq_set_str");
    return *this;
  }

  // string input/output functions
  int set_str(const char *s, int base)
  { return mpq_set_str(mp, s, base); }
  int set_str(const std::string &s, int base)
  { return mpq_set_str(mp, s.c_str(), base); }
  std::string get_str(int base = 10) const
  {
    __gmp_alloc_cstring temp(mpq_get_str(0, base, mp));
    return std::string(temp.str);
  }

  // conversion functions

  // casting a reference to an mpz_t to mpz_class & is a dirty hack,
  // but works because the internal representation of mpz_class is
  // exactly an mpz_t
  const mpz_class & get_num() const
  { return reinterpret_cast<const mpz_class &>(*mpq_numref(mp)); }
  mpz_class & get_num()
  { return reinterpret_cast<mpz_class &>(*mpq_numref(mp)); }
  const mpz_class & get_den() const
  { return reinterpret_cast<const mpz_class &>(*mpq_denref(mp)); }
  mpz_class & get_den()
  { return reinterpret_cast<mpz_class &>(*mpq_denref(mp)); }

  mpq_srcptr __get_mp() const { return mp; }
  mpq_ptr __get_mp() { return mp; }
  mpq_srcptr get_mpq_t() const { return mp; }
  mpq_ptr get_mpq_t() { return mp; }

  mpz_srcptr get_num_mpz_t() const { return mpq_numref(mp); }
  mpz_ptr get_num_mpz_t() { return mpq_numref(mp); }
  mpz_srcptr get_den_mpz_t() const { return mpq_denref(mp); }
  mpz_ptr get_den_mpz_t() { return mpq_denref(mp); }

  double get_d() const { return mpq_get_d(mp); }

#if __GMPXX_USE_CXX11
  explicit operator bool() const { return mpq_numref(mp)->_mp_size != 0; }
#endif

  // compound assignments
  __GMP_DECLARE_COMPOUND_OPERATOR(operator+=)
  __GMP_DECLARE_COMPOUND_OPERATOR(operator-=)
  __GMP_DECLARE_COMPOUND_OPERATOR(operator*=)
  __GMP_DECLARE_COMPOUND_OPERATOR(operator/=)

  __GMP_DECLARE_COMPOUND_OPERATOR_UI(operator<<=)
  __GMP_DECLARE_COMPOUND_OPERATOR_UI(operator>>=)

  __GMP_DECLARE_INCREMENT_OPERATOR(operator++)
  __GMP_DECLARE_INCREMENT_OPERATOR(operator--)
};

typedef __gmp_expr<mpq_t, mpq_t> mpq_class;


/**************** mpf_class -- wrapper for mpf_t ****************/

template <>
class __gmp_expr<mpf_t, mpf_t>
{
private:
  typedef mpf_t value_type;
  value_type mp;

  // Helper functions used for all arithmetic types
  void assign_ui(unsigned long l) { mpf_set_ui(mp, l); }
  void assign_si(signed long l)
  {
    if (__GMPXX_CONSTANT_TRUE(l >= 0))
      assign_ui(l);
    else
      mpf_set_si(mp, l);
  }
  void assign_d (double d)        { mpf_set_d (mp, d); }

  void init_ui(unsigned long l)
  {
    if (__GMPXX_CONSTANT_TRUE(l == 0))
      mpf_init(mp);
    else
      mpf_init_set_ui(mp, l);
  }
  void init_si(signed long l)
  {
    if (__GMPXX_CONSTANT_TRUE(l >= 0))
      init_ui(l);
    else
      mpf_init_set_si(mp, l);
  }
  void init_d (double d)	{ mpf_init_set_d (mp, d); }

public:
  mp_bitcnt_t get_prec() const { return mpf_get_prec(mp); }

  void set_prec(mp_bitcnt_t prec) { mpf_set_prec(mp, prec); }
  void set_prec_raw(mp_bitcnt_t prec) { mpf_set_prec_raw(mp, prec); }

  // constructors and destructor
  __gmp_expr() { mpf_init(mp); }

  __gmp_expr(const __gmp_expr &f)
  { mpf_init2(mp, f.get_prec()); mpf_set(mp, f.mp); }
#if __GMPXX_USE_CXX11
  __gmp_expr(__gmp_expr &&f)
  { *mp = *f.mp; mpf_init2(f.mp, get_prec()); }
#endif
  __gmp_expr(const __gmp_expr &f, mp_bitcnt_t prec)
  { mpf_init2(mp, prec); mpf_set(mp, f.mp); }
  template <class T, class U>
  __gmp_expr(const __gmp_expr<T, U> &expr)
  { mpf_init2(mp, expr.get_prec()); __gmp_set_expr(mp, expr); }
  template <class T, class U>
  __gmp_expr(const __gmp_expr<T, U> &expr, mp_bitcnt_t prec)
  { mpf_init2(mp, prec); __gmp_set_expr(mp, expr); }

  __GMPXX_DEFINE_ARITHMETIC_CONSTRUCTORS

  __gmp_expr(signed char c, mp_bitcnt_t prec)
  { mpf_init2(mp, prec); mpf_set_si(mp, c); }
  __gmp_expr(unsigned char c, mp_bitcnt_t prec)
  { mpf_init2(mp, prec); mpf_set_ui(mp, c); }

  __gmp_expr(signed int i, mp_bitcnt_t prec)
  { mpf_init2(mp, prec); mpf_set_si(mp, i); }
  __gmp_expr(unsigned int i, mp_bitcnt_t prec)
  { mpf_init2(mp, prec); mpf_set_ui(mp, i); }

  __gmp_expr(signed short int s, mp_bitcnt_t prec)
  { mpf_init2(mp, prec); mpf_set_si(mp, s); }
  __gmp_expr(unsigned short int s, mp_bitcnt_t prec)
  { mpf_init2(mp, prec); mpf_set_ui(mp, s); }

  __gmp_expr(signed long int l, mp_bitcnt_t prec)
  { mpf_init2(mp, prec); mpf_set_si(mp, l); }
  __gmp_expr(unsigned long int l, mp_bitcnt_t prec)
  { mpf_init2(mp, prec); mpf_set_ui(mp, l); }

  __gmp_expr(float f, mp_bitcnt_t prec)
  { mpf_init2(mp, prec); mpf_set_d(mp, f); }
  __gmp_expr(double d, mp_bitcnt_t prec)
  { mpf_init2(mp, prec); mpf_set_d(mp, d); }
  // __gmp_expr(long double ld) { mpf_init_set_d(mp, ld); }
  // __gmp_expr(long double ld, mp_bitcnt_t prec)
  // { mpf_init2(mp, prec); mpf_set_d(mp, ld); }

  explicit __gmp_expr(const char *s)
  {
    if (mpf_init_set_str (mp, s, 0) != 0)
      {
        mpf_clear (mp);
        throw std::invalid_argument ("mpf_set_str");
      }
  }
  __gmp_expr(const char *s, mp_bitcnt_t prec, int base = 0)
  {
    mpf_init2(mp, prec);
    if (mpf_set_str(mp, s, base) != 0)
      {
        mpf_clear (mp);
        throw std::invalid_argument ("mpf_set_str");
      }
  }
  explicit __gmp_expr(const std::string &s)
  {
    if (mpf_init_set_str(mp, s.c_str(), 0) != 0)
      {
        mpf_clear (mp);
        throw std::invalid_argument ("mpf_set_str");
      }
  }
  __gmp_expr(const std::string &s, mp_bitcnt_t prec, int base = 0)
  {
    mpf_init2(mp, prec);
    if (mpf_set_str(mp, s.c_str(), base) != 0)
      {
        mpf_clear (mp);
        throw std::invalid_argument ("mpf_set_str");
      }
  }

  explicit __gmp_expr(mpf_srcptr f)
  { mpf_init2(mp, mpf_get_prec(f)); mpf_set(mp, f); }
  __gmp_expr(mpf_srcptr f, mp_bitcnt_t prec)
  { mpf_init2(mp, prec); mpf_set(mp, f); }

  ~__gmp_expr() { mpf_clear(mp); }

  void swap(__gmp_expr& f) __GMPXX_NOEXCEPT { std::swap(*mp, *f.mp); }

  // assignment operators
  __gmp_expr & operator=(const __gmp_expr &f)
  { mpf_set(mp, f.mp); return *this; }
#if __GMPXX_USE_CXX11
  __gmp_expr & operator=(__gmp_expr &&f) noexcept
  { swap(f); return *this; }
#endif
  template <class T, class U>
  __gmp_expr<value_type, value_type> & operator=(const __gmp_expr<T, U> &expr)
  { __gmp_set_expr(mp, expr); return *this; }

  __GMPXX_DEFINE_ARITHMETIC_ASSIGNMENTS

  __gmp_expr & operator=(const char *s)
  {
    if (mpf_set_str (mp, s, 0) != 0)
      throw std::invalid_argument ("mpf_set_str");
    return *this;
  }
  __gmp_expr & operator=(const std::string &s)
  {
    if (mpf_set_str(mp, s.c_str(), 0) != 0)
      throw std::invalid_argument ("mpf_set_str");
    return *this;
  }

  // string input/output functions
  int set_str(const char *s, int base)
  { return mpf_set_str(mp, s, base); }
  int set_str(const std::string &s, int base)
  { return mpf_set_str(mp, s.c_str(), base); }
  std::string get_str(mp_exp_t &expo, int base = 10, size_t size = 0) const
  {
    __gmp_alloc_cstring temp(mpf_get_str(0, &expo, base, size, mp));
    return std::string(temp.str);
  }

  // conversion functions
  mpf_srcptr __get_mp() const { return mp; }
  mpf_ptr __get_mp() { return mp; }
  mpf_srcptr get_mpf_t() const { return mp; }
  mpf_ptr get_mpf_t() { return mp; }

  signed long int get_si() const { return mpf_get_si(mp); }
  unsigned long int get_ui() const { return mpf_get_ui(mp); }
  double get_d() const { return mpf_get_d(mp); }

  // bool fits_schar_p() const { return mpf_fits_schar_p(mp); }
  // bool fits_uchar_p() const { return mpf_fits_uchar_p(mp); }
  bool fits_sint_p() const { return mpf_fits_sint_p(mp); }
  bool fits_uint_p() const { return mpf_fits_uint_p(mp); }
  bool fits_sshort_p() const { return mpf_fits_sshort_p(mp); }
  bool fits_ushort_p() const { return mpf_fits_ushort_p(mp); }
  bool fits_slong_p() const { return mpf_fits_slong_p(mp); }
  bool fits_ulong_p() const { return mpf_fits_ulong_p(mp); }
  // bool fits_float_p() const { return mpf_fits_float_p(mp); }
  // bool fits_double_p() const { return mpf_fits_double_p(mp); }
  // bool fits_ldouble_p() const { return mpf_fits_ldouble_p(mp); }

#if __GMPXX_USE_CXX11
  explicit operator bool() const { return mpf_sgn(mp) != 0; }
#endif

  // compound assignments
  __GMP_DECLARE_COMPOUND_OPERATOR(operator+=)
  __GMP_DECLARE_COMPOUND_OPERATOR(operator-=)
  __GMP_DECLARE_COMPOUND_OPERATOR(operator*=)
  __GMP_DECLARE_COMPOUND_OPERATOR(operator/=)

  __GMP_DECLARE_COMPOUND_OPERATOR_UI(operator<<=)
  __GMP_DECLARE_COMPOUND_OPERATOR_UI(operator>>=)

  __GMP_DECLARE_INCREMENT_OPERATOR(operator++)
  __GMP_DECLARE_INCREMENT_OPERATOR(operator--)
};

typedef __gmp_expr<mpf_t, mpf_t> mpf_class;



/**************** User-defined literals ****************/

#if __GMPXX_USE_CXX11
inline mpz_class operator"" _mpz(const char* s)
{
  return mpz_class(s);
}

inline mpq_class operator"" _mpq(const char* s)
{
  mpq_class q;
  q.get_num() = s;
  return q;
}

inline mpf_class operator"" _mpf(const char* s)
{
  return mpf_class(s);
}
#endif

/**************** I/O operators ****************/

// these should (and will) be provided separately

template <class T, class U>
inline std::ostream & operator<<
(std::ostream &o, const __gmp_expr<T, U> &expr)
{
  __gmp_expr<T, T> const& temp(expr);
  return o << temp.__get_mp();
}

template <class T>
inline std::istream & operator>>(std::istream &i, __gmp_expr<T, T> &expr)
{
  return i >> expr.__get_mp();
}

/*
// you might want to uncomment this
inline std::istream & operator>>(std::istream &i, mpq_class &q)
{
  i >> q.get_mpq_t();
  q.canonicalize();
  return i;
}
*/


/**************** Functions for type conversion ****************/

inline void __gmp_set_expr(mpz_ptr z, const mpz_class &w)
{
  mpz_set(z, w.get_mpz_t());
}

template <class T>
inline void __gmp_set_expr(mpz_ptr z, const __gmp_expr<mpz_t, T> &expr)
{
  expr.eval(z);
}

template <class T>
inline void __gmp_set_expr(mpz_ptr z, const __gmp_expr<mpq_t, T> &expr)
{
  mpq_class const& temp(expr);
  mpz_set_q(z, temp.get_mpq_t());
}

template <class T>
inline void __gmp_set_expr(mpz_ptr z, const __gmp_expr<mpf_t, T> &expr)
{
  mpf_class const& temp(expr);
  mpz_set_f(z, temp.get_mpf_t());
}

inline void __gmp_set_expr(mpq_ptr q, const mpz_class &z)
{
  mpq_set_z(q, z.get_mpz_t());
}

template <class T>
inline void __gmp_set_expr(mpq_ptr q, const __gmp_expr<mpz_t, T> &expr)
{
  __gmp_set_expr(mpq_numref(q), expr);
  mpz_set_ui(mpq_denref(q), 1);
}

inline void __gmp_set_expr(mpq_ptr q, const mpq_class &r)
{
  mpq_set(q, r.get_mpq_t());
}

template <class T>
inline void __gmp_set_expr(mpq_ptr q, const __gmp_expr<mpq_t, T> &expr)
{
  expr.eval(q);
}

template <class T>
inline void __gmp_set_expr(mpq_ptr q, const __gmp_expr<mpf_t, T> &expr)
{
  mpf_class const& temp(expr);
  mpq_set_f(q, temp.get_mpf_t());
}

template <class T>
inline void __gmp_set_expr(mpf_ptr f, const __gmp_expr<mpz_t, T> &expr)
{
  mpz_class const& temp(expr);
  mpf_set_z(f, temp.get_mpz_t());
}

template <class T>
inline void __gmp_set_expr(mpf_ptr f, const __gmp_expr<mpq_t, T> &expr)
{
  mpq_class const& temp(expr);
  mpf_set_q(f, temp.get_mpq_t());
}

inline void __gmp_set_expr(mpf_ptr f, const mpf_class &g)
{
  mpf_set(f, g.get_mpf_t());
}

template <class T>
inline void __gmp_set_expr(mpf_ptr f, const __gmp_expr<mpf_t, T> &expr)
{
  expr.eval(f);
}


/* Temporary objects */

template <class T>
class __gmp_temp
{
  __gmp_expr<T, T> val;
  public:
  template<class U, class V>
  __gmp_temp(U const& u, V) : val (u) {}
  typename __gmp_resolve_expr<T>::srcptr_type
  __get_mp() const { return val.__get_mp(); }
};

template <>
class __gmp_temp <mpf_t>
{
  mpf_class val;
  public:
  template<class U>
  __gmp_temp(U const& u, mpf_ptr res) : val (u, mpf_get_prec(res)) {}
  mpf_srcptr __get_mp() const { return val.__get_mp(); }
};

/**************** Specializations of __gmp_expr ****************/
/* The eval() method of __gmp_expr<T, U> evaluates the corresponding
   expression and assigns the result to its argument, which is either an
   mpz_t, mpq_t, or mpf_t as specified by the T argument.
   Compound expressions are evaluated recursively (temporaries are created
   to hold intermediate values), while for simple expressions the eval()
   method of the appropriate function object (available as the Op argument
   of either __gmp_unary_expr<T, Op> or __gmp_binary_expr<T, U, Op>) is
   called. */


/**************** Unary expressions ****************/
/* cases:
   - simple:   argument is mp*_class, that is, __gmp_expr<T, T>
   - compound: argument is __gmp_expr<T, U> (with U not equal to T) */


// simple expressions

template <class T, class Op>
class __gmp_expr<T, __gmp_unary_expr<__gmp_expr<T, T>, Op> >
{
private:
  typedef __gmp_expr<T, T> val_type;

  __gmp_unary_expr<val_type, Op> expr;
public:
  explicit __gmp_expr(const val_type &val) : expr(val) { }
  void eval(typename __gmp_resolve_expr<T>::ptr_type p) const
  { Op::eval(p, expr.val.__get_mp()); }
  const val_type & get_val() const { return expr.val; }
  mp_bitcnt_t get_prec() const { return expr.val.get_prec(); }
};


// simple expressions, U is a built-in numerical type

template <class T, class U, class Op>
class __gmp_expr<T, __gmp_unary_expr<U, Op> >
{
private:
  typedef U val_type;

  __gmp_unary_expr<val_type, Op> expr;
public:
  explicit __gmp_expr(const val_type &val) : expr(val) { }
  void eval(typename __gmp_resolve_expr<T>::ptr_type p) const
  { Op::eval(p, expr.val); }
  const val_type & get_val() const { return expr.val; }
  mp_bitcnt_t get_prec() const { return mpf_get_default_prec(); }
};


// compound expressions

template <class T, class U, class Op>
class __gmp_expr<T, __gmp_unary_expr<__gmp_expr<T, U>, Op> >
{
private:
  typedef __gmp_expr<T, U> val_type;

  __gmp_unary_expr<val_type, Op> expr;
public:
  explicit __gmp_expr(const val_type &val) : expr(val) { }
  void eval(typename __gmp_resolve_expr<T>::ptr_type p) const
  { expr.val.eval(p); Op::eval(p, p); }
  const val_type & get_val() const { return expr.val; }
  mp_bitcnt_t get_prec() const { return expr.val.get_prec(); }
};


/**************** Binary expressions ****************/
/* simple:
   - arguments are both mp*_class
   - one argument is mp*_class, one is a built-in type
   compound:
   - one is mp*_class, one is __gmp_expr<T, U>
   - one is __gmp_expr<T, U>, one is built-in
   - both arguments are __gmp_expr<...> */


// simple expressions

template <class T, class Op>
class __gmp_expr
<T, __gmp_binary_expr<__gmp_expr<T, T>, __gmp_expr<T, T>, Op> >
{
private:
  typedef __gmp_expr<T, T> val1_type;
  typedef __gmp_expr<T, T> val2_type;

  __gmp_binary_expr<val1_type, val2_type, Op> expr;
public:
  __gmp_expr(const val1_type &val1, const val2_type &val2)
    : expr(val1, val2) { }
  void eval(typename __gmp_resolve_expr<T>::ptr_type p) const
  { Op::eval(p, expr.val1.__get_mp(), expr.val2.__get_mp()); }
  const val1_type & get_val1() const { return expr.val1; }
  const val2_type & get_val2() const { return expr.val2; }
  mp_bitcnt_t get_prec() const
  {
    mp_bitcnt_t prec1 = expr.val1.get_prec(),
      prec2 = expr.val2.get_prec();
    return (prec1 > prec2) ? prec1 : prec2;
  }
};


// simple expressions, U is a built-in numerical type

template <class T, class U, class Op>
class __gmp_expr<T, __gmp_binary_expr<__gmp_expr<T, T>, U, Op> >
{
private:
  typedef __gmp_expr<T, T> val1_type;
  typedef U val2_type;

  __gmp_binary_expr<val1_type, val2_type, Op> expr;
public:
  __gmp_expr(const val1_type &val1, const val2_type &val2)
    : expr(val1, val2) { }
  void eval(typename __gmp_resolve_expr<T>::ptr_type p) const
  { Op::eval(p, expr.val1.__get_mp(), expr.val2); }
  const val1_type & get_val1() const { return expr.val1; }
  const val2_type & get_val2() const { return expr.val2; }
  mp_bitcnt_t get_prec() const { return expr.val1.get_prec(); }
};

template <class T, class U, class Op>
class __gmp_expr<T, __gmp_binary_expr<U, __gmp_expr<T, T>, Op> >
{
private:
  typedef U val1_type;
  typedef __gmp_expr<T, T> val2_type;

  __gmp_binary_expr<val1_type, val2_type, Op> expr;
public:
  __gmp_expr(const val1_type &val1, const val2_type &val2)
    : expr(val1, val2) { }
  void eval(typename __gmp_resolve_expr<T>::ptr_type p) const
  { Op::eval(p, expr.val1, expr.val2.__get_mp()); }
  const val1_type & get_val1() const { return expr.val1; }
  const val2_type & get_val2() const { return expr.val2; }
  mp_bitcnt_t get_prec() const { return expr.val2.get_prec(); }
};


// compound expressions, one argument is a subexpression

template <class T, class U, class V, class Op>
class __gmp_expr
<T, __gmp_binary_expr<__gmp_expr<T, T>, __gmp_expr<U, V>, Op> >
{
private:
  typedef __gmp_expr<T, T> val1_type;
  typedef __gmp_expr<U, V> val2_type;

  __gmp_binary_expr<val1_type, val2_type, Op> expr;
public:
  __gmp_expr(const val1_type &val1, const val2_type &val2)
    : expr(val1, val2) { }
  void eval(typename __gmp_resolve_expr<T>::ptr_type p) const
  {
    if(p != expr.val1.__get_mp())
    {
      __gmp_set_expr(p, expr.val2);
      Op::eval(p, expr.val1.__get_mp(), p);
    }
    else
    {
      __gmp_temp<T> temp(expr.val2, p);
      Op::eval(p, expr.val1.__get_mp(), temp.__get_mp());
    }
  }
  const val1_type & get_val1() const { return expr.val1; }
  const val2_type & get_val2() const { return expr.val2; }
  mp_bitcnt_t get_prec() const
  {
    mp_bitcnt_t prec1 = expr.val1.get_prec(),
      prec2 = expr.val2.get_prec();
    return (prec1 > prec2) ? prec1 : prec2;
  }
};

template <class T, class U, class V, class Op>
class __gmp_expr
<T, __gmp_binary_expr<__gmp_expr<U, V>, __gmp_expr<T, T>, Op> >
{
private:
  typedef __gmp_expr<U, V> val1_type;
  typedef __gmp_expr<T, T> val2_type;

  __gmp_binary_expr<val1_type, val2_type, Op> expr;
public:
  __gmp_expr(const val1_type &val1, const val2_type &val2)
    : expr(val1, val2) { }
  void eval(typename __gmp_resolve_expr<T>::ptr_type p) const
  {
    if(p != expr.val2.__get_mp())
    {
      __gmp_set_expr(p, expr.val1);
      Op::eval(p, p, expr.val2.__get_mp());
    }
    else
    {
      __gmp_temp<T> temp(expr.val1, p);
      Op::eval(p, temp.__get_mp(), expr.val2.__get_mp());
    }
  }
  const val1_type & get_val1() const { return expr.val1; }
  const val2_type & get_val2() const { return expr.val2; }
  mp_bitcnt_t get_prec() const
  {
    mp_bitcnt_t prec1 = expr.val1.get_prec(),
      prec2 = expr.val2.get_prec();
    return (prec1 > prec2) ? prec1 : prec2;
  }
};

template <class T, class U, class Op>
class __gmp_expr
<T, __gmp_binary_expr<__gmp_expr<T, T>, __gmp_expr<T, U>, Op> >
{
private:
  typedef __gmp_expr<T, T> val1_type;
  typedef __gmp_expr<T, U> val2_type;

  __gmp_binary_expr<val1_type, val2_type, Op> expr;
public:
  __gmp_expr(const val1_type &val1, const val2_type &val2)
    : expr(val1, val2) { }
  void eval(typename __gmp_resolve_expr<T>::ptr_type p) const
  {
    if(p != expr.val1.__get_mp())
    {
      __gmp_set_expr(p, expr.val2);
      Op::eval(p, expr.val1.__get_mp(), p);
    }
    else
    {
      __gmp_temp<T> temp(expr.val2, p);
      Op::eval(p, expr.val1.__get_mp(), temp.__get_mp());
    }
  }
  const val1_type & get_val1() const { return expr.val1; }
  const val2_type & get_val2() const { return expr.val2; }
  mp_bitcnt_t get_prec() const
  {
    mp_bitcnt_t prec1 = expr.val1.get_prec(),
      prec2 = expr.val2.get_prec();
    return (prec1 > prec2) ? prec1 : prec2;
  }
};

template <class T, class U, class Op>
class __gmp_expr
<T, __gmp_binary_expr<__gmp_expr<T, U>, __gmp_expr<T, T>, Op> >
{
private:
  typedef __gmp_expr<T, U> val1_type;
  typedef __gmp_expr<T, T> val2_type;

  __gmp_binary_expr<val1_type, val2_type, Op> expr;
public:
  __gmp_expr(const val1_type &val1, const val2_type &val2)
    : expr(val1, val2) { }
  void eval(typename __gmp_resolve_expr<T>::ptr_type p) const
  {
    if(p != expr.val2.__get_mp())
    {
      __gmp_set_expr(p, expr.val1);
      Op::eval(p, p, expr.val2.__get_mp());
    }
    else
    {
      __gmp_temp<T> temp(expr.val1, p);
      Op::eval(p, temp.__get_mp(), expr.val2.__get_mp());
    }
  }
  const val1_type & get_val1() const { return expr.val1; }
  const val2_type & get_val2() const { return expr.val2; }
  mp_bitcnt_t get_prec() const
  {
    mp_bitcnt_t prec1 = expr.val1.get_prec(),
      prec2 = expr.val2.get_prec();
    return (prec1 > prec2) ? prec1 : prec2;
  }
};


// one argument is a subexpression, one is a built-in

template <class T, class U, class V, class Op>
class __gmp_expr<T, __gmp_binary_expr<__gmp_expr<T, U>, V, Op> >
{
private:
  typedef __gmp_expr<T, U> val1_type;
  typedef V val2_type;

  __gmp_binary_expr<val1_type, val2_type, Op> expr;
public:
  __gmp_expr(const val1_type &val1, const val2_type &val2)
    : expr(val1, val2) { }
  void eval(typename __gmp_resolve_expr<T>::ptr_type p) const
  {
    expr.val1.eval(p);
    Op::eval(p, p, expr.val2);
  }
  const val1_type & get_val1() const { return expr.val1; }
  const val2_type & get_val2() const { return expr.val2; }
  mp_bitcnt_t get_prec() const { return expr.val1.get_prec(); }
};

template <class T, class U, class V, class Op>
class __gmp_expr<T, __gmp_binary_expr<U, __gmp_expr<T, V>, Op> >
{
private:
  typedef U val1_type;
  typedef __gmp_expr<T, V> val2_type;

  __gmp_binary_expr<val1_type, val2_type, Op> expr;
public:
  __gmp_expr(const val1_type &val1, const val2_type &val2)
    : expr(val1, val2) { }
  void eval(typename __gmp_resolve_expr<T>::ptr_type p) const
  {
    expr.val2.eval(p);
    Op::eval(p, expr.val1, p);
  }
  const val1_type & get_val1() const { return expr.val1; }
  const val2_type & get_val2() const { return expr.val2; }
  mp_bitcnt_t get_prec() const { return expr.val2.get_prec(); }
};


// both arguments are subexpressions

template <class T, class U, class V, class W, class Op>
class __gmp_expr
<T, __gmp_binary_expr<__gmp_expr<T, U>, __gmp_expr<V, W>, Op> >
{
private:
  typedef __gmp_expr<T, U> val1_type;
  typedef __gmp_expr<V, W> val2_type;

  __gmp_binary_expr<val1_type, val2_type, Op> expr;
public:
  __gmp_expr(const val1_type &val1, const val2_type &val2)
    : expr(val1, val2) { }
  void eval(typename __gmp_resolve_expr<T>::ptr_type p) const
  {
    __gmp_temp<T> temp2(expr.val2, p);
    expr.val1.eval(p);
    Op::eval(p, p, temp2.__get_mp());
  }
  const val1_type & get_val1() const { return expr.val1; }
  const val2_type & get_val2() const { return expr.val2; }
  mp_bitcnt_t get_prec() const
  {
    mp_bitcnt_t prec1 = expr.val1.get_prec(),
      prec2 = expr.val2.get_prec();
    return (prec1 > prec2) ? prec1 : prec2;
  }
};

template <class T, class U, class V, class W, class Op>
class __gmp_expr
<T, __gmp_binary_expr<__gmp_expr<U, V>, __gmp_expr<T, W>, Op> >
{
private:
  typedef __gmp_expr<U, V> val1_type;
  typedef __gmp_expr<T, W> val2_type;

  __gmp_binary_expr<val1_type, val2_type, Op> expr;
public:
  __gmp_expr(const val1_type &val1, const val2_type &val2)
    : expr(val1, val2) { }
  void eval(typename __gmp_resolve_expr<T>::ptr_type p) const
  {
    __gmp_temp<T> temp1(expr.val1, p);
    expr.val2.eval(p);
    Op::eval(p, temp1.__get_mp(), p);
  }
  const val1_type & get_val1() const { return expr.val1; }
  const val2_type & get_val2() const { return expr.val2; }
  mp_bitcnt_t get_prec() const
  {
    mp_bitcnt_t prec1 = expr.val1.get_prec(),
      prec2 = expr.val2.get_prec();
    return (prec1 > prec2) ? prec1 : prec2;
  }
};

template <class T, class U, class V, class Op>
class __gmp_expr
<T, __gmp_binary_expr<__gmp_expr<T, U>, __gmp_expr<T, V>, Op> >
{
private:
  typedef __gmp_expr<T, U> val1_type;
  typedef __gmp_expr<T, V> val2_type;

  __gmp_binary_expr<val1_type, val2_type, Op> expr;
public:
  __gmp_expr(const val1_type &val1, const val2_type &val2)
    : expr(val1, val2) { }
  void eval(typename __gmp_resolve_expr<T>::ptr_type p) const
  {
    __gmp_temp<T> temp2(expr.val2, p);
    expr.val1.eval(p);
    Op::eval(p, p, temp2.__get_mp());
  }
  const val1_type & get_val1() const { return expr.val1; }
  const val2_type & get_val2() const { return expr.val2; }
  mp_bitcnt_t get_prec() const
  {
    mp_bitcnt_t prec1 = expr.val1.get_prec(),
      prec2 = expr.val2.get_prec();
    return (prec1 > prec2) ? prec1 : prec2;
  }
};


/**************** Special cases ****************/

/* Some operations (i.e., add and subtract) with mixed mpz/mpq arguments
   can be done directly without first converting the mpz to mpq.
   Appropriate specializations of __gmp_expr are required. */


#define __GMPZQ_DEFINE_EXPR(eval_fun)                                       \
                                                                            \
template <>                                                                 \
class __gmp_expr<mpq_t, __gmp_binary_expr<mpz_class, mpq_class, eval_fun> > \
{                                                                           \
private:                                                                    \
  typedef mpz_class val1_type;                                              \
  typedef mpq_class val2_type;                                              \
                                                                            \
  __gmp_binary_expr<val1_type, val2_type, eval_fun> expr;                   \
public:                                                                     \
  __gmp_expr(const val1_type &val1, const val2_type &val2)                  \
    : expr(val1, val2) { }                                                  \
  void eval(mpq_ptr q) const                                                \
  { eval_fun::eval(q, expr.val1.get_mpz_t(), expr.val2.get_mpq_t()); }      \
  const val1_type & get_val1() const { return expr.val1; }                  \
  const val2_type & get_val2() const { return expr.val2; }                  \
  mp_bitcnt_t get_prec() const { return mpf_get_default_prec(); }           \
};                                                                          \
                                                                            \
template <>                                                                 \
class __gmp_expr<mpq_t, __gmp_binary_expr<mpq_class, mpz_class, eval_fun> > \
{                                                                           \
private:                                                                    \
  typedef mpq_class val1_type;                                              \
  typedef mpz_class val2_type;                                              \
                                                                            \
  __gmp_binary_expr<val1_type, val2_type, eval_fun> expr;                   \
public:                                                                     \
  __gmp_expr(const val1_type &val1, const val2_type &val2)                  \
    : expr(val1, val2) { }                                                  \
  void eval(mpq_ptr q) const                                                \
  { eval_fun::eval(q, expr.val1.get_mpq_t(), expr.val2.get_mpz_t()); }      \
  const val1_type & get_val1() const { return expr.val1; }                  \
  const val2_type & get_val2() const { return expr.val2; }                  \
  mp_bitcnt_t get_prec() const { return mpf_get_default_prec(); }           \
};                                                                          \
                                                                            \
template <class T>                                                          \
class __gmp_expr                                                            \
<mpq_t, __gmp_binary_expr<mpz_class, __gmp_expr<mpq_t, T>, eval_fun> >      \
{                                                                           \
private:                                                                    \
  typedef mpz_class val1_type;                                              \
  typedef __gmp_expr<mpq_t, T> val2_type;                                   \
                                                                            \
  __gmp_binary_expr<val1_type, val2_type, eval_fun> expr;                   \
public:                                                                     \
  __gmp_expr(const val1_type &val1, const val2_type &val2)                  \
    : expr(val1, val2) { }                                                  \
  void eval(mpq_ptr q) const                                                \
  {                                                                         \
    mpq_class temp(expr.val2);                                              \
    eval_fun::eval(q, expr.val1.get_mpz_t(), temp.get_mpq_t());             \
  }                                                                         \
  const val1_type & get_val1() const { return expr.val1; }                  \
  const val2_type & get_val2() const { return expr.val2; }                  \
  mp_bitcnt_t get_prec() const { return mpf_get_default_prec(); }           \
};                                                                          \
                                                                            \
template <class T>                                                          \
class __gmp_expr                                                            \
<mpq_t, __gmp_binary_expr<mpq_class, __gmp_expr<mpz_t, T>, eval_fun> >      \
{                                                                           \
private:                                                                    \
  typedef mpq_class val1_type;                                              \
  typedef __gmp_expr<mpz_t, T> val2_type;                                   \
                                                                            \
  __gmp_binary_expr<val1_type, val2_type, eval_fun> expr;                   \
public:                                                                     \
  __gmp_expr(const val1_type &val1, const val2_type &val2)                  \
    : expr(val1, val2) { }                                                  \
  void eval(mpq_ptr q) const                                                \
  {                                                                         \
    mpz_class temp(expr.val2);                                              \
    eval_fun::eval(q, expr.val1.get_mpq_t(), temp.get_mpz_t());             \
  }                                                                         \
  const val1_type & get_val1() const { return expr.val1; }                  \
  const val2_type & get_val2() const { return expr.val2; }                  \
  mp_bitcnt_t get_prec() const { return mpf_get_default_prec(); }           \
};                                                                          \
                                                                            \
template <class T>                                                          \
class __gmp_expr                                                            \
<mpq_t, __gmp_binary_expr<__gmp_expr<mpz_t, T>, mpq_class, eval_fun> >      \
{                                                                           \
private:                                                                    \
  typedef __gmp_expr<mpz_t, T> val1_type;                                   \
  typedef mpq_class val2_type;                                              \
                                                                            \
  __gmp_binary_expr<val1_type, val2_type, eval_fun> expr;                   \
public:                                                                     \
  __gmp_expr(const val1_type &val1, const val2_type &val2)                  \
    : expr(val1, val2) { }                                                  \
  void eval(mpq_ptr q) const                                                \
  {                                                                         \
    mpz_class temp(expr.val1);                                              \
    eval_fun::eval(q, temp.get_mpz_t(), expr.val2.get_mpq_t());             \
  }                                                                         \
  const val1_type & get_val1() const { return expr.val1; }                  \
  const val2_type & get_val2() const { return expr.val2; }                  \
  mp_bitcnt_t get_prec() const { return mpf_get_default_prec(); }           \
};                                                                          \
                                                                            \
template <class T>                                                          \
class __gmp_expr                                                            \
<mpq_t, __gmp_binary_expr<__gmp_expr<mpq_t, T>, mpz_class, eval_fun> >      \
{                                                                           \
private:                                                                    \
  typedef __gmp_expr<mpq_t, T> val1_type;                                   \
  typedef mpz_class val2_type;                                              \
                                                                            \
  __gmp_binary_expr<val1_type, val2_type, eval_fun> expr;                   \
public:                                                                     \
  __gmp_expr(const val1_type &val1, const val2_type &val2)                  \
    : expr(val1, val2) { }                                                  \
  void eval(mpq_ptr q) const                                                \
  {                                                                         \
    mpq_class temp(expr.val1);                                              \
    eval_fun::eval(q, temp.get_mpq_t(), expr.val2.get_mpz_t());             \
  }                                                                         \
  const val1_type & get_val1() const { return expr.val1; }                  \
  const val2_type & get_val2() const { return expr.val2; }                  \
  mp_bitcnt_t get_prec() const { return mpf_get_default_prec(); }           \
};                                                                          \
                                                                            \
template <class T, class U>                                                 \
class __gmp_expr<mpq_t, __gmp_binary_expr                                   \
<__gmp_expr<mpz_t, T>, __gmp_expr<mpq_t, U>, eval_fun> >                    \
{                                                                           \
private:                                                                    \
  typedef __gmp_expr<mpz_t, T> val1_type;                                   \
  typedef __gmp_expr<mpq_t, U> val2_type;                                   \
                                                                            \
  __gmp_binary_expr<val1_type, val2_type, eval_fun> expr;                   \
public:                                                                     \
  __gmp_expr(const val1_type &val1, const val2_type &val2)                  \
    : expr(val1, val2) { }                                                  \
  void eval(mpq_ptr q) const                                                \
  {                                                                         \
    mpz_class temp1(expr.val1);                                             \
    expr.val2.eval(q);                                                      \
    eval_fun::eval(q, temp1.get_mpz_t(), q);                                \
  }                                                                         \
  const val1_type & get_val1() const { return expr.val1; }                  \
  const val2_type & get_val2() const { return expr.val2; }                  \
  mp_bitcnt_t get_prec() const { return mpf_get_default_prec(); }           \
};                                                                          \
                                                                            \
template <class T, class U>                                                 \
class __gmp_expr<mpq_t, __gmp_binary_expr                                   \
<__gmp_expr<mpq_t, T>, __gmp_expr<mpz_t, U>, eval_fun> >                    \
{                                                                           \
private:                                                                    \
  typedef __gmp_expr<mpq_t, T> val1_type;                                   \
  typedef __gmp_expr<mpz_t, U> val2_type;                                   \
                                                                            \
  __gmp_binary_expr<val1_type, val2_type, eval_fun> expr;                   \
public:                                                                     \
  __gmp_expr(const val1_type &val1, const val2_type &val2)                  \
    : expr(val1, val2) { }                                                  \
  void eval(mpq_ptr q) const                                                \
  {                                                                         \
    mpz_class temp2(expr.val2);                                             \
    expr.val1.eval(q);                                             \
    eval_fun::eval(q, q, temp2.get_mpz_t());                \
  }                                                                         \
  const val1_type & get_val1() const { return expr.val1; }                  \
  const val2_type & get_val2() const { return expr.val2; }                  \
  mp_bitcnt_t get_prec() const { return mpf_get_default_prec(); }           \
};


__GMPZQ_DEFINE_EXPR(__gmp_binary_plus)
__GMPZQ_DEFINE_EXPR(__gmp_binary_minus)



/**************** Macros for defining functions ****************/
/* Results of operators and functions are instances of __gmp_expr<T, U>.
   T determines the numerical type of the expression: it can be either
   mpz_t, mpq_t, or mpf_t.  When the arguments of a binary
   expression have different numerical types, __gmp_resolve_expr is used
   to determine the "larger" type.
   U is either __gmp_unary_expr<V, Op> or __gmp_binary_expr<V, W, Op>,
   where V and W are the arguments' types -- they can in turn be
   expressions, thus allowing to build compound expressions to any
   degree of complexity.
   Op is a function object that must have an eval() method accepting
   appropriate arguments.
   Actual evaluation of a __gmp_expr<T, U> object is done when it gets
   assigned to an mp*_class ("lazy" evaluation): this is done by calling
   its eval() method. */


// non-member unary operators and functions

#define __GMP_DEFINE_UNARY_FUNCTION(fun, eval_fun)                           \
                                                                             \
template <class T, class U>                                                  \
inline __gmp_expr<T, __gmp_unary_expr<__gmp_expr<T, U>, eval_fun> >          \
fun(const __gmp_expr<T, U> &expr)                                            \
{                                                                            \
  return __gmp_expr<T, __gmp_unary_expr<__gmp_expr<T, U>, eval_fun> >(expr); \
}

// variant that only works for one of { mpz, mpq, mpf }

#define __GMP_DEFINE_UNARY_FUNCTION_1(T, fun, eval_fun)                      \
                                                                             \
template <class U>                                                           \
inline __gmp_expr<T, __gmp_unary_expr<__gmp_expr<T, U>, eval_fun> >          \
fun(const __gmp_expr<T, U> &expr)                                            \
{                                                                            \
  return __gmp_expr<T, __gmp_unary_expr<__gmp_expr<T, U>, eval_fun> >(expr); \
}

#define __GMP_DEFINE_UNARY_TYPE_FUNCTION(type, fun, eval_fun) \
                                                              \
template <class T, class U>                                   \
inline type fun(const __gmp_expr<T, U> &expr)                 \
{                                                             \
  __gmp_expr<T, T> const& temp(expr); \
  return eval_fun::eval(temp.__get_mp());                     \
}


// non-member binary operators and functions

#define __GMPP_DEFINE_BINARY_FUNCTION(fun, eval_fun)                   \
                                                                       \
template <class T, class U, class V, class W>                          \
inline __gmp_expr<typename __gmp_resolve_expr<T, V>::value_type,       \
__gmp_binary_expr<__gmp_expr<T, U>, __gmp_expr<V, W>, eval_fun> >      \
fun(const __gmp_expr<T, U> &expr1, const __gmp_expr<V, W> &expr2)      \
{                                                                      \
  return __gmp_expr<typename __gmp_resolve_expr<T, V>::value_type,     \
     __gmp_binary_expr<__gmp_expr<T, U>, __gmp_expr<V, W>, eval_fun> > \
    (expr1, expr2);                                                    \
}

#define __GMPNN_DEFINE_BINARY_FUNCTION(fun, eval_fun, type, bigtype)       \
                                                                           \
template <class T, class U>                                                \
inline __gmp_expr                                                          \
<T, __gmp_binary_expr<__gmp_expr<T, U>, bigtype, eval_fun> >               \
fun(const __gmp_expr<T, U> &expr, type t)                                  \
{                                                                          \
  return __gmp_expr                                                        \
    <T, __gmp_binary_expr<__gmp_expr<T, U>, bigtype, eval_fun> >(expr, t); \
}                                                                          \
                                                                           \
template <class T, class U>                                                \
inline __gmp_expr                                                          \
<T, __gmp_binary_expr<bigtype, __gmp_expr<T, U>, eval_fun> >               \
fun(type t, const __gmp_expr<T, U> &expr)                                  \
{                                                                          \
  return __gmp_expr                                                        \
    <T, __gmp_binary_expr<bigtype, __gmp_expr<T, U>, eval_fun> >(t, expr); \
}

#define __GMPNS_DEFINE_BINARY_FUNCTION(fun, eval_fun, type)          \
__GMPNN_DEFINE_BINARY_FUNCTION(fun, eval_fun, type, signed long int)

#define __GMPNU_DEFINE_BINARY_FUNCTION(fun, eval_fun, type)            \
__GMPNN_DEFINE_BINARY_FUNCTION(fun, eval_fun, type, unsigned long int)

#define __GMPND_DEFINE_BINARY_FUNCTION(fun, eval_fun, type) \
__GMPNN_DEFINE_BINARY_FUNCTION(fun, eval_fun, type, double)

#define __GMPNLD_DEFINE_BINARY_FUNCTION(fun, eval_fun, type)     \
__GMPNN_DEFINE_BINARY_FUNCTION(fun, eval_fun, type, long double)

#define __GMPN_DEFINE_BINARY_FUNCTION(fun, eval_fun)              \
__GMPNS_DEFINE_BINARY_FUNCTION(fun, eval_fun, signed char)        \
__GMPNU_DEFINE_BINARY_FUNCTION(fun, eval_fun, unsigned char)      \
__GMPNS_DEFINE_BINARY_FUNCTION(fun, eval_fun, signed int)         \
__GMPNU_DEFINE_BINARY_FUNCTION(fun, eval_fun, unsigned int)       \
__GMPNS_DEFINE_BINARY_FUNCTION(fun, eval_fun, signed short int)   \
__GMPNU_DEFINE_BINARY_FUNCTION(fun, eval_fun, unsigned short int) \
__GMPNS_DEFINE_BINARY_FUNCTION(fun, eval_fun, signed long int)    \
__GMPNU_DEFINE_BINARY_FUNCTION(fun, eval_fun, unsigned long int)  \
__GMPND_DEFINE_BINARY_FUNCTION(fun, eval_fun, float)              \
__GMPND_DEFINE_BINARY_FUNCTION(fun, eval_fun, double)             \
/* __GMPNLD_DEFINE_BINARY_FUNCTION(fun, eval_fun, long double) */

#define __GMP_DEFINE_BINARY_FUNCTION(fun, eval_fun) \
__GMPP_DEFINE_BINARY_FUNCTION(fun, eval_fun)        \
__GMPN_DEFINE_BINARY_FUNCTION(fun, eval_fun)

// variant that only works for one of { mpz, mpq, mpf }

#define __GMPP_DEFINE_BINARY_FUNCTION_1(T, fun, eval_fun)              \
                                                                       \
template <class U, class W>                                            \
inline __gmp_expr<T,                                                   \
__gmp_binary_expr<__gmp_expr<T, U>, __gmp_expr<T, W>, eval_fun> >      \
fun(const __gmp_expr<T, U> &expr1, const __gmp_expr<T, W> &expr2)      \
{                                                                      \
  return __gmp_expr<T,                                                 \
     __gmp_binary_expr<__gmp_expr<T, U>, __gmp_expr<T, W>, eval_fun> > \
    (expr1, expr2);                                                    \
}

#define __GMPNN_DEFINE_BINARY_FUNCTION_1(T, fun, eval_fun, type, bigtype)  \
                                                                           \
template <class U>                                                         \
inline __gmp_expr                                                          \
<T, __gmp_binary_expr<__gmp_expr<T, U>, bigtype, eval_fun> >               \
fun(const __gmp_expr<T, U> &expr, type t)                                  \
{                                                                          \
  return __gmp_expr                                                        \
    <T, __gmp_binary_expr<__gmp_expr<T, U>, bigtype, eval_fun> >(expr, t); \
}                                                                          \
                                                                           \
template <class U>                                                         \
inline __gmp_expr                                                          \
<T, __gmp_binary_expr<bigtype, __gmp_expr<T, U>, eval_fun> >               \
fun(type t, const __gmp_expr<T, U> &expr)                                  \
{                                                                          \
  return __gmp_expr                                                        \
    <T, __gmp_binary_expr<bigtype, __gmp_expr<T, U>, eval_fun> >(t, expr); \
}

#define __GMPNS_DEFINE_BINARY_FUNCTION_1(T, fun, eval_fun, type)          \
__GMPNN_DEFINE_BINARY_FUNCTION_1(T, fun, eval_fun, type, signed long int)

#define __GMPNU_DEFINE_BINARY_FUNCTION_1(T, fun, eval_fun, type)          \
__GMPNN_DEFINE_BINARY_FUNCTION_1(T, fun, eval_fun, type, unsigned long int)

#define __GMPND_DEFINE_BINARY_FUNCTION_1(T, fun, eval_fun, type) \
__GMPNN_DEFINE_BINARY_FUNCTION_1(T, fun, eval_fun, type, double)

#define __GMPNLD_DEFINE_BINARY_FUNCTION_1(T, fun, eval_fun, type)     \
__GMPNN_DEFINE_BINARY_FUNCTION_1(T, fun, eval_fun, type, long double)

#define __GMPN_DEFINE_BINARY_FUNCTION_1(T, fun, eval_fun)              \
__GMPNS_DEFINE_BINARY_FUNCTION_1(T, fun, eval_fun, signed char)        \
__GMPNU_DEFINE_BINARY_FUNCTION_1(T, fun, eval_fun, unsigned char)      \
__GMPNS_DEFINE_BINARY_FUNCTION_1(T, fun, eval_fun, signed int)         \
__GMPNU_DEFINE_BINARY_FUNCTION_1(T, fun, eval_fun, unsigned int)       \
__GMPNS_DEFINE_BINARY_FUNCTION_1(T, fun, eval_fun, signed short int)   \
__GMPNU_DEFINE_BINARY_FUNCTION_1(T, fun, eval_fun, unsigned short int) \
__GMPNS_DEFINE_BINARY_FUNCTION_1(T, fun, eval_fun, signed long int)    \
__GMPNU_DEFINE_BINARY_FUNCTION_1(T, fun, eval_fun, unsigned long int)  \
__GMPND_DEFINE_BINARY_FUNCTION_1(T, fun, eval_fun, float)              \
__GMPND_DEFINE_BINARY_FUNCTION_1(T, fun, eval_fun, double)             \
/* __GMPNLD_DEFINE_BINARY_FUNCTION_1(T, fun, eval_fun, long double) */

#define __GMP_DEFINE_BINARY_FUNCTION_1(T, fun, eval_fun) \
__GMPP_DEFINE_BINARY_FUNCTION_1(T, fun, eval_fun)        \
__GMPN_DEFINE_BINARY_FUNCTION_1(T, fun, eval_fun)


#define __GMP_DEFINE_BINARY_FUNCTION_UI(fun, eval_fun)                 \
                                                                       \
template <class T, class U>                                            \
inline __gmp_expr                                                      \
<T, __gmp_binary_expr<__gmp_expr<T, U>, mp_bitcnt_t, eval_fun> > \
fun(const __gmp_expr<T, U> &expr, mp_bitcnt_t l)                 \
{                                                                      \
  return __gmp_expr<T, __gmp_binary_expr                               \
    <__gmp_expr<T, U>, mp_bitcnt_t, eval_fun> >(expr, l);        \
}


#define __GMPP_DEFINE_BINARY_TYPE_FUNCTION(type, fun, eval_fun)         \
                                                                        \
template <class T, class U, class V, class W>                           \
inline type fun(const __gmp_expr<T, U> &expr1,                          \
		const __gmp_expr<V, W> &expr2)                          \
{                                                                       \
  __gmp_expr<T, T> const& temp1(expr1);                                 \
  __gmp_expr<V, V> const& temp2(expr2);                                 \
  return eval_fun::eval(temp1.__get_mp(), temp2.__get_mp());            \
}

#define __GMPNN_DEFINE_BINARY_TYPE_FUNCTION(type, fun, eval_fun,   \
					    type2, bigtype)        \
                                                                   \
template <class T, class U>                                        \
inline type fun(const __gmp_expr<T, U> &expr, type2 t)             \
{                                                                  \
  __gmp_expr<T, T> const& temp(expr);      \
  return eval_fun::eval(temp.__get_mp(), static_cast<bigtype>(t)); \
}                                                                  \
                                                                   \
template <class T, class U>                                        \
inline type fun(type2 t, const __gmp_expr<T, U> &expr)             \
{                                                                  \
  __gmp_expr<T, T> const& temp(expr);      \
  return eval_fun::eval(static_cast<bigtype>(t), temp.__get_mp()); \
}

#define __GMPNS_DEFINE_BINARY_TYPE_FUNCTION(type, fun, eval_fun, type2) \
__GMPNN_DEFINE_BINARY_TYPE_FUNCTION(type, fun, eval_fun,                \
				    type2, signed long int)

#define __GMPNU_DEFINE_BINARY_TYPE_FUNCTION(type, fun, eval_fun, type2) \
__GMPNN_DEFINE_BINARY_TYPE_FUNCTION(type, fun, eval_fun,                \
				    type2, unsigned long int)

#define __GMPND_DEFINE_BINARY_TYPE_FUNCTION(type, fun, eval_fun, type2) \
__GMPNN_DEFINE_BINARY_TYPE_FUNCTION(type, fun, eval_fun, type2, double)

#define __GMPNLD_DEFINE_BINARY_TYPE_FUNCTION(type, fun, eval_fun, type2)     \
__GMPNN_DEFINE_BINARY_TYPE_FUNCTION(type, fun, eval_fun, type2, long double)

#define __GMPN_DEFINE_BINARY_TYPE_FUNCTION(type, fun, eval_fun)              \
__GMPNS_DEFINE_BINARY_TYPE_FUNCTION(type, fun, eval_fun, signed char)        \
__GMPNU_DEFINE_BINARY_TYPE_FUNCTION(type, fun, eval_fun, unsigned char)      \
__GMPNS_DEFINE_BINARY_TYPE_FUNCTION(type, fun, eval_fun, signed int)         \
__GMPNU_DEFINE_BINARY_TYPE_FUNCTION(type, fun, eval_fun, unsigned int)       \
__GMPNS_DEFINE_BINARY_TYPE_FUNCTION(type, fun, eval_fun, signed short int)   \
__GMPNU_DEFINE_BINARY_TYPE_FUNCTION(type, fun, eval_fun, unsigned short int) \
__GMPNS_DEFINE_BINARY_TYPE_FUNCTION(type, fun, eval_fun, signed long int)    \
__GMPNU_DEFINE_BINARY_TYPE_FUNCTION(type, fun, eval_fun, unsigned long int)  \
__GMPND_DEFINE_BINARY_TYPE_FUNCTION(type, fun, eval_fun, float)              \
__GMPND_DEFINE_BINARY_TYPE_FUNCTION(type, fun, eval_fun, double)             \
/* __GMPNLD_DEFINE_BINARY_TYPE_FUNCTION(type, fun, eval_fun, long double) */

#define __GMP_DEFINE_BINARY_TYPE_FUNCTION(type, fun, eval_fun) \
__GMPP_DEFINE_BINARY_TYPE_FUNCTION(type, fun, eval_fun)        \
__GMPN_DEFINE_BINARY_TYPE_FUNCTION(type, fun, eval_fun)


// member operators

#define __GMPP_DEFINE_COMPOUND_OPERATOR(type, fun, eval_fun)                 \
                                                                             \
template <class T, class U>                                                  \
inline type##_class & type##_class::fun(const __gmp_expr<T, U> &expr)        \
{                                                                            \
  __gmp_set_expr(mp, __gmp_expr<type##_t, __gmp_binary_expr                  \
		 <type##_class, __gmp_expr<T, U>, eval_fun> >(*this, expr)); \
  return *this;                                                              \
}

#define __GMPNN_DEFINE_COMPOUND_OPERATOR(type, fun, eval_fun,    \
					 type2, bigtype)         \
                                                                 \
inline type##_class & type##_class::fun(type2 t)                 \
{                                                                \
  __gmp_set_expr(mp, __gmp_expr<type##_t, __gmp_binary_expr      \
		 <type##_class, bigtype, eval_fun> >(*this, t)); \
  return *this;                                                  \
}

#define __GMPNS_DEFINE_COMPOUND_OPERATOR(type, fun, eval_fun, type2) \
__GMPNN_DEFINE_COMPOUND_OPERATOR(type, fun, eval_fun,                \
				 type2, signed long int)

#define __GMPNU_DEFINE_COMPOUND_OPERATOR(type, fun, eval_fun, type2) \
__GMPNN_DEFINE_COMPOUND_OPERATOR(type, fun, eval_fun,                \
				 type2, unsigned long int)

#define __GMPND_DEFINE_COMPOUND_OPERATOR(type, fun, eval_fun, type2) \
__GMPNN_DEFINE_COMPOUND_OPERATOR(type, fun, eval_fun, type2, double)

#define __GMPNLD_DEFINE_COMPOUND_OPERATOR(type, fun, eval_fun, type2)     \
__GMPNN_DEFINE_COMPOUND_OPERATOR(type, fun, eval_fun, type2, long double)

#define __GMPN_DEFINE_COMPOUND_OPERATOR(type, fun, eval_fun)              \
__GMPNS_DEFINE_COMPOUND_OPERATOR(type, fun, eval_fun, signed char)        \
__GMPNU_DEFINE_COMPOUND_OPERATOR(type, fun, eval_fun, unsigned char)      \
__GMPNS_DEFINE_COMPOUND_OPERATOR(type, fun, eval_fun, signed int)         \
__GMPNU_DEFINE_COMPOUND_OPERATOR(type, fun, eval_fun, unsigned int)       \
__GMPNS_DEFINE_COMPOUND_OPERATOR(type, fun, eval_fun, signed short int)   \
__GMPNU_DEFINE_COMPOUND_OPERATOR(type, fun, eval_fun, unsigned short int) \
__GMPNS_DEFINE_COMPOUND_OPERATOR(type, fun, eval_fun, signed long int)    \
__GMPNU_DEFINE_COMPOUND_OPERATOR(type, fun, eval_fun, unsigned long int)  \
__GMPND_DEFINE_COMPOUND_OPERATOR(type, fun, eval_fun, float)              \
__GMPND_DEFINE_COMPOUND_OPERATOR(type, fun, eval_fun, double)             \
/* __GMPNLD_DEFINE_COMPOUND_OPERATOR(type, fun, eval_fun, long double) */

#define __GMP_DEFINE_COMPOUND_OPERATOR(type, fun, eval_fun) \
__GMPP_DEFINE_COMPOUND_OPERATOR(type, fun, eval_fun)        \
__GMPN_DEFINE_COMPOUND_OPERATOR(type, fun, eval_fun)

#define __GMPZ_DEFINE_COMPOUND_OPERATOR(fun, eval_fun) \
__GMP_DEFINE_COMPOUND_OPERATOR(mpz, fun, eval_fun)

#define __GMPQ_DEFINE_COMPOUND_OPERATOR(fun, eval_fun) \
__GMP_DEFINE_COMPOUND_OPERATOR(mpq, fun, eval_fun)

#define __GMPF_DEFINE_COMPOUND_OPERATOR(fun, eval_fun) \
__GMP_DEFINE_COMPOUND_OPERATOR(mpf, fun, eval_fun)



#define __GMP_DEFINE_COMPOUND_OPERATOR_UI(type, fun, eval_fun)  \
                                                                \
inline type##_class & type##_class::fun(mp_bitcnt_t l)    \
{                                                               \
  __gmp_set_expr(mp, __gmp_expr<type##_t, __gmp_binary_expr     \
    <type##_class, mp_bitcnt_t, eval_fun> >(*this, l));   \
  return *this;                                                 \
}

#define __GMPZ_DEFINE_COMPOUND_OPERATOR_UI(fun, eval_fun) \
__GMP_DEFINE_COMPOUND_OPERATOR_UI(mpz, fun, eval_fun)

#define __GMPQ_DEFINE_COMPOUND_OPERATOR_UI(fun, eval_fun) \
__GMP_DEFINE_COMPOUND_OPERATOR_UI(mpq, fun, eval_fun)

#define __GMPF_DEFINE_COMPOUND_OPERATOR_UI(fun, eval_fun) \
__GMP_DEFINE_COMPOUND_OPERATOR_UI(mpf, fun, eval_fun)



#define __GMP_DEFINE_INCREMENT_OPERATOR(type, fun, eval_fun) \
                                                             \
inline type##_class & type##_class::fun()                    \
{                                                            \
  eval_fun::eval(mp);                                        \
  return *this;                                              \
}                                                            \
                                                             \
inline type##_class type##_class::fun(int)                   \
{                                                            \
  type##_class temp(*this);                                  \
  eval_fun::eval(mp);                                        \
  return temp;                                               \
}

#define __GMPZ_DEFINE_INCREMENT_OPERATOR(fun, eval_fun) \
__GMP_DEFINE_INCREMENT_OPERATOR(mpz, fun, eval_fun)

#define __GMPQ_DEFINE_INCREMENT_OPERATOR(fun, eval_fun) \
__GMP_DEFINE_INCREMENT_OPERATOR(mpq, fun, eval_fun)

#define __GMPF_DEFINE_INCREMENT_OPERATOR(fun, eval_fun) \
__GMP_DEFINE_INCREMENT_OPERATOR(mpf, fun, eval_fun)


#define __GMPP_DEFINE_UNARY_STATIC_MEMFUN(T, fun, eval_fun)                  \
template <class U>                                                           \
__gmp_expr<T, __gmp_unary_expr<__gmp_expr<T, U>, eval_fun> >          \
fun(const __gmp_expr<T, U> &expr)                                            \
{                                                                            \
  return __gmp_expr<T, __gmp_unary_expr<__gmp_expr<T, U>, eval_fun> >(expr); \
}

#define __GMPNN_DEFINE_UNARY_STATIC_MEMFUN(T, fun, eval_fun, type, bigtype)  \
inline __gmp_expr<T, __gmp_unary_expr<bigtype, eval_fun> >                   \
fun(type expr)                                                               \
{                                                                            \
  return __gmp_expr<T, __gmp_unary_expr<bigtype, eval_fun> >(expr);          \
}

#define __GMPNS_DEFINE_UNARY_STATIC_MEMFUN(T, fun, eval_fun, type)  \
__GMPNN_DEFINE_UNARY_STATIC_MEMFUN(T, fun, eval_fun, type, signed long)
#define __GMPNU_DEFINE_UNARY_STATIC_MEMFUN(T, fun, eval_fun, type)  \
__GMPNN_DEFINE_UNARY_STATIC_MEMFUN(T, fun, eval_fun, type, unsigned long)
#define __GMPND_DEFINE_UNARY_STATIC_MEMFUN(T, fun, eval_fun, type)  \
__GMPNN_DEFINE_UNARY_STATIC_MEMFUN(T, fun, eval_fun, type, double)

#define __GMPN_DEFINE_UNARY_STATIC_MEMFUN(T, fun, eval_fun)                 \
__GMPNS_DEFINE_UNARY_STATIC_MEMFUN(T, fun, eval_fun, signed char)           \
__GMPNU_DEFINE_UNARY_STATIC_MEMFUN(T, fun, eval_fun, unsigned char)         \
__GMPNS_DEFINE_UNARY_STATIC_MEMFUN(T, fun, eval_fun, signed int)            \
__GMPNU_DEFINE_UNARY_STATIC_MEMFUN(T, fun, eval_fun, unsigned int)          \
__GMPNS_DEFINE_UNARY_STATIC_MEMFUN(T, fun, eval_fun, signed short int)      \
__GMPNU_DEFINE_UNARY_STATIC_MEMFUN(T, fun, eval_fun, unsigned short int)    \
__GMPNS_DEFINE_UNARY_STATIC_MEMFUN(T, fun, eval_fun, signed long int)       \
__GMPNU_DEFINE_UNARY_STATIC_MEMFUN(T, fun, eval_fun, unsigned long int)     \
__GMPND_DEFINE_UNARY_STATIC_MEMFUN(T, fun, eval_fun, float)                 \
__GMPND_DEFINE_UNARY_STATIC_MEMFUN(T, fun, eval_fun, double)                \

#define __GMP_DEFINE_UNARY_STATIC_MEMFUN(T, fun, eval_fun)                  \
__GMPP_DEFINE_UNARY_STATIC_MEMFUN(T, fun, eval_fun)                         \
__GMPN_DEFINE_UNARY_STATIC_MEMFUN(T, fun, eval_fun)                         \


/**************** Arithmetic operators and functions ****************/

// non-member operators and functions

__GMP_DEFINE_UNARY_FUNCTION(operator+, __gmp_unary_plus)
__GMP_DEFINE_UNARY_FUNCTION(operator-, __gmp_unary_minus)
__GMP_DEFINE_UNARY_FUNCTION_1(mpz_t, operator~, __gmp_unary_com)

__GMP_DEFINE_BINARY_FUNCTION(operator+, __gmp_binary_plus)
__GMP_DEFINE_BINARY_FUNCTION(operator-, __gmp_binary_minus)
__GMP_DEFINE_BINARY_FUNCTION(operator*, __gmp_binary_multiplies)
__GMP_DEFINE_BINARY_FUNCTION(operator/, __gmp_binary_divides)
__GMP_DEFINE_BINARY_FUNCTION_1(mpz_t, operator%, __gmp_binary_modulus)
__GMP_DEFINE_BINARY_FUNCTION_1(mpz_t, operator&, __gmp_binary_and)
__GMP_DEFINE_BINARY_FUNCTION_1(mpz_t, operator|, __gmp_binary_ior)
__GMP_DEFINE_BINARY_FUNCTION_1(mpz_t, operator^, __gmp_binary_xor)

__GMP_DEFINE_BINARY_FUNCTION_UI(operator<<, __gmp_binary_lshift)
__GMP_DEFINE_BINARY_FUNCTION_UI(operator>>, __gmp_binary_rshift)

__GMP_DEFINE_BINARY_TYPE_FUNCTION(bool, operator==, __gmp_binary_equal)
__GMP_DEFINE_BINARY_TYPE_FUNCTION(bool, operator!=, ! __gmp_binary_equal)
__GMP_DEFINE_BINARY_TYPE_FUNCTION(bool, operator<, __gmp_binary_less)
__GMP_DEFINE_BINARY_TYPE_FUNCTION(bool, operator<=, ! __gmp_binary_greater)
__GMP_DEFINE_BINARY_TYPE_FUNCTION(bool, operator>, __gmp_binary_greater)
__GMP_DEFINE_BINARY_TYPE_FUNCTION(bool, operator>=, ! __gmp_binary_less)

__GMP_DEFINE_UNARY_FUNCTION(abs, __gmp_abs_function)
__GMP_DEFINE_UNARY_FUNCTION_1(mpf_t, trunc, __gmp_trunc_function)
__GMP_DEFINE_UNARY_FUNCTION_1(mpf_t, floor, __gmp_floor_function)
__GMP_DEFINE_UNARY_FUNCTION_1(mpf_t, ceil, __gmp_ceil_function)
__GMP_DEFINE_UNARY_FUNCTION_1(mpf_t, sqrt, __gmp_sqrt_function)
__GMP_DEFINE_UNARY_FUNCTION_1(mpz_t, sqrt, __gmp_sqrt_function)
__GMP_DEFINE_UNARY_FUNCTION_1(mpz_t, factorial, __gmp_fac_function)
__GMP_DEFINE_UNARY_FUNCTION_1(mpz_t, primorial, __gmp_primorial_function)
__GMP_DEFINE_UNARY_FUNCTION_1(mpz_t, fibonacci, __gmp_fib_function)
__GMP_DEFINE_BINARY_FUNCTION_1(mpf_t, hypot, __gmp_hypot_function)
__GMP_DEFINE_BINARY_FUNCTION_1(mpz_t, gcd, __gmp_gcd_function)
__GMP_DEFINE_BINARY_FUNCTION_1(mpz_t, lcm, __gmp_lcm_function)

__GMP_DEFINE_UNARY_TYPE_FUNCTION(int, sgn, __gmp_sgn_function)
__GMP_DEFINE_BINARY_TYPE_FUNCTION(int, cmp, __gmp_cmp_function)

template <class T>
void swap(__gmp_expr<T, T>& x, __gmp_expr<T, T>& y) __GMPXX_NOEXCEPT
{ x.swap(y); }

// member operators for mpz_class

__GMPZ_DEFINE_COMPOUND_OPERATOR(operator+=, __gmp_binary_plus)
__GMPZ_DEFINE_COMPOUND_OPERATOR(operator-=, __gmp_binary_minus)
__GMPZ_DEFINE_COMPOUND_OPERATOR(operator*=, __gmp_binary_multiplies)
__GMPZ_DEFINE_COMPOUND_OPERATOR(operator/=, __gmp_binary_divides)
__GMPZ_DEFINE_COMPOUND_OPERATOR(operator%=, __gmp_binary_modulus)

__GMPZ_DEFINE_COMPOUND_OPERATOR(operator&=, __gmp_binary_and)
__GMPZ_DEFINE_COMPOUND_OPERATOR(operator|=, __gmp_binary_ior)
__GMPZ_DEFINE_COMPOUND_OPERATOR(operator^=, __gmp_binary_xor)

__GMPZ_DEFINE_COMPOUND_OPERATOR_UI(operator<<=, __gmp_binary_lshift)
__GMPZ_DEFINE_COMPOUND_OPERATOR_UI(operator>>=, __gmp_binary_rshift)

__GMPZ_DEFINE_INCREMENT_OPERATOR(operator++, __gmp_unary_increment)
__GMPZ_DEFINE_INCREMENT_OPERATOR(operator--, __gmp_unary_decrement)

__GMP_DEFINE_UNARY_STATIC_MEMFUN(mpz_t, mpz_class::factorial, __gmp_fac_function)
__GMP_DEFINE_UNARY_STATIC_MEMFUN(mpz_t, mpz_class::primorial, __gmp_primorial_function)
__GMP_DEFINE_UNARY_STATIC_MEMFUN(mpz_t, mpz_class::fibonacci, __gmp_fib_function)

// member operators for mpq_class

__GMPQ_DEFINE_COMPOUND_OPERATOR(operator+=, __gmp_binary_plus)
__GMPQ_DEFINE_COMPOUND_OPERATOR(operator-=, __gmp_binary_minus)
__GMPQ_DEFINE_COMPOUND_OPERATOR(operator*=, __gmp_binary_multiplies)
__GMPQ_DEFINE_COMPOUND_OPERATOR(operator/=, __gmp_binary_divides)

__GMPQ_DEFINE_COMPOUND_OPERATOR_UI(operator<<=, __gmp_binary_lshift)
__GMPQ_DEFINE_COMPOUND_OPERATOR_UI(operator>>=, __gmp_binary_rshift)

__GMPQ_DEFINE_INCREMENT_OPERATOR(operator++, __gmp_unary_increment)
__GMPQ_DEFINE_INCREMENT_OPERATOR(operator--, __gmp_unary_decrement)

// member operators for mpf_class

__GMPF_DEFINE_COMPOUND_OPERATOR(operator+=, __gmp_binary_plus)
__GMPF_DEFINE_COMPOUND_OPERATOR(operator-=, __gmp_binary_minus)
__GMPF_DEFINE_COMPOUND_OPERATOR(operator*=, __gmp_binary_multiplies)
__GMPF_DEFINE_COMPOUND_OPERATOR(operator/=, __gmp_binary_divides)

__GMPF_DEFINE_COMPOUND_OPERATOR_UI(operator<<=, __gmp_binary_lshift)
__GMPF_DEFINE_COMPOUND_OPERATOR_UI(operator>>=, __gmp_binary_rshift)

__GMPF_DEFINE_INCREMENT_OPERATOR(operator++, __gmp_unary_increment)
__GMPF_DEFINE_INCREMENT_OPERATOR(operator--, __gmp_unary_decrement)



/**************** Class wrapper for gmp_randstate_t ****************/

class __gmp_urandomb_value { };
class __gmp_urandomm_value { };

template <>
class __gmp_expr<mpz_t, __gmp_urandomb_value>
{
private:
  __gmp_randstate_struct *state;
  mp_bitcnt_t bits;
public:
  __gmp_expr(gmp_randstate_t s, mp_bitcnt_t l) : state(s), bits(l) { }
  void eval(mpz_ptr z) const { __gmp_rand_function::eval(z, state, bits); }
  mp_bitcnt_t get_prec() const { return mpf_get_default_prec(); }
};

template <>
class __gmp_expr<mpz_t, __gmp_urandomm_value>
{
private:
  __gmp_randstate_struct *state;
  mpz_class range;
public:
  __gmp_expr(gmp_randstate_t s, const mpz_class &z) : state(s), range(z) { }
  void eval(mpz_ptr z) const
  { __gmp_rand_function::eval(z, state, range.get_mpz_t()); }
  mp_bitcnt_t get_prec() const { return mpf_get_default_prec(); }
};

template <>
class __gmp_expr<mpf_t, __gmp_urandomb_value>
{
private:
  __gmp_randstate_struct *state;
  mp_bitcnt_t bits;
public:
  __gmp_expr(gmp_randstate_t s, mp_bitcnt_t l) : state(s), bits(l) { }
  void eval(mpf_ptr f) const
  {
    __gmp_rand_function::eval(f, state,
	(bits>0) ? bits : mpf_get_prec(f));
  }
  mp_bitcnt_t get_prec() const
  {
    if (bits == 0)
      return mpf_get_default_prec();
    else
      return bits;
  }
};

extern "C" {
  typedef void __gmp_randinit_default_t (gmp_randstate_t);
  typedef void __gmp_randinit_lc_2exp_t (gmp_randstate_t, mpz_srcptr, unsigned long int, mp_bitcnt_t);
  typedef int __gmp_randinit_lc_2exp_size_t (gmp_randstate_t, mp_bitcnt_t);
}

class gmp_randclass
{
private:
  gmp_randstate_t state;

  // copy construction and assignment not allowed
  gmp_randclass(const gmp_randclass &);
  void operator=(const gmp_randclass &);
public:
  // constructors and destructor
  gmp_randclass(gmp_randalg_t alg, unsigned long int size)
  {
    switch (alg)
      {
      case GMP_RAND_ALG_LC: // no other cases for now
      default:
	gmp_randinit(state, alg, size);
	break;
      }
  }

  // gmp_randinit_default
  gmp_randclass(__gmp_randinit_default_t* f) { f(state); }

  // gmp_randinit_lc_2exp
  gmp_randclass(__gmp_randinit_lc_2exp_t* f,
		mpz_class z, unsigned long int l1, mp_bitcnt_t l2)
  { f(state, z.get_mpz_t(), l1, l2); }

  // gmp_randinit_lc_2exp_size
  gmp_randclass(__gmp_randinit_lc_2exp_size_t* f,
		mp_bitcnt_t size)
  {
    if (f (state, size) == 0)
      throw std::length_error ("gmp_randinit_lc_2exp_size");
  }

  ~gmp_randclass() { gmp_randclear(state); }

  // initialize
  void seed(); // choose a random seed some way (?)
  void seed(unsigned long int s) { gmp_randseed_ui(state, s); }
  void seed(const mpz_class &z) { gmp_randseed(state, z.get_mpz_t()); }

  // get random number
  __gmp_expr<mpz_t, __gmp_urandomb_value> get_z_bits(mp_bitcnt_t l)
  { return __gmp_expr<mpz_t, __gmp_urandomb_value>(state, l); }
  __gmp_expr<mpz_t, __gmp_urandomb_value> get_z_bits(const mpz_class &z)
  { return get_z_bits(z.get_ui()); }
  // FIXME: z.get_bitcnt_t() ?

  __gmp_expr<mpz_t, __gmp_urandomm_value> get_z_range(const mpz_class &z)
  { return __gmp_expr<mpz_t, __gmp_urandomm_value>(state, z); }

  __gmp_expr<mpf_t, __gmp_urandomb_value> get_f(mp_bitcnt_t prec = 0)
  { return __gmp_expr<mpf_t, __gmp_urandomb_value>(state, prec); }
};


/**************** Specialize std::numeric_limits ****************/

namespace std {
  template <> class numeric_limits<mpz_class>
  {
  public:
    static const bool is_specialized = true;
    static mpz_class min() { return mpz_class(); }
    static mpz_class max() { return mpz_class(); }
    static mpz_class lowest() { return mpz_class(); }
    static const int digits = 0;
    static const int digits10 = 0;
    static const int max_digits10 = 0;
    static const bool is_signed = true;
    static const bool is_integer = true;
    static const bool is_exact = true;
    static const int radix = 2;
    static mpz_class epsilon() { return mpz_class(); }
    static mpz_class round_error() { return mpz_class(); }
    static const int min_exponent = 0;
    static const int min_exponent10 = 0;
    static const int max_exponent = 0;
    static const int max_exponent10 = 0;
    static const bool has_infinity = false;
    static const bool has_quiet_NaN = false;
    static const bool has_signaling_NaN = false;
    static const float_denorm_style has_denorm = denorm_absent;
    static const bool has_denorm_loss = false;
    static mpz_class infinity() { return mpz_class(); }
    static mpz_class quiet_NaN() { return mpz_class(); }
    static mpz_class signaling_NaN() { return mpz_class(); }
    static mpz_class denorm_min() { return mpz_class(); }
    static const bool is_iec559 = false;
    static const bool is_bounded = false;
    static const bool is_modulo = false;
    static const bool traps = false;
    static const bool tinyness_before = false;
    static const float_round_style round_style = round_toward_zero;
  };

  template <> class numeric_limits<mpq_class>
  {
  public:
    static const bool is_specialized = true;
    static mpq_class min() { return mpq_class(); }
    static mpq_class max() { return mpq_class(); }
    static mpq_class lowest() { return mpq_class(); }
    static const int digits = 0;
    static const int digits10 = 0;
    static const int max_digits10 = 0;
    static const bool is_signed = true;
    static const bool is_integer = false;
    static const bool is_exact = true;
    static const int radix = 2;
    static mpq_class epsilon() { return mpq_class(); }
    static mpq_class round_error() { return mpq_class(); }
    static const int min_exponent = 0;
    static const int min_exponent10 = 0;
    static const int max_exponent = 0;
    static const int max_exponent10 = 0;
    static const bool has_infinity = false;
    static const bool has_quiet_NaN = false;
    static const bool has_signaling_NaN = false;
    static const float_denorm_style has_denorm = denorm_absent;
    static const bool has_denorm_loss = false;
    static mpq_class infinity() { return mpq_class(); }
    static mpq_class quiet_NaN() { return mpq_class(); }
    static mpq_class signaling_NaN() { return mpq_class(); }
    static mpq_class denorm_min() { return mpq_class(); }
    static const bool is_iec559 = false;
    static const bool is_bounded = false;
    static const bool is_modulo = false;
    static const bool traps = false;
    static const bool tinyness_before = false;
    static const float_round_style round_style = round_toward_zero;
  };

  template <> class numeric_limits<mpf_class>
  {
  public:
    static const bool is_specialized = true;
    static mpf_class min() { return mpf_class(); }
    static mpf_class max() { return mpf_class(); }
    static mpf_class lowest() { return mpf_class(); }
    static const int digits = 0;
    static const int digits10 = 0;
    static const int max_digits10 = 0;
    static const bool is_signed = true;
    static const bool is_integer = false;
    static const bool is_exact = false;
    static const int radix = 2;
    static mpf_class epsilon() { return mpf_class(); }
    static mpf_class round_error() { return mpf_class(); }
    static const int min_exponent = 0;
    static const int min_exponent10 = 0;
    static const int max_exponent = 0;
    static const int max_exponent10 = 0;
    static const bool has_infinity = false;
    static const bool has_quiet_NaN = false;
    static const bool has_signaling_NaN = false;
    static const float_denorm_style has_denorm = denorm_absent;
    static const bool has_denorm_loss = false;
    static mpf_class infinity() { return mpf_class(); }
    static mpf_class quiet_NaN() { return mpf_class(); }
    static mpf_class signaling_NaN() { return mpf_class(); }
    static mpf_class denorm_min() { return mpf_class(); }
    static const bool is_iec559 = false;
    static const bool is_bounded = false;
    static const bool is_modulo = false;
    static const bool traps = false;
    static const bool tinyness_before = false;
    static const float_round_style round_style = round_indeterminate;
  };
}


/**************** #undef all private macros ****************/

#undef __GMPP_DECLARE_COMPOUND_OPERATOR
#undef __GMPN_DECLARE_COMPOUND_OPERATOR
#undef __GMP_DECLARE_COMPOUND_OPERATOR
#undef __GMP_DECLARE_COMPOUND_OPERATOR_UI
#undef __GMP_DECLARE_INCREMENT_OPERATOR
#undef __GMPXX_DEFINE_ARITHMETIC_CONSTRUCTORS
#undef __GMPXX_DEFINE_ARITHMETIC_ASSIGNMENTS

#undef __GMPZQ_DEFINE_EXPR

#undef __GMP_DEFINE_UNARY_FUNCTION_1
#undef __GMP_DEFINE_UNARY_FUNCTION
#undef __GMP_DEFINE_UNARY_TYPE_FUNCTION

#undef __GMPP_DEFINE_BINARY_FUNCTION
#undef __GMPNN_DEFINE_BINARY_FUNCTION
#undef __GMPNS_DEFINE_BINARY_FUNCTION
#undef __GMPNU_DEFINE_BINARY_FUNCTION
#undef __GMPND_DEFINE_BINARY_FUNCTION
#undef __GMPNLD_DEFINE_BINARY_FUNCTION
#undef __GMPN_DEFINE_BINARY_FUNCTION
#undef __GMP_DEFINE_BINARY_FUNCTION

#undef __GMP_DEFINE_BINARY_FUNCTION_UI

#undef __GMPP_DEFINE_BINARY_TYPE_FUNCTION
#undef __GMPNN_DEFINE_BINARY_TYPE_FUNCTION
#undef __GMPNS_DEFINE_BINARY_TYPE_FUNCTION
#undef __GMPNU_DEFINE_BINARY_TYPE_FUNCTION
#undef __GMPND_DEFINE_BINARY_TYPE_FUNCTION
#undef __GMPNLD_DEFINE_BINARY_TYPE_FUNCTION
#undef __GMPN_DEFINE_BINARY_TYPE_FUNCTION
#undef __GMP_DEFINE_BINARY_TYPE_FUNCTION

#undef __GMPZ_DEFINE_COMPOUND_OPERATOR

#undef __GMPP_DEFINE_COMPOUND_OPERATOR
#undef __GMPNN_DEFINE_COMPOUND_OPERATOR
#undef __GMPNS_DEFINE_COMPOUND_OPERATOR
#undef __GMPNU_DEFINE_COMPOUND_OPERATOR
#undef __GMPND_DEFINE_COMPOUND_OPERATOR
#undef __GMPNLD_DEFINE_COMPOUND_OPERATOR
#undef __GMPN_DEFINE_COMPOUND_OPERATOR
#undef __GMP_DEFINE_COMPOUND_OPERATOR

#undef __GMPQ_DEFINE_COMPOUND_OPERATOR
#undef __GMPF_DEFINE_COMPOUND_OPERATOR

#undef __GMP_DEFINE_COMPOUND_OPERATOR_UI
#undef __GMPZ_DEFINE_COMPOUND_OPERATOR_UI
#undef __GMPQ_DEFINE_COMPOUND_OPERATOR_UI
#undef __GMPF_DEFINE_COMPOUND_OPERATOR_UI

#undef __GMP_DEFINE_INCREMENT_OPERATOR
#undef __GMPZ_DEFINE_INCREMENT_OPERATOR
#undef __GMPQ_DEFINE_INCREMENT_OPERATOR
#undef __GMPF_DEFINE_INCREMENT_OPERATOR

#undef __GMPXX_CONSTANT_TRUE
#undef __GMPXX_CONSTANT

#endif /* __GMP_PLUSPLUS__ */
