//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___NEW_GLOBAL_NEW_DELETE_H
#define _LIBCPP___NEW_GLOBAL_NEW_DELETE_H

#include <__config>
#include <__cstddef/size_t.h>
#include <__new/align_val_t.h>
#include <__new/exceptions.h>
#include <__new/nothrow_t.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

#if defined(_LIBCPP_CXX03_LANG)
#  define _THROW_BAD_ALLOC throw(std::bad_alloc)
#else
#  define _THROW_BAD_ALLOC
#endif

#if defined(__cpp_sized_deallocation) && __cpp_sized_deallocation >= 201309L
#  define _LIBCPP_HAS_SIZED_DEALLOCATION 1
#else
#  define _LIBCPP_HAS_SIZED_DEALLOCATION 0
#endif

#if defined(_LIBCPP_ABI_VCRUNTIME)
#  include <new.h>
#else
[[__nodiscard__]] _LIBCPP_OVERRIDABLE_FUNC_VIS void* operator new(std::size_t __sz) _THROW_BAD_ALLOC;
[[__nodiscard__]] _LIBCPP_OVERRIDABLE_FUNC_VIS void* operator new(std::size_t __sz, const std::nothrow_t&) _NOEXCEPT
    _LIBCPP_NOALIAS;
_LIBCPP_OVERRIDABLE_FUNC_VIS void operator delete(void* __p) _NOEXCEPT;
_LIBCPP_OVERRIDABLE_FUNC_VIS void operator delete(void* __p, const std::nothrow_t&) _NOEXCEPT;
#  if _LIBCPP_HAS_SIZED_DEALLOCATION
_LIBCPP_OVERRIDABLE_FUNC_VIS void operator delete(void* __p, std::size_t __sz) _NOEXCEPT;
#  endif

[[__nodiscard__]] _LIBCPP_OVERRIDABLE_FUNC_VIS void* operator new[](std::size_t __sz) _THROW_BAD_ALLOC;
[[__nodiscard__]] _LIBCPP_OVERRIDABLE_FUNC_VIS void* operator new[](std::size_t __sz, const std::nothrow_t&) _NOEXCEPT
    _LIBCPP_NOALIAS;
_LIBCPP_OVERRIDABLE_FUNC_VIS void operator delete[](void* __p) _NOEXCEPT;
_LIBCPP_OVERRIDABLE_FUNC_VIS void operator delete[](void* __p, const std::nothrow_t&) _NOEXCEPT;
#  if _LIBCPP_HAS_SIZED_DEALLOCATION
_LIBCPP_OVERRIDABLE_FUNC_VIS void operator delete[](void* __p, std::size_t __sz) _NOEXCEPT;
#  endif

#  if _LIBCPP_HAS_LIBRARY_ALIGNED_ALLOCATION
[[__nodiscard__]] _LIBCPP_OVERRIDABLE_FUNC_VIS void* operator new(std::size_t __sz, std::align_val_t) _THROW_BAD_ALLOC;
[[__nodiscard__]] _LIBCPP_OVERRIDABLE_FUNC_VIS void*
operator new(std::size_t __sz, std::align_val_t, const std::nothrow_t&) _NOEXCEPT _LIBCPP_NOALIAS;
_LIBCPP_OVERRIDABLE_FUNC_VIS void operator delete(void* __p, std::align_val_t) _NOEXCEPT;
_LIBCPP_OVERRIDABLE_FUNC_VIS void operator delete(void* __p, std::align_val_t, const std::nothrow_t&) _NOEXCEPT;
#    if _LIBCPP_HAS_SIZED_DEALLOCATION
_LIBCPP_OVERRIDABLE_FUNC_VIS void operator delete(void* __p, std::size_t __sz, std::align_val_t) _NOEXCEPT;
#    endif

[[__nodiscard__]] _LIBCPP_OVERRIDABLE_FUNC_VIS void*
operator new[](std::size_t __sz, std::align_val_t) _THROW_BAD_ALLOC;
[[__nodiscard__]] _LIBCPP_OVERRIDABLE_FUNC_VIS void*
operator new[](std::size_t __sz, std::align_val_t, const std::nothrow_t&) _NOEXCEPT _LIBCPP_NOALIAS;
_LIBCPP_OVERRIDABLE_FUNC_VIS void operator delete[](void* __p, std::align_val_t) _NOEXCEPT;
_LIBCPP_OVERRIDABLE_FUNC_VIS void operator delete[](void* __p, std::align_val_t, const std::nothrow_t&) _NOEXCEPT;
#    if _LIBCPP_HAS_SIZED_DEALLOCATION
_LIBCPP_OVERRIDABLE_FUNC_VIS void operator delete[](void* __p, std::size_t __sz, std::align_val_t) _NOEXCEPT;
#    endif
#  endif
#endif

#endif // _LIBCPP___NEW_GLOBAL_NEW_DELETE_H
