/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
 * Copyright (C) 2011 Apple Inc. All rights reserved.
 * Copyright (C) 2012 Samsung Electronics. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_INPUT_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_INPUT_TYPE_H_

#include <optional>

#include "third_party/blink/public/mojom/forms/form_control_type.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/core/frame/web_feature_forward.h"
#include "third_party/blink/renderer/core/html/forms/color_chooser_client.h"
#include "third_party/blink/renderer/core/html/forms/step_range.h"
#include "third_party/blink/renderer/core/html/forms/text_control_element.h"

namespace blink {

class ChromeClient;
class DragData;
class ExceptionState;
class FileList;
class FormData;
class InputTypeView;

// An InputType object represents the type-specific part of an HTMLInputElement.
// Do not expose instances of InputType and classes derived from it to classes
// other than HTMLInputElement.
class CORE_EXPORT InputType : public GarbageCollected<InputType> {
 public:
  // The type attribute of HTMLInputElement is an enumerated attribute:
  // https://html.spec.whatwg.org/multipage/input.html#attr-input-type
  // These values are a subset of the `FormControlType` enum. They have the same
  // binary representation so that FormControlType() reduces to a type cast.
  enum class Type : std::underlying_type_t<mojom::blink::FormControlType> {
    kButton = base::to_underlying(mojom::blink::FormControlType::kInputButton),
    kColor = base::to_underlying(mojom::blink::FormControlType::kInputColor),
    kFile = base::to_underlying(mojom::blink::FormControlType::kInputFile),
    kHidden = base::to_underlying(mojom::blink::FormControlType::kInputHidden),
    kImage = base::to_underlying(mojom::blink::FormControlType::kInputImage),
    kNumber = base::to_underlying(mojom::blink::FormControlType::kInputNumber),
    kRange = base::to_underlying(mojom::blink::FormControlType::kInputRange),
    kReset = base::to_underlying(mojom::blink::FormControlType::kInputReset),
    kSubmit = base::to_underlying(mojom::blink::FormControlType::kInputSubmit),

    // BaseCheckable
    kRadio = base::to_underlying(mojom::blink::FormControlType::kInputRadio),
    kCheckbox =
        base::to_underlying(mojom::blink::FormControlType::kInputCheckbox),

    // BaseTemporal
    kDate = base::to_underlying(mojom::blink::FormControlType::kInputDate),
    kDateTimeLocal =
        base::to_underlying(mojom::blink::FormControlType::kInputDatetimeLocal),
    kMonth = base::to_underlying(mojom::blink::FormControlType::kInputMonth),
    kTime = base::to_underlying(mojom::blink::FormControlType::kInputTime),
    kWeek = base::to_underlying(mojom::blink::FormControlType::kInputWeek),

    // BaseText
    kEmail = base::to_underlying(mojom::blink::FormControlType::kInputEmail),
    kPassword =
        base::to_underlying(mojom::blink::FormControlType::kInputPassword),
    kSearch = base::to_underlying(mojom::blink::FormControlType::kInputSearch),
    kTelephone =
        base::to_underlying(mojom::blink::FormControlType::kInputTelephone),
    kURL = base::to_underlying(mojom::blink::FormControlType::kInputUrl),
    kText = base::to_underlying(mojom::blink::FormControlType::kInputText),
  };

  static const AtomicString& TypeToString(Type);

  static InputType* Create(HTMLInputElement&, const AtomicString&);
  static const AtomicString& NormalizeTypeName(const AtomicString&);
  InputType(const InputType&) = delete;
  InputType& operator=(const InputType&) = delete;
  virtual ~InputType();
  virtual void Trace(Visitor*) const;

  virtual InputTypeView* CreateView() = 0;

  Type type() const { return type_; }

  const AtomicString& FormControlTypeAsString() const;
  mojom::blink::FormControlType FormControlType() const {
    return static_cast<mojom::blink::FormControlType>(
        base::to_underlying(type_));
  }

  virtual bool IsInteractiveContent() const;
  virtual bool IsAutoDirectionalityFormAssociated() const;

