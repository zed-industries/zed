// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_ANIMATION_WORKLET_MUTATOR_DISPATCHER_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_ANIMATION_WORKLET_MUTATOR_DISPATCHER_IMPL_H_

#include <memory>
#include <utility>

#include "base/memory/raw_ptr.h"
#include "base/memory/weak_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "base/time/tick_clock.h"
#include "third_party/blink/renderer/platform/graphics/animation_worklet_mutator.h"
#include "third_party/blink/renderer/platform/graphics/animation_worklet_mutator_dispatcher.h"
#include "third_party/blink/renderer/platform/graphics/mutator_client.h"
#include "third_party/blink/renderer/platform/heap/cross_thread_persistent.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"

namespace blink {

class CompositorMutatorClient;
class MainThreadMutatorClient;

// Fans out requests to all of the registered AnimationWorkletMutators which can
// then run worklet animations to produce mutation updates. Requests for
// animation frames are received from AnimationWorkletMutators and generate a
// new frame.
class PLATFORM_EXPORT AnimationWorkletMutatorDispatcherImpl final
    : public AnimationWorkletMutatorDispatcher {
 public:
  // There are two outputs for the two interface surfaces of the created
  // class blob. The returned owning pointer to the Client, which
  // also owns the rest of the structure. |mutatee| form a
  // pair for referencing the AnimationWorkletMutatorDispatcherImpl. i.e. Put
  // tasks on the TaskRunner using the WeakPtr to get to the methods.
  static std::unique_ptr<CompositorMutatorClient> CreateCompositorThreadClient(
      base::WeakPtr<AnimationWorkletMutatorDispatcherImpl>& mutatee,
      scoped_refptr<base::SingleThreadTaskRunner> mutatee_runner);
  static std::unique_ptr<MainThreadMutatorClient> CreateMainThreadClient(
      base::WeakPtr<AnimationWorkletMutatorDispatcherImpl>& mutatee,
      scoped_refptr<base::SingleThreadTaskRunner> mutatee_runner);

  explicit AnimationWorkletMutatorDispatcherImpl(
      scoped_refptr<base::SingleThreadTaskRunner> task_runner);
  AnimationWorkletMutatorDispatcherImpl(
      const AnimationWorkletMutatorDispatcherImpl&) = delete;
  AnimationWorkletMutatorDispatcherImpl& operator=(
      const AnimationWorkletMutatorDispatcherImpl&) = delete;
  ~AnimationWorkletMutatorDispatcherImpl() override;

  // AnimationWorkletMutatorDispatcher implementation.
  void MutateSynchronously(
      std::unique_ptr<AnimationWorkletDispatcherInput>) override;

  bool MutateAsynchronously(std::unique_ptr<AnimationWorkletDispatcherInput>,
                            MutateQueuingStrategy,
                            AsyncMutationCompleteCallback) override;

  // TODO(majidvp): Remove when timeline inputs are known.
  bool HasMutators() override;

  // Interface for use by the AnimationWorklet Thread(s) to request calls.
  // (To the given Mutator on the given TaskRunner.)
  void RegisterAnimationWorkletMutator(
      CrossThreadPersistent<AnimationWorkletMutator>,
      scoped_refptr<base::SingleThreadTaskRunner> mutator_runner);

  void UnregisterAnimationWorkletMutator(
      CrossThreadPersistent<AnimationWorkletMutator>);

  void SetClient(MutatorClient* client) { client_ = client; }

  void SynchronizeAnimatorName(const String& animator_name);

  MutatorClient* client() { return client_; }

  base::WeakPtr<AnimationWorkletMutatorDispatcherImpl> GetWeakPtr() {
    return weak_factory_.GetWeakPtr();
  }

  void SetClockForTesting(std::unique_ptr<base::TickClock> tick_clock) {
    tick_clock_ = std::move(tick_clock);
  }

 private:
  class OutputVectorRef;
  struct AsyncMutationRequest;

  using InputMap = HashMap<int, std::unique_ptr<AnimationWorkletInput>>;

  using AnimationWorkletMutatorToTaskRunnerMap =
      HashMap<CrossThreadPersistent<AnimationWorkletMutator>,
              scoped_refptr<base::SingleThreadTaskRunner>>;

  InputMap CreateInputMap(AnimationWorkletDispatcherInput& mutator_input) const;

  // Dispatches mutation update requests. The callback is triggered once all
  // mutation updates have been computed and it runs on the animation worklet
  // thread associated with the last mutation to complete.
  void RequestMutations(CrossThreadOnceClosure done_callback);

  // Dispatches mutation update requests. The request time includes time
  // in the queue. In the event that a queued request is replaced, the
  // replacement uses the original request time.
  // |done_callback| is called on the impl thread on completion of the mutation
  // cycle.
  void MutateAsynchronouslyInternal(
      base::TimeTicks request_time,
      AsyncMutationCompleteCallback done_callback);

  // Called when the asynchronous mutation cycle is complete. The mutation id
  // is used for asynchronous task monitoring and request time is used for
  // collecting UMA stats of the total time between mutation request and
  // completion.
  void AsyncMutationsDone(int async_mutation_id, base::TimeTicks request_time);

  // Returns true if any updates were applied.
  bool ApplyMutationsOnHostThread();

  // Timing function used for UMA metrics. Uses a tick clock that may be
  // overridden for testing purposes.
  base::TimeTicks NowTicks() const;

  // The AnimationWorkletProxyClients are also owned by the WorkerClients
  // dictionary.
  AnimationWorkletMutatorToTaskRunnerMap mutator_map_;

  // |weak_interface| argument will be modified (initialized) by this function.
  template <typename ClientType>
  static std::unique_ptr<ClientType> CreateClient(
      base::WeakPtr<AnimationWorkletMutatorDispatcherImpl>& weak_interface,
      scoped_refptr<base::SingleThreadTaskRunner> queue);

  scoped_refptr<base::SingleThreadTaskRunner> host_queue_;

  // The MutatorClient owns (std::unique_ptr) us, so this pointer is
  // valid as long as this class exists.
  raw_ptr<MutatorClient> client_;

  // Map of mutator scope IDs to mutator input. The Mutate methods safeguards
  // against concurrent calls (important once async mutations are introduced) by
  // checking that the map has been reset on entry. For this reason, it is
  // important to reset the map at the end of the mutation cycle.
  InputMap mutator_input_map_;

  // Reference to a vector for collecting mutation output. The vector is
  // accessed across threads, thus it must be guaranteed to persist until the
  // last mutation update is complete, and updates must be done in a thread-safe
  // manner. The Mutate method guards against concurrent calls (important once
  // async mutations are introduced)  by checking that the output vector is
  // empty. For this reason, it is important to clear the output at the end of
  // the mutation cycle.
  scoped_refptr<OutputVectorRef> outputs_;

  // Active callback for the completion of an async mutation cycle.
  AsyncMutationCompleteCallback on_async_mutation_complete_;

  // Queues for pending mutation requests. Each queue can hold a single request.
  // On completion of a mutation cycle, a fresh mutation cycle is started if
  // either queue contains a request. The priority queue takes precedence if
  // both queues contain a request.  The request stored in the replaceable queue
  // can be updated in a later async mutation call, whereas the priority queue
  // entry cannot, as each priority request is required to run.
  std::unique_ptr<AsyncMutationRequest> queued_priority_request;
  std::unique_ptr<AsyncMutationRequest> queued_replaceable_request;

  std::unique_ptr<base::TickClock> tick_clock_;

  base::WeakPtrFactory<AnimationWorkletMutatorDispatcherImpl> weak_factory_{
      this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_ANIMATION_WORKLET_MUTATOR_DISPATCHER_IMPL_H_
