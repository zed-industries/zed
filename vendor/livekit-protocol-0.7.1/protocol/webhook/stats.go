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
	"sync"

	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promauto"
)

var (
	promWebhookDispatchTotal        *prometheus.CounterVec
	promWebhookQueueLengthHistogram prometheus.Histogram

	promWebhookInitOnce sync.Once
)

func InitWebhookStats(constLabels prometheus.Labels) {
	promWebhookInitOnce.Do(func() { initWebhookStats(constLabels) })
}

func initWebhookStats(constLabels prometheus.Labels) {
	promWebhookDispatchTotal = promauto.NewCounterVec(prometheus.CounterOpts{
		Namespace:   "livekit",
		Subsystem:   "webhook",
		Name:        "dispatch_total",
		ConstLabels: constLabels,
	}, []string{"status", "reason"})

	promWebhookQueueLengthHistogram = promauto.NewHistogram(prometheus.HistogramOpts{
		Namespace:   "livekit",
		Subsystem:   "webhook",
		Name:        "queue_length",
		ConstLabels: constLabels,
		Buckets:     []float64{1, 2, 3, 4, 5, 10, 20, 40, 80},
	})
}

func IncDispatchSuccess() {
	promWebhookDispatchTotal.WithLabelValues("success", "").Inc()
}

func IncDispatchFailure() {
	promWebhookDispatchTotal.WithLabelValues("failure", "").Inc()
}

func IncDispatchDrop(reason string) {
	promWebhookDispatchTotal.WithLabelValues("drop", reason).Inc()
}

func RecordQueueLength(queueLength int) {
	promWebhookQueueLengthHistogram.Observe(float64(queueLength))
}
