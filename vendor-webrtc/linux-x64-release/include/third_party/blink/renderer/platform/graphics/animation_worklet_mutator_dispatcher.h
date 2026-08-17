// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_ANIMATION_WORKLET_MUTATOR_DISPATCHER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_ANIMATION_WORKLET_MUTATOR_DISPATCHER_H_

#include "base/functional/bind.h"
#include "third_party/blink/renderer/platform/graphics/animation_worklet_mutators_state.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"

namespace blink {

class PLATFORM_EXPORT AnimationWorkletMutatorDispatcher {
 public:
  virtual ~AnimationWorkletMutatorDispatcher() = default;

  using AsyncMutationCompleteCallback =
      WTF::CrossThreadOnceFunction<void(MutateStatus)>;

  // Run the animation frame callbacks from all connected AnimationWorklets.
  virtual void MutateSynchronously(
      std::unique_ptr<AnimationWorkletDispatcherInput>) = 0;

  // Queues the animation frame callbacks from all connected AnimationWorklets.
  // The queuing strategy determines what action to take when busy servicing
  // another request. The callback is triggered on completion or canceling of
  // the mutation cycle. Returns true if mutations results are expected.
  virtual bool MutateAsynchronously(
      std::unique_ptr<AnimationWorkletDispatcherInput>,
      MutateQueuingStrategy,
      AsyncMutationCompleteCallback) = 0;

  // Returns true if Mutate may do something if called 'now'.
  virtual bool HasMutators() = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_ANIMATION_WORKLET_MUTATOR_DISPATCHER_H_
