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

#ifndef GRPCPP_SUPPORT_ASYNC_STREAM_H
#define GRPCPP_SUPPORT_ASYNC_STREAM_H

#include <grpc/grpc.h>
#include <grpcpp/impl/call.h>
#include <grpcpp/impl/channel_interface.h>
#include <grpcpp/impl/service_type.h>
#include <grpcpp/server_context.h>
#include <grpcpp/support/status.h>

#include "absl/log/absl_check.h"

namespace grpc {

namespace internal {
/// Common interface for all client side asynchronous streaming.
class ClientAsyncStreamingInterface {
 public:
  virtual ~ClientAsyncStreamingInterface() {}

  /// Start the call that was set up by the constructor, but only if the
  /// constructor was invoked through the "Prepare" API which doesn't actually
  /// start the call.
  ///
  /// It is illegal to start a write-type operation (eg. Write(), WriteLast(),
  /// WritesDone()) while the `StartCall()` operation has not finished
  /// (determined by the returning of \a tag).
  virtual void StartCall(void* tag) = 0;

  /// Request notification of the reading of the initial metadata. Completion
  /// will be notified by \a tag on the associated completion queue.
  /// This call is optional, but if it is used, it cannot be used concurrently
  /// with or after the \a AsyncReaderInterface::Read method.
  ///
  /// \param[in] tag Tag identifying this request.
  virtual void ReadInitialMetadata(void* tag) = 0;

  /// Indicate that the stream is to be finished and request notification for
  /// when the call has been ended.
  /// Should not be used concurrently with other operations.
  ///
  /// It is appropriate to call this method exactly once when both:
  ///   * the client side has no more message to send
  ///     (this can be declared implicitly by calling this method, or
  ///     explicitly through an earlier call to the <i>WritesDone</i> method
  ///     of the class in use, e.g. \a ClientAsyncWriterInterface::WritesDone or
  ///     \a ClientAsyncReaderWriterInterface::WritesDone).
  ///   * there are no more messages to be received from the server (this can
  ///     be known implicitly by the calling code, or explicitly from an
  ///     earlier call to \a AsyncReaderInterface::Read that yielded a failed
  ///     result, e.g. cq->Next(&read_tag, &ok) filled in 'ok' with 'false').
  ///
  /// The tag will be returned when either:
  /// - all incoming messages have been read and the server has returned
  ///   a status.
  /// - the server has returned a non-OK status.
  /// - the call failed for some reason and the library generated a
  ///   status.
  ///
  /// Note that implementations of this method attempt to receive initial
  /// metadata from the server if initial metadata hasn't yet been received.
  ///
  /// \param[in] tag Tag identifying this request.
  /// \param[out] status To be updated with the operation status.
  virtual void Finish(grpc::Status* status, void* tag) = 0;
};

/// An interface that yields a sequence of messages of type \a R.
template <class R>
class AsyncReaderInterface {
 public:
  virtual ~AsyncReaderInterface() {}

  /// Read a message of type \a R into \a msg. Completion will be notified by \a
  /// tag on the associated completion queue.
  /// This is thread-safe with respect to \a Write or \a WritesDone methods. It
  /// should not be called concurrently with other streaming APIs
  /// on the same stream. It is not meaningful to call it concurrently
  /// with another \a AsyncReaderInterface::Read on the same stream since reads
  /// on the same stream are delivered in order.
  ///
  /// \param[out] msg Where to eventually store the read message.
  /// \param[in] tag The tag identifying the operation.
  ///
  /// Side effect: note that this method attempt to receive initial metadata for
  /// a stream if it hasn't yet been received.
  virtual void Read(R* msg, void* tag) = 0;
};

/// An interface that can be fed a sequence of messages of type \a W.
template <class W>
class AsyncWriterInterface {
 public:
  virtual ~AsyncWriterInterface() {}

  /// Request the writing of \a msg with identifying tag \a tag.
  ///
  /// Only one write may be outstanding at any given time. This means that
  /// after calling Write, one must wait to receive \a tag from the completion
  /// queue BEFORE calling Write again.
  /// This is thread-safe with respect to \a AsyncReaderInterface::Read
  ///
  /// gRPC doesn't take ownership or a reference to \a msg, so it is safe to
  /// to deallocate once Write returns.
  ///
  /// \param[in] msg The message to be written.
  /// \param[in] tag The tag identifying the operation.
  virtual void Write(const W& msg, void* tag) = 0;

  /// Request the writing of \a msg using WriteOptions \a options with
  /// identifying tag \a tag.
  ///
  /// Only one write may be outstanding at any given time. This means that
  /// after calling Write, one must wait to receive \a tag from the completion
  /// queue BEFORE calling Write again.
  /// WriteOptions \a options is used to set the write options of this message.
  /// This is thread-safe with respect to \a AsyncReaderInterface::Read
  ///
  /// gRPC doesn't take ownership or a reference to \a msg, so it is safe to
  /// to deallocate once Write returns.
  ///
  /// \param[in] msg The message to be written.
  /// \param[in] options The WriteOptions to be used to write this message.
  /// \param[in] tag The tag identifying the operation.
  virtual void Write(const W& msg, grpc::WriteOptions options, void* tag) = 0;

