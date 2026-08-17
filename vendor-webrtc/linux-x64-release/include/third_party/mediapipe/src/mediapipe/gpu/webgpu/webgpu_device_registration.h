// Copyright 2024 The MediaPipe Authors.
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

#ifndef MEDIAPIPE_GPU_WEBGPU_WEBGPU_DEVICE_REGISTRATION_H_
#define MEDIAPIPE_GPU_WEBGPU_WEBGPU_DEVICE_REGISTRATION_H_

#include "mediapipe/framework/deps/no_destructor.h"
#include "third_party/dawn/include/webgpu/webgpu_cpp.h"

namespace mediapipe {

// Singleton class to register and unregister WebGPU device for native clients.
class WebGpuDeviceRegistration {
 public:
  static WebGpuDeviceRegistration& GetInstance();

  WebGpuDeviceRegistration(const WebGpuDeviceRegistration&) = delete;
  WebGpuDeviceRegistration& operator=(const WebGpuDeviceRegistration&) = delete;

  void RegisterWebGpuDevice(wgpu::Device device);

  void UnRegisterWebGpuDevice();

  wgpu::Device GetWebGpuDevice() const { return device_; }

 private:
  friend class NoDestructor<WebGpuDeviceRegistration>;

  WebGpuDeviceRegistration();
  ~WebGpuDeviceRegistration();

  wgpu::Device device_;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_GPU_WEBGPU_WEBGPU_DEVICE_REGISTRATION_H_
