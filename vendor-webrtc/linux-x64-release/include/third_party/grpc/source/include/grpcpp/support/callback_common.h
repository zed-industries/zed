//
//
// Copyright 2018 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//

#ifndef GRPCPP_SUPPORT_CALLBACK_COMMON_H
#define GRPCPP_SUPPORT_CALLBACK_COMMON_H

#include <grpc/grpc.h>
#include <grpc/impl/grpc_types.h>
#include <grpcpp/impl/call.h>
#include <grpcpp/impl/codegen/channel_interface.h>
#include <grpcpp/impl/completion_queue_tag.h>
#include <grpcpp/support/config.h>
#include <grpcpp/support/global_callback_hook.h>
#include <grpcpp/support/status.h>

#include <functional>

#include "absl/log/absl_check.h"

namespace grpc {
namespace internal {

/// An exception-safe way of invoking a user-specified callback function
// TODO(vjpai): decide whether it is better for this to take a const lvalue
//              parameter or an rvalue parameter, or if it even matters
template <class Func, class... Args>
void CatchingCallback(Func&& func, Args&&... args) {
#if GRPC_ALLOW_EXCEPTIONS
  try {
    func(std::forward<Args>(args)...);
  } catch (...) {
    // nothing to return or change here, just don't crash the library
  }
#else   // GRPC_ALLOW_EXCEPTIONS
  func(std::forward<Args>(args)...);
#endif  // GRPC_ALLOW_EXCEPTIONS
}

template <class Reactor, class Func, class... Args>
Reactor* CatchingReactorGetter(Func&& func, Args&&... args) {
#if GRPC_ALLOW_EXCEPTIONS
  try {
    return func(std::forward<Args>(args)...);
  } catch (...) {
    // fail the RPC, don't crash the library
    return nullptr;
  }
#else   // GRPC_ALLOW_EXCEPTIONS
  return func(std::forward<Args>(args)...);
#endif  // GRPC_ALLOW_EXCEPTIONS
}

// The contract on these tags is that they are single-shot. They must be
// constructed and then fired at exactly one point. There is no expectation
// that they can be reused without reconstruction.

class CallbackWithStatusTag : public grpc_completion_queue_functor {
 public:
  // always allocated against a call arena, no memory free required
  static void operator delete(void* /*ptr*/, std::size_t size) {
    ABSL_CHECK_EQ(size, sizeof(CallbackWithStatusTag));
  }

  // This operator should never be called as the memory should be freed as part
  // of the arena destruction. It only exists to provide a matching operator
  // delete to the operator new so that some compilers will not complain (see
  // https://github.com/grpc/grpc/issues/11301) Note at the time of adding this
  // there are no tests catching the compiler warning.
  static void operator delete(void*, void*) { ABSL_CHECK(false); }

  CallbackWithStatusTag(grpc_call* call, std::function<void(Status)> f,
                        CompletionQueueTag* ops)
      : call_(call), func_(std::move(f)), ops_(ops) {
    grpc_call_ref(call);
    functor_run = &CallbackWithStatusTag::StaticRun;
    // A client-side callback should never be run inline since they will always
    // have work to do from the user application. So, set the parent's
    // inlineable field to false
    inlineable = false;
  }
  ~CallbackWithStatusTag() {}
  Status* status_ptr() { return &status_; }

  // force_run can not be performed on a tag if operations using this tag
  // have been sent to PerformOpsOnCall. It is intended for error conditions
  // that are detected before the operations are internally processed.
  void force_run(Status s) {
    status_ = std::move(s);
    Run(true);
  }

 private:
  grpc_call* call_;
  std::function<void(Status)> func_;
  CompletionQueueTag* ops_;
  Status status_;

