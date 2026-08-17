// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_FRAME_OWNER_PROPERTIES_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_FRAME_OWNER_PROPERTIES_H_

#include "third_party/blink/public/mojom/css/preferred_color_scheme.mojom-shared.h"
#include "third_party/blink/public/mojom/frame/color_scheme.mojom-shared.h"
#include "third_party/blink/public/mojom/scroll/scrollbar_mode.mojom-shared.h"
#include "third_party/blink/public/platform/web_string.h"

namespace blink {

struct WebFrameOwnerProperties {
  WebString name;  // browsing context container's name
  mojom::ScrollbarMode scrollbar_mode{mojom::ScrollbarMode::kAuto};
  int margin_width{-1};
  int margin_height{-1};
  bool allow_fullscreen{false};
  bool allow_payment_request{false};
  bool is_display_none{false};
  mojom::ColorScheme color_scheme{mojom::ColorScheme::kLight};
  mojom::PreferredColorScheme preferred_color_scheme{
      mojom::PreferredColorScheme::kLight};

 public:
  WebFrameOwnerProperties() = default;

#if INSIDE_BLINK
  WebFrameOwnerProperties(const WebString& name,
                          mojom::ScrollbarMode scrollbar_mode,
                          int margin_width,
                          int margin_height,
                          bool allow_fullscreen,
                          bool allow_payment_request,
                          bool is_display_none,
                          mojom::ColorScheme color_scheme,
                          mojom::PreferredColorScheme preferred_color_scheme)
      : name(name),
        scrollbar_mode(scrollbar_mode),
        margin_width(margin_width),
        margin_height(margin_height),
        allow_fullscreen(allow_fullscreen),
        allow_payment_request(allow_payment_request),
        is_display_none(is_display_none),
        color_scheme(color_scheme),
        preferred_color_scheme(preferred_color_scheme) {}
#endif
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_FRAME_OWNER_PROPERTIES_H_
