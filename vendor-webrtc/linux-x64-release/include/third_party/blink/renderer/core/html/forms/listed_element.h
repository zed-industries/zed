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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_LISTED_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_LISTED_ELEMENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/text/text_direction.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ContainerNode;
class Document;
class Element;
class FormAttributeTargetObserver;
class FormControlState;
class FormData;
class HTMLElement;
class HTMLFormElement;
class Node;
class ValidationMessageClient;
class ValidityState;

// https://html.spec.whatwg.org/C/#category-listed
class CORE_EXPORT ListedElement : public GarbageCollectedMixin {
 public:
  virtual ~ListedElement();
  // Returns a valid ListedElement pointer if the specified element is an
  // instance of a ListedElement subclass, or a form-associated custom element.
  // Returns nullptr otherwise.
  static ListedElement* From(Element& element);

  // Cast |this| to HTMLElement, or return the target element associated
  // to ElementInternals.
  const HTMLElement& ToHTMLElement() const;
  HTMLElement& ToHTMLElement();

  // Returns the associated form element or its host element if the form is
  // associated through reference target.
  HTMLElement* RetargetedForm() const;
  // Returns the associated form element.
  HTMLFormElement* Form() const { return form_.Get(); }
  ValidityState* validity();

  virtual bool IsFormControlElement() const;
  virtual bool IsFormControlElementWithState() const;
  virtual bool IsElementInternals() const;
  virtual bool IsObjectElement() const;
  virtual bool IsEnumeratable() const = 0;

  // Returns the 'name' attribute value. If this element has no name
  // attribute, it returns an empty string instead of null string.
  // Note that the 'name' IDL attribute doesn't use this function.
  virtual const AtomicString& GetName() const;

  // Override in derived classes to get the encoded name=value pair for
  // submitting.
  virtual void AppendToFormData(FormData&) {}

  void ResetFormOwner();

  void FormRemovedFromTree(const Node& form_root);

  bool WillValidate() const;

  // ValidityState attribute implementations
  virtual bool CustomError() const;

  // Functions for ValidityState interface methods.
  // Override functions for PatterMismatch, RangeOverflow, RangerUnderflow,
  // StepMismatch, TooLong, TooShort and ValueMissing must call WillValidate
  // method.
  virtual bool HasBadInput() const;
  virtual bool PatternMismatch() const;
  virtual bool RangeOverflow() const;
  virtual bool RangeUnderflow() const;
  virtual bool StepMismatch() const;
  virtual bool TooLong() const;
  virtual bool TooShort() const;
  virtual bool TypeMismatch() const;
  virtual bool ValueMissing() const;
  bool Valid() const;

  using List = HeapVector<Member<ListedElement>>;

  virtual String validationMessage() const;
  virtual String ValidationSubMessage() const;
  virtual void setCustomValidity(const String&);
  void UpdateVisibleValidationMessage();
  void HideVisibleValidationMessage();
  bool checkValidity(List* unhandled_invalid_controls = nullptr);
  bool reportValidity();
  // This must be called only after the caller check the element is focusable.
  void ShowValidationMessage();
  bool IsValidationMessageVisible() const;
  void FindCustomValidationMessageTextDirection(const String& message,
                                                TextDirection& message_dir,
                                                String& sub_message,
                                                TextDirection& sub_message_dir);
  virtual Element& ValidationAnchor() const;
  Element& GetHostOrFocusDelegate() const;
  bool ValidationAnchorOrHostIsFocusable() const;

  // For Element::IsValidElement(), which is for :valid :invalid selectors.
  bool IsValidElement();
  // Returns true if
  //  - this is not a candidate for constraint validation, or
  //    https://html.spec.whatwg.org/C/#candidate-for-constraint-validation
  //  - this satisfies its constraints
  //    https://html.spec.whatwg.org/C/#concept-fv-valid
  bool IsNotCandidateOrValid();

  // This must be called when a validation constraint or control value is
  // changed.
  void SetNeedsValidityCheck();

  // This should be called when |disabled| content attribute is changed.
  virtual void DisabledAttributeChanged();
  // This should be called when |readonly| content attribute is changed.
  void ReadonlyAttributeChanged();
  // Override this if you want to know 'disabled' state changes immediately.
  virtual void DisabledStateMightBeChanged() {}
  // This should be called when |form| content attribute is changed.
  void FormAttributeChanged();
  // This is for FormAttributeTargteObserver class.
  void FormAttributeTargetChanged();
  // This should be called in Node::InsertedInto().
  void InsertedInto(ContainerNode&);
  // This should be called in Node::RemovedFrom().
  void RemovedFrom(ContainerNode&);
  // This should be called in Node::DidMoveToDocument().
  void DidMoveToNewDocument(Document& old_document);
  // This is for HTMLFieldSetElement class.
  virtual void AncestorDisabledStateWasChanged();

