// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_IMAGE_TRACK_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_IMAGE_TRACK_LIST_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise_property.h"
#include "third_party/blink/renderer/core/dom/dom_exception.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {
class ImageDecoderExternal;
class ImageTrack;

class MODULES_EXPORT ImageTrackList final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit ImageTrackList(ImageDecoderExternal* image_decoder);
  ~ImageTrackList() override;

  // image_track_list.idl implementation.
  uint32_t length() const { return tracks_.size(); }
  ImageTrack* AnonymousIndexedGetter(uint32_t index) const;
  int32_t selectedIndex() const;
  ImageTrack* selectedTrack() const;
  ScriptPromise<IDLUndefined> ready(ScriptState* script_state);

  bool IsEmpty() const { return tracks_.empty(); }

  // Called when initial track metadata is known or an error has occurred. Pass
  // a valid |exception| to reject the ready() promise.
  void OnTracksReady(DOMException* exception = nullptr);

  // Helper method for ImageDecoder to add tracks. Only one track may be marked
  // as selected at any given time.
  void AddTrack(uint32_t frame_count, int repetition_count, bool selected);

  // Called by ImageTrack entries to update their track selection status.
  void OnTrackSelectionChanged(wtf_size_t index);

  // Disconnects and clears the ImageTrackList. Disconnecting releases all
  // Member<> references held by the ImageTrackList and ImageTrack.
  void Disconnect();

  // GarbageCollected override
  void Trace(Visitor* visitor) const override;

 private:
  Member<ImageDecoderExternal> image_decoder_;
  HeapVector<Member<ImageTrack>> tracks_;
  std::optional<wtf_size_t> selected_track_id_;

  using ReadyProperty = ScriptPromiseProperty<IDLUndefined, DOMException>;
  Member<ReadyProperty> ready_property_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_IMAGE_TRACK_LIST_H_