  /// Request the writing of \a msg and coalesce it with the writing
  /// of trailing metadata, using WriteOptions \a options with
  /// identifying tag \a tag.
  ///
  /// For client, WriteLast is equivalent of performing Write and
  /// WritesDone in a single step.
  /// For server, WriteLast buffers the \a msg. The writing of \a msg is held
  /// until Finish is called, where \a msg and trailing metadata are coalesced
  /// and write is initiated. Note that WriteLast can only buffer \a msg up to
  /// the flow control window size. If \a msg size is larger than the window
  /// size, it will be sent on wire without buffering.
  ///
  /// gRPC doesn't take ownership or a reference to \a msg, so it is safe to
  /// to deallocate once Write returns.
  ///
  /// \param[in] msg The message to be written.
  /// \param[in] options The WriteOptions to be used to write this message.
  /// \param[in] tag The tag identifying the operation.
  void WriteLast(const W& msg, grpc::WriteOptions options, void* tag) {
    Write(msg, options.set_last_message(), tag);
  }
};

}  // namespace internal

template <class R>
class ClientAsyncReaderInterface
    : public internal::ClientAsyncStreamingInterface,
      public internal::AsyncReaderInterface<R> {};

namespace internal {
template <class R>
class ClientAsyncReaderFactory {
 public:
  /// Create a stream object.
  /// Write the first request out if \a start is set.
  /// \a tag will be notified on \a cq when the call has been started and
  /// \a request has been written out. If \a start is not set, \a tag must be
  /// nullptr and the actual call must be initiated by StartCall
  /// Note that \a context will be used to fill in custom initial metadata
  /// used to send to the server when starting the call.
  template <class W>
  static ClientAsyncReader<R>* Create(grpc::ChannelInterface* channel,
                                      grpc::CompletionQueue* cq,
                                      const grpc::internal::RpcMethod& method,
                                      grpc::ClientContext* context,
                                      const W& request, bool start, void* tag) {
    grpc::internal::Call call = channel->CreateCall(method, context, cq);
    return new (
        grpc_call_arena_alloc(call.call(), sizeof(ClientAsyncReader<R>)))
        ClientAsyncReader<R>(call, context, request, start, tag);
  }
};
}  // namespace internal

/// Async client-side API for doing server-streaming RPCs,
/// where the incoming message stream coming from the server has
/// messages of type \a R.
template <class R>
class ClientAsyncReader final : public ClientAsyncReaderInterface<R> {
 public:
  // always allocated against a call arena, no memory free required
  static void operator delete(void* /*ptr*/, std::size_t size) {
    ABSL_CHECK_EQ(size, sizeof(ClientAsyncReader));
  }

  // This operator should never be called as the memory should be freed as part
  // of the arena destruction. It only exists to provide a matching operator
  // delete to the operator new so that some compilers will not complain (see
  // https://github.com/grpc/grpc/issues/11301) Note at the time of adding this
  // there are no tests catching the compiler warning.
  static void operator delete(void*, void*) { ABSL_CHECK(false); }

  void StartCall(void* tag) override {
    ABSL_CHECK(!started_);
    started_ = true;
    StartCallInternal(tag);
  }

  /// See the \a ClientAsyncStreamingInterface.ReadInitialMetadata
  /// method for semantics.
  ///
  /// Side effect:
  ///   - upon receiving initial metadata from the server,
  ///     the \a ClientContext associated with this call is updated, and the
  ///     calling code can access the received metadata through the
  ///     \a ClientContext.
  void ReadInitialMetadata(void* tag) override {
    ABSL_CHECK(started_);
    ABSL_CHECK(!context_->initial_metadata_received_);

    meta_ops_.set_output_tag(tag);
    meta_ops_.RecvInitialMetadata(context_);
    call_.PerformOps(&meta_ops_);
  }

  void Read(R* msg, void* tag) override {
    ABSL_CHECK(started_);
    read_ops_.set_output_tag(tag);
    if (!context_->initial_metadata_received_) {
      read_ops_.RecvInitialMetadata(context_);
    }
    read_ops_.RecvMessage(msg);
    call_.PerformOps(&read_ops_);
  }

  /// See the \a ClientAsyncStreamingInterface.Finish method for semantics.
  ///
  /// Side effect:
  ///   - the \a ClientContext associated with this call is updated with
  ///     possible initial and trailing metadata received from the server.
  void Finish(grpc::Status* status, void* tag) override {
    ABSL_CHECK(started_);
    finish_ops_.set_output_tag(tag);
    if (!context_->initial_metadata_received_) {
      finish_ops_.RecvInitialMetadata(context_);
    }
    finish_ops_.ClientRecvStatus(context_, status);
    call_.PerformOps(&finish_ops_);
  }

 private:
  friend class internal::ClientAsyncReaderFactory<R>;
  template <class W>
  ClientAsyncReader(grpc::internal::Call call, grpc::ClientContext* context,
                    const W& request, bool start, void* tag)
      : context_(context), call_(call), started_(start) {
    // TODO(ctiller): don't assert
    ABSL_CHECK(init_ops_.SendMessage(request).ok());
    init_ops_.ClientSendClose();
    if (start) {
      StartCallInternal(tag);
    } else {
      ABSL_CHECK(tag == nullptr);
    }
  }

  void StartCallInternal(void* tag) {
    init_ops_.SendInitialMetadata(&context_->send_initial_metadata_,
                                  context_->initial_metadata_flags());
    init_ops_.set_output_tag(tag);
    call_.PerformOps(&init_ops_);
  }

