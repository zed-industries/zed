// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_TEXTURE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_TEXTURE_H_

#include "third_party/blink/renderer/modules/webgpu/dawn_object.h"
#include "third_party/blink/renderer/platform/wtf/ref_counted.h"

namespace blink {

class ExceptionState;
class GPUTextureDescriptor;
class GPUTextureView;
class GPUTextureViewDescriptor;
class StaticBitmapImage;
class V8GPUTextureDimension;
class V8GPUTextureFormat;
class WebGPUMailboxTexture;

class GPUTexture : public DawnObject<wgpu::Texture> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static GPUTexture* Create(GPUDevice* device,
                            const GPUTextureDescriptor* webgpu_desc,
                            ExceptionState& exception_state);
  static GPUTexture* Create(GPUDevice* device,
                            const wgpu::TextureDescriptor* desc);
  static GPUTexture* CreateError(GPUDevice* device,
                                 const wgpu::TextureDescriptor* desc);

  GPUTexture(GPUDevice* device, wgpu::Texture texture, const String& label);
  GPUTexture(GPUDevice* device,
             wgpu::TextureFormat format,
             wgpu::TextureUsage usage,
             scoped_refptr<WebGPUMailboxTexture> mailbox_texture,
             const String& label);

  ~GPUTexture() override;

  GPUTexture(const GPUTexture&) = delete;
  GPUTexture& operator=(const GPUTexture&) = delete;

  // gpu_texture.idl {{{
  GPUTextureView* createView(const GPUTextureViewDescriptor* webgpu_desc,
                             ExceptionState& exception_state);
  void destroy();
  uint32_t width() const;
  uint32_t height() const;
  uint32_t depthOrArrayLayers() const;
  uint32_t mipLevelCount() const;
  uint32_t sampleCount() const;
  V8GPUTextureDimension dimension() const;
  V8GPUTextureFormat format() const;
  uint32_t usage() const;
  // }}} End of WebIDL binding implementation.

  wgpu::TextureDimension Dimension() { return dimension_; }
  wgpu::TextureFormat Format() { return format_; }
  wgpu::TextureUsage Usage() { return usage_; }
  bool IsDestroyed() { return destroyed_; }

  void DissociateMailbox();

  // Returns a shared pointer to the mailbox texture. The mailbox texture
  // remains associated to the GPUTexture.
  scoped_refptr<WebGPUMailboxTexture> GetMailboxTexture();

  // Sets a callback which is called if destroy is called manually, before the
  // WebGPU handle is actually destroyed.
  void SetBeforeDestroyCallback(base::OnceClosure);
  void ClearBeforeDestroyCallback();

 private:
  void SetLabelImpl(const String& value) override {
    std::string utf8_label = value.Utf8();
    GetHandle().SetLabel(utf8_label.c_str());
  }

  wgpu::TextureDimension dimension_;
  wgpu::TextureFormat format_;
  wgpu::TextureUsage usage_;
  scoped_refptr<WebGPUMailboxTexture> mailbox_texture_;
  bool destroyed_ = false;
  base::OnceClosure destroy_callback_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_TEXTURE_H_
