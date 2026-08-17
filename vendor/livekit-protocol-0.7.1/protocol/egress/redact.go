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
	"github.com/livekit/protocol/livekit"
	"github.com/livekit/protocol/utils"
)

func RedactUpload(req UploadRequest) {
	if s3 := req.GetS3(); s3 != nil {
		s3.AccessKey = utils.Redact(s3.AccessKey, "{access_key}")
		s3.Secret = utils.Redact(s3.Secret, "{secret}")
		s3.AssumeRoleExternalId = utils.Redact(s3.AssumeRoleExternalId, "{external_id}")
		s3.SessionToken = utils.Redact(s3.AssumeRoleExternalId, "{session_token}")
		return
	}

	if gcp := req.GetGcp(); gcp != nil {
		gcp.Credentials = utils.Redact(gcp.Credentials, "{credentials}")
		return
	}

	if azure := req.GetAzure(); azure != nil {
		azure.AccountName = utils.Redact(azure.AccountName, "{account_name}")
		azure.AccountKey = utils.Redact(azure.AccountKey, "{account_key}")
		return
	}

	if aliOSS := req.GetAliOSS(); aliOSS != nil {
		aliOSS.AccessKey = utils.Redact(aliOSS.AccessKey, "{access_key}")
		aliOSS.Secret = utils.Redact(aliOSS.Secret, "{secret}")
		return
	}
}

func RedactAutoEncodedOutput(out AutoEncodedOutput) {
	if files := out.GetFileOutputs(); len(files) == 1 {
		RedactUpload(files[0])
	}
	if segments := out.GetSegmentOutputs(); len(segments) == 1 {
		RedactUpload(segments[0])
	}
}

func RedactEncodedOutputs(out EncodedOutput) {
	RedactAutoEncodedOutput(out)

	if streams := out.GetStreamOutputs(); len(streams) == 1 {
		RedactStreamKeys(streams[0])
	}
	if images := out.GetImageOutputs(); len(images) == 1 {
		RedactUpload(images[0])
	}

	if o, ok := out.(EncodedOutputDeprecated); ok {
		if file := o.GetFile(); file != nil {
			RedactUpload(file)
		} else if stream := o.GetStream(); stream != nil {
			RedactStreamKeys(stream)
		} else if segment := o.GetSegments(); segment != nil {
			RedactUpload(segment)
		}
	}
}

func RedactDirectOutputs(out DirectOutput) {
	if f := out.GetFile(); f != nil {
		RedactUpload(f)
	}
}

func RedactStreamKeys(stream *livekit.StreamOutput) {
	for i, url := range stream.Urls {
		if redacted, ok := utils.RedactStreamKey(url); ok {
			stream.Urls[i] = redacted
		}
	}
}
