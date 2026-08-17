package utils

import (
	"fmt"
	"time"

	"github.com/livekit/psrpc"
)

type DataChannelRpcErrorCode uint32

const (
	DataChannelRpcApplicationError DataChannelRpcErrorCode = 1500 + iota
	DataChannelRpcConnectionTimeout
	DataChannelRpcResponseTimeout
	DataChannelRpcRecipientDisconnected
	DataChannelRpcResponsePayloadTooLarge
	DataChannelRpcSendFailed
)

const (
	DataChannelRpcUnsupportedMethod DataChannelRpcErrorCode = 1400 + iota
	DataChannelRpcRecipientNotFound
	DataChannelRpcRequestPayloadTooLarge
	DataChannelRpcUnsupportedServer
	DataChannelRpcUnsupportedVersion
)

const (
	DataChannelRpcMaxRoundTripLatency    = 2000 * time.Millisecond
	DataChannelRpcMaxMessageBytes        = 256
	DataChannelRpcMaxDataBytes           = 15360 // 15KiB
	DataChannelRpcMaxPayloadBytes        = 15360 // 15KiB
	DataChannelRpcDefaultResponseTimeout = 10000 * time.Millisecond
)

var dataChannelRpcErrorMessages = map[DataChannelRpcErrorCode]string{
	DataChannelRpcApplicationError:        "Application error in method handler",
	DataChannelRpcConnectionTimeout:       "Connection timeout",
	DataChannelRpcResponseTimeout:         "Response timeout",
	DataChannelRpcRecipientDisconnected:   "Recipient disconnected",
	DataChannelRpcResponsePayloadTooLarge: "Response payload too large",
	DataChannelRpcSendFailed:              "Failed to send",

	DataChannelRpcUnsupportedMethod:      "Method not supported at destination",
	DataChannelRpcRecipientNotFound:      "Recipient not found",
	DataChannelRpcRequestPayloadTooLarge: "Request payload too large",
	DataChannelRpcUnsupportedServer:      "RPC not supported by server",
	DataChannelRpcUnsupportedVersion:     "Unsupported RPC version",
}

type DataChannelRpcError struct {
	Code    DataChannelRpcErrorCode
	Message string
	Data    string
}

func (e *DataChannelRpcError) Error() string {
	if e.Data != "" {
		return fmt.Sprintf("RpcError %d: %s: %s", e.Code, e.Message, e.Data)
	}
	return fmt.Sprintf("RpcError %d: %s", e.Code, e.Message)
}

func (e *DataChannelRpcError) PsrpcError() psrpc.Error {
	switch e.Code {
	case DataChannelRpcApplicationError:
		return psrpc.NewError(psrpc.Internal, e)
	case DataChannelRpcConnectionTimeout:
		return psrpc.NewError(psrpc.Canceled, e)
	case DataChannelRpcResponseTimeout:
		return psrpc.NewError(psrpc.Canceled, e)
	case DataChannelRpcRecipientDisconnected:
		return psrpc.NewError(psrpc.Unavailable, e)
	case DataChannelRpcResponsePayloadTooLarge:
		return psrpc.NewError(psrpc.MalformedResponse, e)
	case DataChannelRpcSendFailed:
		return psrpc.NewError(psrpc.Internal, e)
	case DataChannelRpcUnsupportedMethod:
		return psrpc.NewError(psrpc.InvalidArgument, e)
	case DataChannelRpcRecipientNotFound:
		return psrpc.NewError(psrpc.NotFound, e)
	case DataChannelRpcRequestPayloadTooLarge:
		return psrpc.NewError(psrpc.MalformedRequest, e)
	case DataChannelRpcUnsupportedServer:
		return psrpc.NewError(psrpc.Unimplemented, e)
	case DataChannelRpcUnsupportedVersion:
		return psrpc.NewError(psrpc.Unimplemented, e)
	default:
		return psrpc.NewError(psrpc.Internal, e)
	}
}

func DataChannelRpcErrorFromBuiltInCodes(code DataChannelRpcErrorCode, data string) *DataChannelRpcError {
	return &DataChannelRpcError{Code: code, Message: dataChannelRpcErrorMessages[code], Data: data}
}

type DataChannelRpcPendingAckHandler struct {
	Resolve             func()
	ParticipantIdentity string
}

type DataChannelRpcPendingResponseHandler struct {
	Resolve             func(payload string, err *DataChannelRpcError)
	ParticipantIdentity string
}
