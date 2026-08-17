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
	"testing"

	"github.com/livekit/protocol/livekit"
	"github.com/stretchr/testify/require"
)

func TestValidate(t *testing.T) {
	info := &livekit.IngressInfo{}

	err := Validate(info)
	require.Error(t, err)

	info.StreamKey = "stream_key"
	err = Validate(info)
	require.Error(t, err)

	info.RoomName = "room_name"
	err = Validate(info)
	require.Error(t, err)

	info.ParticipantIdentity = "participant_identity"
	err = Validate(info)
	require.NoError(t, err)

	// make sure video parameters are validated. Full validation logic tested in the next test
	info.Video = &livekit.IngressVideoOptions{}
	err = Validate(info)
	require.NoError(t, err)

	info.Video.Source = livekit.TrackSource_MICROPHONE
	err = Validate(info)
	require.Error(t, err)

	info.Video.Source = livekit.TrackSource_CAMERA

	// make sure audio parameters are validated. Full validation logic tested in the next test
	info.Audio = &livekit.IngressAudioOptions{}
	err = Validate(info)
	require.NoError(t, err)

	info.Audio.Source = livekit.TrackSource_CAMERA
	err = Validate(info)
	require.Error(t, err)

	info.Audio.Source = livekit.TrackSource_SCREEN_SHARE_AUDIO
	err = Validate(info)
	require.NoError(t, err)
}

func TestValidateBypassTranscoding(t *testing.T) {
	info := &livekit.IngressInfo{}

	err := ValidateBypassTranscoding(info)
	require.NoError(t, err)

	info.BypassTranscoding = true
	err = ValidateBypassTranscoding(info)
	require.Error(t, err)

	info.InputType = livekit.IngressInput_WHIP_INPUT
	err = ValidateBypassTranscoding(info)
	require.NoError(t, err)

	info.Video = &livekit.IngressVideoOptions{}
	err = ValidateBypassTranscoding(info)
	require.NoError(t, err)

	info.Video.EncodingOptions = &livekit.IngressVideoOptions_Preset{}
	err = ValidateBypassTranscoding(info)
	require.Error(t, err)

	info.Video = nil

	info.Audio = &livekit.IngressAudioOptions{}
	err = ValidateBypassTranscoding(info)
	require.NoError(t, err)

	info.Audio.EncodingOptions = &livekit.IngressAudioOptions_Options{
		Options: &livekit.IngressAudioEncodingOptions{},
	}
	err = ValidateBypassTranscoding(info)
	require.Error(t, err)

}

func TestValidateEnableTranscoding(t *testing.T) {
	info := &livekit.IngressInfo{}
	T := true
	F := false

	err := ValidateEnableTranscoding(info)
	require.NoError(t, err)

	info.InputType = livekit.IngressInput_WHIP_INPUT
	err = ValidateEnableTranscoding(info)
	require.NoError(t, err)

	info.Audio = &livekit.IngressAudioOptions{}
	info.Audio.EncodingOptions = &livekit.IngressAudioOptions_Options{}
	err = ValidateEnableTranscoding(info)
	require.Error(t, err)

	info.Audio.EncodingOptions = nil

	info.EnableTranscoding = &T
	err = ValidateEnableTranscoding(info)
	require.NoError(t, err)

	info.EnableTranscoding = &F
	err = ValidateEnableTranscoding(info)
	require.NoError(t, err)

	info.Video = &livekit.IngressVideoOptions{}
	info.Video.EncodingOptions = &livekit.IngressVideoOptions_Preset{}
	err = ValidateEnableTranscoding(info)
	require.Error(t, err)

	info.Video.EncodingOptions = nil

	info.InputType = livekit.IngressInput_RTMP_INPUT
	err = ValidateEnableTranscoding(info)
	require.Error(t, err)

	info.EnableTranscoding = &T
	err = ValidateEnableTranscoding(info)
	require.NoError(t, err)
}

func TestValidateEnabled(t *testing.T) {
	info := &livekit.IngressInfo{
		StreamKey:           "sk",
		RoomName:            "room_name",
		ParticipantIdentity: "id",
	}
	T := true
	F := false

	err := Validate(info)
	require.NoError(t, err)

	info.Enabled = &T
	err = Validate(info)
	require.NoError(t, err)

	info.Enabled = &F
	err = Validate(info)
	require.NoError(t, err)

	info.InputType = livekit.IngressInput_URL_INPUT
	info.Url = "url"
	info.Enabled = nil
	err = Validate(info)
	require.NoError(t, err)

	info.Enabled = &T
	err = Validate(info)
	require.NoError(t, err)

	info.Enabled = &F
	err = Validate(info)
	require.Error(t, err)
}

