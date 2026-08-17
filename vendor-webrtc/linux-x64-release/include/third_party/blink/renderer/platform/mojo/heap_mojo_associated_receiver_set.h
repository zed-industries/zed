// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MOJO_HEAP_MOJO_ASSOCIATED_RECEIVER_SET_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MOJO_HEAP_MOJO_ASSOCIATED_RECEIVER_SET_H_

#include <utility>

#include "base/functional/callback.h"
#include "base/gtest_prod_util.h"
#include "base/task/sequenced_task_runner.h"
#include "mojo/public/cpp/bindings/associated_receiver_set.h"
#include "third_party/blink/renderer/platform/context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/mojo/mojo_binding_context.h"

namespace blink {

// HeapMojoAssociatedReceiverSet is a wrapper for mojo::AssociatedReceiverSet to
// be owned by a garbage-collected object. Blink is expected to use
// HeapMojoAssociatedReceiverSet by default. HeapMojoAssociatedReceiverSet must
// be associated with context. HeapMojoAssociatedReceiverSet's constructor takes
// context as a mandatory parameter. HeapMojoAssociatedReceiverSet resets the
// mojo connection when 1) the owner object is garbage-collected or 2) the
// associated ExecutionContext is detached.
template <typename Interface,
          typename Owner,
          HeapMojoWrapperMode Mode = HeapMojoWrapperMode::kWithContextObserver>
class HeapMojoAssociatedReceiverSet {
  DISALLOW_NEW();

 public:
  explicit HeapMojoAssociatedReceiverSet(Owner* owner,
                                         ContextLifecycleNotifier* context)
      : wrapper_(MakeGarbageCollected<Wrapper>(owner, context)) {
    static_assert(std::is_base_of<Interface, Owner>::value,
                  "Owner should implement Interface");
    static_assert(IsGarbageCollectedType<Owner>::value,
                  "Owner needs to be a garbage collected object");
  }
  HeapMojoAssociatedReceiverSet(const HeapMojoAssociatedReceiverSet&) = delete;
  HeapMojoAssociatedReceiverSet& operator=(
      const HeapMojoAssociatedReceiverSet&) = delete;

  // Methods to redirect to mojo::AssociatedReceiverSet:
  void set_disconnect_handler(base::RepeatingClosure handler) {
    wrapper_->associated_receiver_set().set_disconnect_handler(
        std::move(handler));
  }

  mojo::ReceiverId Add(
      mojo::PendingAssociatedReceiver<Interface> associated_receiver,
      scoped_refptr<base::SequencedTaskRunner> task_runner) {
    DCHECK(task_runner);
    return wrapper_->associated_receiver_set().Add(
        wrapper_->owner(), std::move(associated_receiver), task_runner);
  }

  bool Remove(mojo::ReceiverId id) {
    return wrapper_->associated_receiver_set().Remove(id);
  }

  void Clear() { wrapper_->associated_receiver_set().Clear(); }

  bool HasReceiver(mojo::ReceiverId id) const {
    return wrapper_->associated_receiver_set().HasReceiver(id);
  }

  bool empty() const { return wrapper_->associated_receiver_set().empty(); }

  void Trace(Visitor* visitor) const { visitor->Trace(wrapper_); }

 private:
  FRIEND_TEST_ALL_PREFIXES(
      HeapMojoAssociatedReceiverSetGCWithContextObserverTest,
      NoClearOnConservativeGC);

  // Garbage collected wrapper class to add a prefinalizer.
  class Wrapper final : public GarbageCollected<Wrapper>,
                        public ContextLifecycleObserver {
    USING_PRE_FINALIZER(Wrapper, Dispose);

   public:
    explicit Wrapper(Owner* owner, ContextLifecycleNotifier* notifier)
        : owner_(owner) {
      SetContextLifecycleNotifier(notifier);
    }

    void Trace(Visitor* visitor) const override {
      visitor->Trace(owner_);
      ContextLifecycleObserver::Trace(visitor);
    }

    void Dispose() { associated_receiver_set_.Clear(); }

    mojo::AssociatedReceiverSet<Interface>& associated_receiver_set() {
      return associated_receiver_set_;
    }
    Owner* owner() { return owner_.Get(); }

    // ContextLifecycleObserver methods
    void ContextDestroyed() override {
      if (Mode == HeapMojoWrapperMode::kWithContextObserver)
        associated_receiver_set_.Clear();
    }

   private:
    Member<Owner> owner_;
    mojo::AssociatedReceiverSet<Interface> associated_receiver_set_;
  };

  Member<Wrapper> wrapper_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MOJO_HEAP_MOJO_ASSOCIATED_RECEIVER_SET_H_
