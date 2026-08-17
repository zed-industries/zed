/*
 * Copyright © Microsoft Corporation
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the next
 * paragraph) shall be included in all copies or substantial portions of the
 * Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
 * IN THE SOFTWARE.
 */

 #include "vaapi_display_win32.h"

#include <directx/dxcore_interface.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

static const char* g_device_name;

void dxcore_resolve_adapter(const char* adapter_string,
                            /*out*/ bool* ptr_device_found,
                            /*out*/ LUID* ptr_adapter_luid) {
  int selected_adapter_index = -1;
  IDXCoreAdapterFactory* factory = nullptr;
  IDXCoreAdapterList* adapter_list = nullptr;
  IDXCoreAdapter* adapter = nullptr;
  typedef HRESULT(WINAPI * PFN_CREATE_DXCORE_ADAPTER_FACTORY)(REFIID riid,
                                                              void** ppFactory);
  PFN_CREATE_DXCORE_ADAPTER_FACTORY DXCoreCreateAdapterFactory;
  HRESULT hr = S_OK;

  memset(ptr_adapter_luid, 0, sizeof(LUID));
  *ptr_device_found = false;

  HMODULE dxcore_mod = LoadLibraryA("DXCore.DLL");
  if (!dxcore_mod) {
    fprintf(stderr, "Failed to load DXCore.DLL to enumerate adapters.\n");
    goto fail;
  }

  DXCoreCreateAdapterFactory =
      (PFN_CREATE_DXCORE_ADAPTER_FACTORY)GetProcAddress(
          dxcore_mod, "DXCoreCreateAdapterFactory");
  if (!DXCoreCreateAdapterFactory) {
    fprintf(stderr,
            "Failed to load DXCoreCreateAdapterFactory from DXCore.DLL.\n");
    goto fail;
  }

  hr = DXCoreCreateAdapterFactory(IID_IDXCoreAdapterFactory, (void**)&factory);
  if (FAILED(hr)) {
    fprintf(stderr, "DXCoreCreateAdapterFactory failed: %lx\n", hr);
    goto fail;
  }

  hr =
      factory->CreateAdapterList(1, &DXCORE_ADAPTER_ATTRIBUTE_D3D12_GRAPHICS,
                                 IID_IDXCoreAdapterList, (void**)&adapter_list);
  if (FAILED(hr)) {
    fprintf(stderr, "CreateAdapterList failed: %lx\n", hr);
    goto fail;
  }

  if (adapter_string &&
      (sscanf_s(adapter_string, "%d", &selected_adapter_index) != 1)) {
    fprintf(stderr, "Invalid device index received for -hwaccel_device %s\n",
            adapter_string ? adapter_string : "");
  }

  if (!adapter_string)
    fprintf(stdout, "Available devices for --display win32:\n");
  for (int i = 0; i < adapter_list->GetAdapterCount(); i++) {
    if (SUCCEEDED(adapter_list->GetAdapter(i, IID_IDXCoreAdapter,
                                           (void**)&adapter))) {
      size_t desc_size = 0;
      if (FAILED(adapter->GetPropertySize(
              DXCoreAdapterProperty::DriverDescription, &desc_size))) {
        adapter->Release();
        continue;
      }

      char* adapter_name = (char*)malloc(desc_size);
      if (!adapter_name) {
        adapter->Release();
        continue;
      }

      if (FAILED(adapter->GetProperty(DXCoreAdapterProperty::DriverDescription,
                                      desc_size, adapter_name))) {
        free(adapter_name);
        adapter->Release();
        continue;
      }

      LUID cur_adapter_luid = {0, 0};
      if (FAILED(adapter->GetProperty(DXCoreAdapterProperty::InstanceLuid,
                                      &cur_adapter_luid))) {
        free(adapter_name);
        adapter->Release();
        continue;
      }

      if (selected_adapter_index == i) {
        *ptr_adapter_luid = cur_adapter_luid;
        *ptr_device_found = true;
      }

      if (!adapter_string)
        fprintf(stdout,
                "\tDevice Index: %d Device LUID: %lu %ld - Device Name: %s\n",
                i, cur_adapter_luid.LowPart, cur_adapter_luid.HighPart,
                adapter_name);
      free(adapter_name);
      adapter->Release();
    }
  }

fail:
  if (adapter_list)
    adapter_list->Release();
  if (factory)
    factory->Release();
  if (dxcore_mod)
    FreeLibrary(dxcore_mod);
}

static VADisplay va_open_display_win32(void) {
  LUID adapter_luid = {0, 0};
  bool device_found = false;
  if (g_device_name) {
    bool print_devices = (0 == strcmp(g_device_name, "help"));
    dxcore_resolve_adapter(print_devices ? NULL : g_device_name, &device_found,
                           &adapter_luid);
    if (print_devices) {
      exit(0);
    } else if (g_device_name && !device_found) {
      fprintf(stderr,
              "Could not find device %s for --display win32. Please try "
              "--device help for a list of available devices.\n",
              g_device_name);
      exit(0);
    }
  }

  // Adapter automatic selection supported by sending NULL adapter to
  // vaGetDisplayWin32
  return vaGetDisplayWin32(device_found ? &adapter_luid : NULL);
}

static void va_close_display_win32(VADisplay va_dpy) {}

namespace livekit_ffi {

VaapiDisplayWin32::VaapiDisplayWin32() : va_display_(nullptr) {
  putenv("LIBVA_DRIVER_NAME=vaon12");
  putenv("LIBVA_DRIVERS_PATH=.");
}

bool VaapiDisplayWin32::Open() {
  va_display_ = va_open_display_win32();
  if (!va_display_) {
    fprintf(stderr, "Failed to open VA display\n");
    return false;
  }
  return true;
}

bool VaapiDisplayWin32::isOpen() const {
  return va_display_ != nullptr;
}

void VaapiDisplayWin32::Close() {
  if (va_display_) {
    va_close_display_win32(va_display_);
    va_display_ = nullptr;
  }
}

}  // namespace livekit_ffi
