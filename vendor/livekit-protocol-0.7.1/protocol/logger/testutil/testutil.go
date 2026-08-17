package testutil

import (
	"bytes"
	"encoding/json"
)

type TestLogOutput struct {
	Level  string
	TS     float64
	Caller string
	Msg    string
}

type BufferedWriteSyncer struct {
	bytes.Buffer
}

func (t *BufferedWriteSyncer) Unmarshal(v any) error {
	return json.Unmarshal(t.Bytes(), &v)
}

func (t *BufferedWriteSyncer) Sync() error { return nil }
