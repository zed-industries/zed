// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_TEST_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_TEST_UTILS_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_dom_point_init.h"
#include "third_party/blink/renderer/core/geometry/dom_point_read_only.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_typed_array.h"
#include "ui/gfx/geometry/transform.h"

namespace blink {

constexpr double kEpsilon = 0.0001;

Vector<double> GetMatrixDataForTest(const gfx::Transform& matrix);
DOMPointInit* MakePointForTest(double x, double y, double z, double w);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_TEST_UTILS_H_
