// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTIES_CSS_EXPOSURE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTIES_CSS_EXPOSURE_H_

namespace blink {

// Describes whether a property is exposed to author/user style sheets,
// UA style sheets, or not at all.
enum class CSSExposure {
  // The property can't be used anywhere, i.e. it's disabled.
  kNone,
  // The property may be used in UA stylesheets, but not in author and user
  // stylesheets, and the property is otherwise not visible to the author.
  kUA,
  // The property is "web exposed", which means it's available everywhere.
  kWeb
};

inline bool IsUAExposed(CSSExposure exposure) {
  return exposure >= CSSExposure::kUA;
}

inline bool IsWebExposed(CSSExposure exposure) {
  return exposure == CSSExposure::kWeb;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTIES_CSS_EXPOSURE_H_
