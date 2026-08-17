// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_MEASURE_MEMORY_LOCAL_WEB_MEMORY_MEASURER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_MEASURE_MEMORY_LOCAL_WEB_MEMORY_MEASURER_H_

#include "components/performance_manager/public/mojom/web_memory.mojom-blink.h"

#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "v8/include/v8.h"

namespace blink {

class MeasureMemoryController;

// Performs a memory measurement of a V8 isolate containing
// a single context. It is a much simplified version of
// performance_manager::v8_memory::WebMemoryMeasurer and does
// not require aggregation of multiple nodes.
//
// This is used in service and shared workers.
class LocalWebMemoryMeasurer : public v8::MeasureMemoryDelegate {
 public:
  using WebMemoryAttribution =
      performance_manager::mojom::blink::WebMemoryAttribution;
  using WebMemoryMeasurement =
      performance_manager::mojom::blink::WebMemoryMeasurement;

  ~LocalWebMemoryMeasurer() override;

  // Measures the memory usage the given isolate using the given
  // measurement mode. When the measurement is done, it constructs
  // a new WebMemoryMeasurement containing a single breakdown entry
  // with the attribution set to the given scope and URL.
  // The measurement result is passed to
  // MeasureMemoryController::MeasurementComplete.
  static void StartMeasurement(v8::Isolate*,
                               WebMemoryMeasurement::Mode,
                               MeasureMemoryController*,
                               WebMemoryAttribution::Scope,
                               WTF::String attribution_url);

  // v8::MeasureMemoryDelegate overrides.
  bool ShouldMeasure(v8::Local<v8::Context> context) override;
  void MeasurementComplete(v8::MeasureMemoryDelegate::Result result) override;

 private:
  LocalWebMemoryMeasurer(MeasureMemoryController*,
                         WebMemoryAttribution::Scope,
                         WTF::String attribution_url);
  Persistent<MeasureMemoryController> controller_;
  WebMemoryAttribution::Scope attribution_scope_;
  WTF::String attribution_url_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_MEASURE_MEMORY_LOCAL_WEB_MEMORY_MEASURER_H_
