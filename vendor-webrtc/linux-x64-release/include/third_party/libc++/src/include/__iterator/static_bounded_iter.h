// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___ITERATOR_STATIC_BOUNDED_ITER_H
#define _LIBCPP___ITERATOR_STATIC_BOUNDED_ITER_H

#include <__assert>
#include <__compare/ordering.h>
#include <__compare/three_way_comparable.h>
#include <__config>
#include <__cstddef/size_t.h>
#include <__iterator/iterator_traits.h>
#include <__memory/pointer_traits.h>
#include <__type_traits/conjunction.h>
#include <__type_traits/disjunction.h>
#include <__type_traits/enable_if.h>
#include <__type_traits/integral_constant.h>
#include <__type_traits/is_convertible.h>
#include <__type_traits/is_same.h>
#include <__type_traits/make_const_lvalue_ref.h>
#include <__utility/move.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_PUSH_MACROS
#include <__undef_macros>

_LIBCPP_BEGIN_NAMESPACE_STD

template <class _Iterator, size_t _Size>
struct __static_bounded_iter_storage {
  _LIBCPP_HIDE_FROM_ABI __static_bounded_iter_storage() = default;
  _LIBCPP_HIDE_FROM_ABI
  _LIBCPP_CONSTEXPR_SINCE_CXX14 explicit __static_bounded_iter_storage(_Iterator __current, _Iterator __begin)
      : __current_(__current), __begin_(__begin) {}

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX14 _Iterator& __current() _NOEXCEPT { return __current_; }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX14 _Iterator __current() const _NOEXCEPT { return __current_; }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX14 _Iterator __begin() const _NOEXCEPT { return __begin_; }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX14 _Iterator __end() const _NOEXCEPT { return __begin_ + _Size; }

private:
  _Iterator __current_; // current iterator
  _Iterator __begin_;   // start of the valid range, which is [__begin_, __begin_ + _Size)
};

template <class _Iterator>
struct __static_bounded_iter_storage<_Iterator, 0> {
  _LIBCPP_HIDE_FROM_ABI __static_bounded_iter_storage() = default;
  _LIBCPP_HIDE_FROM_ABI
  _LIBCPP_CONSTEXPR_SINCE_CXX14 explicit __static_bounded_iter_storage(_Iterator __current, _Iterator /* __begin */)
      : __current_(__current) {}

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX14 _Iterator& __current() _NOEXCEPT { return __current_; }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX14 _Iterator __current() const _NOEXCEPT { return __current_; }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX14 _Iterator __begin() const _NOEXCEPT { return __current_; }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX14 _Iterator __end() const _NOEXCEPT { return __current_; }

private:
  _Iterator __current_; // current iterator
};

// This is an iterator wrapper for contiguous iterators that points within a range
// whose size is known at compile-time. This is very similar to `__bounded_iter`,
// except that we don't have to store the end of the range in physical memory since
// it can be computed from the start of the range.
//
// The operations on which this iterator wrapper traps are the same as `__bounded_iter`.
template <class _Iterator, size_t _Size>
struct __static_bounded_iter {
  static_assert(__libcpp_is_contiguous_iterator<_Iterator>::value,
                "Only contiguous iterators can be adapted by __static_bounded_iter.");

  using value_type        = typename iterator_traits<_Iterator>::value_type;
  using difference_type   = typename iterator_traits<_Iterator>::difference_type;
  using pointer           = typename iterator_traits<_Iterator>::pointer;
  using reference         = typename iterator_traits<_Iterator>::reference;
  using iterator_category = typename iterator_traits<_Iterator>::iterator_category;
#if _LIBCPP_STD_VER >= 20
  using iterator_concept = contiguous_iterator_tag;
#endif

  // Create a singular iterator.
  //
  // Such an iterator points past the end of an empty range, so it is not dereferenceable.
  // Operations like comparison and assignment are valid.
  _LIBCPP_HIDE_FROM_ABI __static_bounded_iter() = default;

  _LIBCPP_HIDE_FROM_ABI __static_bounded_iter(__static_bounded_iter const&) = default;
  _LIBCPP_HIDE_FROM_ABI __static_bounded_iter(__static_bounded_iter&&)      = default;

  template <class _OtherIterator,
            __enable_if_t<
                _And< is_convertible<const _OtherIterator&, _Iterator>,
                      _Or<is_same<reference, __iter_reference<_OtherIterator> >,
                          is_same<reference, __make_const_lvalue_ref<__iter_reference<_OtherIterator> > > > >::value,
                int> = 0>
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR
  __static_bounded_iter(__static_bounded_iter<_OtherIterator, _Size> const& __other) _NOEXCEPT
      : __storage_(__other.__storage_.__current(), __other.__storage_.__begin()) {}

