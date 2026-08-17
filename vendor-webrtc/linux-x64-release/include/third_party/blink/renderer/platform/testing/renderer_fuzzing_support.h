// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_RENDERER_FUZZING_SUPPORT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_RENDERER_FUZZING_SUPPORT_H_

#include <vector>

#include "third_party/blink/public/common/associated_interfaces/associated_interface_provider.h"
#include "third_party/blink/public/common/thread_safe_browser_interface_broker_proxy.h"
#include "third_party/blink/public/platform/browser_interface_broker_proxy.h"

namespace blink {

class RendererFuzzingSupport {
 public:
  // Use this constructor in LLVMFuzzerTestOneInput.
  static void Run(
      const blink::BrowserInterfaceBrokerProxy* context_interface_broker_proxy,
      blink::ThreadSafeBrowserInterfaceBrokerProxy*
          process_interface_broker_proxy,
      blink::AssociatedInterfaceProvider* associated_interface_provider,
      const std::string& fuzzer_id,
      std::vector<uint8_t>&& input,
      base::OnceClosure done_closure);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_RENDERER_FUZZING_SUPPORT_H_
