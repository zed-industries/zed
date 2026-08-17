// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SERIALIZATION_UNPACKED_SERIALIZED_SCRIPT_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SERIALIZATION_UNPACKED_SERIALIZED_SCRIPT_VALUE_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/bindings/core/v8/serialization/serialized_script_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class DOMArrayBufferBase;
class ImageBitmap;

// Represents the "unpacked" materialized objects created after receiving the
// transferred contents of a SerializedScriptValue, e.g. from another thread.
//
// These contents should not (or cannot) be copied, but must be attached to a
// non-thread-safe object for use. For instance, an ArrayBuffer must be created
// on the heap to own the array buffer contents, create a JavaScript wrapper
// object, and so on.
//
// This "unpacking" can only be done once, and the resulting state must be
// manipulated and ultimately collected on the thread on which it was unpacked.
//
// However, these unpacked objects aren't necessarily transient either. For
// instance, the data of a MessageEvent might be requested in different isolated
// worlds, for which one object underlying each transferred object must exist,
// but (for security reasons) separate JavaScript wrappers must exist. For this
// reason, a SerializedScriptValue can only be unpacked once, but thereafter it
// can be deserialized multiple times.
class CORE_EXPORT UnpackedSerializedScriptValue final
    : public GarbageCollected<UnpackedSerializedScriptValue> {
 public:
  // Callers should use SerializedScriptValue::Unpack.
  explicit UnpackedSerializedScriptValue(scoped_refptr<SerializedScriptValue>);
  ~UnpackedSerializedScriptValue();

  void Trace(Visitor*) const;

  SerializedScriptValue* Value() { return value_.get(); }
  const SerializedScriptValue* Value() const { return value_.get(); }

  const HeapVector<Member<DOMArrayBufferBase>>& ArrayBuffers() const {
    return array_buffers_;
  }
  const HeapVector<Member<ImageBitmap>>& ImageBitmaps() const {
    return image_bitmaps_;
  }

  using DeserializeOptions = SerializedScriptValue::DeserializeOptions;
  v8::Local<v8::Value> Deserialize(
      v8::Isolate*,
      const DeserializeOptions& = DeserializeOptions());

 private:
  // The underlying serialized data.
  scoped_refptr<SerializedScriptValue> value_;

  // These replace their corresponding members in SerializedScriptValue, once
  // set. Once the value is being deserialized, objects will be materialized
  // here.
  HeapVector<Member<DOMArrayBufferBase>> array_buffers_;
  HeapVector<Member<ImageBitmap>> image_bitmaps_;

  friend class SerializedScriptValue;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SERIALIZATION_UNPACKED_SERIALIZED_SCRIPT_VALUE_H_
