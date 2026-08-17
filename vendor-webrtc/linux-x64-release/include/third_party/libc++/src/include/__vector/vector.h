//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___VECTOR_VECTOR_H
#define _LIBCPP___VECTOR_VECTOR_H

#include <__algorithm/copy.h>
#include <__algorithm/copy_n.h>
#include <__algorithm/fill_n.h>
#include <__algorithm/max.h>
#include <__algorithm/min.h>
#include <__algorithm/move.h>
#include <__algorithm/move_backward.h>
#include <__algorithm/ranges_copy_n.h>
#include <__algorithm/rotate.h>
#include <__assert>
#include <__config>
#include <__debug_utils/sanitizers.h>
#include <__format/enable_insertable.h>
#include <__fwd/vector.h>
#include <__iterator/advance.h>
#include <__iterator/bounded_iter.h>
#include <__iterator/concepts.h>
#include <__iterator/distance.h>
#include <__iterator/iterator_traits.h>
#include <__iterator/move_iterator.h>
#include <__iterator/next.h>
#include <__iterator/reverse_iterator.h>
#include <__iterator/wrap_iter.h>
#include <__memory/addressof.h>
#include <__memory/allocate_at_least.h>
#include <__memory/allocator.h>
#include <__memory/allocator_traits.h>
#include <__memory/compressed_pair.h>
#include <__memory/noexcept_move_assign_container.h>
#include <__memory/pointer_traits.h>
#include <__memory/swap_allocator.h>
#include <__memory/temp_value.h>
#include <__memory/uninitialized_algorithms.h>
#include <__ranges/access.h>
#include <__ranges/concepts.h>
#include <__ranges/container_compatible_range.h>
#include <__ranges/from_range.h>
#include <__split_buffer>
#include <__type_traits/conditional.h>
#include <__type_traits/enable_if.h>
#include <__type_traits/is_allocator.h>
#include <__type_traits/is_constant_evaluated.h>
#include <__type_traits/is_constructible.h>
#include <__type_traits/is_nothrow_assignable.h>
#include <__type_traits/is_nothrow_constructible.h>
#include <__type_traits/is_pointer.h>
#include <__type_traits/is_same.h>
#include <__type_traits/is_trivially_relocatable.h>
#include <__type_traits/type_identity.h>
#include <__utility/declval.h>
#include <__utility/exception_guard.h>
#include <__utility/forward.h>
#include <__utility/is_pointer_in_range.h>
#include <__utility/move.h>
#include <__utility/pair.h>
#include <__utility/swap.h>
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

template <class _Tp, class _Allocator /* = allocator<_Tp> */>
class vector {
public:
  //
  // Types
  //
  typedef vector __self;
  typedef _Tp value_type;
  typedef _Allocator allocator_type;
  typedef allocator_traits<allocator_type> __alloc_traits;
  typedef value_type& reference;
  typedef const value_type& const_reference;
  typedef typename __alloc_traits::size_type size_type;
  typedef typename __alloc_traits::difference_type difference_type;
  typedef typename __alloc_traits::pointer pointer;
  typedef typename __alloc_traits::const_pointer const_pointer;
#ifdef _LIBCPP_ABI_BOUNDED_ITERATORS_IN_VECTOR
  // Users might provide custom allocators, and prior to C++20 we have no existing way to detect whether the allocator's
  // pointer type is contiguous (though it has to be by the Standard). Using the wrapper type ensures the iterator is
  // considered contiguous.
  typedef __bounded_iter<__wrap_iter<pointer> > iterator;
  typedef __bounded_iter<__wrap_iter<const_pointer> > const_iterator;
#else
  typedef __wrap_iter<pointer> iterator;
  typedef __wrap_iter<const_pointer> const_iterator;
#endif
  typedef std::reverse_iterator<iterator> reverse_iterator;
  typedef std::reverse_iterator<const_iterator> const_reverse_iterator;

  // A vector containers the following members which may be trivially relocatable:
  // - pointer: may be trivially relocatable, so it's checked
  // - allocator_type: may be trivially relocatable, so it's checked
  // vector doesn't contain any self-references, so it's trivially relocatable if its members are.
  using __trivially_relocatable _LIBCPP_NODEBUG = __conditional_t<
      __libcpp_is_trivially_relocatable<pointer>::value && __libcpp_is_trivially_relocatable<allocator_type>::value,
      vector,
      void>;

  static_assert(__check_valid_allocator<allocator_type>::value, "");
  static_assert(is_same<typename allocator_type::value_type, value_type>::value,
                "Allocator::value_type must be same type as value_type");

  //
  // [vector.cons], construct/copy/destroy
  //
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI vector()
      _NOEXCEPT_(is_nothrow_default_constructible<allocator_type>::value) {}
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI explicit vector(const allocator_type& __a)
#if _LIBCPP_STD_VER <= 14
      _NOEXCEPT_(is_nothrow_copy_constructible<allocator_type>::value)
#else
      noexcept
#endif
      : __alloc_(__a) {
  }

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI explicit vector(size_type __n) {
    auto __guard = std::__make_exception_guard(__destroy_vector(*this));
    if (__n > 0) {
      __vallocate(__n);
      __construct_at_end(__n);
    }
    __guard.__complete();
  }

#if _LIBCPP_STD_VER >= 14
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI explicit vector(size_type __n, const allocator_type& __a)
      : __alloc_(__a) {
    auto __guard = std::__make_exception_guard(__destroy_vector(*this));
    if (__n > 0) {
      __vallocate(__n);
      __construct_at_end(__n);
    }
    __guard.__complete();
  }
#endif

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI vector(size_type __n, const value_type& __x) {
    auto __guard = std::__make_exception_guard(__destroy_vector(*this));
    if (__n > 0) {
      __vallocate(__n);
      __construct_at_end(__n, __x);
    }
    __guard.__complete();
  }

  template <__enable_if_t<__is_allocator<_Allocator>::value, int> = 0>
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI
  vector(size_type __n, const value_type& __x, const allocator_type& __a)
      : __alloc_(__a) {
    auto __guard = std::__make_exception_guard(__destroy_vector(*this));
    if (__n > 0) {
      __vallocate(__n);
      __construct_at_end(__n, __x);
    }
    __guard.__complete();
  }

  template <class _InputIterator,
            __enable_if_t<__has_exactly_input_iterator_category<_InputIterator>::value &&
                              is_constructible<value_type, typename iterator_traits<_InputIterator>::reference>::value,
                          int> = 0>
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI vector(_InputIterator __first, _InputIterator __last) {
    __init_with_sentinel(__first, __last);
  }

  template <class _InputIterator,
            __enable_if_t<__has_exactly_input_iterator_category<_InputIterator>::value &&
                              is_constructible<value_type, typename iterator_traits<_InputIterator>::reference>::value,
                          int> = 0>
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI
  vector(_InputIterator __first, _InputIterator __last, const allocator_type& __a)
      : __alloc_(__a) {
    __init_with_sentinel(__first, __last);
  }

