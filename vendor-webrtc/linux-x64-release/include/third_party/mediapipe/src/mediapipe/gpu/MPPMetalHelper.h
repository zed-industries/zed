// Copyright 2019 The MediaPipe Authors.
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

#ifndef MEDIAPIPE_GPU_MEDIAPIPE_METAL_HELPER_H_
#define MEDIAPIPE_GPU_MEDIAPIPE_METAL_HELPER_H_

#import <CoreVideo/CVMetalTextureCache.h>
#import <CoreVideo/CoreVideo.h>
#import <Metal/Metal.h>

#include "mediapipe/framework/packet.h"
#include "mediapipe/framework/packet_type.h"
#include "mediapipe/gpu/gpu_shared_data_internal.h"

NS_ASSUME_NONNULL_BEGIN

@interface MPPMetalHelper : NSObject {
}

- (instancetype)init NS_UNAVAILABLE;

/// Initialize. This initializer is recommended for calculators.
- (instancetype)initWithCalculatorContext:(mediapipe::CalculatorContext *)cc;

/// Initialize.
- (instancetype)initWithGpuResources:(mediapipe::GpuResources *)gpuResources
    NS_DESIGNATED_INITIALIZER;

/// Configures a calculator's contract for accessing GPU resources.
/// Calculators should use this in GetContract.
+ (absl::Status)updateContract:(mediapipe::CalculatorContract *)cc;

/// Deprecated initializer.
- (instancetype)initWithSidePackets:(const mediapipe::PacketSet &)inputSidePackets;

/// Deprecated initializer.
- (instancetype)initWithGpuSharedData:(mediapipe::GpuSharedData *)gpuShared;

/// Configures a calculator's side packets for accessing GPU resources.
/// Calculators should use this in FillExpectations.
+ (absl::Status)setupInputSidePackets:(mediapipe::PacketTypeSet *)inputSidePackets;

/// Get a metal command buffer.
/// Calculators should use this method instead of getting a buffer from the
/// MTLCommandQueue directly, not just for convenience, but also because the
/// framework may want to add some custom hooks to the commandBuffers used by
/// calculators.
- (id<MTLCommandBuffer>)commandBuffer;

/// Creates a CVMetalTextureRef linked to the provided GpuBuffer.
/// Ownership follows the copy rule, so the caller is responsible for
/// releasing the CVMetalTextureRef.
- (CVMetalTextureRef)copyCVMetalTextureWithGpuBuffer:(const mediapipe::GpuBuffer &)gpuBuffer;

/// Creates a CVMetalTextureRef linked to the provided GpuBuffer given a specific plane.
/// Ownership follows the copy rule, so the caller is responsible for
/// releasing the CVMetalTextureRef.
- (CVMetalTextureRef)copyCVMetalTextureWithGpuBuffer:(const mediapipe::GpuBuffer &)gpuBuffer
                                               plane:(size_t)plane;

/// Returns a MTLTexture linked to the provided GpuBuffer.
/// A calculator can freely use it as a rendering source, but it should not
/// use it as a rendering target if the GpuBuffer was provided as an input.
- (id<MTLTexture>)metalTextureWithGpuBuffer:(const mediapipe::GpuBuffer &)gpuBuffer;

/// Returns a MTLTexture linked to the provided GpuBuffer given a specific plane.
/// A calculator can freely use it as a rendering source, but it should not
/// use it as a rendering target if the GpuBuffer was provided as an input.
- (id<MTLTexture>)metalTextureWithGpuBuffer:(const mediapipe::GpuBuffer &)gpuBuffer
                                      plane:(size_t)plane;

/// Obtains a new GpuBuffer to be used as an output destination.
- (mediapipe::GpuBuffer)mediapipeGpuBufferWithWidth:(int)width height:(int)height;

/// Obtains a new GpuBuffer to be used as an output destination.
- (mediapipe::GpuBuffer)mediapipeGpuBufferWithWidth:(int)width
                                             height:(int)height
                                             format:(mediapipe::GpuBufferFormat)format;

/// Convenience method to load a Metal library stored as a bundle resource.
- (id<MTLLibrary>)newLibraryWithResourceName:(NSString *)name error:(NSError *_Nullable *)error;

/// Shared Metal resources.
@property(readonly) id<MTLDevice> mtlDevice;
@property(readonly) id<MTLCommandQueue> mtlCommandQueue;
@property(readonly) CVMetalTextureCacheRef mtlTextureCache;

@end

NS_ASSUME_NONNULL_END

#endif  // MEDIAPIPE_GPU_MEDIAPIPE_METAL_HELPER_H_
