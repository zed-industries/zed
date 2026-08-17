package livekit

import (
	"errors"
	"fmt"
	"net/http"
	"time"

	"github.com/livekit/psrpc"
)

const (
	MsgLimitReached             = "Session limit reached"
	MsgStartNotReceived         = "Start signal not received"
	MsgLimitReachedWithoutStart = "Session limit reached before start signal"
	MsgStoppedBeforeStarted     = "Stop called before pipeline could start"

	EndReasonAPI            = "StopEgress API"
	EndReasonKilled         = "Process killed"
	EndReasonSrcClosed      = "Source closed"
	EndReasonLimitReached   = "Session limit reached"
	EndReasonStreamsStopped = "All streams stopped"
	EndReasonFailure        = "Failure"
)

func (e *EgressInfo) UpdateStatus(status EgressStatus) {
	e.Status = status
	e.UpdatedAt = time.Now().UnixNano()
}

func (e *EgressInfo) SetBackupUsed() {
	e.BackupStorageUsed = true
	e.UpdatedAt = time.Now().UnixNano()
}

func (e *EgressInfo) SetEndReason(reason string) {
	e.Details = fmt.Sprintf("End reason: %s", reason)
}

func (e *EgressInfo) SetLimitReached() {
	now := time.Now().UnixNano()
	e.Status = EgressStatus_EGRESS_LIMIT_REACHED
	e.Error = MsgLimitReached
	e.ErrorCode = int32(http.StatusRequestEntityTooLarge)
	e.UpdatedAt = now
	e.EndedAt = now
}

func (e *EgressInfo) SetAborted(msg string) {
	now := time.Now().UnixNano()
	e.Status = EgressStatus_EGRESS_ABORTED
	e.Error = msg
	e.ErrorCode = int32(http.StatusPreconditionFailed)
	e.UpdatedAt = now
	e.EndedAt = now
}

func (e *EgressInfo) SetFailed(err error) {
	now := time.Now().UnixNano()
	e.Status = EgressStatus_EGRESS_FAILED
	e.UpdatedAt = now
	e.EndedAt = now
	e.Error = err.Error()
	if e.Details == "" {
		e.SetEndReason(EndReasonFailure)
	}

	var p psrpc.Error
	if errors.As(err, &p) {
		// unknown is treated the same as an internal error (500)
		if !errors.Is(p.Code(), psrpc.Unknown) {
			e.ErrorCode = int32(p.ToHttp())
		}
	}
}

func (e *EgressInfo) SetComplete() {
	now := time.Now().UnixNano()
	e.Status = EgressStatus_EGRESS_COMPLETE
	e.UpdatedAt = now
	e.EndedAt = now
}
