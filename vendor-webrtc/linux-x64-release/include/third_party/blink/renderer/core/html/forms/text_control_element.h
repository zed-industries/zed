/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2000 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2004, 2005, 2006, 2007, 2008, 2009, 2010 Apple Inc. All rights
 * reserved.
 * Copyright (C) 2009, 2010, 2011 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_TEXT_CONTROL_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_TEXT_CONTROL_ELEMENT_H_

#include "base/gtest_prod_util.h"
#include "third_party/blink/public/mojom/input/focus_type.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/core/html/forms/html_form_control_element_with_state.h"
#include "third_party/blink/renderer/core/html/forms/text_control_inner_elements.h"

namespace blink {

class ExceptionState;
class V8SelectionMode;

enum TextFieldSelectionDirection {
  kSelectionHasNoDirection,
  kSelectionHasForwardDirection,
  kSelectionHasBackwardDirection
};
enum class TextFieldEventBehavior {
  kDispatchNoEvent,
  kDispatchChangeEvent,
  kDispatchInputEvent,
  kDispatchInputAndChangeEvent
};

enum class TextControlSetValueSelection {
  kSetSelectionToStart,
  kSetSelectionToEnd,
  kClamp,
  kDoNotSet,
};

class CORE_EXPORT TextControlElement : public HTMLFormControlElementWithState {
 public:
  // Common flag for HTMLInputElement::tooLong(),
  // HTMLTextAreaElement::tooLong(),
  // HTMLInputElement::tooShort() and HTMLTextAreaElement::tooShort().
  enum NeedsToCheckDirtyFlag { kCheckDirtyFlag, kIgnoreDirtyFlag };

  ~TextControlElement() override;

  void ForwardEvent(Event&);

  void SetFocused(bool, mojom::blink::FocusType) override;

  bool IsRichlyEditableForAccessibility() const override { return false; }

  // The derived class should return true if placeholder processing is needed.
  virtual bool IsPlaceholderVisible() const = 0;
  virtual void SetPlaceholderVisibility(bool) = 0;
  virtual bool SupportsPlaceholder() const = 0;
  String StrippedPlaceholder() const;
  HTMLElement* PlaceholderElement() const;
  void UpdatePlaceholderVisibility();
  void UpdatePlaceholderShadowPseudoId(HTMLElement& placeholder);
  virtual String GetPlaceholderValue() const = 0;

  VisiblePosition VisiblePositionForIndex(int) const;
  unsigned selectionStart() const;
  unsigned selectionEnd() const;
  const AtomicString& selectionDirection() const;
  void setSelectionStart(unsigned);
  void setSelectionEnd(unsigned);
  void setSelectionDirection(const String&);
  void select();
  virtual void setRangeText(const String& replacement, ExceptionState&);
  virtual void setRangeText(const String& replacement,
                            unsigned start,
                            unsigned end,
                            const V8SelectionMode& selection_mode,
                            ExceptionState&);
  // Web-exposed setSelectionRange() function. This schedule to dispatch
  // 'select' event.
  void setSelectionRangeForBinding(unsigned start,
                                   unsigned end,
                                   const String& direction = "none");
  // Blink-internal version of setSelectionRange(). This translates "none"
  // direction to "forward" on platforms without "none" direction.
  // This returns true if it updated cached selection and/or FrameSelection.
  bool SetSelectionRange(
      unsigned start,
      unsigned end,
      TextFieldSelectionDirection = kSelectionHasNoDirection);
  SelectionInDOMTree Selection() const;

  int maxLength() const;
  int minLength() const;
  void setMaxLength(int, ExceptionState&);
  void setMinLength(int, ExceptionState&);

  // Dispatch 'change' event if the value is updated.
  void DispatchFormControlChangeEvent();
  // Enqueue 'change' event if the value is updated.
  void EnqueueChangeEvent();
  // This should be called on every user-input, before the user-input changes
  // the value.
  void SetValueBeforeFirstUserEditIfNotSet();
  // This should be called on every user-input, after the user-input changed the
  // value. The argument is the updated value.
  void CheckIfValueWasReverted(const String&);
  void ClearValueBeforeFirstUserEdit();

