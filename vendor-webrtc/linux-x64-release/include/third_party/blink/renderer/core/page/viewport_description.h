/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2001 Dirk Mueller (mueller@kde.org)
 *           (C) 2006 Alexey Proskuryakov (ap@webkit.org)
 * Copyright (C) 2004, 2005, 2006, 2007, 2008 Apple Inc. All rights reserved.
 * Copyright (C) 2008 Torch Mobile Inc. All rights reserved.
 * (http://www.torchmobile.com/)
 * Copyright (C) 2010 Nokia Corporation and/or its subsidiary(-ies)
 * Copyright (C) 2012-2013 Intel Corporation. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_VIEWPORT_DESCRIPTION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_VIEWPORT_DESCRIPTION_H_

#include <optional>

#include "third_party/blink/public/mojom/page/display_cutout.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/frame/page_scale_constraints.h"
#include "third_party/blink/renderer/platform/geometry/length.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "ui/base/ime/mojom/virtual_keyboard_types.mojom-blink.h"
#include "ui/gfx/geometry/size_f.h"

namespace blink {

class LocalFrame;

struct CORE_EXPORT ViewportDescription {
  DISALLOW_NEW();

  enum Type {
    // These are ordered in increasing importance.
    kUserAgentStyleSheet,
    kHandheldFriendlyMeta,
    kMobileOptimizedMeta,
    kViewportMeta
  } type;

  ui::mojom::blink::VirtualKeyboardMode virtual_keyboard_mode =
      ui::mojom::blink::VirtualKeyboardMode::kUnset;

  // Enums used to record the kind of viewport in the Viewport.MetaTagType
  // histogram. These must match the enums in histograms.xml and existing
  // entries must not be changed.
  enum class ViewportUMAType : int {
    kNoViewportTag = 0,
    kDeviceWidth = 1,
    kConstantWidth = 2,
    kMetaWidthOther = 3,
    kMetaHandheldFriendly = 4,
    kMetaMobileOptimized = 5,
    kXhtmlMobileProfile = 6,

    kMaxValue = kXhtmlMobileProfile,
  };

  constexpr static float kValueAuto = -1.;
  constexpr static float kValueDeviceWidth = -2.;
  constexpr static float kValueDeviceHeight = -3.;
  constexpr static float kValuePortrait = -4.;
  constexpr static float kValueLandscape = -5.;
  constexpr static float kValueDeviceDPI = -6.;
  constexpr static float kValueLowDPI = -7.;
  constexpr static float kValueMediumDPI = -8.;
  constexpr static float kValueHighDPI = -9.;
  constexpr static float kValueExtendToZoom = -10.;

  ViewportDescription(Type type = kUserAgentStyleSheet)
      : type(type),
        zoom(kValueAuto),
        min_zoom(kValueAuto),
        max_zoom(kValueAuto),
        user_zoom(true),
        orientation(kValueAuto),
        deprecated_target_density_dpi(kValueAuto),
        zoom_is_explicit(false),
        min_zoom_is_explicit(false),
        max_zoom_is_explicit(false),
        user_zoom_is_explicit(false) {}

  // All arguments are in CSS units.
  PageScaleConstraints Resolve(const gfx::SizeF& initial_viewport_size,
                               const Length& legacy_fallback_width) const;

  // If the type is kFixed, these Length values (i.e., |min_width|,
  // |max_width|, |min_height|, and |max_height|) must be in physical pixel
  // scale.
  Length min_width;
  Length max_width;
  Length min_height;
  Length max_height;
  float zoom;
  float min_zoom;
  float max_zoom;
  bool user_zoom;
  float orientation;
  float deprecated_target_density_dpi;  // Only used for Android WebView

  // Whether the computed value was explicitly specified rather than being
  // inferred.
  bool zoom_is_explicit;
  bool min_zoom_is_explicit;
  bool max_zoom_is_explicit;
  bool user_zoom_is_explicit;

  mojom::ViewportFit GetViewportFit() const {
    return viewport_fit_.value_or(mojom::ViewportFit::kAuto);
  }
  void SetViewportFit(mojom::ViewportFit value) { viewport_fit_ = value; }

  bool operator==(const ViewportDescription& other) const {
    // Used for figuring out whether to reset the viewport or not,
    // thus we are not taking type into account.
    return min_width == other.min_width && max_width == other.max_width &&
           min_height == other.min_height && max_height == other.max_height &&
           zoom == other.zoom && min_zoom == other.min_zoom &&
           max_zoom == other.max_zoom && user_zoom == other.user_zoom &&
           orientation == other.orientation &&
           deprecated_target_density_dpi ==
               other.deprecated_target_density_dpi &&
           zoom_is_explicit == other.zoom_is_explicit &&
           min_zoom_is_explicit == other.min_zoom_is_explicit &&
           max_zoom_is_explicit == other.max_zoom_is_explicit &&
           user_zoom_is_explicit == other.user_zoom_is_explicit &&
           virtual_keyboard_mode == other.virtual_keyboard_mode &&
           viewport_fit_ == other.viewport_fit_;
  }

  bool operator!=(const ViewportDescription& other) const {
    return !(*this == other);
  }

  bool IsLegacyViewportType() const {
    return type >= kHandheldFriendlyMeta && type <= kViewportMeta;
  }
  bool IsMetaViewportType() const { return type == kViewportMeta; }
  bool IsSpecifiedByAuthor() const { return type != kUserAgentStyleSheet; }

  // Reports UMA stat on whether the page is considered mobile or desktop and
  // what kind of mobile it is. Applies only to Android, must only be called
  // once per page load.
  void ReportMobilePageStats(const LocalFrame*) const;

 private:
  enum class Direction { kHorizontal, kVertical };
  static float ResolveViewportLength(const Length&,
                                     const gfx::SizeF& initial_viewport_size,
                                     Direction);

  // Optional is used to identify if |viewport_fit_| has been explicitly set.
  // This is because a Document will have multiple ViewportDescriptions are
  // which one that will be used is dependent on whether any values have been
  // explicitly set.
  std::optional<mojom::ViewportFit> viewport_fit_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_VIEWPORT_DESCRIPTION_H_
