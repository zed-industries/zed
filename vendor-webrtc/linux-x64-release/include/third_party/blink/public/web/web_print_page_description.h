// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_PRINT_PAGE_DESCRIPTION_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_PRINT_PAGE_DESCRIPTION_H_

#include "third_party/blink/public/common/css/page_orientation.h"
#include "third_party/blink/public/common/css/page_size_type.h"
#include "ui/gfx/geometry/size_f.h"

namespace blink {

// Description of a specific page when printing. All sizes are in CSS pixels.
struct WebPrintPageDescription {
  WebPrintPageDescription() = default;
  explicit WebPrintPageDescription(gfx::SizeF size) : size(size) {}

  // Page box size. Subtract margins to get the page *area* size.
  // https://www.w3.org/TR/css-page-3/#page-model
  gfx::SizeF size;

  float margin_top = 0;
  float margin_right = 0;
  float margin_bottom = 0;
  float margin_left = 0;
  PageOrientation orientation = PageOrientation::kUpright;
  PageSizeType page_size_type = PageSizeType::kAuto;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_PRINT_PAGE_DESCRIPTION_H_
