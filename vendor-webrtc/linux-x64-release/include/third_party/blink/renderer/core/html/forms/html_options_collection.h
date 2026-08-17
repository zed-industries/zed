/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2000 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2004, 2005, 2006, 2007, 2008 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_HTML_OPTIONS_COLLECTION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_HTML_OPTIONS_COLLECTION_H_

#include "third_party/blink/renderer/core/html/forms/html_data_list_element.h"
#include "third_party/blink/renderer/core/html/forms/html_opt_group_element.h"
#include "third_party/blink/renderer/core/html/forms/html_option_element.h"
#include "third_party/blink/renderer/core/html/forms/html_select_element.h"
#include "third_party/blink/renderer/core/html/html_collection.h"
#include "third_party/blink/renderer/platform/bindings/v8_binding.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class ExceptionState;
class V8UnionHTMLElementOrLong;
class V8UnionHTMLOptGroupElementOrHTMLOptionElement;

class HTMLOptionsCollection final : public HTMLCollection {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit HTMLOptionsCollection(ContainerNode&);
  HTMLOptionsCollection(ContainerNode&, CollectionType);

  HTMLOptionElement* item(unsigned offset) const {
    return To<HTMLOptionElement>(HTMLCollection::item(offset));
  }

  void add(const V8UnionHTMLOptGroupElementOrHTMLOptionElement* element,
           const V8UnionHTMLElementOrLong* before,
           ExceptionState& exception_state);
  void remove(int index);

  int selectedIndex() const;
  void setSelectedIndex(int);

  void setLength(unsigned, ExceptionState&);
  IndexedPropertySetterResult AnonymousIndexedSetter(unsigned,
                                                     HTMLOptionElement*,
                                                     ExceptionState&);

  bool ElementMatches(const HTMLElement&) const;

 private:
  void SupportedPropertyNames(Vector<String>& names) override;
};

template <>
struct DowncastTraits<HTMLOptionsCollection> {
  static bool AllowFrom(const LiveNodeListBase& collection) {
    return collection.GetType() == kSelectOptions;
  }
};

inline bool HTMLOptionsCollection::ElementMatches(
    const HTMLElement& element) const {
  if (!IsA<HTMLOptionElement>(element)) {
    return false;
  }
  Node* parent = element.parentNode();
  if (!parent) {
    return false;
  }
  if (parent == &RootNode()) {
    return true;
  }
  if (IsA<HTMLOptGroupElement>(*parent) &&
      parent->parentNode() == &RootNode()) {
    return true;
  }
  if (HTMLSelectElement::SelectParserRelaxationEnabled(&ownerNode())) {
    // If there is another <select> in between RootNode and element, then
    // RootNode should not include element in its options.
    return To<HTMLOptionElement>(element).OwnerSelectElement() == RootNode();
  }
  return false;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_HTML_OPTIONS_COLLECTION_H_
