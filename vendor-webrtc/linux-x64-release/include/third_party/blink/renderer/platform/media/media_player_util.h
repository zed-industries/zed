// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_MEDIA_PLAYER_UTIL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_MEDIA_PLAYER_UTIL_H_

#include "media/base/output_device_info.h"
#include "media/mojo/mojom/media_metrics_provider.mojom-blink.h"
#include "third_party/blink/public/platform/web_media_player.h"
#include "third_party/blink/public/platform/web_set_sink_id_callbacks.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace media {
class MediaLog;
}  // namespace media

namespace blink {

// Translates a |url| into the appropriate URL scheme.
PLATFORM_EXPORT media::mojom::blink::MediaURLScheme GetMediaURLScheme(
    const KURL& url);

// Report various metrics to UMA.
PLATFORM_EXPORT void ReportMetrics(WebMediaPlayer::LoadType load_type,
                                   const KURL& url,
                                   media::MediaLog* media_log);

// Wraps a WebSetSinkIdCompleteCallback into a
// media::OutputDeviceStatusCB and binds it to the current thread
PLATFORM_EXPORT media::OutputDeviceStatusCB ConvertToOutputDeviceStatusCB(
    WebSetSinkIdCompleteCallback completion_callback);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_MEDIA_PLAYER_UTIL_H_
