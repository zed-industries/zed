// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_HTML_MEDIA_TEST_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_HTML_MEDIA_TEST_HELPER_H_

#include "third_party/blink/renderer/core/loader/empty_clients.h"

namespace blink {
namespace test {

// This file contains various class to help with HTML Media tests.

// Generic LocalFrameClient stub to be used by media tests that need a
// WebMediaPlayerimplementation.
class MediaStubLocalFrameClient : public EmptyLocalFrameClient {
 public:
  explicit MediaStubLocalFrameClient(std::unique_ptr<WebMediaPlayer>);
  MediaStubLocalFrameClient(std::unique_ptr<WebMediaPlayer>,
                            bool allow_empty_player);
  MediaStubLocalFrameClient(const MediaStubLocalFrameClient&) = delete;
  MediaStubLocalFrameClient& operator=(const MediaStubLocalFrameClient&) =
      delete;

  std::unique_ptr<WebMediaPlayer> CreateWebMediaPlayer(
      HTMLMediaElement&,
      const WebMediaPlayerSource&,
      WebMediaPlayerClient*) override;

 private:
  std::unique_ptr<WebMediaPlayer> player_;
  bool allow_empty_player_ = false;
};

}  // namespace test
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_HTML_MEDIA_TEST_HELPER_H_
