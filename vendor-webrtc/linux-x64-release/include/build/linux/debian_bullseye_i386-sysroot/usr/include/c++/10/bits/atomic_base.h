// -*- C++ -*- header.

// Copyright (C) 2008-2020 Free Software Foundation, Inc.
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

/** @file bits/atomic_base.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{atomic}
 */

#ifndef _GLIBCXX_ATOMIC_BASE_H
#define _GLIBCXX_ATOMIC_BASE_H 1

#pragma GCC system_header

#include <bits/c++config.h>
#include <stdint.h>
#include <bits/atomic_lockfree_defines.h>
#include <bits/move.h>

#ifndef _GLIBCXX_ALWAYS_INLINE
#define _GLIBCXX_ALWAYS_INLINE inline __attribute__((__always_inline__))
#endif

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  /**
   * @defgroup atomics Atomics
   *
   * Components for performing atomic operations.
   * @{
   */

  /// Enumeration for memory_order
#if __cplusplus > 201703L
  enum class memory_order : int
    {
      relaxed,
      consume,
      acquire,
      release,
      acq_rel,
      seq_cst
    };

  inline constexpr memory_order memory_order_relaxed = memory_order::relaxed;
  inline constexpr memory_order memory_order_consume = memory_order::consume;
  inline constexpr memory_order memory_order_acquire = memory_order::acquire;
  inline constexpr memory_order memory_order_release = memory_order::release;
  inline constexpr memory_order memory_order_acq_rel = memory_order::acq_rel;
  inline constexpr memory_order memory_order_seq_cst = memory_order::seq_cst;
#else
  typedef enum memory_order
    {
      memory_order_relaxed,
      memory_order_consume,
      memory_order_acquire,
      memory_order_release,
      memory_order_acq_rel,
      memory_order_seq_cst
    } memory_order;
#endif

  enum __memory_order_modifier
    {
      __memory_order_mask          = 0x0ffff,
      __memory_order_modifier_mask = 0xffff0000,
      __memory_order_hle_acquire   = 0x10000,
      __memory_order_hle_release   = 0x20000
    };

  constexpr memory_order
  operator|(memory_order __m, __memory_order_modifier __mod)
  {
    return memory_order(int(__m) | int(__mod));
  }

  constexpr memory_order
  operator&(memory_order __m, __memory_order_modifier __mod)
  {
    return memory_order(int(__m) & int(__mod));
  }

  // Drop release ordering as per [atomics.types.operations.req]/21
  constexpr memory_order
  __cmpexch_failure_order2(memory_order __m) noexcept
  {
    return __m == memory_order_acq_rel ? memory_order_acquire
      : __m == memory_order_release ? memory_order_relaxed : __m;
  }

  constexpr memory_order
  __cmpexch_failure_order(memory_order __m) noexcept
  {
    return memory_order(__cmpexch_failure_order2(__m & __memory_order_mask)
      | __memory_order_modifier(__m & __memory_order_modifier_mask));
  }

  _GLIBCXX_ALWAYS_INLINE void
  atomic_thread_fence(memory_order __m) noexcept
  { __atomic_thread_fence(int(__m)); }

  _GLIBCXX_ALWAYS_INLINE void
  atomic_signal_fence(memory_order __m) noexcept
  { __atomic_signal_fence(int(__m)); }

  /// kill_dependency
  template<typename _Tp>
    inline _Tp
    kill_dependency(_Tp __y) noexcept
    {
      _Tp __ret(__y);
      return __ret;
    }


  // Base types for atomics.
  template<typename _IntTp>
    struct __atomic_base;

#if __cplusplus <= 201703L
# define _GLIBCXX20_INIT(I)
#else
# define __cpp_lib_atomic_value_initialization 201911L
# define _GLIBCXX20_INIT(I) = I
#endif

#define ATOMIC_VAR_INIT(_VI) { _VI }

  template<typename _Tp>
    struct atomic;

  template<typename _Tp>
    struct atomic<_Tp*>;

    /* The target's "set" value for test-and-set may not be exactly 1.  */
#if __GCC_ATOMIC_TEST_AND_SET_TRUEVAL == 1
    typedef bool __atomic_flag_data_type;
#else
    typedef unsigned char __atomic_flag_data_type;
#endif

  /**
   *  @brief Base type for atomic_flag.
   *
   *  Base type is POD with data, allowing atomic_flag to derive from
   *  it and meet the standard layout type requirement. In addition to
   *  compatibility with a C interface, this allows different
   *  implementations of atomic_flag to use the same atomic operation
   *  functions, via a standard conversion to the __atomic_flag_base
   *  argument.
  */
  _GLIBCXX_BEGIN_EXTERN_C

  struct __atomic_flag_base
  {
    __atomic_flag_data_type _M_i _GLIBCXX20_INIT({});
  };

  _GLIBCXX_END_EXTERN_C

