// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TYPED_ARRAYS_DOM_ARRAY_BUFFER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TYPED_ARRAYS_DOM_ARRAY_BUFFER_H_

#include <algorithm>

#include "base/compiler_specific.h"
#include "base/containers/span.h"
#include "partition_alloc/oom.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/typed_arrays/array_buffer/array_buffer_contents.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_array_buffer_base.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class ExceptionState;

class CORE_EXPORT DOMArrayBuffer : public DOMArrayBufferBase {
  DEFINE_WRAPPERTYPEINFO();
  static const WrapperTypeInfo wrapper_type_info_body_;

 public:
  static DOMArrayBuffer* Create(ArrayBufferContents contents) {
    return MakeGarbageCollected<DOMArrayBuffer>(std::move(contents));
  }
  static DOMArrayBuffer* Create(size_t num_elements, size_t element_byte_size) {
    ArrayBufferContents contents(
        num_elements, element_byte_size, ArrayBufferContents::kNotShared,
        ArrayBufferContents::kZeroInitialize,
        ArrayBufferContents::AllocationFailureBehavior::kCrash);
    CHECK(contents.IsValid());
    return Create(std::move(contents));
  }
  static DOMArrayBuffer* Create(base::span<const uint8_t> source) {
    ArrayBufferContents contents(
        source.size(), 1, ArrayBufferContents::kNotShared,
        ArrayBufferContents::kDontInitialize,
        ArrayBufferContents::AllocationFailureBehavior::kCrash);
    CHECK(contents.IsValid());
    contents.ByteSpan().copy_from(source);
    return Create(std::move(contents));
  }
  // TODO(tsepez): should be declared UNSAFE_BUFFER_USAGE.
  static DOMArrayBuffer* Create(const void* source, size_t byte_length) {
    // SAFETY: Caller guarantees that `source` contains `byte_length` bytes.
    return Create(UNSAFE_BUFFERS(
        base::span(static_cast<const uint8_t*>(source), byte_length)));
  }

  static DOMArrayBuffer* Create(scoped_refptr<SharedBuffer>);
  static DOMArrayBuffer* Create(const Vector<base::span<const char>>&);

  static DOMArrayBuffer* CreateOrNull(size_t num_elements,
                                      size_t element_byte_size);
  static DOMArrayBuffer* CreateOrNull(base::span<const uint8_t> source);
  // TODO(tsepez): should be declared UNSAFE_BUFFER_USAGE.
  static DOMArrayBuffer* CreateOrNull(const void* source, size_t byte_length) {
    // SAFETY: Caller guarantees that `source` contains `byte_length` bytes.
    return CreateOrNull(UNSAFE_BUFFERS(
        base::span(static_cast<const uint8_t*>(source), byte_length)));
  }

  // Only for use by XMLHttpRequest::responseArrayBuffer,
  // Internals::serializeObject, and
  // FetchDataLoaderAsArrayBuffer::OnStateChange.
  static DOMArrayBuffer* CreateUninitializedOrNull(size_t num_elements,
                                                   size_t element_byte_size);

  explicit DOMArrayBuffer(ArrayBufferContents contents)
      : DOMArrayBufferBase(std::move(contents)) {}

  DOMArrayBuffer* Slice(size_t begin, size_t end) const;

  bool IsDetachable(v8::Isolate*);

  void SetDetachKey(v8::Isolate*, const StringView& detach_key);

  // Transfer the ArrayBuffer with |detach_key| if it is detachable,
  // otherwise make a copy and transfer that. Rethrows a V8 exception
  // or a TypeError on failure.
  bool Transfer(v8::Isolate*,
                v8::Local<v8::Value> detach_key,
                ArrayBufferContents& result,
                ExceptionState& exception_state);

  bool Transfer(v8::Isolate*,
                ArrayBufferContents& result,
                ExceptionState& exception_state);

  // Share the ArrayBuffer, even if it is non-shared. Such sharing is necessary
  // for e.g. WebAudio and WebCodecs which use a separate thread for processing
  // the ArrayBuffer while at the same time exposing a NonShared TypedArray.
  virtual bool ShareNonSharedForInternalUse(ArrayBufferContents& result);

  v8::Local<v8::Value> Wrap(ScriptState*) override;

  void Trace(Visitor*) const override;

  bool IsDetached() const override;

  v8::Local<v8::Object> AssociateWithWrapper(
      v8::Isolate* isolate,
      const WrapperTypeInfo* wrapper_type_info,
      v8::Local<v8::Object> wrapper) override;

  bool has_non_main_world_wrappers() const {
    return has_non_main_world_wrappers_;
  }

 private:
  v8::Maybe<bool> TransferDetachable(v8::Isolate*,
                                     v8::Local<v8::Value> detach_key,
                                     ArrayBufferContents& result);

  // Detach key can be any ECMAScript value (i.e. v8::Value), however, we don't
  // want to use a v8::Context-sensitive detach key like v8::Object. So, we
  // support only v8::String as the detach key type. It's also convenient that
  // we can write `array_buffer->SetDetachKey(isolate, "my key")`.
  TraceWrapperV8Reference<v8::String> detach_key_;

  bool has_non_main_world_wrappers_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TYPED_ARRAYS_DOM_ARRAY_BUFFER_H_
