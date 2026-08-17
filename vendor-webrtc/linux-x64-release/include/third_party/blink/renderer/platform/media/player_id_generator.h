// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_PLAYER_ID_GENERATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_PLAYER_ID_GENERATOR_H_

#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

// Generates unique player IDs for use in media player mojo calls.
PLATFORM_EXPORT int GetNextMediaPlayerId();

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_PLAYER_ID_GENERATOR_H_
