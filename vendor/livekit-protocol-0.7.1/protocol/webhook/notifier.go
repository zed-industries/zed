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
	"context"
	"fmt"
	"sync"
	"time"

	"github.com/livekit/protocol/auth"
	"github.com/livekit/protocol/livekit"
	"github.com/livekit/protocol/logger"
	"google.golang.org/protobuf/types/known/timestamppb"
)

type WebHookConfig struct {
	URLs                []string                  `yaml:"urls,omitempty"`
	APIKey              string                    `yaml:"api_key,omitempty"`
	URLNotifier         URLNotifierConfig         `yaml:"url_notifier,omitempty"`
	ResourceURLNotifier ResourceURLNotifierConfig `yaml:"resource_url_notifier,omitempty"`
	FilterParams        FilterParams              `yaml:"filter_params,omitempty"`
}

var DefaultWebHookConfig = WebHookConfig{
	URLNotifier:         DefaultURLNotifierConfig,
	ResourceURLNotifier: DefaultResourceURLNotifierConfig,
	FilterParams:        FilterParams{},
}

type NotifyParams struct {
	ExtraWebhooks []*livekit.WebhookConfig
	Secret        string
}

type NotifyOption func(*NotifyParams)

func WithExtraWebhooks(wh []*livekit.WebhookConfig) NotifyOption {
	return func(p *NotifyParams) {
		p.ExtraWebhooks = wh
	}
}

func WithSecret(secret string) NotifyOption {
	return func(p *NotifyParams) {
		p.Secret = secret
	}
}

type QueuedNotifier interface {
	RegisterProcessedHook(f func(ctx context.Context, whi *livekit.WebhookInfo))
	SetKeys(apiKey, apiSecret string)
	SetFilter(params FilterParams)
	QueueNotify(ctx context.Context, event *livekit.WebhookEvent, opts ...NotifyOption) error
	Stop(force bool)
}

type DefaultNotifier struct {
	kp auth.KeyProvider

	notifiers            []QueuedNotifier
	extraWebhookNotifier QueuedNotifier
}

func NewDefaultNotifier(config WebHookConfig, kp auth.KeyProvider) (QueuedNotifier, error) {
	apiSecret := kp.GetSecret(config.APIKey)
	if apiSecret == "" && len(config.URLs) > 0 {
		return nil, fmt.Errorf("unknown api key in webhook config")
	}

	n := &DefaultNotifier{
		kp: kp,
	}
	for _, url := range config.URLs {
		u := NewResourceURLNotifier(ResourceURLNotifierParams{
			URL:          url,
			Logger:       logger.GetLogger().WithComponent("webhook"),
			APIKey:       config.APIKey,
			APISecret:    apiSecret,
			Config:       config.ResourceURLNotifier,
			FilterParams: config.FilterParams,
		})
		n.notifiers = append(n.notifiers, u)
	}

	n.extraWebhookNotifier = NewResourceURLNotifier(ResourceURLNotifierParams{
		Logger:    logger.GetLogger().WithComponent("webhook"),
		APIKey:    config.APIKey,
		APISecret: apiSecret,
		Config:    config.ResourceURLNotifier,
	})

	return n, nil
}

func (n *DefaultNotifier) Stop(force bool) {
	wg := sync.WaitGroup{}
	for _, u := range n.notifiers {
		wg.Add(1)
		go func(u QueuedNotifier) {
			defer wg.Done()
			u.Stop(force)
		}(u)
	}

	wg.Add(1)
	go func() {
		defer wg.Done()
		n.extraWebhookNotifier.Stop(force)
	}()

	wg.Wait()
}

func (n *DefaultNotifier) QueueNotify(ctx context.Context, event *livekit.WebhookEvent, opts ...NotifyOption) error {
	for _, u := range n.notifiers {
		// No override for static notifiers
		if err := u.QueueNotify(ctx, event); err != nil {
			return err
		}
	}

	p := &NotifyParams{}
	for _, o := range opts {
		o(p)
	}

	for _, wh := range p.ExtraWebhooks {
		lopts := []NotifyOption{
			WithExtraWebhooks([]*livekit.WebhookConfig{wh}),
		}

		if wh.SigningKey != "" {
			// empty signing key means default
			k := n.kp.GetSecret(wh.SigningKey)
			if k == "" {
				return fmt.Errorf("no secret for provided signing key")
			}

			lopts = append(lopts, WithSecret(k))
		}

		if err := n.extraWebhookNotifier.QueueNotify(ctx, event, lopts...); err != nil {
			return err
		}
	}

	return nil
}

func (n *DefaultNotifier) RegisterProcessedHook(hook func(ctx context.Context, whi *livekit.WebhookInfo)) {
	for _, u := range n.notifiers {
		u.RegisterProcessedHook(hook)
	}
}

