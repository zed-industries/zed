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
	"net"
	"net/http"
	"sync"
	"testing"
	"time"

	"github.com/prometheus/client_golang/prometheus"
	"github.com/stretchr/testify/require"
	"go.uber.org/atomic"

	"github.com/livekit/protocol/auth"
	"github.com/livekit/protocol/livekit"
)

const (
	testAPIKey           = "mykey"
	testAPISecret        = "mysecret"
	testAddr             = ":8765"
	testUrl              = "http://localhost:8765"
	webhookCheckInterval = 100 * time.Millisecond
)

var authProvider = auth.NewSimpleKeyProvider(
	testAPIKey, testAPISecret,
)

func TestWebHook(t *testing.T) {
	InitWebhookStats(prometheus.Labels{})

	s := newServer(testAddr)
	require.NoError(t, s.Start())
	defer s.Stop()

	t.Run("test event payload", func(t *testing.T) {
		notifier := newTestNotifier()
		defer notifier.Stop(false)

		event := &livekit.WebhookEvent{
			Event: EventTrackPublished,
			Participant: &livekit.ParticipantInfo{
				Identity: "test",
			},
			Track: &livekit.TrackInfo{
				Sid: "TR_abcde",
			},
		}

		wg := sync.WaitGroup{}
		wg.Add(1)
		expectedUrl := "/"
		s.handler = func(w http.ResponseWriter, r *http.Request) {
			defer wg.Done()
			decodedEvent, err := ReceiveWebhookEvent(r, authProvider)
			require.NoError(t, err)

			require.EqualValues(t, event, decodedEvent)
			require.Equal(t, expectedUrl, r.URL.String())
		}
		require.NoError(t, notifier.QueueNotify(context.Background(), event))
		wg.Wait()

		wg.Add(1)
		expectedUrl = "/wh"
		require.NoError(t, notifier.QueueNotify(context.Background(), event, WithExtraWebhooks([]*livekit.WebhookConfig{&livekit.WebhookConfig{Url: "http://localhost:8765/wh"}})))
		wg.Wait()

	})
}

func TestURLNotifierDropped(t *testing.T) {
	InitWebhookStats(prometheus.Labels{})

	s := newServer(testAddr)
	require.NoError(t, s.Start())
	defer s.Stop()

	urlNotifier := newTestNotifier()
	defer urlNotifier.Stop(true)
	totalDropped := atomic.Int32{}
	totalReceived := atomic.Int32{}
	s.handler = func(w http.ResponseWriter, r *http.Request) {
		decodedEvent, err := ReceiveWebhookEvent(r, authProvider)
		require.NoError(t, err)
		totalReceived.Inc()
		totalDropped.Add(decodedEvent.NumDropped)
	}
	// send multiple notifications
	for i := 0; i < 10; i++ {
		_ = urlNotifier.QueueNotify(context.Background(), &livekit.WebhookEvent{Event: EventRoomStarted})
		_ = urlNotifier.QueueNotify(context.Background(), &livekit.WebhookEvent{Event: EventParticipantJoined})
		_ = urlNotifier.QueueNotify(context.Background(), &livekit.WebhookEvent{Event: EventRoomFinished})
	}

	time.Sleep(webhookCheckInterval)

	require.Equal(t, int32(30), totalDropped.Load()+totalReceived.Load())
	// at least one request dropped
	require.Less(t, int32(0), totalDropped.Load())
}

