// Copyright 2024 LiveKit, Inc.
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
	"os"
	"testing"
	"time"

	"github.com/stretchr/testify/require"

	"github.com/livekit/protocol/utils/configutil"
)

const testConfig0 = `foo: a`
const testConfig1 = `foo: b`

type TestConfig struct {
	Foo string `yaml:"foo"`
	Bar string `yaml:"bar"`
}

type testConfigBuilder struct{}

func (testConfigBuilder) New() (*TestConfig, error) {
	return &TestConfig{}, nil
}

func (testConfigBuilder) InitDefaults(c *TestConfig) error {
	c.Bar = "c"
	return nil
}

func TestConfigObserver(t *testing.T) {
	f, err := os.CreateTemp(os.TempDir(), "lk-test-*.yaml")
	t.Cleanup(func() {
		_ = f.Close()
	})
	require.NoError(t, err)
	_, err = f.WriteString(testConfig0)
	require.NoError(t, err)

	obs, conf, err := NewConfigObserver(f.Name(), testConfigBuilder{})
	require.NoError(t, err)

	require.Equal(t, "a", conf.Foo)
	require.Equal(t, "c", conf.Bar)

	atomicFoo := configutil.NewAtomicValue(obs, func(c *TestConfig) string {
		return c.Foo
	})

	require.Equal(t, "a", atomicFoo.Load())

	done := make(chan struct{})
	obs.Observe(func(c *TestConfig) {
		require.Equal(t, "b", c.Foo)
		require.Equal(t, "c", c.Bar)
		close(done)
	})

	_, err = f.WriteAt([]byte(testConfig1), 0)
	require.NoError(t, err)

	select {
	case <-done:
	case <-time.After(100 * time.Millisecond):
		require.FailNow(t, "timed out waiting for config update")
	}

	require.Equal(t, "b", atomicFoo.Load())
}
