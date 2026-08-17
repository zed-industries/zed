/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2000 Simon Hausmann <hausmann@kde.org>
 * Copyright (C) 2004, 2006, 2009, 2010 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_BODY_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_BODY_ELEMENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/frame/window_event_handlers.h"
#include "third_party/blink/renderer/core/html/html_element.h"

namespace blink {

class Document;

class CORE_EXPORT HTMLBodyElement final : public HTMLElement,
                                          public WindowEventHandlers {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit HTMLBodyElement(Document&);
  ~HTMLBodyElement() override;

  // HTMLElement override
  bool IsHTMLBodyElement() const override { return true; }

  DEFINE_WINDOW_ATTRIBUTE_EVENT_LISTENER(blur, kBlur)
  DEFINE_WINDOW_ATTRIBUTE_EVENT_LISTENER(error, kError)
  DEFINE_WINDOW_ATTRIBUTE_EVENT_LISTENER(focus, kFocus)
  DEFINE_WINDOW_ATTRIBUTE_EVENT_LISTENER(load, kLoad)
  DEFINE_WINDOW_ATTRIBUTE_EVENT_LISTENER(resize, kResize)
  DEFINE_WINDOW_ATTRIBUTE_EVENT_LISTENER(scroll, kScroll)
  DEFINE_WINDOW_ATTRIBUTE_EVENT_LISTENER(orientationchange, kOrientationchange)

 private:
  void ParseAttribute(const AttributeModificationParams&) override;
  bool IsPresentationAttribute(const QualifiedName&) const override;
  void CollectStyleForPresentationAttribute(
      const QualifiedName&,
      const AtomicString&,
      HeapVector<CSSPropertyValue, 8>&) override;

  InsertionNotificationRequest InsertedInto(ContainerNode&) override;
  void RemovedFrom(ContainerNode&) override;
  void DidNotifySubtreeInsertionsToDocument() override;

  bool IsURLAttribute(const Attribute&) const override;
  bool HasLegalLinkAttribute(const QualifiedName&) const override;

  Document& GetDocumentForWindowEventHandler() const override {
    return GetDocument();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_BODY_ELEMENT_H_
