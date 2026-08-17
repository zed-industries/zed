//
//
// Copyright 2019 gRPC authors.
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

#ifndef GRPCPP_IMPL_SERVER_CALLBACK_HANDLERS_H
#define GRPCPP_IMPL_SERVER_CALLBACK_HANDLERS_H

#include <grpc/grpc.h>
#include <grpc/impl/call.h>
#include <grpcpp/impl/rpc_service_method.h>
#include <grpcpp/server_context.h>
#include <grpcpp/support/message_allocator.h>
#include <grpcpp/support/server_callback.h>
#include <grpcpp/support/status.h>

#include "absl/log/absl_check.h"

namespace grpc {
namespace internal {

template <class RequestType, class ResponseType>
class CallbackUnaryHandler : public grpc::internal::MethodHandler {
 public:
  explicit CallbackUnaryHandler(
      std::function<ServerUnaryReactor*(grpc::CallbackServerContext*,
                                        const RequestType*, ResponseType*)>
          get_reactor)
      : get_reactor_(std::move(get_reactor)) {}

  void SetMessageAllocator(
      MessageAllocator<RequestType, ResponseType>* allocator) {
    allocator_ = allocator;
  }

  void RunHandler(const HandlerParameter& param) final {
    // Arena allocate a controller structure (that includes request/response)
    grpc_call_ref(param.call->call());
    auto* allocator_state =
        static_cast<MessageHolder<RequestType, ResponseType>*>(
            param.internal_data);

    auto* call = new (grpc_call_arena_alloc(param.call->call(),
                                            sizeof(ServerCallbackUnaryImpl)))
        ServerCallbackUnaryImpl(
            static_cast<grpc::CallbackServerContext*>(param.server_context),
            param.call, allocator_state, param.call_requester);
    param.server_context->BeginCompletionOp(
        param.call, [call](bool) { call->MaybeDone(); }, call);

    ServerUnaryReactor* reactor = nullptr;
    if (param.status.ok()) {
      reactor = grpc::internal::CatchingReactorGetter<ServerUnaryReactor>(
          get_reactor_,
          static_cast<grpc::CallbackServerContext*>(param.server_context),
          call->request(), call->response());
    }

    if (reactor == nullptr) {
      // if deserialization or reactor creator failed, we need to fail the call
      reactor = new (grpc_call_arena_alloc(param.call->call(),
                                           sizeof(UnimplementedUnaryReactor)))
          UnimplementedUnaryReactor(
              grpc::Status(grpc::StatusCode::UNIMPLEMENTED, ""));
    }

    /// Invoke SetupReactor as the last part of the handler
    call->SetupReactor(reactor);
  }

  void* Deserialize(grpc_call* call, grpc_byte_buffer* req,
                    grpc::Status* status, void** handler_data) final {
    grpc::ByteBuffer buf;
    buf.set_buffer(req);
    RequestType* request = nullptr;
    MessageHolder<RequestType, ResponseType>* allocator_state;
    if (allocator_ != nullptr) {
      allocator_state = allocator_->AllocateMessages();
    } else {
      allocator_state = new (grpc_call_arena_alloc(
          call, sizeof(DefaultMessageHolder<RequestType, ResponseType>)))
          DefaultMessageHolder<RequestType, ResponseType>();
    }
    *handler_data = allocator_state;
    request = allocator_state->request();
    *status =
        grpc::SerializationTraits<RequestType>::Deserialize(&buf, request);
    buf.Release();
    if (status->ok()) {
      return request;
    }
    return nullptr;
  }

 private:
  std::function<ServerUnaryReactor*(grpc::CallbackServerContext*,
                                    const RequestType*, ResponseType*)>
      get_reactor_;
  MessageAllocator<RequestType, ResponseType>* allocator_ = nullptr;

  class ServerCallbackUnaryImpl : public ServerCallbackUnary {
   public:
    void Finish(grpc::Status s) override {
      // A callback that only contains a call to MaybeDone can be run as an
      // inline callback regardless of whether or not OnDone is inlineable
      // because if the actual OnDone callback needs to be scheduled, MaybeDone
      // is responsible for dispatching to an executor thread if needed. Thus,
      // when setting up the finish_tag_, we can set its own callback to
      // inlineable.
      finish_tag_.Set(
          call_.call(),
          [this](bool) {
            this->MaybeDone(
                reactor_.load(std::memory_order_relaxed)->InternalInlineable());
          },
          &finish_ops_, /*can_inline=*/true);
      finish_ops_.set_core_cq_tag(&finish_tag_);

      if (!ctx_->sent_initial_metadata_) {
        finish_ops_.SendInitialMetadata(&ctx_->initial_metadata_,
                                        ctx_->initial_metadata_flags());
        if (ctx_->compression_level_set()) {
          finish_ops_.set_compression_level(ctx_->compression_level());
        }
        ctx_->sent_initial_metadata_ = true;
      }
      // The response is dropped if the status is not OK.
      if (s.ok()) {
        finish_ops_.ServerSendStatus(&ctx_->trailing_metadata_,
                                     finish_ops_.SendMessagePtr(response()));
      } else {
        finish_ops_.ServerSendStatus(&ctx_->trailing_metadata_, s);
      }
      finish_ops_.set_core_cq_tag(&finish_tag_);
      call_.PerformOps(&finish_ops_);
    }

