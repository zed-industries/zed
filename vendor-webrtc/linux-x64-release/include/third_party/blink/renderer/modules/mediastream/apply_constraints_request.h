// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_APPLY_CONSTRAINTS_REQUEST_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_APPLY_CONSTRAINTS_REQUEST_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/modules/mediastream/media_constraints.h"
#include "third_party/blink/renderer/modules/mediastream/media_stream_track.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"

namespace blink {

class MODULES_EXPORT ApplyConstraintsRequest final
    : public GarbageCollected<ApplyConstraintsRequest> {
 public:
  ApplyConstraintsRequest(MediaStreamTrack*,
                          const MediaConstraints&,
                          ScriptPromiseResolver<IDLUndefined>*);

  MediaStreamComponent* Track() const;
  MediaConstraints Constraints() const;

  void RequestSucceeded();
  void RequestFailed(const String& constraint, const String& message);

  void Trace(Visitor*) const;

 private:
  Member<MediaStreamTrack> track_;
  MediaConstraints constraints_;
  Member<ScriptPromiseResolver<IDLUndefined>> resolver_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_APPLY_CONSTRAINTS_REQUEST_H_
