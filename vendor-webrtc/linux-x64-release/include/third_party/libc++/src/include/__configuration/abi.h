// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___CONFIGURATION_ABI_H
#define _LIBCPP___CONFIGURATION_ABI_H

#include <__config_site>
#include <__configuration/compiler.h>
#include <__configuration/platform.h>

#ifndef _LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER
#  pragma GCC system_header
#endif

// FIXME: ABI detection should be done via compiler builtin macros. This
// is just a placeholder until Clang implements such macros. For now assume
// that Windows compilers pretending to be MSVC++ target the Microsoft ABI,
// and allow the user to explicitly specify the ABI to handle cases where this
// heuristic falls short.
#if _LIBCPP_ABI_FORCE_ITANIUM && _LIBCPP_ABI_FORCE_MICROSOFT
#  error "Only one of _LIBCPP_ABI_FORCE_ITANIUM and _LIBCPP_ABI_FORCE_MICROSOFT can be true"
#elif _LIBCPP_ABI_FORCE_ITANIUM
#  define _LIBCPP_ABI_ITANIUM
#elif _LIBCPP_ABI_FORCE_MICROSOFT
#  define _LIBCPP_ABI_MICROSOFT
#else
#  if defined(_WIN32) && defined(_MSC_VER)
#    define _LIBCPP_ABI_MICROSOFT
#  else
#    define _LIBCPP_ABI_ITANIUM
#  endif
#endif

