// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_SCROLLING_ROOT_SCROLLER_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_SCROLLING_ROOT_SCROLLER_CONTROLLER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class Document;
class Element;
class HTMLFrameOwnerElement;
class Node;

// Manages the root scroller associated with a given document. The root
// scroller causes browser controls movement, overscroll effects and prevents
// chaining scrolls up further in the DOM. It can be set from script using
// document.setRootScroller. High-level details are available in README.md.
//
// There are two notions of a root scroller in this class: m_rootScroller and
// m_effectiveRootScroller. The former is the Element that was set as the root
// scroller using document.setRootScroller. If the page didn't set a root
// scroller this will be nullptr. The "effective" root scroller is the current
// Node we're using internally to apply viewport scrolling actions.  Both these
// elements come from this controller's associated Document. The final "global"
// root scroller, the one whose scrolling hides browser controls, may be in a
// different frame.
//
// If the currently set m_rootScroller is a valid element to become the root
// scroller, it will be promoted to the effective root scroller. If it is not
// valid, the effective root scroller will fall back to the document Node. The
// rules for what makes an element a valid root scroller are set in
// isValidRootScroller(). The validity of the current root scroller is
// re-checked after layout as part of the document lifecycle.
class CORE_EXPORT RootScrollerController
    : public GarbageCollected<RootScrollerController> {
 public:
  explicit RootScrollerController(Document&);

  void Trace(Visitor*) const;

  // This returns the Element that's actually being used to control viewport
  // actions right now. See README.md for the difference between the effective
  // root scroller and the global root scroller in
  // TopDocumentRootScrollerController.
  Node& EffectiveRootScroller() const;

  // This class needs to be informed when the FrameView of its Document changes
  // size. This may occur without a layout (e.g. URL bar hiding) so we can't
  // rely on DidUpdateMainFrameLayout.
  void DidResizeFrameView();

  // Called when an iframe in this document has an updated FrameView (e.g.
  // FrameView removed, swapped, etc.) so that we can recompute the effective
  // root scroller and set the appropriate properties on the view.
  void DidUpdateIFrameFrameView(HTMLFrameOwnerElement&);

  void ElementRemoved(const Element&);

  // In the "implicit root scroller" mode, we might promote an element to
  // become the effective root scroller even if the page doesn't set it as so
  // to improve the user experience.  In this mode, as elements layout they'll
  // call this method and, if they meet the root scroller restrictions, will be
  // added to the implicit candidate set. After layout is done we'll go
  // through that set and select the best candidate.
  void ConsiderForImplicit(Node&);

  // Called as part of the main document lifecycle. This will iterate the frame
  // tree in post order and select the effective root scroller in each frame.
  // Returns true if root scroller selection changed.
  bool PerformRootScrollerSelection();

 private:
  // Ensures the effective root scroller is currently valid and replaces it
  // with the default if not. Returns true if the effective root scroller
  // changed.
  bool RecomputeEffectiveRootScroller();

  // Determines whether the given element meets the criteria to become the
  // effective root scroller.
  bool IsValidRootScroller(const Element&) const;

  // Determines whether the given element meets the criteria to be implicitly
  // set as the root scroller (in addition to being a valid root scroller).
  bool IsValidImplicit(const Element&) const;

  // Determines whether the given element is eligable as a candidate to be
  // implicitly promoted. Intuitively, thiis is any "live" scroller on the page.
  // We add these to a list of candidates and after layout we go through the
  // list and promote the best candidate that satisfies the more exhaustive
  // conditions set by IsValidImplicit above. At that time we also prune the
  // list of any elements that no longer satisfy IsValidImplicitCandidate.
  bool IsValidImplicitCandidate(const Element&) const;

  // Set certain properties to the effective root scroller. Called when a Node
  // becomes or unbecomes the effective root scroller. Calling this method can
  // leave the node's frame with a dirty layout due to the fact that layout size
  // depends on whether the element is the effective root scroller or not.
  void ApplyRootScrollerProperties(Node&);

  void UpdateIFrameGeometryAndLayoutSize(HTMLFrameOwnerElement&) const;

  // Called after layout, runs through implicit candidates, removing ones that
  // are no longer meet the root scroller restrictions. Of the remaining ones,
  // will choose the best and return it.
  Element* ImplicitRootScrollerFromCandidates();

  // Calls function for each non-throttled frame's RootScrollerController in
  // post tree order.
  template <typename Function>
  void ForAllNonThrottledLocalControllers(const Function&);

  // The owning Document whose root scroller this object manages.
  WeakMember<Document> document_;

  // The Node currently being used as the root scroller in this Document.
  // If the m_rootScroller is valid this will point to it. Otherwise, it'll
  // use the document Node. It'll never be nullptr since the Document owns the
  // RootScrollerController.
  Member<Node> effective_root_scroller_;

  // Candidate Elements that we should examine after layout to determine which
  // should be root scroller. This is used when "implicit root scroller" is
  // enabled, where a valid Element can become the root scroller.
  HeapHashSet<WeakMember<Element>> implicit_candidates_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_SCROLLING_ROOT_SCROLLER_CONTROLLER_H_