#define ATOMIC_FLAG_INIT { 0 }

  /// atomic_flag
  struct atomic_flag : public __atomic_flag_base
  {
    atomic_flag() noexcept = default;
    ~atomic_flag() noexcept = default;
    atomic_flag(const atomic_flag&) = delete;
    atomic_flag& operator=(const atomic_flag&) = delete;
    atomic_flag& operator=(const atomic_flag&) volatile = delete;

    // Conversion to ATOMIC_FLAG_INIT.
    constexpr atomic_flag(bool __i) noexcept
      : __atomic_flag_base{ _S_init(__i) }
    { }

    _GLIBCXX_ALWAYS_INLINE bool
    test_and_set(memory_order __m = memory_order_seq_cst) noexcept
    {
      return __atomic_test_and_set (&_M_i, int(__m));
    }

    _GLIBCXX_ALWAYS_INLINE bool
    test_and_set(memory_order __m = memory_order_seq_cst) volatile noexcept
    {
      return __atomic_test_and_set (&_M_i, int(__m));
    }

    _GLIBCXX_ALWAYS_INLINE void
    clear(memory_order __m = memory_order_seq_cst) noexcept
    {
      memory_order __b = __m & __memory_order_mask;
      __glibcxx_assert(__b != memory_order_consume);
      __glibcxx_assert(__b != memory_order_acquire);
      __glibcxx_assert(__b != memory_order_acq_rel);

      __atomic_clear (&_M_i, int(__m));
    }

    _GLIBCXX_ALWAYS_INLINE void
    clear(memory_order __m = memory_order_seq_cst) volatile noexcept
    {
      memory_order __b = __m & __memory_order_mask;
      __glibcxx_assert(__b != memory_order_consume);
      __glibcxx_assert(__b != memory_order_acquire);
      __glibcxx_assert(__b != memory_order_acq_rel);

      __atomic_clear (&_M_i, int(__m));
    }

  private:
    static constexpr __atomic_flag_data_type
    _S_init(bool __i)
    { return __i ? __GCC_ATOMIC_TEST_AND_SET_TRUEVAL : 0; }
  };


  /// Base class for atomic integrals.
  //
  // For each of the integral types, define atomic_[integral type] struct
  //
  // atomic_bool     bool
  // atomic_char     char
  // atomic_schar    signed char
  // atomic_uchar    unsigned char
  // atomic_short    short
  // atomic_ushort   unsigned short
  // atomic_int      int
  // atomic_uint     unsigned int
  // atomic_long     long
  // atomic_ulong    unsigned long
  // atomic_llong    long long
  // atomic_ullong   unsigned long long
  // atomic_char8_t  char8_t
  // atomic_char16_t char16_t
  // atomic_char32_t char32_t
  // atomic_wchar_t  wchar_t
  //
  // NB: Assuming _ITp is an integral scalar type that is 1, 2, 4, or
  // 8 bytes, since that is what GCC built-in functions for atomic
  // memory access expect.
  template<typename _ITp>
    struct __atomic_base
    {
      using value_type = _ITp;
      using difference_type = value_type;

    private:
      typedef _ITp 	__int_type;

      static constexpr int _S_alignment =
	sizeof(_ITp) > alignof(_ITp) ? sizeof(_ITp) : alignof(_ITp);

      alignas(_S_alignment) __int_type _M_i _GLIBCXX20_INIT(0);

    public:
      __atomic_base() noexcept = default;
      ~__atomic_base() noexcept = default;
      __atomic_base(const __atomic_base&) = delete;
      __atomic_base& operator=(const __atomic_base&) = delete;
      __atomic_base& operator=(const __atomic_base&) volatile = delete;

      // Requires __int_type convertible to _M_i.
      constexpr __atomic_base(__int_type __i) noexcept : _M_i (__i) { }

      operator __int_type() const noexcept
      { return load(); }

      operator __int_type() const volatile noexcept
      { return load(); }

      __int_type
      operator=(__int_type __i) noexcept
      {
	store(__i);
	return __i;
      }

      __int_type
      operator=(__int_type __i) volatile noexcept
      {
	store(__i);
	return __i;
      }

      __int_type
      operator++(int) noexcept
      { return fetch_add(1); }

      __int_type
      operator++(int) volatile noexcept
      { return fetch_add(1); }

      __int_type
      operator--(int) noexcept
      { return fetch_sub(1); }

      __int_type
      operator--(int) volatile noexcept
      { return fetch_sub(1); }

      __int_type
      operator++() noexcept
      { return __atomic_add_fetch(&_M_i, 1, int(memory_order_seq_cst)); }

      __int_type
      operator++() volatile noexcept
      { return __atomic_add_fetch(&_M_i, 1, int(memory_order_seq_cst)); }

      __int_type
      operator--() noexcept
      { return __atomic_sub_fetch(&_M_i, 1, int(memory_order_seq_cst)); }

      __int_type
      operator--() volatile noexcept
      { return __atomic_sub_fetch(&_M_i, 1, int(memory_order_seq_cst)); }

      __int_type
      operator+=(__int_type __i) noexcept
      { return __atomic_add_fetch(&_M_i, __i, int(memory_order_seq_cst)); }

      __int_type
      operator+=(__int_type __i) volatile noexcept
      { return __atomic_add_fetch(&_M_i, __i, int(memory_order_seq_cst)); }

      __int_type
      operator-=(__int_type __i) noexcept
      { return __atomic_sub_fetch(&_M_i, __i, int(memory_order_seq_cst)); }

      __int_type
      operator-=(__int_type __i) volatile noexcept
      { return __atomic_sub_fetch(&_M_i, __i, int(memory_order_seq_cst)); }

      __int_type
      operator&=(__int_type __i) noexcept
      { return __atomic_and_fetch(&_M_i, __i, int(memory_order_seq_cst)); }

      __int_type
      operator&=(__int_type __i) volatile noexcept
      { return __atomic_and_fetch(&_M_i, __i, int(memory_order_seq_cst)); }

      __int_type
      operator|=(__int_type __i) noexcept
      { return __atomic_or_fetch(&_M_i, __i, int(memory_order_seq_cst)); }

      __int_type
      operator|=(__int_type __i) volatile noexcept
      { return __atomic_or_fetch(&_M_i, __i, int(memory_order_seq_cst)); }

      __int_type
      operator^=(__int_type __i) noexcept
      { return __atomic_xor_fetch(&_M_i, __i, int(memory_order_seq_cst)); }

      __int_type
      operator^=(__int_type __i) volatile noexcept
      { return __atomic_xor_fetch(&_M_i, __i, int(memory_order_seq_cst)); }

      bool
      is_lock_free() const noexcept
      {
	// Use a fake, minimally aligned pointer.
	return __atomic_is_lock_free(sizeof(_M_i),
	    reinterpret_cast<void *>(-_S_alignment));
      }

      bool
      is_lock_free() const volatile noexcept
      {
	// Use a fake, minimally aligned pointer.
	return __atomic_is_lock_free(sizeof(_M_i),
	    reinterpret_cast<void *>(-_S_alignment));
      }

      _GLIBCXX_ALWAYS_INLINE void
      store(__int_type __i, memory_order __m = memory_order_seq_cst) noexcept
      {
	memory_order __b = __m & __memory_order_mask;
	__glibcxx_assert(__b != memory_order_acquire);
	__glibcxx_assert(__b != memory_order_acq_rel);
	__glibcxx_assert(__b != memory_order_consume);

	__atomic_store_n(&_M_i, __i, int(__m));
      }

      _GLIBCXX_ALWAYS_INLINE void
      store(__int_type __i,
	    memory_order __m = memory_order_seq_cst) volatile noexcept
      {
	memory_order __b = __m & __memory_order_mask;
	__glibcxx_assert(__b != memory_order_acquire);
	__glibcxx_assert(__b != memory_order_acq_rel);
	__glibcxx_assert(__b != memory_order_consume);

	__atomic_store_n(&_M_i, __i, int(__m));
      }

      _GLIBCXX_ALWAYS_INLINE __int_type
      load(memory_order __m = memory_order_seq_cst) const noexcept
      {
	memory_order __b = __m & __memory_order_mask;
	__glibcxx_assert(__b != memory_order_release);
	__glibcxx_assert(__b != memory_order_acq_rel);

	return __atomic_load_n(&_M_i, int(__m));
      }

      _GLIBCXX_ALWAYS_INLINE __int_type
      load(memory_order __m = memory_order_seq_cst) const volatile noexcept
      {
	memory_order __b = __m & __memory_order_mask;
	__glibcxx_assert(__b != memory_order_release);
	__glibcxx_assert(__b != memory_order_acq_rel);

	return __atomic_load_n(&_M_i, int(__m));
      }

      _GLIBCXX_ALWAYS_INLINE __int_type
      exchange(__int_type __i,
	       memory_order __m = memory_order_seq_cst) noexcept
      {
	return __atomic_exchange_n(&_M_i, __i, int(__m));
      }


      _GLIBCXX_ALWAYS_INLINE __int_type
      exchange(__int_type __i,
	       memory_order __m = memory_order_seq_cst) volatile noexcept
      {
	return __atomic_exchange_n(&_M_i, __i, int(__m));
      }

      _GLIBCXX_ALWAYS_INLINE bool
      compare_exchange_weak(__int_type& __i1, __int_type __i2,
			    memory_order __m1, memory_order __m2) noexcept
      {
	memory_order __b2 = __m2 & __memory_order_mask;
	memory_order __b1 = __m1 & __memory_order_mask;
	__glibcxx_assert(__b2 != memory_order_release);
	__glibcxx_assert(__b2 != memory_order_acq_rel);
	__glibcxx_assert(__b2 <= __b1);

	return __atomic_compare_exchange_n(&_M_i, &__i1, __i2, 1,
					   int(__m1), int(__m2));
      }

      _GLIBCXX_ALWAYS_INLINE bool
      compare_exchange_weak(__int_type& __i1, __int_type __i2,
			    memory_order __m1,
			    memory_order __m2) volatile noexcept
      {
	memory_order __b2 = __m2 & __memory_order_mask;
	memory_order __b1 = __m1 & __memory_order_mask;
	__glibcxx_assert(__b2 != memory_order_release);
	__glibcxx_assert(__b2 != memory_order_acq_rel);
	__glibcxx_assert(__b2 <= __b1);

	return __atomic_compare_exchange_n(&_M_i, &__i1, __i2, 1,
					   int(__m1), int(__m2));
      }

      _GLIBCXX_ALWAYS_INLINE bool
      compare_exchange_weak(__int_type& __i1, __int_type __i2,
			    memory_order __m = memory_order_seq_cst) noexcept
      {
	return compare_exchange_weak(__i1, __i2, __m,
				     __cmpexch_failure_order(__m));
      }

      _GLIBCXX_ALWAYS_INLINE bool
      compare_exchange_weak(__int_type& __i1, __int_type __i2,
		   memory_order __m = memory_order_seq_cst) volatile noexcept
      {
	return compare_exchange_weak(__i1, __i2, __m,
				     __cmpexch_failure_order(__m));
      }

      _GLIBCXX_ALWAYS_INLINE bool
      compare_exchange_strong(__int_type& __i1, __int_type __i2,
			      memory_order __m1, memory_order __m2) noexcept
      {
	memory_order __b2 = __m2 & __memory_order_mask;
	memory_order __b1 = __m1 & __memory_order_mask;
	__glibcxx_assert(__b2 != memory_order_release);
	__glibcxx_assert(__b2 != memory_order_acq_rel);
	__glibcxx_assert(__b2 <= __b1);

	return __atomic_compare_exchange_n(&_M_i, &__i1, __i2, 0,
					   int(__m1), int(__m2));
      }

      _GLIBCXX_ALWAYS_INLINE bool
      compare_exchange_strong(__int_type& __i1, __int_type __i2,
			      memory_order __m1,
			      memory_order __m2) volatile noexcept
      {
	memory_order __b2 = __m2 & __memory_order_mask;
	memory_order __b1 = __m1 & __memory_order_mask;

	__glibcxx_assert(__b2 != memory_order_release);
	__glibcxx_assert(__b2 != memory_order_acq_rel);
	__glibcxx_assert(__b2 <= __b1);

	return __atomic_compare_exchange_n(&_M_i, &__i1, __i2, 0,
					   int(__m1), int(__m2));
      }

      _GLIBCXX_ALWAYS_INLINE bool
      compare_exchange_strong(__int_type& __i1, __int_type __i2,
			      memory_order __m = memory_order_seq_cst) noexcept
      {
	return compare_exchange_strong(__i1, __i2, __m,
				       __cmpexch_failure_order(__m));
      }

      _GLIBCXX_ALWAYS_INLINE bool
      compare_exchange_strong(__int_type& __i1, __int_type __i2,
		 memory_order __m = memory_order_seq_cst) volatile noexcept
      {
	return compare_exchange_strong(__i1, __i2, __m,
				       __cmpexch_failure_order(__m));
      }

      _GLIBCXX_ALWAYS_INLINE __int_type
      fetch_add(__int_type __i,
		memory_order __m = memory_order_seq_cst) noexcept
      { return __atomic_fetch_add(&_M_i, __i, int(__m)); }

      _GLIBCXX_ALWAYS_INLINE __int_type
      fetch_add(__int_type __i,
		memory_order __m = memory_order_seq_cst) volatile noexcept
      { return __atomic_fetch_add(&_M_i, __i, int(__m)); }

      _GLIBCXX_ALWAYS_INLINE __int_type
      fetch_sub(__int_type __i,
		memory_order __m = memory_order_seq_cst) noexcept
      { return __atomic_fetch_sub(&_M_i, __i, int(__m)); }

      _GLIBCXX_ALWAYS_INLINE __int_type
      fetch_sub(__int_type __i,
		memory_order __m = memory_order_seq_cst) volatile noexcept
      { return __atomic_fetch_sub(&_M_i, __i, int(__m)); }

      _GLIBCXX_ALWAYS_INLINE __int_type
      fetch_and(__int_type __i,
		memory_order __m = memory_order_seq_cst) noexcept
      { return __atomic_fetch_and(&_M_i, __i, int(__m)); }

      _GLIBCXX_ALWAYS_INLINE __int_type
      fetch_and(__int_type __i,
		memory_order __m = memory_order_seq_cst) volatile noexcept
      { return __atomic_fetch_and(&_M_i, __i, int(__m)); }

      _GLIBCXX_ALWAYS_INLINE __int_type
      fetch_or(__int_type __i,
	       memory_order __m = memory_order_seq_cst) noexcept
      { return __atomic_fetch_or(&_M_i, __i, int(__m)); }

      _GLIBCXX_ALWAYS_INLINE __int_type
      fetch_or(__int_type __i,
	       memory_order __m = memory_order_seq_cst) volatile noexcept
      { return __atomic_fetch_or(&_M_i, __i, int(__m)); }

      _GLIBCXX_ALWAYS_INLINE __int_type
      fetch_xor(__int_type __i,
		memory_order __m = memory_order_seq_cst) noexcept
      { return __atomic_fetch_xor(&_M_i, __i, int(__m)); }

      _GLIBCXX_ALWAYS_INLINE __int_type
      fetch_xor(__int_type __i,
		memory_order __m = memory_order_seq_cst) volatile noexcept
      { return __atomic_fetch_xor(&_M_i, __i, int(__m)); }
    };


  /// Partial specialization for pointer types.
  template<typename _PTp>
    struct __atomic_base<_PTp*>
    {
    private:
      typedef _PTp* 	__pointer_type;

      __pointer_type 	_M_p _GLIBCXX20_INIT(nullptr);

      // Factored out to facilitate explicit specialization.
      constexpr ptrdiff_t
      _M_type_size(ptrdiff_t __d) const { return __d * sizeof(_PTp); }

      constexpr ptrdiff_t
      _M_type_size(ptrdiff_t __d) const volatile { return __d * sizeof(_PTp); }

    public:
      __atomic_base() noexcept = default;
      ~__atomic_base() noexcept = default;
      __atomic_base(const __atomic_base&) = delete;
      __atomic_base& operator=(const __atomic_base&) = delete;
      __atomic_base& operator=(const __atomic_base&) volatile = delete;

      // Requires __pointer_type convertible to _M_p.
      constexpr __atomic_base(__pointer_type __p) noexcept : _M_p (__p) { }

      operator __pointer_type() const noexcept
      { return load(); }

      operator __pointer_type() const volatile noexcept
      { return load(); }

      __pointer_type
      operator=(__pointer_type __p) noexcept
      {
	store(__p);
	return __p;
      }

      __pointer_type
      operator=(__pointer_type __p) volatile noexcept
      {
	store(__p);
	return __p;
      }

      __pointer_type
      operator++(int) noexcept
      { return fetch_add(1); }

      __pointer_type
      operator++(int) volatile noexcept
      { return fetch_add(1); }

      __pointer_type
      operator--(int) noexcept
      { return fetch_sub(1); }

      __pointer_type
      operator--(int) volatile noexcept
      { return fetch_sub(1); }

      __pointer_type
      operator++() noexcept
      { return __atomic_add_fetch(&_M_p, _M_type_size(1),
				  int(memory_order_seq_cst)); }

      __pointer_type
      operator++() volatile noexcept
      { return __atomic_add_fetch(&_M_p, _M_type_size(1),
				  int(memory_order_seq_cst)); }

      __pointer_type
      operator--() noexcept
      { return __atomic_sub_fetch(&_M_p, _M_type_size(1),
				  int(memory_order_seq_cst)); }

      __pointer_type
      operator--() volatile noexcept
      { return __atomic_sub_fetch(&_M_p, _M_type_size(1),
				  int(memory_order_seq_cst)); }

      __pointer_type
      operator+=(ptrdiff_t __d) noexcept
      { return __atomic_add_fetch(&_M_p, _M_type_size(__d),
				  int(memory_order_seq_cst)); }

      __pointer_type
      operator+=(ptrdiff_t __d) volatile noexcept
      { return __atomic_add_fetch(&_M_p, _M_type_size(__d),
				  int(memory_order_seq_cst)); }

      __pointer_type
      operator-=(ptrdiff_t __d) noexcept
      { return __atomic_sub_fetch(&_M_p, _M_type_size(__d),
				  int(memory_order_seq_cst)); }

      __pointer_type
      operator-=(ptrdiff_t __d) volatile noexcept
      { return __atomic_sub_fetch(&_M_p, _M_type_size(__d),
				  int(memory_order_seq_cst)); }

      bool
      is_lock_free() const noexcept
      {
	// Produce a fake, minimally aligned pointer.
	return __atomic_is_lock_free(sizeof(_M_p),
	    reinterpret_cast<void *>(-__alignof(_M_p)));
      }

      bool
      is_lock_free() const volatile noexcept
      {
	// Produce a fake, minimally aligned pointer.
	return __atomic_is_lock_free(sizeof(_M_p),
	    reinterpret_cast<void *>(-__alignof(_M_p)));
      }

      _GLIBCXX_ALWAYS_INLINE void
      store(__pointer_type __p,
	    memory_order __m = memory_order_seq_cst) noexcept
      {
        memory_order __b = __m & __memory_order_mask;

	__glibcxx_assert(__b != memory_order_acquire);
	__glibcxx_assert(__b != memory_order_acq_rel);
	__glibcxx_assert(__b != memory_order_consume);

	__atomic_store_n(&_M_p, __p, int(__m));
      }

      _GLIBCXX_ALWAYS_INLINE void
      store(__pointer_type __p,
	    memory_order __m = memory_order_seq_cst) volatile noexcept
      {
	memory_order __b = __m & __memory_order_mask;
	__glibcxx_assert(__b != memory_order_acquire);
	__glibcxx_assert(__b != memory_order_acq_rel);
	__glibcxx_assert(__b != memory_order_consume);

	__atomic_store_n(&_M_p, __p, int(__m));
      }

      _GLIBCXX_ALWAYS_INLINE __pointer_type
      load(memory_order __m = memory_order_seq_cst) const noexcept
      {
	memory_order __b = __m & __memory_order_mask;
	__glibcxx_assert(__b != memory_order_release);
	__glibcxx_assert(__b != memory_order_acq_rel);

	return __atomic_load_n(&_M_p, int(__m));
      }

      _GLIBCXX_ALWAYS_INLINE __pointer_type
      load(memory_order __m = memory_order_seq_cst) const volatile noexcept
      {
	memory_order __b = __m & __memory_order_mask;
	__glibcxx_assert(__b != memory_order_release);
	__glibcxx_assert(__b != memory_order_acq_rel);

	return __atomic_load_n(&_M_p, int(__m));
      }

      _GLIBCXX_ALWAYS_INLINE __pointer_type
      exchange(__pointer_type __p,
	       memory_order __m = memory_order_seq_cst) noexcept
      {
	return __atomic_exchange_n(&_M_p, __p, int(__m));
      }


      _GLIBCXX_ALWAYS_INLINE __pointer_type
      exchange(__pointer_type __p,
	       memory_order __m = memory_order_seq_cst) volatile noexcept
      {
	return __atomic_exchange_n(&_M_p, __p, int(__m));
      }

      _GLIBCXX_ALWAYS_INLINE bool
      compare_exchange_strong(__pointer_type& __p1, __pointer_type __p2,
			      memory_order __m1,
			      memory_order __m2) noexcept
      {
	memory_order __b2 = __m2 & __memory_order_mask;
	memory_order __b1 = __m1 & __memory_order_mask;
	__glibcxx_assert(__b2 != memory_order_release);
	__glibcxx_assert(__b2 != memory_order_acq_rel);
	__glibcxx_assert(__b2 <= __b1);

	return __atomic_compare_exchange_n(&_M_p, &__p1, __p2, 0,
					   int(__m1), int(__m2));
      }

      _GLIBCXX_ALWAYS_INLINE bool
      compare_exchange_strong(__pointer_type& __p1, __pointer_type __p2,
			      memory_order __m1,
			      memory_order __m2) volatile noexcept
      {
	memory_order __b2 = __m2 & __memory_order_mask;
	memory_order __b1 = __m1 & __memory_order_mask;

	__glibcxx_assert(__b2 != memory_order_release);
	__glibcxx_assert(__b2 != memory_order_acq_rel);
	__glibcxx_assert(__b2 <= __b1);

	return __atomic_compare_exchange_n(&_M_p, &__p1, __p2, 0,
					   int(__m1), int(__m2));
      }

      _GLIBCXX_ALWAYS_INLINE __pointer_type
      fetch_add(ptrdiff_t __d,
		memory_order __m = memory_order_seq_cst) noexcept
      { return __atomic_fetch_add(&_M_p, _M_type_size(__d), int(__m)); }

      _GLIBCXX_ALWAYS_INLINE __pointer_type
      fetch_add(ptrdiff_t __d,
		memory_order __m = memory_order_seq_cst) volatile noexcept
      { return __atomic_fetch_add(&_M_p, _M_type_size(__d), int(__m)); }

      _GLIBCXX_ALWAYS_INLINE __pointer_type
      fetch_sub(ptrdiff_t __d,
		memory_order __m = memory_order_seq_cst) noexcept
      { return __atomic_fetch_sub(&_M_p, _M_type_size(__d), int(__m)); }

      _GLIBCXX_ALWAYS_INLINE __pointer_type
      fetch_sub(ptrdiff_t __d,
		memory_order __m = memory_order_seq_cst) volatile noexcept
      { return __atomic_fetch_sub(&_M_p, _M_type_size(__d), int(__m)); }
    };

