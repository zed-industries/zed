// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_FRAMEWORK_SCHEDULER_QUEUE_H_
#define MEDIAPIPE_FRAMEWORK_SCHEDULER_QUEUE_H_

#include <cstdint>
#include <functional>
#include <queue>
#include <string>
#include <utility>

#include "absl/base/thread_annotations.h"
#include "absl/strings/string_view.h"
#include "absl/synchronization/mutex.h"
#include "mediapipe/framework/calculator_context.h"
#include "mediapipe/framework/executor.h"
#include "mediapipe/framework/scheduler_shared.h"

namespace mediapipe {

class CalculatorNode;

namespace internal {

// Manages a priority queue of nodes to be run on the associated executor.
class SchedulerQueue : public TaskQueue {
 public:
  // Callback to be invoked when the queue's idle state changes.
  // The idle argument is true if the queue became idle, false if it became
  // active. See SetIdleCallback for details.
  typedef std::function<void(bool idle)> IdleCallback;

  // Item in the queue. Wraps a node pointer and helps with priority sorting.
  class Item {
   public:
    Item(CalculatorNode* node, CalculatorContext* cc);
    // A null CalculatorContext indicates the task should run OpenNode().
    Item(CalculatorNode* node);

    CalculatorNode* Node() const { return node_; }

    CalculatorContext* Context() const { return cc_; }

    bool IsOpenNode() const { return is_open_node_; }

    // This comparison is meant to be used with a std::priority_queue. Since
    // the priority queue returns higher priority items first, this function
    // means "this is lower priority than that", i.e. "this runs after that".
    // - Non-sources have priority over sources.
    // - Sources are sorted by layer (lower layer numbers run first), then by
    //   Calculator::SourceProcessOrder (smaller values run first), then by
    //   node id: smaller ids run first, since they come earlier in the config.
    // - Non-sources are sorted by node id: larger ids run first, because they
    //   are closer to the leaves.
    bool operator<(const Item& that) const;

   private:
    int64_t source_process_order_ = 0;
    CalculatorNode* node_;
    CalculatorContext* cc_;
    int id_ = 0;
    int layer_ = 0;
    bool is_source_ = false;
    bool is_open_node_ = false;  // True if the task should run OpenNode().
  };

  explicit SchedulerQueue(absl::string_view queue_name, SchedulerShared* shared)
      : queue_name_(queue_name), shared_(shared) {}

  // Sets the executor that will run the nodes. Must be called before the
  // scheduler is started.
  void SetExecutor(Executor* executor);

  // Sets the idle callback. It is called exactly once whenever the queue goes
  // from idle to active, or vice versa.
  // Note: if the queue is accessed by multiple threads, it is possible for
  // a "become active" callback to be invoked before the "become idle" callback
  // that logically precedes it. However, the opposite cannot happen: a "become
  // idle" invocation is always preceded by the corresponding "become active",
  // because the callback is invoked before sending the tasks to the executor.
  // The scheduler counts the two types of invocations and keeps track of the
  // difference. When the difference reaches 0, the queue is guaranteed to be
  // actually idle.
  void SetIdleCallback(IdleCallback callback) {
    idle_callback_ = std::move(callback);
  }

  // Resets the data members at the beginning of each graph run.
  void Reset();

  // Implements the TaskQueue interface.
  void RunNextTask() override;

  // NOTE: After calling SetRunning(true), the caller must call
  // SubmitWaitingTasksToExecutor since tasks may have been added while the
  // queue was not running.
  void SetRunning(bool running) ABSL_LOCKS_EXCLUDED(mutex_);

  // Gets the number of tasks that need to be submitted to the executor, and
  // updates num_pending_tasks_. If this method is called and returns a
  // non-zero value, the executor's AddTask method *must* be called for each
  // task returned, but it can be called without holding the lock.
  int GetTasksToSubmitToExecutor() ABSL_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  // Submits tasks that are waiting (e.g. that were added while the queue was
  // not running) if the queue is running. The caller must not hold any mutex.
  void SubmitWaitingTasksToExecutor() ABSL_LOCKS_EXCLUDED(mutex_);

  // Adds a node and a calculator context to the scheduler queue if the node is
  // not already running. Note that if the node was running, then it will be
  // rescheduled upon completion (after checking dependencies), so this call is
  // not lost.
  void AddNode(CalculatorNode* node, CalculatorContext* cc)
      ABSL_LOCKS_EXCLUDED(mutex_);

  // Adds a node to the scheduler queue for an OpenNode() call.
  void AddNodeForOpen(CalculatorNode* node) ABSL_LOCKS_EXCLUDED(mutex_);

  // Adds an Item to queue_.
  void AddItemToQueue(Item&& item);

  void CleanupAfterRun() ABSL_LOCKS_EXCLUDED(mutex_);

 private:
  // Used internally by RunNextTask. Invokes ProcessNode or CloseNode, followed
  // by EndScheduling.
  void RunCalculatorNode(CalculatorNode* node, CalculatorContext* cc)
      ABSL_LOCKS_EXCLUDED(mutex_);

  // Used internally by RunNextTask. Invokes OpenNode, followed by
  // CheckIfBecameReady.
  void OpenCalculatorNode(CalculatorNode* node) ABSL_LOCKS_EXCLUDED(mutex_);

  // Checks whether the queue has no queued nodes or pending tasks.
  bool IsIdle() ABSL_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  // Queue name for logging purposes.
  const std::string queue_name_;

  Executor* executor_ = nullptr;

  IdleCallback idle_callback_;

  // The net number of times SetRunning(true) has been called.
  // SetRunning(true) increments running_count_ and SetRunning(false)
  // decrements it. The queue is running if running_count_ > 0. A running
  // queue will submit tasks to the executor.
  // Invariant: running_count_ <= 1.
  int running_count_ ABSL_GUARDED_BY(mutex_) = 0;

  // Number of tasks added to the Executor and not yet complete.
  int num_pending_tasks_ ABSL_GUARDED_BY(mutex_);

  // Number of tasks that need to be added to the Executor.
  int num_tasks_to_add_ ABSL_GUARDED_BY(mutex_);

  // Queue of nodes that need to be run.
  std::priority_queue<Item> queue_ ABSL_GUARDED_BY(mutex_);

  SchedulerShared* const shared_;

  absl::Mutex mutex_;
};

}  // namespace internal
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_SCHEDULER_QUEUE_H_
