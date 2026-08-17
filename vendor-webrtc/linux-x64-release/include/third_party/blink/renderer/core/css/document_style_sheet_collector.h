/*
 * Copyright (C) 2014 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_DOCUMENT_STYLE_SHEET_COLLECTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_DOCUMENT_STYLE_SHEET_COLLECTOR_H_

#include "third_party/blink/renderer/core/css/active_style_sheets.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class StyleSheet;
class StyleSheetCollection;
class RuleSetDiff;

class DocumentStyleSheetCollector {
  // This class contains references to two on-heap collections, therefore
  // it's unhealthy to have it anywhere but on the stack, where stack
  // scanning will keep them alive.
  STACK_ALLOCATED();

 public:
  friend class ImportedDocumentStyleSheetCollector;

  DocumentStyleSheetCollector(StyleSheetCollection*,
                              HeapVector<Member<StyleSheet>>*);

  void AppendActiveStyleSheet(const ActiveStyleSheet&);
  void AppendSheetForList(StyleSheet*);
  void AppendRuleSetDiff(RuleSetDiff*);

 private:
  StyleSheetCollection* collection_;
  HeapVector<Member<StyleSheet>>* style_sheets_for_style_sheet_list_;
};

class ActiveDocumentStyleSheetCollector final
    : public DocumentStyleSheetCollector {
 public:
  ActiveDocumentStyleSheetCollector(StyleSheetCollection&);
};

class ImportedDocumentStyleSheetCollector final
    : public DocumentStyleSheetCollector {
 public:
  ImportedDocumentStyleSheetCollector(DocumentStyleSheetCollector&,
                                      HeapVector<Member<StyleSheet>>&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_DOCUMENT_STYLE_SHEET_COLLECTOR_H_
