/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_WIN_WGC_CAPTURE_SOURCE_H_
#define MODULES_DESKTOP_CAPTURE_WIN_WGC_CAPTURE_SOURCE_H_

#include <windows.graphics.capture.h>
#include <windows.graphics.h>
#include <wrl/client.h>

#include <memory>
#include <optional>

#include "modules/desktop_capture/desktop_capturer.h"
#include "modules/desktop_capture/desktop_geometry.h"

namespace webrtc {

// Abstract class to represent the source that WGC-based capturers capture
// from. Could represent an application window or a screen. Consumers should use
// the appropriate Wgc*SourceFactory class to create WgcCaptureSource objects
// of the appropriate type.
class WgcCaptureSource {
 public:
  explicit WgcCaptureSource(DesktopCapturer::SourceId source_id);
  virtual ~WgcCaptureSource();

  virtual DesktopVector GetTopLeft() = 0;
  // Lightweight version of IsCapturable which avoids allocating/deallocating
  // COM objects for each call. As such may return a different value than
  // IsCapturable.
  virtual bool ShouldBeCapturable();
  virtual bool IsCapturable();
  virtual bool FocusOnSource();
  virtual ABI::Windows::Graphics::SizeInt32 GetSize();
  HRESULT GetCaptureItem(
      Microsoft::WRL::ComPtr<
          ABI::Windows::Graphics::Capture::IGraphicsCaptureItem>* result);
  DesktopCapturer::SourceId GetSourceId() { return source_id_; }

 protected:
  virtual HRESULT CreateCaptureItem(
      Microsoft::WRL::ComPtr<
          ABI::Windows::Graphics::Capture::IGraphicsCaptureItem>* result) = 0;

 private:
  Microsoft::WRL::ComPtr<ABI::Windows::Graphics::Capture::IGraphicsCaptureItem>
      item_;
  const DesktopCapturer::SourceId source_id_;
};

class WgcCaptureSourceFactory {
 public:
  virtual ~WgcCaptureSourceFactory();

  virtual std::unique_ptr<WgcCaptureSource> CreateCaptureSource(
      DesktopCapturer::SourceId) = 0;
};

class WgcWindowSourceFactory final : public WgcCaptureSourceFactory {
 public:
  WgcWindowSourceFactory();

  // Disallow copy and assign.
  WgcWindowSourceFactory(const WgcWindowSourceFactory&) = delete;
  WgcWindowSourceFactory& operator=(const WgcWindowSourceFactory&) = delete;

  ~WgcWindowSourceFactory() override;

  std::unique_ptr<WgcCaptureSource> CreateCaptureSource(
      DesktopCapturer::SourceId) override;
};

class WgcScreenSourceFactory final : public WgcCaptureSourceFactory {
 public:
  WgcScreenSourceFactory();

  WgcScreenSourceFactory(const WgcScreenSourceFactory&) = delete;
  WgcScreenSourceFactory& operator=(const WgcScreenSourceFactory&) = delete;

  ~WgcScreenSourceFactory() override;

  std::unique_ptr<WgcCaptureSource> CreateCaptureSource(
      DesktopCapturer::SourceId) override;
};

// Class for capturing application windows.
class WgcWindowSource final : public WgcCaptureSource {
 public:
  explicit WgcWindowSource(DesktopCapturer::SourceId source_id);

  WgcWindowSource(const WgcWindowSource&) = delete;
  WgcWindowSource& operator=(const WgcWindowSource&) = delete;

  ~WgcWindowSource() override;

  DesktopVector GetTopLeft() override;
  ABI::Windows::Graphics::SizeInt32 GetSize() override;
  bool ShouldBeCapturable() override;
  bool IsCapturable() override;
  bool FocusOnSource() override;

 private:
  HRESULT CreateCaptureItem(
      Microsoft::WRL::ComPtr<
          ABI::Windows::Graphics::Capture::IGraphicsCaptureItem>* result)
      override;
};

// Class for capturing screens/monitors/displays.
class WgcScreenSource final : public WgcCaptureSource {
 public:
  explicit WgcScreenSource(DesktopCapturer::SourceId source_id);

  WgcScreenSource(const WgcScreenSource&) = delete;
  WgcScreenSource& operator=(const WgcScreenSource&) = delete;

  ~WgcScreenSource() override;

  DesktopVector GetTopLeft() override;
  ABI::Windows::Graphics::SizeInt32 GetSize() override;
  bool IsCapturable() override;

 private:
  HRESULT CreateCaptureItem(
      Microsoft::WRL::ComPtr<
          ABI::Windows::Graphics::Capture::IGraphicsCaptureItem>* result)
      override;

  // To maintain compatibility with other capturers, this class accepts a
  // device index as it's SourceId. However, WGC requires we use an HMONITOR to
  // describe which screen to capture. So, we internally convert the supplied
  // device index into an HMONITOR when `IsCapturable()` is called.
  std::optional<HMONITOR> hmonitor_;
};

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_WIN_WGC_CAPTURE_SOURCE_H_
