// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MEMORY_SCOPED_REFPTR_H_
#define BASE_MEMORY_SCOPED_REFPTR_H_

#include <stddef.h>

#include <compare>
#include <concepts>
#include <iosfwd>
#include <type_traits>
#include <utility>

#include "base/check.h"
#include "base/compiler_specific.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/raw_ptr_exclusion.h"

template <class T>
class scoped_refptr;

namespace base {

template <class, typename>
class RefCounted;
template <class, typename>
class RefCountedThreadSafe;
template <class>
class RefCountedDeleteOnSequence;
class SequencedTaskRunner;

template <typename T>
scoped_refptr<T> AdoptRef(T* t);

namespace subtle {

enum AdoptRefTag { kAdoptRefTag };
enum StartRefCountFromZeroTag { kStartRefCountFromZeroTag };
enum StartRefCountFromOneTag { kStartRefCountFromOneTag };

template <typename TagType>
struct RefCountPreferenceTagTraits;

template <>
struct RefCountPreferenceTagTraits<StartRefCountFromZeroTag> {
  static constexpr StartRefCountFromZeroTag kTag = kStartRefCountFromZeroTag;
};

template <>
struct RefCountPreferenceTagTraits<StartRefCountFromOneTag> {
  static constexpr StartRefCountFromOneTag kTag = kStartRefCountFromOneTag;
};

template <typename T, typename Tag = typename T::RefCountPreferenceTag>
constexpr Tag GetRefCountPreference() {
  return RefCountPreferenceTagTraits<Tag>::kTag;
}

// scoped_refptr<T> is typically used with one of several RefCounted<T> base
// classes or with custom AddRef and Release methods. These overloads dispatch
// on which was used.

template <typename T, typename U, typename V>
constexpr bool IsRefCountPreferenceOverridden(const T*,
                                              const RefCounted<U, V>*) {
  return !std::same_as<std::decay_t<decltype(GetRefCountPreference<T>())>,
                       std::decay_t<decltype(GetRefCountPreference<U>())>>;
}

template <typename T, typename U, typename V>
constexpr bool IsRefCountPreferenceOverridden(
    const T*,
    const RefCountedThreadSafe<U, V>*) {
  return !std::same_as<std::decay_t<decltype(GetRefCountPreference<T>())>,
                       std::decay_t<decltype(GetRefCountPreference<U>())>>;
}

template <typename T, typename U>
constexpr bool IsRefCountPreferenceOverridden(
    const T*,
    const RefCountedDeleteOnSequence<U>*) {
  return !std::same_as<std::decay_t<decltype(GetRefCountPreference<T>())>,
                       std::decay_t<decltype(GetRefCountPreference<U>())>>;
}

constexpr bool IsRefCountPreferenceOverridden(...) {
  return false;
}

template <typename T, typename U, typename V>
constexpr void AssertRefCountBaseMatches(const T*, const RefCounted<U, V>*) {
  static_assert(std::derived_from<T, U>,
                "T implements RefCounted<U>, but U is not a base of T.");
}

template <typename T, typename U, typename V>
constexpr void AssertRefCountBaseMatches(const T*,
                                         const RefCountedThreadSafe<U, V>*) {
  static_assert(
      std::derived_from<T, U>,
      "T implements RefCountedThreadSafe<U>, but U is not a base of T.");
}

template <typename T, typename U>
constexpr void AssertRefCountBaseMatches(const T*,
                                         const RefCountedDeleteOnSequence<U>*) {
  static_assert(
      std::derived_from<T, U>,
      "T implements RefCountedDeleteOnSequence<U>, but U is not a base of T.");
}

constexpr void AssertRefCountBaseMatches(...) {}

}  // namespace subtle

// Creates a scoped_refptr from a raw pointer without incrementing the reference
// count. Use this only for a newly created object whose reference count starts
// from 1 instead of 0.
template <typename T>
scoped_refptr<T> AdoptRef(T* obj) {
  using Tag = std::decay_t<decltype(subtle::GetRefCountPreference<T>())>;
  static_assert(std::same_as<subtle::StartRefCountFromOneTag, Tag>,
                "Use AdoptRef only if the reference count starts from one.");

  DCHECK(obj);
  DCHECK(obj->HasOneRef());
  obj->Adopted();
  return scoped_refptr<T>(obj, subtle::kAdoptRefTag);
}

namespace subtle {

template <typename T>
scoped_refptr<T> AdoptRefIfNeeded(T* obj, StartRefCountFromZeroTag) {
  return scoped_refptr<T>(obj);
}

template <typename T>
scoped_refptr<T> AdoptRefIfNeeded(T* obj, StartRefCountFromOneTag) {
  return AdoptRef(obj);
}

}  // namespace subtle

// Constructs an instance of T, which is a ref counted type, and wraps the
// object into a scoped_refptr<T>.
template <typename T, typename... Args>
scoped_refptr<T> MakeRefCounted(Args&&... args) {
  T* obj = new T(std::forward<Args>(args)...);
  return subtle::AdoptRefIfNeeded(obj, subtle::GetRefCountPreference<T>());
}

// Takes an instance of T, which is a ref counted type, and wraps the object
// into a scoped_refptr<T>.
template <typename T>
scoped_refptr<T> WrapRefCounted(T* t) {
  return scoped_refptr<T>(t);
}

template <typename T, base::RawPtrTraits Traits = base::RawPtrTraits::kEmpty>
scoped_refptr<T> WrapRefCounted(const raw_ptr<T, Traits>& t) {
  return scoped_refptr<T>(t.get());
}

}  // namespace base

