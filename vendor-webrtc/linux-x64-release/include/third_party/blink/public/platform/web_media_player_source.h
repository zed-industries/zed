// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_MEDIA_PLAYER_SOURCE_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_MEDIA_PLAYER_SOURCE_H_

#include "third_party/blink/public/platform/modules/mediastream/web_media_stream.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_url.h"

namespace blink {

class BLINK_PLATFORM_EXPORT WebMediaPlayerSource {
 public:
  WebMediaPlayerSource();
  explicit WebMediaPlayerSource(const WebURL&);
  explicit WebMediaPlayerSource(const WebMediaStream&);
  ~WebMediaPlayerSource();

  bool IsURL() const;
  WebURL GetAsURL() const;

  bool IsMediaStream() const;
  WebMediaStream GetAsMediaStream() const;

 private:
  WebURL url_;
  WebMediaStream media_stream_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_MEDIA_PLAYER_SOURCE_H_
