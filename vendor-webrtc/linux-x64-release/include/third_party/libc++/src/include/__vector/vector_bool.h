//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___VECTOR_VECTOR_BOOL_H
#define _LIBCPP___VECTOR_VECTOR_BOOL_H

#include <__algorithm/copy.h>
#include <__algorithm/copy_backward.h>
#include <__algorithm/fill_n.h>
#include <__algorithm/iterator_operations.h>
#include <__algorithm/max.h>
#include <__algorithm/rotate.h>
#include <__assert>
#include <__bit_reference>
#include <__config>
#include <__functional/unary_function.h>
#include <__fwd/bit_reference.h> // TODO: This is a workaround for https://github.com/llvm/llvm-project/issues/131814
#include <__fwd/functional.h>
#include <__fwd/vector.h>
#include <__iterator/distance.h>
#include <__iterator/iterator_traits.h>
#include <__iterator/reverse_iterator.h>
#include <__memory/addressof.h>
#include <__memory/allocate_at_least.h>
#include <__memory/allocator.h>
#include <__memory/allocator_traits.h>
#include <__memory/compressed_pair.h>
#include <__memory/construct_at.h>
#include <__memory/noexcept_move_assign_container.h>
#include <__memory/pointer_traits.h>
#include <__memory/swap_allocator.h>
#include <__ranges/access.h>
#include <__ranges/concepts.h>
#include <__ranges/container_compatible_range.h>
#include <__ranges/from_range.h>
#include <__type_traits/enable_if.h>
#include <__type_traits/is_constant_evaluated.h>
#include <__type_traits/is_nothrow_assignable.h>
#include <__type_traits/is_nothrow_constructible.h>
#include <__type_traits/type_identity.h>
#include <__utility/exception_guard.h>
#include <__utility/forward.h>
#include <__utility/move.h>
#include <__utility/swap.h>
#include <climits>
#include <initializer_list>
#include <limits>
#include <stdexcept>

// These headers define parts of vectors definition, since they define ADL functions or class specializations.
#include <__vector/comparison.h>
#include <__vector/container_traits.h>
#include <__vector/swap.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_PUSH_MACROS
#include <__undef_macros>

_LIBCPP_BEGIN_NAMESPACE_STD

template <class _Allocator>
struct hash<vector<bool, _Allocator> >;

template <class _Allocator>
struct __has_storage_type<vector<bool, _Allocator> > {
  static const bool value = true;
};

template <class _Allocator>
class vector<bool, _Allocator> {
public:
  typedef vector __self;
  typedef bool value_type;
  typedef _Allocator allocator_type;
  typedef allocator_traits<allocator_type> __alloc_traits;
  typedef typename __alloc_traits::size_type size_type;
  typedef typename __alloc_traits::difference_type difference_type;
  typedef size_type __storage_type;
  typedef __bit_iterator<vector, false> pointer;
  typedef __bit_iterator<vector, true> const_pointer;
  typedef pointer iterator;
  typedef const_pointer const_iterator;
  typedef std::reverse_iterator<iterator> reverse_iterator;
  typedef std::reverse_iterator<const_iterator> const_reverse_iterator;

private:
  typedef __rebind_alloc<__alloc_traits, __storage_type> __storage_allocator;
  typedef allocator_traits<__storage_allocator> __storage_traits;
  typedef typename __storage_traits::pointer __storage_pointer;
  typedef typename __storage_traits::const_pointer __const_storage_pointer;

  __storage_pointer __begin_;
  size_type __size_;
  _LIBCPP_COMPRESSED_PAIR(size_type, __cap_, __storage_allocator, __alloc_);

public:
  typedef __bit_reference<vector> reference;
#ifdef _LIBCPP_ABI_BITSET_VECTOR_BOOL_CONST_SUBSCRIPT_RETURN_BOOL
  using const_reference = bool;
#else
  typedef __bit_const_reference<vector> const_reference;
#endif

private:
  static const unsigned __bits_per_word = static_cast<unsigned>(sizeof(__storage_type) * CHAR_BIT);

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 static size_type
  __internal_cap_to_external(size_type __n) _NOEXCEPT {
    return __n * __bits_per_word;
  }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 static size_type
  __external_cap_to_internal(size_type __n) _NOEXCEPT {
    return __n > 0 ? (__n - 1) / __bits_per_word + 1 : size_type(0);
  }

public:
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 vector()
      _NOEXCEPT_(is_nothrow_default_constructible<allocator_type>::value);

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 explicit vector(const allocator_type& __a)
#if _LIBCPP_STD_VER <= 14
      _NOEXCEPT_(is_nothrow_copy_constructible<allocator_type>::value);
#else
      _NOEXCEPT;
#endif

private:
  class __destroy_vector {
  public:
    _LIBCPP_CONSTEXPR _LIBCPP_HIDE_FROM_ABI __destroy_vector(vector& __vec) : __vec_(__vec) {}

    _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void operator()() {
      if (__vec_.__begin_ != nullptr)
        __storage_traits::deallocate(__vec_.__alloc_, __vec_.__begin_, __vec_.__cap_);
    }

