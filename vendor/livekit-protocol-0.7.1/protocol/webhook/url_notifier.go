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

package webhook

import (
	"bytes"
	"context"
	"crypto/sha256"
	"encoding/base64"
	"fmt"
	"net"
	"net/http"
	"sync"
	"time"

	"github.com/frostbyte73/core"
	"github.com/hashicorp/go-retryablehttp"
	"go.uber.org/atomic"
	"google.golang.org/protobuf/encoding/protojson"

	"github.com/livekit/protocol/auth"
	"github.com/livekit/protocol/livekit"
	"github.com/livekit/protocol/logger"
)

type URLNotifierConfig struct {
	NumWorkers int `yaml:"num_workers,omitempty"`
	QueueSize  int `yaml:"queue_size,omitempty"`
}

var DefaultURLNotifierConfig = URLNotifierConfig{
	NumWorkers: 10,
	QueueSize:  100,
}

type URLNotifierParams struct {
	HTTPClientParams
	Logger     logger.Logger
	Config     URLNotifierConfig
	URL        string
	APIKey     string
	APISecret  string
	FieldsHook func(whi *livekit.WebhookInfo)
	EventKey   func(event *livekit.WebhookEvent) string
	FilterParams
}

// URLNotifier is a QueuedNotifier that sends a POST request to a Webhook URL.
// It will retry on failure, and will drop events if notification fall too far behind
type URLNotifier struct {
	mu            sync.RWMutex
	params        URLNotifierParams
	client        *retryablehttp.Client
	dropped       atomic.Int32
	pool          core.QueuePool
	processedHook func(ctx context.Context, whi *livekit.WebhookInfo)
	filter        *filter
}

func NewURLNotifier(params URLNotifierParams) *URLNotifier {
	if params.Config.NumWorkers == 0 {
		params.Config.NumWorkers = DefaultURLNotifierConfig.NumWorkers
	}
	if params.Config.QueueSize == 0 {
		params.Config.QueueSize = DefaultURLNotifierConfig.QueueSize
	}
	if params.Logger == nil {
		params.Logger = logger.GetLogger()
	}

	rhc := retryablehttp.NewClient()
	if params.RetryWaitMin > 0 {
		rhc.RetryWaitMin = params.RetryWaitMin
	}
	if params.RetryWaitMax > 0 {
		rhc.RetryWaitMax = params.RetryWaitMax
	}
	if params.MaxRetries > 0 {
		rhc.RetryMax = params.MaxRetries
	}
	if params.ClientTimeout > 0 {
		rhc.HTTPClient.Timeout = params.ClientTimeout
	}
	if params.ForceIPv4 {
		var tr *http.Transport
		if existing, ok := rhc.HTTPClient.Transport.(*http.Transport); ok && existing != nil {
			tr = existing.Clone()
		} else {
			tr = http.DefaultTransport.(*http.Transport).Clone()
		}
		tr.DialContext = func(ctx context.Context, network, addr string) (net.Conn, error) {
			return (&net.Dialer{}).DialContext(ctx, "tcp4", addr)
		}
		rhc.HTTPClient.Transport = tr
	}
	n := &URLNotifier{
		params: params,
		client: rhc,
		filter: newFilter(params.FilterParams),
	}
	n.client.Logger = &logAdapter{}

	n.pool = core.NewQueuePool(params.Config.NumWorkers, core.QueueWorkerParams{
		QueueSize:    params.Config.QueueSize,
		DropWhenFull: true,
	})
	return n
}

func (n *URLNotifier) SetKeys(apiKey, apiSecret string) {
	n.mu.Lock()
	defer n.mu.Unlock()
	n.params.APIKey = apiKey
	n.params.APISecret = apiSecret
}

func (n *URLNotifier) SetFilter(params FilterParams) {
	n.mu.Lock()
	defer n.mu.Unlock()
	n.filter.SetFilter(params)
}

func (n *URLNotifier) IsAllowed(event string) bool {
	return n.filter.IsAllowed(event)
}

func (n *URLNotifier) RegisterProcessedHook(hook func(ctx context.Context, whi *livekit.WebhookInfo)) {
	n.mu.Lock()
	defer n.mu.Unlock()
	n.processedHook = hook
}

