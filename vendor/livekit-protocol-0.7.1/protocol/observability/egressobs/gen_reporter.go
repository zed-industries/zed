// Code generated; DO NOT EDIT.

package egressobs

import (
	"time"
)

const Version_57DK1I8 = true

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
	WithEgress(id string) EgressReporter
	WithDeferredEgress() (EgressReporter, KeyResolver)
	ProjectTx
}

type EgressTx interface {
	ReportStartTime(v time.Time)
	ReportEndTime(v time.Time)
	ReportUpdateTime(v time.Time)
	ReportDuration(v uint64)
	ReportRequestType(v EgressRequestType)
	ReportSourceType(v EgressSourceType)
	ReportRegion(v string)
	ReportRoomName(v string)
	ReportRoomID(v string)
	ReportStatus(v EgressStatus)
	ReportDetails(v string)
	ReportError(v string)
	ReportErrorCode(v int32)
	ReportManifestLocation(v string)
	ReportBackupStorageUsed(v bool)
	ReportResult(v string)
	ReportRequest(v string)
	ReportAudioOnly(v bool)
}

type EgressReporter interface {
	RegisterFunc(func(ts time.Time, tx EgressTx) bool)
	Tx(func(tx EgressTx))
	TxAt(time.Time, func(tx EgressTx))
	EgressTx
}
