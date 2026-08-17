// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PUSH_MESSAGING_PUSH_SUBSCRIPTION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PUSH_MESSAGING_PUSH_SUBSCRIPTION_H_

#include <optional>

#include "base/gtest_prod_util.h"
#include "base/memory/scoped_refptr.h"
#include "third_party/blink/public/mojom/push_messaging/push_messaging.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/idl_types.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/core/dom/dom_time_stamp.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_array_buffer.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class PushSubscriptionOptions;
class ServiceWorkerRegistration;
class ScriptState;
class V8PushEncryptionKeyName;

class MODULES_EXPORT PushSubscription final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static PushSubscription* Create(
      mojom::blink::PushSubscriptionPtr subscription,
      ServiceWorkerRegistration* service_worker_registration);

  PushSubscription(const KURL& endpoint,
                   bool user_visible_only,
                   const WTF::Vector<uint8_t>& application_server_key,
                   const WTF::Vector<unsigned char>& p256dh,
                   const WTF::Vector<unsigned char>& auth,
                   const std::optional<DOMTimeStamp>& expiration_time,
                   ServiceWorkerRegistration* service_worker_registration);

  ~PushSubscription() override;

  KURL endpoint() const { return endpoint_; }
  std::optional<DOMTimeStamp> expirationTime() const;

  PushSubscriptionOptions* options() const { return options_.Get(); }

  DOMArrayBuffer* getKey(const V8PushEncryptionKeyName& name) const;
  ScriptPromise<IDLBoolean> unsubscribe(ScriptState* script_state);

  ScriptObject toJSONForBinding(ScriptState* script_state);

  void Trace(Visitor* visitor) const override;

 private:
  FRIEND_TEST_ALL_PREFIXES(PushSubscriptionTest,
                           SerializesToBase64URLWithoutPadding);

  KURL endpoint_;

  Member<PushSubscriptionOptions> options_;

  Member<DOMArrayBuffer> p256dh_;
  Member<DOMArrayBuffer> auth_;

  std::optional<DOMTimeStamp> expiration_time_;

  Member<ServiceWorkerRegistration> service_worker_registration_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PUSH_MESSAGING_PUSH_SUBSCRIPTION_H_
