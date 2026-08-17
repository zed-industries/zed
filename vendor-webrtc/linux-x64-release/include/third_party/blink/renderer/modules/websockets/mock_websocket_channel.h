// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBSOCKETS_MOCK_WEBSOCKET_CHANNEL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBSOCKETS_MOCK_WEBSOCKET_CHANNEL_H_

#include <memory>

#include "base/memory/scoped_refptr.h"
#include "testing/gmock/include/gmock/gmock.h"
#include "third_party/blink/public/mojom/devtools/console_message.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_array_buffer.h"
#include "third_party/blink/renderer/modules/websockets/websocket_channel.h"
#include "third_party/blink/renderer/platform/blob/blob_data.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class SourceLocation;

class MockWebSocketChannel : public WebSocketChannel {
 public:
  MockWebSocketChannel();
  ~MockWebSocketChannel() override;

  MOCK_METHOD2(Connect, bool(const KURL&, const String&));
  MOCK_METHOD2(Send,
               WebSocketChannel::SendResult(const std::string&,
                                            base::OnceClosure));
  MOCK_METHOD4(Send,
               WebSocketChannel::SendResult(const DOMArrayBuffer&,
                                            size_t,
                                            size_t,
                                            base::OnceClosure));
  MOCK_METHOD1(SendMock, void(BlobDataHandle*));
  void Send(scoped_refptr<BlobDataHandle> handle) override {
    SendMock(handle.get());
  }
  MOCK_CONST_METHOD0(BufferedAmount, unsigned());
  MOCK_METHOD2(Close, void(int, const String&));
  MOCK_METHOD3(FailMock,
               void(const String&,
                    mojom::ConsoleMessageLevel,
                    SourceLocation*));
  void Fail(const String& reason,
            mojom::ConsoleMessageLevel level,
            std::unique_ptr<SourceLocation> location) override {
    FailMock(reason, level, location.get());
  }
  MOCK_METHOD0(Disconnect, void());
  MOCK_METHOD0(CancelHandshake, void());
  MOCK_METHOD0(ApplyBackpressure, void());
  MOCK_METHOD0(RemoveBackpressure, void());
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBSOCKETS_MOCK_WEBSOCKET_CHANNEL_H_