  private:
    vector& __vec_;
  };

public:
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 ~vector() { __destroy_vector (*this)(); }

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 explicit vector(size_type __n);
#if _LIBCPP_STD_VER >= 14
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 explicit vector(size_type __n, const allocator_type& __a);
#endif
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 vector(size_type __n, const value_type& __v);
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20
  vector(size_type __n, const value_type& __v, const allocator_type& __a);
  template <class _InputIterator, __enable_if_t<__has_exactly_input_iterator_category<_InputIterator>::value, int> = 0>
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 vector(_InputIterator __first, _InputIterator __last);
  template <class _InputIterator, __enable_if_t<__has_exactly_input_iterator_category<_InputIterator>::value, int> = 0>
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20
  vector(_InputIterator __first, _InputIterator __last, const allocator_type& __a);
  template <class _ForwardIterator, __enable_if_t<__has_forward_iterator_category<_ForwardIterator>::value, int> = 0>
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 vector(_ForwardIterator __first, _ForwardIterator __last);
  template <class _ForwardIterator, __enable_if_t<__has_forward_iterator_category<_ForwardIterator>::value, int> = 0>
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20
  vector(_ForwardIterator __first, _ForwardIterator __last, const allocator_type& __a);

#if _LIBCPP_STD_VER >= 23
  template <_ContainerCompatibleRange<bool> _Range>
  _LIBCPP_HIDE_FROM_ABI constexpr vector(from_range_t, _Range&& __range, const allocator_type& __a = allocator_type())
      : __begin_(nullptr), __size_(0), __cap_(0), __alloc_(static_cast<__storage_allocator>(__a)) {
    if constexpr (ranges::forward_range<_Range> || ranges::sized_range<_Range>) {
      auto __n = static_cast<size_type>(ranges::distance(__range));
      __init_with_size(ranges::begin(__range), ranges::end(__range), __n);

    } else {
      __init_with_sentinel(ranges::begin(__range), ranges::end(__range));
    }
  }
#endif

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 vector(const vector& __v);
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 vector(const vector& __v, const allocator_type& __a);
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 vector& operator=(const vector& __v);

#ifndef _LIBCPP_CXX03_LANG
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 vector(initializer_list<value_type> __il);
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20
  vector(initializer_list<value_type> __il, const allocator_type& __a);

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 vector& operator=(initializer_list<value_type> __il) {
    assign(__il.begin(), __il.end());
    return *this;
  }

#endif // !_LIBCPP_CXX03_LANG

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 vector(vector&& __v)
#if _LIBCPP_STD_VER >= 17
      noexcept;
#else
      _NOEXCEPT_(is_nothrow_move_constructible<allocator_type>::value);
#endif
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20
  vector(vector&& __v, const __type_identity_t<allocator_type>& __a);
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 vector& operator=(vector&& __v)
      _NOEXCEPT_(__noexcept_move_assign_container<_Allocator, __alloc_traits>::value);

