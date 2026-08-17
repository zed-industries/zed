// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_JAVASCRIPT_FRAMEWORK_DETECTION_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_JAVASCRIPT_FRAMEWORK_DETECTION_MOJOM_TRAITS_H_

#include "build/build_config.h"
#include "mojo/public/cpp/bindings/enum_traits.h"
#include "mojo/public/cpp/bindings/struct_traits.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/loader/javascript_framework_detection.h"

namespace mojo {

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::JavaScriptFrameworkDetectionResultDataView,
                 blink::JavaScriptFrameworkDetectionResult> {
  static const std::map<blink::JavaScriptFramework, int16_t>& detected_versions(
      const blink::JavaScriptFrameworkDetectionResult& r) {
    return r.detected_versions;
  }

  static bool Read(blink::mojom::JavaScriptFrameworkDetectionResultDataView r,
                   blink::JavaScriptFrameworkDetectionResult* out) {
    return r.ReadDetectedVersions(&out->detected_versions);
  }
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_JAVASCRIPT_FRAMEWORK_DETECTION_MOJOM_TRAITS_H_