func (n *URLNotifier) getProcessedHook() func(ctx context.Context, whi *livekit.WebhookInfo) {
	n.mu.RLock()
	defer n.mu.RUnlock()
	return n.processedHook
}

func (n *URLNotifier) QueueNotify(ctx context.Context, event *livekit.WebhookEvent, opts ...NotifyOption) error {
	if !n.filter.IsAllowed(event.Event) {
		return nil
	}

	enqueuedAt := time.Now()

	var key string
	if n.params.EventKey != nil {
		key = n.params.EventKey(event)
	} else {
		key = EventKey(event)
	}

	p := &NotifyParams{}
	for _, o := range opts {
		o(p)
	}

	n.mu.RLock()
	params := n.params
	n.mu.RUnlock()

	if len(p.ExtraWebhooks) > 1 {
		return fmt.Errorf("more than 1 extra webhook url unexpected")
	}
	if len(p.ExtraWebhooks) == 1 {
		params.URL = p.ExtraWebhooks[0].Url
		if p.ExtraWebhooks[0].SigningKey != "" {
			params.APIKey = p.ExtraWebhooks[0].SigningKey
		}
	}

	if p.Secret != "" {
		params.APISecret = p.Secret
	}

	if params.APIKey == "" || params.APISecret == "" {
		return errNoKey
	}

	if !n.pool.Submit(key, func() {
		fields := logFields(event, params.URL)

		queueDuration := time.Since(enqueuedAt)
		fields = append(fields, "queueDuration", queueDuration)

		sendStart := time.Now()
		err := n.send(event, &params)
		sendDuration := time.Since(sendStart)
		fields = append(fields, "sendDuration", sendDuration)
		if err != nil {
			params.Logger.Warnw("failed to send webhook", err, fields...)
			n.dropped.Add(event.NumDropped + 1)
			IncDispatchFailure()
		} else {
			params.Logger.Infow("sent webhook", fields...)
			IncDispatchSuccess()
		}
		if ph := n.getProcessedHook(); ph != nil {
			whi := webhookInfo(
				event,
				enqueuedAt,
				queueDuration,
				sendStart,
				sendDuration,
				params.URL,
				false,
				err,
			)
			if params.FieldsHook != nil {
				params.FieldsHook(whi)
			}
			ph(ctx, whi)
		}
	}) {
		n.dropped.Inc()

		fields := logFields(event, params.URL)
		params.Logger.Infow("dropped webhook", fields...)
		IncDispatchDrop("overflow")

		if ph := n.getProcessedHook(); ph != nil {
			whi := webhookInfo(
				event,
				time.Time{},
				0,
				time.Time{},
				0,
				params.URL,
				true,
				nil,
			)
			if params.FieldsHook != nil {
				params.FieldsHook(whi)
			}
			ph(ctx, whi)
		}
	}
	return nil
}

func (n *URLNotifier) Stop(force bool) {
	if force {
		n.pool.Kill()
	} else {
		n.pool.Drain()
	}
}

func (n *URLNotifier) send(event *livekit.WebhookEvent, params *URLNotifierParams) error {
	// set dropped count
	event.NumDropped = n.dropped.Swap(0)
	encoded, err := protojson.Marshal(event)
	if err != nil {
		return err
	}
	// sign payload
	sum := sha256.Sum256(encoded)
	b64 := base64.StdEncoding.EncodeToString(sum[:])

	at := auth.NewAccessToken(params.APIKey, params.APISecret).
		SetValidFor(5 * time.Minute).
		SetSha256(b64)
	token, err := at.ToJWT()
	if err != nil {
		return err
	}
	r, err := retryablehttp.NewRequest("POST", params.URL, bytes.NewReader(encoded))
	if err != nil {
		// ignore and continue
		return err
	}
	r.Header.Set(authHeader, token)
	// use a custom mime type to ensure signature is checked prior to parsing
	r.Header.Set("content-type", "application/webhook+json")
	res, err := n.client.Do(r)
	if err != nil {
		return err
	}
	_ = res.Body.Close()
	return nil
}
