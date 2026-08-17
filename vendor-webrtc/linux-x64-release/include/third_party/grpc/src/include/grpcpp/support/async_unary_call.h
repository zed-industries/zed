//
//
// Copyright 2015 gRPC authors.
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
//
//

#ifndef GRPCPP_SUPPORT_ASYNC_UNARY_CALL_H
#define GRPCPP_SUPPORT_ASYNC_UNARY_CALL_H

#include <grpc/grpc.h>
#include <grpcpp/client_context.h>
#include <grpcpp/impl/call.h>
#include <grpcpp/impl/call_op_set.h>
#include <grpcpp/impl/call_op_set_interface.h>
#include <grpcpp/impl/channel_interface.h>
#include <grpcpp/impl/service_type.h>
#include <grpcpp/server_context.h>
#include <grpcpp/support/status.h>

#include "absl/log/absl_check.h"

namespace grpc {

// Forward declaration for use in Helper class
template <class R>
class ClientAsyncResponseReader;

/// An interface relevant for async client side unary RPCs (which send
/// one request message to a server and receive one response message).
template <class R>
class ClientAsyncResponseReaderInterface {
 public:
  virtual ~ClientAsyncResponseReaderInterface() {}

  /// Start the call that was set up by the constructor, but only if the
  /// constructor was invoked through the "Prepare" API which doesn't actually
  /// start the call
  virtual void StartCall() = 0;

  /// Request notification of the reading of initial metadata. Completion
  /// will be notified by \a tag on the associated completion queue.
  /// This call is optional, but if it is used, it cannot be used concurrently
  /// with or after the \a Finish method.
  ///
  /// \param[in] tag Tag identifying this request.
  virtual void ReadInitialMetadata(void* tag) = 0;

  /// Request to receive the server's response \a msg and final \a status for
  /// the call, and to notify \a tag on this call's completion queue when
  /// finished.
  ///
  /// This function will return when either:
  /// - when the server's response message and status have been received.
  /// - when the server has returned a non-OK status (no message expected in
  ///   this case).
  /// - when the call failed for some reason and the library generated a
  ///   non-OK status.
  ///
  /// \param[in] tag Tag identifying this request.
  /// \param[out] status To be updated with the operation status.
  /// \param[out] msg To be filled in with the server's response message.
  virtual void Finish(R* msg, grpc::Status* status, void* tag) = 0;
};

namespace internal {

class ClientAsyncResponseReaderHelper {
 public:
  /// Start a call and write the request out if \a start is set.
  /// \a tag will be notified on \a cq when the call has been started (i.e.
  /// initial metadata sent) and \a request has been written out.
  /// If \a start is not set, the actual call must be initiated by StartCall
  /// Note that \a context will be used to fill in custom initial metadata
  /// used to send to the server when starting the call.
  ///
  /// Optionally pass in a base class for request and response types so that the
  /// internal functions and structs can be templated based on that, allowing
  /// reuse across RPCs (e.g., MessageLite for protobuf). Since constructors
  /// can't have an explicit template parameter, the last argument is an
  /// extraneous parameter just to provide the needed type information.
  template <class R, class W, class BaseR = R, class BaseW = W>
  static ClientAsyncResponseReader<R>* Create(
      grpc::ChannelInterface* channel, grpc::CompletionQueue* cq,
      const grpc::internal::RpcMethod& method, grpc::ClientContext* context,
      const W& request) /* __attribute__((noinline)) */ {
    grpc::internal::Call call = channel->CreateCall(method, context, cq);
    ClientAsyncResponseReader<R>* result = new (grpc_call_arena_alloc(
        call.call(), sizeof(ClientAsyncResponseReader<R>)))
        ClientAsyncResponseReader<R>(call, context);
    SetupRequest<BaseR, BaseW>(
        call.call(), &result->single_buf_, &result->read_initial_metadata_,
        &result->finish_, static_cast<const BaseW&>(request));

    return result;
  }

  // Various helper functions to reduce templating use

