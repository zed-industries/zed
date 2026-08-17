// random number generation -*- C++ -*-

// Copyright (C) 2009-2020 Free Software Foundation, Inc.
//
// This file is part of the GNU ISO C++ Library.  This library is free
// software; you can redistribute it and/or modify it under the
// terms of the GNU General Public License as published by the
// Free Software Foundation; either version 3, or (at your option)
// any later version.

// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// Under Section 7 of GPL version 3, you are granted additional
// permissions described in the GCC Runtime Library Exception, version
// 3.1, as published by the Free Software Foundation.

// You should have received a copy of the GNU General Public License and
// a copy of the GCC Runtime Library Exception along with this program;
// see the files COPYING3 and COPYING.RUNTIME respectively.  If not, see
// <http://www.gnu.org/licenses/>.

/**
 * @file bits/random.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{random}
 */

#ifndef _RANDOM_H
#define _RANDOM_H 1

#include <vector>
#include <bits/uniform_int_dist.h>

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  // [26.4] Random number generation

  /**
   * @defgroup random Random Number Generation
   * @ingroup numerics
   *
   * A facility for generating random numbers on selected distributions.
   * @{
   */

  // std::uniform_random_bit_generator is defined in <bits/uniform_int_dist.h>

  /**
   * @brief A function template for converting the output of a (integral)
   * uniform random number generator to a floatng point result in the range
   * [0-1).
   */
  template<typename _RealType, size_t __bits,
	   typename _UniformRandomNumberGenerator>
    _RealType
    generate_canonical(_UniformRandomNumberGenerator& __g);

  /*
   * Implementation-space details.
   */
  namespace __detail
  {
    template<typename _UIntType, size_t __w,
	     bool = __w < static_cast<size_t>
			  (std::numeric_limits<_UIntType>::digits)>
      struct _Shift
      { static const _UIntType __value = 0; };

    template<typename _UIntType, size_t __w>
      struct _Shift<_UIntType, __w, true>
      { static const _UIntType __value = _UIntType(1) << __w; };

    template<int __s,
	     int __which = ((__s <= __CHAR_BIT__ * sizeof (int))
			    + (__s <= __CHAR_BIT__ * sizeof (long))
			    + (__s <= __CHAR_BIT__ * sizeof (long long))
			    /* assume long long no bigger than __int128 */
			    + (__s <= 128))>
      struct _Select_uint_least_t
      {
	static_assert(__which < 0, /* needs to be dependent */
		      "sorry, would be too much trouble for a slow result");
      };

    template<int __s>
      struct _Select_uint_least_t<__s, 4>
      { typedef unsigned int type; };

    template<int __s>
      struct _Select_uint_least_t<__s, 3>
      { typedef unsigned long type; };

    template<int __s>
      struct _Select_uint_least_t<__s, 2>
      { typedef unsigned long long type; };

#ifdef _GLIBCXX_USE_INT128
    template<int __s>
      struct _Select_uint_least_t<__s, 1>
      { typedef unsigned __int128 type; };
#endif

    // Assume a != 0, a < m, c < m, x < m.
    template<typename _Tp, _Tp __m, _Tp __a, _Tp __c,
	     bool __big_enough = (!(__m & (__m - 1))
				  || (_Tp(-1) - __c) / __a >= __m - 1),
             bool __schrage_ok = __m % __a < __m / __a>
      struct _Mod
      {
	typedef typename _Select_uint_least_t<std::__lg(__a)
					      + std::__lg(__m) + 2>::type _Tp2;
	static _Tp
	__calc(_Tp __x)
	{ return static_cast<_Tp>((_Tp2(__a) * __x + __c) % __m); }
      };

    // Schrage.
    template<typename _Tp, _Tp __m, _Tp __a, _Tp __c>
      struct _Mod<_Tp, __m, __a, __c, false, true>
      {
	static _Tp
	__calc(_Tp __x);
      };

    // Special cases:
    // - for m == 2^n or m == 0, unsigned integer overflow is safe.
    // - a * (m - 1) + c fits in _Tp, there is no overflow.
    template<typename _Tp, _Tp __m, _Tp __a, _Tp __c, bool __s>
      struct _Mod<_Tp, __m, __a, __c, true, __s>
      {
	static _Tp
	__calc(_Tp __x)
	{
	  _Tp __res = __a * __x + __c;
	  if (__m)
	    __res %= __m;
	  return __res;
	}
      };

    template<typename _Tp, _Tp __m, _Tp __a = 1, _Tp __c = 0>
      inline _Tp
      __mod(_Tp __x)
      { return _Mod<_Tp, __m, __a, __c>::__calc(__x); }

    /*
     * An adaptor class for converting the output of any Generator into
     * the input for a specific Distribution.
     */
    template<typename _Engine, typename _DInputType>
      struct _Adaptor
      {
	static_assert(std::is_floating_point<_DInputType>::value,
		      "template argument must be a floating point type");

      public:
	_Adaptor(_Engine& __g)
	: _M_g(__g) { }

	_DInputType
	min() const
	{ return _DInputType(0); }

	_DInputType
	max() const
	{ return _DInputType(1); }

	/*
	 * Converts a value generated by the adapted random number generator
	 * into a value in the input domain for the dependent random number
	 * distribution.
	 */
	_DInputType
	operator()()
	{
	  return std::generate_canonical<_DInputType,
	                            std::numeric_limits<_DInputType>::digits,
	                            _Engine>(_M_g);
	}

      private:
	_Engine& _M_g;
      };

    template<typename _Sseq>
      using __seed_seq_generate_t = decltype(
	  std::declval<_Sseq&>().generate(std::declval<uint_least32_t*>(),
					  std::declval<uint_least32_t*>()));

    // Detect whether _Sseq is a valid seed sequence for
    // a random number engine _Engine with result type _Res.
    template<typename _Sseq, typename _Engine, typename _Res,
	     typename _GenerateCheck = __seed_seq_generate_t<_Sseq>>
      using __is_seed_seq = __and_<
        __not_<is_same<__remove_cvref_t<_Sseq>, _Engine>>,
	is_unsigned<typename _Sseq::result_type>,
	__not_<is_convertible<_Sseq, _Res>>
      >;

  } // namespace __detail

  /**
   * @addtogroup random_generators Random Number Generators
   * @ingroup random
   *
   * These classes define objects which provide random or pseudorandom
   * numbers, either from a discrete or a continuous interval.  The
   * random number generator supplied as a part of this library are
   * all uniform random number generators which provide a sequence of
   * random number uniformly distributed over their range.
   *
   * A number generator is a function object with an operator() that
   * takes zero arguments and returns a number.
   *
   * A compliant random number generator must satisfy the following
   * requirements.  <table border=1 cellpadding=10 cellspacing=0>
   * <caption align=top>Random Number Generator Requirements</caption>
   * <tr><td>To be documented.</td></tr> </table>
   *
   * @{
   */

  /**
   * @brief A model of a linear congruential random number generator.
   *
   * A random number generator that produces pseudorandom numbers via
   * linear function:
   * @f[
   *     x_{i+1}\leftarrow(ax_{i} + c) \bmod m 
   * @f]
   *
   * The template parameter @p _UIntType must be an unsigned integral type
   * large enough to store values up to (__m-1). If the template parameter
   * @p __m is 0, the modulus @p __m used is
   * std::numeric_limits<_UIntType>::max() plus 1. Otherwise, the template
   * parameters @p __a and @p __c must be less than @p __m.
   *
   * The size of the state is @f$1@f$.
   */
  template<typename _UIntType, _UIntType __a, _UIntType __c, _UIntType __m>
    class linear_congruential_engine
    {
      static_assert(std::is_unsigned<_UIntType>::value,
		    "result_type must be an unsigned integral type");
      static_assert(__m == 0u || (__a < __m && __c < __m),
		    "template argument substituting __m out of bounds");

      template<typename _Sseq>
	using _If_seed_seq = typename enable_if<__detail::__is_seed_seq<
	  _Sseq, linear_congruential_engine, _UIntType>::value>::type;

    public:
      /** The type of the generated random value. */
      typedef _UIntType result_type;

      /** The multiplier. */
      static constexpr result_type multiplier   = __a;
      /** An increment. */
      static constexpr result_type increment    = __c;
      /** The modulus. */
      static constexpr result_type modulus      = __m;
      static constexpr result_type default_seed = 1u;

      /**
       * @brief Constructs a %linear_congruential_engine random number
       *        generator engine with seed 1.
       */
      linear_congruential_engine() : linear_congruential_engine(default_seed)
      { }

      /**
       * @brief Constructs a %linear_congruential_engine random number
       *        generator engine with seed @p __s.  The default seed value
       *        is 1.
       *
       * @param __s The initial seed value.
       */
      explicit
      linear_congruential_engine(result_type __s)
      { seed(__s); }

      /**
       * @brief Constructs a %linear_congruential_engine random number
       *        generator engine seeded from the seed sequence @p __q.
       *
       * @param __q the seed sequence.
       */
      template<typename _Sseq, typename = _If_seed_seq<_Sseq>>
        explicit
        linear_congruential_engine(_Sseq& __q)
        { seed(__q); }

      /**
       * @brief Reseeds the %linear_congruential_engine random number generator
       *        engine sequence to the seed @p __s.
       *
       * @param __s The new seed.
       */
      void
      seed(result_type __s = default_seed);

      /**
       * @brief Reseeds the %linear_congruential_engine random number generator
       *        engine
       * sequence using values from the seed sequence @p __q.
       *
       * @param __q the seed sequence.
       */
      template<typename _Sseq>
        _If_seed_seq<_Sseq>
        seed(_Sseq& __q);

      /**
       * @brief Gets the smallest possible value in the output range.
       *
       * The minimum depends on the @p __c parameter: if it is zero, the
       * minimum generated must be > 0, otherwise 0 is allowed.
       */
      static constexpr result_type
      min()
      { return __c == 0u ? 1u : 0u; }

      /**
       * @brief Gets the largest possible value in the output range.
       */
      static constexpr result_type
      max()
      { return __m - 1u; }

      /**
       * @brief Discard a sequence of random numbers.
       */
      void
      discard(unsigned long long __z)
      {
	for (; __z != 0ULL; --__z)
	  (*this)();
      }

      /**
       * @brief Gets the next random number in the sequence.
       */
      result_type
      operator()()
      {
	_M_x = __detail::__mod<_UIntType, __m, __a, __c>(_M_x);
	return _M_x;
      }

      /**
       * @brief Compares two linear congruential random number generator
       * objects of the same type for equality.
       *
       * @param __lhs A linear congruential random number generator object.
       * @param __rhs Another linear congruential random number generator
       *              object.
       *
       * @returns true if the infinite sequences of generated values
       *          would be equal, false otherwise.
       */
      friend bool
      operator==(const linear_congruential_engine& __lhs,
		 const linear_congruential_engine& __rhs)
      { return __lhs._M_x == __rhs._M_x; }

      /**
       * @brief Writes the textual representation of the state x(i) of x to
       *        @p __os.
       *
       * @param __os  The output stream.
       * @param __lcr A % linear_congruential_engine random number generator.
       * @returns __os.
       */
      template<typename _UIntType1, _UIntType1 __a1, _UIntType1 __c1,
	       _UIntType1 __m1, typename _CharT, typename _Traits>
	friend std::basic_ostream<_CharT, _Traits>&
	operator<<(std::basic_ostream<_CharT, _Traits>& __os,
		   const std::linear_congruential_engine<_UIntType1,
		   __a1, __c1, __m1>& __lcr);

      /**
       * @brief Sets the state of the engine by reading its textual
       *        representation from @p __is.
       *
       * The textual representation must have been previously written using
       * an output stream whose imbued locale and whose type's template
       * specialization arguments _CharT and _Traits were the same as those
       * of @p __is.
       *
       * @param __is  The input stream.
       * @param __lcr A % linear_congruential_engine random number generator.
       * @returns __is.
       */
      template<typename _UIntType1, _UIntType1 __a1, _UIntType1 __c1,
	       _UIntType1 __m1, typename _CharT, typename _Traits>
	friend std::basic_istream<_CharT, _Traits>&
	operator>>(std::basic_istream<_CharT, _Traits>& __is,
		   std::linear_congruential_engine<_UIntType1, __a1,
		   __c1, __m1>& __lcr);

    private:
      _UIntType _M_x;
    };

  /**
   * @brief Compares two linear congruential random number generator
   * objects of the same type for inequality.
   *
   * @param __lhs A linear congruential random number generator object.
   * @param __rhs Another linear congruential random number generator
   *              object.
   *
   * @returns true if the infinite sequences of generated values
   *          would be different, false otherwise.
   */
  template<typename _UIntType, _UIntType __a, _UIntType __c, _UIntType __m>
    inline bool
    operator!=(const std::linear_congruential_engine<_UIntType, __a,
	       __c, __m>& __lhs,
	       const std::linear_congruential_engine<_UIntType, __a,
	       __c, __m>& __rhs)
    { return !(__lhs == __rhs); }


  /**
   * A generalized feedback shift register discrete random number generator.
   *
   * This algorithm avoids multiplication and division and is designed to be
   * friendly to a pipelined architecture.  If the parameters are chosen
   * correctly, this generator will produce numbers with a very long period and
   * fairly good apparent entropy, although still not cryptographically strong.
   *
   * The best way to use this generator is with the predefined mt19937 class.
   *
   * This algorithm was originally invented by Makoto Matsumoto and
   * Takuji Nishimura.
   *
   * @tparam __w  Word size, the number of bits in each element of 
   *              the state vector.
   * @tparam __n  The degree of recursion.
   * @tparam __m  The period parameter.
   * @tparam __r  The separation point bit index.
   * @tparam __a  The last row of the twist matrix.
   * @tparam __u  The first right-shift tempering matrix parameter.
   * @tparam __d  The first right-shift tempering matrix mask.
   * @tparam __s  The first left-shift tempering matrix parameter.
   * @tparam __b  The first left-shift tempering matrix mask.
   * @tparam __t  The second left-shift tempering matrix parameter.
   * @tparam __c  The second left-shift tempering matrix mask.
   * @tparam __l  The second right-shift tempering matrix parameter.
   * @tparam __f  Initialization multiplier.
   */
  template<typename _UIntType, size_t __w,
	   size_t __n, size_t __m, size_t __r,
	   _UIntType __a, size_t __u, _UIntType __d, size_t __s,
	   _UIntType __b, size_t __t,
	   _UIntType __c, size_t __l, _UIntType __f>
    class mersenne_twister_engine
    {
      static_assert(std::is_unsigned<_UIntType>::value,
		    "result_type must be an unsigned integral type");
      static_assert(1u <= __m && __m <= __n,
		    "template argument substituting __m out of bounds");
      static_assert(__r <= __w, "template argument substituting "
		    "__r out of bound");
      static_assert(__u <= __w, "template argument substituting "
		    "__u out of bound");
      static_assert(__s <= __w, "template argument substituting "
		    "__s out of bound");
      static_assert(__t <= __w, "template argument substituting "
		    "__t out of bound");
      static_assert(__l <= __w, "template argument substituting "
		    "__l out of bound");
      static_assert(__w <= std::numeric_limits<_UIntType>::digits,
		    "template argument substituting __w out of bound");
      static_assert(__a <= (__detail::_Shift<_UIntType, __w>::__value - 1),
		    "template argument substituting __a out of bound");
      static_assert(__b <= (__detail::_Shift<_UIntType, __w>::__value - 1),
		    "template argument substituting __b out of bound");
      static_assert(__c <= (__detail::_Shift<_UIntType, __w>::__value - 1),
		    "template argument substituting __c out of bound");
      static_assert(__d <= (__detail::_Shift<_UIntType, __w>::__value - 1),
		    "template argument substituting __d out of bound");
      static_assert(__f <= (__detail::_Shift<_UIntType, __w>::__value - 1),
		    "template argument substituting __f out of bound");

      template<typename _Sseq>
	using _If_seed_seq = typename enable_if<__detail::__is_seed_seq<
	  _Sseq, mersenne_twister_engine, _UIntType>::value>::type;

    public:
      /** The type of the generated random value. */
      typedef _UIntType result_type;

      // parameter values
      static constexpr size_t      word_size                 = __w;
      static constexpr size_t      state_size                = __n;
      static constexpr size_t      shift_size                = __m;
      static constexpr size_t      mask_bits                 = __r;
      static constexpr result_type xor_mask                  = __a;
      static constexpr size_t      tempering_u               = __u;
      static constexpr result_type tempering_d               = __d;
      static constexpr size_t      tempering_s               = __s;
      static constexpr result_type tempering_b               = __b;
      static constexpr size_t      tempering_t               = __t;
      static constexpr result_type tempering_c               = __c;
      static constexpr size_t      tempering_l               = __l;
      static constexpr result_type initialization_multiplier = __f;
      static constexpr result_type default_seed = 5489u;

      // constructors and member functions

      mersenne_twister_engine() : mersenne_twister_engine(default_seed) { }

      explicit
      mersenne_twister_engine(result_type __sd)
      { seed(__sd); }

      /**
       * @brief Constructs a %mersenne_twister_engine random number generator
       *        engine seeded from the seed sequence @p __q.
       *
       * @param __q the seed sequence.
       */
      template<typename _Sseq, typename = _If_seed_seq<_Sseq>>
        explicit
        mersenne_twister_engine(_Sseq& __q)
        { seed(__q); }

      void
      seed(result_type __sd = default_seed);

      template<typename _Sseq>
        _If_seed_seq<_Sseq>
        seed(_Sseq& __q);

      /**
       * @brief Gets the smallest possible value in the output range.
       */
      static constexpr result_type
      min()
      { return 0; }

      /**
       * @brief Gets the largest possible value in the output range.
       */
      static constexpr result_type
      max()
      { return __detail::_Shift<_UIntType, __w>::__value - 1; }

      /**
       * @brief Discard a sequence of random numbers.
       */
      void
      discard(unsigned long long __z);

      result_type
      operator()();

      /**
       * @brief Compares two % mersenne_twister_engine random number generator
       *        objects of the same type for equality.
       *
       * @param __lhs A % mersenne_twister_engine random number generator
       *              object.
       * @param __rhs Another % mersenne_twister_engine random number
       *              generator object.
       *
       * @returns true if the infinite sequences of generated values
       *          would be equal, false otherwise.
       */
      friend bool
      operator==(const mersenne_twister_engine& __lhs,
		 const mersenne_twister_engine& __rhs)
      { return (std::equal(__lhs._M_x, __lhs._M_x + state_size, __rhs._M_x)
		&& __lhs._M_p == __rhs._M_p); }

      /**
       * @brief Inserts the current state of a % mersenne_twister_engine
       *        random number generator engine @p __x into the output stream
       *        @p __os.
       *
       * @param __os An output stream.
       * @param __x  A % mersenne_twister_engine random number generator
       *             engine.
       *
       * @returns The output stream with the state of @p __x inserted or in
       * an error state.
       */
      template<typename _UIntType1,
	       size_t __w1, size_t __n1,
	       size_t __m1, size_t __r1,
	       _UIntType1 __a1, size_t __u1,
	       _UIntType1 __d1, size_t __s1,
	       _UIntType1 __b1, size_t __t1,
	       _UIntType1 __c1, size_t __l1, _UIntType1 __f1,
	       typename _CharT, typename _Traits>
	friend std::basic_ostream<_CharT, _Traits>&
	operator<<(std::basic_ostream<_CharT, _Traits>& __os,
		   const std::mersenne_twister_engine<_UIntType1, __w1, __n1,
		   __m1, __r1, __a1, __u1, __d1, __s1, __b1, __t1, __c1,
		   __l1, __f1>& __x);

      /**
       * @brief Extracts the current state of a % mersenne_twister_engine
       *        random number generator engine @p __x from the input stream
       *        @p __is.
       *
       * @param __is An input stream.
       * @param __x  A % mersenne_twister_engine random number generator
       *             engine.
       *
       * @returns The input stream with the state of @p __x extracted or in
       * an error state.
       */
      template<typename _UIntType1,
	       size_t __w1, size_t __n1,
	       size_t __m1, size_t __r1,
	       _UIntType1 __a1, size_t __u1,
	       _UIntType1 __d1, size_t __s1,
	       _UIntType1 __b1, size_t __t1,
	       _UIntType1 __c1, size_t __l1, _UIntType1 __f1,
	       typename _CharT, typename _Traits>
	friend std::basic_istream<_CharT, _Traits>&
	operator>>(std::basic_istream<_CharT, _Traits>& __is,
		   std::mersenne_twister_engine<_UIntType1, __w1, __n1, __m1,
		   __r1, __a1, __u1, __d1, __s1, __b1, __t1, __c1,
		   __l1, __f1>& __x);

    private:
      void _M_gen_rand();

      _UIntType _M_x[state_size];
      size_t    _M_p;
    };

  /**
   * @brief Compares two % mersenne_twister_engine random number generator
   *        objects of the same type for inequality.
   *
   * @param __lhs A % mersenne_twister_engine random number generator
   *              object.
   * @param __rhs Another % mersenne_twister_engine random number
   *              generator object.
   *
   * @returns true if the infinite sequences of generated values
   *          would be different, false otherwise.
   */
  template<typename _UIntType, size_t __w,
	   size_t __n, size_t __m, size_t __r,
	   _UIntType __a, size_t __u, _UIntType __d, size_t __s,
	   _UIntType __b, size_t __t,
	   _UIntType __c, size_t __l, _UIntType __f>
    inline bool
    operator!=(const std::mersenne_twister_engine<_UIntType, __w, __n, __m,
	       __r, __a, __u, __d, __s, __b, __t, __c, __l, __f>& __lhs,
	       const std::mersenne_twister_engine<_UIntType, __w, __n, __m,
	       __r, __a, __u, __d, __s, __b, __t, __c, __l, __f>& __rhs)
    { return !(__lhs == __rhs); }


  /**
   * @brief The Marsaglia-Zaman generator.
   *
   * This is a model of a Generalized Fibonacci discrete random number
   * generator, sometimes referred to as the SWC generator.
   *
   * A discrete random number generator that produces pseudorandom
   * numbers using:
   * @f[
   *     x_{i}\leftarrow(x_{i - s} - x_{i - r} - carry_{i-1}) \bmod m 
   * @f]
   *
   * The size of the state is @f$r@f$
   * and the maximum period of the generator is @f$(m^r - m^s - 1)@f$.
   */
  template<typename _UIntType, size_t __w, size_t __s, size_t __r>
    class subtract_with_carry_engine
    {
      static_assert(std::is_unsigned<_UIntType>::value,
		    "result_type must be an unsigned integral type");
      static_assert(0u < __s && __s < __r,
		    "0 < s < r");
      static_assert(0u < __w && __w <= std::numeric_limits<_UIntType>::digits,
		    "template argument substituting __w out of bounds");

      template<typename _Sseq>
	using _If_seed_seq = typename enable_if<__detail::__is_seed_seq<
	  _Sseq, subtract_with_carry_engine, _UIntType>::value>::type;

    public:
      /** The type of the generated random value. */
      typedef _UIntType result_type;

      // parameter values
      static constexpr size_t      word_size    = __w;
      static constexpr size_t      short_lag    = __s;
      static constexpr size_t      long_lag     = __r;
      static constexpr result_type default_seed = 19780503u;

      subtract_with_carry_engine() : subtract_with_carry_engine(default_seed)
      { }

      /**
       * @brief Constructs an explicitly seeded %subtract_with_carry_engine
       *        random number generator.
       */
      explicit
      subtract_with_carry_engine(result_type __sd)
      { seed(__sd); }

      /**
       * @brief Constructs a %subtract_with_carry_engine random number engine
       *        seeded from the seed sequence @p __q.
       *
       * @param __q the seed sequence.
       */
      template<typename _Sseq, typename = _If_seed_seq<_Sseq>>
        explicit
        subtract_with_carry_engine(_Sseq& __q)
        { seed(__q); }

      /**
       * @brief Seeds the initial state @f$x_0@f$ of the random number
       *        generator.
       *
       * N1688[4.19] modifies this as follows.  If @p __value == 0,
       * sets value to 19780503.  In any case, with a linear
       * congruential generator lcg(i) having parameters @f$ m_{lcg} =
       * 2147483563, a_{lcg} = 40014, c_{lcg} = 0, and lcg(0) = value
       * @f$, sets @f$ x_{-r} \dots x_{-1} @f$ to @f$ lcg(1) \bmod m
       * \dots lcg(r) \bmod m @f$ respectively.  If @f$ x_{-1} = 0 @f$
       * set carry to 1, otherwise sets carry to 0.
       */
      void
      seed(result_type __sd = default_seed);

      /**
       * @brief Seeds the initial state @f$x_0@f$ of the
       * % subtract_with_carry_engine random number generator.
       */
      template<typename _Sseq>
	_If_seed_seq<_Sseq>
        seed(_Sseq& __q);

      /**
       * @brief Gets the inclusive minimum value of the range of random
       * integers returned by this generator.
       */
      static constexpr result_type
      min()
      { return 0; }

      /**
       * @brief Gets the inclusive maximum value of the range of random
       * integers returned by this generator.
       */
      static constexpr result_type
      max()
      { return __detail::_Shift<_UIntType, __w>::__value - 1; }

      /**
       * @brief Discard a sequence of random numbers.
       */
      void
      discard(unsigned long long __z)
      {
	for (; __z != 0ULL; --__z)
	  (*this)();
      }

      /**
       * @brief Gets the next random number in the sequence.
       */
      result_type
      operator()();

      /**
       * @brief Compares two % subtract_with_carry_engine random number
       *        generator objects of the same type for equality.
       *
       * @param __lhs A % subtract_with_carry_engine random number generator
       *              object.
       * @param __rhs Another % subtract_with_carry_engine random number
       *              generator object.
       *
       * @returns true if the infinite sequences of generated values
       *          would be equal, false otherwise.
      */
      friend bool
      operator==(const subtract_with_carry_engine& __lhs,
		 const subtract_with_carry_engine& __rhs)
      { return (std::equal(__lhs._M_x, __lhs._M_x + long_lag, __rhs._M_x)
		&& __lhs._M_carry == __rhs._M_carry
		&& __lhs._M_p == __rhs._M_p); }

      /**
       * @brief Inserts the current state of a % subtract_with_carry_engine
       *        random number generator engine @p __x into the output stream
       *        @p __os.
       *
       * @param __os An output stream.
       * @param __x  A % subtract_with_carry_engine random number generator
       *             engine.
       *
       * @returns The output stream with the state of @p __x inserted or in
       * an error state.
       */
      template<typename _UIntType1, size_t __w1, size_t __s1, size_t __r1,
	       typename _CharT, typename _Traits>
	friend std::basic_ostream<_CharT, _Traits>&
	operator<<(std::basic_ostream<_CharT, _Traits>& __os,
		   const std::subtract_with_carry_engine<_UIntType1, __w1,
		   __s1, __r1>& __x);

      /**
       * @brief Extracts the current state of a % subtract_with_carry_engine
       *        random number generator engine @p __x from the input stream
       *        @p __is.
       *
       * @param __is An input stream.
       * @param __x  A % subtract_with_carry_engine random number generator
       *             engine.
       *
       * @returns The input stream with the state of @p __x extracted or in
       * an error state.
       */
      template<typename _UIntType1, size_t __w1, size_t __s1, size_t __r1,
	       typename _CharT, typename _Traits>
	friend std::basic_istream<_CharT, _Traits>&
	operator>>(std::basic_istream<_CharT, _Traits>& __is,
		   std::subtract_with_carry_engine<_UIntType1, __w1,
		   __s1, __r1>& __x);

    private:
      /// The state of the generator.  This is a ring buffer.
      _UIntType  _M_x[long_lag];
      _UIntType  _M_carry;		///< The carry
      size_t     _M_p;			///< Current index of x(i - r).
    };

  /**
   * @brief Compares two % subtract_with_carry_engine random number
   *        generator objects of the same type for inequality.
   *
   * @param __lhs A % subtract_with_carry_engine random number generator
   *              object.
   * @param __rhs Another % subtract_with_carry_engine random number
   *              generator object.
   *
   * @returns true if the infinite sequences of generated values
   *          would be different, false otherwise.
   */
  template<typename _UIntType, size_t __w, size_t __s, size_t __r>
    inline bool
    operator!=(const std::subtract_with_carry_engine<_UIntType, __w,
	       __s, __r>& __lhs,
	       const std::subtract_with_carry_engine<_UIntType, __w,
	       __s, __r>& __rhs)
    { return !(__lhs == __rhs); }


  /**
   * Produces random numbers from some base engine by discarding blocks of
   * data.
   *
   * 0 <= @p __r <= @p __p
   */
  template<typename _RandomNumberEngine, size_t __p, size_t __r>
    class discard_block_engine
    {
      static_assert(1 <= __r && __r <= __p,
		    "template argument substituting __r out of bounds");

    public:
      /** The type of the generated random value. */
      typedef typename _RandomNumberEngine::result_type result_type;

      template<typename _Sseq>
	using _If_seed_seq = typename enable_if<__detail::__is_seed_seq<
	  _Sseq, discard_block_engine, result_type>::value>::type;

      // parameter values
      static constexpr size_t block_size = __p;
      static constexpr size_t used_block = __r;

      /**
       * @brief Constructs a default %discard_block_engine engine.
       *
       * The underlying engine is default constructed as well.
       */
      discard_block_engine()
      : _M_b(), _M_n(0) { }

      /**
       * @brief Copy constructs a %discard_block_engine engine.
       *
       * Copies an existing base class random number generator.
       * @param __rng An existing (base class) engine object.
       */
      explicit
      discard_block_engine(const _RandomNumberEngine& __rng)
      : _M_b(__rng), _M_n(0) { }

      /**
       * @brief Move constructs a %discard_block_engine engine.
       *
       * Copies an existing base class random number generator.
       * @param __rng An existing (base class) engine object.
       */
      explicit
      discard_block_engine(_RandomNumberEngine&& __rng)
      : _M_b(std::move(__rng)), _M_n(0) { }

      /**
       * @brief Seed constructs a %discard_block_engine engine.
       *
       * Constructs the underlying generator engine seeded with @p __s.
       * @param __s A seed value for the base class engine.
       */
      explicit
      discard_block_engine(result_type __s)
      : _M_b(__s), _M_n(0) { }

      /**
       * @brief Generator construct a %discard_block_engine engine.
       *
       * @param __q A seed sequence.
       */
      template<typename _Sseq, typename = _If_seed_seq<_Sseq>>
        explicit
        discard_block_engine(_Sseq& __q)
	: _M_b(__q), _M_n(0)
        { }

      /**
       * @brief Reseeds the %discard_block_engine object with the default
       *        seed for the underlying base class generator engine.
       */
      void
      seed()
      {
	_M_b.seed();
	_M_n = 0;
      }

      /**
       * @brief Reseeds the %discard_block_engine object with the default
       *        seed for the underlying base class generator engine.
       */
      void
      seed(result_type __s)
      {
	_M_b.seed(__s);
	_M_n = 0;
      }

      /**
       * @brief Reseeds the %discard_block_engine object with the given seed
       *        sequence.
       * @param __q A seed generator function.
       */
      template<typename _Sseq>
        _If_seed_seq<_Sseq>
        seed(_Sseq& __q)
        {
	  _M_b.seed(__q);
	  _M_n = 0;
	}

      /**
       * @brief Gets a const reference to the underlying generator engine
       *        object.
       */
      const _RandomNumberEngine&
      base() const noexcept
      { return _M_b; }

      /**
       * @brief Gets the minimum value in the generated random number range.
       */
      static constexpr result_type
      min()
      { return _RandomNumberEngine::min(); }

      /**
       * @brief Gets the maximum value in the generated random number range.
       */
      static constexpr result_type
      max()
      { return _RandomNumberEngine::max(); }

      /**
       * @brief Discard a sequence of random numbers.
       */
      void
      discard(unsigned long long __z)
      {
	for (; __z != 0ULL; --__z)
	  (*this)();
      }

      /**
       * @brief Gets the next value in the generated random number sequence.
       */
      result_type
      operator()();

      /**
       * @brief Compares two %discard_block_engine random number generator
       *        objects of the same type for equality.
       *
       * @param __lhs A %discard_block_engine random number generator object.
       * @param __rhs Another %discard_block_engine random number generator
       *              object.
       *
       * @returns true if the infinite sequences of generated values
       *          would be equal, false otherwise.
       */
      friend bool
      operator==(const discard_block_engine& __lhs,
		 const discard_block_engine& __rhs)
      { return __lhs._M_b == __rhs._M_b && __lhs._M_n == __rhs._M_n; }

      /**
       * @brief Inserts the current state of a %discard_block_engine random
       *        number generator engine @p __x into the output stream
       *        @p __os.
       *
       * @param __os An output stream.
       * @param __x  A %discard_block_engine random number generator engine.
       *
       * @returns The output stream with the state of @p __x inserted or in
       * an error state.
       */
      template<typename _RandomNumberEngine1, size_t __p1, size_t __r1,
	       typename _CharT, typename _Traits>
	friend std::basic_ostream<_CharT, _Traits>&
	operator<<(std::basic_ostream<_CharT, _Traits>& __os,
		   const std::discard_block_engine<_RandomNumberEngine1,
		   __p1, __r1>& __x);

      /**
       * @brief Extracts the current state of a % subtract_with_carry_engine
       *        random number generator engine @p __x from the input stream
       *        @p __is.
       *
       * @param __is An input stream.
       * @param __x  A %discard_block_engine random number generator engine.
       *
       * @returns The input stream with the state of @p __x extracted or in
       * an error state.
       */
      template<typename _RandomNumberEngine1, size_t __p1, size_t __r1,
	       typename _CharT, typename _Traits>
	friend std::basic_istream<_CharT, _Traits>&
	operator>>(std::basic_istream<_CharT, _Traits>& __is,
		   std::discard_block_engine<_RandomNumberEngine1,
		   __p1, __r1>& __x);

    private:
      _RandomNumberEngine _M_b;
      size_t _M_n;
    };

  /**
   * @brief Compares two %discard_block_engine random number generator
   *        objects of the same type for inequality.
   *
   * @param __lhs A %discard_block_engine random number generator object.
   * @param __rhs Another %discard_block_engine random number generator
   *              object.
   *
   * @returns true if the infinite sequences of generated values
   *          would be different, false otherwise.
   */
  template<typename _RandomNumberEngine, size_t __p, size_t __r>
    inline bool
    operator!=(const std::discard_block_engine<_RandomNumberEngine, __p,
	       __r>& __lhs,
	       const std::discard_block_engine<_RandomNumberEngine, __p,
	       __r>& __rhs)
    { return !(__lhs == __rhs); }


  /**
   * Produces random numbers by combining random numbers from some base
   * engine to produce random numbers with a specifies number of bits @p __w.
   */
  template<typename _RandomNumberEngine, size_t __w, typename _UIntType>
    class independent_bits_engine
    {
      static_assert(std::is_unsigned<_UIntType>::value,
		    "result_type must be an unsigned integral type");
      static_assert(0u < __w && __w <= std::numeric_limits<_UIntType>::digits,
		    "template argument substituting __w out of bounds");

      template<typename _Sseq>
	using _If_seed_seq = typename enable_if<__detail::__is_seed_seq<
	  _Sseq, independent_bits_engine, _UIntType>::value>::type;

    public:
      /** The type of the generated random value. */
      typedef _UIntType result_type;

      /**
       * @brief Constructs a default %independent_bits_engine engine.
       *
       * The underlying engine is default constructed as well.
       */
      independent_bits_engine()
      : _M_b() { }

      /**
       * @brief Copy constructs a %independent_bits_engine engine.
       *
       * Copies an existing base class random number generator.
       * @param __rng An existing (base class) engine object.
       */
      explicit
      independent_bits_engine(const _RandomNumberEngine& __rng)
      : _M_b(__rng) { }

      /**
       * @brief Move constructs a %independent_bits_engine engine.
       *
       * Copies an existing base class random number generator.
       * @param __rng An existing (base class) engine object.
       */
      explicit
      independent_bits_engine(_RandomNumberEngine&& __rng)
      : _M_b(std::move(__rng)) { }

      /**
       * @brief Seed constructs a %independent_bits_engine engine.
       *
       * Constructs the underlying generator engine seeded with @p __s.
       * @param __s A seed value for the base class engine.
       */
      explicit
      independent_bits_engine(result_type __s)
      : _M_b(__s) { }

      /**
       * @brief Generator construct a %independent_bits_engine engine.
       *
       * @param __q A seed sequence.
       */
      template<typename _Sseq, typename = _If_seed_seq<_Sseq>>
        explicit
        independent_bits_engine(_Sseq& __q)
        : _M_b(__q)
        { }

      /**
       * @brief Reseeds the %independent_bits_engine object with the default
       *        seed for the underlying base class generator engine.
       */
      void
      seed()
      { _M_b.seed(); }

      /**
       * @brief Reseeds the %independent_bits_engine object with the default
       *        seed for the underlying base class generator engine.
       */
      void
      seed(result_type __s)
      { _M_b.seed(__s); }

      /**
       * @brief Reseeds the %independent_bits_engine object with the given
       *        seed sequence.
       * @param __q A seed generator function.
       */
      template<typename _Sseq>
        _If_seed_seq<_Sseq>
        seed(_Sseq& __q)
        { _M_b.seed(__q); }

      /**
       * @brief Gets a const reference to the underlying generator engine
       *        object.
       */
      const _RandomNumberEngine&
      base() const noexcept
      { return _M_b; }

      /**
       * @brief Gets the minimum value in the generated random number range.
       */
      static constexpr result_type
      min()
      { return 0U; }

      /**
       * @brief Gets the maximum value in the generated random number range.
       */
      static constexpr result_type
      max()
      { return __detail::_Shift<_UIntType, __w>::__value - 1; }

      /**
       * @brief Discard a sequence of random numbers.
       */
      void
      discard(unsigned long long __z)
      {
	for (; __z != 0ULL; --__z)
	  (*this)();
      }

      /**
       * @brief Gets the next value in the generated random number sequence.
       */
      result_type
      operator()();

      /**
       * @brief Compares two %independent_bits_engine random number generator
       * objects of the same type for equality.
       *
       * @param __lhs A %independent_bits_engine random number generator
       *              object.
       * @param __rhs Another %independent_bits_engine random number generator
       *              object.
       *
       * @returns true if the infinite sequences of generated values
       *          would be equal, false otherwise.
       */
      friend bool
      operator==(const independent_bits_engine& __lhs,
		 const independent_bits_engine& __rhs)
      { return __lhs._M_b == __rhs._M_b; }

      /**
       * @brief Extracts the current state of a % subtract_with_carry_engine
       *        random number generator engine @p __x from the input stream
       *        @p __is.
       *
       * @param __is An input stream.
       * @param __x  A %independent_bits_engine random number generator
       *             engine.
       *
       * @returns The input stream with the state of @p __x extracted or in
       *          an error state.
       */
      template<typename _CharT, typename _Traits>
	friend std::basic_istream<_CharT, _Traits>&
	operator>>(std::basic_istream<_CharT, _Traits>& __is,
		   std::independent_bits_engine<_RandomNumberEngine,
		   __w, _UIntType>& __x)
	{
	  __is >> __x._M_b;
	  return __is;
	}

    private:
      _RandomNumberEngine _M_b;
    };

  /**
   * @brief Compares two %independent_bits_engine random number generator
   * objects of the same type for inequality.
   *
   * @param __lhs A %independent_bits_engine random number generator
   *              object.
   * @param __rhs Another %independent_bits_engine random number generator
   *              object.
   *
   * @returns true if the infinite sequences of generated values
   *          would be different, false otherwise.
   */
  template<typename _RandomNumberEngine, size_t __w, typename _UIntType>
    inline bool
    operator!=(const std::independent_bits_engine<_RandomNumberEngine, __w,
	       _UIntType>& __lhs,
	       const std::independent_bits_engine<_RandomNumberEngine, __w,
	       _UIntType>& __rhs)
    { return !(__lhs == __rhs); }

  /**
   * @brief Inserts the current state of a %independent_bits_engine random
   *        number generator engine @p __x into the output stream @p __os.
   *
   * @param __os An output stream.
   * @param __x  A %independent_bits_engine random number generator engine.
   *
   * @returns The output stream with the state of @p __x inserted or in
   *          an error state.
   */
  template<typename _RandomNumberEngine, size_t __w, typename _UIntType,
	   typename _CharT, typename _Traits>
    std::basic_ostream<_CharT, _Traits>&
    operator<<(std::basic_ostream<_CharT, _Traits>& __os,
	       const std::independent_bits_engine<_RandomNumberEngine,
	       __w, _UIntType>& __x)
    {
      __os << __x.base();
      return __os;
    }


  /**
   * @brief Produces random numbers by combining random numbers from some
   * base engine to produce random numbers with a specifies number of bits
   * @p __k.
   */
  template<typename _RandomNumberEngine, size_t __k>
    class shuffle_order_engine
    {
      static_assert(1u <= __k, "template argument substituting "
		    "__k out of bound");

    public:
      /** The type of the generated random value. */
      typedef typename _RandomNumberEngine::result_type result_type;

      template<typename _Sseq>
	using _If_seed_seq = typename enable_if<__detail::__is_seed_seq<
	  _Sseq, shuffle_order_engine, result_type>::value>::type;

      static constexpr size_t table_size = __k;

      /**
       * @brief Constructs a default %shuffle_order_engine engine.
       *
       * The underlying engine is default constructed as well.
       */
      shuffle_order_engine()
      : _M_b()
      { _M_initialize(); }

      /**
       * @brief Copy constructs a %shuffle_order_engine engine.
       *
       * Copies an existing base class random number generator.
       * @param __rng An existing (base class) engine object.
       */
      explicit
      shuffle_order_engine(const _RandomNumberEngine& __rng)
      : _M_b(__rng)
      { _M_initialize(); }

      /**
       * @brief Move constructs a %shuffle_order_engine engine.
       *
       * Copies an existing base class random number generator.
       * @param __rng An existing (base class) engine object.
       */
      explicit
      shuffle_order_engine(_RandomNumberEngine&& __rng)
      : _M_b(std::move(__rng))
      { _M_initialize(); }

      /**
       * @brief Seed constructs a %shuffle_order_engine engine.
       *
       * Constructs the underlying generator engine seeded with @p __s.
       * @param __s A seed value for the base class engine.
       */
      explicit
      shuffle_order_engine(result_type __s)
      : _M_b(__s)
      { _M_initialize(); }

      /**
       * @brief Generator construct a %shuffle_order_engine engine.
       *
       * @param __q A seed sequence.
       */
      template<typename _Sseq, typename = _If_seed_seq<_Sseq>>
        explicit
        shuffle_order_engine(_Sseq& __q)
        : _M_b(__q)
        { _M_initialize(); }

      /**
       * @brief Reseeds the %shuffle_order_engine object with the default seed
                for the underlying base class generator engine.
       */
      void
      seed()
      {
	_M_b.seed();
	_M_initialize();
      }

      /**
       * @brief Reseeds the %shuffle_order_engine object with the default seed
       *        for the underlying base class generator engine.
       */
      void
      seed(result_type __s)
      {
	_M_b.seed(__s);
	_M_initialize();
      }

      /**
       * @brief Reseeds the %shuffle_order_engine object with the given seed
       *        sequence.
       * @param __q A seed generator function.
       */
      template<typename _Sseq>
        _If_seed_seq<_Sseq>
        seed(_Sseq& __q)
        {
	  _M_b.seed(__q);
	  _M_initialize();
	}

      /**
       * Gets a const reference to the underlying generator engine object.
       */
      const _RandomNumberEngine&
      base() const noexcept
      { return _M_b; }

      /**
       * Gets the minimum value in the generated random number range.
       */
      static constexpr result_type
      min()
      { return _RandomNumberEngine::min(); }

      /**
       * Gets the maximum value in the generated random number range.
       */
      static constexpr result_type
      max()
      { return _RandomNumberEngine::max(); }

      /**
       * Discard a sequence of random numbers.
       */
      void
      discard(unsigned long long __z)
      {
	for (; __z != 0ULL; --__z)
	  (*this)();
      }

      /**
       * Gets the next value in the generated random number sequence.
       */
      result_type
      operator()();

      /**
       * Compares two %shuffle_order_engine random number generator objects
       * of the same type for equality.
       *
       * @param __lhs A %shuffle_order_engine random number generator object.
       * @param __rhs Another %shuffle_order_engine random number generator
       *              object.
       *
       * @returns true if the infinite sequences of generated values
       *          would be equal, false otherwise.
      */
      friend bool
      operator==(const shuffle_order_engine& __lhs,
		 const shuffle_order_engine& __rhs)
      { return (__lhs._M_b == __rhs._M_b
		&& std::equal(__lhs._M_v, __lhs._M_v + __k, __rhs._M_v)
		&& __lhs._M_y == __rhs._M_y); }

      /**
       * @brief Inserts the current state of a %shuffle_order_engine random
       *        number generator engine @p __x into the output stream
	@p __os.
       *
       * @param __os An output stream.
       * @param __x  A %shuffle_order_engine random number generator engine.
       *
       * @returns The output stream with the state of @p __x inserted or in
       * an error state.
       */
      template<typename _RandomNumberEngine1, size_t __k1,
	       typename _CharT, typename _Traits>
	friend std::basic_ostream<_CharT, _Traits>&
	operator<<(std::basic_ostream<_CharT, _Traits>& __os,
		   const std::shuffle_order_engine<_RandomNumberEngine1,
		   __k1>& __x);

      /**
       * @brief Extracts the current state of a % subtract_with_carry_engine
       *        random number generator engine @p __x from the input stream
       *        @p __is.
       *
       * @param __is An input stream.
       * @param __x  A %shuffle_order_engine random number generator engine.
       *
       * @returns The input stream with the state of @p __x extracted or in
       * an error state.
       */
      template<typename _RandomNumberEngine1, size_t __k1,
	       typename _CharT, typename _Traits>
	friend std::basic_istream<_CharT, _Traits>&
	operator>>(std::basic_istream<_CharT, _Traits>& __is,
		   std::shuffle_order_engine<_RandomNumberEngine1, __k1>& __x);

    private:
      void _M_initialize()
      {
	for (size_t __i = 0; __i < __k; ++__i)
	  _M_v[__i] = _M_b();
	_M_y = _M_b();
      }

      _RandomNumberEngine _M_b;
      result_type _M_v[__k];
      result_type _M_y;
    };

  /**
   * Compares two %shuffle_order_engine random number generator objects
   * of the same type for inequality.
   *
   * @param __lhs A %shuffle_order_engine random number generator object.
   * @param __rhs Another %shuffle_order_engine random number generator
   *              object.
   *
   * @returns true if the infinite sequences of generated values
   *          would be different, false otherwise.
   */
  template<typename _RandomNumberEngine, size_t __k>
    inline bool
    operator!=(const std::shuffle_order_engine<_RandomNumberEngine,
	       __k>& __lhs,
	       const std::shuffle_order_engine<_RandomNumberEngine,
	       __k>& __rhs)
    { return !(__lhs == __rhs); }


  /**
   * The classic Minimum Standard rand0 of Lewis, Goodman, and Miller.
   */
  typedef linear_congruential_engine<uint_fast32_t, 16807UL, 0UL, 2147483647UL>
  minstd_rand0;

  /**
   * An alternative LCR (Lehmer Generator function).
   */
  typedef linear_congruential_engine<uint_fast32_t, 48271UL, 0UL, 2147483647UL>
  minstd_rand;

  /**
   * The classic Mersenne Twister.
   *
   * Reference:
   * M. Matsumoto and T. Nishimura, Mersenne Twister: A 623-Dimensionally
   * Equidistributed Uniform Pseudo-Random Number Generator, ACM Transactions
   * on Modeling and Computer Simulation, Vol. 8, No. 1, January 1998, pp 3-30.
   */
  typedef mersenne_twister_engine<
    uint_fast32_t,
    32, 624, 397, 31,
    0x9908b0dfUL, 11,
    0xffffffffUL, 7,
    0x9d2c5680UL, 15,
    0xefc60000UL, 18, 1812433253UL> mt19937;

  /**
   * An alternative Mersenne Twister.
   */
  typedef mersenne_twister_engine<
    uint_fast64_t,
    64, 312, 156, 31,
    0xb5026f5aa96619e9ULL, 29,
    0x5555555555555555ULL, 17,
    0x71d67fffeda60000ULL, 37,
    0xfff7eee000000000ULL, 43,
    6364136223846793005ULL> mt19937_64;

  typedef subtract_with_carry_engine<uint_fast32_t, 24, 10, 24>
    ranlux24_base;

  typedef subtract_with_carry_engine<uint_fast64_t, 48, 5, 12>
    ranlux48_base;

  typedef discard_block_engine<ranlux24_base, 223, 23> ranlux24;

  typedef discard_block_engine<ranlux48_base, 389, 11> ranlux48;

  typedef shuffle_order_engine<minstd_rand0, 256> knuth_b;

  typedef minstd_rand0 default_random_engine;

  /**
   * A standard interface to a platform-specific non-deterministic
   * random number generator (if any are available).
   */
  class random_device
  {
  public:
    /** The type of the generated random value. */
    typedef unsigned int result_type;

    // constructors, destructors and member functions

    random_device() { _M_init("default"); }

    explicit
    random_device(const std::string& __token) { _M_init(__token); }

#if defined _GLIBCXX_USE_DEV_RANDOM
    ~random_device()
    { _M_fini(); }
#endif

    static constexpr result_type
    min()
    { return std::numeric_limits<result_type>::min(); }

    static constexpr result_type
    max()
    { return std::numeric_limits<result_type>::max(); }

    double
    entropy() const noexcept
    {
#ifdef _GLIBCXX_USE_DEV_RANDOM
      return this->_M_getentropy();
#else
      return 0.0;
#endif
    }

    result_type
    operator()()
    { return this->_M_getval(); }

    // No copy functions.
    random_device(const random_device&) = delete;
    void operator=(const random_device&) = delete;

  private:

    void _M_init(const std::string& __token);
    void _M_init_pretr1(const std::string& __token);
    void _M_fini();

    result_type _M_getval();
    result_type _M_getval_pretr1();
    double _M_getentropy() const noexcept;

    void _M_init(const char*, size_t); // not exported from the shared library

    union
    {
      struct
      {
	void*      _M_file;
	result_type (*_M_func)(void*);
	int _M_fd;
      };
      mt19937    _M_mt;
    };
  };

  /* @} */ // group random_generators

  /**
   * @addtogroup random_distributions Random Number Distributions
   * @ingroup random
   * @{
   */

  /**
   * @addtogroup random_distributions_uniform Uniform Distributions
   * @ingroup random_distributions
   * @{
   */

  // std::uniform_int_distribution is defined in <bits/uniform_int_dist.h>

  /**
   * @brief Return true if two uniform integer distributions have
   *        different parameters.
   */
  template<typename _IntType>
    inline bool
    operator!=(const std::uniform_int_distribution<_IntType>& __d1,
	       const std::uniform_int_distribution<_IntType>& __d2)
    { return !(__d1 == __d2); }

  /**
   * @brief Inserts a %uniform_int_distribution random number
   *        distribution @p __x into the output stream @p os.
   *
   * @param __os An output stream.
   * @param __x  A %uniform_int_distribution random number distribution.
   *
   * @returns The output stream with the state of @p __x inserted or in
   * an error state.
   */
  template<typename _IntType, typename _CharT, typename _Traits>
    std::basic_ostream<_CharT, _Traits>&
    operator<<(std::basic_ostream<_CharT, _Traits>&,
	       const std::uniform_int_distribution<_IntType>&);

  /**
   * @brief Extracts a %uniform_int_distribution random number distribution
   * @p __x from the input stream @p __is.
   *
   * @param __is An input stream.
   * @param __x  A %uniform_int_distribution random number generator engine.
   *
   * @returns The input stream with @p __x extracted or in an error state.
   */
  template<typename _IntType, typename _CharT, typename _Traits>
    std::basic_istream<_CharT, _Traits>&
    operator>>(std::basic_istream<_CharT, _Traits>&,
	       std::uniform_int_distribution<_IntType>&);


  /**
   * @brief Uniform continuous distribution for random numbers.
   *
   * A continuous random distribution on the range [min, max) with equal
   * probability throughout the range.  The URNG should be real-valued and
   * deliver number in the range [0, 1).
   */
  template<typename _RealType = double>
    class uniform_real_distribution
    {
      static_assert(std::is_floating_point<_RealType>::value,
		    "result_type must be a floating point type");

    public:
      /** The type of the range of the distribution. */
      typedef _RealType result_type;

      /** Parameter type. */
      struct param_type
      {
	typedef uniform_real_distribution<_RealType> distribution_type;

	param_type() : param_type(0) { }

	explicit
	param_type(_RealType __a, _RealType __b = _RealType(1))
	: _M_a(__a), _M_b(__b)
	{
	  __glibcxx_assert(_M_a <= _M_b);
	}

	result_type
	a() const
	{ return _M_a; }

	result_type
	b() const
	{ return _M_b; }

	friend bool
	operator==(const param_type& __p1, const param_type& __p2)
	{ return __p1._M_a == __p2._M_a && __p1._M_b == __p2._M_b; }

	friend bool
	operator!=(const param_type& __p1, const param_type& __p2)
	{ return !(__p1 == __p2); }

      private:
	_RealType _M_a;
	_RealType _M_b;
      };

    public:
      /**
       * @brief Constructs a uniform_real_distribution object.
       *
       * The lower bound is set to 0.0 and the upper bound to 1.0
       */
      uniform_real_distribution() : uniform_real_distribution(0.0) { }

      /**
       * @brief Constructs a uniform_real_distribution object.
       *
       * @param __a [IN]  The lower bound of the distribution.
       * @param __b [IN]  The upper bound of the distribution.
       */
      explicit
      uniform_real_distribution(_RealType __a, _RealType __b = _RealType(1))
      : _M_param(__a, __b)
      { }

      explicit
      uniform_real_distribution(const param_type& __p)
      : _M_param(__p)
      { }

      /**
       * @brief Resets the distribution state.
       *
       * Does nothing for the uniform real distribution.
       */
      void
      reset() { }

      result_type
      a() const
      { return _M_param.a(); }

      result_type
      b() const
      { return _M_param.b(); }

      /**
       * @brief Returns the parameter set of the distribution.
       */
      param_type
      param() const
      { return _M_param; }

      /**
       * @brief Sets the parameter set of the distribution.
       * @param __param The new parameter set of the distribution.
       */
      void
      param(const param_type& __param)
      { _M_param = __param; }

      /**
       * @brief Returns the inclusive lower bound of the distribution range.
       */
      result_type
      min() const
      { return this->a(); }

      /**
       * @brief Returns the inclusive upper bound of the distribution range.
       */
      result_type
      max() const
      { return this->b(); }

      /**
       * @brief Generating functions.
       */
      template<typename _UniformRandomNumberGenerator>
	result_type
	operator()(_UniformRandomNumberGenerator& __urng)
        { return this->operator()(__urng, _M_param); }

      template<typename _UniformRandomNumberGenerator>
	result_type
	operator()(_UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
	{
	  __detail::_Adaptor<_UniformRandomNumberGenerator, result_type>
	    __aurng(__urng);
	  return (__aurng() * (__p.b() - __p.a())) + __p.a();
	}

      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate(_ForwardIterator __f, _ForwardIterator __t,
		   _UniformRandomNumberGenerator& __urng)
	{ this->__generate(__f, __t, __urng, _M_param); }

      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate(_ForwardIterator __f, _ForwardIterator __t,
		   _UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
	{ this->__generate_impl(__f, __t, __urng, __p); }

      template<typename _UniformRandomNumberGenerator>
	void
	__generate(result_type* __f, result_type* __t,
		   _UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
	{ this->__generate_impl(__f, __t, __urng, __p); }

      /**
       * @brief Return true if two uniform real distributions have
       *        the same parameters.
       */
      friend bool
      operator==(const uniform_real_distribution& __d1,
		 const uniform_real_distribution& __d2)
      { return __d1._M_param == __d2._M_param; }

    private:
      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate_impl(_ForwardIterator __f, _ForwardIterator __t,
			_UniformRandomNumberGenerator& __urng,
			const param_type& __p);

      param_type _M_param;
    };

  /**
   * @brief Return true if two uniform real distributions have
   *        different parameters.
   */
  template<typename _IntType>
    inline bool
    operator!=(const std::uniform_real_distribution<_IntType>& __d1,
	       const std::uniform_real_distribution<_IntType>& __d2)
    { return !(__d1 == __d2); }

  /**
   * @brief Inserts a %uniform_real_distribution random number
   *        distribution @p __x into the output stream @p __os.
   *
   * @param __os An output stream.
   * @param __x  A %uniform_real_distribution random number distribution.
   *
   * @returns The output stream with the state of @p __x inserted or in
   *          an error state.
   */
  template<typename _RealType, typename _CharT, typename _Traits>
    std::basic_ostream<_CharT, _Traits>&
    operator<<(std::basic_ostream<_CharT, _Traits>&,
	       const std::uniform_real_distribution<_RealType>&);

  /**
   * @brief Extracts a %uniform_real_distribution random number distribution
   * @p __x from the input stream @p __is.
   *
   * @param __is An input stream.
   * @param __x  A %uniform_real_distribution random number generator engine.
   *
   * @returns The input stream with @p __x extracted or in an error state.
   */
  template<typename _RealType, typename _CharT, typename _Traits>
    std::basic_istream<_CharT, _Traits>&
    operator>>(std::basic_istream<_CharT, _Traits>&,
	       std::uniform_real_distribution<_RealType>&);

  /* @} */ // group random_distributions_uniform

  /**
   * @addtogroup random_distributions_normal Normal Distributions
   * @ingroup random_distributions
   * @{
   */

  /**
   * @brief A normal continuous distribution for random numbers.
   *
   * The formula for the normal probability density function is
   * @f[
   *     p(x|\mu,\sigma) = \frac{1}{\sigma \sqrt{2 \pi}}
   *            e^{- \frac{{x - \mu}^ {2}}{2 \sigma ^ {2}} } 
   * @f]
   */
  template<typename _RealType = double>
    class normal_distribution
    {
      static_assert(std::is_floating_point<_RealType>::value,
		    "result_type must be a floating point type");

    public:
      /** The type of the range of the distribution. */
      typedef _RealType result_type;

      /** Parameter type. */
      struct param_type
      {
	typedef normal_distribution<_RealType> distribution_type;

	param_type() : param_type(0.0) { }

	explicit
	param_type(_RealType __mean, _RealType __stddev = _RealType(1))
	: _M_mean(__mean), _M_stddev(__stddev)
	{
	  __glibcxx_assert(_M_stddev > _RealType(0));
	}

	_RealType
	mean() const
	{ return _M_mean; }

	_RealType
	stddev() const
	{ return _M_stddev; }

	friend bool
	operator==(const param_type& __p1, const param_type& __p2)
	{ return (__p1._M_mean == __p2._M_mean
		  && __p1._M_stddev == __p2._M_stddev); }

	friend bool
	operator!=(const param_type& __p1, const param_type& __p2)
	{ return !(__p1 == __p2); }

      private:
	_RealType _M_mean;
	_RealType _M_stddev;
      };

    public:
      normal_distribution() : normal_distribution(0.0) { }

      /**
       * Constructs a normal distribution with parameters @f$mean@f$ and
       * standard deviation.
       */
      explicit
      normal_distribution(result_type __mean,
			  result_type __stddev = result_type(1))
      : _M_param(__mean, __stddev), _M_saved_available(false)
      { }

      explicit
      normal_distribution(const param_type& __p)
      : _M_param(__p), _M_saved_available(false)
      { }

      /**
       * @brief Resets the distribution state.
       */
      void
      reset()
      { _M_saved_available = false; }

      /**
       * @brief Returns the mean of the distribution.
       */
      _RealType
      mean() const
      { return _M_param.mean(); }

      /**
       * @brief Returns the standard deviation of the distribution.
       */
      _RealType
      stddev() const
      { return _M_param.stddev(); }

      /**
       * @brief Returns the parameter set of the distribution.
       */
      param_type
      param() const
      { return _M_param; }

      /**
       * @brief Sets the parameter set of the distribution.
       * @param __param The new parameter set of the distribution.
       */
      void
      param(const param_type& __param)
      { _M_param = __param; }

      /**
       * @brief Returns the greatest lower bound value of the distribution.
       */
      result_type
      min() const
      { return std::numeric_limits<result_type>::lowest(); }

      /**
       * @brief Returns the least upper bound value of the distribution.
       */
      result_type
      max() const
      { return std::numeric_limits<result_type>::max(); }

      /**
       * @brief Generating functions.
       */
      template<typename _UniformRandomNumberGenerator>
	result_type
	operator()(_UniformRandomNumberGenerator& __urng)
	{ return this->operator()(__urng, _M_param); }

      template<typename _UniformRandomNumberGenerator>
	result_type
	operator()(_UniformRandomNumberGenerator& __urng,
		   const param_type& __p);

      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate(_ForwardIterator __f, _ForwardIterator __t,
		   _UniformRandomNumberGenerator& __urng)
	{ this->__generate(__f, __t, __urng, _M_param); }

      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate(_ForwardIterator __f, _ForwardIterator __t,
		   _UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
	{ this->__generate_impl(__f, __t, __urng, __p); }

      template<typename _UniformRandomNumberGenerator>
	void
	__generate(result_type* __f, result_type* __t,
		   _UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
	{ this->__generate_impl(__f, __t, __urng, __p); }

      /**
       * @brief Return true if two normal distributions have
       *        the same parameters and the sequences that would
       *        be generated are equal.
       */
      template<typename _RealType1>
	friend bool
        operator==(const std::normal_distribution<_RealType1>& __d1,
		   const std::normal_distribution<_RealType1>& __d2);

      /**
       * @brief Inserts a %normal_distribution random number distribution
       * @p __x into the output stream @p __os.
       *
       * @param __os An output stream.
       * @param __x  A %normal_distribution random number distribution.
       *
       * @returns The output stream with the state of @p __x inserted or in
       * an error state.
       */
      template<typename _RealType1, typename _CharT, typename _Traits>
	friend std::basic_ostream<_CharT, _Traits>&
	operator<<(std::basic_ostream<_CharT, _Traits>& __os,
		   const std::normal_distribution<_RealType1>& __x);

      /**
       * @brief Extracts a %normal_distribution random number distribution
       * @p __x from the input stream @p __is.
       *
       * @param __is An input stream.
       * @param __x  A %normal_distribution random number generator engine.
       *
       * @returns The input stream with @p __x extracted or in an error
       *          state.
       */
      template<typename _RealType1, typename _CharT, typename _Traits>
	friend std::basic_istream<_CharT, _Traits>&
	operator>>(std::basic_istream<_CharT, _Traits>& __is,
		   std::normal_distribution<_RealType1>& __x);

    private:
      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate_impl(_ForwardIterator __f, _ForwardIterator __t,
			_UniformRandomNumberGenerator& __urng,
			const param_type& __p);

      param_type  _M_param;
      result_type _M_saved;
      bool        _M_saved_available;
    };

  /**
   * @brief Return true if two normal distributions are different.
   */
  template<typename _RealType>
    inline bool
    operator!=(const std::normal_distribution<_RealType>& __d1,
	       const std::normal_distribution<_RealType>& __d2)
    { return !(__d1 == __d2); }


  /**
   * @brief A lognormal_distribution random number distribution.
   *
   * The formula for the normal probability mass function is
   * @f[
   *     p(x|m,s) = \frac{1}{sx\sqrt{2\pi}}
   *                \exp{-\frac{(\ln{x} - m)^2}{2s^2}} 
   * @f]
   */
  template<typename _RealType = double>
    class lognormal_distribution
    {
      static_assert(std::is_floating_point<_RealType>::value,
		    "result_type must be a floating point type");

    public:
      /** The type of the range of the distribution. */
      typedef _RealType result_type;

      /** Parameter type. */
      struct param_type
      {
	typedef lognormal_distribution<_RealType> distribution_type;

	param_type() : param_type(0.0) { }

	explicit
	param_type(_RealType __m, _RealType __s = _RealType(1))
	: _M_m(__m), _M_s(__s)
	{ }

	_RealType
	m() const
	{ return _M_m; }

	_RealType
	s() const
	{ return _M_s; }

	friend bool
	operator==(const param_type& __p1, const param_type& __p2)
	{ return __p1._M_m == __p2._M_m && __p1._M_s == __p2._M_s; }

	friend bool
	operator!=(const param_type& __p1, const param_type& __p2)
	{ return !(__p1 == __p2); }

      private:
	_RealType _M_m;
	_RealType _M_s;
      };

      lognormal_distribution() : lognormal_distribution(0.0) { }

      explicit
      lognormal_distribution(_RealType __m, _RealType __s = _RealType(1))
      : _M_param(__m, __s), _M_nd()
      { }

      explicit
      lognormal_distribution(const param_type& __p)
      : _M_param(__p), _M_nd()
      { }

      /**
       * Resets the distribution state.
       */
      void
      reset()
      { _M_nd.reset(); }

      /**
       *
       */
      _RealType
      m() const
      { return _M_param.m(); }

      _RealType
      s() const
      { return _M_param.s(); }

      /**
       * @brief Returns the parameter set of the distribution.
       */
      param_type
      param() const
      { return _M_param; }

      /**
       * @brief Sets the parameter set of the distribution.
       * @param __param The new parameter set of the distribution.
       */
      void
      param(const param_type& __param)
      { _M_param = __param; }

      /**
       * @brief Returns the greatest lower bound value of the distribution.
       */
      result_type
      min() const
      { return result_type(0); }

      /**
       * @brief Returns the least upper bound value of the distribution.
       */
      result_type
      max() const
      { return std::numeric_limits<result_type>::max(); }

      /**
       * @brief Generating functions.
       */
      template<typename _UniformRandomNumberGenerator>
	result_type
	operator()(_UniformRandomNumberGenerator& __urng)
        { return this->operator()(__urng, _M_param); }

      template<typename _UniformRandomNumberGenerator>
	result_type
	operator()(_UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
        { return std::exp(__p.s() * _M_nd(__urng) + __p.m()); }

      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate(_ForwardIterator __f, _ForwardIterator __t,
		   _UniformRandomNumberGenerator& __urng)
	{ this->__generate(__f, __t, __urng, _M_param); }

      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate(_ForwardIterator __f, _ForwardIterator __t,
		   _UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
	{ this->__generate_impl(__f, __t, __urng, __p); }

      template<typename _UniformRandomNumberGenerator>
	void
	__generate(result_type* __f, result_type* __t,
		   _UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
	{ this->__generate_impl(__f, __t, __urng, __p); }

      /**
       * @brief Return true if two lognormal distributions have
       *        the same parameters and the sequences that would
       *        be generated are equal.
       */
      friend bool
      operator==(const lognormal_distribution& __d1,
		 const lognormal_distribution& __d2)
      { return (__d1._M_param == __d2._M_param
		&& __d1._M_nd == __d2._M_nd); }

      /**
       * @brief Inserts a %lognormal_distribution random number distribution
       * @p __x into the output stream @p __os.
       *
       * @param __os An output stream.
       * @param __x  A %lognormal_distribution random number distribution.
       *
       * @returns The output stream with the state of @p __x inserted or in
       * an error state.
       */
      template<typename _RealType1, typename _CharT, typename _Traits>
	friend std::basic_ostream<_CharT, _Traits>&
	operator<<(std::basic_ostream<_CharT, _Traits>& __os,
		   const std::lognormal_distribution<_RealType1>& __x);

      /**
       * @brief Extracts a %lognormal_distribution random number distribution
       * @p __x from the input stream @p __is.
       *
       * @param __is An input stream.
       * @param __x A %lognormal_distribution random number
       *            generator engine.
       *
       * @returns The input stream with @p __x extracted or in an error state.
       */
      template<typename _RealType1, typename _CharT, typename _Traits>
	friend std::basic_istream<_CharT, _Traits>&
	operator>>(std::basic_istream<_CharT, _Traits>& __is,
		   std::lognormal_distribution<_RealType1>& __x);

    private:
      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate_impl(_ForwardIterator __f, _ForwardIterator __t,
			_UniformRandomNumberGenerator& __urng,
			const param_type& __p);

      param_type _M_param;

      std::normal_distribution<result_type> _M_nd;
    };

  /**
   * @brief Return true if two lognormal distributions are different.
   */
  template<typename _RealType>
    inline bool
    operator!=(const std::lognormal_distribution<_RealType>& __d1,
	       const std::lognormal_distribution<_RealType>& __d2)
    { return !(__d1 == __d2); }


  /**
   * @brief A gamma continuous distribution for random numbers.
   *
   * The formula for the gamma probability density function is:
   * @f[
   *     p(x|\alpha,\beta) = \frac{1}{\beta\Gamma(\alpha)}
   *                         (x/\beta)^{\alpha - 1} e^{-x/\beta} 
   * @f]
   */
  template<typename _RealType = double>
    class gamma_distribution
    {
      static_assert(std::is_floating_point<_RealType>::value,
		    "result_type must be a floating point type");

    public:
      /** The type of the range of the distribution. */
      typedef _RealType result_type;

      /** Parameter type. */
      struct param_type
      {
	typedef gamma_distribution<_RealType> distribution_type;
	friend class gamma_distribution<_RealType>;

	param_type() : param_type(1.0) { }

	explicit
	param_type(_RealType __alpha_val, _RealType __beta_val = _RealType(1))
	: _M_alpha(__alpha_val), _M_beta(__beta_val)
	{
	  __glibcxx_assert(_M_alpha > _RealType(0));
	  _M_initialize();
	}

	_RealType
	alpha() const
	{ return _M_alpha; }

	_RealType
	beta() const
	{ return _M_beta; }

	friend bool
	operator==(const param_type& __p1, const param_type& __p2)
	{ return (__p1._M_alpha == __p2._M_alpha
		  && __p1._M_beta == __p2._M_beta); }

	friend bool
	operator!=(const param_type& __p1, const param_type& __p2)
	{ return !(__p1 == __p2); }

      private:
	void
	_M_initialize();

	_RealType _M_alpha;
	_RealType _M_beta;

	_RealType _M_malpha, _M_a2;
      };

    public:
      /**
       * @brief Constructs a gamma distribution with parameters 1 and 1.
       */
      gamma_distribution() : gamma_distribution(1.0) { }

      /**
       * @brief Constructs a gamma distribution with parameters
       * @f$\alpha@f$ and @f$\beta@f$.
       */
      explicit
      gamma_distribution(_RealType __alpha_val,
			 _RealType __beta_val = _RealType(1))
      : _M_param(__alpha_val, __beta_val), _M_nd()
      { }

      explicit
      gamma_distribution(const param_type& __p)
      : _M_param(__p), _M_nd()
      { }

      /**
       * @brief Resets the distribution state.
       */
      void
      reset()
      { _M_nd.reset(); }

      /**
       * @brief Returns the @f$\alpha@f$ of the distribution.
       */
      _RealType
      alpha() const
      { return _M_param.alpha(); }

      /**
       * @brief Returns the @f$\beta@f$ of the distribution.
       */
      _RealType
      beta() const
      { return _M_param.beta(); }

      /**
       * @brief Returns the parameter set of the distribution.
       */
      param_type
      param() const
      { return _M_param; }

      /**
       * @brief Sets the parameter set of the distribution.
       * @param __param The new parameter set of the distribution.
       */
      void
      param(const param_type& __param)
      { _M_param = __param; }

      /**
       * @brief Returns the greatest lower bound value of the distribution.
       */
      result_type
      min() const
      { return result_type(0); }

      /**
       * @brief Returns the least upper bound value of the distribution.
       */
      result_type
      max() const
      { return std::numeric_limits<result_type>::max(); }

      /**
       * @brief Generating functions.
       */
      template<typename _UniformRandomNumberGenerator>
	result_type
	operator()(_UniformRandomNumberGenerator& __urng)
	{ return this->operator()(__urng, _M_param); }

      template<typename _UniformRandomNumberGenerator>
	result_type
	operator()(_UniformRandomNumberGenerator& __urng,
		   const param_type& __p);

      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate(_ForwardIterator __f, _ForwardIterator __t,
		   _UniformRandomNumberGenerator& __urng)
	{ this->__generate(__f, __t, __urng, _M_param); }

      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate(_ForwardIterator __f, _ForwardIterator __t,
		   _UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
	{ this->__generate_impl(__f, __t, __urng, __p); }

      template<typename _UniformRandomNumberGenerator>
	void
	__generate(result_type* __f, result_type* __t,
		   _UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
	{ this->__generate_impl(__f, __t, __urng, __p); }

      /**
       * @brief Return true if two gamma distributions have the same
       *        parameters and the sequences that would be generated
       *        are equal.
       */
      friend bool
      operator==(const gamma_distribution& __d1,
		 const gamma_distribution& __d2)
      { return (__d1._M_param == __d2._M_param
		&& __d1._M_nd == __d2._M_nd); }

      /**
       * @brief Inserts a %gamma_distribution random number distribution
       * @p __x into the output stream @p __os.
       *
       * @param __os An output stream.
       * @param __x  A %gamma_distribution random number distribution.
       *
       * @returns The output stream with the state of @p __x inserted or in
       * an error state.
       */
      template<typename _RealType1, typename _CharT, typename _Traits>
	friend std::basic_ostream<_CharT, _Traits>&
	operator<<(std::basic_ostream<_CharT, _Traits>& __os,
		   const std::gamma_distribution<_RealType1>& __x);

      /**
       * @brief Extracts a %gamma_distribution random number distribution
       * @p __x from the input stream @p __is.
       *
       * @param __is An input stream.
       * @param __x  A %gamma_distribution random number generator engine.
       *
       * @returns The input stream with @p __x extracted or in an error state.
       */
      template<typename _RealType1, typename _CharT, typename _Traits>
	friend std::basic_istream<_CharT, _Traits>&
	operator>>(std::basic_istream<_CharT, _Traits>& __is,
		   std::gamma_distribution<_RealType1>& __x);

    private:
      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate_impl(_ForwardIterator __f, _ForwardIterator __t,
			_UniformRandomNumberGenerator& __urng,
			const param_type& __p);

      param_type _M_param;

      std::normal_distribution<result_type> _M_nd;
    };

  /**
   * @brief Return true if two gamma distributions are different.
   */
   template<typename _RealType>
     inline bool
     operator!=(const std::gamma_distribution<_RealType>& __d1,
		const std::gamma_distribution<_RealType>& __d2)
    { return !(__d1 == __d2); }


  /**
   * @brief A chi_squared_distribution random number distribution.
   *
   * The formula for the normal probability mass function is
   * @f$p(x|n) = \frac{x^{(n/2) - 1}e^{-x/2}}{\Gamma(n/2) 2^{n/2}}@f$
   */
  template<typename _RealType = double>
    class chi_squared_distribution
    {
      static_assert(std::is_floating_point<_RealType>::value,
		    "result_type must be a floating point type");

    public:
      /** The type of the range of the distribution. */
      typedef _RealType result_type;

      /** Parameter type. */
      struct param_type
      {
	typedef chi_squared_distribution<_RealType> distribution_type;

	param_type() : param_type(1) { }

	explicit
	param_type(_RealType __n)
	: _M_n(__n)
	{ }

	_RealType
	n() const
	{ return _M_n; }

	friend bool
	operator==(const param_type& __p1, const param_type& __p2)
	{ return __p1._M_n == __p2._M_n; }

	friend bool
	operator!=(const param_type& __p1, const param_type& __p2)
	{ return !(__p1 == __p2); }

      private:
	_RealType _M_n;
      };

      chi_squared_distribution() : chi_squared_distribution(1) { }

      explicit
      chi_squared_distribution(_RealType __n)
      : _M_param(__n), _M_gd(__n / 2)
      { }

      explicit
      chi_squared_distribution(const param_type& __p)
      : _M_param(__p), _M_gd(__p.n() / 2)
      { }

      /**
       * @brief Resets the distribution state.
       */
      void
      reset()
      { _M_gd.reset(); }

      /**
       *
       */
      _RealType
      n() const
      { return _M_param.n(); }

      /**
       * @brief Returns the parameter set of the distribution.
       */
      param_type
      param() const
      { return _M_param; }

      /**
       * @brief Sets the parameter set of the distribution.
       * @param __param The new parameter set of the distribution.
       */
      void
      param(const param_type& __param)
      {
	_M_param = __param;
	typedef typename std::gamma_distribution<result_type>::param_type
	  param_type;
	_M_gd.param(param_type{__param.n() / 2});
      }

      /**
       * @brief Returns the greatest lower bound value of the distribution.
       */
      result_type
      min() const
      { return result_type(0); }

      /**
       * @brief Returns the least upper bound value of the distribution.
       */
      result_type
      max() const
      { return std::numeric_limits<result_type>::max(); }

      /**
       * @brief Generating functions.
       */
      template<typename _UniformRandomNumberGenerator>
	result_type
	operator()(_UniformRandomNumberGenerator& __urng)
	{ return 2 * _M_gd(__urng); }

      template<typename _UniformRandomNumberGenerator>
	result_type
	operator()(_UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
        {
	  typedef typename std::gamma_distribution<result_type>::param_type
	    param_type;
	  return 2 * _M_gd(__urng, param_type(__p.n() / 2));
	}

      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate(_ForwardIterator __f, _ForwardIterator __t,
		   _UniformRandomNumberGenerator& __urng)
        { this->__generate_impl(__f, __t, __urng); }

      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate(_ForwardIterator __f, _ForwardIterator __t,
		   _UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
	{ typename std::gamma_distribution<result_type>::param_type
	    __p2(__p.n() / 2);
	  this->__generate_impl(__f, __t, __urng, __p2); }

      template<typename _UniformRandomNumberGenerator>
	void
	__generate(result_type* __f, result_type* __t,
		   _UniformRandomNumberGenerator& __urng)
        { this->__generate_impl(__f, __t, __urng); }

      template<typename _UniformRandomNumberGenerator>
	void
	__generate(result_type* __f, result_type* __t,
		   _UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
	{ typename std::gamma_distribution<result_type>::param_type
	    __p2(__p.n() / 2);
	  this->__generate_impl(__f, __t, __urng, __p2); }

      /**
       * @brief Return true if two Chi-squared distributions have
       *        the same parameters and the sequences that would be
       *        generated are equal.
       */
      friend bool
      operator==(const chi_squared_distribution& __d1,
		 const chi_squared_distribution& __d2)
      { return __d1._M_param == __d2._M_param && __d1._M_gd == __d2._M_gd; }

      /**
       * @brief Inserts a %chi_squared_distribution random number distribution
       * @p __x into the output stream @p __os.
       *
       * @param __os An output stream.
       * @param __x  A %chi_squared_distribution random number distribution.
       *
       * @returns The output stream with the state of @p __x inserted or in
       * an error state.
       */
      template<typename _RealType1, typename _CharT, typename _Traits>
	friend std::basic_ostream<_CharT, _Traits>&
	operator<<(std::basic_ostream<_CharT, _Traits>& __os,
		   const std::chi_squared_distribution<_RealType1>& __x);

      /**
       * @brief Extracts a %chi_squared_distribution random number distribution
       * @p __x from the input stream @p __is.
       *
       * @param __is An input stream.
       * @param __x A %chi_squared_distribution random number
       *            generator engine.
       *
       * @returns The input stream with @p __x extracted or in an error state.
       */
      template<typename _RealType1, typename _CharT, typename _Traits>
	friend std::basic_istream<_CharT, _Traits>&
	operator>>(std::basic_istream<_CharT, _Traits>& __is,
		   std::chi_squared_distribution<_RealType1>& __x);

    private:
      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate_impl(_ForwardIterator __f, _ForwardIterator __t,
			_UniformRandomNumberGenerator& __urng);

      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate_impl(_ForwardIterator __f, _ForwardIterator __t,
			_UniformRandomNumberGenerator& __urng,
			const typename
			std::gamma_distribution<result_type>::param_type& __p);

      param_type _M_param;

      std::gamma_distribution<result_type> _M_gd;
    };

  /**
   * @brief Return true if two Chi-squared distributions are different.
   */
  template<typename _RealType>
    inline bool
    operator!=(const std::chi_squared_distribution<_RealType>& __d1,
	       const std::chi_squared_distribution<_RealType>& __d2)
    { return !(__d1 == __d2); }


  /**
   * @brief A cauchy_distribution random number distribution.
   *
   * The formula for the normal probability mass function is
   * @f$p(x|a,b) = (\pi b (1 + (\frac{x-a}{b})^2))^{-1}@f$
   */
  template<typename _RealType = double>
    class cauchy_distribution
    {
      static_assert(std::is_floating_point<_RealType>::value,
		    "result_type must be a floating point type");

    public:
      /** The type of the range of the distribution. */
      typedef _RealType result_type;

      /** Parameter type. */
      struct param_type
      {
	typedef cauchy_distribution<_RealType> distribution_type;

	param_type() : param_type(0) { }

	explicit
	param_type(_RealType __a, _RealType __b = _RealType(1))
	: _M_a(__a), _M_b(__b)
	{ }

	_RealType
	a() const
	{ return _M_a; }

	_RealType
	b() const
	{ return _M_b; }

	friend bool
	operator==(const param_type& __p1, const param_type& __p2)
	{ return __p1._M_a == __p2._M_a && __p1._M_b == __p2._M_b; }

	friend bool
	operator!=(const param_type& __p1, const param_type& __p2)
	{ return !(__p1 == __p2); }

      private:
	_RealType _M_a;
	_RealType _M_b;
      };

      cauchy_distribution() : cauchy_distribution(0.0) { }

      explicit
      cauchy_distribution(_RealType __a, _RealType __b = 1.0)
      : _M_param(__a, __b)
      { }

      explicit
      cauchy_distribution(const param_type& __p)
      : _M_param(__p)
      { }

      /**
       * @brief Resets the distribution state.
       */
      void
      reset()
      { }

      /**
       *
       */
      _RealType
      a() const
      { return _M_param.a(); }

      _RealType
      b() const
      { return _M_param.b(); }

      /**
       * @brief Returns the parameter set of the distribution.
       */
      param_type
      param() const
      { return _M_param; }

      /**
       * @brief Sets the parameter set of the distribution.
       * @param __param The new parameter set of the distribution.
       */
      void
      param(const param_type& __param)
      { _M_param = __param; }

      /**
       * @brief Returns the greatest lower bound value of the distribution.
       */
      result_type
      min() const
      { return std::numeric_limits<result_type>::lowest(); }

      /**
       * @brief Returns the least upper bound value of the distribution.
       */
      result_type
      max() const
      { return std::numeric_limits<result_type>::max(); }

      /**
       * @brief Generating functions.
       */
      template<typename _UniformRandomNumberGenerator>
	result_type
	operator()(_UniformRandomNumberGenerator& __urng)
	{ return this->operator()(__urng, _M_param); }

      template<typename _UniformRandomNumberGenerator>
	result_type
	operator()(_UniformRandomNumberGenerator& __urng,
		   const param_type& __p);

      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate(_ForwardIterator __f, _ForwardIterator __t,
		   _UniformRandomNumberGenerator& __urng)
	{ this->__generate(__f, __t, __urng, _M_param); }

      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate(_ForwardIterator __f, _ForwardIterator __t,
		   _UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
	{ this->__generate_impl(__f, __t, __urng, __p); }

      template<typename _UniformRandomNumberGenerator>
	void
	__generate(result_type* __f, result_type* __t,
		   _UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
	{ this->__generate_impl(__f, __t, __urng, __p); }

      /**
       * @brief Return true if two Cauchy distributions have
       *        the same parameters.
       */
      friend bool
      operator==(const cauchy_distribution& __d1,
		 const cauchy_distribution& __d2)
      { return __d1._M_param == __d2._M_param; }

    private:
      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate_impl(_ForwardIterator __f, _ForwardIterator __t,
			_UniformRandomNumberGenerator& __urng,
			const param_type& __p);

      param_type _M_param;
    };

  /**
   * @brief Return true if two Cauchy distributions have
   *        different parameters.
   */
  template<typename _RealType>
    inline bool
    operator!=(const std::cauchy_distribution<_RealType>& __d1,
	       const std::cauchy_distribution<_RealType>& __d2)
    { return !(__d1 == __d2); }

  /**
   * @brief Inserts a %cauchy_distribution random number distribution
   * @p __x into the output stream @p __os.
   *
   * @param __os An output stream.
   * @param __x  A %cauchy_distribution random number distribution.
   *
   * @returns The output stream with the state of @p __x inserted or in
   * an error state.
   */
  template<typename _RealType, typename _CharT, typename _Traits>
    std::basic_ostream<_CharT, _Traits>&
    operator<<(std::basic_ostream<_CharT, _Traits>& __os,
	       const std::cauchy_distribution<_RealType>& __x);

  /**
   * @brief Extracts a %cauchy_distribution random number distribution
   * @p __x from the input stream @p __is.
   *
   * @param __is An input stream.
   * @param __x A %cauchy_distribution random number
   *            generator engine.
   *
   * @returns The input stream with @p __x extracted or in an error state.
   */
  template<typename _RealType, typename _CharT, typename _Traits>
    std::basic_istream<_CharT, _Traits>&
    operator>>(std::basic_istream<_CharT, _Traits>& __is,
	       std::cauchy_distribution<_RealType>& __x);


  /**
   * @brief A fisher_f_distribution random number distribution.
   *
   * The formula for the normal probability mass function is
   * @f[
   *     p(x|m,n) = \frac{\Gamma((m+n)/2)}{\Gamma(m/2)\Gamma(n/2)}
   *                (\frac{m}{n})^{m/2} x^{(m/2)-1}
   *                (1 + \frac{mx}{n})^{-(m+n)/2} 
   * @f]
   */
  template<typename _RealType = double>
    class fisher_f_distribution
    {
      static_assert(std::is_floating_point<_RealType>::value,
		    "result_type must be a floating point type");

    public:
      /** The type of the range of the distribution. */
      typedef _RealType result_type;

      /** Parameter type. */
      struct param_type
      {
	typedef fisher_f_distribution<_RealType> distribution_type;

	param_type() : param_type(1) { }

	explicit
	param_type(_RealType __m, _RealType __n = _RealType(1))
	: _M_m(__m), _M_n(__n)
	{ }

	_RealType
	m() const
	{ return _M_m; }

	_RealType
	n() const
	{ return _M_n; }

	friend bool
	operator==(const param_type& __p1, const param_type& __p2)
	{ return __p1._M_m == __p2._M_m && __p1._M_n == __p2._M_n; }

	friend bool
	operator!=(const param_type& __p1, const param_type& __p2)
	{ return !(__p1 == __p2); }

      private:
	_RealType _M_m;
	_RealType _M_n;
      };

      fisher_f_distribution() : fisher_f_distribution(1.0) { }

      explicit
      fisher_f_distribution(_RealType __m,
			    _RealType __n = _RealType(1))
      : _M_param(__m, __n), _M_gd_x(__m / 2), _M_gd_y(__n / 2)
      { }

      explicit
      fisher_f_distribution(const param_type& __p)
      : _M_param(__p), _M_gd_x(__p.m() / 2), _M_gd_y(__p.n() / 2)
      { }

      /**
       * @brief Resets the distribution state.
       */
      void
      reset()
      {
	_M_gd_x.reset();
	_M_gd_y.reset();
      }

      /**
       *
       */
      _RealType
      m() const
      { return _M_param.m(); }

      _RealType
      n() const
      { return _M_param.n(); }

      /**
       * @brief Returns the parameter set of the distribution.
       */
      param_type
      param() const
      { return _M_param; }

      /**
       * @brief Sets the parameter set of the distribution.
       * @param __param The new parameter set of the distribution.
       */
      void
      param(const param_type& __param)
      { _M_param = __param; }

      /**
       * @brief Returns the greatest lower bound value of the distribution.
       */
      result_type
      min() const
      { return result_type(0); }

      /**
       * @brief Returns the least upper bound value of the distribution.
       */
      result_type
      max() const
      { return std::numeric_limits<result_type>::max(); }

      /**
       * @brief Generating functions.
       */
      template<typename _UniformRandomNumberGenerator>
	result_type
	operator()(_UniformRandomNumberGenerator& __urng)
	{ return (_M_gd_x(__urng) * n()) / (_M_gd_y(__urng) * m()); }

      template<typename _UniformRandomNumberGenerator>
	result_type
	operator()(_UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
        {
	  typedef typename std::gamma_distribution<result_type>::param_type
	    param_type;
	  return ((_M_gd_x(__urng, param_type(__p.m() / 2)) * n())
		  / (_M_gd_y(__urng, param_type(__p.n() / 2)) * m()));
	}

      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate(_ForwardIterator __f, _ForwardIterator __t,
		   _UniformRandomNumberGenerator& __urng)
	{ this->__generate_impl(__f, __t, __urng); }

      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate(_ForwardIterator __f, _ForwardIterator __t,
		   _UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
	{ this->__generate_impl(__f, __t, __urng, __p); }

      template<typename _UniformRandomNumberGenerator>
	void
	__generate(result_type* __f, result_type* __t,
		   _UniformRandomNumberGenerator& __urng)
	{ this->__generate_impl(__f, __t, __urng); }

      template<typename _UniformRandomNumberGenerator>
	void
	__generate(result_type* __f, result_type* __t,
		   _UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
	{ this->__generate_impl(__f, __t, __urng, __p); }

      /**
       * @brief Return true if two Fisher f distributions have
       *        the same parameters and the sequences that would
       *        be generated are equal.
       */
      friend bool
      operator==(const fisher_f_distribution& __d1,
		 const fisher_f_distribution& __d2)
      { return (__d1._M_param == __d2._M_param
		&& __d1._M_gd_x == __d2._M_gd_x
		&& __d1._M_gd_y == __d2._M_gd_y); }

      /**
       * @brief Inserts a %fisher_f_distribution random number distribution
       * @p __x into the output stream @p __os.
       *
       * @param __os An output stream.
       * @param __x  A %fisher_f_distribution random number distribution.
       *
       * @returns The output stream with the state of @p __x inserted or in
       * an error state.
       */
      template<typename _RealType1, typename _CharT, typename _Traits>
	friend std::basic_ostream<_CharT, _Traits>&
	operator<<(std::basic_ostream<_CharT, _Traits>& __os,
		   const std::fisher_f_distribution<_RealType1>& __x);

      /**
       * @brief Extracts a %fisher_f_distribution random number distribution
       * @p __x from the input stream @p __is.
       *
       * @param __is An input stream.
       * @param __x A %fisher_f_distribution random number
       *            generator engine.
       *
       * @returns The input stream with @p __x extracted or in an error state.
       */
      template<typename _RealType1, typename _CharT, typename _Traits>
	friend std::basic_istream<_CharT, _Traits>&
	operator>>(std::basic_istream<_CharT, _Traits>& __is,
		   std::fisher_f_distribution<_RealType1>& __x);

    private:
      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate_impl(_ForwardIterator __f, _ForwardIterator __t,
			_UniformRandomNumberGenerator& __urng);

      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate_impl(_ForwardIterator __f, _ForwardIterator __t,
			_UniformRandomNumberGenerator& __urng,
			const param_type& __p);

      param_type _M_param;

      std::gamma_distribution<result_type> _M_gd_x, _M_gd_y;
    };

  /**
   * @brief Return true if two Fisher f distributions are different.
   */
  template<typename _RealType>
    inline bool
    operator!=(const std::fisher_f_distribution<_RealType>& __d1,
	       const std::fisher_f_distribution<_RealType>& __d2)
    { return !(__d1 == __d2); }

  /**
   * @brief A student_t_distribution random number distribution.
   *
   * The formula for the normal probability mass function is:
   * @f[
   *     p(x|n) = \frac{1}{\sqrt(n\pi)} \frac{\Gamma((n+1)/2)}{\Gamma(n/2)}
   *              (1 + \frac{x^2}{n}) ^{-(n+1)/2} 
   * @f]
   */
  template<typename _RealType = double>
    class student_t_distribution
    {
      static_assert(std::is_floating_point<_RealType>::value,
		    "result_type must be a floating point type");

    public:
      /** The type of the range of the distribution. */
      typedef _RealType result_type;

      /** Parameter type. */
      struct param_type
      {
	typedef student_t_distribution<_RealType> distribution_type;

	param_type() : param_type(1) { }

	explicit
	param_type(_RealType __n)
	: _M_n(__n)
	{ }

	_RealType
	n() const
	{ return _M_n; }

	friend bool
	operator==(const param_type& __p1, const param_type& __p2)
	{ return __p1._M_n == __p2._M_n; }

	friend bool
	operator!=(const param_type& __p1, const param_type& __p2)
	{ return !(__p1 == __p2); }

      private:
	_RealType _M_n;
      };

      student_t_distribution() : student_t_distribution(1.0) { }

      explicit
      student_t_distribution(_RealType __n)
      : _M_param(__n), _M_nd(), _M_gd(__n / 2, 2)
      { }

      explicit
      student_t_distribution(const param_type& __p)
      : _M_param(__p), _M_nd(), _M_gd(__p.n() / 2, 2)
      { }

      /**
       * @brief Resets the distribution state.
       */
      void
      reset()
      {
	_M_nd.reset();
	_M_gd.reset();
      }

      /**
       *
       */
      _RealType
      n() const
      { return _M_param.n(); }

      /**
       * @brief Returns the parameter set of the distribution.
       */
      param_type
      param() const
      { return _M_param; }

      /**
       * @brief Sets the parameter set of the distribution.
       * @param __param The new parameter set of the distribution.
       */
      void
      param(const param_type& __param)
      { _M_param = __param; }

      /**
       * @brief Returns the greatest lower bound value of the distribution.
       */
      result_type
      min() const
      { return std::numeric_limits<result_type>::lowest(); }

      /**
       * @brief Returns the least upper bound value of the distribution.
       */
      result_type
      max() const
      { return std::numeric_limits<result_type>::max(); }

      /**
       * @brief Generating functions.
       */
      template<typename _UniformRandomNumberGenerator>
	result_type
        operator()(_UniformRandomNumberGenerator& __urng)
        { return _M_nd(__urng) * std::sqrt(n() / _M_gd(__urng)); }

      template<typename _UniformRandomNumberGenerator>
	result_type
	operator()(_UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
        {
	  typedef typename std::gamma_distribution<result_type>::param_type
	    param_type;
	
	  const result_type __g = _M_gd(__urng, param_type(__p.n() / 2, 2));
	  return _M_nd(__urng) * std::sqrt(__p.n() / __g);
        }

      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate(_ForwardIterator __f, _ForwardIterator __t,
		   _UniformRandomNumberGenerator& __urng)
	{ this->__generate_impl(__f, __t, __urng); }

      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate(_ForwardIterator __f, _ForwardIterator __t,
		   _UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
	{ this->__generate_impl(__f, __t, __urng, __p); }

      template<typename _UniformRandomNumberGenerator>
	void
	__generate(result_type* __f, result_type* __t,
		   _UniformRandomNumberGenerator& __urng)
	{ this->__generate_impl(__f, __t, __urng); }

      template<typename _UniformRandomNumberGenerator>
	void
	__generate(result_type* __f, result_type* __t,
		   _UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
	{ this->__generate_impl(__f, __t, __urng, __p); }

      /**
       * @brief Return true if two Student t distributions have
       *        the same parameters and the sequences that would
       *        be generated are equal.
       */
      friend bool
      operator==(const student_t_distribution& __d1,
		 const student_t_distribution& __d2)
      { return (__d1._M_param == __d2._M_param
		&& __d1._M_nd == __d2._M_nd && __d1._M_gd == __d2._M_gd); }

      /**
       * @brief Inserts a %student_t_distribution random number distribution
       * @p __x into the output stream @p __os.
       *
       * @param __os An output stream.
       * @param __x  A %student_t_distribution random number distribution.
       *
       * @returns The output stream with the state of @p __x inserted or in
       * an error state.
       */
      template<typename _RealType1, typename _CharT, typename _Traits>
	friend std::basic_ostream<_CharT, _Traits>&
	operator<<(std::basic_ostream<_CharT, _Traits>& __os,
		   const std::student_t_distribution<_RealType1>& __x);

      /**
       * @brief Extracts a %student_t_distribution random number distribution
       * @p __x from the input stream @p __is.
       *
       * @param __is An input stream.
       * @param __x A %student_t_distribution random number
       *            generator engine.
       *
       * @returns The input stream with @p __x extracted or in an error state.
       */
      template<typename _RealType1, typename _CharT, typename _Traits>
	friend std::basic_istream<_CharT, _Traits>&
	operator>>(std::basic_istream<_CharT, _Traits>& __is,
		   std::student_t_distribution<_RealType1>& __x);

    private:
      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate_impl(_ForwardIterator __f, _ForwardIterator __t,
			_UniformRandomNumberGenerator& __urng);
      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate_impl(_ForwardIterator __f, _ForwardIterator __t,
			_UniformRandomNumberGenerator& __urng,
			const param_type& __p);

      param_type _M_param;

      std::normal_distribution<result_type> _M_nd;
      std::gamma_distribution<result_type> _M_gd;
    };

  /**
   * @brief Return true if two Student t distributions are different.
   */
  template<typename _RealType>
    inline bool
    operator!=(const std::student_t_distribution<_RealType>& __d1,
	       const std::student_t_distribution<_RealType>& __d2)
    { return !(__d1 == __d2); }


  /* @} */ // group random_distributions_normal

  /**
   * @addtogroup random_distributions_bernoulli Bernoulli Distributions
   * @ingroup random_distributions
   * @{
   */

  /**
   * @brief A Bernoulli random number distribution.
   *
   * Generates a sequence of true and false values with likelihood @f$p@f$
   * that true will come up and @f$(1 - p)@f$ that false will appear.
   */
  class bernoulli_distribution
  {
  public:
    /** The type of the range of the distribution. */
    typedef bool result_type;

    /** Parameter type. */
    struct param_type
    {
      typedef bernoulli_distribution distribution_type;

      param_type() : param_type(0.5) { }

      explicit
      param_type(double __p)
      : _M_p(__p)
      {
	__glibcxx_assert((_M_p >= 0.0) && (_M_p <= 1.0));
      }

      double
      p() const
      { return _M_p; }

      friend bool
      operator==(const param_type& __p1, const param_type& __p2)
      { return __p1._M_p == __p2._M_p; }

      friend bool
      operator!=(const param_type& __p1, const param_type& __p2)
      { return !(__p1 == __p2); }

    private:
      double _M_p;
    };

  public:
    /**
     * @brief Constructs a Bernoulli distribution with likelihood 0.5.
     */
    bernoulli_distribution() : bernoulli_distribution(0.5) { }

    /**
     * @brief Constructs a Bernoulli distribution with likelihood @p p.
     *
     * @param __p  [IN]  The likelihood of a true result being returned.
     *                   Must be in the interval @f$[0, 1]@f$.
     */
    explicit
    bernoulli_distribution(double __p)
    : _M_param(__p)
    { }

    explicit
    bernoulli_distribution(const param_type& __p)
    : _M_param(__p)
    { }

    /**
     * @brief Resets the distribution state.
     *
     * Does nothing for a Bernoulli distribution.
     */
    void
    reset() { }

    /**
     * @brief Returns the @p p parameter of the distribution.
     */
    double
    p() const
    { return _M_param.p(); }

    /**
     * @brief Returns the parameter set of the distribution.
     */
    param_type
    param() const
    { return _M_param; }

    /**
     * @brief Sets the parameter set of the distribution.
     * @param __param The new parameter set of the distribution.
     */
    void
    param(const param_type& __param)
    { _M_param = __param; }

    /**
     * @brief Returns the greatest lower bound value of the distribution.
     */
    result_type
    min() const
    { return std::numeric_limits<result_type>::min(); }

    /**
     * @brief Returns the least upper bound value of the distribution.
     */
    result_type
    max() const
    { return std::numeric_limits<result_type>::max(); }

    /**
     * @brief Generating functions.
     */
    template<typename _UniformRandomNumberGenerator>
      result_type
      operator()(_UniformRandomNumberGenerator& __urng)
      { return this->operator()(__urng, _M_param); }

    template<typename _UniformRandomNumberGenerator>
      result_type
      operator()(_UniformRandomNumberGenerator& __urng,
		 const param_type& __p)
      {
	__detail::_Adaptor<_UniformRandomNumberGenerator, double>
	  __aurng(__urng);
	if ((__aurng() - __aurng.min())
	     < __p.p() * (__aurng.max() - __aurng.min()))
	  return true;
	return false;
      }

    template<typename _ForwardIterator,
	     typename _UniformRandomNumberGenerator>
      void
      __generate(_ForwardIterator __f, _ForwardIterator __t,
		 _UniformRandomNumberGenerator& __urng)
      { this->__generate(__f, __t, __urng, _M_param); }

    template<typename _ForwardIterator,
	     typename _UniformRandomNumberGenerator>
      void
      __generate(_ForwardIterator __f, _ForwardIterator __t,
		 _UniformRandomNumberGenerator& __urng, const param_type& __p)
      { this->__generate_impl(__f, __t, __urng, __p); }

    template<typename _UniformRandomNumberGenerator>
      void
      __generate(result_type* __f, result_type* __t,
		 _UniformRandomNumberGenerator& __urng,
		 const param_type& __p)
      { this->__generate_impl(__f, __t, __urng, __p); }

    /**
     * @brief Return true if two Bernoulli distributions have
     *        the same parameters.
     */
    friend bool
    operator==(const bernoulli_distribution& __d1,
	       const bernoulli_distribution& __d2)
    { return __d1._M_param == __d2._M_param; }

  private:
    template<typename _ForwardIterator,
	     typename _UniformRandomNumberGenerator>
      void
      __generate_impl(_ForwardIterator __f, _ForwardIterator __t,
		      _UniformRandomNumberGenerator& __urng,
		      const param_type& __p);

    param_type _M_param;
  };

  /**
   * @brief Return true if two Bernoulli distributions have
   *        different parameters.
   */
  inline bool
  operator!=(const std::bernoulli_distribution& __d1,
	     const std::bernoulli_distribution& __d2)
  { return !(__d1 == __d2); }

  /**
   * @brief Inserts a %bernoulli_distribution random number distribution
   * @p __x into the output stream @p __os.
   *
   * @param __os An output stream.
   * @param __x  A %bernoulli_distribution random number distribution.
   *
   * @returns The output stream with the state of @p __x inserted or in
   * an error state.
   */
  template<typename _CharT, typename _Traits>
    std::basic_ostream<_CharT, _Traits>&
    operator<<(std::basic_ostream<_CharT, _Traits>& __os,
	       const std::bernoulli_distribution& __x);

  /**
   * @brief Extracts a %bernoulli_distribution random number distribution
   * @p __x from the input stream @p __is.
   *
   * @param __is An input stream.
   * @param __x  A %bernoulli_distribution random number generator engine.
   *
   * @returns The input stream with @p __x extracted or in an error state.
   */
  template<typename _CharT, typename _Traits>
    inline std::basic_istream<_CharT, _Traits>&
    operator>>(std::basic_istream<_CharT, _Traits>& __is,
	       std::bernoulli_distribution& __x)
    {
      double __p;
      if (__is >> __p)
	__x.param(bernoulli_distribution::param_type(__p));
      return __is;
    }


  /**
   * @brief A discrete binomial random number distribution.
   *
   * The formula for the binomial probability density function is
   * @f$p(i|t,p) = \binom{t}{i} p^i (1 - p)^{t - i}@f$ where @f$t@f$
   * and @f$p@f$ are the parameters of the distribution.
   */
  template<typename _IntType = int>
    class binomial_distribution
    {
      static_assert(std::is_integral<_IntType>::value,
		    "result_type must be an integral type");

    public:
      /** The type of the range of the distribution. */
      typedef _IntType result_type;

      /** Parameter type. */
      struct param_type
      {
	typedef binomial_distribution<_IntType> distribution_type;
	friend class binomial_distribution<_IntType>;

	param_type() : param_type(1) { }

	explicit
	param_type(_IntType __t, double __p = 0.5)
	: _M_t(__t), _M_p(__p)
	{
	  __glibcxx_assert((_M_t >= _IntType(0))
				&& (_M_p >= 0.0)
				&& (_M_p <= 1.0));
	  _M_initialize();
	}

	_IntType
	t() const
	{ return _M_t; }

	double
	p() const
	{ return _M_p; }

	friend bool
	operator==(const param_type& __p1, const param_type& __p2)
	{ return __p1._M_t == __p2._M_t && __p1._M_p == __p2._M_p; }

	friend bool
	operator!=(const param_type& __p1, const param_type& __p2)
	{ return !(__p1 == __p2); }

      private:
	void
	_M_initialize();

	_IntType _M_t;
	double _M_p;

	double _M_q;
#if _GLIBCXX_USE_C99_MATH_TR1
	double _M_d1, _M_d2, _M_s1, _M_s2, _M_c,
	       _M_a1, _M_a123, _M_s, _M_lf, _M_lp1p;
#endif
	bool   _M_easy;
      };

      // constructors and member functions

      binomial_distribution() : binomial_distribution(1) { }

      explicit
      binomial_distribution(_IntType __t, double __p = 0.5)
      : _M_param(__t, __p), _M_nd()
      { }

      explicit
      binomial_distribution(const param_type& __p)
      : _M_param(__p), _M_nd()
      { }

      /**
       * @brief Resets the distribution state.
       */
      void
      reset()
      { _M_nd.reset(); }

      /**
       * @brief Returns the distribution @p t parameter.
       */
      _IntType
      t() const
      { return _M_param.t(); }

      /**
       * @brief Returns the distribution @p p parameter.
       */
      double
      p() const
      { return _M_param.p(); }

      /**
       * @brief Returns the parameter set of the distribution.
       */
      param_type
      param() const
      { return _M_param; }

      /**
       * @brief Sets the parameter set of the distribution.
       * @param __param The new parameter set of the distribution.
       */
      void
      param(const param_type& __param)
      { _M_param = __param; }

      /**
       * @brief Returns the greatest lower bound value of the distribution.
       */
      result_type
      min() const
      { return 0; }

      /**
       * @brief Returns the least upper bound value of the distribution.
       */
      result_type
      max() const
      { return _M_param.t(); }

      /**
       * @brief Generating functions.
       */
      template<typename _UniformRandomNumberGenerator>
	result_type
	operator()(_UniformRandomNumberGenerator& __urng)
	{ return this->operator()(__urng, _M_param); }

      template<typename _UniformRandomNumberGenerator>
	result_type
	operator()(_UniformRandomNumberGenerator& __urng,
		   const param_type& __p);

      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate(_ForwardIterator __f, _ForwardIterator __t,
		   _UniformRandomNumberGenerator& __urng)
	{ this->__generate(__f, __t, __urng, _M_param); }

      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate(_ForwardIterator __f, _ForwardIterator __t,
		   _UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
	{ this->__generate_impl(__f, __t, __urng, __p); }

      template<typename _UniformRandomNumberGenerator>
	void
	__generate(result_type* __f, result_type* __t,
		   _UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
	{ this->__generate_impl(__f, __t, __urng, __p); }

      /**
       * @brief Return true if two binomial distributions have
       *        the same parameters and the sequences that would
       *        be generated are equal.
       */
	friend bool
        operator==(const binomial_distribution& __d1,
		   const binomial_distribution& __d2)
#ifdef _GLIBCXX_USE_C99_MATH_TR1
	{ return __d1._M_param == __d2._M_param && __d1._M_nd == __d2._M_nd; }
#else
        { return __d1._M_param == __d2._M_param; }
#endif

      /**
       * @brief Inserts a %binomial_distribution random number distribution
       * @p __x into the output stream @p __os.
       *
       * @param __os An output stream.
       * @param __x  A %binomial_distribution random number distribution.
       *
       * @returns The output stream with the state of @p __x inserted or in
       * an error state.
       */
      template<typename _IntType1,
	       typename _CharT, typename _Traits>
	friend std::basic_ostream<_CharT, _Traits>&
	operator<<(std::basic_ostream<_CharT, _Traits>& __os,
		   const std::binomial_distribution<_IntType1>& __x);

      /**
       * @brief Extracts a %binomial_distribution random number distribution
       * @p __x from the input stream @p __is.
       *
       * @param __is An input stream.
       * @param __x  A %binomial_distribution random number generator engine.
       *
       * @returns The input stream with @p __x extracted or in an error
       *          state.
       */
      template<typename _IntType1,
	       typename _CharT, typename _Traits>
	friend std::basic_istream<_CharT, _Traits>&
	operator>>(std::basic_istream<_CharT, _Traits>& __is,
		   std::binomial_distribution<_IntType1>& __x);

    private:
      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate_impl(_ForwardIterator __f, _ForwardIterator __t,
			_UniformRandomNumberGenerator& __urng,
			const param_type& __p);

      template<typename _UniformRandomNumberGenerator>
	result_type
	_M_waiting(_UniformRandomNumberGenerator& __urng,
		   _IntType __t, double __q);

      param_type _M_param;

      // NB: Unused when _GLIBCXX_USE_C99_MATH_TR1 is undefined.
      std::normal_distribution<double> _M_nd;
    };

  /**
   * @brief Return true if two binomial distributions are different.
   */
  template<typename _IntType>
    inline bool
    operator!=(const std::binomial_distribution<_IntType>& __d1,
	       const std::binomial_distribution<_IntType>& __d2)
    { return !(__d1 == __d2); }


  /**
   * @brief A discrete geometric random number distribution.
   *
   * The formula for the geometric probability density function is
   * @f$p(i|p) = p(1 - p)^{i}@f$ where @f$p@f$ is the parameter of the
   * distribution.
   */
  template<typename _IntType = int>
    class geometric_distribution
    {
      static_assert(std::is_integral<_IntType>::value,
		    "result_type must be an integral type");

    public:
      /** The type of the range of the distribution. */
      typedef _IntType  result_type;

      /** Parameter type. */
      struct param_type
      {
	typedef geometric_distribution<_IntType> distribution_type;
	friend class geometric_distribution<_IntType>;

	param_type() : param_type(0.5) { }

	explicit
	param_type(double __p)
	: _M_p(__p)
	{
	  __glibcxx_assert((_M_p > 0.0) && (_M_p < 1.0));
	  _M_initialize();
	}

	double
	p() const
	{ return _M_p; }

	friend bool
	operator==(const param_type& __p1, const param_type& __p2)
	{ return __p1._M_p == __p2._M_p; }

	friend bool
	operator!=(const param_type& __p1, const param_type& __p2)
	{ return !(__p1 == __p2); }

      private:
	void
	_M_initialize()
	{ _M_log_1_p = std::log(1.0 - _M_p); }

	double _M_p;

	double _M_log_1_p;
      };

      // constructors and member functions

      geometric_distribution() : geometric_distribution(0.5) { }

      explicit
      geometric_distribution(double __p)
      : _M_param(__p)
      { }

      explicit
      geometric_distribution(const param_type& __p)
      : _M_param(__p)
      { }

      /**
       * @brief Resets the distribution state.
       *
       * Does nothing for the geometric distribution.
       */
      void
      reset() { }

      /**
       * @brief Returns the distribution parameter @p p.
       */
      double
      p() const
      { return _M_param.p(); }

      /**
       * @brief Returns the parameter set of the distribution.
       */
      param_type
      param() const
      { return _M_param; }

      /**
       * @brief Sets the parameter set of the distribution.
       * @param __param The new parameter set of the distribution.
       */
      void
      param(const param_type& __param)
      { _M_param = __param; }

      /**
       * @brief Returns the greatest lower bound value of the distribution.
       */
      result_type
      min() const
      { return 0; }

      /**
       * @brief Returns the least upper bound value of the distribution.
       */
      result_type
      max() const
      { return std::numeric_limits<result_type>::max(); }

      /**
       * @brief Generating functions.
       */
      template<typename _UniformRandomNumberGenerator>
	result_type
	operator()(_UniformRandomNumberGenerator& __urng)
	{ return this->operator()(__urng, _M_param); }

      template<typename _UniformRandomNumberGenerator>
	result_type
	operator()(_UniformRandomNumberGenerator& __urng,
		   const param_type& __p);

      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate(_ForwardIterator __f, _ForwardIterator __t,
		   _UniformRandomNumberGenerator& __urng)
	{ this->__generate(__f, __t, __urng, _M_param); }

      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate(_ForwardIterator __f, _ForwardIterator __t,
		   _UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
	{ this->__generate_impl(__f, __t, __urng, __p); }

      template<typename _UniformRandomNumberGenerator>
	void
	__generate(result_type* __f, result_type* __t,
		   _UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
	{ this->__generate_impl(__f, __t, __urng, __p); }

      /**
       * @brief Return true if two geometric distributions have
       *        the same parameters.
       */
      friend bool
      operator==(const geometric_distribution& __d1,
		 const geometric_distribution& __d2)
      { return __d1._M_param == __d2._M_param; }

    private:
      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate_impl(_ForwardIterator __f, _ForwardIterator __t,
			_UniformRandomNumberGenerator& __urng,
			const param_type& __p);

      param_type _M_param;
    };

  /**
   * @brief Return true if two geometric distributions have
   *        different parameters.
   */
  template<typename _IntType>
    inline bool
    operator!=(const std::geometric_distribution<_IntType>& __d1,
	       const std::geometric_distribution<_IntType>& __d2)
    { return !(__d1 == __d2); }

  /**
   * @brief Inserts a %geometric_distribution random number distribution
   * @p __x into the output stream @p __os.
   *
   * @param __os An output stream.
   * @param __x  A %geometric_distribution random number distribution.
   *
   * @returns The output stream with the state of @p __x inserted or in
   * an error state.
   */
  template<typename _IntType,
	   typename _CharT, typename _Traits>
    std::basic_ostream<_CharT, _Traits>&
    operator<<(std::basic_ostream<_CharT, _Traits>& __os,
	       const std::geometric_distribution<_IntType>& __x);

  /**
   * @brief Extracts a %geometric_distribution random number distribution
   * @p __x from the input stream @p __is.
   *
   * @param __is An input stream.
   * @param __x  A %geometric_distribution random number generator engine.
   *
   * @returns The input stream with @p __x extracted or in an error state.
   */
  template<typename _IntType,
	   typename _CharT, typename _Traits>
    std::basic_istream<_CharT, _Traits>&
    operator>>(std::basic_istream<_CharT, _Traits>& __is,
	       std::geometric_distribution<_IntType>& __x);


  /**
   * @brief A negative_binomial_distribution random number distribution.
   *
   * The formula for the negative binomial probability mass function is
   * @f$p(i) = \binom{n}{i} p^i (1 - p)^{t - i}@f$ where @f$t@f$
   * and @f$p@f$ are the parameters of the distribution.
   */
  template<typename _IntType = int>
    class negative_binomial_distribution
    {
      static_assert(std::is_integral<_IntType>::value,
		    "result_type must be an integral type");

    public:
      /** The type of the range of the distribution. */
      typedef _IntType result_type;

      /** Parameter type. */
      struct param_type
      {
	typedef negative_binomial_distribution<_IntType> distribution_type;

	param_type() : param_type(1) { }

	explicit
	param_type(_IntType __k, double __p = 0.5)
	: _M_k(__k), _M_p(__p)
	{
	  __glibcxx_assert((_M_k > 0) && (_M_p > 0.0) && (_M_p <= 1.0));
	}

	_IntType
	k() const
	{ return _M_k; }

	double
	p() const
	{ return _M_p; }

	friend bool
	operator==(const param_type& __p1, const param_type& __p2)
	{ return __p1._M_k == __p2._M_k && __p1._M_p == __p2._M_p; }

	friend bool
	operator!=(const param_type& __p1, const param_type& __p2)
	{ return !(__p1 == __p2); }

      private:
	_IntType _M_k;
	double _M_p;
      };

      negative_binomial_distribution() : negative_binomial_distribution(1) { }

      explicit
      negative_binomial_distribution(_IntType __k, double __p = 0.5)
      : _M_param(__k, __p), _M_gd(__k, (1.0 - __p) / __p)
      { }

      explicit
      negative_binomial_distribution(const param_type& __p)
      : _M_param(__p), _M_gd(__p.k(), (1.0 - __p.p()) / __p.p())
      { }

      /**
       * @brief Resets the distribution state.
       */
      void
      reset()
      { _M_gd.reset(); }

      /**
       * @brief Return the @f$k@f$ parameter of the distribution.
       */
      _IntType
      k() const
      { return _M_param.k(); }

      /**
       * @brief Return the @f$p@f$ parameter of the distribution.
       */
      double
      p() const
      { return _M_param.p(); }

      /**
       * @brief Returns the parameter set of the distribution.
       */
      param_type
      param() const
      { return _M_param; }

      /**
       * @brief Sets the parameter set of the distribution.
       * @param __param The new parameter set of the distribution.
       */
      void
      param(const param_type& __param)
      { _M_param = __param; }

      /**
       * @brief Returns the greatest lower bound value of the distribution.
       */
      result_type
      min() const
      { return result_type(0); }

      /**
       * @brief Returns the least upper bound value of the distribution.
       */
      result_type
      max() const
      { return std::numeric_limits<result_type>::max(); }

      /**
       * @brief Generating functions.
       */
      template<typename _UniformRandomNumberGenerator>
	result_type
        operator()(_UniformRandomNumberGenerator& __urng);

      template<typename _UniformRandomNumberGenerator>
	result_type
	operator()(_UniformRandomNumberGenerator& __urng,
		   const param_type& __p);

      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate(_ForwardIterator __f, _ForwardIterator __t,
		   _UniformRandomNumberGenerator& __urng)
	{ this->__generate_impl(__f, __t, __urng); }

      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate(_ForwardIterator __f, _ForwardIterator __t,
		   _UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
	{ this->__generate_impl(__f, __t, __urng, __p); }

      template<typename _UniformRandomNumberGenerator>
	void
	__generate(result_type* __f, result_type* __t,
		   _UniformRandomNumberGenerator& __urng)
	{ this->__generate_impl(__f, __t, __urng); }

      template<typename _UniformRandomNumberGenerator>
	void
	__generate(result_type* __f, result_type* __t,
		   _UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
	{ this->__generate_impl(__f, __t, __urng, __p); }

      /**
       * @brief Return true if two negative binomial distributions have
       *        the same parameters and the sequences that would be
       *        generated are equal.
       */
      friend bool
      operator==(const negative_binomial_distribution& __d1,
		 const negative_binomial_distribution& __d2)
      { return __d1._M_param == __d2._M_param && __d1._M_gd == __d2._M_gd; }

      /**
       * @brief Inserts a %negative_binomial_distribution random
       *        number distribution @p __x into the output stream @p __os.
       *
       * @param __os An output stream.
       * @param __x  A %negative_binomial_distribution random number
       *             distribution.
       *
       * @returns The output stream with the state of @p __x inserted or in
       *          an error state.
       */
      template<typename _IntType1, typename _CharT, typename _Traits>
	friend std::basic_ostream<_CharT, _Traits>&
	operator<<(std::basic_ostream<_CharT, _Traits>& __os,
		   const std::negative_binomial_distribution<_IntType1>& __x);

      /**
       * @brief Extracts a %negative_binomial_distribution random number
       *        distribution @p __x from the input stream @p __is.
       *
       * @param __is An input stream.
       * @param __x A %negative_binomial_distribution random number
       *            generator engine.
       *
       * @returns The input stream with @p __x extracted or in an error state.
       */
      template<typename _IntType1, typename _CharT, typename _Traits>
	friend std::basic_istream<_CharT, _Traits>&
	operator>>(std::basic_istream<_CharT, _Traits>& __is,
		   std::negative_binomial_distribution<_IntType1>& __x);

    private:
      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate_impl(_ForwardIterator __f, _ForwardIterator __t,
			_UniformRandomNumberGenerator& __urng);
      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate_impl(_ForwardIterator __f, _ForwardIterator __t,
			_UniformRandomNumberGenerator& __urng,
			const param_type& __p);

      param_type _M_param;

      std::gamma_distribution<double> _M_gd;
    };

  /**
   * @brief Return true if two negative binomial distributions are different.
   */
  template<typename _IntType>
    inline bool
    operator!=(const std::negative_binomial_distribution<_IntType>& __d1,
	       const std::negative_binomial_distribution<_IntType>& __d2)
    { return !(__d1 == __d2); }


  /* @} */ // group random_distributions_bernoulli

  /**
   * @addtogroup random_distributions_poisson Poisson Distributions
   * @ingroup random_distributions
   * @{
   */

  /**
   * @brief A discrete Poisson random number distribution.
   *
   * The formula for the Poisson probability density function is
   * @f$p(i|\mu) = \frac{\mu^i}{i!} e^{-\mu}@f$ where @f$\mu@f$ is the
   * parameter of the distribution.
   */
  template<typename _IntType = int>
    class poisson_distribution
    {
      static_assert(std::is_integral<_IntType>::value,
		    "result_type must be an integral type");

    public:
      /** The type of the range of the distribution. */
      typedef _IntType  result_type;

      /** Parameter type. */
      struct param_type
      {
	typedef poisson_distribution<_IntType> distribution_type;
	friend class poisson_distribution<_IntType>;

	param_type() : param_type(1.0) { }

	explicit
	param_type(double __mean)
	: _M_mean(__mean)
	{
	  __glibcxx_assert(_M_mean > 0.0);
	  _M_initialize();
	}

	double
	mean() const
	{ return _M_mean; }

	friend bool
	operator==(const param_type& __p1, const param_type& __p2)
	{ return __p1._M_mean == __p2._M_mean; }

	friend bool
	operator!=(const param_type& __p1, const param_type& __p2)
	{ return !(__p1 == __p2); }

      private:
	// Hosts either log(mean) or the threshold of the simple method.
	void
	_M_initialize();

	double _M_mean;

	double _M_lm_thr;
#if _GLIBCXX_USE_C99_MATH_TR1
	double _M_lfm, _M_sm, _M_d, _M_scx, _M_1cx, _M_c2b, _M_cb;
#endif
      };

      // constructors and member functions

      poisson_distribution() : poisson_distribution(1.0) { }

      explicit
      poisson_distribution(double __mean)
      : _M_param(__mean), _M_nd()
      { }

      explicit
      poisson_distribution(const param_type& __p)
      : _M_param(__p), _M_nd()
      { }

      /**
       * @brief Resets the distribution state.
       */
      void
      reset()
      { _M_nd.reset(); }

      /**
       * @brief Returns the distribution parameter @p mean.
       */
      double
      mean() const
      { return _M_param.mean(); }

      /**
       * @brief Returns the parameter set of the distribution.
       */
      param_type
      param() const
      { return _M_param; }

      /**
       * @brief Sets the parameter set of the distribution.
       * @param __param The new parameter set of the distribution.
       */
      void
      param(const param_type& __param)
      { _M_param = __param; }

      /**
       * @brief Returns the greatest lower bound value of the distribution.
       */
      result_type
      min() const
      { return 0; }

      /**
       * @brief Returns the least upper bound value of the distribution.
       */
      result_type
      max() const
      { return std::numeric_limits<result_type>::max(); }

      /**
       * @brief Generating functions.
       */
      template<typename _UniformRandomNumberGenerator>
	result_type
	operator()(_UniformRandomNumberGenerator& __urng)
	{ return this->operator()(__urng, _M_param); }

      template<typename _UniformRandomNumberGenerator>
	result_type
	operator()(_UniformRandomNumberGenerator& __urng,
		   const param_type& __p);

      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate(_ForwardIterator __f, _ForwardIterator __t,
		   _UniformRandomNumberGenerator& __urng)
	{ this->__generate(__f, __t, __urng, _M_param); }

      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate(_ForwardIterator __f, _ForwardIterator __t,
		   _UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
	{ this->__generate_impl(__f, __t, __urng, __p); }

      template<typename _UniformRandomNumberGenerator>
	void
	__generate(result_type* __f, result_type* __t,
		   _UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
	{ this->__generate_impl(__f, __t, __urng, __p); }

       /**
	* @brief Return true if two Poisson distributions have the same
	*        parameters and the sequences that would be generated
	*        are equal.
	*/
      friend bool
      operator==(const poisson_distribution& __d1,
		 const poisson_distribution& __d2)
#ifdef _GLIBCXX_USE_C99_MATH_TR1
      { return __d1._M_param == __d2._M_param && __d1._M_nd == __d2._M_nd; }
#else
      { return __d1._M_param == __d2._M_param; }
#endif

      /**
       * @brief Inserts a %poisson_distribution random number distribution
       * @p __x into the output stream @p __os.
       *
       * @param __os An output stream.
       * @param __x  A %poisson_distribution random number distribution.
       *
       * @returns The output stream with the state of @p __x inserted or in
       * an error state.
       */
      template<typename _IntType1, typename _CharT, typename _Traits>
	friend std::basic_ostream<_CharT, _Traits>&
	operator<<(std::basic_ostream<_CharT, _Traits>& __os,
		   const std::poisson_distribution<_IntType1>& __x);

      /**
       * @brief Extracts a %poisson_distribution random number distribution
       * @p __x from the input stream @p __is.
       *
       * @param __is An input stream.
       * @param __x  A %poisson_distribution random number generator engine.
       *
       * @returns The input stream with @p __x extracted or in an error
       *          state.
       */
      template<typename _IntType1, typename _CharT, typename _Traits>
	friend std::basic_istream<_CharT, _Traits>&
	operator>>(std::basic_istream<_CharT, _Traits>& __is,
		   std::poisson_distribution<_IntType1>& __x);

    private:
      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate_impl(_ForwardIterator __f, _ForwardIterator __t,
			_UniformRandomNumberGenerator& __urng,
			const param_type& __p);

      param_type _M_param;

      // NB: Unused when _GLIBCXX_USE_C99_MATH_TR1 is undefined.
      std::normal_distribution<double> _M_nd;
    };

  /**
   * @brief Return true if two Poisson distributions are different.
   */
  template<typename _IntType>
    inline bool
    operator!=(const std::poisson_distribution<_IntType>& __d1,
	       const std::poisson_distribution<_IntType>& __d2)
    { return !(__d1 == __d2); }


  /**
   * @brief An exponential continuous distribution for random numbers.
   *
   * The formula for the exponential probability density function is
   * @f$p(x|\lambda) = \lambda e^{-\lambda x}@f$.
   *
   * <table border=1 cellpadding=10 cellspacing=0>
   * <caption align=top>Distribution Statistics</caption>
   * <tr><td>Mean</td><td>@f$\frac{1}{\lambda}@f$</td></tr>
   * <tr><td>Median</td><td>@f$\frac{\ln 2}{\lambda}@f$</td></tr>
   * <tr><td>Mode</td><td>@f$zero@f$</td></tr>
   * <tr><td>Range</td><td>@f$[0, \infty]@f$</td></tr>
   * <tr><td>Standard Deviation</td><td>@f$\frac{1}{\lambda}@f$</td></tr>
   * </table>
   */
  template<typename _RealType = double>
    class exponential_distribution
    {
      static_assert(std::is_floating_point<_RealType>::value,
		    "result_type must be a floating point type");

    public:
      /** The type of the range of the distribution. */
      typedef _RealType result_type;

      /** Parameter type. */
      struct param_type
      {
	typedef exponential_distribution<_RealType> distribution_type;

	param_type() : param_type(1.0) { }

	explicit
	param_type(_RealType __lambda)
	: _M_lambda(__lambda)
	{
	  __glibcxx_assert(_M_lambda > _RealType(0));
	}

	_RealType
	lambda() const
	{ return _M_lambda; }

	friend bool
	operator==(const param_type& __p1, const param_type& __p2)
	{ return __p1._M_lambda == __p2._M_lambda; }

	friend bool
	operator!=(const param_type& __p1, const param_type& __p2)
	{ return !(__p1 == __p2); }

      private:
	_RealType _M_lambda;
      };

    public:
      /**
       * @brief Constructs an exponential distribution with inverse scale
       *        parameter 1.0
       */
      exponential_distribution() : exponential_distribution(1.0) { }

      /**
       * @brief Constructs an exponential distribution with inverse scale
       *        parameter @f$\lambda@f$.
       */
      explicit
      exponential_distribution(_RealType __lambda)
      : _M_param(__lambda)
      { }

      explicit
      exponential_distribution(const param_type& __p)
      : _M_param(__p)
      { }

      /**
       * @brief Resets the distribution state.
       *
       * Has no effect on exponential distributions.
       */
      void
      reset() { }

      /**
       * @brief Returns the inverse scale parameter of the distribution.
       */
      _RealType
      lambda() const
      { return _M_param.lambda(); }

      /**
       * @brief Returns the parameter set of the distribution.
       */
      param_type
      param() const
      { return _M_param; }

      /**
       * @brief Sets the parameter set of the distribution.
       * @param __param The new parameter set of the distribution.
       */
      void
      param(const param_type& __param)
      { _M_param = __param; }

      /**
       * @brief Returns the greatest lower bound value of the distribution.
       */
      result_type
      min() const
      { return result_type(0); }

      /**
       * @brief Returns the least upper bound value of the distribution.
       */
      result_type
      max() const
      { return std::numeric_limits<result_type>::max(); }

      /**
       * @brief Generating functions.
       */
      template<typename _UniformRandomNumberGenerator>
	result_type
	operator()(_UniformRandomNumberGenerator& __urng)
        { return this->operator()(__urng, _M_param); }

      template<typename _UniformRandomNumberGenerator>
	result_type
	operator()(_UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
	{
	  __detail::_Adaptor<_UniformRandomNumberGenerator, result_type>
	    __aurng(__urng);
	  return -std::log(result_type(1) - __aurng()) / __p.lambda();
	}

      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate(_ForwardIterator __f, _ForwardIterator __t,
		   _UniformRandomNumberGenerator& __urng)
	{ this->__generate(__f, __t, __urng, _M_param); }

      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate(_ForwardIterator __f, _ForwardIterator __t,
		   _UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
	{ this->__generate_impl(__f, __t, __urng, __p); }

      template<typename _UniformRandomNumberGenerator>
	void
	__generate(result_type* __f, result_type* __t,
		   _UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
	{ this->__generate_impl(__f, __t, __urng, __p); }

      /**
       * @brief Return true if two exponential distributions have the same
       *        parameters.
       */
      friend bool
      operator==(const exponential_distribution& __d1,
		 const exponential_distribution& __d2)
      { return __d1._M_param == __d2._M_param; }

    private:
      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate_impl(_ForwardIterator __f, _ForwardIterator __t,
			_UniformRandomNumberGenerator& __urng,
			const param_type& __p);

      param_type _M_param;
    };

  /**
   * @brief Return true if two exponential distributions have different
   *        parameters.
   */
  template<typename _RealType>
    inline bool
    operator!=(const std::exponential_distribution<_RealType>& __d1,
	       const std::exponential_distribution<_RealType>& __d2)
    { return !(__d1 == __d2); }

  /**
   * @brief Inserts a %exponential_distribution random number distribution
   * @p __x into the output stream @p __os.
   *
   * @param __os An output stream.
   * @param __x  A %exponential_distribution random number distribution.
   *
   * @returns The output stream with the state of @p __x inserted or in
   * an error state.
   */
  template<typename _RealType, typename _CharT, typename _Traits>
    std::basic_ostream<_CharT, _Traits>&
    operator<<(std::basic_ostream<_CharT, _Traits>& __os,
	       const std::exponential_distribution<_RealType>& __x);

  /**
   * @brief Extracts a %exponential_distribution random number distribution
   * @p __x from the input stream @p __is.
   *
   * @param __is An input stream.
   * @param __x A %exponential_distribution random number
   *            generator engine.
   *
   * @returns The input stream with @p __x extracted or in an error state.
   */
  template<typename _RealType, typename _CharT, typename _Traits>
    std::basic_istream<_CharT, _Traits>&
    operator>>(std::basic_istream<_CharT, _Traits>& __is,
	       std::exponential_distribution<_RealType>& __x);


  /**
   * @brief A weibull_distribution random number distribution.
   *
   * The formula for the normal probability density function is:
   * @f[
   *     p(x|\alpha,\beta) = \frac{\alpha}{\beta} (\frac{x}{\beta})^{\alpha-1}
   *                         \exp{(-(\frac{x}{\beta})^\alpha)} 
   * @f]
   */
  template<typename _RealType = double>
    class weibull_distribution
    {
      static_assert(std::is_floating_point<_RealType>::value,
		    "result_type must be a floating point type");

    public:
      /** The type of the range of the distribution. */
      typedef _RealType result_type;

      /** Parameter type. */
      struct param_type
      {
	typedef weibull_distribution<_RealType> distribution_type;

	param_type() : param_type(1.0) { }

	explicit
	param_type(_RealType __a, _RealType __b = _RealType(1.0))
	: _M_a(__a), _M_b(__b)
	{ }

	_RealType
	a() const
	{ return _M_a; }

	_RealType
	b() const
	{ return _M_b; }

	friend bool
	operator==(const param_type& __p1, const param_type& __p2)
	{ return __p1._M_a == __p2._M_a && __p1._M_b == __p2._M_b; }

	friend bool
	operator!=(const param_type& __p1, const param_type& __p2)
	{ return !(__p1 == __p2); }

      private:
	_RealType _M_a;
	_RealType _M_b;
      };

      weibull_distribution() : weibull_distribution(1.0) { }

      explicit
      weibull_distribution(_RealType __a, _RealType __b = _RealType(1))
      : _M_param(__a, __b)
      { }

      explicit
      weibull_distribution(const param_type& __p)
      : _M_param(__p)
      { }

      /**
       * @brief Resets the distribution state.
       */
      void
      reset()
      { }

      /**
       * @brief Return the @f$a@f$ parameter of the distribution.
       */
      _RealType
      a() const
      { return _M_param.a(); }

      /**
       * @brief Return the @f$b@f$ parameter of the distribution.
       */
      _RealType
      b() const
      { return _M_param.b(); }

      /**
       * @brief Returns the parameter set of the distribution.
       */
      param_type
      param() const
      { return _M_param; }

      /**
       * @brief Sets the parameter set of the distribution.
       * @param __param The new parameter set of the distribution.
       */
      void
      param(const param_type& __param)
      { _M_param = __param; }

      /**
       * @brief Returns the greatest lower bound value of the distribution.
       */
      result_type
      min() const
      { return result_type(0); }

      /**
       * @brief Returns the least upper bound value of the distribution.
       */
      result_type
      max() const
      { return std::numeric_limits<result_type>::max(); }

      /**
       * @brief Generating functions.
       */
      template<typename _UniformRandomNumberGenerator>
	result_type
	operator()(_UniformRandomNumberGenerator& __urng)
	{ return this->operator()(__urng, _M_param); }

      template<typename _UniformRandomNumberGenerator>
	result_type
	operator()(_UniformRandomNumberGenerator& __urng,
		   const param_type& __p);

      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate(_ForwardIterator __f, _ForwardIterator __t,
		   _UniformRandomNumberGenerator& __urng)
	{ this->__generate(__f, __t, __urng, _M_param); }

      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate(_ForwardIterator __f, _ForwardIterator __t,
		   _UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
	{ this->__generate_impl(__f, __t, __urng, __p); }

      template<typename _UniformRandomNumberGenerator>
	void
	__generate(result_type* __f, result_type* __t,
		   _UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
	{ this->__generate_impl(__f, __t, __urng, __p); }

      /**
       * @brief Return true if two Weibull distributions have the same
       *        parameters.
       */
      friend bool
      operator==(const weibull_distribution& __d1,
		 const weibull_distribution& __d2)
      { return __d1._M_param == __d2._M_param; }

    private:
      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate_impl(_ForwardIterator __f, _ForwardIterator __t,
			_UniformRandomNumberGenerator& __urng,
			const param_type& __p);

      param_type _M_param;
    };

   /**
    * @brief Return true if two Weibull distributions have different
    *        parameters.
    */
  template<typename _RealType>
    inline bool
    operator!=(const std::weibull_distribution<_RealType>& __d1,
	       const std::weibull_distribution<_RealType>& __d2)
    { return !(__d1 == __d2); }

  /**
   * @brief Inserts a %weibull_distribution random number distribution
   * @p __x into the output stream @p __os.
   *
   * @param __os An output stream.
   * @param __x  A %weibull_distribution random number distribution.
   *
   * @returns The output stream with the state of @p __x inserted or in
   * an error state.
   */
  template<typename _RealType, typename _CharT, typename _Traits>
    std::basic_ostream<_CharT, _Traits>&
    operator<<(std::basic_ostream<_CharT, _Traits>& __os,
	       const std::weibull_distribution<_RealType>& __x);

  /**
   * @brief Extracts a %weibull_distribution random number distribution
   * @p __x from the input stream @p __is.
   *
   * @param __is An input stream.
   * @param __x A %weibull_distribution random number
   *            generator engine.
   *
   * @returns The input stream with @p __x extracted or in an error state.
   */
  template<typename _RealType, typename _CharT, typename _Traits>
    std::basic_istream<_CharT, _Traits>&
    operator>>(std::basic_istream<_CharT, _Traits>& __is,
	       std::weibull_distribution<_RealType>& __x);


  /**
   * @brief A extreme_value_distribution random number distribution.
   *
   * The formula for the normal probability mass function is
   * @f[
   *     p(x|a,b) = \frac{1}{b}
   *                \exp( \frac{a-x}{b} - \exp(\frac{a-x}{b})) 
   * @f]
   */
  template<typename _RealType = double>
    class extreme_value_distribution
    {
      static_assert(std::is_floating_point<_RealType>::value,
		    "result_type must be a floating point type");

    public:
      /** The type of the range of the distribution. */
      typedef _RealType result_type;

      /** Parameter type. */
      struct param_type
      {
	typedef extreme_value_distribution<_RealType> distribution_type;

	param_type() : param_type(0.0) { }

	explicit
	param_type(_RealType __a, _RealType __b = _RealType(1.0))
	: _M_a(__a), _M_b(__b)
	{ }

	_RealType
	a() const
	{ return _M_a; }

	_RealType
	b() const
	{ return _M_b; }

	friend bool
	operator==(const param_type& __p1, const param_type& __p2)
	{ return __p1._M_a == __p2._M_a && __p1._M_b == __p2._M_b; }

	friend bool
	operator!=(const param_type& __p1, const param_type& __p2)
	{ return !(__p1 == __p2); }

      private:
	_RealType _M_a;
	_RealType _M_b;
      };

      extreme_value_distribution() : extreme_value_distribution(0.0) { }

      explicit
      extreme_value_distribution(_RealType __a, _RealType __b = _RealType(1))
      : _M_param(__a, __b)
      { }

      explicit
      extreme_value_distribution(const param_type& __p)
      : _M_param(__p)
      { }

      /**
       * @brief Resets the distribution state.
       */
      void
      reset()
      { }

      /**
       * @brief Return the @f$a@f$ parameter of the distribution.
       */
      _RealType
      a() const
      { return _M_param.a(); }

      /**
       * @brief Return the @f$b@f$ parameter of the distribution.
       */
      _RealType
      b() const
      { return _M_param.b(); }

      /**
       * @brief Returns the parameter set of the distribution.
       */
      param_type
      param() const
      { return _M_param; }

      /**
       * @brief Sets the parameter set of the distribution.
       * @param __param The new parameter set of the distribution.
       */
      void
      param(const param_type& __param)
      { _M_param = __param; }

      /**
       * @brief Returns the greatest lower bound value of the distribution.
       */
      result_type
      min() const
      { return std::numeric_limits<result_type>::lowest(); }

      /**
       * @brief Returns the least upper bound value of the distribution.
       */
      result_type
      max() const
      { return std::numeric_limits<result_type>::max(); }

      /**
       * @brief Generating functions.
       */
      template<typename _UniformRandomNumberGenerator>
	result_type
	operator()(_UniformRandomNumberGenerator& __urng)
	{ return this->operator()(__urng, _M_param); }

      template<typename _UniformRandomNumberGenerator>
	result_type
	operator()(_UniformRandomNumberGenerator& __urng,
		   const param_type& __p);

      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate(_ForwardIterator __f, _ForwardIterator __t,
		   _UniformRandomNumberGenerator& __urng)
	{ this->__generate(__f, __t, __urng, _M_param); }

      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate(_ForwardIterator __f, _ForwardIterator __t,
		   _UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
	{ this->__generate_impl(__f, __t, __urng, __p); }

      template<typename _UniformRandomNumberGenerator>
	void
	__generate(result_type* __f, result_type* __t,
		   _UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
	{ this->__generate_impl(__f, __t, __urng, __p); }

      /**
       * @brief Return true if two extreme value distributions have the same
       *        parameters.
       */
      friend bool
      operator==(const extreme_value_distribution& __d1,
		 const extreme_value_distribution& __d2)
      { return __d1._M_param == __d2._M_param; }

    private:
      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate_impl(_ForwardIterator __f, _ForwardIterator __t,
			_UniformRandomNumberGenerator& __urng,
			const param_type& __p);

      param_type _M_param;
    };

  /**
    * @brief Return true if two extreme value distributions have different
    *        parameters.
   */
  template<typename _RealType>
    inline bool
    operator!=(const std::extreme_value_distribution<_RealType>& __d1,
	       const std::extreme_value_distribution<_RealType>& __d2)
    { return !(__d1 == __d2); }

  /**
   * @brief Inserts a %extreme_value_distribution random number distribution
   * @p __x into the output stream @p __os.
   *
   * @param __os An output stream.
   * @param __x  A %extreme_value_distribution random number distribution.
   *
   * @returns The output stream with the state of @p __x inserted or in
   * an error state.
   */
  template<typename _RealType, typename _CharT, typename _Traits>
    std::basic_ostream<_CharT, _Traits>&
    operator<<(std::basic_ostream<_CharT, _Traits>& __os,
	       const std::extreme_value_distribution<_RealType>& __x);

  /**
   * @brief Extracts a %extreme_value_distribution random number
   *        distribution @p __x from the input stream @p __is.
   *
   * @param __is An input stream.
   * @param __x A %extreme_value_distribution random number
   *            generator engine.
   *
   * @returns The input stream with @p __x extracted or in an error state.
   */
  template<typename _RealType, typename _CharT, typename _Traits>
    std::basic_istream<_CharT, _Traits>&
    operator>>(std::basic_istream<_CharT, _Traits>& __is,
	       std::extreme_value_distribution<_RealType>& __x);


  /**
   * @brief A discrete_distribution random number distribution.
   *
   * The formula for the discrete probability mass function is
   *
   */
  template<typename _IntType = int>
    class discrete_distribution
    {
      static_assert(std::is_integral<_IntType>::value,
		    "result_type must be an integral type");

    public:
      /** The type of the range of the distribution. */
      typedef _IntType result_type;

      /** Parameter type. */
      struct param_type
      {
	typedef discrete_distribution<_IntType> distribution_type;
	friend class discrete_distribution<_IntType>;

	param_type()
	: _M_prob(), _M_cp()
	{ }

	template<typename _InputIterator>
	  param_type(_InputIterator __wbegin,
		     _InputIterator __wend)
	  : _M_prob(__wbegin, __wend), _M_cp()
	  { _M_initialize(); }

	param_type(initializer_list<double> __wil)
	: _M_prob(__wil.begin(), __wil.end()), _M_cp()
	{ _M_initialize(); }

	template<typename _Func>
	  param_type(size_t __nw, double __xmin, double __xmax,
		     _Func __fw);

	// See: http://cpp-next.com/archive/2010/10/implicit-move-must-go/
	param_type(const param_type&) = default;
	param_type& operator=(const param_type&) = default;

	std::vector<double>
	probabilities() const
	{ return _M_prob.empty() ? std::vector<double>(1, 1.0) : _M_prob; }

	friend bool
	operator==(const param_type& __p1, const param_type& __p2)
	{ return __p1._M_prob == __p2._M_prob; }

	friend bool
	operator!=(const param_type& __p1, const param_type& __p2)
	{ return !(__p1 == __p2); }

      private:
	void
	_M_initialize();

	std::vector<double> _M_prob;
	std::vector<double> _M_cp;
      };

      discrete_distribution()
      : _M_param()
      { }

      template<typename _InputIterator>
	discrete_distribution(_InputIterator __wbegin,
			      _InputIterator __wend)
	: _M_param(__wbegin, __wend)
	{ }

      discrete_distribution(initializer_list<double> __wl)
      : _M_param(__wl)
      { }

      template<typename _Func>
	discrete_distribution(size_t __nw, double __xmin, double __xmax,
			      _Func __fw)
	: _M_param(__nw, __xmin, __xmax, __fw)
	{ }

      explicit
      discrete_distribution(const param_type& __p)
      : _M_param(__p)
      { }

      /**
       * @brief Resets the distribution state.
       */
      void
      reset()
      { }

      /**
       * @brief Returns the probabilities of the distribution.
       */
      std::vector<double>
      probabilities() const
      {
	return _M_param._M_prob.empty()
	  ? std::vector<double>(1, 1.0) : _M_param._M_prob;
      }

      /**
       * @brief Returns the parameter set of the distribution.
       */
      param_type
      param() const
      { return _M_param; }

      /**
       * @brief Sets the parameter set of the distribution.
       * @param __param The new parameter set of the distribution.
       */
      void
      param(const param_type& __param)
      { _M_param = __param; }

      /**
       * @brief Returns the greatest lower bound value of the distribution.
       */
      result_type
      min() const
      { return result_type(0); }

      /**
       * @brief Returns the least upper bound value of the distribution.
       */
      result_type
      max() const
      {
	return _M_param._M_prob.empty()
	  ? result_type(0) : result_type(_M_param._M_prob.size() - 1);
      }

      /**
       * @brief Generating functions.
       */
      template<typename _UniformRandomNumberGenerator>
	result_type
	operator()(_UniformRandomNumberGenerator& __urng)
	{ return this->operator()(__urng, _M_param); }

      template<typename _UniformRandomNumberGenerator>
	result_type
	operator()(_UniformRandomNumberGenerator& __urng,
		   const param_type& __p);

      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate(_ForwardIterator __f, _ForwardIterator __t,
		   _UniformRandomNumberGenerator& __urng)
	{ this->__generate(__f, __t, __urng, _M_param); }

      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate(_ForwardIterator __f, _ForwardIterator __t,
		   _UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
	{ this->__generate_impl(__f, __t, __urng, __p); }

      template<typename _UniformRandomNumberGenerator>
	void
	__generate(result_type* __f, result_type* __t,
		   _UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
	{ this->__generate_impl(__f, __t, __urng, __p); }

      /**
       * @brief Return true if two discrete distributions have the same
       *        parameters.
       */
      friend bool
      operator==(const discrete_distribution& __d1,
		 const discrete_distribution& __d2)
      { return __d1._M_param == __d2._M_param; }

      /**
       * @brief Inserts a %discrete_distribution random number distribution
       * @p __x into the output stream @p __os.
       *
       * @param __os An output stream.
       * @param __x  A %discrete_distribution random number distribution.
       *
       * @returns The output stream with the state of @p __x inserted or in
       * an error state.
       */
      template<typename _IntType1, typename _CharT, typename _Traits>
	friend std::basic_ostream<_CharT, _Traits>&
	operator<<(std::basic_ostream<_CharT, _Traits>& __os,
		   const std::discrete_distribution<_IntType1>& __x);

      /**
       * @brief Extracts a %discrete_distribution random number distribution
       * @p __x from the input stream @p __is.
       *
       * @param __is An input stream.
       * @param __x A %discrete_distribution random number
       *            generator engine.
       *
       * @returns The input stream with @p __x extracted or in an error
       *          state.
       */
      template<typename _IntType1, typename _CharT, typename _Traits>
	friend std::basic_istream<_CharT, _Traits>&
	operator>>(std::basic_istream<_CharT, _Traits>& __is,
		   std::discrete_distribution<_IntType1>& __x);

    private:
      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate_impl(_ForwardIterator __f, _ForwardIterator __t,
			_UniformRandomNumberGenerator& __urng,
			const param_type& __p);

      param_type _M_param;
    };

  /**
    * @brief Return true if two discrete distributions have different
    *        parameters.
    */
  template<typename _IntType>
    inline bool
    operator!=(const std::discrete_distribution<_IntType>& __d1,
	       const std::discrete_distribution<_IntType>& __d2)
    { return !(__d1 == __d2); }


  /**
   * @brief A piecewise_constant_distribution random number distribution.
   *
   * The formula for the piecewise constant probability mass function is
   *
   */
  template<typename _RealType = double>
    class piecewise_constant_distribution
    {
      static_assert(std::is_floating_point<_RealType>::value,
		    "result_type must be a floating point type");

    public:
      /** The type of the range of the distribution. */
      typedef _RealType result_type;

      /** Parameter type. */
      struct param_type
      {
	typedef piecewise_constant_distribution<_RealType> distribution_type;
	friend class piecewise_constant_distribution<_RealType>;

	param_type()
	: _M_int(), _M_den(), _M_cp()
	{ }

	template<typename _InputIteratorB, typename _InputIteratorW>
	  param_type(_InputIteratorB __bfirst,
		     _InputIteratorB __bend,
		     _InputIteratorW __wbegin);

	template<typename _Func>
	  param_type(initializer_list<_RealType> __bi, _Func __fw);

	template<typename _Func>
	  param_type(size_t __nw, _RealType __xmin, _RealType __xmax,
		     _Func __fw);

	// See: http://cpp-next.com/archive/2010/10/implicit-move-must-go/
	param_type(const param_type&) = default;
	param_type& operator=(const param_type&) = default;

	std::vector<_RealType>
	intervals() const
	{
	  if (_M_int.empty())
	    {
	      std::vector<_RealType> __tmp(2);
	      __tmp[1] = _RealType(1);
	      return __tmp;
	    }
	  else
	    return _M_int;
	}

	std::vector<double>
	densities() const
	{ return _M_den.empty() ? std::vector<double>(1, 1.0) : _M_den; }

	friend bool
	operator==(const param_type& __p1, const param_type& __p2)
	{ return __p1._M_int == __p2._M_int && __p1._M_den == __p2._M_den; }

	friend bool
	operator!=(const param_type& __p1, const param_type& __p2)
	{ return !(__p1 == __p2); }

      private:
	void
	_M_initialize();

	std::vector<_RealType> _M_int;
	std::vector<double> _M_den;
	std::vector<double> _M_cp;
      };

      piecewise_constant_distribution()
      : _M_param()
      { }

      template<typename _InputIteratorB, typename _InputIteratorW>
	piecewise_constant_distribution(_InputIteratorB __bfirst,
					_InputIteratorB __bend,
					_InputIteratorW __wbegin)
	: _M_param(__bfirst, __bend, __wbegin)
	{ }

      template<typename _Func>
	piecewise_constant_distribution(initializer_list<_RealType> __bl,
					_Func __fw)
	: _M_param(__bl, __fw)
	{ }

      template<typename _Func>
	piecewise_constant_distribution(size_t __nw,
					_RealType __xmin, _RealType __xmax,
					_Func __fw)
	: _M_param(__nw, __xmin, __xmax, __fw)
	{ }

      explicit
      piecewise_constant_distribution(const param_type& __p)
      : _M_param(__p)
      { }

      /**
       * @brief Resets the distribution state.
       */
      void
      reset()
      { }

      /**
       * @brief Returns a vector of the intervals.
       */
      std::vector<_RealType>
      intervals() const
      {
	if (_M_param._M_int.empty())
	  {
	    std::vector<_RealType> __tmp(2);
	    __tmp[1] = _RealType(1);
	    return __tmp;
	  }
	else
	  return _M_param._M_int;
      }

      /**
       * @brief Returns a vector of the probability densities.
       */
      std::vector<double>
      densities() const
      {
	return _M_param._M_den.empty()
	  ? std::vector<double>(1, 1.0) : _M_param._M_den;
      }

      /**
       * @brief Returns the parameter set of the distribution.
       */
      param_type
      param() const
      { return _M_param; }

      /**
       * @brief Sets the parameter set of the distribution.
       * @param __param The new parameter set of the distribution.
       */
      void
      param(const param_type& __param)
      { _M_param = __param; }

      /**
       * @brief Returns the greatest lower bound value of the distribution.
       */
      result_type
      min() const
      {
	return _M_param._M_int.empty()
	  ? result_type(0) : _M_param._M_int.front();
      }

      /**
       * @brief Returns the least upper bound value of the distribution.
       */
      result_type
      max() const
      {
	return _M_param._M_int.empty()
	  ? result_type(1) : _M_param._M_int.back();
      }

      /**
       * @brief Generating functions.
       */
      template<typename _UniformRandomNumberGenerator>
	result_type
	operator()(_UniformRandomNumberGenerator& __urng)
	{ return this->operator()(__urng, _M_param); }

      template<typename _UniformRandomNumberGenerator>
	result_type
	operator()(_UniformRandomNumberGenerator& __urng,
		   const param_type& __p);

      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate(_ForwardIterator __f, _ForwardIterator __t,
		   _UniformRandomNumberGenerator& __urng)
	{ this->__generate(__f, __t, __urng, _M_param); }

      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate(_ForwardIterator __f, _ForwardIterator __t,
		   _UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
	{ this->__generate_impl(__f, __t, __urng, __p); }

      template<typename _UniformRandomNumberGenerator>
	void
	__generate(result_type* __f, result_type* __t,
		   _UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
	{ this->__generate_impl(__f, __t, __urng, __p); }

      /**
       * @brief Return true if two piecewise constant distributions have the
       *        same parameters.
       */
      friend bool
      operator==(const piecewise_constant_distribution& __d1,
		 const piecewise_constant_distribution& __d2)
      { return __d1._M_param == __d2._M_param; }

      /**
       * @brief Inserts a %piecewise_constant_distribution random
       *        number distribution @p __x into the output stream @p __os.
       *
       * @param __os An output stream.
       * @param __x  A %piecewise_constant_distribution random number
       *             distribution.
       *
       * @returns The output stream with the state of @p __x inserted or in
       * an error state.
       */
      template<typename _RealType1, typename _CharT, typename _Traits>
	friend std::basic_ostream<_CharT, _Traits>&
	operator<<(std::basic_ostream<_CharT, _Traits>& __os,
		   const std::piecewise_constant_distribution<_RealType1>& __x);

      /**
       * @brief Extracts a %piecewise_constant_distribution random
       *        number distribution @p __x from the input stream @p __is.
       *
       * @param __is An input stream.
       * @param __x A %piecewise_constant_distribution random number
       *            generator engine.
       *
       * @returns The input stream with @p __x extracted or in an error
       *          state.
       */
      template<typename _RealType1, typename _CharT, typename _Traits>
	friend std::basic_istream<_CharT, _Traits>&
	operator>>(std::basic_istream<_CharT, _Traits>& __is,
		   std::piecewise_constant_distribution<_RealType1>& __x);

    private:
      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate_impl(_ForwardIterator __f, _ForwardIterator __t,
			_UniformRandomNumberGenerator& __urng,
			const param_type& __p);

      param_type _M_param;
    };

  /**
    * @brief Return true if two piecewise constant distributions have 
    *        different parameters.
   */
  template<typename _RealType>
    inline bool
    operator!=(const std::piecewise_constant_distribution<_RealType>& __d1,
	       const std::piecewise_constant_distribution<_RealType>& __d2)
    { return !(__d1 == __d2); }


  /**
   * @brief A piecewise_linear_distribution random number distribution.
   *
   * The formula for the piecewise linear probability mass function is
   *
   */
  template<typename _RealType = double>
    class piecewise_linear_distribution
    {
      static_assert(std::is_floating_point<_RealType>::value,
		    "result_type must be a floating point type");

    public:
      /** The type of the range of the distribution. */
      typedef _RealType result_type;

      /** Parameter type. */
      struct param_type
      {
	typedef piecewise_linear_distribution<_RealType> distribution_type;
	friend class piecewise_linear_distribution<_RealType>;

	param_type()
	: _M_int(), _M_den(), _M_cp(), _M_m()
	{ }

	template<typename _InputIteratorB, typename _InputIteratorW>
	  param_type(_InputIteratorB __bfirst,
		     _InputIteratorB __bend,
		     _InputIteratorW __wbegin);

	template<typename _Func>
	  param_type(initializer_list<_RealType> __bl, _Func __fw);

	template<typename _Func>
	  param_type(size_t __nw, _RealType __xmin, _RealType __xmax,
		     _Func __fw);

	// See: http://cpp-next.com/archive/2010/10/implicit-move-must-go/
	param_type(const param_type&) = default;
	param_type& operator=(const param_type&) = default;

	std::vector<_RealType>
	intervals() const
	{
	  if (_M_int.empty())
	    {
	      std::vector<_RealType> __tmp(2);
	      __tmp[1] = _RealType(1);
	      return __tmp;
	    }
	  else
	    return _M_int;
	}

	std::vector<double>
	densities() const
	{ return _M_den.empty() ? std::vector<double>(2, 1.0) : _M_den; }

	friend bool
	operator==(const param_type& __p1, const param_type& __p2)
	{ return __p1._M_int == __p2._M_int && __p1._M_den == __p2._M_den; }

	friend bool
	operator!=(const param_type& __p1, const param_type& __p2)
	{ return !(__p1 == __p2); }

      private:
	void
	_M_initialize();

	std::vector<_RealType> _M_int;
	std::vector<double> _M_den;
	std::vector<double> _M_cp;
	std::vector<double> _M_m;
      };

      piecewise_linear_distribution()
      : _M_param()
      { }

      template<typename _InputIteratorB, typename _InputIteratorW>
	piecewise_linear_distribution(_InputIteratorB __bfirst,
				      _InputIteratorB __bend,
				      _InputIteratorW __wbegin)
	: _M_param(__bfirst, __bend, __wbegin)
	{ }

      template<typename _Func>
	piecewise_linear_distribution(initializer_list<_RealType> __bl,
				      _Func __fw)
	: _M_param(__bl, __fw)
	{ }

      template<typename _Func>
	piecewise_linear_distribution(size_t __nw,
				      _RealType __xmin, _RealType __xmax,
				      _Func __fw)
	: _M_param(__nw, __xmin, __xmax, __fw)
	{ }

      explicit
      piecewise_linear_distribution(const param_type& __p)
      : _M_param(__p)
      { }

      /**
       * Resets the distribution state.
       */
      void
      reset()
      { }

      /**
       * @brief Return the intervals of the distribution.
       */
      std::vector<_RealType>
      intervals() const
      {
	if (_M_param._M_int.empty())
	  {
	    std::vector<_RealType> __tmp(2);
	    __tmp[1] = _RealType(1);
	    return __tmp;
	  }
	else
	  return _M_param._M_int;
      }

      /**
       * @brief Return a vector of the probability densities of the
       *        distribution.
       */
      std::vector<double>
      densities() const
      {
	return _M_param._M_den.empty()
	  ? std::vector<double>(2, 1.0) : _M_param._M_den;
      }

      /**
       * @brief Returns the parameter set of the distribution.
       */
      param_type
      param() const
      { return _M_param; }

      /**
       * @brief Sets the parameter set of the distribution.
       * @param __param The new parameter set of the distribution.
       */
      void
      param(const param_type& __param)
      { _M_param = __param; }

      /**
       * @brief Returns the greatest lower bound value of the distribution.
       */
      result_type
      min() const
      {
	return _M_param._M_int.empty()
	  ? result_type(0) : _M_param._M_int.front();
      }

      /**
       * @brief Returns the least upper bound value of the distribution.
       */
      result_type
      max() const
      {
	return _M_param._M_int.empty()
	  ? result_type(1) : _M_param._M_int.back();
      }

      /**
       * @brief Generating functions.
       */
      template<typename _UniformRandomNumberGenerator>
	result_type
	operator()(_UniformRandomNumberGenerator& __urng)
	{ return this->operator()(__urng, _M_param); }

      template<typename _UniformRandomNumberGenerator>
	result_type
	operator()(_UniformRandomNumberGenerator& __urng,
		   const param_type& __p);

      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate(_ForwardIterator __f, _ForwardIterator __t,
		   _UniformRandomNumberGenerator& __urng)
	{ this->__generate(__f, __t, __urng, _M_param); }

      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate(_ForwardIterator __f, _ForwardIterator __t,
		   _UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
	{ this->__generate_impl(__f, __t, __urng, __p); }

      template<typename _UniformRandomNumberGenerator>
	void
	__generate(result_type* __f, result_type* __t,
		   _UniformRandomNumberGenerator& __urng,
		   const param_type& __p)
	{ this->__generate_impl(__f, __t, __urng, __p); }

      /**
       * @brief Return true if two piecewise linear distributions have the
       *        same parameters.
       */
      friend bool
      operator==(const piecewise_linear_distribution& __d1,
		 const piecewise_linear_distribution& __d2)
      { return __d1._M_param == __d2._M_param; }

      /**
       * @brief Inserts a %piecewise_linear_distribution random number
       *        distribution @p __x into the output stream @p __os.
       *
       * @param __os An output stream.
       * @param __x  A %piecewise_linear_distribution random number
       *             distribution.
       *
       * @returns The output stream with the state of @p __x inserted or in
       *          an error state.
       */
      template<typename _RealType1, typename _CharT, typename _Traits>
	friend std::basic_ostream<_CharT, _Traits>&
	operator<<(std::basic_ostream<_CharT, _Traits>& __os,
		   const std::piecewise_linear_distribution<_RealType1>& __x);

      /**
       * @brief Extracts a %piecewise_linear_distribution random number
       *        distribution @p __x from the input stream @p __is.
       *
       * @param __is An input stream.
       * @param __x  A %piecewise_linear_distribution random number
       *             generator engine.
       *
       * @returns The input stream with @p __x extracted or in an error
       *          state.
       */
      template<typename _RealType1, typename _CharT, typename _Traits>
	friend std::basic_istream<_CharT, _Traits>&
	operator>>(std::basic_istream<_CharT, _Traits>& __is,
		   std::piecewise_linear_distribution<_RealType1>& __x);

    private:
      template<typename _ForwardIterator,
	       typename _UniformRandomNumberGenerator>
	void
	__generate_impl(_ForwardIterator __f, _ForwardIterator __t,
			_UniformRandomNumberGenerator& __urng,
			const param_type& __p);

      param_type _M_param;
    };

  /**
    * @brief Return true if two piecewise linear distributions have
    *        different parameters.
   */
  template<typename _RealType>
    inline bool
    operator!=(const std::piecewise_linear_distribution<_RealType>& __d1,
	       const std::piecewise_linear_distribution<_RealType>& __d2)
    { return !(__d1 == __d2); }


  /* @} */ // group random_distributions_poisson

  /* @} */ // group random_distributions

  /**
   * @addtogroup random_utilities Random Number Utilities
   * @ingroup random
   * @{
   */

  /**
   * @brief The seed_seq class generates sequences of seeds for random
   *        number generators.
   */
  class seed_seq
  {
  public:
    /** The type of the seed vales. */
    typedef uint_least32_t result_type;

    /** Default constructor. */
    seed_seq() noexcept
    : _M_v()
    { }

    template<typename _IntType>
      seed_seq(std::initializer_list<_IntType> __il);

    template<typename _InputIterator>
      seed_seq(_InputIterator __begin, _InputIterator __end);

    // generating functions
    template<typename _RandomAccessIterator>
      void
      generate(_RandomAccessIterator __begin, _RandomAccessIterator __end);

    // property functions
    size_t size() const noexcept
    { return _M_v.size(); }

    template<typename _OutputIterator>
      void
      param(_OutputIterator __dest) const
      { std::copy(_M_v.begin(), _M_v.end(), __dest); }

    // no copy functions
    seed_seq(const seed_seq&) = delete;
    seed_seq& operator=(const seed_seq&) = delete;

  private:
    std::vector<result_type> _M_v;
  };

  /* @} */ // group random_utilities

  /* @} */ // group random

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace std

#endif