    void SendInitialMetadata() override {
      ABSL_CHECK(!ctx_->sent_initial_metadata_);
      this->Ref();
      // The callback for this function should not be marked inline because it
      // is directly invoking a user-controlled reaction
      // (OnSendInitialMetadataDone). Thus it must be dispatched to an executor
      // thread. However, any OnDone needed after that can be inlined because it
      // is already running on an executor thread.
      meta_tag_.Set(
          call_.call(),
          [this](bool ok) {
            ServerUnaryReactor* reactor =
                reactor_.load(std::memory_order_relaxed);
            reactor->OnSendInitialMetadataDone(ok);
            this->MaybeDone(/*inlineable_ondone=*/true);
          },
          &meta_ops_, /*can_inline=*/false);
      meta_ops_.SendInitialMetadata(&ctx_->initial_metadata_,
                                    ctx_->initial_metadata_flags());
      if (ctx_->compression_level_set()) {
        meta_ops_.set_compression_level(ctx_->compression_level());
      }
      ctx_->sent_initial_metadata_ = true;
      meta_ops_.set_core_cq_tag(&meta_tag_);
      call_.PerformOps(&meta_ops_);
    }

   private:
    friend class CallbackUnaryHandler<RequestType, ResponseType>;

    ServerCallbackUnaryImpl(
        grpc::CallbackServerContext* ctx, grpc::internal::Call* call,
        MessageHolder<RequestType, ResponseType>* allocator_state,
        std::function<void()> call_requester)
        : ctx_(ctx),
          call_(*call),
          allocator_state_(allocator_state),
          call_requester_(std::move(call_requester)) {
      ctx_->set_message_allocator_state(allocator_state);
    }

    grpc_call* call() override { return call_.call(); }

    /// SetupReactor binds the reactor (which also releases any queued
    /// operations), maybe calls OnCancel if possible/needed, and maybe marks
    /// the completion of the RPC. This should be the last component of the
    /// handler.
    void SetupReactor(ServerUnaryReactor* reactor) {
      reactor_.store(reactor, std::memory_order_relaxed);
      this->BindReactor(reactor);
      this->MaybeCallOnCancel(reactor);
      this->MaybeDone(reactor->InternalInlineable());
    }

    const RequestType* request() { return allocator_state_->request(); }
    ResponseType* response() { return allocator_state_->response(); }

    void CallOnDone() override {
      reactor_.load(std::memory_order_relaxed)->OnDone();
      grpc_call* call = call_.call();
      auto call_requester = std::move(call_requester_);
      allocator_state_->Release();
      if (ctx_->context_allocator() != nullptr) {
        ctx_->context_allocator()->Release(ctx_);
      }
      this->~ServerCallbackUnaryImpl();  // explicitly call destructor
      grpc_call_unref(call);
      call_requester();
    }

    ServerReactor* reactor() override {
      return reactor_.load(std::memory_order_relaxed);
    }

    grpc::internal::CallOpSet<grpc::internal::CallOpSendInitialMetadata>
        meta_ops_;
    grpc::internal::CallbackWithSuccessTag meta_tag_;
    grpc::internal::CallOpSet<grpc::internal::CallOpSendInitialMetadata,
                              grpc::internal::CallOpSendMessage,
                              grpc::internal::CallOpServerSendStatus>
        finish_ops_;
    grpc::internal::CallbackWithSuccessTag finish_tag_;