  grpc::ClientContext* context_;
  grpc::internal::Call call_;
  bool started_;
  grpc::internal::CallOpSet<grpc::internal::CallOpSendInitialMetadata,
                            grpc::internal::CallOpSendMessage,
                            grpc::internal::CallOpClientSendClose>
      init_ops_;
  grpc::internal::CallOpSet<grpc::internal::CallOpRecvInitialMetadata>
      meta_ops_;
  grpc::internal::CallOpSet<grpc::internal::CallOpRecvInitialMetadata,
                            grpc::internal::CallOpRecvMessage<R>>
      read_ops_;
  grpc::internal::CallOpSet<grpc::internal::CallOpRecvInitialMetadata,
                            grpc::internal::CallOpClientRecvStatus>
      finish_ops_;
};

/// Common interface for client side asynchronous writing.
template <class W>
class ClientAsyncWriterInterface
    : public internal::ClientAsyncStreamingInterface,
      public internal::AsyncWriterInterface<W> {
 public:
  /// Signal the client is done with the writes (half-close the client stream).
  /// Thread-safe with respect to \a AsyncReaderInterface::Read
  ///
  /// \param[in] tag The tag identifying the operation.
  virtual void WritesDone(void* tag) = 0;
};

namespace internal {
template <class W>
class ClientAsyncWriterFactory {
 public:
  /// Create a stream object.
  /// Start the RPC if \a start is set
  /// \a tag will be notified on \a cq when the call has been started (i.e.
  /// initial metadata sent) and \a request has been written out.
  /// If \a start is not set, \a tag must be nullptr and the actual call
  /// must be initiated by StartCall
  /// Note that \a context will be used to fill in custom initial metadata
  /// used to send to the server when starting the call.
  /// \a response will be filled in with the single expected response
  /// message from the server upon a successful call to the \a Finish
  /// method of this instance.
  template <class R>
  static ClientAsyncWriter<W>* Create(grpc::ChannelInterface* channel,
                                      grpc::CompletionQueue* cq,
                                      const grpc::internal::RpcMethod& method,
                                      grpc::ClientContext* context, R* response,
                                      bool start, void* tag) {
    grpc::internal::Call call = channel->CreateCall(method, context, cq);
    return new (
        grpc_call_arena_alloc(call.call(), sizeof(ClientAsyncWriter<W>)))
        ClientAsyncWriter<W>(call, context, response, start, tag);
  }
};
}  // namespace internal

/// Async API on the client side for doing client-streaming RPCs,
/// where the outgoing message stream going to the server contains
/// messages of type \a W.
template <class W>
class ClientAsyncWriter final : public ClientAsyncWriterInterface<W> {
 public:
  // always allocated against a call arena, no memory free required
  static void operator delete(void* /*ptr*/, std::size_t size) {
    ABSL_CHECK_EQ(size, sizeof(ClientAsyncWriter));
  }

  // This operator should never be called as the memory should be freed as part
  // of the arena destruction. It only exists to provide a matching operator
  // delete to the operator new so that some compilers will not complain (see
  // https://github.com/grpc/grpc/issues/11301) Note at the time of adding this
  // there are no tests catching the compiler warning.
  static void operator delete(void*, void*) { ABSL_CHECK(false); }

  void StartCall(void* tag) override {
    ABSL_CHECK(!started_);
    started_ = true;
    StartCallInternal(tag);
  }

  /// See the \a ClientAsyncStreamingInterface.ReadInitialMetadata method for
  /// semantics.
  ///
  /// Side effect:
  ///   - upon receiving initial metadata from the server, the \a ClientContext
  ///     associated with this call is updated, and the calling code can access
  ///     the received metadata through the \a ClientContext.
  void ReadInitialMetadata(void* tag) override {
    ABSL_CHECK(started_);
    ABSL_CHECK(!context_->initial_metadata_received_);

    meta_ops_.set_output_tag(tag);
    meta_ops_.RecvInitialMetadata(context_);
    call_.PerformOps(&meta_ops_);
  }

  void Write(const W& msg, void* tag) override {
    ABSL_CHECK(started_);
    write_ops_.set_output_tag(tag);
    // TODO(ctiller): don't assert
    ABSL_CHECK(write_ops_.SendMessage(msg).ok());
    call_.PerformOps(&write_ops_);
  }

  void Write(const W& msg, grpc::WriteOptions options, void* tag) override {
    ABSL_CHECK(started_);
    write_ops_.set_output_tag(tag);
    if (options.is_last_message()) {
      options.set_buffer_hint();
      write_ops_.ClientSendClose();
    }
    // TODO(ctiller): don't assert
    ABSL_CHECK(write_ops_.SendMessage(msg, options).ok());
    call_.PerformOps(&write_ops_);
  }

  void WritesDone(void* tag) override {
    ABSL_CHECK(started_);
    write_ops_.set_output_tag(tag);
    write_ops_.ClientSendClose();
    call_.PerformOps(&write_ops_);
  }

  /// See the \a ClientAsyncStreamingInterface.Finish method for semantics.
  ///
  /// Side effect:
  ///   - the \a ClientContext associated with this call is updated with
  ///     possible initial and trailing metadata received from the server.
  ///   - attempts to fill in the \a response parameter passed to this class's
  ///     constructor with the server's response message.
  void Finish(grpc::Status* status, void* tag) override {
    ABSL_CHECK(started_);
    finish_ops_.set_output_tag(tag);
    if (!context_->initial_metadata_received_) {
      finish_ops_.RecvInitialMetadata(context_);
    }
    finish_ops_.ClientRecvStatus(context_, status);
    call_.PerformOps(&finish_ops_);
  }

 private:
  friend class internal::ClientAsyncWriterFactory<W>;
  template <class R>
  ClientAsyncWriter(grpc::internal::Call call, grpc::ClientContext* context,
                    R* response, bool start, void* tag)
      : context_(context), call_(call), started_(start) {
    finish_ops_.RecvMessage(response);
    finish_ops_.AllowNoMessage();
    if (start) {
      StartCallInternal(tag);
    } else {
      ABSL_CHECK(tag == nullptr);
    }
  }

  void StartCallInternal(void* tag) {
    write_ops_.SendInitialMetadata(&context_->send_initial_metadata_,
                                   context_->initial_metadata_flags());
    // if corked bit is set in context, we just keep the initial metadata
    // buffered up to coalesce with later message send. No op is performed.
    if (!context_->initial_metadata_corked_) {
      write_ops_.set_output_tag(tag);
      call_.PerformOps(&write_ops_);
    }
  }

