// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifdef UNSAFE_BUFFERS_BUILD
// TODO(crbug.com/40284755): Remove this and spanify to fix the errors.
#pragma allow_unsafe_buffers
#endif

#ifndef BASE_WIN_SCOPED_SAFEARRAY_H_
#define BASE_WIN_SCOPED_SAFEARRAY_H_

#include <objbase.h>

#include <optional>

#include "base/base_export.h"
#include "base/check_op.h"
#include "base/memory/raw_ptr_exclusion.h"
#include "base/win/variant_conversions.h"

namespace base {
namespace win {

// Manages a Windows SAFEARRAY. This is a minimal wrapper that simply provides
// RAII semantics and does not duplicate the extensive functionality that
// CComSafeArray offers.
class BASE_EXPORT ScopedSafearray {
 public:
  // LockScope<VARTYPE> class for automatically managing the lifetime of a
  // SAFEARRAY lock, and granting easy access to the underlying data either
  // through random access or as an iterator.
  // It is undefined behavior if the underlying SAFEARRAY is destroyed
  // before the LockScope.
  // LockScope implements std::iterator_traits as a random access iterator, so
  // that LockScope is compatible with STL methods that require these traits.
  template <VARTYPE ElementVartype>
  class BASE_EXPORT LockScope final {
   public:
    // Type declarations to support std::iterator_traits
    using iterator_category = std::random_access_iterator_tag;
    using value_type =
        typename internal::VariantConverter<ElementVartype>::Type;
    using difference_type = ptrdiff_t;
    using reference = value_type&;
    using const_reference = const value_type&;
    using pointer = value_type*;
    using const_pointer = const value_type*;

    LockScope() = default;

    LockScope(LockScope<ElementVartype>&& other)
        : safearray_(std::exchange(other.safearray_, nullptr)),
          vartype_(std::exchange(other.vartype_, VT_EMPTY)),
          array_(std::exchange(other.array_, nullptr)),
          array_size_(std::exchange(other.array_size_, 0U)) {}

    LockScope<ElementVartype>& operator=(LockScope<ElementVartype>&& other) {
      DCHECK_NE(this, &other);
      Reset();
      safearray_ = std::exchange(other.safearray_, nullptr);
      vartype_ = std::exchange(other.vartype_, VT_EMPTY);
      array_ = std::exchange(other.array_, nullptr);
      array_size_ = std::exchange(other.array_size_, 0U);
      return *this;
    }

    LockScope(const LockScope&) = delete;
    LockScope& operator=(const LockScope&) = delete;

    ~LockScope() { Reset(); }

    VARTYPE Type() const { return vartype_; }

    size_t size() const { return array_size_; }

    pointer begin() { return array_; }
    pointer end() { return array_ + array_size_; }
    const_pointer begin() const { return array_; }
    const_pointer end() const { return array_ + array_size_; }

    pointer data() { return array_; }
    const_pointer data() const { return array_; }

    reference operator[](size_t index) { return at(index); }
    const_reference operator[](size_t index) const { return at(index); }

    reference at(size_t index) {
      DCHECK_NE(array_, nullptr);
      DCHECK_LT(index, array_size_);
      return array_[index];
    }
    const_reference at(size_t index) const {
      return const_cast<LockScope<ElementVartype>*>(this)->at(index);
    }

   private:
    LockScope(SAFEARRAY* safearray,
              VARTYPE vartype,
              pointer array,
              size_t array_size)
        : safearray_(safearray),
          vartype_(vartype),
          array_(array),
          array_size_(array_size) {}

    void Reset() {
      if (safearray_) {
        SafeArrayUnaccessData(safearray_);
      }
      safearray_ = nullptr;
      vartype_ = VT_EMPTY;
      array_ = nullptr;
      array_size_ = 0U;
    }

    // RAW_PTR_EXCLUSION: Comes from the operating system and may have been
    // laundered. If rewritten, it may generate an incorrect Dangling Pointer
    // Detector error.
    RAW_PTR_EXCLUSION SAFEARRAY* safearray_ = nullptr;
    VARTYPE vartype_ = VT_EMPTY;
    pointer array_ = nullptr;
    size_t array_size_ = 0U;

    friend class ScopedSafearray;
  };

