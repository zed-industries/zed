// The template and inlines for the -*- C++ -*- internal _Meta class.

// Copyright (C) 1997-2020 Free Software Foundation, Inc.
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

/** @file bits/valarray_after.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{valarray}
 */

// Written by Gabriel Dos Reis <Gabriel.Dos-Reis@cmla.ens-cachan.fr>

#ifndef _VALARRAY_AFTER_H
#define _VALARRAY_AFTER_H 1

#pragma GCC system_header

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

namespace __detail
{
  //
  // gslice_array closure.
  //
  template<class _Dom>
    class _GBase
    {
    public:
      typedef typename _Dom::value_type value_type;
      
      _GBase (const _Dom& __e, const valarray<size_t>& __i)
      : _M_expr (__e), _M_index(__i) {}
      
      value_type
      operator[] (size_t __i) const
      { return _M_expr[_M_index[__i]]; }
      
      size_t
      size () const
      { return _M_index.size(); }

    private:
      typename _ValArrayRef<_Dom>::__type	_M_expr;
      const valarray<size_t>&			_M_index;
    };

  template<typename _Tp>
    class _GBase<_Array<_Tp> >
    {
    public:
      typedef _Tp value_type;
      
      _GBase (_Array<_Tp> __a, const valarray<size_t>& __i)
      : _M_array (__a), _M_index(__i) {}
      
      value_type
      operator[] (size_t __i) const
      { return _M_array._M_data[_M_index[__i]]; }
      
      size_t
      size () const
      { return _M_index.size(); }

    private:
      const _Array<_Tp>       _M_array;
      const valarray<size_t>& _M_index;
    };

  template<class _Dom>
    struct _GClos<_Expr, _Dom>
    : _GBase<_Dom>
    {
      typedef _GBase<_Dom> _Base;
      typedef typename _Base::value_type value_type;
      
      _GClos (const _Dom& __e, const valarray<size_t>& __i)
      : _Base (__e, __i) {}
    };

  template<typename _Tp>
    struct _GClos<_ValArray, _Tp>
    : _GBase<_Array<_Tp> >
    {
      typedef _GBase<_Array<_Tp> > _Base;
      typedef typename _Base::value_type value_type;
      
      _GClos (_Array<_Tp> __a, const valarray<size_t>& __i)
      : _Base (__a, __i) {}
    };

  //
  // indirect_array closure
  //
  template<class _Dom>
    class _IBase
    {
    public:
      typedef typename _Dom::value_type value_type;

      _IBase (const _Dom& __e, const valarray<size_t>& __i)
      : _M_expr (__e), _M_index (__i) {}
      
      value_type
      operator[] (size_t __i) const
      { return _M_expr[_M_index[__i]]; }
      
      size_t
      size() const
      { return _M_index.size(); }

    private:
      typename _ValArrayRef<_Dom>::__type	_M_expr;
      const valarray<size_t>&			_M_index;
    };

  template<class _Dom>
    struct _IClos<_Expr, _Dom>
    : _IBase<_Dom>
    {
      typedef _IBase<_Dom> _Base;
      typedef typename _Base::value_type value_type;
      
      _IClos (const _Dom& __e, const valarray<size_t>& __i)
      : _Base (__e, __i) {}
    };

