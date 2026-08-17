/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_DESKTOP_CAPTURER_H_
#define MODULES_DESKTOP_CAPTURE_DESKTOP_CAPTURER_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>
#include <string>
#include <type_traits>
#include <vector>

// TODO(alcooper): Update include usage in downstream consumers and then change
// this to a forward declaration.
#include "modules/desktop_capture/delegated_source_list_controller.h"
#if defined(WEBRTC_USE_GIO)
#include "modules/desktop_capture/desktop_capture_metadata.h"
#endif  // defined(WEBRTC_USE_GIO)
#include "modules/desktop_capture/desktop_capture_types.h"
#include "modules/desktop_capture/desktop_frame.h"
#include "modules/desktop_capture/shared_memory.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

void RTC_EXPORT LogDesktopCapturerFullscreenDetectorUsage();

class DesktopCaptureOptions;
class DesktopFrame;

// Abstract interface for screen and window capturers.
class RTC_EXPORT DesktopCapturer {
 public:
  enum class Result {
    // The frame was captured successfully.
    SUCCESS,

    // There was a temporary error. The caller should continue calling
    // CaptureFrame(), in the expectation that it will eventually recover.
    ERROR_TEMPORARY,

    // Capture has failed and will keep failing if the caller tries calling
    // CaptureFrame() again.
    ERROR_PERMANENT,

    MAX_VALUE = ERROR_PERMANENT
  };

  // Interface that must be implemented by the DesktopCapturer consumers.
  class Callback {
   public:
    // Called before a frame capture is started.
    virtual void OnFrameCaptureStart() {}

    // Called after a frame has been captured. `frame` is not nullptr if and
    // only if `result` is SUCCESS.
    virtual void OnCaptureResult(Result result,
                                 std::unique_ptr<DesktopFrame> frame) = 0;

   protected:
    virtual ~Callback() {}
  };

#if defined(CHROMEOS)
  typedef int64_t SourceId;
#else
  typedef intptr_t SourceId;
#endif

  static_assert(std::is_same<SourceId, ScreenId>::value,
                "SourceId should be a same type as ScreenId.");

  struct Source {
    // The unique id to represent a Source of current DesktopCapturer.
    SourceId id;

    // Title of the window or screen in UTF-8 encoding, maybe empty. This field
    // should not be used to identify a source.
    std::string title;

#if defined(CHROMEOS)
    // TODO(https://crbug.com/1369162): Remove or refactor this value.
    WindowId in_process_id = kNullWindowId;
#endif

    // The display's unique ID. If no ID is defined, it will hold the value
    // kInvalidDisplayId.
    int64_t display_id = kInvalidDisplayId;
  };

  typedef std::vector<Source> SourceList;

  virtual ~DesktopCapturer();

  // Called at the beginning of a capturing session. `callback` must remain
  // valid until capturer is destroyed.
  virtual void Start(Callback* callback) = 0;

  // Sets max frame rate for the capturer. This is best effort and may not be
  // supported by all capturers. This will only affect the frequency at which
  // new frames are available, not the frequency at which you are allowed to
  // capture the frames.
  virtual void SetMaxFrameRate(uint32_t /* max_frame_rate */) {}

  // Returns a valid pointer if the capturer requires the user to make a
  // selection from a source list provided by the capturer.
  // Returns nullptr if the capturer does not provide a UI for the user to make
  // a selection.
  //
  // Callers should not take ownership of the returned pointer, but it is
  // guaranteed to be valid as long as the desktop_capturer is valid.
  // Note that consumers should still use GetSourceList and SelectSource, but
  // their behavior may be modified if this returns a value. See those methods
  // for a more in-depth discussion of those potential modifications.
  virtual DelegatedSourceListController* GetDelegatedSourceListController();

  // Sets SharedMemoryFactory that will be used to create buffers for the
  // captured frames. The factory can be invoked on a thread other than the one
  // where CaptureFrame() is called. It will be destroyed on the same thread.
  // Shared memory is currently supported only by some DesktopCapturer
  // implementations.
  virtual void SetSharedMemoryFactory(
      std::unique_ptr<SharedMemoryFactory> shared_memory_factory);

