/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2001 Peter Kelly (pmk@post.com)
 *           (C) 2001 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2003-2011, 2013, 2014 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ELEMENT_H_

#include "base/check_op.h"
#include "base/dcheck_is_on.h"
#include "base/gtest_prod_util.h"
#include "base/types/pass_key.h"
#include "third_party/blink/public/common/input/pointer_id.h"
#include "third_party/blink/public/common/metrics/document_update_reason.h"
#include "third_party/blink/public/mojom/scroll/scroll_into_view_params.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_typedefs.h"
#include "third_party/blink/renderer/core/animation/animatable.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_primitive_value.h"
#include "third_party/blink/renderer/core/css/css_property_value.h"
#include "third_party/blink/renderer/core/css/css_selector.h"
#include "third_party/blink/renderer/core/css/resolver/cascade_filter.h"
#include "third_party/blink/renderer/core/css/style_recalc_change.h"
#include "third_party/blink/renderer/core/css/style_request.h"
#include "third_party/blink/renderer/core/dom/container_node.h"
#include "third_party/blink/renderer/core/dom/dom_high_res_time_stamp.h"
#include "third_party/blink/renderer/core/dom/element_data.h"
#include "third_party/blink/renderer/core/dom/element_rare_data_field.h"
#include "third_party/blink/renderer/core/dom/events/simulated_click_options.h"
#include "third_party/blink/renderer/core/dom/focusgroup_flags.h"
#include "third_party/blink/renderer/core/dom/names_map.h"
#include "third_party/blink/renderer/core/dom/node.h"
#include "third_party/blink/renderer/core/dom/whitespace_attacher.h"
#include "third_party/blink/renderer/core/html_names.h"
#include "third_party/blink/renderer/core/trustedtypes/trusted_types_util.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/bindings/transform_view.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_linked_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/region_capture_crop_id.h"
#include "third_party/blink/renderer/platform/restriction_target_id.h"
#include "third_party/blink/renderer/platform/text/text_direction.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_table.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"
#include "ui/gfx/geometry/rect_f.h"

namespace gfx {
class QuadF;
class Vector2dF;
}  // namespace gfx

namespace blink {

class AnchorElementObserver;
class AnchorPositionScrollData;
class AriaNotificationOptions;
class Attr;
class Attribute;
class ColumnPseudoElement;
class ContainerQueryData;
class ContainerQueryEvaluator;
class CSSPropertyName;
class CSSPropertyValueSet;
class CSSStyleDeclaration;
class CustomElementDefinition;
class CustomElementRegistry;
class DOMRect;
class DOMRectList;
class DOMStringMap;
class DOMTokenList;
class DisplayLockContext;
class DisplayStyle;
class Document;
class EditContext;
class ElementAnimations;
class ElementInternals;
class ElementIntersectionObserverData;
class ElementRareDataVector;
class ExceptionState;
class FocusOptions;
class HTMLElement;
class HTMLTemplateElement;
class Image;
class InputDeviceCapabilities;
class InvokerData;
class InterestInvokerTargetData;
class KURL;
class Locale;
class MutableCSSPropertyValueSet;
class NamedNodeMap;
class OutOfFlowData;
class PointerLockOptions;
class PopoverData;
class PseudoElement;
class ResizeObservation;
class ResizeObserver;
class ResizeObserverSize;
class ScrollIntoViewOptions;
class CheckVisibilityOptions;
class ScrollMarkerGroupData;
class ScrollMarkerPseudoElement;
class ScrollToOptions;
class SetHTMLOptions;
class SetHTMLUnsafeOptions;
class ShadowRoot;
class ShadowRootInit;
class SpaceSplitString;
class StyleEngine;
class StyleHighlightData;
class StylePropertyMap;
class StylePropertyMapReadOnly;
class StyleRecalcContext;
class StyleScopeData;
class TextVisitor;
class V8UnionBooleanOrScrollIntoViewOptions;
class ComputedStyleBuilder;
class StyleAdjuster;

template <typename IDLType>
class FrozenArray;

enum class CSSPropertyID;
enum class CSSValueID;
enum class DisplayLockActivationReason;
enum class DocumentUpdateReason;

struct FocusParams;

using ScrollOffset = gfx::Vector2dF;

struct AttributeToNameTransform {
  String operator()(const Attribute& attr) const {
    return attr.GetName().ToString();
  }
};

using AttributeNamesView =
    bindings::TransformedView<AttributeCollection, AttributeToNameTransform>;

using ColumnPseudoElementsVector = GCedHeapVector<Member<ColumnPseudoElement>>;

enum SpellcheckAttributeState {
  kSpellcheckAttributeTrue,
  kSpellcheckAttributeFalse,
  kSpellcheckAttributeDefault
};

enum class ElementFlags {
  kTabIndexWasSetExplicitly = 1 << 0,
  kStyleAffectedByEmpty = 1 << 1,
  kIsInCanvasSubtree = 1 << 2,
  kContainsFullScreenElement = 1 << 3,
  kIsInTopLayer = 1 << 4,
  kContainsPersistentVideo = 1 << 5,
  kIsEligibleForElementCapture = 1 << 6,
  kHasCheckedElementCaptureEligibility = 1 << 7,

  kNumberOfElementFlags = 8,  // Size of bitfield used to store the flags.
};

enum class ShadowRootMode;

enum class SlotAssignmentMode { kManual, kNamed };
enum class FocusDelegation { kNone, kDelegateFocus };

enum class SelectionBehaviorOnFocus {
  kReset,
  kRestore,
  kNone,
};

enum class FocusableState {
  kNotFocusable,
  kFocusable,
  kKeyboardFocusableScroller,
};

// https://html.spec.whatwg.org/C/#dom-document-nameditem-filter
enum class NamedItemType {
  kNone,
  kName,
  kNameOrId,
  kNameOrIdWithName,
};

enum class CommandEventType {
  // Action is neither custom, nor built-in (effectively invalid)
  kNone,

  // Custom actions include a `-`.
  kCustom,

  // Popover
  kTogglePopover,
  kHidePopover,
  kShowPopover,
  // Dialog
  kShowModal,
  kClose,
  kRequestClose,
  // Details
  kToggle,
  kOpen,
  // kClose
  // Input / Select
  kShowPicker,
  // Number Input
  kStepUp,
  kStepDown,
  // Fullscreen
  kToggleFullscreen,
  kRequestFullscreen,
  kExitFullscreen,
  // Audio/Video
  kPlayPause,
  kPause,
  kPlay,
  kToggleMuted,
};

typedef HeapVector<Member<Attr>> AttrNodeList;

typedef HashMap<AtomicString, SpecificTrustedType> AttrNameToTrustedType;

class CORE_EXPORT Element : public ContainerNode, public Animatable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  Element(const QualifiedName& tag_name,
          Document*,
          ConstructionType = kCreateElement);

  // ParseDeclarativeShadowRoots specifies whether declarative shadow roots
  // should be parsed by the HTML parser.
  enum class ParseDeclarativeShadowRoots {
    kDontParse = 0,
    kParse = 1,
  };
  // ForceHtml specifies whether the HTML parser should be used when parsing
  // markup even if we are in an XML document.
  enum class ForceHtml {
    kDontForce = 0,
    kForce = 1,
  };

  // Animatable implementation.
  Element* GetAnimationTarget() override;

  DEFINE_ATTRIBUTE_EVENT_LISTENER(beforecopy, kBeforecopy)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(beforecut, kBeforecut)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(beforepaste, kBeforepaste)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(search, kSearch)

  bool hasAttribute(const QualifiedName&) const;
  const AtomicString& getAttribute(const QualifiedName&) const;

  // Set an attribute without Trusted Type validation. Passing g_null_atom
  // is the same as removing the attribute. This should only be used directly
  // if we know the `QualifiedName` is not a special attribute.
  // TODO(crbug.com/374263390): Rename this method and audit callers.
  void setAttribute(const QualifiedName& name, const AtomicString& value) {
    SetAttributeWithoutValidation(name, value);
  }

  // Set an attribute without Trusted Type validation. Passing g_null_atom
  // is the same as removing the attribute. This should only be used directly
  // if we know the `QualifiedName` is not a special attribute or the value
  // has already been validated.
  void SetAttributeWithoutValidation(const QualifiedName&,
                                     const AtomicString& value);

  void SetAttributeWithoutValidation(const QualifiedName& name,
                                     const String& value) {
    SetAttributeWithoutValidation(name, AtomicString(value));
  }

  // Set an attribute with Trusted Type validation. Passing g_null_atom
  // is the same as removing the attribute.
  void SetAttributeWithValidation(Attr*,
                                  const AtomicString& value,
                                  ExceptionState&);

  // TODO(crbug.com/374263390): This method should likely CHECK if
  // QualifiedName is a trusted type.
  void SetSynchronizedLazyAttribute(const QualifiedName&,
                                    const AtomicString& value);

  void removeAttribute(const QualifiedName&);

  // Typed getters and setters for language bindings.
  int GetIntegralAttribute(const QualifiedName& attribute_name) const;
  int GetIntegralAttribute(const QualifiedName& attribute_name,
                           int default_value) const;
  unsigned int GetUnsignedIntegralAttribute(
      const QualifiedName& attribute_name) const;
  void SetIntegralAttribute(const QualifiedName& attribute_name, int value);
  void SetUnsignedIntegralAttribute(const QualifiedName& attribute_name,
                                    unsigned value,
                                    unsigned default_value = 0);
  double GetFloatingPointAttribute(
      const QualifiedName& attribute_name,
      double fallback_value = std::numeric_limits<double>::quiet_NaN()) const;
  void SetFloatingPointAttribute(const QualifiedName& attribute_name,
                                 double value);

  // If this element hosts a shadow root with a referenceTarget, returns the
  // target element inside the shadow root. In the case where there are multiple
  // nested layers of shadow roots, returns the innermost target element.
  Element* GetShadowReferenceTarget(const QualifiedName& name) const;

  // If this element hosts a shadow root with a referenceTarget, returns the
  // target element inside the shadow root (recursively). If at any layer of
  // shadow root, referenceTarget is specified but the ID is invalid, returns
  // nullptr. In other cases, return this element.
  Element* GetShadowReferenceTargetOrSelf(const QualifiedName& name);

  // Returns true if |this| element has attr-associated elements that were set
  // via the IDL, rather than computed from the content attribute.
  // See
  // https://whatpr.org/html/3917/common-dom-interfaces.html#reflecting-content-attributes-in-idl-attributes:element
  // for more information.
  // This is only exposed as an implementation detail to AXRelationCache, which
  // computes aria-owns differently for element reflection.
  bool HasAnyExplicitlySetAttrAssociatedElements() const;
  bool HasExplicitlySetAttrAssociatedElements(const QualifiedName& name) const;
  GCedHeapLinkedHashSet<WeakMember<Element>>* GetExplicitlySetElementsForAttr(
      const QualifiedName& name) const;

  Element* GetElementAttribute(const QualifiedName& name) const;
  Element* GetElementAttributeResolvingReferenceTarget(
      const QualifiedName& name) const;
  void SetElementAttribute(const QualifiedName&, Element*);
  GCedHeapVector<Member<Element>>* GetAttrAssociatedElements(
      const QualifiedName& name,
      bool resolve_reference_target) const;

  // If treescope_element is connected, then we will search treescope_element's
  // TreeScope for an element with the id. If treescope_element is disconnected,
  // then we will use its TreeRoot() to search for an element with the id
  // instead.
  Element* getElementByIdIncludingDisconnected(const Element& treescope_element,
                                               const AtomicString& id) const;

