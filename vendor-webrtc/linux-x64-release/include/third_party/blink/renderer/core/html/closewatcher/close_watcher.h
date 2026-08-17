// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CLOSEWATCHER_CLOSE_WATCHER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CLOSEWATCHER_CLOSE_WATCHER_H_

#include "third_party/blink/public/mojom/close_watcher/close_listener.mojom-blink.h"
#include "third_party/blink/renderer/core/dom/abort_signal.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/dom/events/native_event_listener.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_linked_hash_set.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"

namespace blink {

class CloseWatcherOptions;
class LocalDOMWindow;
class KeyboardEvent;

class CloseWatcher final : public EventTarget, public ExecutionContextClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static CloseWatcher* Create(ScriptState*,
                              CloseWatcherOptions*,
                              ExceptionState&);

  static CloseWatcher* Create(LocalDOMWindow&);

  // If multiple close watchers are active in a given window, they form a stack
  // of groups of close watchers. Groups close together in response to a single
  // close request, and new close watchers are either added to the topmost group
  // or to a new group depending on user activation state.
  class WatcherStack final : public GarbageCollected<WatcherStack>,
                             public mojom::blink::CloseListener {
   public:
    explicit WatcherStack(LocalDOMWindow*);

    void Add(CloseWatcher*);
    void Remove(CloseWatcher*);

    void SetHadUserInteraction(bool);
    bool CancelEventCanBeCancelable() const;

    void Trace(Visitor*) const;

    void EscapeKeyHandler(KeyboardEvent*);

    bool AnyEnabledWatchers();
    void MaybeCloseReceiver();
    void BindNewPipe();

   private:
    // mojom::blink::CloseListener override:
    void Signal() final;

    HeapVector<HeapVector<Member<CloseWatcher>>> watcher_groups_;
    bool next_user_interaction_creates_a_new_allowed_group_ = true;
    wtf_size_t allowed_groups_ = 1;

    // Holds a pipe which the service uses to notify this object
    // when the idle state has changed.
    HeapMojoReceiver<mojom::blink::CloseListener, WatcherStack> receiver_;
    Member<LocalDOMWindow> window_;
  };

  CloseWatcher(LocalDOMWindow&, WatcherStack& stack);

  void Trace(Visitor*) const override;

  bool IsClosed() const { return state_ == State::kClosed; }

  void setEnabled(bool enabled);

  void requestCloseForBinding();

  enum class AllowCancel {
    kAlways,
    kWithUserActivation,
  };
  bool RequestClose(AllowCancel allow_cancel);

  void close();
  void destroy();

  DEFINE_ATTRIBUTE_EVENT_LISTENER(cancel, kCancel)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(close, kClose)

  // EventTarget overrides:
  const AtomicString& InterfaceName() const final;
  ExecutionContext* GetExecutionContext() const final {
    return ExecutionContextClient::GetExecutionContext();
  }

 private:
  static CloseWatcher* CreateInternal(LocalDOMWindow&,
                                      WatcherStack&,
                                      CloseWatcherOptions*);

  enum class State { kActive, kClosed };
  State state_ = State::kActive;
  bool dispatching_cancel_ = false;
  bool enabled_ = true;
  Member<AbortSignal::AlgorithmHandle> abort_handle_;
  Member<WatcherStack> stack_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CLOSEWATCHER_CLOSE_WATCHER_H_
