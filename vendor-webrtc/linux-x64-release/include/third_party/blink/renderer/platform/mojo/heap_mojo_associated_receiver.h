// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MOJO_HEAP_MOJO_ASSOCIATED_RECEIVER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MOJO_HEAP_MOJO_ASSOCIATED_RECEIVER_H_

#include <utility>

#include "base/gtest_prod_util.h"
#include "base/task/sequenced_task_runner.h"
#include "mojo/public/cpp/bindings/associated_receiver.h"
#include "third_party/blink/renderer/platform/context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/mojo/mojo_binding_context.h"

namespace blink {

// HeapMojoAssociatedReceiver is a wrapper for mojo::AssociatedReceiver to be
// owned by a garbage-collected object. Blink is expected to use
// HeapMojoAssociatedReceiver by default. HeapMojoAssociatedReceiver must be
// associated with context. HeapMojoAssociatedReceiver's constructor takes
// context as a mandatory parameter. HeapMojoAssociatedReceiver resets the mojo
// connection when 1) the owner object is garbage-collected and 2) the
// associated ExecutionContext is detached.

// TODO(crbug.com/1058076) HeapMojoWrapperMode should be removed once we ensure
// that the interface is not used after ContextDestroyed().
template <typename Interface,
          typename Owner,
          HeapMojoWrapperMode Mode = HeapMojoWrapperMode::kWithContextObserver>
class HeapMojoAssociatedReceiver {
  DISALLOW_NEW();

 public:
  HeapMojoAssociatedReceiver(Owner* owner, ContextLifecycleNotifier* context)
      : wrapper_(MakeGarbageCollected<Wrapper>(owner, context)) {
    static_assert(std::is_base_of<Interface, Owner>::value,
                  "Owner should implement Interface");
    static_assert(IsGarbageCollectedType<Owner>::value,
                  "Owner needs to be a garbage collected object");
  }
  HeapMojoAssociatedReceiver(const HeapMojoAssociatedReceiver&) = delete;
  HeapMojoAssociatedReceiver& operator=(const HeapMojoAssociatedReceiver&) =
      delete;

  // Methods to redirect to mojo::AssociatedReceiver:
  bool is_bound() const { return wrapper_->associated_receiver().is_bound(); }
  void reset() { wrapper_->associated_receiver().reset(); }
  void set_disconnect_handler(base::OnceClosure handler) {
    wrapper_->associated_receiver().set_disconnect_handler(std::move(handler));
  }
  [[nodiscard]] mojo::PendingAssociatedRemote<Interface>
  BindNewEndpointAndPassRemote(
      scoped_refptr<base::SequencedTaskRunner> task_runner) {
    DCHECK(task_runner);
    return wrapper_->associated_receiver().BindNewEndpointAndPassRemote(
        std::move(task_runner));
  }
  void Bind(
      mojo::PendingAssociatedReceiver<Interface> pending_associated_receiver,
      scoped_refptr<base::SequencedTaskRunner> task_runner) {
    DCHECK(task_runner);
    wrapper_->associated_receiver().Bind(std::move(pending_associated_receiver),
                                         std::move(task_runner));
  }
  bool WaitForIncomingCall() {
    return wrapper_->associated_receiver().WaitForIncomingCall();
  }

  void SetFilter(std::unique_ptr<mojo::MessageFilter> filter) {
    wrapper_->associated_receiver().SetFilter(std::move(filter));
  }

  void Trace(Visitor* visitor) const { visitor->Trace(wrapper_); }

 private:
  FRIEND_TEST_ALL_PREFIXES(HeapMojoAssociatedReceiverGCWithContextObserverTest,
                           NoResetOnConservativeGC);

  // Garbage collected wrapper class to add a prefinalizer.
  class Wrapper final : public GarbageCollected<Wrapper>,
                        public ContextLifecycleObserver {
    USING_PRE_FINALIZER(Wrapper, Dispose);

   public:
    Wrapper(Owner* owner, ContextLifecycleNotifier* notifier)
        : owner_(owner), associated_receiver_(owner) {
      SetContextLifecycleNotifier(notifier);
    }

    void Trace(Visitor* visitor) const override {
      visitor->Trace(owner_);
      ContextLifecycleObserver::Trace(visitor);
    }

    void Dispose() { associated_receiver_.reset(); }

    mojo::AssociatedReceiver<Interface>& associated_receiver() {
      return associated_receiver_;
    }

    // ContextLifecycleObserver methods
    void ContextDestroyed() override {
      if (Mode == HeapMojoWrapperMode::kWithContextObserver)
        associated_receiver_.reset();
    }

   private:
    Member<Owner> owner_;
    mojo::AssociatedReceiver<Interface> associated_receiver_;
  };

  Member<Wrapper> wrapper_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MOJO_HEAP_MOJO_ASSOCIATED_RECEIVER_H_