func TestURLNotifierLifecycle(t *testing.T) {
	InitWebhookStats(prometheus.Labels{})

	s := newServer(testAddr)
	require.NoError(t, s.Start())
	defer s.Stop()

	t.Run("start/stop without use", func(t *testing.T) {
		urlNotifier := newTestNotifier()
		urlNotifier.Stop(false)
	})

	t.Run("stop allowing to drain", func(t *testing.T) {
		urlNotifier := newTestNotifier()
		numCalled := atomic.Int32{}
		s.handler = func(w http.ResponseWriter, r *http.Request) {
			numCalled.Inc()
		}
		for i := 0; i < 10; i++ {
			_ = urlNotifier.QueueNotify(context.Background(), &livekit.WebhookEvent{Event: EventRoomStarted})
			_ = urlNotifier.QueueNotify(context.Background(), &livekit.WebhookEvent{Event: EventRoomFinished})
		}
		urlNotifier.Stop(false)
		require.Eventually(t, func() bool { return numCalled.Load() == 20 }, 5*time.Second, webhookCheckInterval)
	})

	t.Run("force stop", func(t *testing.T) {
		urlNotifier := newTestNotifier()
		numCalled := atomic.Int32{}
		s.handler = func(w http.ResponseWriter, r *http.Request) {
			numCalled.Inc()
		}
		for i := 0; i < 10; i++ {
			_ = urlNotifier.QueueNotify(context.Background(), &livekit.WebhookEvent{Event: EventRoomStarted})
			_ = urlNotifier.QueueNotify(context.Background(), &livekit.WebhookEvent{Event: EventRoomFinished})
		}
		urlNotifier.Stop(true)
		time.Sleep(time.Second)
		require.Greater(t, int32(20), numCalled.Load())
	})

	t.Run("times out after accepting connection", func(t *testing.T) {
		urlNotifier := NewURLNotifier(URLNotifierParams{
			URL:       testUrl,
			APIKey:    testAPIKey,
			APISecret: testAPISecret,
			HTTPClientParams: HTTPClientParams{
				RetryWaitMax:  time.Millisecond,
				MaxRetries:    1,
				ClientTimeout: 100 * time.Millisecond,
			},
			Config: URLNotifierConfig{
				QueueSize: 20,
			},
		})

		numCalled := atomic.Int32{}
		s.handler = func(w http.ResponseWriter, r *http.Request) {
			w.WriteHeader(200)
			w.Write([]byte("ok"))

			// delay the request to cause it to fail
			time.Sleep(time.Second)
			if r.Context().Err() == nil {
				// inc if not canceled
				numCalled.Inc()
			}
		}
		defer urlNotifier.Stop(false)

		err := urlNotifier.send(&livekit.WebhookEvent{Event: EventRoomStarted}, &urlNotifier.params)
		require.Error(t, err)
	})

	t.Run("times out before connection", func(t *testing.T) {
		ln, err := net.Listen("tcp", ":9987")
		require.NoError(t, err)
		defer ln.Close()
		urlNotifier := NewURLNotifier(URLNotifierParams{
			URL:       "http://localhost:9987",
			APIKey:    testAPIKey,
			APISecret: testAPISecret,
			HTTPClientParams: HTTPClientParams{
				RetryWaitMax:  time.Millisecond,
				MaxRetries:    1,
				ClientTimeout: 100 * time.Millisecond,
			},
		})
		defer urlNotifier.Stop(false)

		startedAt := time.Now()
		err = urlNotifier.send(&livekit.WebhookEvent{Event: EventRoomStarted}, &urlNotifier.params)
		require.Error(t, err)
		require.Less(t, time.Since(startedAt).Seconds(), float64(2))
	})
}

