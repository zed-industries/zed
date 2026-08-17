// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_MAC_ATTRIBUTED_STRING_TYPE_CONVERTER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_MAC_ATTRIBUTED_STRING_TYPE_CONVERTER_H_

#include "third_party/blink/renderer/platform/platform_export.h"
#include "ui/base/mojom/attributed_string.mojom-blink.h"

#include <CoreFoundation/CoreFoundation.h>

namespace mojo {

template <>
struct PLATFORM_EXPORT TypeConverter<ui::mojom::blink::AttributedStringPtr,
                                     CFAttributedStringRef> {
  static ui::mojom::blink::AttributedStringPtr Convert(
      CFAttributedStringRef cf_attributed_string);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_MAC_ATTRIBUTED_STRING_TYPE_CONVERTER_H_
