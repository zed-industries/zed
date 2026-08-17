// Code generated; DO NOT EDIT.
package agentsobs

type WorkerStatus string

const (
	WorkerStatusUndefined WorkerStatus = ""
	WorkerStatusAvailable WorkerStatus = "available"
	WorkerStatusFull      WorkerStatus = "full"
)

type WorkerState string

const (
	WorkerStateUndefined WorkerState = ""
	WorkerStateOnline    WorkerState = "online"
	WorkerStateOffline   WorkerState = "offline"
)

type JobKind string

const (
	JobKindUndefined   JobKind = ""
	JobKindRoom        JobKind = "room"
	JobKindPublisher   JobKind = "publisher"
	JobKindParticipant JobKind = "participant"
)

type WorkerKind string

const (
	WorkerKindUndefined WorkerKind = ""
	WorkerKindCloud     WorkerKind = "cloud"
	WorkerKindSelfhost  WorkerKind = "selfhost"
)

type JobStatus string

const (
	JobStatusUndefined JobStatus = ""
	JobStatusPending   JobStatus = "pending"
	JobStatusRunning   JobStatus = "running"
	JobStatusSuccess   JobStatus = "success"
	JobStatusFailed    JobStatus = "failed"
)

type Rollup string

const (
	RollupUndefined    Rollup = ""
	RollupAgent        Rollup = "agent"
	RollupWorker       Rollup = "worker"
	RollupWorkerSeries Rollup = "worker_series"
	RollupJob          Rollup = "job"
)
