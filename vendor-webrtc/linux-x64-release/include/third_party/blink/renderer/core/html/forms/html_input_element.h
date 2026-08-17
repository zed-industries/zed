/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2000 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2004, 2005, 2006, 2007, 2010 Apple Inc. All rights reserved.
 * Copyright (C) 2012 Samsung Electronics. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_HTML_INPUT_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_HTML_INPUT_ELEMENT_H_

#include "base/gtest_prod_util.h"
#include "build/build_config.h"
#include "third_party/blink/public/mojom/input/focus_type.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/create_element_flags.h"
#include "third_party/blink/renderer/core/dom/events/simulated_click_options.h"
#include "third_party/blink/renderer/core/html/forms/step_range.h"
#include "third_party/blink/renderer/core/html/forms/text_control_element.h"
#include "third_party/blink/renderer/platform/bindings/script_regexp.h"
#include "third_party/blink/renderer/platform/theme_types.h"

namespace blink {

class AXObject;
class ComputedStyleBuilder;
class DragData;
class ExceptionState;
class FileList;
class HTMLDataListElement;
class HTMLImageLoader;
class InputType;
class InputTypeView;
class KURL;
class ListAttributeTargetObserver;
class RadioButtonGroupScope;
class ScriptObject;
struct DateTimeChooserParameters;

class CORE_EXPORT HTMLInputElement
    : public TextControlElement,
      public LazyActiveScriptWrappable<HTMLInputElement> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit HTMLInputElement(Document&,
                            const CreateElementFlags = CreateElementFlags());
  ~HTMLInputElement() override;
  void Trace(Visitor*) const override;

  bool HasPendingActivity() const final;

  DEFINE_ATTRIBUTE_EVENT_LISTENER(webkitspeechchange, kWebkitspeechchange)

  bool ShouldAutocomplete() const final;

  // For ValidityState
  bool HasBadInput() const final;
  bool PatternMismatch() const final;
  bool RangeUnderflow() const final;
  bool RangeOverflow() const final;
  bool StepMismatch() const final;
  bool TooLong() const final;
  bool TooShort() const final;
  bool TypeMismatch() const final;
  bool ValueMissing() const final;
  String validationMessage() const final;
  String ValidationSubMessage() const final;

  // Returns the minimum value for type=date, number, or range.  Don't call this
  // for other types.
  double Minimum() const;
  // Returns the maximum value for type=date, number, or range.  Don't call this
  // for other types.  This always returns a value which is >= minimum().
  double Maximum() const;
  // Sets the "allowed value step" defined in the HTML spec to the specified
  // double pointer.  Returns false if there is no "allowed value step."
  bool GetAllowedValueStep(Decimal*) const;
  StepRange CreateStepRange(AnyStepHandling) const;

  Decimal FindClosestTickMarkValue(const Decimal&);

  // Implementations of HTMLInputElement::stepUp() and stepDown().
  void stepUp(int, ExceptionState&);
  void stepDown(int, ExceptionState&);
  // stepUp()/stepDown() for user-interaction.
  bool IsSteppable() const;

  // Returns true if the type is button, reset, submit, or image.
  bool IsButton() const;
  // Returns true if the type is button, reset, or submit.
  bool IsTextButton() const;
  // Returns true if the type is email, number, password, search, tel, text,
  // or url.
  bool IsTextField() const;
  bool IsTelephone() const;
  // To override from TextControlElement
  bool IsAutoDirectionalityFormAssociated() const final;
  // Do not add type check predicates for concrete input types; e.g.  isImage,
  // isRadio, isFile.  If you want to check the input type, you may use
  // |input->FormControlType() == FormControlType::kInputImage|, etc.

  // Returns whether this field is or has ever been a password field, or if
  // autofill classified the field as password by predictions, so that its value
  // can be protected from memorization by autofill or keyboards.
  bool HasBeenPasswordField() const;
  void MaybeSetHasBeenPasswordField();

  bool IsCheckable() const;
  bool checkedForBinding() const { return Checked(); }
  void setCheckedForBinding(bool);
  bool Checked() const;
  void SetChecked(
      bool,
      TextFieldEventBehavior = TextFieldEventBehavior::kDispatchNoEvent,
      WebAutofillState = WebAutofillState::kNotFilled);
  void DispatchChangeEventIfNeeded();
  void DispatchInputAndChangeEventIfNeeded();

  // 'indeterminate' is a state independent of the checked state that causes the
  // control to draw in a way that hides the actual state.
  bool indeterminate() const { return is_indeterminate_; }
  void setIndeterminate(bool);
  // shouldAppearChecked is used by the layout tree/CSS while checked() is used
  // by JS to determine checked state
  bool ShouldAppearChecked() const;
  bool ShouldAppearIndeterminate() const override;

