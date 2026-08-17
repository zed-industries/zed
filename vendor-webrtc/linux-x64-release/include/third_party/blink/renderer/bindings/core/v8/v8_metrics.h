// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_METRICS_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_METRICS_H_

#include "services/metrics/public/cpp/ukm_recorder.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "v8/include/v8-metrics.h"

namespace blink {

// Implements a V8 metrics recorder for gathering events generated
// within the V8 engine. This is used for some UMA and all UKM
// metrics. All event handling methods in here are run in the main
// thread, so thread-safety is not an issue and is handled by the
// V8 engine.
// For UKM events, the context is used to get the source id and the
// UKM recorder.
class CORE_EXPORT V8MetricsRecorder : public v8::metrics::Recorder {
 public:
  explicit V8MetricsRecorder(v8::Isolate* isolate) : isolate_(isolate) {}

  void AddMainThreadEvent(const v8::metrics::WasmModuleDecoded& event,
                          ContextId context_id) override;
  void AddMainThreadEvent(const v8::metrics::WasmModuleCompiled& event,
                          ContextId context_id) override;
  void AddMainThreadEvent(const v8::metrics::WasmModuleInstantiated& event,
                          ContextId context_id) override;

  void AddMainThreadEvent(const v8::metrics::GarbageCollectionFullCycle& event,
                          ContextId context_id) override;
  void AddMainThreadEvent(
      const v8::metrics::GarbageCollectionFullMainThreadIncrementalMark& event,
      ContextId context_id) override;
  void AddMainThreadEvent(
      const v8::metrics::GarbageCollectionFullMainThreadBatchedIncrementalMark&
          event,
      ContextId context_id) override;
  void AddMainThreadEvent(
      const v8::metrics::GarbageCollectionFullMainThreadIncrementalSweep& event,
      ContextId context_id) override;
  void AddMainThreadEvent(
      const v8::metrics::GarbageCollectionFullMainThreadBatchedIncrementalSweep&
          event,
      ContextId context_id) override;
  void AddMainThreadEvent(const v8::metrics::GarbageCollectionYoungCycle& event,
                          ContextId context_id) override;

  void NotifyIsolateDisposal() override;

 private:
  template <typename EventType>
  void AddMainThreadBatchedEvents(
      const v8::metrics::GarbageCollectionBatchedEvents<EventType>&
          batched_events,
      ContextId context_id) {
    for (auto event : batched_events.events) {
      AddMainThreadEvent(event, context_id);
    }
  }

  struct UkmRecorderAndSourceId {
    ukm::UkmRecorder* recorder;
    ukm::SourceId source_id;
    UkmRecorderAndSourceId(ukm::UkmRecorder* ukm_recorder,
                           ukm::SourceId ukm_source_id)
        : recorder(ukm_recorder), source_id(ukm_source_id) {}
  };

  std::optional<UkmRecorderAndSourceId> GetUkmRecorderAndSourceId(
      ContextId context_id);

  v8::Isolate* isolate_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_METRICS_H_