#if __cplusplus > 201703L
  // Implementation details of atomic_ref and atomic<floating-point>.
  namespace __atomic_impl
  {
    // Remove volatile and create a non-deduced context for value arguments.
    template<typename _Tp>
      using _Val = remove_volatile_t<_Tp>;

    // As above, but for difference_type arguments.
    template<typename _Tp>
      using _Diff = conditional_t<is_pointer_v<_Tp>, ptrdiff_t, _Val<_Tp>>;

    template<size_t _Size, size_t _Align>
      _GLIBCXX_ALWAYS_INLINE bool
      is_lock_free() noexcept
      {
	// Produce a fake, minimally aligned pointer.
	return __atomic_is_lock_free(_Size, reinterpret_cast<void *>(-_Align));
      }

    template<typename _Tp>
      _GLIBCXX_ALWAYS_INLINE void
      store(_Tp* __ptr, _Val<_Tp> __t, memory_order __m) noexcept
      { __atomic_store(__ptr, std::__addressof(__t), int(__m)); }

    template<typename _Tp>
      _GLIBCXX_ALWAYS_INLINE _Val<_Tp>
      load(const _Tp* __ptr, memory_order __m) noexcept
      {
	alignas(_Tp) unsigned char __buf[sizeof(_Tp)];
	auto* __dest = reinterpret_cast<_Val<_Tp>*>(__buf);
	__atomic_load(__ptr, __dest, int(__m));
	return *__dest;
      }

    template<typename _Tp>
      _GLIBCXX_ALWAYS_INLINE _Val<_Tp>
      exchange(_Tp* __ptr, _Val<_Tp> __desired, memory_order __m) noexcept
      {
        alignas(_Tp) unsigned char __buf[sizeof(_Tp)];
	auto* __dest = reinterpret_cast<_Val<_Tp>*>(__buf);
	__atomic_exchange(__ptr, std::__addressof(__desired), __dest, int(__m));
	return *__dest;
      }

    template<typename _Tp>
      _GLIBCXX_ALWAYS_INLINE bool
      compare_exchange_weak(_Tp* __ptr, _Val<_Tp>& __expected,
			    _Val<_Tp> __desired, memory_order __success,
			    memory_order __failure) noexcept
      {
	return __atomic_compare_exchange(__ptr, std::__addressof(__expected),
					 std::__addressof(__desired), true,
					 int(__success), int(__failure));
      }

    template<typename _Tp>
      _GLIBCXX_ALWAYS_INLINE bool
      compare_exchange_strong(_Tp* __ptr, _Val<_Tp>& __expected,
			      _Val<_Tp> __desired, memory_order __success,
			      memory_order __failure) noexcept
      {
	return __atomic_compare_exchange(__ptr, std::__addressof(__expected),
					 std::__addressof(__desired), false,
					 int(__success), int(__failure));
      }

    template<typename _Tp>
      _GLIBCXX_ALWAYS_INLINE _Tp
      fetch_add(_Tp* __ptr, _Diff<_Tp> __i, memory_order __m) noexcept
      { return __atomic_fetch_add(__ptr, __i, int(__m)); }

    template<typename _Tp>
      _GLIBCXX_ALWAYS_INLINE _Tp
      fetch_sub(_Tp* __ptr, _Diff<_Tp> __i, memory_order __m) noexcept
      { return __atomic_fetch_sub(__ptr, __i, int(__m)); }

    template<typename _Tp>
      _GLIBCXX_ALWAYS_INLINE _Tp
      fetch_and(_Tp* __ptr, _Val<_Tp> __i, memory_order __m) noexcept
      { return __atomic_fetch_and(__ptr, __i, int(__m)); }

    template<typename _Tp>
      _GLIBCXX_ALWAYS_INLINE _Tp
      fetch_or(_Tp* __ptr, _Val<_Tp> __i, memory_order __m) noexcept
      { return __atomic_fetch_or(__ptr, __i, int(__m)); }

    template<typename _Tp>
      _GLIBCXX_ALWAYS_INLINE _Tp
      fetch_xor(_Tp* __ptr, _Val<_Tp> __i, memory_order __m) noexcept
      { return __atomic_fetch_xor(__ptr, __i, int(__m)); }

    template<typename _Tp>
      _GLIBCXX_ALWAYS_INLINE _Tp
      __add_fetch(_Tp* __ptr, _Diff<_Tp> __i) noexcept
      { return __atomic_add_fetch(__ptr, __i, __ATOMIC_SEQ_CST); }

    template<typename _Tp>
      _GLIBCXX_ALWAYS_INLINE _Tp
      __sub_fetch(_Tp* __ptr, _Diff<_Tp> __i) noexcept
      { return __atomic_sub_fetch(__ptr, __i, __ATOMIC_SEQ_CST); }

    template<typename _Tp>
      _GLIBCXX_ALWAYS_INLINE _Tp
      __and_fetch(_Tp* __ptr, _Val<_Tp> __i) noexcept
      { return __atomic_and_fetch(__ptr, __i, __ATOMIC_SEQ_CST); }

    template<typename _Tp>
      _GLIBCXX_ALWAYS_INLINE _Tp
      __or_fetch(_Tp* __ptr, _Val<_Tp> __i) noexcept
      { return __atomic_or_fetch(__ptr, __i, __ATOMIC_SEQ_CST); }

    template<typename _Tp>
      _GLIBCXX_ALWAYS_INLINE _Tp
      __xor_fetch(_Tp* __ptr, _Val<_Tp> __i) noexcept
      { return __atomic_xor_fetch(__ptr, __i, __ATOMIC_SEQ_CST); }

    template<typename _Tp>
      _Tp
      __fetch_add_flt(_Tp* __ptr, _Val<_Tp> __i, memory_order __m) noexcept
      {
	_Val<_Tp> __oldval = load(__ptr, memory_order_relaxed);
	_Val<_Tp> __newval = __oldval + __i;
	while (!compare_exchange_weak(__ptr, __oldval, __newval, __m,
				      memory_order_relaxed))
	  __newval = __oldval + __i;
	return __oldval;
      }

    template<typename _Tp>
      _Tp
      __fetch_sub_flt(_Tp* __ptr, _Val<_Tp> __i, memory_order __m) noexcept
      {
	_Val<_Tp> __oldval = load(__ptr, memory_order_relaxed);
	_Val<_Tp> __newval = __oldval - __i;
	while (!compare_exchange_weak(__ptr, __oldval, __newval, __m,
				      memory_order_relaxed))
	  __newval = __oldval - __i;
	return __oldval;
      }

    template<typename _Tp>
      _Tp
      __add_fetch_flt(_Tp* __ptr, _Val<_Tp> __i) noexcept
      {
	_Val<_Tp> __oldval = load(__ptr, memory_order_relaxed);
	_Val<_Tp> __newval = __oldval + __i;
	while (!compare_exchange_weak(__ptr, __oldval, __newval,
				      memory_order_seq_cst,
				      memory_order_relaxed))
	  __newval = __oldval + __i;
	return __newval;
      }

    template<typename _Tp>
      _Tp
      __sub_fetch_flt(_Tp* __ptr, _Val<_Tp> __i) noexcept
      {
	_Val<_Tp> __oldval = load(__ptr, memory_order_relaxed);
	_Val<_Tp> __newval = __oldval - __i;
	while (!compare_exchange_weak(__ptr, __oldval, __newval,
				      memory_order_seq_cst,
				      memory_order_relaxed))
	  __newval = __oldval - __i;
	return __newval;
      }
  } // namespace __atomic_impl

  // base class for atomic<floating-point-type>
  template<typename _Fp>
    struct __atomic_float
    {
      static_assert(is_floating_point_v<_Fp>);

      static constexpr size_t _S_alignment = __alignof__(_Fp);

    public:
      using value_type = _Fp;
      using difference_type = value_type;

      static constexpr bool is_always_lock_free
	= __atomic_always_lock_free(sizeof(_Fp), 0);

      __atomic_float() = default;

      constexpr
      __atomic_float(_Fp __t) : _M_fp(__t)
      { }

      __atomic_float(const __atomic_float&) = delete;
      __atomic_float& operator=(const __atomic_float&) = delete;
      __atomic_float& operator=(const __atomic_float&) volatile = delete;

      _Fp
      operator=(_Fp __t) volatile noexcept
      {
	this->store(__t);
	return __t;
      }

      _Fp
      operator=(_Fp __t) noexcept
      {
	this->store(__t);
	return __t;
      }

      bool
      is_lock_free() const volatile noexcept
      { return __atomic_impl::is_lock_free<sizeof(_Fp), _S_alignment>(); }

      bool
      is_lock_free() const noexcept
      { return __atomic_impl::is_lock_free<sizeof(_Fp), _S_alignment>(); }

      void
      store(_Fp __t, memory_order __m = memory_order_seq_cst) volatile noexcept
      { __atomic_impl::store(&_M_fp, __t, __m); }

      void
      store(_Fp __t, memory_order __m = memory_order_seq_cst) noexcept
      { __atomic_impl::store(&_M_fp, __t, __m); }

      _Fp
      load(memory_order __m = memory_order_seq_cst) const volatile noexcept
      { return __atomic_impl::load(&_M_fp, __m); }

      _Fp
      load(memory_order __m = memory_order_seq_cst) const noexcept
      { return __atomic_impl::load(&_M_fp, __m); }

      operator _Fp() const volatile noexcept { return this->load(); }
      operator _Fp() const noexcept { return this->load(); }

      _Fp
      exchange(_Fp __desired,
	       memory_order __m = memory_order_seq_cst) volatile noexcept
      { return __atomic_impl::exchange(&_M_fp, __desired, __m); }

      _Fp
      exchange(_Fp __desired,
	       memory_order __m = memory_order_seq_cst) noexcept
      { return __atomic_impl::exchange(&_M_fp, __desired, __m); }

      bool
      compare_exchange_weak(_Fp& __expected, _Fp __desired,
			    memory_order __success,
			    memory_order __failure) noexcept
      {
	return __atomic_impl::compare_exchange_weak(&_M_fp,
						    __expected, __desired,
						    __success, __failure);
      }

      bool
      compare_exchange_weak(_Fp& __expected, _Fp __desired,
			    memory_order __success,
			    memory_order __failure) volatile noexcept
      {
	return __atomic_impl::compare_exchange_weak(&_M_fp,
						    __expected, __desired,
						    __success, __failure);
      }

      bool
      compare_exchange_strong(_Fp& __expected, _Fp __desired,
			      memory_order __success,
			      memory_order __failure) noexcept
      {
	return __atomic_impl::compare_exchange_strong(&_M_fp,
						      __expected, __desired,
						      __success, __failure);
      }

      bool
      compare_exchange_strong(_Fp& __expected, _Fp __desired,
			      memory_order __success,
			      memory_order __failure) volatile noexcept
      {
	return __atomic_impl::compare_exchange_strong(&_M_fp,
						      __expected, __desired,
						      __success, __failure);
      }

      bool
      compare_exchange_weak(_Fp& __expected, _Fp __desired,
			    memory_order __order = memory_order_seq_cst)
      noexcept
      {
	return compare_exchange_weak(__expected, __desired, __order,
                                     __cmpexch_failure_order(__order));
      }

      bool
      compare_exchange_weak(_Fp& __expected, _Fp __desired,
			    memory_order __order = memory_order_seq_cst)
      volatile noexcept
      {
	return compare_exchange_weak(__expected, __desired, __order,
                                     __cmpexch_failure_order(__order));
      }

      bool
      compare_exchange_strong(_Fp& __expected, _Fp __desired,
			      memory_order __order = memory_order_seq_cst)
      noexcept
      {
	return compare_exchange_strong(__expected, __desired, __order,
				       __cmpexch_failure_order(__order));
      }

      bool
      compare_exchange_strong(_Fp& __expected, _Fp __desired,
			      memory_order __order = memory_order_seq_cst)
      volatile noexcept
      {
	return compare_exchange_strong(__expected, __desired, __order,
				       __cmpexch_failure_order(__order));
      }

      value_type
      fetch_add(value_type __i,
		memory_order __m = memory_order_seq_cst) noexcept
      { return __atomic_impl::__fetch_add_flt(&_M_fp, __i, __m); }

      value_type
      fetch_add(value_type __i,
		memory_order __m = memory_order_seq_cst) volatile noexcept
      { return __atomic_impl::__fetch_add_flt(&_M_fp, __i, __m); }

      value_type
      fetch_sub(value_type __i,
		memory_order __m = memory_order_seq_cst) noexcept
      { return __atomic_impl::__fetch_sub_flt(&_M_fp, __i, __m); }

      value_type
      fetch_sub(value_type __i,
		memory_order __m = memory_order_seq_cst) volatile noexcept
      { return __atomic_impl::__fetch_sub_flt(&_M_fp, __i, __m); }

      value_type
      operator+=(value_type __i) noexcept
      { return __atomic_impl::__add_fetch_flt(&_M_fp, __i); }

      value_type
      operator+=(value_type __i) volatile noexcept
      { return __atomic_impl::__add_fetch_flt(&_M_fp, __i); }

      value_type
      operator-=(value_type __i) noexcept
      { return __atomic_impl::__sub_fetch_flt(&_M_fp, __i); }

      value_type
      operator-=(value_type __i) volatile noexcept
      { return __atomic_impl::__sub_fetch_flt(&_M_fp, __i); }

    private:
      alignas(_S_alignment) _Fp _M_fp _GLIBCXX20_INIT(0);
    };
