// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBSOCKETS_WEBSOCKET_STREAM_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBSOCKETS_WEBSOCKET_STREAM_H_

#include <stdint.h>

#include <optional>

#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_property.h"
#include "third_party/blink/renderer/core/dom/abort_signal.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/websockets/websocket_channel_client.h"
#include "third_party/blink/renderer/modules/websockets/websocket_common.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/bindings/trace_wrapper_v8_reference.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "v8/include/v8.h"

namespace blink {

class ExceptionState;
class ExecutionContext;
class ScriptState;
class ScriptValue;
class WebSocketChannel;
class WebSocketCloseInfo;
class WebSocketOpenInfo;
class WebSocketStreamOptions;

// Implements of JavaScript-exposed WebSocketStream API. See design doc at
// https://docs.google.com/document/d/1XuxEshh5VYBYm1qRVKordTamCOsR-uGQBCYFcHXP4L0/edit

class MODULES_EXPORT WebSocketStream final
    : public ScriptWrappable,
      public ActiveScriptWrappable<WebSocketStream>,
      public ExecutionContextLifecycleObserver,
      public WebSocketChannelClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // IDL constructors
  static WebSocketStream* Create(ScriptState*,
                                 const String& url,
                                 WebSocketStreamOptions*,
                                 ExceptionState&);
  static WebSocketStream* CreateForTesting(ScriptState*,
                                           const String& url,
                                           WebSocketStreamOptions*,
                                           WebSocketChannel*,
                                           ExceptionState&);

  // The constructor only creates the object. It cannot fail. The private
  // Connect() method is used by Create() to start connecting.
  WebSocketStream(ExecutionContext*, ScriptState*);
  ~WebSocketStream() override;

  // IDL properties
  const KURL& url() const { return common_.Url(); }
  ScriptPromise<WebSocketOpenInfo> opened(ScriptState*) const;
  ScriptPromise<WebSocketCloseInfo> closed(ScriptState*) const;

  // IDL functions
  void close(WebSocketCloseInfo*, ExceptionState&);

  // Implementation of WebSocketChannelClient
  void DidConnect(const String& subprotocol, const String& extensions) override;
  void DidReceiveTextMessage(const String&) override;
  void DidReceiveBinaryMessage(
      const Vector<base::span<const char>>& data) override;
  void DidError() override;
  void DidConsumeBufferedAmount(uint64_t consumed) override;
  void DidStartClosingHandshake() override;
  void DidClose(ClosingHandshakeCompletionStatus,
                uint16_t /* code */,
                const String& /* reason */) override;

  // Implementation of ExecutionContextLifecycleObserver.
  void ContextDestroyed() override;

  // Implementation of ActiveScriptWrappable.
  bool HasPendingActivity() const override;

  void Trace(Visitor*) const override;

 private:
  class UnderlyingSource;
  class UnderlyingSink;

  static WebSocketStream* CreateInternal(ScriptState*,
                                         const String& url,
                                         WebSocketStreamOptions*,
                                         WebSocketChannel*,
                                         ExceptionState&);
  void Connect(ScriptState*,
               const String& url,
               WebSocketStreamOptions*,
               ExceptionState&);

  // Closes the connection. If |maybe_reason| is an object with a valid "code"
  // property and optionally a valid "reason" property, will use them as the
  // code and reason, otherwise will close with unspecified close.
  void CloseMaybeWithReason(ScriptValue maybe_reason, ExceptionState&);

  void CloseWithUnspecifiedCode(ExceptionState&);
  void CloseInternal(std::optional<uint16_t> code,
                     const String& reason,
                     ExceptionState& exception_state);
  void OnAbort();

  // Create a WebSocketError with the supplied arguments.
  v8::Local<v8::Value> CreateWebSocketError(
      String message,
      std::optional<uint16_t> close_code = std::nullopt,
      String reason = String());
  static WebSocketCloseInfo* MakeCloseInfo(uint16_t close_code,
                                           const String& reason);

  const Member<ScriptState> script_state_;
  const Member<ScriptPromiseProperty<WebSocketOpenInfo, IDLAny>> opened_;
  const Member<ScriptPromiseProperty<WebSocketCloseInfo, IDLAny>> closed_;

  Member<WebSocketChannel> channel_;

  Member<UnderlyingSource> source_;
  Member<UnderlyingSink> sink_;

  Member<AbortSignal::AlgorithmHandle> abort_handle_;

  WebSocketCommon common_;

  // We need to distinguish between "closing during handshake" and "closing
  // after handshake" in order to reject the |opened_resolver_| correctly.
  bool was_ever_connected_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBSOCKETS_WEBSOCKET_STREAM_H_