  template <class _InputIterator, __enable_if_t<__has_exactly_input_iterator_category<_InputIterator>::value, int> = 0>
  void _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 assign(_InputIterator __first, _InputIterator __last);
  template <class _ForwardIterator, __enable_if_t<__has_forward_iterator_category<_ForwardIterator>::value, int> = 0>
  void _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 assign(_ForwardIterator __first, _ForwardIterator __last);

#if _LIBCPP_STD_VER >= 23
  template <_ContainerCompatibleRange<bool> _Range>
  _LIBCPP_HIDE_FROM_ABI constexpr void assign_range(_Range&& __range) {
    if constexpr (ranges::forward_range<_Range> || ranges::sized_range<_Range>) {
      auto __n = static_cast<size_type>(ranges::distance(__range));
      __assign_with_size(ranges::begin(__range), ranges::end(__range), __n);

    } else {
      __assign_with_sentinel(ranges::begin(__range), ranges::end(__range));
    }
  }
#endif

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 void assign(size_type __n, const value_type& __x);

#ifndef _LIBCPP_CXX03_LANG
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 void assign(initializer_list<value_type> __il) {
    assign(__il.begin(), __il.end());
  }
#endif

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 allocator_type get_allocator() const _NOEXCEPT {
    return allocator_type(this->__alloc_);
  }

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 size_type max_size() const _NOEXCEPT;
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 size_type capacity() const _NOEXCEPT {
    return __internal_cap_to_external(__cap_);
  }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 size_type size() const _NOEXCEPT { return __size_; }
  [[__nodiscard__]] _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 bool empty() const _NOEXCEPT {
    return __size_ == 0;
  }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 void reserve(size_type __n);
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 void shrink_to_fit() _NOEXCEPT;

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 iterator begin() _NOEXCEPT { return __make_iter(0); }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 const_iterator begin() const _NOEXCEPT { return __make_iter(0); }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 iterator end() _NOEXCEPT { return __make_iter(__size_); }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 const_iterator end() const _NOEXCEPT {
    return __make_iter(__size_);
  }

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 reverse_iterator rbegin() _NOEXCEPT {
    return reverse_iterator(end());
  }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 const_reverse_iterator rbegin() const _NOEXCEPT {
    return const_reverse_iterator(end());
  }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 reverse_iterator rend() _NOEXCEPT {
    return reverse_iterator(begin());
  }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 const_reverse_iterator rend() const _NOEXCEPT {
    return const_reverse_iterator(begin());
  }

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 const_iterator cbegin() const _NOEXCEPT { return __make_iter(0); }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 const_iterator cend() const _NOEXCEPT {
    return __make_iter(__size_);
  }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 const_reverse_iterator crbegin() const _NOEXCEPT {
    return rbegin();
  }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 const_reverse_iterator crend() const _NOEXCEPT { return rend(); }

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 reference operator[](size_type __n) {
    _LIBCPP_ASSERT_VALID_ELEMENT_ACCESS(__n < size(), "vector<bool>::operator[] index out of bounds");
    return __make_ref(__n);
  }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 const_reference operator[](size_type __n) const {
    _LIBCPP_ASSERT_VALID_ELEMENT_ACCESS(__n < size(), "vector<bool>::operator[] index out of bounds");
    return __make_ref(__n);
  }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 reference at(size_type __n);
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 const_reference at(size_type __n) const;

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 reference front() {
    _LIBCPP_ASSERT_VALID_ELEMENT_ACCESS(!empty(), "vector<bool>::front() called on an empty vector");
    return __make_ref(0);
  }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 const_reference front() const {
    _LIBCPP_ASSERT_VALID_ELEMENT_ACCESS(!empty(), "vector<bool>::front() called on an empty vector");
    return __make_ref(0);
  }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 reference back() {
    _LIBCPP_ASSERT_VALID_ELEMENT_ACCESS(!empty(), "vector<bool>::back() called on an empty vector");
    return __make_ref(__size_ - 1);
  }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 const_reference back() const {
    _LIBCPP_ASSERT_VALID_ELEMENT_ACCESS(!empty(), "vector<bool>::back() called on an empty vector");
    return __make_ref(__size_ - 1);
  }

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 void push_back(const value_type& __x);
#if _LIBCPP_STD_VER >= 14
  template <class... _Args>
#  if _LIBCPP_STD_VER >= 17
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 reference emplace_back(_Args&&... __args)
#  else
  _LIBCPP_HIDE_FROM_ABI void emplace_back(_Args&&... __args)
#  endif
  {
    push_back(value_type(std::forward<_Args>(__args)...));
#  if _LIBCPP_STD_VER >= 17
    return this->back();
#  endif
  }
#endif

#if _LIBCPP_STD_VER >= 23
  template <_ContainerCompatibleRange<bool> _Range>
  _LIBCPP_HIDE_FROM_ABI constexpr void append_range(_Range&& __range) {
    insert_range(end(), std::forward<_Range>(__range));
  }
#endif

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 void pop_back() {
    _LIBCPP_ASSERT_VALID_ELEMENT_ACCESS(!empty(), "vector<bool>::pop_back called on an empty vector");
    --__size_;
  }

#if _LIBCPP_STD_VER >= 14
  template <class... _Args>
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 iterator emplace(const_iterator __position, _Args&&... __args) {
    return insert(__position, value_type(std::forward<_Args>(__args)...));
  }
#endif

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 iterator insert(const_iterator __position, const value_type& __x);
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 iterator
  insert(const_iterator __position, size_type __n, const value_type& __x);
  template <class _InputIterator, __enable_if_t<__has_exactly_input_iterator_category<_InputIterator>::value, int> = 0>
  iterator _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20
  insert(const_iterator __position, _InputIterator __first, _InputIterator __last);
  template <class _ForwardIterator, __enable_if_t<__has_forward_iterator_category<_ForwardIterator>::value, int> = 0>
  iterator _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20
  insert(const_iterator __position, _ForwardIterator __first, _ForwardIterator __last);

#if _LIBCPP_STD_VER >= 23
  template <_ContainerCompatibleRange<bool> _Range>
  _LIBCPP_HIDE_FROM_ABI constexpr iterator insert_range(const_iterator __position, _Range&& __range) {
    if constexpr (ranges::forward_range<_Range> || ranges::sized_range<_Range>) {
      auto __n = static_cast<size_type>(ranges::distance(__range));
      return __insert_with_size(__position, ranges::begin(__range), ranges::end(__range), __n);

    } else {
      return __insert_with_sentinel(__position, ranges::begin(__range), ranges::end(__range));
    }
  }
#endif

#ifndef _LIBCPP_CXX03_LANG
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 iterator
  insert(const_iterator __position, initializer_list<value_type> __il) {
    return insert(__position, __il.begin(), __il.end());
  }
#endif

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 iterator erase(const_iterator __position);
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 iterator erase(const_iterator __first, const_iterator __last);

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 void clear() _NOEXCEPT { __size_ = 0; }

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 void swap(vector&)
#if _LIBCPP_STD_VER >= 14
      _NOEXCEPT;
#else
      _NOEXCEPT_(!__alloc_traits::propagate_on_container_swap::value || __is_nothrow_swappable_v<allocator_type>);
#endif
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 static void swap(reference __x, reference __y) _NOEXCEPT {
    std::swap(__x, __y);
  }

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 void resize(size_type __sz, value_type __x = false);
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 void flip() _NOEXCEPT;

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 bool __invariants() const;

private:
  [[__noreturn__]] _LIBCPP_HIDE_FROM_ABI static void __throw_length_error() { std::__throw_length_error("vector"); }

  [[__noreturn__]] _LIBCPP_HIDE_FROM_ABI static void __throw_out_of_range() { std::__throw_out_of_range("vector"); }

  template <class _InputIterator, class _Sentinel>
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 void
  __init_with_size(_InputIterator __first, _Sentinel __last, size_type __n) {
    auto __guard = std::__make_exception_guard(__destroy_vector(*this));

    if (__n > 0) {
      __vallocate(__n);
      __construct_at_end(std::move(__first), std::move(__last), __n);
    }

    __guard.__complete();
  }

  template <class _InputIterator, class _Sentinel>
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 void
  __init_with_sentinel(_InputIterator __first, _Sentinel __last) {
    auto __guard = std::__make_exception_guard(__destroy_vector(*this));

    for (; __first != __last; ++__first)
      push_back(*__first);

    __guard.__complete();
  }

  template <class _Iterator, class _Sentinel>
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void __assign_with_sentinel(_Iterator __first, _Sentinel __last);

