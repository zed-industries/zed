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

package ingress

import (
	"fmt"

	"github.com/livekit/psrpc"
)

var (
	ErrIngressOutOfDate        = psrpc.NewErrorf(psrpc.FailedPrecondition, "trying to ovewrite an ingress with an older version")
	ErrIngressTimedOut         = psrpc.NewErrorf(psrpc.DeadlineExceeded, "ingress timed out")
	ErrNoResponse              = psrpc.NewErrorf(psrpc.Unavailable, "no response from ingress service")
	ErrInvalidOutputDimensions = NewInvalidVideoParamsError("invalid output media dimensions")
	ErrInvalidIngressType      = psrpc.NewErrorf(psrpc.InvalidArgument, "invalid ingress type")
	ErrIngressDisabled         = psrpc.NewErrorf(psrpc.PermissionDenied, "session is disabled")
)

func ErrInvalidIngress(s string) psrpc.Error {
	return psrpc.NewErrorf(psrpc.InvalidArgument, "invalid ingress: %s", s)
}

func NewInvalidVideoParamsError(s string) error {
	return psrpc.NewError(psrpc.InvalidArgument, fmt.Errorf("invalid video parameters: %s", s))
}

func NewInvalidAudioParamsError(s string) error {
	return psrpc.NewError(psrpc.InvalidArgument, fmt.Errorf("invalid audio parameters: %s", s))
}

func NewInvalidTranscodingBypassError(s string) error {
	return psrpc.NewError(psrpc.InvalidArgument, fmt.Errorf("invalid transcoding bypass configuration: %s", s))
}
