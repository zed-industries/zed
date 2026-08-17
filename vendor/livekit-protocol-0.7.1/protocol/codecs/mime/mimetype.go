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

package mime

import (
	"strings"

	"github.com/pion/webrtc/v4"

	"github.com/livekit/protocol/observability/roomobs"
)

const (
	MimeTypePrefixAudio = "audio/"
	MimeTypePrefixVideo = "video/"
)

type MimeTypeCodec int

const (
	MimeTypeCodecUnknown MimeTypeCodec = iota
	MimeTypeCodecH264
	MimeTypeCodecH265
	MimeTypeCodecOpus
	MimeTypeCodecRED
	MimeTypeCodecVP8
	MimeTypeCodecVP9
	MimeTypeCodecAV1
	MimeTypeCodecG722
	MimeTypeCodecPCMU
	MimeTypeCodecPCMA
	MimeTypeCodecRTX
	MimeTypeCodecFlexFEC
	MimeTypeCodecULPFEC
)

func (m MimeTypeCodec) String() string {
	switch m {
	case MimeTypeCodecUnknown:
		return "MimeTypeCodecUnknown"
	case MimeTypeCodecH264:
		return "H264"
	case MimeTypeCodecH265:
		return "H265"
	case MimeTypeCodecOpus:
		return "opus"
	case MimeTypeCodecRED:
		return "red"
	case MimeTypeCodecVP8:
		return "VP8"
	case MimeTypeCodecVP9:
		return "VP9"
	case MimeTypeCodecAV1:
		return "AV1"
	case MimeTypeCodecG722:
		return "G722"
	case MimeTypeCodecPCMU:
		return "PCMU"
	case MimeTypeCodecPCMA:
		return "PCMA"
	case MimeTypeCodecRTX:
		return "rtx"
	case MimeTypeCodecFlexFEC:
		return "flexfec"
	case MimeTypeCodecULPFEC:
		return "ulpfec"
	}

	return "MimeTypeCodecUnknown"
}

func (m MimeTypeCodec) ToMimeType() MimeType {
	switch m {
	case MimeTypeCodecUnknown:
		return MimeTypeUnknown
	case MimeTypeCodecH264:
		return MimeTypeH264
	case MimeTypeCodecH265:
		return MimeTypeH265
	case MimeTypeCodecOpus:
		return MimeTypeOpus
	case MimeTypeCodecRED:
		return MimeTypeRED
	case MimeTypeCodecVP8:
		return MimeTypeVP8
	case MimeTypeCodecVP9:
		return MimeTypeVP9
	case MimeTypeCodecAV1:
		return MimeTypeAV1
	case MimeTypeCodecG722:
		return MimeTypeG722
	case MimeTypeCodecPCMU:
		return MimeTypePCMU
	case MimeTypeCodecPCMA:
		return MimeTypePCMA
	case MimeTypeCodecRTX:
		return MimeTypeRTX
	case MimeTypeCodecFlexFEC:
		return MimeTypeFlexFEC
	case MimeTypeCodecULPFEC:
		return MimeTypeULPFEC
	}

	return MimeTypeUnknown
}

func NormalizeMimeTypeCodec(codec string) MimeTypeCodec {
	switch {
	case strings.EqualFold(codec, "h264"):
		return MimeTypeCodecH264
	case strings.EqualFold(codec, "h265"):
		return MimeTypeCodecH265
	case strings.EqualFold(codec, "opus"):
		return MimeTypeCodecOpus
	case strings.EqualFold(codec, "red"):
		return MimeTypeCodecRED
	case strings.EqualFold(codec, "vp8"):
		return MimeTypeCodecVP8
	case strings.EqualFold(codec, "vp9"):
		return MimeTypeCodecVP9
	case strings.EqualFold(codec, "av1"):
		return MimeTypeCodecAV1
	case strings.EqualFold(codec, "g722"):
		return MimeTypeCodecG722
	case strings.EqualFold(codec, "pcmu"):
		return MimeTypeCodecPCMU
	case strings.EqualFold(codec, "pcma"):
		return MimeTypeCodecPCMA
	case strings.EqualFold(codec, "rtx"):
		return MimeTypeCodecRTX
	case strings.EqualFold(codec, "flexfec"):
		return MimeTypeCodecFlexFEC
	case strings.EqualFold(codec, "ulpfec"):
		return MimeTypeCodecULPFEC
	}

	return MimeTypeCodecUnknown
}