  PopoverTriggerSupport SupportsPopoverTriggering() const override;

  // Returns null if this isn't associated with any radio button group.
  RadioButtonGroupScope* GetRadioButtonGroupScope() const;

  unsigned size() const;
  bool SizeShouldIncludeDecoration(int& preferred_size) const;

  void setType(const AtomicString&);

  String Value() const override;
  void SetValue(
      const String&,
      TextFieldEventBehavior = TextFieldEventBehavior::kDispatchNoEvent,
      TextControlSetValueSelection =
          TextControlSetValueSelection::kSetSelectionToEnd,
      WebAutofillState = WebAutofillState::kNotFilled) override;
  String valueForBinding() const { return Value(); }
  void setValueForBinding(const String&, ExceptionState&);
  void SetValueForUser(const String&);
  // Update the value, and clear hasDirtyValue() flag.
  void SetNonDirtyValue(const String&);
  // Checks if the specified string would be a valid value.
  // We should not call this for types with no string value such as CHECKBOX and
  // RADIO.
  bool IsValidValue(const String&) const;
  bool HasDirtyValue() const;

  String SanitizeValue(const String&) const;

  String LocalizeValue(const String&) const;

  // Sets the suggested value and puts the element into
  // WebAutofillState::kPreviewed state if |value| is non-empty, or
  // WebAutofillState::kNotFilled otherwise.
  // A null value indicates that the suggested value should be hidden.
  void SetSuggestedValue(const String& value) override;

  ScriptObject valueAsDate(ScriptState* script_state) const;
  void setValueAsDate(ScriptState* script_state,
                      const ScriptObject& value,
                      ExceptionState& exception_state);

  double valueAsNumber() const;
  void setValueAsNumber(
      double,
      ExceptionState&,
      TextFieldEventBehavior = TextFieldEventBehavior::kDispatchNoEvent);

  // For type=range, returns a ratio of the current value in the range between
  // min and max.  i.e. (value - min) / (max - min)
  // For other types, this function fails with DCHECK().
  Decimal RatioValue() const;

  String ValueOrDefaultLabel() const;

  // This function dispatches 'input' event for non-textfield types. Callers
  // need to handle any DOM structure changes by event handlers, or need to
  // delay the 'input' event with EventQueueScope.
  void SetValueFromRenderer(const String&);

  std::optional<uint32_t> selectionStartForBinding(ExceptionState&) const;
  std::optional<uint32_t> selectionEndForBinding(ExceptionState&) const;
  String selectionDirectionForBinding(ExceptionState&) const;
  void setSelectionStartForBinding(std::optional<uint32_t>, ExceptionState&);
  void setSelectionEndForBinding(std::optional<uint32_t>, ExceptionState&);
  void setSelectionDirectionForBinding(const String&, ExceptionState&);
  void setSelectionRangeForBinding(unsigned start,
                                   unsigned end,
                                   ExceptionState&);
  void setSelectionRangeForBinding(unsigned start,
                                   unsigned end,
                                   const String& direction,
                                   ExceptionState&);
  // This function can be used to allow tests to set the selection
  // range for Number inputs, which do not support the ordinary
  // selection API.
  void SetSelectionRangeForTesting(unsigned start,
                                   unsigned end,
                                   ExceptionState&);

  bool LayoutObjectIsNeeded(const DisplayStyle&) const final;
  LayoutObject* CreateLayoutObject(const ComputedStyle&) override;
  void DetachLayoutTree(bool performing_reattach) final;
  void UpdateSelectionOnFocus(SelectionBehaviorOnFocus,
                              const FocusOptions*) final;

  // FIXME: For isActivatedSubmit and setActivatedSubmit, we should use the
  // NVI-idiom here by making it private virtual in all classes and expose a
  // public method in HTMLFormControlElement to call
  // the private virtual method.
  bool IsActivatedSubmit() const final;
  void SetActivatedSubmit(bool flag) final;

  String AltText() const final;

  const AtomicString& DefaultValue() const;

  Vector<String> AcceptMIMETypes() const;
  Vector<String> AcceptFileExtensions() const;
  const AtomicString& Alt() const;

  void setSize(unsigned, ExceptionState&);

  KURL Src() const;
  bool Multiple() const;

  FileList* files() const;
  void setFiles(FileList*);

  void SetFilesFromPaths(const Vector<String>&);

  // Returns true if the given DragData has more than one dropped files.
  bool ReceiveDroppedFiles(const DragData*);

  String DroppedFileSystemId();

  // These functions are used for laying out the input active during a
  // drag-and-drop operation.
  bool CanReceiveDroppedFiles() const;
  void SetCanReceiveDroppedFiles(bool);

  // Returns 'Choose File(s)' button in a file control. This returns
  // nullptr for other input types.
  HTMLInputElement* UploadButton() const;

  void OnSearch();

