/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_DESKTOP_FRAME_H_
#define MODULES_DESKTOP_CAPTURE_DESKTOP_FRAME_H_

#include <stdint.h>

#include <memory>
#include <optional>
#include <vector>

#include "modules/desktop_capture/desktop_geometry.h"
#include "modules/desktop_capture/desktop_region.h"
#include "modules/desktop_capture/shared_memory.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

const float kStandardDPI = 96.0f;

// DesktopFrame represents a video frame captured from the screen.
class RTC_EXPORT DesktopFrame {
 public:
  // DesktopFrame objects always hold BGRA data.
  static const int kBytesPerPixel = 4;

  virtual ~DesktopFrame();

  DesktopFrame(const DesktopFrame&) = delete;
  DesktopFrame& operator=(const DesktopFrame&) = delete;

  // Returns the rectangle in full desktop coordinates to indicate it covers
  // the area of top_left() to top_letf() + size() / scale_factor().
  DesktopRect rect() const;

  // Returns the scale factor from DIPs to physical pixels of the frame.
  // Assumes same scale in both X and Y directions at present.
  float scale_factor() const;

  // Size of the frame. In physical coordinates, mapping directly from the
  // underlying buffer.
  const DesktopSize& size() const { return size_; }

  // The top-left of the frame in full desktop coordinates. E.g. the top left
  // monitor should start from (0, 0). The desktop coordinates may be scaled by
  // OS, but this is always consistent with the MouseCursorMonitor.
  const DesktopVector& top_left() const { return top_left_; }
  void set_top_left(const DesktopVector& top_left) { top_left_ = top_left; }

  // Distance in the buffer between two neighboring rows in bytes.
  int stride() const { return stride_; }

  // Data buffer used for the frame.
  uint8_t* data() const { return data_; }

  // SharedMemory used for the buffer or NULL if memory is allocated on the
  // heap. The result is guaranteed to be deleted only after the frame is
  // deleted (classes that inherit from DesktopFrame must ensure it).
  SharedMemory* shared_memory() const { return shared_memory_; }

  // Indicates region of the screen that has changed since the previous frame.
  const DesktopRegion& updated_region() const { return updated_region_; }
  DesktopRegion* mutable_updated_region() { return &updated_region_; }

  // DPI of the screen being captured. May be set to zero, e.g. if DPI is
  // unknown.
  const DesktopVector& dpi() const { return dpi_; }
  void set_dpi(const DesktopVector& dpi) { dpi_ = dpi; }

  std::optional<float> device_scale_factor() const {
    return device_scale_factor_;
  }
  void set_device_scale_factor(std::optional<float> device_scale_factor) {
    device_scale_factor_ = device_scale_factor;
  }
  // Indicates if this frame may have the mouse cursor in it. Capturers that
  // support cursor capture may set this to true. If the cursor was
  // outside of the captured area, this may be true even though the cursor is
  // not in the image.
  bool may_contain_cursor() const { return may_contain_cursor_; }
  void set_may_contain_cursor(bool may_contain_cursor) {
    may_contain_cursor_ = may_contain_cursor;
  }

  // Time taken to capture the frame in milliseconds.
  int64_t capture_time_ms() const { return capture_time_ms_; }
  void set_capture_time_ms(int64_t time_ms) { capture_time_ms_ = time_ms; }

  // Copies pixels from a buffer or another frame. `dest_rect` rect must lay
  // within bounds of this frame.
  void CopyPixelsFrom(const uint8_t* src_buffer,
                      int src_stride,
                      const DesktopRect& dest_rect);
  void CopyPixelsFrom(const DesktopFrame& src_frame,
                      const DesktopVector& src_pos,
                      const DesktopRect& dest_rect);

  // Copies pixels from another frame, with the copied & overwritten regions
  // representing the intersection between the two frames. Returns true if
  // pixels were copied, or false if there's no intersection. The scale factors
  // represent the ratios between pixel space & offset coordinate space (e.g.
  // 2.0 would indicate the frames are scaled down by 50% for display, so any
  // offset between their origins should be doubled).
  bool CopyIntersectingPixelsFrom(const DesktopFrame& src_frame,
                                  double horizontal_scale,
                                  double vertical_scale);

  // A helper to return the data pointer of a frame at the specified position.
  uint8_t* GetFrameDataAtPos(const DesktopVector& pos) const;

  // The DesktopCapturer implementation which generates current DesktopFrame.
  // Not all DesktopCapturer implementations set this field; it's set to
  // kUnknown by default.
  uint32_t capturer_id() const { return capturer_id_; }
  void set_capturer_id(uint32_t capturer_id) { capturer_id_ = capturer_id; }

