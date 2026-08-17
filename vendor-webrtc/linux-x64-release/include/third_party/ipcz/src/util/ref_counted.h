// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_UTIL_REF_COUNTED_H_
#define IPCZ_SRC_UTIL_REF_COUNTED_H_

#include <atomic>
#include <cstddef>
#include <type_traits>
#include <utility>

#include "third_party/abseil-cpp/absl/base/macros.h"

namespace ipcz {

namespace internal {

// Base class for RefCounted<T> instances. See that definition below.
class RefCountedBase {
 protected:
  RefCountedBase();
  ~RefCountedBase();

  // Increases the ref count.
  void AcquireImpl();

  // Decreases the ref count, returning true if and only if this call just
  // released the last reference to the object.
  bool ReleaseImpl();

 private:
  std::atomic_int ref_count_{1};
};

}  // namespace internal

// Tag used to construct a Ref<T> which does not increase ref-count.
enum { kAdoptExistingRef };

// Base class for any ref-counted type T whose ownership can be shared by any
// number of Ref<T> objects.
template <typename T>
class RefCounted : public internal::RefCountedBase {
 public:
  RefCounted() = default;
  ~RefCounted() = default;

  void AcquireRef() { AcquireImpl(); }

  void ReleaseRef() {
    if (ReleaseImpl()) {
      delete static_cast<T*>(this);
    }
  }
};

// A smart pointer which can be used to share ownership of an instance of T,
// where T is any type derived from RefCounted above.
//
// This is used instead of std::shared_ptr so that ipcz can release and
// re-acquire ownership of individual references, as some object references may
// be conceptually owned by the embedding application via an IpczHandle across
// ipcz ABI boundary. std::shared_ptr design does not allow for such manual ref
// manipulation without additional indirection.
template <typename T>
class Ref {
 public:
  constexpr Ref() = default;

  constexpr Ref(std::nullptr_t) {}

  explicit Ref(T* ptr) : ptr_(ptr) {
    if (ptr_) {
      ptr_->AcquireRef();
    }
  }

  Ref(decltype(kAdoptExistingRef), T* ptr) : ptr_(ptr) {}

  Ref(const Ref& other) : Ref(other.ptr_) {}
  Ref(Ref&& other) noexcept : ptr_(other.release()) {}

  template <typename U>
  using EnableIfConvertible =
      typename std::enable_if<std::is_convertible<U*, T*>::value>::type;

  template <typename U, typename = EnableIfConvertible<U>>
  Ref(const Ref<U>& other) : Ref(other.get()) {}

  template <typename U, typename = EnableIfConvertible<U>>
  Ref(Ref<U>&& other) noexcept : ptr_(other.release()) {}

  ~Ref() { reset(); }

  Ref& operator=(std::nullptr_t) {
    reset();
    return *this;
  }

  Ref& operator=(Ref&& other) {
    if (this != &other) {
      reset();
      std::swap(ptr_, other.ptr_);
    }
    return *this;
  }

  Ref& operator=(const Ref& other) {
    if (this != &other) {
      reset();
      ptr_ = other.ptr_;
      if (ptr_) {
        ptr_->AcquireRef();
      }
    }
    return *this;
  }

  explicit operator bool() const { return ptr_ != nullptr; }

  T* get() const { return static_cast<T*>(ptr_); }
  T* operator->() const { return get(); }
  T& operator*() const { return *get(); }

  bool operator==(const T* ptr) const { return ptr_ == ptr; }
  bool operator!=(const T* ptr) const { return ptr_ != ptr; }
  bool operator==(const Ref<T>& other) const { return ptr_ == other.ptr_; }
  bool operator!=(const Ref<T>& other) const { return ptr_ != other.ptr_; }

  void reset() {
    if (ptr_) {
      std::exchange(ptr_, nullptr)->ReleaseRef();
    }
  }

  T* release() { return std::exchange(ptr_, nullptr); }

  void swap(Ref<T>& other) noexcept { std::swap(ptr_, other.ptr_); }

  template <typename H>
  friend H AbslHashValue(H h, const Ref<T>& ref) {
    return H::combine(std::move(h), ref.get());
  }

 private:
  T* ptr_ = nullptr;
};

// Wraps `ptr` as a Ref<T>, increasing the refcount by 1.
template <typename T>
Ref<T> WrapRefCounted(T* ptr) {
  return Ref<T>(ptr);
}

// Wraps `ptr` as a Ref<T>, assuming ownership of an existing reference. Does
// not change the object's refcount.
template <typename T>
Ref<T> AdoptRef(T* ptr) {
  return Ref<T>(kAdoptExistingRef, ptr);
}

template <typename T, typename... Args>
Ref<T> MakeRefCounted(Args&&... args) {
  return AdoptRef(new T(std::forward<Args>(args)...));
}

}  // namespace ipcz

#endif  // IPCZ_SRC_UTIL_REF_COUNTED_H_