  void UpdateClearButtonVisibility();

  bool WillRespondToMouseClickEvents() override;

  // Returns the DataList element associated via the list attribute, if any.
  // Resolves the reference target if necessary. This element may be inside a
  // shadow root, and should not be returned to JavaScript.
  HTMLDataListElement* DataList() const;
  // For JavaScript binding, return the DataList element without resolving the
  // reference target, to avoid exposing shadow root content to JavaScript.
  HTMLElement* listForBinding() const;
  bool HasValidDataListOptions() const;
  void ListAttributeTargetChanged();
  // Associated <datalist> options which match to the current INPUT value.
  HeapVector<Member<HTMLOptionElement>> FilteredDataListOptions() const;

  // Functions for InputType classes.
  void SetNonAttributeValue(const String&);
  void SetNonAttributeValueByUserEdit(const String&);
  void UpdateView();
  bool NeedsToUpdateViewValue() const { return needs_to_update_view_value_; }
  void SetInnerEditorValue(const String&) override;

  // For test purposes.
  void SelectColorInColorChooser(const Color&);
  void EndColorChooserForTesting();

  String DefaultToolTip() const override;

  // Type=file only: Text not in the button such as "No file chosen". The string
  // is not truncated by ellipsis.
  // Return a null string for other types.
  String FileStatusText() const;
  // Returns true if an ellipsis should be injected at the middle of the text.
  // This function is called only if text-overflow:ellipsis is specified.
  bool ShouldApplyMiddleEllipsis() const;

  unsigned height() const;
  unsigned width() const;
  void setHeight(unsigned);
  void setWidth(unsigned);

  void blur() final;
  void DefaultBlur();

  const AtomicString& GetName() const final;

  void EndEditing();

  static Vector<String> FilesFromFileInputFormControlState(
      const FormControlState&);

  bool MatchesReadOnlyPseudoClass() const final;
  bool MatchesReadWritePseudoClass() const final;
  AppearanceValue AutoAppearance() const;

  void setRangeText(const String& replacement, ExceptionState&) final;
  void setRangeText(const String& replacement,
                    unsigned start,
                    unsigned end,
                    const V8SelectionMode& selection_mode,
                    ExceptionState&) final;

  HTMLImageLoader* ImageLoader() const { return image_loader_.Get(); }
  HTMLImageLoader& EnsureImageLoader();

  bool SetupDateTimeChooserParameters(DateTimeChooserParameters&);

  bool SupportsInputModeAttribute() const;

  void CapsLockStateMayHaveChanged();
  bool ShouldDrawCapsLockIndicator() const;
  void SetShouldRevealPassword(bool value);
  bool ShouldRevealPassword() const { return should_reveal_password_; }
  bool IsLastInputElementInForm();
  void DispatchSimulatedEnter();
  AXObject* PopupRootAXObject();
  void DidNotifySubtreeInsertionsToDocument() override;

  virtual void EnsureFallbackContent();
  virtual void EnsurePrimaryContent();
  bool HasFallbackContent() const;

  bool IsPlaceholderVisible() const override { return is_placeholder_visible_; }
  void SetPlaceholderVisibility(bool) override;

  unsigned SizeOfRadioGroup() const;

  bool SupportsPlaceholder() const final;
  String GetPlaceholderValue() const final;

  void ChildrenChanged(const ChildrenChange&) override;

  LayoutBox* GetLayoutBoxForScrolling() const final;

  bool IsDraggedSlider() const;

  mojom::blink::FormControlType FormControlType() const final;

  bool isMutable();
  void showPicker(ExceptionState&);
  bool IsPickerVisible() const;

  ShadowRoot* EnsureShadowSubtree();

  bool IsValidBuiltinCommand(HTMLElement& invoker,
                             CommandEventType command) override;
  bool HandleCommandInternal(HTMLElement& invoker,
                             CommandEventType command) override;

  void SetFocused(bool is_focused, mojom::blink::FocusType) override;

 protected:
  void DefaultEventHandler(Event&) override;
  bool IsInnerEditorValueEmpty() const final;

 private:
  enum AutoCompleteSetting { kUninitialized, kOn, kOff };

  void WillChangeForm() final;
  void DidChangeForm() final;
  InsertionNotificationRequest InsertedInto(ContainerNode&) override;
  void RemovedFrom(ContainerNode&) final;
  void DidMoveToNewDocument(Document& old_document) final;
  bool HasActivationBehavior() const override;

  bool HasCustomFocusLogic() const final;
  bool IsKeyboardFocusableSlow(UpdateBehavior update_behavior =
                                   UpdateBehavior::kStyleAndLayout) const final;
  bool MayTriggerVirtualKeyboard() const final;
  bool ShouldHaveFocusAppearance() const final;
  bool IsEnumeratable() const final;
  bool IsInteractiveContent() const final;
  bool IsLabelable() const final;
  bool MatchesDefaultPseudoClass() const override;
  bool IsTextControl() const final { return IsTextField(); }
  int scrollWidth() override;
  int scrollHeight() override;

