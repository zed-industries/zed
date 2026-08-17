/*
 * Copyright (C) 1997 Martin Jones (mjones@kde.org)
 *           (C) 1997 Torben Weis (weis@kde.org)
 *           (C) 1998 Waldo Bastian (bastian@kde.org)
 *           (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 * Copyright (C) 2003, 2004, 2005, 2006, 2007 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_TABLE_SECTION_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_TABLE_SECTION_ELEMENT_H_

#include "third_party/blink/renderer/core/html/html_table_part_element.h"

namespace blink {

class ExceptionState;

class HTMLTableSectionElement final : public HTMLTablePartElement {
  DEFINE_WRAPPERTYPEINFO();

 public:

  HTMLTableSectionElement(const QualifiedName& tag_name, Document&);

  HTMLElement* insertRow(int index, ExceptionState&);
  void deleteRow(int index, ExceptionState&);

  HTMLCollection* rows();

  bool HasNonInBodyInsertionMode() const override { return true; }

 private:
  const CSSPropertyValueSet* AdditionalPresentationAttributeStyle() override;
};

inline bool IsHTMLTableSectionElement(const HTMLElement& element) {
  return element.HasTagName(html_names::kTbodyTag) ||
         element.HasTagName(html_names::kTfootTag) ||
         element.HasTagName(html_names::kTheadTag);
}

template <>
struct DowncastTraits<HTMLTableSectionElement> {
  static bool AllowFrom(const HTMLElement& element) {
    return IsHTMLTableSectionElement(element);
  }
  static bool AllowFrom(const HTMLElement* element) {
    return element && IsHTMLTableSectionElement(*element);
  }
  static bool AllowFrom(const Node& node) {
    auto* html_element = DynamicTo<HTMLElement>(node);
    return html_element && IsHTMLTableSectionElement(*html_element);
  }
  static bool AllowFrom(const Node* node) {
    return node && IsA<HTMLTableSectionElement>(*node);
  }
  static bool AllowFrom(const Element* element) {
    return element && IsA<HTMLTableSectionElement>(*element);
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_TABLE_SECTION_ELEMENT_H_