  // https://html.spec.whatwg.org/C/#concept-element-disabled
  bool IsActuallyDisabled() const;

  // Returns a static value of class-level support of the state restore
  // feature.  If a sub-class of ListedElement supports the state restore
  // feature, this function should return true.
  virtual bool ClassSupportsStateRestore() const;
  // Returns a flag to represent support of the state restore feature per
  // instances.
  virtual bool ShouldSaveAndRestoreFormControlState() const;
  virtual FormControlState SaveFormControlState() const;
  // The specified FormControlState must have at least one string value.
  virtual void RestoreFormControlState(const FormControlState& state);
  // This should be called whenever an element supports state restore changes
  // its state.
  void NotifyFormStateChanged();
  // This should be called in Element::FinishParsingChildren() override.
  void TakeStateAndRestore();
  // Returns the form that owns this element according to Autofill's definition
  // of ownership, or nullptr if no form owns it. The form that owns this
  // element is:
  // - if this element is associated to a form, the furthest shadow-including
  //   form ancestor of that form,
  // - otherwise, the furthest shadow-including form ancestor of this element.
  // For the definition of ownership in Autofill, see
  // //components/autofill/content/renderer/README.md.
  HTMLFormElement* GetOwningFormForAutofill() const;

  void Trace(Visitor*) const override;

 protected:
  ListedElement();

  // FIXME: Remove usage of setForm. resetFormOwner should be enough, and
  // setForm is confusing.
  void SetForm(HTMLFormElement*);
  void AssociateByParser(HTMLFormElement*);

  // If you add an override of willChangeForm() or didChangeForm() to a class
  // derived from this one, you will need to add a call to setForm(0) to the
  // destructor of that class.
  virtual void WillChangeForm();
  virtual void DidChangeForm();

  enum class WillValidateReason {
    kDefault,
    kForInsertionOrRemoval,
  };

  // This must be called any time the result of WillValidate() has changed.
  void UpdateWillValidateCache(
      WillValidateReason = WillValidateReason::kDefault);
  virtual bool RecalcWillValidate() const;

  String CustomValidationMessage() const;
  // This is just a setter. This doesn't set |customError| flag.
  void SetCustomValidationMessage(const String& message);

  // False; There are no FIELDSET ancestors.
  // True; There might be a FIELDSET ancestor, and there might be no
  //       FIELDSET ancestors.
  mutable bool may_have_fieldset_ancestor_ = true;

  enum class AncestorDisabledState { kUnknown, kEnabled, kDisabled };
  mutable AncestorDisabledState ancestor_disabled_state_ =
      AncestorDisabledState::kUnknown;

  // exposed so that HTMLFieldSetElement can update the document's cache of
  // disabled fieldsets.  Should not be used more generally.
  bool IsSelfDisabledIgnoringAncestors() const { return is_element_disabled_; }

 private:
  void UpdateAncestorDisabledState() const;
  void SetFormAttributeTargetObserver(FormAttributeTargetObserver*);
  void ResetFormAttributeTargetObserver();
  // Requests validity recalc for the form owner, if one exists.
  void FormOwnerSetNeedsValidityCheck();
  // Requests validity recalc for all ancestor fieldsets, if exist.
  enum class StartingNodeType { IS_PARENT, IS_INSERTION_POINT };
  void FieldSetAncestorsSetNeedsValidityCheck(Node*, StartingNodeType);

  ValidationMessageClient* GetValidationMessageClient() const;

  Member<FormAttributeTargetObserver> form_attribute_target_observer_;
  Member<HTMLFormElement> form_;
  Member<ValidityState> validity_state_;
  String custom_validation_message_;
  bool has_validation_message_ : 1;
  // If form_was_set_by_parser_ is true, form_ is always non-null.
  bool form_was_set_by_parser_ : 1;

  // The initial value of will_validate_ depends on the derived class. We can't
  // initialize it with a virtual function in the constructor. will_validate_
  // is not deterministic as long as will_validate_initialized_ is false.
  mutable bool will_validate_initialized_ : 1;
  mutable bool will_validate_ : 1;

  // Cache of IsValidElement().
  bool is_valid_ : 1;
  bool validity_is_dirty_ : 1;
  bool is_element_disabled_ : 1;
  bool is_readonly_ : 1;

  enum class DataListAncestorState {
    kUnknown,
    kInsideDataList,
    kNotInsideDataList
  };
  mutable enum DataListAncestorState data_list_ancestor_state_ =
      DataListAncestorState::kUnknown;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_LISTED_ELEMENT_H_
