/*
 *  Copyright 2019 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_OPERATIONS_CHAIN_H_
#define RTC_BASE_OPERATIONS_CHAIN_H_

#include <functional>
#include <memory>
#include <optional>
#include <queue>
#include <type_traits>
#include <utility>

#include "api/ref_counted_base.h"
#include "api/scoped_refptr.h"
#include "api/sequence_checker.h"
#include "rtc_base/checks.h"
#include "rtc_base/system/no_unique_address.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

namespace rtc_operations_chain_internal {

// Abstract base class for operations on the OperationsChain. Run() must be
// invoked exactly once during the Operation's lifespan.
class Operation {
 public:
  virtual ~Operation() {}

  virtual void Run() = 0;
};

// FunctorT is the same as in OperationsChain::ChainOperation(). `callback_` is
// passed on to the `functor_` and is used to inform the OperationsChain that
// the operation completed. The functor is responsible for invoking the
// callback when the operation has completed.
template <typename FunctorT>
class OperationWithFunctor final : public Operation {
 public:
  OperationWithFunctor(FunctorT&& functor, std::function<void()> callback)
      : functor_(std::forward<FunctorT>(functor)),
        callback_(std::move(callback)) {}

  ~OperationWithFunctor() override {
#if RTC_DCHECK_IS_ON
    RTC_DCHECK(has_run_);
#endif  // RTC_DCHECK_IS_ON
  }

  void Run() override {
#if RTC_DCHECK_IS_ON
    RTC_DCHECK(!has_run_);
    has_run_ = true;
#endif  // RTC_DCHECK_IS_ON
    // The functor being executed may invoke the callback synchronously,
    // marking the operation as complete. As such, `this` OperationWithFunctor
    // object may get deleted here, including destroying `functor_`. To
    // protect the functor from self-destruction while running, it is moved to
    // a local variable.
    auto functor = std::move(functor_);
    functor(std::move(callback_));
    // `this` may now be deleted; don't touch any member variables.
  }

 private:
  typename std::remove_reference<FunctorT>::type functor_;
  std::function<void()> callback_;
#if RTC_DCHECK_IS_ON
  bool has_run_ = false;
#endif  // RTC_DCHECK_IS_ON
};

}  // namespace rtc_operations_chain_internal

// An implementation of an operations chain. An operations chain is used to
// ensure that asynchronous tasks are executed in-order with at most one task
// running at a time. The notion of an operation chain is defined in
// https://w3c.github.io/webrtc-pc/#dfn-operations-chain, though unlike this
// implementation, the referenced definition is coupled with a peer connection.
//
// An operation is an asynchronous task. The operation starts when its functor
// is invoked, and completes when the callback that is passed to functor is
// invoked by the operation. The operation must start and complete on the same
// sequence that the operation was "chained" on. As such, the OperationsChain
// operates in a "single-threaded" fashion, but the asynchronous operations may
// use any number of threads to achieve "in parallel" behavior.
//
// When an operation is chained onto the OperationsChain, it is enqueued to be
// executed. Operations are executed in FIFO order, where the next operation
// does not start until the previous operation has completed. OperationsChain
// guarantees that:
// - If the operations chain is empty when an operation is chained, the
//   operation starts immediately, inside ChainOperation().
// - If the operations chain is not empty when an operation is chained, the
//   operation starts upon the previous operation completing, inside the
//   callback.
//
// An operation is contractually obligated to invoke the completion callback
// exactly once. Cancelling a chained operation is not supported by the
// OperationsChain; an operation that wants to be cancellable is responsible for
// aborting its own steps. The callback must still be invoked.
//
// The OperationsChain is kept-alive through reference counting if there are
// operations pending. This, together with the contract, guarantees that all
// operations that are chained get executed.
class OperationsChain final : public RefCountedNonVirtual<OperationsChain> {
 public:
  static scoped_refptr<OperationsChain> Create();
  ~OperationsChain();

  OperationsChain(const OperationsChain&) = delete;
  OperationsChain& operator=(const OperationsChain&) = delete;

  void SetOnChainEmptyCallback(std::function<void()> on_chain_empty_callback);
  bool IsEmpty() const;

  // Chains an operation. Chained operations are executed in FIFO order. The
  // operation starts when `functor` is executed by the OperationsChain and is
  // contractually obligated to invoke the callback passed to it when the
  // operation is complete. Operations must start and complete on the same
  // sequence that this method was invoked on.
  //
  // If the OperationsChain is empty, the operation starts immediately.
  // Otherwise it starts upon the previous operation completing.
  //
  // Requirements of FunctorT:
  // - FunctorT is movable.
  // - FunctorT implements "T operator()(std::function<void()> callback)" or
  //   "T operator()(std::function<void()> callback) const" for some T (if T is
  //   not void, the return value is discarded in the invoking sequence). The
  //   operator starts the operation; when the operation is complete, "callback"
  //   MUST be invoked, and it MUST be so on the sequence that ChainOperation()
  //   was invoked on.
  //
  // Lambda expressions are valid functors.
  template <typename FunctorT>
  void ChainOperation(FunctorT&& functor) {
    RTC_DCHECK_RUN_ON(&sequence_checker_);
    chained_operations_.push(
        std::make_unique<
            rtc_operations_chain_internal::OperationWithFunctor<FunctorT>>(
            std::forward<FunctorT>(functor), CreateOperationsChainCallback()));
    // If this is the only operation in the chain we execute it immediately.
    // Otherwise the callback will get invoked when the pending operation
    // completes which will trigger the next operation to execute.
    if (chained_operations_.size() == 1) {
      chained_operations_.front()->Run();
    }
  }

 private:
  friend class CallbackHandle;

  // The callback that is passed to an operation's functor (that is used to
  // inform the OperationsChain that the operation has completed) is of type
  // std::function<void()>, which is a copyable type. To allow the callback to
  // be copyable, it is backed up by this reference counted handle. See
  // CreateOperationsChainCallback().
  class CallbackHandle final : public RefCountedNonVirtual<CallbackHandle> {
   public:
    explicit CallbackHandle(scoped_refptr<OperationsChain> operations_chain);
    ~CallbackHandle();

    CallbackHandle(const CallbackHandle&) = delete;
    CallbackHandle& operator=(const CallbackHandle&) = delete;

    void OnOperationComplete();

   private:
    scoped_refptr<OperationsChain> operations_chain_;
#if RTC_DCHECK_IS_ON
    bool has_run_ = false;
#endif  // RTC_DCHECK_IS_ON
  };

  OperationsChain();

  std::function<void()> CreateOperationsChainCallback();
  void OnOperationComplete();

  RTC_NO_UNIQUE_ADDRESS SequenceChecker sequence_checker_;
  // FIFO-list of operations that are chained. An operation that is executing
  // remains on this list until it has completed by invoking the callback passed
  // to it.
  std::queue<std::unique_ptr<rtc_operations_chain_internal::Operation>>
      chained_operations_ RTC_GUARDED_BY(sequence_checker_);
  std::optional<std::function<void()>> on_chain_empty_callback_
      RTC_GUARDED_BY(sequence_checker_);
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::OperationsChain;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_OPERATIONS_CHAIN_H_
