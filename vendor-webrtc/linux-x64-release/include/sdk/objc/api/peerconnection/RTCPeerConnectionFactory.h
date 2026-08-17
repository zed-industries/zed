/*
 *  Copyright 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import <Foundation/Foundation.h>

#import "RTCMacros.h"
#import "RTCAudioDeviceModule.h"

NS_ASSUME_NONNULL_BEGIN

@class RTC_OBJC_TYPE(RTCRtpCapabilities);
@class RTC_OBJC_TYPE(RTCAudioSource);
@class RTC_OBJC_TYPE(RTCAudioTrack);
@class RTC_OBJC_TYPE(RTCConfiguration);
@class RTC_OBJC_TYPE(RTCMediaConstraints);
@class RTC_OBJC_TYPE(RTCMediaStream);
@class RTC_OBJC_TYPE(RTCPeerConnection);
@class RTC_OBJC_TYPE(RTCVideoSource);
@class RTC_OBJC_TYPE(RTCVideoTrack);
@class RTC_OBJC_TYPE(RTCPeerConnectionFactoryOptions);
@class RTC_OBJC_TYPE(RTCAudioDeviceModule);
@class RTC_OBJC_TYPE(RTCRtpCapabilities);

typedef NS_ENUM(NSInteger, RTC_OBJC_TYPE(RTCRtpMediaType));

@protocol RTC_OBJC_TYPE
(RTCPeerConnectionDelegate);
@protocol RTC_OBJC_TYPE
(RTCVideoDecoderFactory);
@protocol RTC_OBJC_TYPE
(RTCVideoEncoderFactory);
@protocol RTC_OBJC_TYPE
(RTCSSLCertificateVerifier);
@protocol RTC_OBJC_TYPE
(RTCAudioDevice);
@protocol RTC_OBJC_TYPE
(RTCAudioProcessingModule);

RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCPeerConnectionFactory) : NSObject

/* Initialize object with default H264 video encoder/decoder factories and default ADM */
- (instancetype)init;

/* Initialize object with injectable video encoder/decoder factories and default ADM */
- (instancetype)
    initWithEncoderFactory:(nullable id<RTC_OBJC_TYPE(RTCVideoEncoderFactory)>)encoderFactory
            decoderFactory:(nullable id<RTC_OBJC_TYPE(RTCVideoDecoderFactory)>)decoderFactory;

/* Initialize object with injectable video encoder/decoder factories and injectable ADM */
- (instancetype)
    initWithEncoderFactory:(nullable id<RTC_OBJC_TYPE(RTCVideoEncoderFactory)>)encoderFactory
            decoderFactory:(nullable id<RTC_OBJC_TYPE(RTCVideoDecoderFactory)>)decoderFactory
               audioDevice:(nullable id<RTC_OBJC_TYPE(RTCAudioDevice)>)audioDevice;

/* Initialize object with bypass voice processing */
- (instancetype)
    initWithAudioDeviceModuleType:(RTC_OBJC_TYPE(RTCAudioDeviceModuleType))audioDeviceModuleType
            bypassVoiceProcessing:(BOOL)bypassVoiceProcessing
                   encoderFactory:(nullable id<RTC_OBJC_TYPE(RTCVideoEncoderFactory)>)encoderFactory
                   decoderFactory:(nullable id<RTC_OBJC_TYPE(RTCVideoDecoderFactory)>)decoderFactory
            audioProcessingModule:
                (nullable id<RTC_OBJC_TYPE(RTCAudioProcessingModule)>)audioProcessingModule;

@property(nonatomic, readonly) RTC_OBJC_TYPE(RTCAudioDeviceModule) *audioDeviceModule;

/**
 * Valid kind values are kRTCMediaStreamTrackKindAudio and
 * kRTCMediaStreamTrackKindVideo.
 */
- (RTC_OBJC_TYPE(RTCRtpCapabilities) *)rtpSenderCapabilitiesForKind:(NSString *)kind;

/**
 * Valid kind values are kRTCMediaStreamTrackKindAudio and
 * kRTCMediaStreamTrackKindVideo.
 */
- (RTC_OBJC_TYPE(RTCRtpCapabilities) *)rtpReceiverCapabilitiesForKind:(NSString *)kind;

/** Initialize an RTCAudioSource with constraints. */
- (RTC_OBJC_TYPE(RTCAudioSource) *)audioSourceWithConstraints:
    (nullable RTC_OBJC_TYPE(RTCMediaConstraints) *)constraints;

/** Initialize an RTCAudioTrack with an id. Convenience ctor to use an audio source
 * with no constraints.
 */
- (RTC_OBJC_TYPE(RTCAudioTrack) *)audioTrackWithTrackId:(NSString *)trackId;

/** Initialize an RTCAudioTrack with a source and an id. */
- (RTC_OBJC_TYPE(RTCAudioTrack) *)audioTrackWithSource:(RTC_OBJC_TYPE(RTCAudioSource) *)source
                                               trackId:(NSString *)trackId;

/** Initialize a generic RTCVideoSource. The RTCVideoSource should be
 * passed to a RTCVideoCapturer implementation, e.g.
 * RTCCameraVideoCapturer, in order to produce frames.
 */
- (RTC_OBJC_TYPE(RTCVideoSource) *)videoSource;

/** Initialize a generic RTCVideoSource with he posibility of marking
 * it as usable for screen sharing. The RTCVideoSource should be
 * passed to a RTCVideoCapturer implementation, e.g.
 * RTCCameraVideoCapturer, in order to produce frames.
 */
- (RTC_OBJC_TYPE(RTCVideoSource) *)videoSourceForScreenCast:(BOOL)forScreenCast;

/** Initialize an RTCVideoTrack with a source and an id. */
- (RTC_OBJC_TYPE(RTCVideoTrack) *)videoTrackWithSource:(RTC_OBJC_TYPE(RTCVideoSource) *)source
                                               trackId:(NSString *)trackId;

/** Initialize an RTCMediaStream with an id. */
- (RTC_OBJC_TYPE(RTCMediaStream) *)mediaStreamWithStreamId:(NSString *)streamId;

/** Initialize an RTCPeerConnection with a configuration, constraints, and
 *  delegate.
 */
- (nullable RTC_OBJC_TYPE(RTCPeerConnection) *)
    peerConnectionWithConfiguration:(RTC_OBJC_TYPE(RTCConfiguration) *)configuration
                        constraints:(RTC_OBJC_TYPE(RTCMediaConstraints) *)constraints
                           delegate:(nullable id<RTC_OBJC_TYPE(RTCPeerConnectionDelegate)>)delegate;

- (nullable RTC_OBJC_TYPE(RTCPeerConnection) *)
    peerConnectionWithConfiguration:(RTC_OBJC_TYPE(RTCConfiguration) *)configuration
                        constraints:(RTC_OBJC_TYPE(RTCMediaConstraints) *)constraints
                certificateVerifier:
                    (id<RTC_OBJC_TYPE(RTCSSLCertificateVerifier)>)certificateVerifier
                           delegate:(nullable id<RTC_OBJC_TYPE(RTCPeerConnectionDelegate)>)delegate;

/** Set the options to be used for subsequently created RTCPeerConnections */
- (void)setOptions:(nonnull RTC_OBJC_TYPE(RTCPeerConnectionFactoryOptions) *)options;

/** Start an AecDump recording. This API call will likely change in the future. */
- (BOOL)startAecDumpWithFilePath:(NSString *)filePath maxSizeInBytes:(int64_t)maxSizeInBytes;

/* Stop an active AecDump recording */
- (void)stopAecDump;

@end

NS_ASSUME_NONNULL_END
