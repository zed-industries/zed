// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_MEDIA_SOURCE_REGISTRY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_MEDIA_SOURCE_REGISTRY_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/fileapi/url_registry.h"
#include "third_party/blink/renderer/core/html/media/media_source_attachment.h"

namespace blink {

// Core interface extension of URLRegistry to allow interactions with a
// URLRegistry for registered MediaSourceAttachments handled with a
// scoped_refptr.
class CORE_EXPORT MediaSourceRegistry : public URLRegistry {
 public:
  // Finds the attachment, if any, registered with |url| in the
  // MediaSourceRegistry implementation. |url| must be non-empty. If such an
  // active registration for |url| is not found, returns an unset
  // scoped_refptr<MediaSourceAttachment>.
  virtual scoped_refptr<MediaSourceAttachment> LookupMediaSource(
      const String& url) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_MEDIA_SOURCE_REGISTRY_H_
