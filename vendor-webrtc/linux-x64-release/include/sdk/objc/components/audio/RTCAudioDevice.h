/*
 *  Copyright 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import <AudioUnit/AudioUnit.h>
#import <Foundation/Foundation.h>

#import "sdk/objc/base/RTCMacros.h"

NS_ASSUME_NONNULL_BEGIN

typedef OSStatus (^RTC_OBJC_TYPE(RTCAudioDeviceGetPlayoutDataBlock))(
    AudioUnitRenderActionFlags *_Nonnull actionFlags,
    const AudioTimeStamp *_Nonnull timestamp,
    NSInteger inputBusNumber,
    UInt32 frameCount,
    AudioBufferList *_Nonnull outputData);

typedef OSStatus (^RTC_OBJC_TYPE(RTCAudioDeviceRenderRecordedDataBlock))(
    AudioUnitRenderActionFlags *_Nonnull actionFlags,
    const AudioTimeStamp *_Nonnull timestamp,
    NSInteger inputBusNumber,
    UInt32 frameCount,
    AudioBufferList *_Nonnull inputData,
    void *_Nullable renderContext);

typedef OSStatus (^RTC_OBJC_TYPE(RTCAudioDeviceDeliverRecordedDataBlock))(
    AudioUnitRenderActionFlags *_Nonnull actionFlags,
    const AudioTimeStamp *_Nonnull timestamp,
    NSInteger inputBusNumber,
    UInt32 frameCount,
    const AudioBufferList *_Nullable inputData,
    void *_Nullable renderContext,
    NS_NOESCAPE RTC_OBJC_TYPE(
        RTCAudioDeviceRenderRecordedDataBlock) _Nullable renderBlock);

/**
 * Delegate object provided by native ADM during RTCAudioDevice initialization.
 * Provides blocks to poll playback audio samples from native ADM and to feed
 * recorded audio samples into native ADM.
 */
RTC_OBJC_EXPORT @protocol RTC_OBJC_TYPE
(RTCAudioDeviceDelegate)<NSObject>
    /**
     * Implementation of RTCAudioSource should call this block to feed recorded
     * PCM (16-bit integer) into native ADM. Stereo data is expected to be
     * interleaved starting with the left channel. Either `inputData` with
     * pre-filled audio data must be provided during block call or `renderBlock`
     * must be provided which must fill provided audio buffer with recorded
     * samples.
     *
     * NOTE: Implementation of RTCAudioDevice is expected to call the block on
     * the same thread until `notifyAudioInterrupted` is called. When
     * `notifyAudioInterrupted` is called implementation can call the block on a
     * different thread.
     */
    @property(readonly, nonnull)
        RTC_OBJC_TYPE(RTCAudioDeviceDeliverRecordedDataBlock)
            deliverRecordedData;

/**
 * Provides input sample rate preference as it preferred by native ADM.
 */
@property(readonly) double preferredInputSampleRate;

/**
 * Provides input IO buffer duration preference as it preferred by native ADM.
 */
@property(readonly) NSTimeInterval preferredInputIOBufferDuration;

/**
 * Provides output sample rate preference as it preferred by native ADM.
 */
@property(readonly) double preferredOutputSampleRate;

/**
 * Provides output IO buffer duration preference as it preferred by native ADM.
 */
@property(readonly) NSTimeInterval preferredOutputIOBufferDuration;

/**
 * Implementation of RTCAudioDevice should call this block to request PCM
 * (16-bit integer) from native ADM to play. Stereo data is interleaved starting
 * with the left channel.
 *
 * NOTE: Implementation of RTCAudioDevice is expected to invoke of this block on
 * the same thread until `notifyAudioInterrupted` is called. When
 * `notifyAudioInterrupted` is called implementation can call the block from a
 * different thread.
 */
@property(readonly, nonnull) RTC_OBJC_TYPE(RTCAudioDeviceGetPlayoutDataBlock)
    getPlayoutData;

