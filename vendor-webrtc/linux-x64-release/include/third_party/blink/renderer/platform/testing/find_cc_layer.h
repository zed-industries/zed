// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_FIND_CC_LAYER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_FIND_CC_LAYER_H_

#include "cc/input/scrollbar.h"
#include "third_party/blink/renderer/platform/graphics/compositor_element_id.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace cc {
class Layer;
class ScrollbarLayerBase;
struct ScrollNode;
}  // namespace cc

namespace blink {

// Finds all cc layers whose DebugName()s contain regular expression
// |name_regex|.
Vector<cc::Layer*> CcLayersByName(cc::Layer* root, const String& name_regex);

Vector<const cc::Layer*> CcLayersByName(const cc::Layer* root,
                                        const String& name_regex);

Vector<cc::Layer*> CcLayersByDOMElementId(cc::Layer* root,
                                          const String& dom_id);

Vector<const cc::Layer*> CcLayersByDOMElementId(const cc::Layer* root,
                                                const String& dom_id);

cc::Layer* CcLayerByCcElementId(cc::Layer* root, const CompositorElementId&);

const cc::Layer* CcLayerByCcElementId(const cc::Layer* root,
                                      const CompositorElementId&);

cc::Layer* CcLayerByOwnerNodeId(cc::Layer* root, DOMNodeId);

const cc::Layer* CcLayerByOwnerNodeId(const cc::Layer* root, DOMNodeId);

cc::Layer* ScrollingContentsCcLayerByScrollElementId(
    cc::Layer* root,
    const CompositorElementId& scroll_element_id);

const cc::Layer* ScrollingContentsCcLayerByScrollElementId(
    const cc::Layer* root,
    const CompositorElementId& scroll_element_id);

cc::ScrollbarLayerBase* ScrollbarLayerForScrollNode(
    cc::Layer* root,
    cc::ScrollNode* scroll_node,
    cc::ScrollbarOrientation orientation);

const cc::ScrollbarLayerBase* ScrollbarLayerForScrollNode(
    const cc::Layer* root,
    const cc::ScrollNode* scroll_node,
    cc::ScrollbarOrientation orientation);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_FIND_CC_LAYER_H_
