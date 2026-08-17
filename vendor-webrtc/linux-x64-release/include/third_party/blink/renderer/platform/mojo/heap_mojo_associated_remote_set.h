// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MOJO_HEAP_MOJO_ASSOCIATED_REMOTE_SET_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MOJO_HEAP_MOJO_ASSOCIATED_REMOTE_SET_H_

#include <utility>

#include "base/gtest_prod_util.h"
#include "base/task/sequenced_task_runner.h"
#include "mojo/public/cpp/bindings/remote_set.h"
#include "third_party/blink/renderer/platform/context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/mojo/mojo_binding_context.h"

namespace blink {

// HeapMojoAssociatedRemoteSet is a wrapper for mojo::AssociatedRemoteSet to be
// owned by a garbage-collected object. Blink is expected to use
// HeapMojoAssociatedRemoteSet by default. HeapMojoAssociatedRemoteSet must be
// associated with an ExecutionContext. It resets the mojo connection when the
// ExecutionContext is detached.
template <typename Interface,
          HeapMojoWrapperMode Mode = HeapMojoWrapperMode::kWithContextObserver>
class HeapMojoAssociatedRemoteSet {
  DISALLOW_NEW();

 public:
  using DisconnectHandler =
      typename mojo::AssociatedRemoteSet<Interface>::DisconnectHandler;
  using Iterator = typename mojo::AssociatedRemoteSet<Interface>::Iterator;

  explicit HeapMojoAssociatedRemoteSet(ContextLifecycleNotifier* notifier)
      : wrapper_(MakeGarbageCollected<Wrapper>(notifier)) {}

  HeapMojoAssociatedRemoteSet(const HeapMojoAssociatedRemoteSet&) = delete;
  HeapMojoAssociatedRemoteSet& operator=(const HeapMojoAssociatedRemoteSet&) =
      delete;
  HeapMojoAssociatedRemoteSet(HeapMojoAssociatedRemoteSet&&) = default;
  HeapMojoAssociatedRemoteSet& operator=(HeapMojoAssociatedRemoteSet&&) =
      default;

  // Methods to redirect to mojo::AssociatedRemoteSet:
  mojo::RemoteSetElementId Add(
      mojo::PendingAssociatedRemote<Interface> associated_remote,
      scoped_refptr<base::SequencedTaskRunner> task_runner) {
    DCHECK(task_runner);
    return wrapper_->associated_remote_set().Add(std::move(associated_remote),
                                                 task_runner);
  }

  void Remove(mojo::RemoteSetElementId id) {
    wrapper_->associated_remote_set().Remove(id);
  }

  bool Contains(mojo::RemoteSetElementId id) {
    return wrapper_->associated_remote_set().Contains(id);
  }

  void set_disconnect_handler(DisconnectHandler handler) {
    wrapper_->associated_remote_set().set_disconnect_handler(
        std::move(handler));
  }

  void Clear() { wrapper_->associated_remote_set().Clear(); }

  bool empty() const { return wrapper_->associated_remote_set().empty(); }
  size_t size() const { return wrapper_->associated_remote_set().size(); }

  Iterator begin() { return wrapper_->associated_remote_set().begin(); }
  Iterator begin() const { return wrapper_->associated_remote_set().begin(); }
  Iterator end() { return wrapper_->associated_remote_set().end(); }
  Iterator end() const { return wrapper_->associated_remote_set().end(); }

  void Trace(Visitor* visitor) const { visitor->Trace(wrapper_); }

 private:
  FRIEND_TEST_ALL_PREFIXES(HeapMojoAssociatedRemoteSetGCWithContextObserverTest,
                           NoClearOnConservativeGC);

  // Garbage collected wrapper class to add a prefinalizer.
  class Wrapper final : public GarbageCollected<Wrapper>,
                        public ContextLifecycleObserver {
    USING_PRE_FINALIZER(Wrapper, Dispose);

   public:
    explicit Wrapper(ContextLifecycleNotifier* notifier) {
      SetContextLifecycleNotifier(notifier);
    }

    void Trace(Visitor* visitor) const override {
      ContextLifecycleObserver::Trace(visitor);
    }

    void Dispose() { associated_remote_set_.Clear(); }

    mojo::AssociatedRemoteSet<Interface>& associated_remote_set() {
      return associated_remote_set_;
    }

    // ContextLifecycleObserver methods
    void ContextDestroyed() override {
      if (Mode == HeapMojoWrapperMode::kWithContextObserver)
        associated_remote_set_.Clear();
    }

   private:
    mojo::AssociatedRemoteSet<Interface> associated_remote_set_;
  };

  Member<Wrapper> wrapper_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MOJO_HEAP_MOJO_ASSOCIATED_REMOTE_SET_H_
