// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASESSION_CHAPTER_INFORMATION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASESSION_CHAPTER_INFORMATION_H_

#include "third_party/blink/renderer/bindings/modules/v8/v8_chapter_information_init.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_media_image.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ChapterInformationInit;
class ExceptionState;
class ScriptState;

// Implementation of `ChapterInformation` interface from the Media Session API.
class MODULES_EXPORT ChapterInformation final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static ChapterInformation* From(ScriptState* script_state,
                                  const ChapterInformationInit* chapter,
                                  ExceptionState& exception_state);

  static ChapterInformation* Create(
      ScriptState* script_state,
      const String& title,
      const double& start_time,
      const HeapVector<Member<MediaImage>>& artwork,
      ExceptionState& exception_state);

  ChapterInformation(ScriptState* script_state,
                     const String& title,
                     const double& start_time,
                     const HeapVector<Member<MediaImage>>& artwork,
                     ExceptionState& exception_state);

  String title() const;
  double startTime() const;
  v8::LocalVector<v8::Value> artwork(ScriptState*) const;

  // Internal use only, returns a reference to m_artwork instead of a Frozen
  // copy of a `MediaImage` array.
  const HeapVector<Member<MediaImage>>& artwork() const;

  void Trace(Visitor*) const override;

 private:
  // Make an internal copy of the MediaImage vector with some internal steps
  // such as parsing of the src property.
  void SetArtworkInternal(ScriptState*,
                          const HeapVector<Member<MediaImage>>&,
                          ExceptionState&);

  const String title_;
  const double start_time_;
  HeapVector<Member<MediaImage>> artwork_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASESSION_CHAPTER_INFORMATION_H_
