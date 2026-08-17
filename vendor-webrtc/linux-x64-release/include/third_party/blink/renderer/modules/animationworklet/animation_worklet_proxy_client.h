// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ANIMATIONWORKLET_ANIMATION_WORKLET_PROXY_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ANIMATIONWORKLET_ANIMATION_WORKLET_PROXY_CLIENT_H_

#include "base/gtest_prod_util.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/core/workers/worker_clients.h"
#include "third_party/blink/renderer/modules/animationworklet/animation_worklet_global_scope.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/graphics/animation_worklet_mutator.h"

namespace blink {

class AnimationWorkletMutatorDispatcherImpl;
class Document;
class WorkletGlobalScope;

// Mediates between animation worklet global scope and its associated
// dispatchers. An AnimationWorkletProxyClient is associated with a single
// global scope and up to two dispatchers representing main and compositor
// threads.
//
// This is constructed on the main thread but it is used in the worklet backing
// thread.
class MODULES_EXPORT AnimationWorkletProxyClient
    : public GarbageCollected<AnimationWorkletProxyClient>,
      public Supplement<WorkerClients>,
      public AnimationWorkletMutator {
 public:
  static const char kSupplementName[];
  static const int8_t kNumStatelessGlobalScopes;

  // This client is hooked to the given |mutatee|, on the given
  // |mutatee_runner|.
  explicit AnimationWorkletProxyClient(
      int worklet_id,
      base::WeakPtr<AnimationWorkletMutatorDispatcherImpl> compositor_mutatee,
      scoped_refptr<base::SingleThreadTaskRunner> compositor_mutatee_runner,
      base::WeakPtr<AnimationWorkletMutatorDispatcherImpl> main_thread_mutatee,
      scoped_refptr<base::SingleThreadTaskRunner> main_thread_mutatee_runner);

  AnimationWorkletProxyClient(const AnimationWorkletProxyClient&) = delete;
  AnimationWorkletProxyClient& operator=(const AnimationWorkletProxyClient&) =
      delete;

  void Trace(Visitor*) const override;

  virtual void SynchronizeAnimatorName(const String& animator_name);
  virtual void AddGlobalScope(WorkletGlobalScope*);
  void Dispose();

  // AnimationWorkletMutator:
  // These methods are invoked on the animation worklet thread.
  int GetWorkletId() const override { return worklet_id_; }
  std::unique_ptr<AnimationWorkletOutput> Mutate(
      std::unique_ptr<AnimationWorkletInput> input) override;

  void AddGlobalScopeForTesting(WorkletGlobalScope*);

  static AnimationWorkletProxyClient* FromDocument(Document*, int worklet_id);
  static AnimationWorkletProxyClient* From(WorkerClients*);

 private:
  friend class AnimationWorkletProxyClientTest;
  FRIEND_TEST_ALL_PREFIXES(AnimationWorkletProxyClientTest,
                           AnimationWorkletProxyClientConstruction);
  FRIEND_TEST_ALL_PREFIXES(AnimationWorkletProxyClientTest,
                           RegisteredAnimatorNameShouldSyncOnce);

  // The global scope periodically switches in order to enforce stateless
  // behavior. For stateless animators, prior state is lost on each switch to
  // global scope. For stateful animators, prior state is transferred to the new
  // global scope.
  AnimationWorkletGlobalScope* SelectGlobalScopeAndUpdateAnimatorsIfNecessary();

  const int worklet_id_;

  struct MutatorItem {
    base::WeakPtr<AnimationWorkletMutatorDispatcherImpl> mutator_dispatcher;
    scoped_refptr<base::SingleThreadTaskRunner> mutator_runner;
    MutatorItem(
        base::WeakPtr<AnimationWorkletMutatorDispatcherImpl> mutator_dispatcher,
        scoped_refptr<base::SingleThreadTaskRunner> mutator_runner)
        : mutator_dispatcher(std::move(mutator_dispatcher)),
          mutator_runner(std::move(mutator_runner)) {}
  };
  WTF::Vector<MutatorItem> mutator_items_;

  Vector<CrossThreadPersistent<AnimationWorkletGlobalScope>> global_scopes_;
  HashMap<String, int8_t> registered_animators_;

  enum RunState { kUninitialized, kWorking, kDisposed } state_;

  int next_global_scope_switch_countdown_;
  wtf_size_t current_global_scope_index_;
};

void MODULES_EXPORT
ProvideAnimationWorkletProxyClientTo(WorkerClients*,
                                     AnimationWorkletProxyClient*);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ANIMATIONWORKLET_ANIMATION_WORKLET_PROXY_CLIENT_H_