  virtual String Value() const = 0;
  virtual void SetValue(
      const String&,
      TextFieldEventBehavior = TextFieldEventBehavior::kDispatchNoEvent,
      TextControlSetValueSelection =
          TextControlSetValueSelection::kSetSelectionToEnd,
      WebAutofillState = WebAutofillState::kNotFilled) = 0;

  TextControlInnerEditorElement* InnerEditorElement() const {
    return inner_editor_.Get();
  }
  HTMLElement* CreateInnerEditorElement();
  void DropInnerEditorElement() { inner_editor_ = nullptr; }

  void SelectionChanged(bool user_triggered);
  bool LastChangeWasUserEdit() const;

  virtual void SetInnerEditorValue(const String&);
  String InnerEditorValue() const;
  Node* CreatePlaceholderBreakElement() const;

  String DirectionForFormData() const;
  // https://html.spec.whatwg.org/#auto-directionality-form-associated-elements
  // Check if, when dir=auto, we should use the value to define text direction.
  // For example, when value contains a bidirectional character.
  virtual bool IsAutoDirectionalityFormAssociated() const = 0;

  // Set the value trimmed to the max length of the field and dispatch the input
  // and change events. If |value| is empty, the autofill state is always
  // set to WebAutofillState::kNotFilled.
  void SetAutofillValue(const String& value,
                        WebAutofillState = WebAutofillState::kAutofilled);

  // A null value indicates that the suggested value should be hidden.
  virtual void SetSuggestedValue(const String& value);
  const String& SuggestedValue() const;

  void ScheduleSelectionchangeEvent();

  void ResetEventQueueStatus(const AtomicString& event_type) override {
    if (event_type == event_type_names::kSelectionchange)
      has_scheduled_selectionchange_event_ = false;
  }

  void Trace(Visitor*) const override;

  ETextOverflow ValueForTextOverflow() const;

 protected:
  TextControlElement(const QualifiedName&, Document&);
  virtual HTMLElement* UpdatePlaceholderText() = 0;

  // Creates the editor if necessary. Implementations that support an editor
  // should callback to CreateInnerEditorElement().
  virtual void CreateInnerEditorElementIfNecessary() const = 0;

  void ParseAttribute(const AttributeModificationParams&) override;

  void RestoreCachedSelection();

  void DefaultEventHandler(Event&) override;
  virtual void SubtreeHasChanged() = 0;

  void SetLastChangeWasNotUserEdit() { last_change_was_user_edit_ = false; }
  void AddPlaceholderBreakElementIfNecessary();
  String ValueWithHardLineBreaks() const;

  void CloneNonAttributePropertiesFrom(const Element&,
                                       NodeCloningData&) override;

  // Returns true if the inner-editor value is empty. This may be cheaper
  // than calling InnerEditorValue(), and InnerEditorValue() returns
  // the wrong thing if the editor hasn't been created yet.
  virtual bool IsInnerEditorValueEmpty() const = 0;

  TextControlInnerEditorElement* EnsureInnerEditorElement() const {
    if (!inner_editor_) {
      CreateInnerEditorElementIfNecessary();
    }
    return inner_editor_.Get();
  }

 private:
  // Used by ComputeSelection() to specify which values are needed.
  enum ComputeSelectionFlags {
    kStart = 1 << 0,
    kEnd = 1 << 1,
    kDirection = 1 << 2,
  };

  struct ComputedSelection {
    unsigned start = 0;
    unsigned end = 0;
    TextFieldSelectionDirection direction = kSelectionHasNoDirection;
  };

  bool ShouldApplySelectionCache() const;
  // Computes the selection. `flags` is a bitmask of ComputeSelectionFlags that
  // indicates what values should be computed.
  void ComputeSelection(uint32_t flags,
                        ComputedSelection& computed_selection) const;
  // Returns true if cached values and arguments are not same.
  bool CacheSelection(unsigned start,
                      unsigned end,
                      TextFieldSelectionDirection);
  static unsigned IndexForPosition(HTMLElement* inner_editor, const Position&);

