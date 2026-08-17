// Copyright 2021 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_GPU_WEBGPU_WEBGPU_SERVICE_H_
#define MEDIAPIPE_GPU_WEBGPU_WEBGPU_SERVICE_H_

#include <memory>
#include <utility>

#include "absl/base/attributes.h"
#include "absl/container/flat_hash_map.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "mediapipe/framework/deps/no_destructor.h"
#include "mediapipe/framework/graph_service.h"
#include "mediapipe/gpu/attachments.h"
#include "mediapipe/gpu/webgpu/webgpu_check.h"

#ifdef __EMSCRIPTEN__
#include <webgpu/webgpu_cpp.h>
#else
#include "mediapipe/gpu/webgpu/webgpu_device_registration.h"
#include "third_party/dawn/include/webgpu/webgpu_cpp.h"
#endif  // __EMSCRIPTEN__

namespace mediapipe {

// Attachments can be used to cache common resouces that are associated with
// a device, similarly to what we have for GlContext.
template <class T>
using WebGpuDeviceAttachment = internal::Attachment<wgpu::Device, T>;

template <class T>
T& GetWebGpuDeviceCachedAttachment(const wgpu::Device& device,
                                   const WebGpuDeviceAttachment<T>& attachment);

class WebGpuService {
 public:
  static absl::StatusOr<std::shared_ptr<WebGpuService>> Create() {
    if (IsWebGpuAvailable()) {
      // Using bare new to invoke private constructor.
      return std::shared_ptr<WebGpuService>(new WebGpuService());
    } else {
      return absl::UnavailableError("WebGpu is not available");
    }
  }

  // Note: some clients set DISABLE_DEPRECATED_FIND_EVENT_TARGET_BEHAVIOR=0, so
  // we have to use ids rather than selectors for now.
  // However, note that if we transition to selectors, we will need to change
  // our WebGL canvas handling logic accordingly, and in particular we want to
  // preserve our ability to use canvases not parented to the DOM.
  const char* canvas_selector() const { return canvas_selector_; }

  wgpu::Device device() const { return device_; }

#ifdef __EMSCRIPTEN__
  const wgpu::AdapterInfo& adapter_info() const {
    return *reinterpret_cast<const wgpu::AdapterInfo*>(&adapter_info_);
  }
#endif  // __EMSCRIPTEN__

 private:
  WebGpuService();

  const char* canvas_selector_;
  wgpu::Device device_;
#ifdef __EMSCRIPTEN__
  // Adapter is not yet piped through Emscripten (i.e. `device.GetAdapter()`).
  // Instead we pass GPUAdapterInfo obtained in TypeScript via
  // `GPUAdapter.requestAdapterInfo()` as a part of
  // `preinitializedWebGPUDevice`. Ideally we would want to pass it as a
  // separate object (or more precisely pointer to object in JsValStore), but
  // MediaPipe services don't support parameterized constructors.
  //
  WGPUAdapterInfo adapter_info_;
#endif  // __EMSCRIPTEN__
};

ABSL_CONST_INIT extern const GraphService<WebGpuService> kWebGpuService;

namespace internal {

class WebGpuDeviceAttachmentManager {
 public:
  explicit WebGpuDeviceAttachmentManager(wgpu::Device device)
      : device_(std::move(device)) {}

  // TOOD: const result?
  template <class T>
  T& GetCachedAttachment(const WebGpuDeviceAttachment<T>& attachment) {
    internal::AttachmentPtr<void>& entry = attachments_[&attachment];
    if (entry == nullptr) {
      entry = attachment.factory()(device_);
    }
    return *static_cast<T*>(entry.get());
  }

  const wgpu::Device& device() const { return device_; }

 private:
  wgpu::Device device_;
  absl::flat_hash_map<const AttachmentBase<wgpu::Device>*, AttachmentPtr<void>>
      attachments_;
};

#ifdef __EMSCRIPTEN__
static WebGpuDeviceAttachmentManager& GetEmscriptenDeviceAttachmentManager() {
  static mediapipe::NoDestructor<WebGpuDeviceAttachmentManager> manager(
      wgpu::Device::Acquire(emscripten_webgpu_get_device()));
  return *manager;
}
#else
static WebGpuDeviceAttachmentManager& GetNativeDeviceAttachmentManager() {
  static mediapipe::NoDestructor<WebGpuDeviceAttachmentManager> manager(
      wgpu::Device(WebGpuDeviceRegistration::GetInstance().GetWebGpuDevice()));
  return *manager;
}
#endif  // __EMSCRIPTEN__

}  // namespace internal

#ifdef __EMSCRIPTEN__
template <class T>
T& GetWebGpuDeviceCachedAttachment(
    const wgpu::Device& device, const WebGpuDeviceAttachment<T>& attachment) {
  // Currently we only handle the single device given to Emscripten.
  auto& attachments = internal::GetEmscriptenDeviceAttachmentManager();
  // Note: emscripten_webgpu_get_device, in spite of its name, creates a new
  // wrapper with a new handle each time it's called, even though they all
  // refer to the same device. TODO: fix it in upstream. For now we
  // just rely the assumption that there is one device.
  // ABSL_CHECK_EQ(device.Get(), attachments.device().Get());
  return attachments.GetCachedAttachment<T>(attachment);
}
#else
template <class T>
T& GetWebGpuDeviceCachedAttachment(
    const wgpu::Device& device, const WebGpuDeviceAttachment<T>& attachment) {
  auto& attachments = internal::GetNativeDeviceAttachmentManager();
  return attachments.GetCachedAttachment<T>(attachment);
}
#endif  // __EMSCRIPTEN__

}  // namespace mediapipe

#endif  // MEDIAPIPE_GPU_WEBGPU_WEBGPU_SERVICE_H_
