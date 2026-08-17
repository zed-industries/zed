// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_SELECT_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_SELECT_TYPE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/forms/html_select_element.h"
#include "third_party/blink/renderer/core/html/forms/popup_menu.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class AXObject;

// SelectType class is an abstraction of the MenuList behavior and the ListBox
// behavior of HTMLSelectElement.
class SelectType : public GarbageCollected<SelectType> {
 public:
  // Creates an instance of a SelectType subclass depending on the current mode
  // of |select|.
  static SelectType* Create(HTMLSelectElement& select);
  void WillBeDestroyed();
  virtual void Trace(Visitor* visitor) const;

  // Returns true if the event is handled.
  virtual bool DefaultEventHandler(const Event& event) = 0;

  virtual void DidSelectOption(HTMLOptionElement* element,
                               HTMLSelectElement::SelectOptionFlags flags,
                               bool should_update_popup) = 0;

  virtual void OptionRemoved(HTMLOptionElement& option);
  virtual void DidBlur() = 0;
  virtual void DidDetachLayoutTree();
  virtual void DidRecalcStyle(const StyleRecalcChange change);
  virtual void DidSetSuggestedOption(HTMLOptionElement* option) = 0;
  virtual void SaveLastSelection() = 0;

  // Update style of text in the CSS box on style or selected OPTION change.
  virtual void UpdateTextStyle();

  // Update style of text in the CSS box on style or selected OPTION change,
  // and update the text.
  virtual void UpdateTextStyleAndContent();

  virtual HTMLOptionElement* OptionToBeShown() const;
  virtual const ComputedStyle* OptionStyle() const;
  virtual void MaximumOptionWidthMightBeChanged() const;

  virtual HTMLOptionElement* SpatialNavigationFocusedOption();
  virtual HTMLOptionElement* ActiveSelectionEnd() const;
  virtual void ScrollToSelection();
  virtual void ScrollToOption(HTMLOptionElement* option);
  virtual void SelectAll();
  virtual void SaveListboxActiveSelection();
  virtual void HandleMouseRelease();
  virtual void ListBoxOnChange();
  // Clear OPTION selection information saved by SaveLastSelection().
  // This is for ListBoxes.
  virtual void ClearLastOnChangeSelection();

  virtual void CreateShadowSubtree(ShadowRoot& root) = 0;
  virtual void ManuallyAssignSlots() = 0;
  virtual HTMLButtonElement* SlottedButton() const = 0;
  virtual HTMLElement* PopoverForAppearanceBase() const = 0;
  virtual bool IsAppearanceBaseButton() const = 0;
  virtual bool IsAppearanceBasePicker() const = 0;
  virtual void SetIsAppearanceBasePickerForDisplayNone(bool) = 0;
  virtual HTMLSelectElement::SelectAutofillPreviewElement*
  GetAutofillPreviewElement() const = 0;
  virtual Element& InnerElement() const;
  virtual void ShowPopup(PopupMenu::ShowEventType type);
  virtual void HidePopup(SelectPopupHideBehavior);
  virtual void PopupDidHide();
  virtual bool PopupIsVisible() const;
  virtual PopupMenu* PopupForTesting() const;
  virtual AXObject* PopupRootAXObject() const;
  virtual void ShowPicker();

  enum SkipDirection { kSkipBackwards = -1, kSkipForwards = 1 };
  CORE_EXPORT HTMLOptionElement* NextSelectableOption(HTMLOptionElement*) const;
  CORE_EXPORT HTMLOptionElement* PreviousSelectableOption(
      HTMLOptionElement*) const;
  CORE_EXPORT HTMLOptionElement* FirstSelectableOption() const;
  CORE_EXPORT HTMLOptionElement* LastSelectableOption() const;

 protected:
  explicit SelectType(HTMLSelectElement& select);
  HTMLOptionElement* NextValidOption(int list_index,
                                     SkipDirection direction,
                                     int skip) const;

  const Member<HTMLSelectElement> select_;
  bool will_be_destroyed_ = false;

 private:
  SelectType(const SelectType&) = delete;
  SelectType& operator=(const SelectType&) = delete;
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_SELECT_TYPE_H_
