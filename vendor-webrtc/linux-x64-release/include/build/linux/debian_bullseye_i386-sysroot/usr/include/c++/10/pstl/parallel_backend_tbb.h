// -*- C++ -*-
//===-- parallel_backend_tbb.h --------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _PSTL_PARALLEL_BACKEND_TBB_H
#define _PSTL_PARALLEL_BACKEND_TBB_H

#include <algorithm>
#include <type_traits>

#include "parallel_backend_utils.h"

// Bring in minimal required subset of Intel TBB
#include <tbb/blocked_range.h>
#include <tbb/parallel_for.h>
#include <tbb/parallel_reduce.h>
#include <tbb/parallel_scan.h>
#include <tbb/parallel_invoke.h>
#include <tbb/task_arena.h>
#include <tbb/tbb_allocator.h>

#if TBB_INTERFACE_VERSION < 10000
#    error Intel(R) Threading Building Blocks 2018 is required; older versions are not supported.
#endif

namespace __pstl
{
namespace __par_backend
{

//! Raw memory buffer with automatic freeing and no exceptions.
/** Some of our algorithms need to start with raw memory buffer,
not an initialize array, because initialization/destruction
would make the span be at least O(N). */
// tbb::allocator can improve performance in some cases.
template <typename _Tp>
class __buffer
{
    tbb::tbb_allocator<_Tp> _M_allocator;
    _Tp* _M_ptr;
    const std::size_t _M_buf_size;
    __buffer(const __buffer&) = delete;
    void
    operator=(const __buffer&) = delete;

  public:
    //! Try to obtain buffer of given size to store objects of _Tp type
    __buffer(std::size_t n) : _M_allocator(), _M_ptr(_M_allocator.allocate(n)), _M_buf_size(n) {}
    //! True if buffer was successfully obtained, zero otherwise.
    operator bool() const { return _M_ptr != NULL; }
    //! Return pointer to buffer, or  NULL if buffer could not be obtained.
    _Tp*
    get() const
    {
        return _M_ptr;
    }
    //! Destroy buffer
    ~__buffer() { _M_allocator.deallocate(_M_ptr, _M_buf_size); }
};

// Wrapper for tbb::task
inline void
__cancel_execution()
{
    tbb::task::self().group()->cancel_group_execution();
}

//------------------------------------------------------------------------
// parallel_for
//------------------------------------------------------------------------

template <class _Index, class _RealBody>
class __parallel_for_body
{
  public:
    __parallel_for_body(const _RealBody& __body) : _M_body(__body) {}
    __parallel_for_body(const __parallel_for_body& __body) : _M_body(__body._M_body) {}
    void
    operator()(const tbb::blocked_range<_Index>& __range) const
    {
        _M_body(__range.begin(), __range.end());
    }

  private:
    _RealBody _M_body;
};

//! Evaluation of brick f[i,j) for each subrange [i,j) of [first,last)
// wrapper over tbb::parallel_for
template <class _ExecutionPolicy, class _Index, class _Fp>
void
__parallel_for(_ExecutionPolicy&&, _Index __first, _Index __last, _Fp __f)
{
    tbb::this_task_arena::isolate([=]() {
        tbb::parallel_for(tbb::blocked_range<_Index>(__first, __last), __parallel_for_body<_Index, _Fp>(__f));
    });
}

//! Evaluation of brick f[i,j) for each subrange [i,j) of [first,last)
// wrapper over tbb::parallel_reduce
template <class _ExecutionPolicy, class _Value, class _Index, typename _RealBody, typename _Reduction>
_Value
__parallel_reduce(_ExecutionPolicy&&, _Index __first, _Index __last, const _Value& __identity,
                  const _RealBody& __real_body, const _Reduction& __reduction)
{
    return tbb::this_task_arena::isolate([__first, __last, &__identity, &__real_body, &__reduction]() -> _Value {
        return tbb::parallel_reduce(
            tbb::blocked_range<_Index>(__first, __last), __identity,
            [__real_body](const tbb::blocked_range<_Index>& __r, const _Value& __value) -> _Value {
                return __real_body(__r.begin(), __r.end(), __value);
            },
            __reduction);
    });
}

//------------------------------------------------------------------------
// parallel_transform_reduce
//
// Notation:
//      r(i,j,init) returns reduction of init with reduction over [i,j)
//      u(i) returns f(i,i+1,identity) for a hypothetical left identity element of r
//      c(x,y) combines values x and y that were the result of r or u
//------------------------------------------------------------------------

template <class _Index, class _Up, class _Tp, class _Cp, class _Rp>
struct __par_trans_red_body
{
    alignas(_Tp) char _M_sum_storage[sizeof(_Tp)]; // Holds generalized non-commutative sum when has_sum==true
    _Rp _M_brick_reduce;                           // Most likely to have non-empty layout
    _Up _M_u;
    _Cp _M_combine;
    bool _M_has_sum; // Put last to minimize size of class
    _Tp&
    sum()
    {
        _PSTL_ASSERT_MSG(_M_has_sum, "sum expected");
        return *(_Tp*)_M_sum_storage;
    }
    __par_trans_red_body(_Up __u, _Tp __init, _Cp __c, _Rp __r)
        : _M_brick_reduce(__r), _M_u(__u), _M_combine(__c), _M_has_sum(true)
    {
        new (_M_sum_storage) _Tp(__init);
    }

