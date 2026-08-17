// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_FILE_SYSTEM_ACCESS_ALLOW_SHARED_BUFFER_SOURCE_UTIL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_FILE_SYSTEM_ACCESS_ALLOW_SHARED_BUFFER_SOURCE_UTIL_H_

#include "base/containers/span.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_union_arraybufferallowshared_arraybufferviewallowshared.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_array_buffer.h"

namespace blink {

using AllowSharedBufferSource =
    V8UnionArrayBufferAllowSharedOrArrayBufferViewAllowShared;

// Helper function for turning various DOMArray-like things into a span.
inline base::span<uint8_t> AsByteSpan(
    const AllowSharedBufferSource& buffer_union) {
  switch (buffer_union.GetContentType()) {
    case AllowSharedBufferSource::ContentType::kArrayBufferAllowShared: {
      auto* buffer = buffer_union.GetAsArrayBufferAllowShared();
      return (buffer && !buffer->IsDetached()) ? buffer->ByteSpanMaybeShared()
                                               : base::span<uint8_t>();
    }
    case AllowSharedBufferSource::ContentType::kArrayBufferViewAllowShared: {
      auto* buffer = buffer_union.GetAsArrayBufferViewAllowShared().Get();
      return (buffer && !buffer->IsDetached()) ? buffer->ByteSpanMaybeShared()
                                               : base::span<uint8_t>();
    }
  }
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_FILE_SYSTEM_ACCESS_ALLOW_SHARED_BUFFER_SOURCE_UTIL_H_