  template <
      class _ForwardIterator,
      __enable_if_t<__has_forward_iterator_category<_ForwardIterator>::value &&
                        is_constructible<value_type, typename iterator_traits<_ForwardIterator>::reference>::value,
                    int> = 0>
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI vector(_ForwardIterator __first, _ForwardIterator __last) {
    size_type __n = static_cast<size_type>(std::distance(__first, __last));
    __init_with_size(__first, __last, __n);
  }

  template <
      class _ForwardIterator,
      __enable_if_t<__has_forward_iterator_category<_ForwardIterator>::value &&
                        is_constructible<value_type, typename iterator_traits<_ForwardIterator>::reference>::value,
                    int> = 0>
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI
  vector(_ForwardIterator __first, _ForwardIterator __last, const allocator_type& __a)
      : __alloc_(__a) {
    size_type __n = static_cast<size_type>(std::distance(__first, __last));
    __init_with_size(__first, __last, __n);
  }

#if _LIBCPP_STD_VER >= 23
  template <_ContainerCompatibleRange<_Tp> _Range>
  _LIBCPP_HIDE_FROM_ABI constexpr vector(
      from_range_t, _Range&& __range, const allocator_type& __alloc = allocator_type())
      : __alloc_(__alloc) {
    if constexpr (ranges::forward_range<_Range> || ranges::sized_range<_Range>) {
      auto __n = static_cast<size_type>(ranges::distance(__range));
      __init_with_size(ranges::begin(__range), ranges::end(__range), __n);

    } else {
      __init_with_sentinel(ranges::begin(__range), ranges::end(__range));
    }
  }
#endif

private:
  class __destroy_vector {
  public:
    _LIBCPP_CONSTEXPR _LIBCPP_HIDE_FROM_ABI __destroy_vector(vector& __vec) : __vec_(__vec) {}

    _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void operator()() {
      if (__vec_.__begin_ != nullptr) {
        __vec_.clear();
        __vec_.__annotate_delete();
        __alloc_traits::deallocate(__vec_.__alloc_, __vec_.__begin_, __vec_.capacity());
      }
    }

  private:
    vector& __vec_;
  };

public:
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI ~vector() { __destroy_vector (*this)(); }

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI vector(const vector& __x)
      : __alloc_(__alloc_traits::select_on_container_copy_construction(__x.__alloc_)) {
    __init_with_size(__x.__begin_, __x.__end_, __x.size());
  }
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI
  vector(const vector& __x, const __type_identity_t<allocator_type>& __a)
      : __alloc_(__a) {
    __init_with_size(__x.__begin_, __x.__end_, __x.size());
  }
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI vector& operator=(const vector& __x);

#ifndef _LIBCPP_CXX03_LANG
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI vector(initializer_list<value_type> __il) {
    __init_with_size(__il.begin(), __il.end(), __il.size());
  }

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI
  vector(initializer_list<value_type> __il, const allocator_type& __a)
      : __alloc_(__a) {
    __init_with_size(__il.begin(), __il.end(), __il.size());
  }

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI vector& operator=(initializer_list<value_type> __il) {
    assign(__il.begin(), __il.end());
    return *this;
  }
#endif // !_LIBCPP_CXX03_LANG

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI vector(vector&& __x)
#if _LIBCPP_STD_VER >= 17
      noexcept;
#else
      _NOEXCEPT_(is_nothrow_move_constructible<allocator_type>::value);
#endif

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI
  vector(vector&& __x, const __type_identity_t<allocator_type>& __a);
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI vector& operator=(vector&& __x)
      _NOEXCEPT_(__noexcept_move_assign_container<_Allocator, __alloc_traits>::value) {
    __move_assign(__x, integral_constant<bool, __alloc_traits::propagate_on_container_move_assignment::value>());
    return *this;
  }

  template <class _InputIterator,
            __enable_if_t<__has_exactly_input_iterator_category<_InputIterator>::value &&
                              is_constructible<value_type, typename iterator_traits<_InputIterator>::reference>::value,
                          int> = 0>
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void assign(_InputIterator __first, _InputIterator __last) {
    __assign_with_sentinel(__first, __last);
  }
  template <
      class _ForwardIterator,
      __enable_if_t<__has_forward_iterator_category<_ForwardIterator>::value &&
                        is_constructible<value_type, typename iterator_traits<_ForwardIterator>::reference>::value,
                    int> = 0>
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void assign(_ForwardIterator __first, _ForwardIterator __last) {
    __assign_with_size(__first, __last, std::distance(__first, __last));
  }

#if _LIBCPP_STD_VER >= 23
  template <_ContainerCompatibleRange<_Tp> _Range>
  _LIBCPP_HIDE_FROM_ABI constexpr void assign_range(_Range&& __range) {
    if constexpr (ranges::forward_range<_Range> || ranges::sized_range<_Range>) {
      auto __n = static_cast<size_type>(ranges::distance(__range));
      __assign_with_size(ranges::begin(__range), ranges::end(__range), __n);

    } else {
      __assign_with_sentinel(ranges::begin(__range), ranges::end(__range));
    }
  }
#endif

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void assign(size_type __n, const_reference __u);

#ifndef _LIBCPP_CXX03_LANG
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void assign(initializer_list<value_type> __il) {
    assign(__il.begin(), __il.end());
  }
#endif

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI allocator_type get_allocator() const _NOEXCEPT {
    return this->__alloc_;
  }

  //
  // Iterators
  //
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI iterator begin() _NOEXCEPT {
    return __make_iter(__add_alignment_assumption(this->__begin_));
  }
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI const_iterator begin() const _NOEXCEPT {
    return __make_iter(__add_alignment_assumption(this->__begin_));
  }
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI iterator end() _NOEXCEPT {
    return __make_iter(__add_alignment_assumption(this->__end_));
  }
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI const_iterator end() const _NOEXCEPT {
    return __make_iter(__add_alignment_assumption(this->__end_));
  }

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI reverse_iterator rbegin() _NOEXCEPT {
    return reverse_iterator(end());
  }
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI const_reverse_iterator rbegin() const _NOEXCEPT {
    return const_reverse_iterator(end());
  }
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI reverse_iterator rend() _NOEXCEPT {
    return reverse_iterator(begin());
  }
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI const_reverse_iterator rend() const _NOEXCEPT {
    return const_reverse_iterator(begin());
  }

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI const_iterator cbegin() const _NOEXCEPT { return begin(); }
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI const_iterator cend() const _NOEXCEPT { return end(); }
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI const_reverse_iterator crbegin() const _NOEXCEPT {
    return rbegin();
  }
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI const_reverse_iterator crend() const _NOEXCEPT { return rend(); }

  //
  // [vector.capacity], capacity
  //
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI size_type size() const _NOEXCEPT {
    return static_cast<size_type>(this->__end_ - this->__begin_);
  }
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI size_type capacity() const _NOEXCEPT {
    return static_cast<size_type>(this->__cap_ - this->__begin_);
  }
  [[__nodiscard__]] _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI bool empty() const _NOEXCEPT {
    return this->__begin_ == this->__end_;
  }
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI size_type max_size() const _NOEXCEPT {
    return std::min<size_type>(__alloc_traits::max_size(this->__alloc_), numeric_limits<difference_type>::max());
  }
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void reserve(size_type __n);
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void shrink_to_fit() _NOEXCEPT;

