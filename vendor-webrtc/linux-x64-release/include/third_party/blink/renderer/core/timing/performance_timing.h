// Copyright 2010 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_TIMING_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_TIMING_H_

#include "base/time/time.h"
#include "third_party/blink/public/common/performance/largest_contentful_paint_type.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/instrumentation/tracing/traced_value.h"

namespace blink {

class DocumentLoadTiming;
class DocumentLoader;
class DocumentTiming;
class ResourceLoadTiming;
class ScriptObject;
class ScriptState;

// Legacy support for NT1(https://www.w3.org/TR/navigation-timing/).
class CORE_EXPORT PerformanceTiming final : public ScriptWrappable,
                                            public ExecutionContextClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit PerformanceTiming(ExecutionContext*);

  uint64_t navigationStart() const;
  uint64_t unloadEventStart() const;
  uint64_t unloadEventEnd() const;
  uint64_t redirectStart() const;
  uint64_t redirectEnd() const;
  uint64_t fetchStart() const;
  uint64_t domainLookupStart() const;
  uint64_t domainLookupEnd() const;
  uint64_t connectStart() const;
  uint64_t connectEnd() const;
  uint64_t secureConnectionStart() const;
  uint64_t requestStart() const;
  uint64_t responseStart() const;
  uint64_t responseEnd() const;
  uint64_t domLoading() const;
  uint64_t domInteractive() const;
  uint64_t domContentLoadedEventStart() const;
  uint64_t domContentLoadedEventEnd() const;
  uint64_t domComplete() const;
  uint64_t loadEventStart() const;
  uint64_t loadEventEnd() const;

  // Returns true iff the given string identifies an attribute of
  // |performance.timing|.
  static bool IsAttributeName(const AtomicString&);

  // Returns the attribute value identified by the given string. The string
  // passed as parameter must be an attribute of |performance.timing|.
  uint64_t GetNamedAttribute(const AtomicString&) const;

  ScriptObject toJSONForBinding(ScriptState*) const;

  void Trace(Visitor*) const override;

  uint64_t MonotonicTimeToIntegerMilliseconds(base::TimeTicks) const;

  void WriteInto(perfetto::TracedDictionary&) const;

 private:
  const DocumentTiming* GetDocumentTiming() const;
  DocumentLoader* GetDocumentLoader() const;
  DocumentLoadTiming* GetDocumentLoadTiming() const;
  ResourceLoadTiming* GetResourceLoadTiming() const;

  typedef uint64_t (PerformanceTiming::*PerformanceTimingGetter)() const;
  using NameToAttributeMap = HashMap<AtomicString, PerformanceTimingGetter>;
  static const NameToAttributeMap& GetAttributeMapping();

  bool cross_origin_isolated_capability_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_TIMING_H_
