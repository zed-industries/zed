// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_SERVER_TIMING_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_SERVER_TIMING_H_

#include "third_party/blink/public/mojom/timing/resource_timing.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_object_builder.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ResourceResponse;
class PerformanceServerTiming;

class CORE_EXPORT PerformanceServerTiming final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  PerformanceServerTiming(const String& name,
                          double duration,
                          const String& description);
  ~PerformanceServerTiming() override;

  const String& name() const { return name_; }
  double duration() const { return duration_; }
  const String& description() const { return description_; }

  static HeapVector<Member<PerformanceServerTiming>> ParseServerTiming(
      const ResourceResponse&);
  static HeapVector<Member<PerformanceServerTiming>> FromParsedServerTiming(
      const Vector<mojom::blink::ServerTimingInfoPtr>&);

  ScriptObject toJSONForBinding(ScriptState*) const;

 private:
  const String name_;
  double duration_;
  const String description_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_SERVER_TIMING_H_
