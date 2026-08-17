// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ANIMATIONWORKLET_ANIMATION_WORKLET_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ANIMATIONWORKLET_ANIMATION_WORKLET_H_

#include "third_party/blink/renderer/core/workers/worklet.h"
#include "third_party/blink/renderer/modules/animationworklet/animation_worklet_proxy_client.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/graphics/animation_worklet_mutators_state.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class LocalDOMWindow;

// Represents the animation worklet on the main thread. All the logic for
// loading a new source module is implemented in its parent class |Worklet|. The
// sole responsibility of this class it to create the appropriate
// |WorkletGlobalScopeProxy| instances that are responsible to proxy a
// corresponding |AnimationWorkletGlobalScope| on the worklet thread.
class MODULES_EXPORT AnimationWorklet final : public Worklet {
 public:
  explicit AnimationWorklet(LocalDOMWindow&);

  AnimationWorklet(const AnimationWorklet&) = delete;
  AnimationWorklet& operator=(const AnimationWorklet&) = delete;

  ~AnimationWorklet() override;

  WorkletAnimationId NextWorkletAnimationId();
  void Trace(Visitor*) const override;

 private:
  // Unique id associated with this worklet that is used by cc to identify all
  // animations associated it.
  int worklet_id_;
  int last_animation_id_;

  // Implements Worklet.
  bool NeedsToCreateGlobalScope() final;
  WorkletGlobalScopeProxy* CreateGlobalScope() final;

  Member<AnimationWorkletProxyClient> proxy_client_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ANIMATIONWORKLET_ANIMATION_WORKLET_H_