  FrozenArray<Element>* ariaControlsElements();
  void setAriaControlsElements(GCedHeapVector<Member<Element>>* given_elements);
  FrozenArray<Element>* ariaDescribedByElements();
  void setAriaDescribedByElements(
      GCedHeapVector<Member<Element>>* given_elements);
  FrozenArray<Element>* ariaDetailsElements();
  void setAriaDetailsElements(GCedHeapVector<Member<Element>>* given_elements);
  FrozenArray<Element>* ariaErrorMessageElements();
  void setAriaErrorMessageElements(
      GCedHeapVector<Member<Element>>* given_elements);
  FrozenArray<Element>* ariaFlowToElements();
  void setAriaFlowToElements(GCedHeapVector<Member<Element>>* given_elements);
  FrozenArray<Element>* ariaLabelledByElements();
  void setAriaLabelledByElements(
      GCedHeapVector<Member<Element>>* given_elements);
  FrozenArray<Element>* ariaOwnsElements();
  void setAriaOwnsElements(GCedHeapVector<Member<Element>>* given_elements);

  // Call this to get the value of an attribute that is known not to be the
  // style attribute or one of the SVG animatable attributes.
  bool FastHasAttribute(const QualifiedName&) const;
  const AtomicString& FastGetAttribute(const QualifiedName&) const;
#if DCHECK_IS_ON()
  bool FastAttributeLookupAllowed(const QualifiedName&) const;
#endif

#if DUMP_NODE_STATISTICS
  bool HasNamedNodeMap() const;
#endif
  bool hasAttributes() const;

  bool hasAttribute(const AtomicString& name) const;
  bool hasAttributeNS(const AtomicString& namespace_uri,
                      const AtomicString& local_name) const;

  // Ignores namespace.
  bool HasAttributeIgnoringNamespace(const AtomicString& local_name) const;

  const AtomicString& getAttribute(const AtomicString& local_name) const {
    return GetAttributeHinted(local_name, WeakLowercaseIfNecessary(local_name));
  }

  const AtomicString& getAttributeNS(const AtomicString& namespace_uri,
                                     const AtomicString& local_name) const;

  void setAttribute(AtomicString name,
                    String value,
                    ExceptionState& exception_state = ASSERT_NO_EXCEPTION) {
    WTF::AtomicStringTable::WeakResult weak_lowercase_name =
        WeakLowercaseIfNecessary(name);
    SetAttributeHinted(std::move(name), weak_lowercase_name, std::move(value),
                       exception_state);
  }

  // Trusted Types variant for explicit setAttribute() use.
  void setAttribute(AtomicString name,
                    const V8TrustedType* trusted_string,
                    ExceptionState& exception_state) {
    WTF::AtomicStringTable::WeakResult weak_lowercase_name =
        WeakLowercaseIfNecessary(name);
    SetAttributeHinted(std::move(name), weak_lowercase_name, trusted_string,
                       exception_state);
  }

  // Returns attributes that should be checked against Trusted Types
  virtual const AttrNameToTrustedType& GetCheckedAttributeTypes() const;

  static std::optional<QualifiedName> ParseAttributeName(
      const AtomicString& namespace_uri,
      const AtomicString& qualified_name,
      ExceptionState&);
  void setAttributeNS(const AtomicString& namespace_uri,
                      const AtomicString& qualified_name,
                      String value,
                      ExceptionState& exception_state);
  void setAttributeNS(const AtomicString& namespace_uri,
                      const AtomicString& qualified_name,
                      const V8TrustedType* trusted_string,
                      ExceptionState& exception_state);

  bool toggleAttribute(const AtomicString&, ExceptionState&);
  bool toggleAttribute(const AtomicString&, bool force, ExceptionState&);

  const AtomicString& GetIdAttribute() const;
  void SetIdAttribute(const AtomicString&);

  const AtomicString& GetNameAttribute() const;
  const AtomicString& GetClassAttribute() const;

  // This is an operation defined in the DOM standard like:
  //   If element is in the HTML namespace and its node document is an HTML
  //   document, then set qualifiedName to qualifiedName in ASCII lowercase.
  //   https://dom.spec.whatwg.org/#concept-element-attributes-get-by-name
  AtomicString LowercaseIfNecessary(AtomicString) const;
  WTF::AtomicStringTable::WeakResult WeakLowercaseIfNecessary(
      const AtomicString&) const;

  // NoncedElement implementation: this is only used by HTMLElement and
  // SVGElement, but putting the implementation here allows us to use
  // ElementRareData to hold the data.
  const AtomicString& nonce() const;
  void setNonce(const AtomicString&);

  // Call this to get the value of the id attribute for style resolution
  // purposes.  The value will already be lowercased if the document is in
  // compatibility mode, so this function is not suitable for non-style uses.
  const AtomicString& IdForStyleResolution() const;

  // This getter takes care of synchronizing all attributes before returning the
  // AttributeCollection. If the Element has no attributes, an empty
  // AttributeCollection will be returned. This is not a trivial getter and its
  // return value should be cached for performance.
  AttributeCollection Attributes() const;
  // This variant will not update the potentially invalid attributes. To be used
  // when not interested in style attribute or one of the SVG attributes.
  AttributeCollection AttributesWithoutUpdate() const;
  // Similar to AttributesWithoutUpdate(), but with only the style attribute
  // exempt (ie., SVG attributes are always synchronized, for simplicity).
  // The style attribute is special because it is so frequently updated from
  // JavaScript and also easily identifiable (it is a single attribute).
  AttributeCollection AttributesWithoutStyleUpdate() const;

  void scrollIntoView(const V8UnionBooleanOrScrollIntoViewOptions* arg);
  void scrollIntoView(bool align_to_top = true);
  void scrollIntoViewWithOptions(const ScrollIntoViewOptions*);
  void ScrollIntoViewNoVisualUpdate(mojom::blink::ScrollIntoViewParamsPtr,
                                    const Element* container = nullptr);
  void scrollIntoViewIfNeeded(bool center_if_needed = true);

  int OffsetLeft();
  int OffsetTop();
  int OffsetWidth();
  int OffsetHeight();

  Element* OffsetParent();
  int clientLeft();
  int clientTop();
  int ClientLeftNoLayout() const;
  int ClientTopNoLayout() const;
  int clientWidth();
  int clientHeight();
  double currentCSSZoom();
  double scrollLeft();
  double scrollTop();
  void setScrollLeft(double);
  void setScrollTop(double);
  virtual int scrollWidth();
  virtual int scrollHeight();

  void scrollBy(double x, double y);
  void scrollBy(const ScrollToOptions*);
  void scrollTo(double x, double y);
  void scrollTo(const ScrollToOptions*);

  // Returns the bounds of this Element, unclipped, in the coordinate space of
  // the local root's widget. That is, in the outermost main frame, this will
  // scale and transform the bounds by the visual viewport transform (i.e.
  // pinch-zoom). In a local root that isn't main (i.e. a remote frame), the
  // returned bounds are unscaled by the visual viewport and are relative to
  // the local root frame.
  gfx::Rect BoundsInWidget() const;

  // Same as above but for outline rects.
  Vector<gfx::Rect> OutlineRectsInWidget(
      DocumentUpdateReason reason = DocumentUpdateReason::kUnknown) const;

  // Returns the bounds of this element relative to the local root frame's
  // origin. While the rect is relative to the local root, it is intersected
  // with all ancestor frame clips, including the visual viewport transform and
  // clip in the main frame. While this applies ancestor frame clipping, it
  // does not (yet) apply (overflow) element clipping (crbug.com/889840).
  gfx::Rect VisibleBoundsInLocalRoot() const;

  DOMRectList* getClientRects();
  // Returns a rectangle in zoomed pixel units.
  gfx::RectF GetBoundingClientRectNoLifecycleUpdateNoAdjustment() const;
  // Returns a rectangle in CSS pixel units.  i.e. ignoring zoom.
  gfx::RectF GetBoundingClientRectNoLifecycleUpdate() const;
  DOMRect* GetBoundingClientRect();
  DOMRect* GetBoundingClientRectForBinding();

  // Call the NoLifecycleUpdate variants if you are sure that the lifcycle is
  // already updated to at least pre-paint clean.
  const AtomicString& computedRole();
  const AtomicString& ComputedRoleNoLifecycleUpdate();
  String computedName();
  String ComputedNameNoLifecycleUpdate();

  void ariaNotify(const String& announcement,
                  const AriaNotificationOptions* options);

  void DidMoveToNewDocument(Document&) override;

  void removeAttribute(const AtomicString& name) {
    RemoveAttributeHinted(name, WeakLowercaseIfNecessary(name));
  }
  void removeAttributeNS(const AtomicString& namespace_uri,
                         const AtomicString& local_name);

  Attr* DetachAttribute(wtf_size_t index);

  Attr* getAttributeNode(const AtomicString& name);
  Attr* getAttributeNodeNS(const AtomicString& namespace_uri,
                           const AtomicString& local_name);
  Attr* setAttributeNode(Attr*, ExceptionState&);
  Attr* setAttributeNodeNS(Attr*, ExceptionState&);
  Attr* removeAttributeNode(Attr*, ExceptionState&);

  Attr* AttrIfExists(const QualifiedName&);
  Attr* EnsureAttr(const QualifiedName&);

  AttrNodeList* GetAttrNodeList();

  CSSStyleDeclaration* style();
  StylePropertyMap* attributeStyleMap();
  StylePropertyMapReadOnly* ComputedStyleMap();

  const QualifiedName& TagQName() const { return tag_name_; }
  String tagName() const { return nodeName(); }

  bool HasTagName(const QualifiedName& tag_name) const {
    return tag_name_.Matches(tag_name);
  }
  bool HasTagName(const HTMLQualifiedName& tag_name) const {
    return ContainerNode::HasTagName(tag_name);
  }
  bool HasTagName(const MathMLQualifiedName& tag_name) const {
    return ContainerNode::HasTagName(tag_name);
  }
  bool HasTagName(const SVGQualifiedName& tag_name) const {
    return ContainerNode::HasTagName(tag_name);
  }

  // Should be called only by Document::createElementNS to fix up tag_name_
  // immediately after construction.
  void SetTagNameForCreateElementNS(const QualifiedName&);

  // A fast function for checking the local name against another atomic string.
  bool HasLocalName(const AtomicString& other) const {
    return tag_name_.LocalName() == other;
  }

  const AtomicString& localName() const { return tag_name_.LocalName(); }
  AtomicString LocalNameForSelectorMatching() const;
  const AtomicString& prefix() const { return tag_name_.Prefix(); }
  const AtomicString& namespaceURI() const { return tag_name_.NamespaceURI(); }

  const AtomicString& LocateNamespacePrefix(
      const AtomicString& namespace_uri) const;

  String nodeName() const override;

  Element& CloneWithChildren(NodeCloningData& data,
                             Document*,
                             ContainerNode*,
                             ExceptionState& = ASSERT_NO_EXCEPTION) const;
  Element& CloneWithoutChildren(NodeCloningData& data,
                                Document* = nullptr) const;
  Element& CloneWithoutChildren() const;

  void SetBooleanAttribute(const QualifiedName&, bool);

  virtual const CSSPropertyValueSet* AdditionalPresentationAttributeStyle() {
    return nullptr;
  }
  void InvalidateStyleAttribute(bool only_changed_independent_properties);

  const CSSPropertyValueSet* InlineStyle() const {
    return HasElementData() ? GetElementData()->inline_style_.Get() : nullptr;
  }

  void SetInlineStyleProperty(CSSPropertyID,
                              CSSValueID identifier,
                              bool important = false);
  void SetInlineStyleProperty(CSSPropertyID,
                              double value,
                              CSSPrimitiveValue::UnitType,
                              bool important = false);
  void SetInlineStyleProperty(CSSPropertyID,
                              const CSSValue&,
                              bool important = false);
  bool SetInlineStyleProperty(CSSPropertyID,
                              const String& value,
                              bool important = false);
  void SetInlineStyleProperty(const CSSPropertyName&,
                              const CSSValue&,
                              bool important = false);

  bool RemoveInlineStyleProperty(CSSPropertyID);
  bool RemoveInlineStyleProperty(const AtomicString&);
  void RemoveAllInlineStyleProperties();

  void SynchronizeStyleAttributeInternal() const;

