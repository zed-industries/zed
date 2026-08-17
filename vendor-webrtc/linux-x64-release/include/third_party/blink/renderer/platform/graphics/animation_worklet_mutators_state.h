// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_ANIMATION_WORKLET_MUTATORS_STATE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_ANIMATION_WORKLET_MUTATORS_STATE_H_

#include "cc/trees/layer_tree_mutator.h"

namespace blink {

using AnimationWorkletInput = cc::AnimationWorkletInput;
using AnimationWorkletOutput = cc::AnimationWorkletOutput;
using AnimationWorkletDispatcherInput = cc::MutatorInputState;
using AnimationWorkletDispatcherOutput = cc::MutatorOutputState;
using WorkletAnimationId = cc::WorkletAnimationId;
using MutateQueuingStrategy = cc::MutateQueuingStrategy;
using MutateStatus = cc::MutateStatus;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_ANIMATION_WORKLET_MUTATORS_STATE_H_
