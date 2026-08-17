// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ANIMATIONWORKLET_WORKLET_ANIMATION_OPTIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ANIMATIONWORKLET_WORKLET_ANIMATION_OPTIONS_H_

#include "third_party/blink/renderer/bindings/core/v8/serialization/serialized_script_value.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/animation/compositor_animation.h"

namespace blink {

class MODULES_EXPORT WorkletAnimationOptions final
    : public cc::AnimationOptions {
 public:
  explicit WorkletAnimationOptions(scoped_refptr<SerializedScriptValue>);
  WorkletAnimationOptions(const WorkletAnimationOptions&) = default;
  WorkletAnimationOptions& operator=(const WorkletAnimationOptions&) = default;
  std::unique_ptr<cc::AnimationOptions> Clone() const override;

  scoped_refptr<SerializedScriptValue> GetData() { return data_; }
  ~WorkletAnimationOptions() override;

 private:
  scoped_refptr<SerializedScriptValue> data_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ANIMATIONWORKLET_WORKLET_ANIMATION_OPTIONS_H_
