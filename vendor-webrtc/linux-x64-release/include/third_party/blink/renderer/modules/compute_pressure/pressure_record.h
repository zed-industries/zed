// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_COMPUTE_PRESSURE_PRESSURE_RECORD_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_COMPUTE_PRESSURE_PRESSURE_RECORD_H_

#include "third_party/blink/renderer/bindings/modules/v8/v8_pressure_source.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_pressure_state.h"
#include "third_party/blink/renderer/core/dom/dom_high_res_time_stamp.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class ScriptObject;
class ScriptState;

// https://w3c.github.io/compute-pressure/#the-pressurerecord-interface

class MODULES_EXPORT PressureRecord final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  PressureRecord(V8PressureSource::Enum,
                 V8PressureState::Enum,
                 const DOMHighResTimeStamp);
  ~PressureRecord() override;

  V8PressureSource source() const;
  V8PressureState state() const;
  DOMHighResTimeStamp time() const;

  ScriptObject toJSON(ScriptState*) const;

 private:
  const V8PressureSource::Enum source_;
  const V8PressureState::Enum state_;
  const DOMHighResTimeStamp time_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_COMPUTE_PRESSURE_PRESSURE_RECORD_H_
