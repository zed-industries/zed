package xtwirp_test

import (
	"testing"

	"github.com/stretchr/testify/require"
	"github.com/twitchtv/twirp"
	"google.golang.org/genproto/googleapis/rpc/errdetails"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"

	"github.com/livekit/protocol/utils/xtwirp"
)

func TestStatus(t *testing.T) {
	st := status.New(codes.FailedPrecondition, "test")
	st, err := st.WithDetails(&errdetails.ErrorInfo{Reason: "reason"})
	require.NoError(t, err)

	e := twirp.NewError(twirp.InvalidArgument, "twirp")
	e = xtwirp.WithDetailsFromStatus(e, st)

	got, ok := xtwirp.StatusFromError(e)
	require.True(t, ok)
	require.Equal(t, st, got)
	require.Equal(t, st.Details(), got.Details())
}
