/*
 * This file is part of the theme implementation for form controls in WebCore.
 *
 * Copyright (C) 2005 Apple Computer, Inc.
 * Copyright (C) 2008, 2009 Google, Inc.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_THEME_MAC_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_THEME_MAC_H_

#import "third_party/blink/renderer/core/layout/layout_theme.h"
#import "third_party/blink/renderer/core/layout/layout_theme_default.h"

namespace blink {

class File;

class LayoutThemeMac final : public LayoutThemeDefault {
 public:
  static scoped_refptr<LayoutTheme> Create() {
    return base::AdoptRef(new LayoutThemeMac());
  }

  Color PlatformActiveSelectionBackgroundColor(
      mojom::blink::ColorScheme color_scheme) const override;
  Color PlatformInactiveSelectionBackgroundColor(
      mojom::blink::ColorScheme color_scheme) const override;
  Color PlatformActiveSelectionForegroundColor(
      mojom::blink::ColorScheme color_scheme) const override;
  Color PlatformSpellingMarkerUnderlineColor() const override;
  Color PlatformGrammarMarkerUnderlineColor() const override;
  Color FocusRingColor(mojom::blink::ColorScheme color_scheme) const override;
  String DisplayNameForFile(const File& file) const override;
  bool PopsMenuByArrowKeys() const override { return true; }
  bool PopsMenuByReturnKey() const override { return false; }
  bool SupportsSelectionForegroundColors() const override { return false; }
  bool IsAccentColorCustomized(
      mojom::blink::ColorScheme color_scheme) const override;
  Color GetSystemAccentColor(
      mojom::blink::ColorScheme color_scheme) const override;
  Color SystemHighlightFromColorProvider(
      mojom::blink::ColorScheme color_scheme,
      const ui::ColorProvider* color_provider) const override;

 protected:
  // Controls color values returned from FocusRingColor().
  bool UsesTestModeFocusRingColor() const;
  Color GetCustomFocusRingColor(mojom::blink::ColorScheme color_scheme) const;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_THEME_MAC_H_
