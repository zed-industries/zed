// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CONTROLLER_PERFORMANCE_MANAGER_V8_DETAILED_MEMORY_REPORTER_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CONTROLLER_PERFORMANCE_MANAGER_V8_DETAILED_MEMORY_REPORTER_IMPL_H_

#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "mojo/public/cpp/bindings/receiver.h"
#include "third_party/blink/public/mojom/performance_manager/v8_detailed_memory_reporter.mojom-blink.h"
#include "third_party/blink/renderer/controller/controller_export.h"

namespace blink {

// Exposes V8 per-frame associated memory metrics to the browser.
class CONTROLLER_EXPORT V8DetailedMemoryReporterImpl
    : public mojom::blink::V8DetailedMemoryReporter {
 public:
  static void Bind(
      mojo::PendingReceiver<mojom::blink::V8DetailedMemoryReporter> receiver);

  void GetV8MemoryUsage(Mode mode, GetV8MemoryUsageCallback callback) override;

 private:
  mojo::Receiver<mojom::blink::V8DetailedMemoryReporter> receiver_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CONTROLLER_PERFORMANCE_MANAGER_V8_DETAILED_MEMORY_REPORTER_IMPL_H_
