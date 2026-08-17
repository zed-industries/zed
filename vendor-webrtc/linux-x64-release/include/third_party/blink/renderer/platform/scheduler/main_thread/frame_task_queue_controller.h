// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_FRAME_TASK_QUEUE_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_FRAME_TASK_QUEUE_CONTROLLER_H_

#include <memory>
#include <utility>

#include "base/functional/callback.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/scheduler/main_thread/main_thread_task_queue.h"
#include "third_party/blink/renderer/platform/scheduler/public/web_scheduling_priority.h"
#include "third_party/blink/renderer/platform/scheduler/public/web_scheduling_queue_type.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/perfetto/include/perfetto/tracing/traced_value_forward.h"

namespace base {
namespace sequence_manager {
class TaskQueue;
}  // namespace sequence_manager
namespace trace_event {
class TracedValue;
}  // namespace trace_event
}  // namespace base

namespace blink {
namespace scheduler {

class FrameSchedulerImpl;
class FrameTaskQueueControllerTest;
class MainThreadSchedulerImpl;

// FrameTaskQueueController creates and manages and FrameSchedulerImpl's task
// queues. It is in charge of maintaining mappings between QueueTraits and
// MainThreadTaskQueues for queues, for accessing task queues and
// their related voters, and for creating new task queues.
class PLATFORM_EXPORT FrameTaskQueueController {
  USING_FAST_MALLOC(FrameTaskQueueController);

 public:
  using TaskQueueAndEnabledVoterPair =
      std::pair<MainThreadTaskQueue*,
                base::sequence_manager::TaskQueue::QueueEnabledVoter*>;

  // Delegate receives callbacks when task queues are created to perform any
  // additional task queue setup or tasks.
  class Delegate {
   public:
    Delegate() = default;
    Delegate(const Delegate&) = delete;
    Delegate& operator=(const Delegate&) = delete;
    virtual ~Delegate() = default;

    virtual void OnTaskQueueCreated(
        MainThreadTaskQueue*,
        base::sequence_manager::TaskQueue::QueueEnabledVoter*) = 0;
  };

  FrameTaskQueueController(MainThreadSchedulerImpl*,
                           FrameSchedulerImpl*,
                           Delegate*);
  FrameTaskQueueController(const FrameTaskQueueController&) = delete;
  FrameTaskQueueController& operator=(const FrameTaskQueueController&) = delete;
  ~FrameTaskQueueController();

  // Return the task queue associated with the given queue traits,
  // and create if it doesn't exist.
  scoped_refptr<MainThreadTaskQueue> GetTaskQueue(
      MainThreadTaskQueue::QueueTraits);

  scoped_refptr<MainThreadTaskQueue> NewWebSchedulingTaskQueue(
      MainThreadTaskQueue::QueueTraits,
      WebSchedulingQueueType,
      WebSchedulingPriority);

  void RemoveWebSchedulingTaskQueue(MainThreadTaskQueue*);

  // Get the list of all task queue and voter pairs.
  const Vector<TaskQueueAndEnabledVoterPair>& GetAllTaskQueuesAndVoters() const;

  // Gets the associated QueueEnabledVoter for the given task queue, or nullptr
  // if one doesn't exist.
  base::sequence_manager::TaskQueue::QueueEnabledVoter* GetQueueEnabledVoter(
      const scoped_refptr<MainThreadTaskQueue>&);

  void WriteIntoTrace(perfetto::TracedValue context) const;

 private:
  friend class FrameTaskQueueControllerTest;

  void CreateTaskQueue(MainThreadTaskQueue::QueueTraits);

  void TaskQueueCreated(const scoped_refptr<MainThreadTaskQueue>&);

  // Removes a queue from |all_task_queues_and_voters_| and
  // |task_queue_enabled_voters_|. This method enforces that the queue is in the
  // collection before removal. Removes are linear in the total number of task
  // queues.
  void RemoveTaskQueueAndVoter(MainThreadTaskQueue*);

  // Map a set of QueueTraits to a QueueType.
  // TODO(crbug.com/877245): Consider creating a new queue type kFrameNonLoading
  // and use it instead of this for new queue types.
  static MainThreadTaskQueue::QueueType QueueTypeFromQueueTraits(
      MainThreadTaskQueue::QueueTraits);

  const raw_ptr<MainThreadSchedulerImpl, DanglingUntriaged>
      main_thread_scheduler_impl_;
  const raw_ptr<FrameSchedulerImpl> frame_scheduler_impl_;
  const raw_ptr<Delegate> delegate_;

  using TaskQueueMap =
      WTF::HashMap<MainThreadTaskQueue::QueueTraitsKeyType,
                   scoped_refptr<MainThreadTaskQueue>>;

  // Map of all TaskQueues, indexed by QueueTraits.
  TaskQueueMap task_queues_;

  using TaskQueueEnabledVoterMap = WTF::HashMap<
      scoped_refptr<MainThreadTaskQueue>,
      std::unique_ptr<base::sequence_manager::TaskQueue::QueueEnabledVoter>>;

  // QueueEnabledVoters for the task queues we've created.
  TaskQueueEnabledVoterMap task_queue_enabled_voters_;

  // The list of all task queue and voter pairs for all QueueTypeInternal queue
  // types.
  Vector<TaskQueueAndEnabledVoterPair> all_task_queues_and_voters_;
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_FRAME_TASK_QUEUE_CONTROLLER_H_
