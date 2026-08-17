#ifndef VAAPI_DISPLAY_DRM_H_
#define VAAPI_DISPLAY_DRM_H_

#include <stdio.h>
#include <va/va.h>

namespace livekit_ffi {

// VAAPI drm display wrapper class
class VaapiDisplayDrm {
 public:
  VaapiDisplayDrm() = default;
  VaapiDisplayDrm(const VaapiDisplayDrm&) = delete;
  ~VaapiDisplayDrm() = default;

  // Initialize the VAAPI display
  bool Open();

  // Check if the VAAPI display is open
  bool isOpen() const;
  
  // Close the VAAPI display
  void Close();

  // Get the VAAPI display handle
  VADisplay display() const { return va_display_; }

 private:
  VADisplay va_display_;
  int drm_fd_;
};

}  // namespace livekit_ffi

#endif  // VAAPI_DISPLAY_DRM_H_