  grpc::ClientContext* context_;
  grpc::internal::Call call_;
  bool started_;
  grpc::internal::CallOpSet<grpc::internal::CallOpRecvInitialMetadata>
      meta_ops_;
  grpc::internal::CallOpSet<grpc::internal::CallOpSendInitialMetadata,
                            grpc::internal::CallOpSendMessage,
                            grpc::internal::CallOpClientSendClose>
      write_ops_;
  grpc::internal::CallOpSet<grpc::internal::CallOpRecvInitialMetadata,
                            grpc::internal::CallOpGenericRecvMessage,
                            grpc::internal::CallOpClientRecvStatus>
      finish_ops_;
};

/// Async client-side interface for bi-directional streaming,
/// where the client-to-server message stream has messages of type \a W,
/// and the server-to-client message stream has messages of type \a R.
template <class W, class R>
class ClientAsyncReaderWriterInterface
    : public internal::ClientAsyncStreamingInterface,
      public internal::AsyncWriterInterface<W>,
      public internal::AsyncReaderInterface<R> {
 public:
  /// Signal the client is done with the writes (half-close the client stream).
  /// Thread-safe with respect to \a AsyncReaderInterface::Read
  ///
  /// \param[in] tag The tag identifying the operation.
  virtual void WritesDone(void* tag) = 0;
};

namespace internal {
template <class W, class R>
class ClientAsyncReaderWriterFactory {
 public:
  /// Create a stream object.
  /// Start the RPC request if \a start is set.
  /// \a tag will be notified on \a cq when the call has been started (i.e.
  /// initial metadata sent). If \a start is not set, \a tag must be
  /// nullptr and the actual call must be initiated by StartCall
  /// Note that \a context will be used to fill in custom initial metadata
  /// used to send to the server when starting the call.
  static ClientAsyncReaderWriter<W, R>* Create(
      grpc::ChannelInterface* channel, grpc::CompletionQueue* cq,
      const grpc::internal::RpcMethod& method, grpc::ClientContext* context,
      bool start, void* tag) {
    grpc::internal::Call call = channel->CreateCall(method, context, cq);

    return new (grpc_call_arena_alloc(call.call(),
                                      sizeof(ClientAsyncReaderWriter<W, R>)))
        ClientAsyncReaderWriter<W, R>(call, context, start, tag);
  }
};
}  // namespace internal

/// Async client-side interface for bi-directional streaming,
/// where the outgoing message stream going to the server
/// has messages of type \a W,  and the incoming message stream coming
/// from the server has messages of type \a R.
template <class W, class R>
class ClientAsyncReaderWriter final
    : public ClientAsyncReaderWriterInterface<W, R> {
 public:
  // always allocated against a call arena, no memory free required
  static void operator delete(void* /*ptr*/, std::size_t size) {
    ABSL_CHECK_EQ(size, sizeof(ClientAsyncReaderWriter));
  }

  // This operator should never be called as the memory should be freed as part
  // of the arena destruction. It only exists to provide a matching operator
  // delete to the operator new so that some compilers will not complain (see
  // https://github.com/grpc/grpc/issues/11301) Note at the time of adding this
  // there are no tests catching the compiler warning.
  static void operator delete(void*, void*) { ABSL_CHECK(false); }

  void StartCall(void* tag) override {
    ABSL_CHECK(!started_);
    started_ = true;
    StartCallInternal(tag);
  }

  /// See the \a ClientAsyncStreamingInterface.ReadInitialMetadata method
  /// for semantics of this method.
  ///
  /// Side effect:
  ///   - upon receiving initial metadata from the server, the \a ClientContext
  ///     is updated with it, and then the receiving initial metadata can
  ///     be accessed through this \a ClientContext.
  void ReadInitialMetadata(void* tag) override {
    ABSL_CHECK(started_);
    ABSL_CHECK(!context_->initial_metadata_received_);

    meta_ops_.set_output_tag(tag);
    meta_ops_.RecvInitialMetadata(context_);
    call_.PerformOps(&meta_ops_);
  }

  void Read(R* msg, void* tag) override {
    ABSL_CHECK(started_);
    read_ops_.set_output_tag(tag);
    if (!context_->initial_metadata_received_) {
      read_ops_.RecvInitialMetadata(context_);
    }
    read_ops_.RecvMessage(msg);
    call_.PerformOps(&read_ops_);
  }

  void Write(const W& msg, void* tag) override {
    ABSL_CHECK(started_);
    write_ops_.set_output_tag(tag);
    // TODO(ctiller): don't assert
    ABSL_CHECK(write_ops_.SendMessage(msg).ok());
    call_.PerformOps(&write_ops_);
  }

  void Write(const W& msg, grpc::WriteOptions options, void* tag) override {
    ABSL_CHECK(started_);
    write_ops_.set_output_tag(tag);
    if (options.is_last_message()) {
      options.set_buffer_hint();
      write_ops_.ClientSendClose();
    }
    // TODO(ctiller): don't assert
    ABSL_CHECK(write_ops_.SendMessage(msg, options).ok());
    call_.PerformOps(&write_ops_);
  }

  void WritesDone(void* tag) override {
    ABSL_CHECK(started_);
    write_ops_.set_output_tag(tag);
    write_ops_.ClientSendClose();
    call_.PerformOps(&write_ops_);
  }

  /// See the \a ClientAsyncStreamingInterface.Finish method for semantics.
  /// Side effect
  ///   - the \a ClientContext associated with this call is updated with
  ///     possible initial and trailing metadata sent from the server.
  void Finish(grpc::Status* status, void* tag) override {
    ABSL_CHECK(started_);
    finish_ops_.set_output_tag(tag);
    if (!context_->initial_metadata_received_) {
      finish_ops_.RecvInitialMetadata(context_);
    }
    finish_ops_.ClientRecvStatus(context_, status);
    call_.PerformOps(&finish_ops_);
  }

 private:
  friend class internal::ClientAsyncReaderWriterFactory<W, R>;
  ClientAsyncReaderWriter(grpc::internal::Call call,
                          grpc::ClientContext* context, bool start, void* tag)
      : context_(context), call_(call), started_(start) {
    if (start) {
      StartCallInternal(tag);
    } else {
      ABSL_CHECK(tag == nullptr);
    }
  }

  void StartCallInternal(void* tag) {
    write_ops_.SendInitialMetadata(&context_->send_initial_metadata_,
                                   context_->initial_metadata_flags());
    // if corked bit is set in context, we just keep the initial metadata
    // buffered up to coalesce with later message send. No op is performed.
    if (!context_->initial_metadata_corked_) {
      write_ops_.set_output_tag(tag);
      call_.PerformOps(&write_ops_);
    }
  }

  grpc::ClientContext* context_;
  grpc::internal::Call call_;
  bool started_;
  grpc::internal::CallOpSet<grpc::internal::CallOpRecvInitialMetadata>
      meta_ops_;
  grpc::internal::CallOpSet<grpc::internal::CallOpRecvInitialMetadata,
                            grpc::internal::CallOpRecvMessage<R>>
      read_ops_;
  grpc::internal::CallOpSet<grpc::internal::CallOpSendInitialMetadata,
                            grpc::internal::CallOpSendMessage,
                            grpc::internal::CallOpClientSendClose>
      write_ops_;
  grpc::internal::CallOpSet<grpc::internal::CallOpRecvInitialMetadata,
                            grpc::internal::CallOpClientRecvStatus>
      finish_ops_;
};

template <class W, class R>
class ServerAsyncReaderInterface
    : public grpc::internal::ServerAsyncStreamingInterface,
      public internal::AsyncReaderInterface<R> {
 public:
  /// Indicate that the stream is to be finished with a certain status code
  /// and also send out \a msg response to the client.
  /// Request notification for when the server has sent the response and the
  /// appropriate signals to the client to end the call.
  /// Should not be used concurrently with other operations.
  ///
  /// It is appropriate to call this method when:
  ///   * all messages from the client have been received (either known
  ///     implicitly, or explicitly because a previous
  ///     \a AsyncReaderInterface::Read operation with a non-ok result,
  ///     e.g., cq->Next(&read_tag, &ok) filled in 'ok' with 'false').
  ///
  /// This operation will end when the server has finished sending out initial
  /// metadata (if not sent already), response message, and status, or if
  /// some failure occurred when trying to do so.
  ///
  /// gRPC doesn't take ownership or a reference to \a msg or \a status, so it
  /// is safe to deallocate once Finish returns.
  ///
  /// \param[in] tag Tag identifying this request.
  /// \param[in] status To be sent to the client as the result of this call.
  /// \param[in] msg To be sent to the client as the response for this call.
  virtual void Finish(const W& msg, const grpc::Status& status, void* tag) = 0;

  /// Indicate that the stream is to be finished with a certain
  /// non-OK status code.
  /// Request notification for when the server has sent the appropriate
  /// signals to the client to end the call.
  /// Should not be used concurrently with other operations.
  ///
  /// This call is meant to end the call with some error, and can be called at
  /// any point that the server would like to "fail" the call (though note
  /// this shouldn't be called concurrently with any other "sending" call, like
  /// \a AsyncWriterInterface::Write).
  ///
  /// This operation will end when the server has finished sending out initial
  /// metadata (if not sent already), and status, or if some failure occurred
  /// when trying to do so.
  ///
  /// gRPC doesn't take ownership or a reference to \a status, so it is safe to
  /// to deallocate once FinishWithError returns.
  ///
  /// \param[in] tag Tag identifying this request.
  /// \param[in] status To be sent to the client as the result of this call.
  ///     - Note: \a status must have a non-OK code.
  virtual void FinishWithError(const grpc::Status& status, void* tag) = 0;
};

/// Async server-side API for doing client-streaming RPCs,
/// where the incoming message stream from the client has messages of type \a R,
/// and the single response message sent from the server is type \a W.
template <class W, class R>
class ServerAsyncReader final : public ServerAsyncReaderInterface<W, R> {
 public:
  explicit ServerAsyncReader(grpc::ServerContext* ctx)
      : call_(nullptr, nullptr, nullptr), ctx_(ctx) {}

