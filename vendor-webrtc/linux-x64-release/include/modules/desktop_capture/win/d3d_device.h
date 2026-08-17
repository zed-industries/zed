/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_WIN_D3D_DEVICE_H_
#define MODULES_DESKTOP_CAPTURE_WIN_D3D_DEVICE_H_

#include <comdef.h>
#include <d3d11.h>
#include <dxgi.h>
#include <wrl/client.h>

#include <vector>

namespace webrtc {

// A wrapper of ID3D11Device and its corresponding context and IDXGIAdapter.
// This class represents one video card in the system.
class D3dDevice {
 public:
  D3dDevice(const D3dDevice& other);
  D3dDevice(D3dDevice&& other);
  ~D3dDevice();

  ID3D11Device* d3d_device() const { return d3d_device_.Get(); }

  ID3D11DeviceContext* context() const { return context_.Get(); }

  IDXGIDevice* dxgi_device() const { return dxgi_device_.Get(); }

  IDXGIAdapter* dxgi_adapter() const { return dxgi_adapter_.Get(); }

  // Returns all D3dDevice instances on the system. Returns an empty vector if
  // anything wrong.
  static std::vector<D3dDevice> EnumDevices();

 private:
  // Instances of D3dDevice should only be created by EnumDevices() static
  // function.
  D3dDevice();

  // Initializes the D3dDevice from an IDXGIAdapter.
  bool Initialize(const Microsoft::WRL::ComPtr<IDXGIAdapter>& adapter);

  Microsoft::WRL::ComPtr<ID3D11Device> d3d_device_;
  Microsoft::WRL::ComPtr<ID3D11DeviceContext> context_;
  Microsoft::WRL::ComPtr<IDXGIDevice> dxgi_device_;
  Microsoft::WRL::ComPtr<IDXGIAdapter> dxgi_adapter_;
};

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_WIN_D3D_DEVICE_H_
