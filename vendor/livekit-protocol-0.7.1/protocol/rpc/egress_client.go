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

package rpc

import (
	"context"
	"errors"
	"math"
	"math/rand"
	"time"

	"github.com/livekit/protocol/livekit"
	"github.com/livekit/psrpc"
	"github.com/livekit/psrpc/pkg/middleware"
)

const (
	retries     = 3
	backoffBase = 1 * time.Second
)

type EgressClient interface {
	EgressInternalClient
	EgressHandlerClient
}

type egressClient struct {
	EgressInternalClient
	EgressHandlerClient
}

func isErrRecoverable(err error) bool {
	var e psrpc.Error
	if !errors.As(err, &e) {
		return true
	}
	return e.Code() == psrpc.DeadlineExceeded ||
		e.Code() == psrpc.ResourceExhausted ||
		e.Code() == psrpc.Unavailable
}

func NewEgressClient(params ClientParams) (EgressClient, error) {
	if params.Bus == nil {
		return nil, nil
	}

	opts := params.Options()
	baseTimeout := params.Timeout
	if baseTimeout < 10*time.Second {
		baseTimeout = 10 * time.Second
	}

	internalOpts := append(opts,
		psrpc.WithClientChannelSize(1000),
		middleware.WithRPCRetries(middleware.RetryOptions{
			Timeout: baseTimeout,
			GetRetryParameters: func(err error, attempt int) (retry bool, timeout time.Duration, waitTime time.Duration) {
				if !isErrRecoverable(err) {
					return false, 0, 0
				}

				if attempt >= retries {
					return false, 0, 0
				}

				// backoff = base * 2 ^ (attempt - 1) * rand[1,2)
				backoff := time.Duration(float64(backoffBase) * math.Pow(2, float64(attempt-1)) * (rand.Float64() + 1))
				timeout = time.Duration(float64(baseTimeout) * math.Pow(2, float64(attempt)))

				return true, timeout, backoff
			},
		}))

	internalClient, err := NewEgressInternalClient(params.Bus, internalOpts...)
	if err != nil {
		return nil, err
	}

	handlerClient, err := NewEgressHandlerClient(params.Bus, opts...)
	if err != nil {
		return nil, err
	}

	return &egressClient{
		EgressInternalClient: internalClient,
		EgressHandlerClient:  handlerClient,
	}, nil
}

func (c *egressClient) StartEgress(ctx context.Context, topic string, req *StartEgressRequest, opts ...psrpc.RequestOption) (*livekit.EgressInfo, error) {
	o := append([]psrpc.RequestOption{
		psrpc.WithSelectionOpts(psrpc.SelectionOpts{
			MaximumAffinity:     1,
			AffinityTimeout:     time.Second,
			ShortCircuitTimeout: time.Millisecond * 500,
		}),
	}, opts...)
	return c.EgressInternalClient.StartEgress(ctx, topic, req, o...)
}

func (c *egressClient) Close() {
	c.EgressInternalClient.Close()
	c.EgressHandlerClient.Close()
}