  /// See \a ServerAsyncStreamingInterface::SendInitialMetadata for semantics.
  ///
  /// Implicit input parameter:
  ///   - The initial metadata that will be sent to the client from this op will
  ///     be taken from the \a ServerContext associated with the call.
  void SendInitialMetadata(void* tag) override {
    ABSL_CHECK(!ctx_->sent_initial_metadata_);

    meta_ops_.set_output_tag(tag);
    meta_ops_.SendInitialMetadata(&ctx_->initial_metadata_,
                                  ctx_->initial_metadata_flags());
    if (ctx_->compression_level_set()) {
      meta_ops_.set_compression_level(ctx_->compression_level());
    }
    ctx_->sent_initial_metadata_ = true;
    call_.PerformOps(&meta_ops_);
  }

  void Read(R* msg, void* tag) override {
    read_ops_.set_output_tag(tag);
    read_ops_.RecvMessage(msg);
    call_.PerformOps(&read_ops_);
  }

  /// See the \a ServerAsyncReaderInterface.Read method for semantics
  ///
  /// Side effect:
  ///   - also sends initial metadata if not already sent.
  ///   - uses the \a ServerContext associated with this call to send possible
  ///     initial and trailing metadata.
  ///
  /// Note: \a msg is not sent if \a status has a non-OK code.
  ///
  /// gRPC doesn't take ownership or a reference to \a msg and \a status, so it
  /// is safe to deallocate once Finish returns.
  void Finish(const W& msg, const grpc::Status& status, void* tag) override {
    finish_ops_.set_output_tag(tag);
    if (!ctx_->sent_initial_metadata_) {
      finish_ops_.SendInitialMetadata(&ctx_->initial_metadata_,
                                      ctx_->initial_metadata_flags());
      if (ctx_->compression_level_set()) {
        finish_ops_.set_compression_level(ctx_->compression_level());
      }
      ctx_->sent_initial_metadata_ = true;
    }
    // The response is dropped if the status is not OK.
    if (status.ok()) {
      finish_ops_.ServerSendStatus(&ctx_->trailing_metadata_,
                                   finish_ops_.SendMessage(msg));
    } else {
      finish_ops_.ServerSendStatus(&ctx_->trailing_metadata_, status);
    }
    call_.PerformOps(&finish_ops_);
  }

