// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_PIPE_OPTIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_PIPE_OPTIONS_H_

#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"

namespace blink {

class AbortSignal;
class StreamPipeOptions;

class PipeOptions : public GarbageCollected<PipeOptions> {
 public:
  PipeOptions() = default;
  explicit PipeOptions(const StreamPipeOptions* options);

  bool PreventClose() const { return prevent_close_; }
  bool PreventAbort() const { return prevent_abort_; }
  bool PreventCancel() const { return prevent_cancel_; }
  AbortSignal* Signal() const { return signal_.Get(); }

  void Trace(Visitor*) const;

 private:
  bool prevent_close_ = false;
  bool prevent_abort_ = false;
  bool prevent_cancel_ = false;
  Member<AbortSignal> signal_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_PIPE_OPTIONS_H_
