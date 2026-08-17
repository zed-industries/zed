// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBSOCKETS_WEB_PEPPER_SOCKET_CHANNEL_CLIENT_PROXY_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBSOCKETS_WEB_PEPPER_SOCKET_CHANNEL_CLIENT_PROXY_H_

#include <stdint.h>
#include <memory>
#include "base/memory/raw_ptr.h"
#include "third_party/blink/renderer/modules/websockets/web_pepper_socket_impl.h"
#include "third_party/blink/renderer/modules/websockets/websocket_channel_client.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

// Ideally we want to simply make WebPepperSocketImpl inherit from
// WebPepperSocketChannelClient, but we cannot do that because
// WebSocketChannelClient needs to be on Oilpan's heap whereas
// WebPepperSocketImpl cannot be on Oilpan's heap. Thus we need to introduce a
// proxy class to decouple WebPepperSocketImpl from WebSocketChannelClient.
class WebPepperSocketChannelClientProxy final
    : public GarbageCollected<WebPepperSocketChannelClientProxy>,
      public WebSocketChannelClient {
 public:
  explicit WebPepperSocketChannelClientProxy(WebPepperSocketImpl* impl)
      : impl_(impl) {}

  void DidConnect(const String& subprotocol,
                  const String& extensions) override {
    impl_->DidConnect(subprotocol, extensions);
  }
  void DidReceiveTextMessage(const String& payload) override {
    impl_->DidReceiveTextMessage(payload);
  }
  void DidReceiveBinaryMessage(
      const Vector<base::span<const char>>& data) override {
    std::unique_ptr<Vector<char>> data_to_pass =
        std::make_unique<Vector<char>>();
    for (const auto& span : data) {
      data_to_pass->AppendSpan(span);
    }
    impl_->DidReceiveBinaryMessage(std::move(data_to_pass));
  }
  void DidError() override { impl_->DidError(); }
  void DidConsumeBufferedAmount(uint64_t consumed) override {
    impl_->DidConsumeBufferedAmount(consumed);
  }
  void DidStartClosingHandshake() override {
    impl_->DidStartClosingHandshake();
  }
  void DidClose(ClosingHandshakeCompletionStatus status,
                uint16_t code,
                const String& reason) override {
    WebPepperSocketImpl* impl = impl_;
    impl_ = nullptr;
    impl->DidClose(status, code, reason);
  }

  void Trace(Visitor* visitor) const override {
    WebSocketChannelClient::Trace(visitor);
  }

 private:
  raw_ptr<WebPepperSocketImpl, DanglingUntriaged> impl_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBSOCKETS_WEB_PEPPER_SOCKET_CHANNEL_CLIENT_PROXY_H_
