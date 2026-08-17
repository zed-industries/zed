// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GEOMETRY_FRAGMENT_GEOMETRY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GEOMETRY_FRAGMENT_GEOMETRY_H_

#include "third_party/blink/renderer/core/layout/geometry/logical_size.h"
#include "third_party/blink/renderer/core/layout/geometry/box_strut.h"

namespace blink {

// This represents the initial (pre-layout) geometry of a fragment. E.g.
//  - The inline-size of the fragment.
//  - The block-size of the fragment (might be |kIndefiniteSize| if height is
//    'auto' for example).
//  - The border, scrollbar, and padding.
// This *doesn't* necessarily represent the final geometry of the fragment.
struct FragmentGeometry {
  LogicalSize border_box_size;
  BoxStrut border;
  BoxStrut scrollbar;
  BoxStrut padding;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GEOMETRY_FRAGMENT_GEOMETRY_H_