  static void StaticRun(grpc_completion_queue_functor* cb, int ok) {
    static_cast<CallbackWithStatusTag*>(cb)->Run(static_cast<bool>(ok));
  }
  void Run(bool ok) {
    void* ignored = ops_;

    if (!ops_->FinalizeResult(&ignored, &ok)) {
      // The tag was swallowed
      return;
    }
    ABSL_CHECK(ignored == ops_);

    // Last use of func_ or status_, so ok to move them out
    auto func = std::move(func_);
    auto status = std::move(status_);
    func_ = nullptr;     // reset to clear this out for sure
    status_ = Status();  // reset to clear this out for sure
    GetGlobalCallbackHook()->RunCallback(
        call_, [func = std::move(func), status = std::move(status)]() {
#if GRPC_ALLOW_EXCEPTIONS
          try {
            func(status);
          } catch (...) {
            // nothing to return or change here, just don't crash the library
          }
#else   // GRPC_ALLOW_EXCEPTIONS
  func(status);
#endif  // GRPC_ALLOW_EXCEPTIONS
        });
    grpc_call_unref(call_);
  }
};

/// CallbackWithSuccessTag can be reused multiple times, and will be used in
/// this fashion for streaming operations. As a result, it shouldn't clear
/// anything up until its destructor
class CallbackWithSuccessTag : public grpc_completion_queue_functor {
 public:
  // always allocated against a call arena, no memory free required
  static void operator delete(void* /*ptr*/, std::size_t size) {
    ABSL_CHECK_EQ(size, sizeof(CallbackWithSuccessTag));
  }

  // This operator should never be called as the memory should be freed as part
  // of the arena destruction. It only exists to provide a matching operator
  // delete to the operator new so that some compilers will not complain (see
  // https://github.com/grpc/grpc/issues/11301) Note at the time of adding this
  // there are no tests catching the compiler warning.
  static void operator delete(void*, void*) { ABSL_CHECK(false); }

  CallbackWithSuccessTag() : call_(nullptr) {}

  CallbackWithSuccessTag(const CallbackWithSuccessTag&) = delete;
  CallbackWithSuccessTag& operator=(const CallbackWithSuccessTag&) = delete;

  ~CallbackWithSuccessTag() { Clear(); }

  // Set can only be called on a default-constructed or Clear'ed tag.
  // It should never be called on a tag that was constructed with arguments
  // or on a tag that has been Set before unless the tag has been cleared.
  // can_inline indicates that this particular callback can be executed inline
  // (without needing a thread hop) and is only used for library-provided server
  // callbacks.
  void Set(grpc_call* call, std::function<void(bool)> f,
           CompletionQueueTag* ops, bool can_inline) {
    ABSL_CHECK_EQ(call_, nullptr);
    grpc_call_ref(call);
    call_ = call;
    func_ = std::move(f);
    ops_ = ops;
    functor_run = &CallbackWithSuccessTag::StaticRun;
    inlineable = can_inline;
  }

  void Clear() {
    if (call_ != nullptr) {
      grpc_call* call = call_;
      call_ = nullptr;
      func_ = nullptr;
      grpc_call_unref(call);
    }
  }

  CompletionQueueTag* ops() { return ops_; }

  // force_run can not be performed on a tag if operations using this tag
  // have been sent to PerformOpsOnCall. It is intended for error conditions
  // that are detected before the operations are internally processed.
  void force_run(bool ok) { Run(ok); }

  /// check if this tag is currently set
  // NOLINTNEXTLINE(google-explicit-constructor)
  operator bool() const { return call_ != nullptr; }

 private:
  grpc_call* call_;
  std::function<void(bool)> func_;
  CompletionQueueTag* ops_;

  static void StaticRun(grpc_completion_queue_functor* cb, int ok) {
    static_cast<CallbackWithSuccessTag*>(cb)->Run(static_cast<bool>(ok));
  }
  void Run(bool ok) {
    void* ignored = ops_;
    // Allow a "false" return value from FinalizeResult to silence the
    // callback, just as it silences a CQ tag in the async cases
#ifndef NDEBUG
    auto* ops = ops_;
#endif
    bool do_callback = ops_->FinalizeResult(&ignored, &ok);
#ifndef NDEBUG
    ABSL_DCHECK(ignored == ops);
#endif

    if (do_callback) {
      GetGlobalCallbackHook()->RunCallback(call_, [this, ok]() {
#if GRPC_ALLOW_EXCEPTIONS
        try {
          func_(ok);
        } catch (...) {
          // nothing to return or change here, just don't crash the library
        }
#else   // GRPC_ALLOW_EXCEPTIONS
  func_(ok);
#endif  // GRPC_ALLOW_EXCEPTIONS
      });
    }
  }
};

}  // namespace internal
}  // namespace grpc

#endif  // GRPCPP_SUPPORT_CALLBACK_COMMON_H