  bool IsButton() const {
    return type_ == Type::kButton || type_ == Type::kImage ||
           type_ == Type::kReset || type_ == Type::kSubmit;
  }
  bool IsTextButton() const {
    return type_ == Type::kButton || type_ == Type::kReset ||
           type_ == Type::kSubmit;
  }
  bool IsButtonInputType() const { return type_ == Type::kButton; }
  bool IsColorInputType() const { return type_ == Type::kColor; }
  bool IsFileInputType() const { return type_ == Type::kFile; }
  bool IsHiddenInputType() const { return type_ == Type::kHidden; }
  bool IsImageInputType() const { return type_ == Type::kImage; }
  bool IsNumberInputType() const { return type_ == Type::kNumber; }
  bool IsRangeInputType() const { return type_ == Type::kRange; }
  bool IsResetInputType() const { return type_ == Type::kReset; }
  bool IsSubmitInputType() const { return type_ == Type::kSubmit; }
  bool IsRadioInputType() const { return type_ == Type::kRadio; }
  bool IsCheckboxInputType() const { return type_ == Type::kCheckbox; }
  bool IsBaseCheckableInputType() const {
    return type_ == Type::kRadio || type_ == Type::kCheckbox;
  }
  bool IsDateInputType() const { return type_ == Type::kDate; }
  bool IsDateTimeLocalInputType() const {
    return type_ == Type::kDateTimeLocal;
  }
  bool IsMonthInputType() const { return type_ == Type::kMonth; }
  bool IsTimeInputType() const { return type_ == Type::kTime; }
  bool IsWeekInputType() const { return type_ == Type::kWeek; }
  bool IsBaseTemporalInputType() const {
    return type_ == Type::kDate || type_ == Type::kDateTimeLocal ||
           type_ == Type::kMonth || type_ == Type::kTime ||
           type_ == Type::kWeek;
  }
  bool IsEmailInputType() const { return type_ == Type::kEmail; }
  bool IsPasswordInputType() const { return type_ == Type::kPassword; }
  bool IsSearchInputType() const { return type_ == Type::kSearch; }
  bool IsTelephoneInputType() const { return type_ == Type::kTelephone; }
  bool IsTextInputType() const { return type_ == Type::kText; }
  bool IsURLInputType() const { return type_ == Type::kURL; }
  bool IsBaseTextInputType() const {
    return type_ == Type::kEmail || type_ == Type::kPassword ||
           type_ == Type::kSearch || type_ == Type::kTelephone ||
           type_ == Type::kURL || type_ == Type::kText;
  }
  bool IsTextFieldInputType() const {
    return IsBaseTextInputType() || IsNumberInputType();
  }

  bool IsValidValue(const String&) const;

  // Form value functions

  virtual bool ShouldSaveAndRestoreFormControlState() const;
  virtual bool IsFormDataAppendable() const;
  virtual void AppendToFormData(FormData&) const;
  virtual String ResultForDialogSubmit() const;

  // DOM property functions

  // Returns a string value in ValueMode::kFilename.
  virtual String ValueInFilenameValueMode() const;
  // Default string to be used for showing button and form submission if |value|
  // is missing.
  virtual String DefaultLabel() const;

  // https://html.spec.whatwg.org/C/#dom-input-value
  enum class ValueMode { kValue, kDefault, kDefaultOn, kFilename };
  virtual ValueMode GetValueMode() const = 0;

  virtual double ValueAsDate() const;
  virtual void SetValueAsDate(const std::optional<base::Time>&,
                              ExceptionState&) const;
  virtual double ValueAsDouble() const;
  virtual void SetValueAsDouble(double,
                                TextFieldEventBehavior,
                                ExceptionState&) const;
  virtual void SetValueAsDecimal(const Decimal&,
                                 TextFieldEventBehavior,
                                 ExceptionState&) const;

  // Functions related to 'checked'

  virtual void ReadingChecked() const;
  // The function is called just before updating checkedness.
  virtual void WillUpdateCheckedness(bool new_checked);

  // Validation functions

  // Returns a validation message as .first, and title attribute value as
  // .second if patternMismatch.
  std::pair<String, String> ValidationMessage(const InputTypeView&) const;
  virtual bool SupportsValidation() const;
  bool TypeMismatchFor(const String&) const;
  // Type check for the current input value. We do nothing for some types
  // though typeMismatchFor() does something for them because of value
  // sanitization.
  virtual bool TypeMismatch() const;
  virtual bool SupportsRequired() const;
  bool ValueMissing(const String&) const;
  bool PatternMismatch(const String&) const;
  virtual bool TooLong(const String&,
                       TextControlElement::NeedsToCheckDirtyFlag) const;
  virtual bool TooShort(const String&,
                        TextControlElement::NeedsToCheckDirtyFlag) const;
  bool RangeUnderflow(const String&) const;
  bool RangeOverflow(const String&) const;
  bool IsInRange(const String&) const;
  bool IsOutOfRange(const String&) const;
  void InRangeChanged() const;
  virtual Decimal DefaultValueForStepUp() const;
  double Minimum() const;
  double Maximum() const;
  bool StepMismatch(const String&) const;
  bool GetAllowedValueStep(Decimal*) const;
  virtual StepRange CreateStepRange(AnyStepHandling) const;
  void StepUp(double, ExceptionState&);
  void StepUpFromLayoutObject(int);
  virtual String BadInputText() const;
  virtual String ValueNotEqualText(const Decimal& value) const;
  virtual String RangeOverflowText(const Decimal& maximum) const;
  virtual String RangeUnderflowText(const Decimal& minimum) const;
  virtual String ReversedRangeOutOfRangeText(const Decimal& minimum,
                                             const Decimal& maximum) const;
  virtual String RangeInvalidText(const Decimal& minimum,
                                  const Decimal& maximum) const;
  virtual String TypeMismatchText() const;
  virtual String ValueMissingText() const;
  bool CanSetStringValue() const;
  virtual String LocalizeValue(const String&) const;
  virtual String VisibleValue() const;
  // Returing the null string means "use the default value."
  // This function must be called only by HTMLInputElement::sanitizeValue().
  virtual String SanitizeValue(const String&) const;
  virtual String SanitizeUserInputValue(const String&) const;
  virtual void WarnIfValueIsInvalid(const String&) const;
  void WarnIfValueIsInvalidAndElementIsVisible(const String&) const;

