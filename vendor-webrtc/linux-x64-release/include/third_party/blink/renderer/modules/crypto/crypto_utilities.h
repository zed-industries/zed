// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CRYPTO_CRYPTO_UTILITIES_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CRYPTO_CRYPTO_UTILITIES_H_

#include <vector>

#include "base/containers/to_vector.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_array_piece.h"

namespace blink {
inline std::vector<uint8_t> CopyBytes(const DOMArrayPiece& source) {
  return base::ToVector(source.ByteSpan());
}
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CRYPTO_CRYPTO_UTILITIES_H_
