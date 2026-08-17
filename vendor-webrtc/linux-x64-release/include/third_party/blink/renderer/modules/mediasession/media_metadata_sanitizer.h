// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASESSION_MEDIA_METADATA_SANITIZER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASESSION_MEDIA_METADATA_SANITIZER_H_

#include "third_party/blink/public/mojom/mediasession/media_session.mojom-blink-forward.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class ExecutionContext;
class MediaMetadata;

class MediaMetadataSanitizer {
  STATIC_ONLY(MediaMetadataSanitizer);

 public:
  // Produce the sanitized metadata, which will later be sent to the
  // MediaSession mojo service.
  static blink::mojom::blink::SpecMediaMetadataPtr SanitizeAndConvertToMojo(
      const MediaMetadata*,
      ExecutionContext*);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASESSION_MEDIA_METADATA_SANITIZER_H_
