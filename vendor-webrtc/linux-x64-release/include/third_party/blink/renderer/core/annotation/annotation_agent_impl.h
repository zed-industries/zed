// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANNOTATION_ANNOTATION_AGENT_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANNOTATION_ANNOTATION_AGENT_IMPL_H_

#include "base/types/pass_key.h"
#include "third_party/blink/public/mojom/annotation/annotation.mojom-blink.h"
#include "third_party/blink/public/mojom/scroll/scroll_enums.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/scroll/scroll_into_view_params.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/geometry/physical_rect.h"
#include "third_party/blink/renderer/platform/graphics/dom_node_id.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class AnnotationAgentContainerImpl;
class AnnotationAgentImplTest;
class AnnotationSelector;
class Document;
class RangeInFlatTree;

// This class represents an instantiation of an annotation in a Document. It is
// always owned and stored in an AnnotationAgentContainerImpl. Removing it from
// the container will remove any visible effects on content and disconnect mojo
// bindings. Once removed, an agent cannot be reused.
//
// AnnotationAgentImpl allows its client to provide a selector that it uses to
// "attach" to a particular Range in the Document. Once attached, the
// annotation adds a visible marker to the content and enables the client to
// scroll the attached content into view. An agent is considered attached if it
// has found a Range and that Range is valid and uncollapsed. The range may
// become invalid in response to changes in a Document's DOM.
// TODO(bokan): Changes in the DOM affecting an annotation should be signaled
// via the AnnotationAgentHost interface.
//
// This class is the renderer end of an annotation. It can be instantiated
// directly from Blink as well in which case it need not be bound to a host on
// the browser side. If bound to a corresponding AnnotationAgentHost in the
// browser, it will notify the host of relevant events (e.g. attachment
// succeeded/failed) and self-remove itself from the container if the mojo
// bindings become disconnected (i.e. if the browser closes the connection, the
// AnnotationAgentImpl will be removed).
class CORE_EXPORT AnnotationAgentImpl final
    : public GarbageCollected<AnnotationAgentImpl>,
      public mojom::blink::AnnotationAgent {
 public:
  using PassKey = base::PassKey<AnnotationAgentImpl>;

  // Agents can only be created via AnnotationAgentContainerImpl.
  AnnotationAgentImpl(AnnotationAgentContainerImpl& owning_container,
                      mojom::blink::AnnotationType annotation_type,
                      AnnotationSelector& selector,
                      std::optional<DOMNodeId> search_range_start_node_id,
                      base::PassKey<AnnotationAgentContainerImpl>);
  ~AnnotationAgentImpl() override = default;

  // Non-copyable or moveable
  AnnotationAgentImpl(const AnnotationAgentImpl&) = delete;
  AnnotationAgentImpl& operator=(const AnnotationAgentImpl&) = delete;

  void Trace(Visitor* visitor) const;

  // Binds this agent to a host.
  void Bind(
      mojo::PendingRemote<mojom::blink::AnnotationAgentHost> host_remote,
      mojo::PendingReceiver<mojom::blink::AnnotationAgent> agent_receiver);

  // Attempts to find a Range of DOM matching the search criteria of the
  // AnnotationSelector passed in the constructor. The DOM search is performed
  // synchronously but if that match is in a hidden subtree
  // (content-visibility: auto, <details>, hidden=until-found) that can be
  // shown, attachment will complete asynchronously once the subtree is made
  // visible. This is only called by the AnnotationAgentContainer immediately
  // after layout is completed (clients should use SetNeedsAttachment to
  // request an attachment if the initial one failed).
  void Attach(base::PassKey<AnnotationAgentContainerImpl>);

  // Clients can request an attachment (after the automatic first one has
  // failed) to occur using this setter.
  void SetNeedsAttachment() { needs_attachment_ = true; }
  bool NeedsAttachment() const { return needs_attachment_; }

  // Returns true if the agent has performed attachment and resulted in a valid
  // DOM Range. Note that Range is relocated, meaning that it will update in
  // response to changes in DOM. Hence, an AnnotationAgent that IsAttached may
  // become detached due to changes in the Document.
  bool IsAttached() const;

  // Returns true if the agent has found the requested range but is waiting
  // on DOM mutations before attaching. For example, if the range is in a hidden
  // <details> element, the agent will be in a pending state until the <details>
  // is opened in the next animation frame.
  bool IsAttachmentPending() const;

  // Returns true if this agent is bound to a host.
  bool IsBoundForTesting() const;

  // Removes the agent from its container, clearing all its state, mojo
  // bindings, and any visual indications in the document.
  void Remove();

  // mojom::blink::AnnotationAgent
  void ScrollIntoView(bool applies_focus) override {
    const_cast<const AnnotationAgentImpl*>(this)->ScrollIntoView(applies_focus);
  }
  void ScrollIntoView(bool applies_focus) const;

  const RangeInFlatTree& GetAttachedRange() const {
    CHECK(attached_range_.Get());
    return *attached_range_.Get();
  }

  const AnnotationSelector* GetSelector() const { return selector_.Get(); }

  mojom::blink::AnnotationType GetType() const { return type_; }

 private:
  friend AnnotationAgentImplTest;

  // Callback for when AnnotationSelector::FindRange finishes. If needed, this
  // may post a task to perform DOM mutation for hidden=until-found and similar
  // "activate on find" DOM features before calling ProcessAttachmentFinished.
  // Otherwise, ProcessAttachmentFinished is called synchronously.
  void DidFinishFindRange(const RangeInFlatTree* range);

  bool NeedsDOMMutationToAttach() const;
  void PerformPreAttachDOMMutation();

  // This will add the highlight marker and respond to the host or callback as
  // needed.
  void ProcessAttachmentFinished();

  bool IsRemoved() const;

  mojom::blink::ScrollBehavior ComputeScrollIntoViewBehavior(
      const PhysicalRect& bounding_box,
      const mojom::blink::ScrollIntoViewParams& params) const;

  // Mojo bindings to the remote host and this' remote. These are always
  // connected as a pair and disconnecting one will cause the other to be
  // disconnected as well.
  HeapMojoRemote<mojom::blink::AnnotationAgentHost> agent_host_;
  HeapMojoReceiver<mojom::blink::AnnotationAgent, AnnotationAgentImpl>
      receiver_;

  // These will be cleared when the agent is removed from its container.
  Member<AnnotationAgentContainerImpl> owning_container_;
  Member<AnnotationSelector> selector_;

  // The attached_range_ is null until the agent performs a successful
  // Attach().
  Member<RangeInFlatTree> attached_range_;

  // In some cases attachment may be asynchronous, e.g. while waiting on a new
  // compositor frame to expand a hidden section. If text was found but is
  // waiting for such a "PreAttachDOMMutation", the range will be stored here
  // before being "attached" by transfer to `attached_range_`. At most one of
  // `attached_range_` or `pending_range_` will be non-null
  // TODO(bokan): This doesn't need to be const but is due to the
  // TextFragmentFinder::Client interface.
  Member<const RangeInFlatTree> pending_range_;

  // The start node id of the search range within which the agent will attempt
  // to match the selector in.
  std::optional<DOMNodeId> search_range_start_node_id_;

  // TODO(bokan): Once we have more of this implemented we'll use the type to
  // determine styling and context menu behavior.
  mojom::blink::AnnotationType type_;

  // Attachment is expensive so it's called only once from
  // PerformInitialAttachments. Clients can reset this value to try again (e.g.
  // to try and attach to newly added content).
  bool needs_attachment_ = true;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANNOTATION_ANNOTATION_AGENT_IMPL_H_
