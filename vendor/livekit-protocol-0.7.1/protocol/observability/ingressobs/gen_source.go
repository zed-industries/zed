// Code generated; DO NOT EDIT.
package ingressobs

type SessionInputType string

const (
	SessionInputTypeUndefined SessionInputType = ""
	SessionInputTypeRtmp      SessionInputType = "rtmp"
	SessionInputTypeWhip      SessionInputType = "whip"
	SessionInputTypeURL       SessionInputType = "url"
)

type SessionStatus string

const (
	SessionStatusUndefined  SessionStatus = ""
	SessionStatusInactive   SessionStatus = "inactive"
	SessionStatusBuffering  SessionStatus = "buffering"
	SessionStatusPublishing SessionStatus = "publishing"
	SessionStatusError      SessionStatus = "error"
	SessionStatusComplete   SessionStatus = "complete"
)

type Rollup string

const (
	RollupUndefined      Rollup = ""
	RollupProject        Rollup = "project"
	RollupIngress        Rollup = "ingress"
	RollupSessionIndex   Rollup = "session_index"
	RollupStartTimeIndex Rollup = "start_time_index"
	RollupEndTimeIndex   Rollup = "end_time_index"
)
