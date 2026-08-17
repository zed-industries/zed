// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SERIALIZATION_TRAILER_READER_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SERIALIZATION_TRAILER_READER_H_

#include "third_party/blink/renderer/core/core_export.h"

#include "base/containers/buffer_iterator.h"
#include "base/types/expected.h"
#include "third_party/blink/renderer/bindings/core/v8/serialization/serialization_tag.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class CORE_EXPORT TrailerReader {
 public:
  enum class Error { kInvalidHeader, kInvalidTrailer };
  static constexpr uint32_t kMinWireFormatVersion = 21;

  explicit TrailerReader(base::span<const uint8_t>);
  ~TrailerReader();

  // Call before |Read| if this was initialized with a full message, rather than
  // just the trailer. Returns whether a trailer was found, or an error.
  // If no trailer is found, it is still safe to call |Read|.
  // If an error is returned, this object is left in an unspecified state.
  [[nodiscard]] base::expected<bool, Error> SkipToTrailer();

  // Read trailer data from the current iterator position, and populates the
  // below data. If an error is returned, this object is left in an unspecified
  // state.
  [[nodiscard]] base::expected<void, Error> Read();

  // List of serialization tags, in any order, corresponding to WebIDL
  // interfaces which must be exposed in the receiving realm.
  // In practice these should be unique, but this is not validated.
  base::span<const SerializationTag> required_exposed_interfaces() const {
    return required_exposed_interfaces_;
  }

  size_t GetPositionForTesting() const { return iterator_.position(); }

 private:
  base::BufferIterator<const uint8_t> iterator_;
  Vector<SerializationTag> required_exposed_interfaces_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SERIALIZATION_TRAILER_READER_H_
