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
	"github.com/livekit/protocol/webhook"
)

func GetEgressNotifyOptions(egressInfo *livekit.EgressInfo) []webhook.NotifyOption {
	if egressInfo == nil {
		return nil
	}

	if egressInfo.Request == nil {
		return nil
	}

	var whs []*livekit.WebhookConfig

	switch req := egressInfo.Request.(type) {
	case *livekit.EgressInfo_RoomComposite:
		if req.RoomComposite != nil {
			whs = req.RoomComposite.Webhooks
		}
	case *livekit.EgressInfo_Web:
		if req.Web != nil {
			whs = req.Web.Webhooks
		}
	case *livekit.EgressInfo_Participant:
		if req.Participant != nil {
			whs = req.Participant.Webhooks
		}
	case *livekit.EgressInfo_TrackComposite:
		if req.TrackComposite != nil {
			whs = req.TrackComposite.Webhooks
		}
	case *livekit.EgressInfo_Track:
		if req.Track != nil {
			whs = req.Track.Webhooks
		}
	}

	if len(whs) > 0 {
		return []webhook.NotifyOption{webhook.WithExtraWebhooks(whs)}
	}

	return nil
}