  //
  // element access
  //
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI reference operator[](size_type __n) _NOEXCEPT {
    _LIBCPP_ASSERT_VALID_ELEMENT_ACCESS(__n < size(), "vector[] index out of bounds");
    return this->__begin_[__n];
  }
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI const_reference operator[](size_type __n) const _NOEXCEPT {
    _LIBCPP_ASSERT_VALID_ELEMENT_ACCESS(__n < size(), "vector[] index out of bounds");
    return this->__begin_[__n];
  }
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI reference at(size_type __n) {
    if (__n >= size())
      this->__throw_out_of_range();
    return this->__begin_[__n];
  }
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI const_reference at(size_type __n) const {
    if (__n >= size())
      this->__throw_out_of_range();
    return this->__begin_[__n];
  }

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI reference front() _NOEXCEPT {
    _LIBCPP_ASSERT_VALID_ELEMENT_ACCESS(!empty(), "front() called on an empty vector");
    return *this->__begin_;
  }
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI const_reference front() const _NOEXCEPT {
    _LIBCPP_ASSERT_VALID_ELEMENT_ACCESS(!empty(), "front() called on an empty vector");
    return *this->__begin_;
  }
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI reference back() _NOEXCEPT {
    _LIBCPP_ASSERT_VALID_ELEMENT_ACCESS(!empty(), "back() called on an empty vector");
    return *(this->__end_ - 1);
  }
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI const_reference back() const _NOEXCEPT {
    _LIBCPP_ASSERT_VALID_ELEMENT_ACCESS(!empty(), "back() called on an empty vector");
    return *(this->__end_ - 1);
  }

  //
  // [vector.data], data access
  //
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI value_type* data() _NOEXCEPT {
    return std::__to_address(this->__begin_);
  }

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI const value_type* data() const _NOEXCEPT {
    return std::__to_address(this->__begin_);
  }

  //
  // [vector.modifiers], modifiers
  //
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void push_back(const_reference __x) { emplace_back(__x); }

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void push_back(value_type&& __x) { emplace_back(std::move(__x)); }

  template <class... _Args>
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI
#if _LIBCPP_STD_VER >= 17
  reference
  emplace_back(_Args&&... __args);
#else
  void
  emplace_back(_Args&&... __args);
#endif

  template <class... _Args>
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void __emplace_back_assume_capacity(_Args&&... __args) {
    _LIBCPP_ASSERT_INTERNAL(
        size() < capacity(), "We assume that we have enough space to insert an element at the end of the vector");
    _ConstructTransaction __tx(*this, 1);
    __alloc_traits::construct(this->__alloc_, std::__to_address(__tx.__pos_), std::forward<_Args>(__args)...);
    ++__tx.__pos_;
  }

#if _LIBCPP_STD_VER >= 23
  template <_ContainerCompatibleRange<_Tp> _Range>
  _LIBCPP_HIDE_FROM_ABI constexpr void append_range(_Range&& __range) {
    insert_range(end(), std::forward<_Range>(__range));
  }
#endif

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void pop_back() {
    _LIBCPP_ASSERT_VALID_ELEMENT_ACCESS(!empty(), "vector::pop_back called on an empty vector");
    this->__destruct_at_end(this->__end_ - 1);
  }

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI iterator insert(const_iterator __position, const_reference __x);

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI iterator insert(const_iterator __position, value_type&& __x);
  template <class... _Args>
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI iterator emplace(const_iterator __position, _Args&&... __args);

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI iterator
  insert(const_iterator __position, size_type __n, const_reference __x);

  template <class _InputIterator,
            __enable_if_t<__has_exactly_input_iterator_category<_InputIterator>::value &&
                              is_constructible< value_type, typename iterator_traits<_InputIterator>::reference>::value,
                          int> = 0>
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI iterator
  insert(const_iterator __position, _InputIterator __first, _InputIterator __last) {
    return __insert_with_sentinel(__position, __first, __last);
  }

  template <
      class _ForwardIterator,
      __enable_if_t<__has_forward_iterator_category<_ForwardIterator>::value &&
                        is_constructible< value_type, typename iterator_traits<_ForwardIterator>::reference>::value,
                    int> = 0>
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI iterator
  insert(const_iterator __position, _ForwardIterator __first, _ForwardIterator __last) {
    return __insert_with_size(__position, __first, __last, std::distance(__first, __last));
  }

#if _LIBCPP_STD_VER >= 23
  template <_ContainerCompatibleRange<_Tp> _Range>
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
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI iterator
  insert(const_iterator __position, initializer_list<value_type> __il) {
    return insert(__position, __il.begin(), __il.end());
  }
#endif

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI iterator erase(const_iterator __position);
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI iterator erase(const_iterator __first, const_iterator __last);

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void clear() _NOEXCEPT {
    size_type __old_size = size();
    __base_destruct_at_end(this->__begin_);
    __annotate_shrink(__old_size);
  }

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void resize(size_type __sz);
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void resize(size_type __sz, const_reference __x);

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void swap(vector&)
#if _LIBCPP_STD_VER >= 14
      _NOEXCEPT;
#else
      _NOEXCEPT_(!__alloc_traits::propagate_on_container_swap::value || __is_nothrow_swappable_v<allocator_type>);
#endif

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI bool __invariants() const;

private:
  pointer __begin_ = nullptr;
  pointer __end_   = nullptr;
  _LIBCPP_COMPRESSED_PAIR(pointer, __cap_ = nullptr, allocator_type, __alloc_);

  //  Allocate space for __n objects
  //  throws length_error if __n > max_size()
  //  throws (probably bad_alloc) if memory run out
  //  Precondition:  __begin_ == __end_ == __cap_ == nullptr
  //  Precondition:  __n > 0
  //  Postcondition:  capacity() >= __n
  //  Postcondition:  size() == 0
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void __vallocate(size_type __n) {
    if (__n > max_size())
      this->__throw_length_error();
    auto __allocation = std::__allocate_at_least(this->__alloc_, __n);
    __begin_          = __allocation.ptr;
    __end_            = __allocation.ptr;
    __cap_            = __begin_ + __allocation.count;
    __annotate_new(0);
  }

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void __vdeallocate() _NOEXCEPT;
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI size_type __recommend(size_type __new_size) const;
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void __construct_at_end(size_type __n);
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void __construct_at_end(size_type __n, const_reference __x);

  template <class _InputIterator, class _Sentinel>
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void
  __init_with_size(_InputIterator __first, _Sentinel __last, size_type __n) {
    auto __guard = std::__make_exception_guard(__destroy_vector(*this));

    if (__n > 0) {
      __vallocate(__n);
      __construct_at_end(std::move(__first), std::move(__last), __n);
    }

    __guard.__complete();
  }

  template <class _InputIterator, class _Sentinel>
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void
  __init_with_sentinel(_InputIterator __first, _Sentinel __last) {
    auto __guard = std::__make_exception_guard(__destroy_vector(*this));

    for (; __first != __last; ++__first)
      emplace_back(*__first);

    __guard.__complete();
  }

  template <class _Iterator, class _Sentinel>
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void __assign_with_sentinel(_Iterator __first, _Sentinel __last);

  // The `_Iterator` in `*_with_size` functions can be input-only only if called from `*_range` (since C++23).
  // Otherwise, `_Iterator` is a forward iterator.

