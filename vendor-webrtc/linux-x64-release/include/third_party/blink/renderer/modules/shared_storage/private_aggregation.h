// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SHARED_STORAGE_PRIVATE_AGGREGATION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SHARED_STORAGE_PRIVATE_AGGREGATION_H_

#include <stdint.h>

#include "mojo/public/cpp/bindings/pending_remote.h"
#include "third_party/blink/public/mojom/aggregation_service/aggregatable_report.mojom-blink.h"
#include "third_party/blink/public/mojom/private_aggregation/private_aggregation_host.mojom-blink.h"
#include "third_party/blink/public/mojom/shared_storage/shared_storage_worklet_service.mojom-blink-forward.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/shared_storage/util.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/context_lifecycle_notifier.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"

namespace blink {

class ExceptionState;
class PrivateAggregationDebugModeOptions;
class PrivateAggregationHistogramContribution;
class ScriptState;
class SharedStorageWorkletGlobalScope;

class MODULES_EXPORT PrivateAggregation final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  struct OperationState : public GarbageCollected<OperationState> {
    explicit OperationState(ContextLifecycleNotifier* notifier,
                            size_t filtering_id_max_bytes)
        : filtering_id_max_bytes(filtering_id_max_bytes),
          private_aggregation_host(notifier) {
      CHECK_LE(filtering_id_max_bytes, kMaximumFilteringIdMaxBytes);
    }

    bool enable_debug_mode_called = false;
    size_t filtering_id_max_bytes;

    // No need to be associated as message ordering (relative to shared storage
    // operations) is unimportant.
    HeapMojoRemote<mojom::blink::PrivateAggregationHost>
        private_aggregation_host;

    // Contributions that should be forwarded if and only if an uncaught
    // exception occurs.
    Vector<mojom::blink::AggregatableReportHistogramContributionPtr>
        contributions_conditional_on_uncaught_error;

    void Trace(Visitor* visitor) const {
      visitor->Trace(private_aggregation_host);
    }
  };

  // Indicates whether the operation was terminated due to an uncaught error or
  // not.
  enum class TerminationStatus {
    kNoUncaughtError,
    kUncaughtError,
  };

  explicit PrivateAggregation(SharedStorageWorkletGlobalScope* global_scope);

  ~PrivateAggregation() override;

  void Trace(Visitor*) const override;

  // PrivateAggregation IDL
  void contributeToHistogram(ScriptState*,
                             const PrivateAggregationHistogramContribution*,
                             ExceptionState&);
  void contributeToHistogramOnEvent(
      ScriptState*,
      const String&,
      const PrivateAggregationHistogramContribution*,
      ExceptionState&);
  void enableDebugMode(ScriptState*, ExceptionState&);
  void enableDebugMode(ScriptState*,
                       const PrivateAggregationDebugModeOptions*,
                       ExceptionState&);

  void OnOperationStarted(
      int64_t operation_id,
      mojom::blink::PrivateAggregationOperationDetailsPtr pa_operation_details);
  void OnOperationFinished(int64_t operation_id,
                           TerminationStatus termination_status);

  void OnWorkletDestroyed();

 private:
  // Returns the parsed contribution. In the case of an exception, throws the
  // exception using `exception_state` and returns `nullpr`.
  mojom::blink::AggregatableReportHistogramContributionPtr ParseContribution(
      const PrivateAggregationHistogramContribution* contribution,
      ExceptionState& exception_state);

  OperationState& GetCurrentOperationState();

  void EnsureGeneralUseCountersAreRecorded();
  void EnsureEnableDebugModeUseCounterIsRecorded();
  void EnsureFilteringIdUseCounterIsRecorded();
  void EnsureErrorReportingUseCounterIsRecorded();

  bool has_recorded_general_use_counters_ = false;
  bool has_recorded_enable_debug_mode_use_counter_ = false;
  bool has_recorded_filtering_id_use_counter_ = false;
  bool has_recorded_error_reporting_use_counter_ = false;

  Member<SharedStorageWorkletGlobalScope> global_scope_;
  HeapHashMap<int64_t,
              Member<OperationState>,
              IntWithZeroKeyHashTraits<int64_t>>
      operation_states_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SHARED_STORAGE_PRIVATE_AGGREGATION_H_
