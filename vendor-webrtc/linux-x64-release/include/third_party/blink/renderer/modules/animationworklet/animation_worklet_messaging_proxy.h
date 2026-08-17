// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ANIMATIONWORKLET_ANIMATION_WORKLET_MESSAGING_PROXY_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ANIMATIONWORKLET_ANIMATION_WORKLET_MESSAGING_PROXY_H_

#include <memory>
#include "third_party/blink/renderer/core/workers/threaded_worklet_messaging_proxy.h"
#include "third_party/blink/renderer/modules/animationworklet/animation_worklet_proxy_client.h"

namespace blink {

class ExecutionContext;
class WorkerThread;

// Acts as a proxy for the animation worklet global scopes that live on the
// worklet thread. The logic to actually proxy an off thread global scope is
// implemented in its parent. The main contribution of this class is to
// create an appropriate worklet thread type (in this case an
// |AnimationWorkletThread|) as part of the the worklet initialization process.
class AnimationWorkletMessagingProxy final
    : public ThreadedWorkletMessagingProxy {
 public:
  explicit AnimationWorkletMessagingProxy(ExecutionContext*);
  ~AnimationWorkletMessagingProxy() override;

  void Trace(Visitor*) const override;

 private:
  std::unique_ptr<WorkerThread> CreateWorkerThread() override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ANIMATIONWORKLET_ANIMATION_WORKLET_MESSAGING_PROXY_H_
