/*
 * Copyright (C) 2009 Google Inc.  All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBSOCKETS_WEBSOCKET_CHANNEL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBSOCKETS_WEBSOCKET_CHANNEL_H_

#include <memory>
#include "base/functional/callback_forward.h"
#include "third_party/blink/public/mojom/devtools/console_message.mojom-blink.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/source_location.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class BlobDataHandle;
class DOMArrayBuffer;
class KURL;

class MODULES_EXPORT WebSocketChannel
    : public GarbageCollected<WebSocketChannel> {
 public:
  enum class SendResult { kSentSynchronously, kCallbackWillBeCalled };

  WebSocketChannel() = default;

  WebSocketChannel(const WebSocketChannel&) = delete;
  WebSocketChannel& operator=(const WebSocketChannel&) = delete;

  virtual ~WebSocketChannel() = default;

  enum CloseEventCode {
    kCloseEventCodeNotSpecified = -1,
    kCloseEventCodeNormalClosure = 1000,
    kCloseEventCodeGoingAway = 1001,
    kCloseEventCodeProtocolError = 1002,
    kCloseEventCodeUnsupportedData = 1003,
    kCloseEventCodeFrameTooLarge = 1004,
    kCloseEventCodeNoStatusRcvd = 1005,
    kCloseEventCodeAbnormalClosure = 1006,
    kCloseEventCodeInvalidFramePayloadData = 1007,
    kCloseEventCodePolicyViolation = 1008,
    kCloseEventCodeMessageTooBig = 1009,
    kCloseEventCodeMandatoryExt = 1010,
    kCloseEventCodeInternalError = 1011,
    kCloseEventCodeTLSHandshake = 1015,
    kCloseEventCodeMinimumUserDefined = 3000,
    kCloseEventCodeMaximumUserDefined = 4999
  };

  virtual bool Connect(const KURL&, const String& protocol) = 0;
  virtual SendResult Send(const std::string&,
                          base::OnceClosure completion_callback) = 0;
  virtual SendResult Send(const DOMArrayBuffer&,
                          size_t byte_offset,
                          size_t byte_length,
                          base::OnceClosure completion_callback) = 0;

  // Blobs are always sent asynchronously. No callers currently need completion
  // callbacks for Blobs, so they are not implemented.
  virtual void Send(scoped_refptr<BlobDataHandle>) = 0;

  // Do not call |Send| after calling this method.
  virtual void Close(int code, const String& reason) = 0;

  // Log the reason text and close the connection. Will call DidClose().
  // The mojom::ConsoleMessageLevel parameter will be used for the level
  // of the message shown at the devtools console. SourceLocation parameter may
  // be shown with the reason text at the devtools console. Even if location is
  // specified, it may be ignored and the "current" location in the sense of
  // JavaScript execution may be shown if this method is called in a JS
  // execution context. Location should not be null.
  virtual void Fail(const String& reason,
                    mojom::ConsoleMessageLevel,
                    std::unique_ptr<SourceLocation>) = 0;

  // Do not call any methods after calling this method.
  virtual void Disconnect() = 0;  // Will suppress didClose().

  // Cancel the WebSocket handshake. Does nothing if the connection is already
  // established. Do not call any other methods after this one.
  virtual void CancelHandshake() = 0;

  // Clients can call ApplyBackpressure() to indicate that they want to stop
  // receiving new messages. WebSocketChannelClient::DidReceive*Message() may
  // still be called after this, until existing flow control quota is used up.
  virtual void ApplyBackpressure() = 0;

  // Clients should call RemoveBackpressure() after calling ApplyBackpressure()
  // to indicate that they are ready to receive new messages.
  virtual void RemoveBackpressure() = 0;

  virtual void Trace(Visitor* visitor) const {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBSOCKETS_WEBSOCKET_CHANNEL_H_
