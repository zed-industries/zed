// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FENCED_FRAME_DOCUMENT_FENCED_FRAMES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FENCED_FRAME_DOCUMENT_FENCED_FRAMES_H_

#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class Document;
class HTMLFencedFrameElement;

class DocumentFencedFrames final
    : public GarbageCollected<DocumentFencedFrames>,
      public Supplement<Document> {
 public:
  static const char kSupplementName[];

  // Returns the supplement that stores the fenced frame elements
  // that are associated with the document. Returns nullptr if
  // `GetOrCreate` has yet been called yet.
  static DocumentFencedFrames* Get(Document&);

  // Like `Get` but if the supplement has not been created yet it
  // will be created.
  static DocumentFencedFrames& GetOrCreate(Document&);

  explicit DocumentFencedFrames(Document&);

  void RegisterFencedFrame(HTMLFencedFrameElement*);
  void DeregisterFencedFrame(HTMLFencedFrameElement*);

  // Retrieves all fenced frames in the document.
  const HeapVector<Member<HTMLFencedFrameElement>>& GetFencedFrames() const {
    return fenced_frames_;
  }

  void Trace(Visitor*) const override;

 private:
  HeapVector<Member<HTMLFencedFrameElement>> fenced_frames_;
};  // class DocumentFencedFrames

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FENCED_FRAME_DOCUMENT_FENCED_FRAMES_H_
