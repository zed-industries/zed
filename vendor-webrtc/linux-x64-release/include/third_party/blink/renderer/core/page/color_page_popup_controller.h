// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_COLOR_PAGE_POPUP_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_COLOR_PAGE_POPUP_CONTROLLER_H_

#include "third_party/blink/renderer/core/page/page_popup_controller.h"

namespace blink {

class ColorChooserPopupUIController;
class Page;
class PagePopup;

class ColorPagePopupController final : public PagePopupController {
  DEFINE_WRAPPERTYPEINFO();

 public:
  ColorPagePopupController(Page&, PagePopup&, ColorChooserPopupUIController*);

  void openEyeDropper();
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_COLOR_PAGE_POPUP_CONTROLLER_H_
