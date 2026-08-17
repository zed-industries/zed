/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 * Copyright (C) 2004-2007, 2009, 2014 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_ELEMENT_H_

#include <optional>

#include "third_party/blink/renderer/bindings/core/v8/v8_union_boolean_string_unrestricteddouble.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/core/dom/events/simulated_click_options.h"
#include "third_party/blink/renderer/core/html/forms/labels_node_list.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/text/text_direction.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

struct AttributeTriggers;
class Color;
class DocumentFragment;
class ElementInternals;
class ExceptionState;
class FormAssociated;
class HTMLFormElement;
class KeyboardEvent;
class PointerEvent;
class TextControlElement;
class V8UnionStringLegacyNullToEmptyStringOrTrustedScript;
class V8UnionBooleanOrTogglePopoverOptions;
class ShowPopoverOptions;

enum TranslateAttributeMode {
  kTranslateAttributeYes,
  kTranslateAttributeNo,
  kTranslateAttributeInherit
};

enum class ContentEditableType {
  kInherit,
  kContentEditable,
  kNotContentEditable,
  kPlaintextOnly,
};

enum class PopoverValueType {
  kNone,
  kAuto,
  kHint,
  kManual,
};

enum class PopoverTriggerAction {
  kNone,
  kToggle,
  kShow,
  kHide,
};

enum class HidePopoverFocusBehavior {
  kNone,
  kFocusPreviousElement,
};

enum class HidePopoverTransitionBehavior {
  // Fire events (e.g. beforetoggle) which can run synchronous script, and
  // also wait for CSS transitions of `overlay` before removing the popover
  // from the top layer. This should be used for most "normal" popover hide
  // operations.
  kFireEventsAndWaitForTransitions,
  // Does not fire any events, nor wait for CSS transitions. The popover is
  // removed immediately from the top layer. This should be used when the UA
  // is forcibly removing the popover from the top layer, e.g. when other
  // higher priority elements are entering the top layer, or when the popover
  // element is being removed from the document.
  kNoEventsNoWaiting,
};

enum class TopLayerElementType {
  kPopover,
  kDialog,
  kFullscreen,
};

class CORE_EXPORT HTMLElement : public Element {
  DEFINE_WRAPPERTYPEINFO();

 public:
  HTMLElement(const QualifiedName& tag_name, Document&, ConstructionType);

  bool HasTagName(const HTMLQualifiedName& name) const {
    DCHECK_EQ(name.NamespaceURI(), namespaceURI());
    return HasLocalName(name.LocalName());
  }

  const char* NameInHeapSnapshot() const override;

  String title() const final;

  void setInnerText(const String&);
  V8UnionStringLegacyNullToEmptyStringOrTrustedScript* innerTextForBinding();
  virtual void setInnerTextForBinding(
      const V8UnionStringLegacyNullToEmptyStringOrTrustedScript*
          string_or_trusted_script,
      ExceptionState& exception_state);
  void setOuterText(const String&, ExceptionState&);

  virtual bool HasCustomFocusLogic() const;

  ContentEditableType contentEditableNormalized() const;
  String contentEditable() const;
  void setContentEditable(const String&, ExceptionState&);
  // For HTMLElement.prototype.isContentEditable. This matches to neither
  // blink::isContentEditable() nor blink::isContentRichlyEditable().  Do not
  // use this function in Blink.
  bool isContentEditableForBinding() const;

  virtual const AtomicString& autocapitalize() const;
  void setAutocapitalize(const AtomicString&);

  virtual bool draggable() const;
  void setDraggable(bool);

  bool spellcheck() const;
  void setSpellcheck(bool);

  bool translate() const;
  void setTranslate(bool);

  const AtomicString& dir();
  void setDir(const AtomicString&);

  void click();

  void AccessKeyAction(SimulatedClickCreationScope creation_scope) override;

  bool ShouldSerializeEndTag() const;

  virtual HTMLFormElement* formOwner() const;
  virtual HTMLElement* formForBinding() const;

  HTMLFormElement* FindFormAncestor() const;

  bool HasDirectionAuto() const;

  static bool IsValidDirAttribute(const AtomicString& value);
  static bool ElementAffectsDirectionality(const Node* node);
  static bool ElementInheritsDirectionality(const Node* node) {
    return !HTMLElement::ElementAffectsDirectionality(node);
  }