  template <class R, class W>
  static void SetupRequest(
      grpc_call* call,
      grpc::internal::CallOpSendInitialMetadata** single_buf_ptr,
      std::function<void(ClientContext*, internal::Call*,
                         internal::CallOpSendInitialMetadata*, void*)>*
          read_initial_metadata,
      std::function<
          void(ClientContext*, internal::Call*, bool initial_metadata_read,
               internal::CallOpSendInitialMetadata*,
               internal::CallOpSetInterface**, void*, Status*, void*)>* finish,
      const W& request) {
    using SingleBufType =
        grpc::internal::CallOpSet<grpc::internal::CallOpSendInitialMetadata,
                                  grpc::internal::CallOpSendMessage,
                                  grpc::internal::CallOpClientSendClose,
                                  grpc::internal::CallOpRecvInitialMetadata,
                                  grpc::internal::CallOpRecvMessage<R>,
                                  grpc::internal::CallOpClientRecvStatus>;
    SingleBufType* single_buf =
        new (grpc_call_arena_alloc(call, sizeof(SingleBufType))) SingleBufType;
    *single_buf_ptr = single_buf;
    // TODO(ctiller): don't assert
    ABSL_CHECK(single_buf->SendMessage(request).ok());
    single_buf->ClientSendClose();

    // The purpose of the following functions is to type-erase the actual
    // templated type of the CallOpSet being used by hiding that type inside the
    // function definition rather than specifying it as an argument of the
    // function or a member of the class. The type-erased CallOpSet will get
    // static_cast'ed back to the real type so that it can be used properly.
    *read_initial_metadata =
        [](ClientContext* context, internal::Call* call,
           internal::CallOpSendInitialMetadata* single_buf_view, void* tag) {
          auto* single_buf = static_cast<SingleBufType*>(single_buf_view);
          single_buf->set_output_tag(tag);
          single_buf->RecvInitialMetadata(context);
          call->PerformOps(single_buf);
        };

    // Note that this function goes one step further than the previous one
    // because it type-erases the message being written down to a void*. This
    // will be static-cast'ed back to the class specified here by hiding that
    // class information inside the function definition. Note that this feature
    // expects the class being specified here for R to be a base-class of the
    // "real" R without any multiple-inheritance (as applies in protobuf wrt
    // MessageLite)
    *finish = [](ClientContext* context, internal::Call* call,
                 bool initial_metadata_read,
                 internal::CallOpSendInitialMetadata* single_buf_view,
                 internal::CallOpSetInterface** finish_buf_ptr, void* msg,
                 Status* status, void* tag) {
      if (initial_metadata_read) {
        using FinishBufType =
            grpc::internal::CallOpSet<grpc::internal::CallOpRecvMessage<R>,
                                      grpc::internal::CallOpClientRecvStatus>;
        FinishBufType* finish_buf =
            new (grpc_call_arena_alloc(call->call(), sizeof(FinishBufType)))
                FinishBufType;
        *finish_buf_ptr = finish_buf;
        finish_buf->set_output_tag(tag);
        finish_buf->RecvMessage(static_cast<R*>(msg));
        finish_buf->AllowNoMessage();
        finish_buf->ClientRecvStatus(context, status);
        call->PerformOps(finish_buf);
      } else {
        auto* single_buf = static_cast<SingleBufType*>(single_buf_view);
        single_buf->set_output_tag(tag);
        single_buf->RecvInitialMetadata(context);
        single_buf->RecvMessage(static_cast<R*>(msg));
        single_buf->AllowNoMessage();
        single_buf->ClientRecvStatus(context, status);
        call->PerformOps(single_buf);
      }
    };
  }

