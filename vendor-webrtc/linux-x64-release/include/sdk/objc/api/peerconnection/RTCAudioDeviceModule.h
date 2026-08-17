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

#import <AVFAudio/AVFAudio.h>
#import <CoreMedia/CoreMedia.h>
#import <Foundation/Foundation.h>

#import "RTCIODevice.h"
#import "RTCMacros.h"

NS_ASSUME_NONNULL_BEGIN

typedef NS_ENUM(NSInteger, RTC_OBJC_TYPE(RTCAudioDeviceModuleType)) {
  RTC_OBJC_TYPE(RTCAudioDeviceModuleTypePlatformDefault),
  RTC_OBJC_TYPE(RTCAudioDeviceModuleTypeAudioEngine),
};

typedef NS_ENUM(NSInteger, RTC_OBJC_TYPE(RTCSpeechActivityEvent)) {
  RTC_OBJC_TYPE(RTCSpeechActivityEventStarted),
  RTC_OBJC_TYPE(RTCSpeechActivityEventEnded),
};

typedef NS_ENUM(NSInteger, RTC_OBJC_TYPE(RTCAudioEngineMuteMode)) {
  RTC_OBJC_TYPE(RTCAudioEngineMuteModeUnknown) = -1,
  RTC_OBJC_TYPE(RTCAudioEngineMuteModeVoiceProcessing) = 0,
  RTC_OBJC_TYPE(RTCAudioEngineMuteModeRestartEngine) = 1,
  RTC_OBJC_TYPE(RTCAudioEngineMuteModeInputMixer) = 2,
};

// Ducking level for voice processing.
// Maps to AVAudioVoiceProcessingOtherAudioDuckingLevel (iOS 17.0+, macOS 14.0+).
typedef NS_ENUM(NSInteger, RTC_OBJC_TYPE(RTCAudioDuckingLevel)) {
  RTC_OBJC_TYPE(RTCAudioDuckingLevelDefault) = 0,
  RTC_OBJC_TYPE(RTCAudioDuckingLevelMin) = 1,
  RTC_OBJC_TYPE(RTCAudioDuckingLevelMid) = 2,
  RTC_OBJC_TYPE(RTCAudioDuckingLevelMax) = 3,
};

typedef struct {
  BOOL outputEnabled;
  BOOL outputRunning;
  BOOL inputEnabled;
  BOOL inputRunning;
  BOOL inputMuted;
  RTC_OBJC_TYPE(RTCAudioEngineMuteMode) muteMode;
} RTC_OBJC_TYPE(RTCAudioEngineState);

typedef struct {
  BOOL isInputAvailable;
  BOOL isOutputAvailable;
} RTC_OBJC_TYPE(RTCAudioEngineAvailability);

RTC_EXTERN NSString *const RTC_CONSTANT_TYPE(RTCAudioEngineInputMixerNodeKey);

@class RTC_OBJC_TYPE(RTCAudioDeviceModule);

RTC_OBJC_EXPORT @protocol RTC_OBJC_TYPE
(RTCAudioDeviceModuleDelegate)<NSObject>

    - (void)audioDeviceModule
    : (RTC_OBJC_TYPE(RTCAudioDeviceModule) *)audioDeviceModule didReceiveSpeechActivityEvent
    : (RTC_OBJC_TYPE(RTCSpeechActivityEvent))speechActivityEvent NS_SWIFT_NAME(audioDeviceModule(_:didReceiveSpeechActivityEvent:));

// Engine events
- (NSInteger)audioDeviceModule:(RTC_OBJC_TYPE(RTCAudioDeviceModule) *)audioDeviceModule
               didCreateEngine:(AVAudioEngine *)engine
    NS_SWIFT_NAME(audioDeviceModule(_:didCreateEngine:));

- (NSInteger)audioDeviceModule:(RTC_OBJC_TYPE(RTCAudioDeviceModule) *)audioDeviceModule
              willEnableEngine:(AVAudioEngine *)engine
              isPlayoutEnabled:(BOOL)isPlayoutEnabled
            isRecordingEnabled:(BOOL)isRecordingEnabled
    NS_SWIFT_NAME(audioDeviceModule(_:willEnableEngine:isPlayoutEnabled:isRecordingEnabled:));

- (NSInteger)audioDeviceModule:(RTC_OBJC_TYPE(RTCAudioDeviceModule) *)audioDeviceModule
               willStartEngine:(AVAudioEngine *)engine
              isPlayoutEnabled:(BOOL)isPlayoutEnabled
            isRecordingEnabled:(BOOL)isRecordingEnabled
    NS_SWIFT_NAME(audioDeviceModule(_:willStartEngine:isPlayoutEnabled:isRecordingEnabled:));

- (NSInteger)audioDeviceModule:(RTC_OBJC_TYPE(RTCAudioDeviceModule) *)audioDeviceModule
                 didStopEngine:(AVAudioEngine *)engine
              isPlayoutEnabled:(BOOL)isPlayoutEnabled
            isRecordingEnabled:(BOOL)isRecordingEnabled
    NS_SWIFT_NAME(audioDeviceModule(_:didStopEngine:isPlayoutEnabled:isRecordingEnabled:));

- (NSInteger)audioDeviceModule:(RTC_OBJC_TYPE(RTCAudioDeviceModule) *)audioDeviceModule
              didDisableEngine:(AVAudioEngine *)engine
              isPlayoutEnabled:(BOOL)isPlayoutEnabled
            isRecordingEnabled:(BOOL)isRecordingEnabled
    NS_SWIFT_NAME(audioDeviceModule(_:didDisableEngine:isPlayoutEnabled:isRecordingEnabled:));

