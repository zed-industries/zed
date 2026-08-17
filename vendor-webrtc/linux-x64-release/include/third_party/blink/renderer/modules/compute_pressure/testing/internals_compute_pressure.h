// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_COMPUTE_PRESSURE_TESTING_INTERNALS_COMPUTE_PRESSURE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_COMPUTE_PRESSURE_TESTING_INTERNALS_COMPUTE_PRESSURE_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_pressure_source.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_pressure_state.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class CreateVirtualPressureSourceOptions;
class Internals;
class ScriptState;

class InternalsComputePressure {
  STATIC_ONLY(InternalsComputePressure);

 public:
  static ScriptPromise<IDLUndefined> createVirtualPressureSource(
      ScriptState* script_state,
      Internals&,
      V8PressureSource source,
      CreateVirtualPressureSourceOptions*);

  static ScriptPromise<IDLUndefined> removeVirtualPressureSource(
      ScriptState* script_state,
      Internals&,
      V8PressureSource source);

  static ScriptPromise<IDLUndefined> updateVirtualPressureSource(
      ScriptState* script_state,
      Internals&,
      V8PressureSource posture,
      V8PressureState state);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_COMPUTE_PRESSURE_TESTING_INTERNALS_COMPUTE_PRESSURE_H_
