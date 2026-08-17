/*
 * Copyright (C) 2011 Google Inc.  All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_IDENTIFIERS_FACTORY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_IDENTIFIERS_FACTORY_H_

#include "base/unguessable_token.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/weak_identifier_map.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class DocumentLoader;
class ExecutionContext;
class Frame;
class LocalFrame;
class Node;
class InspectedFrames;
class CSSStyleSheet;

DECLARE_WEAK_IDENTIFIER_MAP(CSSStyleSheet);

class CORE_EXPORT IdentifiersFactory {
  STATIC_ONLY(IdentifiersFactory);

 public:
  static String CreateIdentifier();

  static String RequestId(ExecutionContext*, uint64_t identifier);

  // Same as above, but used when it's inconvenient to get the ExecutionContext
  // or it does not exist. Prefer using RequestId with ExecutionContext. (See
  // above.)
  static String RequestId(DocumentLoader*, uint64_t identifier);

  // Same as RequestId, but can only be used on reuquests that are guaranteed
  // to be subresources, not main resource.
  static String SubresourceRequestId(uint64_t identifier);

  // Returns embedder-provided frame token that is consistent across processes
  // and can be used for request / call attribution to the context frame.
  static const String& FrameId(Frame*);
  static LocalFrame* FrameById(InspectedFrames*, const String&);

  static String LoaderId(DocumentLoader*);

  static String IdFromToken(const base::UnguessableToken&);

  static int IntIdForNode(Node* node);

  // The IdentifiersFactory::IdForCSSStyleSheet() function generates unique
  // IDs for CSS style sheets. These IDs are used to identify the style sheets
  // within the InspectorStyleSheet, InspectorCSSAgent, and ElementRuleCollector
  // classes. If the style sheet has a CSSStyleSheet object, its ID will have a
  // "style-sheet-{process_id}-" prefix. If it lacks a CSSStyleSheet object (a
  // UA stylesheet), its ID will be "ua-style-sheet".
  static String IdForCSSStyleSheet(const CSSStyleSheet*);

 private:
  static String AddProcessIdPrefixTo(uint64_t id);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_IDENTIFIERS_FACTORY_H_