    __par_trans_red_body(__par_trans_red_body& __left, tbb::split)
        : _M_brick_reduce(__left._M_brick_reduce), _M_u(__left._M_u), _M_combine(__left._M_combine), _M_has_sum(false)
    {
    }

    ~__par_trans_red_body()
    {
        // 17.6.5.12 tells us to not worry about catching exceptions from destructors.
        if (_M_has_sum)
            sum().~_Tp();
    }

    void
    join(__par_trans_red_body& __rhs)
    {
        sum() = _M_combine(sum(), __rhs.sum());
    }

    void
    operator()(const tbb::blocked_range<_Index>& __range)
    {
        _Index __i = __range.begin();
        _Index __j = __range.end();
        if (!_M_has_sum)
        {
            _PSTL_ASSERT_MSG(__range.size() > 1, "there should be at least 2 elements");
            new (&_M_sum_storage)
                _Tp(_M_combine(_M_u(__i), _M_u(__i + 1))); // The condition i+1 < j is provided by the grain size of 3
            _M_has_sum = true;
            std::advance(__i, 2);
            if (__i == __j)
                return;
        }
        sum() = _M_brick_reduce(__i, __j, sum());
    }
};

template <class _ExecutionPolicy, class _Index, class _Up, class _Tp, class _Cp, class _Rp>
_Tp
__parallel_transform_reduce(_ExecutionPolicy&&, _Index __first, _Index __last, _Up __u, _Tp __init, _Cp __combine,
                            _Rp __brick_reduce)
{
    __par_backend::__par_trans_red_body<_Index, _Up, _Tp, _Cp, _Rp> __body(__u, __init, __combine, __brick_reduce);
    // The grain size of 3 is used in order to provide mininum 2 elements for each body
    tbb::this_task_arena::isolate(
        [__first, __last, &__body]() { tbb::parallel_reduce(tbb::blocked_range<_Index>(__first, __last, 3), __body); });
    return __body.sum();
}

//------------------------------------------------------------------------
// parallel_scan
//------------------------------------------------------------------------

template <class _Index, class _Up, class _Tp, class _Cp, class _Rp, class _Sp>
class __trans_scan_body
{
    alignas(_Tp) char _M_sum_storage[sizeof(_Tp)]; // Holds generalized non-commutative sum when has_sum==true
    _Rp _M_brick_reduce;                           // Most likely to have non-empty layout
    _Up _M_u;
    _Cp _M_combine;
    _Sp _M_scan;
    bool _M_has_sum; // Put last to minimize size of class
  public:
    __trans_scan_body(_Up __u, _Tp __init, _Cp __combine, _Rp __reduce, _Sp __scan)
        : _M_brick_reduce(__reduce), _M_u(__u), _M_combine(__combine), _M_scan(__scan), _M_has_sum(true)
    {
        new (_M_sum_storage) _Tp(__init);
    }

