// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_WEB_FEATURE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_WEB_FEATURE_H_

// Including the actual file that defines the WebFeature enum, like we do here,
// is heavy on the compiler. Those who do not need the definition, but could do
// with just a forward-declaration, should include WebFeatureForward.h instead.

#include "third_party/blink/public/mojom/use_counter/metrics/web_feature.mojom-blink.h"  // IWYU pragma: export
#include "third_party/blink/public/mojom/use_counter/metrics/webdx_feature.mojom-blink.h"  // IWYU pragma: export

namespace blink {
using WebFeature = mojom::WebFeature;
using WebDXFeature = mojom::blink::WebDXFeature;
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_WEB_FEATURE_H_
