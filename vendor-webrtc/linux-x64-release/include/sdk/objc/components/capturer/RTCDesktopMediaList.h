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
#import "RTCDesktopSource.h"

NS_ASSUME_NONNULL_BEGIN

RTC_OBJC_EXPORT
@protocol RTC_OBJC_TYPE
(RTCDesktopMediaListDelegate)<NSObject>

- (void)didDesktopSourceAdded:(RTC_OBJC_TYPE(RTCDesktopSource) *) source;

- (void)didDesktopSourceRemoved:(RTC_OBJC_TYPE(RTCDesktopSource) *) source;

- (void)didDesktopSourceNameChanged:(RTC_OBJC_TYPE(RTCDesktopSource) *) source;

- (void)didDesktopSourceThumbnailChanged:(RTC_OBJC_TYPE(RTCDesktopSource) *) source;
@end

RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCDesktopMediaList) : NSObject

-(instancetype)initWithType:(RTC_OBJC_TYPE(RTCDesktopSourceType))type delegate:(__weak id<RTC_OBJC_TYPE(RTCDesktopMediaListDelegate)>)delegate;

@property(nonatomic, readonly) RTC_OBJC_TYPE(RTCDesktopSourceType) sourceType;

- (int32_t)UpdateSourceList:(BOOL)forceReload  updateAllThumbnails:(BOOL)updateThumbnail;

- (NSArray<RTC_OBJC_TYPE (RTCDesktopSource) *>*) getSources;

@end

NS_ASSUME_NONNULL_END
