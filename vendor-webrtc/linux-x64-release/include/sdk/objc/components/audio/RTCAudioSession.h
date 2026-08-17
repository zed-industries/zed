/*
 *  Copyright 2016 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import <AVFoundation/AVFoundation.h>
#import <Foundation/Foundation.h>

#import "sdk/objc/base/RTCMacros.h"

NS_ASSUME_NONNULL_BEGIN

extern NSString *const RTC_CONSTANT_TYPE(RTCAudioSessionErrorDomain);
/** Method that requires lock was called without lock. */
extern NSInteger const RTC_CONSTANT_TYPE(RTCAudioSessionErrorLockRequired);
/** Unknown configuration error occurred. */
extern NSInteger const RTC_CONSTANT_TYPE(RTCAudioSessionErrorConfiguration);

@class RTC_OBJC_TYPE(RTCAudioSession);
@class RTC_OBJC_TYPE(RTCAudioSessionConfiguration);

// Surfaces AVAudioSession events. WebRTC will listen directly for notifications
// from AVAudioSession and handle them before calling these delegate methods,
// at which point applications can perform additional processing if required.
RTC_OBJC_EXPORT
@protocol RTC_OBJC_TYPE
(RTCAudioSessionDelegate)<NSObject>

    @optional
/** Called on a system notification thread when AVAudioSession starts an
 *  interruption event.
 */
- (void)audioSessionDidBeginInterruption:
    (RTC_OBJC_TYPE(RTCAudioSession) *)session;

/** Called on a system notification thread when AVAudioSession ends an
 *  interruption event.
 */
- (void)audioSessionDidEndInterruption:(RTC_OBJC_TYPE(RTCAudioSession) *)session
                   shouldResumeSession:(BOOL)shouldResumeSession;

/** Called on a system notification thread when AVAudioSession changes the
 *  route.
 */
- (void)audioSessionDidChangeRoute:(RTC_OBJC_TYPE(RTCAudioSession) *)session
                            reason:(AVAudioSessionRouteChangeReason)reason
                     previousRoute:
                         (AVAudioSessionRouteDescription *)previousRoute;

/** Called on a system notification thread when AVAudioSession media server
 *  terminates.
 */
- (void)audioSessionMediaServerTerminated:
    (RTC_OBJC_TYPE(RTCAudioSession) *)session;

/** Called on a system notification thread when AVAudioSession media server
 *  restarts.
 */
- (void)audioSessionMediaServerReset:(RTC_OBJC_TYPE(RTCAudioSession) *)session;

// TODO(tkchin): Maybe handle SilenceSecondaryAudioHintNotification.

- (void)audioSession:(RTC_OBJC_TYPE(RTCAudioSession) *)session
    didChangeCanPlayOrRecord:(BOOL)canPlayOrRecord;

/** Called on a WebRTC thread when the audio device is notified to begin
 *  playback or recording.
 */
- (void)audioSessionDidStartPlayOrRecord:
    (RTC_OBJC_TYPE(RTCAudioSession) *)session;

/** Called on a WebRTC thread when the audio device is notified to stop
 *  playback or recording.
 */
- (void)audioSessionDidStopPlayOrRecord:
    (RTC_OBJC_TYPE(RTCAudioSession) *)session;

/** Called when the AVAudioSession output volume value changes. */
- (void)audioSession:(RTC_OBJC_TYPE(RTCAudioSession) *)audioSession
    didChangeOutputVolume:(float)outputVolume;

/** Called when the audio device detects a playout glitch. The argument is the
 *  number of glitches detected so far in the current audio playout session.
 */
- (void)audioSession:(RTC_OBJC_TYPE(RTCAudioSession) *)audioSession
    didDetectPlayoutGlitch:(int64_t)totalNumberOfGlitches;

/** Called when the audio session is about to change the active state.
 */
- (void)audioSession:(RTC_OBJC_TYPE(RTCAudioSession) *)audioSession
       willSetActive:(BOOL)active;

/** Called after the audio session sucessfully changed the active state.
 */
- (void)audioSession:(RTC_OBJC_TYPE(RTCAudioSession) *)audioSession
        didSetActive:(BOOL)active;

/** Called after the audio session failed to change the active state.
 */
- (void)audioSession:(RTC_OBJC_TYPE(RTCAudioSession) *)audioSession
    failedToSetActive:(BOOL)active
                error:(NSError *)error;

