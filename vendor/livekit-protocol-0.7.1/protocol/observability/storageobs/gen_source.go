// Code generated; DO NOT EDIT.
package storageobs

type EventService string

const (
	EventServiceUndefined EventService = ""
	EventServiceEgress    EventService = "egress"
	EventServiceReplay    EventService = "replay"
)

type EventOperation string

const (
	EventOperationUndefined EventOperation = ""
	EventOperationUpload    EventOperation = "upload"
	EventOperationDownload  EventOperation = "download"
)

type Rollup string

const (
	RollupUndefined Rollup = ""
	RollupProject   Rollup = "project"
)
