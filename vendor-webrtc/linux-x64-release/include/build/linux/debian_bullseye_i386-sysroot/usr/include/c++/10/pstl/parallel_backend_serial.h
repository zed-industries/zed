// -*- C++ -*-
//===-- parallel_backend_serial.h -----------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _PSTL_PARALLEL_BACKEND_SERIAL_H
#define _PSTL_PARALLEL_BACKEND_SERIAL_H

#include <algorithm>
#include <cstddef>
#include <memory>
#include <numeric>
#include <utility>

namespace __pstl
{
namespace __serial
{

template <typename _Tp>
class __buffer
{
    std::allocator<_Tp> __allocator_;
    _Tp* __ptr_;
    const std::size_t __buf_size_;
    __buffer(const __buffer&) = delete;
    void
    operator=(const __buffer&) = delete;

  public:
    __buffer(std::size_t __n) : __allocator_(), __ptr_(__allocator_.allocate(__n)), __buf_size_(__n) {}

    operator bool() const { return __ptr_ != nullptr; }
    _Tp*
    get() const
    {
        return __ptr_;
    }
    ~__buffer() { __allocator_.deallocate(__ptr_, __buf_size_); }
};

inline void
__cancel_execution()
{
}

template <class _ExecutionPolicy, class _Index, class _Fp>
void
__parallel_for(_ExecutionPolicy&&, _Index __first, _Index __last, _Fp __f)
{
    __f(__first, __last);
}

template <class _ExecutionPolicy, class _Value, class _Index, typename _RealBody, typename _Reduction>
_Value
__parallel_reduce(_ExecutionPolicy&&, _Index __first, _Index __last, const _Value& __identity,
                  const _RealBody& __real_body, const _Reduction&)
{
    if (__first == __last)
    {
        return __identity;
    }
    else
    {
        return __real_body(__first, __last, __identity);
    }
}

template <class _ExecutionPolicy, class _Index, class _UnaryOp, class _Tp, class _BinaryOp, class _Reduce>
_Tp
__parallel_transform_reduce(_ExecutionPolicy&&, _Index __first, _Index __last, _UnaryOp, _Tp __init, _BinaryOp,
                            _Reduce __reduce)
{
    return __reduce(__first, __last, __init);
}

template <class _ExecutionPolicy, typename _Index, typename _Tp, typename _Rp, typename _Cp, typename _Sp, typename _Ap>
void
__parallel_strict_scan(_ExecutionPolicy&&, _Index __n, _Tp __initial, _Rp __reduce, _Cp __combine, _Sp __scan,
                       _Ap __apex)
{
    _Tp __sum = __initial;
    if (__n)
        __sum = __combine(__sum, __reduce(_Index(0), __n));
    __apex(__sum);
    if (__n)
        __scan(_Index(0), __n, __initial);
}

template <class _ExecutionPolicy, class _Index, class _UnaryOp, class _Tp, class _BinaryOp, class _Reduce, class _Scan>
_Tp
__parallel_transform_scan(_ExecutionPolicy&&, _Index __n, _UnaryOp, _Tp __init, _BinaryOp, _Reduce, _Scan __scan)
{
    return __scan(_Index(0), __n, __init);
}

template <class _ExecutionPolicy, typename _RandomAccessIterator, typename _Compare, typename _LeafSort>
void
__parallel_stable_sort(_ExecutionPolicy&&, _RandomAccessIterator __first, _RandomAccessIterator __last, _Compare __comp,
                       _LeafSort __leaf_sort, std::size_t = 0)
{
    __leaf_sort(__first, __last, __comp);
}

template <class _ExecutionPolicy, typename _RandomAccessIterator1, typename _RandomAccessIterator2,
          typename _RandomAccessIterator3, typename _Compare, typename _LeafMerge>
void
__parallel_merge(_ExecutionPolicy&&, _RandomAccessIterator1 __first1, _RandomAccessIterator1 __last1,
                 _RandomAccessIterator2 __first2, _RandomAccessIterator2 __last2, _RandomAccessIterator3 __out,
                 _Compare __comp, _LeafMerge __leaf_merge)
{
    __leaf_merge(__first1, __last1, __first2, __last2, __out, __comp);
}

template <class _ExecutionPolicy, typename _F1, typename _F2>
void
__parallel_invoke(_ExecutionPolicy&&, _F1&& __f1, _F2&& __f2)
{
    std::forward<_F1>(__f1)();
    std::forward<_F2>(__f2)();
}

} // namespace __serial
} // namespace __pstl

namespace __pstl
{
namespace __par_backend
{
using namespace __pstl::__serial;
}
} // namespace __pstl

#endif /* _PSTL_PARALLEL_BACKEND_SERIAL_H */
