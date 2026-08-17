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

#ifndef MEDIAPIPE_FRAMEWORK_SCHEDULER_H_
#define MEDIAPIPE_FRAMEWORK_SCHEDULER_H_

#include <atomic>
#include <functional>
#include <map>
#include <memory>
#include <queue>
#include <set>
#include <utility>
#include <vector>

#include "absl/base/macros.h"
#include "absl/synchronization/mutex.h"
#include "mediapipe/framework/calculator_node.h"
#include "mediapipe/framework/port/status.h"
#include "mediapipe/framework/scheduler_queue.h"
#include "mediapipe/framework/scheduler_shared.h"

namespace mediapipe {

class CalculatorGraph;
class Executor;

namespace internal {

// The class scheduling a calculator graph.
class Scheduler {
 public:
  Scheduler(const Scheduler&) = delete;
  Scheduler& operator=(const Scheduler&) = delete;

  explicit Scheduler(CalculatorGraph* graph);

  ~Scheduler();

  // Sets the executor that will run the nodes. Must be called before the
  // scheduler is started. This is the normal executor used for nodes that
  // do not use a special one.
  void SetExecutor(Executor* executor);

  // Sets the executor that will run the nodes assigned to the executor
  // named |name|. Must be called before the scheduler is started.
  absl::Status SetNonDefaultExecutor(const std::string& name,
                                     Executor* executor);

  // Resets the data members at the beginning of each graph run.
  void Reset();

  // Starts scheduling nodes.
  void Start();

  // Wait for the current run to finish (block the current thread until all
  // source calculators have returned StatusStop(), all graph input streams
  // have been closed, and no more calculators can be run).
  // This function can be called only after Start().
  // Runs application thread tasks while waiting.
  absl::Status WaitUntilDone() ABSL_LOCKS_EXCLUDED(state_mutex_);

  // Wait until the running graph is in the idle mode, which is when nothing can
  // be scheduled and nothing is running in the worker threads.  This function
  // can be called only after Start().
  // Runs application thread tasks while waiting.
  //
  // Idleness requires:
  // 1. either the graph has no source nodes or all source nodes are closed, and
  // 2. no packets are added to graph input streams.
  //
  // For simplicity, we only fully support WaitUntilIdle() to be called on a
  // graph with no source nodes.
  //
  // The application must ensure no other threads are adding packets to graph
  // input streams while a WaitUntilIdle() call is in progress.
  absl::Status WaitUntilIdle() ABSL_LOCKS_EXCLUDED(state_mutex_);

  // Wait until any graph input stream has been unthrottled.
  // This is meant to be used by CalculatorGraph::AddPacketToInputStream, which
  // needs to check a status protected by its own mutex. That mutex, which
  // protects throttle changes on graph input streams, should be passed as the
  // secondary_mutex argument.
  // This function can be called by multiple threads concurrently.
  // Runs application thread tasks while waiting.
  void WaitUntilGraphInputStreamUnthrottled(absl::Mutex* secondary_mutex)
      ABSL_LOCKS_EXCLUDED(state_mutex_)
          ABSL_EXCLUSIVE_LOCKS_REQUIRED(secondary_mutex);

  // Wait until any observed output emits a packet. Like a semaphore,
  // this function returns immediately if an observed packet has already been
  // emitted since the previous call. This relies on the fact that the calls are
  // in sequence. Runs application thread tasks while waiting.
  // Returns absl::OutOfRangeError if the graph terminated.
  absl::Status WaitForObservedOutput() ABSL_LOCKS_EXCLUDED(state_mutex_);

  // Callback that is invoked by a node when it wants to be scheduled.
  // If the node is throttled, the call is ignored.
  // This method is thread-safe.
  void ScheduleNodeIfNotThrottled(CalculatorNode* node, CalculatorContext* cc);

  // Schedules an OpenNode() call for |node|.
  void ScheduleNodeForOpen(CalculatorNode* node);

  // Adds all the nodes in |nodes_to_schedule| to the scheduler queue, without
  // checking if they are ready. Called by the graph when unthrottling nodes.
  void ScheduleUnthrottledReadyNodes(
      const std::vector<CalculatorNode*>& nodes_to_schedule);

  void QueueIdleStateChanged(bool idle);

  // Called by DelegatingExecutor to add an application thread task.
  void AddApplicationThreadTask(std::function<void()> task);

  // Adds |node| to |unopened_sources_|.
  // This can only be called before the scheduler is started.
  void AddUnopenedSourceNode(CalculatorNode* node);

