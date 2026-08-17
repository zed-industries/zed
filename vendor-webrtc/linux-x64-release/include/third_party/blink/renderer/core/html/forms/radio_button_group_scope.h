/*
 * Copyright (C) 2007, 2008, 2009 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_RADIO_BUTTON_GROUP_SCOPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_RADIO_BUTTON_GROUP_SCOPE_H_

#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"

namespace blink {

class HTMLInputElement;
class RadioButtonGroup;

class RadioButtonGroupScope {
  DISALLOW_NEW();

 public:
  RadioButtonGroupScope();
  void Trace(Visitor*) const;
  void AddButton(HTMLInputElement*);
  void UpdateCheckedState(HTMLInputElement*);
  void RequiredAttributeChanged(HTMLInputElement*);
  void RemoveButton(HTMLInputElement*);
  HTMLInputElement* CheckedButtonForGroup(const AtomicString& group_name) const;
  bool IsInRequiredGroup(HTMLInputElement*) const;
  unsigned GroupSizeFor(const HTMLInputElement*) const;

 private:
  RadioButtonGroup* FindGroupByName(const AtomicString&) const;

  using NameToGroupMap =
      GCedHeapHashMap<AtomicString, Member<RadioButtonGroup>>;
  Member<NameToGroupMap> name_to_group_map_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_RADIO_BUTTON_GROUP_SCOPE_H_
