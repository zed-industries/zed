// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MOJO_HEAP_MOJO_RECEIVER_SET_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MOJO_HEAP_MOJO_RECEIVER_SET_H_

#include "base/gtest_prod_util.h"
#include "base/task/sequenced_task_runner.h"
#include "mojo/public/cpp/bindings/receiver.h"
#include "mojo/public/cpp/bindings/receiver_set.h"
#include "third_party/blink/renderer/platform/context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/mojo/mojo_binding_context.h"

namespace blink {

// HeapMojoReceiverSet is a wrapper for mojo::ReceiverSet to be owned by a
// garbage-collected object. Blink is expected to use HeapMojoReceiverSet by
// default. HeapMojoReceiverSet must be associated with context.
// HeapMojoReceiverSet's constructor takes context as a mandatory parameter.
// HeapMojoReceiverSet resets the mojo connection when 1) the owner object is
// garbage-collected or 2) the associated ExecutionContext is detached.

// TODO(crbug.com/1058076) HeapMojoWrapperMode should be removed once we ensure
// that the interface is not used after ContextDestroyed().
template <typename Interface,
          typename Owner,
          HeapMojoWrapperMode Mode = HeapMojoWrapperMode::kWithContextObserver,
          typename ContextType = void>
class HeapMojoReceiverSet {
  DISALLOW_NEW();

 public:
  using ContextTraits = mojo::ReceiverSetContextTraits<ContextType>;
  using Context = typename ContextTraits::Type;
  explicit HeapMojoReceiverSet(Owner* owner, ContextLifecycleNotifier* context)
      : wrapper_(MakeGarbageCollected<Wrapper>(owner, context)) {
    static_assert(std::is_base_of<Interface, Owner>::value,
                  "Owner should implement Interface");
    static_assert(IsGarbageCollectedType<Owner>::value,
                  "Owner needs to be a garbage collected object");
  }
  HeapMojoReceiverSet(const HeapMojoReceiverSet&) = delete;
  HeapMojoReceiverSet& operator=(const HeapMojoReceiverSet&) = delete;

  // Methods to redirect to mojo::ReceiverSet:
  mojo::ReceiverId Add(mojo::PendingReceiver<Interface> receiver,
                       scoped_refptr<base::SequencedTaskRunner> task_runner) {
    DCHECK(task_runner);
    return wrapper_->receiver_set().Add(wrapper_->owner(), std::move(receiver),
                                        task_runner);
  }

  mojo::ReceiverId Add(mojo::PendingReceiver<Interface> receiver,
                       Context context,
                       scoped_refptr<base::SequencedTaskRunner> task_runner) {
    DCHECK(task_runner);
    return wrapper_->receiver_set().Add(wrapper_->owner(), std::move(receiver),
                                        std::move(context), task_runner);
  }

  bool Remove(mojo::ReceiverId id) {
    return wrapper_->receiver_set().Remove(id);
  }

  void Clear() { wrapper_->receiver_set().Clear(); }
  void ClearWithReason(uint32_t custom_reason_code,
                       const std::string& description) {
    wrapper_->receiver_set().ClearWithReason(custom_reason_code, description);
  }

  bool HasReceiver(mojo::ReceiverId id) {
    return wrapper_->receiver_set().HasReceiver(id);
  }

  bool empty() const { return wrapper_->receiver_set().empty(); }
  size_t size() const { return wrapper_->receiver_set().size(); }
  const Context& current_context() const {
    return wrapper_->receiver_set().current_context();
  }

  void Trace(Visitor* visitor) const { visitor->Trace(wrapper_); }

 private:
  FRIEND_TEST_ALL_PREFIXES(HeapMojoReceiverSetGCWithContextObserverTest,
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

    void Dispose() { receiver_set_.Clear(); }

    mojo::ReceiverSet<Interface, ContextType>& receiver_set() {
      return receiver_set_;
    }
    Owner* owner() { return owner_.Get(); }

    // ContextLifecycleObserver methods
    void ContextDestroyed() override {
      if (Mode == HeapMojoWrapperMode::kWithContextObserver)
        receiver_set_.Clear();
    }

   private:
    Member<Owner> owner_;
    mojo::ReceiverSet<Interface, ContextType> receiver_set_;
  };

  Member<Wrapper> wrapper_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MOJO_HEAP_MOJO_RECEIVER_SET_H_
