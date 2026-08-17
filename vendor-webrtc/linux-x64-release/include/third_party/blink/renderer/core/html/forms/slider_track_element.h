// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_SLIDER_TRACK_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_SLIDER_TRACK_ELEMENT_H_

#include "third_party/blink/renderer/core/html/html_div_element.h"

namespace blink {

// SliderTrackElement represents a track part of <input type=range>, and it's a
// parent of a SliderThumbElement. We need this dedicated C++ class because we'd
// like to provide a dedicated LayoutObject for this.
class SliderTrackElement final : public HTMLDivElement {
 public:
  explicit SliderTrackElement(Document& document);

 private:
  LayoutObject* CreateLayoutObject(const ComputedStyle& style) override;
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_SLIDER_TRACK_ELEMENT_H_
