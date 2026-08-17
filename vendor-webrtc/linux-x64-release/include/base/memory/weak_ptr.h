// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Weak pointers are pointers to an object that do not affect its lifetime,
// and which may be invalidated (i.e. reset to nullptr) by the object, or its
// owner, at any time, most commonly when the object is about to be deleted.

// Weak pointers are useful when an object needs to be accessed safely by one
// or more objects other than its owner, and those callers can cope with the
// object vanishing and e.g. tasks posted to it being silently dropped.
// Reference-counting such an object would complicate the ownership graph and
// make it harder to reason about the object's lifetime.

// EXAMPLE:
//
//  class Controller {
//   public:
//    void SpawnWorker() { Worker::StartNew(weak_factory_.GetWeakPtr()); }
//    void WorkComplete(const Result& result) { ... }
//   private:
//    // Member variables should appear before the WeakPtrFactory, to ensure
//    // that any WeakPtrs to Controller are invalidated before its members
//    // variable's destructors are executed, rendering them invalid.
//    WeakPtrFactory<Controller> weak_factory_{this};
//  };
//
//  class Worker {
//   public:
//    static void StartNew(WeakPtr<Controller> controller) {
//      // Move WeakPtr when possible to avoid atomic refcounting churn on its
//      // internal state.
//      Worker* worker = new Worker(std::move(controller));
//      // Kick off asynchronous processing...
//    }
//   private:
//    Worker(WeakPtr<Controller> controller)
//        : controller_(std::move(controller)) {}
//    void DidCompleteAsynchronousProcessing(const Result& result) {
//      if (controller_)
//        controller_->WorkComplete(result);
//    }
//    WeakPtr<Controller> controller_;
//  };
//
// With this implementation a caller may use SpawnWorker() to dispatch multiple
// Workers and subsequently delete the Controller, without waiting for all
// Workers to have completed.

// ------------------------- IMPORTANT: Thread-safety -------------------------

// Weak pointers may be passed safely between sequences, but must always be
// dereferenced and invalidated on the same SequencedTaskRunner otherwise
// checking the pointer would be racey.
//
// To ensure correct use, the first time a WeakPtr issued by a WeakPtrFactory
// is dereferenced, the factory and its WeakPtrs become bound to the calling
// sequence or current SequencedWorkerPool token, and cannot be dereferenced or
// invalidated on any other task runner. Bound WeakPtrs can still be handed
// off to other task runners, e.g. to use to post tasks back to object on the
// bound sequence.
//
// If all WeakPtr objects are destroyed or invalidated then the factory is
// unbound from the SequencedTaskRunner/Thread. The WeakPtrFactory may then be
// destroyed, or new WeakPtr objects may be used, from a different sequence.
//
// Thus, at least one WeakPtr object must exist and have been dereferenced on
// the correct sequence to enforce that other WeakPtr objects will enforce they
// are used on the desired sequence.

#ifndef BASE_MEMORY_WEAK_PTR_H_
#define BASE_MEMORY_WEAK_PTR_H_

#include <cstddef>
#include <type_traits>
#include <utility>

#include "base/base_export.h"
#include "base/check.h"
#include "base/compiler_specific.h"
#include "base/dcheck_is_on.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/ref_counted.h"
#include "base/memory/safe_ref_traits.h"
#include "base/sequence_checker.h"
#include "base/synchronization/atomic_flag.h"

namespace base {

namespace sequence_manager::internal {
class TaskQueueImpl;
}

template <typename T>
class WeakPtr;

namespace internal {
// These classes are part of the WeakPtr implementation.
// DO NOT USE THESE CLASSES DIRECTLY YOURSELF.

class BASE_EXPORT TRIVIAL_ABI WeakReference {
 public:
  // Although Flag is bound to a specific SequencedTaskRunner, it may be
  // deleted from another via base::WeakPtr::~WeakPtr().
  class BASE_EXPORT Flag : public RefCountedThreadSafe<Flag> {
   public:
    Flag();

    void Invalidate();
    bool IsValid() const;

    bool MaybeValid() const;

#if DCHECK_IS_ON()
    void DetachFromSequence();
    void BindToCurrentSequence();
#endif

   private:
    friend class base::RefCountedThreadSafe<Flag>;

    ~Flag();

    SEQUENCE_CHECKER(sequence_checker_);
    AtomicFlag invalidated_;
  };

  WeakReference();
  explicit WeakReference(const scoped_refptr<Flag>& flag);
  ~WeakReference();

  WeakReference(const WeakReference& other);
  WeakReference& operator=(const WeakReference& other);

  WeakReference(WeakReference&& other) noexcept;
  WeakReference& operator=(WeakReference&& other) noexcept;

  void Reset();
  // Returns whether the WeakReference is valid, meaning the WeakPtrFactory has
  // not invalidated the pointer. Unlike, RefIsMaybeValid(), this may only be
  // called from the same sequence as where the WeakPtr was created.
  bool IsValid() const;
  // Returns false if the WeakReference is confirmed to be invalid. This call is
  // safe to make from any thread, e.g. to optimize away unnecessary work, but
  // RefIsValid() must always be called, on the correct sequence, before
  // actually using the pointer.
  //
  // Warning: as with any object, this call is only thread-safe if the WeakPtr
  // instance isn't being re-assigned or reset() racily with this call.
  bool MaybeValid() const;