    __trans_scan_body(__trans_scan_body& __b, tbb::split)
        : _M_brick_reduce(__b._M_brick_reduce), _M_u(__b._M_u), _M_combine(__b._M_combine), _M_scan(__b._M_scan),
          _M_has_sum(false)
    {
    }

    ~__trans_scan_body()
    {
        // 17.6.5.12 tells us to not worry about catching exceptions from destructors.
        if (_M_has_sum)
            sum().~_Tp();
    }

    _Tp&
    sum() const
    {
        _PSTL_ASSERT_MSG(_M_has_sum, "sum expected");
        return *const_cast<_Tp*>(reinterpret_cast<_Tp const*>(_M_sum_storage));
    }

    void
    operator()(const tbb::blocked_range<_Index>& __range, tbb::pre_scan_tag)
    {
        _Index __i = __range.begin();
        _Index __j = __range.end();
        if (!_M_has_sum)
        {
            new (&_M_sum_storage) _Tp(_M_u(__i));
            _M_has_sum = true;
            ++__i;
            if (__i == __j)
                return;
        }
        sum() = _M_brick_reduce(__i, __j, sum());
    }

    void
    operator()(const tbb::blocked_range<_Index>& __range, tbb::final_scan_tag)
    {
        sum() = _M_scan(__range.begin(), __range.end(), sum());
    }

    void
    reverse_join(__trans_scan_body& __a)
    {
        if (_M_has_sum)
        {
            sum() = _M_combine(__a.sum(), sum());
        }
        else
        {
            new (&_M_sum_storage) _Tp(__a.sum());
            _M_has_sum = true;
        }
    }