func TestURLNotifierFilter(t *testing.T) {
	InitWebhookStats(prometheus.Labels{})

	s := newServer(testAddr)
	require.NoError(t, s.Start())
	defer s.Stop()

	t.Run("none", func(t *testing.T) {
		urlNotifier := NewURLNotifier(URLNotifierParams{
			URL:       testUrl,
			APIKey:    testAPIKey,
			APISecret: testAPISecret,
			Config: URLNotifierConfig{
				QueueSize: 20,
			},
			HTTPClientParams: HTTPClientParams{
				ForceIPv4: true,
			},
		})
		defer urlNotifier.Stop(false)

		numCalled := atomic.Int32{}
		s.handler = func(w http.ResponseWriter, r *http.Request) {
			numCalled.Inc()
		}

		_ = urlNotifier.QueueNotify(context.Background(), &livekit.WebhookEvent{Event: EventRoomStarted})
		_ = urlNotifier.QueueNotify(context.Background(), &livekit.WebhookEvent{Event: EventRoomFinished})
		require.Eventually(
			t,
			func() bool {
				return numCalled.Load() == 2
			},
			5*time.Second,
			webhookCheckInterval,
		)
	})

	t.Run("includes", func(t *testing.T) {
		urlNotifier := NewURLNotifier(URLNotifierParams{
			URL:       testUrl,
			APIKey:    testAPIKey,
			APISecret: testAPISecret,
			FilterParams: FilterParams{
				IncludeEvents: []string{EventRoomStarted},
			},
			Config: URLNotifierConfig{
				QueueSize: 20,
			},
		})
		defer urlNotifier.Stop(false)

		numCalled := atomic.Int32{}
		s.handler = func(w http.ResponseWriter, r *http.Request) {
			numCalled.Inc()
		}

		_ = urlNotifier.QueueNotify(context.Background(), &livekit.WebhookEvent{Event: EventRoomStarted})
		_ = urlNotifier.QueueNotify(context.Background(), &livekit.WebhookEvent{Event: EventRoomFinished})
		require.Eventually(
			t,
			func() bool {
				return numCalled.Load() == 1
			},
			5*time.Second,
			webhookCheckInterval,
		)
	})

	t.Run("excludes", func(t *testing.T) {
		urlNotifier := NewURLNotifier(URLNotifierParams{
			URL:       testUrl,
			APIKey:    testAPIKey,
			APISecret: testAPISecret,
			FilterParams: FilterParams{
				ExcludeEvents: []string{EventRoomStarted},
			},
			Config: URLNotifierConfig{
				QueueSize: 20,
			},
		})
		defer urlNotifier.Stop(false)

		numCalled := atomic.Int32{}
		s.handler = func(w http.ResponseWriter, r *http.Request) {
			numCalled.Inc()
		}

		_ = urlNotifier.QueueNotify(context.Background(), &livekit.WebhookEvent{Event: EventRoomStarted})
		_ = urlNotifier.QueueNotify(context.Background(), &livekit.WebhookEvent{Event: EventRoomFinished})
		require.Eventually(
			t,
			func() bool {
				return numCalled.Load() == 1
			},
			5*time.Second,
			webhookCheckInterval,
		)
	})

	t.Run("includes + excludes", func(t *testing.T) {
		urlNotifier := NewURLNotifier(URLNotifierParams{
			URL:       testUrl,
			APIKey:    testAPIKey,
			APISecret: testAPISecret,
			FilterParams: FilterParams{
				IncludeEvents: []string{EventRoomStarted},
				ExcludeEvents: []string{EventRoomStarted, EventRoomFinished},
			},
			Config: URLNotifierConfig{
				QueueSize: 20,
			},
		})
		defer urlNotifier.Stop(false)

		numCalled := atomic.Int32{}
		s.handler = func(w http.ResponseWriter, r *http.Request) {
			numCalled.Inc()
		}

		// EventRoomStarted should be allowed as IncludeEvents take precedence
		_ = urlNotifier.QueueNotify(context.Background(), &livekit.WebhookEvent{Event: EventRoomStarted})
		_ = urlNotifier.QueueNotify(context.Background(), &livekit.WebhookEvent{Event: EventRoomFinished})
		require.Eventually(
			t,
			func() bool {
				return numCalled.Load() == 1
			},
			5*time.Second,
			webhookCheckInterval,
		)
	})
}

func newTestNotifier() *URLNotifier {
	return NewURLNotifier(URLNotifierParams{
		URL:       testUrl,
		APIKey:    testAPIKey,
		APISecret: testAPISecret,
		Config: URLNotifierConfig{
			QueueSize: 20,
		},
	})
}

// --------------------------------------------

func TestResourceWebHook(t *testing.T) {
	s := newServer(testAddr)
	require.NoError(t, s.Start())
	defer s.Stop()

	t.Run("test event payload", func(t *testing.T) {
		resourceURLNotifier, err := NewDefaultNotifier(
			WebHookConfig{
				URLs:   []string{testUrl},
				APIKey: testAPIKey,
			},
			authProvider,
		)
		require.NoError(t, err)
		defer resourceURLNotifier.Stop(false)

		event := &livekit.WebhookEvent{
			Event: EventTrackPublished,
			Participant: &livekit.ParticipantInfo{
				Identity: "test",
			},
			Track: &livekit.TrackInfo{
				Sid: "TR_abcde",
			},
		}

		wg := sync.WaitGroup{}
		wg.Add(1)
		s.handler = func(w http.ResponseWriter, r *http.Request) {
			defer wg.Done()
			decodedEvent, err := ReceiveWebhookEvent(r, authProvider)
			require.NoError(t, err)

			require.EqualValues(t, event, decodedEvent)
		}
		require.NoError(t, resourceURLNotifier.QueueNotify(context.Background(), event))
		wg.Wait()
	})

}

