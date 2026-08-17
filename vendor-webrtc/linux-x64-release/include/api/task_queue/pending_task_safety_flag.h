/*
 *  Copyright 2020 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TASK_QUEUE_PENDING_TASK_SAFETY_FLAG_H_
#define API_TASK_QUEUE_PENDING_TASK_SAFETY_FLAG_H_

#include <utility>

#include "absl/base/nullability.h"
#include "absl/functional/any_invocable.h"
#include "api/ref_counted_base.h"
#include "api/scoped_refptr.h"
#include "api/sequence_checker.h"
#include "api/task_queue/task_queue_base.h"
#include "rtc_base/system/no_unique_address.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// The PendingTaskSafetyFlag and the ScopedTaskSafety are designed to address
// the issue where you have a task to be executed later that has references,
// but cannot guarantee that the referenced object is alive when the task is
// executed.

// This mechanism can be used with tasks that are created and destroyed
// on a single thread / task queue, and with tasks posted to the same
// thread/task queue, but tasks can be posted from any thread/TQ.

// Typical usage:
// When posting a task, post a copy (capture by-value in a lambda) of the flag
// reference and before performing the work, check the `alive()` state. Abort if
// alive() returns `false`:
//
// class ExampleClass {
// ....
//    webrtc::scoped_refptr<PendingTaskSafetyFlag> flag = safety_flag_;
//    my_task_queue_->PostTask(
//        [flag = std::move(flag), this] {
//          // Now running on the main thread.
//          if (!flag->alive())
//            return;
//          MyMethod();
//        });
//   ....
//   ~ExampleClass() {
//     safety_flag_->SetNotAlive();
//   }
//   scoped_refptr<PendingTaskSafetyFlag> safety_flag_
//        = PendingTaskSafetyFlag::Create();
// }
//
// SafeTask makes this check automatic:
//
//   my_task_queue_->PostTask(SafeTask(safety_flag_, [this] { MyMethod(); }));
//
class RTC_EXPORT PendingTaskSafetyFlag final
    : public RefCountedNonVirtual<PendingTaskSafetyFlag> {
 public:
  static scoped_refptr<PendingTaskSafetyFlag> Create();

  // Creates a flag, but with its SequenceChecker initially detached. Hence, it
  // may be created on a different thread than the flag will be used on.
  static scoped_refptr<PendingTaskSafetyFlag> CreateDetached();

  // Creates a flag, but with its SequenceChecker explicitly initialized for
  // a given task queue and the `alive()` flag specified.
  static scoped_refptr<PendingTaskSafetyFlag> CreateAttachedToTaskQueue(
      bool alive,
      TaskQueueBase* absl_nonnull attached_queue);

  // Same as `CreateDetached()` except the initial state of the returned flag
  // will be `!alive()`.
  static scoped_refptr<PendingTaskSafetyFlag> CreateDetachedInactive();

  ~PendingTaskSafetyFlag() = default;

  void SetNotAlive();
  // The SetAlive method is intended to support Start/Stop/Restart usecases.
  // When a class has called SetNotAlive on a flag used for posted tasks, and
  // decides it wants to post new tasks and have them run, there are two
  // reasonable ways to do that:
  //
  // (i) Use the below SetAlive method. One subtlety is that any task posted
  //     prior to SetNotAlive, and still in the queue, is resurrected and will
  //     run.
  //
  // (ii) Create a fresh flag, and just drop the reference to the old one. This
  //      avoids the above problem, and ensures that tasks poster prior to
  //      SetNotAlive stay cancelled. Instead, there's a potential data race on
  //      the flag pointer itself. Some synchronization is required between the
  //      thread overwriting the flag pointer, and the threads that want to post
  //      tasks and therefore read that same pointer.
  void SetAlive();
  bool alive() const;

 protected:
  explicit PendingTaskSafetyFlag(bool alive) : alive_(alive) {}
  PendingTaskSafetyFlag(bool alive, TaskQueueBase* absl_nonnull attached_queue)
      : alive_(alive), main_sequence_(attached_queue) {}

 private:
  static scoped_refptr<PendingTaskSafetyFlag> CreateInternal(bool alive);

  bool alive_ = true;
  RTC_NO_UNIQUE_ADDRESS SequenceChecker main_sequence_;
};

// The ScopedTaskSafety makes using PendingTaskSafetyFlag very simple.
// It does automatic PTSF creation and signalling of destruction when the
// ScopedTaskSafety instance goes out of scope.
//
// Example usage:
//
//     my_task_queue->PostTask(SafeTask(scoped_task_safety.flag(),
//        [this] {
//             // task goes here
//        }
//
// This should be used by the class that wants tasks dropped after destruction.
// The requirement is that the instance has to be constructed and destructed on
// the same thread as the potentially dropped tasks would be running on.
class RTC_EXPORT ScopedTaskSafety final {
 public:
  ScopedTaskSafety() = default;
  explicit ScopedTaskSafety(scoped_refptr<PendingTaskSafetyFlag> flag)
      : flag_(std::move(flag)) {}
  ~ScopedTaskSafety() { flag_->SetNotAlive(); }

  // Returns a new reference to the safety flag.
  scoped_refptr<PendingTaskSafetyFlag> flag() const { return flag_; }

  // Marks the current flag as not-alive and attaches to a new one.
  void reset(scoped_refptr<PendingTaskSafetyFlag> new_flag =
                 PendingTaskSafetyFlag::Create()) {
    flag_->SetNotAlive();
    flag_ = std::move(new_flag);
  }

 private:
  scoped_refptr<PendingTaskSafetyFlag> flag_ = PendingTaskSafetyFlag::Create();
};

// Like ScopedTaskSafety, but allows construction on a different thread than
// where the flag will be used.
class RTC_EXPORT ScopedTaskSafetyDetached final {
 public:
  ScopedTaskSafetyDetached() = default;
  ~ScopedTaskSafetyDetached() { flag_->SetNotAlive(); }

  // Returns a new reference to the safety flag.
  scoped_refptr<PendingTaskSafetyFlag> flag() const { return flag_; }

 private:
  scoped_refptr<PendingTaskSafetyFlag> flag_ =
      PendingTaskSafetyFlag::CreateDetached();
};

inline absl::AnyInvocable<void() &&> SafeTask(
    scoped_refptr<PendingTaskSafetyFlag> flag,
    absl::AnyInvocable<void() &&> task) {
  return [flag = std::move(flag), task = std::move(task)]() mutable {
    if (flag->alive()) {
      std::move(task)();
    }
  };
}

}  // namespace webrtc

#endif  // API_TASK_QUEUE_PENDING_TASK_SAFETY_FLAG_H_
