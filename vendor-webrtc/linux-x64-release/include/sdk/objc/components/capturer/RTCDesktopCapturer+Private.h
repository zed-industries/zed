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

#import "RTCDesktopCapturer.h"

#include "sdk/objc/native/src/objc_desktop_capture.h"

NS_ASSUME_NONNULL_BEGIN

RTC_OBJC_EXPORT
@protocol RTC_OBJC_TYPE(RTCDesktopCapturerPrivateDelegate)<NSObject>
-(void)didCaptureVideoFrame:(RTC_OBJC_TYPE(RTCVideoFrame) *) frame;
-(void)didSourceCaptureStart;
-(void)didSourceCapturePaused;
-(void)didSourceCaptureStop;
-(void)didSourceCaptureError;
@end

@interface RTC_OBJC_TYPE(RTCDesktopCapturer) ()

@property(nonatomic, readonly)std::shared_ptr<webrtc::ObjCDesktopCapturer> nativeCapturer;

- (void)didCaptureVideoFrame:(RTC_OBJC_TYPE(RTCVideoFrame) *)frame;

-(void)didSourceCaptureStart;

-(void)didSourceCapturePaused;

-(void)didSourceCaptureStop;

-(void)didSourceCaptureError;

@end

NS_ASSUME_NONNULL_END