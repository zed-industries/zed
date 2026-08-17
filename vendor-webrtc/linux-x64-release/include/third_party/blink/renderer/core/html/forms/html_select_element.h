/*
 * Copyright (C) 2010 Nokia Corporation and/or its subsidiary(-ies).
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2000 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2004, 2005, 2006, 2007, 2009, 2010, 2011 Apple Inc. All rights
 * reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_HTML_SELECT_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_HTML_SELECT_ELEMENT_H_

#include "base/gtest_prod_util.h"
#include "third_party/blink/public/mojom/input/focus_type.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/simulated_click_options.h"
#include "third_party/blink/renderer/core/dom/tree_ordered_list.h"
#include "third_party/blink/renderer/core/html/forms/html_form_control_element_with_state.h"
#include "third_party/blink/renderer/core/html/forms/html_selected_content_element.h"
#include "third_party/blink/renderer/core/html/forms/option_list.h"
#include "third_party/blink/renderer/core/html/forms/type_ahead.h"
#include "third_party/blink/renderer/core/html/html_div_element.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class AXObject;
class AutoscrollController;
class ExceptionState;
class HTMLHRElement;
class HTMLOptGroupElement;
class HTMLOptionElement;
class HTMLOptionsCollection;
class PopupMenu;
class SelectType;
class V8UnionHTMLElementOrLong;
class V8UnionHTMLOptGroupElementOrHTMLOptionElement;
class HTMLSelectedContentElement;
class SelectDescendantsObserver;

enum class SelectPopupHideBehavior {
  kNormal,
  kNoEventsOrFocusing,
};

class CORE_EXPORT HTMLSelectElement final
    : public HTMLFormControlElementWithState,
      private TypeAheadDataSource {
  DEFINE_WRAPPERTYPEINFO();

 public:
  class SelectAutofillPreviewElement : public HTMLDivElement {
   public:
    SelectAutofillPreviewElement(Document& document, HTMLSelectElement* select);

    const ComputedStyle* CustomStyleForLayoutObject(
        const StyleRecalcContext& style_recalc_context) override;
    Node::InsertionNotificationRequest InsertedInto(ContainerNode&) override;
    void RemovedFrom(ContainerNode&) override;

    void Trace(Visitor*) const override;

   private:
    Member<HTMLSelectElement> select_;
  };

  explicit HTMLSelectElement(Document&);
  ~HTMLSelectElement() override;

  int selectedIndex() const;
  void setSelectedIndex(int);
  // `listIndex' version of |selectedIndex|.
  int SelectedListIndex() const;

  // For ValidityState
  String validationMessage() const override;
  bool ValueMissing() const override;

  String DefaultToolTip() const override;
  void ResetImpl() override;

  unsigned length() const;
  void setLength(unsigned, ExceptionState&);

  // TODO(tkent): Rename |size| to |Size|. This is not an implementation of
  // |size| IDL attribute.
  unsigned size() const { return size_; }
  // The number of items to be shown in the LisBox mode.
  // Do not call this in the MenuList mode.
  unsigned ListBoxSize() const;
  bool IsMultiple() const { return is_multiple_; }

  void showPicker(ExceptionState&);

  bool UsesMenuList() const { return uses_menu_list_; }

  void add(const V8UnionHTMLOptGroupElementOrHTMLOptionElement* element,
           const V8UnionHTMLElementOrLong* before,
           ExceptionState& exception_state);

  using Node::remove;
  void remove(int index);

  String Value() const;
  void SetValue(const String&,
                bool send_events = false,
                WebAutofillState = WebAutofillState::kNotFilled);
  String valueForBinding() const { return Value(); }
  void setValueForBinding(const String&);

  // It is possible to pass WebAutofillState::kNotFilled here in case we need
  // to simulate a reset of a <select> element.
  void SetAutofillValue(const String& value, WebAutofillState);

  String SuggestedValue() const;
  // Sets the suggested value and puts the element into
  // WebAutofillState::kPreviewed state if the value exists, or
  // WebAutofillState::kNotFilled otherwise.
  void SetSuggestedValue(const String&);

  // |options| and |selectedOptions| are not safe to be used in in
  // HTMLOptionElement::removedFrom() and insertedInto() because their cache
  // is inconsistent in these functions.
  HTMLOptionsCollection* options();
  HTMLCollection* selectedOptions();

  // Returns the first selected OPTION, or nullptr.
  HTMLOptionElement* SelectedOption() const;

  // Returns true if any of the <select>'s descendants are disallowed
  // interactive elements.
  bool IsInDialogMode() const;
  void IncreaseContentModelViolationCount();
  void DecreaseContentModelViolationCount();

  // This is similar to |options| HTMLCollection.  But this is safe in
  // HTMLOptionElement::removedFrom() and insertedInto().
  // OptionList supports only forward iteration.
  OptionList GetOptionList() const { return OptionList(*this); }

  void OptionElementChildrenChanged(const HTMLOptionElement&);

  void InvalidateSelectedItems();

  using ListItems = HeapVector<Member<HTMLElement>>;
  // We prefer |optionList()| to |listItems()|.
  const ListItems& GetListItems() const;

  // NearestAncestorSelectNoNesting is called with <hr>, <option>, and
  // <optgroup> elements to determine if they have an ancestor <select> which
  // they are associated with. An ancestor <select> will not be returned in some
  // cases, such as nested <option>s, in order to match the logic in
  // RecalcListItems and OptionList.
  // `insertion_point` and `passed_insertion_point` are optional parameters used
  // by HTMLOptionElement::InsertedInto. If `insertion_point` is encountered
  // during the ancestor traversal, then `passed_insertion_point` will be set to
  // true.
  static HTMLSelectElement* NearestAncestorSelectNoNesting(
      const Element& element,
      ContainerNode* insertion_point = nullptr,
      bool* passed_insertion_point = nullptr);

  void AccessKeyAction(SimulatedClickCreationScope creation_scope) override;
  void SelectOptionByAccessKey(HTMLOptionElement*);

  void SetOption(unsigned index, HTMLOptionElement*, ExceptionState&);

  HTMLOptionElement* namedItem(const AtomicString& name);
  HTMLOptionElement* item(unsigned index);

  bool CanSelectAll() const;
  void SelectAll();
  int ActiveSelectionEndListIndex() const;
  HTMLOptionElement* ActiveSelectionEnd() const;

  // For use in the implementation of HTMLOptionElement.
  void OptionSelectionStateChanged(HTMLOptionElement*, bool option_is_selected);
  void ElementInserted(Node& node);
  void OptionInserted(HTMLOptionElement&, bool option_is_selected);
  void OptionRemoved(HTMLOptionElement&);
  IndexedPropertySetterResult AnonymousIndexedSetter(unsigned,
                                                     HTMLOptionElement*,
                                                     ExceptionState&);

  void OptGroupInsertedOrRemoved(HTMLOptGroupElement&);
  void HrInsertedOrRemoved(HTMLHRElement&);

  HTMLOptionElement* SpatialNavigationFocusedOption();

  int ListIndexForOption(const HTMLOptionElement&);

  // Helper functions for popup menu implementations.
  String ItemText(const Element&) const;
  bool ItemIsDisplayNone(Element&, bool ensure_style) const;
  // ItemComputedStyle() may return nullptr if the element is not rendered.
  const ComputedStyle* ItemComputedStyle(Element&) const;
  // Text starting offset in LTR.
  LayoutUnit ClientPaddingLeft() const;
  // Text starting offset in RTL.
  LayoutUnit ClientPaddingRight() const;
  void SelectOptionByPopup(int list_index);
  void SelectOptionByPopup(HTMLOptionElement* option);
  void SelectMultipleOptionsByPopup(const Vector<int>& list_indices);
  // A popup is canceled when the popup was hidden without selecting an item.
  void PopupDidCancel();
  // Provisional selection is a selection made using arrow keys or type ahead.
  void ProvisionalSelectionChanged(unsigned);
  void PopupDidHide();
  bool PopupIsVisible() const;
  // Returns the active option. Only available in menulist mode.
  HTMLOptionElement* OptionToBeShown() const;
  // Style of the selected OPTION. This is nullable, and only for
  // the menulist mode.
  const ComputedStyle* OptionStyle() const;
  void ShowPopup();
  void HidePopup(SelectPopupHideBehavior);
  PopupMenu* PopupForTesting() const;

  void ResetTypeAheadSessionForTesting();

  bool HasNonInBodyInsertionMode() const override { return true; }

  void Trace(Visitor*) const override;
  void CloneNonAttributePropertiesFrom(const Element&,
                                       NodeCloningData&) override;

  // These are all utilities that check the relevant runtime flag, *plus* check
  // that the SelectParserRelaxationOptOut origin trial is not enabled.
  static bool SelectParserRelaxationEnabled(const Document* document);
  static bool SelectParserRelaxationEnabled(const Node* node);
  static bool CustomizableSelectEnabled(const Document* document);
  static bool CustomizableSelectEnabled(const Node* node);
  static bool CustomizableSelectEnabledNoDocument();

  // InnerElement and PopupRootAXObject should be called only if UsesMenuList().
  // InnerElement is the in-page <div> element in the UA shadowroot for MenuList
  // rendering. It is excluded from the layout tree if the author sets
  // appearance:base-select on this <select> and provides their own child
  // <button>.
  Element& InnerElement() const;
  AXObject* PopupRootAXObject() const;

  bool IsRichlyEditableForAccessibility() const override { return false; }

  bool IsValidBuiltinCommand(HTMLElement& invoker,
                             CommandEventType command) override;
  bool HandleCommandInternal(HTMLElement& invoker,
                             CommandEventType command) override;

  // SlottedButton returns the first child <button> in the light dom tree. If
  // this select is in a state where the <button> can't be rendered, such as a
  // <select multiple>, then nullptr will be returned. Since this method is
  // called during style calculation to compute internal pseudo-classes, the
  // value of the appearance property is not checked.
  HTMLButtonElement* SlottedButton() const;

  // Returns true if the provided node is some select element's SlottedButton.
  static bool IsSlottedButton(const Node*);

  // This method returns the UA popover element which is used for
  // appearance:base-select. If this select is rendering in a mode which doesn't
  // use the UA popover, such as appearance:auto/none or size=2/multiple, then
  // this will return null.
  HTMLElement* PopoverForAppearanceBase() const;

  // Returns true if the provided element is some select element's
  // PopoverForAppearanceBase.
  static bool IsPopoverForAppearanceBase(const Node*);
  static bool IsPopoverForAppearanceBase(const Element*);

  // <select> supports appearance:base-select on both the main element and
  // ::picker(select). When the main element has appearance:base-select,
  // IsAppearanceBaseButton will return true and the in-page button part of the
  // <select> will have base appearance and support rendering of the
  // author-provided <button>. When both the element and its ::picker(select)
  // has appearance:base-select, IsAppearanceBasePicker will return true and the
  // popup will be a popover element. The SelectType must also support base
  // appearance, which is currently only MenuListSelectType.
  // IsAppearanceBaseButton should be used for code which is concerned with the
  // in-page rendering of the button, and IsAppearanceBasePicker should be used
  // for code which is concerned with the popup/popover and the other elements
  // which are rendered in it.
  // SetIsAppearanceBasePickerForDisplayNone is called during style recalc for
  // the case where the picker is closed and is therefore display:none and
  // doesn't have a computed style to look at inside IsAppearanceBasePicker.
  bool IsAppearanceBaseButton() const;
  bool IsAppearanceBasePicker() const;
  void SetIsAppearanceBasePickerForDisplayNone(bool);

  void SelectedContentElementInserted(
      HTMLSelectedContentElement* selectedcontent);
  void SelectedContentElementRemoved(
      HTMLSelectedContentElement* selectedcontent);

  // This will only return an element if IsAppearanceBaseButton(). The element
  // is a popover inside the UA shadowroot which is used to show the user a
  // preview of what is going to be autofilled.
  SelectAutofillPreviewElement* GetAutofillPreviewElement() const;

  // Getter and setter for the selectedcontentelement attribute
  HTMLSelectedContentElement* selectedContentElement() const;
  void setSelectedContentElement(HTMLSelectedContentElement*);

  void DefaultEventHandler(Event&) override;

  void UpdateAllSelectedcontents(HTMLOptionElement* selected_option);

 private:
  mojom::blink::FormControlType FormControlType() const override;
  const AtomicString& FormControlTypeAsString() const override;

  bool MayTriggerVirtualKeyboard() const override;

  bool ShouldHaveFocusAppearance() const final;

  bool DispatchFocusEvent(
      Element* old_focused_element,
      mojom::blink::FocusType,
      InputDeviceCapabilities* source_capabilities) override;
  void DispatchBlurEvent(Element* new_focused_element,
                         mojom::blink::FocusType,
                         InputDeviceCapabilities* source_capabilities) override;

  bool CanStartSelection() const override { return false; }

  bool IsEnumeratable() const override { return true; }
  bool IsInteractiveContent() const override;
  bool IsLabelable() const override { return true; }

  FormControlState SaveFormControlState() const override;
  void RestoreFormControlState(const FormControlState&) override;

  void ChildrenChanged(const ChildrenChange& change) override;
  bool ChildrenChangedAllChildrenRemovedNeedsList() const override;
  void ParseAttribute(const AttributeModificationParams&) override;
  bool IsPresentationAttribute(const QualifiedName&) const override;

  LayoutObject* CreateLayoutObject(const ComputedStyle&) override;
  void DidRecalcStyle(const StyleRecalcChange) override;
  void AttachLayoutTree(AttachContext&) override;
  void DetachLayoutTree(bool performing_reattach) override;
  void AppendToFormData(FormData&) override;
  void DidAddUserAgentShadowRoot(ShadowRoot&) override;
  void ManuallyAssignSlots() override;

  void SetRecalcListItems();
  void RecalcListItems() const;
  enum ResetReason { kResetReasonSelectedOptionRemoved, kResetReasonOthers };
  void ResetToDefaultSelection(ResetReason = kResetReasonOthers);
  void TypeAheadFind(const KeyboardEvent&);

  bool IsOptionalFormControl() const override {
    return !IsRequiredFormControl();
  }
  bool IsRequiredFormControl() const override;

  bool HasPlaceholderLabelOption() const;

  enum SelectOptionFlag {
    kDeselectOtherOptionsFlag = 1 << 0,
    kDispatchInputAndChangeEventFlag = 1 << 1,
    kMakeOptionDirtyFlag = 1 << 2,
  };
  typedef unsigned SelectOptionFlags;
  void SelectOption(HTMLOptionElement*,
                    SelectOptionFlags,
                    WebAutofillState = WebAutofillState::kNotFilled);
  bool DeselectItemsWithoutValidation(
      HTMLOptionElement* element_to_exclude = nullptr);
  void ParseMultipleAttribute(const AtomicString&);
  HTMLOptionElement* LastSelectedOption() const;
  wtf_size_t SearchOptionsForValue(const String&,
                                   wtf_size_t list_index_start,
                                   wtf_size_t list_index_end) const;
  void SetIndexToSelectOnCancel(int list_index);
  void SetSuggestedOption(HTMLOptionElement*);

  // Returns nullptr if listIndex is out of bounds, or it doesn't point an
  // HTMLOptionElement.
  HTMLOptionElement* OptionAtListIndex(int list_index) const;

  AutoscrollController* GetAutoscrollController() const;
  LayoutBox* AutoscrollBox() override;
  void StopAutoscroll() override;

  bool AreAuthorShadowsAllowed() const override { return false; }
  void FinishParsingChildren() override;

  // TypeAheadDataSource functions.
  int IndexOfSelectedOption() const override;
  int OptionCount() const override;
  String OptionAtIndex(int index) const override;

  void UpdateUsesMenuList();
  // Apply changes to rendering as a result of attribute changes (multiple,
  // size).
  void ChangeRendering();
  void UpdateUserAgentShadowTree(ShadowRoot& root);

  // Helper to update the select descendants' mutation observer.
  void UpdateMutationObserver();

  // list_items_ contains HTMLOptionElement, HTMLOptGroupElement, and
  // HTMLHRElement objects.
  mutable ListItems list_items_;
  TypeAhead type_ahead_;
  unsigned size_;
  Member<HTMLOptionElement> last_on_change_option_;
  Member<HTMLOptionElement> suggested_option_;
  TreeOrderedList<HTMLSelectedContentElement> descendant_selectedcontents_;
  bool uses_menu_list_ = true;
  bool is_multiple_;
  mutable bool should_recalc_list_items_;

  Member<SelectType> select_type_;
  int index_to_select_on_cancel_;

  Member<SelectDescendantsObserver> descendants_observer_;
  unsigned content_model_violations_count_ = 0U;

  friend class ListBoxSelectType;
  friend class MenuListSelectType;
  friend class SelectType;
  friend class HTMLSelectElementTest;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_HTML_SELECT_ELEMENT_H_
