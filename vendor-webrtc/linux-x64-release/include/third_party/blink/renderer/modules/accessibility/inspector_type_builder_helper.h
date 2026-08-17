// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_INSPECTOR_TYPE_BUILDER_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_INSPECTOR_TYPE_BUILDER_HELPER_H_

#include "third_party/blink/renderer/core/accessibility/axid.h"
#include "third_party/blink/renderer/core/inspector/protocol/accessibility.h"

namespace blink {

class AXObject;
using protocol::Accessibility::AXNode;

inline constexpr AXID kIDForInspectedNodeWithNoAXNode = 0;

std::unique_ptr<AXNode> BuildProtocolAXNodeForDOMNodeWithNoAXNode(
    int backend_node_id);
std::unique_ptr<AXNode> BuildProtocolAXNodeForAXObject(
    AXObject&,
    bool force_name_and_role = false);
std::unique_ptr<AXNode> BuildProtocolAXNodeForIgnoredAXObject(
    AXObject&,
    bool force_name_and_role);
std::unique_ptr<AXNode> BuildProtocolAXNodeForUnignoredAXObject(AXObject&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_INSPECTOR_TYPE_BUILDER_HELPER_H_