#if _LIBCPP_ABI_VERSION >= 2
// Change short string representation so that string data starts at offset 0,
// improving its alignment in some cases.
#  define _LIBCPP_ABI_ALTERNATE_STRING_LAYOUT
// Fix deque iterator type in order to support incomplete types.
#  define _LIBCPP_ABI_INCOMPLETE_TYPES_IN_DEQUE
// Fix undefined behavior in how std::list stores its linked nodes.
#  define _LIBCPP_ABI_LIST_REMOVE_NODE_POINTER_UB
// Fix undefined behavior in  how __tree stores its end and parent nodes.
#  define _LIBCPP_ABI_TREE_REMOVE_NODE_POINTER_UB
// Fix undefined behavior in how __hash_table stores its pointer types.
#  define _LIBCPP_ABI_FIX_UNORDERED_NODE_POINTER_UB
#  define _LIBCPP_ABI_FORWARD_LIST_REMOVE_NODE_POINTER_UB
#  define _LIBCPP_ABI_FIX_UNORDERED_CONTAINER_SIZE_TYPE
// Give reverse_iterator<T> one data member of type T, not two.
// Also, in C++17 and later, don't derive iterator types from std::iterator.
#  define _LIBCPP_ABI_NO_ITERATOR_BASES
// Use the smallest possible integer type to represent the index of the variant.
// Previously libc++ used "unsigned int" exclusively.
#  define _LIBCPP_ABI_VARIANT_INDEX_TYPE_OPTIMIZATION
// Unstable attempt to provide a more optimized std::function
#  define _LIBCPP_ABI_OPTIMIZED_FUNCTION
// All the regex constants must be distinct and nonzero.
#  define _LIBCPP_ABI_REGEX_CONSTANTS_NONZERO
// Re-worked external template instantiations for std::string with a focus on
// performance and fast-path inlining.
#  define _LIBCPP_ABI_STRING_OPTIMIZED_EXTERNAL_INSTANTIATION
// Enable clang::trivial_abi on std::unique_ptr.
#  define _LIBCPP_ABI_ENABLE_UNIQUE_PTR_TRIVIAL_ABI
// Enable clang::trivial_abi on std::shared_ptr and std::weak_ptr
#  define _LIBCPP_ABI_ENABLE_SHARED_PTR_TRIVIAL_ABI
// std::random_device holds some state when it uses an implementation that gets
// entropy from a file (see _LIBCPP_USING_DEV_RANDOM). When switching from this
// implementation to another one on a platform that has already shipped
// std::random_device, one needs to retain the same object layout to remain ABI
// compatible. This switch removes these workarounds for platforms that don't care
// about ABI compatibility.
#  define _LIBCPP_ABI_NO_RANDOM_DEVICE_COMPATIBILITY_LAYOUT
// Don't export the legacy __basic_string_common class and its methods from the built library.
#  define _LIBCPP_ABI_DO_NOT_EXPORT_BASIC_STRING_COMMON
// Don't export the legacy __vector_base_common class and its methods from the built library.
#  define _LIBCPP_ABI_DO_NOT_EXPORT_VECTOR_BASE_COMMON
// According to the Standard, `bitset::operator[] const` returns bool
#  define _LIBCPP_ABI_BITSET_VECTOR_BOOL_CONST_SUBSCRIPT_RETURN_BOOL
// Fix the implementation of CityHash used for std::hash<fundamental-type>.
// This is an ABI break because `std::hash` will return a different result,
// which means that hashing the same object in translation units built against
// different versions of libc++ can return inconsistent results. This is especially
// tricky since std::hash is used in the implementation of unordered containers.
//
// The incorrect implementation of CityHash has the problem that it drops some
// bits on the floor.
#  define _LIBCPP_ABI_FIX_CITYHASH_IMPLEMENTATION
// Remove the base 10 implementation of std::to_chars from the dylib.
// The implementation moved to the header, but we still export the symbols from
// the dylib for backwards compatibility.
#  define _LIBCPP_ABI_DO_NOT_EXPORT_TO_CHARS_BASE_10
// Define std::array/std::string_view iterators to be __wrap_iters instead of raw
// pointers, which prevents people from relying on a non-portable implementation
// detail. This is especially useful because enabling bounded iterators hardening
// requires code not to make these assumptions.
#  define _LIBCPP_ABI_USE_WRAP_ITER_IN_STD_ARRAY
#  define _LIBCPP_ABI_USE_WRAP_ITER_IN_STD_STRING_VIEW
// Dont' add an inline namespace for `std::filesystem`
#  define _LIBCPP_ABI_NO_FILESYSTEM_INLINE_NAMESPACE
// std::basic_ios uses WEOF to indicate that the fill value is
// uninitialized. However, on platforms where the size of char_type is
// equal to or greater than the size of int_type and char_type is unsigned,
// std::char_traits<char_type>::eq_int_type() cannot distinguish between WEOF
// and WCHAR_MAX. This ABI setting determines whether we should instead track whether the fill
// value has been initialized using a separate boolean, which changes the ABI.
#  define _LIBCPP_ABI_IOS_ALLOW_ARBITRARY_FILL_VALUE
// Historically, libc++ used a type called `__compressed_pair` to reduce storage needs in cases of empty types (e.g. an
// empty allocator in std::vector). We switched to using `[[no_unique_address]]`. However, for ABI compatibility reasons
// we had to add artificial padding in a few places.
//
// This setting disables the addition of such artificial padding, leading to a more optimal
// representation for several types.
#  define _LIBCPP_ABI_NO_COMPRESSED_PAIR_PADDING
#elif _LIBCPP_ABI_VERSION == 1
#  if !(defined(_LIBCPP_OBJECT_FORMAT_COFF) || defined(_LIBCPP_OBJECT_FORMAT_XCOFF))
// Enable compiling copies of now inline methods into the dylib to support
// applications compiled against older libraries. This is unnecessary with
// COFF dllexport semantics, since dllexport forces a non-inline definition
// of inline functions to be emitted anyway. Our own non-inline copy would
// conflict with the dllexport-emitted copy, so we disable it. For XCOFF,
// the linker will take issue with the symbols in the shared object if the
// weak inline methods get visibility (such as from -fvisibility-inlines-hidden),
// so disable it.
#    define _LIBCPP_DEPRECATED_ABI_LEGACY_LIBRARY_DEFINITIONS_FOR_INLINE_FUNCTIONS
#  endif
// Feature macros for disabling pre ABI v1 features. All of these options
// are deprecated.
#  if defined(__FreeBSD__) && __FreeBSD__ < 14
#    define _LIBCPP_DEPRECATED_ABI_DISABLE_PAIR_TRIVIAL_COPY_CTOR
#  endif
#endif

