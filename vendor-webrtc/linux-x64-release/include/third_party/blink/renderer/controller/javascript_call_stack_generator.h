// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CONTROLLER_JAVASCRIPT_CALL_STACK_GENERATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CONTROLLER_JAVASCRIPT_CALL_STACK_GENERATOR_H_

#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "mojo/public/cpp/bindings/receiver.h"
#include "third_party/blink/public/mojom/call_stack_generator/call_stack_generator.mojom-blink.h"
#include "third_party/blink/renderer/controller/controller_export.h"
#include "third_party/blink/renderer/controller/javascript_call_stack_collector.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "v8/include/v8.h"

namespace blink {

class CONTROLLER_EXPORT JavaScriptCallStackGenerator
    : public mojom::blink::CallStackGenerator {
 public:
  static void Bind(
      mojo::PendingReceiver<mojom::blink::CallStackGenerator> receiver);
  void CollectJavaScriptCallStack(
      CollectJavaScriptCallStackCallback callback) override;
  void OnCollectorFinished(JavaScriptCallStackCollector* collector);

 private:
  void InterruptIsolateAndCollectCallStack(v8::Isolate* isolate);

  mojo::Receiver<mojom::blink::CallStackGenerator> receiver_{this};
  WTF::HashMap<JavaScriptCallStackCollector*,
               std::unique_ptr<JavaScriptCallStackCollector>>
      collectors_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CONTROLLER_JAVASCRIPT_CALL_STACK_GENERATOR_H_
