// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ANIMATIONWORKLET_WORKLET_GROUP_EFFECT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ANIMATIONWORKLET_WORKLET_GROUP_EFFECT_H_

#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/modules/animationworklet/worklet_animation_effect.h"
#include "third_party/blink/renderer/modules/animationworklet/worklet_animation_effect_timings.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"

namespace blink {

class MODULES_EXPORT WorkletGroupEffect : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit WorkletGroupEffect(
      const Vector<std::optional<base::TimeDelta>>& local_times,
      const Vector<Timing>& timings,
      const Vector<Timing::NormalizedTiming>& normalized_timings);
  const HeapVector<Member<WorkletAnimationEffect>>& getChildren() {
    return effects_;
  }
  void Trace(Visitor*) const override;

 private:
  HeapVector<Member<WorkletAnimationEffect>> effects_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ANIMATIONWORKLET_WORKLET_GROUP_EFFECT_H_
