/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_WIN_DXGI_ADAPTER_DUPLICATOR_H_
#define MODULES_DESKTOP_CAPTURE_WIN_DXGI_ADAPTER_DUPLICATOR_H_

#include <wrl/client.h>

#include <vector>

#include "modules/desktop_capture/desktop_geometry.h"
#include "modules/desktop_capture/shared_desktop_frame.h"
#include "modules/desktop_capture/win/d3d_device.h"
#include "modules/desktop_capture/win/dxgi_context.h"
#include "modules/desktop_capture/win/dxgi_output_duplicator.h"

namespace webrtc {

// A container of DxgiOutputDuplicators to duplicate monitors attached to a
// single video card.
class DxgiAdapterDuplicator {
 public:
  using Context = DxgiAdapterContext;

  // Creates an instance of DxgiAdapterDuplicator from a D3dDevice. Only
  // DxgiDuplicatorController can create an instance.
  explicit DxgiAdapterDuplicator(const D3dDevice& device);

  // Move constructor, to make it possible to store instances of
  // DxgiAdapterDuplicator in std::vector<>.
  DxgiAdapterDuplicator(DxgiAdapterDuplicator&& other);

  ~DxgiAdapterDuplicator();

  // Initializes the DxgiAdapterDuplicator from a D3dDevice.
  bool Initialize();

  // Sequentially calls Duplicate function of all the DxgiOutputDuplicator
  // instances owned by this instance, and writes into `target`.
  bool Duplicate(Context* context, SharedDesktopFrame* target);

  // Captures one monitor and writes into `target`. `monitor_id` should be
  // between [0, screen_count()).
  bool DuplicateMonitor(Context* context,
                        int monitor_id,
                        SharedDesktopFrame* target);

  // Returns desktop rect covered by this DxgiAdapterDuplicator.
  DesktopRect desktop_rect() const { return desktop_rect_; }

  // Returns the device scale factor of screen identified by `screen_id`, which
  // is owned by this DxgiAdapterDuplicator. `screen_id` should be between [0,
  // screen_count()).
  std::optional<float> GetDeviceScaleFactor(int screen_id) const;

  // Returns the size of one screen owned by this DxgiAdapterDuplicator. `id`
  // should be between [0, screen_count()).
  DesktopRect ScreenRect(int id) const;

  // Returns the device name of one screen owned by this DxgiAdapterDuplicator
  // in utf8 encoding. `id` should be between [0, screen_count()).
  const std::string& GetDeviceName(int id) const;

  // Returns the count of screens owned by this DxgiAdapterDuplicator. These
  // screens can be retrieved by an interger in the range of
  // [0, screen_count()).
  int screen_count() const;

  void Setup(Context* context);

  void Unregister(const Context* const context);

  // The minimum num_frames_captured() returned by `duplicators_`.
  int64_t GetNumFramesCaptured(int monitor_id) const;

  // Moves `desktop_rect_` and all underlying `duplicators_`. See
  // DxgiDuplicatorController::TranslateRect().
  void TranslateRect(const DesktopVector& position);

 private:
  bool DoInitialize();

  const D3dDevice device_;
  std::vector<DxgiOutputDuplicator> duplicators_;
  DesktopRect desktop_rect_;
};

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_WIN_DXGI_ADAPTER_DUPLICATOR_H_
