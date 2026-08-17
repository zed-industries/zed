/*
 * Copyright (C) 2010 Google, Inc. All Rights Reserved.
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
 * THIS SOFTWARE IS PROVIDED BY GOOGLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL GOOGLE INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_DOCUMENT_TIMING_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_DOCUMENT_TIMING_H_

#include "base/time/time.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class Document;
class LocalFrame;

// The values need to outlive DocumentTiming for metrics reporting.
struct DocumentTimingValues final
    : public GarbageCollected<DocumentTimingValues> {
  base::TimeTicks dom_loading;
  base::TimeTicks dom_interactive;
  base::TimeTicks dom_content_loaded_event_start;
  base::TimeTicks dom_content_loaded_event_end;
  base::TimeTicks dom_complete;
  void Trace(Visitor*) const {}
};

class DocumentTiming final {
  DISALLOW_NEW();

 public:
  explicit DocumentTiming(Document&);

  void MarkDomLoading();
  void MarkDomInteractive();
  void MarkDomContentLoadedEventStart();
  void MarkDomContentLoadedEventEnd();
  void MarkDomComplete();
  DocumentTimingValues* GetDocumentTimingValues() const {
    return document_timing_values_.Get();
  }

  // These return monotonically-increasing time.
  base::TimeTicks DomLoading() const {
    return document_timing_values_->dom_loading;
  }
  base::TimeTicks DomInteractive() const {
    return document_timing_values_->dom_interactive;
  }
  base::TimeTicks DomContentLoadedEventStart() const {
    return document_timing_values_->dom_content_loaded_event_start;
  }
  base::TimeTicks DomContentLoadedEventEnd() const {
    return document_timing_values_->dom_content_loaded_event_end;
  }
  base::TimeTicks DomComplete() const {
    return document_timing_values_->dom_complete;
  }

  void Trace(Visitor*) const;

 private:
  LocalFrame* GetFrame() const;
  void NotifyDocumentTimingChanged();

  Member<Document> document_;
  Member<DocumentTimingValues> document_timing_values_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_DOCUMENT_TIMING_H_
