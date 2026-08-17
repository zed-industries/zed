// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_TIME_DISPLAY_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_TIME_DISPLAY_ELEMENT_H_

#include "third_party/blink/renderer/modules/media_controls/elements/media_control_div_element.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace blink {

class MediaControlsImpl;

class MediaControlTimeDisplayElement : public MediaControlDivElement {
 public:
  // Exported to be used in unit tests.
  MODULES_EXPORT void SetCurrentValue(double);
  MODULES_EXPORT double CurrentValue() const;

  gfx::Size GetSizeOrDefault() const override;

  virtual String FormatTime() const;

 protected:
  explicit MediaControlTimeDisplayElement(MediaControlsImpl&);

  virtual int EstimateElementWidth() const;

 private:
  void SetAriaLabel();

  std::optional<double> current_value_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_TIME_DISPLAY_ELEMENT_H_
