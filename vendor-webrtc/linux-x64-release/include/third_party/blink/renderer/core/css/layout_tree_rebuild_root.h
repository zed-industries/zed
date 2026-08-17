// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_LAYOUT_TREE_REBUILD_ROOT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_LAYOUT_TREE_REBUILD_ROOT_H_

#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/style_traversal_root.h"

namespace blink {

class CORE_EXPORT LayoutTreeRebuildRoot : public StyleTraversalRoot {
  DISALLOW_NEW();

 public:
  Element& RootElement() const;
  void SubtreeModified(ContainerNode& parent) final;

 private:
#if DCHECK_IS_ON()
  ContainerNode* Parent(const Node& node) const final;
  bool IsChildDirty(const Node& node) const final;
#endif  // DCHECK_IS_ON()
  bool IsDirty(const Node& node) const final;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_LAYOUT_TREE_REBUILD_ROOT_H_