  const CSSPropertyValueSet* PresentationAttributeStyle();
  virtual bool IsPresentationAttribute(const QualifiedName& attr) const {
    return false;
  }
  virtual void CollectStyleForPresentationAttribute(
      const QualifiedName&,
      const AtomicString&,
      HeapVector<CSSPropertyValue, 8>&) {}
  // Subclasses can override these functions if there is extra style that needs
  // to be mapped like attributes.
  virtual bool HasExtraStyleForPresentationAttribute() const { return false; }
  virtual void CollectExtraStyleForPresentationAttribute(
      HeapVector<CSSPropertyValue, 8>&) {}

  // For exposing to DOM only.
  NamedNodeMap* attributesForBindings() const;
  AttributeNamesView getAttributeNamesForBindings() const;
  // Note that the method above returns a live view of underlying
  // attribute collection, which may be unsafe to use for iteration
  // if element attributes are modified during iteration, hence the
  // safe (but slower) alternative below.
  Vector<AtomicString> getAttributeNames() const;
  Vector<QualifiedName> getAttributeQualifiedNames() const;

  enum class AttributeModificationReason {
    kDirectly,
    kByParser,
    kByCloning,
    kByMoveToNewDocument,
    kBySynchronizationOfLazyAttribute
  };
  struct AttributeModificationParams {
    STACK_ALLOCATED();

   public:
    AttributeModificationParams(const QualifiedName& qname,
                                const AtomicString& old_value,
                                const AtomicString& new_value,
                                AttributeModificationReason reason)
        : name(qname),
          old_value(old_value),
          new_value(new_value),
          reason(reason) {}

    const QualifiedName& name;
    const AtomicString& old_value;
    const AtomicString& new_value;
    const AttributeModificationReason reason;
  };

  // |attributeChanged| is called whenever an attribute is added, changed or
  // removed. It handles very common attributes such as id, class, name, style,
  // and slot.
  //
  // While the owner document is parsed, this function is called after all
  // attributes in a start tag were added to the element.
  virtual void AttributeChanged(const AttributeModificationParams&);

  // |ParseAttribute()| is called by |AttributeChanged()|. If an element
  // implementation needs to check an attribute update, override this function.
  // This function is called before Element handles the change. This means
  // changes like `kSlotAttr` will not have been processed. Subclasses should
  // take care to avoid any processing that needs Element to have handled the
  // change. For example, flat-tree-travesal could be problematic. In such
  // cases subclasses should override AttributeChanged() and do the processing
  // after calling Element::AttributeChanged().
  //
  // While the owner document is parsed, this function is called after all
  // attributes in a start tag were added to the element.
  virtual void ParseAttribute(const AttributeModificationParams&);

  virtual bool HasLegalLinkAttribute(const QualifiedName&) const;

  // Only called by the parser immediately after element construction.
  void ParserSetAttributes(const Vector<Attribute, kAttributePrealloc>&);

  // Remove attributes that might introduce scripting from the vector leaving
  // the element unchanged.
  void StripScriptingAttributes(Vector<Attribute, kAttributePrealloc>&) const;

  bool SharesSameElementData(const Element& other) const {
    return GetElementData() == other.GetElementData();
  }

  // Clones attributes only.
  void CloneAttributesFrom(const Element&);

  bool HasEquivalentAttributes(const Element& other) const;

  // Returns false if the element definitely does not have an attribute
  // matching the given name. Is allowed to return false positives.
  bool CouldHaveAttribute(const QualifiedName& attribute_name) const {
    return CouldHaveAttributeWithPrecomputedFilter(
        FilterForAttribute(attribute_name));
  }

  // A variant of CouldHaveAttribute() that allows you to compute
  // the filter ahead-of-time; useful if you want to test many elements
  // against the same attribute name.
  static uint32_t FilterForAttribute(const QualifiedName& attribute_name) {
    unsigned hash = attribute_name.LocalNameUpper().Hash();
    uint32_t filter = 0;
    // Build a 32-bit Bloom filter, with k=2. We extract the two
    // (5-bit) hashes that we need from non-overlapping parts of the
    // (24-bit) String hash, which should be independent.
    filter |= 1u << (hash & 31);
    filter |= 1u << ((hash >> 5) & 31);
    return filter;
  }
  bool CouldHaveAttributeWithPrecomputedFilter(uint32_t filter) const {
    return (attribute_bloom_ & filter) == filter;
  }
#if DCHECK_IS_ON()
  uint32_t AttributeBloomFilterForDebug() const { return attribute_bloom_; }
#endif

  // Step 5 of https://dom.spec.whatwg.org/#concept-node-clone
  virtual void CloneNonAttributePropertiesFrom(const Element&,
                                               NodeCloningData&) {}

  const ComputedStyle* GetComputedStyle() const {
    return computed_style_.Get();
  }
  const ComputedStyle& ComputedStyleRef() const {
    DCHECK(computed_style_);
    return *computed_style_;
  }

  void SetComputedStyle(const ComputedStyle* computed_style) {
    computed_style_ = computed_style;
  }

  using Node::DetachLayoutTree;
  void AttachLayoutTree(AttachContext&) override;
  void DetachLayoutTree(bool performing_reattach) override;

  virtual LayoutObject* CreateLayoutObject(const ComputedStyle&);
  virtual bool LayoutObjectIsNeeded(const DisplayStyle&) const;
  bool LayoutObjectIsNeeded(const ComputedStyle&) const;

  const ComputedStyle* ParentComputedStyle() const;

  void RecalcStyle(const StyleRecalcChange, const StyleRecalcContext&);
  void RecalcStyleForTraversalRootAncestor();

  // RecalcHighlightStyles for the originating element's new_style and return
  // a new new_style if highlight styles were added. Otherwise return a pointer
  // to the passed in new_style.
  const ComputedStyle* RecalcHighlightStyles(
      const StyleRecalcContext& style_recalc_context,
      const ComputedStyle* old_style,
      const ComputedStyle& new_style,
      const ComputedStyle* parent_style);

  void RebuildLayoutTreeForTraversalRootAncestor() {
    RebuildFirstLetterLayoutTree();
    WhitespaceAttacher whitespace_attacher;
    RebuildPseudoElementLayoutTree(kPseudoIdMarker, whitespace_attacher);
    HandleSubtreeModifications();
  }
  void RebuildLayoutTreeForSizeContainerAncestor() {
    RebuildFirstLetterLayoutTree();
  }
  bool NeedsRebuildChildLayoutTrees(
      const WhitespaceAttacher& whitespace_attacher) const {
    return ChildNeedsReattachLayoutTree() || NeedsWhitespaceChildrenUpdate() ||
           (whitespace_attacher.TraverseIntoDisplayContents() &&
            HasDisplayContentsStyle());
  }
  bool NeedsRebuildLayoutTree(
      const WhitespaceAttacher& whitespace_attacher) const {
    return NeedsReattachLayoutTree() ||
           NeedsRebuildChildLayoutTrees(whitespace_attacher) ||
           NeedsLayoutSubtreeUpdate();
  }
  void RebuildLayoutTree(WhitespaceAttacher&);

  // Reattach layout tree for all children but not the element itself. This is
  // only used for UpdateStyleAndLayoutTreeForContainer when:
  // 1. Re-attaching fieldset when the fieldset layout tree changes and the size
  //    query container is a fieldset.
  // 2. Re-attaching for legacy box tree when table-* boxes have columns.
  //
  // Case 2 is only necessary until table fragmentation is shipped for LayoutNG.
  //
  void ReattachLayoutTreeChildren(base::PassKey<StyleEngine>);

  void HandleSubtreeModifications();
  void PseudoStateChanged(CSSSelector::PseudoType);
  void PseudoStateChangedForTesting(CSSSelector::PseudoType);
  void SetAnimationStyleChange(bool);
  void SetNeedsAnimationStyleRecalc();

  void SetNeedsCompositingUpdate();

  // Associates the element with a RegionCaptureCropId, which is the object
  // internally backing a CropTarget.
  // This method may be called at most once. The ID must be non-null.
  void SetRegionCaptureCropId(std::unique_ptr<RegionCaptureCropId> id);

  // If SetRegionCaptureCropId(id) was previously called on `this`,
  // returns the non-empty `id` which it previously provided.
  // Otherwise, returns a nullptr.
  const RegionCaptureCropId* GetRegionCaptureCropId() const;

  // Associates the element with a RestrictionTargetId, which is the object
  // internally backing a RestrictionTarget.
  // This method may be called at most once. The ID must be non-null.
  void SetRestrictionTargetId(std::unique_ptr<RestrictionTargetId> id);

  // If SetRestrictionTargetId(id) was previously called on `this`,
  // returns the non-empty `id` which it previously provided.
  // Otherwise, returns a nullptr.
  const RestrictionTargetId* GetRestrictionTargetId() const;

  // Set whether the element is eligible for element level capture. This is
  // based on how the element is painted. Should only be called if the element
  // has a RestrictionTargetId.
  void SetIsEligibleForElementCapture(bool value);

  ShadowRoot* attachShadow(const ShadowRootInit*, ExceptionState&);

  // Returns true if the attachment was successful.
  bool AttachDeclarativeShadowRoot(HTMLTemplateElement&,
                                   String,
                                   FocusDelegation,
                                   SlotAssignmentMode,
                                   bool serializable,
                                   bool clonable,
                                   const AtomicString& reference_target);

  ShadowRoot& CreateUserAgentShadowRoot(
      SlotAssignmentMode = SlotAssignmentMode::kNamed);
  ShadowRoot& AttachShadowRootInternal(ShadowRootMode,
                                       FocusDelegation,
                                       SlotAssignmentMode,
                                       CustomElementRegistry*,
                                       bool serializable,
                                       bool clonable,
                                       const AtomicString& reference_target);
  // This version is for testing only, and allows easy attachment of a shadow
  // root, specifying only the type and none of the other arguments.
  ShadowRoot& AttachShadowRootForTesting(ShadowRootMode type) {
    return AttachShadowRootInternal(type, FocusDelegation::kNone,
                                    SlotAssignmentMode::kNamed,
                                    /*registry*/ nullptr,
                                    /*serializable*/ false,
                                    /*clonable*/ false,
                                    /*reference_target*/ g_null_atom);
  }

  // Returns the shadow root attached to this element if it is a shadow host.
  ShadowRoot* GetShadowRoot() const;
  ShadowRoot* OpenShadowRoot() const;
  ShadowRoot* ClosedShadowRoot() const;
  ShadowRoot* AuthorShadowRoot() const;
  ShadowRoot* UserAgentShadowRoot() const;

  ShadowRoot& EnsureUserAgentShadowRoot(
      SlotAssignmentMode = SlotAssignmentMode::kNamed);

  // Implements manual slot assignment for user agent shadow roots.
  virtual void ManuallyAssignSlots() { DCHECK(false); }

  bool IsInDescendantTreeOf(const Element* shadow_host) const;

  // Returns the Element’s ComputedStyle. If the ComputedStyle is not already
  // stored on the Element, computes the ComputedStyle and stores it on the
  // Element’s ElementRareData.  Used for getComputedStyle when Element is
  // display none.
  const ComputedStyle* EnsureComputedStyle(
      PseudoId = kPseudoIdNone,
      const AtomicString& pseudo_argument = g_null_atom);

  bool HasDisplayContentsStyle() const;

  bool ShouldStoreComputedStyle(const ComputedStyle&) const;

  // Methods for indicating the style is affected by dynamic updates (e.g.,
  // children changing, our position changing in our sibling list, etc.)
  bool StyleAffectedByEmpty() const {
    return HasElementFlag(ElementFlags::kStyleAffectedByEmpty);
  }
  void SetStyleAffectedByEmpty() {
    SetElementFlag(ElementFlags::kStyleAffectedByEmpty);
  }

  void SetIsInCanvasSubtree(bool value) {
    SetElementFlag(ElementFlags::kIsInCanvasSubtree, value);
  }
  bool IsInCanvasSubtree() const {
    return HasElementFlag(ElementFlags::kIsInCanvasSubtree);
  }