  template<typename _Tp>
    struct _IClos<_ValArray, _Tp>
    : _IBase<valarray<_Tp> >
    {
      typedef _IBase<valarray<_Tp> > _Base;
      typedef _Tp value_type;
      
      _IClos (const valarray<_Tp>& __a, const valarray<size_t>& __i)
      : _Base (__a, __i) {}
    };
} // namespace __detail
  
  //
  // class _Expr
  //
  template<class _Clos, typename _Tp>
    class _Expr
    {
    public:
      typedef _Tp value_type;

      _Expr(const _Clos&);

      const _Clos& operator()() const;

      value_type operator[](size_t) const;
      valarray<value_type> operator[](slice) const;
      valarray<value_type> operator[](const gslice&) const;
      valarray<value_type> operator[](const valarray<bool>&) const;
      valarray<value_type> operator[](const valarray<size_t>&) const;

      _Expr<_UnClos<__unary_plus, std::_Expr, _Clos>, value_type>
      operator+() const;

      _Expr<_UnClos<__negate, std::_Expr, _Clos>, value_type>
      operator-() const;

      _Expr<_UnClos<__bitwise_not, std::_Expr, _Clos>, value_type>
      operator~() const;

      _Expr<_UnClos<__logical_not, std::_Expr, _Clos>, bool>
      operator!() const;

      size_t size() const;
      value_type sum() const;

      valarray<value_type> shift(int) const;
      valarray<value_type> cshift(int) const;

      value_type min() const;
      value_type max() const;

      valarray<value_type> apply(value_type (*)(const value_type&)) const;
      valarray<value_type> apply(value_type (*)(value_type)) const;

    private:
      const _Clos _M_closure;
    };

  template<class _Clos, typename _Tp>
    inline
    _Expr<_Clos, _Tp>::_Expr(const _Clos& __c) : _M_closure(__c) {}

  template<class _Clos, typename _Tp>
    inline const _Clos&
    _Expr<_Clos, _Tp>::operator()() const
    { return _M_closure; }

  template<class _Clos, typename _Tp>
    inline _Tp
    _Expr<_Clos, _Tp>::operator[](size_t __i) const
    { return _M_closure[__i]; }

  template<class _Clos, typename _Tp>
    inline valarray<_Tp>
    _Expr<_Clos, _Tp>::operator[](slice __s) const
    {
      valarray<_Tp> __v = valarray<_Tp>(*this)[__s];
      return __v;
    }

  template<class _Clos, typename _Tp>
    inline valarray<_Tp>
    _Expr<_Clos, _Tp>::operator[](const gslice& __gs) const
    {
      valarray<_Tp> __v = valarray<_Tp>(*this)[__gs];
      return __v;
    }

  template<class _Clos, typename _Tp>
    inline valarray<_Tp>
    _Expr<_Clos, _Tp>::operator[](const valarray<bool>& __m) const
    {
      valarray<_Tp> __v = valarray<_Tp>(*this)[__m];
      return __v;
    }

  template<class _Clos, typename _Tp>
    inline valarray<_Tp>
    _Expr<_Clos, _Tp>::operator[](const valarray<size_t>& __i) const
    {
      valarray<_Tp> __v = valarray<_Tp>(*this)[__i];
      return __v;
    }

  template<class _Clos, typename _Tp>
    inline size_t
    _Expr<_Clos, _Tp>::size() const
    { return _M_closure.size(); }

  template<class _Clos, typename _Tp>
    inline valarray<_Tp>
    _Expr<_Clos, _Tp>::shift(int __n) const
    {
      valarray<_Tp> __v = valarray<_Tp>(*this).shift(__n);
      return __v;
    }

  template<class _Clos, typename _Tp>
    inline valarray<_Tp>
    _Expr<_Clos, _Tp>::cshift(int __n) const
    {
      valarray<_Tp> __v = valarray<_Tp>(*this).cshift(__n);
      return __v;
    }

  template<class _Clos, typename _Tp>
    inline valarray<_Tp>
    _Expr<_Clos, _Tp>::apply(_Tp __f(const _Tp&)) const
    {
      valarray<_Tp> __v = valarray<_Tp>(*this).apply(__f);
      return __v;
    }

  template<class _Clos, typename _Tp>
    inline valarray<_Tp>
    _Expr<_Clos, _Tp>::apply(_Tp __f(_Tp)) const
    {
      valarray<_Tp> __v = valarray<_Tp>(*this).apply(__f);
      return __v;
    }

  // XXX: replace this with a more robust summation algorithm.
  template<class _Clos, typename _Tp>
    inline _Tp
    _Expr<_Clos, _Tp>::sum() const
    {
      size_t __n = _M_closure.size();
      if (__n == 0)
	return _Tp();
      else
	{
	  _Tp __s = _M_closure[--__n];
	  while (__n != 0)
	    __s += _M_closure[--__n];
	  return __s;
        }
    }

  template<class _Clos, typename _Tp>
    inline _Tp
    _Expr<_Clos, _Tp>::min() const
    { return __valarray_min(_M_closure); }

  template<class _Clos, typename _Tp>
    inline _Tp
    _Expr<_Clos, _Tp>::max() const
    { return __valarray_max(_M_closure); }

  template<class _Dom, typename _Tp>
    inline _Expr<_UnClos<__logical_not, _Expr, _Dom>, bool>
    _Expr<_Dom, _Tp>::operator!() const
    {
      typedef _UnClos<__logical_not, std::_Expr, _Dom> _Closure;
      return _Expr<_Closure, bool>(_Closure(this->_M_closure));
    }

