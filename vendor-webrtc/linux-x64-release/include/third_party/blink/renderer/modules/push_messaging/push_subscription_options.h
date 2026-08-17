// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PUSH_MESSAGING_PUSH_SUBSCRIPTION_OPTIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PUSH_MESSAGING_PUSH_SUBSCRIPTION_OPTIONS_H_

#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class DOMArrayBuffer;
class ExceptionState;
class PushSubscriptionOptionsInit;

class PushSubscriptionOptions final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Converts developer-provided dictionary to PushSubscriptionOptions.
  // Throws if applicationServerKey is invalid.
  static MODULES_EXPORT PushSubscriptionOptions* FromOptionsInit(
      const PushSubscriptionOptionsInit* options_init,
      ExceptionState& exception_state);

  explicit PushSubscriptionOptions(
      bool user_visible_only,
      const WTF::Vector<uint8_t>& application_server_key);

  bool userVisibleOnly() const { return user_visible_only_; }

  // Mutable by web developer. See https://github.com/w3c/push-api/issues/198.
  DOMArrayBuffer* applicationServerKey() const {
    return application_server_key_.Get();
  }

  void Trace(Visitor* visitor) const override;

 private:
  bool user_visible_only_;
  Member<DOMArrayBuffer> application_server_key_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PUSH_MESSAGING_PUSH_SUBSCRIPTION_OPTIONS_H_