    grpc::CallbackServerContext* const ctx_;
    grpc::internal::Call call_;
    MessageHolder<RequestType, ResponseType>* const allocator_state_;
    std::function<void()> call_requester_;
    // reactor_ can always be loaded/stored with relaxed memory ordering because
    // its value is only set once, independently of other data in the object,
    // and the loads that use it will always actually come provably later even
    // though they are from different threads since they are triggered by
    // actions initiated only by the setting up of the reactor_ variable. In
    // a sense, it's a delayed "const": it gets its value from the SetupReactor
    // method (not the constructor, so it's not a true const), but it doesn't
    // change after that and it only gets used by actions caused, directly or
    // indirectly, by that setup. This comment also applies to the reactor_
    // variables of the other streaming objects in this file.
    std::atomic<ServerUnaryReactor*> reactor_;
    // callbacks_outstanding_ follows a refcount pattern
    std::atomic<intptr_t> callbacks_outstanding_{
        3};  // reserve for start, Finish, and CompletionOp
  };
};

template <class RequestType, class ResponseType>
class CallbackClientStreamingHandler : public grpc::internal::MethodHandler {
 public:
  explicit CallbackClientStreamingHandler(
      std::function<ServerReadReactor<RequestType>*(
          grpc::CallbackServerContext*, ResponseType*)>
          get_reactor)
      : get_reactor_(std::move(get_reactor)) {}
  void RunHandler(const HandlerParameter& param) final {
    // Arena allocate a reader structure (that includes response)
    grpc_call_ref(param.call->call());

    auto* reader = new (grpc_call_arena_alloc(param.call->call(),
                                              sizeof(ServerCallbackReaderImpl)))
        ServerCallbackReaderImpl(
            static_cast<grpc::CallbackServerContext*>(param.server_context),
            param.call, param.call_requester);
    // Inlineable OnDone can be false in the CompletionOp callback because there
    // is no read reactor that has an inlineable OnDone; this only applies to
    // the DefaultReactor (which is unary).
    param.server_context->BeginCompletionOp(
        param.call,
        [reader](bool) { reader->MaybeDone(/*inlineable_ondone=*/false); },
        reader);

    ServerReadReactor<RequestType>* reactor = nullptr;
    if (param.status.ok()) {
      reactor =
          grpc::internal::CatchingReactorGetter<ServerReadReactor<RequestType>>(
              get_reactor_,
              static_cast<grpc::CallbackServerContext*>(param.server_context),
              reader->response());
    }

    if (reactor == nullptr) {
      // if deserialization or reactor creator failed, we need to fail the call
      reactor = new (grpc_call_arena_alloc(
          param.call->call(), sizeof(UnimplementedReadReactor<RequestType>)))
          UnimplementedReadReactor<RequestType>(
              grpc::Status(grpc::StatusCode::UNIMPLEMENTED, ""));
    }

    reader->SetupReactor(reactor);
  }

 private:
  std::function<ServerReadReactor<RequestType>*(grpc::CallbackServerContext*,
                                                ResponseType*)>
      get_reactor_;

  class ServerCallbackReaderImpl : public ServerCallbackReader<RequestType> {
   public:
    void Finish(grpc::Status s) override {
      // A finish tag with only MaybeDone can have its callback inlined
      // regardless even if OnDone is not inlineable because this callback just
      // checks a ref and then decides whether or not to dispatch OnDone.
      finish_tag_.Set(
          call_.call(),
          [this](bool) {
            // Inlineable OnDone can be false here because there is
            // no read reactor that has an inlineable OnDone; this
            // only applies to the DefaultReactor (which is unary).
            this->MaybeDone(/*inlineable_ondone=*/false);
          },
          &finish_ops_, /*can_inline=*/true);
      if (!ctx_->sent_initial_metadata_) {
        finish_ops_.SendInitialMetadata(&ctx_->initial_metadata_,
                                        ctx_->initial_metadata_flags());
        if (ctx_->compression_level_set()) {
          finish_ops_.set_compression_level(ctx_->compression_level());
        }
        ctx_->sent_initial_metadata_ = true;
      }
      // The response is dropped if the status is not OK.
      if (s.ok()) {
        finish_ops_.ServerSendStatus(&ctx_->trailing_metadata_,
                                     finish_ops_.SendMessagePtr(&resp_));
      } else {
        finish_ops_.ServerSendStatus(&ctx_->trailing_metadata_, s);
      }
      finish_ops_.set_core_cq_tag(&finish_tag_);
      call_.PerformOps(&finish_ops_);
    }

