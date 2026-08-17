// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EXPORTED_WEB_INPUT_METHOD_CONTROLLER_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EXPORTED_WEB_INPUT_METHOD_CONTROLLER_IMPL_H_

#include "third_party/blink/public/web/web_input_method_controller.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace ui {
struct ImeTextSpan;
}  // namespace ui

namespace blink {

class InputMethodController;
class LocalFrame;
class WebLocalFrameImpl;
class WebPlugin;
class WebRange;
class WebString;

class CORE_EXPORT WebInputMethodControllerImpl
    : public WebInputMethodController {
  DISALLOW_NEW();

 public:
  explicit WebInputMethodControllerImpl(WebLocalFrameImpl& web_frame);
  WebInputMethodControllerImpl(const WebInputMethodControllerImpl&) = delete;
  WebInputMethodControllerImpl& operator=(const WebInputMethodControllerImpl&) =
      delete;
  ~WebInputMethodControllerImpl() override;

  // WebInputMethodController overrides.
  bool SetComposition(const WebString& text,
                      const std::vector<ui::ImeTextSpan>& ime_text_spans,
                      const WebRange& replacement_range,
                      int selection_start,
                      int selection_end) override;
  bool CommitText(const WebString& text,
                  const std::vector<ui::ImeTextSpan>& ime_text_spans,
                  const WebRange& replacement_range,
                  int relative_caret_position) override;
  bool FinishComposingText(
      ConfirmCompositionBehavior selection_behavior) override;
  WebTextInputInfo TextInputInfo() override;
  int ComputeWebTextInputNextPreviousFlags() override;
  WebTextInputType TextInputType() override;
  WebRange CompositionRange() const override;
  bool GetCompositionCharacterBounds(std::vector<gfx::Rect>& bounds) override;

  WebRange GetSelectionOffsets() const override;

  void GetLayoutBounds(gfx::Rect* control_bounds,
                       gfx::Rect* selection_bounds) override;
  bool IsEditContextActive() const override;
  ui::mojom::VirtualKeyboardVisibilityRequest
  GetLastVirtualKeyboardVisibilityRequest() const override;
  void SetVirtualKeyboardVisibilityRequest(
      ui::mojom::VirtualKeyboardVisibilityRequest vk_visibility_request)
      override;

  void Trace(Visitor*) const;

 private:
  LocalFrame* GetFrame() const;
  InputMethodController& GetInputMethodController() const;
  WebPlugin* FocusedPluginIfInputMethodSupported() const;

  const Member<WebLocalFrameImpl> web_frame_;
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EXPORTED_WEB_INPUT_METHOD_CONTROLLER_IMPL_H_
