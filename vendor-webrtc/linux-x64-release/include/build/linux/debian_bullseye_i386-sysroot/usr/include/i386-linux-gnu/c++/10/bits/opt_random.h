// Optimizations for random number functions, x86 version -*- C++ -*-

// Copyright (C) 2012-2020 Free Software Foundation, Inc.
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

/** @file bits/opt_random.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{random}
 */

#ifndef _BITS_OPT_RANDOM_H
#define _BITS_OPT_RANDOM_H 1

#ifdef __SSE3__
#include <pmmintrin.h>
#endif


#pragma GCC system_header


namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

#ifdef __SSE3__
  template<>
    template<typename _UniformRandomNumberGenerator>
      void
      normal_distribution<double>::
      __generate(typename normal_distribution<double>::result_type* __f,
		 typename normal_distribution<double>::result_type* __t,
		 _UniformRandomNumberGenerator& __urng,
		 const param_type& __param)
      {
	typedef uint64_t __uctype;

	if (__f == __t)
	  return;

	if (_M_saved_available)
	  {
	    _M_saved_available = false;
	    *__f++ = _M_saved * __param.stddev() + __param.mean();

	    if (__f == __t)
	      return;
	  }

	constexpr uint64_t __maskval = 0xfffffffffffffull;
	static const __m128i __mask = _mm_set1_epi64x(__maskval);
	static const __m128i __two = _mm_set1_epi64x(0x4000000000000000ull);
	static const __m128d __three = _mm_set1_pd(3.0);
	const __m128d __av = _mm_set1_pd(__param.mean());

	const __uctype __urngmin = __urng.min();
	const __uctype __urngmax = __urng.max();
	const __uctype __urngrange = __urngmax - __urngmin;
	const __uctype __uerngrange = __urngrange + 1;

	while (__f + 1 < __t)
	  {
	    double __le;
	    __m128d __x;
	    do
	      {
                union
                {
                  __m128i __i;
                  __m128d __d;
		} __v;

		if (__urngrange > __maskval)
		  {
		    if (__detail::_Power_of_2(__uerngrange))
		      __v.__i = _mm_and_si128(_mm_set_epi64x(__urng(),
							     __urng()),
					      __mask);
		    else
		      {
			const __uctype __uerange = __maskval + 1;
			const __uctype __scaling = __urngrange / __uerange;
			const __uctype __past = __uerange * __scaling;
			uint64_t __v1;
			do
			  __v1 = __uctype(__urng()) - __urngmin;
			while (__v1 >= __past);
			__v1 /= __scaling;
			uint64_t __v2;
			do
			  __v2 = __uctype(__urng()) - __urngmin;
			while (__v2 >= __past);
			__v2 /= __scaling;

			__v.__i = _mm_set_epi64x(__v1, __v2);
		      }
		  }
		else if (__urngrange == __maskval)
		  __v.__i = _mm_set_epi64x(__urng(), __urng());
		else if ((__urngrange + 2) * __urngrange >= __maskval
			 && __detail::_Power_of_2(__uerngrange))
		  {
		    uint64_t __v1 = __urng() * __uerngrange + __urng();
		    uint64_t __v2 = __urng() * __uerngrange + __urng();

		    __v.__i = _mm_and_si128(_mm_set_epi64x(__v1, __v2),
					    __mask);
		  }
		else
		  {
		    size_t __nrng = 2;
		    __uctype __high = __maskval / __uerngrange / __uerngrange;
		    while (__high > __uerngrange)
		      {
			++__nrng;
			__high /= __uerngrange;
		      }
		    const __uctype __highrange = __high + 1;
		    const __uctype __scaling = __urngrange / __highrange;
		    const __uctype __past = __highrange * __scaling;
		    __uctype __tmp;

		    uint64_t __v1;
		    do
		      {
			do
			  __tmp = __uctype(__urng()) - __urngmin;
			while (__tmp >= __past);
			__v1 = __tmp / __scaling;
			for (size_t __cnt = 0; __cnt < __nrng; ++__cnt)
			  {
			    __tmp = __v1;
			    __v1 *= __uerngrange;
			    __v1 += __uctype(__urng()) - __urngmin;
			  }
		      }
		    while (__v1 > __maskval || __v1 < __tmp);

		    uint64_t __v2;
		    do
		      {
			do
			  __tmp = __uctype(__urng()) - __urngmin;
			while (__tmp >= __past);
			__v2 = __tmp / __scaling;
			for (size_t __cnt = 0; __cnt < __nrng; ++__cnt)
			  {
			    __tmp = __v2;
			    __v2 *= __uerngrange;
			    __v2 += __uctype(__urng()) - __urngmin;
			  }
		      }
		    while (__v2 > __maskval || __v2 < __tmp);

		    __v.__i = _mm_set_epi64x(__v1, __v2);
		  }

		__v.__i = _mm_or_si128(__v.__i, __two);
		__x = _mm_sub_pd(__v.__d, __three);
		__m128d __m = _mm_mul_pd(__x, __x);
		__le = _mm_cvtsd_f64(_mm_hadd_pd (__m, __m));
              }
            while (__le == 0.0 || __le >= 1.0);

            double __mult = (std::sqrt(-2.0 * std::log(__le) / __le)
                             * __param.stddev());

            __x = _mm_add_pd(_mm_mul_pd(__x, _mm_set1_pd(__mult)), __av);

            _mm_storeu_pd(__f, __x);
            __f += 2;
          }

        if (__f != __t)
          {
            result_type __x, __y, __r2;

            __detail::_Adaptor<_UniformRandomNumberGenerator, result_type>
              __aurng(__urng);

            do
              {
                __x = result_type(2.0) * __aurng() - 1.0;
                __y = result_type(2.0) * __aurng() - 1.0;
                __r2 = __x * __x + __y * __y;
              }
            while (__r2 > 1.0 || __r2 == 0.0);

            const result_type __mult = std::sqrt(-2 * std::log(__r2) / __r2);
            _M_saved = __x * __mult;
            _M_saved_available = true;
            *__f = __y * __mult * __param.stddev() + __param.mean();
          }
      }
#endif


_GLIBCXX_END_NAMESPACE_VERSION
} // namespace


#endif // _BITS_OPT_RANDOM_H
