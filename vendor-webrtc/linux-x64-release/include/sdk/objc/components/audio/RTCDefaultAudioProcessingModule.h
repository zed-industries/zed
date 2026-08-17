/*
 * Copyright 2023 LiveKit
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

#import <Foundation/Foundation.h>

#import "RTCAudioProcessingModule.h"
#import "RTCMacros.h"

NS_ASSUME_NONNULL_BEGIN

@class RTC_OBJC_TYPE(RTCAudioProcessingConfig);
@protocol RTC_OBJC_TYPE (RTCAudioCustomProcessingDelegate);

RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCDefaultAudioProcessingModule) : NSObject <RTC_OBJC_TYPE(RTCAudioProcessingModule)>

- (instancetype)initWithConfig: (nullable RTC_OBJC_TYPE(RTCAudioProcessingConfig) *)config
 capturePostProcessingDelegate: (nullable id<RTC_OBJC_TYPE(RTCAudioCustomProcessingDelegate)>)capturePostProcessingDelegate
   renderPreProcessingDelegate: (nullable id<RTC_OBJC_TYPE(RTCAudioCustomProcessingDelegate)>)renderPreProcessingDelegate
   NS_SWIFT_NAME(init(config:capturePostProcessingDelegate:renderPreProcessingDelegate:)) NS_DESIGNATED_INITIALIZER;

@property(nonatomic, assign) RTC_OBJC_TYPE(RTCAudioProcessingConfig) * config;
@property(nonatomic, assign, getter=isMuted) BOOL muted;

// Dynamically update delegates at runtime

@property(nonatomic, weak, nullable) id<RTC_OBJC_TYPE(RTCAudioCustomProcessingDelegate)>
    capturePostProcessingDelegate;
@property(nonatomic, weak, nullable) id<RTC_OBJC_TYPE(RTCAudioCustomProcessingDelegate)>
    renderPreProcessingDelegate;

@end

NS_ASSUME_NONNULL_END
