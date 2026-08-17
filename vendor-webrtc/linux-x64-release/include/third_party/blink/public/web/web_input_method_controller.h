// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_INPUT_METHOD_CONTROLLER_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_INPUT_METHOD_CONTROLLER_H_

#include <vector>

#include "third_party/blink/public/platform/web_text_input_info.h"
#include "third_party/blink/public/web/web_range.h"
#include "third_party/blink/public/web/web_widget.h"
#include "ui/base/ime/ime_text_span.h"

namespace gfx {
class Rect;
}  // namespace gfx

namespace blink {

class WebString;

class WebInputMethodController {
 public:
  enum ConfirmCompositionBehavior {
    kDoNotKeepSelection,
    kKeepSelection,
  };

  virtual ~WebInputMethodController() = default;

  // Called to inform the WebInputMethodController of a new composition text. If
  // selectionStart and selectionEnd has the same value, then it indicates the
  // input caret position. If the text is empty, then the existing composition
  // text will be canceled. |replacementRange| (when not null) is the range in
  // current text which should be replaced by |text|. Returns true if the
  // composition text was set successfully.
  virtual bool SetComposition(
      const WebString& text,
      const std::vector<ui::ImeTextSpan>& ime_text_spans,
      const WebRange& replacement_range,
      int selection_start,
      int selection_end) = 0;

  // Called to inform the controller to delete the ongoing composition if any,
  // insert |text|, and move the caret according to |relativeCaretPosition|.
  // |replacementRange| (when not null) is the range in current text which
  // should be replaced by |text|.
  virtual bool CommitText(const WebString& text,
                          const std::vector<ui::ImeTextSpan>& ime_text_spans,
                          const WebRange& replacement_range,
                          int relative_caret_position) = 0;

  // Called to inform the controller to confirm an ongoing composition.
  virtual bool FinishComposingText(
      ConfirmCompositionBehavior selection_behavior) = 0;

  // Returns information about the current text input of this controller. Note
  // that this query can be expensive for long fields, as it returns the
  // plain-text representation of the current editable element. Consider using
  // the lighter-weight textInputType() when appropriate.
  // WebTextInputInfo.flags won't contain Next/Previous flags.
  virtual WebTextInputInfo TextInputInfo() { return WebTextInputInfo(); }

  // This function returns a combination of
  // kWebTextInputFlagHaveNextFocusableElement and
  // kWebTextInputFlagHavePreviousFocusableElement This is splitted from
  // TextInputInfo() due to expensive operations involved.
  virtual int ComputeWebTextInputNextPreviousFlags() { return 0; }

  // Returns the type of current text input of this controller.
  virtual WebTextInputType TextInputType() { return kWebTextInputTypeNone; }

  // Fetch the current selection range of this frame.
  virtual WebRange GetSelectionOffsets() const = 0;

  // Fetches the character range of the current composition, also called the
  // "marked range."
  virtual WebRange CompositionRange() const { return WebRange(); }

  // Populate |bounds| with the composition character bounds for the ongoing
  // composition. Returns false if there is no focused input or any ongoing
  // composition.
  virtual bool GetCompositionCharacterBounds(std::vector<gfx::Rect>& bounds) {
    return false;
  }

  // Populate |control_bounds| and |selection_bounds| with the bounds fetched
  // from the active EditContext. If there isn't any active |EditContext|, then
  // these bounds are empty.
  virtual void GetLayoutBounds(gfx::Rect* control_bounds,
                               gfx::Rect* selection_bounds) = 0;
  // Returns true if there is an active |EditContext|.
  virtual bool IsEditContextActive() const = 0;

  // Returns whether show()/hide() API is called from virtualkeyboard or not.
  virtual ui::mojom::VirtualKeyboardVisibilityRequest
  GetLastVirtualKeyboardVisibilityRequest() const = 0;
  // Sets the VirtualKeyboard visibility request(show/hide/none).
  virtual void SetVirtualKeyboardVisibilityRequest(
      ui::mojom::VirtualKeyboardVisibilityRequest vk_visibility_request) = 0;
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_INPUT_METHOD_CONTROLLER_H_
