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

package pprof

import (
	"bytes"
	"context"
	"runtime/pprof"
	"time"

	"github.com/livekit/psrpc"
)

const (
	cpuProfileName = "cpu"
	defaultTimeout = 30
)

var (
	ErrProfileNotFound = psrpc.NewErrorf(psrpc.NotFound, "profile not found")
)

func GetProfileData(ctx context.Context, profileName string, timeout int, debug int) (b []byte, err error) {
	switch profileName {
	case cpuProfileName:
		return GetCpuProfileData(ctx, timeout)
	default:
		return GetGenericProfileData(profileName, debug)
	}
}

func GetCpuProfileData(ctx context.Context, timeout int) (b []byte, err error) {
	if timeout == 0 {
		timeout = defaultTimeout
	}

	buf := &bytes.Buffer{}
	err = pprof.StartCPUProfile(buf)
	if err != nil {
		return nil, err
	}

	select {
	case <-ctx.Done():
		// finish async in order not to block, since we will not use the results
		go pprof.StopCPUProfile()
		return nil, context.Canceled
	case <-time.After(time.Duration(timeout) * time.Second):
		// break
	}

	pprof.StopCPUProfile()

	return buf.Bytes(), nil
}

func GetGenericProfileData(profileName string, debug int) (b []byte, err error) {
	pp := pprof.Lookup(profileName)
	if pp == nil {
		return nil, ErrProfileNotFound
	}

	buf := &bytes.Buffer{}

	err = pp.WriteTo(buf, debug)
	if err != nil {
		return nil, err
	}

	return buf.Bytes(), nil
}
