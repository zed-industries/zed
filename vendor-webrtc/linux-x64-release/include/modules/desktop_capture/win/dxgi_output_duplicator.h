/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_WIN_DXGI_OUTPUT_DUPLICATOR_H_
#define MODULES_DESKTOP_CAPTURE_WIN_DXGI_OUTPUT_DUPLICATOR_H_

#include <comdef.h>
#include <dxgi.h>
#include <dxgi1_2.h>
#include <shellscalingapi.h>
#include <wrl/client.h>

#include <memory>
#include <string>
#include <vector>

#include "modules/desktop_capture/desktop_frame_rotation.h"
#include "modules/desktop_capture/desktop_geometry.h"
#include "modules/desktop_capture/desktop_region.h"
#include "modules/desktop_capture/shared_desktop_frame.h"
#include "modules/desktop_capture/win/d3d_device.h"
#include "modules/desktop_capture/win/dxgi_context.h"
#include "modules/desktop_capture/win/dxgi_texture.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

// Duplicates the content on one IDXGIOutput, i.e. one monitor attached to one
// video card. None of functions in this class is thread-safe.
class DxgiOutputDuplicator {
 public:
  using Context = DxgiOutputContext;

  // Creates an instance of DxgiOutputDuplicator from a D3dDevice and one of its
  // IDXGIOutput1. Caller must maintain the lifetime of device, to make sure it
  // outlives this instance. Only DxgiAdapterDuplicator can create an instance.
  DxgiOutputDuplicator(const D3dDevice& device,
                       const Microsoft::WRL::ComPtr<IDXGIOutput1>& output,
                       const DXGI_OUTPUT_DESC& desc);

  // To allow this class to work with vector.
  DxgiOutputDuplicator(DxgiOutputDuplicator&& other);

  // Destructs this instance. We need to make sure texture_ has been released
  // before duplication_.
  ~DxgiOutputDuplicator();

  // Initializes duplication_ object.
  bool Initialize();

  // Copies the content of current IDXGIOutput to the `target`. To improve the
  // performance, this function copies only regions merged from
  // `context`->updated_region and DetectUpdatedRegion(). The `offset` decides
  // the offset in the `target` where the content should be copied to. i.e. this
  // function copies the content to the rectangle of (offset.x(), offset.y()) to
  // (offset.x() + desktop_rect_.width(), offset.y() + desktop_rect_.height()).
  // Returns false in case of a failure.
  // May retain a reference to `target` so that a "captured" frame can be
  // returned in the event that a new frame is not ready to be captured yet.
  // (Or in other words, if the call to IDXGIOutputDuplication::AcquireNextFrame
  // indicates that there is not yet a new frame, this is usually because no
  // updates have occurred to the frame).
  bool Duplicate(Context* context,
                 DesktopVector offset,
                 SharedDesktopFrame* target);

  // Returns the desktop rect covered by this DxgiOutputDuplicator.
  DesktopRect desktop_rect() const { return desktop_rect_; }

  // Returns the device name from DXGI_OUTPUT_DESC in utf8 encoding.
  const std::string& device_name() const { return device_name_; }

  void Setup(Context* context);

  void Unregister(const Context* const context);

  // How many frames have been captured by this DxigOutputDuplicator.
  int64_t num_frames_captured() const;

  // Device scale factor of the monitor associated with this
  // DxigOutputDuplicator.
  std::optional<float> device_scale_factor() const;

  // Moves `desktop_rect_`. See DxgiDuplicatorController::TranslateRect().
  void TranslateRect(const DesktopVector& position);

 private:
  // Calls DoDetectUpdatedRegion(). If it fails, this function sets the
  // `updated_region` as entire UntranslatedDesktopRect().
  void DetectUpdatedRegion(const DXGI_OUTDUPL_FRAME_INFO& frame_info,
                           DesktopRegion* updated_region);

  // Returns untranslated updated region, which are directly returned by Windows
  // APIs. Returns false in case of a failure.
  bool DoDetectUpdatedRegion(const DXGI_OUTDUPL_FRAME_INFO& frame_info,
                             DesktopRegion* updated_region);

  // Returns true if the mouse cursor is embedded in the captured frame and
  // false if not. Also logs the same boolean as
  // WebRTC.DesktopCapture.Win.DirectXCursorEmbedded UMA.
  bool ContainsMouseCursor(const DXGI_OUTDUPL_FRAME_INFO& frame_info);

  bool ReleaseFrame();

  // Initializes duplication_ instance. Expects duplication_ is in empty status.
  // Returns false if system does not support IDXGIOutputDuplication.
  bool DuplicateOutput();

  // Returns a DesktopRect with the same size of desktop_size(), but translated
  // by offset.
  DesktopRect GetTranslatedDesktopRect(DesktopVector offset) const;

  // Returns a DesktopRect with the same size of desktop_size(), but starts from
  // (0, 0).
  DesktopRect GetUntranslatedDesktopRect() const;

  // Spreads changes from `context` to other registered Context(s) in
  // contexts_.
  void SpreadContextChange(const Context* const context);

  // Returns the size of desktop rectangle current instance representing.
  DesktopSize desktop_size() const;

  const D3dDevice device_;
  const Microsoft::WRL::ComPtr<IDXGIOutput1> output_;
  const std::string device_name_;
  DesktopRect desktop_rect_;
  const HMONITOR monitor_;
  Microsoft::WRL::ComPtr<IDXGIOutputDuplication> duplication_;
  DXGI_OUTDUPL_DESC desc_;
  std::vector<uint8_t> metadata_;
  std::unique_ptr<DxgiTexture> texture_;
  Rotation rotation_;
  DesktopSize unrotated_size_;

  // After each AcquireNextFrame() function call, updated_region_(s) of all
  // active Context(s) need to be updated. Since they have missed the
  // change this time. And during next Duplicate() function call, their
  // updated_region_ will be merged and copied.
  std::vector<Context*> contexts_;

  // The last full frame of this output and its offset. If on AcquireNextFrame()
  // failed because of timeout, i.e. no update, we can copy content from
  // `last_frame_`.
  std::unique_ptr<SharedDesktopFrame> last_frame_;
  DesktopVector last_frame_offset_;

  int64_t num_frames_captured_ = 0;
};

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_WIN_DXGI_OUTPUT_DUPLICATOR_H_