  bool IsDefined() const {
    // An element whose custom element state is "uncustomized" or "custom"
    // is said to be defined.
    // https://dom.spec.whatwg.org/#concept-element-defined
    return GetCustomElementState() == CustomElementState::kUncustomized ||
           GetCustomElementState() == CustomElementState::kCustom;
  }

  AtomicString ComputeInheritedLanguage() const;
  Locale& GetLocale() const;

  virtual void AccessKeyAction(SimulatedClickCreationScope) {}

  virtual bool IsURLAttribute(const Attribute&) const { return false; }
  virtual bool IsHTMLContentAttribute(const Attribute&) const { return false; }
  bool IsJavaScriptURLAttribute(const Attribute&) const;
  virtual bool IsSVGAnimationAttributeSettingJavaScriptURL(
      const Attribute&) const {
    return false;
  }
  bool IsScriptingAttribute(const Attribute&) const;

  virtual bool IsLiveLink() const { return false; }
  KURL HrefURL() const;

  String GetURLAttribute(const QualifiedName&) const;
  KURL GetURLAttributeAsKURL(const QualifiedName&) const;

  KURL GetNonEmptyURLAttribute(const QualifiedName&) const;

  virtual const AtomicString ImageSourceURL() const;
  virtual Image* ImageContents() { return nullptr; }

  // Returns true if this is a shadow host, and its ShadowRoot has
  // delegatesFocus flag.
  bool IsShadowHostWithDelegatesFocus() const;
  // in_descendant_traversal is used in GetFocusableArea and GetFocusDelegate to
  // indicate that GetFocusDelegate is currently iterating over all descendants
  // in a DOM subtree. Since GetFocusDelegate calls GetFocusableArea and
  // GetFocusableArea calls GetFocusDelegate, this allows us to skip redundant
  // recursive calls to the same descendants.
  Element* GetFocusableArea(bool in_descendant_traversal = false) const;
  Element* GetFocusDelegate(bool in_descendant_traversal = false) const;
  // Element focus function called through IDL (i.e. element.focus() in JS)
  // Delegates to Focus() with focus type set to kScript
  void focusForBindings(const FocusOptions*);
  // Element focus function called from outside IDL (user focus,
  // accessibility, etc...)
  virtual void Focus(const FocusParams&);
  // Delegates to virtual Focus() with focus type set to kNone
  void Focus();
  void Focus(const FocusOptions*);

  virtual void SetFocused(bool received, mojom::blink::FocusType);
  virtual void SetHasFocusWithinUpToAncestor(bool has_focus_within,
                                             Element* ancestor,
                                             bool need_snap_container_search);
  void FocusStateChanged();
  void FocusVisibleStateChanged();
  void FocusWithinStateChanged();
  void ActiveViewTransitionStateChanged();
  void ActiveViewTransitionTypeStateChanged();
  void SetDragged(bool) override;

  void UpdateSelectionOnFocus(SelectionBehaviorOnFocus);
  // This function is called after SetFocused(true) before dispatching 'focus'
  // event, or is called just after a layout after changing <input> type.
  virtual void UpdateSelectionOnFocus(SelectionBehaviorOnFocus,
                                      const FocusOptions*);
  virtual void blur();

  enum class UpdateBehavior {
    // The normal update behavior - update style and layout if needed.
    kStyleAndLayout,
    // Don't update style and layout. This should only be called by
    // accessibility-related code, when needed.
    kNoneForAccessibility,
    // Don't update style and layout. This should only be called by
    // functions that are updating focused state, such as
    // ShouldHaveFocusAppearance() and ClearFocusedElementIfNeeded().
    kNoneForFocusManagement,
    // Don't update style and layout, and assert that layout is clean already.
    kAssertNoLayoutUpdates,
  };

  // Whether the element is clickable. This checks for whether the node is
  // a clickable control (e.g. form control elements) or has activation
  // behavior. It also checks for whether the node has a click handler.
  // Note: this should not be taken as a guarantee that the element is
  // clickable; this is used as a heuristic to determine whether the element
  // is likely to be clickable.
  bool IsMaybeClickable();

  // Focusability logic:
  //   IsFocusable: true if the element can be focused via element.focus().
  //   IsMouseFocusable: true if clicking on the element will focus it.
  //   IsKeyboardFocusableSlow: true if the element appears in the sequential
  //     focus navigation loop. I.e. if the tab key can focus it.
  //
  // Helpers:
  //   SupportsFocus: true if it is *possible* for the element to be focused. An
  //     element supports focus if it has a tabindex attribute, or it is
  //     editable, etc. Note that the element might *support* focus while not
  //     *being focusable*, e.g. when the element is disconnected.
  //   IsFocusableState: can be not focusable, focusable, or focusable because
  //     of keyboard focusable scrollers.
  //
  // IsFocusable can only be true if SupportsFocus is true. And both
  // IsMouseFocusable and IsKeyboardFocusableSlow require IsFocusable to be
  // true. But it is possible for an element to be keyboard-focusable without
  // being mouse-focusable, or vice versa.
  //
  // All of these methods can be called when layout is not clean, but a
  // lifecycle update might be triggered in that case. If layout is already
  // clean, these methods will not trigger an additional lifecycle update.
  // If UpdateBehavior::kNoneForAccessibility is passed (only to be used by
  // accessibility code), then no layout updates will be performed even in the
  // case that layout is dirty.
  bool IsFocusable(
      UpdateBehavior update_behavior = UpdateBehavior::kStyleAndLayout) const;
  bool IsMouseFocusable(
      UpdateBehavior update_behavior = UpdateBehavior::kStyleAndLayout) const;
  // If the element might be a keyboard-focusable scroller, then it will call
  // IsKeyboardFocusableScroller which can be slow. Avoid calling this function
  // outside of focus sequential navigation.
  virtual bool IsKeyboardFocusableSlow(
      UpdateBehavior update_behavior = UpdateBehavior::kStyleAndLayout) const;

  bool IsFocusedElementInDocument() const;
  Element* AdjustedFocusedElementInTreeScope() const;
  bool IsAutofocusable() const;

  // Returns false if the event was canceled, and true otherwise.
  virtual bool DispatchFocusEvent(
      Element* old_focused_element,
      mojom::blink::FocusType,
      InputDeviceCapabilities* source_capabilities = nullptr);
  virtual void DispatchBlurEvent(
      Element* new_focused_element,
      mojom::blink::FocusType,
      InputDeviceCapabilities* source_capabilities = nullptr);
  virtual void DispatchFocusInEvent(
      const AtomicString& event_type,
      Element* old_focused_element,
      mojom::blink::FocusType,
      InputDeviceCapabilities* source_capabilities = nullptr);
  void DispatchFocusOutEvent(
      const AtomicString& event_type,
      Element* new_focused_element,
      InputDeviceCapabilities* source_capabilities = nullptr);

  // This allows customization of how Invoker Commands are handled, per element.
  // See: crbug.com/1490919, https://open-ui.org/components/invokers.explainer/
  virtual bool IsValidBuiltinCommand(HTMLElement& invoker,
                                     CommandEventType command) {
    return false;
  }
  virtual bool HandleCommandInternal(HTMLElement& invoker,
                                     CommandEventType command) {
    CHECK(command != CommandEventType::kCustom &&
          command != CommandEventType::kNone);
    return false;
  }

  // If this element is a triggering element for an *open* popover, in one of
  // several ways, this returns the targeted popover. These forms of triggering
  // are supported:
  //   <button popovertarget=foo>
  //   <button command=*-popover commandfor=foo>
  //   <button interesttarget=foo>
  //   (JS) popover.showPopover({source: foo})
  // Note: this function returns the *target* popover. Or nullptr if there isn't
  // a target, it isn't a popover, or the popover isn't open as the result of
  // this triggering element. (E.g. if the popover is just open on its own and
  // wasn't triggered by this invoker, this will return nullptr.)
  HTMLElement* GetOpenPopoverTarget() const;

  // Implementation of the `interesttarget` feature. These are called on the
  // element with the `interesttarget` attribute, and not on the target itself.
  // These are called when interest is actually gained or lost on the element,
  // e.g. after any hover-delays. They return true if the event was *not*
  // cancelled, and the action was performed.
  bool InterestGained(Element& interest_target);
  bool InterestLost(Element& interest_target);
  // Returns the target of the `interesttarget` attribute, if any, and only if
  // the element supports this attribute. For example, `interesttarget` is not
  // allowed on a `<div>`.
  virtual Element* InterestTargetElement() const { return nullptr; }
  // Returns the active interest invoker for which this element is the target,
  // or nullptr otherwise.
  Element* GetInterestInvoker() const;
  enum class InterestState {
    kNoInterest,
    kPartialInterest,
    kFullInterest,
  };
  // Returns the current state of "interest" in an element that is an interest
  // invoker.
  InterestState GetInterestState();
  // Returns true if this element is (inclusively) contained within an open
  // popover that is the target of an interest invoker that has partial
  // interest.
  bool IsInPartialInterestPopover() const;

  // The implementations of |innerText()| and |GetInnerTextWithoutUpdate()| are
  // found in "element_inner_text.cc".
  // Avoids layout update.
  String GetInnerTextWithoutUpdate(TextVisitor* visitor = nullptr);
  // `visitor` is called as each node is considered. Note that this is not
  // called for nodes that are not considered in generating the text. For
  // example, all descendants of hidden nodes are not considered.
  String innerText(TextVisitor* visitor = nullptr);
  String outerText();

  Element* insertAdjacentElement(const String& where,
                                 Element* new_child,
                                 ExceptionState&);
  void insertAdjacentText(const String& where,
                          const String& text,
                          ExceptionState&);
  void insertAdjacentHTML(const String& where,
                          const String& html,
                          ExceptionState&);

  String innerHTML() const;
  String outerHTML() const;
  void setInnerHTML(const String&, ExceptionState& = ASSERT_NO_EXCEPTION);
  void setOuterHTML(const String&, ExceptionState& = ASSERT_NO_EXCEPTION);

  // The setHTMLUnsafe method is like `setInnerHTML()` except that a) it parses
  // declarative shadow DOM by default, and b) will eventually have a second
  // argument to set Sanitizer parameters.
  // See https://github.com/whatwg/html/pull/9538.
  void setHTMLUnsafe(const String& html, ExceptionState& = ASSERT_NO_EXCEPTION);
  void setHTMLUnsafe(const String& html,
                     SetHTMLUnsafeOptions*,
                     ExceptionState&);
  void setHTML(const String& html, SetHTMLOptions*, ExceptionState&);

  void setPointerCapture(PointerId poinetr_id, ExceptionState&);
  void releasePointerCapture(PointerId pointer_id, ExceptionState&);

  // Returns true iff the element would capture the next pointer event. This
  // is true between a setPointerCapture call and a releasePointerCapture (or
  // implicit release) call:
  // https://w3c.github.io/pointerevents/#dom-element-haspointercapture
  bool hasPointerCapture(PointerId pointer_id) const;

  String TextFromChildren();

  virtual String title() const { return String(); }
  virtual String DefaultToolTip() const { return String(); }

  virtual const AtomicString& ShadowPseudoId() const;
  // The specified string must start with "-webkit-" or "-internal-". The
  // former can be used as a selector in any places, and the latter can be
  // used only in UA stylesheet.
  void SetShadowPseudoId(const AtomicString&);

  // Called by the parser when this element's close tag is reached, signaling
  // that all child tags have been parsed and added.  This is needed for
  // <applet> and <object> elements, which can't lay themselves out until they
  // know all of their nested <param>s. [Radar 3603191, 4040848].  Also used for
  // script elements and some SVG elements for similar purposes, but making
  // parsing a special case in this respect should be avoided if possible.
  virtual void FinishParsingChildren();

  void BeginParsingChildren() { SetIsFinishedParsingChildren(false); }