    void
    assign(__trans_scan_body& __b)
    {
        sum() = __b.sum();
    }
};

template <typename _Index>
_Index
__split(_Index __m)
{
    _Index __k = 1;
    while (2 * __k < __m)
        __k *= 2;
    return __k;
}

//------------------------------------------------------------------------
// __parallel_strict_scan
//------------------------------------------------------------------------

template <typename _Index, typename _Tp, typename _Rp, typename _Cp>
void
__upsweep(_Index __i, _Index __m, _Index __tilesize, _Tp* __r, _Index __lastsize, _Rp __reduce, _Cp __combine)
{
    if (__m == 1)
        __r[0] = __reduce(__i * __tilesize, __lastsize);
    else
    {
        _Index __k = __split(__m);
        tbb::parallel_invoke(
            [=] { __par_backend::__upsweep(__i, __k, __tilesize, __r, __tilesize, __reduce, __combine); },
            [=] {
                __par_backend::__upsweep(__i + __k, __m - __k, __tilesize, __r + __k, __lastsize, __reduce, __combine);
            });
        if (__m == 2 * __k)
            __r[__m - 1] = __combine(__r[__k - 1], __r[__m - 1]);
    }
}

template <typename _Index, typename _Tp, typename _Cp, typename _Sp>
void
__downsweep(_Index __i, _Index __m, _Index __tilesize, _Tp* __r, _Index __lastsize, _Tp __initial, _Cp __combine,
            _Sp __scan)
{
    if (__m == 1)
        __scan(__i * __tilesize, __lastsize, __initial);
    else
    {
        const _Index __k = __split(__m);
        tbb::parallel_invoke(
            [=] { __par_backend::__downsweep(__i, __k, __tilesize, __r, __tilesize, __initial, __combine, __scan); },
            // Assumes that __combine never throws.
            //TODO: Consider adding a requirement for user functors to be constant.
            [=, &__combine] {
                __par_backend::__downsweep(__i + __k, __m - __k, __tilesize, __r + __k, __lastsize,
                                           __combine(__initial, __r[__k - 1]), __combine, __scan);
            });
    }
}

// Adapted from Intel(R) Cilk(TM) version from cilkpub.
// Let i:len denote a counted interval of length n starting at i.  s denotes a generalized-sum value.
// Expected actions of the functors are:
//     reduce(i,len) -> s  -- return reduction value of i:len.
//     combine(s1,s2) -> s -- return merged sum
//     apex(s) -- do any processing necessary between reduce and scan.
//     scan(i,len,initial) -- perform scan over i:len starting with initial.
// The initial range 0:n is partitioned into consecutive subranges.
// reduce and scan are each called exactly once per subrange.
// Thus callers can rely upon side effects in reduce.
// combine must not throw an exception.
// apex is called exactly once, after all calls to reduce and before all calls to scan.
// For example, it's useful for allocating a __buffer used by scan but whose size is the sum of all reduction values.
// T must have a trivial constructor and destructor.
template <class _ExecutionPolicy, typename _Index, typename _Tp, typename _Rp, typename _Cp, typename _Sp, typename _Ap>
void
__parallel_strict_scan(_ExecutionPolicy&&, _Index __n, _Tp __initial, _Rp __reduce, _Cp __combine, _Sp __scan,
                       _Ap __apex)
{
    tbb::this_task_arena::isolate([=, &__combine]() {
        if (__n > 1)
        {
            _Index __p = tbb::this_task_arena::max_concurrency();
            const _Index __slack = 4;
            _Index __tilesize = (__n - 1) / (__slack * __p) + 1;
            _Index __m = (__n - 1) / __tilesize;
            __buffer<_Tp> __buf(__m + 1);
            _Tp* __r = __buf.get();
            __par_backend::__upsweep(_Index(0), _Index(__m + 1), __tilesize, __r, __n - __m * __tilesize, __reduce,
                                     __combine);

            // When __apex is a no-op and __combine has no side effects, a good optimizer
            // should be able to eliminate all code between here and __apex.
            // Alternatively, provide a default value for __apex that can be
            // recognized by metaprogramming that conditionlly executes the following.
            size_t __k = __m + 1;
            _Tp __t = __r[__k - 1];
            while ((__k &= __k - 1))
                __t = __combine(__r[__k - 1], __t);
            __apex(__combine(__initial, __t));
            __par_backend::__downsweep(_Index(0), _Index(__m + 1), __tilesize, __r, __n - __m * __tilesize, __initial,
                                       __combine, __scan);
            return;
        }
        // Fewer than 2 elements in sequence, or out of memory.  Handle has single block.
        _Tp __sum = __initial;
        if (__n)
            __sum = __combine(__sum, __reduce(_Index(0), __n));
        __apex(__sum);
        if (__n)
            __scan(_Index(0), __n, __initial);
    });
}

template <class _ExecutionPolicy, class _Index, class _Up, class _Tp, class _Cp, class _Rp, class _Sp>
_Tp
__parallel_transform_scan(_ExecutionPolicy&&, _Index __n, _Up __u, _Tp __init, _Cp __combine, _Rp __brick_reduce,
                          _Sp __scan)
{
    __trans_scan_body<_Index, _Up, _Tp, _Cp, _Rp, _Sp> __body(__u, __init, __combine, __brick_reduce, __scan);
    auto __range = tbb::blocked_range<_Index>(0, __n);
    tbb::this_task_arena::isolate([__range, &__body]() { tbb::parallel_scan(__range, __body); });
    return __body.sum();
}

//------------------------------------------------------------------------
// parallel_stable_sort
//------------------------------------------------------------------------

//------------------------------------------------------------------------
// stable_sort utilities
//
// These are used by parallel implementations but do not depend on them.
//------------------------------------------------------------------------

template <typename _RandomAccessIterator1, typename _RandomAccessIterator2, typename _RandomAccessIterator3,
          typename _Compare, typename _Cleanup, typename _LeafMerge>
class __merge_task : public tbb::task
{
    /*override*/ tbb::task*
    execute();
    _RandomAccessIterator1 _M_xs, _M_xe;
    _RandomAccessIterator2 _M_ys, _M_ye;
    _RandomAccessIterator3 _M_zs;
    _Compare _M_comp;
    _Cleanup _M_cleanup;
    _LeafMerge _M_leaf_merge;

