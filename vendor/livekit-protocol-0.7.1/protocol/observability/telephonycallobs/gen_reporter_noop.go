// Code generated; DO NOT EDIT.

package telephonycallobs

import (
	"time"
)

var (
	_ Reporter        = (*noopReporter)(nil)
	_ ProjectReporter = (*noopProjectReporter)(nil)
	_ CallReporter    = (*noopCallReporter)(nil)
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
func (r *noopProjectReporter) WithCall(id string) CallReporter {
	return &noopCallReporter{}
}
func (r *noopProjectReporter) WithDeferredCall() (CallReporter, KeyResolver) {
	return &noopCallReporter{}, noopKeyResolver{}
}

type noopCallReporter struct{}

func NewNoopCallReporter() CallReporter {
	return &noopCallReporter{}
}

func (r *noopCallReporter) RegisterFunc(f func(ts time.Time, tx CallTx) bool)           {}
func (r *noopCallReporter) Tx(f func(CallTx))                                           {}
func (r *noopCallReporter) TxAt(ts time.Time, f func(CallTx))                           {}
func (r *noopCallReporter) ReportStartTime(v time.Time)                                 {}
func (r *noopCallReporter) ReportEndTime(v time.Time)                                   {}
func (r *noopCallReporter) ReportDuration(v uint64)                                     {}
func (r *noopCallReporter) ReportDurationMinutes(v uint16)                              {}
func (r *noopCallReporter) ReportTrunkID(v string)                                      {}
func (r *noopCallReporter) ReportDispatchID(v string)                                   {}
func (r *noopCallReporter) ReportToNumber(v string)                                     {}
func (r *noopCallReporter) ReportToHost(v string)                                       {}
func (r *noopCallReporter) ReportFromNumber(v string)                                   {}
func (r *noopCallReporter) ReportFromHost(v string)                                     {}
func (r *noopCallReporter) ReportDirection(v CallDirection)                             {}
func (r *noopCallReporter) ReportTransport(v CallTransport)                             {}
func (r *noopCallReporter) ReportProviderCallID(v string)                               {}
func (r *noopCallReporter) ReportProviderName(v string)                                 {}
func (r *noopCallReporter) ReportSIPCallID(v string)                                    {}
func (r *noopCallReporter) ReportRoomID(v string)                                       {}
func (r *noopCallReporter) ReportRoomName(v string)                                     {}
func (r *noopCallReporter) ReportParticipantIdentity(v string)                          {}
func (r *noopCallReporter) ReportError(v string)                                        {}
func (r *noopCallReporter) ReportStatus(v CallStatus)                                   {}
func (r *noopCallReporter) ReportResponseCode(v uint16)                                 {}
func (r *noopCallReporter) ReportDisconnectReason(v string)                             {}
func (r *noopCallReporter) ReportTransferID(v string)                                   {}
func (r *noopCallReporter) ReportTransferTo(v string)                                   {}
func (r *noopCallReporter) ReportTransferDuration(v uint32)                             {}
func (r *noopCallReporter) ReportTransferStatus(v CallTransferStatus)                   {}
func (r *noopCallReporter) ReportTransferStatusCode(v uint16)                           {}
func (r *noopCallReporter) ReportTransferError(v string)                                {}
func (r *noopCallReporter) ReportCodec(v string)                                        {}
func (r *noopCallReporter) ReportRegion(v string)                                       {}
func (r *noopCallReporter) ReportPcapLink(v string)                                     {}
func (r *noopCallReporter) ReportAttributes(v string)                                   {}
func (r *noopCallReporter) ReportFeatures(v uint16)                                     {}
func (r *noopCallReporter) ReportMediaEncryptionSettings(v CallMediaEncryptionSettings) {}
func (r *noopCallReporter) ReportMediaEncryption(v string)                              {}