    void SendInitialMetadata() override {
      ABSL_CHECK(!ctx_->sent_initial_metadata_);
      this->Ref();
      // The callback for this function should not be inlined because it invokes
      // a user-controlled reaction, but any resulting OnDone can be inlined in
      // the executor to which this callback is dispatched.
      meta_tag_.Set(
          call_.call(),
          [this](bool ok) {
            ServerReadReactor<RequestType>* reactor =
                reactor_.load(std::memory_order_relaxed);
            reactor->OnSendInitialMetadataDone(ok);
            this->MaybeDone(/*inlineable_ondone=*/true);
          },
          &meta_ops_, /*can_inline=*/false);
      meta_ops_.SendInitialMetadata(&ctx_->initial_metadata_,
                                    ctx_->initial_metadata_flags());
      if (ctx_->compression_level_set()) {
        meta_ops_.set_compression_level(ctx_->compression_level());
      }
      ctx_->sent_initial_metadata_ = true;
      meta_ops_.set_core_cq_tag(&meta_tag_);
      call_.PerformOps(&meta_ops_);
    }

    void Read(RequestType* req) override {
      this->Ref();
      read_ops_.RecvMessage(req);
      call_.PerformOps(&read_ops_);
    }

   private:
    friend class CallbackClientStreamingHandler<RequestType, ResponseType>;

    ServerCallbackReaderImpl(grpc::CallbackServerContext* ctx,
                             grpc::internal::Call* call,
                             std::function<void()> call_requester)
        : ctx_(ctx), call_(*call), call_requester_(std::move(call_requester)) {}

    grpc_call* call() override { return call_.call(); }

    void SetupReactor(ServerReadReactor<RequestType>* reactor) {
      reactor_.store(reactor, std::memory_order_relaxed);
      // The callback for this function should not be inlined because it invokes
      // a user-controlled reaction, but any resulting OnDone can be inlined in
      // the executor to which this callback is dispatched.
      read_tag_.Set(
          call_.call(),
          [this, reactor](bool ok) {
            if (GPR_UNLIKELY(!ok)) {
              ctx_->MaybeMarkCancelledOnRead();
            }
            reactor->OnReadDone(ok);
            this->MaybeDone(/*inlineable_ondone=*/true);
          },
          &read_ops_, /*can_inline=*/false);
      read_ops_.set_core_cq_tag(&read_tag_);
      this->BindReactor(reactor);
      this->MaybeCallOnCancel(reactor);
      // Inlineable OnDone can be false here because there is no read
      // reactor that has an inlineable OnDone; this only applies to the
      // DefaultReactor (which is unary).
      this->MaybeDone(/*inlineable_ondone=*/false);
    }

    ~ServerCallbackReaderImpl() {}

    ResponseType* response() { return &resp_; }

    void CallOnDone() override {
      reactor_.load(std::memory_order_relaxed)->OnDone();
      grpc_call* call = call_.call();
      auto call_requester = std::move(call_requester_);
      if (ctx_->context_allocator() != nullptr) {
        ctx_->context_allocator()->Release(ctx_);
      }
      this->~ServerCallbackReaderImpl();  // explicitly call destructor
      grpc_call_unref(call);
      call_requester();
    }

    ServerReactor* reactor() override {
      return reactor_.load(std::memory_order_relaxed);
    }

    grpc::internal::CallOpSet<grpc::internal::CallOpSendInitialMetadata>
        meta_ops_;
    grpc::internal::CallbackWithSuccessTag meta_tag_;
    grpc::internal::CallOpSet<grpc::internal::CallOpSendInitialMetadata,
                              grpc::internal::CallOpSendMessage,
                              grpc::internal::CallOpServerSendStatus>
        finish_ops_;
    grpc::internal::CallbackWithSuccessTag finish_tag_;
    grpc::internal::CallOpSet<grpc::internal::CallOpRecvMessage<RequestType>>
        read_ops_;
    grpc::internal::CallbackWithSuccessTag read_tag_;

