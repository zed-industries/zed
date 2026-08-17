//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___STRING_EXTERN_TEMPLATE_LISTS_H
#define _LIBCPP___STRING_EXTERN_TEMPLATE_LISTS_H

#include <__config>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

// clang-format off

// We maintain multiple ABI lists:
// - _LIBCPP_STRING_COMMON_EXTERN_TEMPLATE_LIST
// - _LIBCPP_STRING_V1_EXTERN_TEMPLATE_LIST
// - _LIBCPP_STRING_UNSTABLE_EXTERN_TEMPLATE_LIST
// As the name implies, the ABI lists define a common subset, the V1 (Stable) and unstable ABI.
//
// For unstable, we may explicitly remove function that are external in V1.
//
// For stable, the ABI list should rarely change, except for adding new
// functions supporting new c++ version / API changes. Typically entries
// must never be removed from the stable list.
#define _LIBCPP_STRING_COMMON_EXTERN_TEMPLATE_LIST(Func, CharT)                                                        \
    Func(void basic_string<CharT>::__init(const value_type*, size_type))                                               \
    Func(void basic_string<CharT>::__init(size_type, value_type))                                                      \
    Func(basic_string<CharT>::basic_string(const basic_string&, size_type, size_type, const allocator<CharT>&))        \
    Func(basic_string<CharT>::~basic_string())                                                                         \
    Func(basic_string<CharT>& basic_string<CharT>::operator=(value_type))                                              \
    Func(basic_string<CharT>& basic_string<CharT>::assign(size_type, value_type))                                      \
    Func(basic_string<CharT>& basic_string<CharT>::assign(const basic_string&, size_type, size_type))                  \
    Func(basic_string<CharT>& basic_string<CharT>::append(size_type, value_type))                                      \
    Func(basic_string<CharT>& basic_string<CharT>::append(const value_type*))                                          \
    Func(basic_string<CharT>& basic_string<CharT>::append(const value_type*, size_type))                               \
    Func(basic_string<CharT>& basic_string<CharT>::append(const basic_string&, size_type, size_type))                  \
    Func(void basic_string<CharT>::push_back(value_type))                                                              \
    Func(basic_string<CharT>& basic_string<CharT>::insert(size_type, const value_type*))                               \
    Func(basic_string<CharT>& basic_string<CharT>::insert(size_type, size_type, value_type))                           \
    Func(basic_string<CharT>& basic_string<CharT>::insert(size_type, const value_type*, size_type))                    \
    Func(basic_string<CharT>& basic_string<CharT>::insert(size_type, const basic_string&, size_type, size_type))       \
    Func(basic_string<CharT>::iterator basic_string<CharT>::insert(basic_string::const_iterator, value_type))          \
    Func(basic_string<CharT>& basic_string<CharT>::replace(size_type, size_type, const value_type*))                   \
    Func(basic_string<CharT>& basic_string<CharT>::replace(size_type, size_type, size_type, value_type))               \
    Func(basic_string<CharT>& basic_string<CharT>::replace(size_type, size_type, const value_type*, size_type))        \
    Func(basic_string<CharT>& basic_string<CharT>::replace(size_type, size_type, const basic_string&, size_type, size_type)) \
    Func(void basic_string<CharT>::__grow_by_and_replace(size_type, size_type, size_type, size_type, size_type, size_type, const value_type*)) \
    Func(void basic_string<CharT>::resize(size_type, value_type))                                                      \
    Func(void basic_string<CharT>::reserve(size_type))                                                                 \
    Func(basic_string<CharT>::size_type basic_string<CharT>::copy(value_type*, size_type, size_type) const)            \
    Func(basic_string<CharT>::size_type basic_string<CharT>::find(value_type, size_type) const)                        \
    Func(basic_string<CharT>::size_type basic_string<CharT>::find(const value_type*, size_type, size_type) const)      \
    Func(basic_string<CharT>::size_type basic_string<CharT>::rfind(value_type, size_type) const)                       \
    Func(basic_string<CharT>::size_type basic_string<CharT>::rfind(const value_type*, size_type, size_type) const)     \
    Func(basic_string<CharT>::size_type basic_string<CharT>::find_first_of(const value_type*, size_type, size_type) const) \
    Func(basic_string<CharT>::size_type basic_string<CharT>::find_last_of(const value_type*, size_type, size_type) const) \
    Func(basic_string<CharT>::size_type basic_string<CharT>::find_first_not_of(const value_type*, size_type, size_type) const) \
    Func(basic_string<CharT>::size_type basic_string<CharT>::find_last_not_of(const value_type*, size_type, size_type) const) \
    Func(CharT& basic_string<CharT>::at(size_type))                                                                    \
    Func(const CharT& basic_string<CharT>::at(size_type) const)                                                        \
    Func(int basic_string<CharT>::compare(const value_type*) const)                                                    \
    Func(int basic_string<CharT>::compare(size_type, size_type, const value_type*) const)                              \
    Func(int basic_string<CharT>::compare(size_type, size_type, const value_type*, size_type) const)                   \
    Func(int basic_string<CharT>::compare(size_type, size_type, const basic_string&, size_type, size_type) const)      \
    Func(const basic_string<CharT>::size_type basic_string<CharT>::npos)                                               \

#define _LIBCPP_STRING_V1_EXTERN_TEMPLATE_LIST(Func, CharT)                                                            \
  _LIBCPP_STRING_COMMON_EXTERN_TEMPLATE_LIST(Func, CharT)                                                              \
  Func(basic_string<CharT>::basic_string(const basic_string&))                                                         \
  Func(basic_string<CharT>::basic_string(const basic_string&, const allocator<CharT>&))                                \
  Func(basic_string<CharT>& basic_string<CharT>::assign(const value_type*))                                            \
  Func(basic_string<CharT>& basic_string<CharT>::assign(const value_type*, size_type))                                 \
  Func(basic_string<CharT>& basic_string<CharT>::operator=(basic_string const&))                                       \
  Func(void basic_string<CharT>::__grow_by(size_type, size_type, size_type, size_type, size_type, size_type))          \
  Func(basic_string<CharT>& basic_string<CharT>::erase(size_type, size_type))                                          \

#define _LIBCPP_STRING_UNSTABLE_EXTERN_TEMPLATE_LIST(Func, CharT)                                                      \
  _LIBCPP_STRING_COMMON_EXTERN_TEMPLATE_LIST(Func, CharT)                                                              \
  Func(void basic_string<CharT>::__init_copy_ctor_external(const value_type*, size_type))                              \
  Func(basic_string<CharT>& basic_string<CharT>::__assign_external(const value_type*, size_type))                      \
  Func(basic_string<CharT>& basic_string<CharT>::__assign_external(const value_type*))                                 \
  Func(basic_string<CharT>& basic_string<CharT>::__assign_no_alias<false>(const value_type*, size_type))               \
  Func(basic_string<CharT>& basic_string<CharT>::__assign_no_alias<true>(const value_type*, size_type))                \
  Func(void basic_string<CharT>::__erase_external_with_move(size_type, size_type))

// clang-format on

#endif // _LIBCPP___STRING_EXTERN_TEMPLATE_LISTS_H
