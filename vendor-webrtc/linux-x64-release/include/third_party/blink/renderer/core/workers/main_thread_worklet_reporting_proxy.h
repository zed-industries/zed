// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_MAIN_THREAD_WORKLET_REPORTING_PROXY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_MAIN_THREAD_WORKLET_REPORTING_PROXY_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/frame/web_feature_forward.h"
#include "third_party/blink/renderer/core/workers/worker_reporting_proxy.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"

namespace blink {

class ExecutionContext;

class CORE_EXPORT MainThreadWorkletReportingProxy
    : public WorkerReportingProxy {
 public:
  explicit MainThreadWorkletReportingProxy(ExecutionContext*);
  ~MainThreadWorkletReportingProxy() override = default;

  // Implements WorkerReportingProxy.
  void CountFeature(WebFeature) override;
  void CountWebDXFeature(mojom::blink::WebDXFeature) override;
  void DidTerminateWorkerThread() override;

 private:
  Persistent<ExecutionContext> context_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_MAIN_THREAD_WORKLET_REPORTING_PROXY_H_
