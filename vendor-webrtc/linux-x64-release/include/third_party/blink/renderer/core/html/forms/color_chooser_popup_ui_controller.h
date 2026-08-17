/*
 * Copyright (C) 2011 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_COLOR_CHOOSER_POPUP_UI_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_COLOR_CHOOSER_POPUP_UI_CONTROLLER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/forms/color_chooser_ui_controller.h"
#include "third_party/blink/renderer/core/page/page_popup_client.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"

namespace blink {

class ChromeClient;
class ColorChooserClient;
class Page;
class PagePopup;
class PagePopupController;

class CORE_EXPORT ColorChooserPopupUIController final
    : public ColorChooserUIController,
      public PagePopupClient {
 public:
  ColorChooserPopupUIController(LocalFrame*,
                                ChromeClient*,
                                blink::ColorChooserClient*);
  ~ColorChooserPopupUIController() override;
  void Trace(Visitor*) const override;

  // ColorChooserUIController functions:
  void OpenUI() override;

  // ColorChooser functions
  void EndChooser() override;
  AXObject* RootAXObject(Element* popup_owner) override;
  bool IsPickerVisible() const override;

  // PagePopupClient functions:
  void WriteDocument(SegmentedBuffer&) override;
  Locale& GetLocale() override;
  void SetValueAndClosePopup(int, const String&, bool) override;
  void SetValue(const String&) override;
  void CancelPopup() override;
  Element& OwnerElement() override;
  void DidClosePopup() override;
  PagePopupController* CreatePagePopupController(Page&, PagePopup&) override;
  void AdjustSettings(Settings& popup_settings) override;

  void OpenEyeDropper();
  void EyeDropperResponseHandler(bool success, uint32_t color);

 private:
  ChromeClient& GetChromeClient() override;

  void OpenPopup();

  void WriteColorPickerDocument(SegmentedBuffer&);
  void WriteColorSuggestionPickerDocument(SegmentedBuffer&);

  Member<ChromeClient> chrome_client_;
  PagePopup* popup_;
  Locale& locale_;
  HeapMojoRemote<mojom::blink::EyeDropperChooser> eye_dropper_chooser_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_COLOR_CHOOSER_POPUP_UI_CONTROLLER_H_
