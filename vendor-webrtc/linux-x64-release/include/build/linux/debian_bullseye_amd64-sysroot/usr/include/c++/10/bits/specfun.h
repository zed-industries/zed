// Mathematical Special Functions for -*- C++ -*-

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

/** @file bits/specfun.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{cmath}
 */

#ifndef _GLIBCXX_BITS_SPECFUN_H
#define _GLIBCXX_BITS_SPECFUN_H 1

#pragma GCC visibility push(default)

#include <bits/c++config.h>

#define __STDCPP_MATH_SPEC_FUNCS__ 201003L

#define __cpp_lib_math_special_functions 201603L

#if __cplusplus <= 201403L && __STDCPP_WANT_MATH_SPEC_FUNCS__ == 0
# error include <cmath> and define __STDCPP_WANT_MATH_SPEC_FUNCS__
#endif

#include <bits/stl_algobase.h>
#include <limits>
#include <type_traits>

#include <tr1/gamma.tcc>
#include <tr1/bessel_function.tcc>
#include <tr1/beta_function.tcc>
#include <tr1/ell_integral.tcc>
#include <tr1/exp_integral.tcc>
#include <tr1/hypergeometric.tcc>
#include <tr1/legendre_function.tcc>
#include <tr1/modified_bessel_func.tcc>
#include <tr1/poly_hermite.tcc>
#include <tr1/poly_laguerre.tcc>
#include <tr1/riemann_zeta.tcc>

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  /**
   * @defgroup mathsf Mathematical Special Functions
   * @ingroup numerics
   *
   * @section mathsf_desc Mathematical Special Functions
   *
   * A collection of advanced mathematical special functions,
   * defined by ISO/IEC IS 29124 and then added to ISO C++ 2017.
   *
   *
   * @subsection mathsf_intro Introduction and History
   * The first significant library upgrade on the road to C++2011,
   * <a href="http://www.open-std.org/JTC1/SC22/WG21/docs/papers/2005/n1836.pdf">
   * TR1</a>, included a set of 23 mathematical functions that significantly
   * extended the standard transcendental functions inherited from C and declared
   * in @<cmath@>.
   *
   * Although most components from TR1 were eventually adopted for C++11 these
   * math functions were left behind out of concern for implementability.
   * The math functions were published as a separate international standard
   * <a href="http://www.open-std.org/JTC1/SC22/WG21/docs/papers/2010/n3060.pdf">
   * IS 29124 - Extensions to the C++ Library to Support Mathematical Special
   * Functions</a>.
   *
   * For C++17 these functions were incorporated into the main standard.
   *
   * @subsection mathsf_contents Contents
   * The following functions are implemented in namespace @c std:
   * - @ref assoc_laguerre "assoc_laguerre - Associated Laguerre functions"
   * - @ref assoc_legendre "assoc_legendre - Associated Legendre functions"
   * - @ref beta "beta - Beta functions"
   * - @ref comp_ellint_1 "comp_ellint_1 - Complete elliptic functions of the first kind"
   * - @ref comp_ellint_2 "comp_ellint_2 - Complete elliptic functions of the second kind"
   * - @ref comp_ellint_3 "comp_ellint_3 - Complete elliptic functions of the third kind"
   * - @ref cyl_bessel_i "cyl_bessel_i - Regular modified cylindrical Bessel functions"
   * - @ref cyl_bessel_j "cyl_bessel_j - Cylindrical Bessel functions of the first kind"
   * - @ref cyl_bessel_k "cyl_bessel_k - Irregular modified cylindrical Bessel functions"
   * - @ref cyl_neumann "cyl_neumann - Cylindrical Neumann functions or Cylindrical Bessel functions of the second kind"
   * - @ref ellint_1 "ellint_1 - Incomplete elliptic functions of the first kind"
   * - @ref ellint_2 "ellint_2 - Incomplete elliptic functions of the second kind"
   * - @ref ellint_3 "ellint_3 - Incomplete elliptic functions of the third kind"
   * - @ref expint "expint - The exponential integral"
   * - @ref hermite "hermite - Hermite polynomials"
   * - @ref laguerre "laguerre - Laguerre functions"
   * - @ref legendre "legendre - Legendre polynomials"
   * - @ref riemann_zeta "riemann_zeta - The Riemann zeta function"
   * - @ref sph_bessel "sph_bessel - Spherical Bessel functions"
   * - @ref sph_legendre "sph_legendre - Spherical Legendre functions"
   * - @ref sph_neumann "sph_neumann - Spherical Neumann functions"
   *
   * The hypergeometric functions were stricken from the TR29124 and C++17
   * versions of this math library because of implementation concerns.
   * However, since they were in the TR1 version and since they are popular
   * we kept them as an extension in namespace @c __gnu_cxx:
   * - @ref __gnu_cxx::conf_hyperg "conf_hyperg - Confluent hypergeometric functions"
   * - @ref __gnu_cxx::hyperg "hyperg - Hypergeometric functions"
   *
   * <!-- @subsection mathsf_general General Features -->
   *
   * @subsection mathsf_promotion Argument Promotion
   * The arguments suppled to the non-suffixed functions will be promoted
   * according to the following rules:
   * 1. If any argument intended to be floating point is given an integral value
   * That integral value is promoted to double.
   * 2. All floating point arguments are promoted up to the largest floating
   *    point precision among them.
   *
   * @subsection mathsf_NaN NaN Arguments
   * If any of the floating point arguments supplied to these functions is
   * invalid or NaN (std::numeric_limits<Tp>::quiet_NaN),
   * the value NaN is returned.
   *
   * @subsection mathsf_impl Implementation
   *
   * We strive to implement the underlying math with type generic algorithms
   * to the greatest extent possible.  In practice, the functions are thin
   * wrappers that dispatch to function templates. Type dependence is
   * controlled with std::numeric_limits and functions thereof.
   *
   * We don't promote @c float to @c double or @c double to <tt>long double</tt>
   * reflexively.  The goal is for @c float functions to operate more quickly,
   * at the cost of @c float accuracy and possibly a smaller domain of validity.
   * Similaryly, <tt>long double</tt> should give you more dynamic range
   * and slightly more pecision than @c double on many systems.
   *
   * @subsection mathsf_testing Testing
   *
   * These functions have been tested against equivalent implementations
   * from the <a href="http://www.gnu.org/software/gsl">
   * Gnu Scientific Library, GSL</a> and
   * <a href="http://www.boost.org/doc/libs/1_60_0/libs/math/doc/html/index.html">Boost</a>
   * and the ratio
   * @f[
   *   \frac{|f - f_{test}|}{|f_{test}|}
   * @f]
   * is generally found to be within 10<sup>-15</sup> for 64-bit double on
   * linux-x86_64 systems over most of the ranges of validity.
   * 
   * @todo Provide accuracy comparisons on a per-function basis for a small
   *       number of targets.
   *
   * @subsection mathsf_bibliography General Bibliography
   *
   * @see Abramowitz and Stegun: Handbook of Mathematical Functions,
   * with Formulas, Graphs, and Mathematical Tables
   * Edited by Milton Abramowitz and Irene A. Stegun,
   * National Bureau of Standards  Applied Mathematics Series - 55
   * Issued June 1964, Tenth Printing, December 1972, with corrections
   * Electronic versions of A&S abound including both pdf and navigable html.
   * @see for example  http://people.math.sfu.ca/~cbm/aands/
   *
   * @see The old A&S has been redone as the
   * NIST Digital Library of Mathematical Functions: http://dlmf.nist.gov/
   * This version is far more navigable and includes more recent work.
   *
   * @see An Atlas of Functions: with Equator, the Atlas Function Calculator
   * 2nd Edition, by Oldham, Keith B., Myland, Jan, Spanier, Jerome
   *
   * @see Asymptotics and Special Functions by Frank W. J. Olver,
   * Academic Press, 1974
   *
   * @see Numerical Recipes in C, The Art of Scientific Computing,
   * by William H. Press, Second Ed., Saul A. Teukolsky,
   * William T. Vetterling, and Brian P. Flannery,
   * Cambridge University Press, 1992
   *
   * @see The Special Functions and Their Approximations: Volumes 1 and 2,
   * by Yudell L. Luke, Academic Press, 1969
   *
   * @{
   */

  // Associated Laguerre polynomials

  /**
   * Return the associated Laguerre polynomial of order @c n,
   * degree @c m: @f$ L_n^m(x) @f$ for @c float argument.
   *
   * @see assoc_laguerre for more details.
   */
  inline float
  assoc_laguerref(unsigned int __n, unsigned int __m, float __x)
  { return __detail::__assoc_laguerre<float>(__n, __m, __x); }

  /**
   * Return the associated Laguerre polynomial of order @c n,
   * degree @c m: @f$ L_n^m(x) @f$.
   *
   * @see assoc_laguerre for more details.
   */
  inline long double
  assoc_laguerrel(unsigned int __n, unsigned int __m, long double __x)
  { return __detail::__assoc_laguerre<long double>(__n, __m, __x); }

  /**
   * Return the associated Laguerre polynomial of nonnegative order @c n,
   * nonnegative degree @c m and real argument @c x: @f$ L_n^m(x) @f$.
   *
   * The associated Laguerre function of real degree @f$ \alpha @f$,
   * @f$ L_n^\alpha(x) @f$, is defined by
   * @f[
   * 	 L_n^\alpha(x) = \frac{(\alpha + 1)_n}{n!}
   * 			 {}_1F_1(-n; \alpha + 1; x)
   * @f]
   * where @f$ (\alpha)_n @f$ is the Pochhammer symbol and
   * @f$ {}_1F_1(a; c; x) @f$ is the confluent hypergeometric function.
   *
   * The associated Laguerre polynomial is defined for integral
   * degree @f$ \alpha = m @f$ by:
   * @f[
   * 	 L_n^m(x) = (-1)^m \frac{d^m}{dx^m} L_{n + m}(x)
   * @f]
   * where the Laguerre polynomial is defined by:
   * @f[
   * 	 L_n(x) = \frac{e^x}{n!} \frac{d^n}{dx^n} (x^ne^{-x})
   * @f]
   * and @f$ x >= 0 @f$.
   * @see laguerre for details of the Laguerre function of degree @c n
   *
   * @tparam _Tp The floating-point type of the argument @c __x.
   * @param __n The order of the Laguerre function, <tt>__n >= 0</tt>.
   * @param __m The degree of the Laguerre function, <tt>__m >= 0</tt>.
   * @param __x The argument of the Laguerre function, <tt>__x >= 0</tt>.
   * @throw std::domain_error if <tt>__x < 0</tt>.
   */
  template<typename _Tp>
    inline typename __gnu_cxx::__promote<_Tp>::__type
    assoc_laguerre(unsigned int __n, unsigned int __m, _Tp __x)
    {
      typedef typename __gnu_cxx::__promote<_Tp>::__type __type;
      return __detail::__assoc_laguerre<__type>(__n, __m, __x);
    }

  // Associated Legendre functions

  /**
   * Return the associated Legendre function of degree @c l and order @c m
   * for @c float argument.
   *
   * @see assoc_legendre for more details.
   */
  inline float
  assoc_legendref(unsigned int __l, unsigned int __m, float __x)
  { return __detail::__assoc_legendre_p<float>(__l, __m, __x); }

  /**
   * Return the associated Legendre function of degree @c l and order @c m.
   *
   * @see assoc_legendre for more details.
   */
  inline long double
  assoc_legendrel(unsigned int __l, unsigned int __m, long double __x)
  { return __detail::__assoc_legendre_p<long double>(__l, __m, __x); }


  /**
   * Return the associated Legendre function of degree @c l and order @c m.
   *
   * The associated Legendre function is derived from the Legendre function
   * @f$ P_l(x) @f$ by the Rodrigues formula:
   * @f[
   *   P_l^m(x) = (1 - x^2)^{m/2}\frac{d^m}{dx^m}P_l(x)
   * @f]
   * @see legendre for details of the Legendre function of degree @c l
   *
   * @tparam _Tp The floating-point type of the argument @c __x.
   * @param  __l  The degree <tt>__l >= 0</tt>.
   * @param  __m  The order <tt>__m <= l</tt>.
   * @param  __x  The argument, <tt>abs(__x) <= 1</tt>.
   * @throw std::domain_error if <tt>abs(__x) > 1</tt>.
   */
  template<typename _Tp>
    inline typename __gnu_cxx::__promote<_Tp>::__type
    assoc_legendre(unsigned int __l, unsigned int __m, _Tp __x)
    {
      typedef typename __gnu_cxx::__promote<_Tp>::__type __type;
      return __detail::__assoc_legendre_p<__type>(__l, __m, __x);
    }

  // Beta functions

  /**
   * Return the beta function, @f$ B(a,b) @f$, for @c float parameters @c a, @c b.
   *
   * @see beta for more details.
   */
  inline float
  betaf(float __a, float __b)
  { return __detail::__beta<float>(__a, __b); }

  /**
   * Return the beta function, @f$B(a,b)@f$, for long double
   * parameters @c a, @c b.
   *
   * @see beta for more details.
   */
  inline long double
  betal(long double __a, long double __b)
  { return __detail::__beta<long double>(__a, __b); }

  /**
   * Return the beta function, @f$B(a,b)@f$, for real parameters @c a, @c b.
   *
   * The beta function is defined by
   * @f[
   *   B(a,b) = \int_0^1 t^{a - 1} (1 - t)^{b - 1} dt
   *          = \frac{\Gamma(a)\Gamma(b)}{\Gamma(a+b)}
   * @f]
   * where @f$ a > 0 @f$ and @f$ b > 0 @f$
   *
   * @tparam _Tpa The floating-point type of the parameter @c __a.
   * @tparam _Tpb The floating-point type of the parameter @c __b.
   * @param __a The first argument of the beta function, <tt> __a > 0 </tt>.
   * @param __b The second argument of the beta function, <tt> __b > 0 </tt>.
   * @throw std::domain_error if <tt> __a < 0 </tt> or <tt> __b < 0 </tt>.
   */
  template<typename _Tpa, typename _Tpb>
    inline typename __gnu_cxx::__promote_2<_Tpa, _Tpb>::__type
    beta(_Tpa __a, _Tpb __b)
    {
      typedef typename __gnu_cxx::__promote_2<_Tpa, _Tpb>::__type __type;
      return __detail::__beta<__type>(__a, __b);
    }

  // Complete elliptic integrals of the first kind

  /**
   * Return the complete elliptic integral of the first kind @f$ E(k) @f$
   * for @c float modulus @c k.
   *
   * @see comp_ellint_1 for details.
   */
  inline float
  comp_ellint_1f(float __k)
  { return __detail::__comp_ellint_1<float>(__k); }

  /**
   * Return the complete elliptic integral of the first kind @f$ E(k) @f$
   * for long double modulus @c k.
   *
   * @see comp_ellint_1 for details.
   */
  inline long double
  comp_ellint_1l(long double __k)
  { return __detail::__comp_ellint_1<long double>(__k); }

  /**
   * Return the complete elliptic integral of the first kind
   * @f$ K(k) @f$ for real modulus @c k.
   *
   * The complete elliptic integral of the first kind is defined as
   * @f[
   *   K(k) = F(k,\pi/2) = \int_0^{\pi/2}\frac{d\theta}
   * 					     {\sqrt{1 - k^2 sin^2\theta}}
   * @f]
   * where @f$ F(k,\phi) @f$ is the incomplete elliptic integral of the
   * first kind and the modulus @f$ |k| <= 1 @f$.
   * @see ellint_1 for details of the incomplete elliptic function
   * of the first kind.
   *
   * @tparam _Tp The floating-point type of the modulus @c __k.
   * @param  __k  The modulus, <tt> abs(__k) <= 1 </tt>
   * @throw std::domain_error if <tt> abs(__k) > 1 </tt>.
   */
  template<typename _Tp>
    inline typename __gnu_cxx::__promote<_Tp>::__type
    comp_ellint_1(_Tp __k)
    {
      typedef typename __gnu_cxx::__promote<_Tp>::__type __type;
      return __detail::__comp_ellint_1<__type>(__k);
    }

  // Complete elliptic integrals of the second kind

  /**
   * Return the complete elliptic integral of the second kind @f$ E(k) @f$
   * for @c float modulus @c k.
   *
   * @see comp_ellint_2 for details.
   */
  inline float
  comp_ellint_2f(float __k)
  { return __detail::__comp_ellint_2<float>(__k); }

  /**
   * Return the complete elliptic integral of the second kind @f$ E(k) @f$
   * for long double modulus @c k.
   *
   * @see comp_ellint_2 for details.
   */
  inline long double
  comp_ellint_2l(long double __k)
  { return __detail::__comp_ellint_2<long double>(__k); }

  /**
   * Return the complete elliptic integral of the second kind @f$ E(k) @f$
   * for real modulus @c k.
   *
   * The complete elliptic integral of the second kind is defined as
   * @f[
   *   E(k) = E(k,\pi/2) = \int_0^{\pi/2}\sqrt{1 - k^2 sin^2\theta}
   * @f]
   * where @f$ E(k,\phi) @f$ is the incomplete elliptic integral of the
   * second kind and the modulus @f$ |k| <= 1 @f$.
   * @see ellint_2 for details of the incomplete elliptic function
   * of the second kind.
   *
   * @tparam _Tp The floating-point type of the modulus @c __k.
   * @param  __k  The modulus, @c abs(__k) <= 1
   * @throw std::domain_error if @c abs(__k) > 1.
   */
  template<typename _Tp>
    inline typename __gnu_cxx::__promote<_Tp>::__type
    comp_ellint_2(_Tp __k)
    {
      typedef typename __gnu_cxx::__promote<_Tp>::__type __type;
      return __detail::__comp_ellint_2<__type>(__k);
    }

  // Complete elliptic integrals of the third kind

  /**
   * @brief Return the complete elliptic integral of the third kind
   * @f$ \Pi(k,\nu) @f$ for @c float modulus @c k.
   *
   * @see comp_ellint_3 for details.
   */
  inline float
  comp_ellint_3f(float __k, float __nu)
  { return __detail::__comp_ellint_3<float>(__k, __nu); }

  /**
   * @brief Return the complete elliptic integral of the third kind
   * @f$ \Pi(k,\nu) @f$ for <tt>long double</tt> modulus @c k.
   *
   * @see comp_ellint_3 for details.
   */
  inline long double
  comp_ellint_3l(long double __k, long double __nu)
  { return __detail::__comp_ellint_3<long double>(__k, __nu); }

  /**
   * Return the complete elliptic integral of the third kind
   * @f$ \Pi(k,\nu) = \Pi(k,\nu,\pi/2) @f$ for real modulus @c k.
   *
   * The complete elliptic integral of the third kind is defined as
   * @f[
   *   \Pi(k,\nu) = \Pi(k,\nu,\pi/2) = \int_0^{\pi/2}
   * 		     \frac{d\theta}
   * 		   {(1 - \nu \sin^2\theta)\sqrt{1 - k^2 \sin^2\theta}}
   * @f]
   * where @f$ \Pi(k,\nu,\phi) @f$ is the incomplete elliptic integral of the
   * second kind and the modulus @f$ |k| <= 1 @f$.
   * @see ellint_3 for details of the incomplete elliptic function
   * of the third kind.
   *
   * @tparam _Tp The floating-point type of the modulus @c __k.
   * @tparam _Tpn The floating-point type of the argument @c __nu.
   * @param  __k  The modulus, @c abs(__k) <= 1
   * @param  __nu  The argument
   * @throw std::domain_error if @c abs(__k) > 1.
   */
  template<typename _Tp, typename _Tpn>
    inline typename __gnu_cxx::__promote_2<_Tp, _Tpn>::__type
    comp_ellint_3(_Tp __k, _Tpn __nu)
    {
      typedef typename __gnu_cxx::__promote_2<_Tp, _Tpn>::__type __type;
      return __detail::__comp_ellint_3<__type>(__k, __nu);
    }

  // Regular modified cylindrical Bessel functions

  /**
   * Return the regular modified Bessel function @f$ I_{\nu}(x) @f$
   * for @c float order @f$ \nu @f$ and argument @f$ x >= 0 @f$.
   *
   * @see cyl_bessel_i for setails.
   */
  inline float
  cyl_bessel_if(float __nu, float __x)
  { return __detail::__cyl_bessel_i<float>(__nu, __x); }

  /**
   * Return the regular modified Bessel function @f$ I_{\nu}(x) @f$
   * for <tt>long double</tt> order @f$ \nu @f$ and argument @f$ x >= 0 @f$.
   *
   * @see cyl_bessel_i for setails.
   */
  inline long double
  cyl_bessel_il(long double __nu, long double __x)
  { return __detail::__cyl_bessel_i<long double>(__nu, __x); }

  /**
   * Return the regular modified Bessel function @f$ I_{\nu}(x) @f$
   * for real order @f$ \nu @f$ and argument @f$ x >= 0 @f$.
   *
   * The regular modified cylindrical Bessel function is:
   * @f[
   *  I_{\nu}(x) = i^{-\nu}J_\nu(ix) = \sum_{k=0}^{\infty}
   * 		\frac{(x/2)^{\nu + 2k}}{k!\Gamma(\nu+k+1)}
   * @f]
   *
   * @tparam _Tpnu The floating-point type of the order @c __nu.
   * @tparam _Tp The floating-point type of the argument @c __x.
   * @param  __nu  The order
   * @param  __x   The argument, <tt> __x >= 0 </tt>
   * @throw std::domain_error if <tt> __x < 0 </tt>.
   */
  template<typename _Tpnu, typename _Tp>
    inline typename __gnu_cxx::__promote_2<_Tpnu, _Tp>::__type
    cyl_bessel_i(_Tpnu __nu, _Tp __x)
    {
      typedef typename __gnu_cxx::__promote_2<_Tpnu, _Tp>::__type __type;
      return __detail::__cyl_bessel_i<__type>(__nu, __x);
    }

  // Cylindrical Bessel functions (of the first kind)

  /**
   * Return the Bessel function of the first kind @f$ J_{\nu}(x) @f$
   * for @c float order @f$ \nu @f$ and argument @f$ x >= 0 @f$.
   *
   * @see cyl_bessel_j for setails.
   */
  inline float
  cyl_bessel_jf(float __nu, float __x)
  { return __detail::__cyl_bessel_j<float>(__nu, __x); }

  /**
   * Return the Bessel function of the first kind @f$ J_{\nu}(x) @f$
   * for <tt>long double</tt> order @f$ \nu @f$ and argument @f$ x >= 0 @f$.
   *
   * @see cyl_bessel_j for setails.
   */
  inline long double
  cyl_bessel_jl(long double __nu, long double __x)
  { return __detail::__cyl_bessel_j<long double>(__nu, __x); }

  /**
   * Return the Bessel function @f$ J_{\nu}(x) @f$ of real order @f$ \nu @f$
   * and argument @f$ x >= 0 @f$.
   *
   * The cylindrical Bessel function is:
   * @f[
   *    J_{\nu}(x) = \sum_{k=0}^{\infty}
   *              \frac{(-1)^k (x/2)^{\nu + 2k}}{k!\Gamma(\nu+k+1)}
   * @f]
   *
   * @tparam _Tpnu The floating-point type of the order @c __nu.
   * @tparam _Tp The floating-point type of the argument @c __x.
   * @param  __nu  The order
   * @param  __x   The argument, <tt> __x >= 0 </tt>
   * @throw std::domain_error if <tt> __x < 0 </tt>.
   */
  template<typename _Tpnu, typename _Tp>
    inline typename __gnu_cxx::__promote_2<_Tpnu, _Tp>::__type
    cyl_bessel_j(_Tpnu __nu, _Tp __x)
    {
      typedef typename __gnu_cxx::__promote_2<_Tpnu, _Tp>::__type __type;
      return __detail::__cyl_bessel_j<__type>(__nu, __x);
    }

  // Irregular modified cylindrical Bessel functions

  /**
   * Return the irregular modified Bessel function @f$ K_{\nu}(x) @f$
   * for @c float order @f$ \nu @f$ and argument @f$ x >= 0 @f$.
   *
   * @see cyl_bessel_k for setails.
   */
  inline float
  cyl_bessel_kf(float __nu, float __x)
  { return __detail::__cyl_bessel_k<float>(__nu, __x); }

  /**
   * Return the irregular modified Bessel function @f$ K_{\nu}(x) @f$
   * for <tt>long double</tt> order @f$ \nu @f$ and argument @f$ x >= 0 @f$.
   *
   * @see cyl_bessel_k for setails.
   */
  inline long double
  cyl_bessel_kl(long double __nu, long double __x)
  { return __detail::__cyl_bessel_k<long double>(__nu, __x); }

  /**
   * Return the irregular modified Bessel function @f$ K_{\nu}(x) @f$
   * of real order @f$ \nu @f$ and argument @f$ x @f$.
   *
   * The irregular modified Bessel function is defined by:
   * @f[
   * 	K_{\nu}(x) = \frac{\pi}{2}
   * 		     \frac{I_{-\nu}(x) - I_{\nu}(x)}{\sin \nu\pi}
   * @f]
   * where for integral @f$ \nu = n @f$ a limit is taken:
   * @f$ lim_{\nu \to n} @f$.
   * For negative argument we have simply:
   * @f[
   * 	K_{-\nu}(x) = K_{\nu}(x)
   * @f]
   *
   * @tparam _Tpnu The floating-point type of the order @c __nu.
   * @tparam _Tp The floating-point type of the argument @c __x.
   * @param  __nu  The order
   * @param  __x   The argument, <tt> __x >= 0 </tt>
   * @throw std::domain_error if <tt> __x < 0 </tt>.
   */
  template<typename _Tpnu, typename _Tp>
    inline typename __gnu_cxx::__promote_2<_Tpnu, _Tp>::__type
    cyl_bessel_k(_Tpnu __nu, _Tp __x)
    {
      typedef typename __gnu_cxx::__promote_2<_Tpnu, _Tp>::__type __type;
      return __detail::__cyl_bessel_k<__type>(__nu, __x);
    }

  // Cylindrical Neumann functions

  /**
   * Return the Neumann function @f$ N_{\nu}(x) @f$
   * of @c float order @f$ \nu @f$ and argument @f$ x @f$.
   *
   * @see cyl_neumann for setails.
   */
  inline float
  cyl_neumannf(float __nu, float __x)
  { return __detail::__cyl_neumann_n<float>(__nu, __x); }

  /**
   * Return the Neumann function @f$ N_{\nu}(x) @f$
   * of <tt>long double</tt> order @f$ \nu @f$ and argument @f$ x @f$.
   *
   * @see cyl_neumann for setails.
   */
  inline long double
  cyl_neumannl(long double __nu, long double __x)
  { return __detail::__cyl_neumann_n<long double>(__nu, __x); }

  /**
   * Return the Neumann function @f$ N_{\nu}(x) @f$
   * of real order @f$ \nu @f$ and argument @f$ x >= 0 @f$.
   *
   * The Neumann function is defined by:
   * @f[
   *    N_{\nu}(x) = \frac{J_{\nu}(x) \cos \nu\pi - J_{-\nu}(x)}
   *                      {\sin \nu\pi}
   * @f]
   * where @f$ x >= 0 @f$ and for integral order @f$ \nu = n @f$
   * a limit is taken: @f$ lim_{\nu \to n} @f$.
   *
   * @tparam _Tpnu The floating-point type of the order @c __nu.
   * @tparam _Tp The floating-point type of the argument @c __x.
   * @param  __nu  The order
   * @param  __x   The argument, <tt> __x >= 0 </tt>
   * @throw std::domain_error if <tt> __x < 0 </tt>.
   */
  template<typename _Tpnu, typename _Tp>
    inline typename __gnu_cxx::__promote_2<_Tpnu, _Tp>::__type
    cyl_neumann(_Tpnu __nu, _Tp __x)
    {
      typedef typename __gnu_cxx::__promote_2<_Tpnu, _Tp>::__type __type;
      return __detail::__cyl_neumann_n<__type>(__nu, __x);
    }

  // Incomplete elliptic integrals of the first kind

  /**
   * Return the incomplete elliptic integral of the first kind @f$ E(k,\phi) @f$
   * for @c float modulus @f$ k @f$ and angle @f$ \phi @f$.
   *
   * @see ellint_1 for details.
   */
  inline float
  ellint_1f(float __k, float __phi)
  { return __detail::__ellint_1<float>(__k, __phi); }

  /**
   * Return the incomplete elliptic integral of the first kind @f$ E(k,\phi) @f$
   * for <tt>long double</tt> modulus @f$ k @f$ and angle @f$ \phi @f$.
   *
   * @see ellint_1 for details.
   */
  inline long double
  ellint_1l(long double __k, long double __phi)
  { return __detail::__ellint_1<long double>(__k, __phi); }

  /**
   * Return the incomplete elliptic integral of the first kind @f$ F(k,\phi) @f$
   * for @c real modulus @f$ k @f$ and angle @f$ \phi @f$.
   *
   * The incomplete elliptic integral of the first kind is defined as
   * @f[
   *   F(k,\phi) = \int_0^{\phi}\frac{d\theta}
   * 				     {\sqrt{1 - k^2 sin^2\theta}}
   * @f]
   * For  @f$ \phi= \pi/2 @f$ this becomes the complete elliptic integral of
   * the first kind, @f$ K(k) @f$.  @see comp_ellint_1.
   *
   * @tparam _Tp The floating-point type of the modulus @c __k.
   * @tparam _Tpp The floating-point type of the angle @c __phi.
   * @param  __k  The modulus, <tt> abs(__k) <= 1 </tt>
   * @param  __phi  The integral limit argument in radians
   * @throw std::domain_error if <tt> abs(__k) > 1 </tt>.
   */
  template<typename _Tp, typename _Tpp>
    inline typename __gnu_cxx::__promote_2<_Tp, _Tpp>::__type
    ellint_1(_Tp __k, _Tpp __phi)
    {
      typedef typename __gnu_cxx::__promote_2<_Tp, _Tpp>::__type __type;
      return __detail::__ellint_1<__type>(__k, __phi);
    }

  // Incomplete elliptic integrals of the second kind

  /**
   * @brief Return the incomplete elliptic integral of the second kind
   * @f$ E(k,\phi) @f$ for @c float argument.
   *
   * @see ellint_2 for details.
   */
  inline float
  ellint_2f(float __k, float __phi)
  { return __detail::__ellint_2<float>(__k, __phi); }

  /**
   * @brief Return the incomplete elliptic integral of the second kind
   * @f$ E(k,\phi) @f$.
   *
   * @see ellint_2 for details.
   */
  inline long double
  ellint_2l(long double __k, long double __phi)
  { return __detail::__ellint_2<long double>(__k, __phi); }

  /**
   * Return the incomplete elliptic integral of the second kind
   * @f$ E(k,\phi) @f$.
   *
   * The incomplete elliptic integral of the second kind is defined as
   * @f[
   *   E(k,\phi) = \int_0^{\phi} \sqrt{1 - k^2 sin^2\theta}
   * @f]
   * For  @f$ \phi= \pi/2 @f$ this becomes the complete elliptic integral of
   * the second kind, @f$ E(k) @f$.  @see comp_ellint_2.
   *
   * @tparam _Tp The floating-point type of the modulus @c __k.
   * @tparam _Tpp The floating-point type of the angle @c __phi.
   * @param  __k  The modulus, <tt> abs(__k) <= 1 </tt>
   * @param  __phi  The integral limit argument in radians
   * @return  The elliptic function of the second kind.
   * @throw std::domain_error if <tt> abs(__k) > 1 </tt>.
   */
  template<typename _Tp, typename _Tpp>
    inline typename __gnu_cxx::__promote_2<_Tp, _Tpp>::__type
    ellint_2(_Tp __k, _Tpp __phi)
    {
      typedef typename __gnu_cxx::__promote_2<_Tp, _Tpp>::__type __type;
      return __detail::__ellint_2<__type>(__k, __phi);
    }

  // Incomplete elliptic integrals of the third kind

  /**
   * @brief Return the incomplete elliptic integral of the third kind
   * @f$ \Pi(k,\nu,\phi) @f$ for @c float argument.
   *
   * @see ellint_3 for details.
   */
  inline float
  ellint_3f(float __k, float __nu, float __phi)
  { return __detail::__ellint_3<float>(__k, __nu, __phi); }

  /**
   * @brief Return the incomplete elliptic integral of the third kind
   * @f$ \Pi(k,\nu,\phi) @f$.
   *
   * @see ellint_3 for details.
   */
  inline long double
  ellint_3l(long double __k, long double __nu, long double __phi)
  { return __detail::__ellint_3<long double>(__k, __nu, __phi); }

  /**
   * @brief Return the incomplete elliptic integral of the third kind
   * @f$ \Pi(k,\nu,\phi) @f$.
   *
   * The incomplete elliptic integral of the third kind is defined by:
   * @f[
   *   \Pi(k,\nu,\phi) = \int_0^{\phi}
   * 			 \frac{d\theta}
   * 			 {(1 - \nu \sin^2\theta)
   * 			  \sqrt{1 - k^2 \sin^2\theta}}
   * @f]
   * For  @f$ \phi= \pi/2 @f$ this becomes the complete elliptic integral of
   * the third kind, @f$ \Pi(k,\nu) @f$.  @see comp_ellint_3.
   *
   * @tparam _Tp The floating-point type of the modulus @c __k.
   * @tparam _Tpn The floating-point type of the argument @c __nu.
   * @tparam _Tpp The floating-point type of the angle @c __phi.
   * @param  __k  The modulus, <tt> abs(__k) <= 1 </tt>
   * @param  __nu  The second argument
   * @param  __phi  The integral limit argument in radians
   * @return  The elliptic function of the third kind.
   * @throw std::domain_error if <tt> abs(__k) > 1 </tt>.
   */
  template<typename _Tp, typename _Tpn, typename _Tpp>
    inline typename __gnu_cxx::__promote_3<_Tp, _Tpn, _Tpp>::__type
    ellint_3(_Tp __k, _Tpn __nu, _Tpp __phi)
    {
      typedef typename __gnu_cxx::__promote_3<_Tp, _Tpn, _Tpp>::__type __type;
      return __detail::__ellint_3<__type>(__k, __nu, __phi);
    }

  // Exponential integrals

  /**
   * Return the exponential integral @f$ Ei(x) @f$ for @c float argument @c x.
   *
   * @see expint for details.
   */
  inline float
  expintf(float __x)
  { return __detail::__expint<float>(__x); }

  /**
   * Return the exponential integral @f$ Ei(x) @f$
   * for <tt>long double</tt> argument @c x.
   *
   * @see expint for details.
   */
  inline long double
  expintl(long double __x)
  { return __detail::__expint<long double>(__x); }

  /**
   * Return the exponential integral @f$ Ei(x) @f$ for @c real argument @c x.
   *
   * The exponential integral is given by
   * \f[
   *   Ei(x) = -\int_{-x}^\infty \frac{e^t}{t} dt
   * \f]
   *
   * @tparam _Tp The floating-point type of the argument @c __x.
   * @param  __x  The argument of the exponential integral function.
   */
  template<typename _Tp>
    inline typename __gnu_cxx::__promote<_Tp>::__type
    expint(_Tp __x)
    {
      typedef typename __gnu_cxx::__promote<_Tp>::__type __type;
      return __detail::__expint<__type>(__x);
    }

  // Hermite polynomials

  /**
   * Return the Hermite polynomial @f$ H_n(x) @f$ of nonnegative order n
   * and float argument @c x.
   *
   * @see hermite for details.
   */
  inline float
  hermitef(unsigned int __n, float __x)
  { return __detail::__poly_hermite<float>(__n, __x); }

  /**
   * Return the Hermite polynomial @f$ H_n(x) @f$ of nonnegative order n
   * and <tt>long double</tt> argument @c x.
   *
   * @see hermite for details.
   */
  inline long double
  hermitel(unsigned int __n, long double __x)
  { return __detail::__poly_hermite<long double>(__n, __x); }

  /**
   * Return the Hermite polynomial @f$ H_n(x) @f$ of order n
   * and @c real argument @c x.
   *
   * The Hermite polynomial is defined by:
   * @f[
   *   H_n(x) = (-1)^n e^{x^2} \frac{d^n}{dx^n} e^{-x^2}
   * @f]
   *
   * The Hermite polynomial obeys a reflection formula:
   * @f[
   *   H_n(-x) = (-1)^n H_n(x)
   * @f]
   *
   * @tparam _Tp The floating-point type of the argument @c __x.
   * @param __n The order
   * @param __x The argument
   */
  template<typename _Tp>
    inline typename __gnu_cxx::__promote<_Tp>::__type
    hermite(unsigned int __n, _Tp __x)
    {
      typedef typename __gnu_cxx::__promote<_Tp>::__type __type;
      return __detail::__poly_hermite<__type>(__n, __x);
    }

  // Laguerre polynomials

  /**
   * Returns the Laguerre polynomial @f$ L_n(x) @f$ of nonnegative degree @c n
   * and @c float argument  @f$ x >= 0 @f$.
   *
   * @see laguerre for more details.
   */
  inline float
  laguerref(unsigned int __n, float __x)
  { return __detail::__laguerre<float>(__n, __x); }

  /**
   * Returns the Laguerre polynomial @f$ L_n(x) @f$ of nonnegative degree @c n
   * and <tt>long double</tt> argument @f$ x >= 0 @f$.
   *
   * @see laguerre for more details.
   */
  inline long double
  laguerrel(unsigned int __n, long double __x)
  { return __detail::__laguerre<long double>(__n, __x); }

  /**
   * Returns the Laguerre polynomial @f$ L_n(x) @f$
   * of nonnegative degree @c n and real argument @f$ x >= 0 @f$.
   *
   * The Laguerre polynomial is defined by:
   * @f[
   * 	 L_n(x) = \frac{e^x}{n!} \frac{d^n}{dx^n} (x^ne^{-x})
   * @f]
   *
   * @tparam _Tp The floating-point type of the argument @c __x.
   * @param __n The nonnegative order
   * @param __x The argument <tt> __x >= 0 </tt>
   * @throw std::domain_error if <tt> __x < 0 </tt>.
   */
  template<typename _Tp>
    inline typename __gnu_cxx::__promote<_Tp>::__type
    laguerre(unsigned int __n, _Tp __x)
    {
      typedef typename __gnu_cxx::__promote<_Tp>::__type __type;
      return __detail::__laguerre<__type>(__n, __x);
    }

  // Legendre polynomials

  /**
   * Return the Legendre polynomial @f$ P_l(x) @f$ of nonnegative
   * degree @f$ l @f$ and @c float argument @f$ |x| <= 0 @f$.
   *
   * @see legendre for more details.
   */
  inline float
  legendref(unsigned int __l, float __x)
  { return __detail::__poly_legendre_p<float>(__l, __x); }

  /**
   * Return the Legendre polynomial @f$ P_l(x) @f$ of nonnegative
   * degree @f$ l @f$ and <tt>long double</tt> argument @f$ |x| <= 0 @f$.
   *
   * @see legendre for more details.
   */
  inline long double
  legendrel(unsigned int __l, long double __x)
  { return __detail::__poly_legendre_p<long double>(__l, __x); }

  /**
   * Return the Legendre polynomial @f$ P_l(x) @f$ of nonnegative
   * degree @f$ l @f$ and real argument @f$ |x| <= 0 @f$.
   *
   * The Legendre function of order @f$ l @f$ and argument @f$ x @f$,
   * @f$ P_l(x) @f$, is defined by:
   * @f[
   *   P_l(x) = \frac{1}{2^l l!}\frac{d^l}{dx^l}(x^2 - 1)^{l}
   * @f]
   *
   * @tparam _Tp The floating-point type of the argument @c __x.
   * @param __l The degree @f$ l >= 0 @f$
   * @param __x The argument @c abs(__x) <= 1
   * @throw std::domain_error if @c abs(__x) > 1
   */
  template<typename _Tp>
    inline typename __gnu_cxx::__promote<_Tp>::__type
    legendre(unsigned int __l, _Tp __x)
    {
      typedef typename __gnu_cxx::__promote<_Tp>::__type __type;
      return __detail::__poly_legendre_p<__type>(__l, __x);
    }

  // Riemann zeta functions

  /**
   * Return the Riemann zeta function @f$ \zeta(s) @f$
   * for @c float argument @f$ s @f$.
   *
   * @see riemann_zeta for more details.
   */
  inline float
  riemann_zetaf(float __s)
  { return __detail::__riemann_zeta<float>(__s); }

  /**
   * Return the Riemann zeta function @f$ \zeta(s) @f$
   * for <tt>long double</tt> argument @f$ s @f$.
   *
   * @see riemann_zeta for more details.
   */
  inline long double
  riemann_zetal(long double __s)
  { return __detail::__riemann_zeta<long double>(__s); }

  /**
   * Return the Riemann zeta function @f$ \zeta(s) @f$
   * for real argument @f$ s @f$.
   *
   * The Riemann zeta function is defined by:
   * @f[
   * 	\zeta(s) = \sum_{k=1}^{\infty} k^{-s} \hbox{ for } s > 1
   * @f]
   * and
   * @f[
   * 	\zeta(s) = \frac{1}{1-2^{1-s}}\sum_{k=1}^{\infty}(-1)^{k-1}k^{-s}
   *              \hbox{ for } 0 <= s <= 1
   * @f]
   * For s < 1 use the reflection formula:
   * @f[
   * 	\zeta(s) = 2^s \pi^{s-1} \sin(\frac{\pi s}{2}) \Gamma(1-s) \zeta(1-s)
   * @f]
   *
   * @tparam _Tp The floating-point type of the argument @c __s.
   * @param __s The argument <tt> s != 1 </tt>
   */
  template<typename _Tp>
    inline typename __gnu_cxx::__promote<_Tp>::__type
    riemann_zeta(_Tp __s)
    {
      typedef typename __gnu_cxx::__promote<_Tp>::__type __type;
      return __detail::__riemann_zeta<__type>(__s);
    }

  // Spherical Bessel functions

  /**
   * Return the spherical Bessel function @f$ j_n(x) @f$ of nonnegative order n
   * and @c float argument @f$ x >= 0 @f$.
   *
   * @see sph_bessel for more details.
   */
  inline float
  sph_besself(unsigned int __n, float __x)
  { return __detail::__sph_bessel<float>(__n, __x); }

  /**
   * Return the spherical Bessel function @f$ j_n(x) @f$ of nonnegative order n
   * and <tt>long double</tt> argument @f$ x >= 0 @f$.
   *
   * @see sph_bessel for more details.
   */
  inline long double
  sph_bessell(unsigned int __n, long double __x)
  { return __detail::__sph_bessel<long double>(__n, __x); }

  /**
   * Return the spherical Bessel function @f$ j_n(x) @f$ of nonnegative order n
   * and real argument @f$ x >= 0 @f$.
   *
   * The spherical Bessel function is defined by:
   * @f[
   *  j_n(x) = \left(\frac{\pi}{2x} \right) ^{1/2} J_{n+1/2}(x)
   * @f]
   *
   * @tparam _Tp The floating-point type of the argument @c __x.
   * @param  __n  The integral order <tt> n >= 0 </tt>
   * @param  __x  The real argument <tt> x >= 0 </tt>
   * @throw std::domain_error if <tt> __x < 0 </tt>.
   */
  template<typename _Tp>
    inline typename __gnu_cxx::__promote<_Tp>::__type
    sph_bessel(unsigned int __n, _Tp __x)
    {
      typedef typename __gnu_cxx::__promote<_Tp>::__type __type;
      return __detail::__sph_bessel<__type>(__n, __x);
    }

  // Spherical associated Legendre functions

  /**
   * Return the spherical Legendre function of nonnegative integral
   * degree @c l and order @c m and float angle @f$ \theta @f$ in radians.
   *
   * @see sph_legendre for details.
   */
  inline float
  sph_legendref(unsigned int __l, unsigned int __m, float __theta)
  { return __detail::__sph_legendre<float>(__l, __m, __theta); }

  /**
   * Return the spherical Legendre function of nonnegative integral
   * degree @c l and order @c m and <tt>long double</tt> angle @f$ \theta @f$
   * in radians.
   *
   * @see sph_legendre for details.
   */
  inline long double
  sph_legendrel(unsigned int __l, unsigned int __m, long double __theta)
  { return __detail::__sph_legendre<long double>(__l, __m, __theta); }

  /**
   * Return the spherical Legendre function of nonnegative integral
   * degree @c l and order @c m and real angle @f$ \theta @f$ in radians.
   *
   * The spherical Legendre function is defined by
   * @f[
   *  Y_l^m(\theta,\phi) = (-1)^m[\frac{(2l+1)}{4\pi}
   *                              \frac{(l-m)!}{(l+m)!}]
   *                   P_l^m(\cos\theta) \exp^{im\phi}
   * @f]
   *
   * @tparam _Tp The floating-point type of the angle @c __theta.
   * @param __l The order <tt> __l >= 0 </tt>
   * @param __m The degree <tt> __m >= 0 </tt> and <tt> __m <= __l </tt>
   * @param __theta The radian polar angle argument
   */
  template<typename _Tp>
    inline typename __gnu_cxx::__promote<_Tp>::__type
    sph_legendre(unsigned int __l, unsigned int __m, _Tp __theta)
    {
      typedef typename __gnu_cxx::__promote<_Tp>::__type __type;
      return __detail::__sph_legendre<__type>(__l, __m, __theta);
    }

  // Spherical Neumann functions

  /**
   * Return the spherical Neumann function of integral order @f$ n >= 0 @f$
   * and @c float argument @f$ x >= 0 @f$.
   *
   * @see sph_neumann for details.
   */
  inline float
  sph_neumannf(unsigned int __n, float __x)
  { return __detail::__sph_neumann<float>(__n, __x); }

  /**
   * Return the spherical Neumann function of integral order @f$ n >= 0 @f$
   * and <tt>long double</tt> @f$ x >= 0 @f$.
   *
   * @see sph_neumann for details.
   */
  inline long double
  sph_neumannl(unsigned int __n, long double __x)
  { return __detail::__sph_neumann<long double>(__n, __x); }

  /**
   * Return the spherical Neumann function of integral order @f$ n >= 0 @f$
   * and real argument @f$ x >= 0 @f$.
   *
   * The spherical Neumann function is defined by
   * @f[
   *    n_n(x) = \left(\frac{\pi}{2x} \right) ^{1/2} N_{n+1/2}(x)
   * @f]
   *
   * @tparam _Tp The floating-point type of the argument @c __x.
   * @param  __n  The integral order <tt> n >= 0 </tt>
   * @param  __x  The real argument <tt> __x >= 0 </tt>
   * @throw std::domain_error if <tt> __x < 0 </tt>.
   */
  template<typename _Tp>
    inline typename __gnu_cxx::__promote<_Tp>::__type
    sph_neumann(unsigned int __n, _Tp __x)
    {
      typedef typename __gnu_cxx::__promote<_Tp>::__type __type;
      return __detail::__sph_neumann<__type>(__n, __x);
    }

  // @} group mathsf

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace std