- (NSInteger)audioDeviceModule:(RTC_OBJC_TYPE(RTCAudioDeviceModule) *)audioDeviceModule
             willReleaseEngine:(AVAudioEngine *)engine
    NS_SWIFT_NAME(audioDeviceModule(_:willReleaseEngine:));

- (NSInteger)audioDeviceModule:(RTC_OBJC_TYPE(RTCAudioDeviceModule) *)audioDeviceModule
                        engine:(AVAudioEngine *)engine
      configureInputFromSource:(nullable AVAudioNode *)source
                 toDestination:(AVAudioNode *)destination
                    withFormat:(AVAudioFormat *)format
                       context:(NSDictionary *)context
    NS_SWIFT_NAME(audioDeviceModule(_:engine:configureInputFromSource:toDestination:format:context:));

- (NSInteger)audioDeviceModule:(RTC_OBJC_TYPE(RTCAudioDeviceModule) *)audioDeviceModule
                        engine:(AVAudioEngine *)engine
     configureOutputFromSource:(AVAudioNode *)source
                 toDestination:(nullable AVAudioNode *)destination
                    withFormat:(AVAudioFormat *)format
                       context:(NSDictionary *)context
    NS_SWIFT_NAME(audioDeviceModule(_:engine:configureOutputFromSource:toDestination:format:context:));

- (void)audioDeviceModuleDidUpdateDevices:(RTC_OBJC_TYPE(RTCAudioDeviceModule) *)audioDeviceModule
    NS_SWIFT_NAME(audioDeviceModuleDidUpdateDevices(_:));

@end

RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCAudioDeviceModule) : NSObject

@property(nonatomic, readonly) NSArray<RTC_OBJC_TYPE(RTCIODevice) *> *outputDevices;
@property(nonatomic, readonly) NSArray<RTC_OBJC_TYPE(RTCIODevice) *> *inputDevices;

@property(nonatomic, readonly) BOOL playing;
@property(nonatomic, readonly) BOOL recording;

@property(nonatomic, assign) RTC_OBJC_TYPE(RTCIODevice) * outputDevice;
@property(nonatomic, assign) RTC_OBJC_TYPE(RTCIODevice) * inputDevice;

// Executes low-level API's in sequence to switch the device
// Use outputDevice / inputDevice property unless you need to know if setting the device is
// successful.
- (BOOL)trySetOutputDevice:(nullable RTC_OBJC_TYPE(RTCIODevice) *)device;
- (BOOL)trySetInputDevice:(nullable RTC_OBJC_TYPE(RTCIODevice) *)device;

- (NSInteger)startPlayout;
- (NSInteger)stopPlayout;
- (NSInteger)initPlayout;
- (NSInteger)startRecording;
- (NSInteger)stopRecording;
- (NSInteger)initRecording;

- (NSInteger)initAndStartRecording;

- (NSInteger)setEngineAvailability:(RTC_OBJC_TYPE(RTCAudioEngineAvailability))availability;

// For testing purposes
@property(nonatomic, readonly) BOOL isPlayoutInitialized;
@property(nonatomic, readonly) BOOL isRecordingInitialized;
@property(nonatomic, readonly) BOOL isPlaying;
@property(nonatomic, readonly) BOOL isRecording;
@property(nonatomic, readonly) BOOL isEngineRunning;
@property(nonatomic, readonly) BOOL isMicrophoneMuted;
- (NSInteger)setMicrophoneMuted:(BOOL)muted;

// Directly get & set engine state.
@property(nonatomic, assign) RTC_OBJC_TYPE(RTCAudioEngineState) engineState;

@property(nonatomic, readonly, getter=isRecordingAlwaysPreparedMode)
    BOOL recordingAlwaysPreparedMode;
- (NSInteger)setRecordingAlwaysPreparedMode:(BOOL)enabled;

@property(nonatomic, weak, nullable) id<RTC_OBJC_TYPE(RTCAudioDeviceModuleDelegate)> observer;

// Manual rendering.
@property(nonatomic, readonly, getter=isManualRenderingMode) BOOL manualRenderingMode;
- (NSInteger)setManualRenderingMode:(BOOL)enabled;

// Advanced other audio ducking.
@property(nonatomic, assign, getter=isAdvancedDuckingEnabled) BOOL advancedDuckingEnabled;

// Audio ducking level. Maps to AVAudioVoiceProcessingOtherAudioDuckingLevel (iOS 17.0+, macOS 14.0+).
@property(nonatomic, assign) RTC_OBJC_TYPE(RTCAudioDuckingLevel) duckingLevel;

@property(nonatomic, readonly) RTC_OBJC_TYPE(RTCAudioEngineMuteMode) muteMode;
- (NSInteger)setMuteMode:(RTC_OBJC_TYPE(RTCAudioEngineMuteMode))mode;

/// Indicates whether Voice-Processing I/O is enabled. Requires restarting the Audio Engine to
/// toggle. Defaults to true.
@property(nonatomic, readonly, getter=isVoiceProcessingEnabled) BOOL voiceProcessingEnabled;
- (NSInteger)setVoiceProcessingEnabled:(BOOL)enabled;

/// Temporarily bypasses Voice-Processing I/O. Can be toggled at runtime without restarting the
/// Audio Engine. Defaults to false.
@property(nonatomic, assign, getter=isVoiceProcessingBypassed) BOOL voiceProcessingBypassed;

/// Indicates whether Automatic Gain Control (AGC) is enabled. Requires Voice-Processing I/O to be
/// enabled. Enabled by default when VPIO is enabled.
@property(nonatomic, assign, getter=isVoiceProcessingAGCEnabled) BOOL voiceProcessingAGCEnabled;

@property(nonatomic, readonly) RTC_OBJC_TYPE(RTCAudioEngineAvailability) engineAvailability;

@end

NS_ASSUME_NONNULL_END
