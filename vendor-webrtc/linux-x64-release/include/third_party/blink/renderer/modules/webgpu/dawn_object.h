// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_DAWN_OBJECT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_DAWN_OBJECT_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/graphics/gpu/dawn_control_client_holder.h"
#include "third_party/blink/renderer/platform/graphics/gpu/webgpu_cpp.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace gpu {
namespace webgpu {

class WebGPUInterface;

}  // namespace webgpu
}  // namespace gpu

namespace blink {

namespace scheduler {
class EventLoop;
}  // namespace scheduler

class GPUDevice;

// This class allows objects to hold onto a DawnControlClientHolder.
// The DawnControlClientHolder is used to hold the WebGPUInterface and keep
// track of whether or not the client has been destroyed. If the client is
// destroyed, we should not call any Dawn functions.
class DawnObjectBase {
 public:
  explicit DawnObjectBase(
      scoped_refptr<DawnControlClientHolder> dawn_control_client,
      const String& label);

  const scoped_refptr<DawnControlClientHolder>& GetDawnControlClient() const;
  base::WeakPtr<WebGraphicsContext3DProviderWrapper> GetContextProviderWeakPtr()
      const {
    return dawn_control_client_->GetContextProviderWeakPtr();
  }

  // Ensure commands up until now on this object's parent device are flushed by
  // the end of the task.
  void EnsureFlush(scheduler::EventLoop& event_loop);

  // Flush commands up until now on this object's parent device immediately.
  void FlushNow();

  // GPUObjectBase mixin implementation
  const String& label() const { return label_; }
  void setLabel(const String& value);

  virtual void SetLabelImpl(const String& value) = 0;

 private:
  scoped_refptr<DawnControlClientHolder> dawn_control_client_;
  String label_;
};

class DawnObjectImpl : public ScriptWrappable, public DawnObjectBase {
 public:
  explicit DawnObjectImpl(GPUDevice* device, const String& label);
  ~DawnObjectImpl() override;

  const wgpu::Device& GetDeviceHandle() const;
  GPUDevice* device() { return device_.Get(); }

  void Trace(Visitor* visitor) const override;

 protected:
  Member<GPUDevice> device_;
};

template <typename Obj>
class DawnObject : public DawnObjectImpl {
 public:
  DawnObject(GPUDevice* device, Obj handle, const String& label)
      : DawnObjectImpl(device, label),
        device_handle_(GetDeviceHandle()),
        handle_(std::move(handle)) {}

  const Obj& GetHandle() const { return handle_; }

 private:
  // All WebGPU Blink objects created directly or by the Device hold a
  // Member<GPUDevice> which keeps the device alive. However, this does not
  // enforce that the GPUDevice is finalized after all objects referencing it.
  // Declare the device as a member first before the object to ensure that the
  // Dawn device is destroyed last.
  // TODO(enga): Investigate removing Member<GPUDevice>.
  const wgpu::Device device_handle_;
  const Obj handle_;
};

template <>
class DawnObject<wgpu::Device> : public DawnObjectBase {
 public:
  const wgpu::Device& GetHandle() const { return handle_; }

 protected:
  // Support setting the handle after creation to allow for GPUDevice's
  // two-step initialization.
  using DawnObjectBase::DawnObjectBase;
  void SetHandle(wgpu::Device handle) { handle_ = std::move(handle); }

 private:
  wgpu::Device handle_;
};

template <>
class DawnObject<wgpu::Adapter> : public DawnObjectBase {
 public:
  DawnObject(scoped_refptr<DawnControlClientHolder> dawn_control_client,
             wgpu::Adapter handle,
             const String& label)
      : DawnObjectBase(dawn_control_client, label),
        handle_(std::move(handle)) {}

  const wgpu::Adapter& GetHandle() const { return handle_; }

 private:
  const wgpu::Adapter handle_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_DAWN_OBJECT_H_
