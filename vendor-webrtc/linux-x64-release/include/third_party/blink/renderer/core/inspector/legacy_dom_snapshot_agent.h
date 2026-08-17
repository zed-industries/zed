// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_LEGACY_DOM_SNAPSHOT_AGENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_LEGACY_DOM_SNAPSHOT_AGENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_property_names.h"
#include "third_party/blink/renderer/core/inspector/inspector_base_agent.h"
#include "third_party/blink/renderer/core/inspector/protocol/dom_snapshot.h"
#include "third_party/blink/renderer/platform/graphics/dom_node_id.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class Document;
class Element;
class LayoutObject;
class Node;
class PaintLayer;

struct OriginUrlMap {
  WTF::HashMap<DOMNodeId, String> map;
  base::WeakPtrFactory<OriginUrlMap> weak_ptr_factory{this};
};

class CORE_EXPORT LegacyDOMSnapshotAgent {
  STACK_ALLOCATED();

 public:
  LegacyDOMSnapshotAgent(InspectorDOMDebuggerAgent*,
                         base::WeakPtr<OriginUrlMap>);
  LegacyDOMSnapshotAgent(const LegacyDOMSnapshotAgent&) = delete;
  LegacyDOMSnapshotAgent& operator=(const LegacyDOMSnapshotAgent&) = delete;
  ~LegacyDOMSnapshotAgent();

  void Restore();

  protocol::Response GetSnapshot(
      Document* document,
      std::unique_ptr<protocol::Array<String>> style_filter,
      std::optional<bool> include_event_listeners,
      std::optional<bool> include_paint_order,
      std::optional<bool> include_user_agent_shadow_tree,
      std::unique_ptr<protocol::Array<protocol::DOMSnapshot::DOMNode>>*
          dom_nodes,
      std::unique_ptr<protocol::Array<protocol::DOMSnapshot::LayoutTreeNode>>*
          layout_tree_nodes,
      std::unique_ptr<protocol::Array<protocol::DOMSnapshot::ComputedStyle>>*
          computed_styles);

 private:
  int VisitNode(Node*,
                bool include_event_listeners,
                bool include_user_agent_shadow_tree);

  std::unique_ptr<protocol::Array<int>> VisitContainerChildren(
      Node* container,
      bool include_event_listeners,
      bool include_user_agent_shadow_tree);

  std::unique_ptr<protocol::Array<int>> VisitPseudoElements(
      Element* parent,
      int index,
      bool include_event_listeners,
      bool include_user_agent_shadow_tree);
  std::unique_ptr<protocol::Array<protocol::DOMSnapshot::NameValue>>
  BuildArrayForElementAttributes(Element*);

  // Adds a LayoutTreeNode for the LayoutObject to |layout_tree_nodes_| and
  // returns its index. Returns -1 if the Node has no associated LayoutObject.
  // Associates LayoutObjects under a pseudo element with the element.
  int VisitLayoutTreeNode(LayoutObject*, Node*, int node_index);
  int BuildLayoutTreeNode(LayoutObject*, Node*, int node_index);

  // Returns the index of the ComputedStyle in |computed_styles_| for the given
  // Node. Adds a new ComputedStyle if necessary, but ensures no duplicates are
  // added to |computed_styles_|. Returns -1 if the node has no values for
  // styles in |style_filter_|.
  int GetStyleIndexForNode(Node*);

  struct VectorStringHashTraits;
  using ComputedStylesMap =
      WTF::HashMap<Vector<String>, int, VectorStringHashTraits>;
  using CSSPropertyFilter = Vector<std::pair<String, CSSPropertyID>>;
  using PaintOrderMap = GCedHeapHashMap<Member<PaintLayer>, int>;

  // State of current snapshot.
  std::unique_ptr<protocol::Array<protocol::DOMSnapshot::DOMNode>> dom_nodes_;
  std::unique_ptr<protocol::Array<protocol::DOMSnapshot::LayoutTreeNode>>
      layout_tree_nodes_;
  std::unique_ptr<protocol::Array<protocol::DOMSnapshot::ComputedStyle>>
      computed_styles_;

  // Maps a style string vector to an index in |computed_styles_|. Used to avoid
  // duplicate entries in |computed_styles_|.
  std::unique_ptr<ComputedStylesMap> computed_styles_map_;
  std::unique_ptr<CSSPropertyFilter> css_property_filter_;
  // Maps a PaintLayer to its paint order index.
  PaintOrderMap* paint_order_map_ = nullptr;
  // Maps a backend node id to the url of the script (if any) that generates
  // the corresponding node.
  base::WeakPtr<OriginUrlMap> origin_url_map_;
  using DocumentOrderMap = HeapHashMap<Member<Document>, int>;
  InspectorDOMDebuggerAgent* dom_debugger_agent_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_LEGACY_DOM_SNAPSHOT_AGENT_H_
