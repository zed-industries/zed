// Copyright (c) Microsoft Corporation

#ifndef THIRD_PARTY_WIN_VIRTUAL_DISPLAY_DRIVER_SWAPCHAINPROCESSOR_H_
#define THIRD_PARTY_WIN_VIRTUAL_DISPLAY_DRIVER_SWAPCHAINPROCESSOR_H_

// Make sure we don't get min/max macros
#ifndef NOMINMAX
#define NOMINMAX
#endif

#include <windows.h>

#include <avrt.h>

#include <memory>

#include "Direct3DDevice.h"
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
/// <summary>
/// Manages a thread that consumes buffers from an indirect display swap-chain
/// object.
/// </summary>
class SwapChainProcessor {
 public:
  SwapChainProcessor(IDDCX_SWAPCHAIN hSwapChain,
                     std::unique_ptr<display::test::Direct3DDevice> Device,
                     HANDLE NewFrameEvent);
  ~SwapChainProcessor();

 private:
  static DWORD CALLBACK RunThread(LPVOID Argument);

  void Run();
  void RunCore();

  IDDCX_SWAPCHAIN m_hSwapChain;
  std::unique_ptr<display::test::Direct3DDevice> m_Device;
  HANDLE m_hAvailableBufferEvent;
  Microsoft::WRL::Wrappers::Thread m_hThread;
  Microsoft::WRL::Wrappers::Event m_hTerminateEvent;
};
}  // namespace display::test

#endif  // THIRD_PARTY_WIN_VIRTUAL_DISPLAY_DRIVER_SWAPCHAINPROCESSOR_H_
