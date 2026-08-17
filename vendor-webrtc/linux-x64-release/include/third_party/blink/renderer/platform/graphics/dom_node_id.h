// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_DOM_NODE_ID_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_DOM_NODE_ID_H_

namespace blink {

// Uniquely identifies a DOM node. See renderer/core/dom/dom_node_ids.h.
// The DevTools protocol requires the type to be int, see
// renderer/core/inspector/browser_protocol-*.json.
using DOMNodeId = int;

static const DOMNodeId kInvalidDOMNodeId = 0;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_DOM_NODE_ID_H_
