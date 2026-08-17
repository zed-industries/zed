// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SELECTION_STRATEGY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SELECTION_STRATEGY_H_

namespace blink {

enum class SelectionStrategy {
  // Always using CharacterGranularity
  kCharacter,
  // Switches between WordGranularity and CharacterGranularity
  // Depending on whether the selection or growing or shrinking
  kDirection,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SELECTION_STRATEGY_H_
