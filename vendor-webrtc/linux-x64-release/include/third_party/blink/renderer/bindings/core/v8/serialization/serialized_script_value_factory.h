// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SERIALIZATION_SERIALIZED_SCRIPT_VALUE_FACTORY_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SERIALIZATION_SERIALIZED_SCRIPT_VALUE_FACTORY_H_

#include "base/notreached.h"
#include "third_party/blink/renderer/bindings/core/v8/serialization/serialization_tag.h"
#include "third_party/blink/renderer/bindings/core/v8/serialization/serialized_script_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class CORE_EXPORT SerializedScriptValueFactory {
  USING_FAST_MALLOC(SerializedScriptValueFactory);

 public:
  SerializedScriptValueFactory(const SerializedScriptValueFactory&) = delete;
  SerializedScriptValueFactory& operator=(const SerializedScriptValueFactory&) =
      delete;

  // SerializedScriptValueFactory::initialize() should be invoked when Blink is
  // initialized, i.e. initialize() in WebKit.cpp.
  static void Initialize(SerializedScriptValueFactory* new_factory) {
    DCHECK(!instance_);
    instance_ = new_factory;
  }

 protected:
  friend class SerializedScriptValue;
  friend class UnpackedSerializedScriptValue;

  // Following methods are expected to be called by SerializedScriptValue.
  // |object_index| is for use in exception messages.
  virtual bool ExtractTransferable(v8::Isolate*,
                                   v8::Local<v8::Value>,
                                   wtf_size_t object_index,
                                   Transferables&,
                                   ExceptionState&);

  virtual scoped_refptr<SerializedScriptValue> Create(
      v8::Isolate*,
      v8::Local<v8::Value>,
      const SerializedScriptValue::SerializeOptions&,
      ExceptionState&);

  virtual v8::Local<v8::Value> Deserialize(
      scoped_refptr<SerializedScriptValue>,
      v8::Isolate*,
      const SerializedScriptValue::DeserializeOptions&);

  virtual v8::Local<v8::Value> Deserialize(
      UnpackedSerializedScriptValue*,
      v8::Isolate*,
      const SerializedScriptValue::DeserializeOptions&);

  virtual bool ExecutionContextExposesInterface(ExecutionContext*,
                                                SerializationTag);

  // Following methods are expected to be called in
  // SerializedScriptValueFactory{ForModules}.
  SerializedScriptValueFactory() = default;

 private:
  static SerializedScriptValueFactory& Instance() {
    if (!instance_) {
      NOTREACHED();
    }
    return *instance_;
  }

  static SerializedScriptValueFactory* instance_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SERIALIZATION_SERIALIZED_SCRIPT_VALUE_FACTORY_H_
