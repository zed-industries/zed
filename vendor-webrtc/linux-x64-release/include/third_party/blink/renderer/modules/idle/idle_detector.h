// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_IDLE_IDLE_DETECTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_IDLE_IDLE_DETECTOR_H_

#include "base/memory/scoped_refptr.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/public/mojom/idle/idle_manager.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_idle_options.h"
#include "third_party/blink/renderer/core/dom/abort_signal.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/event_modules.h"
#include "third_party/blink/renderer/modules/event_target_modules.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/timer.h"

namespace blink {

class ExceptionState;
class V8PermissionState;
class V8ScreenIdleState;
class V8UserIdleState;

class MODULES_EXPORT IdleDetector final
    : public EventTarget,
      public ActiveScriptWrappable<IdleDetector>,
      public ExecutionContextLifecycleObserver,
      public mojom::blink::IdleMonitor {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static IdleDetector* Create(ScriptState*);

  explicit IdleDetector(ExecutionContext*);
  ~IdleDetector() override;

  IdleDetector(const IdleDetector&) = delete;
  IdleDetector& operator=(const IdleDetector&) = delete;

  // EventTarget implementation.
  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;

  // ExecutionContextLifecycleObserver implementation.
  void ContextDestroyed() override;

  // ActiveScriptWrappable implementation.
  bool HasPendingActivity() const final;

  // IdleDetector IDL interface.
  std::optional<V8UserIdleState> userState() const;
  std::optional<V8ScreenIdleState> screenState() const;
  static ScriptPromise<V8PermissionState> requestPermission(ScriptState*,
                                                            ExceptionState&);
  ScriptPromise<IDLUndefined> start(ScriptState*,
                                    const IdleOptions*,
                                    ExceptionState&);
  DEFINE_ATTRIBUTE_EVENT_LISTENER(change, kChange)

  void Trace(Visitor*) const override;

  void SetTaskRunnerForTesting(
      scoped_refptr<base::SingleThreadTaskRunner> task_runner,
      const base::TickClock* tick_clock);

 private:
  class StartAbortAlgorithm;

  // mojom::blink::IdleMonitor implementation. Invoked on a state change, and
  // causes an event to be dispatched.
  void Update(mojom::blink::IdleStatePtr state,
              bool is_overridden_by_devtools) override;

  void DispatchUserIdleEvent(TimerBase*);
  void Abort();
  void OnMonitorDisconnected();
  void OnAddMonitor(ScriptPromiseResolver<IDLUndefined>*,
                    mojom::blink::IdleManagerError,
                    mojom::blink::IdleStatePtr);
  void Clear();

  // State currently visible to script.
  bool has_state_ = false;
  bool screen_locked_ = false;
  bool user_idle_ = false;

  // Task runner for change events. Overridden for testing.
  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;

  // If script has specified a threshold longer than the default this timer is
  // used to delay the update until the user has been idle for the specified
  // threshold.
  HeapTaskRunnerTimer<IdleDetector> timer_;

  base::TimeDelta threshold_ = base::Seconds(60);
  Member<AbortSignal> signal_;
  // The handle is valid from the time start() is called until the detector is
  // stopped, if an AbortSignal is passed to start().
  Member<AbortSignal::AlgorithmHandle> abort_handle_;
  Member<ScriptPromiseResolver<IDLUndefined>> resolver_;

  // Holds a pipe which the service uses to notify this object
  // when the idle state has changed.
  HeapMojoReceiver<mojom::blink::IdleMonitor, IdleDetector> receiver_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_IDLE_IDLE_DETECTOR_H_
