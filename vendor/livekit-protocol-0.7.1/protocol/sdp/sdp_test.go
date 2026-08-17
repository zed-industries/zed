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

package sdp

import (
	"testing"

	pionsdp "github.com/pion/sdp/v3"
	"github.com/pion/webrtc/v4"
	"github.com/stretchr/testify/require"
)

func TestSDPUtilFunctions(t *testing.T) {
	sdp := "v=0\r\no=- 4648475892259889561 3 IN IP4 127.0.0.1\r\ns=-\r\nt=0 0\r\na=group:BUNDLE 0 1\r\na=ice-ufrag:1hhfzwf0ijpzm\r\na=ice-pwd:jm5puo2ab1op3vs59ca53bdk7s\r\na=fingerprint:sha-256 40:42:FB:47:87:52:BF:CB:EC:3A:DF:EB:06:DA:2D:B7:2F:59:42:10:23:7B:9D:4C:C9:58:DD:FF:A2:8F:17:67\r\nm=video 9 UDP/TLS/RTP/SAVPF 96\r\nc=IN IP4 0.0.0.0\r\na=rtcp:9 IN IP4 0.0.0.0\r\na=setup:passive\r\na=mid:0\r\na=sendonly\r\na=rtcp-mux\r\na=rtpmap:96 H264/90000\r\na=rtcp-fb:96 nack\r\na=rtcp-fb:96 goog-remb\r\na=fmtp:96 packetization-mode=1;profile-level-id=42e01f\r\na=ssrc:1505338584 cname:10000000b5810aac\r\nm=audio 9 UDP/TLS/RTP/SAVPF 111\r\nc=IN IP4 0.0.0.0\r\na=rtcp:9 IN IP4 0.0.0.0\r\na=setup:passive\r\na=mid:1\r\na=sendonly\r\na=rtcp-mux\r\na=rtpmap:111 opus/48000/2\r\na=ssrc:697641945 cname:10000000b5810aac\r\n"
	sd := webrtc.SessionDescription{
		Type: webrtc.SDPTypeOffer,
		SDP:  sdp,
	}
	parsed, err := sd.Unmarshal()
	require.NoError(t, err)

	require.Equal(t, "0", GetMidValue(parsed.MediaDescriptions[0]))
	require.Equal(t, "1", GetMidValue(parsed.MediaDescriptions[1]))

	fp, alg, err := ExtractFingerprint(parsed)
	require.NoError(t, err)
	require.Equal(t, "40:42:FB:47:87:52:BF:CB:EC:3A:DF:EB:06:DA:2D:B7:2F:59:42:10:23:7B:9D:4C:C9:58:DD:FF:A2:8F:17:67", fp)
	require.Equal(t, "sha-256", alg)

	dtlsRole := ExtractDTLSRole(parsed)
	require.Equal(t, webrtc.DTLSRoleServer, dtlsRole)

	ufrag, pwd, err := ExtractICECredential(parsed)
	require.NoError(t, err)
	require.Equal(t, "1hhfzwf0ijpzm", ufrag)
	require.Equal(t, "jm5puo2ab1op3vs59ca53bdk7s", pwd)

	streamID, found := ExtractStreamID(parsed.MediaDescriptions[0])
	require.False(t, found)
	require.Empty(t, streamID)

	_, ok := GetSimulcastRids(parsed.MediaDescriptions[1])
	require.False(t, ok)

	codecs, err := CodecsFromMediaDescription(parsed.MediaDescriptions[0])
	require.NoError(t, err)
	expectedCodec := pionsdp.Codec{
		PayloadType:  96,
		Name:         "H264",
		ClockRate:    90000,
		Fmtp:         "packetization-mode=1;profile-level-id=42e01f",
		RTCPFeedback: []string{"nack", "goog-remb"},
	}
	require.Equal(t, expectedCodec, codecs[0])

	bundleMid, found := GetBundleMid(parsed)
	require.True(t, found)
	require.Equal(t, "0", bundleMid)
}