  // Assign a bounded iterator to another one, rebinding the bounds of the iterator as well.
  _LIBCPP_HIDE_FROM_ABI __static_bounded_iter& operator=(__static_bounded_iter const&) = default;
  _LIBCPP_HIDE_FROM_ABI __static_bounded_iter& operator=(__static_bounded_iter&&)      = default;

private:
  // Create an iterator wrapping the given iterator, and whose bounds are described
  // by the provided [begin, begin + _Size] range.
  _LIBCPP_HIDE_FROM_ABI
  _LIBCPP_CONSTEXPR_SINCE_CXX14 explicit __static_bounded_iter(_Iterator __current, _Iterator __begin)
      : __storage_(__current, __begin) {
    _LIBCPP_ASSERT_INTERNAL(
        __begin <= __current, "__static_bounded_iter(current, begin): current and begin are inconsistent");
    _LIBCPP_ASSERT_INTERNAL(
        __current <= __end(), "__static_bounded_iter(current, begin): current and (begin + Size) are inconsistent");
  }

  template <size_t _Sz, class _It>
  friend _LIBCPP_CONSTEXPR __static_bounded_iter<_It, _Sz> __make_static_bounded_iter(_It, _It);

public:
  // Dereference and indexing operations.
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX14 reference operator*() const _NOEXCEPT {
    _LIBCPP_ASSERT_VALID_ELEMENT_ACCESS(
        __current() != __end(), "__static_bounded_iter::operator*: Attempt to dereference an iterator at the end");
    return *__current();
  }

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX14 pointer operator->() const _NOEXCEPT {
    _LIBCPP_ASSERT_VALID_ELEMENT_ACCESS(
        __current() != __end(), "__static_bounded_iter::operator->: Attempt to dereference an iterator at the end");
    return std::__to_address(__current());
  }

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX14 reference operator[](difference_type __n) const _NOEXCEPT {
    _LIBCPP_ASSERT_VALID_ELEMENT_ACCESS(
        __n >= __begin() - __current(),
        "__static_bounded_iter::operator[]: Attempt to index an iterator past the start");
    _LIBCPP_ASSERT_VALID_ELEMENT_ACCESS(
        __n < __end() - __current(),
        "__static_bounded_iter::operator[]: Attempt to index an iterator at or past the end");
    return __current()[__n];
  }

  // Arithmetic operations.
  //
  // These operations check that the iterator remains within `[begin, end]`.
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX14 __static_bounded_iter& operator++() _NOEXCEPT {
    _LIBCPP_ASSERT_VALID_ELEMENT_ACCESS(
        __current() != __end(), "__static_bounded_iter::operator++: Attempt to advance an iterator past the end");
    ++__current();
    return *this;
  }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX14 __static_bounded_iter operator++(int) _NOEXCEPT {
    __static_bounded_iter __tmp(*this);
    ++*this;
    return __tmp;
  }

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX14 __static_bounded_iter& operator--() _NOEXCEPT {
    _LIBCPP_ASSERT_VALID_ELEMENT_ACCESS(
        __current() != __begin(), "__static_bounded_iter::operator--: Attempt to rewind an iterator past the start");
    --__current();
    return *this;
  }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX14 __static_bounded_iter operator--(int) _NOEXCEPT {
    __static_bounded_iter __tmp(*this);
    --*this;
    return __tmp;
  }

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX14 __static_bounded_iter& operator+=(difference_type __n) _NOEXCEPT {
    _LIBCPP_ASSERT_VALID_ELEMENT_ACCESS(
        __n >= __begin() - __current(),
        "__static_bounded_iter::operator+=: Attempt to rewind an iterator past the start");
    _LIBCPP_ASSERT_VALID_ELEMENT_ACCESS(
        __n <= __end() - __current(), "__static_bounded_iter::operator+=: Attempt to advance an iterator past the end");
    __current() += __n;
    return *this;
  }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX14 friend __static_bounded_iter
  operator+(__static_bounded_iter const& __self, difference_type __n) _NOEXCEPT {
    __static_bounded_iter __tmp(__self);
    __tmp += __n;
    return __tmp;
  }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX14 friend __static_bounded_iter
  operator+(difference_type __n, __static_bounded_iter const& __self) _NOEXCEPT {
    __static_bounded_iter __tmp(__self);
    __tmp += __n;
    return __tmp;
  }

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX14 __static_bounded_iter& operator-=(difference_type __n) _NOEXCEPT {
    _LIBCPP_ASSERT_VALID_ELEMENT_ACCESS(
        __n <= __current() - __begin(),
        "__static_bounded_iter::operator-=: Attempt to rewind an iterator past the start");
    _LIBCPP_ASSERT_VALID_ELEMENT_ACCESS(
        __n >= __current() - __end(), "__static_bounded_iter::operator-=: Attempt to advance an iterator past the end");
    __current() -= __n;
    return *this;
  }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX14 friend __static_bounded_iter
  operator-(__static_bounded_iter const& __self, difference_type __n) _NOEXCEPT {
    __static_bounded_iter __tmp(__self);
    __tmp -= __n;
    return __tmp;
  }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX14 friend difference_type
  operator-(__static_bounded_iter const& __x, __static_bounded_iter const& __y) _NOEXCEPT {
    return __x.__current() - __y.__current();
  }