  // The `_Iterator` in `*_with_size` functions can be input-only only if called from `*_range` (since C++23).
  // Otherwise, `_Iterator` is a forward iterator.

  template <class _Iterator, class _Sentinel>
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void
  __assign_with_size(_Iterator __first, _Sentinel __last, difference_type __ns);

  template <class _InputIterator, class _Sentinel>
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI iterator
  __insert_with_sentinel(const_iterator __position, _InputIterator __first, _Sentinel __last);

  template <class _Iterator, class _Sentinel>
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI iterator
  __insert_with_size(const_iterator __position, _Iterator __first, _Sentinel __last, difference_type __n);

  //  Allocate space for __n objects
  //  throws length_error if __n > max_size()
  //  throws (probably bad_alloc) if memory run out
  //  Precondition:  __begin_ == __end_ == __cap_ == nullptr
  //  Precondition:  __n > 0
  //  Postcondition:  capacity() >= __n
  //  Postcondition:  size() == 0
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 void __vallocate(size_type __n) {
    if (__n > max_size())
      this->__throw_length_error();
    auto __allocation = std::__allocate_at_least(__alloc_, __external_cap_to_internal(__n));
    __begin_          = __allocation.ptr;
    __size_           = 0;
    __cap_            = __allocation.count;
    if (__libcpp_is_constant_evaluated()) {
      for (size_type __i = 0; __i != __cap_; ++__i)
        std::__construct_at(std::__to_address(__begin_) + __i);
    }
  }

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 void __vdeallocate() _NOEXCEPT;
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 static size_type __align_it(size_type __new_size) _NOEXCEPT {
    return (__new_size + (__bits_per_word - 1)) & ~((size_type)__bits_per_word - 1);
  }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 size_type __recommend(size_type __new_size) const;
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 void __construct_at_end(size_type __n, bool __x);
  template <class _InputIterator, class _Sentinel>
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 void
  __construct_at_end(_InputIterator __first, _Sentinel __last, size_type __n);
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 reference __make_ref(size_type __pos) _NOEXCEPT {
    return reference(__begin_ + __pos / __bits_per_word, __storage_type(1) << __pos % __bits_per_word);
  }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 const_reference __make_ref(size_type __pos) const _NOEXCEPT {
    return __bit_const_reference<vector>(
        __begin_ + __pos / __bits_per_word, __storage_type(1) << __pos % __bits_per_word);
  }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 iterator __make_iter(size_type __pos) _NOEXCEPT {
    return iterator(__begin_ + __pos / __bits_per_word, static_cast<unsigned>(__pos % __bits_per_word));
  }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 const_iterator __make_iter(size_type __pos) const _NOEXCEPT {
    return const_iterator(__begin_ + __pos / __bits_per_word, static_cast<unsigned>(__pos % __bits_per_word));
  }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 iterator __const_iterator_cast(const_iterator __p) _NOEXCEPT {
    return begin() + (__p - cbegin());
  }

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 void __copy_assign_alloc(const vector& __v) {
    __copy_assign_alloc(
        __v, integral_constant<bool, __storage_traits::propagate_on_container_copy_assignment::value>());
  }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 void __copy_assign_alloc(const vector& __c, true_type) {
    if (__alloc_ != __c.__alloc_)
      __vdeallocate();
    __alloc_ = __c.__alloc_;
  }

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 void __copy_assign_alloc(const vector&, false_type) {}

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 void __move_assign(vector& __c, false_type);
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 void __move_assign(vector& __c, true_type)
      _NOEXCEPT_(is_nothrow_move_assignable<allocator_type>::value);
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 void __move_assign_alloc(vector& __c)
      _NOEXCEPT_(!__storage_traits::propagate_on_container_move_assignment::value ||
                 is_nothrow_move_assignable<allocator_type>::value) {
    __move_assign_alloc(
        __c, integral_constant<bool, __storage_traits::propagate_on_container_move_assignment::value>());
  }
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 void __move_assign_alloc(vector& __c, true_type)
      _NOEXCEPT_(is_nothrow_move_assignable<allocator_type>::value) {
    __alloc_ = std::move(__c.__alloc_);
  }

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 void __move_assign_alloc(vector&, false_type) _NOEXCEPT {}

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 size_t __hash_code() const _NOEXCEPT;

  friend class __bit_reference<vector>;
  friend class __bit_const_reference<vector>;
  friend class __bit_iterator<vector, false>;
  friend class __bit_iterator<vector, true>;
  friend struct __bit_array<vector>;
  friend struct hash<vector>;
};

template <class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 void vector<bool, _Allocator>::__vdeallocate() _NOEXCEPT {
  if (this->__begin_ != nullptr) {
    __storage_traits::deallocate(this->__alloc_, this->__begin_, __cap_);
    this->__begin_ = nullptr;
    this->__size_ = this->__cap_ = 0;
  }
}

template <class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 typename vector<bool, _Allocator>::size_type
vector<bool, _Allocator>::max_size() const _NOEXCEPT {
  size_type __amax = __storage_traits::max_size(__alloc_);
  size_type __nmax = numeric_limits<difference_type>::max();
  return __nmax / __bits_per_word <= __amax ? __nmax : __internal_cap_to_external(__amax);
}

//  Precondition:  __new_size > capacity()
template <class _Allocator>
inline _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 typename vector<bool, _Allocator>::size_type
vector<bool, _Allocator>::__recommend(size_type __new_size) const {
  const size_type __ms = max_size();
  if (__new_size > __ms)
    this->__throw_length_error();
  const size_type __cap = capacity();
  if (__cap >= __ms / 2)
    return __ms;
  return std::max<size_type>(2 * __cap, __align_it(__new_size));
}

