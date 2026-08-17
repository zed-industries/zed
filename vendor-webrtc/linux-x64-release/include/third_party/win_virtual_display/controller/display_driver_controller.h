// Copyright (c) Microsoft Corporation

#ifndef THIRD_PARTY_WIN_VIRTUAL_DISPLAY_CONTROLLER_DISPLAY_DRIVER_CONTROLLER_H_
#define THIRD_PARTY_WIN_VIRTUAL_DISPLAY_CONTROLLER_DISPLAY_DRIVER_CONTROLLER_H_

#include "third_party/win_virtual_display/driver/public/properties.h"

#include <swdevice.h>

namespace display::test {

// Low level controls for communicating with the virtual display driver.
class DisplayDriverController {
 public:
  ~DisplayDriverController();

  // Returns true if the driver is detected to be installed on the host machine.
  static bool IsDriverInstalled();

  // Sets the configuration of the virtual display driver. Overwrites any
  // previously set configuration. Returns true on success, or false on failure.
  bool SetDisplayConfig(DriverProperties config);

  // Resets the virtual display configuration back to default. Removes all
  // configured virtual displays.
  void Reset();

 private:
  // Open the software device with the specified initial configuration.
  bool Initialize(DriverProperties config);

  // Current handle for software device, or nullptr if none is opened.
  HSWDEVICE device_handle_ = nullptr;
};

}  // namespace display::test

#endif  // THIRD_PARTY_WIN_VIRTUAL_DISPLAY_CONTROLLER_DISPLAY_DRIVER_CONTROLLER_H_
