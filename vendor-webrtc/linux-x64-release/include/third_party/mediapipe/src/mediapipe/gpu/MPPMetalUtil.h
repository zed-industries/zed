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

#ifndef MEDIAPIPE_GPU_MPP_METAL_UTIL_H_
#define MEDIAPIPE_GPU_MPP_METAL_UTIL_H_

#import <CoreVideo/CVMetalTextureCache.h>
#import <CoreVideo/CoreVideo.h>
#import <Metal/Metal.h>

NS_ASSUME_NONNULL_BEGIN

@interface MPPMetalUtil : NSObject {
}

/// Copies a Metal Buffer from source to destination.
/// Uses blitCommandEncoder and assumes offset of 0.
+ (void)blitMetalBufferTo:(id<MTLBuffer>)destination
                     from:(id<MTLBuffer>)source
                 blocking:(bool)blocking
            commandBuffer:(id<MTLCommandBuffer>)commandBuffer;

/// Copies a Metal Buffer from source to destination.
/// Simple wrapper for blitCommandEncoder.
/// Optionally block until operation is completed.
+ (void)blitMetalBufferTo:(id<MTLBuffer>)destination
        destinationOffset:(int)destinationOffset
                     from:(id<MTLBuffer>)source
             sourceOffset:(int)sourceOffset
                    bytes:(size_t)bytes
                 blocking:(bool)blocking
            commandBuffer:(id<MTLCommandBuffer>)commandBuffer;

/// Commits the command buffer and waits until execution completes. This is
/// functionally equivalent to calling commit and waitUntilCompleted on the
/// command buffer, but may use different synchronization strategies, such as
/// active wait.
+ (void)commitCommandBufferAndWait:(id<MTLCommandBuffer>)commandBuffer;

@end

NS_ASSUME_NONNULL_END

#endif  // MEDIAPIPE_GPU_MPP_METAL_UTIL_H_
