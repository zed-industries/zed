// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MOJO_HEAP_MOJO_REMOTE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MOJO_HEAP_MOJO_REMOTE_H_

#include <utility>

#include "base/task/sequenced_task_runner.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "third_party/blink/renderer/platform/context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"

namespace blink {

// HeapMojoRemote is a wrapper for mojo::Remote to be owned by a
// garbage-collected object. Blink is expected to use HeapMojoRemote by
// default. HeapMojoRemote must be associated with context.
// HeapMojoRemote's constructor takes context as a mandatory parameter.
// HeapMojoRemote resets the mojo connection when the associated
// ExecutionContext is detached.

// TODO(crbug.com/1058076) HeapMojoWrapperMode should be removed once we ensure
// that the interface is not used after ContextDestroyed().
template <typename Interface,
          HeapMojoWrapperMode Mode = HeapMojoWrapperMode::kWithContextObserver>
class HeapMojoRemote {
  DISALLOW_NEW();

 public:
  explicit HeapMojoRemote(ContextLifecycleNotifier* notifier)
      : wrapper_(MakeGarbageCollected<Wrapper>(notifier)) {}
  HeapMojoRemote(const HeapMojoRemote&) = delete;
  HeapMojoRemote& operator=(const HeapMojoRemote&) = delete;
  HeapMojoRemote(HeapMojoRemote&&) = default;
  HeapMojoRemote& operator=(HeapMojoRemote&&) = default;

  // Methods to redirect to mojo::Remote.
  using Proxy = typename Interface::Proxy_;
  Proxy* operator->() const { return get(); }
  Proxy* get() const { return wrapper_->remote().get(); }
  bool is_bound() const { return wrapper_->remote().is_bound(); }
  explicit operator bool() const { return is_bound(); }
  bool is_connected() const { return wrapper_->remote().is_connected(); }
  void reset() { wrapper_->remote().reset(); }
  void ResetWithReason(uint32_t custom_reason, const std::string& description) {
    wrapper_->remote().ResetWithReason(custom_reason, description);
  }
  void set_disconnect_handler(base::OnceClosure handler) {
    wrapper_->remote().set_disconnect_handler(std::move(handler));
  }
  void set_disconnect_with_reason_handler(
      mojo::ConnectionErrorWithReasonCallback handler) {
    wrapper_->remote().set_disconnect_with_reason_handler(std::move(handler));
  }
  [[nodiscard]] mojo::PendingReceiver<Interface> BindNewPipeAndPassReceiver(
      scoped_refptr<base::SequencedTaskRunner> task_runner) {
    DCHECK(task_runner);
    return wrapper_->remote().BindNewPipeAndPassReceiver(
        std::move(task_runner));
  }
  void Bind(mojo::PendingRemote<Interface> pending_remote,
            scoped_refptr<base::SequencedTaskRunner> task_runner) {
    DCHECK(task_runner);
    wrapper_->remote().Bind(std::move(pending_remote), std::move(task_runner));
  }
  void PauseReceiverUntilFlushCompletes(mojo::PendingFlush flush) {
    wrapper_->remote().PauseReceiverUntilFlushCompletes(std::move(flush));
  }
  mojo::PendingFlush FlushAsync() { return wrapper_->remote().FlushAsync(); }
  void FlushForTesting() { return wrapper_->remote().FlushForTesting(); }

  void Trace(Visitor* visitor) const { visitor->Trace(wrapper_); }

 private:
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

    mojo::Remote<Interface>& remote() { return remote_; }

    // ContextLifecycleObserver methods
    void ContextDestroyed() override {
      if (Mode == HeapMojoWrapperMode::kWithContextObserver)
        remote_.reset();
    }

   private:
    mojo::Remote<Interface> remote_;
  };

  Member<Wrapper> wrapper_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MOJO_HEAP_MOJO_REMOTE_H_
