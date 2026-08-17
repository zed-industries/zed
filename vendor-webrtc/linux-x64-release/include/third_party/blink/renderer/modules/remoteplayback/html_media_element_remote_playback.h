// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_REMOTEPLAYBACK_HTML_MEDIA_ELEMENT_REMOTE_PLAYBACK_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_REMOTEPLAYBACK_HTML_MEDIA_ELEMENT_REMOTE_PLAYBACK_H_

#include "third_party/blink/renderer/core/html/media/html_media_element.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class HTMLMediaElement;
class QualifiedName;
class RemotePlayback;

// Collection of static methods only used for bindings in the context of the
// Remote Playback API.
class MODULES_EXPORT HTMLMediaElementRemotePlayback final {
  STATIC_ONLY(HTMLMediaElementRemotePlayback);

 public:
  static bool FastHasAttribute(const HTMLMediaElement&, const QualifiedName&);
  static void SetBooleanAttribute(HTMLMediaElement&,
                                  const QualifiedName&,
                                  bool);

  static HTMLMediaElementRemotePlayback& From(HTMLMediaElement&);
  static RemotePlayback* remote(HTMLMediaElement&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_REMOTEPLAYBACK_HTML_MEDIA_ELEMENT_REMOTE_PLAYBACK_H_