  /// See the \a ServerAsyncReaderInterface.Read method for semantics
  ///
  /// Side effect:
  ///   - also sends initial metadata if not already sent.
  ///   - uses the \a ServerContext associated with this call to send possible
  ///     initial and trailing metadata.
  ///
  /// gRPC doesn't take ownership or a reference to \a status, so it is safe to
  /// to deallocate once FinishWithError returns.
  void FinishWithError(const grpc::Status& status, void* tag) override {
    ABSL_CHECK(!status.ok());
    finish_ops_.set_output_tag(tag);
    if (!ctx_->sent_initial_metadata_) {
      finish_ops_.SendInitialMetadata(&ctx_->initial_metadata_,
                                      ctx_->initial_metadata_flags());
      if (ctx_->compression_level_set()) {
        finish_ops_.set_compression_level(ctx_->compression_level());
      }
      ctx_->sent_initial_metadata_ = true;
    }
    finish_ops_.ServerSendStatus(&ctx_->trailing_metadata_, status);
    call_.PerformOps(&finish_ops_);
  }

 private:
  void BindCall(grpc::internal::Call* call) override { call_ = *call; }

  grpc::internal::Call call_;
  grpc::ServerContext* ctx_;
  grpc::internal::CallOpSet<grpc::internal::CallOpSendInitialMetadata>
      meta_ops_;
  grpc::internal::CallOpSet<grpc::internal::CallOpRecvMessage<R>> read_ops_;
  grpc::internal::CallOpSet<grpc::internal::CallOpSendInitialMetadata,
                            grpc::internal::CallOpSendMessage,
                            grpc::internal::CallOpServerSendStatus>
      finish_ops_;
};

template <class W>
class ServerAsyncWriterInterface
    : public grpc::internal::ServerAsyncStreamingInterface,
      public internal::AsyncWriterInterface<W> {
 public:
  /// Indicate that the stream is to be finished with a certain status code.
  /// Request notification for when the server has sent the appropriate
  /// signals to the client to end the call.
  /// Should not be used concurrently with other operations.
  ///
  /// It is appropriate to call this method when either:
  ///   * all messages from the client have been received (either known
  ///     implicitly, or explicitly because a previous \a
  ///     AsyncReaderInterface::Read operation with a non-ok
  ///     result (e.g., cq->Next(&read_tag, &ok) filled in 'ok' with 'false'.
  ///   * it is desired to end the call early with some non-OK status code.
  ///
  /// This operation will end when the server has finished sending out initial
  /// metadata (if not sent already), response message, and status, or if
  /// some failure occurred when trying to do so.
  ///
  /// gRPC doesn't take ownership or a reference to \a status, so it is safe to
  /// to deallocate once Finish returns.
  ///
  /// \param[in] tag Tag identifying this request.
  /// \param[in] status To be sent to the client as the result of this call.
  virtual void Finish(const grpc::Status& status, void* tag) = 0;

  /// Request the writing of \a msg and coalesce it with trailing metadata which
  /// contains \a status, using WriteOptions options with
  /// identifying tag \a tag.
  ///
  /// WriteAndFinish is equivalent of performing WriteLast and Finish
  /// in a single step.
  ///
  /// gRPC doesn't take ownership or a reference to \a msg and \a status, so it
  /// is safe to deallocate once WriteAndFinish returns.
  ///
  /// \param[in] msg The message to be written.
  /// \param[in] options The WriteOptions to be used to write this message.
  /// \param[in] status The Status that server returns to client.
  /// \param[in] tag The tag identifying the operation.
  virtual void WriteAndFinish(const W& msg, grpc::WriteOptions options,
                              const grpc::Status& status, void* tag) = 0;
};

/// Async server-side API for doing server streaming RPCs,
/// where the outgoing message stream from the server has messages of type \a W.
template <class W>
class ServerAsyncWriter final : public ServerAsyncWriterInterface<W> {
 public:
  explicit ServerAsyncWriter(grpc::ServerContext* ctx)
      : call_(nullptr, nullptr, nullptr), ctx_(ctx) {}

