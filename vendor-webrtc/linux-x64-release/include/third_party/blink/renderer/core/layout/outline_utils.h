// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_OUTLINE_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_OUTLINE_UTILS_H_

namespace blink {

class ComputedStyle;
class Node;

bool HasPaintedOutline(const ComputedStyle& style, const Node* node);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_OUTLINE_UTILS_H_