 private:
  scoped_refptr<const Flag> flag_;
};

class BASE_EXPORT WeakReferenceOwner {
 public:
  WeakReferenceOwner();
  ~WeakReferenceOwner();

  WeakReference GetRef() const;

  bool HasRefs() const { return !flag_->HasOneRef(); }

  void Invalidate();
  void BindToCurrentSequence();

 private:
  scoped_refptr<WeakReference::Flag> flag_;
};

// Forward declaration from safe_ptr.h.
template <typename T>
SafeRef<T> MakeSafeRefFromWeakPtrInternals(internal::WeakReference&& ref,
                                           T* ptr);

}  // namespace internal

template <typename T>
class WeakPtrFactory;

// The WeakPtr class holds a weak reference to |T*|.
//
// This class is designed to be used like a normal pointer.  You should always
// null-test an object of this class before using it or invoking a method that
// may result in the underlying object being destroyed.
//
// EXAMPLE:
//
//   class Foo { ... };
//   WeakPtr<Foo> foo;
//   if (foo)
//     foo->method();
//
// WeakPtr intentionally doesn't implement operator== or operator<=>, because
// comparisons of weak references are inherently unstable. If the comparison
// takes validity into account, the result can change at any time as pointers
// are invalidated. If it depends only on the underlying pointer value, even
// after the pointer is invalidated, unrelated WeakPtrs can unexpectedly
// compare equal if the address is reused.
template <typename T>
class TRIVIAL_ABI WeakPtr {
 public:
  WeakPtr() = default;
  // NOLINTNEXTLINE(google-explicit-constructor)
  WeakPtr(std::nullptr_t) {}

  // Allow conversion from U to T provided U "is a" T. Note that this
  // is separate from the (implicit) copy and move constructors.
  template <typename U>
    requires(std::convertible_to<U*, T*>)
  // NOLINTNEXTLINE(google-explicit-constructor)
  WeakPtr(const WeakPtr<U>& other) : ref_(other.ref_), ptr_(other.ptr_) {}
  template <typename U>
    requires(std::convertible_to<U*, T*>)
  // NOLINTNEXTLINE(google-explicit-constructor)
  WeakPtr& operator=(const WeakPtr<U>& other) {
    ref_ = other.ref_;
    ptr_ = other.ptr_;
    return *this;
  }

  template <typename U>
    requires(std::convertible_to<U*, T*>)
  // NOLINTNEXTLINE(google-explicit-constructor)
  WeakPtr(WeakPtr<U>&& other)
      : ref_(std::move(other.ref_)), ptr_(std::move(other.ptr_)) {}
  template <typename U>
    requires(std::convertible_to<U*, T*>)
  // NOLINTNEXTLINE(google-explicit-constructor)
  WeakPtr& operator=(WeakPtr<U>&& other) {
    ref_ = std::move(other.ref_);
    ptr_ = std::move(other.ptr_);
    return *this;
  }

  T* get() const { return ref_.IsValid() ? ptr_ : nullptr; }

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
    return ptr_;
  }

  // Allow conditionals to test validity, e.g. if (weak_ptr) {...};
  explicit operator bool() const { return get() != nullptr; }

  // Resets the WeakPtr to hold nothing.
  //
  // The `get()` method will return `nullptr` thereafter, and `MaybeValid()`
  // will be `false`.
  void reset() {
    ref_.Reset();
    ptr_ = nullptr;
  }

  // Do not use this method. Almost all callers should instead use operator
  // bool().
  //
  // There are a few rare cases where the caller intentionally needs to check
  // validity of a base::WeakPtr on a sequence different from the bound sequence
  // as an unavoidable performance optimization. This is the only valid use-case
  // for this method. See
  // https://docs.google.com/document/d/1IGzq9Nx69GS_2iynGmPWo5sFAD2DcCyBY1zIvZwF7k8
  // for details.
  //
  // Returns false if the WeakPtr is confirmed to be invalid. This call is safe
  // to make from any thread, e.g. to optimize away unnecessary work, but
  // RefIsValid() must always be called, on the correct sequence, before
  // actually using the pointer.
  //
  // Warning: as with any object, this call is only thread-safe if the WeakPtr
  // instance isn't being re-assigned or reset() racily with this call.
  bool MaybeValid() const { return ref_.MaybeValid(); }

  // Returns whether the object |this| points to has been invalidated. This can
  // be used to distinguish a WeakPtr to a destroyed object from one that has
  // been explicitly set to null.
  bool WasInvalidated() const { return ptr_ && !ref_.IsValid(); }

 private:
  template <typename U>
  friend class WeakPtr;
  friend class WeakPtrFactory<T>;
  friend class WeakPtrFactory<std::remove_const_t<T>>;