/**
 * Notifies native ADM that some of the audio input parameters of RTCAudioDevice
 * like samle rate and/or IO buffer duration and/or IO latency had possibly
 * changed. Native ADM will adjust its audio input buffer to match current
 * parameters of audio device.
 *
 * NOTE: Must be called within block executed via `dispatchAsync` or
 * `dispatchSync`.
 */
- (void)notifyAudioInputParametersChange;

/**
 * Notifies native ADM that some of the audio output parameters of
 * RTCAudioDevice like samle rate and/or IO buffer duration and/or IO latency
 * had possibly changed. Native ADM will adjust its audio output buffer to match
 * current parameters of audio device.
 *
 * NOTE: Must be called within block executed via `dispatchAsync` or
 * `dispatchSync`.
 */
- (void)notifyAudioOutputParametersChange;

/**
 * Notifies native ADM that audio input is interrupted and further audio playout
 * and recording might happen on a different thread.
 *
 * NOTE: Must be called within block executed via `dispatchAsync` or
 * `dispatchSync`.
 */
- (void)notifyAudioInputInterrupted;

/**
 * Notifies native ADM that audio output is interrupted and further audio
 * playout and recording might happen on a different thread.
 *
 * NOTE: Must be called within block executed via `dispatchAsync` or
 * `dispatchSync`.
 */
- (void)notifyAudioOutputInterrupted;

/**
 * Asynchronously execute block of code within the context of
 * thread which owns native ADM.
 *
 * NOTE: Intended to be used to invoke `notifyAudioInputParametersChange`,
 * `notifyAudioOutputParametersChange`, `notifyAudioInputInterrupted`,
 * `notifyAudioOutputInterrupted` on native ADM thread.
 * Also could be used by `RTCAudioDevice` implementation to tie
 * mutations of underlying audio objects (AVAudioEngine, AudioUnit, etc)
 * to the native ADM thread. Could be useful to handle events like audio route
 * change, which could lead to audio parameters change.
 */
- (void)dispatchAsync:(dispatch_block_t)block;

/**
 * Synchronously execute block of code within the context of
 * thread which owns native ADM. Allows reentrancy.
 *
 * NOTE: Intended to be used to invoke `notifyAudioInputParametersChange`,
 * `notifyAudioOutputParametersChange`, `notifyAudioInputInterrupted`,
 * `notifyAudioOutputInterrupted` on native ADM thread and make sure
 * aforementioned is completed before `dispatchSync` returns. Could be useful
 * when implementation of `RTCAudioDevice` tie mutation to underlying audio
 * objects (AVAudioEngine, AudioUnit, etc) to own thread to satisfy requirement
 * that native ADM audio parameters must be kept in sync with current audio
 * parameters before audio is actually played or recorded.
 */
- (void)dispatchSync:(dispatch_block_t)block;

@end

/**
 * Protocol to abstract platform specific ways to implement playback and
 * recording.
 *
 * NOTE: All the members of protocol are called by native ADM from the same
 * thread between calls to `initializeWithDelegate` and `terminate`. NOTE:
 * Implementation is fully responsible for configuring application's
 * AVAudioSession. An example implementation of RTCAudioDevice:
 * https://github.com/mstyura/RTCAudioDevice
 * TODO(yura.yaroshevich): Implement custom RTCAudioDevice for AppRTCMobile demo
 * app.
 */
RTC_OBJC_EXPORT @protocol RTC_OBJC_TYPE
(RTCAudioDevice)<NSObject>

    /**
     * Indicates current sample rate of audio recording. Changes to this
     * property must be notified back to native ADM via
     * `-[RTCAudioDeviceDelegate notifyAudioParametersChange]`.
     */
    @property(readonly) double deviceInputSampleRate;

/**
 * Indicates current size of record buffer. Changes to this property
 * must be notified back to native ADM via `-[RTCAudioDeviceDelegate
 * notifyAudioParametersChange]`.
 */
@property(readonly) NSTimeInterval inputIOBufferDuration;

/**
 * Indicates current number of recorded audio channels. Changes to this property
 * must be notified back to native ADM via `-[RTCAudioDeviceDelegate
 * notifyAudioParametersChange]`.
 */
@property(readonly) NSInteger inputNumberOfChannels;

/**
 * Indicates current input latency
 */
