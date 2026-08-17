/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 * Copyright (C) 2003, 2004, 2005, 2006, 2007, 2008 Apple Inc. All rights
 * reserved.
 * Copyright (C) 2014 Samsung Electronics. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_HTML_FORM_CONTROLS_COLLECTION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_HTML_FORM_CONTROLS_COLLECTION_H_

#include "third_party/blink/renderer/core/html/forms/listed_element.h"
#include "third_party/blink/renderer/core/html/forms/radio_node_list.h"
#include "third_party/blink/renderer/core/html/html_collection.h"
#include "third_party/blink/renderer/core/html/html_element.h"

namespace blink {

class HTMLImageElement;
class V8UnionElementOrRadioNodeList;

// This class is just a big hack to find form elements even in malformed HTML
// elements.  The famous <table><tr><form><td> problem.

class HTMLFormControlsCollection final : public HTMLCollection {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit HTMLFormControlsCollection(ContainerNode&);
  HTMLFormControlsCollection(ContainerNode&, CollectionType);

  ~HTMLFormControlsCollection() override;

  HTMLElement* item(unsigned offset) const {
    return To<HTMLElement>(HTMLCollection::item(offset));
  }

  HTMLElement* namedItem(const AtomicString& name) const override;
  V8UnionElementOrRadioNodeList* namedGetter(const AtomicString& name);

  void Trace(Visitor*) const override;

 private:
  void UpdateIdNameCache() const override;
  void SupportedPropertyNames(Vector<String>& names) override;

  const ListedElement::List& ListedElements() const;
  const HeapVector<Member<HTMLImageElement>>& FormImageElements() const;
  HTMLElement* VirtualItemAfter(Element*) const override;
  void InvalidateCache(Document* old_document = nullptr) const override;

  mutable Member<HTMLElement> cached_element_;
  mutable unsigned cached_element_offset_in_array_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_HTML_FORM_CONTROLS_COLLECTION_H_
