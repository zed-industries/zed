/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MEDIA_BASE_MEDIA_CONSTANTS_H_
#define MEDIA_BASE_MEDIA_CONSTANTS_H_

#include <stddef.h>

#include "rtc_base/system/rtc_export.h"

// This file contains constants related to media.

namespace webrtc {

extern const int kVideoCodecClockrate;

extern const int kVideoMtu;
extern const int kVideoRtpSendBufferSize;
extern const int kVideoRtpRecvBufferSize;

// Default CPU thresholds.
extern const float kHighSystemCpuThreshold;
extern const float kLowSystemCpuThreshold;
extern const float kProcessCpuThreshold;

extern const char kRedCodecName[];
extern const char kUlpfecCodecName[];
extern const char kFlexfecCodecName[];
extern const char kMultiplexCodecName[];

extern const char kFlexfecFmtpRepairWindow[];

extern const char kRtxCodecName[];
extern const char kCodecParamRtxTime[];
extern const char kCodecParamAssociatedPayloadType[];

extern const char kCodecParamAssociatedCodecName[];
extern const char kCodecParamNotInNameValueFormat[];

extern const char kOpusCodecName[];
extern const char kL16CodecName[];
extern const char kG722CodecName[];
extern const char kPcmuCodecName[];
extern const char kPcmaCodecName[];
extern const char kCnCodecName[];
extern const char kDtmfCodecName[];

// Attribute parameters
extern const char kCodecParamPTime[];
extern const char kCodecParamMaxPTime[];
// fmtp parameters
extern const char kCodecParamMinPTime[];
extern const char kCodecParamSPropStereo[];
extern const char kCodecParamStereo[];
extern const char kCodecParamUseInbandFec[];
extern const char kCodecParamUseDtx[];
extern const char kCodecParamCbr[];
extern const char kCodecParamMaxAverageBitrate[];
extern const char kCodecParamMaxPlaybackRate[];
extern const char kCodecParamPerLayerPictureLossIndication[];

extern const char kParamValueTrue[];
// Parameters are stored as parameter/value pairs. For parameters who do not
// have a value, `kParamValueEmpty` should be used as value.
extern const char kParamValueEmpty[];

// opus parameters.
// Default value for maxptime according to
// http://tools.ietf.org/html/draft-spittka-payload-rtp-opus-03
extern const int kOpusDefaultMaxPTime;
extern const int kOpusDefaultPTime;
extern const int kOpusDefaultMinPTime;
extern const int kOpusDefaultSPropStereo;
extern const int kOpusDefaultStereo;
extern const int kOpusDefaultUseInbandFec;
extern const int kOpusDefaultUseDtx;
extern const int kOpusDefaultMaxPlaybackRate;

// Prefered values in this code base. Note that they may differ from the default
// values in http://tools.ietf.org/html/draft-spittka-payload-rtp-opus-03
// Only frames larger or equal to 10 ms are currently supported in this code
// base.
extern const int kPreferredMaxPTime;
extern const int kPreferredMinPTime;
extern const int kPreferredSPropStereo;
extern const int kPreferredStereo;
extern const int kPreferredUseInbandFec;

extern const char kPacketizationParamRaw[];

// rtcp-fb message in its first experimental stages. Documentation pending.
extern const char kRtcpFbParamLntf[];
// rtcp-fb messages according to RFC 4585
extern const char kRtcpFbParamNack[];
extern const char kRtcpFbNackParamPli[];
// rtcp-fb messages according to
// http://tools.ietf.org/html/draft-alvestrand-rmcat-remb-00
extern const char kRtcpFbParamRemb[];
// rtcp-fb messages according to
// https://tools.ietf.org/html/draft-holmer-rmcat-transport-wide-cc-extensions-01
extern const char kRtcpFbParamTransportCc[];
// ccm submessages according to RFC 5104
extern const char kRtcpFbParamCcm[];
extern const char kRtcpFbCcmParamFir[];
// Receiver reference time report
// https://tools.ietf.org/html/rfc3611 section 4.4
extern const char kRtcpFbParamRrtr[];
// Google specific parameters
extern const char kCodecParamMaxBitrate[];
extern const char kCodecParamMinBitrate[];
extern const char kCodecParamStartBitrate[];
extern const char kCodecParamMaxQuantization[];

extern const char kComfortNoiseCodecName[];

RTC_EXPORT extern const char kVp8CodecName[];
RTC_EXPORT extern const char kVp9CodecName[];
RTC_EXPORT extern const char kAv1CodecName[];
RTC_EXPORT extern const char kH264CodecName[];
RTC_EXPORT extern const char kH265CodecName[];

// RFC 6184 RTP Payload Format for H.264 video
RTC_EXPORT extern const char kH264FmtpProfileLevelId[];
RTC_EXPORT extern const char kH264FmtpLevelAsymmetryAllowed[];
RTC_EXPORT extern const char kH264FmtpPacketizationMode[];
extern const char kH264FmtpSpropParameterSets[];
extern const char kH264FmtpSpsPpsIdrInKeyframe[];
extern const char kH264ProfileLevelConstrainedBaseline[];
extern const char kH264ProfileLevelConstrainedHigh[];

// RFC 7798 RTP Payload Format for H.265 video.
// According to RFC 7742, the sprop parameters MUST NOT be included
// in SDP generated by WebRTC, so for H.265 we don't handle them, though
// current H.264 implementation honors them when receiving
// sprop-parameter-sets in SDP.
RTC_EXPORT extern const char kH265FmtpProfileSpace[];
RTC_EXPORT extern const char kH265FmtpTierFlag[];
RTC_EXPORT extern const char kH265FmtpProfileId[];
RTC_EXPORT extern const char kH265FmtpLevelId[];
RTC_EXPORT extern const char kH265FmtpProfileCompatibilityIndicator[];
RTC_EXPORT extern const char kH265FmtpInteropConstraints[];
RTC_EXPORT extern const char kH265FmtpTxMode[];

// draft-ietf-payload-vp9
extern const char kVP9ProfileId[];

// https://aomediacodec.github.io/av1-rtp-spec/
extern const char kAv1FmtpProfile[];
extern const char kAv1FmtpLevelIdx[];
extern const char kAv1FmtpTier[];

extern const int kDefaultVideoMaxFramerate;
extern const int kDefaultVideoMaxQpVpx;
extern const int kDefaultVideoMaxQpAv1;
extern const int kDefaultVideoMaxQpH26x;

extern const size_t kConferenceMaxNumSpatialLayers;
extern const size_t kConferenceMaxNumTemporalLayers;
extern const size_t kConferenceDefaultNumTemporalLayers;

extern const char kApplicationSpecificBandwidth[];
extern const char kTransportSpecificBandwidth[];
}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::kApplicationSpecificBandwidth;
using ::webrtc::kAv1CodecName;
using ::webrtc::kAv1FmtpLevelIdx;
using ::webrtc::kAv1FmtpProfile;
using ::webrtc::kAv1FmtpTier;
using ::webrtc::kCnCodecName;
using ::webrtc::kCodecParamAssociatedCodecName;
using ::webrtc::kCodecParamAssociatedPayloadType;
using ::webrtc::kCodecParamCbr;
using ::webrtc::kCodecParamMaxAverageBitrate;
using ::webrtc::kCodecParamMaxBitrate;
using ::webrtc::kCodecParamMaxPlaybackRate;
using ::webrtc::kCodecParamMaxPTime;
using ::webrtc::kCodecParamMaxQuantization;
using ::webrtc::kCodecParamMinBitrate;
using ::webrtc::kCodecParamMinPTime;
using ::webrtc::kCodecParamNotInNameValueFormat;
using ::webrtc::kCodecParamPerLayerPictureLossIndication;
using ::webrtc::kCodecParamPTime;
using ::webrtc::kCodecParamRtxTime;
using ::webrtc::kCodecParamSPropStereo;
using ::webrtc::kCodecParamStartBitrate;
using ::webrtc::kCodecParamStereo;
using ::webrtc::kCodecParamUseDtx;
using ::webrtc::kCodecParamUseInbandFec;
using ::webrtc::kComfortNoiseCodecName;
using ::webrtc::kConferenceDefaultNumTemporalLayers;
using ::webrtc::kConferenceMaxNumSpatialLayers;
using ::webrtc::kConferenceMaxNumTemporalLayers;
using ::webrtc::kDefaultVideoMaxFramerate;
using ::webrtc::kDefaultVideoMaxQpAv1;
using ::webrtc::kDefaultVideoMaxQpH26x;
using ::webrtc::kDefaultVideoMaxQpVpx;
using ::webrtc::kDtmfCodecName;
using ::webrtc::kFlexfecCodecName;
using ::webrtc::kFlexfecFmtpRepairWindow;
using ::webrtc::kG722CodecName;
using ::webrtc::kH264CodecName;
using ::webrtc::kH264FmtpLevelAsymmetryAllowed;
using ::webrtc::kH264FmtpPacketizationMode;
using ::webrtc::kH264FmtpProfileLevelId;
using ::webrtc::kH264FmtpSpropParameterSets;
using ::webrtc::kH264FmtpSpsPpsIdrInKeyframe;
using ::webrtc::kH264ProfileLevelConstrainedBaseline;
using ::webrtc::kH264ProfileLevelConstrainedHigh;
using ::webrtc::kH265CodecName;
using ::webrtc::kH265FmtpInteropConstraints;
using ::webrtc::kH265FmtpLevelId;
using ::webrtc::kH265FmtpProfileCompatibilityIndicator;
using ::webrtc::kH265FmtpProfileId;
using ::webrtc::kH265FmtpProfileSpace;
using ::webrtc::kH265FmtpTierFlag;
using ::webrtc::kH265FmtpTxMode;
using ::webrtc::kHighSystemCpuThreshold;
using ::webrtc::kL16CodecName;
using ::webrtc::kLowSystemCpuThreshold;
using ::webrtc::kMultiplexCodecName;
using ::webrtc::kOpusCodecName;
using ::webrtc::kOpusDefaultMaxPlaybackRate;
using ::webrtc::kOpusDefaultMaxPTime;
using ::webrtc::kOpusDefaultMinPTime;
using ::webrtc::kOpusDefaultPTime;
using ::webrtc::kOpusDefaultSPropStereo;
using ::webrtc::kOpusDefaultStereo;
using ::webrtc::kOpusDefaultUseDtx;
using ::webrtc::kOpusDefaultUseInbandFec;
using ::webrtc::kPacketizationParamRaw;
using ::webrtc::kParamValueEmpty;
using ::webrtc::kParamValueTrue;
using ::webrtc::kPcmaCodecName;
using ::webrtc::kPcmuCodecName;
using ::webrtc::kPreferredMaxPTime;
using ::webrtc::kPreferredMinPTime;
using ::webrtc::kPreferredSPropStereo;
using ::webrtc::kPreferredStereo;
using ::webrtc::kPreferredUseInbandFec;
using ::webrtc::kProcessCpuThreshold;
using ::webrtc::kRedCodecName;
using ::webrtc::kRtcpFbCcmParamFir;
using ::webrtc::kRtcpFbNackParamPli;
using ::webrtc::kRtcpFbParamCcm;
using ::webrtc::kRtcpFbParamLntf;
using ::webrtc::kRtcpFbParamNack;
using ::webrtc::kRtcpFbParamRemb;
using ::webrtc::kRtcpFbParamRrtr;
using ::webrtc::kRtcpFbParamTransportCc;
using ::webrtc::kRtxCodecName;
using ::webrtc::kTransportSpecificBandwidth;
using ::webrtc::kUlpfecCodecName;
using ::webrtc::kVideoCodecClockrate;
using ::webrtc::kVideoMtu;
using ::webrtc::kVideoRtpRecvBufferSize;
using ::webrtc::kVideoRtpSendBufferSize;
using ::webrtc::kVp8CodecName;
using ::webrtc::kVp9CodecName;
using ::webrtc::kVP9ProfileId;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // MEDIA_BASE_MEDIA_CONSTANTS_H_