  template <class _Iterator, class _Sentinel>
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void
  __assign_with_size(_Iterator __first, _Sentinel __last, difference_type __n);

  template <class _Iterator,
            __enable_if_t<!is_same<decltype(*std::declval<_Iterator&>())&&, value_type&&>::value, int> = 0>
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void
  __insert_assign_n_unchecked(_Iterator __first, difference_type __n, pointer __position) {
    for (pointer __end_position = __position + __n; __position != __end_position; ++__position, (void)++__first) {
      __temp_value<value_type, _Allocator> __tmp(this->__alloc_, *__first);
      *__position = std::move(__tmp.get());
    }
  }

  template <class _Iterator,
            __enable_if_t<is_same<decltype(*std::declval<_Iterator&>())&&, value_type&&>::value, int> = 0>
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void
  __insert_assign_n_unchecked(_Iterator __first, difference_type __n, pointer __position) {
#if _LIBCPP_STD_VER >= 23
    if constexpr (!forward_iterator<_Iterator>) { // Handles input-only sized ranges for insert_range
      ranges::copy_n(std::move(__first), __n, __position);
    } else
#endif
    {
      std::copy_n(__first, __n, __position);
    }
  }

  template <class _InputIterator, class _Sentinel>
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI iterator
  __insert_with_sentinel(const_iterator __position, _InputIterator __first, _Sentinel __last);

  template <class _Iterator, class _Sentinel>
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI iterator
  __insert_with_size(const_iterator __position, _Iterator __first, _Sentinel __last, difference_type __n);

  template <class _InputIterator, class _Sentinel>
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void
  __construct_at_end(_InputIterator __first, _Sentinel __last, size_type __n);

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void __append(size_type __n);
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void __append(size_type __n, const_reference __x);

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI iterator __make_iter(pointer __p) _NOEXCEPT {
#ifdef _LIBCPP_ABI_BOUNDED_ITERATORS_IN_VECTOR
    // Bound the iterator according to the capacity, rather than the size.
    //
    // Vector guarantees that iterators stay valid as long as no reallocation occurs even if new elements are inserted
    // into the container; for these cases, we need to make sure that the newly-inserted elements can be accessed
    // through the bounded iterator without failing checks. The downside is that the bounded iterator won't catch
    // access that is logically out-of-bounds, i.e., goes beyond the size, but is still within the capacity. With the
    // current implementation, there is no connection between a bounded iterator and its associated container, so we
    // don't have a way to update existing valid iterators when the container is resized and thus have to go with
    // a laxer approach.
    return std::__make_bounded_iter(
        std::__wrap_iter<pointer>(__p),
        std::__wrap_iter<pointer>(this->__begin_),
        std::__wrap_iter<pointer>(this->__cap_));
#else
    return iterator(__p);
#endif // _LIBCPP_ABI_BOUNDED_ITERATORS_IN_VECTOR
  }

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI const_iterator __make_iter(const_pointer __p) const _NOEXCEPT {
#ifdef _LIBCPP_ABI_BOUNDED_ITERATORS_IN_VECTOR
    // Bound the iterator according to the capacity, rather than the size.
    return std::__make_bounded_iter(
        std::__wrap_iter<const_pointer>(__p),
        std::__wrap_iter<const_pointer>(this->__begin_),
        std::__wrap_iter<const_pointer>(this->__cap_));
#else
    return const_iterator(__p);
#endif // _LIBCPP_ABI_BOUNDED_ITERATORS_IN_VECTOR
  }

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void
  __swap_out_circular_buffer(__split_buffer<value_type, allocator_type&>& __v);
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI pointer
  __swap_out_circular_buffer(__split_buffer<value_type, allocator_type&>& __v, pointer __p);
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void
  __move_range(pointer __from_s, pointer __from_e, pointer __to);
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void __move_assign(vector& __c, true_type)
      _NOEXCEPT_(is_nothrow_move_assignable<allocator_type>::value);
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void __move_assign(vector& __c, false_type)
      _NOEXCEPT_(__alloc_traits::is_always_equal::value);
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void __destruct_at_end(pointer __new_last) _NOEXCEPT {
    size_type __old_size = size();
    __base_destruct_at_end(__new_last);
    __annotate_shrink(__old_size);
  }

  template <class... _Args>
  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI inline pointer __emplace_back_slow_path(_Args&&... __args);

  // The following functions are no-ops outside of AddressSanitizer mode.
  // We call annotations for every allocator, unless explicitly disabled.
  //
  // To disable annotations for a particular allocator, change value of
  // __asan_annotate_container_with_allocator to false.
  // For more details, see the "Using libc++" documentation page or
  // the documentation for __sanitizer_annotate_contiguous_container.

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void
  __annotate_contiguous_container(const void* __old_mid, const void* __new_mid) const {
    std::__annotate_contiguous_container<_Allocator>(data(), data() + capacity(), __old_mid, __new_mid);
  }

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void __annotate_new(size_type __current_size) const _NOEXCEPT {
    (void)__current_size;
#if _LIBCPP_HAS_ASAN
    __annotate_contiguous_container(data() + capacity(), data() + __current_size);
#endif
  }

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void __annotate_delete() const _NOEXCEPT {
#if _LIBCPP_HAS_ASAN
    __annotate_contiguous_container(data() + size(), data() + capacity());
#endif
  }

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void __annotate_increase(size_type __n) const _NOEXCEPT {
    (void)__n;
#if _LIBCPP_HAS_ASAN
    __annotate_contiguous_container(data() + size(), data() + size() + __n);
#endif
  }

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void __annotate_shrink(size_type __old_size) const _NOEXCEPT {
    (void)__old_size;
#if _LIBCPP_HAS_ASAN
    __annotate_contiguous_container(data() + __old_size, data() + size());
#endif
  }

  struct _ConstructTransaction {
    _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI explicit _ConstructTransaction(vector& __v, size_type __n)
        : __v_(__v), __pos_(__v.__end_), __new_end_(__v.__end_ + __n) {
#if _LIBCPP_HAS_ASAN
      __v_.__annotate_increase(__n);
#endif
    }

    _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI ~_ConstructTransaction() {
      __v_.__end_ = __pos_;
#if _LIBCPP_HAS_ASAN
      if (__pos_ != __new_end_) {
        __v_.__annotate_shrink(__new_end_ - __v_.__begin_);
      }
#endif
    }

    vector& __v_;
    pointer __pos_;
    const_pointer const __new_end_;

    _ConstructTransaction(_ConstructTransaction const&)            = delete;
    _ConstructTransaction& operator=(_ConstructTransaction const&) = delete;
  };

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void __base_destruct_at_end(pointer __new_last) _NOEXCEPT {
    pointer __soon_to_be_end = this->__end_;
    while (__new_last != __soon_to_be_end)
      __alloc_traits::destroy(this->__alloc_, std::__to_address(--__soon_to_be_end));
    this->__end_ = __new_last;
  }

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void __copy_assign_alloc(const vector& __c) {
    __copy_assign_alloc(__c, integral_constant<bool, __alloc_traits::propagate_on_container_copy_assignment::value>());
  }

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void __move_assign_alloc(vector& __c)
      _NOEXCEPT_(!__alloc_traits::propagate_on_container_move_assignment::value ||
                 is_nothrow_move_assignable<allocator_type>::value) {
    __move_assign_alloc(__c, integral_constant<bool, __alloc_traits::propagate_on_container_move_assignment::value>());
  }

