// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MOJO_HEAP_MOJO_UNIQUE_RECEIVER_SET_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MOJO_HEAP_MOJO_UNIQUE_RECEIVER_SET_H_

#include "base/task/sequenced_task_runner.h"
#include "mojo/public/cpp/bindings/receiver.h"
#include "mojo/public/cpp/bindings/unique_receiver_set.h"
#include "third_party/blink/renderer/platform/context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/mojo/mojo_binding_context.h"

namespace blink {

// HeapMojoUniqueReceiverSet is a wrapper for mojo::UniqueReceiverSet to be
// owned by a garbage-collected object. Blink is expected to use
// HeapMojoUniqueReceiverSet by default. HeapMojoUniqueReceiverSet must be
// associated with context. HeapMojoUniqueReceiverSet's constructor takes
// context as a mandatory parameter. HeapMojoUniqueReceiverSet resets the mojo
// connection when the associated ExecutionContext is detached.
template <typename Interface,
          typename Deleter = std::default_delete<Interface>,
          HeapMojoWrapperMode Mode = HeapMojoWrapperMode::kWithContextObserver>
class HeapMojoUniqueReceiverSet {
  DISALLOW_NEW();

 public:
  using ImplPointerType = typename mojo::Receiver<
      Interface,
      mojo::UniquePtrImplRefTraits<Interface, Deleter>>::ImplPointerType;

  explicit HeapMojoUniqueReceiverSet(ContextLifecycleNotifier* context)
      : wrapper_(MakeGarbageCollected<Wrapper>(context)) {}
  HeapMojoUniqueReceiverSet(const HeapMojoUniqueReceiverSet&) = delete;
  HeapMojoUniqueReceiverSet& operator=(const HeapMojoUniqueReceiverSet&) =
      delete;

  // Methods to redirect to mojo::ReceiverSet:
  mojo::ReceiverId Add(ImplPointerType impl,
                       mojo::PendingReceiver<Interface> receiver,
                       scoped_refptr<base::SequencedTaskRunner> task_runner) {
    return wrapper_->receiver_set().Add(std::move(impl), std::move(receiver),
                                        task_runner);
  }

  bool Remove(mojo::ReceiverId id) {
    return wrapper_->receiver_set().Remove(id);
  }

  void Clear() { wrapper_->receiver_set().Clear(); }

  bool HasReceiver(mojo::ReceiverId id) {
    return wrapper_->receiver_set().HasReceiver(id);
  }

  void Trace(Visitor* visitor) const { visitor->Trace(wrapper_); }

 private:
  // Garbage collected wrapper class to add ContextLifecycleObserver.
  class Wrapper final : public GarbageCollected<Wrapper>,
                        public ContextLifecycleObserver {
   public:
    explicit Wrapper(ContextLifecycleNotifier* notifier) {
      SetContextLifecycleNotifier(notifier);
    }

    void Trace(Visitor* visitor) const override {
      ContextLifecycleObserver::Trace(visitor);
    }

    mojo::UniqueReceiverSet<Interface, void, Deleter>& receiver_set() {
      return receiver_set_;
    }

    // ContextLifecycleObserver methods
    void ContextDestroyed() override {
      if (Mode == HeapMojoWrapperMode::kWithContextObserver)
        receiver_set_.Clear();
    }

   private:
    mojo::UniqueReceiverSet<Interface, void, Deleter> receiver_set_;
  };

  Member<Wrapper> wrapper_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MOJO_HEAP_MOJO_UNIQUE_RECEIVER_SET_H_
