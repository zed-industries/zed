/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_THEME_ENGINE_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_THEME_ENGINE_H_

#include <map>
#include <optional>
#include <variant>

#include "base/notreached.h"
#include "base/time/time.h"
#include "build/build_config.h"
#include "third_party/blink/public/common/css/forced_colors.h"
#include "third_party/blink/public/mojom/frame/color_scheme.mojom-shared.h"
#include "third_party/skia/include/core/SkColor.h"
#include "ui/color/color_provider_utils.h"
#include "ui/gfx/color_palette.h"
#include "ui/gfx/geometry/rect.h"
#include "ui/gfx/geometry/size.h"

namespace cc {
class PaintCanvas;
}

namespace blink {

class WebThemeEngine {
 public:
  // The current state of the associated Part.
  enum State {
    kStateDisabled,
    kStateHover,
    kStateNormal,
    kStatePressed,
    kStateFocused,
    kStateReadonly,
  };

  // The UI part which is being accessed.
  enum Part {
    // ScrollbarTheme parts
    kPartScrollbarDownArrow,
    kPartScrollbarLeftArrow,
    kPartScrollbarRightArrow,
    kPartScrollbarUpArrow,
    kPartScrollbarHorizontalThumb,
    kPartScrollbarVerticalThumb,
    kPartScrollbarHorizontalTrack,
    kPartScrollbarVerticalTrack,
    kPartScrollbarCorner,

    // LayoutTheme parts
    kPartCheckbox,
    kPartRadio,
    kPartButton,
    kPartTextField,
    kPartMenuList,
    kPartSliderTrack,
    kPartSliderThumb,
    kPartInnerSpinButton,
    kPartProgressBar
  };

  enum class SystemThemeColor {
    kNotSupported,
    kButtonFace,
    kButtonText,
    kGrayText,
    kHighlight,
    kHighlightText,
    kHotlight,
    kMenuHighlight,
    kScrollbar,
    kWindow,
    kWindowText,
    kMaxValue = kWindowText,
  };

  // Extra parameters for drawing the PartScrollbarHorizontalTrack and
  // PartScrollbarVerticalTrack.
  struct ScrollbarTrackExtraParams {
    bool is_back =
        false;  // Whether this is the 'back' part or the 'forward' part.

    // The bounds of the entire track, as opposed to the part being painted.
    int track_x = 0;
    int track_y = 0;
    int track_width = 0;
    int track_height = 0;
    std::optional<SkColor> track_color;
  };

  // Extra parameters for PartCheckbox, PartPushButton and PartRadio.
  struct ButtonExtraParams {
    bool checked = false;
    bool indeterminate = false;  // Whether the button state is indeterminate.
    bool has_border = false;
    SkColor background_color = gfx::kPlaceholderColor;
    float zoom = 0;
  };

  // Extra parameters for PartTextField
  struct TextFieldExtraParams {
    bool is_text_area = false;
    bool is_listbox = false;
    SkColor background_color = gfx::kPlaceholderColor;
    bool has_border = false;
    bool auto_complete_active = false;
    float zoom = 0;
  };

  enum class ArrowDirection : int {
    kDown,
    kLeft,
    kRight,
  };

  // Extra parameters for PartMenuList
  struct MenuListExtraParams {
    bool has_border = false;
    bool has_border_radius = false;
    int arrow_x = 0;
    int arrow_y = 0;
    int arrow_size = 0;
    ArrowDirection arrow_direction = ArrowDirection::kDown;
    SkColor arrow_color = gfx::kPlaceholderColor;
    SkColor background_color = gfx::kPlaceholderColor;
    bool fill_content_area = false;
    float zoom = 0;
  };

  // Extra parameters for PartSliderTrack and PartSliderThumb
  struct SliderExtraParams {
    bool vertical = false;
    bool in_drag = false;
    int thumb_x = 0;
    int thumb_y = 0;
    float zoom = 0;
    bool right_to_left = false;
  };

  enum class SpinArrowsDirection : int {
    kLeftRight,
    kUpDown,
  };

  // Extra parameters for PartInnerSpinButton
  struct InnerSpinButtonExtraParams {
    bool spin_up = false;
    bool read_only = false;
    SpinArrowsDirection spin_arrows_direction = SpinArrowsDirection::kUpDown;
  };

  // Extra parameters for PartProgressBar
  struct ProgressBarExtraParams {
    bool determinate = false;
    int value_rect_x = 0;
    int value_rect_y = 0;
    int value_rect_width = 0;
    int value_rect_height = 0;
    float zoom = 0;
    bool is_horizontal = false;
  };

