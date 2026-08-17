/*
 * Copyright (C) 2007 Apple Inc.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_PROGRESS_TRACKER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_PROGRESS_TRACKER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/loader/frame_loader_types.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_load_priority.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"

namespace blink {

class LocalFrameClient;
class LocalFrame;
class ResourceResponse;

struct ProgressItem {
  USING_FAST_MALLOC(ProgressItem);

 public:
  int64_t bytes_received = 0;
  int64_t estimated_length = 0;
};

// FIXME: This is only used on Android. Android is the only Chrome
// browser which shows a progress bar during loading.
// We should find a better way for Android to get this data and remove this!
class CORE_EXPORT ProgressTracker final
    : public GarbageCollected<ProgressTracker> {
 public:
  explicit ProgressTracker(LocalFrame*);
  ProgressTracker(const ProgressTracker&) = delete;
  ProgressTracker& operator=(const ProgressTracker&) = delete;
  ~ProgressTracker();
  void Trace(Visitor*) const;
  void Dispose();

  double EstimatedProgress() const;

  void ProgressStarted();
  void ProgressCompleted();

  void FinishedParsing();
  void DidFirstContentfulPaint();

  void WillStartLoading(uint64_t identifier, ResourceLoadPriority);
  void IncrementProgress(uint64_t identifier, const ResourceResponse&);
  void IncrementProgress(uint64_t identifier, uint64_t);
  void CompleteProgress(uint64_t identifier);

 private:
  LocalFrameClient* GetLocalFrameClient() const;

  void UpdateProgressItem(ProgressItem& item,
                          int64_t bytes_received,
                          int64_t estimated_length);

  void MaybeSendProgress();
  void SendFinalProgress();
  void Reset();

  bool HaveParsedAndPainted();

  Member<LocalFrame> frame_;
  double last_notified_progress_value_;
  double last_notified_progress_time_;
  bool finished_parsing_;
  bool did_first_contentful_paint_;
  double progress_value_;

  int64_t bytes_received_ = 0;
  int64_t estimated_bytes_for_pending_requests_ = 0;

  HashMap<uint64_t, ProgressItem> progress_items_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_PROGRESS_TRACKER_H_
