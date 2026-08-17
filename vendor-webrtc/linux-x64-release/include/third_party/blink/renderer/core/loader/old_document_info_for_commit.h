// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_OLD_DOCUMENT_INFO_FOR_COMMIT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_OLD_DOCUMENT_INFO_FOR_COMMIT_H_

#include "third_party/blink/public/common/frame/user_activation_state.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/loader/history_item.h"

namespace blink {
// Contains information related to the previous document in a frame, to be
// given to the next document that is going to commit in this FrameLoader.
// Note that the "previous document" might not necessarily use the same
// FrameLoader as this one, e.g. in case of local RenderFrame swap.
struct OldDocumentInfoForCommit : GarbageCollected<OldDocumentInfoForCommit> {
  explicit OldDocumentInfoForCommit(
      scoped_refptr<SecurityOrigin> new_document_origin);
  void Trace(Visitor* visitor) const;
  // The unload timing info of the previous document in the frame. The new
  // document can access this information if it is a same-origin, to be
  // exposed through the Navigation Timing API.
  UnloadEventTimingInfo unload_timing_info;
  // The HistoryItem of the previous document in the frame. Some of the state
  // from the old document's HistoryItem will be copied to the new document
  // e.g. history.state will be copied on same-URL navigations. See also
  // https://github.com/whatwg/html/issues/6213.
  Member<HistoryItem> history_item;
  // Whether the previous document in the frame had sticky activation before
  // the commit.
  bool had_sticky_activation_before_navigation = false;
  // The `unreported_task_time` accumulated by the FrameSchedulerImpl, which
  // needs to be carried over in case of subframe navigations.
  base::TimeDelta frame_scheduler_unreported_task_time;
  // Whether the previous LocalFrame is the focused frame or not.
  bool was_focused_frame = false;
};

// Owns the OldDocumentInfoForCommit and exposes it through `info_`
// so that both the unloading old document and the committing new document
// can access and modify the value, without explicitly passing it between
// them on unload/commit time.
class ScopedOldDocumentInfoForCommitCapturer {
  STACK_ALLOCATED();

 public:
  explicit ScopedOldDocumentInfoForCommitCapturer(
      OldDocumentInfoForCommit* info)
      : info_(info), previous_capturer_(current_capturer_) {
    current_capturer_ = this;
  }

  ~ScopedOldDocumentInfoForCommitCapturer();

  // The last OldDocumentInfoForCommit set for `info_` that is still in scope.
  static OldDocumentInfoForCommit* CurrentInfo() {
    return current_capturer_ ? current_capturer_->info_ : nullptr;
  }

 private:
  OldDocumentInfoForCommit* info_;
  ScopedOldDocumentInfoForCommitCapturer* previous_capturer_;
  static ScopedOldDocumentInfoForCommitCapturer* current_capturer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_OLD_DOCUMENT_INFO_FOR_COMMIT_H_