  // Copies various information from `other`. Anything initialized in
  // constructor are not copied.
  // This function is usually used when sharing a source DesktopFrame with
  // several clients: the original DesktopFrame should be kept unchanged. For
  // example, BasicDesktopFrame::CopyOf() and SharedDesktopFrame::Share().
  void CopyFrameInfoFrom(const DesktopFrame& other);

  // Copies various information from `other`. Anything initialized in
  // constructor are not copied. Not like CopyFrameInfoFrom() function, this
  // function uses swap or move constructor to avoid data copy. It won't break
  // the `other`, but some of its information may be missing after this
  // operation. E.g. other->updated_region_;
  // This function is usually used when wrapping a DesktopFrame: the wrapper
  // instance takes the ownership of `other`, so other components cannot access
  // `other` anymore. For example, CroppedDesktopFrame and
  // DesktopFrameWithCursor.
  void MoveFrameInfoFrom(DesktopFrame* other);

  // Set and get the ICC profile of the frame data pixels. Useful to build the
  // a ColorSpace object from clients of webrtc library like chromium. The
  // format of an ICC profile is defined in the following specification
  // http://www.color.org/specification/ICC1v43_2010-12.pdf.
  const std::vector<uint8_t>& icc_profile() const { return icc_profile_; }
  void set_icc_profile(const std::vector<uint8_t>& icc_profile) {
    icc_profile_ = icc_profile;
  }

  // Sets all pixel values in the data buffer to zero.
  void SetFrameDataToBlack();

  // Returns true if all pixel values in the data buffer are zero or false
  // otherwise. Also returns false if the frame is empty.
  bool FrameDataIsBlack() const;

 protected:
  DesktopFrame(DesktopSize size,
               int stride,
               uint8_t* data,
               SharedMemory* shared_memory);

  // Ownership of the buffers is defined by the classes that inherit from this
  // class. They must guarantee that the buffer is not deleted before the frame
  // is deleted.
  uint8_t* const data_;
  SharedMemory* const shared_memory_;

 private:
  const DesktopSize size_;
  const int stride_;

  DesktopRegion updated_region_;
  DesktopVector top_left_;
  DesktopVector dpi_;
  bool may_contain_cursor_ = false;
  int64_t capture_time_ms_;
  uint32_t capturer_id_;
  std::vector<uint8_t> icc_profile_;
  // Currently only used on Windows. It stores the device scale factor of the
  // captured surface and has distinct values possible in the range of
  // [1,5].
  std::optional<float> device_scale_factor_;
};

// A DesktopFrame that stores data in the heap.
class RTC_EXPORT BasicDesktopFrame : public DesktopFrame {
 public:
  // The entire data buffer used for the frame is initialized with zeros.
  explicit BasicDesktopFrame(DesktopSize size);

  ~BasicDesktopFrame() override;

  BasicDesktopFrame(const BasicDesktopFrame&) = delete;
  BasicDesktopFrame& operator=(const BasicDesktopFrame&) = delete;

  // Creates a BasicDesktopFrame that contains copy of `frame`.
  // TODO(zijiehe): Return std::unique_ptr<DesktopFrame>
  static DesktopFrame* CopyOf(const DesktopFrame& frame);
};

// A DesktopFrame that stores data in shared memory.
class RTC_EXPORT SharedMemoryDesktopFrame : public DesktopFrame {
 public:
  // May return nullptr if `shared_memory_factory` failed to create a
  // SharedMemory instance.
  // `shared_memory_factory` should not be nullptr.
  static std::unique_ptr<DesktopFrame> Create(
      DesktopSize size,
      SharedMemoryFactory* shared_memory_factory);

  // Takes ownership of `shared_memory`.
  // Deprecated, use the next constructor.
  SharedMemoryDesktopFrame(DesktopSize size,
                           int stride,
                           SharedMemory* shared_memory);

  // Preferred.
  SharedMemoryDesktopFrame(DesktopSize size,
                           int stride,
                           std::unique_ptr<SharedMemory> shared_memory);

  ~SharedMemoryDesktopFrame() override;

  SharedMemoryDesktopFrame(const SharedMemoryDesktopFrame&) = delete;
  SharedMemoryDesktopFrame& operator=(const SharedMemoryDesktopFrame&) = delete;

 private:
  // Avoid unexpected order of parameter evaluation.
  // Executing both std::unique_ptr<T>::operator->() and
  // std::unique_ptr<T>::release() in the member initializer list is not safe.
  // Depends on the order of parameter evaluation,
  // std::unique_ptr<T>::operator->() may trigger assertion failure if it has
  // been evaluated after std::unique_ptr<T>::release(). By using this
  // constructor, std::unique_ptr<T>::operator->() won't be involved anymore.
  SharedMemoryDesktopFrame(DesktopRect rect,
                           int stride,
                           SharedMemory* shared_memory);
};

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_DESKTOP_FRAME_H_
