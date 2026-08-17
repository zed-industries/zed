//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___MEMORY_SHARED_COUNT_H
#define _LIBCPP___MEMORY_SHARED_COUNT_H

#include <__config>
#include <__memory/addressof.h>
#include <typeinfo>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_BEGIN_NAMESPACE_STD

// NOTE: Relaxed and acq/rel atomics (for increment and decrement respectively)
// should be sufficient for thread safety.
// See https://llvm.org/PR22803
#if (defined(__clang__) && __has_builtin(__atomic_add_fetch) && defined(__ATOMIC_RELAXED) &&                           \
     defined(__ATOMIC_ACQ_REL)) ||                                                                                     \
    defined(_LIBCPP_COMPILER_GCC)
#  define _LIBCPP_HAS_BUILTIN_ATOMIC_SUPPORT 1
#else
#  define _LIBCPP_HAS_BUILTIN_ATOMIC_SUPPORT 0
#endif

template <class _ValueType>
inline _LIBCPP_HIDE_FROM_ABI _ValueType __libcpp_relaxed_load(_ValueType const* __value) {
#if _LIBCPP_HAS_THREADS && defined(__ATOMIC_RELAXED) &&                                                                \
    (__has_builtin(__atomic_load_n) || defined(_LIBCPP_COMPILER_GCC))
  return __atomic_load_n(__value, __ATOMIC_RELAXED);
#else
  return *__value;
#endif
}

template <class _ValueType>
inline _LIBCPP_HIDE_FROM_ABI _ValueType __libcpp_acquire_load(_ValueType const* __value) {
#if _LIBCPP_HAS_THREADS && defined(__ATOMIC_ACQUIRE) &&                                                                \
    (__has_builtin(__atomic_load_n) || defined(_LIBCPP_COMPILER_GCC))
  return __atomic_load_n(__value, __ATOMIC_ACQUIRE);
#else
  return *__value;
#endif
}

template <class _Tp>
inline _LIBCPP_HIDE_FROM_ABI _Tp __libcpp_atomic_refcount_increment(_Tp& __t) _NOEXCEPT {
#if _LIBCPP_HAS_BUILTIN_ATOMIC_SUPPORT && _LIBCPP_HAS_THREADS
  return __atomic_add_fetch(std::addressof(__t), 1, __ATOMIC_RELAXED);
#else
  return __t += 1;
#endif
}

template <class _Tp>
inline _LIBCPP_HIDE_FROM_ABI _Tp __libcpp_atomic_refcount_decrement(_Tp& __t) _NOEXCEPT {
#if _LIBCPP_HAS_BUILTIN_ATOMIC_SUPPORT && _LIBCPP_HAS_THREADS
  return __atomic_add_fetch(std::addressof(__t), -1, __ATOMIC_ACQ_REL);
#else
  return __t -= 1;
#endif
}

class _LIBCPP_EXPORTED_FROM_ABI __shared_count {
  __shared_count(const __shared_count&);
  __shared_count& operator=(const __shared_count&);

protected:
  long __shared_owners_;
  virtual ~__shared_count();

private:
  virtual void __on_zero_shared() _NOEXCEPT = 0;

public:
  _LIBCPP_HIDE_FROM_ABI explicit __shared_count(long __refs = 0) _NOEXCEPT : __shared_owners_(__refs) {}

#if defined(_LIBCPP_SHARED_PTR_DEFINE_LEGACY_INLINE_FUNCTIONS)
  void __add_shared() noexcept;
  bool __release_shared() noexcept;
#else
  _LIBCPP_HIDE_FROM_ABI void __add_shared() _NOEXCEPT { __libcpp_atomic_refcount_increment(__shared_owners_); }
  _LIBCPP_HIDE_FROM_ABI bool __release_shared() _NOEXCEPT {
    if (__libcpp_atomic_refcount_decrement(__shared_owners_) == -1) {
      __on_zero_shared();
      return true;
    }
    return false;
  }
#endif
  _LIBCPP_HIDE_FROM_ABI long use_count() const _NOEXCEPT { return __libcpp_relaxed_load(&__shared_owners_) + 1; }
};

class _LIBCPP_EXPORTED_FROM_ABI __shared_weak_count : private __shared_count {
  long __shared_weak_owners_;

public:
  _LIBCPP_HIDE_FROM_ABI explicit __shared_weak_count(long __refs = 0) _NOEXCEPT
      : __shared_count(__refs),
        __shared_weak_owners_(__refs) {}

protected:
  ~__shared_weak_count() override;

public:
#if defined(_LIBCPP_SHARED_PTR_DEFINE_LEGACY_INLINE_FUNCTIONS)
  void __add_shared() noexcept;
  void __add_weak() noexcept;
  void __release_shared() noexcept;
#else
  _LIBCPP_HIDE_FROM_ABI void __add_shared() _NOEXCEPT { __shared_count::__add_shared(); }
  _LIBCPP_HIDE_FROM_ABI void __add_weak() _NOEXCEPT { __libcpp_atomic_refcount_increment(__shared_weak_owners_); }
  _LIBCPP_HIDE_FROM_ABI void __release_shared() _NOEXCEPT {
    if (__shared_count::__release_shared())
      __release_weak();
  }
#endif
  void __release_weak() _NOEXCEPT;
  _LIBCPP_HIDE_FROM_ABI long use_count() const _NOEXCEPT { return __shared_count::use_count(); }
  __shared_weak_count* lock() _NOEXCEPT;

  virtual const void* __get_deleter(const type_info&) const _NOEXCEPT;

private:
  virtual void __on_zero_shared_weak() _NOEXCEPT = 0;
};

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___MEMORY_SHARED_COUNT_H
