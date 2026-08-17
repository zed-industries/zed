// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_SERVER_TIMING_HEADER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_SERVER_TIMING_HEADER_H_

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ServerTimingHeader {
  USING_FAST_MALLOC(ServerTimingHeader);

 public:
  explicit ServerTimingHeader(const String& name) : name_(name) {}

  const String& Name() const { return name_; }
  const double& Duration() const { return duration_; }
  const String& Description() const { return description_; }

  void SetParameter(StringView, String);

 private:
  String name_;
  double duration_ = 0.0;
  String description_ = "";

  bool duration_set_ = false;
  bool description_set_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_SERVER_TIMING_HEADER_H_
