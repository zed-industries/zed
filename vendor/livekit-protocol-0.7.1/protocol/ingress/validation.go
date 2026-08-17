// Copyright 2023 LiveKit, Inc.
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

package ingress

import (
	"github.com/livekit/protocol/livekit"
)

// This validates that ingress options have no consistency issues and provide enough parameters
// to be usable by the ingress service. Options that pass this test may still need some fields to be poulated with default values
// before being used in a media pipeline.

func Validate(info *livekit.IngressInfo) error {
	if info == nil {
		return ErrInvalidIngress("missing IngressInfo")
	}

	// For now, require a room to be set. We should eventually allow changing the room on an active ingress
	if info.RoomName == "" {
		return ErrInvalidIngress("no room name")
	}

	return ValidateForSerialization(info)
}

// Sparse info with not all required fields populated are acceptable for serialization, provided values are consistent
func ValidateForSerialization(info *livekit.IngressInfo) error {
	if info == nil {
		return ErrInvalidIngress("missing IngressInfo")
	}

	if info.InputType != livekit.IngressInput_RTMP_INPUT && info.InputType != livekit.IngressInput_WHIP_INPUT && info.InputType != livekit.IngressInput_URL_INPUT {
		return ErrInvalidIngress("unsupported input type")
	}

	// Validate source
	switch info.InputType {
	case livekit.IngressInput_RTMP_INPUT,
		livekit.IngressInput_WHIP_INPUT:
		if info.StreamKey == "" {
			return ErrInvalidIngress("no stream key")
		}
	case livekit.IngressInput_URL_INPUT:
		if info.Url == "" {
			return ErrInvalidIngress("no source URL")
		}
		if info.Enabled != nil && !*info.Enabled {
			return ErrInvalidIngress("disabled non reusable ingress")
		}
	}

	if info.ParticipantIdentity == "" {
		return ErrInvalidIngress("no participant identity")
	}

	err := ValidateBypassTranscoding(info)
	if err != nil {
		return err
	}

	err = ValidateEnableTranscoding(info)
	if err != nil {
		return err
	}

	err = ValidateVideoOptionsConsistency(info.Video)
	if err != nil {
		return err
	}

	err = ValidateAudioOptionsConsistency(info.Audio)
	if err != nil {
		return err
	}

	return nil

}

func ValidateBypassTranscoding(info *livekit.IngressInfo) error {
	if !info.BypassTranscoding {
		return nil
	}

	if info.InputType != livekit.IngressInput_WHIP_INPUT {
		return NewInvalidTranscodingBypassError("bypassing transcoding impossible with selected input type")
	}

	videoOptions := info.Video
	if videoOptions != nil && videoOptions.EncodingOptions != nil {
		return NewInvalidTranscodingBypassError("video encoding options must be empty if transcoding bypass is selected")
	}

	audioOptions := info.Audio
	if audioOptions != nil && audioOptions.EncodingOptions != nil {
		return NewInvalidTranscodingBypassError("audio encoding options must be empty if transcoding bypass is selected")
	}

	return nil
}

func ValidateEnableTranscoding(info *livekit.IngressInfo) error {
	var enableTranscoding bool
	if info.InputType == livekit.IngressInput_WHIP_INPUT {
		enableTranscoding = false
		if info.EnableTranscoding != nil {
			enableTranscoding = *info.EnableTranscoding
		}
	} else {
		enableTranscoding = true
		if info.EnableTranscoding != nil && *info.EnableTranscoding == false {
			return NewInvalidTranscodingBypassError("bypassing transcoding impossible with selected input type")
		}
	}

	if !enableTranscoding {
		videoOptions := info.Video
		if videoOptions != nil && videoOptions.EncodingOptions != nil {
			return NewInvalidTranscodingBypassError("video encoding options must be empty if transcoding is disabled")
		}

		audioOptions := info.Audio
		if audioOptions != nil && audioOptions.EncodingOptions != nil {
			return NewInvalidTranscodingBypassError("audio encoding options must be empty if transcoding is disabled")
		}
	}

	return nil
}

