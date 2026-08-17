// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_NAVIGATION_TIMING_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_NAVIGATION_TIMING_H_

#include "services/network/public/mojom/url_response_head.mojom.h"
#include "third_party/blink/public/mojom/back_forward_cache_not_restored_reasons.mojom-blink.h"
#include "third_party/blink/public/mojom/timing/resource_timing.mojom-blink-forward.h"
#include "third_party/blink/public/web/web_navigation_type.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_navigation_entropy.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_navigation_timing_type.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/dom_high_res_time_stamp.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/timing/not_restored_reasons.h"
#include "third_party/blink/renderer/core/timing/performance_resource_timing.h"
#include "third_party/blink/renderer/core/timing/performance_timing_confidence.h"

namespace blink {

struct DocumentTimingValues;
struct DocumentLoadTimingValues;
class DocumentLoader;
class LocalDOMWindow;
class ExecutionContext;
class V8NavigationEntropy;

class CORE_EXPORT PerformanceNavigationTiming final
    : public PerformanceResourceTiming,
      public ExecutionContextClient {
  DEFINE_WRAPPERTYPEINFO();
  friend class PerformanceNavigationTimingTest;

 public:
  PerformanceNavigationTiming(LocalDOMWindow&,
                              mojom::blink::ResourceTimingInfoPtr,
                              base::TimeTicks time_origin);
  ~PerformanceNavigationTiming() override;

  // Attributes inherited from PerformanceEntry.
  DOMHighResTimeStamp duration() const override;
  const AtomicString& entryType() const override;
  PerformanceEntryType EntryTypeEnum() const override;

  // PerformanceNavigationTiming's unique attributes.
  DOMHighResTimeStamp unloadEventStart() const;
  DOMHighResTimeStamp unloadEventEnd() const;
  DOMHighResTimeStamp domInteractive() const;
  DOMHighResTimeStamp domContentLoadedEventStart() const;
  DOMHighResTimeStamp domContentLoadedEventEnd() const;
  DOMHighResTimeStamp domComplete() const;
  DOMHighResTimeStamp loadEventStart() const;
  DOMHighResTimeStamp loadEventEnd() const;
  V8NavigationTimingType type() const;
  uint16_t redirectCount() const;
  NotRestoredReasons* notRestoredReasons() const;
  PerformanceTimingConfidence* confidence() const;
  V8NavigationEntropy systemEntropy() const;
  DOMHighResTimeStamp criticalCHRestart(ScriptState* script_state) const;

  // PerformanceResourceTiming overrides:
  DOMHighResTimeStamp fetchStart() const override;
  DOMHighResTimeStamp redirectStart() const override;
  DOMHighResTimeStamp redirectEnd() const override;
  DOMHighResTimeStamp responseEnd() const override;
  AtomicString deliveryType() const override;

  void Trace(Visitor*) const override;

  void OnBodyLoadFinished(int64_t encoded_body_size, int64_t decoded_body_size);

 protected:
  void BuildJSONValue(V8ObjectBuilder&) const override;

 private:
  friend class PerformanceNavigationTimingActivationStart;

  static V8NavigationTimingType::Enum GetNavigationTimingType(
      WebNavigationType);

  V8NavigationEntropy::Enum GetSystemEntropy() const;
  DocumentLoader* GetDocumentLoader() const;

  NotRestoredReasons* BuildNotRestoredReasons(
      const mojom::blink::BackForwardCacheNotRestoredReasonsPtr& reasons) const;

  const network::mojom::NavigationDeliveryType navigation_delivery_type_;
  const WebNavigationType navigation_type_;

  Member<DocumentTimingValues> document_timing_values_;
  Member<DocumentLoadTimingValues> document_load_timing_values_;
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_NAVIGATION_TIMING_H_
