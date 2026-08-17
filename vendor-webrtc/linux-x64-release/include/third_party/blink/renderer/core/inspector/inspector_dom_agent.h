/*
 * Copyright (C) 2009 Apple Inc. All rights reserved.
 * Copyright (C) 2011 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 * 3.  Neither the name of Apple Computer, Inc. ("Apple") nor the names of
 *     its contributors may be used to endorse or promote products derived
 *     from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_DOM_AGENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_DOM_AGENT_H_

#include <memory>

#include "base/functional/callback.h"
#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/event_listener_map.h"
#include "third_party/blink/renderer/core/inspector/inspector_base_agent.h"
#include "third_party/blink/renderer/core/inspector/protocol/dom.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/platform/bindings/source_location.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "ui/gfx/geometry/quad_f.h"
#include "v8/include/v8-inspector.h"
#include "v8/include/v8-profiler.h"

namespace blink {

class CharacterData;
class DOMEditor;
class Document;
class DocumentLoader;
class DummyExceptionStateForTesting;
class Element;
class HTMLFrameOwnerElement;
class HTMLSlotElement;
class InspectedFrames;
class InspectorHistory;
class Node;
class QualifiedName;
class PseudoElement;
class InspectorRevalidateDOMTask;
class ShadowRoot;

class CORE_EXPORT InspectorDOMAgent final
    : public InspectorBaseAgent<protocol::DOM::Metainfo> {
 public:
  struct CORE_EXPORT DOMListener : public GarbageCollectedMixin {
    virtual ~DOMListener() = default;
    virtual void DidAddDocument(Document*) = 0;
    virtual void WillRemoveDOMNode(Node*) = 0;
    virtual void DidModifyDOMAttr(Element*) = 0;
  };

  enum class IncludeWhitespaceEnum : int32_t { NONE = 0, ALL = 2 };

  class CORE_EXPORT InspectorSourceLocation final
      : public GarbageCollected<InspectorSourceLocation> {
   public:
    InspectorSourceLocation(std::unique_ptr<SourceLocation> source_location)
        : source_location_(std::move(source_location)) {}

    SourceLocation& GetSourceLocation() { return *source_location_; }
    void Trace(Visitor* visitor) const {}

   private:
    std::unique_ptr<SourceLocation> source_location_;
  };

  static protocol::Response ToResponse(DummyExceptionStateForTesting&);
  static protocol::DOM::PseudoType ProtocolPseudoElementType(PseudoId);
  static PseudoId ProtocolPseudoTypeToPseudoId(protocol::DOM::PseudoType);
  static protocol::DOM::ShadowRootType GetShadowRootType(ShadowRoot*);
  static protocol::DOM::CompatibilityMode GetDocumentCompatibilityMode(
      Document*);
  static ShadowRoot* UserAgentShadowRoot(Node*);

  InspectorDOMAgent(v8::Isolate*,
                    InspectedFrames*,
                    v8_inspector::V8InspectorSession*);
  InspectorDOMAgent(const InspectorDOMAgent&) = delete;
  InspectorDOMAgent& operator=(const InspectorDOMAgent&) = delete;
  ~InspectorDOMAgent() override;

  // InspectorBaseAgent overrides.
  void Trace(Visitor*) const override;
  void Dispose() override;
  void Restore() override;

  HeapVector<Member<Document>> Documents();
  void Reset();

  // Methods called from the frontend for DOM nodes inspection.
  protocol::Response enable(std::optional<String> includeWhitespace) override;
  protocol::Response disable() override;
  protocol::Response getDocument(
      std::optional<int> depth,
      std::optional<bool> traverse_frames,
      std::unique_ptr<protocol::DOM::Node>* root) override;
  protocol::Response getNodesForSubtreeByStyle(
      int node_id,
      std::unique_ptr<protocol::Array<protocol::DOM::CSSComputedStyleProperty>>
          computed_styles,
      std::optional<bool> pierce,
      std::unique_ptr<protocol::Array<int>>* node_ids) override;
  protocol::Response getFlattenedDocument(
      std::optional<int> depth,
      std::optional<bool> pierce,
      std::unique_ptr<protocol::Array<protocol::DOM::Node>>* nodes) override;
  protocol::Response collectClassNamesFromSubtree(
      int node_id,
      std::unique_ptr<protocol::Array<String>>* class_names) override;
  protocol::Response requestChildNodes(
      int node_id,
      std::optional<int> depth,
      std::optional<bool> traverse_frames) override;
  protocol::Response querySelector(int node_id,
                                   const String& selector,
                                   int* out_node_id) override;
  protocol::Response querySelectorAll(
      int node_id,
      const String& selector,
      std::unique_ptr<protocol::Array<int>>* node_ids) override;
  protocol::Response setNodeName(int node_id,
                                 const String& name,
                                 int* out_node_id) override;
  protocol::Response setNodeValue(int node_id, const String& value) override;
  protocol::Response removeNode(int node_id) override;
  protocol::Response setAttributeValue(int node_id,
                                       const String& name,
                                       const String& value) override;
  protocol::Response setAttributesAsText(int node_id,
                                         const String& text,
                                         std::optional<String> name) override;
  protocol::Response removeAttribute(int node_id, const String& name) override;
  protocol::Response getOuterHTML(std::optional<int> node_id,
                                  std::optional<int> backend_node_id,
                                  std::optional<String> object_id,
                                  String* outer_html) override;
  protocol::Response setOuterHTML(int node_id,
                                  const String& outer_html) override;
  protocol::Response performSearch(
      const String& query,
      std::optional<bool> include_user_agent_shadow_dom,
      String* search_id,
      int* result_count) override;
  protocol::Response getSearchResults(
      const String& search_id,
      int from_index,
      int to_index,
      std::unique_ptr<protocol::Array<int>>* node_ids) override;
  protocol::Response discardSearchResults(const String& search_id) override;
  protocol::Response requestNode(const String& object_id,
                                 int* out_node_id) override;
  protocol::Response pushNodeByPathToFrontend(const String& path,
                                              int* out_node_id) override;
  protocol::Response pushNodesByBackendIdsToFrontend(
      std::unique_ptr<protocol::Array<int>> backend_node_ids,
      std::unique_ptr<protocol::Array<int>>* node_ids) override;
  protocol::Response setInspectedNode(int node_id) override;
  protocol::Response resolveNode(
      std::optional<int> node_id,
      std::optional<int> backend_node_id,
      std::optional<String> object_group,
      std::optional<int> execution_context_id,
      std::unique_ptr<v8_inspector::protocol::Runtime::API::RemoteObject>*)
      override;
  protocol::Response getAttributes(
      int node_id,
      std::unique_ptr<protocol::Array<String>>* attributes) override;
  protocol::Response copyTo(int node_id,
                            int target_node_id,
                            std::optional<int> insert_before_node_id,
                            int* out_node_id) override;
  protocol::Response moveTo(int node_id,
                            int target_node_id,
                            std::optional<int> insert_before_node_id,
                            int* out_node_id) override;
  protocol::Response undo() override;
  protocol::Response redo() override;
  protocol::Response markUndoableState() override;
  protocol::Response focus(std::optional<int> node_id,
                           std::optional<int> backend_node_id,
                           std::optional<String> object_id) override;
  protocol::Response setFileInputFiles(
      std::unique_ptr<protocol::Array<String>> files,
      std::optional<int> node_id,
      std::optional<int> backend_node_id,
      std::optional<String> object_id) override;
  protocol::Response setNodeStackTracesEnabled(bool enable) override;
  protocol::Response getNodeStackTraces(
      int node_id,
      std::unique_ptr<v8_inspector::protocol::Runtime::API::StackTrace>*
          creation) override;
  protocol::Response getBoxModel(
      std::optional<int> node_id,
      std::optional<int> backend_node_id,
      std::optional<String> object_id,
      std::unique_ptr<protocol::DOM::BoxModel>*) override;
  protocol::Response getContentQuads(
      std::optional<int> node_id,
      std::optional<int> backend_node_id,
      std::optional<String> object_id,
      std::unique_ptr<protocol::Array<protocol::Array<double>>>* quads)
      override;
  protocol::Response getNodeForLocation(
      int x,
      int y,
      std::optional<bool> include_user_agent_shadow_dom,
      std::optional<bool> ignore_pointer_events_none,
      int* backend_node_id,
      String* frame_id,
      std::optional<int>* node_id) override;
  protocol::Response getRelayoutBoundary(int node_id,
                                         int* out_node_id) override;
  protocol::Response describeNode(
      std::optional<int> node_id,
      std::optional<int> backend_node_id,
      std::optional<String> object_id,
      std::optional<int> depth,
      std::optional<bool> pierce,
      std::unique_ptr<protocol::DOM::Node>*) override;
  protocol::Response scrollIntoViewIfNeeded(
      std::optional<int> node_id,
      std::optional<int> backend_node_id,
      std::optional<String> object_id,
      std::unique_ptr<protocol::DOM::Rect> rect) override;

  protocol::Response getFrameOwner(const String& frame_id,
                                   int* backend_node_id,
                                   std::optional<int>* node_id) override;

  protocol::Response getFileInfo(const String& object_id,
                                 String* path) override;

  protocol::Response getDetachedDomNodes(
      std::unique_ptr<protocol::Array<protocol::DOM::DetachedElementInfo>>*
          detached_nodes) override;

  // Find the closest size query container ascendant for a node given an
  // optional container-name.
  protocol::Response getContainerForNode(
      int node_id,
      std::optional<String> container_name,
      std::optional<protocol::DOM::PhysicalAxes> physical_axes,
      std::optional<protocol::DOM::LogicalAxes> logical_axes,
      std::optional<bool> queries_scroll_state,
      std::optional<int>* container_node_id) override;
  protocol::Response getQueryingDescendantsForContainer(
      int node_id,
      std::unique_ptr<protocol::Array<int>>* node_ids) override;
  static const HeapVector<Member<Element>> GetContainerQueryingDescendants(
      Element* container);

  protocol::Response getElementByRelation(int node_id,
                                          const String& relation,
                                          int* out_node_id) override;

  protocol::Response getAnchorElement(int node_id,
                                      std::optional<String> anchor_specifier,
                                      int* out_node_id) override;

  bool Enabled() const;
  IncludeWhitespaceEnum IncludeWhitespace() const;
  void ReleaseDanglingNodes();

  // Methods called from the InspectorInstrumentation.
  void DomContentLoadedEventFired(LocalFrame*);
  void DidCommitLoad(LocalFrame*, DocumentLoader*);
  void DidRestoreFromBackForwardCache(LocalFrame*);
  void DidInsertDOMNode(Node*);
  void WillRemoveDOMNode(Node*);
  void WillModifyDOMAttr(Element*,
                         const AtomicString& old_value,
                         const AtomicString& new_value);
  void DidModifyDOMAttr(Element*,
                        const QualifiedName&,
                        const AtomicString& value);
  void DidRemoveDOMAttr(Element*, const QualifiedName&);
  void StyleAttributeInvalidated(const HeapVector<Member<Element>>& elements);
  void CharacterDataModified(CharacterData*);
  void DidInvalidateStyleAttr(Node*);
  void DidPushShadowRoot(Element* host, ShadowRoot*);
  void WillPopShadowRoot(Element* host, ShadowRoot*);
  void DidPerformSlotDistribution(HTMLSlotElement*);
  void FrameDocumentUpdated(LocalFrame*);
  void FrameOwnerContentUpdated(LocalFrame*, HTMLFrameOwnerElement*);
  void PseudoElementCreated(PseudoElement*);
  void TopLayerElementsChanged();
  void PseudoElementDestroyed(PseudoElement*);
  void NodeCreated(Node* node);
  void UpdateScrollableFlag(Node* node, std::optional<bool>);

  Node* NodeForId(int node_id) const;
  int BoundNodeId(Node*) const;
  void AddDOMListener(DOMListener*);
  void RemoveDOMListener(DOMListener*);
  int PushNodePathToFrontend(Node*);
  protocol::Response NodeForRemoteObjectId(const String& remote_object_id,
                                           Node*&);

  static String DocumentURLString(Document*);
  static String DocumentBaseURLString(Document*);

  InspectorHistory* History() { return history_.Get(); }

  // We represent embedded doms as a part of the same hierarchy. Hence we treat
  // children of frame owners differently.  We also skip whitespace text nodes
  // conditionally. Following methods encapsulate these specifics.
  static Node* InnerFirstChild(Node*, IncludeWhitespaceEnum include_whitespace);
  static Node* InnerNextSibling(Node*,
                                IncludeWhitespaceEnum include_whitespace);
  static Node* InnerPreviousSibling(Node*,
                                    IncludeWhitespaceEnum include_whitespace);
  static unsigned InnerChildNodeCount(Node*,
                                      IncludeWhitespaceEnum include_whitespace);
  static Node* InnerParentNode(Node*);
  static bool ShouldSkipNode(Node*, IncludeWhitespaceEnum include_whitespace);
  static void CollectNodes(Node* root,
                           int depth,
                           bool pierce,
                           IncludeWhitespaceEnum include_whitespace,
                           base::RepeatingCallback<bool(Node*)>,
                           HeapVector<Member<Node>>* result);

  protocol::Response AssertNode(int node_id, Node*&);
  protocol::Response AssertNode(const std::optional<int>& node_id,
                                const std::optional<int>& backend_node_id,
                                const std::optional<String>& object_id,
                                Node*&);
  protocol::Response AssertElement(int node_id, Element*&);
  Document* GetDocument() const { return document_.Get(); }
  protocol::Response getTopLayerElements(
      std::unique_ptr<protocol::Array<int>>* node_ids) override;

 private:
  void SetDocument(Document*);
  // Unconditionally enables the agent, even if |enabled_.Get()==true|.
  // For idempotence, call enable().
  void EnableAndReset();

  void NotifyDidAddDocument(Document*);
  void NotifyWillRemoveDOMNode(Node*);
  void NotifyDidModifyDOMAttr(Element*);

  // Node-related methods.
  using NodeToIdMap = GCedHeapHashMap<Member<Node>, int>;
  int Bind(Node*, NodeToIdMap*);
  void Unbind(Node*);

  protocol::Response AssertEditableNode(int node_id, Node*&);
  protocol::Response AssertEditableChildNode(Element* parent_element,
                                             int node_id,
                                             Node*&);
  protocol::Response AssertEditableElement(int node_id, Element*&);

  int PushNodePathToFrontend(Node*, NodeToIdMap* node_map);
  void PushChildNodesToFrontend(int node_id,
                                int depth = 1,
                                bool traverse_frames = false);
  void DOMNodeRemoved(Node*);

  void InvalidateFrameOwnerElement(HTMLFrameOwnerElement*);

  std::unique_ptr<protocol::DOM::Node> BuildObjectForNode(
      Node*,
      int depth,
      bool traverse_frames,
      NodeToIdMap*,
      protocol::Array<protocol::DOM::Node>* flatten_result = nullptr);
  std::unique_ptr<protocol::Array<String>> BuildArrayForElementAttributes(
      Element*);
  std::unique_ptr<protocol::Array<protocol::DOM::Node>>
  BuildArrayForContainerChildren(
      Node* container,
      int depth,
      bool traverse_frames,
      NodeToIdMap* nodes_map,
      protocol::Array<protocol::DOM::Node>* flatten_result);
  std::unique_ptr<protocol::Array<protocol::DOM::Node>>
  BuildArrayForPseudoElements(Element*, NodeToIdMap* nodes_map);
  std::unique_ptr<protocol::DOM::BackendNode> BuildBackendNode(Node* node);
  std::unique_ptr<protocol::Array<protocol::DOM::BackendNode>>
  BuildDistributedNodesForSlot(HTMLSlotElement*);

  static bool ContainerQueriedByElement(Element* container, Element* element);

  Node* NodeForPath(const String& path);

  void DiscardFrontendBindings();

  InspectorRevalidateDOMTask* RevalidateTask();

  bool isNodeScrollable(Node*);

  v8::Isolate* isolate_;  // null after Dispose().
  Member<InspectedFrames> inspected_frames_;
  v8_inspector::V8InspectorSession* v8_session_;  // null after Dispose().
  HeapHashSet<Member<DOMListener>> dom_listeners_;
  Member<NodeToIdMap> document_node_to_id_map_;
  // Owns node mappings for dangling nodes.
  HeapVector<Member<NodeToIdMap>> dangling_node_to_id_maps_;
  HeapHashMap<int, Member<Node>> id_to_node_;
  HeapHashMap<int, Member<NodeToIdMap>> id_to_nodes_map_;
  HeapHashMap<WeakMember<Node>, Member<InspectorSourceLocation>>
      node_to_creation_source_location_map_;
  HashSet<int> children_requested_;
  HashSet<int> distributed_nodes_requested_;
  HashMap<int, int> cached_child_count_;
  int last_node_id_;
  Member<Document> document_;
  using SearchResults =
      HeapHashMap<String, Member<GCedHeapVector<Member<Node>>>>;
  SearchResults search_results_;
  Member<InspectorRevalidateDOMTask> revalidate_task_;
  Member<InspectorHistory> history_;
  Member<DOMEditor> dom_editor_;
  bool suppress_attribute_modified_event_;
  InspectorAgentState::Boolean enabled_;
  InspectorAgentState::Integer include_whitespace_;
  InspectorAgentState::Boolean capture_node_stack_traces_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_DOM_AGENT_H_