  /// See \a ServerAsyncStreamingInterface::SendInitialMetadata for semantics.
  ///
  /// Implicit input parameter:
  ///   - The initial metadata that will be sent to the client from this op will
  ///     be taken from the \a ServerContext associated with the call.
  ///
  /// \param[in] tag Tag identifying this request.
  void SendInitialMetadata(void* tag) override {
    ABSL_CHECK(!ctx_->sent_initial_metadata_);

    meta_ops_.set_output_tag(tag);
    meta_ops_.SendInitialMetadata(&ctx_->initial_metadata_,
                                  ctx_->initial_metadata_flags());
    if (ctx_->compression_level_set()) {
      meta_ops_.set_compression_level(ctx_->compression_level());
    }
    ctx_->sent_initial_metadata_ = true;
    call_.PerformOps(&meta_ops_);
  }

  void Write(const W& msg, void* tag) override {
    write_ops_.set_output_tag(tag);
    EnsureInitialMetadataSent(&write_ops_);
    // TODO(ctiller): don't assert
    ABSL_CHECK(write_ops_.SendMessage(msg).ok());
    call_.PerformOps(&write_ops_);
  }

  void Write(const W& msg, grpc::WriteOptions options, void* tag) override {
    write_ops_.set_output_tag(tag);
    if (options.is_last_message()) {
      options.set_buffer_hint();
    }

    EnsureInitialMetadataSent(&write_ops_);
    // TODO(ctiller): don't assert
    ABSL_CHECK(write_ops_.SendMessage(msg, options).ok());
    call_.PerformOps(&write_ops_);
  }

  /// See the \a ServerAsyncWriterInterface.WriteAndFinish method for semantics.
  ///
  /// Implicit input parameter:
  ///   - the \a ServerContext associated with this call is used
  ///     for sending trailing (and initial) metadata to the client.
  ///
  /// Note: \a status must have an OK code.
  ///
  /// gRPC doesn't take ownership or a reference to \a msg and \a status, so it
  /// is safe to deallocate once WriteAndFinish returns.
  void WriteAndFinish(const W& msg, grpc::WriteOptions options,
                      const grpc::Status& status, void* tag) override {
    write_ops_.set_output_tag(tag);
    EnsureInitialMetadataSent(&write_ops_);
    options.set_buffer_hint();
    ABSL_CHECK(write_ops_.SendMessage(msg, options).ok());
    write_ops_.ServerSendStatus(&ctx_->trailing_metadata_, status);
    call_.PerformOps(&write_ops_);
  }

  /// See the \a ServerAsyncWriterInterface.Finish method for semantics.
  ///
  /// Implicit input parameter:
  ///   - the \a ServerContext associated with this call is used for sending
  ///     trailing (and initial if not already sent) metadata to the client.
  ///
  /// Note: there are no restrictions are the code of
  /// \a status,it may be non-OK
  ///
  /// gRPC doesn't take ownership or a reference to \a status, so it is safe to
  /// to deallocate once Finish returns.
  void Finish(const grpc::Status& status, void* tag) override {
    finish_ops_.set_output_tag(tag);
    EnsureInitialMetadataSent(&finish_ops_);
    finish_ops_.ServerSendStatus(&ctx_->trailing_metadata_, status);
    call_.PerformOps(&finish_ops_);
  }

 private:
  void BindCall(grpc::internal::Call* call) override { call_ = *call; }

  template <class T>
  void EnsureInitialMetadataSent(T* ops) {
    if (!ctx_->sent_initial_metadata_) {
      ops->SendInitialMetadata(&ctx_->initial_metadata_,
                               ctx_->initial_metadata_flags());
      if (ctx_->compression_level_set()) {
        ops->set_compression_level(ctx_->compression_level());
      }
      ctx_->sent_initial_metadata_ = true;
    }
  }

