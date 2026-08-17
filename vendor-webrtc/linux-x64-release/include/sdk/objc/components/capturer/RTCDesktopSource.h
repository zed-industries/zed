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
#import <AppKit/AppKit.h>
#import <AVFoundation/AVFoundation.h>
#import <Foundation/Foundation.h>

#import "RTCMacros.h"

typedef NS_ENUM(NSInteger, RTC_OBJC_TYPE(RTCDesktopSourceType)) {
  RTC_OBJC_TYPE(RTCDesktopSourceTypeScreen),
  RTC_OBJC_TYPE(RTCDesktopSourceTypeWindow),
};

RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCDesktopSource) : NSObject

@property(nonatomic, readonly) NSString *sourceId;

@property(nonatomic, readonly) NSString *name;

@property(nonatomic, readonly) NSImage *thumbnail;

@property(nonatomic, readonly) RTC_OBJC_TYPE(RTCDesktopSourceType) sourceType;

-( NSImage *)UpdateThumbnail;

@end