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

package utils

import (
	"testing"
	"time"

	"github.com/stretchr/testify/require"
	"google.golang.org/protobuf/proto"
	"google.golang.org/protobuf/types/known/timestamppb"

	"github.com/livekit/protocol/livekit"
	"github.com/livekit/protocol/utils/mono"
)

func TestMetricsBatchBuilder(t *testing.T) {
	t.Run("basic", func(t *testing.T) {
		at := mono.Now()
		normalizedAt := mono.Now().Add(10 * time.Millisecond)
		expected := &livekit.MetricsBatch{
			TimestampMs:         at.UnixMilli(),
			NormalizedTimestamp: timestamppb.New(normalizedAt),
		}

		mbb := NewMetricsBatchBuilder()
		require.True(t, mbb.IsEmpty())
		mbb.SetTime(
			time.Unix(0, expected.TimestampMs*int64(time.Millisecond)),
			expected.NormalizedTimestamp.AsTime(),
		)
		mb := mbb.ToProto()
		require.True(t, proto.Equal(expected, mb))
	})

	t.Run("time series metric", func(t *testing.T) {
		at := mono.Now()
		normalizedAt := mono.Now().Add(10 * time.Millisecond)

		expected := &livekit.MetricsBatch{
			TimestampMs:         at.UnixMilli(),
			NormalizedTimestamp: timestamppb.New(normalizedAt),
			StrData: []string{
				"PA_1",
				"TR_VC1",
				"f",
				"CustomMetric",
				"TR_VC2",
				"q",
				"PA_2",
			},
			TimeSeries: []*livekit.TimeSeriesMetric{
				{
					Label:               uint32(livekit.MetricLabel_PUBLISHER_RTT),
					ParticipantIdentity: uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE),
					TrackSid:            uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE) + 1,
					Samples: []*livekit.MetricSample{
						{
							TimestampMs:         at.UnixMilli(),
							NormalizedTimestamp: timestamppb.New(normalizedAt),
							Value:               42.4,
						},
						{
							TimestampMs:         at.UnixMilli(),
							NormalizedTimestamp: timestamppb.New(normalizedAt),
							Value:               52.4,
						},
						{
							TimestampMs:         at.UnixMilli(),
							NormalizedTimestamp: timestamppb.New(normalizedAt),
							Value:               62.4,
						},
					},
					Rid: uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE) + 2,
				},
				{
					Label:               uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE) + 3,
					ParticipantIdentity: uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE),
					TrackSid:            uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE) + 4,
					Samples: []*livekit.MetricSample{
						{
							TimestampMs:         at.UnixMilli(),
							NormalizedTimestamp: timestamppb.New(normalizedAt),
							Value:               72.4,
						},
						{
							TimestampMs:         at.UnixMilli(),
							NormalizedTimestamp: timestamppb.New(normalizedAt),
							Value:               82.4,
						},
						{
							TimestampMs:         at.UnixMilli(),
							NormalizedTimestamp: timestamppb.New(normalizedAt),
							Value:               92.4,
						},
					},
					Rid: uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE) + 5,
				},
				{
					Label:               uint32(livekit.MetricLabel_SUBSCRIBER_RTT),
					ParticipantIdentity: uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE) + 6,
					Samples: []*livekit.MetricSample{
						{
							TimestampMs:         at.UnixMilli(),
							NormalizedTimestamp: timestamppb.New(normalizedAt),
							Value:               102.4,
						},
					},
				},
				{
					Label:               uint32(livekit.MetricLabel_CLIENT_VIDEO_SUBSCRIBER_FREEZE_COUNT),
					ParticipantIdentity: uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE),
					Samples: []*livekit.MetricSample{
						{
							TimestampMs:         at.UnixMilli(),
							NormalizedTimestamp: timestamppb.New(normalizedAt),
							Value:               102.4,
						},
					},
				},
			},
		}

		mbb := NewMetricsBatchBuilder()
		mbb.SetTime(at, normalizedAt)
		mbb.SetRestrictedLabels(MetricRestrictedLabels{
			LabelRanges: []MetricLabelRange{
				{
					StartInclusive: livekit.MetricLabel_CLIENT_VIDEO_SUBSCRIBER_FREEZE_COUNT,
					EndInclusive:   livekit.MetricLabel_CLIENT_VIDEO_PUBLISHER_QUALITY_LIMITATION_DURATION_OTHER,
				},
			},
			ParticipantIdentity: "PA_1",
		})

		// should not be able to add invalid metric label index
		_, err := mbb.AddTimeSeriesMetric(TimeSeriesMetric{
			MetricLabel: livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE,
		})
		require.ErrorIs(t, err, ErrInvalidMetricLabel)

		// add a time series metric
		ts1Idx, err := mbb.AddTimeSeriesMetric(TimeSeriesMetric{
			MetricLabel:         livekit.MetricLabel_PUBLISHER_RTT,
			ParticipantIdentity: "PA_1",
			TrackID:             "TR_VC1",
			Samples: []MetricSample{
				{
					At:           at,
					NormalizedAt: normalizedAt,
					Value:        42.4,
				},
			},
			Rid: "f",
		})
		require.NoError(t, err)
		require.False(t, mbb.IsEmpty())

		// should not be able to add sample to invalid index
		err = mbb.AddMetricSamplesToTimeSeriesMetric(-1, nil)
		require.ErrorIs(t, err, ErrInvalidTimeSeriesMetricIndex)
		err = mbb.AddMetricSamplesToTimeSeriesMetric(ts1Idx+1, nil)
		require.ErrorIs(t, err, ErrInvalidTimeSeriesMetricIndex)

		// add a second one
		ts2Idx, err := mbb.AddTimeSeriesMetric(TimeSeriesMetric{
			CustomMetricLabel:   "CustomMetric",
			ParticipantIdentity: "PA_1",
			TrackID:             "TR_VC2",
			Samples: []MetricSample{
				{
					At:           at,
					NormalizedAt: normalizedAt,
					Value:        72.4,
				},
			},
			Rid: "q",
		})
		require.NoError(t, err)

		// add extra samples to first metric to ensure it gets added to the right metric
		err = mbb.AddMetricSamplesToTimeSeriesMetric(ts1Idx, []MetricSample{
			{
				At:           at,
				NormalizedAt: normalizedAt,
				Value:        52.4,
			},
			{
				At:           at,
				NormalizedAt: normalizedAt,
				Value:        62.4,
			},
		})
		require.NoError(t, err)

		// add extra samples to second metric to ensure it gets added to the right metric
		err = mbb.AddMetricSamplesToTimeSeriesMetric(ts2Idx, []MetricSample{
			{
				At:           at,
				NormalizedAt: normalizedAt,
				Value:        82.4,
			},
			{
				At:           at,
				NormalizedAt: normalizedAt,
				Value:        92.4,
			},
		})
		require.NoError(t, err)

		// add a third metric with some fields not populated to ensure that those indices default to 0
		_, err = mbb.AddTimeSeriesMetric(TimeSeriesMetric{
			MetricLabel:         livekit.MetricLabel_SUBSCRIBER_RTT,
			ParticipantIdentity: "PA_2",
			Samples: []MetricSample{
				{
					At:           at,
					NormalizedAt: normalizedAt,
					Value:        102.4,
				},
			},
		})
		require.NoError(t, err)

		// should accept restricted labels from that participant
		_, err = mbb.AddTimeSeriesMetric(TimeSeriesMetric{
			MetricLabel:         livekit.MetricLabel_CLIENT_VIDEO_SUBSCRIBER_FREEZE_COUNT,
			ParticipantIdentity: "PA_1",
			Samples: []MetricSample{
				{
					At:           at,
					NormalizedAt: normalizedAt,
					Value:        102.4,
				},
			},
		})
		require.NoError(t, err)

		// should not accept restricted labels from any other participant
		_, err = mbb.AddTimeSeriesMetric(TimeSeriesMetric{
			MetricLabel:         livekit.MetricLabel_CLIENT_VIDEO_PUBLISHER_QUALITY_LIMITATION_DURATION_OTHER,
			ParticipantIdentity: "PA_2",
			Samples: []MetricSample{
				{
					At:           at,
					NormalizedAt: normalizedAt,
					Value:        102.4,
				},
			},
		})
		require.ErrorIs(t, err, ErrFilteredMetricLabel)

		mb := mbb.ToProto()
		require.True(t, proto.Equal(expected, mb))
	})

	t.Run("event metric", func(t *testing.T) {
		at := mono.Now()
		atMilli := at.UnixMilli()
		normalizedAt := mono.Now().Add(10 * time.Millisecond)

		expected := &livekit.MetricsBatch{
			TimestampMs:         at.UnixMilli(),
			NormalizedTimestamp: timestamppb.New(normalizedAt),
			StrData: []string{
				"PA_1",
				"TR_VC1",
				"f",
				"CustomMetric",
				"TR_VC2",
				"PA_2",
			},
			Events: []*livekit.EventMetric{
				{
					Label:                    uint32(livekit.MetricLabel_PUBLISHER_RTT),
					ParticipantIdentity:      uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE),
					TrackSid:                 uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE) + 1,
					StartTimestampMs:         at.UnixMilli(),
					EndTimestampMs:           &atMilli,
					NormalizedStartTimestamp: timestamppb.New(normalizedAt),
					NormalizedEndTimestamp:   timestamppb.New(normalizedAt),
					Metadata:                 "md1",
					Rid:                      uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE) + 2,
				},
				{
					Label:                    uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE) + 3,
					ParticipantIdentity:      uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE),
					TrackSid:                 uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE) + 4,
					StartTimestampMs:         at.UnixMilli(),
					NormalizedStartTimestamp: timestamppb.New(normalizedAt),
					Metadata:                 "md2",
				},
				{
					Label:                    uint32(livekit.MetricLabel_CLIENT_VIDEO_PUBLISHER_QUALITY_LIMITATION_DURATION_OTHER),
					ParticipantIdentity:      uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE),
					TrackSid:                 uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE) + 1,
					StartTimestampMs:         at.UnixMilli(),
					EndTimestampMs:           &atMilli,
					NormalizedStartTimestamp: timestamppb.New(normalizedAt),
					NormalizedEndTimestamp:   timestamppb.New(normalizedAt),
					Metadata:                 "md1",
					Rid:                      uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE) + 2,
				},
			},
			TimeSeries: []*livekit.TimeSeriesMetric{
				{
					Label:               uint32(livekit.MetricLabel_SUBSCRIBER_RTT),
					ParticipantIdentity: uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE) + 5,
					Samples: []*livekit.MetricSample{
						{
							TimestampMs:         at.UnixMilli(),
							NormalizedTimestamp: timestamppb.New(normalizedAt),
							Value:               102.4,
						},
					},
				},
			},
		}

		mbb := NewMetricsBatchBuilder()
		mbb.SetTime(at, normalizedAt)
		mbb.SetRestrictedLabels(MetricRestrictedLabels{
			LabelRanges: []MetricLabelRange{
				{
					StartInclusive: livekit.MetricLabel_CLIENT_VIDEO_SUBSCRIBER_FREEZE_COUNT,
					EndInclusive:   livekit.MetricLabel_CLIENT_VIDEO_PUBLISHER_QUALITY_LIMITATION_DURATION_OTHER,
				},
			},
			ParticipantIdentity: "PA_1",
		})

		// should not be able to add invalid metric label index
		err := mbb.AddEventMetric(EventMetric{
			MetricLabel: livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE,
		})
		require.ErrorIs(t, err, ErrInvalidMetricLabel)

		// add an event metric
		err = mbb.AddEventMetric(EventMetric{
			MetricLabel:         livekit.MetricLabel_PUBLISHER_RTT,
			ParticipantIdentity: "PA_1",
			TrackID:             "TR_VC1",
			StartedAt:           at,
			EndedAt:             at,
			NormalizedStartedAt: normalizedAt,
			NormalizedEndedAt:   normalizedAt,
			Metadata:            "md1",
			Rid:                 "f",
		})
		require.NoError(t, err)

		// add a second one with some optional fields not included
		err = mbb.AddEventMetric(EventMetric{
			CustomMetricLabel:   "CustomMetric",
			ParticipantIdentity: "PA_1",
			TrackID:             "TR_VC2",
			StartedAt:           at,
			NormalizedStartedAt: normalizedAt,
			Metadata:            "md2",
		})

		// should accept restricted label from PA_1
		err = mbb.AddEventMetric(EventMetric{
			MetricLabel:         livekit.MetricLabel_CLIENT_VIDEO_PUBLISHER_QUALITY_LIMITATION_DURATION_OTHER,
			ParticipantIdentity: "PA_1",
			TrackID:             "TR_VC1",
			StartedAt:           at,
			EndedAt:             at,
			NormalizedStartedAt: normalizedAt,
			NormalizedEndedAt:   normalizedAt,
			Metadata:            "md1",
			Rid:                 "f",
		})
		require.NoError(t, err)

		// should not accept restricted label from !PA_1
		err = mbb.AddEventMetric(EventMetric{
			MetricLabel:         livekit.MetricLabel_CLIENT_VIDEO_PUBLISHER_QUALITY_LIMITATION_DURATION_OTHER,
			ParticipantIdentity: "PA_2",
			TrackID:             "TR_VC1",
			StartedAt:           at,
			EndedAt:             at,
			NormalizedStartedAt: normalizedAt,
			NormalizedEndedAt:   normalizedAt,
			Metadata:            "md1",
			Rid:                 "f",
		})
		require.ErrorIs(t, err, ErrFilteredMetricLabel)

		// add a time series metric to ensure both time series metric and event metric can be in same batch
		_, err = mbb.AddTimeSeriesMetric(TimeSeriesMetric{
			MetricLabel:         livekit.MetricLabel_SUBSCRIBER_RTT,
			ParticipantIdentity: "PA_2",
			Samples: []MetricSample{
				{
					At:           at,
					NormalizedAt: normalizedAt,
					Value:        102.4,
				},
			},
		})
		require.NoError(t, err)

		mb := mbb.ToProto()
		require.True(t, proto.Equal(expected, mb))
	})

	t.Run("merge", func(t *testing.T) {
		at := mono.Now()
		atMilli := at.UnixMilli()
		normalizedAt := mono.Now().Add(10 * time.Millisecond)

		expected := &livekit.MetricsBatch{
			TimestampMs:         at.UnixMilli(),
			NormalizedTimestamp: timestamppb.New(normalizedAt),
			StrData: []string{
				"PA_1",
				"TR_VC1",
				"f",
				"CustomMetric",
				"TR_VC2",
				"q",
				"PA_2",
			},
			Events: []*livekit.EventMetric{
				{
					Label:                    uint32(livekit.MetricLabel_PUBLISHER_RTT),
					ParticipantIdentity:      uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE),
					TrackSid:                 uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE) + 1,
					StartTimestampMs:         at.UnixMilli(),
					EndTimestampMs:           &atMilli,
					NormalizedStartTimestamp: timestamppb.New(normalizedAt),
					NormalizedEndTimestamp:   timestamppb.New(normalizedAt),
					Metadata:                 "md1",
					Rid:                      uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE) + 2,
				},
				{
					Label:                    uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE) + 3,
					ParticipantIdentity:      uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE),
					TrackSid:                 uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE) + 4,
					StartTimestampMs:         at.UnixMilli(),
					NormalizedStartTimestamp: timestamppb.New(normalizedAt),
					Metadata:                 "md2",
					Rid:                      uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE) + 5,
				},
				{
					Label:                    uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE) + 3,
					ParticipantIdentity:      uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE),
					TrackSid:                 uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE) + 4,
					StartTimestampMs:         at.UnixMilli(),
					NormalizedStartTimestamp: timestamppb.New(normalizedAt),
					Metadata:                 "md2",
					Rid:                      uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE) + 5,
				},
				{
					Label:                    uint32(livekit.MetricLabel_CLIENT_VIDEO_PUBLISHER_QUALITY_LIMITATION_DURATION_BANDWIDTH),
					ParticipantIdentity:      uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE),
					TrackSid:                 uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE) + 4,
					StartTimestampMs:         at.UnixMilli(),
					NormalizedStartTimestamp: timestamppb.New(normalizedAt),
					Metadata:                 "md2",
					Rid:                      uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE) + 5,
				},
			},
			TimeSeries: []*livekit.TimeSeriesMetric{
				{
					Label:               uint32(livekit.MetricLabel_SUBSCRIBER_RTT),
					ParticipantIdentity: uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE) + 6,
					Samples: []*livekit.MetricSample{
						{
							TimestampMs:         at.UnixMilli(),
							NormalizedTimestamp: timestamppb.New(normalizedAt),
							Value:               102.4,
						},
					},
				},
			},
		}

		mbb := NewMetricsBatchBuilder()
		mbb.SetTime(at, normalizedAt)
		mbb.SetRestrictedLabels(MetricRestrictedLabels{
			LabelRanges: []MetricLabelRange{
				{
					StartInclusive: livekit.MetricLabel_CLIENT_VIDEO_SUBSCRIBER_FREEZE_COUNT,
					EndInclusive:   livekit.MetricLabel_CLIENT_VIDEO_PUBLISHER_QUALITY_LIMITATION_DURATION_OTHER,
				},
			},
			ParticipantIdentity: "PA_1",
		})

		// should not be able to add invalid metric label index
		err := mbb.AddEventMetric(EventMetric{
			MetricLabel: livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE,
		})
		require.ErrorIs(t, err, ErrInvalidMetricLabel)

		// add an event metric
		err = mbb.AddEventMetric(EventMetric{
			MetricLabel:         livekit.MetricLabel_PUBLISHER_RTT,
			ParticipantIdentity: "PA_1",
			TrackID:             "TR_VC1",
			StartedAt:           at,
			EndedAt:             at,
			NormalizedStartedAt: normalizedAt,
			NormalizedEndedAt:   normalizedAt,
			Metadata:            "md1",
			Rid:                 "f",
		})
		require.NoError(t, err)

		// add a second one with some optional fields not included
		// including this here and in the one to merge to test index translation
		err = mbb.AddEventMetric(EventMetric{
			CustomMetricLabel:   "CustomMetric",
			ParticipantIdentity: "PA_1",
			TrackID:             "TR_VC2",
			StartedAt:           at,
			NormalizedStartedAt: normalizedAt,
			Metadata:            "md2",
			Rid:                 "q",
		})
		require.NoError(t, err)

		toMerge := &livekit.MetricsBatch{
			TimestampMs:         at.UnixMilli(),
			NormalizedTimestamp: timestamppb.New(normalizedAt),
			StrData: []string{
				"CustomMetric",
				"PA_1",
				"TR_VC2",
				"q",
				"PA_2",
			},
			Events: []*livekit.EventMetric{
				{
					Label:                    uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE),
					ParticipantIdentity:      uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE) + 1,
					TrackSid:                 uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE) + 2,
					StartTimestampMs:         at.UnixMilli(),
					NormalizedStartTimestamp: timestamppb.New(normalizedAt),
					Metadata:                 "md2",
					Rid:                      uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE) + 3,
				},
				{
					Label:                    uint32(livekit.MetricLabel_CLIENT_VIDEO_PUBLISHER_QUALITY_LIMITATION_DURATION_BANDWIDTH),
					ParticipantIdentity:      uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE) + 1,
					TrackSid:                 uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE) + 2,
					StartTimestampMs:         at.UnixMilli(),
					NormalizedStartTimestamp: timestamppb.New(normalizedAt),
					Metadata:                 "md2",
					Rid:                      uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE) + 3,
				},
			},
			TimeSeries: []*livekit.TimeSeriesMetric{
				{
					Label:               uint32(livekit.MetricLabel_SUBSCRIBER_RTT),
					ParticipantIdentity: uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE) + 4,
					Samples: []*livekit.MetricSample{
						{
							TimestampMs:         at.UnixMilli(),
							NormalizedTimestamp: timestamppb.New(normalizedAt),
							Value:               102.4,
						},
					},
				},
				{
					// should be filtered
					Label:               uint32(livekit.MetricLabel_CLIENT_VIDEO_PUBLISHER_QUALITY_LIMITATION_DURATION_CPU),
					ParticipantIdentity: uint32(livekit.MetricLabel_METRIC_LABEL_PREDEFINED_MAX_VALUE) + 4,
					Samples: []*livekit.MetricSample{
						{
							TimestampMs:         at.UnixMilli(),
							NormalizedTimestamp: timestamppb.New(normalizedAt),
							Value:               102.4,
						},
					},
				},
			},
		}
		mbb.Merge(toMerge)

		mb := mbb.ToProto()
		require.True(t, proto.Equal(expected, mb))
	})
}
