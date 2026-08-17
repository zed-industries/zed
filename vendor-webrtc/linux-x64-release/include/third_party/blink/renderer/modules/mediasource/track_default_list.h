// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASOURCE_TRACK_DEFAULT_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASOURCE_TRACK_DEFAULT_LIST_H_

#include "third_party/blink/renderer/modules/mediasource/track_default.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class ExceptionState;

class TrackDefaultList final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Implement the IDL
  static TrackDefaultList* Create(const HeapVector<Member<TrackDefault>>&,
                                  ExceptionState&);

  TrackDefaultList() = default;
  explicit TrackDefaultList(const HeapVector<Member<TrackDefault>>&);

  unsigned length() const { return track_defaults_.size(); }
  TrackDefault* item(unsigned) const;

  void Trace(Visitor*) const override;

 private:
  const HeapVector<Member<TrackDefault>> track_defaults_{};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASOURCE_TRACK_DEFAULT_LIST_H_