  static const TextControlElement*
  ElementIfAutoDirectionalityFormAssociatedOrNull(const Element* element);
  static TextControlElement* ElementIfAutoDirectionalityFormAssociatedOrNull(
      Element* element) {
    return const_cast<TextControlElement*>(
        ElementIfAutoDirectionalityFormAssociatedOrNull(
            const_cast<const Element*>(element)));
  }

  virtual bool IsHTMLBodyElement() const { return false; }
  // TODO(crbug.com/1123606): Remove this virtual method once the fenced frame
  // origin trial is over.
  virtual bool IsHTMLFencedFrameElement() const { return false; }
  virtual bool IsHTMLFrameSetElement() const { return false; }
  virtual bool IsHTMLPermissionElement() const { return false; }
  virtual bool IsHTMLUnknownElement() const { return false; }
  virtual bool IsPluginElement() const { return false; }

  // https://html.spec.whatwg.org/C/#category-label
  virtual bool IsLabelable() const;
  // |labels| IDL attribute implementation for IsLabelable()==true elements.
  LabelsNodeList* labels();
  bool HasActiveLabel() const;

  // https://html.spec.whatwg.org/C/#interactive-content
  virtual bool IsInteractiveContent() const;
  void DefaultEventHandler(Event&) override;

  // Used to handle return/space key events and simulate clicks. Returns true
  // if the event is handled.
  bool HandleKeyboardActivation(Event& event);

  static const AtomicString& EventNameForAttributeName(
      const QualifiedName& attr_name);

  bool IsDisabledFormControl() const override;
  bool MatchesEnabledPseudoClass() const override;
  bool MatchesReadOnlyPseudoClass() const override;
  bool MatchesReadWritePseudoClass() const override;
  bool MatchesValidityPseudoClasses() const override;
  bool willValidate() const override;
  bool IsValidElement() override;

  static const AtomicString& EventParameterName();

  virtual String AltText() const { return String(); }

  // unclosedOffsetParent doesn't return Elements which are closed shadow hidden
  // from this element. offsetLeftForBinding and offsetTopForBinding have their
  // values adjusted for this as well.
  Element* unclosedOffsetParent();
  int offsetLeftForBinding();
  int offsetTopForBinding();
  int offsetWidthForBinding();
  int offsetHeightForBinding();

  ElementInternals* attachInternals(ExceptionState& exception_state);
  virtual FormAssociated* ToFormAssociatedOrNull() { return nullptr; }
  bool IsFormAssociatedCustomElement() const;

  void UpdateDescendantDirectionality(TextDirection direction);
  void UpdateDirectionalityAfterInputTypeChange(const AtomicString& old_value,
                                                const AtomicString& new_value);
  void AdjustDirectionAutoAfterRecalcAssignedNodes();
  virtual bool CalculateAndAdjustAutoDirectionality();

  V8UnionBooleanOrStringOrUnrestrictedDouble* hidden() const;
  void setHidden(const V8UnionBooleanOrStringOrUnrestrictedDouble*);

  // https://html.spec.whatwg.org/C/#potentially-render-blocking
  virtual bool IsPotentiallyRenderBlocking() const { return false; }

