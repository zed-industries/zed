/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 * Copyright (C) 2004, 2006, 2007, 2008, 2009 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_DOCUMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_DOCUMENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/dom/document_init.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_client.h"
#include "third_party/blink/renderer/platform/wtf/hash_counted_set.h"

namespace blink {

class CORE_EXPORT HTMLDocument : public Document {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit HTMLDocument(
      const DocumentInit&,
      DocumentClassFlags extended_document_classes = DocumentClassFlags());
  ~HTMLDocument() override;

  static HTMLDocument* CreateForTest(ExecutionContext& execution_context);

  void AddNamedItem(const AtomicString& name);
  void RemoveNamedItem(const AtomicString& name);
  bool HasNamedItem(const AtomicString& name);

  static bool IsCaseSensitiveAttribute(const QualifiedName&);

  Document* CloneDocumentWithoutChildren() const final;

 private:
  HashCountedSet<AtomicString> named_item_counts_;
};

inline bool HTMLDocument::HasNamedItem(const AtomicString& name) {
  return named_item_counts_.Contains(name);
}

template <>
struct DowncastTraits<HTMLDocument> {
  static bool AllowFrom(const Document& document) {
    return document.IsHTMLDocument();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_DOCUMENT_H_