  [[__noreturn__]] _LIBCPP_HIDE_FROM_ABI static void __throw_length_error() { std::__throw_length_error("vector"); }

  [[__noreturn__]] _LIBCPP_HIDE_FROM_ABI static void __throw_out_of_range() { std::__throw_out_of_range("vector"); }

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void __copy_assign_alloc(const vector& __c, true_type) {
    if (this->__alloc_ != __c.__alloc_) {
      clear();
      __annotate_delete();
      __alloc_traits::deallocate(this->__alloc_, this->__begin_, capacity());
      this->__begin_ = this->__end_ = this->__cap_ = nullptr;
    }
    this->__alloc_ = __c.__alloc_;
  }

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void __copy_assign_alloc(const vector&, false_type) {}

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void __move_assign_alloc(vector& __c, true_type)
      _NOEXCEPT_(is_nothrow_move_assignable<allocator_type>::value) {
    this->__alloc_ = std::move(__c.__alloc_);
  }

  _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void __move_assign_alloc(vector&, false_type) _NOEXCEPT {}

  template <class _Ptr = pointer, __enable_if_t<is_pointer<_Ptr>::value, int> = 0>
  static _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI _LIBCPP_NO_CFI pointer
  __add_alignment_assumption(_Ptr __p) _NOEXCEPT {
    if (!__libcpp_is_constant_evaluated()) {
      return static_cast<pointer>(__builtin_assume_aligned(__p, _LIBCPP_ALIGNOF(decltype(*__p))));
    }
    return __p;
  }

  template <class _Ptr = pointer, __enable_if_t<!is_pointer<_Ptr>::value, int> = 0>
  static _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI _LIBCPP_NO_CFI pointer
  __add_alignment_assumption(_Ptr __p) _NOEXCEPT {
    return __p;
  }
};

#if _LIBCPP_STD_VER >= 17
template <class _InputIterator,
          class _Alloc = allocator<__iter_value_type<_InputIterator>>,
          class        = enable_if_t<__has_input_iterator_category<_InputIterator>::value>,
          class        = enable_if_t<__is_allocator<_Alloc>::value> >
vector(_InputIterator, _InputIterator) -> vector<__iter_value_type<_InputIterator>, _Alloc>;

template <class _InputIterator,
          class _Alloc,
          class = enable_if_t<__has_input_iterator_category<_InputIterator>::value>,
          class = enable_if_t<__is_allocator<_Alloc>::value> >
vector(_InputIterator, _InputIterator, _Alloc) -> vector<__iter_value_type<_InputIterator>, _Alloc>;
#endif

#if _LIBCPP_STD_VER >= 23
template <ranges::input_range _Range,
          class _Alloc = allocator<ranges::range_value_t<_Range>>,
          class        = enable_if_t<__is_allocator<_Alloc>::value> >
vector(from_range_t, _Range&&, _Alloc = _Alloc()) -> vector<ranges::range_value_t<_Range>, _Alloc>;
#endif

// __swap_out_circular_buffer relocates the objects in [__begin_, __end_) into the front of __v and swaps the buffers of
// *this and __v. It is assumed that __v provides space for exactly (__end_ - __begin_) objects in the front. This
// function has a strong exception guarantee.
template <class _Tp, class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 void
vector<_Tp, _Allocator>::__swap_out_circular_buffer(__split_buffer<value_type, allocator_type&>& __v) {
  __annotate_delete();
  auto __new_begin = __v.__begin_ - (__end_ - __begin_);
  std::__uninitialized_allocator_relocate(
      this->__alloc_, std::__to_address(__begin_), std::__to_address(__end_), std::__to_address(__new_begin));
  __v.__begin_ = __new_begin;
  __end_       = __begin_; // All the objects have been destroyed by relocating them.
  std::swap(this->__begin_, __v.__begin_);
  std::swap(this->__end_, __v.__end_);
  std::swap(this->__cap_, __v.__cap_);
  __v.__first_ = __v.__begin_;
  __annotate_new(size());
}

// __swap_out_circular_buffer relocates the objects in [__begin_, __p) into the front of __v, the objects in
// [__p, __end_) into the back of __v and swaps the buffers of *this and __v. It is assumed that __v provides space for
// exactly (__p - __begin_) objects in the front and space for at least (__end_ - __p) objects in the back. This
// function has a strong exception guarantee if __begin_ == __p || __end_ == __p.
template <class _Tp, class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 typename vector<_Tp, _Allocator>::pointer
vector<_Tp, _Allocator>::__swap_out_circular_buffer(__split_buffer<value_type, allocator_type&>& __v, pointer __p) {
  __annotate_delete();
  pointer __ret = __v.__begin_;

  // Relocate [__p, __end_) first to avoid having a hole in [__begin_, __end_)
  // in case something in [__begin_, __p) throws.
  std::__uninitialized_allocator_relocate(
      this->__alloc_, std::__to_address(__p), std::__to_address(__end_), std::__to_address(__v.__end_));
  __v.__end_ += (__end_ - __p);
  __end_           = __p; // The objects in [__p, __end_) have been destroyed by relocating them.
  auto __new_begin = __v.__begin_ - (__p - __begin_);

  std::__uninitialized_allocator_relocate(
      this->__alloc_, std::__to_address(__begin_), std::__to_address(__p), std::__to_address(__new_begin));
  __v.__begin_ = __new_begin;
  __end_       = __begin_; // All the objects have been destroyed by relocating them.

  std::swap(this->__begin_, __v.__begin_);
  std::swap(this->__end_, __v.__end_);
  std::swap(this->__cap_, __v.__cap_);
  __v.__first_ = __v.__begin_;
  __annotate_new(size());
  return __ret;
}

template <class _Tp, class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 void vector<_Tp, _Allocator>::__vdeallocate() _NOEXCEPT {
  if (this->__begin_ != nullptr) {
    clear();
    __annotate_delete();
    __alloc_traits::deallocate(this->__alloc_, this->__begin_, capacity());
    this->__begin_ = this->__end_ = this->__cap_ = nullptr;
  }
}

//  Precondition:  __new_size > capacity()
template <class _Tp, class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 inline _LIBCPP_HIDE_FROM_ABI typename vector<_Tp, _Allocator>::size_type
vector<_Tp, _Allocator>::__recommend(size_type __new_size) const {
  const size_type __ms = max_size();
  if (__new_size > __ms)
    this->__throw_length_error();
  const size_type __cap = capacity();
  if (__cap >= __ms / 2)
    return __ms;
  return std::max<size_type>(2 * __cap, __new_size);
}

//  Default constructs __n objects starting at __end_
//  throws if construction throws
//  Precondition:  __n > 0
//  Precondition:  size() + __n <= capacity()
//  Postcondition:  size() == size() + __n
template <class _Tp, class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 void vector<_Tp, _Allocator>::__construct_at_end(size_type __n) {
  _ConstructTransaction __tx(*this, __n);
  const_pointer __new_end = __tx.__new_end_;
  for (pointer __pos = __tx.__pos_; __pos != __new_end; __tx.__pos_ = ++__pos) {
    __alloc_traits::construct(this->__alloc_, std::__to_address(__pos));
  }
}

