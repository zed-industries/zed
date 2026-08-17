// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MOJO_HEAP_MOJO_REMOTE_SET_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MOJO_HEAP_MOJO_REMOTE_SET_H_

#include <utility>

#include "base/gtest_prod_util.h"
#include "base/memory/scoped_refptr.h"
#include "base/task/sequenced_task_runner.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "mojo/public/cpp/bindings/remote_set.h"
#include "third_party/blink/renderer/platform/context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/mojo/mojo_binding_context.h"

namespace blink {

// HeapMojoRemoteSet is a wrapper for mojo::RemoteSet to be owned by a
// garbage-collected object. Blink is expected to use HeapMojoRemoteSet by
// default. HeapMojoRemoteSet must be associated with context.
// HeapMojoRemoteSet's constructor takes context as a mandatory parameter.
// HeapMojoRemoteSet resets the mojo connection when the associated
// ExecutionContext is detached.

// TODO(crbug.com/1058076) HeapMojoWrapperMode should be removed once we ensure
// that the interface is not used after ContextDestroyed().
template <typename Interface,
          HeapMojoWrapperMode Mode = HeapMojoWrapperMode::kWithContextObserver>
class HeapMojoRemoteSet {
  DISALLOW_NEW();

 public:
  using DisconnectHandler =
      typename mojo::RemoteSet<Interface>::DisconnectHandler;
  using Iterator = typename mojo::RemoteSet<Interface>::Iterator;

  explicit HeapMojoRemoteSet(ContextLifecycleNotifier* notifier)
      : wrapper_(MakeGarbageCollected<Wrapper>(notifier)) {}
  HeapMojoRemoteSet(const HeapMojoRemoteSet&) = delete;
  HeapMojoRemoteSet& operator=(const HeapMojoRemoteSet&) = delete;
  HeapMojoRemoteSet(HeapMojoRemoteSet&&) = default;
  HeapMojoRemoteSet& operator=(HeapMojoRemoteSet&&) = default;

  // Methods to redirect to mojo::RemoteSet:
  void set_disconnect_handler(DisconnectHandler handler) {
    wrapper_->remote_set().set_disconnect_handler(std::move(handler));
  }

  mojo::RemoteSetElementId Add(mojo::Remote<Interface> remote) {
    return wrapper_->remote_set().Add(std::move(remote));
  }

  mojo::RemoteSetElementId Add(
      mojo::PendingRemote<Interface> remote,
      scoped_refptr<base::SequencedTaskRunner> task_runner) {
    DCHECK(task_runner);
    return wrapper_->remote_set().Add(std::move(remote), task_runner);
  }

  void Remove(mojo::RemoteSetElementId id) {
    wrapper_->remote_set().Remove(id);
  }

  bool Contains(mojo::RemoteSetElementId id) {
    return wrapper_->remote_set().Contains(id);
  }

  void Clear() { wrapper_->remote_set().Clear(); }

  bool empty() const { return wrapper_->remote_set().empty(); }
  size_t size() const { return wrapper_->remote_set().size(); }

  Iterator begin() { return wrapper_->remote_set().begin(); }
  Iterator begin() const { return wrapper_->remote_set().begin(); }
  Iterator end() { return wrapper_->remote_set().end(); }
  Iterator end() const { return wrapper_->remote_set().end(); }

  void Trace(Visitor* visitor) const { visitor->Trace(wrapper_); }

 private:
  FRIEND_TEST_ALL_PREFIXES(HeapMojoRemoteSetGCWithContextObserverTest,
                           NoClearOnConservativeGC);

  // Garbage collected wrapper class to add ContextLifecycleObserver.
  class Wrapper final : public GarbageCollected<Wrapper>,
                        public ContextLifecycleObserver {
   public:
    explicit Wrapper(ContextLifecycleNotifier* notifier) {
      SetContextLifecycleNotifier(notifier);
    }
    Wrapper(const Wrapper&) = delete;
    Wrapper& operator=(const Wrapper&) = delete;
    Wrapper(Wrapper&&) = default;
    Wrapper& operator=(Wrapper&&) = default;

    void Trace(Visitor* visitor) const override {
      ContextLifecycleObserver::Trace(visitor);
    }

    mojo::RemoteSet<Interface>& remote_set() { return remote_set_; }

    // ContextLifecycleObserver methods
    void ContextDestroyed() override {
      if (Mode == HeapMojoWrapperMode::kWithContextObserver)
        remote_set_.Clear();
    }

   private:
    mojo::RemoteSet<Interface> remote_set_;
  };

  Member<Wrapper> wrapper_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MOJO_HEAP_MOJO_REMOTE_SET_H_
