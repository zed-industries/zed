// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_COMPUTE_PRESSURE_PRESSURE_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_COMPUTE_PRESSURE_PRESSURE_OBSERVER_H_

#include <array>

#include "services/device/public/mojom/pressure_manager.mojom-blink.h"
#include "services/device/public/mojom/pressure_update.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_pressure_source.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_pressure_state.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_pressure_update_callback.h"
#include "third_party/blink/renderer/core/dom/dom_high_res_time_stamp.h"
#include "third_party/blink/renderer/modules/compute_pressure/change_rate_monitor.h"
#include "third_party/blink/renderer/platform/bindings/exception_code.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/scheduler/public/post_cancellable_task.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"

namespace blink {

namespace {

// https://w3c.github.io/compute-pressure/#dfn-max-queued-records
constexpr wtf_size_t kMaxQueuedRecords = 10;

}  // namespace

class ExecutionContext;
class ExceptionState;
class PressureObserverManager;
class PressureObserverOptions;
class PressureRecord;
class ScriptState;

class MODULES_EXPORT PressureObserver final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit PressureObserver(V8PressureUpdateCallback*);
  ~PressureObserver() override;

  static PressureObserver* Create(V8PressureUpdateCallback*);

  // PressureObserver IDL implementation.
  ScriptPromise<IDLUndefined> observe(ScriptState*,
                                      V8PressureSource,
                                      PressureObserverOptions*,
                                      ExceptionState&);
  void unobserve(V8PressureSource);
  void disconnect();
  HeapVector<Member<PressureRecord>> takeRecords();
  static Vector<V8PressureSource> knownSources();

  PressureObserver(const PressureObserver&) = delete;
  PressureObserver operator=(const PressureObserver&) = delete;

  // GarbageCollected implementation.
  void Trace(blink::Visitor*) const override;

  // Called by PressureClientImpl.
  void OnUpdate(ExecutionContext*,
                V8PressureSource::Enum,
                V8PressureState::Enum,
                DOMHighResTimeStamp);

  // Called by PressureObserverManager.
  void OnBindingSucceeded(V8PressureSource::Enum);
  void OnBindingFailed(V8PressureSource::Enum, DOMExceptionCode);
  void OnConnectionError();

  ChangeRateMonitor& change_rate_monitor_for_testing() {
    return change_rate_monitor_;
  }

 private:
  // Verifies if the latest update was at least longer than the sample period.
  bool PassesRateTest(V8PressureSource::Enum, const DOMHighResTimeStamp&) const;

  // Verifies if there is data change in between last update and new one.
  bool ShouldDispatch(V8PressureSource::Enum, V8PressureState::Enum) const;

  // Verifies if there is data changes in a defined time span is not too high.
  bool PassesRateObfuscation(V8PressureSource::Enum) const;

  // Queues valid `PressureRecord` to be reported after penalty.
  void QueueAfterPenaltyRecord(ExecutionContext*, V8PressureSource::Enum);

  // Queues valid `PressureRecord` to be reported.
  void QueuePressureRecord(ExecutionContext*,
                           V8PressureSource::Enum,
                           PressureRecord*);

  // Resolve/reject pending resolvers.
  void ResolvePendingResolvers(V8PressureSource::Enum);
  void RejectPendingResolvers(V8PressureSource::Enum,
                              DOMExceptionCode,
                              const String&);

  // Scheduled method to invoke callback.
  void ReportToCallback(ExecutionContext*);

  // Manages registered observer list for each source.
  WeakMember<PressureObserverManager> manager_;

  // The callback that receives pressure state updates.
  Member<V8PressureUpdateCallback> observer_callback_;

  // Requested sample interval from the user.
  // https://w3c.github.io/compute-pressure/#dfn-sampleinterval
  uint32_t sample_interval_ = 0;

  std::array<HeapHashSet<Member<ScriptPromiseResolver<IDLUndefined>>>,
             V8PressureSource::kEnumSize>
      pending_resolvers_;

  // Manages rate obfuscation mitigation parameters.
  ChangeRateMonitor change_rate_monitor_;

  // Last received valid record from PressureClientImpl.
  // Stored to avoid sending updates whenever the new record is the same.
  std::array<Member<PressureRecord>, V8PressureSource::kEnumSize>
      last_record_map_;

  // Last received valid record from PressureClientImpl during
  // the penalty duration, to restore when the penalty duration is over.
  std::array<Member<PressureRecord>, V8PressureSource::kEnumSize>
      after_penalty_records_;

  // Last received records from the platform collector.
  // The records are only collected when there is a change in the status.
  HeapVector<Member<PressureRecord>, kMaxQueuedRecords> records_;

  // Task handle to check if the posted task is still pending.
  TaskHandle pending_report_to_callback_;

  // Task handle array to check if the posted task is still pending.
  std::array<TaskHandle, V8PressureSource::kEnumSize>
      pending_delayed_report_to_callback_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_COMPUTE_PRESSURE_PRESSURE_OBSERVER_H_
