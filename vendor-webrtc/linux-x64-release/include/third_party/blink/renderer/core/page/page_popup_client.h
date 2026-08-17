/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_PAGE_POPUP_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_PAGE_POPUP_CLIENT_H_

#include <string_view>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/shared_buffer.h"
#include "third_party/blink/renderer/platform/wtf/text/string_utf8_adaptor.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "ui/gfx/geometry/rect.h"

namespace blink {

class CSSFontSelector;
class ChromeClient;
class Document;
class Element;
class Locale;
class Page;
class PagePopup;
class PagePopupController;
class Settings;

class CORE_EXPORT PagePopupClient {
 public:
  // Provide an HTML source to the specified buffer. The HTML
  // source is rendered in a PagePopup.
  // The content HTML supports:
  //  - No <select> popups
  //  - window.setValueAndClosePopup(number, string, bool).
  virtual void WriteDocument(SegmentedBuffer&) = 0;

  virtual Element& OwnerElement() = 0;

  virtual ChromeClient& GetChromeClient() = 0;

  virtual CSSFontSelector* CreateCSSFontSelector(Document& popup_document);

  virtual PagePopupController* CreatePagePopupController(Page&, PagePopup&);

  // Returns effective zoom factor of ownerElement, or the page zoom factor if
  // the effective zoom factor is not available.
  virtual float ZoomFactor();

  // Returns the zoom factor, adjusted for the viewport scale.
  float ScaledZoomFactor();

  // Returns a Locale object associated to the client.
  virtual Locale& GetLocale() = 0;

  // This is called by the content HTML of a PagePopup.
  // An implementation of this function should call
  // ChromeClient::closePagePopup().
  virtual void SetValueAndClosePopup(int num_value,
                                     const String& string_value,
                                     bool is_keyboard_event) = 0;

  // This is called by the content HTML of a PagePopup.
  virtual void SetValue(const String&) = 0;

  // This is called by the content HTML of a PagePopup.
  virtual void CancelPopup() = 0;

  // This is called whenever a PagePopup was closed.
  virtual void DidClosePopup() = 0;

  // This is called when popup content or its owner's position changed.
  virtual void Update(bool force_update) {}

  // Called when creating the popup to allow the popup implementation to adjust
  // the settings used for the popup document.
  virtual void AdjustSettings(Settings& popup_settings) {}

  virtual ~PagePopupClient() = default;

  // Helper functions to be used in PagePopupClient::WriteDocument().
  static void AddString(const String&, SegmentedBuffer&);
  static void AddJavaScriptString(const StringView&, SegmentedBuffer&);
  static void AddProperty(std::string_view name,
                          const StringView& value,
                          SegmentedBuffer&);
  static void AddProperty(std::string_view name, int value, SegmentedBuffer&);
  static void AddProperty(std::string_view name,
                          unsigned value,
                          SegmentedBuffer&);
  static void AddProperty(std::string_view name, bool value, SegmentedBuffer&);
  static void AddProperty(std::string_view name, double, SegmentedBuffer&);
  static void AddProperty(std::string_view name,
                          const Vector<String>& values,
                          SegmentedBuffer&);
  static void AddProperty(std::string_view name,
                          const gfx::Rect&,
                          SegmentedBuffer&);
  void AddLocalizedProperty(std::string_view name,
                            int resource_id,
                            SegmentedBuffer&);

  virtual void SetMenuListOptionsBoundsInAXTree(WTF::Vector<gfx::Rect>&,
                                                gfx::Point) {}

 protected:
  void AdjustSettingsFromOwnerColorScheme(Settings& popup_settings);
};

inline void PagePopupClient::AddString(const String& str,
                                       SegmentedBuffer& data) {
  StringUTF8Adaptor utf8(str);
  data.Append(base::span(utf8));
}

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_PAGE_POPUP_CLIENT_H_
