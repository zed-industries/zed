// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TYPED_ARRAYS_DOM_DATA_VIEW_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TYPED_ARRAYS_DOM_DATA_VIEW_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_array_buffer_view.h"

namespace blink {

class CORE_EXPORT DOMDataView final : public DOMArrayBufferView {
  DEFINE_WRAPPERTYPEINFO();
  static const WrapperTypeInfo wrapper_type_info_body_;

 public:
  typedef char ValueType;

  static DOMDataView* Create(DOMArrayBufferBase*,
                             size_t byte_offset,
                             size_t byte_length);

  DOMDataView(DOMArrayBufferBase* dom_array_buffer,
              size_t byte_offset,
              size_t byte_length);

  v8::Local<v8::Value> Wrap(ScriptState*) override;

  size_t byteLength() const final {
    return !IsDetached() ? raw_byte_length_ : 0;
  }

  // DOMDataView is a byte array, therefore each field has size 1.
  unsigned TypeSize() const final { return 1; }

  DOMArrayBufferView::ViewType GetType() const final { return kTypeDataView; }

 private:
  // It may be stale after Detach. Use ByteLength instead.
  size_t raw_byte_length_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TYPED_ARRAYS_DOM_DATA_VIEW_H_
