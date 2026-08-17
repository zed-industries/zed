// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_SHADOW_SHADOW_ELEMENT_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_SHADOW_SHADOW_ELEMENT_UTILS_H_

#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

class Element;
class Node;

bool IsSliderContainer(const Element& elmenet);
bool IsSliderThumb(const Node* node);
bool IsTextControlContainer(const Node* node);
bool IsTextControlPlaceholder(const Node* node);

namespace shadow_element_utils {

const AtomicString& StringForUAShadowPseudoId(PseudoId pseudo_id);

}  // namespace shadow_element_utils

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_SHADOW_SHADOW_ELEMENT_UTILS_H_
