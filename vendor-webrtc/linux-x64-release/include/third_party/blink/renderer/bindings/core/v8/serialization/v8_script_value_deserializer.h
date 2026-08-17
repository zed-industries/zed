// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SERIALIZATION_V8_SCRIPT_VALUE_DESERIALIZER_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SERIALIZATION_V8_SCRIPT_VALUE_DESERIALIZER_H_

#include "base/dcheck_is_on.h"
#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/bindings/core/v8/serialization/serialization_tag.h"
#include "third_party/blink/renderer/bindings/core/v8/serialization/serialized_params.h"
#include "third_party/blink/renderer/bindings/core/v8/serialization/serialized_script_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "v8/include/v8.h"

namespace blink {

class DOMRectReadOnly;
class ExceptionState;
class File;
class UnpackedSerializedScriptValue;
class ScriptState;

// Deserializes V8 values serialized using V8ScriptValueSerializer (or its
// predecessor, ScriptValueSerializer).
//
// Supports only basic JavaScript objects and core DOM types. Support for
// modules types is implemented in a subclass.
//
// A deserializer cannot be used multiple times; it is expected that its
// deserialize method will be invoked exactly once.
class CORE_EXPORT V8ScriptValueDeserializer
    : public v8::ValueDeserializer::Delegate {
  STACK_ALLOCATED();

 public:
  using Options = SerializedScriptValue::DeserializeOptions;
  V8ScriptValueDeserializer(ScriptState*,
                            UnpackedSerializedScriptValue*,
                            const Options& = Options());
  V8ScriptValueDeserializer(ScriptState*,
                            scoped_refptr<SerializedScriptValue>,
                            const Options& = Options());

  V8ScriptValueDeserializer(const V8ScriptValueDeserializer&) = delete;
  V8ScriptValueDeserializer& operator=(const V8ScriptValueDeserializer&) =
      delete;

  v8::Local<v8::Value> Deserialize();

  static bool ExecutionContextExposesInterface(ExecutionContext*,
                                               SerializationTag interface_tag);

 protected:
  virtual ScriptWrappable* ReadDOMObject(SerializationTag, ExceptionState&);

  ScriptState* GetScriptState() const { return script_state_; }

  uint32_t Version() const { return version_; }
  bool ReadTag(SerializationTag* tag) {
    const void* tag_bytes = nullptr;
    if (!deserializer_.ReadRawBytes(1, &tag_bytes))
      return false;
    *tag = static_cast<SerializationTag>(
        *reinterpret_cast<const uint8_t*>(tag_bytes));
    return true;
  }
  bool ReadUint32(uint32_t* value) { return deserializer_.ReadUint32(value); }
  bool ReadUint64(uint64_t* value) { return deserializer_.ReadUint64(value); }
  bool ReadDouble(double* value) { return deserializer_.ReadDouble(value); }
  bool ReadRawBytes(size_t size, const void** data) {
    return deserializer_.ReadRawBytes(size, data);
  }
  bool ReadRawBytesToSpan(size_t size, base::span<const uint8_t>* out_span) {
    const void* data = nullptr;
    if (!deserializer_.ReadRawBytes(size, &data)) {
      return false;
    }
    // SAFETY: ReadRawBytes() ensures `data` and `size` are safe.
    *out_span = UNSAFE_BUFFERS(
        base::span(reinterpret_cast<const uint8_t*>(data), size));
    return true;
  }
  bool ReadUnguessableToken(base::UnguessableToken* token_out);
  bool ReadUTF8String(String* string_out);
  DOMRectReadOnly* ReadDOMRectReadOnly();

  template <typename E>
  bool ReadUint32Enum(E* value) {
    static_assert(
        std::is_enum<E>::value &&
            std::is_same<uint32_t,
                         typename std::underlying_type<E>::type>::value,
        "Only enums backed by uint32_t are accepted.");
    uint32_t tmp;
    if (ReadUint32(&tmp) && tmp <= static_cast<uint32_t>(E::kLast)) {
      *value = static_cast<E>(tmp);
      return true;
    }
    return false;
  }

  SerializedScriptValue* GetSerializedScriptValue() {
    return serialized_script_value_.get();
  }

 private:
  V8ScriptValueDeserializer(ScriptState*,
                            UnpackedSerializedScriptValue*,
                            scoped_refptr<SerializedScriptValue>,
                            const Options&);
  void Transfer();

  File* ReadFile();
  File* ReadFileIndex();

  scoped_refptr<BlobDataHandle> GetBlobDataHandle(const String& uuid);

  // v8::ValueDeserializer::Delegate
  v8::MaybeLocal<v8::Object> ReadHostObject(v8::Isolate*) override;
  v8::MaybeLocal<v8::WasmModuleObject> GetWasmModuleFromId(v8::Isolate*,
                                                           uint32_t) override;
  v8::MaybeLocal<v8::SharedArrayBuffer> GetSharedArrayBufferFromId(
      v8::Isolate*,
      uint32_t) override;
  const v8::SharedValueConveyor* GetSharedValueConveyor(v8::Isolate*) override;

  ScriptState* script_state_;
  UnpackedSerializedScriptValue* unpacked_value_;
  scoped_refptr<SerializedScriptValue> serialized_script_value_;
  v8::ValueDeserializer deserializer_;

  // Message ports which were transferred in.
  const MessagePortArray* transferred_message_ports_ = nullptr;

  Vector<SerializedScriptValue::Stream> streams_;

  // Blob info for blobs stored by index.
  const WebBlobInfoArray* blob_info_array_ = nullptr;

  // Set during deserialize after the header is read.
  uint32_t version_ = 0;

#if DCHECK_IS_ON()
  bool deserialize_invoked_ = false;
#endif
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SERIALIZATION_V8_SCRIPT_VALUE_DESERIALIZER_H_