//
// A smart pointer class for reference counted objects.  Use this class instead
// of calling AddRef and Release manually on a reference counted object to
// avoid common memory leaks caused by forgetting to Release an object
// reference.  Sample usage:
//
//   class MyFoo : public RefCounted<MyFoo> {
//    ...
//    private:
//     friend class RefCounted<MyFoo>;  // Allow destruction by RefCounted<>.
//     ~MyFoo();                        // Destructor must be private/protected.
//   };
//
//   void some_function() {
//     scoped_refptr<MyFoo> foo = MakeRefCounted<MyFoo>();
//     foo->Method(param);
//     // |foo| is released when this function returns
//   }
//
//   void some_other_function() {
//     scoped_refptr<MyFoo> foo = MakeRefCounted<MyFoo>();
//     ...
//     foo.reset();  // explicitly releases |foo|
//     ...
//     if (foo)
//       foo->Method(param);
//   }
//
// The above examples show how scoped_refptr<T> acts like a pointer to T.
// Given two scoped_refptr<T> classes, it is also possible to exchange
// references between the two objects, like so:
//
//   {
//     scoped_refptr<MyFoo> a = MakeRefCounted<MyFoo>();
//     scoped_refptr<MyFoo> b;
//
//     b.swap(a);
//     // now, |b| references the MyFoo object, and |a| references nullptr.
//   }
//
// To make both |a| and |b| in the above example reference the same MyFoo
// object, simply use the assignment operator:
//
//   {
//     scoped_refptr<MyFoo> a = MakeRefCounted<MyFoo>();
//     scoped_refptr<MyFoo> b;
//
//     b = a;
//     // now, |a| and |b| each own a reference to the same MyFoo object.
//   }
//
// Also see Chromium's ownership and calling conventions:
// https://chromium.googlesource.com/chromium/src/+/lkgr/styleguide/c++/c++.md#object-ownership-and-calling-conventions
// Specifically:
//   If the function (at least sometimes) takes a ref on a refcounted object,
//   declare the param as scoped_refptr<T>. The caller can decide whether it
//   wishes to transfer ownership (by calling std::move(t) when passing t) or
//   retain its ref (by simply passing t directly).
//   In other words, use scoped_refptr like you would a std::unique_ptr except
//   in the odd case where it's required to hold on to a ref while handing one
//   to another component (if a component merely needs to use t on the stack
//   without keeping a ref: pass t as a raw T*).
template <class T>
class TRIVIAL_ABI scoped_refptr {
 public:
  typedef T element_type;

  constexpr scoped_refptr() = default;

  // Allow implicit construction from nullptr.
  constexpr scoped_refptr(std::nullptr_t) {}

  // Constructs from a raw pointer. Note that this constructor allows implicit
  // conversion from T* to scoped_refptr<T> which is strongly discouraged. If
  // you are creating a new ref-counted object please use
  // base::MakeRefCounted<T>() or base::WrapRefCounted<T>(). Otherwise you
  // should move or copy construct from an existing scoped_refptr<T> to the
  // ref-counted object.
  scoped_refptr(T* p) : ptr_(p) {
    if (ptr_) {
      AddRef(ptr_);
    }
  }

  // Copy constructor. This is required in addition to the copy conversion
  // constructor below.
  scoped_refptr(const scoped_refptr& r) : scoped_refptr(r.ptr_) {}

  // Copy conversion constructor.
  template <typename U>
    requires(std::convertible_to<U*, T*>)
  scoped_refptr(const scoped_refptr<U>& r) : scoped_refptr(r.ptr_) {}

  // Move constructor. This is required in addition to the move conversion
  // constructor below.
  scoped_refptr(scoped_refptr&& r) noexcept : ptr_(r.ptr_) { r.ptr_ = nullptr; }

