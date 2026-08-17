// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
//
// WebSocketHandshakeThrottle provides a facility for embedders to delay
// WebSocket connection establishment. Specifically, at the same time as the
// handshake is started blink::Platform::CreateWebSocketHandshakeThrottle() will
// be called. If a non-null WebSocketHandshakeThrottle is returned then
// ThrottleHandshake() will be called on it. If the result is error then the
// handshake will be aborted, and a connection error will be reported to
// Javascript. If the throttle hasn't reported a result when the WebSocket
// handshake succeeds then Blink will wait for the throttle result before
// reporting the connection is open to Javascript.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEBSOCKET_HANDSHAKE_THROTTLE_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEBSOCKET_HANDSHAKE_THROTTLE_H_

#include <optional>

#include "base/functional/callback.h"

namespace blink {

class WebURL;
class WebString;
class WebSecurityOrigin;

// Embedders can implement this class to delay WebSocket connections.
class WebSocketHandshakeThrottle {
 public:
  // Destruction implies that the handshake has been aborted. Any ongoing work
  // should be cleaned up if possible.
  virtual ~WebSocketHandshakeThrottle() = default;

  // The closure callback OnCompletion should be called asynchronously
  // to permit Javascript to use the connection or not. On error, a message
  // should be passed to be displayed on the console indicating why the
  // handshake was blocked. This object will be destroyed synchronously inside
  // the callback. Callback must not be called after this object has been
  // destroyed.
  using OnCompletion =
      base::OnceCallback<void(const std::optional<WebString>& error)>;
  // |creator_origin| is the origin of the execution context that created
  // this WebSocket.
  // |isolated_world_origin| indicates the origin of the isolated world if the
  // subresource request is initiated from an isolated world (e.g. from a
  // content script of a Chrome Extension). Otherwise, |isolated_world_origin|
  // is null.
  virtual void ThrottleHandshake(const WebURL&,
                                 const WebSecurityOrigin& creator_origin,
                                 const WebSecurityOrigin& isolated_world_origin,
                                 OnCompletion) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEBSOCKET_HANDSHAKE_THROTTLE_H_
