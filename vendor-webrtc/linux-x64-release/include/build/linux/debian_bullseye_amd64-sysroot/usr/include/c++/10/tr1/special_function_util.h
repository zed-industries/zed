// Special functions -*- C++ -*-

// Copyright (C) 2006-2020 Free Software Foundation, Inc.
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

/** @file tr1/special_function_util.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{tr1/cmath}
 */

//
// ISO C++ 14882 TR1: 5.2  Special functions
//

// Written by Edward Smith-Rowland based on numerous mathematics books.

#ifndef _GLIBCXX_TR1_SPECIAL_FUNCTION_UTIL_H
#define _GLIBCXX_TR1_SPECIAL_FUNCTION_UTIL_H 1

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

#if _GLIBCXX_USE_STD_SPEC_FUNCS
#elif defined(_GLIBCXX_TR1_CMATH)
namespace tr1
{
#else
# error do not include this header directly, use <cmath> or <tr1/cmath>
#endif
  namespace __detail
  {
    /// A class to encapsulate type dependent floating point
    /// constants.  Not everything will be able to be expressed as
    /// type logic.
    template<typename _Tp>
    struct __floating_point_constant
    {
      static const _Tp __value;
    };


    /// A structure for numeric constants.
    template<typename _Tp>
      struct __numeric_constants
      {
        ///  Constant @f$ \pi @f$.
        static _Tp __pi() throw()
        { return static_cast<_Tp>(3.1415926535897932384626433832795029L); }
        ///  Constant @f$ \pi / 2 @f$.
        static _Tp __pi_2() throw()
        { return static_cast<_Tp>(1.5707963267948966192313216916397514L); }
        ///  Constant @f$ \pi / 3 @f$.
        static _Tp __pi_3() throw()
        { return static_cast<_Tp>(1.0471975511965977461542144610931676L); }
        ///  Constant @f$ \pi / 4 @f$.
        static _Tp __pi_4() throw()
        { return static_cast<_Tp>(0.7853981633974483096156608458198757L); }
        ///  Constant @f$ 1 / \pi @f$.
        static _Tp __1_pi() throw()
        { return static_cast<_Tp>(0.3183098861837906715377675267450287L); }
        ///  Constant @f$ 2 / \sqrt(\pi) @f$.
        static _Tp __2_sqrtpi() throw()
        { return static_cast<_Tp>(1.1283791670955125738961589031215452L); }
        ///  Constant @f$ \sqrt(2) @f$.
        static _Tp __sqrt2() throw()
        { return static_cast<_Tp>(1.4142135623730950488016887242096981L); }
        ///  Constant @f$ \sqrt(3) @f$.
        static _Tp __sqrt3() throw()
        { return static_cast<_Tp>(1.7320508075688772935274463415058723L); }
        ///  Constant @f$ \sqrt(\pi/2) @f$.
        static _Tp __sqrtpio2() throw()
        { return static_cast<_Tp>(1.2533141373155002512078826424055226L); }
        ///  Constant @f$ 1 / sqrt(2) @f$.
        static _Tp __sqrt1_2() throw()
        { return static_cast<_Tp>(0.7071067811865475244008443621048490L); }
        ///  Constant @f$ \log(\pi) @f$.
        static _Tp __lnpi() throw()
        { return static_cast<_Tp>(1.1447298858494001741434273513530587L); }
        ///  Constant Euler's constant @f$ \gamma_E @f$.
        static _Tp __gamma_e() throw()
        { return static_cast<_Tp>(0.5772156649015328606065120900824024L); }
        ///  Constant Euler-Mascheroni @f$ e @f$
        static _Tp __euler() throw()
        { return static_cast<_Tp>(2.7182818284590452353602874713526625L); }
      };


#if _GLIBCXX_USE_C99_MATH && !_GLIBCXX_USE_C99_FP_MACROS_DYNAMIC

    /// This is a wrapper for the isnan function. Otherwise, for NaN,
    /// all comparisons result in false. If/when we build a std::isnan
    /// out of intrinsics, this will disappear completely in favor of
    /// std::isnan.
    template<typename _Tp>
    inline bool __isnan(_Tp __x)
    { return std::isnan(__x); }

#else

    template<typename _Tp>
    inline bool __isnan(const _Tp __x)
    { return __builtin_isnan(__x); }

    template<>
    inline bool __isnan<float>(float __x)
    { return __builtin_isnanf(__x); }

    template<>
    inline bool __isnan<long double>(long double __x)
    { return __builtin_isnanl(__x); }

#endif
  } // namespace __detail
#if ! _GLIBCXX_USE_STD_SPEC_FUNCS && defined(_GLIBCXX_TR1_CMATH)
} // namespace tr1
#endif

_GLIBCXX_END_NAMESPACE_VERSION
}

#endif // _GLIBCXX_TR1_SPECIAL_FUNCTION_UTIL_H

