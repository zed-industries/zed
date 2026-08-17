// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_VIEW_TRANSITION_VIEW_TRANSITION_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_VIEW_TRANSITION_VIEW_TRANSITION_UTILS_H_

#include "base/functional/function_ref.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/view_transition/view_transition_request_forward.h"
#include "third_party/blink/renderer/platform/heap/heap_traits.h"

namespace blink {

class DOMViewTransition;
class Document;
class Element;
class LayoutObject;
class Node;
class PseudoElement;
class ViewTransition;

class CORE_EXPORT ViewTransitionUtils {
 public:
  using PseudoFunctor = base::FunctionRef<void(PseudoElement*)>;
  using PseudoPredicate = base::FunctionRef<bool(PseudoElement*)>;

  static void ForEachTransitionPseudo(Element&, PseudoFunctor);
  static PseudoElement* FindPseudoIf(const Element&, PseudoPredicate);
  static void ForEachDirectTransitionPseudo(const Element*, PseudoFunctor);

  // Returns the view transition in-progress in the given document, if one
  // exists.
  static ViewTransition* GetTransition(const Document& document);

  static ViewTransition* GetTransition(const Element& element);

  static ViewTransition* GetTransition(const Node& node);

  // Returns the view transition that the element associated with the specified
  // layout object is participating in, if one exists.
  static ViewTransition* TransitionForTaggedElement(const LayoutObject&);

  // Calls the supplied function for every active transition (document-level or
  // element-scoped).
  // Note: making this a function template blows up compile size.
  // TODO(crbug.com/394052227): Consider converting other ForEach* methods in
  // this class to take base::FunctionRef instead of being templates.
  static void ForEachTransition(const Document& document,
                                base::FunctionRef<void(ViewTransition&)>);

  // Return the incoming cross-document view transition, if one exists.
  static ViewTransition* GetIncomingCrossDocumentTransition(
      const Document& document);

  // Return the outgoing cross-document view transition, if one exists.
  static ViewTransition* GetOutgoingCrossDocumentTransition(
      const Document& document);

  // If the given document has an in-progress view transition, this will return
  // the script delegate associated with that view transition (which may be
  // null).
  static DOMViewTransition* GetTransitionScriptDelegate(
      const Document& document);

  // Returns the ::view-transition pseudo element that is the root of the
  // view-transition DOM hierarchy.
  static PseudoElement* GetRootPseudo(const Document& document);

  // Returns any queued view transition requests.
  static VectorOf<std::unique_ptr<ViewTransitionRequest>> GetPendingRequests(
      const Document& document);

  // Returns true if the given layout object corresponds to the root
  // ::view-transition pseudo element of a view transition hierarchy.
  static bool IsViewTransitionRoot(const LayoutObject& object);

  // Returns true if this element is a view transition participant. This is a
  // slow check that walks all of the view transition elements in the
  // ViewTransitionStyleTracker.
  static bool IsViewTransitionElementExcludingRootFromSupplement(
      const Element& element);

  // Returns true if this object represents an element that is a view transition
  // participant. This is a slow check that walks all of the view transition
  // elements in the ViewTransitionStyleTracker.
  static bool IsViewTransitionParticipantFromSupplement(
      const LayoutObject& object);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_VIEW_TRANSITION_VIEW_TRANSITION_UTILS_H_
