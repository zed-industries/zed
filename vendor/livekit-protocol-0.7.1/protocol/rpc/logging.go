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

package rpc

import (
	"context"
	"time"

	"github.com/puzpuzpuz/xsync/v3"
	"google.golang.org/protobuf/proto"

	"github.com/livekit/protocol/logger"
	"github.com/livekit/psrpc"
)

type loggerCache struct {
	m *xsync.MapOf[string, logger.Logger]
}

func newLoggerCache() loggerCache {
	return loggerCache{m: xsync.NewMapOf[string, logger.Logger]()}
}

func (c loggerCache) Get(info psrpc.RPCInfo, l logger.Logger) logger.Logger {
	wl, _ := c.m.LoadOrCompute(info.Method, func() logger.Logger {
		return l.WithComponent("psrpc").WithComponent(info.Service).WithComponent(info.Method)
	})
	return wl
}

func WithClientLogger(logger logger.Logger) psrpc.ClientOption {
	return psrpc.WithClientOptions(
		psrpc.WithClientRPCInterceptors(newClientRPCLoggerInterceptor(logger)),
		psrpc.WithClientMultiRPCInterceptors(newMultiRPCLoggerInterceptor(logger)),
		psrpc.WithClientStreamInterceptors(newStreamLoggerInterceptor(logger)),
	)
}

func WithServerLogger(logger logger.Logger) psrpc.ServerOption {
	return psrpc.WithServerOptions(
		psrpc.WithServerRPCInterceptors(newServerRPCLoggerInterceptor(logger)),
		psrpc.WithServerStreamInterceptors(newStreamLoggerInterceptor(logger)),
	)
}

func newClientRPCLoggerInterceptor(l logger.Logger) psrpc.ClientRPCInterceptor {
	loggers := newLoggerCache()
	return func(rpcInfo psrpc.RPCInfo, next psrpc.ClientRPCHandler) psrpc.ClientRPCHandler {
		l := loggers.Get(rpcInfo, l)
		return func(ctx context.Context, req proto.Message, opts ...psrpc.RequestOption) (res proto.Message, err error) {
			start := time.Now()
			defer func() {
				if err != nil {
					l.Warnw("client error", err, "topic", rpcInfo.Topic, "request", logger.Proto(req), "response", logger.Proto(res), "duration", time.Since(start))
				} else {
					l.Debugw("client response", "topic", rpcInfo.Topic, "request", logger.Proto(req), "response", logger.Proto(res), "duration", time.Since(start))
				}
			}()
			return next(ctx, req, opts...)
		}
	}
}

func newServerRPCLoggerInterceptor(l logger.Logger) psrpc.ServerRPCInterceptor {
	loggers := newLoggerCache()
	return func(ctx context.Context, req proto.Message, rpcInfo psrpc.RPCInfo, handler psrpc.ServerRPCHandler) (res proto.Message, err error) {
		l := loggers.Get(rpcInfo, l)
		start := time.Now()
		defer func() {
			if err != nil {
				l.Warnw("server error", err, "topic", rpcInfo.Topic, "request", logger.Proto(req), "response", logger.Proto(res), "duration", time.Since(start))
			} else {
				l.Debugw("server response", "topic", rpcInfo.Topic, "request", logger.Proto(req), "response", logger.Proto(res), "duration", time.Since(start))
			}
		}()
		return handler(ctx, req)
	}
}

func newStreamLoggerInterceptor(l logger.Logger) psrpc.StreamInterceptor {
	loggers := newLoggerCache()
	return func(rpcInfo psrpc.RPCInfo, next psrpc.StreamHandler) psrpc.StreamHandler {
		l := loggers.Get(rpcInfo, l).WithValues("topic", rpcInfo.Topic)
		l.Debugw("stream opened")
		return &streamLoggerInterceptor{
			StreamHandler: next,
			logger:        l,
		}
	}
}

type streamLoggerInterceptor struct {
	psrpc.StreamHandler
	logger logger.Logger
}

func (s *streamLoggerInterceptor) Recv(msg proto.Message) (err error) {
	s.logger.Debugw("received message", "message", logger.Proto(msg))
	return s.StreamHandler.Recv(msg)
}

func (s *streamLoggerInterceptor) Send(msg proto.Message, opts ...psrpc.StreamOption) (err error) {
	start := time.Now()
	defer func() {
		if err != nil {
			s.logger.Warnw("failed to send message", err, "message", logger.Proto(msg), "duration", time.Since(start))
		} else {
			s.logger.Debugw("sent message", "message", logger.Proto(msg), "duration", time.Since(start))
		}
	}()
	return s.StreamHandler.Send(msg, opts...)
}

func (s *streamLoggerInterceptor) Close(cause error) error {
	s.logger.Debugw("stream closed")
	return s.StreamHandler.Close(cause)
}

func newMultiRPCLoggerInterceptor(l logger.Logger) psrpc.ClientMultiRPCInterceptor {
	loggers := newLoggerCache()
	return func(rpcInfo psrpc.RPCInfo, next psrpc.ClientMultiRPCHandler) psrpc.ClientMultiRPCHandler {
		return &multiRPCLoggerInterceptor{
			ClientMultiRPCHandler: next,
			logger:                loggers.Get(rpcInfo, l).WithValues("topic", rpcInfo.Topic),
			start:                 time.Now(),
		}
	}
}

type multiRPCLoggerInterceptor struct {
	psrpc.ClientMultiRPCHandler
	logger        logger.Logger
	start         time.Time
	responseCount int
	errorCount    int
}

func (r *multiRPCLoggerInterceptor) Send(ctx context.Context, req proto.Message, opts ...psrpc.RequestOption) error {
	r.start = time.Now()
	r.logger.Debugw("multirpc opened", "request", logger.Proto(req))
	return r.ClientMultiRPCHandler.Send(ctx, req, opts...)
}

func (r *multiRPCLoggerInterceptor) Recv(msg proto.Message, err error) {
	if err != nil {
		r.logger.Warnw("received error", err)
		r.errorCount++
	} else {
		r.logger.Debugw("received response", "response", logger.Proto(msg))
		r.responseCount++
	}
	r.ClientMultiRPCHandler.Recv(msg, err)
}

func (r *multiRPCLoggerInterceptor) Close() {
	r.logger.Debugw("multirpc closed", "responseCount", r.responseCount, "errorCount", r.errorCount)
	r.ClientMultiRPCHandler.Close()
}