  // Returns the pseudo element for the given PseudoId type.
  // |view_transition_name| is used to uniquely identify a pseudo element
  // from a set of pseudo elements which share the same |pseudo_id|. The current
  // usage of this ID is limited to pseudo elements generated for a
  // ViewTransition. See
  // third_party/blink/renderer/core/view_transition/README.md
  //
  // Also see GetStyledPseudoElement() below.
  PseudoElement* GetPseudoElement(
      PseudoId,
      const AtomicString& view_transition_name = g_null_atom) const;
  LayoutObject* PseudoElementLayoutObject(PseudoId) const;

  // Returns true if this element has ::view-transition-group children.
  bool HasViewTransitionGroupChildren() const;

  // Returns true if this element contains any ::scroll-button or
  // ::scroll-marker-group pseudos.
  bool HasScrollButtonOrMarkerGroupPseudos() const;

  bool PseudoElementStylesAffectCounters() const;

  bool PseudoElementStylesDependOnFontMetrics() const;
  bool PseudoElementStylesDependOnAttr() const;

  // Retrieve the ComputedStyle (if any) corresponding to the provided
  // PseudoId from cache, calculating the ComputedStyle on-demand if it's
  // missing from the cache. The |pseudo_argument| is also used to match the
  // ComputedStyle in cases where the PseudoId corresponds to a pseudo element
  // that takes arguments (e.g. ::highlight()).
  const ComputedStyle* CachedStyleForPseudoElement(
      PseudoId,
      const AtomicString& pseudo_argument = g_null_atom);

  // Calculate the ComputedStyle corresponding to the provided StyleRequest,
  // bypassing the pseudo style cache.
  //
  // This is appropriate to use if the cached version is invalid in a given
  // situation.
  const ComputedStyle* UncachedStyleForPseudoElement(const StyleRequest&);

  // This is the same as UncachedStyleForPseudoElement, except that the caller
  // must provide an appropriate StyleRecalcContext such that e.g. @container
  // queries are evaluated correctly.
  //
  // See StyleRecalcContext for more information.
  const ComputedStyle* StyleForPseudoElement(const StyleRecalcContext&,
                                             const StyleRequest&);

  // These are used by ResolveStyle with Highlight Inheritance when caching
  // is not used.
  const ComputedStyle* StyleForHighlightPseudoElement(
      const StyleRecalcContext& style_recalc_context,
      const ComputedStyle* highlight_parent,
      const ComputedStyle& originating_style,
      const PseudoId pseudo_id,
      const AtomicString& pseudo_argument = g_null_atom);
  const ComputedStyle* StyleForSearchTextPseudoElement(
      const StyleRecalcContext& style_recalc_context,
      const ComputedStyle* highlight_parent,
      const ComputedStyle& originating_style,
      StyleRequest::SearchTextRequest search_text_request);

  virtual bool CanGeneratePseudoElement(PseudoId) const;

  virtual bool MatchesDefaultPseudoClass() const { return false; }
  virtual bool MatchesEnabledPseudoClass() const { return false; }
  virtual bool MatchesReadOnlyPseudoClass() const { return false; }
  virtual bool MatchesReadWritePseudoClass() const { return false; }
  virtual bool MatchesValidityPseudoClasses() const { return false; }

  virtual bool MayTriggerVirtualKeyboard() const;

  // https://dom.spec.whatwg.org/#dom-element-matches
  bool matches(const AtomicString& selectors, ExceptionState&);
  bool matches(const AtomicString& selectors);

  // https://dom.spec.whatwg.org/#dom-element-closest
  Element* closest(const AtomicString& selectors, ExceptionState&);
  Element* closest(const AtomicString& selectors);

  virtual bool ShouldAppearIndeterminate() const { return false; }

  DOMTokenList& classList();

  DOMStringMap& dataset();

  virtual bool IsDateTimeEditElement() const { return false; }
  virtual bool IsDateTimeFieldElement() const { return false; }
  virtual bool IsPickerIndicatorElement() const { return false; }

  virtual bool IsFormControlElement() const { return false; }
  virtual bool IsFormControlElementWithState() const { return false; }
  virtual bool IsSpinButtonElement() const { return false; }
  // This returns true for <textarea> and some types of <input>.
  virtual bool IsTextControl() const { return false; }
  virtual bool IsOptionalFormControl() const { return false; }
  virtual bool IsRequiredFormControl() const { return false; }
  virtual bool willValidate() const { return false; }
  virtual bool IsValidElement() { return false; }
  virtual bool IsInRange() const { return false; }
  virtual bool IsOutOfRange() const { return false; }
  virtual bool IsClearButtonElement() const { return false; }
  virtual bool IsScriptElement() const { return false; }
  virtual bool IsVTTCueBackgroundBox() const { return false; }
  virtual bool IsVTTCueBox() const { return false; }
  virtual bool IsSliderThumbElement() const { return false; }
  virtual bool IsOutputElement() const { return false; }

  // Elements that may have an insertion mode other than "in body" should
  // override this and return true.
  // https://html.spec.whatwg.org/C/#reset-the-insertion-mode-appropriately
  virtual bool HasNonInBodyInsertionMode() const { return false; }

  bool CanContainRangeEndPoint() const override { return true; }

  // Used for disabled form elements; if true, prevents mouse events from being
  // dispatched to event listeners, and prevents DOMActivate events from being
  // sent at all.
  virtual bool IsDisabledFormControl() const { return false; }

  void SetCustomElementDefinition(CustomElementDefinition*);
  CustomElementDefinition* GetCustomElementDefinition() const;

  // Scoped Custom Elements
  CustomElementRegistry* customElementRegistry() const;

  // https://dom.spec.whatwg.org/#concept-element-is-value
  void SetIsValue(const AtomicString&);
  const AtomicString& IsValue() const;
  void SetDidAttachInternals();
  bool DidAttachInternals() const;
  ElementInternals& EnsureElementInternals();
  const ElementInternals* GetElementInternals() const;

  bool ContainsFullScreenElement() const {
    return HasElementFlag(ElementFlags::kContainsFullScreenElement);
  }
  void SetContainsFullScreenElement(bool);
  void SetContainsFullScreenElementOnAncestorsCrossingFrameBoundaries(bool);

  bool ContainsPersistentVideo() const {
    return HasElementFlag(ElementFlags::kContainsPersistentVideo);
  }
  void SetContainsPersistentVideo(bool);

  bool IsInTopLayer() const {
    return HasElementFlag(ElementFlags::kIsInTopLayer);
  }
  void SetIsInTopLayer(bool);

  ScriptPromise<IDLUndefined> requestPointerLock(
      ScriptState* script_state,
      const PointerLockOptions* options,
      ExceptionState& exception_state);

  bool IsSpellCheckingEnabled() const;

  // FIXME: public for LayoutTreeBuilder, we shouldn't expose this though.
  const ComputedStyle* StyleForLayoutObject(const StyleRecalcContext&);

  // Called by StyleAdjuster during style resolution. Provides an opportunity to
  // make final Element-specific adjustments to the ComputedStyle.
  void AdjustStyle(base::PassKey<StyleAdjuster>, ComputedStyleBuilder&);

  bool HasID() const;
  bool HasClass() const;
  const SpaceSplitString& ClassNames() const;
  bool HasClassName(const AtomicString& class_name) const;

  // Returns true if the element has 1 or more part names.
  bool HasPart() const;
  // Returns the list of part names if it has ever been created.
  DOMTokenList* GetPart() const;
  // IDL method.
  // Returns the list of part names, creating it if it doesn't exist.
  DOMTokenList& part();

  bool HasPartNamesMap() const;
  const NamesMap* PartNamesMap() const;

  ScrollOffset SavedLayerScrollOffset() const;
  void SetSavedLayerScrollOffset(const ScrollOffset&);

  ElementAnimations* GetElementAnimations() const;
  ElementAnimations& EnsureElementAnimations();
  bool HasAnimations() const;

  void SynchronizeAttribute(const AtomicString& local_name) const {
    SynchronizeAttributeHinted(local_name,
                               WeakLowercaseIfNecessary(local_name));
  }

  MutableCSSPropertyValueSet& EnsureMutableInlineStyle();
  void ClearMutableInlineStyleIfEmpty();

  CSSPropertyValueSet* CreatePresentationAttributeStyle();

  void setTabIndex(int);
  int tabIndex() const;
  // If element is a reading flow container or display: contents whose layout
  // parent is one, return the nodes corresponding to its direct children
  // sorted in reading flow order.
  const HeapVector<Member<Node>> ReadingFlowChildren() const;

  void setHeadingOffset(int);
  int headingOffset() const;
  void setHeadingReset(bool);
  bool headingReset() const;
  int GetComputedHeadingOffset(int max_offset);

  void setEditContext(EditContext* editContext, ExceptionState&);
  EditContext* editContext() const;

  // Helpers for V8DOMActivityLogger::logEvent.  They call logEvent only if
  // the element is isConnected() and the context is an isolated world.
  void LogAddElementIfIsolatedWorldAndInDocument(const char element[],
                                                 const QualifiedName& attr1);
  void LogAddElementIfIsolatedWorldAndInDocument(const char element[],
                                                 const QualifiedName& attr1,
                                                 const QualifiedName& attr2);
  void LogAddElementIfIsolatedWorldAndInDocument(const char element[],
                                                 const QualifiedName& attr1,
                                                 const QualifiedName& attr2,
                                                 const QualifiedName& attr3);
  void LogUpdateAttributeIfIsolatedWorldAndInDocument(
      const char element[],
      const AttributeModificationParams&);

  void Trace(Visitor*) const override;

  SpellcheckAttributeState GetSpellcheckAttributeState() const;

  ElementIntersectionObserverData* IntersectionObserverData() const;
  ElementIntersectionObserverData& EnsureIntersectionObserverData();

  HeapHashMap<Member<ResizeObserver>, Member<ResizeObservation>>*
  ResizeObserverData() const;
  HeapHashMap<Member<ResizeObserver>, Member<ResizeObservation>>&
  EnsureResizeObserverData();

  DisplayLockContext* GetDisplayLockContext() const {
    if (!HasDisplayLockContext()) [[likely]] {
      return nullptr;
    }
    return GetDisplayLockContextFromRareData();
  }
  DisplayLockContext& EnsureDisplayLockContext();

  bool ChildStyleRecalcBlockedByDisplayLock() const;

  // Activates all activatable (for a given reason) locked ancestors for this
  // element. Return true if we activated at least one previously locked
  // element.
  bool ActivateDisplayLockIfNeeded(DisplayLockActivationReason reason);

  ContainerQueryData* GetContainerQueryData() const;
  ContainerQueryEvaluator* GetContainerQueryEvaluator() const;
  ContainerQueryEvaluator& EnsureContainerQueryEvaluator();
  bool SkippedContainerStyleRecalc() const;

  StyleScopeData& EnsureStyleScopeData();
  StyleScopeData* GetStyleScopeData() const;

  OutOfFlowData& EnsureOutOfFlowData();
  OutOfFlowData* GetOutOfFlowData() const;

  // See PostStyleUpdateScope::PseudoData::AddPendingBackdrop
  void ApplyPendingBackdropPseudoElementUpdate();

  virtual void SetActive(bool active);
  virtual void SetHovered(bool hovered);

  // Classes overriding this method can return true when an element has
  // been determined to be from an ad. Returns false by default.
  virtual bool IsAdRelated() const { return false; }

  void NotifyInlineStyleMutation();

  // For undo stack cleanup
  bool HasUndoStack() const;
  void SetHasUndoStack(bool);

  // For font-related style invalidation.
  void SetScrollbarPseudoElementStylesDependOnFontMetrics(bool);

  void SetPseudoElementStylesChangeCounters(bool value);

  // Get (or create if it doesn't already exist) a per-column (fragmentainer)
  // ::column pseudo-element for the given column index.
  ///
  // Also, if ::column::scroll-marker is specified, create one ::scroll-marker
  // per ::column pseudo-element, if needed, and if it doesn't already exist.
  ColumnPseudoElement* GetOrCreateColumnPseudoElementIfNeeded(
      wtf_size_t index,
      const PhysicalRect& column_rect);
  const ColumnPseudoElementsVector* GetColumnPseudoElements() const;

