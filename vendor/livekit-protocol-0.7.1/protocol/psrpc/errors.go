// Copyright 2023 LiveKit, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package psrpc

import (
	"context"

	"google.golang.org/protobuf/proto"

	"github.com/livekit/protocol/utils"
	"github.com/livekit/psrpc"
)

func WithSuppressClientErrors(errs ...error) psrpc.ClientOption {
	return psrpc.WithClientOptions(
		psrpc.WithClientRPCInterceptors(newClientRPCErrorInterceptor(errs...)),
		psrpc.WithClientMultiRPCInterceptors(newMultiRPCErorrInterceptor(errs...)),
	)
}

func WithSuppressServerErrors(errs ...error) psrpc.ServerOption {
	return psrpc.WithServerOptions(
		psrpc.WithServerRPCInterceptors(newServerRPCErorrInterceptor(errs...)),
	)
}

func newClientRPCErrorInterceptor(errs ...error) psrpc.ClientRPCInterceptor {
	return func(rpcInfo psrpc.RPCInfo, next psrpc.ClientRPCHandler) psrpc.ClientRPCHandler {
		return func(ctx context.Context, req proto.Message, opts ...psrpc.RequestOption) (res proto.Message, err error) {
			res, err = next(ctx, req, opts...)
			return res, utils.ScreenError(err, errs...)
		}
	}
}

func newServerRPCErorrInterceptor(errs ...error) psrpc.ServerRPCInterceptor {
	return func(ctx context.Context, req proto.Message, rpcInfo psrpc.RPCInfo, handler psrpc.ServerRPCHandler) (res proto.Message, err error) {
		res, err = handler(ctx, req)
		return res, utils.ScreenError(err, errs...)
	}
}

func newMultiRPCErorrInterceptor(errs ...error) psrpc.ClientMultiRPCInterceptor {
	return func(rpcInfo psrpc.RPCInfo, next psrpc.ClientMultiRPCHandler) psrpc.ClientMultiRPCHandler {
		return &multiRPCErorrInterceptor{
			ClientMultiRPCHandler: next,
			errors:                errs,
		}
	}
}

type multiRPCErorrInterceptor struct {
	psrpc.ClientMultiRPCHandler
	errors []error
}

func (r *multiRPCErorrInterceptor) Recv(msg proto.Message, err error) {
	r.ClientMultiRPCHandler.Recv(msg, utils.ScreenError(err, r.errors...))
}