  public:
    __merge_task(_RandomAccessIterator1 __xs, _RandomAccessIterator1 __xe, _RandomAccessIterator2 __ys,
                 _RandomAccessIterator2 __ye, _RandomAccessIterator3 __zs, _Compare __comp, _Cleanup __cleanup,
                 _LeafMerge __leaf_merge)
        : _M_xs(__xs), _M_xe(__xe), _M_ys(__ys), _M_ye(__ye), _M_zs(__zs), _M_comp(__comp), _M_cleanup(__cleanup),
          _M_leaf_merge(__leaf_merge)
    {
    }
};

#define _PSTL_MERGE_CUT_OFF 2000

template <typename _RandomAccessIterator1, typename _RandomAccessIterator2, typename _RandomAccessIterator3,
          typename __M_Compare, typename _Cleanup, typename _LeafMerge>
tbb::task*
__merge_task<_RandomAccessIterator1, _RandomAccessIterator2, _RandomAccessIterator3, __M_Compare, _Cleanup,
             _LeafMerge>::execute()
{
    typedef typename std::iterator_traits<_RandomAccessIterator1>::difference_type _DifferenceType1;
    typedef typename std::iterator_traits<_RandomAccessIterator2>::difference_type _DifferenceType2;
    typedef typename std::common_type<_DifferenceType1, _DifferenceType2>::type _SizeType;
    const _SizeType __n = (_M_xe - _M_xs) + (_M_ye - _M_ys);
    const _SizeType __merge_cut_off = _PSTL_MERGE_CUT_OFF;
    if (__n <= __merge_cut_off)
    {
        _M_leaf_merge(_M_xs, _M_xe, _M_ys, _M_ye, _M_zs, _M_comp);

        //we clean the buffer one time on last step of the sort
        _M_cleanup(_M_xs, _M_xe);
        _M_cleanup(_M_ys, _M_ye);
        return nullptr;
    }
    else
    {
        _RandomAccessIterator1 __xm;
        _RandomAccessIterator2 __ym;
        if (_M_xe - _M_xs < _M_ye - _M_ys)
        {
            __ym = _M_ys + (_M_ye - _M_ys) / 2;
            __xm = std::upper_bound(_M_xs, _M_xe, *__ym, _M_comp);
        }
        else
        {
            __xm = _M_xs + (_M_xe - _M_xs) / 2;
            __ym = std::lower_bound(_M_ys, _M_ye, *__xm, _M_comp);
        }
        const _RandomAccessIterator3 __zm = _M_zs + ((__xm - _M_xs) + (__ym - _M_ys));
        tbb::task* __right = new (tbb::task::allocate_additional_child_of(*parent()))
            __merge_task(__xm, _M_xe, __ym, _M_ye, __zm, _M_comp, _M_cleanup, _M_leaf_merge);
        tbb::task::spawn(*__right);
        tbb::task::recycle_as_continuation();
        _M_xe = __xm;
        _M_ye = __ym;
    }
    return this;
}

template <typename _RandomAccessIterator1, typename _RandomAccessIterator2, typename _Compare, typename _LeafSort>
class __stable_sort_task : public tbb::task
{
  public:
    typedef typename std::iterator_traits<_RandomAccessIterator1>::difference_type _DifferenceType1;
    typedef typename std::iterator_traits<_RandomAccessIterator2>::difference_type _DifferenceType2;
    typedef typename std::common_type<_DifferenceType1, _DifferenceType2>::type _SizeType;

  private:
    /*override*/ tbb::task*
    execute();
    _RandomAccessIterator1 _M_xs, _M_xe;
    _RandomAccessIterator2 _M_zs;
    _Compare _M_comp;
    _LeafSort _M_leaf_sort;
    int32_t _M_inplace;
    _SizeType _M_nsort;

