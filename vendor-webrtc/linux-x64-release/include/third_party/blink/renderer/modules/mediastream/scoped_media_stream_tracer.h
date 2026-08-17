// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_SCOPED_MEDIA_STREAM_TRACER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_SCOPED_MEDIA_STREAM_TRACER_H_

#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {
// The class traces with "mediastream" category with passed event name.
// The client should provide the `event_name` of the event that is being traced.

// The trace will begin with the class instantiation and end with the class
// destruction or exclusive call of `End()` method.
class ScopedMediaStreamTracer {
 public:
  // It uses `this` object as a trace id as each ScopedMediaStreamTracer
  // represent the unique event.
  explicit ScopedMediaStreamTracer(const WTF::String& event_name);
  ~ScopedMediaStreamTracer();

  // Finish the trace. This method should be called only once.
  void End();

 private:
  const WTF::String event_name_;
  bool finished_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_SCOPED_MEDIA_STREAM_TRACER_H_
