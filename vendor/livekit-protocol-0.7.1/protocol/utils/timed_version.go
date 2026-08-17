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

package utils

import (
	"database/sql/driver"
	"encoding/binary"
	"errors"
	"fmt"
	"sync"
	"time"

	"github.com/livekit/protocol/livekit"
)

const tickBits uint64 = 13
const tickMask uint64 = (1 << tickBits) - 1

var epoch = time.Date(2000, 0, 0, 0, 0, 0, 0, time.UTC).UnixMicro()

type TimedVersionGenerator interface {
	Next() TimedVersion
}

func timedVersionComponents(v TimedVersion) (ts int64, ticks int32) {
	return int64(v>>tickBits) + epoch, int32(uint64(v) & tickMask)
}

func timedVersionFromComponents(ts int64, ticks int32) TimedVersion {
	if ts < epoch {
		ts = epoch
	}
	return TimedVersion((uint64(ts-epoch) << tickBits) | uint64(ticks))
}

type timedVersionGenerator struct {
	mu    sync.Mutex
	ts    int64
	ticks uint64
}

func NewDefaultTimedVersionGenerator() TimedVersionGenerator {
	return new(timedVersionGenerator)
}

func (g *timedVersionGenerator) New() *TimedVersion {
	v := g.Next()
	return &v
}

func (g *timedVersionGenerator) Next() TimedVersion {
	ts := time.Now().UnixMicro()

	g.mu.Lock()
	defer g.mu.Unlock()

	for {
		if ts < g.ts {
			ts = g.ts
		}
		if g.ts == ts {
			// if incrementing the ticks would overflow the version sleep for a
			// microsecond then try again.
			if g.ticks == tickMask {
				time.Sleep(time.Microsecond)
				ts = time.Now().UnixMicro()
				continue
			}
			g.ticks++
		} else {
			g.ts = ts
			g.ticks = 0
		}
		return timedVersionFromComponents(g.ts, int32(g.ticks))
	}
}

type TimedVersion uint64

func TimedVersionFromProto(proto *livekit.TimedVersion) TimedVersion {
	return timedVersionFromComponents(proto.GetUnixMicro(), proto.GetTicks())
}

func TimedVersionFromTime(t time.Time) TimedVersion {
	return timedVersionFromComponents(t.UnixMicro(), 0)
}

func (t *TimedVersion) Update(other TimedVersion) bool {
	return t.Upgrade(other)
}

func (t *TimedVersion) Upgrade(other TimedVersion) bool {
	if *t < other {
		*t = other
		return true
	}
	return false
}

func (t *TimedVersion) Downgrade(other TimedVersion) bool {
	if *t > other {
		*t = other
		return true
	}
	return false
}

func (t *TimedVersion) Store(other TimedVersion) {
	*t = other
}

func (t TimedVersion) Load() TimedVersion {
	return t
}

func (t TimedVersion) Next() TimedVersion {
	return t + 1
}

func (t TimedVersion) Prev() TimedVersion {
	if t == 0 {
		return t
	}
	return t - 1
}

func (t TimedVersion) After(other TimedVersion) bool {
	return t > other
}

func (t TimedVersion) Compare(other TimedVersion) int {
	if t < other {
		return -1
	}
	if t == other {
		return 0
	}
	return 1
}

func (t TimedVersion) IsZero() bool {
	return t == 0
}

func (t TimedVersion) ToProto() *livekit.TimedVersion {
	ts, ticks := timedVersionComponents(t)
	return &livekit.TimedVersion{
		UnixMicro: ts,
		Ticks:     ticks,
	}
}

func (t TimedVersion) Time() time.Time {
	ts, _ := timedVersionComponents(t)
	return time.UnixMicro(ts)
}

func (t TimedVersion) String() string {
	ts, ticks := timedVersionComponents(t)
	return fmt.Sprintf("%d.%d", ts, ticks)
}

func (t TimedVersion) Value() (driver.Value, error) {
	if t.IsZero() {
		return nil, nil
	}

	return t.MarshalBinary()
}

func (t *TimedVersion) Scan(src interface{}) (err error) {
	switch b := src.(type) {
	case []byte:
		switch len(b) {
		case 0:
			*t = 0
		case 12:
			t.UnmarshalBinary(b)
		default:
			return errors.New("(*TimedVersion).Scan: unsupported format")
		}
	case nil:
		*t = 0
	default:
		return errors.New("(*TimedVersion).Scan: unsupported data type")
	}
	return nil
}

func (t TimedVersion) MarshalBinary() ([]byte, error) {
	ts, ticks := timedVersionComponents(t)
	b := make([]byte, 0, 12)
	b = binary.BigEndian.AppendUint64(b, uint64(ts))
	b = binary.BigEndian.AppendUint32(b, uint32(ticks))
	return b, nil
}

func (t *TimedVersion) UnmarshalBinary(b []byte) error {
	ts := int64(binary.BigEndian.Uint64(b))
	ticks := int32(binary.BigEndian.Uint32(b[8:]))
	*t = timedVersionFromComponents(ts, ticks)
	return nil
}

func (t TimedVersion) GormDataType() string {
	return "bytes"
}
