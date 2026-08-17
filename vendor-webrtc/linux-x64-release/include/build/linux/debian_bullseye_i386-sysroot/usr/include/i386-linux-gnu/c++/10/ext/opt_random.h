// Optimizations for random number extensions, x86 version -*- C++ -*-

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

/** @file ext/random.tcc
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{ext/random}
 */

#ifndef _EXT_OPT_RANDOM_H
#define _EXT_OPT_RANDOM_H 1

#pragma GCC system_header

#if __BYTE_ORDER__ == __ORDER_LITTLE_ENDIAN__

#ifdef __SSE2__

namespace __gnu_cxx _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  namespace {

    template<size_t __sl1, size_t __sl2, size_t __sr1, size_t __sr2,
	     uint32_t __msk1, uint32_t __msk2, uint32_t __msk3, uint32_t __msk4>
      inline __m128i __sse2_recursion(__m128i __a, __m128i __b,
				      __m128i __c, __m128i __d)
      {
	__m128i __y = _mm_srli_epi32(__b, __sr1);
	__m128i __z = _mm_srli_si128(__c, __sr2);
	__m128i __v = _mm_slli_epi32(__d, __sl1);
	__z = _mm_xor_si128(__z, __a);
	__z = _mm_xor_si128(__z, __v);
	__m128i __x = _mm_slli_si128(__a, __sl2);
	__y = _mm_and_si128(__y, _mm_set_epi32(__msk4, __msk3, __msk2, __msk1));
	__z = _mm_xor_si128(__z, __x);
	return _mm_xor_si128(__z, __y);
      }

  }


#define _GLIBCXX_OPT_HAVE_RANDOM_SFMT_GEN_READ	1
  template<typename _UIntType, size_t __m,
	   size_t __pos1, size_t __sl1, size_t __sl2,
	   size_t __sr1, size_t __sr2,
	   uint32_t __msk1, uint32_t __msk2,
	   uint32_t __msk3, uint32_t __msk4,
	   uint32_t __parity1, uint32_t __parity2,
	   uint32_t __parity3, uint32_t __parity4>
    void simd_fast_mersenne_twister_engine<_UIntType, __m,
					   __pos1, __sl1, __sl2, __sr1, __sr2,
					   __msk1, __msk2, __msk3, __msk4,
					   __parity1, __parity2, __parity3,
					   __parity4>::
    _M_gen_rand(void)
    {
      __m128i __r1 = _mm_load_si128(&_M_state[_M_nstate - 2]);
      __m128i __r2 = _mm_load_si128(&_M_state[_M_nstate - 1]);

      size_t __i;
      for (__i = 0; __i < _M_nstate - __pos1; ++__i)
	{
	  __m128i __r = __sse2_recursion<__sl1, __sl2, __sr1, __sr2,
					 __msk1, __msk2, __msk3, __msk4>
	    (_M_state[__i], _M_state[__i + __pos1], __r1, __r2);
	  _mm_store_si128(&_M_state[__i], __r);
	  __r1 = __r2;
	  __r2 = __r;
	}
      for (; __i < _M_nstate; ++__i)
	{
	  __m128i __r = __sse2_recursion<__sl1, __sl2, __sr1, __sr2,
					 __msk1, __msk2, __msk3, __msk4>
	    (_M_state[__i], _M_state[__i + __pos1 - _M_nstate], __r1, __r2);
	  _mm_store_si128(&_M_state[__i], __r);
	  __r1 = __r2;
	  __r2 = __r;
	}

      _M_pos = 0;
    }


#define _GLIBCXX_OPT_HAVE_RANDOM_SFMT_OPERATOREQUAL	1
  template<typename _UIntType, size_t __m,
	   size_t __pos1, size_t __sl1, size_t __sl2,
	   size_t __sr1, size_t __sr2,
	   uint32_t __msk1, uint32_t __msk2,
	   uint32_t __msk3, uint32_t __msk4,
	   uint32_t __parity1, uint32_t __parity2,
	   uint32_t __parity3, uint32_t __parity4>
    bool
    operator==(const __gnu_cxx::simd_fast_mersenne_twister_engine<_UIntType,
	       __m, __pos1, __sl1, __sl2, __sr1, __sr2,
	       __msk1, __msk2, __msk3, __msk4,
	       __parity1, __parity2, __parity3, __parity4>& __lhs,
	       const __gnu_cxx::simd_fast_mersenne_twister_engine<_UIntType,
	       __m, __pos1, __sl1, __sl2, __sr1, __sr2,
	       __msk1, __msk2, __msk3, __msk4,
	       __parity1, __parity2, __parity3, __parity4>& __rhs)
    {
      __m128i __res = _mm_cmpeq_epi8(__lhs._M_state[0], __rhs._M_state[0]);
      for (size_t __i = 1; __i < __lhs._M_nstate; ++__i)
	__res = _mm_and_si128(__res, _mm_cmpeq_epi8(__lhs._M_state[__i],
						    __rhs._M_state[__i]));
      return (_mm_movemask_epi8(__res) == 0xffff
	      && __lhs._M_pos == __rhs._M_pos);
    }


_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

#endif // __SSE2__

#endif // __BYTE_ORDER__ == __ORDER_LITTLE_ENDIAN__

#endif // _EXT_OPT_RANDOM_H
