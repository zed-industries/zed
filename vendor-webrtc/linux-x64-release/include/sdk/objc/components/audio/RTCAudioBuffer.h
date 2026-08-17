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

#import "RTCMacros.h"

NS_ASSUME_NONNULL_BEGIN

RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCAudioBuffer) : NSObject

@property(nonatomic, readonly) size_t channels;
@property(nonatomic, readonly) size_t frames;
@property(nonatomic, readonly) size_t framesPerBand;
@property(nonatomic, readonly) size_t bands;

// Returns pointer arrays. Index range from 0 to `frames`.
- (float* _Nonnull)rawBufferForChannel:(size_t)channel;

// TODO: More convenience methods...

@end

NS_ASSUME_NONNULL_END
