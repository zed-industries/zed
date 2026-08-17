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

package guid

import (
	"crypto/rand"
	"crypto/sha1"
	"crypto/sha256"
	"fmt"
	mrand "math/rand/v2"
	"os"
	"regexp"
	"sync"
	"unsafe"

	"github.com/jxskiss/base62"
	"github.com/lithammer/shortuuid/v4"

	"github.com/livekit/protocol/livekit"
	"github.com/livekit/protocol/utils/must"
)

const Size = 12

const (
	RoomPrefix                = "RM_"
	NodePrefix                = "ND_"
	ParticipantPrefix         = "PA_"
	TrackPrefix               = "TR_"
	DataTrackPrefix           = "DTR_"
	APIKeyPrefix              = "API"
	EgressPrefix              = "EG_"
	IngressPrefix             = "IN_"
	SIPTrunkPrefix            = "ST_"
	SIPDispatchRulePrefix     = "SDR_"
	SIPCallPrefix             = "SCL_"
	SIPTransferPrefix         = "STR_"
	RPCPrefix                 = "RPC_"
	WHIPResourcePrefix        = "WH_"
	RTMPResourcePrefix        = "RT_"
	URLResourcePrefix         = "UR_"
	SIPHostnamePrefix         = "SH"
	AgentPrefix               = "A_"
	AgentWorkerPrefix         = "AW_"
	AgentJobPrefix            = "AJ_"
	AgentDispatchPrefix       = "AD_"
	AgentBuilderPrefix        = "AB_"
	AgentBuilderVersionPrefix = "ABV_"
	CloudAgentPrefix          = "CA_"
	CloudAgentRegionPrefix    = "CAR_"
	CloudAgentVersionPrefix   = "CAV_"
	CloudAgentSecretPrefix    = "CAS_"
	CloudAgentWorkerPrefix    = "CAW_"
	AgentGatewayPrefix        = "GW_"
	CarrierPrefix             = "CR_"
	PhoneNumberPrefix         = "PN_"
)

var guidGeneratorPool = sync.Pool{
	New: func() any {
		return must.Get(newGenerator())
	},
}

func New(prefix string) string {
	g := guidGeneratorPool.Get().(*guidGenerator)
	defer guidGeneratorPool.Put(g)
	return g.New(prefix)
}

func Hash(prefix string, data []byte) string {
	g := newGeneratorFromSeed(sha256.Sum256(data))
	return g.New(prefix)
}

// HashedID creates a hashed ID from a unique string
func HashedID(id string) string {
	h := sha1.New()
	h.Write([]byte(id))
	val := h.Sum(nil)

	return base62.EncodeToString(val)
}

func LocalNodeID() (string, error) {
	hostname, err := os.Hostname()
	if err != nil {
		return "", err
	}
	return fmt.Sprintf("%s%s", NodePrefix, HashedID(hostname)[:8]), nil
}

var b57Index = newB57Index()
var b57Chars = []byte(shortuuid.DefaultAlphabet)

func newB57Index() [256]byte {
	var index [256]byte
	for i := 0; i < len(b57Chars); i++ {
		index[b57Chars[i]] = byte(i)
	}
	return index
}

type guidGenerator struct {
	rng *mrand.ChaCha8
}

func newGenerator() (*guidGenerator, error) {
	var seed [32]byte
	if _, err := rand.Read(seed[:]); err != nil {
		return nil, err
	}

	return newGeneratorFromSeed(seed), nil
}

func newGeneratorFromSeed(seed [32]byte) *guidGenerator {
	return &guidGenerator{
		rng: mrand.NewChaCha8(seed),
	}
}

func (g *guidGenerator) readIDChars(b []byte) {
	var n int
	for {
		r := g.rng.Uint64()
		for i := 0; i < 10; i++ {
			if int(r&0x3f) < len(b57Chars) {
				b[n] = b57Chars[r&0x3f]
				n++
				if n == len(b) {
					return
				}
			}
			r >>= 6
		}
	}
}

func (g *guidGenerator) New(prefix string) string {
	b := make([]byte, len(prefix)+Size)
	copy(b, prefix)
	g.readIDChars(b[len(prefix):])
	return unsafe.String(unsafe.SliceData(b), len(b))
}

func guidPrefix[T livekit.Guid]() string {
	var id T
	switch any(id).(type) {
	case livekit.TrackID:
		return TrackPrefix
	case livekit.ParticipantID:
		return ParticipantPrefix
	case livekit.RoomID:
		return RoomPrefix
	default:
		panic("unreachable")
	}
}

func Marshal[T livekit.Guid](id T) livekit.GuidBlock {
	return livekit.GuidBlock(MarshalAppend(nil, id))
}

func MarshalAppend[T livekit.Guid](b []byte, id T) []byte {
	off := len(b)
	b = append(b, make([]byte, Size*3/4)...)
	idb := []byte(id)[len(id)-Size:]
	for i := 0; i < 3; i++ {
		j := i*3 + off
		k := i * 4
		b[j] = b57Index[idb[k]]<<2 | b57Index[idb[k+1]]>>4
		b[j+1] = b57Index[idb[k+1]]<<4 | b57Index[idb[k+2]]>>2
		b[j+2] = b57Index[idb[k+2]]<<6 | b57Index[idb[k+3]]
	}
	return b
}

func Unmarshal[T livekit.Guid](b livekit.GuidBlock) T {
	prefix := guidPrefix[T]()
	id := make([]byte, len(prefix)+Size)
	copy(id, []byte(prefix))
	idb := id[len(prefix):]
	for i := 0; i < 3; i++ {
		j := i * 3
		k := i * 4
		idb[k] = b57Chars[b[j]>>2]
		idb[k+1] = b57Chars[(b[j]&3)<<4|b[j+1]>>4]
		idb[k+2] = b57Chars[(b[j+1]&15)<<2|b[j+2]>>6]
		idb[k+3] = b57Chars[b[j+2]&63]
	}
	return T(unsafe.String(unsafe.SliceData(id), len(id)))
}

var validIDPattern = regexp.MustCompile(`^([a-zA-Z0-9]{1,16}_){1,2}[a-zA-Z0-9]{0,12}$`)

func IsValidID[T ~string](id T) bool {
	return validIDPattern.MatchString(string(id))
}