  // Adds |node| to |sources_queue_|.
  void AddNodeToSourcesQueue(CalculatorNode* node)
      ABSL_LOCKS_EXCLUDED(state_mutex_);

  // Assigns node to a scheduler queue.
  void AssignNodeToSchedulerQueue(CalculatorNode* node);

  // Pauses the scheduler.  Does nothing if Cancel has been called.
  void Pause() ABSL_LOCKS_EXCLUDED(state_mutex_);

  // Resumes the scheduler.
  void Resume() ABSL_LOCKS_EXCLUDED(state_mutex_);

  // Aborts the scheduler if the graph is started but is not terminated; no-op
  // otherwise.  For the graph to properly be cancelled, graph_->HasError()
  // must also return true.
  void Cancel() ABSL_LOCKS_EXCLUDED(state_mutex_);

  // Returns true if scheduler is paused.
  bool IsPaused() ABSL_LOCKS_EXCLUDED(state_mutex_);

  // Returns true if scheduler is terminated.
  bool IsTerminated() ABSL_LOCKS_EXCLUDED(state_mutex_);

  // Cleanup any remaining state after the run.
  void CleanupAfterRun();

  void SetHasError(bool error) { shared_.has_error = error; }

  // Notifies the scheduler that a packet was added to a graph input stream.
  // The scheduler needs to check whether it is still deadlocked, and
  // unthrottle again if so.
  void AddedPacketToGraphInputStream() ABSL_LOCKS_EXCLUDED(state_mutex_);

  void ThrottledGraphInputStream() ABSL_LOCKS_EXCLUDED(state_mutex_);
  void UnthrottledGraphInputStream() ABSL_LOCKS_EXCLUDED(state_mutex_);
  void EmittedObservedOutput() ABSL_LOCKS_EXCLUDED(state_mutex_);

  // Closes all source nodes at the next scheduling opportunity.
  void CloseAllSourceNodes();

  // Notifies the scheduler that all graph input streams have been closed.
  void ClosedAllGraphInputStreams();

  // Returns the scheduler's runtime measures for overhead measurement.
  // Only meant for test purposes. See SchedulerTimer for details.
  internal::SchedulerTimes GetSchedulerTimes();

 private:
  // State of the scheduler. The figure shows the allowed state transitons.
  //
  //   NOT_STARTED
  //        |
  //        v
  //     RUNNING--+
  //     | | ^    |
  //     | |  \   |
  //     | |   \  v
  //     | |  PAUSED
  //     | |    |
  //     | v    v
  //     | CANCELLING
  //     |     |
  //     v     v
  //   TERMINATING
  //        |
  //        v
  //    TERMINATED
  enum State {
    STATE_NOT_STARTED = 0,  // The initial state.
    STATE_RUNNING = 1,      // The scheduler is running and scheduling nodes.
    STATE_PAUSED = 2,       // The scheduler is not scheduling nodes.
    STATE_CANCELLING = 3,   // The scheduler is being cancelled. The scheduler
                            // cannot be paused in this state so that
                            // scheduler_queue_ can be drained.
    STATE_TERMINATED = 4,   // The scheduler has terminated.
  };

  // Sorts the source nodes in unopened_sources_ by source layer.
  struct SourceLayerCompare {
    bool operator()(CalculatorNode* lhs, CalculatorNode* rhs) const {
      if (lhs->source_layer() != rhs->source_layer()) {
        return lhs->source_layer() < rhs->source_layer();
      }
      return lhs->Id() < rhs->Id();
    }
  };

  // Start (or resume) or stop all queues.
  void SetQueuesRunning(bool running);

  // Submit waiting tasks on all queues after resuming.
  void SubmitWaitingTasksOnQueues();

  // Returns true if nothing can be scheduled and no tasks are running or
  // scheduled to run on the Executor.
  bool IsIdle() ABSL_EXCLUSIVE_LOCKS_REQUIRED(state_mutex_);

  // Clean up active_sources_ by removing closed sources. If all the active
  // sources are closed, this will leave active_sources_ empty. If not, some
  // closed sources may be left in there.
  void CleanupActiveSources();

  // Adds the next layer of sources to the scheduler queue if the previous layer
  // has finished running.
  // Returns true if it scheduled any sources.
  bool TryToScheduleNextSourceLayer()
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(state_mutex_);

  // Takes care of three different operations, as needed:
  // - activating sources;
  // - unthrottling sources or graph input streams to resolve a deadlock.
  // - terminating the scheduler.
  // Thread-safe and reentrant.
  // TODO: analyze call sites, split it up further.
  void HandleIdle() ABSL_EXCLUSIVE_LOCKS_REQUIRED(state_mutex_);

