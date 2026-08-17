// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ANIMATIONWORKLET_ANIMATION_WORKLET_GLOBAL_SCOPE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ANIMATIONWORKLET_ANIMATION_WORKLET_GLOBAL_SCOPE_H_

#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/core/workers/worklet_global_scope.h"
#include "third_party/blink/renderer/modules/animationworklet/animator.h"
#include "third_party/blink/renderer/modules/animationworklet/animator_definition.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/graphics/animation_worklet_mutators_state.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class ExceptionState;
class V8AnimatorConstructor;
class WorkletAnimationOptions;

// Represents the animation worklet global scope and implements all methods that
// the global scope exposes to user script (See
// |AnimationWorkletGlobalScope.idl|). The instances of this class live on the
// worklet thread but have a corresponding proxy on the main thread which is
// accessed by the animation worklet instance. User scripts can register
// animator definitions with the global scope (via |registerAnimator| method).
// The scope keeps a map of these animator definitions and can look them up
// based on their name. The scope also owns a list of active animators that it
// animates.
class MODULES_EXPORT AnimationWorkletGlobalScope : public WorkletGlobalScope {
  DEFINE_WRAPPERTYPEINFO();

 public:
  AnimationWorkletGlobalScope(std::unique_ptr<GlobalScopeCreationParams>,
                              WorkerThread*);
  ~AnimationWorkletGlobalScope() override;

  void Trace(Visitor*) const override;
  void Dispose() override;
  bool IsAnimationWorkletGlobalScope() const final { return true; }

  void UpdateAnimatorsList(const AnimationWorkletInput&);

  // Invokes the |animate| function of selected animators.
  void UpdateAnimators(const AnimationWorkletInput&,
                       AnimationWorkletOutput*,
                       bool (*predicate)(Animator*));

  // Registers a animator definition with the given name and constructor.
  void registerAnimator(const String& name,
                        V8AnimatorConstructor* animator_ctor,
                        ExceptionState&);

  AnimatorDefinition* FindDefinitionForTest(const String& name);
  bool IsAnimatorStateful(int animation_id);
  void MigrateAnimatorsTo(AnimationWorkletGlobalScope*);
  Animator* GetAnimator(int animation_id) {
    return animators_.at(animation_id);
  }
  unsigned GetAnimatorsSizeForTest() { return animators_.size(); }

  // Returns the token that uniquely identifies this worklet.
  const AnimationWorkletToken& GetAnimationWorkletToken() const {
    return token_;
  }
  WorkletToken GetWorkletToken() const final { return token_; }
  ExecutionContextToken GetExecutionContextToken() const final {
    return token_;
  }

 private:
  void RegisterWithProxyClientIfNeeded();

  // TODO(crbug.com/1286242): Return a proper destination for AnimationWorklet.
  network::mojom::RequestDestination GetDestination() const override {
    return network::mojom::RequestDestination::kScript;
  }

  Animator* CreateInstance(
      const String& name,
      WorkletAnimationOptions options,
      scoped_refptr<SerializedScriptValue> serialized_state,
      const Vector<std::optional<base::TimeDelta>>& local_times,
      const Vector<Timing>& timings,
      const Vector<Timing::NormalizedTiming>& normalized_timings);
  Animator* CreateAnimatorFor(
      int animation_id,
      const String& name,
      WorkletAnimationOptions options,
      scoped_refptr<SerializedScriptValue> serialized_state,
      const Vector<std::optional<base::TimeDelta>>& local_times,
      const Vector<Timing>& timings,
      const Vector<Timing::NormalizedTiming>& normalized_timings);
  typedef HeapHashMap<String, Member<AnimatorDefinition>> DefinitionMap;
  DefinitionMap animator_definitions_;

  typedef HeapHashMap<int, Member<Animator>> AnimatorMap;
  AnimatorMap animators_;

  bool registered_ = false;

  // Default initialized to generate a distinct token for this worklet.
  const AnimationWorkletToken token_;
};

template <>
struct DowncastTraits<AnimationWorkletGlobalScope> {
  static bool AllowFrom(const ExecutionContext& context) {
    return context.IsAnimationWorkletGlobalScope();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ANIMATIONWORKLET_ANIMATION_WORKLET_GLOBAL_SCOPE_H_
