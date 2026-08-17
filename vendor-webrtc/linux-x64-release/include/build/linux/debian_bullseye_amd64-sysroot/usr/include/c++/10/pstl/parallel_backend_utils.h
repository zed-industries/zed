// -*- C++ -*-
//===-- parallel_backend_utils.h ------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _PSTL_PARALLEL_BACKEND_UTILS_H
#define _PSTL_PARALLEL_BACKEND_UTILS_H

#include <iterator>
#include <utility>
#include "utils.h"

namespace __pstl
{
namespace __par_backend
{

//! Destroy sequence [xs,xe)
struct __serial_destroy
{
    template <typename _RandomAccessIterator>
    void
    operator()(_RandomAccessIterator __zs, _RandomAccessIterator __ze)
    {
        typedef typename std::iterator_traits<_RandomAccessIterator>::value_type _ValueType;
        while (__zs != __ze)
        {
            --__ze;
            (*__ze).~_ValueType();
        }
    }
};

//! Merge sequences [__xs,__xe) and [__ys,__ye) to output sequence [__zs,(__xe-__xs)+(__ye-__ys)), using std::move
template <class _MoveValues, class _MoveSequences>
struct __serial_move_merge
{
    const std::size_t _M_nmerge;
    _MoveValues _M_move_values;
    _MoveSequences _M_move_sequences;

    explicit __serial_move_merge(std::size_t __nmerge, _MoveValues __move_values, _MoveSequences __move_sequences)
        : _M_nmerge(__nmerge), _M_move_values(__move_values), _M_move_sequences(__move_sequences)
    {
    }
    template <class _RandomAccessIterator1, class _RandomAccessIterator2, class _RandomAccessIterator3, class _Compare>
    void
    operator()(_RandomAccessIterator1 __xs, _RandomAccessIterator1 __xe, _RandomAccessIterator2 __ys,
               _RandomAccessIterator2 __ye, _RandomAccessIterator3 __zs, _Compare __comp)
    {
        auto __n = _M_nmerge;
        _PSTL_ASSERT(__n > 0);
        if (__xs != __xe)
        {
            if (__ys != __ye)
            {
                for (;;)
                {
                    if (__comp(*__ys, *__xs))
                    {
                        _M_move_values(__ys, __zs);
                        ++__zs, --__n;
                        if (++__ys == __ye)
                        {
                            break;
                        }
                        else if (__n == 0)
                        {
                            __zs = _M_move_sequences(__ys, __ye, __zs);
                            break;
                        }
                        else
                        {
                        }
                    }
                    else
                    {
                        _M_move_values(__xs, __zs);
                        ++__zs, --__n;
                        if (++__xs == __xe)
                        {
                            _M_move_sequences(__ys, __ye, __zs);
                            return;
                        }
                        else if (__n == 0)
                        {
                            __zs = _M_move_sequences(__xs, __xe, __zs);
                            _M_move_sequences(__ys, __ye, __zs);
                            return;
                        }
                        else
                        {
                        }
                    }
                }
            }
            __ys = __xs;
            __ye = __xe;
        }
        _M_move_sequences(__ys, __ye, __zs);
    }
};

template <typename _RandomAccessIterator1, typename _OutputIterator>
void
__init_buf(_RandomAccessIterator1 __xs, _RandomAccessIterator1 __xe, _OutputIterator __zs, bool __bMove)
{
    const _OutputIterator __ze = __zs + (__xe - __xs);
    typedef typename std::iterator_traits<_OutputIterator>::value_type _ValueType;
    if (__bMove)
    {
        // Initialize the temporary buffer and move keys to it.
        for (; __zs != __ze; ++__xs, ++__zs)
            new (&*__zs) _ValueType(std::move(*__xs));
    }
    else
    {
        // Initialize the temporary buffer
        for (; __zs != __ze; ++__zs)
            new (&*__zs) _ValueType;
    }
}

// TODO is this actually used anywhere?
template <typename _Buf>
class __stack
{
    typedef typename std::iterator_traits<decltype(_Buf(0).get())>::value_type _ValueType;
    typedef typename std::iterator_traits<_ValueType*>::difference_type _DifferenceType;

    _Buf _M_buf;
    _ValueType* _M_ptr;
    _DifferenceType _M_maxsize;

    __stack(const __stack&) = delete;
    void
    operator=(const __stack&) = delete;

  public:
    __stack(_DifferenceType __max_size) : _M_buf(__max_size), _M_maxsize(__max_size) { _M_ptr = _M_buf.get(); }

    ~__stack()
    {
        _PSTL_ASSERT(size() <= _M_maxsize);
        while (!empty())
            pop();
    }

    const _Buf&
    buffer() const
    {
        return _M_buf;
    }
    size_t
    size() const
    {
        _PSTL_ASSERT(_M_ptr - _M_buf.get() <= _M_maxsize);
        _PSTL_ASSERT(_M_ptr - _M_buf.get() >= 0);
        return _M_ptr - _M_buf.get();
    }
    bool
    empty() const
    {
        _PSTL_ASSERT(_M_ptr >= _M_buf.get());
        return _M_ptr == _M_buf.get();
    }
    void
    push(const _ValueType& __v)
    {
        _PSTL_ASSERT(size() < _M_maxsize);
        new (_M_ptr) _ValueType(__v);
        ++_M_ptr;
    }
    const _ValueType&
    top() const
    {
        return *(_M_ptr - 1);
    }
    void
    pop()
    {
        _PSTL_ASSERT(_M_ptr > _M_buf.get());
        --_M_ptr;
        (*_M_ptr).~_ValueType();
    }
};

} // namespace __par_backend
} // namespace __pstl

#endif /* _PSTL_PARALLEL_BACKEND_UTILS_H */
