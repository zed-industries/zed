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

package rtputil

import "time"

type RTPConverter struct {
	ts  uint64
	rtp uint64
}

func NewRTPConverter(clockRate int64) *RTPConverter {
	ts := time.Second.Nanoseconds()
	for _, i := range []int64{10, 3, 2} {
		for ts%i == 0 && clockRate%i == 0 {
			ts /= i
			clockRate /= i
		}
	}

	return &RTPConverter{ts: uint64(ts), rtp: uint64(clockRate)}
}

func (r *RTPConverter) ToDuration(rtpTicks uint32) time.Duration {
	return time.Duration(uint64(rtpTicks) * r.ts / r.rtp)
}

func (r *RTPConverter) ToDurationExt(rtpExtTicks uint64) time.Duration {
	return time.Duration(uint64(rtpExtTicks) * r.ts / r.rtp)
}

func (r *RTPConverter) ToRTP(duration time.Duration) uint32 {
	return uint32(duration.Nanoseconds() * int64(r.rtp) / int64(r.ts))
}

func (r *RTPConverter) ToRTPExt(duration time.Duration) uint64 {
	return uint64(duration.Nanoseconds() * int64(r.rtp) / int64(r.ts))
}