  public:
    __stable_sort_task(_RandomAccessIterator1 __xs, _RandomAccessIterator1 __xe, _RandomAccessIterator2 __zs,
                       int32_t __inplace, _Compare __comp, _LeafSort __leaf_sort, _SizeType __n)
        : _M_xs(__xs), _M_xe(__xe), _M_zs(__zs), _M_comp(__comp), _M_leaf_sort(__leaf_sort), _M_inplace(__inplace),
          _M_nsort(__n)
    {
    }
};

//! Binary operator that does nothing
struct __binary_no_op
{
    template <typename _Tp>
    void operator()(_Tp, _Tp)
    {
    }
};

#define _PSTL_STABLE_SORT_CUT_OFF 500

template <typename _RandomAccessIterator1, typename _RandomAccessIterator2, typename _Compare, typename _LeafSort>
tbb::task*
__stable_sort_task<_RandomAccessIterator1, _RandomAccessIterator2, _Compare, _LeafSort>::execute()
{
    const _SizeType __n = _M_xe - _M_xs;
    const _SizeType __nmerge = _M_nsort > 0 ? _M_nsort : __n;
    const _SizeType __sort_cut_off = _PSTL_STABLE_SORT_CUT_OFF;
    if (__n <= __sort_cut_off)
    {
        _M_leaf_sort(_M_xs, _M_xe, _M_comp);
        if (_M_inplace != 2)
            __par_backend::__init_buf(_M_xs, _M_xe, _M_zs, _M_inplace == 0);
        return NULL;
    }
    else
    {
        const _RandomAccessIterator1 __xm = _M_xs + __n / 2;
        const _RandomAccessIterator2 __zm = _M_zs + (__xm - _M_xs);
        const _RandomAccessIterator2 __ze = _M_zs + __n;
        task* __m;
        auto __move_values = [](_RandomAccessIterator2 __x, _RandomAccessIterator1 __z) { *__z = std::move(*__x); };
        auto __move_sequences = [](_RandomAccessIterator2 __first1, _RandomAccessIterator2 __last1,
                                   _RandomAccessIterator1 __first2) { return std::move(__first1, __last1, __first2); };
        if (_M_inplace == 2)
            __m = new (tbb::task::allocate_continuation())
                __merge_task<_RandomAccessIterator2, _RandomAccessIterator2, _RandomAccessIterator1, _Compare,
                             __serial_destroy,
                             __par_backend::__serial_move_merge<decltype(__move_values), decltype(__move_sequences)>>(
                    _M_zs, __zm, __zm, __ze, _M_xs, _M_comp, __serial_destroy(),
                    __par_backend::__serial_move_merge<decltype(__move_values), decltype(__move_sequences)>(
                        __nmerge, __move_values, __move_sequences));
        else if (_M_inplace)
            __m = new (tbb::task::allocate_continuation())
                __merge_task<_RandomAccessIterator2, _RandomAccessIterator2, _RandomAccessIterator1, _Compare,
                             __par_backend::__binary_no_op,
                             __par_backend::__serial_move_merge<decltype(__move_values), decltype(__move_sequences)>>(
                    _M_zs, __zm, __zm, __ze, _M_xs, _M_comp, __par_backend::__binary_no_op(),
                    __par_backend::__serial_move_merge<decltype(__move_values), decltype(__move_sequences)>(
                        __nmerge, __move_values, __move_sequences));
        else
        {
            auto __move_values = [](_RandomAccessIterator1 __x, _RandomAccessIterator2 __z) { *__z = std::move(*__x); };
            auto __move_sequences = [](_RandomAccessIterator1 __first1, _RandomAccessIterator1 __last1,
                                       _RandomAccessIterator2 __first2) {
                return std::move(__first1, __last1, __first2);
            };
            __m = new (tbb::task::allocate_continuation())
                __merge_task<_RandomAccessIterator1, _RandomAccessIterator1, _RandomAccessIterator2, _Compare,
                             __par_backend::__binary_no_op,
                             __par_backend::__serial_move_merge<decltype(__move_values), decltype(__move_sequences)>>(
                    _M_xs, __xm, __xm, _M_xe, _M_zs, _M_comp, __par_backend::__binary_no_op(),
                    __par_backend::__serial_move_merge<decltype(__move_values), decltype(__move_sequences)>(
                        __nmerge, __move_values, __move_sequences));
        }
        __m->set_ref_count(2);
        task* __right = new (__m->allocate_child())
            __stable_sort_task(__xm, _M_xe, __zm, !_M_inplace, _M_comp, _M_leaf_sort, __nmerge);
        tbb::task::spawn(*__right);
        tbb::task::recycle_as_child_of(*__m);
        _M_xe = __xm;
        _M_inplace = !_M_inplace;
    }
    return this;
}

template <class _ExecutionPolicy, typename _RandomAccessIterator, typename _Compare, typename _LeafSort>
void
__parallel_stable_sort(_ExecutionPolicy&&, _RandomAccessIterator __xs, _RandomAccessIterator __xe, _Compare __comp,
                       _LeafSort __leaf_sort, std::size_t __nsort = 0)
{
    tbb::this_task_arena::isolate([=, &__nsort]() {
        //sorting based on task tree and parallel merge
        typedef typename std::iterator_traits<_RandomAccessIterator>::value_type _ValueType;
        typedef typename std::iterator_traits<_RandomAccessIterator>::difference_type _DifferenceType;
        const _DifferenceType __n = __xe - __xs;
        if (__nsort == 0)
            __nsort = __n;

        const _DifferenceType __sort_cut_off = _PSTL_STABLE_SORT_CUT_OFF;
        if (__n > __sort_cut_off)
        {
            _PSTL_ASSERT(__nsort > 0 && __nsort <= __n);
            __buffer<_ValueType> __buf(__n);
            using tbb::task;
            task::spawn_root_and_wait(*new (task::allocate_root())
                                          __stable_sort_task<_RandomAccessIterator, _ValueType*, _Compare, _LeafSort>(
                                              __xs, __xe, (_ValueType*)__buf.get(), 2, __comp, __leaf_sort, __nsort));
            return;
        }
        //serial sort
        __leaf_sort(__xs, __xe, __comp);
    });
}

//------------------------------------------------------------------------
// parallel_merge
//------------------------------------------------------------------------

template <class _ExecutionPolicy, typename _RandomAccessIterator1, typename _RandomAccessIterator2,
          typename _RandomAccessIterator3, typename _Compare, typename _LeafMerge>
void
__parallel_merge(_ExecutionPolicy&&, _RandomAccessIterator1 __xs, _RandomAccessIterator1 __xe,
                 _RandomAccessIterator2 __ys, _RandomAccessIterator2 __ye, _RandomAccessIterator3 __zs, _Compare __comp,
                 _LeafMerge __leaf_merge)
{
    typedef typename std::iterator_traits<_RandomAccessIterator1>::difference_type _DifferenceType1;
    typedef typename std::iterator_traits<_RandomAccessIterator2>::difference_type _DifferenceType2;
    typedef typename std::common_type<_DifferenceType1, _DifferenceType2>::type _SizeType;
    const _SizeType __n = (__xe - __xs) + (__ye - __ys);
    const _SizeType __merge_cut_off = _PSTL_MERGE_CUT_OFF;
    if (__n <= __merge_cut_off)
    {
        // Fall back on serial merge
        __leaf_merge(__xs, __xe, __ys, __ye, __zs, __comp);
    }
    else
    {
        tbb::this_task_arena::isolate([=]() {
            typedef __merge_task<_RandomAccessIterator1, _RandomAccessIterator2, _RandomAccessIterator3, _Compare,
                                 __par_backend::__binary_no_op, _LeafMerge>
                _TaskType;
            tbb::task::spawn_root_and_wait(*new (tbb::task::allocate_root()) _TaskType(
                __xs, __xe, __ys, __ye, __zs, __comp, __par_backend::__binary_no_op(), __leaf_merge));
        });
    }
}

//------------------------------------------------------------------------
// parallel_invoke
//------------------------------------------------------------------------
template <class _ExecutionPolicy, typename _F1, typename _F2>
void
__parallel_invoke(_ExecutionPolicy&&, _F1&& __f1, _F2&& __f2)
{
    //TODO: a version of tbb::this_task_arena::isolate with variadic arguments pack should be added in the future
    tbb::this_task_arena::isolate([&]() { tbb::parallel_invoke(std::forward<_F1>(__f1), std::forward<_F2>(__f2)); });
}

} // namespace __par_backend
} // namespace __pstl

#endif /* _PSTL_PARALLEL_BACKEND_TBB_H */
