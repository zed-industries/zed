/*
 * Copyright (C) 2011 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_TEXT_TRACK_CUE_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_TEXT_TRACK_CUE_LIST_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/track/text_track_cue.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class CORE_EXPORT TextTrackCueList final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  TextTrackCueList();

  wtf_size_t length() const;

  TextTrackCue* AnonymousIndexedGetter(wtf_size_t index) const;
  TextTrackCue* getCueById(const AtomicString&) const;

  bool Add(TextTrackCue*);
  bool Remove(TextTrackCue*);

  void RemoveAll();

  void CollectActiveCues(TextTrackCueList&) const;
  void UpdateCueIndex(TextTrackCue*);
  bool IsCueIndexValid(wtf_size_t probe_index) const {
    return probe_index < first_invalid_index_;
  }
  void ValidateCueIndexes();

  void Trace(Visitor*) const override;

 private:
  wtf_size_t FindInsertionIndex(const TextTrackCue*) const;
  void InvalidateCueIndex(wtf_size_t index);
  void Clear();

  HeapVector<Member<TextTrackCue>> list_;
  wtf_size_t first_invalid_index_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_TEXT_TRACK_CUE_LIST_H_
