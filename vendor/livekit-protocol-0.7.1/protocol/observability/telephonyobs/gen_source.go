// Code generated; DO NOT EDIT.
package telephonyobs

type DirectionType string

const (
	DirectionTypeUndefined DirectionType = ""
	DirectionTypeInbound   DirectionType = "inbound"
	DirectionTypeOutbound  DirectionType = "outbound"
)

type NumberType string

const (
	NumberTypeUndefined NumberType = ""
	NumberTypeTollFree  NumberType = "toll_free"
	NumberTypeRegular   NumberType = "regular"
)

type CallStatus string

const (
	CallStatusUndefined CallStatus = ""
	CallStatusAnswered  CallStatus = "answered"
	CallStatusMissed    CallStatus = "missed"
	CallStatusBusy      CallStatus = "busy"
	CallStatusFailed    CallStatus = "failed"
)

type TrunkType string

const (
	TrunkTypeUndefined TrunkType = ""
	TrunkTypeInternal  TrunkType = "internal"
	TrunkTypeExternal  TrunkType = "external"
)

type Rollup string

const (
	RollupUndefined Rollup = ""
	RollupPhone     Rollup = "phone"
	RollupCall      Rollup = "call"
)