  // Popover API related functions.
  void UpdatePopoverAttribute(const AtomicString&);
  bool HasPopoverAttribute() const;
  PopoverValueType PopoverType() const;
  bool popoverOpen() const;
  // IsPopoverReady returns true if the popover is in a state where it can be
  // either shown or hidden based on |action|. If exception_state is set, then
  // it will throw an exception if the state is not ready to transition to the
  // state in |action|. |include_event_handler_text| adds some additional text
  // to the exception if an exception is thrown. When |expected_document| is
  // set, it will be compared to the current document and return false if they
  // do not match.
  bool IsPopoverReady(PopoverTriggerAction action,
                      ExceptionState* exception_state,
                      bool include_event_handler_text,
                      Document* expected_document) const;
  bool togglePopover(ExceptionState& exception_state);
  bool togglePopover(V8UnionBooleanOrTogglePopoverOptions* options_or_force,
                     ExceptionState& exception_state);
  void showPopover(ExceptionState& exception_state);
  void showPopover(ShowPopoverOptions* options,
                   ExceptionState& exception_state);
  void hidePopover(ExceptionState& exception_state);
  // |exception_state| can be nullptr when exceptions can't be thrown, such as
  // when the browser hides a popover during light dismiss or shows a popover in
  // response to clicking a button with popovershowtarget.
  virtual void ShowPopoverInternal(Element* invoker,
                                   ExceptionState* exception_state);
  virtual void HidePopoverInternal(HidePopoverFocusBehavior focus_behavior,
                                   HidePopoverTransitionBehavior event_firing,
                                   ExceptionState* exception_state);
  void PopoverHideFinishIfNeeded(bool immediate);
  static const HTMLElement* FindTopmostPopoverAncestor(
      Element& new_popover_or_top_layer_element,
      HeapVector<Member<HTMLElement>>& stack_to_check,
      Element* new_popovers_invoker,
      TopLayerElementType top_layer_element_type =
          TopLayerElementType::kPopover);
  static const HTMLElement* TopLayerElementPopoverAncestor(
      Element& top_layer_element,
      TopLayerElementType top_layer_element_type);

  static void HandlePopoverLightDismiss(const PointerEvent& event,
                                        const Node& node);
  void InvokePopover(Element& invoker);
  void SetPopoverFocusOnShow();
  // This hides all visible popovers up to, but not including,
  // |endpoint|. If |endpoint| is nullptr, all popovers are hidden.
  static void HideAllPopoversUntil(const HTMLElement*,
                                   Document&,
                                   HidePopoverFocusBehavior,
                                   HidePopoverTransitionBehavior);

  void SetImplicitAnchor(Element* element);
  Element* implicitAnchor() const;

  bool DispatchFocusEvent(
      Element* old_focused_element,
      mojom::blink::FocusType,
      InputDeviceCapabilities* source_capabilities) override;

  // This allows customization of how Invoker Commands are handled, per
  // element. The default HTMLElement behavior handles popovers, and specific
  // element subclasses - such as HTMLDialogElement - can handle
  // other commands such as showModal. Implementations should return
  // `true` if they have handled, so that overrides can exit early.
  // Additionally, override implementations should not execute their own
  // behavior before calling `HTMLElement::HandleCommandInternal` as that
  // override governs the logic for global attributes such as `popover`;
  // for example a `<dialog popover>` should run `popover` invocation steps
  // before `<dialog>` invocation steps.
  // See: crbug.com/1490919, https://open-ui.org/components/invokers.explainer/
  bool IsValidBuiltinCommand(HTMLElement& invoker,
                             CommandEventType command) override;
  bool IsValidBuiltinPopoverCommand(HTMLElement& invoker,
                                    CommandEventType command);
  bool HandleCommandInternal(HTMLElement& invoker,
                             CommandEventType command) override;

  // This allows developers to enable or disable browser-provided writing
  // suggestions. If the attribute is not explicitly set on an element, it
  // inherits its value from ancestor elements; otherwise, it defaults to
  // "true". Spec: https://github.com/whatwg/html/pull/10018.
  AtomicString writingSuggestions() const;
  void setWritingSuggestions(const AtomicString& value);

 protected:
  FocusableState SupportsFocus(UpdateBehavior update_behavior) const override;

  enum AllowPercentage { kDontAllowPercentageValues, kAllowPercentageValues };
  enum AllowZero { kDontAllowZeroValues, kAllowZeroValues };
  void AddHTMLLengthToStyle(HeapVector<CSSPropertyValue, 8>&,
                            CSSPropertyID,
                            const String& value,
                            AllowPercentage = kAllowPercentageValues,
                            AllowZero = kAllowZeroValues);
  void AddHTMLColorToStyle(HeapVector<CSSPropertyValue, 8>&,
                           CSSPropertyID,
                           const String& color);
  void AddHTMLBackgroundImageToStyle(
      HeapVector<CSSPropertyValue, 8>&,
      const String& url_value,
      const AtomicString& initiator_name = g_null_atom);

