// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_MEASURE_MEMORY_MEASURE_MEMORY_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_MEASURE_MEMORY_MEASURE_MEMORY_CONTROLLER_H_

#include "base/types/pass_key.h"
#include "components/performance_manager/public/mojom/coordination_unit.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/platform/bindings/scoped_persistent.h"
#include "third_party/blink/renderer/platform/bindings/trace_wrapper_v8_reference.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "v8/include/v8.h"

namespace blink {
class MemoryMeasurement;
class ScriptState;
class ExceptionState;

// The implementation of performance.measureUserAgentSpecificMemory() Web API.
// It is responsible for:
// 1. Starting an asynchronous memory measurement of the main V8 isolate.
// 2. Starting an asynchronous memory measurement of dedicated workers.
// 3. Waiting for measurements to complete.
// 4. Constructing the result and resolving the JS promise.
class MeasureMemoryController final
    : public GarbageCollected<MeasureMemoryController> {
 public:
  // Private constructor guarded by PassKey. Use the StartMeasurement() wrapper
  // to construct the object and start the measurement.
  MeasureMemoryController(base::PassKey<MeasureMemoryController>,
                          v8::Isolate*,
                          v8::Local<v8::Context>,
                          ScriptPromiseResolver<MemoryMeasurement>*);

  ~MeasureMemoryController() = default;

  static ScriptPromise<MemoryMeasurement> StartMeasurement(ScriptState*,
                                                           ExceptionState&);

  void Trace(Visitor* visitor) const;

  void MeasurementComplete(
      performance_manager::mojom::blink::WebMemoryMeasurementPtr);

 private:
  ScopedPersistent<v8::Context> context_;
  Member<ScriptPromiseResolver<MemoryMeasurement>> resolver_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_MEASURE_MEMORY_MEASURE_MEMORY_CONTROLLER_H_
