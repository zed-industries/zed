// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_INTERNAL_POPUP_MENU_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_INTERNAL_POPUP_MENU_H_

#include "base/gtest_prod_util.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/forms/popup_menu.h"
#include "third_party/blink/renderer/core/page/page_popup_client.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class AXObject;
class ChromeClient;
class ComputedStyle;
class CSSFontSelector;
class PagePopup;
class HTMLElement;
class HTMLHRElement;
class HTMLOptGroupElement;
class HTMLOptionElement;
class HTMLSelectElement;

// InternalPopupMenu is a PopupMenu implementation for platforms other than
// macOS and Android. The UI is built with an HTML page inside a PagePopup.
class CORE_EXPORT InternalPopupMenu final : public PopupMenu,
                                            public PagePopupClient {
 public:
  InternalPopupMenu(ChromeClient*, HTMLSelectElement&);
  ~InternalPopupMenu() override;
  void Trace(Visitor*) const override;

  void Update(bool force_update) override;

  void Dispose();

 private:
  FRIEND_TEST_ALL_PREFIXES(InternalPopupMenuTest, ShowSelectDisplayNone);

  class ItemIterationContext;
  void AddOption(ItemIterationContext&, HTMLOptionElement&);
  void AddOptGroup(ItemIterationContext&, HTMLOptGroupElement&);
  void AddSeparator(ItemIterationContext&, HTMLHRElement&);
  void AddElementStyle(ItemIterationContext&, HTMLElement&);

  void AppendOwnerElementPseudoStyles(const String&,
                                      SegmentedBuffer&,
                                      const ComputedStyle&);

  // PopupMenu functions:
  void Show(ShowEventType type) override;
  void Hide() override;
  void DisconnectClient() override;
  void UpdateFromElement(UpdateReason) override;
  AXObject* PopupRootAXObject() const override;

  // PagePopupClient functions:
  void WriteDocument(SegmentedBuffer&) override;
  CSSFontSelector* CreateCSSFontSelector(Document& popup_document) override;
  void SetValueAndClosePopup(int,
                             const String&,
                             bool is_keyboard_event) override;
  void SetValue(const String&) override;
  void CancelPopup() override;
  Element& OwnerElement() override;
  ChromeClient& GetChromeClient() override;
  float ZoomFactor() override { return 1.0; }
  Locale& GetLocale() override;
  void DidClosePopup() override;
  void SetMenuListOptionsBoundsInAXTree(WTF::Vector<gfx::Rect>&,
                                        gfx::Point) override;

  Member<ChromeClient> chrome_client_;
  Member<HTMLSelectElement> owner_element_;
  PagePopup* popup_;
  bool needs_update_;
  bool taller_options_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_INTERNAL_POPUP_MENU_H_