  // Terminates the scheduler. Should only be called by HandleIdle.
  void Quit() ABSL_EXCLUSIVE_LOCKS_REQUIRED(state_mutex_);

  // Helper for the various Wait methods. Waits for the given condition,
  // running application thread tasks in the meantime.
  void ApplicationThreadAwait(const std::function<bool()>& stop_condition);

  // The calculator graph to run.
  CalculatorGraph* graph_;

  // Data accessed by all SchedulerQueues.
  SchedulerShared shared_;

  // Queue of nodes that need to be run.
  SchedulerQueue default_queue_;

  // Non-default scheduler queues, keyed by their executor names.
  std::map<std::string, std::unique_ptr<SchedulerQueue>> non_default_queues_;

  // Holds pointers to all queues used by the scheduler, for convenience.
  std::vector<SchedulerQueue*> scheduler_queues_;

  // Priority queue of source nodes ordered by layer and then source process
  // order. This stores the set of sources that are yet to be run.
  std::priority_queue<SchedulerQueue::Item> sources_queue_
      ABSL_GUARDED_BY(state_mutex_);

  // Source nodes with the smallest source layer are at the beginning of
  // unopened_sources_. Before the scheduler is started, all source nodes are
  // added to unopened_sources_. Once the scheduler starts running,
  // unopened_sources_ should only be accessed under the protection of
  // state_mutex_. A source node is removed from unopened_sources_ after it is
  // opened.
  std::set<CalculatorNode*, SourceLayerCompare> unopened_sources_;

  // Keeps track of sources that can be considered for scheduling. Sources are
  // scheduled in layers, and those that are not currently active will not be
  // scheduled even if ready. Sources are removed once they are closed.
  std::vector<CalculatorNode*> active_sources_;

  // Condition variable used to wait for some changes to the scheduler state.
  // These correspond to the Wait* methods in this class.
  // Not all state changes need to signal this, only those that enter one of
  // the waitable states.
  absl::CondVar state_cond_var_ ABSL_GUARDED_BY(state_mutex_);

  // Number of queues which are not idle.
  // Note: this indicates two slightly different things:
  //  a. the number of queues which still have nodes running;
  //  b. the number of queues whose executors may still access the scheduler.
  // When a queue becomes idle, it has stopped running nodes, and the scheduler
  // decrements the count. However, it is not done accessing the scheduler
  // until HandleIdle returns. Therefore, a and b are briefly out of sync.
  // This is ok, because it happens within a single critical section, which is
  // guarded by state_mutex_. If we wanted to split this critical section, we
  // would have to separate a and b into two variables.
  int non_idle_queue_count_ ABSL_GUARDED_BY(state_mutex_) = 0;

  // Tasks to be executed on the application thread.
  std::deque<std::function<void()>> app_thread_tasks_
      ABSL_GUARDED_BY(state_mutex_);

  // Used by HandleIdle to avoid multiple concurrent executions.
  // We cannot simply hold a mutex throughout it, for two reasons:
  // - We need it to be reentrant, which Mutex does not support.
  // - We want simultaneous calls to return immediately instead of waiting,
  //   and Mutex's TryLock is not guaranteed to work.
  int handling_idle_ ABSL_GUARDED_BY(state_mutex_) = 0;

  // Mutex for the scheduler state and related things.
  // Note: state_ is declared as atomic so that its getter methods don't need
  // to acquire state_mutex_.
  absl::Mutex state_mutex_;

  // Current state of the scheduler.
  std::atomic<State> state_ = STATE_NOT_STARTED;

  // True if all graph input streams are closed.
  bool graph_input_streams_closed_ ABSL_GUARDED_BY(state_mutex_) = false;

  // Number of throttled graph input streams.
  int throttled_graph_input_stream_count_ ABSL_GUARDED_BY(state_mutex_) = 0;

  // Used to stop WaitUntilGraphInputStreamUnthrottled.
  int unthrottle_seq_num_ ABSL_GUARDED_BY(state_mutex_) = 0;

  // Used to stop WaitForObservedOutput.
  bool observed_output_signal_ ABSL_GUARDED_BY(state_mutex_) = false;

  // True if an application thread is waiting in WaitForObservedOutput.
  bool waiting_for_observed_output_ ABSL_GUARDED_BY(state_mutex_) = false;
};

}  // namespace internal
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_SCHEDULER_H_
