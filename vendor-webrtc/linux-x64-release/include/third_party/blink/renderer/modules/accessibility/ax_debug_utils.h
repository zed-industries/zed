// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_AX_DEBUG_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_AX_DEBUG_UTILS_H_

#include "third_party/blink/renderer/modules/accessibility/ax_object.h"
#include "third_party/blink/renderer/modules/accessibility/ax_object_cache_impl.h"
#include "ui/accessibility/ax_tree_serializer.h"

namespace blink {

class LayoutBlockFlow;
class FragmentItems;

// Friendly output of a subtree, useful for debugging.
std::string TreeToStringHelper(const AXObject* obj,
                               bool verbose = true);
std::string TreeToStringWithMarkedObjectHelper(const AXObject* obj,
                                               const AXObject* marked_object,
                                               bool verbose = true);
std::string ParentChainToStringHelper(const AXObject* obj);

// Ensure the tree serializer expects to serializer the same number of included
// nodes as the AXObjectCache thinks exists.
void CheckTreeConsistency(
    AXObjectCacheImpl& cache,
    ui::AXTreeSerializer<const AXObject*,
                         HeapVector<Member<const AXObject>>,
                         ui::AXTreeUpdate*,
                         ui::AXTreeData*,
                         ui::AXNodeData>& serializer,
    ui::AXTreeSerializer<const ui::AXNode*,
                         std::vector<const ui::AXNode*>,
                         ui::AXTreeUpdate*,
                         ui::AXTreeData*,
                         ui::AXNodeData>* plugin_serializer);

#if DCHECK_IS_ON()
// Performs a diagnostic dump of block fragmentation.
// Though not performning DCHECKS the diagnostic tools for block fragmentation
// are restricted to DCHECK-enabled builds to avoid including in release builds.
// Launch with --vmodule=ax_debug_utils=2 to see a diagnostic dump of the block
// fragmentation.
void DumpBlockFragmentationData(const LayoutBlockFlow* layout_block_flow);
void DumpBlockFragmentationData(const FragmentItems* fragment_items,
                                int indent);
#endif

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_AX_DEBUG_UTILS_H_
