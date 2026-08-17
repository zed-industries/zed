// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_MAIN_THREAD_MUTATOR_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_MAIN_THREAD_MUTATOR_CLIENT_H_

#include "base/memory/raw_ptr.h"
#include "third_party/blink/renderer/platform/graphics/animation_worklet_mutators_state.h"
#include "third_party/blink/renderer/platform/graphics/mutator_client.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

class AnimationWorkletMutatorDispatcherImpl;

class PLATFORM_EXPORT MainThreadMutatorClient : public MutatorClient {
 public:
  explicit MainThreadMutatorClient(
      std::unique_ptr<AnimationWorkletMutatorDispatcherImpl>);
  ~MainThreadMutatorClient() override = default;

  void SynchronizeAnimatorName(const String& animator_name) override;
  void SetMutationUpdate(std::unique_ptr<AnimationWorkletOutput>) override;
  void SetDelegate(MutatorClient* client);
  AnimationWorkletMutatorDispatcherImpl* Mutator() { return mutator_.get(); }

 private:
  std::unique_ptr<AnimationWorkletMutatorDispatcherImpl> mutator_;
  raw_ptr<MutatorClient> delegate_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_MAIN_THREAD_MUTATOR_CLIENT_H_
