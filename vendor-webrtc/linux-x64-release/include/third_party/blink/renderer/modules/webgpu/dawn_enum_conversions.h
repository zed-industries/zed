#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_DAWN_ENUM_CONVERSIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_DAWN_ENUM_CONVERSIONS_H_

#include "third_party/blink/renderer/platform/graphics/gpu/webgpu_cpp.h"

namespace blink {

class V8GPUBufferBindingType;
class V8GPUBufferMapState;
class V8GPUSamplerBindingType;
class V8GPUTextureSampleType;
class V8GPUStorageTextureAccess;
class V8GPUCompareFunction;
class V8GPUQueryType;
class V8GPUTextureFormat;
class V8GPUTextureDimension;
class V8GPUTextureViewDimension;
class V8GPUStencilOperation;
class V8GPUStoreOp;
class V8GPULoadOp;
class V8GPUFeatureName;
class V8GPUIndexFormat;
class V8GPUPrimitiveTopology;
class V8GPUBlendFactor;
class V8GPUBlendOperation;
class V8GPUVertexStepMode;
class V8GPUVertexFormat;
class V8GPUAddressMode;
class V8GPUMipmapFilterMode;
class V8GPUFilterMode;
class V8GPUCullMode;
class V8GPUFrontFace;
class V8GPUTextureAspect;
class V8GPUErrorFilter;
enum class PredefinedColorSpace;

// Convert WebGPU bitfield values to Dawn enums. These have the same value.
template <typename DawnFlags>
DawnFlags AsDawnFlags(uint32_t webgpu_enum) {
  return static_cast<DawnFlags>(webgpu_enum);
}

// Convert WebGPU IDL enums to Dawn enums.
wgpu::BufferBindingType AsDawnEnum(const V8GPUBufferBindingType& webgpu_enum);
wgpu::SamplerBindingType AsDawnEnum(const V8GPUSamplerBindingType& webgpu_enum);
wgpu::TextureSampleType AsDawnEnum(const V8GPUTextureSampleType& webgpu_enum);
wgpu::StorageTextureAccess AsDawnEnum(
    const V8GPUStorageTextureAccess& webgpu_enum);
wgpu::CompareFunction AsDawnEnum(const V8GPUCompareFunction& webgpu_enum);
wgpu::QueryType AsDawnEnum(const V8GPUQueryType& webgpu_enum);
wgpu::TextureFormat AsDawnEnum(const V8GPUTextureFormat& webgpu_enum);
wgpu::TextureDimension AsDawnEnum(const V8GPUTextureDimension& webgpu_enum);
wgpu::TextureViewDimension AsDawnEnum(
    const V8GPUTextureViewDimension& webgpu_enum);
wgpu::StencilOperation AsDawnEnum(const V8GPUStencilOperation& webgpu_enum);
wgpu::StoreOp AsDawnEnum(const V8GPUStoreOp& webgpu_enum);
wgpu::LoadOp AsDawnEnum(const V8GPULoadOp& webgpu_enum);
wgpu::IndexFormat AsDawnEnum(const V8GPUIndexFormat& webgpu_enum);
wgpu::FeatureName AsDawnEnum(const V8GPUFeatureName& webgpu_enum);
wgpu::PrimitiveTopology AsDawnEnum(const V8GPUPrimitiveTopology& webgpu_enum);
wgpu::BlendFactor AsDawnEnum(const V8GPUBlendFactor& webgpu_enum);
wgpu::BlendOperation AsDawnEnum(const V8GPUBlendOperation& webgpu_enum);
wgpu::VertexStepMode AsDawnEnum(const V8GPUVertexStepMode& webgpu_enum);
wgpu::VertexFormat AsDawnEnum(const V8GPUVertexFormat& webgpu_enum);
wgpu::AddressMode AsDawnEnum(const V8GPUAddressMode& webgpu_enum);
wgpu::FilterMode AsDawnEnum(const V8GPUFilterMode& webgpu_enum);
wgpu::MipmapFilterMode AsDawnEnum(const V8GPUMipmapFilterMode& webgpu_enum);
wgpu::CullMode AsDawnEnum(const V8GPUCullMode& webgpu_enum);
wgpu::FrontFace AsDawnEnum(const V8GPUFrontFace& webgpu_enum);
wgpu::TextureAspect AsDawnEnum(const V8GPUTextureAspect& webgpu_enum);
wgpu::ErrorFilter AsDawnEnum(const V8GPUErrorFilter& webgpu_enum);

// Convert Dawn enums to WebGPU IDL enums.
V8GPUQueryType FromDawnEnum(wgpu::QueryType dawn_enum);
V8GPUTextureDimension FromDawnEnum(wgpu::TextureDimension dawn_enum);
V8GPUTextureFormat FromDawnEnum(wgpu::TextureFormat dawn_enum);
V8GPUBufferMapState FromDawnEnum(wgpu::BufferMapState dawn_enum);
const char* FromDawnEnum(wgpu::BackendType dawn_enum);
const char* FromDawnEnum(wgpu::AdapterType dawn_enum);
const char* FromDawnEnum(wgpu::PowerPreference dawn_enum);
const char* FromDawnEnum(wgpu::WGSLLanguageFeatureName dawn_enum);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_DAWN_ENUM_CONVERSIONS_H_