func ValidateVideoOptionsConsistency(options *livekit.IngressVideoOptions) error {
	if options == nil {
		return nil
	}

	switch options.Source {
	case livekit.TrackSource_UNKNOWN,
		livekit.TrackSource_CAMERA,
		livekit.TrackSource_SCREEN_SHARE:
		// continue
	default:
		return NewInvalidVideoParamsError("invalid track source")
	}

	switch o := options.EncodingOptions.(type) {
	case nil:
		// Default, continue
	case *livekit.IngressVideoOptions_Preset:
		_, ok := livekit.IngressVideoEncodingPreset_name[int32(o.Preset)]
		if !ok {
			return NewInvalidVideoParamsError("invalid preset")
		}

	case *livekit.IngressVideoOptions_Options:
		err := ValidateVideoEncodingOptionsConsistency(o.Options)
		if err != nil {
			return err
		}
	}

	return nil
}

func ValidateVideoEncodingOptionsConsistency(options *livekit.IngressVideoEncodingOptions) error {
	if options == nil {
		return NewInvalidVideoParamsError("empty options")
	}

	if options.FrameRate < 0 {
		return NewInvalidVideoParamsError("invalid framerate")
	}

	switch options.VideoCodec {
	case livekit.VideoCodec_DEFAULT_VC,
		livekit.VideoCodec_H264_BASELINE,
		livekit.VideoCodec_VP8:
		// continue
	default:
		return NewInvalidVideoParamsError("video codec unsupported")
	}

	layersByQuality := make(map[livekit.VideoQuality]*livekit.VideoLayer)

	for _, layer := range options.Layers {
		if layer.Height == 0 || layer.Width == 0 {
			return ErrInvalidOutputDimensions
		}

		if layer.Width%2 == 1 {
			return ErrInvalidIngress("layer width must be even")
		}

		if layer.Height%2 == 1 {
			return ErrInvalidIngress("layer height must be even")
		}

		if _, ok := layersByQuality[layer.Quality]; ok {
			return NewInvalidVideoParamsError("more than one layer with the same quality level")
		}
		layersByQuality[layer.Quality] = layer
	}

	var oldW, oldH uint32
	for q := livekit.VideoQuality_LOW; q <= livekit.VideoQuality_HIGH; q++ {
		layer, ok := layersByQuality[q]
		if !ok {
			continue
		}

		if layer.Height < oldH {
			return NewInvalidVideoParamsError("video layers do not have increasing height with increasing quality")
		}
		if layer.Width < oldW {
			return NewInvalidVideoParamsError("video layers do not have increasing width with increasing quality")
		}
		oldW = layer.Width
		oldH = layer.Height
	}

	return nil
}

func ValidateAudioOptionsConsistency(options *livekit.IngressAudioOptions) error {
	if options == nil {
		return nil
	}

	switch options.Source {
	case livekit.TrackSource_UNKNOWN,
		livekit.TrackSource_MICROPHONE,
		livekit.TrackSource_SCREEN_SHARE_AUDIO:
		// continue
	default:
		return NewInvalidAudioParamsError("invalid track source")
	}

	switch o := options.EncodingOptions.(type) {
	case nil:
		// Default, continue
	case *livekit.IngressAudioOptions_Preset:
		_, ok := livekit.IngressAudioEncodingPreset_name[int32(o.Preset)]
		if !ok {
			return NewInvalidAudioParamsError("invalid preset")
		}

	case *livekit.IngressAudioOptions_Options:
		err := ValidateAudioEncodingOptionsConsistency(o.Options)
		if err != nil {
			return err
		}
	}

	return nil
}

func ValidateAudioEncodingOptionsConsistency(options *livekit.IngressAudioEncodingOptions) error {
	if options == nil {
		return NewInvalidAudioParamsError("empty options")
	}

	switch options.AudioCodec {
	case livekit.AudioCodec_DEFAULT_AC,
		livekit.AudioCodec_OPUS:
		// continue
	default:
		return NewInvalidAudioParamsError("audio codec unsupported")
	}

	if options.Channels > 2 {
		return NewInvalidAudioParamsError("invalid audio channel count")
	}

	return nil
}