@property(readonly) NSTimeInterval inputLatency;

/**
 * Indicates current sample rate of audio playback. Changes to this property
 * must be notified back to native ADM via `-[RTCAudioDeviceDelegate
 * notifyAudioParametersChange]`.
 */
@property(readonly) double deviceOutputSampleRate;

/**
 * Indicates current size of playback buffer. Changes to this property
 * must be notified back to native ADM via `-[RTCAudioDeviceDelegate
 * notifyAudioParametersChange]`.
 */
@property(readonly) NSTimeInterval outputIOBufferDuration;

/**
 * Indicates current number of playback audio channels. Changes to this property
 * must be notified back to WebRTC via `[RTCAudioDeviceDelegate
 * notifyAudioParametersChange]`.
 */
@property(readonly) NSInteger outputNumberOfChannels;

/**
 * Indicates current output latency
 */
@property(readonly) NSTimeInterval outputLatency;

/**
 * Indicates if invocation of `initializeWithDelegate` required before usage of
 * RTCAudioDevice. YES indicates that `initializeWithDelegate` was called
 * earlier without subsequent call to `terminate`. NO indicates that either
 * `initializeWithDelegate` not called or `terminate` called.
 */
@property(readonly) BOOL isInitialized;

/**
 * Initializes RTCAudioDevice with RTCAudioDeviceDelegate.
 * Implementation must return YES if RTCAudioDevice initialized successfully and
 * NO otherwise.
 */
- (BOOL)initializeWithDelegate:
    (id<RTC_OBJC_TYPE(RTCAudioDeviceDelegate)>)delegate;

/**
 * De-initializes RTCAudioDevice. Implementation should forget about `delegate`
 * provided in `initializeWithDelegate`.
 */
- (BOOL)terminateDevice;

/**
 * Property to indicate if `initializePlayout` call required before invocation
 * of `startPlayout`. YES indicates that `initializePlayout` was successfully
 * invoked earlier or not necessary, NO indicates that `initializePlayout`
 * invocation required.
 */
@property(readonly) BOOL isPlayoutInitialized;

/**
 * Prepares RTCAudioDevice to play audio.
 * Called by native ADM before invocation of `startPlayout`.
 * Implementation is expected to return YES in case of successful playout
 * initialization and NO otherwise.
 */
- (BOOL)initializePlayout;

/**
 * Property to indicate if RTCAudioDevice should be playing according to
 * earlier calls of `startPlayout` and `stopPlayout`.
 */
@property(readonly) BOOL isPlaying;

/**
 * Method is called when native ADM wants to play audio.
 * Implementation is expected to return YES if playback start request
 * successfully handled and NO otherwise.
 */
- (BOOL)startPlayout;

/**
 * Method is called when native ADM no longer needs to play audio.
 * Implementation is expected to return YES if playback stop request
 * successfully handled and NO otherwise.
 */
- (BOOL)stopPlayout;

/**
 * Property to indicate if `initializeRecording` call required before usage of
 * `startRecording`. YES indicates that `initializeRecording` was successfully
 * invoked earlier or not necessary, NO indicates that `initializeRecording`
 * invocation required.
 */
@property(readonly) BOOL isRecordingInitialized;

/**
 * Prepares RTCAudioDevice to record audio.
 * Called by native ADM before invocation of `startRecording`.
 * Implementation may use this method to prepare resources required to record
 * audio. Implementation is expected to return YES in case of successful record
 * initialization and NO otherwise.
 */
- (BOOL)initializeRecording;

/**
 * Property to indicate if RTCAudioDevice should record audio according to
 * earlier calls to `startRecording` and `stopRecording`.
 */
@property(readonly) BOOL isRecording;

/**
 * Method is called when native ADM wants to record audio.
 * Implementation is expected to return YES if recording start request
 * successfully handled and NO otherwise.
 */
- (BOOL)startRecording;

/**
 * Method is called when native ADM no longer needs to record audio.
 * Implementation is expected to return YES if recording stop request
 * successfully handled and NO otherwise.
 */
- (BOOL)stopRecording;

@end

NS_ASSUME_NONNULL_END
