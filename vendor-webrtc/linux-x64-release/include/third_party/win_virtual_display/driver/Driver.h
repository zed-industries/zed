// Copyright (c) Microsoft Corporation

#ifndef THIRD_PARTY_WIN_VIRTUAL_DISPLAY_DRIVER_DRIVER_H_
#define THIRD_PARTY_WIN_VIRTUAL_DISPLAY_DRIVER_DRIVER_H_

// Make sure we don't get min/max macros
#ifndef NOMINMAX
#define NOMINMAX
#endif

#include <windows.h>

#include "Direct3DDevice.h"
#include "IndirectMonitor.h"
#include "SwapChainProcessor.h"
#include "Trace.h"

namespace Microsoft {
namespace WRL {
namespace Wrappers {
// Adds a wrapper for thread handles to the existing set of WRL handle wrapper
// classes
typedef HandleT<HandleTraits::HANDLENullTraits> Thread;
}  // namespace Wrappers
}  // namespace WRL
}  // namespace Microsoft

namespace display::test {

// Contains data and handles related to a single monitor (IDDCX_MONITOR) object.
class IndirectMonitorContext {
 public:
  IndirectMonitorContext(_In_ IDDCX_MONITOR Monitor, IndirectMonitor config);
  virtual ~IndirectMonitorContext();

  void AssignSwapChain(IDDCX_SWAPCHAIN SwapChain,
                       LUID RenderAdapter,
                       HANDLE NewFrameEvent);
  void UnassignSwapChain();
  // Attach this monitor to the adaptor to trigger OS detection.
  NTSTATUS Attach();
  // Detatch this monitor from the adaptor to remove it from the OS.
  NTSTATUS Detach();

  const IndirectMonitor& monitor_config() const { return monitor_config_; }

 private:
  IDDCX_MONITOR m_Monitor;
  // Underlying monitor config and EDID data.
  IndirectMonitor monitor_config_;
  std::unique_ptr<SwapChainProcessor> m_ProcessingThread;
};

// Contains data and handles related to a single device (WDFDEVICE) object.
class IndirectDeviceContext {
 public:
  IndirectDeviceContext(_In_ WDFDEVICE WdfDevice);
  virtual ~IndirectDeviceContext();

  void InitAdapter();
  void FinishInit();
  // Read driver properties and sync any configuration changes.
  void SyncRequestedConfig();

 protected:
  // Array of monitors, indexed by connector values. Each entry contains a
  // connected monitor for the given connector index, or null if no monitor is
  // connected at that connector index.
  std::array<std::unique_ptr<IndirectMonitorContext>,
             DriverProperties::kMaxMonitors>
      monitors;
  WDFDEVICE m_WdfDevice;
  IDDCX_ADAPTER m_Adapter;
  // Background thread to poll configuration changes.
  Microsoft::WRL::Wrappers::Thread m_hThread;

 private:
  static DWORD CALLBACK RunThread(LPVOID Argument);
  // Adds and attaches a new monitor.
  NTSTATUS AddMonitor(IndirectMonitor monitor);
};

}  // namespace display::test

#endif  // THIRD_PARTY_WIN_VIRTUAL_DISPLAY_DRIVER_DRIVER_H_
