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
	"fmt"
	"os"

	"github.com/fsnotify/fsnotify"
	"go.uber.org/atomic"
	"gopkg.in/yaml.v3"

	"github.com/livekit/protocol/logger"
	"github.com/livekit/protocol/utils/events"
)

type ConfigBuilder[T any] interface {
	New() (*T, error)
}

type ConfigDefaulter[T any] interface {
	InitDefaults(*T) error
}

type ConfigObserver[T any] struct {
	builder   ConfigBuilder[T]
	watcher   *fsnotify.Watcher
	observers *events.ObserverList[*T]
	conf      atomic.Pointer[T]
}

func NewConfigObserver[T any](path string, builder ConfigBuilder[T]) (*ConfigObserver[T], *T, error) {
	c := &ConfigObserver[T]{
		builder:   builder,
		observers: events.NewObserverList[*T](events.WithBlocking()),
	}

	conf, err := c.load(path)
	if err != nil {
		return nil, nil, err
	}

	if path != "" {
		c.watcher, err = fsnotify.NewWatcher()
		if err != nil {
			return nil, nil, err
		}
		if err := c.watcher.Add(path); err != nil {
			c.watcher.Close()
			return nil, nil, err
		}
		go c.watch()
	}

	return c, conf, nil
}

func (c *ConfigObserver[T]) Close() {
	if c != nil && c.watcher != nil {
		c.watcher.Close()
	}
}

func (c *ConfigObserver[T]) EmitConfigUpdate(conf *T) {
	c.observers.Emit(conf)
}

func (c *ConfigObserver[T]) Observe(cb func(*T)) func() {
	if c == nil {
		return func() {}
	}
	return c.observers.On(cb)
}

func (c *ConfigObserver[T]) Load() *T {
	return c.conf.Load()
}

func (c *ConfigObserver[T]) watch() {
	for {
		select {
		case event, ok := <-c.watcher.Events:
			if !ok {
				return
			}
			if event.Has(fsnotify.Remove) {
				if err := c.watcher.Add(event.Name); err != nil {
					logger.Errorw("unable to rewatch config file", err, "file", event.Name)
				}
			}
			if event.Has(fsnotify.Write | fsnotify.Remove) {
				if err := c.reload(event.Name); err != nil {
					logger.Errorw("unable to update config file", err, "file", event.Name)
				} else {
					logger.Infow("config file has been updated", "file", event.Name)
				}
			}
		case err, ok := <-c.watcher.Errors:
			if !ok {
				return
			}
			logger.Errorw("config file watcher error", err)
		}
	}
}

func (c *ConfigObserver[T]) reload(path string) error {
	conf, err := c.load(path)
	if err != nil {
		return err
	}

	c.EmitConfigUpdate(conf)
	return nil
}

func (c *ConfigObserver[T]) load(path string) (*T, error) {
	conf, err := c.builder.New()
	if err != nil {
		return nil, err
	}

	if path != "" {
		b, err := os.ReadFile(path)
		if err != nil {
			return nil, err
		}

		if len(b) == 0 {
			return nil, fmt.Errorf("cannot parse config: file empty")
		}

		if err := yaml.Unmarshal(b, conf); err != nil {
			return nil, fmt.Errorf("cannot parse config: %v", err)
		}
	}

	if d, ok := c.builder.(ConfigDefaulter[T]); ok {
		d.InitDefaults(conf)
	}

	c.conf.Store(conf)

	return conf, err
}