  explicit ScopedSafearray(SAFEARRAY* safearray = nullptr)
      : safearray_(safearray) {}

  ScopedSafearray(const ScopedSafearray&) = delete;
  ScopedSafearray& operator=(const ScopedSafearray&) = delete;

  // Move constructor
  ScopedSafearray(ScopedSafearray&& r) noexcept : safearray_(r.safearray_) {
    r.safearray_ = nullptr;
  }

  // Move operator=. Allows assignment from a ScopedSafearray rvalue.
  ScopedSafearray& operator=(ScopedSafearray&& rvalue) {
    Reset(rvalue.Release());
    return *this;
  }

  ~ScopedSafearray() { Destroy(); }

  // Creates a LockScope for accessing the contents of a
  // single-dimensional SAFEARRAYs.
  template <VARTYPE ElementVartype>
  std::optional<LockScope<ElementVartype>> CreateLockScope() const {
    if (!safearray_ || SafeArrayGetDim(safearray_) != 1) {
      return std::nullopt;
    }

    VARTYPE vartype;
    HRESULT hr = SafeArrayGetVartype(safearray_, &vartype);
    if (FAILED(hr) ||
        !internal::VariantConverter<ElementVartype>::IsConvertibleTo(vartype)) {
      return std::nullopt;
    }

    typename LockScope<ElementVartype>::pointer array = nullptr;
    hr = SafeArrayAccessData(safearray_, reinterpret_cast<void**>(&array));
    if (FAILED(hr)) {
      return std::nullopt;
    }

    const size_t array_size = GetCount();
    return LockScope<ElementVartype>(safearray_, vartype, array, array_size);
  }

  void Destroy() {
    if (safearray_) {
      HRESULT hr = SafeArrayDestroy(safearray_);
      DCHECK_EQ(S_OK, hr);
      safearray_ = nullptr;
    }
  }

  // Give ScopedSafearray ownership over an already allocated SAFEARRAY or
  // nullptr.
  void Reset(SAFEARRAY* safearray = nullptr) {
    if (safearray != safearray_) {
      Destroy();
      safearray_ = safearray;
    }
  }

  // Releases ownership of the SAFEARRAY to the caller.
  SAFEARRAY* Release() {
    SAFEARRAY* safearray = safearray_;
    safearray_ = nullptr;
    return safearray;
  }

  // Retrieves the pointer address.
  // Used to receive SAFEARRAYs as out arguments (and take ownership).
  // This function releases any existing references because it will leak
  // the existing ref otherwise.
  // Usage: GetSafearray(safearray.Receive());
  SAFEARRAY** Receive() {
    Destroy();
    return &safearray_;
  }

  // Returns the number of elements in a dimension of the array.
  size_t GetCount(UINT dimension = 0) const {
    DCHECK(safearray_);
    // Initialize |lower| and |upper| so this method will return zero if either
    // SafeArrayGetLBound or SafeArrayGetUBound returns failure because they
    // only write to the output parameter when successful.
    LONG lower = 0;
    LONG upper = -1;
    DCHECK_LT(dimension, SafeArrayGetDim(safearray_));
    HRESULT hr = SafeArrayGetLBound(safearray_, dimension + 1, &lower);
    DCHECK(SUCCEEDED(hr));
    hr = SafeArrayGetUBound(safearray_, dimension + 1, &upper);
    DCHECK(SUCCEEDED(hr));
    LONG count = upper - lower + 1;
    // SafeArrays may have negative lower bounds, so check for wraparound.
    DCHECK_GE(count, 0);
    return static_cast<size_t>(count);
  }

  // Returns the internal pointer.
  SAFEARRAY* Get() const { return safearray_; }

  // Forbid comparison of ScopedSafearray types.  You should never have the same
  // SAFEARRAY owned by two different scoped_ptrs.
  bool operator==(const ScopedSafearray& safearray2) const = delete;
  bool operator!=(const ScopedSafearray& safearray2) const = delete;

 private:
  // RAW_PTR_EXCLUSION: Like LockScope::safearray_, this comes from the
  // operating system.
  RAW_PTR_EXCLUSION SAFEARRAY* safearray_;
};

}  // namespace win
}  // namespace base

#endif  // BASE_WIN_SCOPED_SAFEARRAY_H_