  WeakPtr(internal::WeakReference&& ref, T* ptr)
      : ref_(std::move(ref)), ptr_(ptr) {
    DCHECK(ptr);
  }

  internal::WeakReference CloneWeakReference() const { return ref_; }

  internal::WeakReference ref_;

  // This pointer is only valid when ref_.is_valid() is true.  Otherwise, its
  // value is undefined (as opposed to nullptr). The pointer is allowed to
  // dangle as we verify its liveness through `ref_` before allowing access to
  // the pointee. We don't use raw_ptr<T> here to prevent WeakPtr from keeping
  // the memory allocation in quarantine, as it can't be accessed through the
  // WeakPtr.
  RAW_PTR_EXCLUSION T* ptr_ = nullptr;
};

// Allow callers to compare WeakPtrs against nullptr to test validity.
template <class T>
bool operator!=(const WeakPtr<T>& weak_ptr, std::nullptr_t) {
  return !(weak_ptr == nullptr);
}
template <class T>
bool operator!=(std::nullptr_t, const WeakPtr<T>& weak_ptr) {
  return weak_ptr != nullptr;
}
template <class T>
bool operator==(const WeakPtr<T>& weak_ptr, std::nullptr_t) {
  return weak_ptr.get() == nullptr;
}
template <class T>
bool operator==(std::nullptr_t, const WeakPtr<T>& weak_ptr) {
  return weak_ptr == nullptr;
}

namespace internal {
class BASE_EXPORT WeakPtrFactoryBase {
 protected:
  WeakPtrFactoryBase(uintptr_t ptr);
  ~WeakPtrFactoryBase();
  internal::WeakReferenceOwner weak_reference_owner_;
  uintptr_t ptr_;
};
}  // namespace internal

namespace subtle {

// Restricts access to WeakPtrFactory::BindToCurrentSequence() to authorized
// callers.
class BASE_EXPORT BindWeakPtrFactoryPassKey {
 private:
  BindWeakPtrFactoryPassKey() = default;

  friend class BindWeakPtrFactoryForTesting;
  friend class sequence_manager::internal::TaskQueueImpl;
};

}  // namespace subtle

// A class may be composed of a WeakPtrFactory and thereby
// control how it exposes weak pointers to itself.  This is helpful if you only
// need weak pointers within the implementation of a class.  This class is also
// useful when working with primitive types.  For example, you could have a
// WeakPtrFactory<bool> that is used to pass around a weak reference to a bool.
template <class T>
class WeakPtrFactory : public internal::WeakPtrFactoryBase {
 public:
  WeakPtrFactory() = delete;

  explicit WeakPtrFactory(T* ptr)
      : WeakPtrFactoryBase(reinterpret_cast<uintptr_t>(ptr)) {}

  WeakPtrFactory(const WeakPtrFactory&) = delete;
  WeakPtrFactory& operator=(const WeakPtrFactory&) = delete;

  ~WeakPtrFactory() = default;

  WeakPtr<const T> GetWeakPtr() const {
    return WeakPtr<const T>(weak_reference_owner_.GetRef(),
                            reinterpret_cast<const T*>(ptr_));
  }

  WeakPtr<T> GetWeakPtr()
    requires(!std::is_const_v<T>)
  {
    return WeakPtr<T>(weak_reference_owner_.GetRef(),
                      reinterpret_cast<T*>(ptr_));
  }

  WeakPtr<T> GetMutableWeakPtr() const
    requires(!std::is_const_v<T>)
  {
    return WeakPtr<T>(weak_reference_owner_.GetRef(),
                      reinterpret_cast<T*>(ptr_));
  }

  // Returns a smart pointer that is valid until the WeakPtrFactory is
  // invalidated. Unlike WeakPtr, this smart pointer cannot be null, and cannot
  // be checked to see if the WeakPtrFactory is invalidated. It's intended to
  // express that the pointer will not (intentionally) outlive the `T` object it
  // points to, and to crash safely in the case of a bug instead of causing a
  // use-after-free. This type provides an alternative to WeakPtr to prevent
  // use-after-free bugs without also introducing "fuzzy lifetimes" that can be
  // checked for at runtime.
  SafeRef<T> GetSafeRef() const {
    return internal::MakeSafeRefFromWeakPtrInternals(
        weak_reference_owner_.GetRef(), reinterpret_cast<T*>(ptr_));
  }

  // Call this method to invalidate all existing weak pointers.
  void InvalidateWeakPtrs() {
    DCHECK(ptr_);
    weak_reference_owner_.Invalidate();
  }

  // Call this method to determine if any weak pointers exist.
  bool HasWeakPtrs() const {
    DCHECK(ptr_);
    return weak_reference_owner_.HasRefs();
  }

  // Rebind the factory to the current sequence. This allows creating an object
  // and associated weak pointers on a different thread from the one they are
  // used on.
  void BindToCurrentSequence(subtle::BindWeakPtrFactoryPassKey) {
    weak_reference_owner_.BindToCurrentSequence();
  }
};

}  // namespace base

#endif  // BASE_MEMORY_WEAK_PTR_H_
