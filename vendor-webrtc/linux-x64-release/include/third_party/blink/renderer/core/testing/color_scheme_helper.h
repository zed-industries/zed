// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_COLOR_SCHEME_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_COLOR_SCHEME_HELPER_H_

#include "third_party/blink/public/mojom/css/preferred_color_scheme.mojom-shared.h"
#include "third_party/blink/public/mojom/css/preferred_contrast.mojom-shared.h"

namespace blink {

class Document;
class Page;
class Settings;

// ColorSchemeHelper is used to update the following values and eventually reset
// the values back to their default upon deconstruction for testing.
// Values include:
//   - PreferredRootScrollbarColorScheme,
//   - PreferredColorScheme,
//   - PreferredContrast,
//   - ForcedColors.
class ColorSchemeHelper {
 public:
  ColorSchemeHelper(Document& document);
  ColorSchemeHelper(Page& page);
  ~ColorSchemeHelper();

  void SetPreferredRootScrollbarColorScheme(
      mojom::PreferredColorScheme preferred_root_scrollbar_color_scheme);
  void SetPreferredColorScheme(
      mojom::PreferredColorScheme preferred_color_scheme);
  void SetPreferredContrast(mojom::PreferredContrast preferred_contrast);
  void SetInForcedColors(Document& document, bool in_forced_colors);
  void SetEmulatedForcedColors(Document& document, bool is_dark_theme);

 private:
  Settings& settings_;
  mojom::PreferredColorScheme default_preferred_root_scrollbar_color_scheme_ =
      mojom::PreferredColorScheme::kLight;
  mojom::PreferredColorScheme default_preferred_color_scheme_ =
      mojom::PreferredColorScheme::kLight;
  mojom::PreferredContrast default_preferred_contrast_ =
      mojom::PreferredContrast::kNoPreference;
  bool default_in_forced_colors_ = false;
  // Only to be used by the destructor, since we need to cleanup but don't store
  // the Document/Page.
  void SetInForcedColors(bool in_forced_colors);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_COLOR_SCHEME_HELPER_H_
