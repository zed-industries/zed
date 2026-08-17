// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CANVAS_INTERVENTIONS_CANVAS_INTERVENTIONS_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CANVAS_INTERVENTIONS_CANVAS_INTERVENTIONS_HELPER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/html/canvas/canvas_rendering_context.h"
#include "third_party/blink/renderer/platform/graphics/canvas_resource_host.h"
#include "third_party/blink/renderer/platform/graphics/static_bitmap_image.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

inline constexpr std::string_view kNoiseReasonMetricName =
    "FingerprintingProtection.CanvasNoise.InterventionReason";
inline constexpr std::string_view kNoiseDurationMetricName =
    "FingerprintingProtection.CanvasNoise.NoiseDuration";
inline constexpr std::string_view kCanvasSizeMetricName =
    "FingerprintingProtection.CanvasNoise.NoisedCanvasSize";
inline constexpr std::string_view kCanvasOperationMetricName =
    "FingerprintingProtection.CanvasNoise.OperationTriggered";

class CORE_EXPORT CanvasInterventionsHelper
    : public GarbageCollected<CanvasInterventionsHelper>,
      public Supplement<ExecutionContext>,
      public ExecutionContextLifecycleObserver {
 public:
  enum class CanvasInterventionType {
    kNone,
    kNoise,
  };

  static const char kSupplementName[];

  static CanvasInterventionsHelper* From(ExecutionContext* execution_context);

  explicit CanvasInterventionsHelper(ExecutionContext& execution_context);

  // If allowed, performs noising on a copy of the snapshot StaticBitmapImage
  // and returns the noised snapshot, otherwise it will return the original
  // inputted snapshot.
  static bool MaybeNoiseSnapshot(CanvasRenderingContext* rendering_context,
                                 ExecutionContext* execution_context,
                                 scoped_refptr<StaticBitmapImage>& snapshot);

  void Trace(Visitor* visitor) const override {
    Supplement<ExecutionContext>::Trace(visitor);
    ExecutionContextLifecycleObserver::Trace(visitor);
  }

  void IncrementNoisedCanvasReadbacks() { ++num_noised_canvas_readbacks_; }

  void ContextDestroyed() override;

 private:
  int num_noised_canvas_readbacks_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CANVAS_INTERVENTIONS_CANVAS_INTERVENTIONS_HELPER_H_
