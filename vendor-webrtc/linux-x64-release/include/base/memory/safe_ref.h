// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MEMORY_SAFE_REF_H_
#define BASE_MEMORY_SAFE_REF_H_

#include <compare>
#include <concepts>
#include <utility>

#include "base/check.h"
#include "base/memory/safe_ref_traits.h"
#include "base/memory/weak_ptr.h"

namespace base {

// SafeRef smart pointers are used to represent a non-owning pointer to an
// object, where the pointer is always intended to be valid. These are useful in
// the same cases that a raw pointer `T*` (or a `T&`) would traditionally be
// used, as the owner of the SafeRef knows the lifetime of the pointed-to object
// from other means and will not use the pointer after the pointed-to object is
// destroyed. However, unlike a `T*` or `T&`, a logic bug will manifest as a
// benign crash instead of as a Use-after-Free.
//
// SafeRef pointers cannot be null (as expressed by the "Ref" suffix instead of
// "Ptr"). A SafeRef can be wrapped in an std::optional if it should not always
// point to something valid. (A SafePtr sibling type can be introduced if this
// is problematic, or if consuming moves are needed!)
//
// If code wants to track the lifetime of the object directly through its
// pointer, and dynamically handle the case of the pointer outliving the object
// it points to, then base::WeakPtr should be used instead.
//
// The SafeRef pointer is constructed from a base::WeakPtrFactory's GetSafeRef()
// method. Since it is tied to the base::WeakPtrFactory, it will consider its
// pointee invalid when the base::WeakPtrFactory is invalidated, in the same way
// as base::WeakPtr does, including after a call to InvalidateWeakPtrs().
//
// SafeRefTraits are only meant to mark SafeRefs that were found to be dangling,
// thus one should not use this flag to disable dangling pointer detection on
// SafeRef. This parameter is set to SafeRefTraits::kEmpty by default.
//
// THREAD SAFETY: SafeRef pointers (like base::WeakPtr) may only be used on the
// sequence (or thread) where the associated base::WeakPtrFactory will be
// invalidated and/or destroyed. They are safe to passively hold or to destroy
// on any thread though.
//
// This class is expected to one day be replaced by a more flexible and safe
// smart pointer abstraction which is not tied to base::WeakPtrFactory, such as
// raw_ptr<T> from the MiraclePtr project (though perhaps a non-nullable raw_ref
// equivalent).
template <typename T, SafeRefTraits Traits /*= SafeRefTraits::kEmpty*/>
class SafeRef {
 public:
  // No default constructor, since there's no null state. Use an optional
  // SafeRef if the pointer may not be present.

  // Copy construction and assignment.
  SafeRef(const SafeRef& other) : ref_(other.ref_), ptr_(other.ptr_) {
    // Avoid use-after-move.
    CHECK(ref_.IsValid());
  }
  SafeRef& operator=(const SafeRef& other) {
    ref_ = other.ref_;
    ptr_ = other.ptr_;
    // Avoid use-after-move.
    CHECK(ref_.IsValid());
    return *this;
  }

  // Move construction and assignment.
  SafeRef(SafeRef&& other)
      : ref_(std::move(other.ref_)), ptr_(std::move(other.ptr_)) {
    // Avoid use-after-move.
    CHECK(ref_.IsValid());
  }
  SafeRef& operator=(SafeRef&& other) {
    ref_ = std::move(other.ref_);
    ptr_ = std::move(other.ptr_);
    // Avoid use-after-move.
    CHECK(ref_.IsValid());
    return *this;
  }

  // Copy conversion from SafeRef<U>.
  template <typename U>
    requires(std::convertible_to<U*, T*>)
  // NOLINTNEXTLINE(google-explicit-constructor)
  SafeRef(const SafeRef<U>& other)
      : ref_(other.ref_),
        ptr_(other.ptr_)  // raw_ptr<U> converts to raw_ptr<T>.
  {
    // Avoid use-after-move.
    CHECK(ref_.IsValid());
  }
  template <typename U>
  SafeRef& operator=(const SafeRef<U>& other) {
    ref_ = other.ref_;
    ptr_ = other.ptr_;  // raw_ptr<U> converts to raw_ptr<T>.
    // Avoid use-after-move.
    CHECK(ref_.IsValid());
    return *this;
  }

  // Move conversion from SafeRef<U>.
  template <typename U>
  // NOLINTNEXTLINE(google-explicit-constructor)
  SafeRef(SafeRef<U>&& other)
      : ref_(std::move(other.ref_)),
        ptr_(std::move(other.ptr_))  // raw_ptr<U> converts to raw_ptr<T>.
  {
    // Avoid use-after-move.
    CHECK(ref_.IsValid());
  }
  template <typename U>
  SafeRef& operator=(SafeRef<U>&& other) {
    ref_ = std::move(other.ref_);
    ptr_ = std::move(other.ptr_);  // raw_ptr<U> converts to raw_ptr<T>.
    // Avoid use-after-move.
    CHECK(ref_.IsValid());
    return *this;
  }

  // Ordered by the pointer, not the pointee.
  template <typename U>
  std::strong_ordering operator<=>(const SafeRef<U>& other) const {
    return ptr_ <=> other.ptr_;
  }

  // Provide access to the underlying T as a reference. Will CHECK() if the T
  // pointee is no longer alive.
  T& operator*() const {
    CHECK(ref_.IsValid());
    return *ptr_;
  }

  // Used to call methods on the underlying T. Will CHECK() if the T pointee is
  // no longer alive.
  T* operator->() const {
    CHECK(ref_.IsValid());
    return &*ptr_;
  }

 private:
  template <typename U, SafeRefTraits PassedTraits>
  friend class SafeRef;
  template <typename U>
  friend SafeRef<U> internal::MakeSafeRefFromWeakPtrInternals(
      internal::WeakReference&& ref,
      U* ptr);

  // Construction from a from a WeakPtr's internals. Will CHECK() if the WeakPtr
  // is already invalid.
  explicit SafeRef(internal::WeakReference&& ref, T* ptr)
      : ref_(std::move(ref)), ptr_(ptr) {
    CHECK(ref_.IsValid());
  }

  internal::WeakReference ref_;

  static constexpr RawPtrTraits PtrTrait = Traits == SafeRefTraits::kEmpty
                                               ? RawPtrTraits::kEmpty
                                               : DanglingUntriaged;
  // This pointer is only valid when ref_.is_valid() is true.  Otherwise, its
  // value is undefined (as opposed to nullptr). Unlike WeakPtr, this raw_ptr is
  // not allowed to dangle.
  raw_ptr<T, PtrTrait> ptr_;
};

namespace internal {
template <typename T>
SafeRef<T> MakeSafeRefFromWeakPtrInternals(internal::WeakReference&& ref,
                                           T* ptr) {
  return SafeRef<T>(std::move(ref), ptr);
}
}  // namespace internal

}  // namespace base

#endif  // BASE_MEMORY_SAFE_REF_H_
