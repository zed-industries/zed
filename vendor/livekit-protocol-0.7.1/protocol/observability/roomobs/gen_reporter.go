// Code generated; DO NOT EDIT.

package roomobs

import (
	"time"
)

const Version_PPAKM9O = true

type KeyResolver interface {
	Resolve(string)
	Reset()
}

type Reporter interface {
	WithProject(id string) ProjectReporter
	WithDeferredProject() (ProjectReporter, KeyResolver)
}

type ProjectTx interface{}

type ProjectReporter interface {
	RegisterFunc(func(ts time.Time, tx ProjectTx) bool)
	Tx(func(tx ProjectTx))
	TxAt(time.Time, func(tx ProjectTx))
	WithRoom(name string) RoomReporter
	WithDeferredRoom() (RoomReporter, KeyResolver)
	ProjectTx
}

type RoomTx interface{}

type RoomReporter interface {
	RegisterFunc(func(ts time.Time, tx RoomTx) bool)
	Tx(func(tx RoomTx))
	TxAt(time.Time, func(tx RoomTx))
	WithRoomSession(id string) RoomSessionReporter
	WithDeferredRoomSession() (RoomSessionReporter, KeyResolver)
	RoomTx
}

type RoomSessionTx interface {
	ReportStartTime(v time.Time)
	ReportEndTime(v time.Time)
	ReportFeatures(v uint16)
	ReportRoomDuration(v uint32)
}

type RoomSessionReporter interface {
	RegisterFunc(func(ts time.Time, tx RoomSessionTx) bool)
	Tx(func(tx RoomSessionTx))
	TxAt(time.Time, func(tx RoomSessionTx))
	WithParticipant(identity string) ParticipantReporter
	WithDeferredParticipant() (ParticipantReporter, KeyResolver)
	RoomSessionTx
}

type ParticipantTx interface{}

type ParticipantReporter interface {
	RegisterFunc(func(ts time.Time, tx ParticipantTx) bool)
	Tx(func(tx ParticipantTx))
	TxAt(time.Time, func(tx ParticipantTx))
	WithParticipantSession(id string) ParticipantSessionReporter
	WithDeferredParticipantSession() (ParticipantSessionReporter, KeyResolver)
	ParticipantTx
}

type ParticipantSessionTx interface {
	ReportRegion(v string)
	ReportClientConnectTime(v uint16)
	ReportConnectResult(v ConnectionResult)
	ReportConnectionType(v ConnectionType)
	ReportOs(v ClientOS)
	ReportDeviceModel(v string)
	ReportBrowser(v string)
	ReportSdkVersion(v string)
	ReportCountry(v uint16)
	ReportIspAsn(v uint32)
	ReportStartTime(v time.Time)
	ReportEndTime(v time.Time)
	ReportDuration(v uint16)
	ReportDurationMinutes(v uint8)
	ReportKind(v string)
	ReportName(v string)
	ReportFeatures(v uint16)
}

type ParticipantSessionReporter interface {
	RegisterFunc(func(ts time.Time, tx ParticipantSessionTx) bool)
	Tx(func(tx ParticipantSessionTx))
	TxAt(time.Time, func(tx ParticipantSessionTx))
	WithTrack(id string) TrackReporter
	WithDeferredTrack() (TrackReporter, KeyResolver)
	ParticipantSessionTx
}

type TrackTx interface {
	ReportName(v string)
	ReportKind(v TrackKind)
	ReportType(v TrackType)
	ReportSource(v TrackSource)
	ReportMime(v MimeType)
	ReportLayer(v uint32)
	ReportDuration(v uint16)
	ReportFrames(v uint16)
	ReportSendBytes(v uint32)
	ReportRecvBytes(v uint32)
	ReportSendPackets(v uint32)
	ReportRecvPackets(v uint32)
	ReportPacketsLost(v uint32)
	ReportScore(v float32)
}

type TrackReporter interface {
	RegisterFunc(func(ts time.Time, tx TrackTx) bool)
	Tx(func(tx TrackTx))
	TxAt(time.Time, func(tx TrackTx))
	TrackTx
}
