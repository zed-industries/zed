// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_IMAGE_TRACK_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_IMAGE_TRACK_H_

#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/webcodecs/image_track_list.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class MODULES_EXPORT ImageTrack final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  ImageTrack(ImageTrackList* image_track_list,
             wtf_size_t id,
             uint32_t frame_count,
             int repetition_count,
             bool selected);
  ~ImageTrack() override;

  // image_track.idl implementation.
  uint32_t frameCount() const;
  bool animated() const;
  float repetitionCount() const;
  bool selected() const;
  void setSelected(bool selected);

  // Internal helpers for ImageTrackList.
  void disconnect() { image_track_list_ = nullptr; }
  void set_selected(bool selected) { selected_ = selected; }
  void UpdateTrack(uint32_t frame_count, int repetition_count);

  // GarbageCollected override
  void Trace(Visitor* visitor) const override;

 private:
  const wtf_size_t id_;
  Member<ImageTrackList> image_track_list_;
  uint32_t frame_count_ = 0u;
  int repetition_count_ = 0;
  bool selected_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_IMAGE_TRACK_H_
