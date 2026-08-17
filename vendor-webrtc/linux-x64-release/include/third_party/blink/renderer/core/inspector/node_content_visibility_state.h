// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_NODE_CONTENT_VISIBILITY_STATE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_NODE_CONTENT_VISIBILITY_STATE_H_

// The state of content visibility:
//  - kNone means that the node has no content-visibility interactions
//  - kIsLocked means that the node is itself locked and is skipping its
//    contents. However, the node is not in a subtree of a locked element.
//  - kIsLockedAncestor means that the initial node was in a locked subtree
//    so we instead are showing this ancestor.
enum class NodeContentVisibilityState { kNone, kIsLocked, kIsLockedAncestor };

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_NODE_CONTENT_VISIBILITY_STATE_H_
