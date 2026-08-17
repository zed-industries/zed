// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_TESTING_FAKE_RESOURCE_LISTENER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_TESTING_FAKE_RESOURCE_LISTENER_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/webrtc/api/adaptation/resource.h"

namespace blink {

class FakeResourceListener : public webrtc::ResourceListener {
 public:
  ~FakeResourceListener() override = default;

  size_t measurement_count() const;
  webrtc::ResourceUsageState latest_measurement() const;

  // webrtc::ResourceListener implementation.
  void OnResourceUsageStateMeasured(
      webrtc::scoped_refptr<webrtc::Resource> resource,
      webrtc::ResourceUsageState usage_state) override;

 private:
  size_t measurement_count_ = 0u;
  webrtc::ResourceUsageState latest_measurement_ =
      webrtc::ResourceUsageState::kUnderuse;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_TESTING_FAKE_RESOURCE_LISTENER_H_
