/*
 * Copyright 2025 LiveKit, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#include "livekit/video_frame_buffer.h"

#import <CoreVideo/CoreVideo.h>
#import <sdk/objc/components/video_frame_buffer/RTCCVPixelBuffer.h>
#include "sdk/objc/native/api/video_frame_buffer.h"

namespace livekit_ffi {

std::unique_ptr<VideoFrameBuffer> new_native_buffer_from_platform_image_buffer(
    CVPixelBufferRef pixelBuffer
) {
    RTCCVPixelBuffer *buffer = [[RTCCVPixelBuffer alloc] initWithPixelBuffer:pixelBuffer];
    webrtc::scoped_refptr<webrtc::VideoFrameBuffer> frame_buffer = webrtc::ObjCToNativeVideoFrameBuffer(buffer);
    [buffer release];
    CVPixelBufferRelease(pixelBuffer);
    return std::make_unique<VideoFrameBuffer>(frame_buffer);
}

CVPixelBufferRef native_buffer_to_platform_image_buffer(
    const std::unique_ptr<VideoFrameBuffer> &buffer
) {
    id<RTC_OBJC_TYPE(RTCVideoFrameBuffer)> rtc_pixel_buffer = webrtc::NativeToObjCVideoFrameBuffer(buffer->get());

    if ([rtc_pixel_buffer isKindOfClass:[RTCCVPixelBuffer class]]) {
        RTCCVPixelBuffer *cv_pixel_buffer = (RTCCVPixelBuffer *)rtc_pixel_buffer;
        return [cv_pixel_buffer pixelBuffer];
    } else {
        return nullptr;
    }
}

}  // namespace livekit_ffi
