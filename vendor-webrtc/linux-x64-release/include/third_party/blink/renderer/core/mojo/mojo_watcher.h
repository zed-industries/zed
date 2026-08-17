// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_MOJO_MOJO_WATCHER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_MOJO_MOJO_WATCHER_H_

#include "mojo/public/cpp/system/handle.h"
#include "mojo/public/cpp/system/trap.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/self_keep_alive.h"

namespace base {
class SingleThreadTaskRunner;
}  // namespace base

namespace blink {

class ExecutionContext;
class MojoHandleSignals;
class V8MojoWatchCallback;

class MojoWatcher final : public ScriptWrappable,
                          public ActiveScriptWrappable<MojoWatcher>,
                          public ExecutionContextLifecycleObserver {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static MojoWatcher* Create(mojo::Handle,
                             const MojoHandleSignals*,
                             V8MojoWatchCallback*,
                             ExecutionContext*);

  MojoWatcher(ExecutionContext*, V8MojoWatchCallback*);
  ~MojoWatcher() override;

  MojoResult cancel();

  void Trace(Visitor*) const override;

  // ActiveScriptWrappable
  bool HasPendingActivity() const final;

  // ExecutionContextLifecycleObserver
  void ContextDestroyed() final;

 private:
  friend class V8MojoWatcher;

  MojoResult Watch(mojo::Handle, const MojoHandleSignals*);
  MojoResult Arm(MojoResult* ready_result);

  static void OnHandleReady(const MojoTrapEvent*);
  void RunReadyCallback(MojoResult);

  SelfKeepAlive<MojoWatcher> keep_alive_;
  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;
  Member<V8MojoWatchCallback> callback_;
  mojo::ScopedTrapHandle trap_handle_;
  mojo::Handle handle_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_MOJO_MOJO_WATCHER_H_
