// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEBSOCKET_HANDSHAKE_THROTTLE_PROVIDER_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEBSOCKET_HANDSHAKE_THROTTLE_PROVIDER_H_

#include <memory>

#include "base/task/single_thread_task_runner.h"
#include "base/types/optional_ref.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/platform/web_common.h"

namespace blink {
class WebSocketHandshakeThrottle;

// This interface allows the embedder to provide a WebSocketHandshakeThrottle
// implementation. An instance of this class must be constructed on the render
// thread, and then used and destructed on a single thread, which can be
// different from the render thread.
class BLINK_PLATFORM_EXPORT WebSocketHandshakeThrottleProvider {
 public:
  virtual ~WebSocketHandshakeThrottleProvider() {}

  // Used to copy a WebSocketHandshakeThrottleProvider between worker threads.
  //
  // |task_runner| is used for internal IPC handling of the throttle, and must
  // be bound to the same sequence to the current one.
  virtual std::unique_ptr<WebSocketHandshakeThrottleProvider> Clone(
      scoped_refptr<base::SingleThreadTaskRunner> task_runner) = 0;

  // For requests from frames and dedicated workers, `local_frame_token` should
  // be set to the corresponding frame. For requests from shared or service
  // workers, `local_frame_token` will not be set.
  //
  // |task_runner| is used for internal IPC handling of the throttle, and must
  // be bound to the same sequence to the current one.
  virtual std::unique_ptr<blink::WebSocketHandshakeThrottle> CreateThrottle(
      base::optional_ref<const LocalFrameToken> local_frame_token,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEBSOCKET_HANDSHAKE_THROTTLE_PROVIDER_H_
