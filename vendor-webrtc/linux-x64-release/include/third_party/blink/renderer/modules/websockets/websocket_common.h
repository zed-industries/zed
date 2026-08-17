// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Common functionality shared between DOMWebSocket and WebSocketStream.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBSOCKETS_WEBSOCKET_COMMON_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBSOCKETS_WEBSOCKET_COMMON_H_

#include <stdint.h>

#include <optional>

#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class ExecutionContext;
class ExceptionState;
class WebSocketChannel;
enum class MixedContentAutoupgradeStatus;

// Implements connection- and closing- related functionality that otherwise
// would be duplicated between DOMWebSocket and WebSocketStream. Is embedded
// into those classes and delegated to when needed.
class MODULES_EXPORT WebSocketCommon {
  DISALLOW_NEW();

 public:
  WebSocketCommon() = default;

  WebSocketCommon(const WebSocketCommon&) = delete;
  WebSocketCommon& operator=(const WebSocketCommon&) = delete;

  ~WebSocketCommon() = default;

  enum State { kConnecting = 0, kOpen = 1, kClosing = 2, kClosed = 3 };
  enum class ConnectResult { kSuccess, kException, kAsyncError };

  // Checks |url| and |protocols| are valid, and starts a connection if they
  // are.
  ConnectResult Connect(ExecutionContext*,
                        const String& url,
                        const Vector<String>& protocols,
                        WebSocketChannel*,
                        ExceptionState&);

  // Closes the connection if |code| and |reason| are valid.
  void CloseInternal(std::optional<uint16_t> code,
                     const String& reason,
                     WebSocketChannel*,
                     ExceptionState&);

  State GetState() const { return state_; }
  void SetState(State state) { state_ = state; }
  const KURL& Url() const { return url_; }

  // The following methods are public for testing.

  // Returns true if |protocol| is a valid WebSocket subprotocol name.
  static bool IsValidSubprotocolString(const String& protocol);

  // Escapes non-printing or non-ASCII characters in |protocol| as "\uXXXX" for
  // inclusion in exception messages.
  static String EncodeSubprotocolString(const String& protocol);

  // Joins the strings in |strings| into a single string, with |separator|
  // between each string.
  static String JoinStrings(const Vector<String>& strings,
                            const char* separator);

  // Determines if `code` and `reason` are valid and throws an exception if not.
  // Returns `code` if supplied, otherwise std::nullopt if
  // `reason` is empty and kCloseEventCodeNormalClosure otherwise. `reason` is
  // expected to be a USVString, ie. no unmatched surrogates.
  static std::optional<uint16_t> ValidateCloseCodeAndReason(
      std::optional<uint16_t> code,
      const String& reason,
      ExceptionState&);

 private:
  // Returns true if |character| is allowed in a WebSocket subprotocol name.
  static bool IsValidSubprotocolCharacter(UChar character);

  KURL url_;
  State state_ = kConnecting;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBSOCKETS_WEBSOCKET_COMMON_H_
