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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_PAGE_POPUP_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_PAGE_POPUP_CONTROLLER_H_

#include <optional>

#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "ui/gfx/geometry/rect.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class CSSFontSelector;
class Document;
class DOMRect;
class Page;
class PagePopup;
class PagePopupClient;

class PagePopupController : public ScriptWrappable, public Supplement<Page> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static const char kSupplementName[];
  PagePopupController(Page&, PagePopup&, PagePopupClient*);

  static PagePopupController* From(Page&);

  void setValueAndClosePopup(int num_value,
                             const WTF::String& string_value,
                             bool is_keyboard_event);
  void setValue(const WTF::String&);
  void closePopup();
  WTF::String localizeNumberString(const WTF::String&);
  WTF::String formatMonth(int year, int zero_base_month);
  WTF::String formatShortMonth(int year, int zero_base_month);
  WTF::String formatWeek(int year,
                         int week_number,
                         const WTF::String& localized_start_date);
  void ClearPagePopupClient();
  void setWindowRect(int x, int y, int width, int height);

  static CSSFontSelector* CreateCSSFontSelector(Document& popup_document);

  void Trace(Visitor*) const override;

  // Set children_updated to true if additional children have been added to the
  // menu list. The bounds are only sent to the tree if children_updated is
  // true.
  void setMenuListOptionsBoundsInAXTree(
      const HeapVector<Member<DOMRect>>& options_bounds,
      bool children_updated);

 private:
  PagePopup& popup_;
  std::optional<gfx::Point> popup_origin_;

  WTF::Vector<gfx::Rect> options_bounds_;

 protected:
  PagePopupClient* popup_client_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_PAGE_POPUP_CONTROLLER_H_
