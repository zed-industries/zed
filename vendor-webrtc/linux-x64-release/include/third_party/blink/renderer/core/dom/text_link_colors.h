/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2001 Dirk Mueller (mueller@kde.org)
 *           (C) 2006 Alexey Proskuryakov (ap@webkit.org)
 * Copyright (C) 2004, 2005, 2006, 2007, 2008, 2009, 2010, 2012 Apple Inc. All
 * rights reserved.
 * Copyright (C) 2008, 2009 Torch Mobile Inc. All rights reserved.
 * (http://www.torchmobile.com/)
 * Copyright (C) 2010 Nokia Corporation and/or its subsidiary(-ies)
 * Copyright (C) 2013 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_TEXT_LINK_COLORS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_TEXT_LINK_COLORS_H_

#include "third_party/blink/public/mojom/frame/color_scheme.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/graphics/color.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class CORE_EXPORT TextLinkColors {
  DISALLOW_NEW();

 public:
  TextLinkColors();
  TextLinkColors(const TextLinkColors&) = delete;
  TextLinkColors& operator=(const TextLinkColors&) = delete;

  void SetTextColor(const Color& color);
  Color TextColor() const;
  Color TextColor(mojom::blink::ColorScheme color_scheme) const;

  const Color& LinkColor() const;
  const Color& LinkColor(mojom::blink::ColorScheme color_scheme) const;
  const Color& VisitedLinkColor() const;
  const Color& VisitedLinkColor(mojom::blink::ColorScheme color_scheme) const;
  const Color& ActiveLinkColor() const;
  const Color& ActiveLinkColor(mojom::blink::ColorScheme color_scheme) const;
  void SetLinkColor(const Color& color);
  void SetVisitedLinkColor(const Color& color);
  void SetActiveLinkColor(const Color& color);
  void ResetLinkColor() { has_custom_link_color_ = false; }
  void ResetVisitedLinkColor() { has_custom_visited_link_color_ = false; }
  void ResetActiveLinkColor() { has_custom_active_link_color_ = false; }

 private:
  Color text_color_;
  Color link_color_;
  Color visited_link_color_;
  Color active_link_color_;

  bool has_custom_text_color_{false};
  bool has_custom_link_color_{false};
  bool has_custom_visited_link_color_{false};
  bool has_custom_active_link_color_{false};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_TEXT_LINK_COLORS_H_