#define _DEFINE_EXPR_UNARY_OPERATOR(_Op, _Name)                           \
  template<class _Dom, typename _Tp>                                      \
    inline _Expr<_UnClos<_Name, std::_Expr, _Dom>, _Tp>                   \
    _Expr<_Dom, _Tp>::operator _Op() const                                \
    {                                                                     \
      typedef _UnClos<_Name, std::_Expr, _Dom> _Closure;                  \
      return _Expr<_Closure, _Tp>(_Closure(this->_M_closure));            \
    }

    _DEFINE_EXPR_UNARY_OPERATOR(+, __unary_plus)
    _DEFINE_EXPR_UNARY_OPERATOR(-, __negate)
    _DEFINE_EXPR_UNARY_OPERATOR(~, __bitwise_not)

#undef _DEFINE_EXPR_UNARY_OPERATOR

#define _DEFINE_EXPR_BINARY_OPERATOR(_Op, _Name)                        \
  template<class _Dom1, class _Dom2>					\
    inline _Expr<_BinClos<_Name, _Expr, _Expr, _Dom1, _Dom2>,           \
           typename __fun<_Name, typename _Dom1::value_type>::result_type> \
    operator _Op(const _Expr<_Dom1, typename _Dom1::value_type>& __v,   \
	         const _Expr<_Dom2, typename _Dom2::value_type>& __w)   \
    {                                                                   \
      typedef typename _Dom1::value_type _Arg;                          \
      typedef typename __fun<_Name, _Arg>::result_type _Value;          \
      typedef _BinClos<_Name, _Expr, _Expr, _Dom1, _Dom2> _Closure;     \
      return _Expr<_Closure, _Value>(_Closure(__v(), __w()));           \
    }                                                                   \
                                                                        \
  template<class _Dom>                                                  \
    inline _Expr<_BinClos<_Name, _Expr, _Constant, _Dom,                \
                          typename _Dom::value_type>,                   \
             typename __fun<_Name, typename _Dom::value_type>::result_type> \
    operator _Op(const _Expr<_Dom, typename _Dom::value_type>& __v,     \
                 const typename _Dom::value_type& __t)                  \
    {                                                                   \
      typedef typename _Dom::value_type _Arg;                           \
      typedef typename __fun<_Name, _Arg>::result_type _Value;          \
      typedef _BinClos<_Name, _Expr, _Constant, _Dom, _Arg> _Closure;   \
      return _Expr<_Closure, _Value>(_Closure(__v(), __t));             \
    }                                                                   \
                                                                        \
  template<class _Dom>                                                  \
    inline _Expr<_BinClos<_Name, _Constant, _Expr,                      \
                          typename _Dom::value_type, _Dom>,             \
             typename __fun<_Name, typename _Dom::value_type>::result_type> \
    operator _Op(const typename _Dom::value_type& __t,                  \
                 const _Expr<_Dom, typename _Dom::value_type>& __v)     \
    {                                                                   \
      typedef typename _Dom::value_type _Arg;                           \
      typedef typename __fun<_Name, _Arg>::result_type _Value;          \
      typedef _BinClos<_Name, _Constant, _Expr, _Arg, _Dom> _Closure;   \
      return _Expr<_Closure, _Value>(_Closure(__t, __v()));             \
    }                                                                   \
                                                                        \
  template<class _Dom>                                                  \
    inline _Expr<_BinClos<_Name, _Expr, _ValArray,                      \
                          _Dom, typename _Dom::value_type>,             \
             typename __fun<_Name, typename _Dom::value_type>::result_type> \
    operator _Op(const _Expr<_Dom,typename _Dom::value_type>& __e,      \
                 const valarray<typename _Dom::value_type>& __v)        \
    {                                                                   \
      typedef typename _Dom::value_type _Arg;                           \
      typedef typename __fun<_Name, _Arg>::result_type _Value;          \
      typedef _BinClos<_Name, _Expr, _ValArray, _Dom, _Arg> _Closure;   \
      return _Expr<_Closure, _Value>(_Closure(__e(), __v));             \
    }                                                                   \
                                                                        \
  template<class _Dom>                                                  \
    inline _Expr<_BinClos<_Name, _ValArray, _Expr,                      \
                 typename _Dom::value_type, _Dom>,                      \
             typename __fun<_Name, typename _Dom::value_type>::result_type> \
    operator _Op(const valarray<typename _Dom::value_type>& __v,        \
                 const _Expr<_Dom, typename _Dom::value_type>& __e)     \
    {                                                                   \
      typedef typename _Dom::value_type _Tp;                            \
      typedef typename __fun<_Name, _Tp>::result_type _Value;           \
      typedef _BinClos<_Name, _ValArray, _Expr, _Tp, _Dom> _Closure;    \
      return _Expr<_Closure, _Value>(_Closure(__v, __e ()));            \
    }

    _DEFINE_EXPR_BINARY_OPERATOR(+, __plus)
    _DEFINE_EXPR_BINARY_OPERATOR(-, __minus)
    _DEFINE_EXPR_BINARY_OPERATOR(*, __multiplies)
    _DEFINE_EXPR_BINARY_OPERATOR(/, __divides)
    _DEFINE_EXPR_BINARY_OPERATOR(%, __modulus)
    _DEFINE_EXPR_BINARY_OPERATOR(^, __bitwise_xor)
    _DEFINE_EXPR_BINARY_OPERATOR(&, __bitwise_and)
    _DEFINE_EXPR_BINARY_OPERATOR(|, __bitwise_or)
    _DEFINE_EXPR_BINARY_OPERATOR(<<, __shift_left)
    _DEFINE_EXPR_BINARY_OPERATOR(>>, __shift_right)
    _DEFINE_EXPR_BINARY_OPERATOR(&&, __logical_and)
    _DEFINE_EXPR_BINARY_OPERATOR(||, __logical_or)
    _DEFINE_EXPR_BINARY_OPERATOR(==, __equal_to)
    _DEFINE_EXPR_BINARY_OPERATOR(!=, __not_equal_to)
    _DEFINE_EXPR_BINARY_OPERATOR(<, __less)
    _DEFINE_EXPR_BINARY_OPERATOR(>, __greater)
    _DEFINE_EXPR_BINARY_OPERATOR(<=, __less_equal)
    _DEFINE_EXPR_BINARY_OPERATOR(>=, __greater_equal)

