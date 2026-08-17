//
//
// Copyright 2017 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_IOMGR_CALL_COMBINER_H
#define GRPC_SRC_CORE_LIB_IOMGR_CALL_COMBINER_H

#include <grpc/support/atm.h>
#include <grpc/support/port_platform.h>
#include <stddef.h>

#include "absl/container/inlined_vector.h"
#include "absl/log/log.h"
#include "src/core/lib/iomgr/closure.h"
#include "src/core/lib/iomgr/dynamic_annotations.h"
#include "src/core/lib/iomgr/exec_ctx.h"
#include "src/core/util/mpscq.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"

// A simple, lock-free mechanism for serializing activity related to a
// single call.  This is similar to a combiner but is more lightweight.
//
// It requires the callback (or, in the common case where the callback
// actually kicks off a chain of callbacks, the last callback in that
// chain) to explicitly indicate (by calling GRPC_CALL_COMBINER_STOP())
// when it is done with the action that was kicked off by the original
// callback.

namespace grpc_core {

class CallCombiner {
 public:
  CallCombiner();
  ~CallCombiner();

#ifndef NDEBUG
#define GRPC_CALL_COMBINER_START(call_combiner, closure, error, reason) \
  (call_combiner)->Start((closure), (error), __FILE__, __LINE__, (reason))
#define GRPC_CALL_COMBINER_STOP(call_combiner, reason) \
  (call_combiner)->Stop(__FILE__, __LINE__, (reason))
  /// Starts processing \a closure.
  void Start(grpc_closure* closure, grpc_error_handle error, const char* file,
             int line, const char* reason);
  /// Yields the call combiner to the next closure in the queue, if any.
  void Stop(const char* file, int line, const char* reason);
#else
#define GRPC_CALL_COMBINER_START(call_combiner, closure, error, reason) \
  (call_combiner)->Start((closure), (error), (reason))
#define GRPC_CALL_COMBINER_STOP(call_combiner, reason) \
  (call_combiner)->Stop((reason))
  /// Starts processing \a closure.
  void Start(grpc_closure* closure, grpc_error_handle error,
             const char* reason);
  /// Yields the call combiner to the next closure in the queue, if any.
  void Stop(const char* reason);
#endif

  /// Registers \a closure to be invoked when Cancel() is called.
  ///
  /// Once a closure is registered, it will always be scheduled exactly
  /// once; this allows the closure to hold references that will be freed
  /// regardless of whether or not the call was cancelled.  If a cancellation
  /// does occur, the closure will be scheduled with the cancellation error;
  /// otherwise, it will be scheduled with absl::OkStatus().
  ///
  /// The closure will be scheduled in the following cases:
  /// - If Cancel() was called prior to registering the closure, it will be
  ///   scheduled immediately with the cancellation error.
  /// - If Cancel() is called after registering the closure, the closure will
  ///   be scheduled with the cancellation error.
  /// - If SetNotifyOnCancel() is called again to register a new cancellation
  ///   closure, the previous cancellation closure will be scheduled with
  ///   absl::OkStatus().
  ///
  /// If \a closure is NULL, then no closure will be invoked on
  /// cancellation; this effectively unregisters the previously set closure.
  /// However, most filters will not need to explicitly unregister their
  /// callbacks, as this is done automatically when the call is destroyed.
  void SetNotifyOnCancel(grpc_closure* closure);

  /// Indicates that the call has been cancelled.
  void Cancel(grpc_error_handle error);

 private:
  void ScheduleClosure(grpc_closure* closure, grpc_error_handle error);
#ifdef GRPC_TSAN_ENABLED
  static void TsanClosure(void* arg, grpc_error_handle error);
#endif

