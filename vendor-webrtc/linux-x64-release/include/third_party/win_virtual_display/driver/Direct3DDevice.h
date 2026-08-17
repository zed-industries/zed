// Copyright (c) Microsoft Corporation

#ifndef THIRD_PARTY_WIN_VIRTUAL_DISPLAY_DRIVER_DIRECT3DDEVICE_H_
#define THIRD_PARTY_WIN_VIRTUAL_DISPLAY_DRIVER_DIRECT3DDEVICE_H_

// Make sure we don't get min/max macros
#ifndef NOMINMAX
#define NOMINMAX
#endif

#include <windows.h>

#include <wdf.h>

#include <iddcx.h>
#include <wrl.h>

namespace display::test {
// Manages the creation and lifetime of a Direct3D render device.
struct Direct3DDevice {
  Direct3DDevice(LUID AdapterLuid);
  Direct3DDevice();
  HRESULT Init();

  LUID AdapterLuid;
  Microsoft::WRL::ComPtr<IDXGIFactory5> DxgiFactory;
  Microsoft::WRL::ComPtr<IDXGIAdapter1> Adapter;
  Microsoft::WRL::ComPtr<ID3D11Device> Device;
  Microsoft::WRL::ComPtr<ID3D11DeviceContext> DeviceContext;
};
}  // namespace display::test

#endif  // THIRD_PARTY_WIN_VIRTUAL_DISPLAY_DRIVER_DIRECT3DDEVICE_H_
