// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CONTENT_CAPTURE_CONTENT_CAPTURE_MANAGER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CONTENT_CAPTURE_CONTENT_CAPTURE_MANAGER_H_

#include "base/memory/scoped_refptr.h"
#include "base/time/time.h"
#include "third_party/blink/public/common/input/web_input_event.h"
#include "third_party/blink/renderer/core/content_capture/content_capture_task.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/graphics/dom_node_id.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class LocalFrame;
class Node;

// This class is used to create the NodeHolder, and start the ContentCaptureTask
// when necessary. The ContentCaptureManager is owned by main frame.
class CORE_EXPORT ContentCaptureManager
    : public GarbageCollected<ContentCaptureManager> {
 public:
  explicit ContentCaptureManager(LocalFrame& local_frame_root);
  virtual ~ContentCaptureManager();

  // Schedules ContentCaptureTask if it isn't already scheduled. The |node| is
  // the one newly painted.
  void ScheduleTaskIfNeeded(const Node& node);

  // Invokes when the |node_holder| associated LayoutText will be destroyed.
  void OnLayoutTextWillBeDestroyed(const Node& node);

  // Invokes when scroll position was changed.
  void OnScrollPositionChanged();

  // Invoked on the user input on the |local_frame|.
  void NotifyInputEvent(WebInputEvent::Type type,
                        const LocalFrame& local_frame);

  // Invokes when text node content was changed.
  void OnNodeTextChanged(Node& node);

  // Invokes when the LocalFrameRoot was shown/hidden.
  void OnFrameWasShown();
  void OnFrameWasHidden();

  // Invokes when the local_frame_root shutdown.
  void Shutdown();

  virtual void Trace(Visitor*) const;

  ContentCaptureTask* GetContentCaptureTaskForTesting() const {
    return content_capture_idle_task_.Get();
  }

 protected:
  virtual ContentCaptureTask* CreateContentCaptureTask();
  TaskSession& GetTaskSessionForTesting() const { return *task_session_; }

 private:
  struct UserActivation : public GarbageCollected<UserActivation> {
    explicit UserActivation(const LocalFrame& local_frame);

    // The LocalFrame that the user activation occurred.
    const WeakMember<const LocalFrame> local_frame;
    const base::TimeTicks activation_time;

    virtual void Trace(Visitor*) const;
  };

  void ScheduleTask(ContentCaptureTask::ScheduleReason reason);

  // Returns true if the user had the input in last
  // |kUserActivationExpiryPeriod| on the |node|'s frame.
  bool UserActivated(const Node& node) const;

  Member<ContentCaptureTask> content_capture_idle_task_;

  Member<LocalFrame> local_frame_root_;

  // Indicates if the first NodeHolder is created.
  bool first_node_holder_created_ = false;

  Member<TaskSession> task_session_;

  // The latest user activation in any frame of the |local_frame_root_|.
  Member<UserActivation> latest_user_activation_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CONTENT_CAPTURE_CONTENT_CAPTURE_MANAGER_H_
