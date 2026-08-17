/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 * Copyright (C) 2003, 2004, 2005, 2006, 2007, 2008, 2009, 2010, 2011 Apple Inc.
 * All rights reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_STYLE_RESOLVER_STATE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_STYLE_RESOLVER_STATE_H_

#include "third_party/blink/renderer/core/animation/css/css_animation_update.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_property_name.h"
#include "third_party/blink/renderer/core/css/css_property_names.h"
#include "third_party/blink/renderer/core/css/css_to_length_conversion_data.h"
#include "third_party/blink/renderer/core/css/parser/css_parser_mode.h"
#include "third_party/blink/renderer/core/css/resolver/css_to_style_map.h"
#include "third_party/blink/renderer/core/css/resolver/element_resolve_context.h"
#include "third_party/blink/renderer/core/css/resolver/element_style_resources.h"
#include "third_party/blink/renderer/core/css/resolver/font_builder.h"
#include "third_party/blink/renderer/core/css/style_recalc_context.h"
#include "third_party/blink/renderer/core/css/style_request.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/dom/element.h"

namespace blink {

class AnchorEvaluator;
class ComputedStyle;
class FontDescription;
class PseudoElement;

// A per-element object which wraps an ElementResolveContext. It collects state
// throughout the process of computing the style. It also gives convenient
// access to other element-related information.
class CORE_EXPORT StyleResolverState {
  STACK_ALLOCATED();

 public:
  StyleResolverState(Document&,
                     Element&,
                     const StyleRecalcContext* = nullptr,
                     const StyleRequest& = StyleRequest());
  StyleResolverState(const StyleResolverState&) = delete;
  StyleResolverState& operator=(const StyleResolverState&) = delete;
  ~StyleResolverState();

  bool IsForPseudoElement() const {
    return pseudo_id_ != kPseudoIdNone || element_context_.GetPseudoElement();
  }
  bool IsInheritedForUnset(const CSSProperty& property) const;

  // In FontFaceSet and CanvasRenderingContext2D, we don't have an element to
  // grab the document from.  This is why we have to store the document
  // separately.
  Document& GetDocument() const { return *document_; }
  // Returns the element we are computing style for. This returns the same as
  // GetElement() unless this is a pseudo element request or we are resolving
  // style for an SVG element instantiated in a <use> shadow tree. This method
  // may return nullptr if it is a pseudo element request with no actual
  // PseudoElement present.
  Element* GetStyledElement() const { return styled_element_; }
  // These are all just pass-through methods to ElementResolveContext.
  Element& GetElement() const { return element_context_.GetElement(); }
  Element& GetUltimateOriginatingElementOrSelf() const {
    return element_context_.GetUltimateOriginatingElementOrSelf();
  }
  const Element* ParentElement() const {
    return element_context_.ParentElement();
  }
  const ComputedStyle* RootElementStyle() const {
    return element_context_.RootElementStyle();
  }
  EInsideLink ElementLinkState() const {
    return element_context_.ElementLinkState();
  }

  // See inside_link_.
  EInsideLink InsideLink() const;

  const ElementResolveContext& ElementContext() const {
    return element_context_;
  }

  void SetStyle(const ComputedStyle& style) {
    // FIXME: Improve RAII of StyleResolverState to remove this function.
    style_builder_.emplace(style);
    UpdateLengthConversionData();
  }

  // Initialize the style builder. source_for_noninherited holds initial values
  // to use for non-inherited properties. inherit_parent is simply the style to
  // inherit from (either implicitly or explicitly).
  void CreateNewStyle(
      const ComputedStyle& source_for_noninherited,
      const ComputedStyle& inherit_parent,
      ComputedStyleBuilderBase::IsAtShadowBoundary is_at_shadow_boundary =
          ComputedStyleBuilderBase::kNotAtShadowBoundary) {
    // FIXME: Improve RAII of StyleResolverState to remove this function.
    style_builder_.emplace(source_for_noninherited, inherit_parent,
                           is_at_shadow_boundary);
    UpdateLengthConversionData();
  }
  ComputedStyleBuilder& StyleBuilder() { return *style_builder_; }
  const ComputedStyleBuilder& StyleBuilder() const { return *style_builder_; }
  const ComputedStyle* TakeStyle();

  const CSSToLengthConversionData& CssToLengthConversionData() const {
    return css_to_length_conversion_data_;
  }
  CSSToLengthConversionData FontSizeConversionData();
  CSSToLengthConversionData UnzoomedLengthConversionData();

  CSSToLengthConversionData::Flags TakeLengthConversionFlags() {
    CSSToLengthConversionData::Flags flags = length_conversion_flags_;
    length_conversion_flags_ = 0;
    return flags;
  }

