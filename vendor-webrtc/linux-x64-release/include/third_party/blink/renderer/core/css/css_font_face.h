/*
 * Copyright (C) 2007, 2008 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_FONT_FACE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_FONT_FACE_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_font_face_source.h"
#include "third_party/blink/renderer/core/css/css_segmented_font_face.h"
#include "third_party/blink/renderer/core/css/font_face.h"
#include "third_party/blink/renderer/platform/fonts/segmented_font_data.h"
#include "third_party/blink/renderer/platform/fonts/unicode_range_set.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_deque.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class FontDescription;
class RemoteFontFaceSource;
class SimpleFontData;

class CORE_EXPORT CSSFontFace final : public GarbageCollected<CSSFontFace> {
 public:
  CSSFontFace(FontFace* font_face, HeapVector<UnicodeRange>&& ranges)
      : ranges_(MakeGarbageCollected<UnicodeRangeSet>(std::move(ranges))),
        font_face_(font_face) {
    DCHECK(font_face_);
  }
  CSSFontFace(const CSSFontFace&) = delete;
  CSSFontFace& operator=(const CSSFontFace&) = delete;

  // Front source is the first successfully loaded source.
  const CSSFontFaceSource* FrontSource() const {
    return sources_.empty() ? nullptr : sources_.front().Get();
  }
  FontFace* GetFontFace() const { return font_face_.Get(); }

  const UnicodeRangeSet* Ranges() { return ranges_.Get(); }

  void AddSegmentedFontFace(CSSSegmentedFontFace*);
  void RemoveSegmentedFontFace(CSSSegmentedFontFace*);

  bool IsValid() const { return !sources_.empty(); }
  size_t ApproximateBlankCharacterCount() const;

  void AddSource(CSSFontFaceSource*);
  void SetDisplay(FontDisplay);

  void DidBeginLoad();
  bool FontLoaded(CSSFontFaceSource*);
  bool FallbackVisibilityChanged(RemoteFontFaceSource*);

  const SimpleFontData* GetFontData(const FontDescription&);

  FontFace::LoadStatusType LoadStatus() const {
    return font_face_->LoadStatus();
  }
  bool MaybeLoadFont(const FontDescription&, const String&);
  bool MaybeLoadFont(const FontDescription&, const FontDataForRangeSet&);
  void Load();
  void Load(const FontDescription&);

  // Recalculate the font loading timeline period for the font face.
  // https://drafts.csswg.org/css-fonts-4/#font-display-timeline
  // Returns true if the display period is changed.
  bool UpdatePeriod();

  bool HadBlankText() { return IsValid() && sources_.front()->HadBlankText(); }

  void Trace(Visitor*) const;

 private:
  void SetLoadStatus(FontFace::LoadStatusType);

  HeapHashSet<Member<CSSSegmentedFontFace>> segmented_font_faces_;
  HeapDeque<Member<CSSFontFaceSource>> sources_;
  Member<const UnicodeRangeSet> ranges_;
  Member<FontFace> font_face_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_FONT_FACE_H_
