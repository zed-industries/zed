// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_PERMISSION_TESTING_PLATFORM_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_PERMISSION_TESTING_PLATFORM_H_

#include <memory>

#include "third_party/blink/renderer/platform/testing/testing_platform_support.h"

namespace blink {

class MediaPermissionTestingPlatform : public TestingPlatformSupport {
 public:
  explicit MediaPermissionTestingPlatform(
      std::unique_ptr<media::MediaPermission> media_permission);

  media::MediaPermission* GetWebRTCMediaPermission(
      WebLocalFrame* web_frame) override;

 private:
  std::unique_ptr<media::MediaPermission> media_permission_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_PERMISSION_TESTING_PLATFORM_H_
