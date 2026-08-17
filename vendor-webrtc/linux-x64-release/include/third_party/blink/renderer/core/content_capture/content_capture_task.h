// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CONTENT_CAPTURE_CONTENT_CAPTURE_TASK_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CONTENT_CAPTURE_CONTENT_CAPTURE_TASK_H_

#include <memory>
#include <optional>

#include "base/time/time.h"
#include "cc/paint/node_id.h"
#include "third_party/blink/renderer/core/content_capture/content_capture_task_histogram_reporter.h"
#include "third_party/blink/renderer/core/content_capture/task_session.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/disallow_new_wrapper.h"
#include "third_party/blink/renderer/platform/timer.h"
#include "third_party/blink/renderer/platform/wtf/ref_counted.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class WebContentCaptureClient;
class Document;
class LocalFrame;

// This class is used to capture the on-screen content and send them out
// through WebContentCaptureClient.
class CORE_EXPORT ContentCaptureTask
    : public GarbageCollected<ContentCaptureTask> {
 public:
  enum class ScheduleReason {
    kFirstContentChange,
    kUserActivatedContentChange,
    kNonUserActivatedContentChange,
    kScrolling,
    kRetryTask,
  };

  enum class TaskState {
    kProcessRetryTask,
    kCaptureContent,
    kProcessCurrentSession,
    kStop,
  };

  class CORE_EXPORT TaskDelay {
   public:
    explicit TaskDelay(const base::TimeDelta& task_initial_delay);
    // Resets the |delay_exponent| and returns the initial delay.
    base::TimeDelta ResetAndGetInitialDelay();

    // Returns the delay time for the next task.
    base::TimeDelta GetNextTaskDelay() const;

    // Increases delay time of next task exponentially after the task started.
    void IncreaseDelayExponent();

    base::TimeDelta task_initial_delay() const { return task_initial_delay_; }

   private:
    const base::TimeDelta task_initial_delay_;

    // The exponent to calculate the next task delay time.
    int delay_exponent_ = 0;
  };

  ContentCaptureTask(LocalFrame& local_frame_root, TaskSession& task_session);
  virtual ~ContentCaptureTask();

  // Schedule the task if it hasn't been done.
  void Schedule(ScheduleReason reason);
  void Shutdown();

  // Make those const public for testing purpose.
  static constexpr size_t kBatchSize = 5;

  // TODO(crbug.com/1115836): Replacing the ForTesting methods with friend
  // TestHelper class.
  TaskState GetTaskStateForTesting() const { return task_state_; }

  void RunTaskForTestingUntil(TaskState stop_state) {
    task_stop_for_testing_ = stop_state;
    Run(nullptr);
  }

  void SetCapturedContentForTesting(
      const Vector<cc::NodeInfo>& captured_content) {
    captured_content_for_testing_ = captured_content;
  }

  void ClearDocumentSessionsForTesting();

  base::TimeDelta GetTaskNextFireIntervalForTesting() const;
  void CancelTaskForTesting();
  const TaskDelay& GetTaskDelayForTesting() const { return *task_delay_; }
  void SetTaskStopForTesting(TaskState state) {
    task_stop_for_testing_ = state;
  }

  void Trace(Visitor*) const;

 protected:
  // All protected data and methods are for testing purpose.
  // Return true if the task should pause.
  // TODO(michaelbai): Uses RunTaskForTestingUntil().
  virtual bool ShouldPause();
  virtual WebContentCaptureClient* GetWebContentCaptureClient(const Document&);

 private:
  // Callback method of delay_task_, runs the content capture task and
  // reschedule it if it necessary.
  void Run(TimerBase*);

  // The actual run method, return if the task completed.
  bool RunInternal();

  // Runs the sub task to capture content.
  bool CaptureContent();

  // Runs the sub task to process the captured content and the detached nodes.
  bool ProcessSession();

  // Processes |doc_session|, return True if |doc_session| has been processed,
  // otherwise, the process was interrupted because the task has to pause.
  bool ProcessDocumentSession(TaskSession::DocumentSession& doc_session);

  // End a batch of content capture events.
  void EndBatchContent(TaskSession::DocumentSession& doc_session);

  // Sends the captured content in batch.
  void SendContent(TaskSession::DocumentSession& doc_session);

  // Gets the delay time of the next task according to the |reason|, this method
  // might adjusts the delay if applicable.
  base::TimeDelta GetAndAdjustDelay(ScheduleReason reason);

  void ScheduleInternal(ScheduleReason reason);
  bool CaptureContent(Vector<cc::NodeInfo>& data);

  void CancelTask();

  // Indicates if there is content change since last run.
  bool has_content_change_ = false;

  Member<LocalFrame> local_frame_root_;
  Member<TaskSession> task_session_;
  HeapTaskRunnerTimer<ContentCaptureTask> delay_task_;
  TaskState task_state_ = TaskState::kStop;

  std::unique_ptr<TaskDelay> task_delay_;

  scoped_refptr<ContentCaptureTaskHistogramReporter> histogram_reporter_;
  std::optional<TaskState> task_stop_for_testing_;
  std::optional<Vector<cc::NodeInfo>> captured_content_for_testing_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CONTENT_CAPTURE_CONTENT_CAPTURE_TASK_H_
