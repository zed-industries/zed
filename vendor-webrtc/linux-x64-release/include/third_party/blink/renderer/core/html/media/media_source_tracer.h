// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_MEDIA_SOURCE_TRACER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_MEDIA_SOURCE_TRACER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

// Interface that encapsulates the Oilpan tracing of either same-thread or
// cross-thread attachment of an HTMLMediaElement and MediaSource, necessary
// since MediaSourceAttachments themselves are not managed by Oilpan. See
// concrete implementation(s) in modules/mediasource.
class CORE_EXPORT MediaSourceTracer
    : public GarbageCollected<MediaSourceTracer> {
 public:
  virtual ~MediaSourceTracer() = default;
  virtual void Trace(Visitor*) const;

  // Returns true iff the concrete tracer implementation supports cross-thread
  // attachments, for debug check only.
  virtual bool IsCrossThreadForDebugging() const = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_MEDIA_SOURCE_TRACER_H_