// We had some bugs where we use [[no_unique_address]] together with construct_at,
// which causes UB as the call on construct_at could write to overlapping subobjects
//
// https://github.com/llvm/llvm-project/issues/70506
// https://github.com/llvm/llvm-project/issues/70494
//
// To fix the bug we had to change the ABI of some classes to remove [[no_unique_address]] under certain conditions.
// The macro below is used for all classes whose ABI have changed as part of fixing these bugs.
#define _LIBCPP_ABI_LLVM18_NO_UNIQUE_ADDRESS __attribute__((__abi_tag__("llvm18_nua")))

// Changes the iterator type of select containers (see below) to a bounded iterator that keeps track of whether it's
// within the bounds of the original container and asserts it on every dereference.
//
// ABI impact: changes the iterator type of the relevant containers.
//
// Supported containers:
// - `span`;
// - `string_view`.
// #define _LIBCPP_ABI_BOUNDED_ITERATORS

// Changes the iterator type of `basic_string` to a bounded iterator that keeps track of whether it's within the bounds
// of the original container and asserts it on every dereference and when performing iterator arithmetics.
//
// ABI impact: changes the iterator type of `basic_string` and its specializations, such as `string` and `wstring`.
// #define _LIBCPP_ABI_BOUNDED_ITERATORS_IN_STRING

// Changes the iterator type of `vector` to a bounded iterator that keeps track of whether it's within the bounds of the
// original container and asserts it on every dereference and when performing iterator arithmetics. Note: this doesn't
// yet affect `vector<bool>`.
//
// ABI impact: changes the iterator type of `vector` (except `vector<bool>`).
// #define _LIBCPP_ABI_BOUNDED_ITERATORS_IN_VECTOR

// Changes the iterator type of `array` to a bounded iterator that keeps track of whether it's within the bounds of the
// container and asserts it on every dereference and when performing iterator arithmetic.
//
// ABI impact: changes the iterator type of `array`, its size and its layout.
// #define _LIBCPP_ABI_BOUNDED_ITERATORS_IN_STD_ARRAY

// [[msvc::no_unique_address]] seems to mostly affect empty classes, so the padding scheme for Itanium doesn't work.
#if defined(_LIBCPP_ABI_MICROSOFT) && !defined(_LIBCPP_ABI_NO_COMPRESSED_PAIR_PADDING)
#  define _LIBCPP_ABI_NO_COMPRESSED_PAIR_PADDING
#endif

// Tracks the bounds of the array owned by std::unique_ptr<T[]>, allowing it to trap when accessed out-of-bounds.
// Note that limited bounds checking is also available outside of this ABI configuration, but only some categories
// of types can be checked.
//
// ABI impact: This causes the layout of std::unique_ptr<T[]> to change and its size to increase.
//             This also affects the representation of a few library types that use std::unique_ptr
//             internally, such as the unordered containers.
// #define _LIBCPP_ABI_BOUNDED_UNIQUE_PTR

#if defined(_LIBCPP_COMPILER_CLANG_BASED)
#  if defined(__APPLE__)
#    if defined(__i386__) || defined(__x86_64__)
// use old string layout on x86_64 and i386
#    elif defined(__arm__)
// use old string layout on arm (which does not include aarch64/arm64), except on watch ABIs
#      if defined(__ARM_ARCH_7K__) && __ARM_ARCH_7K__ >= 2
#        define _LIBCPP_ABI_ALTERNATE_STRING_LAYOUT
#      endif
#    else
#      define _LIBCPP_ABI_ALTERNATE_STRING_LAYOUT
#    endif
#  endif
#endif

#endif // _LIBCPP___CONFIGURATION_ABI_H
