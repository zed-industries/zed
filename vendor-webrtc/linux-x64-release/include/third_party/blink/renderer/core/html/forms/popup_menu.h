/*
 * Copyright (C) 2010 Nokia Corporation and/or its subsidiary(-ies).
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_POPUP_MENU_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_POPUP_MENU_H_

#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

class AXObject;

class PopupMenu : public GarbageCollected<PopupMenu> {
 public:
  virtual ~PopupMenu() = default;
  virtual void Trace(Visitor* visitor) const {}
  enum ShowEventType { kTouch, kOther };
  virtual void Show(ShowEventType type) = 0;
  virtual void Hide() = 0;
  enum UpdateReason {
    kBySelectionChange,
    kByStyleChange,
    kByDOMChange,
  };
  virtual void UpdateFromElement(UpdateReason) = 0;
  virtual void DisconnectClient() = 0;
  virtual AXObject* PopupRootAXObject() const { return nullptr; }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_POPUP_MENU_H_
