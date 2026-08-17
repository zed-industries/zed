// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_COMPOSITOR_MUTATOR_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_COMPOSITOR_MUTATOR_CLIENT_H_

#include <memory>
#include "base/memory/raw_ptr.h"
#include "cc/trees/layer_tree_mutator.h"
#include "third_party/blink/renderer/platform/graphics/mutator_client.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

class AnimationWorkletMutatorDispatcherImpl;

class PLATFORM_EXPORT CompositorMutatorClient : public cc::LayerTreeMutator,
                                                public MutatorClient {
 public:
  explicit CompositorMutatorClient(
      std::unique_ptr<AnimationWorkletMutatorDispatcherImpl>);
  ~CompositorMutatorClient() override;

  void SynchronizeAnimatorName(const String& animator_name) override {}
  void SetMutationUpdate(std::unique_ptr<cc::MutatorOutputState>) override;

  // cc::LayerTreeMutator
  void SetClient(cc::LayerTreeMutatorClient*) override;
  bool Mutate(std::unique_ptr<cc::MutatorInputState>,
              MutateQueuingStrategy,
              DoneCallback) override;
  bool HasMutators() override;

 private:
  std::unique_ptr<AnimationWorkletMutatorDispatcherImpl> mutator_;
  raw_ptr<cc::LayerTreeMutatorClient> client_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_COMPOSITOR_MUTATOR_CLIENT_H_
