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

package egress

import "github.com/livekit/protocol/livekit"

const (
	EgressTypeRoomComposite  = "room_composite"
	EgressTypeWeb            = "web"
	EgressTypeParticipant    = "participant"
	EgressTypeTrackComposite = "track_composite"
	EgressTypeTrack          = "track"

	OutputTypeFile     = "file"
	OutputTypeStream   = "stream"
	OutputTypeSegments = "segments"
	OutputTypeImages   = "images"
	OutputTypeMultiple = "multiple"

	Unknown = "unknown"
)

// Outputs that can be used in egress that are started automatically on room creation
type AutoEncodedOutput interface {
	GetFileOutputs() []*livekit.EncodedFileOutput
	GetSegmentOutputs() []*livekit.SegmentedFileOutput
}

type EncodedOutput interface {
	AutoEncodedOutput
	GetStreamOutputs() []*livekit.StreamOutput
	GetImageOutputs() []*livekit.ImageOutput
}

type EncodedOutputDeprecated interface {
	GetFile() *livekit.EncodedFileOutput
	GetStream() *livekit.StreamOutput
	GetSegments() *livekit.SegmentedFileOutput
}

type DirectOutput interface {
	GetFile() *livekit.DirectFileOutput
	GetWebsocketUrl() string
}

type UploadRequest interface {
	GetS3() *livekit.S3Upload
	GetGcp() *livekit.GCPUpload
	GetAzure() *livekit.AzureBlobUpload
	GetAliOSS() *livekit.AliOSSUpload
}

func GetTypes(request interface{}) (string, string) {
	switch req := request.(type) {
	case *livekit.EgressInfo_RoomComposite:
		return EgressTypeRoomComposite, GetOutputType(req.RoomComposite)

	case *livekit.EgressInfo_Web:
		return EgressTypeWeb, GetOutputType(req.Web)

	case *livekit.EgressInfo_Participant:
		return EgressTypeParticipant, GetOutputType(req.Participant)

	case *livekit.EgressInfo_TrackComposite:
		return EgressTypeTrackComposite, GetOutputType(req.TrackComposite)

	case *livekit.EgressInfo_Track:
		return EgressTypeTrack, GetOutputType(req.Track)
	}

	return Unknown, Unknown
}

func GetOutputType(req interface{}) string {
	if r, ok := req.(EncodedOutput); ok {
		outputs := make([]string, 0)
		if len(r.GetFileOutputs()) > 0 {
			outputs = append(outputs, OutputTypeFile)
		}
		if len(r.GetStreamOutputs()) > 0 {
			outputs = append(outputs, OutputTypeStream)
		}
		if len(r.GetSegmentOutputs()) > 0 {
			outputs = append(outputs, OutputTypeSegments)
		}
		if len(r.GetImageOutputs()) > 0 {
			outputs = append(outputs, OutputTypeImages)
		}

		switch len(outputs) {
		default:
			return OutputTypeMultiple
		case 1:
			return outputs[0]
		case 0:
			if r, ok := req.(EncodedOutputDeprecated); ok {
				if r.GetFile() != nil {
					return OutputTypeFile
				}
				if r.GetStream() != nil {
					return OutputTypeStream
				}
				if r.GetSegments() != nil {
					return OutputTypeSegments
				}
			}
		}
	}

	if r, ok := req.(DirectOutput); ok {
		if r.GetFile() != nil {
			return OutputTypeFile
		}
		if r.GetWebsocketUrl() != "" {
			return OutputTypeStream
		}
	}

	return Unknown
}
