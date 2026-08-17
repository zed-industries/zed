// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_AUDIO_CODEC_FACTORY_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_AUDIO_CODEC_FACTORY_H_

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/webrtc/api/audio_codecs/audio_decoder_factory.h"
#include "third_party/webrtc/api/audio_codecs/audio_encoder_factory.h"
#include "third_party/webrtc/api/scoped_refptr.h"

namespace blink {

PLATFORM_EXPORT webrtc::scoped_refptr<webrtc::AudioEncoderFactory>
CreateWebrtcAudioEncoderFactory();

PLATFORM_EXPORT webrtc::scoped_refptr<webrtc::AudioDecoderFactory>
CreateWebrtcAudioDecoderFactory();

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_AUDIO_CODEC_FACTORY_H_
