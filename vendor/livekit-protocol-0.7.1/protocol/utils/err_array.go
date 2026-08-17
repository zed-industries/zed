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
	"errors"
	"strings"

	"github.com/livekit/psrpc"
)

type ErrArray struct {
	errs []error
}

func (e *ErrArray) AppendErr(err error) {
	e.errs = append(e.errs, err)
}

func (e *ErrArray) ToError() psrpc.Error {
	if len(e.errs) == 0 {
		return nil
	}

	code := psrpc.Unknown
	var errStr []string

	// Return the code for the first error of type psrpc.Error
	for _, err := range e.errs {
		var psrpcErr psrpc.Error

		if code == psrpc.Unknown && errors.As(err, &psrpcErr) {
			code = psrpcErr.Code()
		}

		errStr = append(errStr, err.Error())
	}

	return psrpc.NewErrorf(code, "%s", strings.Join(errStr, "\n"))
}