  // Clear all ::column pseudo-elements, except for the leading `to_keep` ones.
  void ClearColumnPseudoElements(wtf_size_t to_keep = 0);

  // True if a scroller has not been explicitly scrolled by a user or by a
  // programmatic scroll. Indicates that we should use the CSS scroll-start
  // property.
  bool HasBeenExplicitlyScrolled() const;
  void SetHasBeenExplicitlyScrolled();

  bool AffectedBySubjectHas() const;
  void SetAffectedBySubjectHas();
  bool AffectedByNonSubjectHas() const;
  void SetAffectedByNonSubjectHas();
  bool AncestorsOrAncestorSiblingsAffectedByHas() const;
  void SetAncestorsOrAncestorSiblingsAffectedByHas();
  unsigned GetSiblingsAffectedByHasFlags() const;
  bool HasSiblingsAffectedByHasFlags(unsigned flags) const;
  void SetSiblingsAffectedByHasFlags(unsigned flags);
  bool AffectedByPseudoInHas() const;
  void SetAffectedByPseudoInHas();
  bool AncestorsOrSiblingsAffectedByHoverInHas() const;
  void SetAncestorsOrSiblingsAffectedByHoverInHas();
  bool AncestorsOrSiblingsAffectedByActiveInHas() const;
  void SetAncestorsOrSiblingsAffectedByActiveInHas();
  bool AncestorsOrSiblingsAffectedByFocusInHas() const;
  void SetAncestorsOrSiblingsAffectedByFocusInHas();
  bool AncestorsOrSiblingsAffectedByFocusVisibleInHas() const;
  void SetAncestorsOrSiblingsAffectedByFocusVisibleInHas();
  bool AffectedByLogicalCombinationsInHas() const;
  void SetAffectedByLogicalCombinationsInHas();
  bool AffectedByMultipleHas() const;
  void SetAffectedByMultipleHas();

  // This is meant to be used by document's resize observer to notify that the
  // size has changed.
  void LastRememberedSizeChanged(ResizeObserverSize* size);

  void SetLastRememberedInlineSize(std::optional<LayoutUnit>);
  void SetLastRememberedBlockSize(std::optional<LayoutUnit>);
  std::optional<LayoutUnit> LastRememberedInlineSize() const;
  std::optional<LayoutUnit> LastRememberedBlockSize() const;

  // Returns the element that represents the given |pseudo_id| and
  // |view_transition_name| originating from this DOM element.  The
  // returned element may be a PseudoElement, or (for element-backed
  // pseudo-elements) an Element.
  //
  // The returned pseudo element may be directly associated with this
  // element or (as with view transition pseudo-elements) nested inside
  // a hierarchy of pseudo elements.
  //
  // Callers that need to deal with all CSS pseudo-elements should use
  // this rather than GetPseudoElement().
  Element* GetStyledPseudoElement(
      PseudoId pseudo_id,
      const AtomicString& view_transition_name) const;

  void RecalcTransitionPseudoTreeStyle(
      const Vector<AtomicString>& view_transition_names);
  void RebuildTransitionPseudoLayoutTree(
      const Vector<AtomicString>& view_transition_names);

  // Returns true if the element has the 'inert' attribute, forcing itself and
  // all its subtree to be inert.
  // TODO(crbug.com/1511354): Make this not virtual after the override in
  // HTMLButtonElement::IsInertRoot is removed.
  virtual bool IsInertRoot() const;

  FocusgroupFlags GetFocusgroupFlags() const;

  bool checkVisibility(CheckVisibilityOptions* options) const;

  bool IsDocumentElement() const;

  bool IsReplacedElementRespectingCSSOverflow() const;

  void RemovePopoverData();
  PopoverData& EnsurePopoverData();
  PopoverData* GetPopoverData() const;

  InvokerData& EnsureInvokerData();
  InvokerData* GetInvokerData() const;

  void RemoveInterestInvokerTargetData();
  InterestInvokerTargetData& EnsureInterestInvokerTargetData();
  InterestInvokerTargetData* GetInterestInvokerTargetData() const;

  void DefaultEventHandler(Event&) override;

  // Set on elements with scroll-marker-contain property to
  // collect HTMLAnchorElement scroll markers.
  ScrollMarkerGroupData& EnsureScrollMarkerGroupData();
  void RemoveScrollMarkerGroupData();
  ScrollMarkerGroupData* GetScrollMarkerGroupData() const;

  // Used for HTMLAnchorElement scroll markers to point to
  // its scroll marker group container (element with scroll-marker-contain).
  void SetScrollMarkerGroupContainerData(ScrollMarkerGroupData*);
  ScrollMarkerGroupData* GetScrollMarkerGroupContainerData() const;

  // Retrieves the element pointed to by this element's 'anchor' content
  // attribute, if that element exists.
  // TODO(crbug.com/40059176) If the HTMLAnchorAttribute feature is disabled,
  // this will return nullptr;
  Element* anchorElement() const;
  Element* anchorElementForBinding() const;
  void setAnchorElementForBinding(Element*);

  AnchorPositionScrollData& EnsureAnchorPositionScrollData();
  void RemoveAnchorPositionScrollData();
  AnchorPositionScrollData* GetAnchorPositionScrollData() const;

  // Returns true if any element is implicitly anchored to this element.
  bool HasImplicitlyAnchoredElement() const;
  void DecrementImplicitlyAnchoredElementCount();
  void IncrementImplicitlyAnchoredElementCount();

  bool HasAnchorElementObserverForTesting() const {
    return GetAnchorElementObserver();
  }

  // https://drafts.csswg.org/css-anchor-1/#implicit-anchor-element
  Element* ImplicitAnchorElement() const;

  void UpdateDirectionalityAndDescendant(TextDirection direction);
  void UpdateDescendantHasDirAutoAttribute(bool has_dir_auto);
  enum class UpdateAncestorTraversal {
    IncludeSelf,  // self and ancestors
    ExcludeSelf,  // ancestors, but not self
  };
  void UpdateAncestorWithDirAuto(UpdateAncestorTraversal traversal);
  void AdjustDirectionalityIfNeededAfterChildrenChanged(
      const ChildrenChange& change);

  void UpdateDescendantHasContainerTiming(bool has_container_timing);
  void AdjustContainerTimingIfNeededAfterChildrenChanged(
      const ChildrenChange& change);
  bool ShouldAdjustContainerTimingForInsert(const ChildrenChange& change) const;
  bool DoesChildContainerTimingNeedChange(const Node& node) const;

  bool RecalcSelfOrAncestorHasContainerTiming() const;

  // The "nonce" attribute is hidden when:
  // 1) The Content-Security-Policy is delivered from the HTTP headers.
  // 2) The Element is part of the active document.
  // See https://github.com/whatwg/html/pull/2373
  //
  // This applies to the element of the HTML and SVG namespaces.
  //
  // This function clears the "nonce" attribute whenever conditions (1) and (2)
  // are met.
  void HideNonce();

  // These update every scroll container that is an ancestor of
  // of this element, letting them know which snap area of theirs, if any,
  // either is a targeted[1] element or contains a targeted[1] element.
  // [1]https://drafts.csswg.org/selectors/#the-target-pseudo
  void SetTargetedSnapAreaIdsForSnapContainers();
  void ClearTargetedSnapAreaIdsForSnapContainers();

  // Subclasses can override this method to specify a CascadeFilter to
  // filter out any unwanted CSS properties.
  virtual CascadeFilter GetCascadeFilter() const { return CascadeFilter(); }

  GCedHeapVector<Member<Element>>* ElementsFromAttributeOrInternals(
      const QualifiedName& attribute) const;

 protected:
  bool HasElementData() const { return static_cast<bool>(element_data_); }
  const ElementData* GetElementData() const { return element_data_.Get(); }
  UniqueElementData& EnsureUniqueElementData();

  bool IsViewportScrollElement();

  void AddPropertyToPresentationAttributeStyle(HeapVector<CSSPropertyValue, 8>&,
                                               CSSPropertyID,
                                               CSSValueID identifier);
  void AddPropertyToPresentationAttributeStyle(HeapVector<CSSPropertyValue, 8>&,
                                               CSSPropertyID,
                                               double value,
                                               CSSPrimitiveValue::UnitType);
  void AddPropertyToPresentationAttributeStyle(HeapVector<CSSPropertyValue, 8>&,
                                               CSSPropertyID,
                                               const String& value);
  void AddPropertyToPresentationAttributeStyle(HeapVector<CSSPropertyValue, 8>&,
                                               CSSPropertyID,
                                               const CSSValue&);
  void MapLanguageAttributeToLocale(const AtomicString&,
                                    HeapVector<CSSPropertyValue, 8>&);

  InsertionNotificationRequest InsertedInto(ContainerNode&) override;
  void RemovedFrom(ContainerNode&) override;
  void ChildrenChanged(const ChildrenChange&) override;

  // This is an implementation of
  // https://whatpr.org/html/10657/infrastructure.html#html-element-moving-steps
  void MovedFrom(ContainerNode& old_parent) override;

  virtual void WillRecalcStyle(const StyleRecalcChange);
  virtual void DidRecalcStyle(const StyleRecalcChange);
  virtual const ComputedStyle* CustomStyleForLayoutObject(
      const StyleRecalcContext&);
  virtual void AdjustStyle(ComputedStyleBuilder&);

  virtual NamedItemType GetNamedItemType() const {
    return NamedItemType::kNone;
  }

  // See description of SupportsFocus and IsFocusableState above, near
  // IsFocusable(). These two methods should stay protected. Use IsFocusable()
  // and friends.
  virtual FocusableState SupportsFocus(UpdateBehavior update_behavior) const;
  virtual FocusableState IsFocusableState(UpdateBehavior update_behavior) const;

  bool SupportsSpatialNavigationFocus() const;

  void ClearTabIndexExplicitlyIfNeeded();
  void SetTabIndexExplicitly();
  // Returns false if the style prevents focus. Returning true doesn't imply
  // focusability, there may be other conditions like SupportsFocus().
  // Subclasses may override this method to affect focusability. This method
  // might update layout/style, as it may use existence of layoutObject and the
  // LayoutObject::style() to reason about focusability.
  // However, it must not retrieve layout information like position and size.
  // This method cannot be moved to LayoutObject because some focusable nodes
  // don't have layoutObjects. e.g., HTMLOptionElement.
  // If UpdateBehavior::kNoneForAccessibility argument is passed, which should
  // only be used by a11y code, layout updates will never be performed.
  virtual bool IsFocusableStyle(
      UpdateBehavior update_behavior = UpdateBehavior::kStyleAndLayout) const;
  // Is the node descendant of this in something clickable/activatable, such
  // that we shouldn't handle events targeting it?
  bool IsClickableControl(Node*);

  // ClassAttributeChanged() and UpdateClassList() exist to share code between
  // ParseAttribute (called via setAttribute()) and SvgAttributeChanged (called
  // when element.className.baseVal is set or when the 'class' attribute is
  // animated by SMIL).
  void ClassAttributeChanged(const AtomicString& new_class_string);
  void UpdateClassList(const AtomicString& old_class_string,
                       const AtomicString& new_class_string);

  static bool AttributeValueIsJavaScriptURL(const Attribute&);

  const ComputedStyle* OriginalStyleForLayoutObject(const StyleRecalcContext&);

  // Step 4 of http://domparsing.spec.whatwg.org/#insertadjacenthtml()
  Node* InsertAdjacent(const String& where, Node* new_child, ExceptionState&);

  virtual void ParserDidSetAttributes() {}

  // Mark for style invalidation/recalc for :lang() selectors to pick up the
  // changes.
  virtual void LangAttributeChanged();

  TextDirection ParentDirectionality() const;
  bool RecalcSelfOrAncestorHasDirAuto();
  std::optional<TextDirection> ResolveAutoDirectionality() const;

