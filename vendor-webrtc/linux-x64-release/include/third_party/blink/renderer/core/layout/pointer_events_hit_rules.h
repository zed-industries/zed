/*
    Copyright (C) 2007 Rob Buis <buis@kde.org>

    This library is free software; you can redistribute it and/or
    modify it under the terms of the GNU Library General Public
    License as published by the Free Software Foundation; either
    version 2 of the License, or (at your option) any later version.

    This library is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
    Library General Public License for more details.

    You should have received a copy of the GNU Library General Public License
    aint with this library; see the file COPYING.LIB.  If not, write to
    the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
    Boston, MA 02110-1301, USA.
*/

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_POINTER_EVENTS_HIT_RULES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_POINTER_EVENTS_HIT_RULES_H_

#include "third_party/blink/renderer/core/layout/hit_test_request.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class PointerEventsHitRules {
  STACK_ALLOCATED();

 public:
  enum EHitTesting {
    kSvgImageHitTesting,
    kSvgGeometryHitTesting,
    kSvgTextHitTesting
  };

  PointerEventsHitRules(EHitTesting, const HitTestRequest&, EPointerEvents);

  unsigned require_visible : 1;
  unsigned require_fill : 1;
  unsigned require_stroke : 1;
  unsigned can_hit_stroke : 1;
  unsigned can_hit_fill : 1;
  unsigned can_hit_bounding_box : 1;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_POINTER_EVENTS_HIT_RULES_H_
