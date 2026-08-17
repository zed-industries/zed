/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
 * Copyright (C) 2012 Intel Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_RESOURCE_TIMING_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_RESOURCE_TIMING_H_

#include "base/time/time.h"
#include "third_party/blink/public/mojom/fetch/fetch_api_request.mojom-blink.h"
#include "third_party/blink/public/mojom/timing/performance_mark_or_measure.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/timing/resource_timing.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/timing/resource_timing.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/dom_high_res_time_stamp.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/timing/performance_entry.h"
#include "third_party/blink/renderer/core/timing/performance_server_timing.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class V8RenderBlockingStatusType;

class CORE_EXPORT PerformanceResourceTiming : public PerformanceEntry {
  DEFINE_WRAPPERTYPEINFO();
  friend class PerformanceResourceTimingTest;

 public:
  // This constructor transfers ownership of the ResourceTimingInfo data to the
  // PerformanceResourceTiming entry.
  PerformanceResourceTiming(mojom::blink::ResourceTimingInfoPtr,
                            const AtomicString& initiator_type,
                            base::TimeTicks time_origin,
                            bool cross_origin_isolated_capability,
                            ExecutionContext* context);
  ~PerformanceResourceTiming() override;

  const AtomicString& entryType() const override;
  PerformanceEntryType EntryTypeEnum() const override;

  // Related doc: https://goo.gl/uNecAj.
  AtomicString initiatorType() const { return initiator_type_; }
  virtual AtomicString deliveryType() const;
  AtomicString nextHopProtocol() const;
  virtual V8RenderBlockingStatusType renderBlockingStatus() const;
  virtual AtomicString contentType() const;
  virtual AtomicString contentEncoding() const;
  DOMHighResTimeStamp workerStart() const;
  DOMHighResTimeStamp workerRouterEvaluationStart() const;
  DOMHighResTimeStamp workerCacheLookupStart() const;
  virtual DOMHighResTimeStamp redirectStart() const;
  virtual DOMHighResTimeStamp redirectEnd() const;
  virtual DOMHighResTimeStamp fetchStart() const;
  DOMHighResTimeStamp domainLookupStart() const;
  DOMHighResTimeStamp domainLookupEnd() const;
  DOMHighResTimeStamp connectStart() const;
  DOMHighResTimeStamp connectEnd() const;
  DOMHighResTimeStamp secureConnectionStart() const;
  DOMHighResTimeStamp requestStart() const;
  DOMHighResTimeStamp responseStart() const;
  DOMHighResTimeStamp firstInterimResponseStart() const;
  DOMHighResTimeStamp finalResponseHeadersStart() const;
  virtual DOMHighResTimeStamp responseEnd() const;
  uint64_t transferSize() const;
  virtual uint64_t encodedBodySize() const;
  virtual uint64_t decodedBodySize() const;
  uint16_t responseStatus() const;
  const HeapVector<Member<PerformanceServerTiming>>& serverTiming() const;
  AtomicString workerMatchedSourceType() const;
  AtomicString workerFinalSourceType() const;

  void Trace(Visitor*) const override;

 protected:
  void BuildJSONValue(V8ObjectBuilder&) const override;

  base::TimeTicks TimeOrigin() const { return time_origin_; }
  bool CrossOriginIsolatedCapability() const {
    return cross_origin_isolated_capability_;
  }

  bool AllowNegativeValues() const { return info_->allow_negative_values; }

  static uint64_t GetTransferSize(uint64_t encoded_body_size,
                                  mojom::blink::CacheState cache_state);

 protected:
  AtomicString GetDeliveryType() const;
  void UpdateBodySizes(int64_t encoded_body_size, int64_t decoded_body_size) {
    info_->encoded_body_size = encoded_body_size;
    info_->decoded_body_size = decoded_body_size;
  }

 private:
  // https://w3c.github.io/resource-timing/#dom-performanceresourcetiming-transfersize
  static const size_t kHeaderSize = 300;

  AtomicString GetNextHopProtocol(const AtomicString& alpn_negotiated_protocol,
                                  const AtomicString& connection_info) const;

  // Returns true if the response comes from the CacheStorage. This is
  // regardless of where the response came from whether it is from the `cache`
  // rule of ServiceWorker static routing API, or the fetch handler's
  // respondWith().
  bool IsResponseFromCacheStorage() const;

  DOMHighResTimeStamp GetAnyFirstResponseStart() const;
  double WorkerReady() const;

  AtomicString initiator_type_;

  // Do not access private fields directly. Use getter methods.
  base::TimeTicks time_origin_;
  bool cross_origin_isolated_capability_;

  HeapVector<Member<PerformanceServerTiming>> server_timing_;
  mojom::blink::ResourceTimingInfoPtr info_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_RESOURCE_TIMING_H_