  // Comparison operations.
  //
  // These operations do not check whether the iterators are within their bounds.
  // The valid range for each iterator is also not considered as part of the comparison,
  // i.e. two iterators pointing to the same location will be considered equal even
  // if they have different validity ranges.
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR friend bool
  operator==(__static_bounded_iter const& __x, __static_bounded_iter const& __y) _NOEXCEPT {
    return __x.__current() == __y.__current();
  }

#if _LIBCPP_STD_VER <= 17
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR friend bool
  operator!=(__static_bounded_iter const& __x, __static_bounded_iter const& __y) _NOEXCEPT {
    return __x.__current() != __y.__current();
  }

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR friend bool
  operator<(__static_bounded_iter const& __x, __static_bounded_iter const& __y) _NOEXCEPT {
    return __x.__current() < __y.__current();
  }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR friend bool
  operator>(__static_bounded_iter const& __x, __static_bounded_iter const& __y) _NOEXCEPT {
    return __x.__current() > __y.__current();
  }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR friend bool
  operator<=(__static_bounded_iter const& __x, __static_bounded_iter const& __y) _NOEXCEPT {
    return __x.__current() <= __y.__current();
  }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR friend bool
  operator>=(__static_bounded_iter const& __x, __static_bounded_iter const& __y) _NOEXCEPT {
    return __x.__current() >= __y.__current();
  }

#else
  _LIBCPP_HIDE_FROM_ABI constexpr friend strong_ordering
  operator<=>(__static_bounded_iter const& __x, __static_bounded_iter const& __y) noexcept {
    if constexpr (three_way_comparable<_Iterator, strong_ordering>) {
      return __x.__current() <=> __y.__current();
    } else {
      if (__x.__current() < __y.__current())
        return strong_ordering::less;

      if (__x.__current() == __y.__current())
        return strong_ordering::equal;

      return strong_ordering::greater;
    }
  }
#endif // _LIBCPP_STD_VER >= 20

private:
  template <class>
  friend struct pointer_traits;
  template <class, size_t>
  friend struct __static_bounded_iter;
  __static_bounded_iter_storage<_Iterator, _Size> __storage_;

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX14 _Iterator& __current() _NOEXCEPT {
    return __storage_.__current();
  }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX14 _Iterator __current() const _NOEXCEPT {
    return __storage_.__current();
  }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX14 _Iterator __begin() const _NOEXCEPT {
    return __storage_.__begin();
  }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX14 _Iterator __end() const _NOEXCEPT { return __storage_.__end(); }
};

template <size_t _Size, class _It>
_LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR __static_bounded_iter<_It, _Size>
__make_static_bounded_iter(_It __it, _It __begin) {
  return __static_bounded_iter<_It, _Size>(std::move(__it), std::move(__begin));
}

#if _LIBCPP_STD_VER <= 17
template <class _Iterator, size_t _Size>
struct __libcpp_is_contiguous_iterator<__static_bounded_iter<_Iterator, _Size> > : true_type {};
#endif

template <class _Iterator, size_t _Size>
struct pointer_traits<__static_bounded_iter<_Iterator, _Size> > {
  using pointer         = __static_bounded_iter<_Iterator, _Size>;
  using element_type    = typename pointer_traits<_Iterator>::element_type;
  using difference_type = typename pointer_traits<_Iterator>::difference_type;

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR static element_type* to_address(pointer __it) _NOEXCEPT {
    return std::__to_address(__it.__current());
  }
};

_LIBCPP_END_NAMESPACE_STD

_LIBCPP_POP_MACROS

#endif // _LIBCPP___ITERATOR_STATIC_BOUNDED_ITER_H
