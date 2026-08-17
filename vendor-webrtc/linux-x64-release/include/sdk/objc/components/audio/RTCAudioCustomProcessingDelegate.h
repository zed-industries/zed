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

@class RTC_OBJC_TYPE(RTCAudioBuffer);

RTC_OBJC_EXPORT @protocol RTC_OBJC_TYPE (RTCAudioCustomProcessingDelegate)<NSObject>

/**
* (Re-)initialize the audio processor.
* This method can be invoked multiple times.
*/
- (void)audioProcessingInitializeWithSampleRate : (size_t)sampleRateHz channels
: (size_t)channels NS_SWIFT_NAME(audioProcessingInitialize(sampleRate:channels:));

/**
 * Process (read or write) the audio buffer.
 * RTCAudioBuffer is a simple wrapper for webrtc::AudioBuffer and the valid scope is only inside
 * this method. Do not retain it.
 */
- (void)audioProcessingProcess:(RTC_OBJC_TYPE(RTCAudioBuffer) *)audioBuffer
    NS_SWIFT_NAME(audioProcessingProcess(audioBuffer:));

// TOOD:
// virtual void SetRuntimeSetting(AudioProcessing::RuntimeSetting setting);

/**
 * Suggests releasing resources allocated by the audio processor.
 */
- (void)audioProcessingRelease;

@end

NS_ASSUME_NONNULL_END