func (n *DefaultNotifier) SetKeys(apiKey, apiSecret string) {
	for _, u := range n.notifiers {
		u.SetKeys(apiKey, apiSecret)
	}
}

func (n *DefaultNotifier) SetFilter(params FilterParams) {
	for _, u := range n.notifiers {
		u.SetFilter(params)
	}
}

// ---------------------------------

type HTTPClientParams struct {
	RetryWaitMin  time.Duration
	RetryWaitMax  time.Duration
	MaxRetries    int
	ClientTimeout time.Duration
	ForceIPv4     bool
}

type FilterParams struct {
	IncludeEvents []string `yaml:"include_events,omitempty"`
	ExcludeEvents []string `yaml:"exclude_events,omitempty"`
}

// ---------------------------------

type logAdapter struct{}

func (l *logAdapter) Printf(string, ...interface{}) {}

// ---------------------------------

func EventKey(event *livekit.WebhookEvent) string {
	if event.EgressInfo != nil {
		return event.EgressInfo.EgressId
	}
	if event.IngressInfo != nil {
		return event.IngressInfo.IngressId
	}
	if event.Room != nil {
		return event.Room.Name
	}
	if event.Participant != nil {
		return event.Participant.Identity
	}
	if event.Track != nil {
		return event.Track.Sid
	}
	logger.Warnw("webhook using default event", nil, "event", logger.Proto(event))
	return "default"
}

func logFields(event *livekit.WebhookEvent, url string) []interface{} {
	fields := make([]interface{}, 0, 20)
	fields = append(fields,
		"event", event.Event,
		"id", event.Id,
		"webhookTime", event.CreatedAt,
		"url", url,
	)

	if event.Room != nil {
		fields = append(fields,
			"room", event.Room.Name,
			"roomID", event.Room.Sid,
		)
	}
	if event.Participant != nil {
		fields = append(fields,
			"participant", event.Participant.Identity,
			"pID", event.Participant.Sid,
		)
	}
	if event.Track != nil {
		fields = append(fields,
			"trackID", event.Track.Sid,
		)
	}
	if event.EgressInfo != nil {
		fields = append(fields,
			"egressID", event.EgressInfo.EgressId,
			"status", event.EgressInfo.Status,
		)
		if event.EgressInfo.Error != "" {
			fields = append(fields, "error", event.EgressInfo.Error)
		}
	}
	if event.IngressInfo != nil {
		fields = append(fields,
			"ingressID", event.IngressInfo.IngressId,
		)
		if event.IngressInfo.State != nil {
			fields = append(fields, "status", event.IngressInfo.State.Status)
			if event.IngressInfo.State.Error != "" {
				fields = append(fields, "error", event.IngressInfo.State.Error)
			}
		}
	}
	return fields
}

func webhookInfo(
	event *livekit.WebhookEvent,
	queuedAt time.Time,
	queueDuration time.Duration,
	sentAt time.Time,
	sendDuration time.Duration,
	url string,
	isDropped bool,
	sendError error,
) *livekit.WebhookInfo {
	whi := &livekit.WebhookInfo{
		EventId:         event.Id,
		Event:           event.Event,
		CreatedAt:       timestamppb.New(time.Unix(event.CreatedAt, 0)),
		QueuedAt:        timestamppb.New(queuedAt),
		QueueDurationNs: queueDuration.Nanoseconds(),
		SentAt:          timestamppb.New(sentAt),
		SendDurationNs:  sendDuration.Nanoseconds(),
		Url:             url,
		NumDropped:      event.NumDropped,
		IsDropped:       isDropped,
	}
	if !queuedAt.IsZero() {
		whi.QueuedAt = timestamppb.New(queuedAt)
	}
	if !sentAt.IsZero() {
		whi.SentAt = timestamppb.New(sentAt)
	}
	if event.Room != nil {
		whi.RoomName = event.Room.Name
		whi.RoomId = event.Room.Sid
	}
	if event.Participant != nil {
		whi.ParticipantIdentity = event.Participant.Identity
		whi.ParticipantId = event.Participant.Sid
	}
	if event.Track != nil {
		whi.TrackId = event.Track.Sid
	}
	if event.EgressInfo != nil {
		whi.EgressId = event.EgressInfo.EgressId
		whi.ServiceStatus = event.EgressInfo.Status.String()
		if event.EgressInfo.Error != "" {
			whi.ServiceErrorCode = event.EgressInfo.ErrorCode
			whi.ServiceError = event.EgressInfo.Error
		}
	}
	if event.IngressInfo != nil {
		whi.IngressId = event.IngressInfo.IngressId
		if event.IngressInfo.State != nil {
			whi.ServiceStatus = event.IngressInfo.State.Status.String()
			if event.IngressInfo.State.Error != "" {
				whi.ServiceError = event.IngressInfo.State.Error
			}
		}
	}
	if sendError != nil {
		whi.SendError = sendError.Error()
	}
	return whi
}
