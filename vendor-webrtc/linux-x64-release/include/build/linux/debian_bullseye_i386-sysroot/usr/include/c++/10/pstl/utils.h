// -*- C++ -*-
//===-- utils.h -----------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _PSTL_UTILS_H
#define _PSTL_UTILS_H

#include <new>
#include <iterator>

namespace __pstl
{
namespace __internal
{

template <typename _Fp>
typename std::result_of<_Fp()>::type
__except_handler(_Fp __f)
{
    try
    {
        return __f();
    }
    catch (const std::bad_alloc&)
    {
        throw; // re-throw bad_alloc according to the standard [algorithms.parallel.exceptions]
    }
    catch (...)
    {
        std::terminate(); // Good bye according to the standard [algorithms.parallel.exceptions]
    }
}

template <typename _Fp>
void
__invoke_if(std::true_type, _Fp __f)
{
    __f();
}

template <typename _Fp>
void
__invoke_if(std::false_type, _Fp __f)
{
}

template <typename _Fp>
void
__invoke_if_not(std::false_type, _Fp __f)
{
    __f();
}

template <typename _Fp>
void
__invoke_if_not(std::true_type, _Fp __f)
{
}

template <typename _F1, typename _F2>
typename std::result_of<_F1()>::type
__invoke_if_else(std::true_type, _F1 __f1, _F2 __f2)
{
    return __f1();
}

template <typename _F1, typename _F2>
typename std::result_of<_F2()>::type
__invoke_if_else(std::false_type, _F1 __f1, _F2 __f2)
{
    return __f2();
}

//! Unary operator that returns reference to its argument.
struct __no_op
{
    template <typename _Tp>
    _Tp&&
    operator()(_Tp&& __a) const
    {
        return std::forward<_Tp>(__a);
    }
};

//! Logical negation of a predicate
template <typename _Pred>
class __not_pred
{
    _Pred _M_pred;

  public:
    explicit __not_pred(_Pred __pred) : _M_pred(__pred) {}

    template <typename... _Args>
    bool
    operator()(_Args&&... __args)
    {
        return !_M_pred(std::forward<_Args>(__args)...);
    }
};

template <typename _Pred>
class __reorder_pred
{
    _Pred _M_pred;

  public:
    explicit __reorder_pred(_Pred __pred) : _M_pred(__pred) {}

    template <typename _FTp, typename _STp>
    bool
    operator()(_FTp&& __a, _STp&& __b)
    {
        return _M_pred(std::forward<_STp>(__b), std::forward<_FTp>(__a));
    }
};

//! "==" comparison.
/** Not called "equal" to avoid (possibly unfounded) concerns about accidental invocation via
    argument-dependent name lookup by code expecting to find the usual std::equal. */
class __pstl_equal
{
  public:
    explicit __pstl_equal() {}

    template <typename _Xp, typename _Yp>
    bool
    operator()(_Xp&& __x, _Yp&& __y) const
    {
        return std::forward<_Xp>(__x) == std::forward<_Yp>(__y);
    }
};

//! "<" comparison.
class __pstl_less
{
  public:
    explicit __pstl_less() {}

    template <typename _Xp, typename _Yp>
    bool
    operator()(_Xp&& __x, _Yp&& __y) const
    {
        return std::forward<_Xp>(__x) < std::forward<_Yp>(__y);
    }
};

//! Like a polymorphic lambda for pred(...,value)
template <typename _Tp, typename _Predicate>
class __equal_value_by_pred
{
    const _Tp& _M_value;
    _Predicate _M_pred;

  public:
    __equal_value_by_pred(const _Tp& __value, _Predicate __pred) : _M_value(__value), _M_pred(__pred) {}

    template <typename _Arg>
    bool
    operator()(_Arg&& __arg)
    {
        return _M_pred(std::forward<_Arg>(__arg), _M_value);
    }
};

//! Like a polymorphic lambda for ==value
template <typename _Tp>
class __equal_value
{
    const _Tp& _M_value;

  public:
    explicit __equal_value(const _Tp& __value) : _M_value(__value) {}

    template <typename _Arg>
    bool
    operator()(_Arg&& __arg) const
    {
        return std::forward<_Arg>(__arg) == _M_value;
    }
};

//! Logical negation of ==value
template <typename _Tp>
class __not_equal_value
{
    const _Tp& _M_value;

  public:
    explicit __not_equal_value(const _Tp& __value) : _M_value(__value) {}

    template <typename _Arg>
    bool
    operator()(_Arg&& __arg) const
    {
        return !(std::forward<_Arg>(__arg) == _M_value);
    }
};

template <typename _ForwardIterator, typename _Compare>
_ForwardIterator
__cmp_iterators_by_values(_ForwardIterator __a, _ForwardIterator __b, _Compare __comp)
{
    if (__a < __b)
    { // we should return closer iterator
        return __comp(*__b, *__a) ? __b : __a;
    }
    else
    {
        return __comp(*__a, *__b) ? __a : __b;
    }
}

} // namespace __internal
} // namespace __pstl

#endif /* _PSTL_UTILS_H */
