// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_MENU_LIST_INNER_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_MENU_LIST_INNER_ELEMENT_H_

#include "third_party/blink/renderer/core/html/html_div_element.h"

namespace blink {

class MenuListInnerElement : public HTMLDivElement {
 public:
  explicit MenuListInnerElement(Document& document);

 private:
  const ComputedStyle* CustomStyleForLayoutObject(
      const StyleRecalcContext&) override;
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_MENU_LIST_INNER_ELEMENT_H_
