#ifndef VAAPI_DISPLAY_WIN32_H_
#define VAAPI_DISPLAY_WIN32_H_

#include <va/va.h>
#include <va/va_win32.h>

namespace livekit_ffi {

// VAAPI win32 display wrapper class
class VaapiDisplayWin32 {
 public:
  VaapiDisplayWin32();
  ~VaapiDisplayWin32() {}

  // Initialize the VAAPI display
  bool Open();

  // Check if the VAAPI display is open
  bool isOpen() const;
  
  // Close the VAAPI display
  void Close();

  // Get the VAAPI display handle
  VADisplay display() const { return va_display_; }

 private:
  VADisplay va_display_ = nullptr;
};

}  // namespace livekit_ffi

#endif  // VAAPI_DISPLAY_WIN32_H_
