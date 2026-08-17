/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2000 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2004, 2005, 2006, 2010, 2011 Apple Inc. All rights reserved.
 * Copyright (C) 2010 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_HTML_OPTION_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_HTML_OPTION_ELEMENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/simulated_click_options.h"
#include "third_party/blink/renderer/core/html/html_element.h"

namespace blink {

class ExceptionState;
class HTMLDataListElement;
class HTMLSelectElement;
class OptionTextObserver;

class CORE_EXPORT HTMLOptionElement final : public HTMLElement {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static HTMLOptionElement* CreateForJSConstructor(
      Document& document,
      const String& data,
      ExceptionState& exception_state) {
    return CreateForJSConstructor(document, data, AtomicString(), false, false,
                                  exception_state);
  }
  static HTMLOptionElement* CreateForJSConstructor(Document&,
                                                   const String& data,
                                                   const AtomicString& value,
                                                   bool default_selected,
                                                   bool selected,
                                                   ExceptionState&);

  explicit HTMLOptionElement(Document&);
  ~HTMLOptionElement() override;
  void Trace(Visitor* visitor) const override;

  // A text to be shown to users.  The difference from |label()| is |label()|
  // returns an empty string if |label| content attribute is empty.
  // |displayLabel()| returns the value string in that case.
  String DisplayLabel() const;

  // |text| IDL attribute implementations.
  String text() const;
  void setText(const String&);

  int index() const;

  String value() const;
  void setValue(const AtomicString&);

  bool Selected() const;
  void SetSelected(bool);
  bool selectedForBinding() const;
  void setSelectedForBinding(bool);

  HTMLDataListElement* OwnerDataListElement() const;

  // OwnerSelectElement gets nearest_ancestor_select_ and SetOwnerSelectElement
  // assigns to it. See comment on nearest_ancestor_select_.
  HTMLSelectElement* OwnerSelectElement(bool skip_check = false) const;
  void SetOwnerSelectElement(HTMLSelectElement*);

  String label() const;
  void setLabel(const AtomicString&);

  bool OwnElementDisabled() const;

  bool IsDisabledFormControl() const override;
  String DefaultToolTip() const override;
  void DefaultEventHandler(Event&) override;

  String TextIndentedToRespectGroupLabel() const;

  // Update 'selectedness'.
  void SetSelectedState(bool);
  // Update 'dirtiness'.
  void SetDirty(bool);

  HTMLElement* formForBinding() const override;
  bool SpatialNavigationFocused() const;

  bool IsDisplayNone(bool ensure_style);

  int ListIndex() const;

  void SetMultiSelectFocusedState(bool);
  bool IsMultiSelectFocused() const;

  void SetWasOptionInsertedCalled(bool flag) {
    was_option_inserted_called_ = flag;
  }
  bool WasOptionInsertedCalled() const { return was_option_inserted_called_; }

  Node::InsertionNotificationRequest InsertedInto(ContainerNode&) override;
  void RemovedFrom(ContainerNode&) override;

  void FinishParsingChildren() override;

  // Callback for OptionTextObserver.
  void DidChangeTextContent();

  bool IsRichlyEditableForAccessibility() const override { return false; }

  // This method returns true if the provided element is the label_container_ of
  // an HTMLOptionElement.
  static bool IsLabelContainerElement(const Element& element);

 private:
  FocusableState SupportsFocus(UpdateBehavior update_behavior) const override;
  bool MatchesDefaultPseudoClass() const override;
  bool MatchesEnabledPseudoClass() const override;
  void ParseAttribute(const AttributeModificationParams&) override;
  void AccessKeyAction(SimulatedClickCreationScope) override;
  void ChildrenChanged(const ChildrenChange&) override;

  void DidAddUserAgentShadowRoot(ShadowRoot&) override;

  String CollectOptionInnerText() const;

  void UpdateLabel();

  void DefaultEventHandlerInternal(Event&);

  void RecalcOwnerSelectElement() const;

  Member<OptionTextObserver> text_observer_;

  // The closest ancestor <select> in the DOM tree, without crossing any shadow
  // boundaries. This is cached as a performance optimization for
  // OwnerSelectElement(), and is kept up to date in InsertedInto() and
  // RemovedFrom(). Only set when SelectParserRelaxation is enabled.
  // TODO(crbug.com/1511354): Consider using a flat tree traversal here
  // instead of a node traversal. That would probably also require changing
  // HTMLOptionsCollection to support flat tree traversals as well.
  Member<HTMLSelectElement> nearest_ancestor_select_;

  // label_container_ contains the text content of DisplayLabel(). Based on UA
  // style rules, it is rendered when this option is not inside of a select
  // element with appearance:base-select.
  Member<HTMLElement> label_container_;

  // Represents 'selectedness'.
  // https://html.spec.whatwg.org/C/#concept-option-selectedness
  bool is_selected_;
  // Represents 'dirtiness'.
  // https://html.spec.whatwg.org/C/#concept-option-dirtiness
  bool is_dirty_ = false;
  // Represents the option being focused on in a multi-select non-contiguous
  // traversal via the keyboard.
  bool is_multi_select_focused_ = false;

  // True while HTMLSelectElement::OptionInserted(this) and OptionRemoved(this);
  // This flag is necessary to detect a state where DOM tree is updated and
  // OptionInserted() is not called yet.
  bool was_option_inserted_called_ = false;

  friend class HTMLOptionElementTest;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_HTML_OPTION_ELEMENT_H_
