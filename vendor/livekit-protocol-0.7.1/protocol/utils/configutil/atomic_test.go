package configutil

import (
	"testing"

	"github.com/livekit/protocol/utils/events"
)

type testConfig struct {
	events.ObserverList[*testConfig]
}

func (c *testConfig) Observe(cb func(*testConfig)) func() {
	return c.On(cb)
}

func (c *testConfig) Load() *testConfig {
	return c
}

func TestInterfaceAtomicValue(t *testing.T) {
	config := &testConfig{}
	_ = NewAtomicValue(config, func(*testConfig) any {
		return 1
	})
	config.Emit(&testConfig{})
}
