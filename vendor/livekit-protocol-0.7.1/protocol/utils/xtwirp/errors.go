package xtwirp

import (
	"context"
	"encoding/base64"
	"errors"

	"github.com/twitchtv/twirp"
	spb "google.golang.org/genproto/googleapis/rpc/status"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
	"google.golang.org/protobuf/proto"
)

// DetailsMetaKey is a Twirp error metadata key that is used to pass protobuf-encoded error details.
const DetailsMetaKey = "error_details"

// ErrorMeta is an optional interface that allows attaching Twirp error metadata.
type ErrorMeta interface {
	TwirpErrorMeta() map[string]string
}

// ErrorCodeFromGRPC converts gRPC error code to Twirp.
func ErrorCodeFromGRPC(code codes.Code) twirp.ErrorCode {
	switch code {
	case codes.OK:
		return twirp.NoError
	case codes.Canceled:
		return twirp.Canceled
	case codes.Unknown:
		return twirp.Unknown
	case codes.InvalidArgument:
		return twirp.InvalidArgument
	case codes.DeadlineExceeded:
		return twirp.DeadlineExceeded
	case codes.NotFound:
		return twirp.NotFound
	case codes.AlreadyExists:
		return twirp.AlreadyExists
	case codes.PermissionDenied:
		return twirp.PermissionDenied
	case codes.ResourceExhausted:
		return twirp.ResourceExhausted
	case codes.FailedPrecondition:
		return twirp.FailedPrecondition
	case codes.Aborted:
		return twirp.Aborted
	case codes.OutOfRange:
		return twirp.OutOfRange
	case codes.Unimplemented:
		return twirp.Unimplemented
	case codes.Internal:
		return twirp.Internal
	case codes.Unavailable:
		return twirp.Unavailable
	case codes.DataLoss:
		return twirp.DataLoss
	case codes.Unauthenticated:
		return twirp.Unauthenticated
	default:
		return twirp.Unknown
	}
}

// WithDetailsFrom sets gRPC/PSRPC error details from src as Twirp error metadata on dst.
//
// If error details implement ErrorMeta, their custom metadata fields will be included as well.
func WithDetailsFrom(dst twirp.Error, src error) twirp.Error {
	st, ok := status.FromError(src)
	if !ok {
		return dst
	}
	if dst.Code() == twirp.Unknown {
		dst = twirp.NewError(ErrorCodeFromGRPC(st.Code()), dst.Error())
	}
	return WithDetailsFromStatus(dst, st)
}

// ToError converts any error to Twirp error, preserving error details.
func ToError(err error) twirp.Error {
	if e, ok := err.(twirp.Error); ok {
		return e
	}
	e := twirp.NewError(twirp.Unknown, err.Error())
	e = WithDetailsFrom(e, err)
	return e
}

// WithDetailsFromStatus sets gRPC error details from status as Twirp error metadata on dst.
//
// If error details implement ErrorMeta, their custom metadata fields will be included as well.
func WithDetailsFromStatus(dst twirp.Error, st *status.Status) twirp.Error {
	if st == nil {
		return dst
	}
	details := st.Details()
	if len(details) == 0 {
		return dst
	}
	for _, d := range details {
		if m, ok := d.(ErrorMeta); ok {
			for k, v := range m.TwirpErrorMeta() {
				dst = dst.WithMeta(k, v)
			}
		}
	}
	p := st.Proto()
	if len(p.Details) == 0 {
		return dst
	}
	data, err := proto.Marshal(p)
	if err != nil {
		return dst
	}
	val := base64.StdEncoding.EncodeToString(data)
	return dst.WithMeta(DetailsMetaKey, val)
}

// StatusFromError is an analog of gRPCs status.FromError, but it also considers
// error details encoded in Twirp metadata.
func StatusFromError(err error) (*status.Status, bool) {
	if st, ok := status.FromError(err); ok {
		return st, true
	}
	var e twirp.Error
	if !errors.As(err, &e) {
		return nil, false
	}
	val := e.Meta(DetailsMetaKey)
	if val == "" {
		return nil, false
	}
	data, err := base64.StdEncoding.DecodeString(val)
	if err != nil {
		return nil, false
	}
	var p spb.Status
	if err := proto.Unmarshal(data, &p); err != nil {
		return nil, false
	}
	return status.FromProto(&p), true
}

// ClientPassErrorDetails converts Twirp errors to gRPC errors, if possible.
// This allows passing error details returned via gRPC/PSRPC by the backend server.
func ClientPassErrorDetails() twirp.ClientOption {
	return twirp.WithClientInterceptors(func(fnc twirp.Method) twirp.Method {
		return func(ctx context.Context, req any) (any, error) {
			resp, err := fnc(ctx, req)
			if err != nil {
				if st, ok := StatusFromError(err); ok && st != nil {
					err = st.Err()
				}
			}
			return resp, err
		}
	})
}

// ServerPassErrorDetails converts gRPC errors to Twirp errors.
// It properly converts gRPC/PSRPC error codes and preserves custom error details.
func ServerPassErrorDetails() twirp.ServerOption {
	return twirp.WithServerInterceptors(func(fnc twirp.Method) twirp.Method {
		return func(ctx context.Context, req any) (any, error) {
			resp, err := fnc(ctx, req)
			if err != nil {
				err = ToError(err)
			}
			return resp, err
		}
	})
}