//  Copy constructs __n objects starting at __end_ from __x
//  throws if construction throws
//  Precondition:  __n > 0
//  Precondition:  size() + __n <= capacity()
//  Postcondition:  size() == old size() + __n
//  Postcondition:  [i] == __x for all i in [size() - __n, __n)
template <class _Tp, class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 inline void
vector<_Tp, _Allocator>::__construct_at_end(size_type __n, const_reference __x) {
  _ConstructTransaction __tx(*this, __n);
  const_pointer __new_end = __tx.__new_end_;
  for (pointer __pos = __tx.__pos_; __pos != __new_end; __tx.__pos_ = ++__pos) {
    __alloc_traits::construct(this->__alloc_, std::__to_address(__pos), __x);
  }
}

template <class _Tp, class _Allocator>
template <class _InputIterator, class _Sentinel>
_LIBCPP_CONSTEXPR_SINCE_CXX20 void
vector<_Tp, _Allocator>::__construct_at_end(_InputIterator __first, _Sentinel __last, size_type __n) {
  _ConstructTransaction __tx(*this, __n);
  __tx.__pos_ = std::__uninitialized_allocator_copy(this->__alloc_, std::move(__first), std::move(__last), __tx.__pos_);
}

//  Default constructs __n objects starting at __end_
//  throws if construction throws
//  Postcondition:  size() == size() + __n
//  Exception safety: strong.
template <class _Tp, class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 void vector<_Tp, _Allocator>::__append(size_type __n) {
  if (static_cast<size_type>(this->__cap_ - this->__end_) >= __n)
    this->__construct_at_end(__n);
  else {
    __split_buffer<value_type, allocator_type&> __v(__recommend(size() + __n), size(), this->__alloc_);
    __v.__construct_at_end(__n);
    __swap_out_circular_buffer(__v);
  }
}

//  Default constructs __n objects starting at __end_
//  throws if construction throws
//  Postcondition:  size() == size() + __n
//  Exception safety: strong.
template <class _Tp, class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 void vector<_Tp, _Allocator>::__append(size_type __n, const_reference __x) {
  if (static_cast<size_type>(this->__cap_ - this->__end_) >= __n)
    this->__construct_at_end(__n, __x);
  else {
    __split_buffer<value_type, allocator_type&> __v(__recommend(size() + __n), size(), this->__alloc_);
    __v.__construct_at_end(__n, __x);
    __swap_out_circular_buffer(__v);
  }
}

template <class _Tp, class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 inline _LIBCPP_HIDE_FROM_ABI vector<_Tp, _Allocator>::vector(vector&& __x)
#if _LIBCPP_STD_VER >= 17
    noexcept
#else
    _NOEXCEPT_(is_nothrow_move_constructible<allocator_type>::value)
#endif
    : __alloc_(std::move(__x.__alloc_)) {
  this->__begin_ = __x.__begin_;
  this->__end_   = __x.__end_;
  this->__cap_   = __x.__cap_;
  __x.__begin_ = __x.__end_ = __x.__cap_ = nullptr;
}

template <class _Tp, class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 inline _LIBCPP_HIDE_FROM_ABI
vector<_Tp, _Allocator>::vector(vector&& __x, const __type_identity_t<allocator_type>& __a)
    : __alloc_(__a) {
  if (__a == __x.__alloc_) {
    this->__begin_ = __x.__begin_;
    this->__end_   = __x.__end_;
    this->__cap_   = __x.__cap_;
    __x.__begin_ = __x.__end_ = __x.__cap_ = nullptr;
  } else {
    typedef move_iterator<iterator> _Ip;
    __init_with_size(_Ip(__x.begin()), _Ip(__x.end()), __x.size());
  }
}

template <class _Tp, class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 void vector<_Tp, _Allocator>::__move_assign(vector& __c, false_type)
    _NOEXCEPT_(__alloc_traits::is_always_equal::value) {
  if (this->__alloc_ != __c.__alloc_) {
    typedef move_iterator<iterator> _Ip;
    assign(_Ip(__c.begin()), _Ip(__c.end()));
  } else
    __move_assign(__c, true_type());
}

template <class _Tp, class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 void vector<_Tp, _Allocator>::__move_assign(vector& __c, true_type)
    _NOEXCEPT_(is_nothrow_move_assignable<allocator_type>::value) {
  __vdeallocate();
  __move_assign_alloc(__c); // this can throw
  this->__begin_ = __c.__begin_;
  this->__end_   = __c.__end_;
  this->__cap_   = __c.__cap_;
  __c.__begin_ = __c.__end_ = __c.__cap_ = nullptr;
}

template <class _Tp, class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 inline _LIBCPP_HIDE_FROM_ABI vector<_Tp, _Allocator>&
vector<_Tp, _Allocator>::operator=(const vector& __x) {
  if (this != std::addressof(__x)) {
    __copy_assign_alloc(__x);
    assign(__x.__begin_, __x.__end_);
  }
  return *this;
}

template <class _Tp, class _Allocator>
template <class _Iterator, class _Sentinel>
_LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void
vector<_Tp, _Allocator>::__assign_with_sentinel(_Iterator __first, _Sentinel __last) {
  pointer __cur = __begin_;
  for (; __first != __last && __cur != __end_; ++__first, (void)++__cur)
    *__cur = *__first;
  if (__cur != __end_) {
    __destruct_at_end(__cur);
  } else {
    for (; __first != __last; ++__first)
      emplace_back(*__first);
  }
}

template <class _Tp, class _Allocator>
template <class _Iterator, class _Sentinel>
_LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI void
vector<_Tp, _Allocator>::__assign_with_size(_Iterator __first, _Sentinel __last, difference_type __n) {
  size_type __new_size = static_cast<size_type>(__n);
  if (__new_size <= capacity()) {
    if (__new_size > size()) {
#if _LIBCPP_STD_VER >= 23
      auto __mid = ranges::copy_n(std::move(__first), size(), this->__begin_).in;
      __construct_at_end(std::move(__mid), std::move(__last), __new_size - size());
#else
      _Iterator __mid = std::next(__first, size());
      std::copy(__first, __mid, this->__begin_);
      __construct_at_end(__mid, __last, __new_size - size());
#endif
    } else {
      pointer __m = std::__copy(std::move(__first), __last, this->__begin_).second;
      this->__destruct_at_end(__m);
    }
  } else {
    __vdeallocate();
    __vallocate(__recommend(__new_size));
    __construct_at_end(std::move(__first), std::move(__last), __new_size);
  }
}

template <class _Tp, class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 void vector<_Tp, _Allocator>::assign(size_type __n, const_reference __u) {
  if (__n <= capacity()) {
    size_type __s = size();
    std::fill_n(this->__begin_, std::min(__n, __s), __u);
    if (__n > __s)
      __construct_at_end(__n - __s, __u);
    else
      this->__destruct_at_end(this->__begin_ + __n);
  } else {
    __vdeallocate();
    __vallocate(__recommend(static_cast<size_type>(__n)));
    __construct_at_end(__n, __u);
  }
}

