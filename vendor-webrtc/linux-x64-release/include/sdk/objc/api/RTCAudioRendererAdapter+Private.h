/*
 * Copyright 2024 LiveKit
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

#import "RTCAudioRendererAdapter.h"

#import "base/RTCAudioRenderer.h"

#include "api/media_stream_interface.h"

NS_ASSUME_NONNULL_BEGIN

@interface RTC_OBJC_TYPE(RTCAudioRendererAdapter) ()

@property(nonatomic, readonly) id<RTC_OBJC_TYPE(RTCAudioRenderer)> audioRenderer;

@property(nonatomic, readonly) webrtc::AudioTrackSinkInterface *nativeAudioRenderer;

- (instancetype)initWithNativeRenderer:(id<RTC_OBJC_TYPE(RTCAudioRenderer)>)audioRenderer
    NS_DESIGNATED_INITIALIZER;

@end

NS_ASSUME_NONNULL_END
