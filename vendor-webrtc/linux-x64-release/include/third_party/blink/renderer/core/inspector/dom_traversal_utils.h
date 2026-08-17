// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_DOM_TRAVERSAL_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_DOM_TRAVERSAL_UTILS_H_

namespace blink {

class Node;

namespace dom_traversal_utils {

// These are deprecated, do not use in new code. Use FlatTreeTraversal directly.
Node* FirstChild(const Node& node, bool include_user_agent_shadow_tree);
bool HasChildren(const Node& node, bool include_user_agent_shadow_tree);
Node* NextSibling(const Node& node, bool include_user_agent_shadow_tree);

}  // namespace dom_traversal_utils
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_DOM_TRAVERSAL_UTILS_H_