  virtual bool IsKeyboardFocusableSlow(
      Element::UpdateBehavior update_behavior =
          Element::UpdateBehavior::kStyleAndLayout) const;
  virtual bool MayTriggerVirtualKeyboard() const;
  virtual bool CanBeSuccessfulSubmitButton();
  virtual bool MatchesDefaultPseudoClass();

  // Miscellaneous functions

  virtual bool LayoutObjectIsNeeded();
  virtual void CountUsage();
  virtual void DidRecalcStyle(const StyleRecalcChange);
  virtual void SanitizeValueInResponseToMinOrMaxAttributeChange();
  virtual bool ShouldRespectAlignAttribute();
  virtual FileList* Files();
  // Should return true if the file list was were changed.
  virtual bool SetFiles(FileList*);
  virtual void SetFilesAndDispatchEvents(FileList*);
  virtual void SetFilesFromPaths(const Vector<String>&);
  // Should return true if the given DragData has more than one dropped files.
  virtual bool ReceiveDroppedFiles(const DragData*);
  virtual String DroppedFileSystemId();
  // Should return true if the corresponding layoutObject for a type can display
  // a suggested value.
  virtual bool CanSetSuggestedValue();
  virtual bool ShouldSendChangeEventAfterCheckedChanged();
  virtual bool CanSetValue(const String&);
  virtual void SetValue(const String&,
                        bool value_changed,
                        TextFieldEventBehavior,
                        TextControlSetValueSelection);
  virtual bool ShouldRespectListAttribute();
  virtual bool IsEnumeratable();
  virtual bool IsCheckable();
  bool IsSteppable() const;
  virtual HTMLFormControlElement::PopoverTriggerSupport
  SupportsPopoverTriggering() const;
  virtual bool ShouldRespectHeightAndWidthAttributes();
  virtual int MaxLength() const;
  virtual int MinLength() const;
  virtual bool SupportsPlaceholder() const;
  virtual bool SupportsReadOnly() const;
  virtual String DefaultToolTip(const InputTypeView&) const;
  virtual Decimal FindClosestTickMarkValue(const Decimal&);
  virtual bool HasLegalLinkAttribute(const QualifiedName&) const;
  virtual void CopyNonAttributeProperties(const HTMLInputElement&);
  virtual void OnAttachWithLayoutObject();

  // Parses the specified string for the type, and return
  // the Decimal value for the parsing result if the parsing
  // succeeds; Returns defaultValue otherwise. This function can
  // return NaN or Infinity only if defaultValue is NaN or Infinity.
  virtual Decimal ParseToNumber(const String&,
                                const Decimal& default_value) const;

  // Create a string representation of the specified Decimal value for the
  // input type. If NaN or Infinity is specified, this returns an empty
  // string. This should not be called for types without valueAsNumber.
  virtual String Serialize(const Decimal&) const;

  virtual bool ShouldAppearIndeterminate() const;

  virtual bool SupportsInputModeAttribute() const;

  virtual bool SupportsSelectionAPI() const;

  // Gets width and height of the input element if the type of the
  // element is image. It returns 0 if the element is not image type.
  virtual unsigned Height() const;
  virtual unsigned Width() const;

  virtual void DispatchSearchEvent();

  // For test purpose
  virtual ColorChooserClient* GetColorChooserClient();

 protected:
  InputType(Type type, HTMLInputElement& element)
      : type_(type), element_(element) {}
  HTMLInputElement& GetElement() const { return *element_; }
  ChromeClient* GetChromeClient() const;
  Locale& GetLocale() const;
  Decimal ParseToNumberOrNaN(const String&) const;
  void CountUsageIfVisible(WebFeature) const;

  // Derive the step base, following the HTML algorithm steps.
  Decimal FindStepBase(const Decimal&) const;

  StepRange CreateStepRange(AnyStepHandling,
                            const Decimal& step_base_default,
                            const Decimal& minimum_default,
                            const Decimal& maximum_default,
                            const StepRange::StepDescription&) const;
  StepRange CreateReversibleStepRange(AnyStepHandling,
                                      const Decimal& step_base_default,
                                      const Decimal& minimum_default,
                                      const Decimal& maximum_default,
                                      const StepRange::StepDescription&) const;
  void AddWarningToConsole(const char* message_format,
                           const String& value) const;

 private:
  // Helper for stepUp()/stepDown(). Adds step value * count to the current
  // value.
  void ApplyStep(const Decimal& current,
                 const bool current_was_invalid,
                 double count,
                 AnyStepHandling,
                 TextFieldEventBehavior,
                 ExceptionState&);

  StepRange CreateStepRange(AnyStepHandling,
                            const Decimal& step_base_default,
                            const Decimal& minimum_default,
                            const Decimal& maximum_default,
                            const StepRange::StepDescription&,
                            bool supports_reversed_range) const;

  const Type type_;
  Member<HTMLInputElement> element_;
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_INPUT_TYPE_H_
