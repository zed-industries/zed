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

package livekit

import (
	"testing"

	"github.com/stretchr/testify/require"
	"go.uber.org/zap/zapcore"

	"github.com/livekit/protocol/logger"
	"github.com/livekit/protocol/logger/testutil"
	"github.com/livekit/protocol/logger/zaputil"
)

type TestEgressLogOutput struct {
	testutil.TestLogOutput
	S3 map[string]string
}

func TestLoggerProto(t *testing.T) {
	ws := &testutil.BufferedWriteSyncer{}
	l, err := logger.NewZapLogger(&logger.Config{}, logger.WithTap(zaputil.NewWriteEnabler(ws, zapcore.DebugLevel)))
	require.NoError(t, err)

	s3 := &S3Upload{
		AccessKey:     "Field1",
		Secret:        "Field2",
		AssumeRoleArn: "Field3",
		SessionToken:  "Field4",
		Endpoint:      "Field5",
	}

	l.Debugw("foo", "s3", logger.Proto(s3))

	var log TestEgressLogOutput
	require.NoError(t, ws.Unmarshal(&log))

	require.NotEqual(t, 0, log.TS)
	require.NotEqual(t, "", log.Caller)
	require.Equal(t, "foo", log.Msg)
	require.Equal(t, "<redacted>", log.S3["accessKey"])
	require.Equal(t, "<redacted>", log.S3["secret"])
	require.Equal(t, "<redacted>", log.S3["assumeRoleArn"])
	require.Equal(t, "<redacted>", log.S3["sessionToken"])
	require.Equal(t, "Field5", log.S3["endpoint"])
}
