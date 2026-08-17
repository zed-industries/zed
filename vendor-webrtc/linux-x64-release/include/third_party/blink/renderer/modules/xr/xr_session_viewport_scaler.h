// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_SESSION_VIEWPORT_SCALER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_SESSION_VIEWPORT_SCALER_H_

namespace blink {

class XRSessionViewportScaler final {
 public:
  XRSessionViewportScaler() { ResetLoad(); }
  void UpdateRenderingTimeRatio(float value);
  float Scale() { return scale_; }
  void ResetLoad();

 private:
  float gpu_load_ = 1.0f;
  float scale_ = 1.0f;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_SESSION_VIEWPORT_SCALER_H_
