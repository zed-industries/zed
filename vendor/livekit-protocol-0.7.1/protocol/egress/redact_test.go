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

	"google.golang.org/protobuf/proto"

	"github.com/livekit/protocol/livekit"
	"github.com/stretchr/testify/require"
)

var (
	file = &livekit.EncodedFileOutput{
		Output: &livekit.EncodedFileOutput_S3{
			S3: &livekit.S3Upload{
				AccessKey:            "ACCESS_KEY",
				Secret:               "LONG_SECRET_STRING",
				AssumeRoleExternalId: "EXTERNAL_ID",
				SessionToken:         "SESSION_TOKEN",
			},
		},
	}

	image = &livekit.ImageOutput{
		Output: &livekit.ImageOutput_AliOSS{
			AliOSS: &livekit.AliOSSUpload{
				AccessKey: "ACCESS_KEY",
				Secret:    "LONG_SECRET_STRING",
			},
		},
	}

	segments = &livekit.SegmentedFileOutput{
		Output: &livekit.SegmentedFileOutput_Gcp{
			Gcp: &livekit.GCPUpload{
				Credentials: "CREDENTIALS",
			},
		},
	}

	directFile = &livekit.DirectFileOutput{
		Output: &livekit.DirectFileOutput_Azure{
			Azure: &livekit.AzureBlobUpload{
				AccountName: "ACCOUNT_NAME",
				AccountKey:  "ACCOUNT_KEY",
			},
		},
	}
)

func TestRedactUpload(t *testing.T) {
	cl := proto.Clone(file)
	RedactUpload(cl.(UploadRequest))

	require.Equal(t, "{access_key}", cl.(*livekit.EncodedFileOutput).Output.(*livekit.EncodedFileOutput_S3).S3.AccessKey)
	require.Equal(t, "{secret}", cl.(*livekit.EncodedFileOutput).Output.(*livekit.EncodedFileOutput_S3).S3.Secret)
	require.Equal(t, "{external_id}", cl.(*livekit.EncodedFileOutput).Output.(*livekit.EncodedFileOutput_S3).S3.AssumeRoleExternalId)
	require.Equal(t, "{session_token}", cl.(*livekit.EncodedFileOutput).Output.(*livekit.EncodedFileOutput_S3).S3.SessionToken)

	cl = proto.Clone(image)
	RedactUpload(cl.(UploadRequest))

	require.Equal(t, "{access_key}", cl.(*livekit.ImageOutput).Output.(*livekit.ImageOutput_AliOSS).AliOSS.AccessKey)
	require.Equal(t, "{secret}", cl.(*livekit.ImageOutput).Output.(*livekit.ImageOutput_AliOSS).AliOSS.Secret)

	cl = proto.Clone(segments)
	RedactUpload(cl.(UploadRequest))

	require.Equal(t, "{credentials}", cl.(*livekit.SegmentedFileOutput).Output.(*livekit.SegmentedFileOutput_Gcp).Gcp.Credentials)

	cl = proto.Clone(directFile)
	RedactUpload(cl.(UploadRequest))

	require.Equal(t, "{account_name}", cl.(*livekit.DirectFileOutput).Output.(*livekit.DirectFileOutput_Azure).Azure.AccountName)
	require.Equal(t, "{account_key}", cl.(*livekit.DirectFileOutput).Output.(*livekit.DirectFileOutput_Azure).Azure.AccountKey)
}

func TestRedactStreamOutput(t *testing.T) {
	so := &livekit.StreamOutput{
		Urls: []string{
			"rtmps://foo.bar.com/app/secret_stream_key",
		},
	}

	RedactStreamKeys(so)
	require.Equal(t, "rtmps://foo.bar.com/app/{sec...key}", so.Urls[0])
}

func TestRedactEncodedOutputs(t *testing.T) {
	trackComposite := &livekit.TrackCompositeEgressRequest{
		FileOutputs: []*livekit.EncodedFileOutput{
			file,
		},
	}

	cl := proto.Clone(trackComposite)
	RedactEncodedOutputs(cl.(EncodedOutput))

	require.Equal(t, "{access_key}", cl.(*livekit.TrackCompositeEgressRequest).FileOutputs[0].Output.(*livekit.EncodedFileOutput_S3).S3.AccessKey)
	require.Equal(t, "{secret}", cl.(*livekit.TrackCompositeEgressRequest).FileOutputs[0].Output.(*livekit.EncodedFileOutput_S3).S3.Secret)

	roomComposite := &livekit.RoomCompositeEgressRequest{
		ImageOutputs: []*livekit.ImageOutput{
			image,
		},
	}

	cl = proto.Clone(roomComposite)
	RedactEncodedOutputs(cl.(EncodedOutput))

	require.Equal(t, "{access_key}", cl.(*livekit.RoomCompositeEgressRequest).ImageOutputs[0].Output.(*livekit.ImageOutput_AliOSS).AliOSS.AccessKey)
	require.Equal(t, "{secret}", cl.(*livekit.RoomCompositeEgressRequest).ImageOutputs[0].Output.(*livekit.ImageOutput_AliOSS).AliOSS.Secret)

	participant := &livekit.ParticipantEgressRequest{
		SegmentOutputs: []*livekit.SegmentedFileOutput{
			segments,
		},
	}

	cl = proto.Clone(participant)
	RedactEncodedOutputs(cl.(EncodedOutput))

	require.Equal(t, "{credentials}", cl.(*livekit.ParticipantEgressRequest).SegmentOutputs[0].Output.(*livekit.SegmentedFileOutput_Gcp).Gcp.Credentials)
}

func TestRedactDirectOutput(t *testing.T) {
	track := &livekit.TrackEgressRequest{
		Output: &livekit.TrackEgressRequest_File{
			File: &livekit.DirectFileOutput{
				Output: &livekit.DirectFileOutput_S3{
					S3: &livekit.S3Upload{
						AccessKey: "ACCESS_KEY",
						Secret:    "SECRET",
					},
				},
			},
		},
	}

	RedactDirectOutputs(track)
	require.Equal(t, "{access_key}", track.Output.(*livekit.TrackEgressRequest_File).File.Output.(*livekit.DirectFileOutput_S3).S3.AccessKey)
	require.Equal(t, "{secret}", track.Output.(*livekit.TrackEgressRequest_File).File.Output.(*livekit.DirectFileOutput_S3).S3.Secret)
}