func GetMimeTypeCodec(mime string) MimeTypeCodec {
	i := strings.IndexByte(mime, '/')
	if i == -1 {
		return MimeTypeCodecUnknown
	}

	return NormalizeMimeTypeCodec(mime[i+1:])
}

func IsMimeTypeCodecStringOpus(codec string) bool {
	return NormalizeMimeTypeCodec(codec) == MimeTypeCodecOpus
}

func IsMimeTypeCodecStringRED(codec string) bool {
	return NormalizeMimeTypeCodec(codec) == MimeTypeCodecRED
}

func IsMimeTypeCodecStringPCMA(codec string) bool {
	return NormalizeMimeTypeCodec(codec) == MimeTypeCodecPCMA
}

func IsMimeTypeCodecStringPCMU(codec string) bool {
	return NormalizeMimeTypeCodec(codec) == MimeTypeCodecPCMU
}

func IsMimeTypeCodecStringH264(codec string) bool {
	return NormalizeMimeTypeCodec(codec) == MimeTypeCodecH264
}

type MimeType int

const (
	MimeTypeUnknown MimeType = iota
	MimeTypeH264
	MimeTypeH265
	MimeTypeOpus
	MimeTypeRED
	MimeTypeVP8
	MimeTypeVP9
	MimeTypeAV1
	MimeTypeG722
	MimeTypePCMU
	MimeTypePCMA
	MimeTypeRTX
	MimeTypeFlexFEC
	MimeTypeULPFEC
)

func (m MimeType) String() string {
	switch m {
	case MimeTypeUnknown:
		return "MimeTypeUnknown"
	case MimeTypeH264:
		return webrtc.MimeTypeH264
	case MimeTypeH265:
		return webrtc.MimeTypeH265
	case MimeTypeOpus:
		return webrtc.MimeTypeOpus
	case MimeTypeRED:
		return "audio/red"
	case MimeTypeVP8:
		return webrtc.MimeTypeVP8
	case MimeTypeVP9:
		return webrtc.MimeTypeVP9
	case MimeTypeAV1:
		return webrtc.MimeTypeAV1
	case MimeTypeG722:
		return webrtc.MimeTypeG722
	case MimeTypePCMU:
		return webrtc.MimeTypePCMU
	case MimeTypePCMA:
		return webrtc.MimeTypePCMA
	case MimeTypeRTX:
		return webrtc.MimeTypeRTX
	case MimeTypeFlexFEC:
		return webrtc.MimeTypeFlexFEC
	case MimeTypeULPFEC:
		return "video/ulpfec"
	}

	return "MimeTypeUnknown"
}

func (m MimeType) ReporterType() roomobs.MimeType {
	switch m {
	case MimeTypeUnknown:
		return roomobs.MimeTypeUndefined
	case MimeTypeH264:
		return roomobs.MimeTypeVideoH264
	case MimeTypeH265:
		return roomobs.MimeTypeVideoH265
	case MimeTypeOpus:
		return roomobs.MimeTypeAudioOpus
	case MimeTypeRED:
		return roomobs.MimeTypeAudioRed
	case MimeTypeVP8:
		return roomobs.MimeTypeVideoVp8
	case MimeTypeVP9:
		return roomobs.MimeTypeVideoVp9
	case MimeTypeAV1:
		return roomobs.MimeTypeVideoAv1
	case MimeTypeG722:
		return roomobs.MimeTypeAudioG722
	case MimeTypePCMU:
		return roomobs.MimeTypeAudioPcmu
	case MimeTypePCMA:
		return roomobs.MimeTypeAudioPcma
	case MimeTypeRTX:
		return roomobs.MimeTypeVideoRtx
	case MimeTypeFlexFEC:
		return roomobs.MimeTypeVideoFlexfec
	case MimeTypeULPFEC:
		return roomobs.MimeTypeVideoUlpfec
	}

	return roomobs.MimeTypeUndefined
}

