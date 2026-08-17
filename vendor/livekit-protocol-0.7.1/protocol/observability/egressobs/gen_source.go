// Code generated; DO NOT EDIT.
package egressobs

type EgressRequestType string

const (
	EgressRequestTypeUndefined      EgressRequestType = ""
	EgressRequestTypeRoomComposite  EgressRequestType = "room_composite"
	EgressRequestTypeTrackComposite EgressRequestType = "track_composite"
	EgressRequestTypeTrack          EgressRequestType = "track"
	EgressRequestTypeParticipant    EgressRequestType = "participant"
	EgressRequestTypeWeb            EgressRequestType = "web"
)

type EgressSourceType string

const (
	EgressSourceTypeUndefined EgressSourceType = ""
	EgressSourceTypeSdk       EgressSourceType = "sdk"
	EgressSourceTypeWeb       EgressSourceType = "web"
)

type EgressStatus string

const (
	EgressStatusUndefined    EgressStatus = ""
	EgressStatusStarting     EgressStatus = "starting"
	EgressStatusActive       EgressStatus = "active"
	EgressStatusEnding       EgressStatus = "ending"
	EgressStatusComplete     EgressStatus = "complete"
	EgressStatusFailed       EgressStatus = "failed"
	EgressStatusAborted      EgressStatus = "aborted"
	EgressStatusLimitReached EgressStatus = "limit_reached"
)

type Rollup string

const (
	RollupUndefined      Rollup = ""
	RollupProject        Rollup = "project"
	RollupEgressIndex    Rollup = "egress_index"
	RollupEndTimeIndex   Rollup = "end_time_index"
	RollupStartTimeIndex Rollup = "start_time_index"
	RollupRoomNameIndex  Rollup = "room_name_index"
)