  // Move conversion constructor.
  template <typename U>
    requires(std::convertible_to<U*, T*>)
  scoped_refptr(scoped_refptr<U>&& r) noexcept : ptr_(r.ptr_) {
    r.ptr_ = nullptr;
  }

  ~scoped_refptr() {
    static_assert(!base::subtle::IsRefCountPreferenceOverridden(
                      static_cast<T*>(nullptr), static_cast<T*>(nullptr)),
                  "It's unsafe to override the ref count preference."
                  " Please remove REQUIRE_ADOPTION_FOR_REFCOUNTED_TYPE"
                  " from subclasses.");
    if (ptr_) {
      Release(ptr_);
    }
  }

  T* get() const { return ptr_; }

  T& operator*() const {
    DCHECK(ptr_);
    return *ptr_;
  }

  T* operator->() const {
    DCHECK(ptr_);
    return ptr_;
  }

  scoped_refptr& operator=(std::nullptr_t) {
    reset();
    return *this;
  }

  scoped_refptr& operator=(T* p) { return *this = scoped_refptr(p); }

  // Unified assignment operator.
  scoped_refptr& operator=(scoped_refptr r) noexcept {
    swap(r);
    return *this;
  }

  // Sets managed object to null and releases reference to the previous managed
  // object, if it existed.
  void reset() { scoped_refptr().swap(*this); }

  // Returns the owned pointer (if any), releasing ownership to the caller. The
  // caller is responsible for managing the lifetime of the reference.
  [[nodiscard]] T* release();

  void swap(scoped_refptr& r) noexcept { std::swap(ptr_, r.ptr_); }

  explicit operator bool() const { return ptr_ != nullptr; }

  template <typename U>
  friend bool operator==(const scoped_refptr<T>& lhs,
                         const scoped_refptr<U>& rhs) {
    return lhs.ptr_ == rhs.ptr_;
  }

  // This operator is an optimization to avoid implicitly constructing a
  // scoped_refptr<U> when comparing scoped_refptr against raw pointer. If the
  // implicit conversion is ever removed this operator can also be removed.
  template <typename U>
  friend bool operator==(const scoped_refptr<T>& lhs, const U* rhs) {
    return lhs.ptr_ == rhs;
  }

  friend bool operator==(const scoped_refptr<T>& lhs, std::nullptr_t null) {
    return !static_cast<bool>(lhs);
  }

  template <typename U>
  friend auto operator<=>(const scoped_refptr<T>& lhs,
                          const scoped_refptr<U>& rhs) {
    return lhs.ptr_ <=> rhs.ptr_;
  }

  friend auto operator<=>(const scoped_refptr<T>& lhs, std::nullptr_t null) {
    return lhs.ptr_ <=> static_cast<T*>(nullptr);
  }

 protected:
  // RAW_PTR_EXCLUSION: scoped_refptr<> has its own UaF prevention mechanism.
  // Given how widespread it is, we it'll likely a perf regression for no
  // additional security benefit.
  RAW_PTR_EXCLUSION T* ptr_ = nullptr;

 private:
  template <typename U>
  friend scoped_refptr<U> base::AdoptRef(U*);
  friend class ::base::SequencedTaskRunner;

  scoped_refptr(T* p, base::subtle::AdoptRefTag) : ptr_(p) {}

  // Friend required for move constructors that set r.ptr_ to null.
  template <typename U>
  friend class scoped_refptr;

  // Non-inline helpers to allow:
  //     class Opaque;
  //     extern template class scoped_refptr<Opaque>;
  // Otherwise the compiler will complain that Opaque is an incomplete type.
  static void AddRef(T* ptr);
  static void Release(T* ptr);
};

template <typename T>
T* scoped_refptr<T>::release() {
  T* ptr = ptr_;
  ptr_ = nullptr;
  return ptr;
}

// static
template <typename T>
void scoped_refptr<T>::AddRef(T* ptr) {
  base::subtle::AssertRefCountBaseMatches(ptr, ptr);
  ptr->AddRef();
}

// static
template <typename T>
void scoped_refptr<T>::Release(T* ptr) {
  base::subtle::AssertRefCountBaseMatches(ptr, ptr);
  ptr->Release();
}

template <typename T>
std::ostream& operator<<(std::ostream& out, const scoped_refptr<T>& p) {
  return out << p.get();
}

template <typename T>
void swap(scoped_refptr<T>& lhs, scoped_refptr<T>& rhs) noexcept {
  lhs.swap(rhs);
}

#endif  // BASE_MEMORY_SCOPED_REFPTR_H_
