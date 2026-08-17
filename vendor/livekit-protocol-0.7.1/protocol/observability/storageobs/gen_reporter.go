// Code generated; DO NOT EDIT.

package storageobs

import (
	"time"
)

const Version_AP8D1RO = true

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
	WithEvent(id string) EventReporter
	WithDeferredEvent() (EventReporter, KeyResolver)
	ProjectTx
}

type EventTx interface {
	ReportService(v EventService)
	ReportServiceID(v string)
	ReportOperation(v EventOperation)
	ReportPath(v string)
	ReportSize(v uint64)
	ReportLifetime(v uint64)
}

type EventReporter interface {
	RegisterFunc(func(ts time.Time, tx EventTx) bool)
	Tx(func(tx EventTx))
	TxAt(time.Time, func(tx EventTx))
	EventTx
}
