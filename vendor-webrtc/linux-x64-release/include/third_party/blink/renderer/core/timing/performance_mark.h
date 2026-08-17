/*
 * Copyright (C) 2012 Intel Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_MARK_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_MARK_H_

#include "base/time/time.h"
#include "third_party/blink/public/mojom/timing/performance_mark_or_measure.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/serialization/serialized_script_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/timing/performance_entry.h"
#include "third_party/blink/renderer/platform/bindings/trace_wrapper_v8_reference.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ExceptionState;
class PerformanceMarkOptions;

class CORE_EXPORT PerformanceMark final : public PerformanceEntry {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // This method corresponds to the PerformanceMark constructor defined in the
  // User Timing L3 spec
  // (https://w3c.github.io/user-timing/#the-performancemark-constructor). It
  // gets called as a subroutine of the `performance.mark` method as well as
  // whenever script runs `new PerformanceMark(..)`.
  static PerformanceMark* Create(ScriptState*,
                                 const AtomicString& mark_name,
                                 PerformanceMarkOptions*,
                                 ExceptionState&);

  // This constructor is only public so that MakeGarbageCollected can call it.
  PerformanceMark(const AtomicString& name,
                  double start_time,
                  base::TimeTicks unsafe_time_for_traces,
                  scoped_refptr<SerializedScriptValue>,
                  ExceptionState& exception_state,
                  DOMWindow* source);

  ~PerformanceMark() override = default;

  const AtomicString& entryType() const override;
  PerformanceEntryType EntryTypeEnum() const override;
  mojom::blink::PerformanceMarkOrMeasurePtr ToMojoPerformanceMarkOrMeasure()
      override;

  ScriptValue detail(ScriptState*);

  void Trace(Visitor*) const override;

  base::TimeTicks UnsafeTimeForTraces() const {
    return unsafe_time_for_traces_;
  }

  using UserFeatureNameToWebFeatureMap =
      HashMap<String, mojom::blink::WebFeature>;
  static const UserFeatureNameToWebFeatureMap&
  GetUseCounterMappingForTesting() {
    return GetUseCounterMapping();
  }
  static std::optional<mojom::blink::WebFeature>
  GetWebFeatureForUserFeatureName(const String& feature_name);

 private:
  scoped_refptr<SerializedScriptValue> serialized_detail_;
  // In order to prevent cross-world reference leak, we create a copy of the
  // detail for each world.
  HeapHashMap<WeakMember<ScriptState>, TraceWrapperV8Reference<v8::Value>>
      deserialized_detail_map_;
  static const UserFeatureNameToWebFeatureMap& GetUseCounterMapping();
  base::TimeTicks unsafe_time_for_traces_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_MARK_H_