  void AttachPseudoElement(PseudoId, AttachContext&);
  void DetachPseudoElement(PseudoId, bool performing_reattach);

  void ProcessElementRenderBlocking(const AtomicString& id_or_name);

 private:
  friend class AXObject;
  friend class KeyboardEventManager;
  struct AffectedByPseudoStateChange;

  template <typename Functor>
  bool PseudoElementStylesDependOnFunc(Functor& func) const;

  // Returns true if the element satisfies conditions for focusability for
  // spatial navigation, even if the spatial navigation is not currently
  // enabled.
  bool HasSpatialNavigationFocusHeuristics() const;

  // Returns true if this element has generate a pseudo element whose box is a
  // sibling box of its originating element's box. In this case we cannot skip
  // style recalc for size containers because that would break necessary layout
  // containment by modifying the box tree outside the container during layout.
  bool HasSiblingBoxPseudoElements() const;

  void ScrollLayoutBoxBy(const ScrollToOptions*);
  void ScrollLayoutBoxTo(const ScrollToOptions*);
  void ScrollFrameBy(const ScrollToOptions*);
  void ScrollFrameTo(const ScrollToOptions*);

  bool HasElementFlag(ElementFlags mask) const;
  void SetElementFlag(ElementFlags, bool value = true);
  void ClearElementFlag(ElementFlags);

  void ClearPseudoElement(
      PseudoId,
      const AtomicString& view_transition_name = g_null_atom);
  void ClearTransitionPseudoTreeIfNeeded(const StyleRecalcChange);

  bool IsElementNode() const =
      delete;  // This will catch anyone doing an unnecessary check.
  bool IsDocumentFragment() const =
      delete;  // This will catch anyone doing an unnecessary check.
  bool IsDocumentNode() const =
      delete;  // This will catch anyone doing an unnecessary check.

  bool CanAttachShadowRoot() const;
  const char* ErrorMessageForAttachShadow(String mode_string,
                                          bool for_declarative,
                                          ShadowRootMode& mode_out) const;

  void StyleAttributeChanged(const AtomicString& new_style_string,
                             AttributeModificationReason);

  void UpdatePresentationAttributeStyle();

  void InlineStyleChanged();
  void SetInlineStyleFromString(const AtomicString&);

  void NotifyAXOfAttachedSubtree();

  AnchorElementObserver& EnsureAnchorElementObserver();
  AnchorElementObserver* GetAnchorElementObserver() const;

  // If the only inherited changes in the parent element are independent,
  // these changes can be directly propagated to this element (the child).
  // If these conditions are met, propagates the changes to the current style
  // and returns the new style. Otherwise, returns null.
  const ComputedStyle* PropagateInheritedProperties();

  const ComputedStyle* EnsureOwnComputedStyle(
      const StyleRecalcContext&,
      PseudoId,
      const AtomicString& pseudo_argument = g_null_atom);

  enum class HighlightRecalc {
    // No highlight recalc is needed.
    kNone,
    // The HighlightData from the old style can be reused.
    kReuse,
    // The HighlightData contains relative units and may need recalc.
    kOriginatingDependent,
    // Highlights must be calculated in full.
    kFull,
  };

  // Determine whether pseudo highlight style must be recalculated,
  // either because full recalc is required or the parent has relative
  // units and the parent's relative units source differs from the
  // originating element (font size, container or writing mode).
  bool ShouldRecalcHighlightPseudoStyle(
      HighlightRecalc highlight_recalc,
      const ComputedStyle* highlight_parent,
      const ComputedStyle& originating_style,
      const Element* originating_container) const;

  // Recalc those custom highlights that require it.
  void RecalcCustomHighlightPseudoStyle(const StyleRecalcContext&,
                                        HighlightRecalc,
                                        ComputedStyleBuilder&,
                                        const StyleHighlightData*,
                                        const ComputedStyle&);

  // Recalculate the ComputedStyle for this element and return a
  // StyleRecalcChange for propagation/traversal into child nodes.
  StyleRecalcChange RecalcOwnStyle(const StyleRecalcChange,
                                   const StyleRecalcContext&);

  // Returns true if we should skip style recalc for the subtree because this
  // element is a container for size container queries and we are guaranteed to
  // reach this element during the subsequent layout to continue doing
  // interleaved style and layout.
  bool SkipStyleRecalcForContainer(const ComputedStyle& style,
                                   const StyleRecalcChange& child_change,
                                   const StyleRecalcContext&);

  void MarkNonSlottedHostChildrenForStyleRecalc();

  void RebuildPseudoElementLayoutTree(PseudoId, WhitespaceAttacher&);
  void RebuildColumnLayoutTrees(WhitespaceAttacher&);
  void RebuildFirstLetterLayoutTree();
  void RebuildShadowRootLayoutTree(WhitespaceAttacher&);
  inline void CheckForEmptyStyleChange(const Node* node_before_change,
                                       const Node* node_after_change);

  void UpdateColumnPseudoElements(const StyleRecalcChange,
                                  const StyleRecalcContext&);
  PseudoElement* UpdateLayoutSiblingPseudoElement(PseudoId pseudo_id,
                                                  const StyleRecalcChange,
                                                  const StyleRecalcContext&);
  PseudoElement* UpdatePseudoElement(
      PseudoId,
      const StyleRecalcChange,
      const StyleRecalcContext&,
      const AtomicString& view_transition_name = g_null_atom);
  enum class StyleUpdatePhase {
    kRecalc,
    kRebuildLayoutTree,
    kAttachLayoutTree,
  };

  bool ShouldUpdateBackdropPseudoElement(const StyleRecalcChange);

  void UpdateBackdropPseudoElement(const StyleRecalcChange,
                                   const StyleRecalcContext&);

  void UpdateFirstLetterPseudoElement(StyleUpdatePhase,
                                      const StyleRecalcContext&);

  // Creates a StyleRecalcContext and invokes the method above. Only use this
  // when there is no StyleRecalcContext available.
  void UpdateFirstLetterPseudoElement(StyleUpdatePhase);

  inline PseudoElement* CreatePseudoElementIfNeeded(
      PseudoId,
      const StyleRecalcContext&,
      const AtomicString& view_transition_name = g_null_atom);

  // For document element scroll control pseudo elements become not layout
  // siblings, but layout children.
  void AttachDocumentElementPrecedingPseudoElements(AttachContext& context) {
    if (!IsDocumentElement()) {
      return;
    }
    AttachPrecedingScrollControlsPseudoElements(context);
  }

  void AttachLayoutPrecedingPseudoElements(AttachContext& context) {
    if (IsDocumentElement()) {
      return;
    }
    AttachPrecedingScrollControlsPseudoElements(context);
  }

  void AttachPrecedingScrollControlsPseudoElements(AttachContext& context) {
    AttachPseudoElement(kPseudoIdScrollMarkerGroupBefore, context);
  }

  void AttachPrecedingPseudoElements(AttachContext& context) {
    AttachDocumentElementPrecedingPseudoElements(context);
    AttachPseudoElement(kPseudoIdScrollMarker, context);
    AttachPseudoElement(kPseudoIdMarker, context);
    AttachPseudoElement(kPseudoIdCheckMark, context);
    AttachPseudoElement(kPseudoIdBefore, context);
  }

  // For document element scroll control pseudo elements become not layout
  // siblings, but layout children.
  void AttachDocumentElementSucceedingPseudoElements(AttachContext& context) {
    if (!IsDocumentElement()) {
      return;
    }
    AttachSucceedingScrollControlsPseudoElements(context);
  }

  void AttachLayoutSucceedingPseudoElements(AttachContext& context) {
    if (IsDocumentElement()) {
      return;
    }
    AttachSucceedingScrollControlsPseudoElements(context);
  }

  void AttachSucceedingPseudoElements(AttachContext& context) {
    AttachPseudoElement(kPseudoIdPickerIcon, context);
    AttachPseudoElement(kPseudoIdAfter, context);
    AttachDocumentElementSucceedingPseudoElements(context);
    AttachPseudoElement(kPseudoIdBackdrop, context);
    UpdateFirstLetterPseudoElement(StyleUpdatePhase::kAttachLayoutTree);
    AttachPseudoElement(kPseudoIdFirstLetter, context);
  }

  void AttachSucceedingScrollControlsPseudoElements(AttachContext& context) {
    // The order for buttons is described in
    // https://drafts.csswg.org/css-overflow-5/#scroll-buttons.
    AttachPseudoElement(kPseudoIdScrollButtonBlockStart, context);
    AttachPseudoElement(kPseudoIdScrollButtonInlineStart, context);
    AttachPseudoElement(kPseudoIdScrollButtonInlineEnd, context);
    AttachPseudoElement(kPseudoIdScrollButtonBlockEnd, context);
    AttachPseudoElement(kPseudoIdScrollMarkerGroupAfter, context);
  }

  void AttachColumnPseudoElements(AttachContext& context);

  void DetachPrecedingPseudoElements(bool performing_reattach) {
    DetachPseudoElement(kPseudoIdScrollMarker, performing_reattach);
    DetachPseudoElement(kPseudoIdScrollMarkerGroupBefore, performing_reattach);
    DetachPseudoElement(kPseudoIdMarker, performing_reattach);
    DetachPseudoElement(kPseudoIdCheckMark, performing_reattach);
    DetachPseudoElement(kPseudoIdBefore, performing_reattach);
  }

  void DetachSucceedingPseudoElements(bool performing_reattach) {
    DetachPseudoElement(kPseudoIdPickerIcon, performing_reattach);
    DetachPseudoElement(kPseudoIdAfter, performing_reattach);
    DetachPseudoElement(kPseudoIdScrollButtonBlockStart, performing_reattach);
    DetachPseudoElement(kPseudoIdScrollButtonInlineStart, performing_reattach);
    DetachPseudoElement(kPseudoIdScrollButtonInlineEnd, performing_reattach);
    DetachPseudoElement(kPseudoIdScrollButtonBlockEnd, performing_reattach);
    DetachPseudoElement(kPseudoIdScrollMarkerGroupAfter, performing_reattach);
    DetachPseudoElement(kPseudoIdBackdrop, performing_reattach);
    DetachPseudoElement(kPseudoIdFirstLetter, performing_reattach);
  }

  void DetachColumnPseudoElements(bool performing_reattach);

  void RecomputeDirectionFromParent();

  // Returns true if the directionality needs to be updated for insert.
  bool ShouldAdjustDirectionalityForInsert(const ChildrenChange& change) const;

  // Returns true if node is a text node and the direction is not set, or
  // matches this. Generally only useful from
  // ShouldAdjustDirectionalityForInsert().
  bool DoesChildTextNodesDirectionMatchThis(const Node& node) const;

  ShadowRoot& CreateAndAttachShadowRoot(
      ShadowRootMode,
      SlotAssignmentMode = SlotAssignmentMode::kNamed);

  // FIXME: Everyone should allow author shadows.
  virtual bool AreAuthorShadowsAllowed() const { return true; }
  virtual void DidAddUserAgentShadowRoot(ShadowRoot&) {}
  virtual bool AlwaysCreateUserAgentShadowRoot() const { return false; }

  void DidAddAttribute(const QualifiedName&, const AtomicString&);
  void WillModifyAttribute(const QualifiedName&,
                           const AtomicString& old_value,
                           const AtomicString& new_value);
  void DidModifyAttribute(const QualifiedName&,
                          const AtomicString& old_value,
                          const AtomicString& new_value,
                          AttributeModificationReason reason);
  void DidRemoveAttribute(const QualifiedName&, const AtomicString& old_value);

  void SynchronizeAllAttributes() const;
  void SynchronizeAttribute(const QualifiedName&) const;
  void SynchronizeAllAttributesExceptStyle() const;

  void UpdateId(const AtomicString& old_id, const AtomicString& new_id);
  void UpdateId(TreeScope&,
                const AtomicString& old_id,
                const AtomicString& new_id);
  void UpdateName(const AtomicString& old_name, const AtomicString& new_name);