#undef _GLIBCXX20_INIT

  template<typename _Tp,
	   bool = is_integral_v<_Tp>, bool = is_floating_point_v<_Tp>>
    struct __atomic_ref;

  // base class for non-integral, non-floating-point, non-pointer types
  template<typename _Tp>
    struct __atomic_ref<_Tp, false, false>
    {
      static_assert(is_trivially_copyable_v<_Tp>);

      // 1/2/4/8/16-byte types must be aligned to at least their size.
      static constexpr int _S_min_alignment
	= (sizeof(_Tp) & (sizeof(_Tp) - 1)) || sizeof(_Tp) > 16
	? 0 : sizeof(_Tp);

    public:
      using value_type = _Tp;

      static constexpr bool is_always_lock_free
	= __atomic_always_lock_free(sizeof(_Tp), 0);

      static constexpr size_t required_alignment
	= _S_min_alignment > alignof(_Tp) ? _S_min_alignment : alignof(_Tp);

      __atomic_ref& operator=(const __atomic_ref&) = delete;

      explicit
      __atomic_ref(_Tp& __t) : _M_ptr(std::__addressof(__t))
      { __glibcxx_assert(((uintptr_t)_M_ptr % required_alignment) == 0); }

      __atomic_ref(const __atomic_ref&) noexcept = default;

      _Tp
      operator=(_Tp __t) const noexcept
      {
	this->store(__t);
	return __t;
      }

      operator _Tp() const noexcept { return this->load(); }

      bool
      is_lock_free() const noexcept
      { return __atomic_impl::is_lock_free<sizeof(_Tp), required_alignment>(); }

      void
      store(_Tp __t, memory_order __m = memory_order_seq_cst) const noexcept
      { __atomic_impl::store(_M_ptr, __t, __m); }

      _Tp
      load(memory_order __m = memory_order_seq_cst) const noexcept
      { return __atomic_impl::load(_M_ptr, __m); }

      _Tp
      exchange(_Tp __desired, memory_order __m = memory_order_seq_cst)
      const noexcept
      { return __atomic_impl::exchange(_M_ptr, __desired, __m); }

      bool
      compare_exchange_weak(_Tp& __expected, _Tp __desired,
			    memory_order __success,
			    memory_order __failure) const noexcept
      {
	return __atomic_impl::compare_exchange_weak(_M_ptr,
						    __expected, __desired,
						    __success, __failure);
      }

      bool
      compare_exchange_strong(_Tp& __expected, _Tp __desired,
			    memory_order __success,
			    memory_order __failure) const noexcept
      {
	return __atomic_impl::compare_exchange_strong(_M_ptr,
						      __expected, __desired,
						      __success, __failure);
      }

      bool
      compare_exchange_weak(_Tp& __expected, _Tp __desired,
			    memory_order __order = memory_order_seq_cst)
      const noexcept
      {
	return compare_exchange_weak(__expected, __desired, __order,
                                     __cmpexch_failure_order(__order));
      }

      bool
      compare_exchange_strong(_Tp& __expected, _Tp __desired,
			      memory_order __order = memory_order_seq_cst)
      const noexcept
      {
	return compare_exchange_strong(__expected, __desired, __order,
				       __cmpexch_failure_order(__order));
      }

    private:
      _Tp* _M_ptr;
    };

  // base class for atomic_ref<integral-type>
  template<typename _Tp>
    struct __atomic_ref<_Tp, true, false>
    {
      static_assert(is_integral_v<_Tp>);

    public:
      using value_type = _Tp;
      using difference_type = value_type;

      static constexpr bool is_always_lock_free
	= __atomic_always_lock_free(sizeof(_Tp), 0);

      static constexpr size_t required_alignment
	= sizeof(_Tp) > alignof(_Tp) ? sizeof(_Tp) : alignof(_Tp);

      __atomic_ref() = delete;
      __atomic_ref& operator=(const __atomic_ref&) = delete;

      explicit
      __atomic_ref(_Tp& __t) : _M_ptr(&__t)
      { __glibcxx_assert(((uintptr_t)_M_ptr % required_alignment) == 0); }

      __atomic_ref(const __atomic_ref&) noexcept = default;

      _Tp
      operator=(_Tp __t) const noexcept
      {
	this->store(__t);
	return __t;
      }

      operator _Tp() const noexcept { return this->load(); }

      bool
      is_lock_free() const noexcept
      {
	return __atomic_impl::is_lock_free<sizeof(_Tp), required_alignment>();
      }

      void
      store(_Tp __t, memory_order __m = memory_order_seq_cst) const noexcept
      { __atomic_impl::store(_M_ptr, __t, __m); }

      _Tp
      load(memory_order __m = memory_order_seq_cst) const noexcept
      { return __atomic_impl::load(_M_ptr, __m); }

      _Tp
      exchange(_Tp __desired,
	       memory_order __m = memory_order_seq_cst) const noexcept
      { return __atomic_impl::exchange(_M_ptr, __desired, __m); }

      bool
      compare_exchange_weak(_Tp& __expected, _Tp __desired,
			    memory_order __success,
			    memory_order __failure) const noexcept
      {
	return __atomic_impl::compare_exchange_weak(_M_ptr,
						    __expected, __desired,
						    __success, __failure);
      }

      bool
      compare_exchange_strong(_Tp& __expected, _Tp __desired,
			      memory_order __success,
			      memory_order __failure) const noexcept
      {
	return __atomic_impl::compare_exchange_strong(_M_ptr,
						      __expected, __desired,
						      __success, __failure);
      }

      bool
      compare_exchange_weak(_Tp& __expected, _Tp __desired,
			    memory_order __order = memory_order_seq_cst)
      const noexcept
      {
	return compare_exchange_weak(__expected, __desired, __order,
                                     __cmpexch_failure_order(__order));
      }

      bool
      compare_exchange_strong(_Tp& __expected, _Tp __desired,
			      memory_order __order = memory_order_seq_cst)
      const noexcept
      {
	return compare_exchange_strong(__expected, __desired, __order,
				       __cmpexch_failure_order(__order));
      }

      value_type
      fetch_add(value_type __i,
		memory_order __m = memory_order_seq_cst) const noexcept
      { return __atomic_impl::fetch_add(_M_ptr, __i, __m); }

      value_type
      fetch_sub(value_type __i,
		memory_order __m = memory_order_seq_cst) const noexcept
      { return __atomic_impl::fetch_sub(_M_ptr, __i, __m); }

      value_type
      fetch_and(value_type __i,
		memory_order __m = memory_order_seq_cst) const noexcept
      { return __atomic_impl::fetch_and(_M_ptr, __i, __m); }

      value_type
      fetch_or(value_type __i,
	       memory_order __m = memory_order_seq_cst) const noexcept
      { return __atomic_impl::fetch_or(_M_ptr, __i, __m); }

      value_type
      fetch_xor(value_type __i,
		memory_order __m = memory_order_seq_cst) const noexcept
      { return __atomic_impl::fetch_xor(_M_ptr, __i, __m); }

      _GLIBCXX_ALWAYS_INLINE value_type
      operator++(int) const noexcept
      { return fetch_add(1); }

      _GLIBCXX_ALWAYS_INLINE value_type
      operator--(int) const noexcept
      { return fetch_sub(1); }

      value_type
      operator++() const noexcept
      { return __atomic_impl::__add_fetch(_M_ptr, value_type(1)); }

      value_type
      operator--() const noexcept
      { return __atomic_impl::__sub_fetch(_M_ptr, value_type(1)); }

      value_type
      operator+=(value_type __i) const noexcept
      { return __atomic_impl::__add_fetch(_M_ptr, __i); }

      value_type
      operator-=(value_type __i) const noexcept
      { return __atomic_impl::__sub_fetch(_M_ptr, __i); }

      value_type
      operator&=(value_type __i) const noexcept
      { return __atomic_impl::__and_fetch(_M_ptr, __i); }

      value_type
      operator|=(value_type __i) const noexcept
      { return __atomic_impl::__or_fetch(_M_ptr, __i); }

      value_type
      operator^=(value_type __i) const noexcept
      { return __atomic_impl::__xor_fetch(_M_ptr, __i); }

    private:
      _Tp* _M_ptr;
    };

  // base class for atomic_ref<floating-point-type>
  template<typename _Fp>
    struct __atomic_ref<_Fp, false, true>
    {
      static_assert(is_floating_point_v<_Fp>);

    public:
      using value_type = _Fp;
      using difference_type = value_type;

      static constexpr bool is_always_lock_free
	= __atomic_always_lock_free(sizeof(_Fp), 0);

      static constexpr size_t required_alignment = __alignof__(_Fp);

      __atomic_ref() = delete;
      __atomic_ref& operator=(const __atomic_ref&) = delete;

      explicit
      __atomic_ref(_Fp& __t) : _M_ptr(&__t)
      { __glibcxx_assert(((uintptr_t)_M_ptr % required_alignment) == 0); }

      __atomic_ref(const __atomic_ref&) noexcept = default;

      _Fp
      operator=(_Fp __t) const noexcept
      {
	this->store(__t);
	return __t;
      }

      operator _Fp() const noexcept { return this->load(); }

      bool
      is_lock_free() const noexcept
      {
	return __atomic_impl::is_lock_free<sizeof(_Fp), required_alignment>();
      }

      void
      store(_Fp __t, memory_order __m = memory_order_seq_cst) const noexcept
      { __atomic_impl::store(_M_ptr, __t, __m); }

      _Fp
      load(memory_order __m = memory_order_seq_cst) const noexcept
      { return __atomic_impl::load(_M_ptr, __m); }

      _Fp
      exchange(_Fp __desired,
	       memory_order __m = memory_order_seq_cst) const noexcept
      { return __atomic_impl::exchange(_M_ptr, __desired, __m); }

      bool
      compare_exchange_weak(_Fp& __expected, _Fp __desired,
			    memory_order __success,
			    memory_order __failure) const noexcept
      {
	return __atomic_impl::compare_exchange_weak(_M_ptr,
						    __expected, __desired,
						    __success, __failure);
      }

      bool
      compare_exchange_strong(_Fp& __expected, _Fp __desired,
			    memory_order __success,
			    memory_order __failure) const noexcept
      {
	return __atomic_impl::compare_exchange_strong(_M_ptr,
						      __expected, __desired,
						      __success, __failure);
      }

      bool
      compare_exchange_weak(_Fp& __expected, _Fp __desired,
			    memory_order __order = memory_order_seq_cst)
      const noexcept
      {
	return compare_exchange_weak(__expected, __desired, __order,
                                     __cmpexch_failure_order(__order));
      }

      bool
      compare_exchange_strong(_Fp& __expected, _Fp __desired,
			      memory_order __order = memory_order_seq_cst)
      const noexcept
      {
	return compare_exchange_strong(__expected, __desired, __order,
				       __cmpexch_failure_order(__order));
      }

      value_type
      fetch_add(value_type __i,
		memory_order __m = memory_order_seq_cst) const noexcept
      { return __atomic_impl::__fetch_add_flt(_M_ptr, __i, __m); }

      value_type
      fetch_sub(value_type __i,
		memory_order __m = memory_order_seq_cst) const noexcept
      { return __atomic_impl::__fetch_sub_flt(_M_ptr, __i, __m); }

      value_type
      operator+=(value_type __i) const noexcept
      { return __atomic_impl::__add_fetch_flt(_M_ptr, __i); }

      value_type
      operator-=(value_type __i) const noexcept
      { return __atomic_impl::__sub_fetch_flt(_M_ptr, __i); }

    private:
      _Fp* _M_ptr;
    };

  // base class for atomic_ref<pointer-type>
  template<typename _Tp>
    struct __atomic_ref<_Tp*, false, false>
    {
    public:
      using value_type = _Tp*;
      using difference_type = ptrdiff_t;

      static constexpr bool is_always_lock_free = ATOMIC_POINTER_LOCK_FREE == 2;

      static constexpr size_t required_alignment = __alignof__(_Tp*);

      __atomic_ref() = delete;
      __atomic_ref& operator=(const __atomic_ref&) = delete;

      explicit
      __atomic_ref(_Tp*& __t) : _M_ptr(std::__addressof(__t))
      { __glibcxx_assert(((uintptr_t)_M_ptr % required_alignment) == 0); }

      __atomic_ref(const __atomic_ref&) noexcept = default;

      _Tp*
      operator=(_Tp* __t) const noexcept
      {
	this->store(__t);
	return __t;
      }

      operator _Tp*() const noexcept { return this->load(); }

      bool
      is_lock_free() const noexcept
      {
	return __atomic_impl::is_lock_free<sizeof(_Tp*), required_alignment>();
      }

      void
      store(_Tp* __t, memory_order __m = memory_order_seq_cst) const noexcept
      { __atomic_impl::store(_M_ptr, __t, __m); }

      _Tp*
      load(memory_order __m = memory_order_seq_cst) const noexcept
      { return __atomic_impl::load(_M_ptr, __m); }

      _Tp*
      exchange(_Tp* __desired,
	       memory_order __m = memory_order_seq_cst) const noexcept
      { return __atomic_impl::exchange(_M_ptr, __desired, __m); }

      bool
      compare_exchange_weak(_Tp*& __expected, _Tp* __desired,
			    memory_order __success,
			    memory_order __failure) const noexcept
      {
	return __atomic_impl::compare_exchange_weak(_M_ptr,
						    __expected, __desired,
						    __success, __failure);
      }

      bool
      compare_exchange_strong(_Tp*& __expected, _Tp* __desired,
			    memory_order __success,
			    memory_order __failure) const noexcept
      {
	return __atomic_impl::compare_exchange_strong(_M_ptr,
						      __expected, __desired,
						      __success, __failure);
      }

      bool
      compare_exchange_weak(_Tp*& __expected, _Tp* __desired,
			    memory_order __order = memory_order_seq_cst)
      const noexcept
      {
	return compare_exchange_weak(__expected, __desired, __order,
                                     __cmpexch_failure_order(__order));
      }

      bool
      compare_exchange_strong(_Tp*& __expected, _Tp* __desired,
			      memory_order __order = memory_order_seq_cst)
      const noexcept
      {
	return compare_exchange_strong(__expected, __desired, __order,
				       __cmpexch_failure_order(__order));
      }

      _GLIBCXX_ALWAYS_INLINE value_type
      fetch_add(difference_type __d,
		memory_order __m = memory_order_seq_cst) const noexcept
      { return __atomic_impl::fetch_add(_M_ptr, _S_type_size(__d), __m); }

      _GLIBCXX_ALWAYS_INLINE value_type
      fetch_sub(difference_type __d,
		memory_order __m = memory_order_seq_cst) const noexcept
      { return __atomic_impl::fetch_sub(_M_ptr, _S_type_size(__d), __m); }

      value_type
      operator++(int) const noexcept
      { return fetch_add(1); }

      value_type
      operator--(int) const noexcept
      { return fetch_sub(1); }

      value_type
      operator++() const noexcept
      {
	return __atomic_impl::__add_fetch(_M_ptr, _S_type_size(1));
      }

      value_type
      operator--() const noexcept
      {
	return __atomic_impl::__sub_fetch(_M_ptr, _S_type_size(1));
      }

      value_type
      operator+=(difference_type __d) const noexcept
      {
	return __atomic_impl::__add_fetch(_M_ptr, _S_type_size(__d));
      }

      value_type
      operator-=(difference_type __d) const noexcept
      {
	return __atomic_impl::__sub_fetch(_M_ptr, _S_type_size(__d));
      }

    private:
      static constexpr ptrdiff_t
      _S_type_size(ptrdiff_t __d) noexcept
      {
	static_assert(is_object_v<_Tp>);
	return __d * sizeof(_Tp);
      }

      _Tp** _M_ptr;
    };

#endif // C++2a

  // @} group atomics

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace std

#endif