  // Extra parameters for scrollbar thumb.
  struct ScrollbarThumbExtraParams {
    std::optional<SkColor> thumb_color;
    std::optional<SkColor> track_color;
    bool is_thumb_minimal_mode = false;
    bool is_web_test = false;
  };

  struct ScrollbarButtonExtraParams {
    // TODO(crbug.com/1493088): We should probably pass the border-radius
    // instead.
    float zoom = 0;
    bool needs_rounded_corner = false;
    bool right_to_left = false;
    std::optional<SkColor> thumb_color;
    std::optional<SkColor> track_color;
  };

#if BUILDFLAG(IS_MAC)
  enum ScrollbarOrientation {
    // Vertical scrollbar on the right side of content.
    kVerticalOnRight,
    // Vertical scrollbar on the left side of content.
    kVerticalOnLeft,
    // Horizontal scrollbar (on the bottom of content).
    kHorizontal,
  };

  struct ScrollbarExtraParams {
    bool is_hovering = false;
    bool is_overlay = false;
    ScrollbarOrientation orientation = ScrollbarOrientation::kVerticalOnRight;
    float scale_from_dip = 0;
    std::optional<SkColor> thumb_color;
    std::optional<SkColor> track_color;
  };
#endif

  using ExtraParams = std::variant<ScrollbarTrackExtraParams,
                                   ButtonExtraParams,
                                   TextFieldExtraParams,
                                   MenuListExtraParams,
                                   SliderExtraParams,
                                   InnerSpinButtonExtraParams,
                                   ProgressBarExtraParams,
                                   ScrollbarThumbExtraParams,
#if BUILDFLAG(IS_MAC)
                                   ScrollbarExtraParams,
#endif
                                   ScrollbarButtonExtraParams>;

  virtual ~WebThemeEngine() {}

  // Gets the size of the given theme part. For variable sized items
  // like vertical scrollbar thumbs, the width will be the required width of
  // the track while the height will be the minimum height.
  virtual gfx::Size GetSize(Part) { return gfx::Size(); }

  virtual bool SupportsNinePatch(Part) const { return false; }
  virtual gfx::Size NinePatchCanvasSize(Part) const { return gfx::Size(); }
  virtual gfx::Rect NinePatchAperture(Part) const { return gfx::Rect(); }

  struct ScrollbarStyle {
    int thumb_thickness;
    int scrollbar_margin;
    SkColor4f color;
    base::TimeDelta fade_out_delay;
    base::TimeDelta fade_out_duration;
    float idle_thickness_scale;
  };

  // Gets the overlay scrollbar style. Not used on Mac.
  virtual void GetOverlayScrollbarStyle(ScrollbarStyle* style) {
    // Disable overlay scrollbar fade out (for non-composited scrollers) unless
    // explicitly enabled by the implementing child class. NOTE: these values
    // aren't used to control Mac fade out - that happens in ScrollAnimatorMac.
    style->fade_out_delay = base::TimeDelta();
    style->fade_out_duration = base::TimeDelta();
    // The other fields in this struct are used only on Android to draw solid
    // color scrollbars. On other platforms the scrollbars are painted in
    // NativeTheme so these fields are unused in non-Android WebThemeEngines.
  }

  virtual bool IsFluentScrollbarEnabled() const { return false; }
  virtual bool IsFluentOverlayScrollbarEnabled() const { return false; }
  virtual int GetPaintedScrollbarTrackInset() const { return 0; }
  virtual gfx::Insets GetScrollbarSolidColorThumbInsets(Part) const {
    return gfx::Insets();
  }
  // Returns the color the thumb should be painted in based on the state and
  // extra params. This is called only if the theme uses solid color thumbs.
  virtual SkColor4f GetScrollbarThumbColor(State,
                                           const ExtraParams*,
                                           const ui::ColorProvider*) const {
    NOTREACHED();
  }

  // Paint the given the given theme part.
  virtual void Paint(
      cc::PaintCanvas*,
      Part,
      State,
      const gfx::Rect&,
      const ExtraParams*,
      blink::mojom::ColorScheme,
      bool in_forced_colors,
      const ui::ColorProvider*,
      const std::optional<SkColor>& accent_color = std::nullopt) {}

  virtual std::optional<SkColor> GetAccentColor() const { return std::nullopt; }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_THEME_ENGINE_H_
