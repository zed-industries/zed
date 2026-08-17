// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_MEDIA_SOURCE_HANDLE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_MEDIA_SOURCE_HANDLE_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/media/media_source_attachment.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class CORE_EXPORT MediaSourceHandle : public GarbageCollectedMixin {
 public:
  MediaSourceHandle(const MediaSourceHandle&) = delete;
  MediaSourceHandle& operator=(const MediaSourceHandle&) = delete;
  virtual ~MediaSourceHandle() = default;

  // Removes our reference on the attachment, giving it to the caller.
  virtual scoped_refptr<MediaSourceAttachment> TakeAttachment() = 0;

  virtual String GetInternalBlobURL() = 0;

  void mark_used() { used_ = true; }

  bool is_used() const { return used_; }
  bool is_serialized() const { return serialized_; }

 protected:
  MediaSourceHandle() = default;

  bool serialized_ = false;
  bool used_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_MEDIA_SOURCE_HANDLE_H_
