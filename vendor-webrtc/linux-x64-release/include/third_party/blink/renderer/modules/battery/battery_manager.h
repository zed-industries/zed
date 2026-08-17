// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BATTERY_BATTERY_MANAGER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BATTERY_BATTERY_MANAGER_H_

#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_property.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_state_observer.h"
#include "third_party/blink/renderer/core/frame/platform_event_controller.h"
#include "third_party/blink/renderer/modules/battery/battery_dispatcher.h"
#include "third_party/blink/renderer/modules/battery/battery_status.h"
#include "third_party/blink/renderer/modules/event_target_modules.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class Navigator;

class BatteryManager final : public EventTarget,
                             public ActiveScriptWrappable<BatteryManager>,
                             public Supplement<Navigator>,
                             public ExecutionContextLifecycleStateObserver,
                             public PlatformEventController {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static const char kSupplementName[];
  static ScriptPromise<BatteryManager> getBattery(ScriptState*, Navigator&);
  explicit BatteryManager(Navigator&);
  ~BatteryManager() override;

  // Returns a promise object that will be resolved with this BatteryManager.
  ScriptPromise<BatteryManager> StartRequest(ScriptState*);

  // EventTarget implementation.
  const WTF::AtomicString& InterfaceName() const override {
    return event_target_names::kBatteryManager;
  }
  ExecutionContext* GetExecutionContext() const override {
    return ExecutionContextLifecycleObserver::GetExecutionContext();
  }

  bool charging();
  double chargingTime();
  double dischargingTime();
  double level();

  DEFINE_ATTRIBUTE_EVENT_LISTENER(chargingchange, kChargingchange)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(chargingtimechange, kChargingtimechange)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(dischargingtimechange, kDischargingtimechange)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(levelchange, kLevelchange)

  // Inherited from PlatformEventController.
  void DidUpdateData() override;
  void RegisterWithDispatcher() override;
  void UnregisterWithDispatcher() override;
  bool HasLastData() override;

  // ContextLifecycleState implementation.
  void ContextLifecycleStateChanged(mojom::FrameLifecycleState) override;
  void ContextDestroyed() override;

  // ScriptWrappable implementation.
  bool HasPendingActivity() const final;

  void Trace(Visitor*) const override;

 private:
  using BatteryProperty = ScriptPromiseProperty<BatteryManager, DOMException>;
  Member<BatteryProperty> battery_property_;
  BatteryStatus battery_status_;
  Member<BatteryDispatcher> battery_dispatcher_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BATTERY_BATTERY_MANAGER_H_