  grpc::internal::Call call_;
  grpc::ServerContext* ctx_;
  grpc::internal::CallOpSet<grpc::internal::CallOpSendInitialMetadata>
      meta_ops_;
  grpc::internal::CallOpSet<grpc::internal::CallOpSendInitialMetadata,
                            grpc::internal::CallOpSendMessage,
                            grpc::internal::CallOpServerSendStatus>
      write_ops_;
  grpc::internal::CallOpSet<grpc::internal::CallOpSendInitialMetadata,
                            grpc::internal::CallOpServerSendStatus>
      finish_ops_;
};

/// Server-side interface for asynchronous bi-directional streaming.
template <class W, class R>
class ServerAsyncReaderWriterInterface
    : public grpc::internal::ServerAsyncStreamingInterface,
      public internal::AsyncWriterInterface<W>,
      public internal::AsyncReaderInterface<R> {
 public:
  /// Indicate that the stream is to be finished with a certain status code.
  /// Request notification for when the server has sent the appropriate
  /// signals to the client to end the call.
  /// Should not be used concurrently with other operations.
  ///
  /// It is appropriate to call this method when either:
  ///   * all messages from the client have been received (either known
  ///     implicitly, or explicitly because a previous \a
  ///     AsyncReaderInterface::Read operation
  ///     with a non-ok result (e.g., cq->Next(&read_tag, &ok) filled in 'ok'
  ///     with 'false'.
  ///   * it is desired to end the call early with some non-OK status code.
  ///
  /// This operation will end when the server has finished sending out initial
  /// metadata (if not sent already), response message, and status, or if some
  /// failure occurred when trying to do so.
  ///
  /// gRPC doesn't take ownership or a reference to \a status, so it is safe to
  /// to deallocate once Finish returns.
  ///
  /// \param[in] tag Tag identifying this request.
  /// \param[in] status To be sent to the client as the result of this call.
  virtual void Finish(const grpc::Status& status, void* tag) = 0;

  /// Request the writing of \a msg and coalesce it with trailing metadata which
  /// contains \a status, using WriteOptions options with
  /// identifying tag \a tag.
  ///
  /// WriteAndFinish is equivalent of performing WriteLast and Finish in a
  /// single step.
  ///
  /// gRPC doesn't take ownership or a reference to \a msg and \a status, so it
  /// is safe to deallocate once WriteAndFinish returns.
  ///
  /// \param[in] msg The message to be written.
  /// \param[in] options The WriteOptions to be used to write this message.
  /// \param[in] status The Status that server returns to client.
  /// \param[in] tag The tag identifying the operation.
  virtual void WriteAndFinish(const W& msg, grpc::WriteOptions options,
                              const grpc::Status& status, void* tag) = 0;
};

/// Async server-side API for doing bidirectional streaming RPCs,
/// where the incoming message stream coming from the client has messages of
/// type \a R, and the outgoing message stream coming from the server has
/// messages of type \a W.
template <class W, class R>
class ServerAsyncReaderWriter final
    : public ServerAsyncReaderWriterInterface<W, R> {
 public:
  explicit ServerAsyncReaderWriter(grpc::ServerContext* ctx)
      : call_(nullptr, nullptr, nullptr), ctx_(ctx) {}

  /// See \a ServerAsyncStreamingInterface::SendInitialMetadata for semantics.
  ///
  /// Implicit input parameter:
  ///   - The initial metadata that will be sent to the client from this op will
  ///     be taken from the \a ServerContext associated with the call.
  ///
  /// \param[in] tag Tag identifying this request.
  void SendInitialMetadata(void* tag) override {
    ABSL_CHECK(!ctx_->sent_initial_metadata_);

    meta_ops_.set_output_tag(tag);
    meta_ops_.SendInitialMetadata(&ctx_->initial_metadata_,
                                  ctx_->initial_metadata_flags());
    if (ctx_->compression_level_set()) {
      meta_ops_.set_compression_level(ctx_->compression_level());
    }
    ctx_->sent_initial_metadata_ = true;
    call_.PerformOps(&meta_ops_);
  }

  void Read(R* msg, void* tag) override {
    read_ops_.set_output_tag(tag);
    read_ops_.RecvMessage(msg);
    call_.PerformOps(&read_ops_);
  }

  void Write(const W& msg, void* tag) override {
    write_ops_.set_output_tag(tag);
    EnsureInitialMetadataSent(&write_ops_);
    // TODO(ctiller): don't assert
    ABSL_CHECK(write_ops_.SendMessage(msg).ok());
    call_.PerformOps(&write_ops_);
  }

  void Write(const W& msg, grpc::WriteOptions options, void* tag) override {
    write_ops_.set_output_tag(tag);
    if (options.is_last_message()) {
      options.set_buffer_hint();
    }
    EnsureInitialMetadataSent(&write_ops_);
    ABSL_CHECK(write_ops_.SendMessage(msg, options).ok());
    call_.PerformOps(&write_ops_);
  }

  /// See the \a ServerAsyncReaderWriterInterface.WriteAndFinish
  /// method for semantics.
  ///
  /// Implicit input parameter:
  ///   - the \a ServerContext associated with this call is used
  ///     for sending trailing (and initial) metadata to the client.
  ///
  /// Note: \a status must have an OK code.
  //
  /// gRPC doesn't take ownership or a reference to \a msg and \a status, so it
  /// is safe to deallocate once WriteAndFinish returns.
  void WriteAndFinish(const W& msg, grpc::WriteOptions options,
                      const grpc::Status& status, void* tag) override {
    write_ops_.set_output_tag(tag);
    EnsureInitialMetadataSent(&write_ops_);
    options.set_buffer_hint();
    ABSL_CHECK(write_ops_.SendMessage(msg, options).ok());
    write_ops_.ServerSendStatus(&ctx_->trailing_metadata_, status);
    call_.PerformOps(&write_ops_);
  }

  /// See the \a ServerAsyncReaderWriterInterface.Finish method for semantics.
  ///
  /// Implicit input parameter:
  ///   - the \a ServerContext associated with this call is used for sending
  ///     trailing (and initial if not already sent) metadata to the client.
  ///
  /// Note: there are no restrictions are the code of \a status,
  /// it may be non-OK
  //
  /// gRPC doesn't take ownership or a reference to \a status, so it is safe to
  /// to deallocate once Finish returns.
  void Finish(const grpc::Status& status, void* tag) override {
    finish_ops_.set_output_tag(tag);
    EnsureInitialMetadataSent(&finish_ops_);

    finish_ops_.ServerSendStatus(&ctx_->trailing_metadata_, status);
    call_.PerformOps(&finish_ops_);
  }

 private:
  friend class grpc::Server;

  void BindCall(grpc::internal::Call* call) override { call_ = *call; }

  template <class T>
  void EnsureInitialMetadataSent(T* ops) {
    if (!ctx_->sent_initial_metadata_) {
      ops->SendInitialMetadata(&ctx_->initial_metadata_,
                               ctx_->initial_metadata_flags());
      if (ctx_->compression_level_set()) {
        ops->set_compression_level(ctx_->compression_level());
      }
      ctx_->sent_initial_metadata_ = true;
    }
  }

  grpc::internal::Call call_;
  grpc::ServerContext* ctx_;
  grpc::internal::CallOpSet<grpc::internal::CallOpSendInitialMetadata>
      meta_ops_;
  grpc::internal::CallOpSet<grpc::internal::CallOpRecvMessage<R>> read_ops_;
  grpc::internal::CallOpSet<grpc::internal::CallOpSendInitialMetadata,
                            grpc::internal::CallOpSendMessage,
                            grpc::internal::CallOpServerSendStatus>
      write_ops_;
  grpc::internal::CallOpSet<grpc::internal::CallOpSendInitialMetadata,
                            grpc::internal::CallOpServerSendStatus>
      finish_ops_;
};

}  // namespace grpc

#endif  // GRPCPP_SUPPORT_ASYNC_STREAM_H
