// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FENCED_FRAME_FENCED_FRAME_AD_SIZES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FENCED_FRAME_FENCED_FRAME_AD_SIZES_H_

#include "ui/gfx/geometry/size.h"

namespace blink {

// On mobile devices only, certain ad sizes are allowed to scale the declared
// width to match the screen width. If the viewport width is vw, then we can
// declare a family of flexible ad sizes as follows:
static constexpr std::array<gfx::Size, 11> kAllowedAdSizes = {{
    {320, 50},
    {728, 90},
    {970, 90},
    {320, 100},
    {160, 600},
    {300, 250},
    {970, 250},
    {336, 280},
    {320, 480},
    {300, 600},
    {300, 1050},
}};

// On mobile devices only, if the viewport width is vw, then for all heights
// `h` on the following list, (vw, h) is an allowed size.
static constexpr std::array<int, 3> kAllowedAdHeights = {{
    50,
    100,
    250,
}};

// On mobile devices only, if the viewport width is vw, then for all aspect
// ratios w/h on the following list, (vw, (vw*h)/w) is an allowed size
// (rounded to the nearest integer).
static constexpr std::array<gfx::Size, 5> kAllowedAdAspectRatios = {{
    {32, 5},
    {16, 5},
    {6, 5},
    {2, 3},
    {1, 2},
}};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FENCED_FRAME_FENCED_FRAME_AD_SIZES_H_
