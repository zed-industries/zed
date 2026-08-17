// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_GEOMETRY_TEST_HELPERS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_GEOMETRY_TEST_HELPERS_H_

#include "third_party/blink/renderer/platform/geometry/physical_offset.h"
#include "third_party/blink/renderer/platform/geometry/physical_size.h"

namespace blink {

// These constructors are for convenience of tests to construct these geometries
// from integers.
template <typename ValueType>
constexpr PhysicalFixedOffset<ValueType>::PhysicalFixedOffset(int left, int top)
    : left(left), top(top) {}
constexpr PhysicalSize::PhysicalSize(int width, int height)
    : width(width), height(height) {}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_GEOMETRY_TEST_HELPERS_H_
