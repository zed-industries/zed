/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 2000 Simon Hausmann <hausmann@kde.org>
 * Copyright (C) 2004, 2005, 2006, 2008, 2009, 2010, 2012 Apple Inc.
 *               All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_EMBEDDED_OBJECT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_EMBEDDED_OBJECT_H_

#include "third_party/blink/renderer/core/layout/layout_embedded_content.h"

namespace blink {

// LayoutObject for embeds and objects, often, but not always, rendered via
// plugins. For example, <embed src="foo.html"> does not invoke a plugin.
class LayoutEmbeddedObject final : public LayoutEmbeddedContent {
 public:
  LayoutEmbeddedObject(HTMLFrameOwnerElement*);
  ~LayoutEmbeddedObject() override;

  enum PluginAvailability {
    kPluginAvailable,
    kPluginMissing,
    kPluginBlockedByContentSecurityPolicy,
  };
  void SetPluginAvailability(PluginAvailability);
  bool ShowsUnavailablePluginIndicator() const;

  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutEmbeddedObject";
  }

  const String& UnavailablePluginReplacementText() const {
    NOT_DESTROYED();
    return unavailable_plugin_replacement_text_;
  }

 private:
  void PaintReplaced(const PaintInfo&,
                     const PhysicalOffset& paint_offset) const final;

  void UpdateAfterLayout() final;

  bool IsEmbeddedObject() const final {
    NOT_DESTROYED();
    return true;
  }
  PhysicalNaturalSizingInfo GetNaturalDimensions() const override;
  bool ShouldApplyObjectViewBox() const override {
    NOT_DESTROYED();
    return false;
  }

  PluginAvailability plugin_availability_ = kPluginAvailable;
  String unavailable_plugin_replacement_text_;
};

template <>
struct DowncastTraits<LayoutEmbeddedObject> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsEmbeddedObject();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_EMBEDDED_OBJECT_H_