#undef _DEFINE_EXPR_BINARY_OPERATOR

#define _DEFINE_EXPR_UNARY_FUNCTION(_Name, _UName)                       \
  template<class _Dom>                                                   \
    inline _Expr<_UnClos<_UName, _Expr, _Dom>,                           \
                 typename _Dom::value_type>                              \
    _Name(const _Expr<_Dom, typename _Dom::value_type>& __e)             \
    {                                                                    \
      typedef typename _Dom::value_type _Tp;                             \
      typedef _UnClos<_UName, _Expr, _Dom> _Closure;                     \
      return _Expr<_Closure, _Tp>(_Closure(__e()));                      \
    }                                                                    \
                                                                         \
  template<typename _Tp>                                                 \
    inline _Expr<_UnClos<_UName, _ValArray, _Tp>, _Tp>                   \
    _Name(const valarray<_Tp>& __v)                                      \
    {                                                                    \
      typedef _UnClos<_UName, _ValArray, _Tp> _Closure;                  \
      return _Expr<_Closure, _Tp>(_Closure(__v));                        \
    }

    _DEFINE_EXPR_UNARY_FUNCTION(abs, _Abs)
    _DEFINE_EXPR_UNARY_FUNCTION(cos, _Cos)
    _DEFINE_EXPR_UNARY_FUNCTION(acos, _Acos)
    _DEFINE_EXPR_UNARY_FUNCTION(cosh, _Cosh)
    _DEFINE_EXPR_UNARY_FUNCTION(sin, _Sin)
    _DEFINE_EXPR_UNARY_FUNCTION(asin, _Asin)
    _DEFINE_EXPR_UNARY_FUNCTION(sinh, _Sinh)
    _DEFINE_EXPR_UNARY_FUNCTION(tan, _Tan)
    _DEFINE_EXPR_UNARY_FUNCTION(tanh, _Tanh)
    _DEFINE_EXPR_UNARY_FUNCTION(atan, _Atan)
    _DEFINE_EXPR_UNARY_FUNCTION(exp, _Exp)
    _DEFINE_EXPR_UNARY_FUNCTION(log, _Log)
    _DEFINE_EXPR_UNARY_FUNCTION(log10, _Log10)
    _DEFINE_EXPR_UNARY_FUNCTION(sqrt, _Sqrt)

#undef _DEFINE_EXPR_UNARY_FUNCTION