//  Default constructs __n objects starting at __end_
//  Precondition:  size() + __n <= capacity()
//  Postcondition:  size() == size() + __n
template <class _Allocator>
inline _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 void
vector<bool, _Allocator>::__construct_at_end(size_type __n, bool __x) {
  _LIBCPP_ASSERT_INTERNAL(
      capacity() >= size() + __n, "vector<bool>::__construct_at_end called with insufficient capacity");
  std::fill_n(end(), __n, __x);
  this->__size_ += __n;
  if (end().__ctz_ != 0) // Ensure uninitialized leading bits in the last word are set to zero
    std::fill_n(end(), __bits_per_word - end().__ctz_, 0);
}

template <class _Allocator>
template <class _InputIterator, class _Sentinel>
_LIBCPP_CONSTEXPR_SINCE_CXX20 void
vector<bool, _Allocator>::__construct_at_end(_InputIterator __first, _Sentinel __last, size_type __n) {
  _LIBCPP_ASSERT_INTERNAL(
      capacity() >= size() + __n, "vector<bool>::__construct_at_end called with insufficient capacity");
  std::__copy(std::move(__first), std::move(__last), end());
  this->__size_ += __n;
  if (end().__ctz_ != 0) // Ensure uninitialized leading bits in the last word are set to zero
    std::fill_n(end(), __bits_per_word - end().__ctz_, 0);
}

template <class _Allocator>
inline _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 vector<bool, _Allocator>::vector()
    _NOEXCEPT_(is_nothrow_default_constructible<allocator_type>::value)
    : __begin_(nullptr), __size_(0), __cap_(0) {}

template <class _Allocator>
inline _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 vector<bool, _Allocator>::vector(const allocator_type& __a)
#if _LIBCPP_STD_VER <= 14
    _NOEXCEPT_(is_nothrow_copy_constructible<allocator_type>::value)
#else
        _NOEXCEPT
#endif
    : __begin_(nullptr), __size_(0), __cap_(0), __alloc_(static_cast<__storage_allocator>(__a)) {
}

template <class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 vector<bool, _Allocator>::vector(size_type __n)
    : __begin_(nullptr), __size_(0), __cap_(0) {
  if (__n > 0) {
    __vallocate(__n);
    __construct_at_end(__n, false);
  }
}

#if _LIBCPP_STD_VER >= 14
template <class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 vector<bool, _Allocator>::vector(size_type __n, const allocator_type& __a)
    : __begin_(nullptr), __size_(0), __cap_(0), __alloc_(static_cast<__storage_allocator>(__a)) {
  if (__n > 0) {
    __vallocate(__n);
    __construct_at_end(__n, false);
  }
}
#endif

template <class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 vector<bool, _Allocator>::vector(size_type __n, const value_type& __x)
    : __begin_(nullptr), __size_(0), __cap_(0) {
  if (__n > 0) {
    __vallocate(__n);
    __construct_at_end(__n, __x);
  }
}

template <class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20
vector<bool, _Allocator>::vector(size_type __n, const value_type& __x, const allocator_type& __a)
    : __begin_(nullptr), __size_(0), __cap_(0), __alloc_(static_cast<__storage_allocator>(__a)) {
  if (__n > 0) {
    __vallocate(__n);
    __construct_at_end(__n, __x);
  }
}

template <class _Allocator>
template <class _InputIterator, __enable_if_t<__has_exactly_input_iterator_category<_InputIterator>::value, int> >
_LIBCPP_CONSTEXPR_SINCE_CXX20 vector<bool, _Allocator>::vector(_InputIterator __first, _InputIterator __last)
    : __begin_(nullptr), __size_(0), __cap_(0) {
  __init_with_sentinel(__first, __last);
}

template <class _Allocator>
template <class _InputIterator, __enable_if_t<__has_exactly_input_iterator_category<_InputIterator>::value, int> >
_LIBCPP_CONSTEXPR_SINCE_CXX20
vector<bool, _Allocator>::vector(_InputIterator __first, _InputIterator __last, const allocator_type& __a)
    : __begin_(nullptr), __size_(0), __cap_(0), __alloc_(static_cast<__storage_allocator>(__a)) {
  __init_with_sentinel(__first, __last);
}

template <class _Allocator>
template <class _ForwardIterator, __enable_if_t<__has_forward_iterator_category<_ForwardIterator>::value, int> >
_LIBCPP_CONSTEXPR_SINCE_CXX20 vector<bool, _Allocator>::vector(_ForwardIterator __first, _ForwardIterator __last)
    : __begin_(nullptr), __size_(0), __cap_(0) {
  auto __n = static_cast<size_type>(std::distance(__first, __last));
  __init_with_size(__first, __last, __n);
}

template <class _Allocator>
template <class _ForwardIterator, __enable_if_t<__has_forward_iterator_category<_ForwardIterator>::value, int> >
_LIBCPP_CONSTEXPR_SINCE_CXX20
vector<bool, _Allocator>::vector(_ForwardIterator __first, _ForwardIterator __last, const allocator_type& __a)
    : __begin_(nullptr), __size_(0), __cap_(0), __alloc_(static_cast<__storage_allocator>(__a)) {
  auto __n = static_cast<size_type>(std::distance(__first, __last));
  __init_with_size(__first, __last, __n);
}

#ifndef _LIBCPP_CXX03_LANG