    grpc::CallbackServerContext* const ctx_;
    grpc::internal::Call call_;
    ResponseType resp_;
    std::function<void()> call_requester_;
    // The memory ordering of reactor_ follows ServerCallbackUnaryImpl.
    std::atomic<ServerReadReactor<RequestType>*> reactor_;
    // callbacks_outstanding_ follows a refcount pattern
    std::atomic<intptr_t> callbacks_outstanding_{
        3};  // reserve for OnStarted, Finish, and CompletionOp
  };
};

template <class RequestType, class ResponseType>
class CallbackServerStreamingHandler : public grpc::internal::MethodHandler {
 public:
  explicit CallbackServerStreamingHandler(
      std::function<ServerWriteReactor<ResponseType>*(
          grpc::CallbackServerContext*, const RequestType*)>
          get_reactor)
      : get_reactor_(std::move(get_reactor)) {}
  void RunHandler(const HandlerParameter& param) final {
    // Arena allocate a writer structure
    grpc_call_ref(param.call->call());

    auto* writer = new (grpc_call_arena_alloc(param.call->call(),
                                              sizeof(ServerCallbackWriterImpl)))
        ServerCallbackWriterImpl(
            static_cast<grpc::CallbackServerContext*>(param.server_context),
            param.call, static_cast<RequestType*>(param.request),
            param.call_requester);
    // Inlineable OnDone can be false in the CompletionOp callback because there
    // is no write reactor that has an inlineable OnDone; this only applies to
    // the DefaultReactor (which is unary).
    param.server_context->BeginCompletionOp(
        param.call,
        [writer](bool) { writer->MaybeDone(/*inlineable_ondone=*/false); },
        writer);

    ServerWriteReactor<ResponseType>* reactor = nullptr;
    if (param.status.ok()) {
      reactor = grpc::internal::CatchingReactorGetter<
          ServerWriteReactor<ResponseType>>(
          get_reactor_,
          static_cast<grpc::CallbackServerContext*>(param.server_context),
          writer->request());
    }
    if (reactor == nullptr) {
      // if deserialization or reactor creator failed, we need to fail the call
      reactor = new (grpc_call_arena_alloc(
          param.call->call(), sizeof(UnimplementedWriteReactor<ResponseType>)))
          UnimplementedWriteReactor<ResponseType>(
              grpc::Status(grpc::StatusCode::UNIMPLEMENTED, ""));
    }

    writer->SetupReactor(reactor);
  }

  void* Deserialize(grpc_call* call, grpc_byte_buffer* req,
                    grpc::Status* status, void** /*handler_data*/) final {
    grpc::ByteBuffer buf;
    buf.set_buffer(req);
    auto* request =
        new (grpc_call_arena_alloc(call, sizeof(RequestType))) RequestType();
    *status =
        grpc::SerializationTraits<RequestType>::Deserialize(&buf, request);
    buf.Release();
    if (status->ok()) {
      return request;
    }
    request->~RequestType();
    return nullptr;
  }

 private:
  std::function<ServerWriteReactor<ResponseType>*(grpc::CallbackServerContext*,
                                                  const RequestType*)>
      get_reactor_;

  class ServerCallbackWriterImpl : public ServerCallbackWriter<ResponseType> {
   public:
    void Finish(grpc::Status s) override {
      // A finish tag with only MaybeDone can have its callback inlined
      // regardless even if OnDone is not inlineable because this callback just
      // checks a ref and then decides whether or not to dispatch OnDone.
      finish_tag_.Set(
          call_.call(),
          [this](bool) {
            // Inlineable OnDone can be false here because there is
            // no write reactor that has an inlineable OnDone; this
            // only applies to the DefaultReactor (which is unary).
            this->MaybeDone(/*inlineable_ondone=*/false);
          },
          &finish_ops_, /*can_inline=*/true);
      finish_ops_.set_core_cq_tag(&finish_tag_);

      if (!ctx_->sent_initial_metadata_) {
        finish_ops_.SendInitialMetadata(&ctx_->initial_metadata_,
                                        ctx_->initial_metadata_flags());
        if (ctx_->compression_level_set()) {
          finish_ops_.set_compression_level(ctx_->compression_level());
        }
        ctx_->sent_initial_metadata_ = true;
      }
      finish_ops_.ServerSendStatus(&ctx_->trailing_metadata_, s);
      call_.PerformOps(&finish_ops_);
    }

    void SendInitialMetadata() override {
      ABSL_CHECK(!ctx_->sent_initial_metadata_);
      this->Ref();
      // The callback for this function should not be inlined because it invokes
      // a user-controlled reaction, but any resulting OnDone can be inlined in
      // the executor to which this callback is dispatched.
      meta_tag_.Set(
          call_.call(),
          [this](bool ok) {
            ServerWriteReactor<ResponseType>* reactor =
                reactor_.load(std::memory_order_relaxed);
            reactor->OnSendInitialMetadataDone(ok);
            this->MaybeDone(/*inlineable_ondone=*/true);
          },
          &meta_ops_, /*can_inline=*/false);
      meta_ops_.SendInitialMetadata(&ctx_->initial_metadata_,
                                    ctx_->initial_metadata_flags());
      if (ctx_->compression_level_set()) {
        meta_ops_.set_compression_level(ctx_->compression_level());
      }
      ctx_->sent_initial_metadata_ = true;
      meta_ops_.set_core_cq_tag(&meta_tag_);
      call_.PerformOps(&meta_ops_);
    }

