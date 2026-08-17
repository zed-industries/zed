/*
 * Copyright (C) 2014 Samsung Electronics. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *  * Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *  * Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE COPYRIGHT OWNER OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_XML_DOCUMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_XML_DOCUMENT_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class XMLDocument final : public Document {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static XMLDocument* CreateXHTML(const DocumentInit& initializer) {
    return MakeGarbageCollected<XMLDocument>(
        initializer, DocumentClassFlags({DocumentClass::kXHTML}));
  }

  static XMLDocument* CreateSVG(const DocumentInit& initializer) {
    return MakeGarbageCollected<XMLDocument>(
        initializer, DocumentClassFlags({DocumentClass::kSVG}));
  }

  explicit XMLDocument(
      const DocumentInit&,
      DocumentClassFlags extended_document_classes = DocumentClassFlags());
};

template <>
struct DowncastTraits<XMLDocument> {
  static bool AllowFrom(const Document& document) {
    return document.IsXMLDocument();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_XML_DOCUMENT_H_
