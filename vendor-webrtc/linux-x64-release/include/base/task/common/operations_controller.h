// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_COMMON_OPERATIONS_CONTROLLER_H_
#define BASE_TASK_COMMON_OPERATIONS_CONTROLLER_H_

#include <atomic>
#include <cstdint>

#include "base/base_export.h"
#include "base/memory/stack_allocated.h"
#include "base/synchronization/waitable_event.h"

namespace base {
namespace internal {

// A lock-free thread-safe controller to manage critical multi-threaded
// operations without locks.
//
// The controller is used to determine if operations are allowed, and to keep
// track of how many are currently active. Users will call TryBeginOperation()
// before starting such operations. If the call succeeds the user can run the
// operation and the controller will keep track of it until the user signals
// that the operation is completed. No operations are allowed before
// StartAcceptingOperations() is called, or after
// ShutdownAndWaitForZeroOperations() is called.
//
// There is no explicit way of telling the controller when an operation is
// completed, instead for convenience TryBeginOperation() will return a RAII
// like object that will do so on destruction.
//
// For example:
//
// OperationsController controller_;
//
// void SetUp() {
//   controller_.StartAcceptingOperations();
// }
//
// void TearDown() {
//   controller_.ShutdownAndWaitForZeroOperations();
// }
//
// void MaybeRunOperation() {
//   auto operation_token = controller_.TryBeginOperation();
//   if (operation_token) {
//     Process();
//   }
// }
//
// This class is thread-safe.
// But note that StartAcceptingOperations can never be called after
// ShutdownAndWaitForZeroOperations.
class BASE_EXPORT OperationsController {
 public:
  // The owner of an OperationToken which evaluates to true can safely perform
  // an operation while being certain it happens-after
  // StartAcceptingOperations() and happens-before
  // ShutdownAndWaitForZeroOperations(). Releasing this OperationToken
  // relinquishes this right.
  //
  // This class is thread-safe
  class OperationToken {
    STACK_ALLOCATED();

   public:
    ~OperationToken() {
      if (outer_) {
        outer_->DecrementBy(1);
      }
    }
    OperationToken(const OperationToken&) = delete;
    OperationToken(OperationToken&& other) {
      this->outer_ = other.outer_;
      other.outer_ = nullptr;
    }

    operator bool() const { return !!outer_; }

   private:
    friend class OperationsController;
    explicit OperationToken(OperationsController* outer) : outer_(outer) {}

    OperationsController* outer_;
  };

  OperationsController();

  // Users must call ShutdownAndWaitForZeroOperations() before destroying an
  // instance of this class if StartAcceptingOperations() was called.
  ~OperationsController();

  OperationsController(const OperationsController&) = delete;
  OperationsController& operator=(const OperationsController&) = delete;

  // Starts to accept operations (before this point TryBeginOperation() returns
  // an invalid token). Returns true if an attempt to perform an operation was
  // made and denied before StartAcceptingOperations() was called. Can be called
  // at most once, never after ShutdownAndWaitForZeroOperations().
  bool StartAcceptingOperations();

  // Returns a RAII like object that implicitly converts to true if operations
  // are allowed i.e. if this call happens-after StartAcceptingOperations() and
  // happens-before Shutdown(), otherwise the object will convert to false. On
  // successful return, this OperationsController will keep track of the
  // operation until the returned object goes out of scope.
  OperationToken TryBeginOperation();

  // Prevents further calls to TryBeginOperation() from succeeding and waits for
  // all the ongoing operations to complete.
  //
  // Attention: Can only be called once.
  void ShutdownAndWaitForZeroOperations();

 private:
  // Atomic representation of the state of this class. We use the upper 2 bits
  // to keep track of flag like values and the remainder bits are used as a
  // counter. The 2 flags are used to represent 3 different states:
  //
  // State                   | AcceptOperations Bit | ShuttingDown Bit
  // --------------------------------------------------------------
  // kRejectingOperations    | 0                    | 0
  // kAcceptingOperations    | 1                    | 0
  // kShuttingDown           | *                    | 1
  //
  // The counter keeps track of the rejected operations when we are in
  // the kRejectingOperations state, the number of inflight operations
  // otherwise. If the count reaches zero and we are in the shutting down state
  // |shutdown_complete_| will be signaled.
  static constexpr uint32_t kShuttingDownBitMask = uint32_t{1} << 31;
  static constexpr uint32_t kAcceptingOperationsBitMask = uint32_t{1} << 30;
  static constexpr uint32_t kFlagsBitMask =
      (kShuttingDownBitMask | kAcceptingOperationsBitMask);
  static constexpr uint32_t kCountBitMask = ~kFlagsBitMask;
  enum class State {
    kRejectingOperations,
    kAcceptingOperations,
    kShuttingDown,
  };

  // Helper methods for the bit fiddling. Pass a |state_and_count_| value to
  // extract state or count out of it.
  static uint32_t ExtractCount(uint32_t value) { return value & kCountBitMask; }
  static State ExtractState(uint32_t value);

  // Decrements the counter by |n| and signals |shutdown_complete_| if needed.
  void DecrementBy(uint32_t n);

  std::atomic<uint32_t> state_and_count_{0};
  WaitableEvent shutdown_complete_;
};

}  // namespace internal
}  // namespace base

#endif  // BASE_TASK_COMMON_OPERATIONS_CONTROLLER_H_
