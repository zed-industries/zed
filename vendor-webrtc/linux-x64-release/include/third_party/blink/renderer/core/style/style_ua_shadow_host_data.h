// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_UA_SHADOW_HOST_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_UA_SHADOW_HOST_DATA_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/style/computed_style_initial_values.h"
#include "third_party/blink/renderer/core/style/style_aspect_ratio.h"
#include "third_party/blink/renderer/platform/geometry/length.h"

namespace blink {

// Used to transport data from UA shadow hosts to descendants within
// that shadow host. The StyleUAShadowHostData can be used by descendants
// to produce a style which isn't possible to express with normal CSS.
//
// See HTMLImageFallbackHelper for an example.
class CORE_EXPORT StyleUAShadowHostData {
 public:
  StyleUAShadowHostData(const Length& width,
                        const Length& height,
                        const StyleAspectRatio& aspect_ratio,
                        const String& alt_text,
                        const AtomicString& alt_attribute,
                        const AtomicString& src_attribute,
                        const bool has_appearance)
      : width_(width),
        height_(height),
        aspect_ratio_(aspect_ratio),
        alt_text_(alt_text),
        alt_attribute_(alt_attribute),
        src_attribute_(src_attribute),
        has_appearance_(has_appearance) {}

  std::unique_ptr<StyleUAShadowHostData> Clone() const {
    return std::make_unique<StyleUAShadowHostData>(*this);
  }

  const Length& Width() const { return width_; }
  const Length& Height() const { return height_; }
  const StyleAspectRatio& AspectRatio() const { return aspect_ratio_; }
  const String& AltText() const { return alt_text_; }
  const AtomicString& AltAttribute() const { return alt_attribute_; }
  const AtomicString& SrcAttribute() const { return src_attribute_; }

  bool operator==(const StyleUAShadowHostData& o) const {
    return width_ == o.width_ && height_ == o.height_ &&
           aspect_ratio_ == o.aspect_ratio_ && alt_text_ == o.alt_text_ &&
           alt_attribute_ == o.alt_attribute_ &&
           src_attribute_ == o.src_attribute_ &&
           has_appearance_ == o.has_appearance_;
  }

 private:
  Length width_;
  Length height_;
  StyleAspectRatio aspect_ratio_;
  String alt_text_;
  AtomicString alt_attribute_;
  AtomicString src_attribute_;
  bool has_appearance_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_UA_SHADOW_HOST_DATA_H_
