// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CONTROLLER_JAVASCRIPT_CALL_STACK_COLLECTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CONTROLLER_JAVASCRIPT_CALL_STACK_COLLECTOR_H_

#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/mojom/call_stack_generator/call_stack_generator.mojom-blink.h"
#include "third_party/blink/renderer/controller/controller_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "v8/include/v8.h"

namespace blink {

inline constexpr char kExtensionFrameOmittedMessage[] =
    "Stack omitted due to extension script frames.";
inline constexpr char kStackFramePrefix[] = "\n    at ";
inline constexpr char kWebsiteOwnerNotOptedInMessage[] =
    "Website owner has not opted in for JS call stacks in crash reports.";

class CONTROLLER_EXPORT JavaScriptCallStackCollector {
 public:
  using FinishedCallback =
      base::OnceCallback<void(JavaScriptCallStackCollector*)>;
  using CollectJavaScriptCallStackCallback =
      mojom::blink::CallStackGenerator::CollectJavaScriptCallStackCallback;

  explicit JavaScriptCallStackCollector(
      CollectJavaScriptCallStackCallback&& result_callback,
      FinishedCallback finished_callback)
      : result_callback_(std::move(result_callback)),
        finished_callback_(std::move(finished_callback)) {}

  void InterruptIsolateAndCollectCallStack(v8::Isolate* isolate);
  void CollectJavaScriptCallStack();
  void HandleCallStackCollected(
      const String& call_stack,
      const std::optional<LocalFrameToken> frame_token);

 private:
  CollectJavaScriptCallStackCallback result_callback_;
  FinishedCallback finished_callback_;
  bool has_interrupted_isolate_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CONTROLLER_JAVASCRIPT_CALL_STACK_COLLECTOR_H_
