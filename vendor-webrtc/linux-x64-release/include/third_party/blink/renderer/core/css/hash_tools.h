/*
 * Copyright (C) 2010 Andras Becsi <abecsi@inf.u-szeged.hu>, University of
 * Szeged
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
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_HASH_TOOLS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_HASH_TOOLS_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

static constexpr int kNotKnownExposedPropertyBit = 0x8000U;

struct Property {
  DISALLOW_NEW();
  int name_offset;
  // NOTE: kNotKnownExposedPropertyBit is set unless property is known to
  // be web-exposed. There is an assert in make_css_property_names.py
  // that verifies that no property IDs actually have this bit already.
  int id_and_exposed_bit;
};

struct Value {
  DISALLOW_NEW();
  int name_offset;
  int id;
};

const Property* FindProperty(const char* str, unsigned len);
const Value* FindValue(const char* str, unsigned len);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_HASH_TOOLS_H_
