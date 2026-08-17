// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PREFERENCES_PREFERENCE_OVERRIDES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PREFERENCES_PREFERENCE_OVERRIDES_H_

#include <optional>

#include "third_party/blink/public/mojom/css/preferred_color_scheme.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/css/preferred_contrast.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_hash.h"

namespace blink {

class Document;

// PreferenceOverrides represents the Web Preferences API overrides.
// Spec: https://wicg.github.io/web-preferences-api/
class CORE_EXPORT PreferenceOverrides {
  USING_FAST_MALLOC(PreferenceOverrides);

 public:
  // When value_string is empty, or otherwise invalid, it clears the override.
  void SetOverride(const AtomicString& feature,
                   const String& value_string,
                   const Document*);

  std::optional<mojom::blink::PreferredColorScheme> GetPreferredColorScheme()
      const {
    return preferred_color_scheme_;
  }
  std::optional<mojom::blink::PreferredContrast> GetPreferredContrast() const {
    return preferred_contrast_;
  }
  std::optional<bool> GetPrefersReducedMotion() const {
    return prefers_reduced_motion_;
  }
  std::optional<bool> GetPrefersReducedTransparency() const {
    return prefers_reduced_transparency_;
  }
  std::optional<bool> GetPrefersReducedData() const {
    return prefers_reduced_data_;
  }

 private:
  std::optional<mojom::blink::PreferredColorScheme> preferred_color_scheme_;
  std::optional<mojom::blink::PreferredContrast> preferred_contrast_;
  std::optional<bool> prefers_reduced_motion_;
  std::optional<bool> prefers_reduced_data_;
  std::optional<bool> prefers_reduced_transparency_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PREFERENCES_PREFERENCE_OVERRIDES_H_
