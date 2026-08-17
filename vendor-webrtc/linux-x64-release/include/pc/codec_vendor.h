/*
 *  Copyright 2025 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_CODEC_VENDOR_H_
#define PC_CODEC_VENDOR_H_

#include <utility>
#include <vector>

#include "api/field_trials_view.h"
#include "api/rtc_error.h"
#include "api/rtp_transceiver_direction.h"
#include "call/payload_type.h"
#include "media/base/codec.h"
#include "media/base/codec_list.h"
#include "media/base/media_engine.h"
#include "pc/media_options.h"
#include "pc/session_description.h"
#include "pc/typed_codec_vendor.h"

namespace webrtc {

// This class contains the functions required to compute the list of codecs
// for SDP offer/answer. It is exposed to MediaSessionDescriptionFactory
// for the construction of offers and answers.

// TODO: bugs.webrtc.org/360058654 - complete the architectural changes
// The list of things to be done:
// - Make as much as possible private.
// - Split object usage into four objects: sender/receiver/audio/video.
// - Remove audio/video from the call names, merge code where possible.
// - Make the class instances owned by transceivers, so that codec
//   lists can differ per transceiver.
// For cleanliness:
// - Thread guard
class CodecVendor {
 public:
  CodecVendor(MediaEngineInterface* media_engine,
              bool rtx_enabled,
              const FieldTrialsView& trials);

 public:
  RTCErrorOr<std::vector<Codec>> GetNegotiatedCodecsForOffer(
      const MediaDescriptionOptions& media_description_options,
      const MediaSessionOptions& session_options,
      const ContentInfo* current_content,
      PayloadTypeSuggester& pt_suggester);

  RTCErrorOr<Codecs> GetNegotiatedCodecsForAnswer(
      const MediaDescriptionOptions& media_description_options,
      const MediaSessionOptions& session_options,
      RtpTransceiverDirection offer_rtd,
      RtpTransceiverDirection answer_rtd,
      const ContentInfo* current_content,
      std::vector<Codec> codecs_from_offer,
      PayloadTypeSuggester& pt_suggester);

  // Function exposed for issues.webrtc.org/412904801
  // Modify the video codecs to return on subsequent GetNegotiated* calls.
  // The input is a vector of pairs of codecs.
  // For each pair, the first element is the codec to be replaced,
  // and the second element is the codec to replace it with.
  void ModifyVideoCodecs(std::vector<std::pair<Codec, Codec>> changes);
  // Functions exposed for testing
  void set_audio_codecs(const CodecList& send_codecs,
                        const CodecList& recv_codecs);
  void set_audio_codecs(const std::vector<Codec>& send_codecs,
                        const std::vector<Codec>& recv_codecs) {
    set_audio_codecs(CodecList::CreateFromTrustedData(send_codecs),
                     CodecList::CreateFromTrustedData(recv_codecs));
  }
  void set_video_codecs(const CodecList& send_codecs,
                        const CodecList& recv_codecs);
  void set_video_codecs(const std::vector<Codec>& send_codecs,
                        const std::vector<Codec>& recv_codecs) {
    set_video_codecs(CodecList::CreateFromTrustedData(send_codecs),
                     CodecList::CreateFromTrustedData(recv_codecs));
  }
  CodecList audio_sendrecv_codecs() const;
  const CodecList& audio_send_codecs() const;
  const CodecList& audio_recv_codecs() const;
  CodecList video_sendrecv_codecs() const;
  const CodecList& video_send_codecs() const;
  const CodecList& video_recv_codecs() const;

 private:
  CodecList GetAudioCodecsForOffer(
      const RtpTransceiverDirection& direction) const;
  CodecList GetAudioCodecsForAnswer(
      const RtpTransceiverDirection& offer,
      const RtpTransceiverDirection& answer) const;
  CodecList GetVideoCodecsForOffer(
      const RtpTransceiverDirection& direction) const;
  CodecList GetVideoCodecsForAnswer(
      const RtpTransceiverDirection& offer,
      const RtpTransceiverDirection& answer) const;

  CodecList all_video_codecs() const;
  CodecList all_audio_codecs() const;

  TypedCodecVendor audio_send_codecs_;
  TypedCodecVendor audio_recv_codecs_;

  TypedCodecVendor video_send_codecs_;
  TypedCodecVendor video_recv_codecs_;
};

// A class to assist in looking up data for a codec mapping.
// Pure virtual to allow implementations that depend on things that
// codec_vendor.h should not depend on.
// Pointers returned are not stable, and should not be stored.
class CodecLookupHelper {
 public:
  virtual ~CodecLookupHelper() = default;
  virtual PayloadTypeSuggester* PayloadTypeSuggester() = 0;
  // Look up the codec vendor to use, depending on context.
  // This call may get additional arguments in the future, to aid
  // in selection of the correct context.
  virtual CodecVendor* GetCodecVendor() = 0;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::CodecLookupHelper;
using ::webrtc::CodecVendor;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // PC_CODEC_VENDOR_H_
