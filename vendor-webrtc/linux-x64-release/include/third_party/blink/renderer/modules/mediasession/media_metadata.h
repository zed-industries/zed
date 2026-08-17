// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASESSION_MEDIA_METADATA_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASESSION_MEDIA_METADATA_H_

#include "third_party/blink/renderer/bindings/modules/v8/v8_chapter_information.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_chapter_information_init.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_media_image.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/timer.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ChapterInformation;
class ExceptionState;
class MediaMetadataInit;
class MediaSession;
class ScriptState;

// Implementation of MediaMetadata interface from the Media Session API.
// The MediaMetadata object is linked to a MediaSession that owns it. When one
// of its properties are updated, the object will notify its MediaSession if
// any. The notification will be made asynchronously in order to combine changes
// made inside the same event loop. When a MediaMetadata is created and assigned
// to a MediaSession, the MediaSession will automatically update.
class MODULES_EXPORT MediaMetadata final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static MediaMetadata* Create(ScriptState*,
                               const MediaMetadataInit*,
                               ExceptionState&);

  MediaMetadata(ScriptState*, const MediaMetadataInit*, ExceptionState&);

  String title() const;
  String artist() const;
  String album() const;
  v8::LocalVector<v8::Value> artwork(ScriptState*) const;
  v8::LocalVector<v8::Value> chapterInfo(ScriptState*) const;

  // Internal use only, returns a reference to m_artwork instead of a Frozen
  // copy of a `MediaImage` array. Same for the `ChapterInformation`.
  const HeapVector<Member<MediaImage>>& artwork() const;
  const HeapVector<Member<ChapterInformation>>& chapterInfo() const;

  void setTitle(const String&);
  void setArtist(const String&);
  void setAlbum(const String&);
  void setArtwork(ScriptState*,
                  const HeapVector<Member<MediaImage>>&,
                  ExceptionState&);

  // Called by MediaSession to associate or de-associate itself.
  void SetSession(MediaSession*);

  void Trace(Visitor*) const override;

 private:
  // Called when one of the metadata fields is updated from script. It will
  // notify the session asynchronously in order to bundle multiple call in one
  // notification.
  void NotifySessionAsync();

  // Called asynchronously after at least one field of MediaMetadata has been
  // modified.
  void NotifySessionTimerFired(TimerBase*);

  // Make an internal copy of the MediaImage vector with some internal steps
  // such as parsing of the src property.
  void SetArtworkInternal(ScriptState*,
                          const HeapVector<Member<MediaImage>>&,
                          ExceptionState&);

  // Set the `ChapterInfo` from `ChapterInformationInit` list.
  void SetChapterInfoFromInit(ScriptState*,
                              const HeapVector<Member<ChapterInformationInit>>&,
                              ExceptionState&);

  String title_;
  String artist_;
  String album_;
  HeapVector<Member<MediaImage>> artwork_;
  HeapVector<Member<ChapterInformation>> chapterInfo_;

  Member<MediaSession> session_;
  HeapTaskRunnerTimer<MediaMetadata> notify_session_timer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASESSION_MEDIA_METADATA_H_
