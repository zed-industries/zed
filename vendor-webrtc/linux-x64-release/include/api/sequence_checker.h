/*
 *  Copyright 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef API_SEQUENCE_CHECKER_H_
#define API_SEQUENCE_CHECKER_H_

#include "api/task_queue/task_queue_base.h"
#include "rtc_base/checks.h"
#include "rtc_base/synchronization/sequence_checker_internal.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

// SequenceChecker is a helper class used to help verify that some methods
// of a class are called on the same task queue or thread. A
// SequenceChecker is bound to a a task queue if the object is
// created on a task queue, or a thread otherwise.
//
//
// Example:
// class MyClass {
//  public:
//   void Foo() {
//     RTC_DCHECK_RUN_ON(&sequence_checker_);
//     ... (do stuff) ...
//   }
//
//  private:
//   SequenceChecker sequence_checker_;
// }
//
// In Release mode, IsCurrent will always return true.
class RTC_LOCKABLE SequenceChecker
#if RTC_DCHECK_IS_ON
    : public webrtc_sequence_checker_internal::SequenceCheckerImpl {
  using Impl = webrtc_sequence_checker_internal::SequenceCheckerImpl;
#else
    : public webrtc_sequence_checker_internal::SequenceCheckerDoNothing {
  using Impl = webrtc_sequence_checker_internal::SequenceCheckerDoNothing;
#endif
 public:
  enum InitialState : bool { kDetached = false, kAttached = true };

  // TODO(tommi): We could maybe join these two ctors and have fewer factory
  // functions. At the moment they're separate to minimize code changes when
  // we added the second ctor as well as avoiding to have unnecessary code at
  // the SequenceChecker which much only run for the SequenceCheckerImpl
  // implementation.
  // In theory we could have something like:
  //
  //  SequenceChecker(InitialState initial_state = kAttached,
  //                  TaskQueueBase* attached_queue = TaskQueueBase::Current());
  //
  // But the problem with that is having the call to `Current()` exist for
  // `SequenceCheckerDoNothing`.
  explicit SequenceChecker(InitialState initial_state = kAttached)
      : Impl(initial_state) {}
  explicit SequenceChecker(TaskQueueBase* attached_queue)
      : Impl(attached_queue) {}

  // Returns true if sequence checker is attached to the current sequence.
  bool IsCurrent() const { return Impl::IsCurrent(); }
  // Detaches checker from sequence to which it is attached. Next attempt
  // to do a check with this checker will result in attaching this checker
  // to the sequence on which check was performed.
  void Detach() { Impl::Detach(); }
};

}  // namespace webrtc

// RTC_RUN_ON/RTC_GUARDED_BY/RTC_DCHECK_RUN_ON macros allows to annotate
// variables are accessed from same thread/task queue.
// Using tools designed to check mutexes, it checks at compile time everywhere
// variable is access, there is a run-time dcheck thread/task queue is correct.
//
// class SequenceCheckerExample {
//  public:
//   int CalledFromPacer() RTC_RUN_ON(pacer_sequence_checker_) {
//     return var2_;
//   }
//
//   void CallMeFromPacer() {
//     RTC_DCHECK_RUN_ON(&pacer_sequence_checker_)
//        << "Should be called from pacer";
//     CalledFromPacer();
//   }
//
//  private:
//   int pacer_var_ RTC_GUARDED_BY(pacer_sequence_checker_);
//   SequenceChecker pacer_sequence_checker_;
// };
//
// class TaskQueueExample {
//  public:
//   class Encoder {
//    public:
//     webrtc::TaskQueueBase& Queue() { return encoder_queue_; }
//     void Encode() {
//       RTC_DCHECK_RUN_ON(&encoder_queue_);
//       DoSomething(var_);
//     }
//
//    private:
//     webrtc::TaskQueueBase& encoder_queue_;
//     Frame var_ RTC_GUARDED_BY(encoder_queue_);
//   };
//
//   void Encode() {
//     // Will fail at runtime when DCHECK is enabled:
//     // encoder_->Encode();
//     // Will work:
//     webrtc::scoped_refptr<Encoder> encoder = encoder_;
//     encoder_->Queue().PostTask([encoder] { encoder->Encode(); });
//   }
//
//  private:
//   webrtc::scoped_refptr<Encoder> encoder_;
// }

// Document if a function expected to be called from same thread/task queue.
#define RTC_RUN_ON(x) \
  RTC_THREAD_ANNOTATION_ATTRIBUTE__(exclusive_locks_required(x))

// Checks current code is running on the desired sequence.
//
// First statement annotates for the thread safety analyzer the check was done.
// Second statement validates it is running on the sequence `x`.
// Such annotation has to be attached to a function, and that function has to be
// called. Thus current implementation creates a noop lambda and calls it.
#define RTC_DCHECK_RUN_ON(x)                                               \
  []() RTC_ASSERT_EXCLUSIVE_LOCK(x) {}();                                  \
  RTC_DCHECK((x)->IsCurrent())                                             \
      << webrtc::webrtc_sequence_checker_internal::ExpectationToString(x);

#endif  // API_SEQUENCE_CHECKER_H_