template <class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 vector<bool, _Allocator>::vector(initializer_list<value_type> __il)
    : __begin_(nullptr), __size_(0), __cap_(0) {
  size_type __n = static_cast<size_type>(__il.size());
  if (__n > 0) {
    __vallocate(__n);
    __construct_at_end(__il.begin(), __il.end(), __n);
  }
}

template <class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20
vector<bool, _Allocator>::vector(initializer_list<value_type> __il, const allocator_type& __a)
    : __begin_(nullptr), __size_(0), __cap_(0), __alloc_(static_cast<__storage_allocator>(__a)) {
  size_type __n = static_cast<size_type>(__il.size());
  if (__n > 0) {
    __vallocate(__n);
    __construct_at_end(__il.begin(), __il.end(), __n);
  }
}

#endif // _LIBCPP_CXX03_LANG

template <class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 vector<bool, _Allocator>::vector(const vector& __v)
    : __begin_(nullptr),
      __size_(0),
      __cap_(0),
      __alloc_(__storage_traits::select_on_container_copy_construction(__v.__alloc_)) {
  if (__v.size() > 0) {
    __vallocate(__v.size());
    __construct_at_end(__v.begin(), __v.end(), __v.size());
  }
}

template <class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 vector<bool, _Allocator>::vector(const vector& __v, const allocator_type& __a)
    : __begin_(nullptr), __size_(0), __cap_(0), __alloc_(__a) {
  if (__v.size() > 0) {
    __vallocate(__v.size());
    __construct_at_end(__v.begin(), __v.end(), __v.size());
  }
}

template <class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 vector<bool, _Allocator>& vector<bool, _Allocator>::operator=(const vector& __v) {
  if (this != std::addressof(__v)) {
    __copy_assign_alloc(__v);
    if (__v.__size_) {
      if (__v.__size_ > capacity()) {
        __vdeallocate();
        __vallocate(__v.__size_);
      }
      std::copy(__v.__begin_, __v.__begin_ + __external_cap_to_internal(__v.__size_), __begin_);
    }
    __size_ = __v.__size_;
  }
  return *this;
}

template <class _Allocator>
inline _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 vector<bool, _Allocator>::vector(vector&& __v)
#if _LIBCPP_STD_VER >= 17
    _NOEXCEPT
#else
    _NOEXCEPT_(is_nothrow_move_constructible<allocator_type>::value)
#endif
    : __begin_(__v.__begin_),
      __size_(__v.__size_),
      __cap_(__v.__cap_),
      __alloc_(std::move(__v.__alloc_)) {
  __v.__begin_ = nullptr;
  __v.__size_  = 0;
  __v.__cap_   = 0;
}

template <class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20
vector<bool, _Allocator>::vector(vector&& __v, const __type_identity_t<allocator_type>& __a)
    : __begin_(nullptr), __size_(0), __cap_(0), __alloc_(__a) {
  if (__a == allocator_type(__v.__alloc_)) {
    this->__begin_ = __v.__begin_;
    this->__size_  = __v.__size_;
    this->__cap_   = __v.__cap_;
    __v.__begin_   = nullptr;
    __v.__cap_ = __v.__size_ = 0;
  } else if (__v.size() > 0) {
    __vallocate(__v.size());
    __construct_at_end(__v.begin(), __v.end(), __v.size());
  }
}

template <class _Allocator>
inline _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 vector<bool, _Allocator>&
vector<bool, _Allocator>::operator=(vector&& __v)
    _NOEXCEPT_(__noexcept_move_assign_container<_Allocator, __alloc_traits>::value) {
  __move_assign(__v, integral_constant<bool, __storage_traits::propagate_on_container_move_assignment::value>());
  return *this;
}

template <class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 void vector<bool, _Allocator>::__move_assign(vector& __c, false_type) {
  if (__alloc_ != __c.__alloc_)
    assign(__c.begin(), __c.end());
  else
    __move_assign(__c, true_type());
}

template <class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 void vector<bool, _Allocator>::__move_assign(vector& __c, true_type)
    _NOEXCEPT_(is_nothrow_move_assignable<allocator_type>::value) {
  __vdeallocate();
  __move_assign_alloc(__c);
  this->__begin_ = __c.__begin_;
  this->__size_  = __c.__size_;
  this->__cap_   = __c.__cap_;
  __c.__begin_   = nullptr;
  __c.__cap_ = __c.__size_ = 0;
}

template <class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 void vector<bool, _Allocator>::assign(size_type __n, const value_type& __x) {
  __size_ = 0;
  if (__n > 0) {
    size_type __c = capacity();
    if (__n <= __c)
      __size_ = __n;
    else {
      vector __v(get_allocator());
      __v.reserve(__recommend(__n));
      __v.__size_ = __n;
      swap(__v);
    }
    std::fill_n(begin(), __n, __x);
  }
}

template <class _Allocator>
template <class _InputIterator, __enable_if_t<__has_exactly_input_iterator_category<_InputIterator>::value, int> >
_LIBCPP_CONSTEXPR_SINCE_CXX20 void vector<bool, _Allocator>::assign(_InputIterator __first, _InputIterator __last) {
  __assign_with_sentinel(__first, __last);
}

template <class _Allocator>
template <class _Iterator, class _Sentinel>
_LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void
vector<bool, _Allocator>::__assign_with_sentinel(_Iterator __first, _Sentinel __last) {
  clear();
  for (; __first != __last; ++__first)
    push_back(*__first);
}

