// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TYPED_ARRAYS_DOM_SHARED_ARRAY_BUFFER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TYPED_ARRAYS_DOM_SHARED_ARRAY_BUFFER_H_

#include "base/compiler_specific.h"
#include "partition_alloc/oom.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/typed_arrays/array_buffer/array_buffer_contents.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_array_buffer_base.h"

namespace blink {

class CORE_EXPORT DOMSharedArrayBuffer final : public DOMArrayBufferBase {
  DEFINE_WRAPPERTYPEINFO();
  static const WrapperTypeInfo wrapper_type_info_body_;

 public:
  static DOMSharedArrayBuffer* Create(ArrayBufferContents contents) {
    DCHECK(contents.IsShared());
    return MakeGarbageCollected<DOMSharedArrayBuffer>(std::move(contents));
  }

  static DOMSharedArrayBuffer* Create(unsigned num_elements,
                                      unsigned element_byte_size) {
    ArrayBufferContents contents(
        num_elements, element_byte_size, ArrayBufferContents::kShared,
        ArrayBufferContents::kZeroInitialize,
        ArrayBufferContents::AllocationFailureBehavior::kCrash);
    CHECK(contents.IsValid());
    return Create(std::move(contents));
  }

  static DOMSharedArrayBuffer* Create(const void* source,
                                      unsigned byte_length) {
    ArrayBufferContents contents(
        byte_length, 1, ArrayBufferContents::kShared,
        ArrayBufferContents::kDontInitialize,
        ArrayBufferContents::AllocationFailureBehavior::kCrash);
    CHECK(contents.IsValid());
    UNSAFE_TODO(memcpy(contents.DataShared(), source, byte_length));
    return Create(std::move(contents));
  }

  explicit DOMSharedArrayBuffer(ArrayBufferContents contents)
      : DOMArrayBufferBase(std::move(contents)) {}

  bool ShareContentsWith(ArrayBufferContents& result) {
    if (!Content()->BackingStore()) {
      result.Detach();
      return false;
    }
    Content()->ShareWith(result);
    return true;
  }

  v8::Local<v8::Value> Wrap(ScriptState*) override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TYPED_ARRAYS_DOM_SHARED_ARRAY_BUFFER_H_
