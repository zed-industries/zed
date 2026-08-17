/*
 *  Copyright 2016 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import "RTCAudioSession.h"

NS_ASSUME_NONNULL_BEGIN

@class RTC_OBJC_TYPE(RTCAudioSessionConfiguration);

@interface RTC_OBJC_TYPE (RTCAudioSession) ()

/** Convenience BOOL that checks useManualAudio and isAudioEnebled. */
@property(readonly) BOOL canPlayOrRecord;

/** Tracks whether we have been sent an interruption event that hasn't been
 * matched by either an interrupted end event or a foreground event.
 */
@property(nonatomic, assign) BOOL isInterrupted;

/** Adds the delegate to the list of delegates, and places it at the front of
 *  the list. This delegate will be notified before other delegates of
 *  audio events.
 */
- (void)pushDelegate:(id<RTC_OBJC_TYPE(RTCAudioSessionDelegate)>)delegate;

/** Signals RTCAudioSession that a WebRTC session is about to begin and
 *  audio configuration is needed. Will configure the audio session for WebRTC
 *  if not already configured and if configuration is not delayed.
 *  Successful calls must be balanced by a call to endWebRTCSession.
 */
- (BOOL)beginWebRTCSession:(NSError **)outError;

/** Signals RTCAudioSession that a WebRTC session is about to end and audio
 *  unconfiguration is needed. Will unconfigure the audio session for WebRTC
 *  if this is the last unmatched call and if configuration is not delayed.
 */
- (BOOL)endWebRTCSession:(NSError **)outError;

/** Configure the audio session for WebRTC. This call will fail if the session
 *  is already configured. On other failures, we will attempt to restore the
 *  previously used audio session configuration.
 *  `lockForConfiguration` must be called first.
 *  Successful calls to configureWebRTCSession must be matched by calls to
 *  `unconfigureWebRTCSession`.
 */
- (BOOL)configureWebRTCSession:(NSError **)outError;

/** Unconfigures the session for WebRTC. This will attempt to restore the
 *  audio session to the settings used before `configureWebRTCSession` was
 *  called.
 *  `lockForConfiguration` must be called first.
 */
- (BOOL)unconfigureWebRTCSession:(NSError **)outError;

/** Returns a configuration error with the given description. */
- (NSError *)configurationErrorWithDescription:(NSString *)description;

/** Notifies the receiver that a playout glitch was detected. */
- (void)notifyDidDetectPlayoutGlitch:(int64_t)totalNumberOfGlitches;

/** Notifies the receiver that there was an error when starting an audio unit.
 */
- (void)notifyAudioUnitStartFailedWithError:(OSStatus)error;

// Properties and methods for tests.
- (void)notifyDidBeginInterruption;
- (void)notifyDidEndInterruptionWithShouldResumeSession:
    (BOOL)shouldResumeSession;
- (void)notifyDidChangeRouteWithReason:(AVAudioSessionRouteChangeReason)reason
                         previousRoute:
                             (AVAudioSessionRouteDescription *)previousRoute;
- (void)notifyMediaServicesWereLost;
- (void)notifyMediaServicesWereReset;
- (void)notifyDidChangeCanPlayOrRecord:(BOOL)canPlayOrRecord;
- (void)notifyDidStartPlayOrRecord;
- (void)notifyDidStopPlayOrRecord;

@end

NS_ASSUME_NONNULL_END
