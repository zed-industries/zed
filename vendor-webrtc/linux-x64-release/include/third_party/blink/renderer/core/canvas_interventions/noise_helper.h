// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CANVAS_INTERVENTIONS_NOISE_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CANVAS_INTERVENTIONS_NOISE_HELPER_H_

#include <cstdint>

#include "base/containers/span.h"
#include "third_party/blink/renderer/core/canvas_interventions/noise_hash.h"
#include "third_party/blink/renderer/core/core_export.h"

namespace blink {
// Adds noise to the provided pixels in place, using |token_hash| as the
// source for pseudo-randomness. This function assumes that there are 4
// channels per pixel.
CORE_EXPORT void NoisePixels(const NoiseHash& token_hash,
                             base::span<uint8_t> pixels,
                             const int width,
                             const int height);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CANVAS_INTERVENTIONS_NOISE_HELPER_H_