  static void StartCall(grpc::ClientContext* context,
                        grpc::internal::CallOpSendInitialMetadata* single_buf) {
    single_buf->SendInitialMetadata(&context->send_initial_metadata_,
                                    context->initial_metadata_flags());
  }
};

// TODO(vjpai): This templated factory is deprecated and will be replaced by
//.             the non-templated helper as soon as possible.
template <class R>
class ClientAsyncResponseReaderFactory {
 public:
  template <class W>
  static ClientAsyncResponseReader<R>* Create(
      grpc::ChannelInterface* channel, grpc::CompletionQueue* cq,
      const grpc::internal::RpcMethod& method, grpc::ClientContext* context,
      const W& request, bool start) {
    auto* result = ClientAsyncResponseReaderHelper::Create<R>(
        channel, cq, method, context, request);
    if (start) {
      result->StartCall();
    }
    return result;
  }
};

}  // namespace internal

/// Async API for client-side unary RPCs, where the message response
/// received from the server is of type \a R.
template <class R>
class ClientAsyncResponseReader final
    : public ClientAsyncResponseReaderInterface<R> {
 public:
  // always allocated against a call arena, no memory free required
  static void operator delete(void* /*ptr*/, std::size_t size) {
    ABSL_CHECK_EQ(size, sizeof(ClientAsyncResponseReader));
  }

  // This operator should never be called as the memory should be freed as part
  // of the arena destruction. It only exists to provide a matching operator
  // delete to the operator new so that some compilers will not complain (see
  // https://github.com/grpc/grpc/issues/11301) Note at the time of adding this
  // there are no tests catching the compiler warning.
  static void operator delete(void*, void*) { ABSL_CHECK(false); }

  void StartCall() override {
    ABSL_DCHECK(!started_);
    started_ = true;
    internal::ClientAsyncResponseReaderHelper::StartCall(context_, single_buf_);
  }

  /// See \a ClientAsyncResponseReaderInterface::ReadInitialMetadata for
  /// semantics.
  ///
  /// Side effect:
  ///   - the \a ClientContext associated with this call is updated with
  ///     possible initial and trailing metadata sent from the server.
  void ReadInitialMetadata(void* tag) override {
    ABSL_DCHECK(started_);
    ABSL_DCHECK(!context_->initial_metadata_received_);
    read_initial_metadata_(context_, &call_, single_buf_, tag);
    initial_metadata_read_ = true;
  }

  /// See \a ClientAsyncResponseReaderInterface::Finish for semantics.
  ///
  /// Side effect:
  ///   - the \a ClientContext associated with this call is updated with
  ///     possible initial and trailing metadata sent from the server.
  void Finish(R* msg, grpc::Status* status, void* tag) override {
    ABSL_DCHECK(started_);
    finish_(context_, &call_, initial_metadata_read_, single_buf_, &finish_buf_,
            static_cast<void*>(msg), status, tag);
  }

 private:
  friend class internal::ClientAsyncResponseReaderHelper;
  grpc::ClientContext* const context_;
  grpc::internal::Call call_;
  bool started_ = false;
  bool initial_metadata_read_ = false;

  ClientAsyncResponseReader(grpc::internal::Call call,
                            grpc::ClientContext* context)
      : context_(context), call_(call) {}

  // disable operator new
  static void* operator new(std::size_t size);
  static void* operator new(std::size_t /*size*/, void* p) { return p; }

  internal::CallOpSendInitialMetadata* single_buf_;
  internal::CallOpSetInterface* finish_buf_ = nullptr;
  std::function<void(ClientContext*, internal::Call*,
                     internal::CallOpSendInitialMetadata*, void*)>
      read_initial_metadata_;
  std::function<void(ClientContext*, internal::Call*,
                     bool initial_metadata_read,
                     internal::CallOpSendInitialMetadata*,
                     internal::CallOpSetInterface**, void*, Status*, void*)>
      finish_;
};

/// Async server-side API for handling unary calls, where the single
/// response message sent to the client is of type \a W.
template <class W>
class ServerAsyncResponseWriter final
    : public grpc::internal::ServerAsyncStreamingInterface {
 public:
  explicit ServerAsyncResponseWriter(grpc::ServerContext* ctx)
      : call_(nullptr, nullptr, nullptr), ctx_(ctx) {}

  /// See \a ServerAsyncStreamingInterface::SendInitialMetadata for semantics.
  ///
  /// Side effect:
  ///   The initial metadata that will be sent to the client from this op will
  ///   be taken from the \a ServerContext associated with the call.
  ///
  /// \param[in] tag Tag identifying this request.
  void SendInitialMetadata(void* tag) override {
    ABSL_CHECK(!ctx_->sent_initial_metadata_);

    meta_buf_.set_output_tag(tag);
    meta_buf_.SendInitialMetadata(&ctx_->initial_metadata_,
                                  ctx_->initial_metadata_flags());
    if (ctx_->compression_level_set()) {
      meta_buf_.set_compression_level(ctx_->compression_level());
    }
    ctx_->sent_initial_metadata_ = true;
    call_.PerformOps(&meta_buf_);
  }

  /// Indicate that the stream is to be finished and request notification
  /// when the server has sent the appropriate signals to the client to
  /// end the call. Should not be used concurrently with other operations.
  ///
  /// \param[in] tag Tag identifying this request.
  /// \param[in] status To be sent to the client as the result of the call.
  /// \param[in] msg Message to be sent to the client.
  ///
  /// Side effect:
  ///   - also sends initial metadata if not already sent (using the
  ///     \a ServerContext associated with this call).
  ///
  /// Note: if \a status has a non-OK code, then \a msg will not be sent,
  /// and the client will receive only the status with possible trailing
  /// metadata.
  ///
  /// gRPC doesn't take ownership or a reference to msg and status, so it is
  /// safe to deallocate them once the Finish operation is complete (i.e. a
  /// result arrives in the completion queue).
  void Finish(const W& msg, const grpc::Status& status, void* tag) {
    finish_buf_.set_output_tag(tag);
    finish_buf_.set_core_cq_tag(&finish_buf_);
    if (!ctx_->sent_initial_metadata_) {
      finish_buf_.SendInitialMetadata(&ctx_->initial_metadata_,
                                      ctx_->initial_metadata_flags());
      if (ctx_->compression_level_set()) {
        finish_buf_.set_compression_level(ctx_->compression_level());
      }
      ctx_->sent_initial_metadata_ = true;
    }
    // The response is dropped if the status is not OK.
    if (status.ok()) {
      finish_buf_.ServerSendStatus(&ctx_->trailing_metadata_,
                                   finish_buf_.SendMessage(msg));
    } else {
      finish_buf_.ServerSendStatus(&ctx_->trailing_metadata_, status);
    }
    call_.PerformOps(&finish_buf_);
  }

  /// Indicate that the stream is to be finished with a non-OK status,
  /// and request notification for when the server has finished sending the
  /// appropriate signals to the client to end the call.
  /// Should not be used concurrently with other operations.
  ///
  /// \param[in] tag Tag identifying this request.
  /// \param[in] status To be sent to the client as the result of the call.
  ///   - Note: \a status must have a non-OK code.
  ///
  /// Side effect:
  ///   - also sends initial metadata if not already sent (using the
  ///     \a ServerContext associated with this call).
  ///
  /// gRPC doesn't take ownership or a reference to status, so it is safe to
  /// deallocate them once the Finish operation is complete (i.e. a result
  /// arrives in the completion queue).
  void FinishWithError(const grpc::Status& status, void* tag) {
    ABSL_CHECK(!status.ok());
    finish_buf_.set_output_tag(tag);
    if (!ctx_->sent_initial_metadata_) {
      finish_buf_.SendInitialMetadata(&ctx_->initial_metadata_,
                                      ctx_->initial_metadata_flags());
      if (ctx_->compression_level_set()) {
        finish_buf_.set_compression_level(ctx_->compression_level());
      }
      ctx_->sent_initial_metadata_ = true;
    }
    finish_buf_.ServerSendStatus(&ctx_->trailing_metadata_, status);
    call_.PerformOps(&finish_buf_);
  }

 private:
  void BindCall(grpc::internal::Call* call) override { call_ = *call; }

  grpc::internal::Call call_;
  grpc::ServerContext* ctx_;
  grpc::internal::CallOpSet<grpc::internal::CallOpSendInitialMetadata>
      meta_buf_;
  grpc::internal::CallOpSet<grpc::internal::CallOpSendInitialMetadata,
                            grpc::internal::CallOpSendMessage,
                            grpc::internal::CallOpServerSendStatus>
      finish_buf_;
};

}  // namespace grpc

namespace std {
template <class R>
class default_delete<grpc::ClientAsyncResponseReader<R>> {
 public:
  void operator()(void* /*p*/) {}
};
template <class R>
class default_delete<grpc::ClientAsyncResponseReaderInterface<R>> {
 public:
  void operator()(void* /*p*/) {}
};
}  // namespace std

#endif  // GRPCPP_SUPPORT_ASYNC_UNARY_CALL_H
