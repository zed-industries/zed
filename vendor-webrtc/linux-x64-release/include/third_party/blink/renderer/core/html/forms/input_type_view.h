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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_INPUT_TYPE_VIEW_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_INPUT_TYPE_VIEW_H_

#include "third_party/blink/public/mojom/input/focus_type.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/event_dispatcher.h"
#include "third_party/blink/renderer/core/dom/events/simulated_click_options.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/runtime_enabled_features.h"
#include "third_party/blink/renderer/platform/text/text_direction.h"
#include "third_party/blink/renderer/platform/theme_types.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class AXObject;
class BeforeTextInsertedEvent;
class ComputedStyle;
class ComputedStyleBuilder;
class Element;
class Event;
class FormControlState;
class HTMLElement;
class HTMLFormElement;
class HTMLInputElement;
class KeyboardEvent;
class LayoutObject;
class MouseEvent;

class ClickHandlingState final : public EventDispatchHandlingState {
 public:
  void Trace(Visitor*) const override;

  bool checked;
  bool indeterminate;
  Member<HTMLInputElement> checked_radio_button;
};

// An InputTypeView object represents the UI-specific part of an
// HTMLInputElement. Do not expose instances of InputTypeView and classes
// derived from it to classes other than HTMLInputElement.
class CORE_EXPORT InputTypeView : public GarbageCollectedMixin {
 public:
  // Called by the owner HTMLInputElement when this InputType is disconnected
  // from the HTMLInputElement.
  void WillBeDestroyed();
  InputTypeView(const InputTypeView&) = delete;
  InputTypeView& operator=(const InputTypeView&) = delete;
  virtual ~InputTypeView();
  void Trace(Visitor*) const override;

  virtual bool SizeShouldIncludeDecoration(int default_size,
                                           int& preferred_size) const;

  // Event handling functions

  virtual void HandleClickEvent(MouseEvent&);
  virtual void HandleMouseDownEvent(MouseEvent&);
  virtual ClickHandlingState* WillDispatchClick();
  virtual void DidDispatchClick(Event&, const ClickHandlingState&);
  virtual void HandleKeydownEvent(KeyboardEvent&);
  virtual void HandleKeypressEvent(KeyboardEvent&);
  virtual void HandleKeyupEvent(KeyboardEvent&);
  virtual void HandleBeforeTextInsertedEvent(BeforeTextInsertedEvent&);
  virtual void ForwardEvent(Event&);
  virtual bool ShouldSubmitImplicitly(const Event&);
  virtual HTMLFormElement* FormForSubmission() const;
  virtual bool HasCustomFocusLogic() const;
  virtual void HandleFocusInEvent(Element* old_focused_element,
                                  mojom::blink::FocusType);
  virtual void HandleBlurEvent();
  virtual void HandleDOMActivateEvent(Event&);
  virtual void AccessKeyAction(SimulatedClickCreationScope creation_scope);
  virtual void Blur();
  void DispatchSimulatedClickIfActive(KeyboardEvent&) const;

  virtual void SubtreeHasChanged();
  virtual LayoutObject* CreateLayoutObject(const ComputedStyle&) const;
  virtual void AdjustStyle(ComputedStyleBuilder&) {}
  virtual AppearanceValue AutoAppearance() const;
  virtual TextDirection ComputedTextDirection();
  virtual void OpenPopupView();
  virtual void ClosePopupView();
  // HasOpenedPopup will return true if the popup has ever been opened on this
  // element. IsPickerVisible will return true if the popup is currently open.
  virtual bool HasOpenedPopup() const;
  virtual bool IsPickerVisible() const;

  // Functions for shadow trees

  bool HasCreatedShadowSubtree() const { return has_created_shadow_subtree_; }
  // If a shadow tree is needed and it hasn't been created yet, one is created.
  // `is_type_changing` indicates whether this is being called as a result of
  // changing the input-type.
  void CreateShadowSubtreeIfNeeded(bool is_type_changing = false);
  void set_needs_update_view_in_create_shadow_subtree(bool value) {
    needs_update_view_in_create_shadow_subtree_ = value;
  }
  virtual bool IsInnerEditorValueEmpty() const { return false; }
  virtual bool NeedsShadowSubtree() const;
  virtual void DestroyShadowSubtree();
  virtual HTMLInputElement* UploadButton() const;
  virtual WTF::String FileStatusText() const;

  virtual void MinOrMaxAttributeChanged();
  virtual void StepAttributeChanged();
  virtual void AltAttributeChanged();
  virtual void SrcAttributeChanged();
  virtual void UpdateView();
  virtual void MultipleAttributeChanged();
  virtual void DisabledAttributeChanged();
  virtual void ReadonlyAttributeChanged();
  virtual void RequiredAttributeChanged();
  virtual void ValueAttributeChanged();
  virtual void DidSetValue(const WTF::String&, bool value_changed);
  virtual void ListAttributeTargetChanged();
  virtual void CapsLockStateMayHaveChanged();
  virtual bool ShouldDrawCapsLockIndicator() const;
  virtual void UpdateClearButtonVisibility();

  // Updates the text in the placeholder, returning the Element representing the
  // placeholder. Returns null if there is no placeholder.
  virtual HTMLElement* UpdatePlaceholderText(bool is_suggested_value);
  virtual AXObject* PopupRootAXObject();
  virtual void EnsureFallbackContent() {}
  virtual void EnsurePrimaryContent() {}
  virtual bool HasFallbackContent() const { return false; }
  virtual FormControlState SaveFormControlState() const;
  virtual void RestoreFormControlState(const FormControlState&);
  virtual bool IsDraggedSlider() const;

  // Validation functions
  virtual bool HasBadInput() const;

  virtual wtf_size_t FocusedFieldIndex() const { return 0; }

  virtual bool IsMultipleFieldsTemporal() const { return false; }

 protected:
  InputTypeView(HTMLInputElement& element) : element_(&element) {}
  HTMLInputElement& GetElement() const { return *element_; }

  virtual void CreateShadowSubtree();

  bool will_be_destroyed_ = false;

 private:
  bool has_created_shadow_subtree_ = false;
  // If true, CreateShadowSubtreeIfNeeded() may also call UpdateView().
  bool needs_update_view_in_create_shadow_subtree_ = false;
  Member<HTMLInputElement> element_;
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_INPUT_TYPE_VIEW_H_
