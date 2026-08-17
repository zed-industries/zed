// Copyright 2026 LiveKit, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package codecs

import (
	"strings"

	"github.com/livekit/protocol/codecs/mime"
	"github.com/livekit/protocol/livekit"
	"github.com/livekit/protocol/logger"
	"github.com/pion/webrtc/v4"
)

var (
	OpusCodecParameters = webrtc.RTPCodecParameters{
		RTPCodecCapability: webrtc.RTPCodecCapability{
			MimeType:    mime.MimeTypeOpus.String(),
			ClockRate:   48000,
			Channels:    2,
			SDPFmtpLine: "minptime=10;useinbandfec=1",
		},
		PayloadType: 111,
	}

	RedCodecParameters = webrtc.RTPCodecParameters{
		RTPCodecCapability: webrtc.RTPCodecCapability{
			MimeType:    mime.MimeTypeRED.String(),
			ClockRate:   48000,
			Channels:    2,
			SDPFmtpLine: "111/111",
		},
		PayloadType: 63,
	}

	PCMUCodecParameters = webrtc.RTPCodecParameters{
		RTPCodecCapability: webrtc.RTPCodecCapability{
			MimeType:  mime.MimeTypePCMU.String(),
			ClockRate: 8000,
		},
		PayloadType: 0,
	}

	PCMACodecParameters = webrtc.RTPCodecParameters{
		RTPCodecCapability: webrtc.RTPCodecCapability{
			MimeType:  mime.MimeTypePCMA.String(),
			ClockRate: 8000,
		},
		PayloadType: 8,
	}

	VideoRTXCodecParameters = webrtc.RTPCodecParameters{
		RTPCodecCapability: webrtc.RTPCodecCapability{
			MimeType:  mime.MimeTypeRTX.String(),
			ClockRate: 90000,
		},
	}

	VP8CodecParameters = webrtc.RTPCodecParameters{
		RTPCodecCapability: webrtc.RTPCodecCapability{
			MimeType:  mime.MimeTypeVP8.String(),
			ClockRate: 90000,
		},
		PayloadType: 96,
	}

	VP9ProfileId0CodecParameters = webrtc.RTPCodecParameters{
		RTPCodecCapability: webrtc.RTPCodecCapability{
			MimeType:    mime.MimeTypeVP9.String(),
			ClockRate:   90000,
			SDPFmtpLine: "profile-id=0",
		},
		PayloadType: 98,
	}

	VP9ProfileId1CodecParameters = webrtc.RTPCodecParameters{
		RTPCodecCapability: webrtc.RTPCodecCapability{
			MimeType:    mime.MimeTypeVP9.String(),
			ClockRate:   90000,
			SDPFmtpLine: "profile-id=1",
		},
		PayloadType: 100,
	}

	H264ProfileLevelId42e01fPacketizationMode0CodecParameters = webrtc.RTPCodecParameters{
		RTPCodecCapability: webrtc.RTPCodecCapability{
			MimeType:    mime.MimeTypeH264.String(),
			ClockRate:   90000,
			SDPFmtpLine: "level-asymmetry-allowed=1;packetization-mode=0;profile-level-id=42e01f",
		},
		PayloadType: 125,
	}

	H264ProfileLevelId42e01fPacketizationMode1CodecParameters = webrtc.RTPCodecParameters{
		RTPCodecCapability: webrtc.RTPCodecCapability{
			MimeType:    mime.MimeTypeH264.String(),
			ClockRate:   90000,
			SDPFmtpLine: "level-asymmetry-allowed=1;packetization-mode=1;profile-level-id=42e01f",
		},
		PayloadType: 108,
	}

	H264HighProfileFmtp            = "level-asymmetry-allowed=1;packetization-mode=1;profile-level-id=640032"
	H264HighProfileCodecParameters = webrtc.RTPCodecParameters{
		RTPCodecCapability: webrtc.RTPCodecCapability{
			MimeType:    mime.MimeTypeH264.String(),
			ClockRate:   90000,
			SDPFmtpLine: H264HighProfileFmtp,
		},
		PayloadType: 123,
	}

	AV1CodecParameters = webrtc.RTPCodecParameters{
		RTPCodecCapability: webrtc.RTPCodecCapability{
			MimeType:  mime.MimeTypeAV1.String(),
			ClockRate: 90000,
		},
		PayloadType: 35,
	}

	H265CodecParameters = webrtc.RTPCodecParameters{
		RTPCodecCapability: webrtc.RTPCodecCapability{
			MimeType:  mime.MimeTypeH265.String(),
			ClockRate: 90000,
		},
		PayloadType: 116,
	}

	VideoCodecsParameters = []webrtc.RTPCodecParameters{
		VP8CodecParameters,
		VP9ProfileId0CodecParameters,
		VP9ProfileId1CodecParameters,
		H264ProfileLevelId42e01fPacketizationMode0CodecParameters,
		H264ProfileLevelId42e01fPacketizationMode1CodecParameters,
		H264HighProfileCodecParameters,
		AV1CodecParameters,
		H265CodecParameters,
	}
)

func areFmtpLinesEqual(a, b string) bool {
	if a == b {
		return true
	}

	paramsA := make(map[string]string)
	for _, param := range strings.Split(a, ";") {
		if key, value, found := strings.Cut(param, "="); found {
			paramsA[key] = value
		}
	}

	count := 0
	for _, param := range strings.Split(b, ";") {
		if key, value, found := strings.Cut(param, "="); found {
			if paramsA[key] != value {
				return false
			}
			count++
		}
	}
	return count == len(paramsA)
}

func ToWebrtcCodecParameters(codec *livekit.Codec) webrtc.RTPCodecParameters {
	fmtp := codec.GetFmtpLine()
	var params webrtc.RTPCodecParameters

	switch mime.NormalizeMimeType(codec.GetMime()) {
	case mime.MimeTypeOpus:
		params = OpusCodecParameters
	case mime.MimeTypeRED:
		params = RedCodecParameters
	case mime.MimeTypePCMU:
		params = PCMUCodecParameters
	case mime.MimeTypePCMA:
		params = PCMACodecParameters
	case mime.MimeTypeRTX:
		params = VideoRTXCodecParameters
	case mime.MimeTypeVP8:
		params = VP8CodecParameters
	case mime.MimeTypeVP9:
		if strings.Contains(fmtp, "profile-id=1") {
			params = VP9ProfileId1CodecParameters
		} else {
			params = VP9ProfileId0CodecParameters
		}
	case mime.MimeTypeAV1:
		params = AV1CodecParameters
	case mime.MimeTypeH264:
		if strings.Contains(fmtp, "profile-level-id=640032") {
			params = H264HighProfileCodecParameters
		} else if strings.Contains(fmtp, "packetization-mode=0") {
			params = H264ProfileLevelId42e01fPacketizationMode0CodecParameters
		} else {
			params = H264ProfileLevelId42e01fPacketizationMode1CodecParameters
		}
	case mime.MimeTypeH265:
		params = H265CodecParameters
	default:
		return webrtc.RTPCodecParameters{}
	}

	if fmtp != "" && !areFmtpLinesEqual(fmtp, params.SDPFmtpLine) {
		logger.Warnw("non-standard fmtp, may not be supported",
			nil,
			"mime", codec.GetMime(),
			"fmtp", fmtp,
			"supportedFmtp", params.SDPFmtpLine,
		)
		params.SDPFmtpLine = fmtp
	}

	return params
}
