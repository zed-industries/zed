/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2000 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2004, 2005, 2006, 2007, 2008, 2009, 2010 Apple Inc. All rights
 * reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_HTML_FORM_CONTROL_ELEMENT_WITH_STATE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_HTML_FORM_CONTROL_ELEMENT_WITH_STATE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/forms/html_form_control_element.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class CORE_EXPORT HTMLFormControlElementWithState
    : public HTMLFormControlElement {
 public:
  ~HTMLFormControlElementWithState() override;

  bool CanContainRangeEndPoint() const final { return false; }

  virtual bool ShouldAutocomplete() const;
  // Implementations of 'autocomplete' IDL attribute.
  String IDLExposedAutofillValue() const;
  void setIDLExposedAutofillValue(const String& autocomplete_value);

  // ListedElement override:
  bool ClassSupportsStateRestore() const override;
  bool ShouldSaveAndRestoreFormControlState() const override;

  bool UserHasEditedTheField() const {
    return interacted_state_ >= InteractedState::kInteractedAndStillFocused;
  }
  void SetUserHasEditedTheField();
  void ClearUserHasEditedTheField() {
    interacted_state_ = InteractedState::kNotInteracted;
    force_user_valid_ = false;
  }
  bool UserHasEditedTheFieldAndBlurred() const {
    return interacted_state_ >= InteractedState::kInteractedAndBlurred;
  }
  void SetUserHasEditedTheFieldAndBlurred();

  void ForceUserValid();

  bool MatchesUserInvalidPseudo();
  bool MatchesUserValidPseudo();

  void DispatchInputEvent();
  void DispatchChangeEvent();
  void DispatchCancelEvent();

 protected:
  HTMLFormControlElementWithState(const QualifiedName& tag_name, Document&);

  void FinishParsingChildren() override;
  bool IsFormControlElementWithState() const final;

  void ResetImpl() override;

  // State machine that denotes whether the user made some manual modifications
  // to the field since page load or form reset. Used for autofill and for
  // :user-valid/:user-invalid. The order of this enum matters:
  // kInteractedAndBlurred implies kInteractedAndStillFocused.
  enum class InteractedState {
    // kNotInteracted means that the user has not interacted with this element
    // since page load or since the owner form was reset.
    kNotInteracted = 0,
    // kInteracted means that the user has interacted with this element, but has
    // not blurred the element yet. In this state, autofill considers the
    // element dirty but :user-valid and :user-invalid should not match yet.
    kInteractedAndStillFocused = 1,
    // kInteractedAndBlurred is intended to signal that this element should be
    // eligible to match :user-valid/:user-invalid, and corresponds to "user
    // interacted" in HTML:
    // https://html.spec.whatwg.org/multipage/form-control-infrastructure.html#user-interacted
    // This is set when the user edits an input and then blurs by focusing
    // something else, which may fire a change event, or when form submission
    // occurs.
    kInteractedAndBlurred = 2,
  };
  InteractedState interacted_state_ = InteractedState::kNotInteracted;

 private:
  int DefaultTabIndex() const override;

  // https://html.spec.whatwg.org/C/#autofill-anchor-mantle
  bool IsWearingAutofillAnchorMantle() const;

  // Flag that is set by form submission to force :user-valid/:user-invalid to
  // start applying regardless of the InteractedState.
  bool force_user_valid_ = false;
};

template <>
struct DowncastTraits<HTMLFormControlElementWithState> {
  static bool AllowFrom(const ListedElement& control) {
    return control.IsFormControlElementWithState();
  }
  static bool AllowFrom(const Element& element) {
    return element.IsFormControlElementWithState();
  }
  static bool AllowFrom(const Node& node) {
    const Element* element = DynamicTo<Element>(node);
    return element && element->IsFormControlElementWithState();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_HTML_FORM_CONTROL_ELEMENT_WITH_STATE_H_
