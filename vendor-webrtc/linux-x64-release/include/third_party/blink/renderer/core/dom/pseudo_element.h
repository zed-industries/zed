/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_PSEUDO_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_PSEUDO_ELEMENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class ComputedStyle;

class CORE_EXPORT PseudoElement : public Element {
 public:
  // |view_transition_name| is used to uniquely identify a pseudo element
  // from a set of pseudo elements which share the same |pseudo_id|. The current
  // usage of this ID is limited to pseudo elements generated for a
  // ViewTransition. See
  // third_party/blink/renderer/core/view_transition/README.md
  static PseudoElement* Create(
      Element* parent,
      PseudoId pseudo_id,
      const AtomicString& view_transition_name = g_null_atom);

  PseudoElement(Element*,
                PseudoId,
                const AtomicString& view_transition_name = g_null_atom);

  bool IsPseudoElement() const final { return true; }

  const AtomicString& view_transition_name() const {
    return view_transition_name_;
  }
  const ComputedStyle* CustomStyleForLayoutObject(
      const StyleRecalcContext&) override;
  void AttachLayoutTree(AttachContext&) override;
  bool LayoutObjectIsNeeded(const DisplayStyle&) const override;
  bool CanGeneratePseudoElement(PseudoId) const override;

  bool CanGenerateContent() const;
  bool CanHaveNestedPseudoElement() const;
  bool CanStartSelection() const override { return false; }
  bool CanContainRangeEndPoint() const override { return false; }
  PseudoId GetPseudoId() const override { return pseudo_id_; }
  // PseudoId that can be alias, e.g. kPseudoScrollMarkerGroupAfter is
  // unresolved = alias, kPseudoScrollMarkerGroup is resolved.
  // For styling and selector matching, return resolved version.
  PseudoId GetPseudoIdForStyling() const override;
  const AtomicString& GetPseudoArgument() const override {
    return view_transition_name_;
  }

  // Return the adjusted style needed by layout. In some cases computed style
  // cannot be used as-is by layout. display:contents needs to be adjusted to
  // display:inline. Scroll marker pseudo elements may need to blockify the
  // display type (depending on the parent). Returns nullptr if no adjustment is
  // necessary.
  const ComputedStyle* AdjustedLayoutStyle(
      const ComputedStyle& style,
      const ComputedStyle& layout_parent_style);

  static AtomicString PseudoElementNameForEvents(Element*);
  static bool IsWebExposed(PseudoId, const Node*);

  // Pseudo elements are not allowed to be the inner node for hit testing.
  // Find the closest ancestor which is a real dom node.
  virtual Node* InnerNodeForHitTesting();

  void AccessKeyAction(SimulatedClickCreationScope creation_scope) override;

  // Returns the DOM element that this pseudo element originates from. If the
  // pseudo element is nested inside another pseudo element, this returns the
  // DOM element which the pseudo element tree originates from.
  // This is different from |parentElement()| which returns the element's direct
  // ancestor.
  Element& UltimateOriginatingElement() const;

  virtual void Dispose();

  static bool IsLayoutSiblingOfOriginatingElement(PseudoId pseudo_id);

  bool IsInertRoot() const override;

 protected:
  void SetIsGeneratedName(bool generated) { is_generated_name_ = generated; }

 private:
  class AttachLayoutTreeScope {
    STACK_ALLOCATED();

   public:
    AttachLayoutTreeScope(PseudoElement*, const AttachContext&);
    ~AttachLayoutTreeScope();

   private:
    PseudoElement* element_;
    const ComputedStyle* original_style_{nullptr};
  };

  PseudoId pseudo_id_;
  const AtomicString view_transition_name_;
  bool is_generated_name_ = false;
};

CORE_EXPORT const QualifiedName& PseudoElementTagName(PseudoId);

bool PseudoElementLayoutObjectIsNeeded(PseudoId pseudo_id,
                                       const ComputedStyle* pseudo_style,
                                       const Element* originating_element);
bool PseudoElementLayoutObjectIsNeeded(PseudoId pseudo_id,
                                       const DisplayStyle& pseudo_style,
                                       const Element* originating_element);

template <>
struct DowncastTraits<PseudoElement> {
  static bool AllowFrom(const Node& node) { return node.IsPseudoElement(); }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_PSEUDO_ELEMENT_H_