- (void)audioSession:(RTC_OBJC_TYPE(RTCAudioSession) *)audioSession
    audioUnitStartFailedWithError:(NSError *)error;

@end

/** This is a protocol used to inform RTCAudioSession when the audio session
 *  activation state has changed outside of RTCAudioSession. The current known
 * use case of this is when CallKit activates the audio session for the
 * application
 */
RTC_OBJC_EXPORT
@protocol RTC_OBJC_TYPE
(RTCAudioSessionActivationDelegate)<NSObject>

    /** Called when the audio session is activated outside of the app by iOS. */
    - (void)audioSessionDidActivate : (AVAudioSession *)session;

/** Called when the audio session is deactivated outside of the app by iOS. */
- (void)audioSessionDidDeactivate:(AVAudioSession *)session;

@end

/** Proxy class for AVAudioSession that adds a locking mechanism similar to
 *  AVCaptureDevice. This is used to that interleaving configurations between
 *  WebRTC and the application layer are avoided.
 *
 *  RTCAudioSession also coordinates activation so that the audio session is
 *  activated only once. See `setActive:error:`.
 */
RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCAudioSession) : NSObject <RTC_OBJC_TYPE(RTCAudioSessionActivationDelegate)>

/** Convenience property to access the AVAudioSession singleton. Callers should
 *  not call setters on AVAudioSession directly, but other method invocations
 *  are fine.
 */
@property(nonatomic, readonly) AVAudioSession *session;

/** Our best guess at whether the session is active based on results of calls to
 *  AVAudioSession.
 */
@property(nonatomic, readonly) BOOL isActive;

/** If YES, WebRTC will not initialize the audio unit automatically when an
 *  audio track is ready for playout or recording. Instead, applications should
 *  call setIsAudioEnabled. If NO, WebRTC will initialize the audio unit
 *  as soon as an audio track is ready for playout or recording.
 */
@property(nonatomic, assign) BOOL useManualAudio;

/** This property is only effective if useManualAudio is YES.
 *  Represents permission for WebRTC to initialize the VoIP audio unit.
 *  When set to NO, if the VoIP audio unit used by WebRTC is active, it will be
 *  stopped and uninitialized. This will stop incoming and outgoing audio.
 *  When set to YES, WebRTC will initialize and start the audio unit when it is
 *  needed (e.g. due to establishing an audio connection).
 *  This property was introduced to work around an issue where if an AVPlayer is
 *  playing audio while the VoIP audio unit is initialized, its audio would be
 *  either cut off completely or played at a reduced volume. By preventing
 *  the audio unit from being initialized until after the audio has completed,
 *  we are able to prevent the abrupt cutoff.
 */
@property(nonatomic, assign) BOOL isAudioEnabled;

// Proxy properties.
@property(readonly) NSString *category;
@property(readonly) AVAudioSessionCategoryOptions categoryOptions;
@property(readonly) NSString *mode;
@property(readonly) BOOL secondaryAudioShouldBeSilencedHint;
@property(readonly) AVAudioSessionRouteDescription *currentRoute;
@property(readonly) NSInteger maximumInputNumberOfChannels;
@property(readonly) NSInteger maximumOutputNumberOfChannels;
@property(readonly) float inputGain;
@property(readonly) BOOL inputGainSettable;
@property(readonly) BOOL inputAvailable;
@property(readonly, nullable)
    NSArray<AVAudioSessionDataSourceDescription *> *inputDataSources;
@property(readonly, nullable)
    AVAudioSessionDataSourceDescription *inputDataSource;
@property(readonly, nullable)
    NSArray<AVAudioSessionDataSourceDescription *> *outputDataSources;
@property(readonly, nullable)
    AVAudioSessionDataSourceDescription *outputDataSource;
@property(readonly) double sampleRate;
@property(readonly) double preferredSampleRate;
@property(readonly) NSInteger inputNumberOfChannels;
@property(readonly) NSInteger outputNumberOfChannels;
@property(readonly) float outputVolume;
@property(readonly) NSTimeInterval inputLatency;
@property(readonly) NSTimeInterval outputLatency;
@property(readonly) NSTimeInterval IOBufferDuration;
@property(readonly) NSTimeInterval preferredIOBufferDuration;