  void UpdateFocusgroup(const AtomicString& input);
  void UpdateFocusgroupInShadowRootIfNeeded();

  void ClientQuads(Vector<gfx::QuadF>& quads) const;

  bool ChildTypeAllowed(NodeType) const final;

  // Returns the attribute's index or `kNotFound` if not found.
  wtf_size_t FindAttributeIndex(const QualifiedName&) const;

  void SetAttributeInternal(wtf_size_t index,
                            const QualifiedName&,
                            const AtomicString& value,
                            AttributeModificationReason);
  void AppendAttributeInternal(const QualifiedName&,
                               const AtomicString& value,
                               AttributeModificationReason);
  void RemoveAttributeInternal(wtf_size_t index, AttributeModificationReason);
  SpecificTrustedType ExpectedTrustedTypeForAttribute(
      const QualifiedName&) const;

  // These Hinted versions of the functions are subtle hot path
  // optimizations designed to reduce the number of unnecessary AtomicString
  // creations, AtomicStringTable lookups, and LowerCaseIfNecessary calls.
  //
  // The `hint` is the result of a WeakLowercaseIfNecessary() call unless it is
  // known that the the incoming string already has the right case. Then
  // the `hint` can be constructed from calling AtomicString::Impl().
  const AtomicString& GetAttributeHinted(
      const AtomicString& name,
      WTF::AtomicStringTable::WeakResult hint) const;
  void RemoveAttributeHinted(const AtomicString& name,
                             WTF::AtomicStringTable::WeakResult hint);
  void SynchronizeAttributeHinted(
      const AtomicString& name,
      WTF::AtomicStringTable::WeakResult hint) const;
  void SetAttributeHinted(AtomicString name,
                          WTF::AtomicStringTable::WeakResult hint,
                          String value,
                          ExceptionState& = ASSERT_NO_EXCEPTION);
  void SetAttributeHinted(AtomicString name,
                          WTF::AtomicStringTable::WeakResult hint,
                          const V8TrustedType* trusted_string,
                          ExceptionState& exception_state);
  std::pair<wtf_size_t, const QualifiedName> LookupAttributeQNameHinted(
      AtomicString name,
      WTF::AtomicStringTable::WeakResult hint) const;
  wtf_size_t ValidateAttributeIndex(wtf_size_t index,
                                    const QualifiedName& qname) const;

  void CancelSelectionAfterLayout();
  virtual int DefaultTabIndex() const;

  inline void UpdateCallbackSelectors(const ComputedStyle* old_style,
                                      const ComputedStyle* new_style);
  inline void NotifyIfMatchedDocumentRulesSelectorsChanged(
      const ComputedStyle* old_style,
      const ComputedStyle* new_style);

  // Clone is private so that non-virtual CloneElementWithChildren and
  // CloneElementWithoutChildren are used instead.
  Node* Clone(Document& factory,
              NodeCloningData& data,
              ContainerNode* append_to,
              ExceptionState& append_exception_state) const override;

  virtual Element& CloneWithoutAttributesAndChildren(Document& factory) const;

  void UpdateNamedItemRegistration(NamedItemType,
                                   const AtomicString& old_name,
                                   const AtomicString& new_name);
  void UpdateIdNamedItemRegistration(NamedItemType,
                                     const AtomicString& old_name,
                                     const AtomicString& new_name);

  void CreateUniqueElementData();

  bool ShouldInvalidateDistributionWhenAttributeChanged(ShadowRoot&,
                                                        const QualifiedName&,
                                                        const AtomicString&);

  void SetInnerHTMLInternal(
      const String&,
      ParseDeclarativeShadowRoots parse_declarative_shadows,
      ForceHtml force_html_over_xml,
      ExceptionState&);

  ElementRareDataVector* GetElementRareData() const;
  ElementRareDataVector& EnsureElementRareData();

  void RemoveAttrNodeList();
  void DetachAllAttrNodesFromElement();
  void DetachAttrNodeFromElementWithValue(Attr*, const AtomicString& value);
  void DetachAttrNodeAtIndex(Attr*, wtf_size_t index);

  void SynchronizeContentAttributeAndElementReference(
      const QualifiedName& name);

  DisplayLockContext* GetDisplayLockContextFromRareData() const;

  void PseudoStateChanged(CSSSelector::PseudoType pseudo,
                          AffectedByPseudoStateChange&&);

  void ProcessContainIntrinsicSizeChanges();

  bool ShouldUpdateLastRememberedBlockSize() const;
  bool ShouldUpdateLastRememberedInlineSize() const;

  bool IsStyleAttributeChangeAllowed(const AtomicString& style_string);

  // These schedule interest gained/lost events, for `interesttarget` invokers.
  void ScheduleInterestGainedTask(InterestState);
  void ScheduleInterestLostTask();
  void ChangeInterestState(Element* target, InterestState new_state);
  static bool GainOrLoseInterest(Element* invoker,
                                 Element* target,
                                 InterestState new_state);
  enum class InterestTargetSource {
    // This element was hovered.
    kHover,
    // This element was de-hovered.
    kDeHover,
    // This element was focused.
    kFocus,
    // This element was blurred.
    kBlur,
    // (Recursive call only) Inclusive ancestor chain of an element focused.
    kFocusElementChain,
    // (Recursive call only) Inclusive ancestor chain of an element blurred.
    kBlurElementChain,
  };
  void HandleInterestTargetHoverOrFocus(InterestTargetSource source);

  // Highlight pseudos inherit all properties from the corresponding highlight
  // in the parent, but virtually all existing content uses universal rules
  // like *::selection. To improve runtime and keep copy-on-write inheritance,
  // avoid recalc if neither parent nor child matched any non-universal rules.
  HighlightRecalc CalculateHighlightRecalc(
      const ComputedStyle* old_style,
      const ComputedStyle& new_style,
      const ComputedStyle* parent_style) const;

  // This checks that the feature KeyboardFocusableScrollers is enabled and
  // element is a scroller. This will call IsScrollableNode, which might update
  // layout.
  // If UpdateBehavior::kNoneForAccessibility argument is passed, which should
  // only be used by a11y code, layout updates will never be performed.
  bool CanBeKeyboardFocusableScroller(
      UpdateBehavior update_behavior = UpdateBehavior::kStyleAndLayout) const;
  // This checks whether the element is a scrollable container that should be
  // made keyboard focusable. Note that this is slow, because it must do a tree
  // walk to look for descendant focusable nodes.
  // If UpdateBehavior::kNoneForAccessibility argument is passed, which should
  // only be used by a11y code, layout updates will never be performed.
  bool IsKeyboardFocusableScroller(
      UpdateBehavior update_behavior = UpdateBehavior::kStyleAndLayout) const;

  FrozenArray<Element>* GetElementArrayAttribute(const QualifiedName& name);
  void SetElementArrayAttribute(
      const QualifiedName& name,
      const GCedHeapVector<Member<Element>>* given_elements);

  // Find the scroll-marker that should be active when told to scroll |this|
  // into view.
  ScrollMarkerPseudoElement* FindScrollMarkerForTargetedScroll();
  // Let the appropriate scroll-marker-group know to pin its active
  // scroll-marker due to a targeted scroll.
  void NotifyScrollMarkerGroupOfTargetedScroll();

  QualifiedName tag_name_;
  // This `ComputedStyle` field is a hot accessed member. Keep uncompressed for
  // performance reasons.
  subtle::UncompressedMember<const ComputedStyle> computed_style_;
  Member<ElementData> element_data_;

  // A tiny Bloom filter for which attribute names we have; saves going to
  // ElementData if the attribute doesn't exist. May have false positives,
  // of course. We do not currently update this when attributes are removed,
  // only when they are added. Attribute _values_ are not part of this filter.
  uint32_t attribute_bloom_ = 0;
};

template <>
struct DowncastTraits<Element> {
  static bool AllowFrom(const Node& node) { return node.IsElementNode(); }
};

inline bool IsDisabledFormControl(const Node* node) {
  auto* element = DynamicTo<Element>(node);
  return element && element->IsDisabledFormControl();
}

inline Element* Node::parentElement() const {
  return DynamicTo<Element>(parentNode());
}

inline bool Element::FastHasAttribute(const QualifiedName& name) const {
#if DCHECK_IS_ON()
  DCHECK(FastAttributeLookupAllowed(name))
      << TagQName().ToString().Utf8() << "/@" << name.ToString().Utf8();
#endif
  return HasElementData() && GetElementData()->Attributes().Find(name);
}

inline const AtomicString& Element::FastGetAttribute(
    const QualifiedName& name) const {
#if DCHECK_IS_ON()
  DCHECK(FastAttributeLookupAllowed(name))
      << TagQName().ToString().Utf8() << "/@" << name.ToString().Utf8();
#endif
  if (HasElementData()) {
    if (const Attribute* attribute = GetElementData()->Attributes().Find(name))
      return attribute->Value();
  }
  return g_null_atom;
}

inline AttributeCollection Element::Attributes() const {
  if (!HasElementData())
    return AttributeCollection();
  SynchronizeAllAttributes();
  return GetElementData()->Attributes();
}

inline AttributeCollection Element::AttributesWithoutUpdate() const {
  if (!HasElementData())
    return AttributeCollection();
  return GetElementData()->Attributes();
}

inline AttributeCollection Element::AttributesWithoutStyleUpdate() const {
  if (!HasElementData())
    return AttributeCollection();
  SynchronizeAllAttributesExceptStyle();
  return GetElementData()->Attributes();
}

inline bool Element::hasAttributes() const {
  return !Attributes().IsEmpty();
}

inline const AtomicString& Element::IdForStyleResolution() const {
  DCHECK(HasID());
  return GetElementData()->IdForStyleResolution();
}

inline const AtomicString& Element::GetIdAttribute() const {
  return HasID() ? FastGetAttribute(html_names::kIdAttr) : g_null_atom;
}

inline const AtomicString& Element::GetNameAttribute() const {
  return HasName() ? FastGetAttribute(html_names::kNameAttr) : g_null_atom;
}

inline const AtomicString& Element::GetClassAttribute() const {
  if (!HasClass())
    return g_null_atom;
  if (IsSVGElement())
    return getAttribute(html_names::kClassAttr);
  return FastGetAttribute(html_names::kClassAttr);
}

inline void Element::SetIdAttribute(const AtomicString& value) {
  setAttribute(html_names::kIdAttr, value);
}

inline const SpaceSplitString& Element::ClassNames() const {
  DCHECK(HasClass());
  DCHECK(HasElementData());
  return GetElementData()->ClassNames();
}

inline bool Element::HasClassName(const AtomicString& class_name) const {
  return HasElementData() &&
         GetElementData()->ClassNames().Contains(class_name);
}

inline bool Element::HasID() const {
  return HasElementData() && GetElementData()->HasID();
}

inline bool Element::HasClass() const {
  return HasElementData() && GetElementData()->HasClass();
}

inline UniqueElementData& Element::EnsureUniqueElementData() {
  if (!HasElementData() || !GetElementData()->IsUnique())
    CreateUniqueElementData();
  return To<UniqueElementData>(*element_data_);
}

inline const CSSPropertyValueSet* Element::PresentationAttributeStyle() {
  if (!HasElementData())
    return nullptr;
  if (GetElementData()->presentation_attribute_style_is_dirty())
    UpdatePresentationAttributeStyle();
  // Need to call elementData() again since updatePresentationAttributeStyle()
  // might swap it with a UniqueElementData.
  return GetElementData()->PresentationAttributeStyle();
}

inline void Element::SetTagNameForCreateElementNS(
    const QualifiedName& tag_name) {
  // We expect this method to be called only to reset the prefix.
  DCHECK_EQ(tag_name.LocalName(), tag_name_.LocalName());
  DCHECK_EQ(tag_name.NamespaceURI(), tag_name_.NamespaceURI());
  tag_name_ = tag_name;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ELEMENT_H_