  void SetConversionFontSizes(
      const CSSToLengthConversionData::FontSizes& font_sizes) {
    css_to_length_conversion_data_.SetFontSizes(font_sizes);
  }
  void SetConversionZoom(float zoom) {
    css_to_length_conversion_data_.SetZoom(zoom);
  }

  CSSAnimationUpdate& AnimationUpdate() { return animation_update_; }
  const CSSAnimationUpdate& AnimationUpdate() const {
    return animation_update_;
  }

  Element* GetAnimatingElement() const;

  // Returns the pseudo element if the style resolution is targeting a pseudo
  // element, null otherwise.
  PseudoElement* GetPseudoElement() const;

  void SetParentStyle(const ComputedStyle*);
  void EnsureParentStyle();
  const ComputedStyle* ParentStyle() const { return parent_style_; }

  void SetLayoutParentStyle(const ComputedStyle*);
  const ComputedStyle* LayoutParentStyle() const {
    return layout_parent_style_;
  }

  void SetOldStyle(const ComputedStyle* old_style) { old_style_ = old_style; }
  const ComputedStyle* OldStyle() const { return old_style_; }

  ElementStyleResources& GetElementStyleResources() {
    return element_style_resources_;
  }

  void LoadPendingResources();

  // FIXME: Once styleImage can be made to not take a StyleResolverState
  // this convenience function should be removed. As-is, without this, call
  // sites are extremely verbose.
  StyleImage* GetStyleImage(CSSPropertyID property_id, const CSSValue& value) {
    return element_style_resources_.GetStyleImage(property_id, value);
  }
  SVGResource* GetSVGResource(CSSPropertyID, const cssvalue::CSSURIValue&);

  FontBuilder& GetFontBuilder() { return font_builder_; }
  const FontBuilder& GetFontBuilder() const { return font_builder_; }
  // FIXME: These exist as a primitive way to track mutations to font-related
  // properties on a ComputedStyle. As designed, these are very error-prone, as
  // some callers set these directly on the ComputedStyle w/o telling us.
  // Presumably we'll want to design a better wrapper around ComputedStyle for
  // tracking these mutations and separate it from StyleResolverState.
  const FontDescription& ParentFontDescription() const;

  void SetZoom(float);
  void SetEffectiveZoom(float);
  void SetWritingMode(WritingMode);
  void SetTextSizeAdjust(TextSizeAdjust);
  void SetTextOrientation(ETextOrientation);
  void SetPositionAnchor(ScopedCSSName*);
  void SetPositionAreaOffsets(const std::optional<PositionAreaOffsets>&);

  CSSParserMode GetParserMode() const;

  // If the input CSSValue is a CSSLightDarkValuePair, return the light or dark
  // CSSValue based on the UsedColorScheme. For all other values, just return a
  // reference to the passed value.
  const CSSValue& ResolveLightDarkPair(const CSSValue&);

  const ComputedStyle* OriginatingElementStyle() const {
    return originating_element_style_;
  }
  bool IsForHighlight() const { return is_for_highlight_; }
  bool UsesHighlightPseudoInheritance() const {
    return uses_highlight_pseudo_inheritance_;
  }
  // See StyleRecalcContext::is_outside_flat_tree.
  bool IsOutsideFlatTree() const {
    return style_recalc_context_ && style_recalc_context_->is_outside_flat_tree;
  }

  bool CanTriggerAnimations() const { return can_trigger_animations_; }

  bool HadNoMatchedProperties() const { return had_no_matched_properties_; }
  void SetHadNoMatchedProperties() { had_no_matched_properties_ = true; }

  // True if the cascade observed any  "animation" or "transition" properties,
  // or when such properties were found within non-matching container queries.
  //
  // The method is supposed to represent whether or not animations can be
  // affected by at least one of the style variations produced by evaluating
  // @container rules differently.
  bool CanAffectAnimations() const;

  // Mark the state to say that animations can be affected by at least one of
  // the style variations produced by evaluating @container rules differently.
  void SetConditionallyAffectsAnimations() {
    conditionally_affects_animations_ = true;
  }

  bool AffectsCompositorSnapshots() const {
    return affects_compositor_snapshots_;
  }
  void SetAffectsCompositorSnapshots() { affects_compositor_snapshots_ = true; }

  bool RejectedLegacyOverlapping() const {
    return rejected_legacy_overlapping_;
  }
  void SetRejectedLegacyOverlapping() { rejected_legacy_overlapping_ = true; }

  // Update the Font object on the ComputedStyle and the CSSLengthResolver to
  // reflect applied font properties.
  void UpdateFont();

  // Update computed line-height and font used for 'lh' unit resolution.
  void UpdateLineHeight();

