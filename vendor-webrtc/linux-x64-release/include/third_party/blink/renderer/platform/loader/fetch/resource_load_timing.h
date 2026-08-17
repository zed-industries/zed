/*
 * Copyright (C) 2010 Google, Inc. All Rights Reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_LOAD_TIMING_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_LOAD_TIMING_H_

#include "base/memory/scoped_refptr.h"
#include "base/time/time.h"
#include "services/network/public/mojom/load_timing_info.mojom-blink-forward.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/ref_counted.h"

namespace blink {

class PLATFORM_EXPORT ResourceLoadTiming
    : public RefCounted<ResourceLoadTiming> {
 public:
  static scoped_refptr<ResourceLoadTiming> Create();

  network::mojom::blink::LoadTimingInfoPtr ToMojo() const;

  void SetDomainLookupStart(base::TimeTicks);
  void SetRequestTime(base::TimeTicks);
  void SetProxyStart(base::TimeTicks);
  void SetProxyEnd(base::TimeTicks);
  void SetDomainLookupEnd(base::TimeTicks);
  void SetConnectStart(base::TimeTicks);
  void SetConnectEnd(base::TimeTicks);
  void SetWorkerStart(base::TimeTicks);
  void SetWorkerReady(base::TimeTicks);
  void SetWorkerFetchStart(base::TimeTicks);
  void SetWorkerRespondWithSettled(base::TimeTicks);
  void SetWorkerRouterEvaluationStart(base::TimeTicks);
  void SetWorkerCacheLookupStart(base::TimeTicks);
  void SetSendStart(base::TimeTicks);
  void SetSendEnd(base::TimeTicks);
  void SetReceiveHeadersStart(base::TimeTicks);
  void SetReceiveHeadersEnd(base::TimeTicks);
  void SetReceiveNonInformationalHeaderStart(base::TimeTicks);
  void SetReceiveEarlyHintsStart(base::TimeTicks);
  void SetSslStart(base::TimeTicks);
  void SetSslEnd(base::TimeTicks);
  void SetPushStart(base::TimeTicks);
  void SetPushEnd(base::TimeTicks);
  void SetDiscoveryTime(base::TimeTicks);
  void SetResponseEnd(base::TimeTicks);

  base::TimeTicks DomainLookupStart() const { return domain_lookup_start_; }
  base::TimeTicks RequestTime() const { return request_time_; }
  base::TimeTicks ProxyStart() const { return proxy_start_; }
  base::TimeTicks ProxyEnd() const { return proxy_end_; }
  base::TimeTicks DomainLookupEnd() const { return domain_lookup_end_; }
  base::TimeTicks ConnectStart() const { return connect_start_; }
  base::TimeTicks ConnectEnd() const { return connect_end_; }
  base::TimeTicks WorkerStart() const { return worker_start_; }
  base::TimeTicks WorkerRouterEvaluationStart() const {
    return worker_router_evaluation_start_;
  }
  base::TimeTicks WorkerCacheLokupStart() const {
    return worker_cache_lookup_start_;
  }
  base::TimeTicks WorkerReady() const { return worker_ready_; }
  base::TimeTicks WorkerFetchStart() const { return worker_fetch_start_; }
  base::TimeTicks WorkerRespondWithSettled() const {
    return worker_respond_with_settled_;
  }
  base::TimeTicks SendStart() const { return send_start_; }
  base::TimeTicks SendEnd() const { return send_end_; }
  base::TimeTicks ReceiveHeadersStart() const { return receive_headers_start_; }
  base::TimeTicks ReceiveHeadersEnd() const { return receive_headers_end_; }
  base::TimeTicks ReceiveNonInformationalHeadersStart() const {
    return receive_non_informational_headers_start_;
  }
  base::TimeTicks ReceiveEarlyHintsStart() const {
    return receive_early_hints_start_;
  }
  base::TimeTicks SslStart() const { return ssl_start_; }
  base::TimeTicks SslEnd() const { return ssl_end_; }
  base::TimeTicks PushStart() const { return push_start_; }
  base::TimeTicks PushEnd() const { return push_end_; }
  base::TimeTicks DiscoveryTime() const { return discovery_time_; }
  base::TimeTicks ResponseEnd() const { return response_end_; }

  double CalculateMillisecondDelta(base::TimeTicks) const;

 private:
  ResourceLoadTiming();

  // We want to present a unified timeline to Javascript. Using walltime is
  // problematic, because the clock may skew while resources load. To prevent
  // that skew, we record a single reference walltime when root document
  // navigation begins. All other times are recorded using
  // monotonicallyIncreasingTime(). When a time needs to be presented to
  // Javascript, we build a pseudo-walltime using the following equation
  // (m_requestTime as example):
  //   pseudo time = document wall reference +
  //                     (m_requestTime - document monotonic reference).

  // All values from monotonicallyIncreasingTime(), in base::TimeTicks.
  base::TimeTicks request_time_;
  base::TimeTicks proxy_start_;
  base::TimeTicks proxy_end_;
  base::TimeTicks domain_lookup_start_;
  base::TimeTicks domain_lookup_end_;
  base::TimeTicks connect_start_;
  base::TimeTicks connect_end_;
  base::TimeTicks worker_start_;
  base::TimeTicks worker_ready_;
  base::TimeTicks worker_fetch_start_;
  base::TimeTicks worker_respond_with_settled_;
  base::TimeTicks worker_router_evaluation_start_;
  base::TimeTicks worker_cache_lookup_start_;
  base::TimeTicks send_start_;
  base::TimeTicks send_end_;
  base::TimeTicks receive_headers_start_;
  base::TimeTicks receive_headers_end_;
  base::TimeTicks receive_non_informational_headers_start_;
  base::TimeTicks receive_early_hints_start_;
  base::TimeTicks ssl_start_;
  base::TimeTicks ssl_end_;
  base::TimeTicks push_start_;
  base::TimeTicks push_end_;
  base::TimeTicks discovery_time_;
  base::TimeTicks response_end_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_LOAD_TIMING_H_
