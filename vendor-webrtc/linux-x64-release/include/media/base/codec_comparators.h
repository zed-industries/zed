/*
 *  Copyright 2024 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MEDIA_BASE_CODEC_COMPARATORS_H_
#define MEDIA_BASE_CODEC_COMPARATORS_H_

#include <optional>
#include <vector>

#include "api/rtp_parameters.h"
#include "media/base/codec.h"

namespace webrtc {

// Comparison used for the Codec::Matches function
bool MatchesWithCodecRules(const Codec& left_codec, const Codec& codec);

// Comparison that also checks on codecs referenced by PT in the
// fmtp line, as used with RED and RTX "codecs".
bool MatchesWithReferenceAttributes(const Codec& left_codec,
                                    const Codec& right_codec);

// Finds a codec in `codecs2` that matches `codec_to_match`, which is
// a member of `codecs1`. If `codec_to_match` is an RED or RTX codec, both
// the codecs themselves and their associated codecs must match.
// The purpose of this function is that codecs1 and codecs2 are different
// PT numbering spaces, and it is trying to find the codec in codecs2
// that has the same functionality as `codec_to_match` so that its PT
// can be used in place of the original.
std::optional<Codec> FindMatchingCodec(const std::vector<Codec>& codecs1,
                                       const std::vector<Codec>& codecs2,
                                       const Codec& codec_to_match);

// Similar to `Codec::MatchesRtpCodec` but not an exact match of parameters.
// Unspecified parameters are treated as default.
bool IsSameRtpCodec(const Codec& codec, const RtpCodec& rtp_codec);

// Similar to `IsSameRtpCodec` but ignoring the level related parameter.
bool IsSameRtpCodecIgnoringLevel(const Codec& codec, const RtpCodec& rtp_codec);
}  // namespace webrtc

#endif  // MEDIA_BASE_CODEC_COMPARATORS_H_
