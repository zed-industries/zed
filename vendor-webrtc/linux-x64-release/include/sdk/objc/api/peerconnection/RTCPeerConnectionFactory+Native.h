/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import "RTCAudioDeviceModule.h"
#import "RTCPeerConnectionFactory.h"

#include "api/audio/audio_device.h"
#include "api/audio/audio_processing.h"
#include "api/audio_codecs/audio_decoder_factory.h"
#include "api/audio_codecs/audio_encoder_factory.h"
#include "api/peer_connection_interface.h"
#include "api/scoped_refptr.h"
#include "api/transport/network_control.h"
#include "api/video_codecs/video_decoder_factory.h"
#include "api/video_codecs/video_encoder_factory.h"

NS_ASSUME_NONNULL_BEGIN

/**
 * This class extension exposes methods that work directly with injectable C++
 * components.
 */
@interface RTC_OBJC_TYPE (RTCPeerConnectionFactory)
()

    /* Initializer used when WebRTC is compiled with no media support */
    - (instancetype)initWithNoMedia;

/* Initialize object with provided dependencies and with media support. */
- (instancetype)initWithMediaAndDependencies:
    (webrtc::PeerConnectionFactoryDependencies)dependencies;

/* Initialize object with injectable native audio/video encoder/decoder factories */
- (instancetype)
    initWithNativeAudioEncoderFactory:
        (webrtc::scoped_refptr<webrtc::AudioEncoderFactory>)audioEncoderFactory
            nativeAudioDecoderFactory:
                (webrtc::scoped_refptr<webrtc::AudioDecoderFactory>)audioDecoderFactory
            nativeVideoEncoderFactory:
                (std::unique_ptr<webrtc::VideoEncoderFactory>)videoEncoderFactory
            nativeVideoDecoderFactory:
                (std::unique_ptr<webrtc::VideoDecoderFactory>)videoDecoderFactory
                    audioDeviceModule:
                        (webrtc::scoped_refptr<webrtc::AudioDeviceModule>)audioDeviceModule
                audioProcessingModule:
                    (webrtc::scoped_refptr<webrtc::AudioProcessing>)audioProcessingModule
             networkControllerFactory:(std::unique_ptr<webrtc::NetworkControllerFactoryInterface>)
                                          networkControllerFactory
                audioDeviceModuleType:(RTC_OBJC_TYPE(RTCAudioDeviceModuleType))audioDeviceModuleType
                bypassVoiceProcessing:(BOOL)bypassVoiceProcessing;

- (instancetype)
    initWithEncoderFactory:(nullable id<RTC_OBJC_TYPE(RTCVideoEncoderFactory)>)encoderFactory
            decoderFactory:(nullable id<RTC_OBJC_TYPE(RTCVideoDecoderFactory)>)decoderFactory;

/** Initialize an RTCPeerConnection with a configuration, constraints, and
 *  dependencies.
 */
- (nullable RTC_OBJC_TYPE(RTCPeerConnection) *)
    peerConnectionWithDependencies:(RTC_OBJC_TYPE(RTCConfiguration) *)configuration
                       constraints:(RTC_OBJC_TYPE(RTCMediaConstraints) *)constraints
                      dependencies:(std::unique_ptr<webrtc::PeerConnectionDependencies>)dependencies
                          delegate:(nullable id<RTC_OBJC_TYPE(RTCPeerConnectionDelegate)>)delegate;

@end

NS_ASSUME_NONNULL_END
