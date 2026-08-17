/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2000 Simon Hausmann <hausmann@kde.org>
 * Copyright (C) 2010 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_QUOTE_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_QUOTE_ELEMENT_H_

#include "third_party/blink/renderer/core/html/html_element.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class ComputedStyleBuilder;

class HTMLQuoteElement final : public HTMLElement {
  DEFINE_WRAPPERTYPEINFO();

 public:
  HTMLQuoteElement(const QualifiedName&, Document&);
  void AdjustPseudoStyleLocale(ComputedStyleBuilder& pseudo_style_builder);

 private:
  bool IsURLAttribute(const Attribute&) const override;
  bool HasLegalLinkAttribute(const QualifiedName&) const override;
};

inline bool IsHTMLQuoteElement(const HTMLElement& element) {
  return element.HasTagName(html_names::kQTag) ||
         element.HasTagName(html_names::kBlockquoteTag);
}

template <>
struct DowncastTraits<HTMLQuoteElement> {
  static bool AllowFrom(const HTMLElement& element) {
    return IsHTMLQuoteElement(element);
  }
  static bool AllowFrom(const HTMLElement* element) {
    return element && IsHTMLQuoteElement(*element);
  }
  static bool AllowFrom(const Node& node) {
    auto* html_element = DynamicTo<HTMLElement>(node);
    return html_element && IsHTMLQuoteElement(*html_element);
  }
  static bool AllowFrom(const Node* node) {
    return node && IsA<HTMLQuoteElement>(*node);
  }
  static bool AllowFrom(const Element* element) {
    return element && IsA<HTMLQuoteElement>(*element);
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_QUOTE_ELEMENT_H_