func TestResourceURLNotifierDropped(t *testing.T) {
	s := newServer(testAddr)
	require.NoError(t, s.Start())
	defer s.Stop()

	t.Run("depth drop", func(t *testing.T) {
		resourceURLNotifier := newTestResourceNotifier(time.Minute, time.Minute, 5)
		defer resourceURLNotifier.Stop(true)
		totalDropped := atomic.Int32{}
		totalReceived := atomic.Int32{}
		s.handler = func(w http.ResponseWriter, r *http.Request) {
			_, err := ReceiveWebhookEvent(r, authProvider)
			require.NoError(t, err)
			totalReceived.Inc()
		}
		// send multiple notifications
		for i := 0; i < 10; i++ {
			err := resourceURLNotifier.QueueNotify(context.Background(), &livekit.WebhookEvent{Event: EventRoomStarted})
			if err == errQueueFull {
				totalDropped.Inc()
			}
			err = resourceURLNotifier.QueueNotify(context.Background(), &livekit.WebhookEvent{Event: EventParticipantJoined})
			if err == errQueueFull {
				totalDropped.Inc()
			}
			err = resourceURLNotifier.QueueNotify(context.Background(), &livekit.WebhookEvent{Event: EventRoomFinished})
			if err == errQueueFull {
				totalDropped.Inc()
			}
		}

		time.Sleep(webhookCheckInterval)

		require.Eventually(
			t,
			func() bool {
				return totalDropped.Load()+totalReceived.Load() == 30
			},
			5*time.Second,
			webhookCheckInterval,
		)
		// at least one request dropped, but not all dropped
		require.Less(t, int32(0), totalDropped.Load())
		require.Less(t, int32(0), totalReceived.Load())
	})

	t.Run("age drop", func(t *testing.T) {
		resourceURLNotifier := newTestResourceNotifier(time.Minute, 10*time.Millisecond, 500)
		defer resourceURLNotifier.Stop(true)
		totalReceived := atomic.Int32{}
		s.handler = func(w http.ResponseWriter, r *http.Request) {
			time.Sleep(5 * time.Millisecond)
			_, err := ReceiveWebhookEvent(r, authProvider)
			require.NoError(t, err)
			totalReceived.Inc()
		}
		// send multiple notifications
		for i := 0; i < 10; i++ {
			_ = resourceURLNotifier.QueueNotify(context.Background(), &livekit.WebhookEvent{Event: EventRoomStarted})
			time.Sleep(time.Millisecond)
			_ = resourceURLNotifier.QueueNotify(context.Background(), &livekit.WebhookEvent{Event: EventParticipantJoined})
			time.Sleep(time.Millisecond)
			_ = resourceURLNotifier.QueueNotify(context.Background(), &livekit.WebhookEvent{Event: EventRoomFinished})
			time.Sleep(time.Millisecond)
		}

		time.Sleep(2 * webhookCheckInterval)

		// at least one request dropped
		require.Greater(t, int32(30), totalReceived.Load())
		require.Less(t, int32(0), totalReceived.Load())
	})

	t.Run("resource queue timeout", func(t *testing.T) {
		resourceURLNotifier := newTestResourceNotifier(5*time.Millisecond, time.Minute, 500)
		defer resourceURLNotifier.Stop(true)
		totalReceived := atomic.Int32{}
		s.handler = func(w http.ResponseWriter, r *http.Request) {
			_, err := ReceiveWebhookEvent(r, authProvider)
			require.NoError(t, err)
			totalReceived.Inc()
		}

		// check that resource queues change for the same event key
		for i := 0; i < 3; i++ {
			var rq *resourceQueue

			roomName := fmt.Sprintf("room%d", i)

			_ = resourceURLNotifier.QueueNotify(
				context.Background(),
				&livekit.WebhookEvent{
					Event: EventRoomStarted,
					Room: &livekit.Room{
						Name: roomName,
					},
				},
			)
			resourceURLNotifier.mu.RLock()
			rqi := resourceURLNotifier.resourceQueues[roomName]
			resourceURLNotifier.mu.RUnlock()
			require.NotNil(t, rqi)
			require.NotNil(t, rqi.resourceQueue)
			require.NotSame(t, rqi.resourceQueue, rq)
			rq = rqi.resourceQueue
			time.Sleep(10 * time.Millisecond)

			_ = resourceURLNotifier.QueueNotify(
				context.Background(),
				&livekit.WebhookEvent{
					Event: EventParticipantJoined,
					Room: &livekit.Room{
						Name: roomName,
					},
				},
			)
			resourceURLNotifier.mu.RLock()
			rqi = resourceURLNotifier.resourceQueues[roomName]
			resourceURLNotifier.mu.RUnlock()
			require.NotNil(t, rqi)
			require.NotNil(t, rqi.resourceQueue)
			require.NotSame(t, rqi.resourceQueue, rq)
			rq = rqi.resourceQueue
			time.Sleep(10 * time.Millisecond)

			_ = resourceURLNotifier.QueueNotify(
				context.Background(),
				&livekit.WebhookEvent{
					Event: EventParticipantLeft,
					Room: &livekit.Room{
						Name: roomName,
					},
				},
			)
			resourceURLNotifier.mu.RLock()
			rqi = resourceURLNotifier.resourceQueues[roomName]
			resourceURLNotifier.mu.RUnlock()
			require.NotNil(t, rqi)
			require.NotNil(t, rqi.resourceQueue)
			require.NotSame(t, rqi.resourceQueue, rq)
			rq = rqi.resourceQueue
			time.Sleep(10 * time.Millisecond)
		}

		time.Sleep(webhookCheckInterval)

		require.Equal(t, int32(9), totalReceived.Load())
	})
}