    void Write(const ResponseType* resp, grpc::WriteOptions options) override {
      this->Ref();
      if (options.is_last_message()) {
        options.set_buffer_hint();
      }
      if (!ctx_->sent_initial_metadata_) {
        write_ops_.SendInitialMetadata(&ctx_->initial_metadata_,
                                       ctx_->initial_metadata_flags());
        if (ctx_->compression_level_set()) {
          write_ops_.set_compression_level(ctx_->compression_level());
        }
        ctx_->sent_initial_metadata_ = true;
      }
      // TODO(vjpai): don't assert
      ABSL_CHECK(write_ops_.SendMessagePtr(resp, options).ok());
      call_.PerformOps(&write_ops_);
    }

    void WriteAndFinish(const ResponseType* resp, grpc::WriteOptions options,
                        grpc::Status s) override {
      // This combines the write into the finish callback
      // TODO(vjpai): don't assert
      ABSL_CHECK(finish_ops_.SendMessagePtr(resp, options).ok());
      Finish(std::move(s));
    }

   private:
    friend class CallbackServerStreamingHandler<RequestType, ResponseType>;

    ServerCallbackWriterImpl(grpc::CallbackServerContext* ctx,
                             grpc::internal::Call* call, const RequestType* req,
                             std::function<void()> call_requester)
        : ctx_(ctx),
          call_(*call),
          req_(req),
          call_requester_(std::move(call_requester)) {}

    grpc_call* call() override { return call_.call(); }

    void SetupReactor(ServerWriteReactor<ResponseType>* reactor) {
      reactor_.store(reactor, std::memory_order_relaxed);
      // The callback for this function should not be inlined because it invokes
      // a user-controlled reaction, but any resulting OnDone can be inlined in
      // the executor to which this callback is dispatched.
      write_tag_.Set(
          call_.call(),
          [this, reactor](bool ok) {
            reactor->OnWriteDone(ok);
            this->MaybeDone(/*inlineable_ondone=*/true);
          },
          &write_ops_, /*can_inline=*/false);
      write_ops_.set_core_cq_tag(&write_tag_);
      this->BindReactor(reactor);
      this->MaybeCallOnCancel(reactor);
      // Inlineable OnDone can be false here because there is no write
      // reactor that has an inlineable OnDone; this only applies to the
      // DefaultReactor (which is unary).
      this->MaybeDone(/*inlineable_ondone=*/false);
    }
    ~ServerCallbackWriterImpl() {
      if (req_ != nullptr) {
        req_->~RequestType();
      }
    }

    const RequestType* request() { return req_; }

    void CallOnDone() override {
      reactor_.load(std::memory_order_relaxed)->OnDone();
      grpc_call* call = call_.call();
      auto call_requester = std::move(call_requester_);
      if (ctx_->context_allocator() != nullptr) {
        ctx_->context_allocator()->Release(ctx_);
      }
      this->~ServerCallbackWriterImpl();  // explicitly call destructor
      grpc_call_unref(call);
      call_requester();
    }

    ServerReactor* reactor() override {
      return reactor_.load(std::memory_order_relaxed);
    }

    grpc::internal::CallOpSet<grpc::internal::CallOpSendInitialMetadata>
        meta_ops_;
    grpc::internal::CallbackWithSuccessTag meta_tag_;
    grpc::internal::CallOpSet<grpc::internal::CallOpSendInitialMetadata,
                              grpc::internal::CallOpSendMessage,
                              grpc::internal::CallOpServerSendStatus>
        finish_ops_;
    grpc::internal::CallbackWithSuccessTag finish_tag_;
    grpc::internal::CallOpSet<grpc::internal::CallOpSendInitialMetadata,
                              grpc::internal::CallOpSendMessage>
        write_ops_;
    grpc::internal::CallbackWithSuccessTag write_tag_;

