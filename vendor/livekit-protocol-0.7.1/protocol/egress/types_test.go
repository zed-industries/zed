// Copyright 2025 LiveKit, Inc.
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

import (
	"testing"

	"github.com/livekit/protocol/livekit"
	"github.com/stretchr/testify/require"
)

func TestGetOutputType(t *testing.T) {
	roomReq := &livekit.RoomCompositeEgressRequest{
		FileOutputs: []*livekit.EncodedFileOutput{
			&livekit.EncodedFileOutput{},
		},
	}

	ot := GetOutputType(roomReq)
	require.Equal(t, OutputTypeFile, ot)

	roomReq = &livekit.RoomCompositeEgressRequest{
		Output: &livekit.RoomCompositeEgressRequest_File{
			File: &livekit.EncodedFileOutput{},
		},
	}

	ot = GetOutputType(roomReq)
	require.Equal(t, OutputTypeFile, ot)

	trackReq := &livekit.TrackCompositeEgressRequest{
		SegmentOutputs: []*livekit.SegmentedFileOutput{
			&livekit.SegmentedFileOutput{},
		},
	}

	ot = GetOutputType(trackReq)
	require.Equal(t, OutputTypeSegments, ot)

	trackReq = &livekit.TrackCompositeEgressRequest{
		Output: &livekit.TrackCompositeEgressRequest_Segments{
			Segments: &livekit.SegmentedFileOutput{},
		},
	}

	ot = GetOutputType(trackReq)
	require.Equal(t, OutputTypeSegments, ot)

	webReq := &livekit.WebEgressRequest{
		StreamOutputs: []*livekit.StreamOutput{
			&livekit.StreamOutput{},
		},
	}

	ot = GetOutputType(webReq)
	require.Equal(t, OutputTypeStream, ot)

	webReq = &livekit.WebEgressRequest{
		Output: &livekit.WebEgressRequest_Stream{
			Stream: &livekit.StreamOutput{},
		},
	}

	ot = GetOutputType(webReq)
	require.Equal(t, OutputTypeStream, ot)

	participantReq := &livekit.ParticipantEgressRequest{
		ImageOutputs: []*livekit.ImageOutput{
			&livekit.ImageOutput{},
		},
	}

	ot = GetOutputType(participantReq)
	require.Equal(t, OutputTypeImages, ot)

	participantReq.SegmentOutputs = []*livekit.SegmentedFileOutput{
		&livekit.SegmentedFileOutput{},
	}

	ot = GetOutputType(participantReq)
	require.Equal(t, OutputTypeMultiple, ot)

}