func TestSDPFragment(t *testing.T) {
	fragment := "a=ice-lite\r\na=ice-options:trickle ice2\r\na=group:BUNDLE 0 1\r\nm=audio 9 UDP/TLS/RTP/SAVPF 111\r\na=mid:0\r\na=ice-ufrag:ysXw\r\na=ice-pwd:vw5LmwG4y/e6dPP/zAP9Gp5k\r\na=candidate:1387637174 1 udp 2122260223 192.0.2.1 61764 typ host generation 0 ufrag EsAw network-id 1\r\na=candidate:3471623853 1 udp 2122194687 198.51.100.2 61765 typ host generation 0 ufrag EsAw network-id 2\r\na=candidate:473322822 1 tcp 1518280447 192.0.2.1 9 typ host tcptype active generation 0 ufrag EsAw network-id 1\r\na=candidate:2154773085 1 tcp 1518214911 198.51.100.2 9 typ host tcptype active generation 0 ufrag EsAw network-id 2\r\na=candidate:393455558 0 tcp 1518283007 [2401:4900:633c:959f:2037:680c:7c40:b3db] 9 typ host tcptype active\r\n"

	lite := true
	expectedSDPFragment := SDPFragment{
		group: "BUNDLE 0 1",
		ice: &sdpFragmentICE{
			options: "trickle ice2",
			lite:    &lite,
		},
		media: &sdpFragmentMedia{
			info: "audio 9 UDP/TLS/RTP/SAVPF 111",
			mid:  "0",
			ice: &sdpFragmentICE{
				ufrag: "ysXw",
				pwd:   "vw5LmwG4y/e6dPP/zAP9Gp5k",
			},
			candidates: []string{
				"1387637174 1 udp 2122260223 192.0.2.1 61764 typ host generation 0 ufrag EsAw network-id 1",
				"3471623853 1 udp 2122194687 198.51.100.2 61765 typ host generation 0 ufrag EsAw network-id 2",
				"473322822 1 tcp 1518280447 192.0.2.1 9 typ host tcptype active generation 0 ufrag EsAw network-id 1",
				"2154773085 1 tcp 1518214911 198.51.100.2 9 typ host tcptype active generation 0 ufrag EsAw network-id 2",
				"393455558 0 tcp 1518283007 [2401:4900:633c:959f:2037:680c:7c40:b3db] 9 typ host tcptype active",
			},
		},
	}

	sdpFragment := &SDPFragment{}
	err := sdpFragment.Unmarshal(fragment)
	require.NoError(t, err)
	require.Equal(t, expectedSDPFragment, *sdpFragment)

	expectedMarshalled := "a=group:BUNDLE 0 1\r\na=ice-lite\r\na=ice-options:trickle ice2\r\nm=audio 9 UDP/TLS/RTP/SAVPF 111\r\na=mid:0\r\na=ice-ufrag:ysXw\r\na=ice-pwd:vw5LmwG4y/e6dPP/zAP9Gp5k\r\na=candidate:1387637174 1 udp 2122260223 192.0.2.1 61764 typ host generation 0 ufrag EsAw network-id 1\r\na=candidate:3471623853 1 udp 2122194687 198.51.100.2 61765 typ host generation 0 ufrag EsAw network-id 2\r\na=candidate:473322822 1 tcp 1518280447 192.0.2.1 9 typ host tcptype active generation 0 ufrag EsAw network-id 1\r\na=candidate:2154773085 1 tcp 1518214911 198.51.100.2 9 typ host tcptype active generation 0 ufrag EsAw network-id 2\r\na=candidate:393455558 0 tcp 1518283007 [2401:4900:633c:959f:2037:680c:7c40:b3db] 9 typ host tcptype active\r\n"
	marshalled, err := sdpFragment.Marshal()
	require.NoError(t, err)
	require.Equal(t, expectedMarshalled, marshalled)

	expectedCandidates := []string{
		"1387637174 1 udp 2122260223 192.0.2.1 61764 typ host generation 0 ufrag EsAw network-id 1",
		"3471623853 1 udp 2122194687 198.51.100.2 61765 typ host generation 0 ufrag EsAw network-id 2",
		"473322822 1 tcp 1518280447 192.0.2.1 9 typ host tcptype active generation 0 ufrag EsAw network-id 1",
		"2154773085 1 tcp 1518214911 198.51.100.2 9 typ host tcptype active generation 0 ufrag EsAw network-id 2",
		"393455558 0 tcp 1518283007 [2401:4900:633c:959f:2037:680c:7c40:b3db] 9 typ host tcptype active",
	}

	sdpFragment1 := &SDPFragment{}
	err = sdpFragment1.Unmarshal(marshalled)
	require.NoError(t, err)
	require.Equal(t, expectedSDPFragment, *sdpFragment1)
	require.Equal(t, "0", sdpFragment1.Mid())
	require.Equal(t, expectedCandidates, sdpFragment1.Candidates())
	ufrag, pwd, err := sdpFragment1.ExtractICECredential()
	require.NoError(t, err)
	require.Equal(t, "ysXw", ufrag)
	require.Equal(t, "vw5LmwG4y/e6dPP/zAP9Gp5k", pwd)

	mismatchedMidFragment := "a=ice-options:trickle ice2\r\na=group:BUNDLE 0 1\r\nm=audio 9 UDP/TLS/RTP/SAVPF 111\r\na=mid:1\r\na=ice-ufrag:ysXw\r\na=ice-pwd:vw5LmwG4y/e6dPP/zAP9Gp5k\r\na=candidate:1387637174 1 udp 2122260223 192.0.2.1 61764 typ host generation 0 ufrag EsAw network-id 1\r\na=candidate:3471623853 1 udp 2122194687 198.51.100.2 61765 typ host generation 0 ufrag EsAw network-id 2\r\na=candidate:473322822 1 tcp 1518280447 192.0.2.1 9 typ host tcptype active generation 0 ufrag EsAw network-id 1\r\na=candidate:2154773085 1 tcp 1518214911 198.51.100.2 9 typ host tcptype active generation 0 ufrag EsAw network-id 2\r\n"
	sdpFragment2 := &SDPFragment{}
	err = sdpFragment2.Unmarshal(mismatchedMidFragment)
	require.Error(t, err)

	sdp := "v=0\r\no=- 4648475892259889561 3 IN IP4 127.0.0.1\r\ns=-\r\nt=0 0\r\na=group:BUNDLE 0 1\r\na=ice-options:trickle ice2\r\na=fingerprint:sha-256 40:42:FB:47:87:52:BF:CB:EC:3A:DF:EB:06:DA:2D:B7:2F:59:42:10:23:7B:9D:4C:C9:58:DD:FF:A2:8F:17:67\r\nm=video 9 UDP/TLS/RTP/SAVPF 96\r\nc=IN IP4 0.0.0.0\r\na=ice-ufrag:1hhfzwf0ijpzm\r\na=ice-pwd:jm5puo2ab1op3vs59ca53bdk7s\r\na=rtcp:9 IN IP4 0.0.0.0\r\na=setup:passive\r\na=mid:0\r\na=sendonly\r\na=rtcp-mux\r\na=rtpmap:96 H264/90000\r\na=rtcp-fb:96 nack\r\na=rtcp-fb:96 goog-remb\r\na=fmtp:96 packetization-mode=1;profile-level-id=42e01f\r\na=ssrc:1505338584 cname:10000000b5810aac\r\nm=audio 9 UDP/TLS/RTP/SAVPF 111\r\nc=IN IP4 0.0.0.0\r\na=rtcp:9 IN IP4 0.0.0.0\r\na=setup:passive\r\na=mid:1\r\na=sendonly\r\na=rtcp-mux\r\na=rtpmap:111 opus/48000/2\r\na=ssrc:697641945 cname:10000000b5810aac\r\n"
	sd := webrtc.SessionDescription{
		Type: webrtc.SDPTypeOffer,
		SDP:  sdp,
	}
	parsed, _ := sd.Unmarshal()
	err = sdpFragment1.PatchICECredentialAndCandidatesIntoSDP(parsed)
	require.NoError(t, err)

	ufrag, pwd, err = ExtractICECredential(parsed)
	require.NoError(t, err)
	require.Equal(t, "ysXw", ufrag)
	require.Equal(t, "vw5LmwG4y/e6dPP/zAP9Gp5k", pwd)

	candidates := []string{}
	for _, a := range parsed.MediaDescriptions[0].Attributes {
		if a.IsICECandidate() {
			candidates = append(candidates, a.Value)
		}
	}
	require.Equal(t, expectedCandidates, candidates)

	sdpFragment3, err := ExtractSDPFragment(parsed)
	require.NoError(t, err)

	expectedSDPFragment3 := SDPFragment{
		group: "BUNDLE 0 1",
		ice: &sdpFragmentICE{
			options: "trickle ice2",
		},
		media: &sdpFragmentMedia{
			info: "video 9 UDP/TLS/RTP/SAVPF 96",
			mid:  "0",
			ice: &sdpFragmentICE{
				ufrag: "ysXw",
				pwd:   "vw5LmwG4y/e6dPP/zAP9Gp5k",
			},
			candidates: []string{
				"1387637174 1 udp 2122260223 192.0.2.1 61764 typ host generation 0 ufrag EsAw network-id 1",
				"3471623853 1 udp 2122194687 198.51.100.2 61765 typ host generation 0 ufrag EsAw network-id 2",
				"473322822 1 tcp 1518280447 192.0.2.1 9 typ host tcptype active generation 0 ufrag EsAw network-id 1",
				"2154773085 1 tcp 1518214911 198.51.100.2 9 typ host tcptype active generation 0 ufrag EsAw network-id 2",
				"393455558 0 tcp 1518283007 [2401:4900:633c:959f:2037:680c:7c40:b3db] 9 typ host tcptype active",
			},
		},
	}
	require.Equal(t, expectedSDPFragment3, *sdpFragment3)

	marshalledSDPFragment3, err := sdpFragment3.Marshal()
	require.NoError(t, err)
	expectedMarshalledSDPFragment3 := "a=group:BUNDLE 0 1\r\na=ice-options:trickle ice2\r\nm=video 9 UDP/TLS/RTP/SAVPF 96\r\na=mid:0\r\na=ice-ufrag:ysXw\r\na=ice-pwd:vw5LmwG4y/e6dPP/zAP9Gp5k\r\na=candidate:1387637174 1 udp 2122260223 192.0.2.1 61764 typ host generation 0 ufrag EsAw network-id 1\r\na=candidate:3471623853 1 udp 2122194687 198.51.100.2 61765 typ host generation 0 ufrag EsAw network-id 2\r\na=candidate:473322822 1 tcp 1518280447 192.0.2.1 9 typ host tcptype active generation 0 ufrag EsAw network-id 1\r\na=candidate:2154773085 1 tcp 1518214911 198.51.100.2 9 typ host tcptype active generation 0 ufrag EsAw network-id 2\r\na=candidate:393455558 0 tcp 1518283007 [2401:4900:633c:959f:2037:680c:7c40:b3db] 9 typ host tcptype active\r\n"
	require.Equal(t, expectedMarshalledSDPFragment3, marshalledSDPFragment3)
}
