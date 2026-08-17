// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_BYTE_LENGTH_QUEUING_STRATEGY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_BYTE_LENGTH_QUEUING_STRATEGY_H_

#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/bindings/trace_wrapper_v8_reference.h"
#include "v8/include/v8.h"

namespace blink {

class QueuingStrategyInit;
class ScriptState;
class ScriptValue;

// https://streams.spec.whatwg.org/#blqs-class
class ByteLengthQueuingStrategy final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static ByteLengthQueuingStrategy* Create(ScriptState*,
                                           const QueuingStrategyInit*);

  ByteLengthQueuingStrategy(ScriptState*, const QueuingStrategyInit*);
  ~ByteLengthQueuingStrategy() override;

  double highWaterMark() const { return high_water_mark_; }
  ScriptValue size(ScriptState*) const;

 private:
  const double high_water_mark_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_BYTE_LENGTH_QUEUING_STRATEGY_H_
