// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_MODULES_V8_SERIALIZATION_SERIALIZED_SCRIPT_VALUE_FOR_MODULES_FACTORY_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_MODULES_V8_SERIALIZATION_SERIALIZED_SCRIPT_VALUE_FOR_MODULES_FACTORY_H_

#include "third_party/blink/renderer/bindings/core/v8/serialization/serialized_script_value_factory.h"

namespace blink {

class SerializedScriptValueForModulesFactory final
    : public SerializedScriptValueFactory {
  USING_FAST_MALLOC(SerializedScriptValueForModulesFactory);

 public:
  SerializedScriptValueForModulesFactory() : SerializedScriptValueFactory() {}

  SerializedScriptValueForModulesFactory(
      const SerializedScriptValueForModulesFactory&) = delete;
  SerializedScriptValueForModulesFactory& operator=(
      const SerializedScriptValueForModulesFactory&) = delete;

 protected:
  bool ExtractTransferable(v8::Isolate*,
                           v8::Local<v8::Value>,
                           wtf_size_t,
                           Transferables&,
                           ExceptionState&) override;

  scoped_refptr<SerializedScriptValue> Create(
      v8::Isolate*,
      v8::Local<v8::Value>,
      const SerializedScriptValue::SerializeOptions&,
      ExceptionState&) override;

  v8::Local<v8::Value> Deserialize(
      scoped_refptr<SerializedScriptValue>,
      v8::Isolate*,
      const SerializedScriptValue::DeserializeOptions&) override;

  v8::Local<v8::Value> Deserialize(
      UnpackedSerializedScriptValue*,
      v8::Isolate*,
      const SerializedScriptValue::DeserializeOptions&) override;

  bool ExecutionContextExposesInterface(ExecutionContext*,
                                        SerializationTag) override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_MODULES_V8_SERIALIZATION_SERIALIZED_SCRIPT_VALUE_FOR_MODULES_FACTORY_H_