    grpc::CallbackServerContext* const ctx_;
    grpc::internal::Call call_;
    const RequestType* req_;
    std::function<void()> call_requester_;
    // The memory ordering of reactor_ follows ServerCallbackUnaryImpl.
    std::atomic<ServerWriteReactor<ResponseType>*> reactor_;
    // callbacks_outstanding_ follows a refcount pattern
    std::atomic<intptr_t> callbacks_outstanding_{
        3};  // reserve for OnStarted, Finish, and CompletionOp
  };
};

template <class RequestType, class ResponseType>
class CallbackBidiHandler : public grpc::internal::MethodHandler {
 public:
  explicit CallbackBidiHandler(
      std::function<ServerBidiReactor<RequestType, ResponseType>*(
          grpc::CallbackServerContext*)>
          get_reactor)
      : get_reactor_(std::move(get_reactor)) {}
  void RunHandler(const HandlerParameter& param) final {
    grpc_call_ref(param.call->call());

    auto* stream = new (grpc_call_arena_alloc(
        param.call->call(), sizeof(ServerCallbackReaderWriterImpl)))
        ServerCallbackReaderWriterImpl(
            static_cast<grpc::CallbackServerContext*>(param.server_context),
            param.call, param.call_requester);
    // Inlineable OnDone can be false in the CompletionOp callback because there
    // is no bidi reactor that has an inlineable OnDone; this only applies to
    // the DefaultReactor (which is unary).
    param.server_context->BeginCompletionOp(
        param.call,
        [stream](bool) { stream->MaybeDone(/*inlineable_ondone=*/false); },
        stream);

    ServerBidiReactor<RequestType, ResponseType>* reactor = nullptr;
    if (param.status.ok()) {
      reactor = grpc::internal::CatchingReactorGetter<
          ServerBidiReactor<RequestType, ResponseType>>(
          get_reactor_,
          static_cast<grpc::CallbackServerContext*>(param.server_context));
    }

    if (reactor == nullptr) {
      // if deserialization or reactor creator failed, we need to fail the call
      reactor = new (grpc_call_arena_alloc(
          param.call->call(),
          sizeof(UnimplementedBidiReactor<RequestType, ResponseType>)))
          UnimplementedBidiReactor<RequestType, ResponseType>(
              grpc::Status(grpc::StatusCode::UNIMPLEMENTED, ""));
    }

    stream->SetupReactor(reactor);
  }

 private:
  std::function<ServerBidiReactor<RequestType, ResponseType>*(
      grpc::CallbackServerContext*)>
      get_reactor_;

