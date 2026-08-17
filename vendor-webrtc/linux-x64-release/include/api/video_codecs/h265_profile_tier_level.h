/*
 *  Copyright (c) 2023 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_CODECS_H265_PROFILE_TIER_LEVEL_H_
#define API_VIDEO_CODECS_H265_PROFILE_TIER_LEVEL_H_

#include <optional>
#include <string>

#include "api/rtp_parameters.h"
#include "api/video/resolution.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// Profiles can be found at:
// https://www.itu.int/rec/T-REC-H.265
// The enum values match the number specified in the SDP.
enum class H265Profile {
  kProfileMain = 1,
  kProfileMain10 = 2,
  kProfileMainStill = 3,
  kProfileRangeExtensions = 4,
  kProfileHighThroughput = 5,
  kProfileMultiviewMain = 6,
  kProfileScalableMain = 7,
  kProfile3dMain = 8,
  kProfileScreenContentCoding = 9,
  kProfileScalableRangeExtensions = 10,
  kProfileHighThroughputScreenContentCoding = 11,
};

// Tiers can be found at https://www.itu.int/rec/T-REC-H.265
enum class H265Tier {
  kTier0,
  kTier1,
};

// All values are equal to 30 times the level number.
enum class H265Level {
  kLevel1 = 30,
  kLevel2 = 60,
  kLevel2_1 = 63,
  kLevel3 = 90,
  kLevel3_1 = 93,
  kLevel4 = 120,
  kLevel4_1 = 123,
  kLevel5 = 150,
  kLevel5_1 = 153,
  kLevel5_2 = 156,
  kLevel6 = 180,
  kLevel6_1 = 183,
  kLevel6_2 = 186,
};

struct H265ProfileTierLevel {
  constexpr H265ProfileTierLevel(H265Profile profile,
                                 H265Tier tier,
                                 H265Level level)
      : profile(profile), tier(tier), level(level) {}
  H265Profile profile;
  H265Tier tier;
  H265Level level;
};

// Helper function to convert H265Profile to std::string.
RTC_EXPORT std::string H265ProfileToString(H265Profile profile);

// Helper function to convert H265Tier to std::string.
RTC_EXPORT std::string H265TierToString(H265Tier tier);

// Helper function to convert H265Level to std::string.
RTC_EXPORT std::string H265LevelToString(H265Level level);

// Helper function to get H265Profile from profile string.
RTC_EXPORT std::optional<H265Profile> StringToH265Profile(
    const std::string& profile);

// Helper function to get H265Tier from tier string.
RTC_EXPORT std::optional<H265Tier> StringToH265Tier(const std::string& tier);

// Helper function to get H265Level from level string.
RTC_EXPORT std::optional<H265Level> StringToH265Level(const std::string& level);

// Given that a decoder supports up to a give frame size(in pixels) at up to a
// given number of frames per second, return the highest H.265 level where it
// can guranatee that it will be able to support all valid encoded streams that
// are within that level.
RTC_EXPORT std::optional<H265Level> GetSupportedH265Level(
    const Resolution& resolution,
    float max_fps);

// Parses an SDP key-value map of format parameters to retrive an H265
// profile/tier/level. Returns an H265ProfileTierlevel by setting its
// members. profile defaults to `kProfileMain` if no profile-id is specified.
// tier defaults to "kTier0" if no tier-flag is specified.
// level defaults to "kLevel3_1" if no level-id is specified.
// Returns empty value if any of the profile/tier/level key is present but
// contains an invalid value.
RTC_EXPORT std::optional<H265ProfileTierLevel> ParseSdpForH265ProfileTierLevel(
    const CodecParameterMap& params);

// Returns true if the parameters have the same H265 profile/tier/level or
// neither contains an H265 profile/tier/level, otherwise false.
RTC_EXPORT bool H265IsSameProfileTierLevel(const CodecParameterMap& params1,
                                           const CodecParameterMap& params2);

// Returns true if the parameters have the same H265 profile, or neither
// contains an H265 profile, otherwise false.
RTC_EXPORT bool H265IsSameProfile(const CodecParameterMap& params1,
                                  const CodecParameterMap& params2);

// Returns true if the parameters have the same H265 tier, or neither
// contains an H265 tier, otherwise false.
RTC_EXPORT bool H265IsSameTier(const CodecParameterMap& params1,
                               const CodecParameterMap& params2);

}  // namespace webrtc

#endif  // API_VIDEO_CODECS_H265_PROFILE_TIER_LEVEL_H_