  bool CanTriggerImplicitSubmission() const final { return IsTextField(); }

  const AtomicString& FormControlTypeAsString() const override;

  bool ShouldSaveAndRestoreFormControlState() const final;
  FormControlState SaveFormControlState() const final;
  void RestoreFormControlState(const FormControlState&) final;

  bool CanStartSelection() const final;

  void AccessKeyAction(SimulatedClickCreationScope creation_scope) final;

  void DidRecalcStyle(const StyleRecalcChange) override;
  void ParseAttribute(const AttributeModificationParams&) override;
  bool IsPresentationAttribute(const QualifiedName&) const final;
  void CollectStyleForPresentationAttribute(
      const QualifiedName&,
      const AtomicString&,
      HeapVector<CSSPropertyValue, 8>&) final;
  void FinishParsingChildren() final;
  void ParserDidSetAttributes() final;

  void CloneNonAttributePropertiesFrom(const Element&, NodeCloningData&) final;

  void AttachLayoutTree(AttachContext&) final;

  void AppendToFormData(FormData&) final;
  String ResultForDialogSubmit() final;

  bool CanBeSuccessfulSubmitButton() const final;

  void ResetImpl() final;

  EventDispatchHandlingState* PreDispatchEventHandler(Event&) final;
  void PostDispatchEventHandler(Event&, EventDispatchHandlingState*) final;

  bool IsURLAttribute(const Attribute&) const final;
  bool HasLegalLinkAttribute(const QualifiedName&) const final;
  bool IsInRange() const final;
  bool IsOutOfRange() const final;

  bool TooLong(const String&, NeedsToCheckDirtyFlag) const;
  bool TooShort(const String&, NeedsToCheckDirtyFlag) const;

  void CreateInnerEditorElementIfNecessary() const final;
  HTMLElement* UpdatePlaceholderText() final;
  void HandleBlurEvent() final;
  void DispatchFocusInEvent(const AtomicString& event_type,
                            Element* old_focused_element,
                            mojom::blink::FocusType,
                            InputDeviceCapabilities* source_capabilities) final;

  bool IsOptionalFormControl() const final { return !IsRequiredFormControl(); }
  bool IsRequiredFormControl() const final;
  bool RecalcWillValidate() const final;
  void RequiredAttributeChanged() final;
  void DisabledAttributeChanged() final;

  void InitializeTypeInParsing();
  void UpdateType(const AtomicString&);

  void UpdateHasBeenPasswordField(const AtomicString& new_type_name);

  void SubtreeHasChanged() final;

  void SetListAttributeTargetObserver(ListAttributeTargetObserver*);
  void ResetListAttributeTargetObserver();

  void AddToRadioButtonGroup();
  void RemoveFromRadioButtonGroup();
  void AdjustStyle(ComputedStyleBuilder&) override;

  void MaybeReportPiiMetrics();

  AtomicString name_;
  // The value string in |value| value mode.
  String non_attribute_value_;
  unsigned size_;
  // https://html.spec.whatwg.org/C/#concept-input-value-dirty-flag
  unsigned has_dirty_value_ : 1;
  // https://html.spec.whatwg.org/C/#concept-fe-checked
  unsigned is_checked_ : 1;
  // https://html.spec.whatwg.org/C/#concept-input-checked-dirty-flag
  unsigned dirty_checkedness_ : 1;
  unsigned is_indeterminate_ : 1;
  unsigned is_activated_submit_ : 1;
  unsigned autocomplete_ : 2;  // AutoCompleteSetting
  unsigned has_non_empty_list_ : 1;
  unsigned state_restored_ : 1;
  unsigned parsing_in_progress_ : 1;
  unsigned can_receive_dropped_files_ : 1;
  unsigned should_reveal_password_ : 1;
  unsigned needs_to_update_view_value_ : 1;
  unsigned is_placeholder_visible_ : 1;
  unsigned has_been_password_field_ : 1;
  unsigned scheduled_create_shadow_tree_ : 1;
  Member<InputType> input_type_;
  Member<InputTypeView> input_type_view_;
  // The ImageLoader must be owned by this element because the loader code
  // assumes that it lives as long as its owning element lives. If we move the
  // loader into the ImageInput object we may delete the loader while this
  // element lives on.
  Member<HTMLImageLoader> image_loader_;
  Member<ListAttributeTargetObserver> list_attribute_target_observer_;

  FRIEND_TEST_ALL_PREFIXES(HTMLInputElementTest, RadioKeyDownDCHECKFailure);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_HTML_INPUT_ELEMENT_H_
