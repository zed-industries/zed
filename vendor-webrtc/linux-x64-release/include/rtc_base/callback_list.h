/*
 *  Copyright 2020 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_CALLBACK_LIST_H_
#define RTC_BASE_CALLBACK_LIST_H_

#include <utility>
#include <vector>

#include "api/function_view.h"
#include "rtc_base/checks.h"
#include "rtc_base/system/assume.h"
#include "rtc_base/system/inline.h"
#include "rtc_base/system/rtc_export.h"
#include "rtc_base/untyped_function.h"

namespace webrtc {
namespace callback_list_impl {

class RTC_EXPORT CallbackListReceivers {
 public:
  CallbackListReceivers();
  CallbackListReceivers(const CallbackListReceivers&) = delete;
  CallbackListReceivers& operator=(const CallbackListReceivers&) = delete;
  CallbackListReceivers(CallbackListReceivers&&) = delete;
  CallbackListReceivers& operator=(CallbackListReceivers&&) = delete;
  ~CallbackListReceivers();

  template <typename UntypedFunctionArgsT>
  RTC_NO_INLINE void AddReceiver(const void* removal_tag,
                                 UntypedFunctionArgsT args) {
    RTC_CHECK(!send_in_progress_);
    RTC_DCHECK(removal_tag != nullptr);
    receivers_.push_back({removal_tag, UntypedFunction::Create(args)});
  }

  template <typename UntypedFunctionArgsT>
  RTC_NO_INLINE void AddReceiver(UntypedFunctionArgsT args) {
    RTC_CHECK(!send_in_progress_);
    receivers_.push_back({nullptr, UntypedFunction::Create(args)});
  }

  void RemoveReceivers(const void* removal_tag);

  void Foreach(FunctionView<void(UntypedFunction&)> fv);

 private:
  // Special protected pointer value that's used as a removal_tag for
  // receivers that want to unsubscribe from within a callback.
  // Note we could use `&receivers_` too, but since it's the first member
  // variable of the class, its address will be the same as the instance
  // CallbackList instance, so we take an extra step to avoid collision.
  const void* pending_removal_tag() const { return &send_in_progress_; }

  struct Callback {
    const void* removal_tag;
    UntypedFunction function;
  };

  std::vector<Callback> receivers_;
  bool send_in_progress_ = false;
};

extern template void CallbackListReceivers::AddReceiver(
    const void*,
    UntypedFunction::TrivialUntypedFunctionArgs<1>);
extern template void CallbackListReceivers::AddReceiver(
    const void*,
    UntypedFunction::TrivialUntypedFunctionArgs<2>);
extern template void CallbackListReceivers::AddReceiver(
    const void*,
    UntypedFunction::TrivialUntypedFunctionArgs<3>);
extern template void CallbackListReceivers::AddReceiver(
    const void*,
    UntypedFunction::TrivialUntypedFunctionArgs<4>);
extern template void CallbackListReceivers::AddReceiver(
    const void*,
    UntypedFunction::NontrivialUntypedFunctionArgs);
extern template void CallbackListReceivers::AddReceiver(
    const void*,
    UntypedFunction::FunctionPointerUntypedFunctionArgs);

extern template void CallbackListReceivers::AddReceiver(
    UntypedFunction::TrivialUntypedFunctionArgs<1>);
extern template void CallbackListReceivers::AddReceiver(
    UntypedFunction::TrivialUntypedFunctionArgs<2>);
extern template void CallbackListReceivers::AddReceiver(
    UntypedFunction::TrivialUntypedFunctionArgs<3>);
extern template void CallbackListReceivers::AddReceiver(
    UntypedFunction::TrivialUntypedFunctionArgs<4>);
extern template void CallbackListReceivers::AddReceiver(
    UntypedFunction::NontrivialUntypedFunctionArgs);
extern template void CallbackListReceivers::AddReceiver(
    UntypedFunction::FunctionPointerUntypedFunctionArgs);

}  // namespace callback_list_impl

// A collection of receivers (callable objects) that can be called all at once.
// Optimized for minimal binary size. The template arguments dictate what
// signature the callbacks must have; for example, a CallbackList<int, float>
// will require callbacks with signature void(int, float).
//
// CallbackList is neither copyable nor movable (could easily be made movable if
// necessary). Callbacks must be movable, but need not be copyable.
//
// Usage example:
//
//   // Declaration (usually a member variable).
//   CallbackList<int, float> foo_;
//
//   // Register callbacks. This can be done zero or more times. The
//   // callbacks must accept the arguments types listed in the CallbackList's
//   // template argument list, and must return void.
//   foo_.AddReceiver([...](int a, float b) {...});  // Lambda.
//   foo_.AddReceiver(SomeFunction);                 // Function pointer.
//
//   // Call the zero or more receivers, one after the other.
//   foo_.Send(17, 3.14);
//
// Callback lifetime considerations
// --------------------------------
//
// CallbackList::AddReceiver() takes ownership of the given callback by moving
// it in place. The callback can be any callable object; in particular, it may
// have a nontrivial destructor, which will be run when the CallbackList is
// destroyed. The callback may thus access data via any type of smart pointer,
// expressing e.g. unique, shared, or weak ownership. Of course, if the data is
// guaranteed to outlive the callback, a plain raw pointer can be used.
//
// Take care when trying to have the callback own reference-counted data. The
// CallbackList will keep the callback alive, and the callback will keep its
// data alive, so as usual with reference-counted ownership, keep an eye out for
// cycles!
//
// Thread safety
// -------------
//
// Like most C++ types, CallbackList is thread compatible: it's not safe to
// access it concurrently from multiple threads, but it can be made safe if it
// is protected by a mutex, for example.
//
// Excercise some care when deciding what mutexes to hold when you call
// CallbackList::Send(). In particular, do not hold mutexes that callbacks may
// need to grab. If a larger object has a CallbackList member and a single mutex
// that protects all of its data members, this may e.g. make it necessary to
// protect its CallbackList with a separate mutex; otherwise, there will be a
// deadlock if the callbacks try to access the object.
//
// CallbackList as a class data member
// -----------------------------------
//
// CallbackList is a normal C++ data type, and should be private when it is a
// data member of a class. For thread safety reasons (see above), it is likely
// best to not have an accessor for the entire CallbackList, and instead only
// allow callers to add callbacks:
//
//   template <typename F>
//   void AddFooCallback(F&& callback) {
//     // Maybe grab a mutex here?
//     foo_callbacks_.AddReceiver(std::forward<F>(callback));
//   }
//
template <typename... ArgT>
class CallbackList {
 public:
  CallbackList() = default;
  CallbackList(const CallbackList&) = delete;
  CallbackList& operator=(const CallbackList&) = delete;
  CallbackList(CallbackList&&) = delete;
  CallbackList& operator=(CallbackList&&) = delete;

  // Adds a new receiver. The receiver (a callable object or a function pointer)
  // must be movable, but need not be copyable. Its call signature should be
  // `void(ArgT...)`. The removal tag is a pointer to an arbitrary object that
  // you own, and that will stay alive until the CallbackList is gone, or until
  // all receivers using it as a removal tag have been removed; you can use it
  // to remove the receiver.
  template <typename F>
  void AddReceiver(const void* removal_tag, F&& f) {
    receivers_.AddReceiver(
        removal_tag,
        UntypedFunction::PrepareArgs<void(ArgT...)>(std::forward<F>(f)));
  }

  // Adds a new receiver with no removal tag.
  template <typename F>
  void AddReceiver(F&& f) {
    receivers_.AddReceiver(
        UntypedFunction::PrepareArgs<void(ArgT...)>(std::forward<F>(f)));
  }

  // Removes all receivers that were added with the given removal tag.
  void RemoveReceivers(const void* removal_tag) {
    receivers_.RemoveReceivers(removal_tag);
  }

  // Calls all receivers with the given arguments. While the Send is in
  // progress, no method calls are allowed; specifically, this means that the
  // callbacks may not do anything with this CallbackList instance.
  //
  // Note: Receivers are called serially, but not necessarily in the same order
  // they were added.
  template <typename... ArgU>
  void Send(ArgU&&... args) {
    receivers_.Foreach([&](UntypedFunction& f) {
      f.Call<void(ArgT...)>(std::forward<ArgU>(args)...);
    });
  }

 private:
  callback_list_impl::CallbackListReceivers receivers_;
};

}  // namespace webrtc

#endif  // RTC_BASE_CALLBACK_LIST_H_
