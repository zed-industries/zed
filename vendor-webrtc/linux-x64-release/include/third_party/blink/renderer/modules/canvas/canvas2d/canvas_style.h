/*
 * Copyright (C) 2006, 2008 Apple Inc. All rights reserved.
 * Copyright (C) 2010 Nokia Corporation and/or its subsidiary(-ies)
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_CANVAS_STYLE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_CANVAS_STYLE_H_

#include "base/check.h"
#include "base/check_op.h"
#include "base/compiler_specific.h"
#include "cc/paint/paint_flags.h"
#include "third_party/blink/public/mojom/frame/color_scheme.mojom-blink.h"
#include "third_party/blink/renderer/platform/graphics/color.h"
#include "third_party/blink/renderer/platform/heap/forward.h"  // IWYU pragma: keep (blink::Visitor)
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

// https://github.com/include-what-you-use/include-what-you-use/issues/1546
// IWYU pragma: no_forward_declare WTF::internal::__thisIsHereToForceASemicolonAfterThisMacro

// IWYU pragma: no_include "third_party/blink/renderer/platform/heap/visitor.h"

namespace ui {
class ColorProvider;
}  // namespace ui

namespace blink {

class CanvasGradient;
class CanvasPattern;

class CanvasStyle final {
  DISALLOW_NEW();

 public:
  CanvasStyle() : type_(kColor), color_(Color::kBlack) {}
  CanvasStyle(const CanvasStyle& other) = default;

  String GetColorAsString() const {
    DCHECK_EQ(type_, kColor);
    return color_.SerializeAsCanvasColor();
  }
  CanvasGradient* GetCanvasGradient() const { return gradient_.Get(); }
  CanvasPattern* GetCanvasPattern() const { return pattern_.Get(); }

  // Applies the CanvasStyle to PaintFlags. This is the slow path to be used
  // in cases where PaintFlags has never been initialized and no assumptions
  // can be made about the CanvasStyle's type
  void ApplyToFlags(cc::PaintFlags&, float global_alpha) const;

  // Like ApplyFlags, but does nothing if type is Color. This method is called
  // at draw time to synchronize the states of live CanvasPattern and
  // CanvasGradientObjects.
  void SyncFlags(cc::PaintFlags&, float global_alpha) const;

  // FastPath: Call this instead of ApplyToFlags when the CanvasStyle is known
  // to be of type kColor.
  void ApplyColorToFlags(cc::PaintFlags&, float global_alpha) const;

  bool IsEquivalentColor(Color color) const {
    return type_ == kColor && color_ == color;
  }

  bool SetColor(Color color) {
    if (type_ == kColor) [[likely]] {
      if (color == color_) {
        return false;
      }
      color_ = color;
      return true;
    }
    type_ = kColor;
    color_ = color;
    gradient_ = nullptr;
    pattern_ = nullptr;
    return true;
  }

  void SetPattern(CanvasPattern* pattern) {
    type_ = kImagePattern;
    pattern_ = pattern;
    gradient_ = nullptr;
  }

  void SetGradient(CanvasGradient* gradient) {
    type_ = kGradient;
    gradient_ = gradient;
    pattern_ = nullptr;
  }

  void Trace(Visitor*) const;

 private:
  enum Type { kColor, kGradient, kImagePattern };
  Type type_;

  Color color_;
  Member<CanvasGradient> gradient_;
  Member<CanvasPattern> pattern_;
};

ALWAYS_INLINE void CanvasStyle::ApplyColorToFlags(cc::PaintFlags& flags,
                                                  float global_alpha) const {
  // Inlined fast path for color values: because color values are immutable
  // they can be applied once at style set time.
  DCHECK(type_ == kColor);
  flags.setShader(nullptr);
  Color color = color_;
  color.SetAlpha(color.Alpha() * global_alpha);
  flags.setColor(color.toSkColor4f());
}

ALWAYS_INLINE void CanvasStyle::SyncFlags(cc::PaintFlags& flags,
                                          float global_alpha) const {
  if (type_ == kColor) [[likely]] {
    // Color values are immutable so they never need to be sync'ed at draw time.
    return;
  }
  ApplyToFlags(flags, global_alpha);
}

enum class ColorParseResult {
  // The string identified a valid color.
  kColor,

  // The string identified the current color.
  kCurrentColor,

  // The string contains a color-mix or relative color function, which may
  // contain currentcolor.
  kColorFunction,

  // Parsing failed.
  kParseFailed
};

// Parses the canvas color string and returns the result. If the result is
// `kParsedColor`, `parsed_color` is set appropriately.
ColorParseResult ParseCanvasColorString(const String& color_string,
                                        mojom::blink::ColorScheme color_scheme,
                                        Color& parsed_color,
                                        const ui::ColorProvider* color_provider,
                                        bool is_in_web_app_scope);

// Parses the canvas color string, returning true on success. If `color_string`
// indicates the current color should be used, `parsed_color` is set to black.
// Use this function in places not associated with an HTMLCanvasElement.
bool ParseCanvasColorString(const String& color_string, Color& parsed_color);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_CANVAS_STYLE_H_
