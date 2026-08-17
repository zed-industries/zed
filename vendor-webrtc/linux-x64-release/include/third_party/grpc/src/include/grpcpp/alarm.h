//
//
// Copyright 2015 gRPC authors.
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

/// An Alarm posts the user-provided tag to its associated completion queue or
/// invokes the user-provided function on expiry or cancellation.
#ifndef GRPCPP_ALARM_H
#define GRPCPP_ALARM_H

#include <grpc/event_engine/event_engine.h>
#include <grpc/grpc.h>
#include <grpcpp/completion_queue.h>
#include <grpcpp/impl/completion_queue_tag.h>
#include <grpcpp/impl/grpc_library.h>
#include <grpcpp/support/time.h>

#include <functional>

namespace grpc {

/// Trigger a \a CompletionQueue event, or asynchronous callback execution,
/// after some deadline.
///
/// The \a Alarm API has separate \a Set methods for CompletionQueues and
/// callbacks, but only one can be used at any given time. After an alarm has
/// been triggered or cancelled, the same Alarm object may reused.
///
/// Alarm methods are not thread-safe. Applications must ensure a strict
/// ordering between calls to \a Set and \a Cancel. This also implies that any
/// cancellation that occurs before the alarm has been set will have no effect
/// on any future \a Set calls.
class Alarm : private grpc::internal::GrpcLibrary {
 public:
  /// Create an unset Alarm.
  Alarm();

  /// Destroy the given completion queue alarm, cancelling it in the process.
  ~Alarm() override;

  /// DEPRECATED: Create and set a completion queue alarm instance associated to
  /// \a cq.
  /// This form is deprecated because it is inherently racy.
  /// \internal We rely on the presence of \a cq for grpc initialization. If \a
  /// cq were ever to be removed, a reference to a static
  /// internal::GrpcLibraryInitializer instance would need to be introduced
  /// here. \endinternal.
  template <typename T>
  Alarm(grpc::CompletionQueue* cq, const T& deadline, void* tag) : Alarm() {
    SetInternal(cq, grpc::TimePoint<T>(deadline).raw_time(), tag);
  }

  /// Trigger an alarm instance on completion queue \a cq at the specified time.
  /// Once the alarm expires (at \a deadline) or it's cancelled (see \a Cancel),
  /// an event with tag \a tag will be added to \a cq. If the alarm expired, the
  /// event's success bit will be true, false otherwise (ie, upon cancellation).
  //
  // USAGE NOTE: This is frequently used to inject arbitrary tags into \a cq by
  // setting an immediate deadline. Such usage allows synchronizing an external
  // event with an application's \a grpc::CompletionQueue::Next loop.
  template <typename T>
  void Set(grpc::CompletionQueue* cq, const T& deadline, void* tag) {
    SetInternal(cq, grpc::TimePoint<T>(deadline).raw_time(), tag);
  }

  /// Alarms aren't copyable.
  Alarm(const Alarm&) = delete;
  Alarm& operator=(const Alarm&) = delete;

  /// Alarms are movable.
  Alarm(Alarm&& rhs) noexcept : alarm_(rhs.alarm_) { rhs.alarm_ = nullptr; }
  Alarm& operator=(Alarm&& rhs) noexcept {
    std::swap(alarm_, rhs.alarm_);
    return *this;
  }

  /// Cancel a completion queue alarm. Calling this function over an alarm that
  /// has already fired has no effect.
  void Cancel();

  /// Set an alarm to invoke callback \a f. The argument to the callback
  /// states whether the alarm expired at \a deadline (true) or was cancelled
  /// (false)
  template <typename T>
  void Set(const T& deadline, std::function<void(bool)> f) {
    SetInternal(grpc::TimePoint<T>(deadline).raw_time(), std::move(f));
  }

 private:
  void SetInternal(grpc::CompletionQueue* cq, gpr_timespec deadline, void* tag);
  void SetInternal(gpr_timespec deadline, std::function<void(bool)> f);

  grpc::internal::CompletionQueueTag* alarm_;
};

}  // namespace grpc

#endif  // GRPCPP_ALARM_H
