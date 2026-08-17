// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_EXTENDABLE_MESSAGE_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_EXTENDABLE_MESSAGE_EVENT_H_

#include "third_party/blink/renderer/bindings/core/v8/serialization/serialized_script_value.h"
#include "third_party/blink/renderer/bindings/core/v8/serialization/transferables.h"
#include "third_party/blink/renderer/bindings/core/v8/world_safe_v8_reference.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_extendable_message_event_init.h"
#include "third_party/blink/renderer/modules/event_modules.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/service_worker/extendable_event.h"

namespace blink {

class MessagePort;
class ServiceWorker;
class ServiceWorkerClient;
class V8UnionClientOrMessagePortOrServiceWorker;

class MODULES_EXPORT ExtendableMessageEvent final : public ExtendableEvent {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static ExtendableMessageEvent* Create(
      const AtomicString& type,
      const ExtendableMessageEventInit* initializer);
  static ExtendableMessageEvent* Create(
      scoped_refptr<SerializedScriptValue> data,
      const String& origin,
      GCedMessagePortArray* ports,
      ServiceWorkerClient* source,
      WaitUntilObserver*);
  static ExtendableMessageEvent* Create(
      scoped_refptr<SerializedScriptValue> data,
      const String& origin,
      GCedMessagePortArray* ports,
      ServiceWorker* source,
      WaitUntilObserver*);
  static ExtendableMessageEvent* CreateError(const String& origin,
                                             GCedMessagePortArray* ports,
                                             ServiceWorkerClient* source,
                                             WaitUntilObserver*);
  static ExtendableMessageEvent* CreateError(const String& origin,
                                             GCedMessagePortArray* ports,
                                             ServiceWorker* source,
                                             WaitUntilObserver*);

  ExtendableMessageEvent(const AtomicString& type,
                         const ExtendableMessageEventInit* initializer);
  ExtendableMessageEvent(const AtomicString& type,
                         const ExtendableMessageEventInit* initializer,
                         WaitUntilObserver*);
  ExtendableMessageEvent(scoped_refptr<SerializedScriptValue> data,
                         const String& origin,
                         GCedMessagePortArray* ports,
                         WaitUntilObserver*);
  // Creates a 'messageerror' event.
  ExtendableMessageEvent(const String& origin,
                         GCedMessagePortArray* ports,
                         WaitUntilObserver*);

  SerializedScriptValue* SerializedData() const {
    return serialized_data_.get();
  }
  void SetSerializedData(scoped_refptr<SerializedScriptValue> serialized_data) {
    serialized_data_ = std::move(serialized_data);
  }

  ScriptValue data(ScriptState* script_state) const;
  bool isDataDirty() const { return false; }
  const String& origin() const { return origin_; }
  const String& lastEventId() const { return last_event_id_; }
  V8UnionClientOrMessagePortOrServiceWorker* source() const;
  MessagePortArray ports() const;

  const AtomicString& InterfaceName() const override;

  void Trace(Visitor*) const override;

 private:
  scoped_refptr<SerializedScriptValue> serialized_data_;
  WorldSafeV8Reference<v8::Value> data_;
  String origin_;
  String last_event_id_;
  Member<ServiceWorkerClient> source_as_client_;
  Member<ServiceWorker> source_as_service_worker_;
  Member<MessagePort> source_as_message_port_;
  Member<GCedMessagePortArray> ports_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_EXTENDABLE_MESSAGE_EVENT_H_
