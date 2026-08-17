// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_TIMING_CONFIDENCE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_TIMING_CONFIDENCE_H_

#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_performance_timing_confidence_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class CORE_EXPORT PerformanceTimingConfidence final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  PerformanceTimingConfidence(double, V8PerformanceTimingConfidenceValue);

  double randomizedTriggerRate() const { return randomizedTriggerRate_; }
  const V8PerformanceTimingConfidenceValue value() const { return value_; }

  ScriptObject toJSON(ScriptState* script_state) const;

 private:
  double randomizedTriggerRate_;
  V8PerformanceTimingConfidenceValue value_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_TIMING_CONFIDENCE_H_
