package agentsobs

import "github.com/livekit/protocol/livekit"

func JobKindFromProto(kind livekit.JobType) JobKind {
	switch kind {
	case livekit.JobType_JT_ROOM:
		return JobKindRoom
	case livekit.JobType_JT_PUBLISHER:
		return JobKindPublisher
	case livekit.JobType_JT_PARTICIPANT:
		return JobKindParticipant
	default:
		return JobKindUndefined
	}
}

func JobStatusFromProto(status livekit.JobStatus) JobStatus {
	switch status {
	case livekit.JobStatus_JS_PENDING:
		return JobStatusPending
	case livekit.JobStatus_JS_RUNNING:
		return JobStatusRunning
	case livekit.JobStatus_JS_SUCCESS:
		return JobStatusSuccess
	case livekit.JobStatus_JS_FAILED:
		return JobStatusFailed
	default:
		return JobStatusUndefined
	}
}

func WorkerStatusFromProto(status livekit.WorkerStatus) WorkerStatus {
	switch status {
	case livekit.WorkerStatus_WS_AVAILABLE:
		return WorkerStatusAvailable
	case livekit.WorkerStatus_WS_FULL:
		return WorkerStatusFull
	default:
		return WorkerStatusUndefined
	}
}
