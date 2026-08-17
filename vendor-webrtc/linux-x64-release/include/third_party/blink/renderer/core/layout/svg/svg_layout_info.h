// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_SVG_LAYOUT_INFO_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_SVG_LAYOUT_INFO_H_

namespace blink {

struct SVGLayoutInfo {
  bool force_layout = false;
  bool scale_factor_changed = false;
  bool viewport_changed = false;
};

struct SVGLayoutResult {
  SVGLayoutResult() = delete;
  SVGLayoutResult(bool bounds_changed, bool has_viewport_dependence)
      : bounds_changed(bounds_changed),
        has_viewport_dependence(has_viewport_dependence) {}
  bool bounds_changed;
  bool has_viewport_dependence;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_SVG_LAYOUT_INFO_H_
