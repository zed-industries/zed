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
	"testing"

	"github.com/livekit/psrpc"
	"github.com/stretchr/testify/assert"
)

func TestErrArray(t *testing.T) {
	err1 := errors.New("error 1")
	err2 := errors.New("error 2")
	err3 := psrpc.NewErrorf(psrpc.NotFound, "error 3")
	err4 := psrpc.NewErrorf(psrpc.Internal, "error 4")

	errArray := &ErrArray{}
	assert.Nil(t, errArray.ToError())

	errArray.AppendErr(err1)
	assert.Equal(t, psrpc.Unknown, errArray.ToError().Code())
	assert.Equal(t, err1.Error(), errArray.ToError().Error())

	errArray.AppendErr(err2)
	assert.Equal(t, psrpc.Unknown, errArray.ToError().Code())
	assert.Equal(t, 2, len(strings.Split(errArray.ToError().Error(), "\n")))

	errArray.AppendErr(err3)
	assert.Equal(t, psrpc.NotFound, errArray.ToError().Code())
	assert.Equal(t, 3, len(strings.Split(errArray.ToError().Error(), "\n")))

	errArray.AppendErr(err4)
	assert.Equal(t, psrpc.NotFound, errArray.ToError().Code())
	assert.Equal(t, 4, len(strings.Split(errArray.ToError().Error(), "\n")))
}
