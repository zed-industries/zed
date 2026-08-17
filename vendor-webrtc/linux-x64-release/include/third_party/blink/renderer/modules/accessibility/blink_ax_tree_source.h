// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_BLINK_AX_TREE_SOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_BLINK_AX_TREE_SOURCE_H_

#include <stdint.h>

#include <optional>
#include <string>

#include "third_party/blink/public/web/web_ax_object.h"
#include "third_party/blink/renderer/modules/accessibility/ax_object.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "ui/accessibility/ax_common.h"
#include "ui/accessibility/ax_mode.h"
#include "ui/accessibility/ax_node_data.h"
#include "ui/accessibility/ax_tree_source.h"

namespace ui {
struct AXTreeData;
}

namespace blink {

class AXObjectCacheImpl;

class MODULES_EXPORT BlinkAXTreeSource
    : public GarbageCollected<BlinkAXTreeSource>,
      public ui::AXTreeSource<const AXObject*, ui::AXTreeData*, ui::AXNodeData> {
 public:
  // Pass truncate_inline_textboxes_ if inline textboxes should be removed
  // from the serialized tree, even if they are already available in the cache.
  explicit BlinkAXTreeSource(AXObjectCacheImpl& ax_object_cache,
                             bool is_snapshot);
  ~BlinkAXTreeSource() override;

  static BlinkAXTreeSource* Create(AXObjectCacheImpl& ax_object_cache,
                                   bool is_snapshot = false) {
    return MakeGarbageCollected<BlinkAXTreeSource>(ax_object_cache,
                                                   is_snapshot);
  }

  // AXTreeSource implementation.
  bool GetTreeData(ui::AXTreeData* tree_data) const override;
  const AXObject* GetRoot() const override;
  const AXObject* GetFromId(int32_t id) const override;
  int32_t GetId(const AXObject* node) const override;
  void CacheChildrenIfNeeded(const AXObject*) override {}
  size_t GetChildCount(const AXObject* node) const override;
  AXObject* ChildAt(const AXObject* node, size_t) const override;
  void ClearChildCache(const AXObject*) override {}
  AXObject* GetParent(const AXObject* node) const override;
  void SerializeNode(const AXObject* node, ui::AXNodeData* out_data) const override;
  bool IsIgnored(const AXObject* node) const override;
  bool IsEqual(const AXObject* node1, const AXObject* node2) const override;
  AXObject* GetNull() const override;
  std::string GetDebugString(const AXObject* node) const override;

  // Ignore code that limits based on the protocol (like https, file, etc.)
  // to enable tests to run.
  static void IgnoreProtocolChecksForTesting();

  void Trace(Visitor*) const;

  void Freeze();

  void Thaw();

 private:
  void Selection(const AXObject* obj,
                 bool& is_selection_backward,
                 const AXObject** anchor_object,
                 int& anchor_offset,
                 ax::mojom::blink::TextAffinity& anchor_affinity,
                 const AXObject** focus_object,
                 int& focus_offset,
                 ax::mojom::blink::TextAffinity& focus_affinity) const;

  const AXObject* GetFocusedObject() const;

  // Truncate inline text boxes in snapshots, as they are just extra noise for
  // consumers of the entire tree (e.g. AXTreeSnapshotter). This avoids passing
  // the inline text boxes, even if a previous AXContext had built them.
  bool ShouldTruncateInlineTextBoxes() const { return is_snapshot_; }

  // Whether we should highlight annotation results visually on the page
  // for debugging.
  bool image_annotation_debugging_ = false;

  Member<AXObjectCacheImpl> ax_object_cache_;

  bool frozen_ = false;
  // TODO(accessibility) If caching these does not improv perf, remove these.
  Member<const AXObject> root_ = nullptr;
  Member<const AXObject> focus_ = nullptr;

  // The AxID of the first unlabeled image we have encountered in this tree.
  //
  // Used to ensure that the tutor message that explains to screen reader users
  // how to turn on automatic image labels is provided only once.
  mutable std::optional<int32_t> first_unlabeled_image_id_ = std::nullopt;

  const bool is_snapshot_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_BLINK_AX_TREE_SOURCE_H_
