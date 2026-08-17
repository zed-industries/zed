// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifdef UNSAFE_BUFFERS_BUILD
// TODO(crbug.com/40284755): Remove this and spanify to fix the errors.
#pragma allow_unsafe_buffers
#endif

#ifndef BASE_WIN_WINRT_FOUNDATION_HELPERS_H_
#define BASE_WIN_WINRT_FOUNDATION_HELPERS_H_

#include <windows.foundation.h>
#include <wrl/client.h>

#include <algorithm>
#include <optional>
#include <vector>

#include "base/check.h"

// This file provides helpers for WinRT types.

namespace base::win::internal {

// Template tricks needed to dispatch to the correct implementation.
//
// For all types which are neither InterfaceGroups nor RuntimeClasses, the
// following three typedefs are synonyms for a single C++ type.  But for
// InterfaceGroups and RuntimeClasses, they are different types:
//   LogicalT: The C++ Type for the InterfaceGroup or RuntimeClass, when
//             used as a template parameter.  Eg "RCFoo*"
//   AbiT:     The C++ type for the default interface used to represent the
//             InterfaceGroup or RuntimeClass when passed as a method parameter.
//             Eg "IFoo*"
//   ComplexT: An instantiation of the Internal "AggregateType" template that
//             combines LogicalT with AbiT. Eg "AggregateType<RCFoo*,IFoo*>".
//             ComplexT is tightly coupled to the interface being implemented,
//             hence defined in headers which include this file.
//             For instance base/win/async_operation.h or
//             base/win/collection_helpers.h
//
// windows.foundation.collections.h defines the following template and
// semantics in Windows::Foundation::Internal:
//
// template <class LogicalType, class AbiType>
// struct AggregateType;
//
//   LogicalType - the Windows Runtime type (eg, runtime class, interface group,
//                 etc) being provided as an argument to an _impl template, when
//                 that type cannot be represented at the ABI.
//   AbiType     - the type used for marshalling, ie "at the ABI", for the
//                 logical type.
template <typename TComplex>
using AbiType =
    typename ABI::Windows::Foundation::Internal::GetAbiType<TComplex>::type;

template <typename TComplex>
using LogicalType =
    typename ABI::Windows::Foundation::Internal::GetLogicalType<TComplex>::type;

// Compile time switch to decide what container to use for |TComplex|.
// Depends on whether the underlying Abi type is a pointer to IUnknown or not.
// It queries the internals of Windows::Foundation to obtain this information.
template <typename TComplex>
using StorageType = std::conditional_t<
    std::is_convertible_v<AbiType<TComplex>, IUnknown*>,
    Microsoft::WRL::ComPtr<std::remove_pointer_t<AbiType<TComplex>>>,
    AbiType<TComplex>>;

// Similar to StorageType, but returns a std::optional in case underlying Abi
// type is not a pointer to IUnknown.
template <typename TComplex>
using OptionalStorageType = std::conditional_t<
    std::is_convertible_v<AbiType<TComplex>, IUnknown*>,
    Microsoft::WRL::ComPtr<std::remove_pointer_t<AbiType<TComplex>>>,
    std::optional<AbiType<TComplex>>>;

template <typename T>
HRESULT CopyTo(const T& value, T* ptr) {
  *ptr = value;
  return S_OK;
}

template <typename T>
HRESULT CopyTo(const Microsoft::WRL::ComPtr<T>& value, T** ptr) {
  return value.CopyTo(ptr);
}

template <typename T>
HRESULT CopyTo(const std::optional<T>& value, T* ptr) {
  *ptr = *value;
  return S_OK;
}

template <typename T>
HRESULT CopyN(typename std::vector<T>::const_iterator first,
              unsigned count,
              T* result) {
  std::copy_n(first, count, result);
  return S_OK;
}

template <typename T>
HRESULT CopyN(
    typename std::vector<Microsoft::WRL::ComPtr<T>>::const_iterator first,
    unsigned count,
    T** result) {
  for (unsigned i = 0; i < count; ++i) {
    CopyTo(*first++, result++);
  }
  return S_OK;
}

inline bool IsEqual(const HSTRING& lhs, const HSTRING& rhs) {
  INT32 result;
  HRESULT hr = ::WindowsCompareStringOrdinal(lhs, rhs, &result);
  DCHECK(SUCCEEDED(hr));
  return result == 0;
}

template <typename T>
bool IsEqual(const T& lhs, const T& rhs) {
  return lhs == rhs;
}

template <typename T>
bool IsEqual(const Microsoft::WRL::ComPtr<T>& com_ptr, const T* ptr) {
  return com_ptr.Get() == ptr;
}

struct Less {
  bool operator()(const HSTRING& lhs, const HSTRING& rhs) const {
    INT32 result;
    HRESULT hr = ::WindowsCompareStringOrdinal(lhs, rhs, &result);
    DCHECK(SUCCEEDED(hr));
    return result < 0;
  }

  template <typename T>
  bool operator()(const Microsoft::WRL::ComPtr<T>& com_ptr,
                  const T* ptr) const {
    return com_ptr.Get() < ptr;
  }

  template <typename T>
  constexpr bool operator()(const T& lhs, const T& rhs) const {
    return lhs < rhs;
  }
};

}  // namespace base::win::internal

#endif  // BASE_WIN_WINRT_FOUNDATION_HELPERS_H_
