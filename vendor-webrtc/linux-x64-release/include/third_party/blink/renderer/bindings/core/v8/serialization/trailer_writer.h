// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SERIALIZATION_TRAILER_WRITER_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SERIALIZATION_TRAILER_WRITER_H_

#include "third_party/blink/renderer/bindings/core/v8/serialization/serialization_tag.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class CORE_EXPORT TrailerWriter {
 public:
  TrailerWriter();
  ~TrailerWriter();

  void RequireExposedInterface(SerializationTag);

  Vector<uint8_t> MakeTrailerData() const;

 private:
  Vector<SerializationTag> requires_exposed_interfaces_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SERIALIZATION_TRAILER_WRITER_H_