  // Captures next frame, and involve callback provided by Start() function.
  // Pending capture requests are canceled when DesktopCapturer is deleted.
  virtual void CaptureFrame() = 0;

  // Sets the window to be excluded from the captured image in the future
  // Capture calls. Used to exclude the screenshare notification window for
  // screen capturing.
  virtual void SetExcludedWindow(WindowId window);

  // TODO(zijiehe): Following functions should be pure virtual. The default
  // implementations are for backward compatibility only. Remove default
  // implementations once all DesktopCapturer implementations in Chromium have
  // implemented these functions.

  // Gets a list of sources current capturer supports. Returns false in case of
  // a failure.
  // For DesktopCapturer implementations to capture screens, this function
  // should return monitors.
  // For DesktopCapturer implementations to capture windows, this function
  // should only return root windows owned by applications.
  //
  // Note that capturers who use a delegated source list will return a
  // SourceList with exactly one value, but it may not be viable for capture
  // (e.g. CaptureFrame will return ERROR_TEMPORARY) until a selection has been
  // made.
  virtual bool GetSourceList(SourceList* sources);

  // Selects a source to be captured. Returns false in case of a failure (e.g.
  // if there is no source with the specified type and id.)
  //
  // Note that some capturers with delegated source lists may also support
  // selecting a SourceID that is not in the returned source list as a form of
  // restore token.
  virtual bool SelectSource(SourceId id);

  // Brings the selected source to the front and sets the input focus on it.
  // Returns false in case of a failure or no source has been selected or the
  // implementation does not support this functionality.
  virtual bool FocusOnSelectedSource();

  // Returns true if the `pos` on the selected source is covered by other
  // elements on the display, and is not visible to the users.
  // `pos` is in full desktop coordinates, i.e. the top-left monitor always
  // starts from (0, 0).
  // The return value if `pos` is out of the scope of the source is undefined.
  virtual bool IsOccluded(const DesktopVector& pos);

  // Creates a DesktopCapturer instance which targets to capture windows.
  static std::unique_ptr<DesktopCapturer> CreateWindowCapturer(
      const DesktopCaptureOptions& options);

  // Creates a DesktopCapturer instance which targets to capture screens.
  static std::unique_ptr<DesktopCapturer> CreateScreenCapturer(
      const DesktopCaptureOptions& options);

  // Creates a DesktopCapturer instance which targets to capture windows and
  // screens.
  static std::unique_ptr<DesktopCapturer> CreateGenericCapturer(
      const DesktopCaptureOptions& options);

#if defined(WEBRTC_USE_PIPEWIRE) || defined(WEBRTC_USE_X11)
  static bool IsRunningUnderWayland();

  virtual void UpdateResolution(uint32_t width, uint32_t height) {}
#endif  // defined(WEBRTC_USE_PIPEWIRE) || defined(WEBRTC_USE_X11)

#if defined(WEBRTC_USE_GIO)
  // Populates implementation specific metadata into the passed in pointer.
  // Classes can choose to override it or use the default no-op implementation.
  virtual DesktopCaptureMetadata GetMetadata() { return {}; }
#endif  // defined(WEBRTC_USE_GIO)

 protected:
  // CroppingWindowCapturer needs to create raw capturers without wrappers, so
  // the following two functions are protected.

  // Creates a platform specific DesktopCapturer instance which targets to
  // capture windows.
  static std::unique_ptr<DesktopCapturer> CreateRawWindowCapturer(
      const DesktopCaptureOptions& options);

  // Creates a platform specific DesktopCapturer instance which targets to
  // capture screens.
  static std::unique_ptr<DesktopCapturer> CreateRawScreenCapturer(
      const DesktopCaptureOptions& options);
};

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_DESKTOP_CAPTURER_H_