  class ServerCallbackReaderWriterImpl
      : public ServerCallbackReaderWriter<RequestType, ResponseType> {
   public:
    void Finish(grpc::Status s) override {
      // A finish tag with only MaybeDone can have its callback inlined
      // regardless even if OnDone is not inlineable because this callback just
      // checks a ref and then decides whether or not to dispatch OnDone.
      finish_tag_.Set(
          call_.call(),
          [this](bool) {
            // Inlineable OnDone can be false here because there is
            // no bidi reactor that has an inlineable OnDone; this
            // only applies to the DefaultReactor (which is unary).
            this->MaybeDone(/*inlineable_ondone=*/false);
          },
          &finish_ops_, /*can_inline=*/true);
      finish_ops_.set_core_cq_tag(&finish_tag_);

      if (!ctx_->sent_initial_metadata_) {
        finish_ops_.SendInitialMetadata(&ctx_->initial_metadata_,
                                        ctx_->initial_metadata_flags());
        if (ctx_->compression_level_set()) {
          finish_ops_.set_compression_level(ctx_->compression_level());
        }
        ctx_->sent_initial_metadata_ = true;
      }
      finish_ops_.ServerSendStatus(&ctx_->trailing_metadata_, s);
      call_.PerformOps(&finish_ops_);
    }

    void SendInitialMetadata() override {
      ABSL_CHECK(!ctx_->sent_initial_metadata_);
      this->Ref();
      // The callback for this function should not be inlined because it invokes
      // a user-controlled reaction, but any resulting OnDone can be inlined in
      // the executor to which this callback is dispatched.
      meta_tag_.Set(
          call_.call(),
          [this](bool ok) {
            ServerBidiReactor<RequestType, ResponseType>* reactor =
                reactor_.load(std::memory_order_relaxed);
            reactor->OnSendInitialMetadataDone(ok);
            this->MaybeDone(/*inlineable_ondone=*/true);
          },
          &meta_ops_, /*can_inline=*/false);
      meta_ops_.SendInitialMetadata(&ctx_->initial_metadata_,
                                    ctx_->initial_metadata_flags());
      if (ctx_->compression_level_set()) {
        meta_ops_.set_compression_level(ctx_->compression_level());
      }
      ctx_->sent_initial_metadata_ = true;
      meta_ops_.set_core_cq_tag(&meta_tag_);
      call_.PerformOps(&meta_ops_);
    }

    void Write(const ResponseType* resp, grpc::WriteOptions options) override {
      this->Ref();
      if (options.is_last_message()) {
        options.set_buffer_hint();
      }
      if (!ctx_->sent_initial_metadata_) {
        write_ops_.SendInitialMetadata(&ctx_->initial_metadata_,
                                       ctx_->initial_metadata_flags());
        if (ctx_->compression_level_set()) {
          write_ops_.set_compression_level(ctx_->compression_level());
        }
        ctx_->sent_initial_metadata_ = true;
      }
      // TODO(vjpai): don't assert
      ABSL_CHECK(write_ops_.SendMessagePtr(resp, options).ok());
      call_.PerformOps(&write_ops_);
    }

    void WriteAndFinish(const ResponseType* resp, grpc::WriteOptions options,
                        grpc::Status s) override {
      // TODO(vjpai): don't assert
      ABSL_CHECK(finish_ops_.SendMessagePtr(resp, options).ok());
      Finish(std::move(s));
    }

    void Read(RequestType* req) override {
      this->Ref();
      read_ops_.RecvMessage(req);
      call_.PerformOps(&read_ops_);
    }

   private:
    friend class CallbackBidiHandler<RequestType, ResponseType>;

    ServerCallbackReaderWriterImpl(grpc::CallbackServerContext* ctx,
                                   grpc::internal::Call* call,
                                   std::function<void()> call_requester)
        : ctx_(ctx), call_(*call), call_requester_(std::move(call_requester)) {}

    grpc_call* call() override { return call_.call(); }

    void SetupReactor(ServerBidiReactor<RequestType, ResponseType>* reactor) {
      reactor_.store(reactor, std::memory_order_relaxed);
      // The callbacks for these functions should not be inlined because they
      // invoke user-controlled reactions, but any resulting OnDones can be
      // inlined in the executor to which a callback is dispatched.
      write_tag_.Set(
          call_.call(),
          [this, reactor](bool ok) {
            reactor->OnWriteDone(ok);
            this->MaybeDone(/*inlineable_ondone=*/true);
          },
          &write_ops_, /*can_inline=*/false);
      write_ops_.set_core_cq_tag(&write_tag_);
      read_tag_.Set(
          call_.call(),
          [this, reactor](bool ok) {
            if (GPR_UNLIKELY(!ok)) {
              ctx_->MaybeMarkCancelledOnRead();
            }
            reactor->OnReadDone(ok);
            this->MaybeDone(/*inlineable_ondone=*/true);
          },
          &read_ops_, /*can_inline=*/false);
      read_ops_.set_core_cq_tag(&read_tag_);
      this->BindReactor(reactor);
      this->MaybeCallOnCancel(reactor);
      // Inlineable OnDone can be false here because there is no bidi
      // reactor that has an inlineable OnDone; this only applies to the
      // DefaultReactor (which is unary).
      this->MaybeDone(/*inlineable_ondone=*/false);
    }

    void CallOnDone() override {
      reactor_.load(std::memory_order_relaxed)->OnDone();
      grpc_call* call = call_.call();
      auto call_requester = std::move(call_requester_);
      if (ctx_->context_allocator() != nullptr) {
        ctx_->context_allocator()->Release(ctx_);
      }
      this->~ServerCallbackReaderWriterImpl();  // explicitly call destructor
      grpc_call_unref(call);
      call_requester();
    }

    ServerReactor* reactor() override {
      return reactor_.load(std::memory_order_relaxed);
    }

    grpc::internal::CallOpSet<grpc::internal::CallOpSendInitialMetadata>
        meta_ops_;
    grpc::internal::CallbackWithSuccessTag meta_tag_;
    grpc::internal::CallOpSet<grpc::internal::CallOpSendInitialMetadata,
                              grpc::internal::CallOpSendMessage,
                              grpc::internal::CallOpServerSendStatus>
        finish_ops_;
    grpc::internal::CallbackWithSuccessTag finish_tag_;
    grpc::internal::CallOpSet<grpc::internal::CallOpSendInitialMetadata,
                              grpc::internal::CallOpSendMessage>
        write_ops_;
    grpc::internal::CallbackWithSuccessTag write_tag_;
    grpc::internal::CallOpSet<grpc::internal::CallOpRecvMessage<RequestType>>
        read_ops_;
    grpc::internal::CallbackWithSuccessTag read_tag_;

    grpc::CallbackServerContext* const ctx_;
    grpc::internal::Call call_;
    std::function<void()> call_requester_;
    // The memory ordering of reactor_ follows ServerCallbackUnaryImpl.
    std::atomic<ServerBidiReactor<RequestType, ResponseType>*> reactor_;
    // callbacks_outstanding_ follows a refcount pattern
    std::atomic<intptr_t> callbacks_outstanding_{
        3};  // reserve for OnStarted, Finish, and CompletionOp
  };
};

}  // namespace internal
}  // namespace grpc

#endif  // GRPCPP_IMPL_SERVER_CALLBACK_HANDLERS_H