template <class _Tp, class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 void vector<_Tp, _Allocator>::reserve(size_type __n) {
  if (__n > capacity()) {
    if (__n > max_size())
      this->__throw_length_error();
    __split_buffer<value_type, allocator_type&> __v(__n, size(), this->__alloc_);
    __swap_out_circular_buffer(__v);
  }
}

template <class _Tp, class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 void vector<_Tp, _Allocator>::shrink_to_fit() _NOEXCEPT {
  if (capacity() > size()) {
#if _LIBCPP_HAS_EXCEPTIONS
    try {
#endif // _LIBCPP_HAS_EXCEPTIONS
      __split_buffer<value_type, allocator_type&> __v(size(), size(), this->__alloc_);
      // The Standard mandates shrink_to_fit() does not increase the capacity.
      // With equal capacity keep the existing buffer. This avoids extra work
      // due to swapping the elements.
      if (__v.capacity() < capacity())
        __swap_out_circular_buffer(__v);
#if _LIBCPP_HAS_EXCEPTIONS
    } catch (...) {
    }
#endif // _LIBCPP_HAS_EXCEPTIONS
  }
}

template <class _Tp, class _Allocator>
template <class... _Args>
_LIBCPP_CONSTEXPR_SINCE_CXX20 typename vector<_Tp, _Allocator>::pointer
vector<_Tp, _Allocator>::__emplace_back_slow_path(_Args&&... __args) {
  __split_buffer<value_type, allocator_type&> __v(__recommend(size() + 1), size(), this->__alloc_);
  //    __v.emplace_back(std::forward<_Args>(__args)...);
  __alloc_traits::construct(this->__alloc_, std::__to_address(__v.__end_), std::forward<_Args>(__args)...);
  __v.__end_++;
  __swap_out_circular_buffer(__v);
  return this->__end_;
}

template <class _Tp, class _Allocator>
template <class... _Args>
_LIBCPP_CONSTEXPR_SINCE_CXX20 inline
#if _LIBCPP_STD_VER >= 17
    typename vector<_Tp, _Allocator>::reference
#else
    void
#endif
    vector<_Tp, _Allocator>::emplace_back(_Args&&... __args) {
  pointer __end = this->__end_;
  if (__end < this->__cap_) {
    __emplace_back_assume_capacity(std::forward<_Args>(__args)...);
    ++__end;
  } else {
    __end = __emplace_back_slow_path(std::forward<_Args>(__args)...);
  }
  this->__end_ = __end;
#if _LIBCPP_STD_VER >= 17
  return *(__end - 1);
#endif
}

template <class _Tp, class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 inline _LIBCPP_HIDE_FROM_ABI typename vector<_Tp, _Allocator>::iterator
vector<_Tp, _Allocator>::erase(const_iterator __position) {
  _LIBCPP_ASSERT_VALID_ELEMENT_ACCESS(
      __position != end(), "vector::erase(iterator) called with a non-dereferenceable iterator");
  difference_type __ps = __position - cbegin();
  pointer __p          = this->__begin_ + __ps;
  this->__destruct_at_end(std::move(__p + 1, this->__end_, __p));
  return __make_iter(__p);
}

template <class _Tp, class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 typename vector<_Tp, _Allocator>::iterator
vector<_Tp, _Allocator>::erase(const_iterator __first, const_iterator __last) {
  _LIBCPP_ASSERT_VALID_INPUT_RANGE(__first <= __last, "vector::erase(first, last) called with invalid range");
  pointer __p = this->__begin_ + (__first - begin());
  if (__first != __last) {
    this->__destruct_at_end(std::move(__p + (__last - __first), this->__end_, __p));
  }
  return __make_iter(__p);
}

template <class _Tp, class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 void
vector<_Tp, _Allocator>::__move_range(pointer __from_s, pointer __from_e, pointer __to) {
  pointer __old_last  = this->__end_;
  difference_type __n = __old_last - __to;
  {
    pointer __i = __from_s + __n;
    _ConstructTransaction __tx(*this, __from_e - __i);
    for (pointer __pos = __tx.__pos_; __i < __from_e; ++__i, (void)++__pos, __tx.__pos_ = __pos) {
      __alloc_traits::construct(this->__alloc_, std::__to_address(__pos), std::move(*__i));
    }
  }
  std::move_backward(__from_s, __from_s + __n, __old_last);
}

template <class _Tp, class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 typename vector<_Tp, _Allocator>::iterator
vector<_Tp, _Allocator>::insert(const_iterator __position, const_reference __x) {
  pointer __p = this->__begin_ + (__position - begin());
  if (this->__end_ < this->__cap_) {
    if (__p == this->__end_) {
      __emplace_back_assume_capacity(__x);
    } else {
      __move_range(__p, this->__end_, __p + 1);
      const_pointer __xr = pointer_traits<const_pointer>::pointer_to(__x);
      if (std::__is_pointer_in_range(std::__to_address(__p), std::__to_address(__end_), std::addressof(__x)))
        ++__xr;
      *__p = *__xr;
    }
  } else {
    __split_buffer<value_type, allocator_type&> __v(__recommend(size() + 1), __p - this->__begin_, this->__alloc_);
    __v.emplace_back(__x);
    __p = __swap_out_circular_buffer(__v, __p);
  }
  return __make_iter(__p);
}

template <class _Tp, class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 typename vector<_Tp, _Allocator>::iterator
vector<_Tp, _Allocator>::insert(const_iterator __position, value_type&& __x) {
  pointer __p = this->__begin_ + (__position - begin());
  if (this->__end_ < this->__cap_) {
    if (__p == this->__end_) {
      __emplace_back_assume_capacity(std::move(__x));
    } else {
      __move_range(__p, this->__end_, __p + 1);
      *__p = std::move(__x);
    }
  } else {
    __split_buffer<value_type, allocator_type&> __v(__recommend(size() + 1), __p - this->__begin_, this->__alloc_);
    __v.emplace_back(std::move(__x));
    __p = __swap_out_circular_buffer(__v, __p);
  }
  return __make_iter(__p);
}

template <class _Tp, class _Allocator>
template <class... _Args>
_LIBCPP_CONSTEXPR_SINCE_CXX20 typename vector<_Tp, _Allocator>::iterator
vector<_Tp, _Allocator>::emplace(const_iterator __position, _Args&&... __args) {
  pointer __p = this->__begin_ + (__position - begin());
  if (this->__end_ < this->__cap_) {
    if (__p == this->__end_) {
      __emplace_back_assume_capacity(std::forward<_Args>(__args)...);
    } else {
      __temp_value<value_type, _Allocator> __tmp(this->__alloc_, std::forward<_Args>(__args)...);
      __move_range(__p, this->__end_, __p + 1);
      *__p = std::move(__tmp.get());
    }
  } else {
    __split_buffer<value_type, allocator_type&> __v(__recommend(size() + 1), __p - this->__begin_, this->__alloc_);
    __v.emplace_back(std::forward<_Args>(__args)...);
    __p = __swap_out_circular_buffer(__v, __p);
  }
  return __make_iter(__p);
}