  // This corresponds to:
  //  'map to the aspect-ratio property (using dimension rules)'
  // described by:
  // https://html.spec.whatwg.org/multipage/rendering.html#map-to-the-aspect-ratio-property-(using-dimension-rules)
  void ApplyAspectRatioToStyle(const AtomicString& width,
                               const AtomicString& height,
                               HeapVector<CSSPropertyValue, 8>&);
  // This corresponds to:
  //  'map to the aspect-ratio property'
  // described by:
  // https://html.spec.whatwg.org/multipage/rendering.html#map-to-the-aspect-ratio-property
  void ApplyIntegerAspectRatioToStyle(const AtomicString& width,
                                      const AtomicString& height,
                                      HeapVector<CSSPropertyValue, 8>&);
  void ApplyAlignmentAttributeToStyle(const AtomicString&,
                                      HeapVector<CSSPropertyValue, 8>&);
  void ApplyBorderAttributeToStyle(const AtomicString&,
                                   HeapVector<CSSPropertyValue, 8>&);

  void AttributeChanged(const AttributeModificationParams&) override;
  void ParseAttribute(const AttributeModificationParams&) override;
  static bool ParseColorWithLegacyRules(const String& attribute_value,
                                        Color& parsed_color);
  bool IsPresentationAttribute(const QualifiedName&) const override;
  void CollectStyleForPresentationAttribute(
      const QualifiedName&,
      const AtomicString&,
      HeapVector<CSSPropertyValue, 8>&) override;
  unsigned ParseBorderWidthAttribute(const AtomicString&) const;

  InsertionNotificationRequest InsertedInto(ContainerNode&) override;
  void RemovedFrom(ContainerNode& insertion_point) override;
  void DidMoveToNewDocument(Document& old_document) override;
  void FinishParsingChildren() override;

 private:
  String nodeName() const final;

  bool IsHTMLElement() const =
      delete;  // This will catch anyone doing an unnecessary check.
  bool IsStyledElement() const =
      delete;  // This will catch anyone doing an unnecessary check.

  void ApplyAspectRatioToStyle(double width,
                               double height,
                               HeapVector<CSSPropertyValue, 8>& style);

  DocumentFragment* TextToFragment(const String&, ExceptionState&);

  TranslateAttributeMode GetTranslateAttributeMode() const;

  void HandleKeypressEvent(KeyboardEvent&);

  void SetPopoverInvoker(Element* invoker);

  static void CloseEntirePopoverStack(
      HeapVector<Member<HTMLElement>>& stack,
      HidePopoverFocusBehavior focus_behavior,
      HidePopoverTransitionBehavior transition_behavior);

  static const AttributeTriggers* TriggersForAttributeName(
      const QualifiedName& attr_name);

  void OnDirAttrChanged(const AttributeModificationParams&);
  void OnFormAttrChanged(const AttributeModificationParams&);
  void OnLangAttrChanged(const AttributeModificationParams&);
  void OnNonceAttrChanged(const AttributeModificationParams&);
  void OnPopoverChanged(const AttributeModificationParams&);
  void OnContainerTimingAttrChanged(const AttributeModificationParams&);
  void OnRoleAttrChanged(const AttributeModificationParams&);

  int AdjustedOffsetForZoom(LayoutUnit);
  int OffsetTopOrLeft(bool top);
};

template <>
struct DowncastTraits<HTMLElement> {
  static bool AllowFrom(const Node& node) { return node.IsHTMLElement(); }
  static bool AllowFrom(const Element& element) {
    return element.IsHTMLElement();
  }
};

inline HTMLElement::HTMLElement(const QualifiedName& tag_name,
                                Document& document,
                                ConstructionType type = kCreateHTMLElement)
    : Element(tag_name, &document, type) {
  DCHECK(!tag_name.LocalName().IsNull());
}

inline bool Node::HasTagName(const HTMLQualifiedName& name) const {
  auto* html_element = DynamicTo<HTMLElement>(this);
  return html_element && html_element->HasTagName(name);
}

// Functor used to match HTMLElements with a specific HTML tag when using the
// ElementTraversal API.
class HasHTMLTagName {
  STACK_ALLOCATED();

 public:
  explicit HasHTMLTagName(const HTMLQualifiedName& tag_name)
      : tag_name_(tag_name) {}
  bool operator()(const HTMLElement& element) const {
    return element.HasTagName(tag_name_);
  }

 private:
  const HTMLQualifiedName& tag_name_;
};

}  // namespace blink

#include "third_party/blink/renderer/core/html_element_type_helpers.h"

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_ELEMENT_H_
