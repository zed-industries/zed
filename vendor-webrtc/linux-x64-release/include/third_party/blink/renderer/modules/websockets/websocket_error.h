// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBSOCKETS_WEBSOCKET_ERROR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBSOCKETS_WEBSOCKET_ERROR_H_

#include <stdint.h>

#include <optional>

#include "base/types/pass_key.h"
#include "third_party/blink/renderer/core/dom/dom_exception.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "v8/include/v8.h"

namespace blink {

class WebSocketCloseInfo;

// https://websocket.spec.whatwg.org/#websocket-error-interface
class MODULES_EXPORT WebSocketError : public DOMException {
  DEFINE_WRAPPERTYPEINFO();

 public:
  using PassKey = base::PassKey<WebSocketError>;

  // Constructor exposed to script. Called by the V8 bindings.
  static WebSocketError* Create(String message,
                                const WebSocketCloseInfo*,
                                ExceptionState&);

  // For creating a WebSocketError from C++. Attaches a stack trace to the
  // created object. Typically this will be immediately passed to
  // ScriptPromiseResolverBase::Reject.
  static v8::Local<v8::Value> Create(v8::Isolate*,
                                     String message,
                                     std::optional<uint16_t> close_code,
                                     String reason);

  // Use one of the Create() methods instead. This constructor has to be public
  // so that it can be used with MakeGarbageCollected<> inside the
  // ValidateAndCreate() method.
  WebSocketError(PassKey,
                 String message,
                 std::optional<uint16_t> close_code,
                 String reason);
  ~WebSocketError() override;

  std::optional<uint16_t> closeCode() const { return close_code_; }

  String reason() const { return reason_; }

 private:
  static WebSocketError* ValidateAndCreate(String message,
                                           std::optional<uint16_t> close_code,
                                           String reason,
                                           ExceptionState& exception_state);

  const std::optional<uint16_t> close_code_;
  const String reason_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBSOCKETS_WEBSOCKET_ERROR_H_
