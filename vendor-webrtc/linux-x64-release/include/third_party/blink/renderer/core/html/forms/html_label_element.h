/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2000 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2004, 2005, 2006, 2007, 2010 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_HTML_LABEL_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_HTML_LABEL_ELEMENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/simulated_click_options.h"
#include "third_party/blink/renderer/core/html/html_element.h"

namespace blink {

class CORE_EXPORT HTMLLabelElement final : public HTMLElement {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit HTMLLabelElement(Document&);

  HTMLElement* controlForBinding() const;
  HTMLElement* Control() const;
  HTMLElement* formForBinding() const override;

  bool WillRespondToMouseClickEvents() override;

 private:
  bool IsInInteractiveContent(Node*) const;

  bool IsInteractiveContent() const override;
  void AccessKeyAction(SimulatedClickCreationScope creation_scope) override;

  // Overridden to update the hover/active state of the corresponding control.
  void SetActive(bool active) override;
  void SetHovered(bool hovered) override;

  // Overridden to either click() or focus() the corresponding control.
  void DefaultEventHandler(Event&) override;
  void DefaultEventHandlerInternal(Event&);
  bool HasActivationBehavior() const override;

  void Focus(const FocusParams&) override;

  bool processing_click_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_HTML_LABEL_ELEMENT_H_
