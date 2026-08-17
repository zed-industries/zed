/*
 * Copyright (C) 2004, 2005, 2006, 2007, 2008, 2009 Apple Inc. All rights
 * reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SERIALIZERS_MARKUP_FORMATTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SERIALIZERS_MARKUP_FORMATTER_H_

#include "third_party/blink/renderer/core/editing/editing_strategy.h"
#include "third_party/blink/renderer/core/editing/serializers/serialization.h"
#include "third_party/blink/renderer/platform/wtf/text/string_builder.h"

namespace blink {

class Attribute;
class DocumentType;
class Element;
class Node;

enum EntityMask {
  kEntityAmp = 0x0001,
  kEntityLt = 0x0002,
  kEntityGt = 0x0004,
  kEntityQuot = 0x0008,
  kEntityNbsp = 0x0010,
  kEntityTab = 0x0020,
  kEntityLineFeed = 0x0040,
  kEntityCarriageReturn = 0x0080,

  // Non-breaking space needs to be escaped in innerHTML for compatibility
  // reasons. See http://trac.webkit.org/changeset/32879. However, we cannot do
  // this in an XML document because it does not have the entity reference
  // defined (see bug 19215).
  kEntityMaskInCDATA = 0,
  kEntityMaskInPCDATA = kEntityAmp | kEntityLt | kEntityGt,
  kEntityMaskInHTMLPCDATA = kEntityMaskInPCDATA | kEntityNbsp,
  kEntityMaskInAttributeValue = kEntityAmp | kEntityQuot | kEntityLt |
                                kEntityGt | kEntityTab | kEntityLineFeed |
                                kEntityCarriageReturn,
  kEntityMaskInHTMLAttributeValue = kEntityAmp | kEntityQuot | kEntityNbsp,
  // Entity mask for an experiment with escaping "<" and ">" in attributes.
  // See: crbug.com/1175016
  kEntityExperimentalMaskInHTMLAttributeValue =
      kEntityMaskInHTMLAttributeValue | kEntityLt | kEntityGt,
};

enum class SerializationType { kHTML, kXML };

class MarkupFormatter final {
  STACK_ALLOCATED();

 public:
  static void AppendAttributeValue(StringBuilder&,
                                   const String&,
                                   bool,
                                   const Document&);
  static void AppendAttributeAsHTML(StringBuilder& result,
                                    const Attribute& attribute,
                                    const String& value,
                                    const Document& node);
  static void AppendAttributeAsXMLWithoutNamespace(StringBuilder& result,
                                                   const Attribute& attribute,
                                                   const String& value,
                                                   const Document& document);
  static void AppendAttribute(StringBuilder& result,
                              const AtomicString& prefix,
                              const AtomicString& local_name,
                              const String& value,
                              bool document_is_html,
                              const Document& document);
  static void AppendCDATASection(StringBuilder&, const String&);
  static void AppendCharactersReplacingEntities(StringBuilder& result,
                                                const StringView& source,
                                                EntityMask entity_mask);
  static void AppendComment(StringBuilder&, const String&);
  static void AppendDocumentType(StringBuilder&, const DocumentType&);
  static void AppendProcessingInstruction(StringBuilder&,
                                          const String& target,
                                          const String& data);
  static void AppendXMLDeclaration(StringBuilder&, const Document&);

  MarkupFormatter(AbsoluteURLs, SerializationType);
  MarkupFormatter(const MarkupFormatter&) = delete;
  MarkupFormatter& operator=(const MarkupFormatter&) = delete;

  void AppendStartMarkup(StringBuilder&, const Node&);
  void AppendEndMarkup(StringBuilder&, const Element&);
  void AppendEndMarkup(StringBuilder& result,
                       const Element& element,
                       const AtomicString& prefix,
                       const AtomicString& local_name);

  bool SerializeAsHTML() const;

  void AppendText(StringBuilder&, const Text&);
  // Serialize '<' and the element name.
  void AppendStartTagOpen(StringBuilder&, const Element&);
  void AppendStartTagOpen(StringBuilder& result,
                          const AtomicString& prefix,
                          const AtomicString& local_name);
  // Serialize '>' or '/>'
  void AppendStartTagClose(StringBuilder&, const Element&);

  EntityMask EntityMaskForText(const Text&) const;
  bool ShouldSelfClose(const Element&) const;
  String ResolveURLIfNeeded(const Element&, const Attribute& attribute) const;

 private:
  const AbsoluteURLs resolve_urls_method_;
  SerializationType serialization_type_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SERIALIZERS_MARKUP_FORMATTER_H_
