// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_DOM_SNAPSHOT_AGENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_DOM_SNAPSHOT_AGENT_H_

#include "base/memory/weak_ptr.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_property_names.h"
#include "third_party/blink/renderer/core/inspector/inspector_base_agent.h"
#include "third_party/blink/renderer/core/inspector/inspector_contrast.h"
#include "third_party/blink/renderer/core/inspector/protocol/dom_snapshot.h"
#include "third_party/blink/renderer/core/layout/layout_text.h"
#include "third_party/blink/renderer/platform/graphics/dom_node_id.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class CharacterData;
class ComputedStyle;
class Document;
class Element;
class InspectedFrames;
class Node;
class PaintLayer;
struct OriginUrlMap;

class CORE_EXPORT InspectorDOMSnapshotAgent final
    : public InspectorBaseAgent<protocol::DOMSnapshot::Metainfo> {
 public:
  InspectorDOMSnapshotAgent(InspectedFrames*, InspectorDOMDebuggerAgent*);
  InspectorDOMSnapshotAgent(const InspectorDOMSnapshotAgent&) = delete;
  InspectorDOMSnapshotAgent& operator=(const InspectorDOMSnapshotAgent&) =
      delete;
  ~InspectorDOMSnapshotAgent() override;
  void Trace(Visitor*) const override;

  void Restore() override;

  protocol::Response enable() override;
  protocol::Response disable() override;
  protocol::Response getSnapshot(
      std::unique_ptr<protocol::Array<String>> style_filter,
      std::optional<bool> include_event_listeners,
      std::optional<bool> include_paint_order,
      std::optional<bool> include_user_agent_shadow_tree,
      std::unique_ptr<protocol::Array<protocol::DOMSnapshot::DOMNode>>*
          dom_nodes,
      std::unique_ptr<protocol::Array<protocol::DOMSnapshot::LayoutTreeNode>>*
          layout_tree_nodes,
      std::unique_ptr<protocol::Array<protocol::DOMSnapshot::ComputedStyle>>*
          computed_styles) override;
  protocol::Response captureSnapshot(
      std::unique_ptr<protocol::Array<String>> computed_styles,
      std::optional<bool> include_paint_order,
      std::optional<bool> include_dom_rects,
      std::optional<bool> include_blended_background_colors,
      std::optional<bool> include_text_color_opacities,
      std::unique_ptr<protocol::Array<protocol::DOMSnapshot::DocumentSnapshot>>*
          documents,
      std::unique_ptr<protocol::Array<String>>* strings) override;

  // InspectorInstrumentation API.
  void CharacterDataModified(CharacterData*);
  void DidInsertDOMNode(Node*);

  // Helpers for rects
  static PhysicalRect RectInDocument(const LayoutObject* layout_object);
  static PhysicalRect TextFragmentRectInDocument(
      const LayoutObject* layout_object,
      const LayoutText::TextBoxInfo& text_box);

  using PaintOrderMap = GCedHeapHashMap<Member<PaintLayer>, int>;
  static PaintOrderMap* BuildPaintLayerTree(Document*);

 private:
  // Unconditionally enables the agent, even if |enabled_.Get()==true|.
  // For idempotence, call enable().
  void EnableAndReset();

  int AddString(const String& string);
  void SetRare(protocol::DOMSnapshot::RareIntegerData* data,
               int index,
               int value);
  void SetRare(protocol::DOMSnapshot::RareStringData* data,
               int index,
               const String& value);
  void SetRare(protocol::DOMSnapshot::RareBooleanData* data, int index);
  void VisitDocument(Document*);

  void VisitNode(Node*, int parent_index, InspectorContrast& contrast);
  void VisitContainerChildren(Node* container, int parent_index);
  void VisitPseudoElements(Element* parent,
                           int parent_index,
                           InspectorContrast& contrast);
  std::unique_ptr<protocol::Array<int>> BuildArrayForElementAttributes(Node*);
  int BuildLayoutTreeNode(LayoutObject*,
                          Node*,
                          int node_index,
                          InspectorContrast& contrast);
  std::unique_ptr<protocol::Array<int>> BuildStylesForNode(Node*);

  static void TraversePaintLayerTree(Document*, PaintOrderMap* paint_order_map);
  static void VisitPaintLayer(PaintLayer*, PaintOrderMap* paint_order_map);

  using CSSPropertyFilter = Vector<const CSSProperty*>;

  // State of current snapshot.
  std::unique_ptr<protocol::Array<protocol::DOMSnapshot::DOMNode>> dom_nodes_;
  std::unique_ptr<protocol::Array<protocol::DOMSnapshot::LayoutTreeNode>>
      layout_tree_nodes_;
  std::unique_ptr<protocol::Array<protocol::DOMSnapshot::ComputedStyle>>
      computed_styles_;

  std::unique_ptr<protocol::Array<String>> strings_;
  WTF::HashMap<String, int> string_table_;

  HeapHashMap<Member<const CSSValue>, int> css_value_cache_;
  HeapHashMap<Member<const ComputedStyle>, protocol::Array<int>*> style_cache_;

  std::unique_ptr<protocol::Array<protocol::DOMSnapshot::DocumentSnapshot>>
      documents_;
  std::unique_ptr<protocol::DOMSnapshot::DocumentSnapshot> document_;

  bool include_snapshot_dom_rects_ = false;
  bool include_blended_background_colors_ = false;
  bool include_text_color_opacities_ = false;
  std::unique_ptr<CSSPropertyFilter> css_property_filter_;
  // Maps a PaintLayer to its paint order index.
  Member<PaintOrderMap> paint_order_map_;
  // Maps a backend node id to the url of the script (if any) that generates
  // the corresponding node.
  std::unique_ptr<OriginUrlMap> origin_url_map_;
  using DocumentOrderMap = HeapHashMap<Member<Document>, int>;
  DocumentOrderMap document_order_map_;
  Member<InspectedFrames> inspected_frames_;
  Member<InspectorDOMDebuggerAgent> dom_debugger_agent_;
  InspectorAgentState::Boolean enabled_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_DOM_SNAPSHOT_AGENT_H_
