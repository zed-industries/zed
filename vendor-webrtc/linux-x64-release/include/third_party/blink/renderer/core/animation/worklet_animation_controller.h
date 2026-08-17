// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_WORKLET_ANIMATION_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_WORKLET_ANIMATION_CONTROLLER_H_

#include "base/memory/weak_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/core/animation/animation_effect.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/graphics/animation_worklet_mutators_state.h"
#include "third_party/blink/renderer/platform/graphics/mutator_client.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"

namespace blink {

class AnimationWorkletMutatorDispatcherImpl;
class Document;
class MainThreadMutatorClient;
class WorkletAnimationBase;

// Handles AnimationWorklet animations on the main-thread.
//
// The WorkletAnimationController is responsible for owning WorkletAnimation
// instances are long as they are relevant to the animation system. It is also
// responsible for starting valid WorkletAnimations on the compositor side and
// updating WorkletAnimations with updated results from their underpinning
// AnimationWorklet animator instance.
//
// For more details on AnimationWorklet, see the spec:
// https://wicg.github.io/animation-worklet
class CORE_EXPORT WorkletAnimationController
    : public GarbageCollected<WorkletAnimationController>,
      public MutatorClient {
 public:
  explicit WorkletAnimationController(Document*);
  ~WorkletAnimationController() override;

  void AttachAnimation(WorkletAnimationBase&);
  void DetachAnimation(WorkletAnimationBase&);
  void InvalidateAnimation(WorkletAnimationBase&);

  void UpdateAnimationStates();
  void UpdateAnimationTimings(TimingUpdateReason);

  base::WeakPtr<AnimationWorkletMutatorDispatcherImpl>
  EnsureMainThreadMutatorDispatcher(
      scoped_refptr<base::SingleThreadTaskRunner> mutator_task_runner);

  void SetMutationUpdate(
      std::unique_ptr<AnimationWorkletOutput> output) override;

  void SynchronizeAnimatorName(const String& animator_name) override;
  // Returns true if the animator with given name is registered in
  // AnimationWorkletGlobalScope.
  bool IsAnimatorRegistered(const String& animator_name) const;

  void Trace(Visitor*) const;

 private:
  void MutateAnimations();
  std::unique_ptr<AnimationWorkletDispatcherInput> CollectAnimationStates();
  void ApplyAnimationTimings(TimingUpdateReason reason);

  HeapHashSet<Member<WorkletAnimationBase>> pending_animations_;
  HeapHashMap<int, Member<WorkletAnimationBase>> animations_;

  WTF::HashSet<String> animator_names_;

  // TODO(crbug.com/1090515): The following proxy is needed for platform/ to
  // access this class. We should bypass it eventually.
  std::unique_ptr<MainThreadMutatorClient> main_thread_mutator_client_;

  Member<Document> document_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_WORKLET_ANIMATION_CONTROLLER_H_