func TestResourceURLNotifierLifecycle(t *testing.T) {
	s := newServer(testAddr)
	require.NoError(t, s.Start())
	defer s.Stop()

	t.Run("start/stop without use", func(t *testing.T) {
		resourceURLNotifier := newTestResourceNotifier(time.Minute, 200*time.Millisecond, 50)
		resourceURLNotifier.Stop(false)
	})

	t.Run("sweeper", func(t *testing.T) {
		resourceURLNotifier := newTestResourceNotifier(200*time.Millisecond, 200*time.Millisecond, 50)
		numCalled := atomic.Int32{}
		s.handler = func(w http.ResponseWriter, r *http.Request) {
			numCalled.Inc()
		}
		for i := 0; i < 10; i++ {
			roomName := fmt.Sprintf("room%d", i)
			_ = resourceURLNotifier.QueueNotify(
				context.Background(),
				&livekit.WebhookEvent{
					Event: EventRoomStarted,
					Room: &livekit.Room{
						Name: roomName,
					},
				},
			)

			_ = resourceURLNotifier.QueueNotify(
				context.Background(),
				&livekit.WebhookEvent{
					Event: EventRoomFinished,
					Room: &livekit.Room{
						Name: roomName,
					},
				},
			)
		}

		resourceURLNotifier.mu.RLock()
		require.Equal(t, 10, len(resourceURLNotifier.resourceQueues))
		resourceURLNotifier.mu.RUnlock()

		time.Sleep(time.Second)

		// should have reaped after some time
		resourceURLNotifier.mu.RLock()
		require.Equal(t, 0, len(resourceURLNotifier.resourceQueues))
		resourceURLNotifier.mu.RUnlock()

		require.Equal(t, int32(20), numCalled.Load())
	})

	t.Run("stop allowing to drain", func(t *testing.T) {
		resourceURLNotifier := newTestResourceNotifier(time.Minute, 200*time.Millisecond, 50)
		numCalled := atomic.Int32{}
		s.handler = func(w http.ResponseWriter, r *http.Request) {
			numCalled.Inc()
		}
		for i := 0; i < 10; i++ {
			_ = resourceURLNotifier.QueueNotify(context.Background(), &livekit.WebhookEvent{Event: EventRoomStarted})
			_ = resourceURLNotifier.QueueNotify(context.Background(), &livekit.WebhookEvent{Event: EventRoomFinished})
		}
		resourceURLNotifier.Stop(false)
		require.Eventually(t, func() bool { return numCalled.Load() == 20 }, 5*time.Second, webhookCheckInterval)
	})

	t.Run("force stop", func(t *testing.T) {
		resourceURLNotifier := newTestResourceNotifier(time.Minute, 200*time.Millisecond, 50)
		numCalled := atomic.Int32{}
		s.handler = func(w http.ResponseWriter, r *http.Request) {
			numCalled.Inc()
		}
		for i := 0; i < 10; i++ {
			_ = resourceURLNotifier.QueueNotify(context.Background(), &livekit.WebhookEvent{Event: EventRoomStarted})
			_ = resourceURLNotifier.QueueNotify(context.Background(), &livekit.WebhookEvent{Event: EventRoomFinished})
		}
		resourceURLNotifier.Stop(true)
		time.Sleep(time.Second)
		require.Greater(t, int32(20), numCalled.Load())
	})

	t.Run("times out after accepting connection", func(t *testing.T) {
		params := ResourceURLNotifierParams{
			URL:       testUrl,
			APIKey:    testAPIKey,
			APISecret: testAPISecret,
			Config: ResourceURLNotifierConfig{
				MaxAge:   200 * time.Millisecond,
				MaxDepth: 50,
			},
			HTTPClientParams: HTTPClientParams{
				RetryWaitMax:  time.Millisecond,
				MaxRetries:    1,
				ClientTimeout: 100 * time.Millisecond,
			},
		}

		resourceURLNotifier := NewResourceURLNotifier(params)

		numCalled := atomic.Int32{}
		s.handler = func(w http.ResponseWriter, r *http.Request) {
			w.WriteHeader(200)
			w.Write([]byte("ok"))

			// delay the request to cause it to fail
			time.Sleep(time.Second)
			if r.Context().Err() == nil {
				// inc if not canceled
				numCalled.Inc()
			}
		}
		defer resourceURLNotifier.Stop(false)

		err := resourceURLNotifier.send(&livekit.WebhookEvent{Event: EventRoomStarted}, &params)
		require.Error(t, err)
	})

	t.Run("times out before connection", func(t *testing.T) {
		ln, err := net.Listen("tcp", ":9987")
		require.NoError(t, err)
		defer ln.Close()

		params := ResourceURLNotifierParams{
			URL:       "http://localhost:9987",
			APIKey:    testAPIKey,
			APISecret: testAPISecret,
			Config: ResourceURLNotifierConfig{
				MaxAge:   200 * time.Millisecond,
				MaxDepth: 50,
			},
			HTTPClientParams: HTTPClientParams{
				RetryWaitMax:  time.Millisecond,
				MaxRetries:    1,
				ClientTimeout: 100 * time.Millisecond,
			},
		}

		resourceURLNotifier := NewResourceURLNotifier(params)
		defer resourceURLNotifier.Stop(false)

		startedAt := time.Now()
		err = resourceURLNotifier.send(&livekit.WebhookEvent{Event: EventRoomStarted}, &params)
		require.Error(t, err)
		require.Less(t, time.Since(startedAt).Seconds(), float64(2))
	})
}