  bool DispatchFocusEvent(Element* old_focused_element,
                          mojom::blink::FocusType,
                          InputDeviceCapabilities* source_capabilities) final;
  void DispatchBlurEvent(Element* new_focused_element,
                         mojom::blink::FocusType,
                         InputDeviceCapabilities* source_capabilities) final;
  void ScheduleSelectEvent();
  void ScheduleSelectionchangeEventOnThisOrDocument();
  void DisabledOrReadonlyAttributeChanged(const QualifiedName&);

  // Called in dispatchFocusEvent(), after placeholder process, before calling
  // parent's dispatchFocusEvent().
  virtual void HandleFocusEvent(Element* /* oldFocusedNode */,
                                mojom::blink::FocusType) {}
  // Called in dispatchBlurEvent(), after placeholder process, before calling
  // parent's dispatchBlurEvent().
  virtual void HandleBlurEvent() {}

  // Whether the placeholder attribute value should be visible. Does not
  // necessarily match the placeholder_element visibility because it can be used
  // for suggested values too.
  bool PlaceholderShouldBeVisible() const;

  // Held directly instead of looked up by ID for speed.
  // Not only is the lookup faster, but for simple text inputs it avoids
  // creating a number of TreeScope data structures to track elements by ID.
  Member<TextControlInnerEditorElement> inner_editor_;

  // In value_before_first_user_edit_, we distinguish a null String and
  // zero-length String. Null String means the field doesn't have any data yet,
  // and zero-length String is a valid data.
  String value_before_first_user_edit_;
  bool last_change_was_user_edit_;

  unsigned cached_selection_start_;
  unsigned cached_selection_end_;
  TextFieldSelectionDirection cached_selection_direction_;

  String suggested_value_;
  String value_before_set_suggested_value_;

  // Indicate whether there is one scheduled selectionchange event.
  bool has_scheduled_selectionchange_event_ = false;

  FRIEND_TEST_ALL_PREFIXES(TextControlElementTest, IndexForPosition);
  FRIEND_TEST_ALL_PREFIXES(HTMLTextAreaElementTest, ValueWithHardLineBreaks);
  FRIEND_TEST_ALL_PREFIXES(HTMLTextAreaElementTest, ValueWithHardLineBreaksRtl);
};

inline bool IsTextControl(const Node& node) {
  auto* element = DynamicTo<Element>(node);
  return element && element->IsTextControl();
}
inline bool IsTextControl(const Node* node) {
  return node && IsTextControl(*node);
}

// ToTextControl() is stricter than To<TextControlElement>(). ToTextControl()
// does not accept HTMLInputElement with non-text types.
#define DEFINE_TEXT_CONTROL_CASTS(Type, ArgType)                              \
  inline Type* ToTextControl(ArgType* node) {                                 \
    DCHECK(!node || IsTextControl(*node));                                    \
    return To<TextControlElement>(node);                                      \
  }                                                                           \
  inline Type& ToTextControl(ArgType& node) {                                 \
    DCHECK(IsTextControl(node));                                              \
    return To<TextControlElement>(node);                                      \
  }                                                                           \
  inline Type* ToTextControlOrNull(ArgType* node) {                           \
    return node && IsTextControl(*node) ? static_cast<Type*>(node) : nullptr; \
  }                                                                           \
  inline Type* ToTextControlOrNull(ArgType& node) {                           \
    return IsTextControl(node) ? static_cast<Type*>(&node) : nullptr;         \
  }                                                                           \
  void ToTextControl(Type*);                                                  \
  void ToTextControl(Type&)

DEFINE_TEXT_CONTROL_CASTS(TextControlElement, Node);
DEFINE_TEXT_CONTROL_CASTS(const TextControlElement, const Node);

#undef DEFINE_TEXT_CONTROL_CASTS

template <>
struct DowncastTraits<TextControlElement> {
  static bool AllowFrom(const Node& node) {
    return node.HasTagName(html_names::kInputTag) ||
           node.HasTagName(html_names::kTextareaTag);
  }
};

TextControlElement* EnclosingTextControl(const Position&);
TextControlElement* EnclosingTextControl(const PositionInFlatTree&);
CORE_EXPORT TextControlElement* EnclosingTextControl(const Node*);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_TEXT_CONTROL_ELEMENT_H_