/**
 When YES, calls to -setConfiguration:error: and -setConfiguration:active:error:
 ignore errors in configuring the audio session's "preferred" attributes (e.g.
 preferredInputNumberOfChannels). Typically, configurations to preferred
 attributes are optimizations, and ignoring this type of configuration error
 allows code flow to continue along the happy path when these optimization are
 not available. The default value of this property is NO.
 */
@property(nonatomic) BOOL ignoresPreferredAttributeConfigurationErrors;

/** Number of times setActive:YES has succeeded without a balanced call to
 *  setActive:NO.
 */
@property(nonatomic, readonly) int activationCount;

/** The number of times `beginWebRTCSession` was called without a balanced call
 *  to `endWebRTCSession`.
 */
@property(nonatomic, readonly) int webRTCSessionCount;

/** Default constructor. */
+ (instancetype)sharedInstance;
- (instancetype)init NS_UNAVAILABLE;

/** Adds a delegate, which is held weakly. */
- (void)addDelegate:(id<RTC_OBJC_TYPE(RTCAudioSessionDelegate)>)delegate;
/** Removes an added delegate. */
- (void)removeDelegate:(id<RTC_OBJC_TYPE(RTCAudioSessionDelegate)>)delegate;

/** Request exclusive access to the audio session for configuration. This call
 *  will block if the lock is held by another object.
 */
- (void)lockForConfiguration;
/** Relinquishes exclusive access to the audio session. */
- (void)unlockForConfiguration;

/** If `active`, activates the audio session if it isn't already active.
 *  Successful calls must be balanced with a setActive:NO when activation is no
 *  longer required. If not `active`, deactivates the audio session if one is
 *  active and this is the last balanced call. When deactivating, the
 *  AVAudioSessionSetActiveOptionNotifyOthersOnDeactivation option is passed to
 *  AVAudioSession.
 */
- (BOOL)setActive:(BOOL)active error:(NSError **)outError;

// The following methods are proxies for the associated methods on
// AVAudioSession. `lockForConfiguration` must be called before using them
// otherwise they will fail with kRTCAudioSessionErrorLockRequired.

- (BOOL)setCategory:(AVAudioSessionCategory)category
               mode:(AVAudioSessionMode)mode
            options:(AVAudioSessionCategoryOptions)options
              error:(NSError **)outError;
- (BOOL)setCategory:(AVAudioSessionCategory)category
        withOptions:(AVAudioSessionCategoryOptions)options
              error:(NSError **)outError;
- (BOOL)setMode:(AVAudioSessionMode)mode error:(NSError **)outError;
- (BOOL)setInputGain:(float)gain error:(NSError **)outError;
- (BOOL)setPreferredSampleRate:(double)sampleRate error:(NSError **)outError;
- (BOOL)setPreferredIOBufferDuration:(NSTimeInterval)duration
                               error:(NSError **)outError;
- (BOOL)setPreferredInputNumberOfChannels:(NSInteger)count
                                    error:(NSError **)outError;
- (BOOL)setPreferredOutputNumberOfChannels:(NSInteger)count
                                     error:(NSError **)outError;
- (BOOL)overrideOutputAudioPort:(AVAudioSessionPortOverride)portOverride
                          error:(NSError **)outError;
- (BOOL)setPreferredInput:(AVAudioSessionPortDescription *)inPort
                    error:(NSError **)outError;
- (BOOL)setInputDataSource:(AVAudioSessionDataSourceDescription *)dataSource
                     error:(NSError **)outError;
- (BOOL)setOutputDataSource:(AVAudioSessionDataSourceDescription *)dataSource
                      error:(NSError **)outError;
@end

@interface RTC_OBJC_TYPE (RTCAudioSession)
(Configuration)

    /** Applies the configuration to the current session. Attempts to set all
     *  properties even if previous ones fail. Only the last error will be
     *  returned.
     *  `lockForConfiguration` must be called first.
     */
    - (BOOL)setConfiguration
    : (RTC_OBJC_TYPE(RTCAudioSessionConfiguration) *)configuration error
    : (NSError **)outError;

/** Convenience method that calls both setConfiguration and setActive.
 *  `lockForConfiguration` must be called first.
 */
- (BOOL)setConfiguration:
            (RTC_OBJC_TYPE(RTCAudioSessionConfiguration) *)configuration
                  active:(BOOL)active
                   error:(NSError **)outError;

@end

NS_ASSUME_NONNULL_END
