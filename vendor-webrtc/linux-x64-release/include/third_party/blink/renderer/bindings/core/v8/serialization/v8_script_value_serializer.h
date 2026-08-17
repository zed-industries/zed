// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SERIALIZATION_V8_SCRIPT_VALUE_SERIALIZER_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SERIALIZATION_V8_SCRIPT_VALUE_SERIALIZER_H_

#include "base/dcheck_is_on.h"
#include "base/memory/scoped_refptr.h"
#include "base/types/pass_key.h"
#include "third_party/blink/renderer/bindings/core/v8/serialization/serialization_tag.h"
#include "third_party/blink/renderer/bindings/core/v8/serialization/serialized_params.h"
#include "third_party/blink/renderer/bindings/core/v8/serialization/serialized_script_value.h"
#include "third_party/blink/renderer/bindings/core/v8/serialization/trailer_writer.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "v8/include/v8.h"

namespace blink {

class File;
class Transferables;
class ScriptState;

// Serializes V8 values according to the HTML structured clone algorithm:
// https://html.spec.whatwg.org/C/#structured-clone
//
// Supports only basic JavaScript objects and core DOM types. Support for
// modules types is implemented in a subclass.
//
// A serializer cannot be used multiple times; it is expected that its serialize
// method will be invoked exactly once.
class CORE_EXPORT V8ScriptValueSerializer
    : public v8::ValueSerializer::Delegate {
  STACK_ALLOCATED();

 public:
  using Options = SerializedScriptValue::SerializeOptions;
  using PassKey = base::PassKey<V8ScriptValueSerializer>;

  // |object_index| is for use in exceptiun messages.
  static bool ExtractTransferable(v8::Isolate*,
                                  v8::Local<v8::Value>,
                                  wtf_size_t object_index,
                                  Transferables&,
                                  ExceptionState&);

  explicit V8ScriptValueSerializer(ScriptState*, const Options& = Options());

  V8ScriptValueSerializer(const V8ScriptValueSerializer&) = delete;
  V8ScriptValueSerializer& operator=(const V8ScriptValueSerializer&) = delete;

  scoped_refptr<SerializedScriptValue> Serialize(v8::Local<v8::Value>,
                                                 ExceptionState&);

 protected:
  // Returns true if the DOM object was successfully written.
  // If false is returned and no more specific exception is thrown, a generic
  // DataCloneError message will be used.
  virtual bool WriteDOMObject(ScriptWrappable*, ExceptionState&);

  ScriptState* GetScriptState() const { return script_state_; }

  void WriteTag(SerializationTag tag) {
    uint8_t tag_byte = tag;
    serializer_.WriteRawBytes(&tag_byte, 1);
  }
  void WriteUint32(uint32_t value) { serializer_.WriteUint32(value); }
  void WriteUint64(uint64_t value) { serializer_.WriteUint64(value); }
  void WriteDouble(double value) { serializer_.WriteDouble(value); }
  void WriteRawBytes(const void* data, size_t size) {
    serializer_.WriteRawBytes(data, size);
  }
  void WriteUnguessableToken(const base::UnguessableToken& token);
  void WriteUTF8String(const StringView&);

  void WriteAndRequireInterfaceTag(SerializationTag tag) {
    GetTrailerWriter().RequireExposedInterface(tag);
    WriteTag(tag);
  }

  template <typename E>
  void WriteUint32Enum(E value) {
    static_assert(
        std::is_enum<E>::value &&
            std::is_same<uint32_t,
                         typename std::underlying_type<E>::type>::value,
        "Only enums backed by uint32_t are accepted.");
    WriteUint32(static_cast<uint32_t>(value));
  }

  SerializedScriptValue* GetSerializedScriptValue() {
    return serialized_script_value_.get();
  }

  bool IsForStorage() const { return for_storage_; }

  const Transferables* GetTransferables() const { return transferables_; }

  TrailerWriter& GetTrailerWriter() { return trailer_writer_; }

 private:
  // Transfer is split into two phases: scanning the transferables so that we
  // don't have to serialize the data (just an index), and finalizing (to
  // neuter objects in the source context).
  // This separation is required by the spec (it prevents neutering from
  // happening if there's a failure earlier in serialization).
  void PrepareTransfer(ExceptionState&);
  void FinalizeTransfer(ExceptionState&);

  // Shared between File and FileList logic; does not write a leading tag.
  bool WriteFile(File*, ExceptionState&);

  // v8::ValueSerializer::Delegate
  void ThrowDataCloneError(v8::Local<v8::String> message) override;

  bool HasCustomHostObject(v8::Isolate* isolate) override { return true; }
  v8::Maybe<bool> IsHostObject(v8::Isolate* isolate,
                               v8::Local<v8::Object> object) override;
  v8::Maybe<bool> WriteHostObject(v8::Isolate*,
                                  v8::Local<v8::Object> message) override;
  v8::Maybe<uint32_t> GetSharedArrayBufferId(
      v8::Isolate*,
      v8::Local<v8::SharedArrayBuffer>) override;

  v8::Maybe<uint32_t> GetWasmModuleTransferId(
      v8::Isolate*,
      v8::Local<v8::WasmModuleObject>) override;
  // Reallocates memory at |ptr| to the new size and returns the new pointer or
  // nullptr on failure. |actual_size| will hold the actual size of allocation
  // requested.
  void* ReallocateBufferMemory(void* old_buffer,
                               size_t,
                               size_t* actual_size) override;
  void FreeBufferMemory(void* buffer) override;

  bool AdoptSharedValueConveyor(v8::Isolate* isolate,
                                v8::SharedValueConveyor&& conveyor) override;

  ScriptState* script_state_;
  scoped_refptr<SerializedScriptValue> serialized_script_value_;
  v8::ValueSerializer serializer_;
  TrailerWriter trailer_writer_;
  const Transferables* transferables_ = nullptr;
  WebBlobInfoArray* blob_info_array_ = nullptr;
  SharedArrayBufferArray shared_array_buffers_;
  Options::WasmSerializationPolicy wasm_policy_;
  bool for_storage_ = false;
#if DCHECK_IS_ON()
  bool serialize_invoked_ = false;
#endif
};

// For code testing V8ScriptValueSerializer. Behaves the same as
// SerializedScriptValue::Create, except it can be called on an initializer
// list.
scoped_refptr<SerializedScriptValue> SerializedValue(
    const Vector<uint8_t>& bytes);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SERIALIZATION_V8_SCRIPT_VALUE_SERIALIZER_H_