func NormalizeMimeType(mime string) MimeType {
	switch {
	case strings.EqualFold(mime, webrtc.MimeTypeH264):
		return MimeTypeH264
	case strings.EqualFold(mime, webrtc.MimeTypeH265):
		return MimeTypeH265
	case strings.EqualFold(mime, webrtc.MimeTypeOpus):
		return MimeTypeOpus
	case strings.EqualFold(mime, "audio/red"):
		return MimeTypeRED
	case strings.EqualFold(mime, webrtc.MimeTypeVP8):
		return MimeTypeVP8
	case strings.EqualFold(mime, webrtc.MimeTypeVP9):
		return MimeTypeVP9
	case strings.EqualFold(mime, webrtc.MimeTypeAV1):
		return MimeTypeAV1
	case strings.EqualFold(mime, webrtc.MimeTypeG722):
		return MimeTypeG722
	case strings.EqualFold(mime, webrtc.MimeTypePCMU):
		return MimeTypePCMU
	case strings.EqualFold(mime, webrtc.MimeTypePCMA):
		return MimeTypePCMA
	case strings.EqualFold(mime, webrtc.MimeTypeRTX):
		return MimeTypeRTX
	case strings.EqualFold(mime, webrtc.MimeTypeFlexFEC):
		return MimeTypeFlexFEC
	case strings.EqualFold(mime, "video/ulpfec"):
		return MimeTypeULPFEC
	}

	return MimeTypeUnknown
}

func IsMimeTypeStringEqual(mime1 string, mime2 string) bool {
	return NormalizeMimeType(mime1) == NormalizeMimeType(mime2)
}

func IsMimeTypeStringAudio(mime string) bool {
	return strings.HasPrefix(mime, MimeTypePrefixAudio)
}

func IsMimeTypeAudio(mimeType MimeType) bool {
	return strings.HasPrefix(mimeType.String(), MimeTypePrefixAudio)
}

func IsMimeTypeStringVideo(mime string) bool {
	return strings.HasPrefix(mime, MimeTypePrefixVideo)
}

func IsMimeTypeVideo(mimeType MimeType) bool {
	return strings.HasPrefix(mimeType.String(), MimeTypePrefixVideo)
}

func IsMimeTypeSVCCapable(mimeType MimeType) bool {
	switch mimeType {
	case MimeTypeAV1, MimeTypeVP9:
		return true
	}
	return false
}

func IsMimeTypeStringSVCCapable(mime string) bool {
	return IsMimeTypeSVCCapable(NormalizeMimeType(mime))
}

func IsMimeTypeStringRED(mime string) bool {
	return NormalizeMimeType(mime) == MimeTypeRED
}

func IsMimeTypeStringOpus(mime string) bool {
	return NormalizeMimeType(mime) == MimeTypeOpus
}

func IsMimeTypeStringPCMA(mime string) bool {
	return NormalizeMimeType(mime) == MimeTypePCMA
}

func IsMimeTypeStringPCMU(mime string) bool {
	return NormalizeMimeType(mime) == MimeTypePCMU
}

func IsMimeTypeStringRTX(mime string) bool {
	return NormalizeMimeType(mime) == MimeTypeRTX
}

func IsMimeTypeStringVP8(mime string) bool {
	return NormalizeMimeType(mime) == MimeTypeVP8
}

func IsMimeTypeStringVP9(mime string) bool {
	return NormalizeMimeType(mime) == MimeTypeVP9
}

func IsMimeTypeStringH264(mime string) bool {
	return NormalizeMimeType(mime) == MimeTypeH264
}
