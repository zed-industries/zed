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

package signalling

import (
	"encoding/json"

	"github.com/livekit/protocol/livekit"
	"github.com/pion/webrtc/v4"
)

func ToProtoSessionDescription(sd webrtc.SessionDescription, id uint32, midToTrackID map[string]string) *livekit.SessionDescription {
	if sd.SDP == "" {
		return nil
	}

	return &livekit.SessionDescription{
		Type:         sd.Type.String(),
		Sdp:          sd.SDP,
		Id:           id,
		MidToTrackId: midToTrackID,
	}
}

func FromProtoSessionDescription(sd *livekit.SessionDescription) (webrtc.SessionDescription, uint32, map[string]string) {
	var sdType webrtc.SDPType
	switch sd.Type {
	case webrtc.SDPTypeOffer.String():
		sdType = webrtc.SDPTypeOffer
	case webrtc.SDPTypeAnswer.String():
		sdType = webrtc.SDPTypeAnswer
	case webrtc.SDPTypePranswer.String():
		sdType = webrtc.SDPTypePranswer
	case webrtc.SDPTypeRollback.String():
		sdType = webrtc.SDPTypeRollback
	}
	return webrtc.SessionDescription{
		Type: sdType,
		SDP:  sd.Sdp,
	}, sd.Id, sd.MidToTrackId
}

func ToProtoTrickle(candidateInit webrtc.ICECandidateInit, target livekit.SignalTarget, final bool) *livekit.TrickleRequest {
	data, _ := json.Marshal(candidateInit)
	return &livekit.TrickleRequest{
		CandidateInit: string(data),
		Target:        target,
		Final:         final,
	}
}

func FromProtoTrickle(trickle *livekit.TrickleRequest) (webrtc.ICECandidateInit, error) {
	ci := webrtc.ICECandidateInit{}
	err := json.Unmarshal([]byte(trickle.CandidateInit), &ci)
	if err != nil {
		return webrtc.ICECandidateInit{}, err
	}
	return ci, nil
}

func FromProtoIceServers(iceservers []*livekit.ICEServer) []webrtc.ICEServer {
	if iceservers == nil {
		return nil
	}
	servers := make([]webrtc.ICEServer, 0, len(iceservers))
	for _, server := range iceservers {
		servers = append(servers, webrtc.ICEServer{
			URLs:       server.Urls,
			Username:   server.Username,
			Credential: server.Credential,
		})
	}
	return servers
}