  void UpdateLengthConversionData();

  float TextAutosizingMultiplier() const {
    const ComputedStyle* old_style = GetElement().GetComputedStyle();
    if (!IsForPseudoElement() && old_style) {
      return old_style->TextAutosizingMultiplier();
    } else {
      return 1.0f;
    }
  }

  void SetHasTreeScopedReference() { has_tree_scoped_reference_ = true; }
  bool HasTreeScopedReference() const { return has_tree_scoped_reference_; }

  void SetHasUnsupportedGuaranteedInvalid() {
    has_unsupported_guaranteed_invalid_ = true;
  }
  bool HasUnsupportedGuaranteedInvalid() const {
    return has_unsupported_guaranteed_invalid_;
  }

  // The element to start the search from, when looking for a CQ size container.
  Element* NearestSizeContainer() const {
    return style_recalc_context_ ? style_recalc_context_->container : nullptr;
  }

  // See StyleRequest.pseudo_id.
  PseudoId GetPseudoId() const { return pseudo_id_; }

 private:
  CSSToLengthConversionData UnzoomedLengthConversionData(const FontSizeStyle&);
  // When resolving cq* units, this element is used to start the search
  // for suitable size containers.
  Element* ContainerUnitContext() const;
  // See StyleRecalcContext::GetAnchorEvaluator().
  AnchorEvaluator* GetAnchorEvaluator() const;

  ElementResolveContext element_context_;
  const StyleRecalcContext* style_recalc_context_ = nullptr;
  Document* document_;

  // The primary output for each element's style resolve.
  std::optional<ComputedStyleBuilder> style_builder_;

  CSSToLengthConversionData::Flags length_conversion_flags_ = 0;
  CSSToLengthConversionData css_to_length_conversion_data_;

  // parent_style_ is not always just ElementResolveContext::ParentStyle(),
  // so we keep it separate.
  const ComputedStyle* parent_style_;
  // This will almost-always be the same that parent_style_, except in the
  // presence of display: contents. This is the style against which we have to
  // do adjustment.
  const ComputedStyle* layout_parent_style_;
  // The ComputedStyle stored on the element before the current lifecycle update
  // started.
  const ComputedStyle* old_style_;

  CSSAnimationUpdate animation_update_;
  StyleRequest::RequestType pseudo_request_type_;

  FontBuilder font_builder_;

  // May be different than GetElement() if the element being styled is a pseudo
  // element or an instantiation via an SVG <use> element. In those cases,
  // GetElement() returns the originating element, or the element instatiated
  // from respectively.
  Element* styled_element_;

  ElementStyleResources element_style_resources_;
  // See StyleRequest.pseudo_id.
  PseudoId pseudo_id_ = kPseudoIdNone;

  // Whether this element is inside a link or not. Note that this is different
  // from ElementLinkState() if the element is not a link itself but is inside
  // one. It may also be overridden from non-visited to visited by devtools.
  // This will eventually get stored on ComputedStyle, but since we do not have
  // a ComputedStyle until pretty late in the process, keep it here until
  // we have one.
  //
  // This is computed only once, lazily (thus the std::optional).
  mutable std::optional<EInsideLink> inside_link_;

  const ComputedStyle* originating_element_style_;
  // True if we are resolving styles for a highlight pseudo-element.
  const bool is_for_highlight_;
  // True if this is a highlight style request, and highlight inheritance
  // should be used for this highlight pseudo.
  const bool uses_highlight_pseudo_inheritance_;

  // True if this style resolution can start or stop animations and transitions.
  // One case where animations and transitions can not be triggered is when we
  // resolve FirstLineInherited style for an element on the first line. Styles
  // inherited from the ::first-line styles should not cause transitions to
  // start on such elements. Still, animations and transitions in progress still
  // need to apply the effect for theses styles as well.
  bool can_trigger_animations_ = false;

  // Set to true if a given style resolve produced an empty MatchResult.
  // This is used to return a nullptr style for pseudo-element style
  // resolves.
  bool had_no_matched_properties_ = false;

  // True whenever a matching rule in a non-matching container query contains
  // any properties that can affect animations or transitions.
  bool conditionally_affects_animations_ = false;

  // True if snapshots of composited keyframes require re-validation.
  bool affects_compositor_snapshots_ = false;

  // True if the cascade rejected any properties with the kLegacyOverlapping
  // flag.
  bool rejected_legacy_overlapping_ = false;

  // True if the resolved ComputedStyle depends on tree-scoped references.
  bool has_tree_scoped_reference_ = false;

  // Tried to apply a guaranteed-invalid value to a custom property that doesn't
  // support it.
  bool has_unsupported_guaranteed_invalid_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_STYLE_RESOLVER_STATE_H_