template <class _Allocator>
template <class _ForwardIterator, __enable_if_t<__has_forward_iterator_category<_ForwardIterator>::value, int> >
_LIBCPP_CONSTEXPR_SINCE_CXX20 void vector<bool, _Allocator>::assign(_ForwardIterator __first, _ForwardIterator __last) {
  __assign_with_size(__first, __last, std::distance(__first, __last));
}

template <class _Allocator>
template <class _Iterator, class _Sentinel>
_LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void
vector<bool, _Allocator>::__assign_with_size(_Iterator __first, _Sentinel __last, difference_type __ns) {
  _LIBCPP_ASSERT_VALID_INPUT_RANGE(__ns >= 0, "invalid range specified");

  clear();

  const size_t __n = static_cast<size_type>(__ns);
  if (__n) {
    if (__n > capacity()) {
      __vdeallocate();
      __vallocate(__n);
    }
    __construct_at_end(std::move(__first), std::move(__last), __n);
  }
}

template <class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 void vector<bool, _Allocator>::reserve(size_type __n) {
  if (__n > capacity()) {
    if (__n > max_size())
      this->__throw_length_error();
    vector __v(this->get_allocator());
    __v.__vallocate(__n);
    __v.__construct_at_end(this->begin(), this->end(), this->size());
    swap(__v);
  }
}

template <class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 void vector<bool, _Allocator>::shrink_to_fit() _NOEXCEPT {
  if (__external_cap_to_internal(size()) < __cap_) {
#if _LIBCPP_HAS_EXCEPTIONS
    try {
#endif // _LIBCPP_HAS_EXCEPTIONS
      vector __v(*this, allocator_type(__alloc_));
      if (__v.__cap_ < __cap_)
        __v.swap(*this);
#if _LIBCPP_HAS_EXCEPTIONS
    } catch (...) {
    }
#endif // _LIBCPP_HAS_EXCEPTIONS
  }
}

template <class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 typename vector<bool, _Allocator>::reference vector<bool, _Allocator>::at(size_type __n) {
  if (__n >= size())
    this->__throw_out_of_range();
  return (*this)[__n];
}

template <class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 typename vector<bool, _Allocator>::const_reference
vector<bool, _Allocator>::at(size_type __n) const {
  if (__n >= size())
    this->__throw_out_of_range();
  return (*this)[__n];
}

template <class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 void vector<bool, _Allocator>::push_back(const value_type& __x) {
  if (this->__size_ == this->capacity())
    reserve(__recommend(this->__size_ + 1));
  ++this->__size_;
  back() = __x;
}

template <class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 typename vector<bool, _Allocator>::iterator
vector<bool, _Allocator>::insert(const_iterator __position, const value_type& __x) {
  iterator __r;
  if (size() < capacity()) {
    const_iterator __old_end = end();
    ++__size_;
    std::copy_backward(__position, __old_end, end());
    __r = __const_iterator_cast(__position);
  } else {
    vector __v(get_allocator());
    __v.reserve(__recommend(__size_ + 1));
    __v.__size_ = __size_ + 1;
    __r         = std::copy(cbegin(), __position, __v.begin());
    std::copy_backward(__position, cend(), __v.end());
    swap(__v);
  }
  *__r = __x;
  return __r;
}

template <class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 typename vector<bool, _Allocator>::iterator
vector<bool, _Allocator>::insert(const_iterator __position, size_type __n, const value_type& __x) {
  iterator __r;
  size_type __c = capacity();
  if (__n <= __c && size() <= __c - __n) {
    const_iterator __old_end = end();
    __size_ += __n;
    std::copy_backward(__position, __old_end, end());
    __r = __const_iterator_cast(__position);
  } else {
    vector __v(get_allocator());
    __v.reserve(__recommend(__size_ + __n));
    __v.__size_ = __size_ + __n;
    __r         = std::copy(cbegin(), __position, __v.begin());
    std::copy_backward(__position, cend(), __v.end());
    swap(__v);
  }
  std::fill_n(__r, __n, __x);
  return __r;
}

template <class _Allocator>
template <class _InputIterator, __enable_if_t<__has_exactly_input_iterator_category<_InputIterator>::value, int> >
_LIBCPP_CONSTEXPR_SINCE_CXX20 typename vector<bool, _Allocator>::iterator
vector<bool, _Allocator>::insert(const_iterator __position, _InputIterator __first, _InputIterator __last) {
  return __insert_with_sentinel(__position, __first, __last);
}

template <class _Allocator>
template <class _InputIterator, class _Sentinel>
_LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI typename vector<bool, _Allocator>::iterator
vector<bool, _Allocator>::__insert_with_sentinel(const_iterator __position, _InputIterator __first, _Sentinel __last) {
  difference_type __off = __position - begin();
  iterator __p          = __const_iterator_cast(__position);
  iterator __old_end    = end();
  for (; size() != capacity() && __first != __last; ++__first) {
    ++this->__size_;
    back() = *__first;
  }
  vector __v(get_allocator());
  if (__first != __last) {
#if _LIBCPP_HAS_EXCEPTIONS
    try {
#endif // _LIBCPP_HAS_EXCEPTIONS
      __v.__assign_with_sentinel(std::move(__first), std::move(__last));
      difference_type __old_size = static_cast<difference_type>(__old_end - begin());
      difference_type __old_p    = __p - begin();
      reserve(__recommend(size() + __v.size()));
      __p       = begin() + __old_p;
      __old_end = begin() + __old_size;
#if _LIBCPP_HAS_EXCEPTIONS
    } catch (...) {
      erase(__old_end, end());
      throw;
    }
#endif // _LIBCPP_HAS_EXCEPTIONS
  }
  __p = std::rotate(__p, __old_end, end());
  insert(__p, __v.begin(), __v.end());
  return begin() + __off;
}

