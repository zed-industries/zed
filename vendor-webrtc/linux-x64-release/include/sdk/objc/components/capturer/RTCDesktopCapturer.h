/*
 * Copyright 2022 LiveKit
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

#import <AVFoundation/AVFoundation.h>
#import <Foundation/Foundation.h>

#import "RTCMacros.h"
#import "RTCVideoCapturer.h"
#import "RTCDesktopSource.h"

NS_ASSUME_NONNULL_BEGIN

@class RTC_OBJC_TYPE(RTCDesktopCapturer);

RTC_OBJC_EXPORT
@protocol RTC_OBJC_TYPE(RTCDesktopCapturerDelegate)<NSObject>
-(void)didSourceCaptureStart:(RTC_OBJC_TYPE(RTCDesktopCapturer) *) capturer;

-(void)didSourceCapturePaused:(RTC_OBJC_TYPE(RTCDesktopCapturer) *) capturer;

-(void)didSourceCaptureStop:(RTC_OBJC_TYPE(RTCDesktopCapturer) *) capturer;

-(void)didSourceCaptureError:(RTC_OBJC_TYPE(RTCDesktopCapturer) *) capturer;
@end

RTC_OBJC_EXPORT
// Screen capture that implements RTCVideoCapturer. Delivers frames to a
// RTCVideoCapturerDelegate (usually RTCVideoSource).
@interface RTC_OBJC_TYPE (RTCDesktopCapturer) : RTC_OBJC_TYPE(RTCVideoCapturer)

@property(nonatomic, readonly) RTC_OBJC_TYPE(RTCDesktopSource) *source;

- (instancetype)initWithSource:(RTC_OBJC_TYPE(RTCDesktopSource) *)source delegate:(__weak id<RTC_OBJC_TYPE(RTCDesktopCapturerDelegate)>)delegate captureDelegate:(__weak id<RTC_OBJC_TYPE(RTCVideoCapturerDelegate)>)captureDelegate;

- (instancetype)initWithDefaultScreen:(__weak id<RTC_OBJC_TYPE(RTCDesktopCapturerDelegate)>)delegate captureDelegate:(__weak id<RTC_OBJC_TYPE(RTCVideoCapturerDelegate)>)captureDelegate;

- (void)startCapture;

- (void)startCaptureWithFPS:(NSInteger)fps;

- (void)stopCapture;

- (void)stopCaptureWithCompletionHandler:(nullable void (^)(void))completionHandler;

@end

NS_ASSUME_NONNULL_END