#ifndef __STRICT_ANSI__
namespace __gnu_cxx _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  /** @addtogroup mathsf
   *  @{
   */

  // Airy functions

  /**
   * Return the Airy function @f$ Ai(x) @f$ of @c float argument x.
   */
  inline float
  airy_aif(float __x)
  {
    float __Ai, __Bi, __Aip, __Bip;
    std::__detail::__airy<float>(__x, __Ai, __Bi, __Aip, __Bip);
    return __Ai;
  }

  /**
   * Return the Airy function @f$ Ai(x) @f$ of <tt>long double</tt> argument x.
   */
  inline long double
  airy_ail(long double __x)
  {
    long double __Ai, __Bi, __Aip, __Bip;
    std::__detail::__airy<long double>(__x, __Ai, __Bi, __Aip, __Bip);
    return __Ai;
  }

  /**
   * Return the Airy function @f$ Ai(x) @f$ of real argument x.
   */
  template<typename _Tp>
    inline typename __gnu_cxx::__promote<_Tp>::__type
    airy_ai(_Tp __x)
    {
      typedef typename __gnu_cxx::__promote<_Tp>::__type __type;
      __type __Ai, __Bi, __Aip, __Bip;
      std::__detail::__airy<__type>(__x, __Ai, __Bi, __Aip, __Bip);
      return __Ai;
    }

  /**
   * Return the Airy function @f$ Bi(x) @f$ of @c float argument x.
   */
  inline float
  airy_bif(float __x)
  {
    float __Ai, __Bi, __Aip, __Bip;
    std::__detail::__airy<float>(__x, __Ai, __Bi, __Aip, __Bip);
    return __Bi;
  }

  /**
   * Return the Airy function @f$ Bi(x) @f$ of <tt>long double</tt> argument x.
   */
  inline long double
  airy_bil(long double __x)
  {
    long double __Ai, __Bi, __Aip, __Bip;
    std::__detail::__airy<long double>(__x, __Ai, __Bi, __Aip, __Bip);
    return __Bi;
  }

  /**
   * Return the Airy function @f$ Bi(x) @f$ of real argument x.
   */
  template<typename _Tp>
    inline typename __gnu_cxx::__promote<_Tp>::__type
    airy_bi(_Tp __x)
    {
      typedef typename __gnu_cxx::__promote<_Tp>::__type __type;
      __type __Ai, __Bi, __Aip, __Bip;
      std::__detail::__airy<__type>(__x, __Ai, __Bi, __Aip, __Bip);
      return __Bi;
    }

  // Confluent hypergeometric functions

  /**
   * Return the confluent hypergeometric function @f$ {}_1F_1(a;c;x) @f$
   * of @c float numeratorial parameter @c a, denominatorial parameter @c c,
   * and argument @c x.
   *
   * @see conf_hyperg for details.
   */
  inline float
  conf_hypergf(float __a, float __c, float __x)
  { return std::__detail::__conf_hyperg<float>(__a, __c, __x); }

  /**
   * Return the confluent hypergeometric function @f$ {}_1F_1(a;c;x) @f$
   * of <tt>long double</tt> numeratorial parameter @c a,
   * denominatorial parameter @c c, and argument @c x.
   *
   * @see conf_hyperg for details.
   */
  inline long double
  conf_hypergl(long double __a, long double __c, long double __x)
  { return std::__detail::__conf_hyperg<long double>(__a, __c, __x); }

  /**
   * Return the confluent hypergeometric function @f$ {}_1F_1(a;c;x) @f$
   * of real numeratorial parameter @c a, denominatorial parameter @c c,
   * and argument @c x.
   *
   * The confluent hypergeometric function is defined by
   * @f[
   *    {}_1F_1(a;c;x) = \sum_{n=0}^{\infty} \frac{(a)_n x^n}{(c)_n n!}
   * @f]
   * where the Pochhammer symbol is @f$ (x)_k = (x)(x+1)...(x+k-1) @f$,
   * @f$ (x)_0 = 1 @f$
   *
   * @param __a The numeratorial parameter
   * @param __c The denominatorial parameter
   * @param __x The argument
   */
  template<typename _Tpa, typename _Tpc, typename _Tp>
    inline typename __gnu_cxx::__promote_3<_Tpa, _Tpc, _Tp>::__type
    conf_hyperg(_Tpa __a, _Tpc __c, _Tp __x)
    {
      typedef typename __gnu_cxx::__promote_3<_Tpa, _Tpc, _Tp>::__type __type;
      return std::__detail::__conf_hyperg<__type>(__a, __c, __x);
    }

  // Hypergeometric functions

  /**
   * Return the hypergeometric function @f$ {}_2F_1(a,b;c;x) @f$
   * of @ float numeratorial parameters @c a and @c b,
   * denominatorial parameter @c c, and argument @c x.
   *
   * @see hyperg for details.
   */
  inline float
  hypergf(float __a, float __b, float __c, float __x)
  { return std::__detail::__hyperg<float>(__a, __b, __c, __x); }

  /**
   * Return the hypergeometric function @f$ {}_2F_1(a,b;c;x) @f$
   * of <tt>long double</tt> numeratorial parameters @c a and @c b,
   * denominatorial parameter @c c, and argument @c x.
   *
   * @see hyperg for details.
   */
  inline long double
  hypergl(long double __a, long double __b, long double __c, long double __x)
  { return std::__detail::__hyperg<long double>(__a, __b, __c, __x); }

  /**
   * Return the hypergeometric function @f$ {}_2F_1(a,b;c;x) @f$
   * of real numeratorial parameters @c a and @c b,
   * denominatorial parameter @c c, and argument @c x.
   *
   * The hypergeometric function is defined by
   * @f[
   *    {}_2F_1(a;c;x) = \sum_{n=0}^{\infty} \frac{(a)_n (b)_n x^n}{(c)_n n!}
   * @f]
   * where the Pochhammer symbol is @f$ (x)_k = (x)(x+1)...(x+k-1) @f$,
   * @f$ (x)_0 = 1 @f$
   *
   * @param __a The first numeratorial parameter
   * @param __b The second numeratorial parameter
   * @param __c The denominatorial parameter
   * @param __x The argument
   */
  template<typename _Tpa, typename _Tpb, typename _Tpc, typename _Tp>
    inline typename __gnu_cxx::__promote_4<_Tpa, _Tpb, _Tpc, _Tp>::__type
    hyperg(_Tpa __a, _Tpb __b, _Tpc __c, _Tp __x)
    {
      typedef typename __gnu_cxx::__promote_4<_Tpa, _Tpb, _Tpc, _Tp>
		::__type __type;
      return std::__detail::__hyperg<__type>(__a, __b, __c, __x);
    }

  // @}
_GLIBCXX_END_NAMESPACE_VERSION
} // namespace __gnu_cxx
#endif // __STRICT_ANSI__

#pragma GCC visibility pop

#endif // _GLIBCXX_BITS_SPECFUN_H