template <class _Allocator>
template <class _ForwardIterator, __enable_if_t<__has_forward_iterator_category<_ForwardIterator>::value, int> >
_LIBCPP_CONSTEXPR_SINCE_CXX20 typename vector<bool, _Allocator>::iterator
vector<bool, _Allocator>::insert(const_iterator __position, _ForwardIterator __first, _ForwardIterator __last) {
  return __insert_with_size(__position, __first, __last, std::distance(__first, __last));
}

template <class _Allocator>
template <class _Iterator, class _Sentinel>
_LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI typename vector<bool, _Allocator>::iterator
vector<bool, _Allocator>::__insert_with_size(
    const_iterator __position, _Iterator __first, _Sentinel __last, difference_type __n_signed) {
  _LIBCPP_ASSERT_VALID_INPUT_RANGE(__n_signed >= 0, "invalid range specified");
  const size_type __n = static_cast<size_type>(__n_signed);
  iterator __r;
  size_type __c = capacity();
  if (__n <= __c && size() <= __c - __n) {
    const_iterator __old_end = end();
    __size_ += __n;
    std::copy_backward(__position, __old_end, end());
    __r = __const_iterator_cast(__position);
  } else {
    vector __v(get_allocator());
    __v.reserve(__recommend(__size_ + __n));
    __v.__size_ = __size_ + __n;
    __r         = std::copy(cbegin(), __position, __v.begin());
    std::copy_backward(__position, cend(), __v.end());
    swap(__v);
  }
  std::__copy(std::move(__first), std::move(__last), __r);
  return __r;
}

template <class _Allocator>
inline _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 typename vector<bool, _Allocator>::iterator
vector<bool, _Allocator>::erase(const_iterator __position) {
  _LIBCPP_ASSERT_VALID_ELEMENT_ACCESS(
      __position != end(), "vector<bool>::erase(iterator) called with a non-dereferenceable iterator");
  iterator __r = __const_iterator_cast(__position);
  std::copy(__position + 1, this->cend(), __r);
  --__size_;
  return __r;
}

template <class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 typename vector<bool, _Allocator>::iterator
vector<bool, _Allocator>::erase(const_iterator __first, const_iterator __last) {
  _LIBCPP_ASSERT_VALID_INPUT_RANGE(
      __first <= __last, "vector<bool>::erase(iterator, iterator) called with an invalid range");
  iterator __r        = __const_iterator_cast(__first);
  difference_type __d = __last - __first;
  std::copy(__last, this->cend(), __r);
  __size_ -= __d;
  return __r;
}

template <class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 void vector<bool, _Allocator>::swap(vector& __x)
#if _LIBCPP_STD_VER >= 14
    _NOEXCEPT
#else
    _NOEXCEPT_(!__alloc_traits::propagate_on_container_swap::value || __is_nothrow_swappable_v<allocator_type>)
#endif
{
  std::swap(this->__begin_, __x.__begin_);
  std::swap(this->__size_, __x.__size_);
  std::swap(this->__cap_, __x.__cap_);
  std::__swap_allocator(this->__alloc_, __x.__alloc_);
}

template <class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 void vector<bool, _Allocator>::resize(size_type __sz, value_type __x) {
  size_type __cs = size();
  if (__cs < __sz) {
    iterator __r;
    size_type __c = capacity();
    size_type __n = __sz - __cs;
    if (__n <= __c && __cs <= __c - __n) {
      __r = end();
      __size_ += __n;
    } else {
      vector __v(get_allocator());
      __v.reserve(__recommend(__size_ + __n));
      __v.__size_ = __size_ + __n;
      __r         = std::copy(cbegin(), cend(), __v.begin());
      swap(__v);
    }
    std::fill_n(__r, __n, __x);
  } else
    __size_ = __sz;
}

template <class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 void vector<bool, _Allocator>::flip() _NOEXCEPT {
  // Flip each storage word entirely, including the last potentially partial word.
  // The unused bits in the last word are safe to flip as they won't be accessed.
  __storage_pointer __p = __begin_;
  for (size_type __n = __external_cap_to_internal(size()); __n != 0; ++__p, --__n)
    *__p = ~*__p;
}

template <class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 bool vector<bool, _Allocator>::__invariants() const {
  if (this->__begin_ == nullptr) {
    if (this->__size_ != 0 || this->__cap_ != 0)
      return false;
  } else {
    if (this->__cap_ == 0)
      return false;
    if (this->__size_ > this->capacity())
      return false;
  }
  return true;
}

template <class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 size_t vector<bool, _Allocator>::__hash_code() const _NOEXCEPT {
  size_t __h = 0;
  // do middle whole words
  size_type __n         = __size_;
  __storage_pointer __p = __begin_;
  for (; __n >= __bits_per_word; ++__p, __n -= __bits_per_word)
    __h ^= *__p;
  // do last partial word
  if (__n > 0) {
    const __storage_type __m = ~__storage_type(0) >> (__bits_per_word - __n);
    __h ^= *__p & __m;
  }
  return __h;
}

template <class _Allocator>
struct hash<vector<bool, _Allocator> > : public __unary_function<vector<bool, _Allocator>, size_t> {
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 size_t
  operator()(const vector<bool, _Allocator>& __vec) const _NOEXCEPT {
    return __vec.__hash_code();
  }
};

_LIBCPP_END_NAMESPACE_STD

_LIBCPP_POP_MACROS

#endif // _LIBCPP___VECTOR_VECTOR_BOOL_H