func TestResourceURLNotifierFilter(t *testing.T) {
	s := newServer(testAddr)
	require.NoError(t, s.Start())
	defer s.Stop()

	t.Run("none", func(t *testing.T) {
		resourceURLNotifier := NewResourceURLNotifier(ResourceURLNotifierParams{
			URL:       testUrl,
			APIKey:    testAPIKey,
			APISecret: testAPISecret,
			Config: ResourceURLNotifierConfig{
				MaxAge:   200 * time.Millisecond,
				MaxDepth: 50,
			},
			FilterParams: FilterParams{},
		})
		defer resourceURLNotifier.Stop(false)

		numCalled := atomic.Int32{}
		s.handler = func(w http.ResponseWriter, r *http.Request) {
			numCalled.Inc()
		}

		_ = resourceURLNotifier.QueueNotify(context.Background(), &livekit.WebhookEvent{Event: EventRoomStarted})
		_ = resourceURLNotifier.QueueNotify(context.Background(), &livekit.WebhookEvent{Event: EventRoomFinished})
		require.Eventually(
			t,
			func() bool {
				return numCalled.Load() == 2
			},
			5*time.Second,
			webhookCheckInterval,
		)
	})

	t.Run("includes", func(t *testing.T) {
		resourceURLNotifier := NewResourceURLNotifier(ResourceURLNotifierParams{
			URL:       testUrl,
			APIKey:    testAPIKey,
			APISecret: testAPISecret,
			Config: ResourceURLNotifierConfig{
				MaxAge:   200 * time.Millisecond,
				MaxDepth: 50,
			},
			FilterParams: FilterParams{
				IncludeEvents: []string{EventRoomStarted},
			},
		})
		defer resourceURLNotifier.Stop(false)

		numCalled := atomic.Int32{}
		s.handler = func(w http.ResponseWriter, r *http.Request) {
			numCalled.Inc()
		}

		_ = resourceURLNotifier.QueueNotify(context.Background(), &livekit.WebhookEvent{Event: EventRoomStarted})
		_ = resourceURLNotifier.QueueNotify(context.Background(), &livekit.WebhookEvent{Event: EventRoomFinished})
		require.Eventually(
			t,
			func() bool {
				return numCalled.Load() == 1
			},
			5*time.Second,
			webhookCheckInterval,
		)
	})

	t.Run("excludes", func(t *testing.T) {
		resourceURLNotifier := NewResourceURLNotifier(ResourceURLNotifierParams{
			URL:       testUrl,
			APIKey:    testAPIKey,
			APISecret: testAPISecret,
			Config: ResourceURLNotifierConfig{
				MaxAge:   200 * time.Millisecond,
				MaxDepth: 50,
			},
			FilterParams: FilterParams{
				ExcludeEvents: []string{EventRoomStarted},
			},
		})
		defer resourceURLNotifier.Stop(false)

		numCalled := atomic.Int32{}
		s.handler = func(w http.ResponseWriter, r *http.Request) {
			numCalled.Inc()
		}

		_ = resourceURLNotifier.QueueNotify(context.Background(), &livekit.WebhookEvent{Event: EventRoomStarted})
		_ = resourceURLNotifier.QueueNotify(context.Background(), &livekit.WebhookEvent{Event: EventRoomFinished})
		require.Eventually(
			t,
			func() bool {
				return numCalled.Load() == 1
			},
			5*time.Second,
			webhookCheckInterval,
		)
	})

	t.Run("includes + excludes", func(t *testing.T) {
		resourceURLNotifier := NewResourceURLNotifier(ResourceURLNotifierParams{
			URL:       testUrl,
			APIKey:    testAPIKey,
			APISecret: testAPISecret,
			Config: ResourceURLNotifierConfig{
				MaxAge:   200 * time.Millisecond,
				MaxDepth: 50,
			},
			FilterParams: FilterParams{
				IncludeEvents: []string{EventRoomStarted},
				ExcludeEvents: []string{EventRoomStarted, EventRoomFinished},
			},
		})
		defer resourceURLNotifier.Stop(false)

		numCalled := atomic.Int32{}
		s.handler = func(w http.ResponseWriter, r *http.Request) {
			numCalled.Inc()
		}

		// EventRoomStarted should be allowed as IncludeEvents take precedence
		_ = resourceURLNotifier.QueueNotify(context.Background(), &livekit.WebhookEvent{Event: EventRoomStarted})
		_ = resourceURLNotifier.QueueNotify(context.Background(), &livekit.WebhookEvent{Event: EventRoomFinished})
		require.Eventually(
			t,
			func() bool {
				return numCalled.Load() == 1
			},
			5*time.Second,
			webhookCheckInterval,
		)
	})
}

func newTestResourceNotifier(timeout time.Duration, maxAge time.Duration, maxDepth int) *ResourceURLNotifier {
	return NewResourceURLNotifier(ResourceURLNotifierParams{
		URL:       testUrl,
		APIKey:    testAPIKey,
		APISecret: testAPISecret,
		Timeout:   timeout,
		Config: ResourceURLNotifierConfig{
			MaxAge:   maxAge,
			MaxDepth: maxDepth,
		},
	})
}

// ---------------------------------------

type testServer struct {
	handler func(w http.ResponseWriter, r *http.Request)
	server  *http.Server
}

func newServer(addr string) *testServer {
	s := &testServer{}
	s.server = &http.Server{
		Addr:    addr,
		Handler: s,
	}
	return s
}

func (s *testServer) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	if s.handler != nil {
		s.handler(w, r)
	}
}

func (s *testServer) Start() error {
	l, err := net.Listen("tcp", s.server.Addr)
	if err != nil {
		return err
	}
	go s.server.Serve(l)
	return nil
}

func (s *testServer) Stop() {
	_ = s.server.Shutdown(context.Background())
}