#define _DEFINE_EXPR_BINARY_FUNCTION(_Fun, _UFun)		       \
  template<class _Dom1, class _Dom2>                                   \
    inline _Expr<_BinClos<_UFun, _Expr, _Expr, _Dom1, _Dom2>,          \
		 typename _Dom1::value_type>                           \
    _Fun(const _Expr<_Dom1, typename _Dom1::value_type>& __e1,         \
	 const _Expr<_Dom2, typename _Dom2::value_type>& __e2)	       \
    {                                                                  \
      typedef typename _Dom1::value_type _Tp;                          \
      typedef _BinClos<_UFun, _Expr, _Expr, _Dom1, _Dom2> _Closure;    \
      return _Expr<_Closure, _Tp>(_Closure(__e1(), __e2()));           \
    }                                                                  \
                                                                       \
  template<class _Dom>                                                 \
    inline _Expr<_BinClos<_UFun, _Expr, _ValArray, _Dom,               \
			  typename _Dom::value_type>,                  \
		 typename _Dom::value_type>                            \
    _Fun(const _Expr<_Dom, typename _Dom::value_type>& __e,            \
	 const valarray<typename _Dom::value_type>& __v)               \
    {                                                                  \
      typedef typename _Dom::value_type _Tp;                           \
      typedef _BinClos<_UFun, _Expr, _ValArray, _Dom, _Tp> _Closure;   \
      return _Expr<_Closure, _Tp>(_Closure(__e(), __v));               \
    }                                                                  \
                                                                       \
  template<class _Dom>                                                 \
    inline _Expr<_BinClos<_UFun, _ValArray, _Expr,                     \
			  typename _Dom::value_type, _Dom>,            \
		 typename _Dom::value_type>                            \
    _Fun(const valarray<typename _Dom::valarray>& __v,                 \
	 const _Expr<_Dom, typename _Dom::value_type>& __e)            \
    {                                                                  \
      typedef typename _Dom::value_type _Tp;                           \
      typedef _BinClos<_UFun, _ValArray, _Expr, _Tp, _Dom> _Closure;   \
      return _Expr<_Closure, _Tp>(_Closure(__v, __e()));               \
    }                                                                  \
                                                                       \
  template<class _Dom>                                                 \
    inline _Expr<_BinClos<_UFun, _Expr, _Constant, _Dom,               \
			  typename _Dom::value_type>,                  \
		 typename _Dom::value_type>                            \
    _Fun(const _Expr<_Dom, typename _Dom::value_type>& __e,            \
	 const typename _Dom::value_type& __t)                         \
    {                                                                  \
      typedef typename _Dom::value_type _Tp;                           \
      typedef _BinClos<_UFun, _Expr, _Constant, _Dom, _Tp> _Closure;   \
      return _Expr<_Closure, _Tp>(_Closure(__e(), __t));               \
    }                                                                  \
                                                                       \
  template<class _Dom>                                                 \
    inline _Expr<_BinClos<_UFun, _Constant, _Expr,                     \
			  typename _Dom::value_type, _Dom>,            \
		 typename _Dom::value_type>                            \
    _Fun(const typename _Dom::value_type& __t,                         \
	 const _Expr<_Dom, typename _Dom::value_type>& __e)            \
    {                                                                  \
      typedef typename _Dom::value_type _Tp;                           \
      typedef _BinClos<_UFun, _Constant, _Expr, _Tp, _Dom> _Closure;   \
      return _Expr<_Closure, _Tp>(_Closure(__t, __e()));               \
    }                                                                  \
                                                                       \
  template<typename _Tp>                                               \
    inline _Expr<_BinClos<_UFun, _ValArray, _ValArray, _Tp, _Tp>, _Tp> \
    _Fun(const valarray<_Tp>& __v, const valarray<_Tp>& __w)           \
    {                                                                  \
      typedef _BinClos<_UFun, _ValArray, _ValArray, _Tp, _Tp> _Closure;\
      return _Expr<_Closure, _Tp>(_Closure(__v, __w));                 \
    }                                                                  \
                                                                       \
  template<typename _Tp>                                               \
    inline _Expr<_BinClos<_UFun, _ValArray, _Constant, _Tp, _Tp>, _Tp> \
    _Fun(const valarray<_Tp>& __v,				       \
	 const typename valarray<_Tp>::value_type& __t)                \
    {                                                                  \
      typedef _BinClos<_UFun, _ValArray, _Constant, _Tp, _Tp> _Closure;\
      return _Expr<_Closure, _Tp>(_Closure(__v, __t));                 \
    }                                                                  \
								       \
  template<typename _Tp>                                               \
    inline _Expr<_BinClos<_UFun, _Constant, _ValArray, _Tp, _Tp>, _Tp> \
    _Fun(const typename valarray<_Tp>::value_type& __t,		       \
	 const valarray<_Tp>& __v)                                     \
    {                                                                  \
      typedef _BinClos<_UFun, _Constant, _ValArray, _Tp, _Tp> _Closure;\
      return _Expr<_Closure, _Tp>(_Closure(__t, __v));                 \
    }

_DEFINE_EXPR_BINARY_FUNCTION(atan2, _Atan2)
_DEFINE_EXPR_BINARY_FUNCTION(pow, _Pow)

#undef _DEFINE_EXPR_BINARY_FUNCTION

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

#endif /* _CPP_VALARRAY_AFTER_H */
