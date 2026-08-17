// Code generated; DO NOT EDIT.

package ingressobs

import (
	"time"
)

const Version_UTO6LDG = true

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
	WithIngress(id string) IngressReporter
	WithDeferredIngress() (IngressReporter, KeyResolver)
	ProjectTx
}

type IngressTx interface{}

type IngressReporter interface {
	RegisterFunc(func(ts time.Time, tx IngressTx) bool)
	Tx(func(tx IngressTx))
	TxAt(time.Time, func(tx IngressTx))
	WithSession(id string) SessionReporter
	WithDeferredSession() (SessionReporter, KeyResolver)
	IngressTx
}

type SessionTx interface {
	ReportStartTime(v time.Time)
	ReportEndTime(v time.Time)
	ReportDuration(v uint64)
	ReportInputType(v SessionInputType)
	ReportRegion(v string)
	ReportRoomName(v string)
	ReportRoomID(v string)
	ReportError(v string)
	ReportStatus(v SessionStatus)
	ReportAudioOnly(v bool)
	ReportTranscoded(v bool)
	ReportReusable(v bool)
}

type SessionReporter interface {
	RegisterFunc(func(ts time.Time, tx SessionTx) bool)
	Tx(func(tx SessionTx))
	TxAt(time.Time, func(tx SessionTx))
	SessionTx
}