template <class _Tp, class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 typename vector<_Tp, _Allocator>::iterator
vector<_Tp, _Allocator>::insert(const_iterator __position, size_type __n, const_reference __x) {
  pointer __p = this->__begin_ + (__position - begin());
  if (__n > 0) {
    // We can't compare unrelated pointers inside constant expressions
    if (!__libcpp_is_constant_evaluated() && __n <= static_cast<size_type>(this->__cap_ - this->__end_)) {
      size_type __old_n  = __n;
      pointer __old_last = this->__end_;
      if (__n > static_cast<size_type>(this->__end_ - __p)) {
        size_type __cx = __n - (this->__end_ - __p);
        __construct_at_end(__cx, __x);
        __n -= __cx;
      }
      if (__n > 0) {
        __move_range(__p, __old_last, __p + __old_n);
        const_pointer __xr = pointer_traits<const_pointer>::pointer_to(__x);
        if (__p <= __xr && __xr < this->__end_)
          __xr += __old_n;
        std::fill_n(__p, __n, *__xr);
      }
    } else {
      __split_buffer<value_type, allocator_type&> __v(__recommend(size() + __n), __p - this->__begin_, this->__alloc_);
      __v.__construct_at_end(__n, __x);
      __p = __swap_out_circular_buffer(__v, __p);
    }
  }
  return __make_iter(__p);
}

template <class _Tp, class _Allocator>
template <class _InputIterator, class _Sentinel>
_LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI typename vector<_Tp, _Allocator>::iterator
vector<_Tp, _Allocator>::__insert_with_sentinel(const_iterator __position, _InputIterator __first, _Sentinel __last) {
  difference_type __off = __position - begin();
  pointer __p           = this->__begin_ + __off;
  pointer __old_last    = this->__end_;
  for (; this->__end_ != this->__cap_ && __first != __last; ++__first)
    __emplace_back_assume_capacity(*__first);

  if (__first == __last)
    (void)std::rotate(__p, __old_last, this->__end_);
  else {
    __split_buffer<value_type, allocator_type&> __v(__alloc_);
    auto __guard = std::__make_exception_guard(
        _AllocatorDestroyRangeReverse<allocator_type, pointer>(__alloc_, __old_last, this->__end_));
    __v.__construct_at_end_with_sentinel(std::move(__first), std::move(__last));
    __split_buffer<value_type, allocator_type&> __merged(
        __recommend(size() + __v.size()), __off, __alloc_); // has `__off` positions available at the front
    std::__uninitialized_allocator_relocate(
        __alloc_, std::__to_address(__old_last), std::__to_address(this->__end_), std::__to_address(__merged.__end_));
    __guard.__complete(); // Release the guard once objects in [__old_last_, __end_) have been successfully relocated.
    __merged.__end_ += this->__end_ - __old_last;
    this->__end_ = __old_last;
    std::__uninitialized_allocator_relocate(
        __alloc_, std::__to_address(__v.__begin_), std::__to_address(__v.__end_), std::__to_address(__merged.__end_));
    __merged.__end_ += __v.size();
    __v.__end_ = __v.__begin_;
    __p        = __swap_out_circular_buffer(__merged, __p);
  }
  return __make_iter(__p);
}

template <class _Tp, class _Allocator>
template <class _Iterator, class _Sentinel>
_LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI typename vector<_Tp, _Allocator>::iterator
vector<_Tp, _Allocator>::__insert_with_size(
    const_iterator __position, _Iterator __first, _Sentinel __last, difference_type __n) {
  pointer __p = this->__begin_ + (__position - begin());
  if (__n > 0) {
    if (__n <= this->__cap_ - this->__end_) {
      pointer __old_last   = this->__end_;
      difference_type __dx = this->__end_ - __p;
      if (__n > __dx) {
#if _LIBCPP_STD_VER >= 23
        if constexpr (!forward_iterator<_Iterator>) {
          __construct_at_end(std::move(__first), std::move(__last), __n);
          std::rotate(__p, __old_last, this->__end_);
        } else
#endif
        {
          _Iterator __m = std::next(__first, __dx);
          __construct_at_end(__m, __last, __n - __dx);
          if (__dx > 0) {
            __move_range(__p, __old_last, __p + __n);
            __insert_assign_n_unchecked(__first, __dx, __p);
          }
        }
      } else {
        __move_range(__p, __old_last, __p + __n);
        __insert_assign_n_unchecked(std::move(__first), __n, __p);
      }
    } else {
      __split_buffer<value_type, allocator_type&> __v(__recommend(size() + __n), __p - this->__begin_, this->__alloc_);
      __v.__construct_at_end_with_size(std::move(__first), __n);
      __p = __swap_out_circular_buffer(__v, __p);
    }
  }
  return __make_iter(__p);
}

template <class _Tp, class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 void vector<_Tp, _Allocator>::resize(size_type __sz) {
  size_type __cs = size();
  if (__cs < __sz)
    this->__append(__sz - __cs);
  else if (__cs > __sz)
    this->__destruct_at_end(this->__begin_ + __sz);
}

template <class _Tp, class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 void vector<_Tp, _Allocator>::resize(size_type __sz, const_reference __x) {
  size_type __cs = size();
  if (__cs < __sz)
    this->__append(__sz - __cs, __x);
  else if (__cs > __sz)
    this->__destruct_at_end(this->__begin_ + __sz);
}

template <class _Tp, class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 void vector<_Tp, _Allocator>::swap(vector& __x)
#if _LIBCPP_STD_VER >= 14
    _NOEXCEPT
#else
    _NOEXCEPT_(!__alloc_traits::propagate_on_container_swap::value || __is_nothrow_swappable_v<allocator_type>)
#endif
{
  _LIBCPP_ASSERT_COMPATIBLE_ALLOCATOR(
      __alloc_traits::propagate_on_container_swap::value || this->__alloc_ == __x.__alloc_,
      "vector::swap: Either propagate_on_container_swap must be true"
      " or the allocators must compare equal");
  std::swap(this->__begin_, __x.__begin_);
  std::swap(this->__end_, __x.__end_);
  std::swap(this->__cap_, __x.__cap_);
  std::__swap_allocator(this->__alloc_, __x.__alloc_);
}

template <class _Tp, class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 bool vector<_Tp, _Allocator>::__invariants() const {
  if (this->__begin_ == nullptr) {
    if (this->__end_ != nullptr || this->__cap_ != nullptr)
      return false;
  } else {
    if (this->__begin_ > this->__end_)
      return false;
    if (this->__begin_ == this->__cap_)
      return false;
    if (this->__end_ > this->__cap_)
      return false;
  }
  return true;
}

#if _LIBCPP_STD_VER >= 20
template <>
inline constexpr bool __format::__enable_insertable<vector<char>> = true;
#  if _LIBCPP_HAS_WIDE_CHARACTERS
template <>
inline constexpr bool __format::__enable_insertable<vector<wchar_t>> = true;
#  endif
#endif // _LIBCPP_STD_VER >= 20

_LIBCPP_END_NAMESPACE_STD

_LIBCPP_POP_MACROS

#endif // _LIBCPP___VECTOR_VECTOR_H
