/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import "RTCPeerConnectionFactory.h"

#include "api/audio/audio_device.h"
#include "api/audio/audio_processing.h"
#include "api/audio_codecs/audio_decoder_factory.h"
#include "api/audio_codecs/audio_encoder_factory.h"
#include "api/field_trials_view.h"
#include "api/scoped_refptr.h"
#include "api/video_codecs/video_decoder_factory.h"
#include "api/video_codecs/video_encoder_factory.h"

NS_ASSUME_NONNULL_BEGIN

@interface RTC_OBJC_TYPE(RTCPeerConnectionFactoryBuilder) : NSObject

+ (RTC_OBJC_TYPE(RTCPeerConnectionFactoryBuilder) *)builder;

- (RTC_OBJC_TYPE(RTCPeerConnectionFactory) *)createPeerConnectionFactory;

- (void)setFieldTrials:(std::unique_ptr<webrtc::FieldTrialsView>)fieldTrials;

- (void)setVideoEncoderFactory:
    (std::unique_ptr<webrtc::VideoEncoderFactory>)videoEncoderFactory;

- (void)setVideoDecoderFactory:
    (std::unique_ptr<webrtc::VideoDecoderFactory>)videoDecoderFactory;

- (void)setAudioEncoderFactory:
    (webrtc::scoped_refptr<webrtc::AudioEncoderFactory>)audioEncoderFactory;

- (void)setAudioDecoderFactory:
    (webrtc::scoped_refptr<webrtc::AudioDecoderFactory>)audioDecoderFactory;

- (void)setAudioDeviceModule:
    (webrtc::scoped_refptr<webrtc::AudioDeviceModule>)audioDeviceModule;

- (void)setAudioProcessingModule:
    (webrtc::scoped_refptr<webrtc::AudioProcessing>)audioProcessingModule;

- (void)setAudioProcessingBuilder:
    (std::unique_ptr<webrtc::AudioProcessingBuilderInterface>)
        audioProcessingBuilder;

@end

NS_ASSUME_NONNULL_END
