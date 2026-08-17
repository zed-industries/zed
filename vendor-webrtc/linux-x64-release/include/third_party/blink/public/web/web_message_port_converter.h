// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_MESSAGE_PORT_CONVERTER_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_MESSAGE_PORT_CONVERTER_H_

#include <optional>

#include "third_party/blink/public/platform/web_common.h"
#include "v8/include/v8-local-handle.h"

namespace v8 {
class Isolate;
class Value;
}  // namespace v8

namespace blink {
class MessagePortChannel;

class BLINK_EXPORT WebMessagePortConverter {
 public:
  // Disentangle and extract a MessagePortChannel from a v8 wrapper of
  // MessagePort. If the wrapper is not MessagePort or the MessagePort is
  // neutered, it will return nullopt.
  static std::optional<MessagePortChannel>
  DisentangleAndExtractMessagePortChannel(v8::Isolate*, v8::Local<v8::Value>);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_MESSAGE_PORT_CONVERTER_H_
