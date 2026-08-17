// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_NG_FLAT_TREE_SHORTHANDS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_NG_FLAT_TREE_SHORTHANDS_H_

#include "third_party/blink/renderer/core/editing/forward.h"

namespace blink {

struct InlineCaretPosition;
class LayoutBlockFlow;

// This file contains shorthands that converts FlatTree-variants of editing
// objects into DOM tree variants, and then pass them to LayoutNG utility
// functions that accept DOM tree variants only.

const LayoutBlockFlow* NGInlineFormattingContextOf(const PositionInFlatTree&);

InlineCaretPosition ComputeInlineCaretPosition(
    const PositionInFlatTreeWithAffinity&);

bool InSameNGLineBox(const PositionInFlatTreeWithAffinity&,
                     const PositionInFlatTreeWithAffinity&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_NG_FLAT_TREE_SHORTHANDS_H_
