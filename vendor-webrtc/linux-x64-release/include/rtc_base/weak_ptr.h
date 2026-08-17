/*
 *  Copyright 2016 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_WEAK_PTR_H_
#define RTC_BASE_WEAK_PTR_H_

#include <memory>
#include <utility>

#include "api/scoped_refptr.h"
#include "api/sequence_checker.h"
#include "rtc_base/checks.h"
#include "rtc_base/ref_count.h"
#include "rtc_base/ref_counted_object.h"
#include "rtc_base/system/no_unique_address.h"
#include "rtc_base/thread_annotations.h"
#include "rtc_base/weak_ptr.h"

// The implementation is borrowed from chromium except that it does not
// implement SupportsWeakPtr.

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
//    Controller() : weak_factory_(this) {}
//    void SpawnWorker() { Worker::StartNew(weak_factory_.GetWeakPtr()); }
//    void WorkComplete(const Result& result) { ... }
//   private:
//    // Member variables should appear before the WeakPtrFactory, to ensure
//    // that any WeakPtrs to Controller are invalidated before its members
//    // variable's destructors are executed, rendering them invalid.
//    WeakPtrFactory<Controller> weak_factory_;
//  };
//
//  class Worker {
//   public:
//    static void StartNew(const WeakPtr<Controller>& controller) {
//      Worker* worker = new Worker(controller);
//      // Kick off asynchronous processing...
//    }
//   private:
//    Worker(const WeakPtr<Controller>& controller)
//        : controller_(controller) {}
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

// Weak pointers may be passed safely between threads, but must always be
// dereferenced and invalidated on the same TaskQueue or thread, otherwise
// checking the pointer would be racey.
//
// To ensure correct use, the first time a WeakPtr issued by a WeakPtrFactory
// is dereferenced, the factory and its WeakPtrs become bound to the calling
// TaskQueue/thread, and cannot be dereferenced or
// invalidated on any other TaskQueue/thread. Bound WeakPtrs can still be handed
// off to other TaskQueues, e.g. to use to post tasks back to object on the
// bound sequence.
//
// Thus, at least one WeakPtr object must exist and have been dereferenced on
// the correct thread to enforce that other WeakPtr objects will enforce they
// are used on the desired thread.

namespace webrtc {

namespace internal {

class WeakReference {
 public:
  // Although Flag is bound to a specific sequence, it may be
  // deleted from another via base::WeakPtr::~WeakPtr().
  class Flag {
   public:
    Flag() = default;

    void Invalidate();
    bool IsValid() const;

   private:
    friend class FinalRefCountedObject<Flag>;

    ~Flag() = default;

    RTC_NO_UNIQUE_ADDRESS SequenceChecker checker_{
        webrtc::SequenceChecker::kDetached};
    bool is_valid_ RTC_GUARDED_BY(checker_) = true;
  };

  // `RefCountedFlag` is the reference counted (shared), non-virtual, flag type.
  using RefCountedFlag = FinalRefCountedObject<Flag>;

  WeakReference();
  explicit WeakReference(const RefCountedFlag* flag);
  ~WeakReference();

  WeakReference(WeakReference&& other);
  WeakReference(const WeakReference& other);
  WeakReference& operator=(WeakReference&& other) = default;
  WeakReference& operator=(const WeakReference& other) = default;

  bool is_valid() const;

 private:
  scoped_refptr<const RefCountedFlag> flag_;
};

class WeakReferenceOwner {
 public:
  WeakReferenceOwner();
  ~WeakReferenceOwner();

  WeakReference GetRef() const;

  bool HasRefs() const { return flag_.get() && !flag_->HasOneRef(); }

  void Invalidate();

 private:
  mutable scoped_refptr<WeakReference::RefCountedFlag> flag_;
};

// This class simplifies the implementation of WeakPtr's type conversion
// constructor by avoiding the need for a public accessor for ref_.  A
// WeakPtr<T> cannot access the private members of WeakPtr<U>, so this
// base class gives us a way to access ref_ in a protected fashion.
class WeakPtrBase {
 public:
  WeakPtrBase();
  ~WeakPtrBase();

  WeakPtrBase(const WeakPtrBase& other) = default;
  WeakPtrBase(WeakPtrBase&& other) = default;
  WeakPtrBase& operator=(const WeakPtrBase& other) = default;
  WeakPtrBase& operator=(WeakPtrBase&& other) = default;

 protected:
  explicit WeakPtrBase(const WeakReference& ref);

  WeakReference ref_;
};

}  // namespace internal

template <typename T>
class WeakPtrFactory;

template <typename T>
class WeakPtr : public internal::WeakPtrBase {
 public:
  WeakPtr() : ptr_(nullptr) {}

  // Allow conversion from U to T provided U "is a" T. Note that this
  // is separate from the (implicit) copy and move constructors.
  template <typename U>
  WeakPtr(const WeakPtr<U>& other)
      : internal::WeakPtrBase(other), ptr_(other.ptr_) {}
  template <typename U>
  WeakPtr(WeakPtr<U>&& other)
      : internal::WeakPtrBase(std::move(other)), ptr_(other.ptr_) {}

  T* get() const { return ref_.is_valid() ? ptr_ : nullptr; }

  T& operator*() const {
    RTC_DCHECK(get() != nullptr);
    return *get();
  }
  T* operator->() const {
    RTC_DCHECK(get() != nullptr);
    return get();
  }

  void reset() {
    ref_ = internal::WeakReference();
    ptr_ = nullptr;
  }

  // Allow conditionals to test validity, e.g. if (weak_ptr) {...};
  explicit operator bool() const { return get() != nullptr; }

 private:
  template <typename U>
  friend class WeakPtr;
  friend class WeakPtrFactory<T>;

  WeakPtr(const internal::WeakReference& ref, T* ptr)
      : internal::WeakPtrBase(ref), ptr_(ptr) {}

  // This pointer is only valid when ref_.is_valid() is true.  Otherwise, its
  // value is undefined (as opposed to nullptr).
  T* ptr_;
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

// A class may be composed of a WeakPtrFactory and thereby
// control how it exposes weak pointers to itself.  This is helpful if you only
// need weak pointers within the implementation of a class.  This class is also
// useful when working with primitive types.  For example, you could have a
// WeakPtrFactory<bool> that is used to pass around a weak reference to a bool.

// Note that GetWeakPtr must be called on one and only one TaskQueue or thread
// and the WeakPtr must only be dereferenced and invalidated on that same
// TaskQueue/thread. A WeakPtr instance can be copied and posted to other
// sequences though as long as it is not dereferenced (WeakPtr<T>::get()).
template <class T>
class WeakPtrFactory {
 public:
  explicit WeakPtrFactory(T* ptr) : ptr_(ptr) {}

  WeakPtrFactory() = delete;
  WeakPtrFactory(const WeakPtrFactory&) = delete;
  WeakPtrFactory& operator=(const WeakPtrFactory&) = delete;

  ~WeakPtrFactory() { ptr_ = nullptr; }

  WeakPtr<T> GetWeakPtr() {
    RTC_DCHECK(ptr_);
    return WeakPtr<T>(weak_reference_owner_.GetRef(), ptr_);
  }

  // Call this method to invalidate all existing weak pointers.
  void InvalidateWeakPtrs() {
    RTC_DCHECK(ptr_);
    weak_reference_owner_.Invalidate();
  }

  // Call this method to determine if any weak pointers exist.
  bool HasWeakPtrs() const {
    RTC_DCHECK(ptr_);
    return weak_reference_owner_.HasRefs();
  }

 private:
  internal::WeakReferenceOwner weak_reference_owner_;
  T* ptr_;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::WeakPtr;
using ::webrtc::WeakPtrFactory;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_WEAK_PTR_H_