  gpr_atm size_ = 0;  // size_t, num closures in queue or currently executing
  MultiProducerSingleConsumerQueue queue_;
  // Either 0 (if not cancelled and no cancellation closure set),
  // a grpc_closure* (if the lowest bit is 0),
  // or a grpc_error_handle (if the lowest bit is 1).
  gpr_atm cancel_state_ = 0;
#ifdef GRPC_TSAN_ENABLED
  // A fake ref-counted lock that is kept alive after the destruction of
  // grpc_call_combiner, when we are running the original closure.
  //
  // Ideally we want to lock and unlock the call combiner as a pointer, when the
  // callback is called. However, original_closure is free to trigger
  // anything on the call combiner (including destruction of grpc_call).
  // Thus, we need a ref-counted structure that can outlive the call combiner.
  struct TsanLock : public RefCounted<TsanLock, NonPolymorphicRefCount> {
    TsanLock() { TSAN_ANNOTATE_RWLOCK_CREATE(&taken); }
    ~TsanLock() { TSAN_ANNOTATE_RWLOCK_DESTROY(&taken); }
    // To avoid double-locking by the same thread, we should acquire/release
    // the lock only when taken is false. On each acquire taken must be set to
    // true.
    std::atomic<bool> taken{false};
  };
  RefCountedPtr<TsanLock> tsan_lock_ = MakeRefCounted<TsanLock>();
  grpc_closure tsan_closure_;
  grpc_closure* original_closure_;
#endif
};

// Helper for running a list of closures in a call combiner.
//
// Each callback running in the call combiner will eventually be
// returned to the surface, at which point the surface will yield the
// call combiner.  So when we are running in the call combiner and have
// more than one callback to return to the surface, we need to re-enter
// the call combiner for all but one of those callbacks.
class CallCombinerClosureList {
 public:
  CallCombinerClosureList() {}

  // Adds a closure to the list.  The closure must eventually result in
  // the call combiner being yielded.
  void Add(grpc_closure* closure, grpc_error_handle error, const char* reason) {
    closures_.emplace_back(closure, error, reason);
  }

  // Runs all closures in the call combiner and yields the call combiner.
  //
  // All but one of the closures in the list will be scheduled via
  // GRPC_CALL_COMBINER_START(), and the remaining closure will be
  // scheduled via ExecCtx::Run(), which will eventually result
  // in yielding the call combiner.  If the list is empty, then the call
  // combiner will be yielded immediately.
  void RunClosures(CallCombiner* call_combiner) {
    if (closures_.empty()) {
      GRPC_CALL_COMBINER_STOP(call_combiner, "no closures to schedule");
      return;
    }
    for (size_t i = 1; i < closures_.size(); ++i) {
      auto& closure = closures_[i];
      GRPC_CALL_COMBINER_START(call_combiner, closure.closure, closure.error,
                               closure.reason);
    }
    GRPC_TRACE_LOG(call_combiner, INFO)
        << "CallCombinerClosureList executing closure while already "
           "holding call_combiner "
        << call_combiner << ": closure=" << closures_[0].closure->DebugString()
        << " error=" << StatusToString(closures_[0].error)
        << " reason=" << closures_[0].reason;
    // This will release the call combiner.
    ExecCtx::Run(DEBUG_LOCATION, closures_[0].closure, closures_[0].error);
    closures_.clear();
  }

  // Runs all closures in the call combiner, but does NOT yield the call
  // combiner.  All closures will be scheduled via GRPC_CALL_COMBINER_START().
  void RunClosuresWithoutYielding(CallCombiner* call_combiner) {
    for (size_t i = 0; i < closures_.size(); ++i) {
      auto& closure = closures_[i];
      GRPC_CALL_COMBINER_START(call_combiner, closure.closure, closure.error,
                               closure.reason);
    }
    closures_.clear();
  }

  size_t size() const { return closures_.size(); }

 private:
  struct CallCombinerClosure {
    grpc_closure* closure;
    grpc_error_handle error;
    const char* reason;

    CallCombinerClosure(grpc_closure* closure, grpc_error_handle error,
                        const char* reason)
        : closure(closure), error(error), reason(reason) {}
  };

  // There are generally a maximum of 6 closures to run in the call
  // combiner, one for each pending op.
  absl::InlinedVector<CallCombinerClosure, 6> closures_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_IOMGR_CALL_COMBINER_H
