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

package redis

import (
	"context"
	"crypto/tls"
	"time"

	"github.com/pkg/errors"
	"github.com/redis/go-redis/v9"

	"github.com/livekit/protocol/xtls"

	"github.com/livekit/protocol/logger"
)

var ErrNotConfigured = errors.New("Redis is not configured")

type RedisConfig struct {
	Address  string `yaml:"address,omitempty"`
	Username string `yaml:"username,omitempty"`
	Password string `yaml:"password,omitempty"`
	DB       int    `yaml:"db,omitempty"`
	// Deprecated: use TLS instead of UseTLS
	UseTLS            bool         `yaml:"use_tls,omitempty"`
	TLS               *xtls.Config `yaml:"tls,omitempty"`
	MasterName        string       `yaml:"sentinel_master_name,omitempty"`
	SentinelUsername  string       `yaml:"sentinel_username,omitempty"`
	SentinelPassword  string       `yaml:"sentinel_password,omitempty"`
	SentinelAddresses []string     `yaml:"sentinel_addresses,omitempty"`
	ClusterAddresses  []string     `yaml:"cluster_addresses,omitempty"`
	DialTimeout       int          `yaml:"dial_timeout,omitempty"`
	ReadTimeout       int          `yaml:"read_timeout,omitempty"`
	WriteTimeout      int          `yaml:"write_timeout,omitempty"`
	// for clustererd mode only, number of redirects to follow, defaults to 2
	MaxRedirects *int          `yaml:"max_redirects,omitempty"`
	PoolTimeout  time.Duration `yaml:"pool_timeout,omitempty"`
	PoolSize     int           `yaml:"pool_size,omitempty"`
}

func (r *RedisConfig) IsConfigured() bool {
	if r.Address != "" {
		return true
	}
	if len(r.SentinelAddresses) > 0 {
		return true
	}
	if len(r.ClusterAddresses) > 0 {
		return true
	}
	return false
}

func (r *RedisConfig) GetMaxRedirects() int {
	if r.MaxRedirects != nil {
		return *r.MaxRedirects
	}
	return 2
}

func GetRedisClient(conf *RedisConfig) (redis.UniversalClient, error) {
	if conf == nil {
		return nil, nil
	}

	if !conf.IsConfigured() {
		return nil, ErrNotConfigured
	}

	var rcOptions *redis.UniversalOptions
	var rc redis.UniversalClient
	var tlsConfig *tls.Config

	if conf.TLS != nil && conf.TLS.Enabled {
		var err error
		tlsConfig, err = conf.TLS.ClientTLSConfig()
		if err != nil {
			return nil, err
		}
	} else if conf.UseTLS {
		tlsConfig = &tls.Config{
			MinVersion: tls.VersionTLS12,
		}
	}

	if len(conf.SentinelAddresses) > 0 {
		logger.Infow("connecting to redis", "sentinel", true, "addr", conf.SentinelAddresses, "masterName", conf.MasterName)

		// By default DialTimeout set to 2s
		if conf.DialTimeout == 0 {
			conf.DialTimeout = 2000
		}
		// By default ReadTimeout set to 0.2s
		if conf.ReadTimeout == 0 {
			conf.ReadTimeout = 200
		}
		// By default WriteTimeout set to 0.2s
		if conf.WriteTimeout == 0 {
			conf.WriteTimeout = 200
		}

		rcOptions = &redis.UniversalOptions{
			Addrs:            conf.SentinelAddresses,
			SentinelUsername: conf.SentinelUsername,
			SentinelPassword: conf.SentinelPassword,
			MasterName:       conf.MasterName,
			Username:         conf.Username,
			Password:         conf.Password,
			DB:               conf.DB,
			TLSConfig:        tlsConfig,
			DialTimeout:      time.Duration(conf.DialTimeout) * time.Millisecond,
			ReadTimeout:      time.Duration(conf.ReadTimeout) * time.Millisecond,
			WriteTimeout:     time.Duration(conf.WriteTimeout) * time.Millisecond,
			PoolTimeout:      conf.PoolTimeout,
			PoolSize:         conf.PoolSize,
		}
	} else if len(conf.ClusterAddresses) > 0 {
		logger.Infow("connecting to redis", "cluster", true, "addr", conf.ClusterAddresses)
		rcOptions = &redis.UniversalOptions{
			Addrs:         conf.ClusterAddresses,
			Username:      conf.Username,
			Password:      conf.Password,
			DB:            conf.DB,
			TLSConfig:     tlsConfig,
			MaxRedirects:  conf.GetMaxRedirects(),
			PoolTimeout:   conf.PoolTimeout,
			PoolSize:      conf.PoolSize,
			IsClusterMode: true,
		}
	} else {
		logger.Infow("connecting to redis", "simple", true, "addr", conf.Address)
		rcOptions = &redis.UniversalOptions{
			Addrs:       []string{conf.Address},
			Username:    conf.Username,
			Password:    conf.Password,
			DB:          conf.DB,
			TLSConfig:   tlsConfig,
			PoolTimeout: conf.PoolTimeout,
			PoolSize:    conf.PoolSize,
		}
	}
	rc = redis.NewUniversalClient(rcOptions)

	if err := rc.Ping(context.Background()).Err(); err != nil {
		err = errors.Wrap(err, "unable to connect to redis")
		return nil, err
	}

	return rc, nil
}