func TestValidateVideoOptionsConsistency(t *testing.T) {
	video := &livekit.IngressVideoOptions{}
	err := ValidateVideoOptionsConsistency(video)
	require.NoError(t, err)

	video.Source = livekit.TrackSource_MICROPHONE
	err = ValidateVideoOptionsConsistency(video)
	require.Error(t, err)

	video.Source = livekit.TrackSource_CAMERA
	video.EncodingOptions = &livekit.IngressVideoOptions_Preset{
		Preset: livekit.IngressVideoEncodingPreset(42),
	}
	err = ValidateVideoOptionsConsistency(video)
	require.Error(t, err)

	video.EncodingOptions = &livekit.IngressVideoOptions_Preset{
		Preset: livekit.IngressVideoEncodingPreset_H264_1080P_30FPS_1_LAYER,
	}
	err = ValidateVideoOptionsConsistency(video)
	require.NoError(t, err)

	video.EncodingOptions = &livekit.IngressVideoOptions_Options{
		Options: &livekit.IngressVideoEncodingOptions{
			VideoCodec: livekit.VideoCodec_H264_HIGH,
		},
	}
	err = ValidateVideoOptionsConsistency(video)
	require.Error(t, err)

	video.EncodingOptions = &livekit.IngressVideoOptions_Options{
		Options: &livekit.IngressVideoEncodingOptions{
			VideoCodec: livekit.VideoCodec_DEFAULT_VC,
		},
	}
	err = ValidateVideoOptionsConsistency(video)
	require.NoError(t, err)

	video.EncodingOptions.(*livekit.IngressVideoOptions_Options).Options.Layers = []*livekit.VideoLayer{
		&livekit.VideoLayer{},
	}
	err = ValidateVideoOptionsConsistency(video)
	require.Error(t, err)

	video.EncodingOptions.(*livekit.IngressVideoOptions_Options).Options.Layers = []*livekit.VideoLayer{
		&livekit.VideoLayer{
			Width:  640,
			Height: 480,
		},
	}
	err = ValidateVideoOptionsConsistency(video)
	require.NoError(t, err)

	video.EncodingOptions.(*livekit.IngressVideoOptions_Options).Options.Layers = []*livekit.VideoLayer{
		&livekit.VideoLayer{
			Width:  641,
			Height: 480,
		},
	}
	err = ValidateVideoOptionsConsistency(video)
	require.Error(t, err)

	video.EncodingOptions.(*livekit.IngressVideoOptions_Options).Options.Layers = []*livekit.VideoLayer{
		&livekit.VideoLayer{
			Width:   640,
			Height:  480,
			Quality: livekit.VideoQuality_HIGH,
		},
		&livekit.VideoLayer{
			Width:   640,
			Height:  480,
			Quality: livekit.VideoQuality_LOW,
		},
	}
	err = ValidateVideoOptionsConsistency(video)
	require.NoError(t, err)

	video.EncodingOptions.(*livekit.IngressVideoOptions_Options).Options.Layers = []*livekit.VideoLayer{
		&livekit.VideoLayer{
			Width:   640,
			Height:  480,
			Quality: livekit.VideoQuality_HIGH,
		},
		&livekit.VideoLayer{
			Width:   1280,
			Height:  720,
			Quality: livekit.VideoQuality_HIGH,
		},
	}
	err = ValidateVideoOptionsConsistency(video)
	require.Error(t, err)

	video.EncodingOptions.(*livekit.IngressVideoOptions_Options).Options.Layers = []*livekit.VideoLayer{
		&livekit.VideoLayer{
			Width:   640,
			Height:  480,
			Quality: livekit.VideoQuality_HIGH,
		},
		&livekit.VideoLayer{
			Width:   1280,
			Height:  720,
			Quality: livekit.VideoQuality_LOW,
		},
	}
	err = ValidateVideoOptionsConsistency(video)
	require.Error(t, err)

	video.EncodingOptions.(*livekit.IngressVideoOptions_Options).Options.Layers = []*livekit.VideoLayer{
		&livekit.VideoLayer{
			Width:   640,
			Height:  480,
			Quality: livekit.VideoQuality_LOW,
		},
		&livekit.VideoLayer{
			Width:   1280,
			Height:  720,
			Quality: livekit.VideoQuality_HIGH,
		},
	}

	err = ValidateVideoOptionsConsistency(video)
	require.NoError(t, err)
}

func TestValidateAudioOptionsConsistency(t *testing.T) {
	audio := &livekit.IngressAudioOptions{}
	err := ValidateAudioOptionsConsistency(audio)
	require.NoError(t, err)

	audio.Source = livekit.TrackSource_CAMERA
	err = ValidateAudioOptionsConsistency(audio)
	require.Error(t, err)

	audio.Source = livekit.TrackSource_SCREEN_SHARE_AUDIO
	audio.EncodingOptions = &livekit.IngressAudioOptions_Preset{
		Preset: livekit.IngressAudioEncodingPreset(42),
	}
	err = ValidateAudioOptionsConsistency(audio)
	require.Error(t, err)

	audio.EncodingOptions = &livekit.IngressAudioOptions_Preset{
		Preset: livekit.IngressAudioEncodingPreset_OPUS_MONO_64KBS,
	}
	err = ValidateAudioOptionsConsistency(audio)
	require.NoError(t, err)

	audio.EncodingOptions = &livekit.IngressAudioOptions_Options{
		Options: &livekit.IngressAudioEncodingOptions{
			AudioCodec: livekit.AudioCodec_AAC,
		},
	}
	err = ValidateAudioOptionsConsistency(audio)
	require.Error(t, err)

	audio.EncodingOptions = &livekit.IngressAudioOptions_Options{
		Options: &livekit.IngressAudioEncodingOptions{
			AudioCodec: livekit.AudioCodec_OPUS,
			Channels:   3,
		},
	}
	err = ValidateAudioOptionsConsistency(audio)
	require.Error(t, err)

	audio.EncodingOptions.(*livekit.IngressAudioOptions_Options).Options.Channels = 2
	err = ValidateAudioOptionsConsistency(audio)
	require.NoError(t, err)
}
