// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_DEPRECATION_DEPRECATION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_DEPRECATION_DEPRECATION_H_

#include <bitset>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/frame/web_feature.h"
#include "third_party/blink/renderer/platform/instrumentation/use_counter.h"

namespace blink {
namespace mojom {
enum class PermissionsPolicyFeature;
}  // namespace mojom

class ExecutionContext;
class LocalDOMWindow;

class CORE_EXPORT Deprecation final {
  DISALLOW_NEW();

 public:
  Deprecation();
  Deprecation(const Deprecation&) = delete;
  Deprecation& operator=(const Deprecation&) = delete;

  void ClearSuppression();

  void MuteForInspector();
  void UnmuteForInspector();

  // "countDeprecation" sets the bit for this feature to 1, and sends a
  // deprecation warning to the console. Repeated calls are ignored.
  //
  // Be considerate to developers' consoles: features should only send
  // deprecation warnings when we're actively interested in removing them from
  // the platform.
  static void CountDeprecation(ExecutionContext*, WebFeature);

  // Count only features if they're being used in an iframe which does not
  // have script access into the top level window.
  static void CountDeprecationCrossOriginIframe(LocalDOMWindow*, WebFeature);

  static bool IsDeprecated(WebFeature feature);

 private:
  void SetReported(WebFeature feature);
  bool GetReported(WebFeature feature) const;

  // To minimize the report/console spam from frames coming and going, report
  // each deprecation at most once per page load per renderer process.
  std::bitset<static_cast<size_t>(WebFeature::kMaxValue) + 1>
      features_deprecation_bits_;
  unsigned mute_count_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_DEPRECATION_DEPRECATION_H_
