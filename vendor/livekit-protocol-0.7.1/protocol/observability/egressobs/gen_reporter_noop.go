// Code generated; DO NOT EDIT.

package egressobs

import (
	"time"
)

var (
	_ Reporter        = (*noopReporter)(nil)
	_ ProjectReporter = (*noopProjectReporter)(nil)
	_ EgressReporter  = (*noopEgressReporter)(nil)
)

type noopKeyResolver struct{}

func (noopKeyResolver) Resolve(string) {}
func (noopKeyResolver) Reset()         {}

type noopReporter struct{}

func NewNoopReporter() Reporter {
	return &noopReporter{}
}

func (r *noopReporter) WithProject(id string) ProjectReporter {
	return &noopProjectReporter{}
}

func (r *noopReporter) WithDeferredProject() (ProjectReporter, KeyResolver) {
	return &noopProjectReporter{}, noopKeyResolver{}
}

type noopProjectReporter struct{}

func NewNoopProjectReporter() ProjectReporter {
	return &noopProjectReporter{}
}

func (r *noopProjectReporter) RegisterFunc(f func(ts time.Time, tx ProjectTx) bool) {}
func (r *noopProjectReporter) Tx(f func(ProjectTx))                                 {}
func (r *noopProjectReporter) TxAt(ts time.Time, f func(ProjectTx))                 {}
func (r *noopProjectReporter) WithEgress(id string) EgressReporter {
	return &noopEgressReporter{}
}
func (r *noopProjectReporter) WithDeferredEgress() (EgressReporter, KeyResolver) {
	return &noopEgressReporter{}, noopKeyResolver{}
}

type noopEgressReporter struct{}

func NewNoopEgressReporter() EgressReporter {
	return &noopEgressReporter{}
}

func (r *noopEgressReporter) RegisterFunc(f func(ts time.Time, tx EgressTx) bool) {}
func (r *noopEgressReporter) Tx(f func(EgressTx))                                 {}
func (r *noopEgressReporter) TxAt(ts time.Time, f func(EgressTx))                 {}
func (r *noopEgressReporter) ReportStartTime(v time.Time)                         {}
func (r *noopEgressReporter) ReportEndTime(v time.Time)                           {}
func (r *noopEgressReporter) ReportUpdateTime(v time.Time)                        {}
func (r *noopEgressReporter) ReportDuration(v uint64)                             {}
func (r *noopEgressReporter) ReportRequestType(v EgressRequestType)               {}
func (r *noopEgressReporter) ReportSourceType(v EgressSourceType)                 {}
func (r *noopEgressReporter) ReportRegion(v string)                               {}
func (r *noopEgressReporter) ReportRoomName(v string)                             {}
func (r *noopEgressReporter) ReportRoomID(v string)                               {}
func (r *noopEgressReporter) ReportStatus(v EgressStatus)                         {}
func (r *noopEgressReporter) ReportDetails(v string)                              {}
func (r *noopEgressReporter) ReportError(v string)                                {}
func (r *noopEgressReporter) ReportErrorCode(v int32)                             {}
func (r *noopEgressReporter) ReportManifestLocation(v string)                     {}
func (r *noopEgressReporter) ReportBackupStorageUsed(v bool)                      {}
func (r *noopEgressReporter) ReportResult(v string)                               {}
func (r *noopEgressReporter) ReportRequest(v string)                              {}
func (r *noopEgressReporter) ReportAudioOnly(v bool)                              {}
