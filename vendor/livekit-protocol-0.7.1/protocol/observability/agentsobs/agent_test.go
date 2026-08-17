package agentsobs

import (
	"testing"

	"github.com/livekit/protocol/livekit"
	"github.com/stretchr/testify/require"
)

func TestJobKindFromProto(t *testing.T) {
	tests := []struct {
		input    livekit.JobType
		expected JobKind
	}{
		{livekit.JobType_JT_ROOM, JobKindRoom},
		{livekit.JobType_JT_PUBLISHER, JobKindPublisher},
		{livekit.JobType_JT_PARTICIPANT, JobKindParticipant},
		{livekit.JobType(999), JobKindUndefined}, // Undefined case
	}

	for _, test := range tests {
		result := JobKindFromProto(test.input)
		require.Equal(t, test.expected, result)
	}
}

func TestJobStatusFromProto(t *testing.T) {
	tests := []struct {
		input    livekit.JobStatus
		expected JobStatus
	}{
		{livekit.JobStatus_JS_PENDING, JobStatusPending},
		{livekit.JobStatus_JS_RUNNING, JobStatusRunning},
		{livekit.JobStatus_JS_SUCCESS, JobStatusSuccess},
		{livekit.JobStatus_JS_FAILED, JobStatusFailed},
		{livekit.JobStatus(999), JobStatusUndefined}, // Undefined case
	}

	for _, test := range tests {
		result := JobStatusFromProto(test.input)
		require.Equal(t, test.expected, result)
	}
}

func TestWorkerStatusFromProto(t *testing.T) {
	tests := []struct {
		input    livekit.WorkerStatus
		expected WorkerStatus
	}{
		{livekit.WorkerStatus_WS_AVAILABLE, WorkerStatusAvailable},
		{livekit.WorkerStatus_WS_FULL, WorkerStatusFull},
		{livekit.WorkerStatus(999), WorkerStatusUndefined}, // Undefined case
	}

	for _, test := range tests {
		result := WorkerStatusFromProto(test.input)
		require.Equal(t, test.expected, result)
	}
}
