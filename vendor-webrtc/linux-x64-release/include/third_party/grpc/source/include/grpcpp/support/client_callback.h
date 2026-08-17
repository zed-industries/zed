//
//
// Copyright 2018 gRPC authors.
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

#ifndef GRPCPP_SUPPORT_CLIENT_CALLBACK_H
#define GRPCPP_SUPPORT_CLIENT_CALLBACK_H

#include <grpc/grpc.h>
#include <grpc/impl/call.h>
#include <grpcpp/impl/call.h>
#include <grpcpp/impl/call_op_set.h>
#include <grpcpp/impl/sync.h>
#include <grpcpp/support/callback_common.h>
#include <grpcpp/support/config.h>
#include <grpcpp/support/status.h>

#include <atomic>
#include <functional>

#include "absl/log/absl_check.h"

namespace grpc {
class Channel;
class ClientContext;

namespace internal {
class RpcMethod;

/// Perform a callback-based unary call.  May optionally specify the base
/// class of the Request and Response so that the internal calls and structures
/// below this may be based on those base classes and thus achieve code reuse
/// across different RPCs (e.g., for protobuf, MessageLite would be a base
/// class).
/// TODO(vjpai): Combine as much as possible with the blocking unary call code
template <class InputMessage, class OutputMessage,
          class BaseInputMessage = InputMessage,
          class BaseOutputMessage = OutputMessage>
void CallbackUnaryCall(grpc::ChannelInterface* channel,
                       const grpc::internal::RpcMethod& method,
                       grpc::ClientContext* context,
                       const InputMessage* request, OutputMessage* result,
                       std::function<void(grpc::Status)> on_completion) {
  static_assert(std::is_base_of<BaseInputMessage, InputMessage>::value,
                "Invalid input message specification");
  static_assert(std::is_base_of<BaseOutputMessage, OutputMessage>::value,
                "Invalid output message specification");
  CallbackUnaryCallImpl<BaseInputMessage, BaseOutputMessage> x(
      channel, method, context, request, result, on_completion);
}

template <class InputMessage, class OutputMessage>
class CallbackUnaryCallImpl {
 public:
  CallbackUnaryCallImpl(grpc::ChannelInterface* channel,
                        const grpc::internal::RpcMethod& method,
                        grpc::ClientContext* context,
                        const InputMessage* request, OutputMessage* result,
                        std::function<void(grpc::Status)> on_completion) {
    grpc::CompletionQueue* cq = channel->CallbackCQ();
    ABSL_CHECK_NE(cq, nullptr);
    grpc::internal::Call call(channel->CreateCall(method, context, cq));

    using FullCallOpSet = grpc::internal::CallOpSet<
        grpc::internal::CallOpSendInitialMetadata,
        grpc::internal::CallOpSendMessage,
        grpc::internal::CallOpRecvInitialMetadata,
        grpc::internal::CallOpRecvMessage<OutputMessage>,
        grpc::internal::CallOpClientSendClose,
        grpc::internal::CallOpClientRecvStatus>;

    struct OpSetAndTag {
      FullCallOpSet opset;
      grpc::internal::CallbackWithStatusTag tag;
    };
    const size_t alloc_sz = sizeof(OpSetAndTag);
    auto* const alloced =
        static_cast<OpSetAndTag*>(grpc_call_arena_alloc(call.call(), alloc_sz));
    auto* ops = new (&alloced->opset) FullCallOpSet;
    auto* tag = new (&alloced->tag)
        grpc::internal::CallbackWithStatusTag(call.call(), on_completion, ops);

    // TODO(vjpai): Unify code with sync API as much as possible
    grpc::Status s = ops->SendMessagePtr(request);
    if (!s.ok()) {
      tag->force_run(s);
      return;
    }
    ops->SendInitialMetadata(&context->send_initial_metadata_,
                             context->initial_metadata_flags());
    ops->RecvInitialMetadata(context);
    ops->RecvMessage(result);
    ops->AllowNoMessage();
    ops->ClientSendClose();
    ops->ClientRecvStatus(context, tag->status_ptr());
    ops->set_core_cq_tag(tag);
    call.PerformOps(ops);
  }
};

// Base class for public API classes.
class ClientReactor {
 public:
  virtual ~ClientReactor() = default;

  /// Called by the library when all operations associated with this RPC have
  /// completed and all Holds have been removed. OnDone provides the RPC status
  /// outcome for both successful and failed RPCs. If it is never called on an
  /// RPC, it indicates an application-level problem (like failure to remove a
  /// hold).
  ///
  /// \param[in] s The status outcome of this RPC
  virtual void OnDone(const grpc::Status& /*s*/) = 0;

  /// InternalTrailersOnly is not part of the API and is not meant to be
  /// overridden. It is virtual to allow successful builds for certain bazel
  /// build users that only want to depend on gRPC codegen headers and not the
  /// full library (although this is not a generally-supported option). Although
  /// the virtual call is slower than a direct call, this function is
  /// heavyweight and the cost of the virtual call is not much in comparison.
  /// This function may be removed or devirtualized in the future.
  virtual bool InternalTrailersOnly(const grpc_call* call) const;
};

}  // namespace internal

// Forward declarations
template <class Request, class Response>
class ClientBidiReactor;
template <class Response>
class ClientReadReactor;
template <class Request>
class ClientWriteReactor;
class ClientUnaryReactor;

// NOTE: The streaming objects are not actually implemented in the public API.
//       These interfaces are provided for mocking only. Typical applications
//       will interact exclusively with the reactors that they define.
template <class Request, class Response>
class ClientCallbackReaderWriter {
 public:
  virtual ~ClientCallbackReaderWriter() {}
  virtual void StartCall() = 0;
  virtual void Write(const Request* req, grpc::WriteOptions options) = 0;
  virtual void WritesDone() = 0;
  virtual void Read(Response* resp) = 0;
  virtual void AddHold(int holds) = 0;
  virtual void RemoveHold() = 0;

 protected:
  void BindReactor(ClientBidiReactor<Request, Response>* reactor) {
    reactor->BindStream(this);
  }
};

template <class Response>
class ClientCallbackReader {
 public:
  virtual ~ClientCallbackReader() {}
  virtual void StartCall() = 0;
  virtual void Read(Response* resp) = 0;
  virtual void AddHold(int holds) = 0;
  virtual void RemoveHold() = 0;

 protected:
  void BindReactor(ClientReadReactor<Response>* reactor) {
    reactor->BindReader(this);
  }
};

template <class Request>
class ClientCallbackWriter {
 public:
  virtual ~ClientCallbackWriter() {}
  virtual void StartCall() = 0;
  void Write(const Request* req) { Write(req, grpc::WriteOptions()); }
  virtual void Write(const Request* req, grpc::WriteOptions options) = 0;
  void WriteLast(const Request* req, grpc::WriteOptions options) {
    Write(req, options.set_last_message());
  }
  virtual void WritesDone() = 0;

  virtual void AddHold(int holds) = 0;
  virtual void RemoveHold() = 0;

 protected:
  void BindReactor(ClientWriteReactor<Request>* reactor) {
    reactor->BindWriter(this);
  }
};

class ClientCallbackUnary {
 public:
  virtual ~ClientCallbackUnary() {}
  virtual void StartCall() = 0;

 protected:
  void BindReactor(ClientUnaryReactor* reactor);
};

// The following classes are the reactor interfaces that are to be implemented
// by the user. They are passed in to the library as an argument to a call on a
// stub (either a codegen-ed call or a generic call). The streaming RPC is
// activated by calling StartCall, possibly after initiating StartRead,
// StartWrite, or AddHold operations on the streaming object. Note that none of
// the classes are pure; all reactions have a default empty reaction so that the
// user class only needs to override those reactions that it cares about.
// The reactor must be passed to the stub invocation before any of the below
// operations can be called and its reactions will be invoked by the library in
// response to the completion of various operations. Reactions must not include
// blocking operations (such as blocking I/O, starting synchronous RPCs, or
// waiting on condition variables). Reactions may be invoked concurrently,
// except that OnDone is called after all others (assuming proper API usage).
// The reactor may not be deleted until OnDone is called.

/// \a ClientBidiReactor is the interface for a bidirectional streaming RPC.
template <class Request, class Response>
class ClientBidiReactor : public internal::ClientReactor {
 public:
  /// Activate the RPC and initiate any reads or writes that have been Start'ed
  /// before this call. All streaming RPCs issued by the client MUST have
  /// StartCall invoked on them (even if they are canceled) as this call is the
  /// activation of their lifecycle.
  void StartCall() { stream_->StartCall(); }

  /// Initiate a read operation (or post it for later initiation if StartCall
  /// has not yet been invoked).
  ///
  /// \param[out] resp Where to eventually store the read message. Valid when
  ///                  the library calls OnReadDone
  void StartRead(Response* resp) { stream_->Read(resp); }

  /// Initiate a write operation (or post it for later initiation if StartCall
  /// has not yet been invoked).
  ///
  /// \param[in] req The message to be written. The library does not take
  ///                ownership but the caller must ensure that the message is
  ///                not deleted or modified until OnWriteDone is called.
  void StartWrite(const Request* req) { StartWrite(req, grpc::WriteOptions()); }

  /// Initiate/post a write operation with specified options.
  ///
  /// \param[in] req The message to be written. The library does not take
  ///                ownership but the caller must ensure that the message is
  ///                not deleted or modified until OnWriteDone is called.
  /// \param[in] options The WriteOptions to use for writing this message
  void StartWrite(const Request* req, grpc::WriteOptions options) {
    stream_->Write(req, options);
  }

  /// Initiate/post a write operation with specified options and an indication
  /// that this is the last write (like StartWrite and StartWritesDone, merged).
  /// Note that calling this means that no more calls to StartWrite,
  /// StartWriteLast, or StartWritesDone are allowed.
  ///
  /// \param[in] req The message to be written. The library does not take
  ///                ownership but the caller must ensure that the message is
  ///                not deleted or modified until OnWriteDone is called.
  /// \param[in] options The WriteOptions to use for writing this message
  void StartWriteLast(const Request* req, grpc::WriteOptions options) {
    StartWrite(req, options.set_last_message());
  }

  /// Indicate that the RPC will have no more write operations. This can only be
  /// issued once for a given RPC. This is not required or allowed if
  /// StartWriteLast is used since that already has the same implication.
  /// Note that calling this means that no more calls to StartWrite,
  /// StartWriteLast, or StartWritesDone are allowed.
  void StartWritesDone() { stream_->WritesDone(); }

  /// Holds are needed if (and only if) this stream has operations that take
  /// place on it after StartCall but from outside one of the reactions
  /// (OnReadDone, etc). This is _not_ a common use of the streaming API.
  ///
  /// Holds must be added before calling StartCall. If a stream still has a hold
  /// in place, its resources will not be destroyed even if the status has
  /// already come in from the wire and there are currently no active callbacks
  /// outstanding. Similarly, the stream will not call OnDone if there are still
  /// holds on it.
  ///
  /// For example, if a StartRead or StartWrite operation is going to be
  /// initiated from elsewhere in the application, the application should call
  /// AddHold or AddMultipleHolds before StartCall.  If there is going to be,
  /// for example, a read-flow and a write-flow taking place outside the
  /// reactions, then call AddMultipleHolds(2) before StartCall. When the
  /// application knows that it won't issue any more read operations (such as
  /// when a read comes back as not ok), it should issue a RemoveHold(). It
  /// should also call RemoveHold() again after it does StartWriteLast or
  /// StartWritesDone that indicates that there will be no more write ops.
  /// The number of RemoveHold calls must match the total number of AddHold
  /// calls plus the number of holds added by AddMultipleHolds.
  /// The argument to AddMultipleHolds must be positive.
  void AddHold() { AddMultipleHolds(1); }
  void AddMultipleHolds(int holds) {
    ABSL_DCHECK_GT(holds, 0);
    stream_->AddHold(holds);
  }
  void RemoveHold() { stream_->RemoveHold(); }

  /// Notifies the application that all operations associated with this RPC
  /// have completed and all Holds have been removed. OnDone provides the RPC
  /// status outcome for both successful and failed RPCs and will be called in
  /// all cases. If it is not called, it indicates an application-level problem
  /// (like failure to remove a hold).
  ///
  /// OnDone is called exactly once, and not concurrently with any (other)
  /// reaction. (Holds may be needed (see above) to prevent OnDone from being
  /// called concurrently with calls to the reactor from outside of reactions.)
  ///
  /// \param[in] s The status outcome of this RPC
  void OnDone(const grpc::Status& /*s*/) override {}

  /// Notifies the application that a read of initial metadata from the
  /// server is done. If the application chooses not to implement this method,
  /// it can assume that the initial metadata has been read before the first
  /// call of OnReadDone or OnDone.
  ///
  /// \param[in] ok Was the initial metadata read successfully? If false, no
  ///               new read/write operation will succeed.
  virtual void OnReadInitialMetadataDone(bool /*ok*/) {}

  /// Notifies the application that a StartRead operation completed.
  ///
  /// \param[in] ok Was it successful? If false, no new read/write operation
  ///               will succeed.
  virtual void OnReadDone(bool /*ok*/) {}

  /// Notifies the application that a StartWrite or StartWriteLast operation
  /// completed.
  ///
  /// \param[in] ok Was it successful? If false, no new read/write operation
  ///               will succeed.
  virtual void OnWriteDone(bool /*ok*/) {}

  /// Notifies the application that a StartWritesDone operation completed. Note
  /// that this is only used on explicit StartWritesDone operations and not for
  /// those that are implicitly invoked as part of a StartWriteLast.
  ///
  /// \param[in] ok Was it successful? If false, the application will later see
  ///               the failure reflected as a bad status in OnDone and no
  ///               further Start* should be called.
  virtual void OnWritesDoneDone(bool /*ok*/) {}

 private:
  friend class ClientCallbackReaderWriter<Request, Response>;
  void BindStream(ClientCallbackReaderWriter<Request, Response>* stream) {
    stream_ = stream;
  }
  ClientCallbackReaderWriter<Request, Response>* stream_;
};

/// \a ClientReadReactor is the interface for a server-streaming RPC.
/// All public methods behave as in ClientBidiReactor.
template <class Response>
class ClientReadReactor : public internal::ClientReactor {
 public:
  void StartCall() { reader_->StartCall(); }
  void StartRead(Response* resp) { reader_->Read(resp); }

  void AddHold() { AddMultipleHolds(1); }
  void AddMultipleHolds(int holds) {
    ABSL_DCHECK_GT(holds, 0);
    reader_->AddHold(holds);
  }
  void RemoveHold() { reader_->RemoveHold(); }

  void OnDone(const grpc::Status& /*s*/) override {}
  virtual void OnReadInitialMetadataDone(bool /*ok*/) {}
  virtual void OnReadDone(bool /*ok*/) {}

 private:
  friend class ClientCallbackReader<Response>;
  void BindReader(ClientCallbackReader<Response>* reader) { reader_ = reader; }
  ClientCallbackReader<Response>* reader_;
};

/// \a ClientWriteReactor is the interface for a client-streaming RPC.
/// All public methods behave as in ClientBidiReactor.
template <class Request>
class ClientWriteReactor : public internal::ClientReactor {
 public:
  void StartCall() { writer_->StartCall(); }
  void StartWrite(const Request* req) { StartWrite(req, grpc::WriteOptions()); }
  void StartWrite(const Request* req, grpc::WriteOptions options) {
    writer_->Write(req, options);
  }
  void StartWriteLast(const Request* req, grpc::WriteOptions options) {
    StartWrite(req, options.set_last_message());
  }
  void StartWritesDone() { writer_->WritesDone(); }

  void AddHold() { AddMultipleHolds(1); }
  void AddMultipleHolds(int holds) {
    ABSL_DCHECK_GT(holds, 0);
    writer_->AddHold(holds);
  }
  void RemoveHold() { writer_->RemoveHold(); }

  void OnDone(const grpc::Status& /*s*/) override {}
  virtual void OnReadInitialMetadataDone(bool /*ok*/) {}
  virtual void OnWriteDone(bool /*ok*/) {}
  virtual void OnWritesDoneDone(bool /*ok*/) {}

 private:
  friend class ClientCallbackWriter<Request>;
  void BindWriter(ClientCallbackWriter<Request>* writer) { writer_ = writer; }

  ClientCallbackWriter<Request>* writer_;
};

/// \a ClientUnaryReactor is a reactor-style interface for a unary RPC.
/// This is _not_ a common way of invoking a unary RPC. In practice, this
/// option should be used only if the unary RPC wants to receive initial
/// metadata without waiting for the response to complete. Most deployments of
/// RPC systems do not use this option, but it is needed for generality.
/// All public methods behave as in ClientBidiReactor.
/// StartCall is included for consistency with the other reactor flavors: even
/// though there are no StartRead or StartWrite operations to queue before the
/// call (that is part of the unary call itself) and there is no reactor object
/// being created as a result of this call, we keep a consistent 2-phase
/// initiation API among all the reactor flavors.
class ClientUnaryReactor : public internal::ClientReactor {
 public:
  void StartCall() { call_->StartCall(); }
  void OnDone(const grpc::Status& /*s*/) override {}
  virtual void OnReadInitialMetadataDone(bool /*ok*/) {}

 private:
  friend class ClientCallbackUnary;
  void BindCall(ClientCallbackUnary* call) { call_ = call; }
  ClientCallbackUnary* call_;
};

// Define function out-of-line from class to avoid forward declaration issue
inline void ClientCallbackUnary::BindReactor(ClientUnaryReactor* reactor) {
  reactor->BindCall(this);
}

namespace internal {

// Forward declare factory classes for friendship
template <class Request, class Response>
class ClientCallbackReaderWriterFactory;
template <class Response>
class ClientCallbackReaderFactory;
template <class Request>
class ClientCallbackWriterFactory;

template <class Request, class Response>
class ClientCallbackReaderWriterImpl
    : public ClientCallbackReaderWriter<Request, Response> {
 public:
  // always allocated against a call arena, no memory free required
  static void operator delete(void* /*ptr*/, std::size_t size) {
    ABSL_CHECK_EQ(size, sizeof(ClientCallbackReaderWriterImpl));
  }

  // This operator should never be called as the memory should be freed as part
  // of the arena destruction. It only exists to provide a matching operator
  // delete to the operator new so that some compilers will not complain (see
  // https://github.com/grpc/grpc/issues/11301) Note at the time of adding this
  // there are no tests catching the compiler warning.
  static void operator delete(void*, void*) { ABSL_CHECK(false); }

  void StartCall() ABSL_LOCKS_EXCLUDED(start_mu_) override {
    // This call initiates two batches, plus any backlog, each with a callback
    // 1. Send initial metadata (unless corked) + recv initial metadata
    // 2. Any read backlog
    // 3. Any write backlog
    // 4. Recv trailing metadata (unless corked)
    if (!start_corked_) {
      start_ops_.SendInitialMetadata(&context_->send_initial_metadata_,
                                     context_->initial_metadata_flags());
    }

    call_.PerformOps(&start_ops_);

    {
      grpc::internal::MutexLock lock(&start_mu_);

      if (backlog_.read_ops) {
        call_.PerformOps(&read_ops_);
      }
      if (backlog_.write_ops) {
        call_.PerformOps(&write_ops_);
      }
      if (backlog_.writes_done_ops) {
        call_.PerformOps(&writes_done_ops_);
      }
      call_.PerformOps(&finish_ops_);
      // The last thing in this critical section is to set started_ so that it
      // can be used lock-free as well.
      started_.store(true, std::memory_order_release);
    }
    // MaybeFinish outside the lock to make sure that destruction of this object
    // doesn't take place while holding the lock (which would cause the lock to
    // be released after destruction)
    this->MaybeFinish(/*from_reaction=*/false);
  }

  void Read(Response* msg) override {
    read_ops_.RecvMessage(msg);
    callbacks_outstanding_.fetch_add(1, std::memory_order_relaxed);
    if (GPR_UNLIKELY(!started_.load(std::memory_order_acquire))) {
      grpc::internal::MutexLock lock(&start_mu_);
      if (GPR_LIKELY(!started_.load(std::memory_order_relaxed))) {
        backlog_.read_ops = true;
        return;
      }
    }
    call_.PerformOps(&read_ops_);
  }

  void Write(const Request* msg, grpc::WriteOptions options)
      ABSL_LOCKS_EXCLUDED(start_mu_) override {
    if (options.is_last_message()) {
      options.set_buffer_hint();
      write_ops_.ClientSendClose();
    }
    // TODO(vjpai): don't assert
    ABSL_CHECK(write_ops_.SendMessagePtr(msg, options).ok());
    callbacks_outstanding_.fetch_add(1, std::memory_order_relaxed);
    if (GPR_UNLIKELY(corked_write_needed_)) {
      write_ops_.SendInitialMetadata(&context_->send_initial_metadata_,
                                     context_->initial_metadata_flags());
      corked_write_needed_ = false;
    }

    if (GPR_UNLIKELY(!started_.load(std::memory_order_acquire))) {
      grpc::internal::MutexLock lock(&start_mu_);
      if (GPR_LIKELY(!started_.load(std::memory_order_relaxed))) {
        backlog_.write_ops = true;
        return;
      }
    }
    call_.PerformOps(&write_ops_);
  }
  void WritesDone() ABSL_LOCKS_EXCLUDED(start_mu_) override {
    writes_done_ops_.ClientSendClose();
    writes_done_tag_.Set(
        call_.call(),
        [this](bool ok) {
          reactor_->OnWritesDoneDone(ok);
          MaybeFinish(/*from_reaction=*/true);
        },
        &writes_done_ops_, /*can_inline=*/false);
    writes_done_ops_.set_core_cq_tag(&writes_done_tag_);
    callbacks_outstanding_.fetch_add(1, std::memory_order_relaxed);
    if (GPR_UNLIKELY(corked_write_needed_)) {
      writes_done_ops_.SendInitialMetadata(&context_->send_initial_metadata_,
                                           context_->initial_metadata_flags());
      corked_write_needed_ = false;
    }
    if (GPR_UNLIKELY(!started_.load(std::memory_order_acquire))) {
      grpc::internal::MutexLock lock(&start_mu_);
      if (GPR_LIKELY(!started_.load(std::memory_order_relaxed))) {
        backlog_.writes_done_ops = true;
        return;
      }
    }
    call_.PerformOps(&writes_done_ops_);
  }

  void AddHold(int holds) override {
    callbacks_outstanding_.fetch_add(holds, std::memory_order_relaxed);
  }
  void RemoveHold() override { MaybeFinish(/*from_reaction=*/false); }

 private:
  friend class ClientCallbackReaderWriterFactory<Request, Response>;

  ClientCallbackReaderWriterImpl(grpc::internal::Call call,
                                 grpc::ClientContext* context,
                                 ClientBidiReactor<Request, Response>* reactor)
      : context_(context),
        call_(call),
        reactor_(reactor),
        start_corked_(context_->initial_metadata_corked_),
        corked_write_needed_(start_corked_) {
    this->BindReactor(reactor);

    // Set up the unchanging parts of the start, read, and write tags and ops.
    start_tag_.Set(
        call_.call(),
        [this](bool ok) {
          reactor_->OnReadInitialMetadataDone(
              ok && !reactor_->InternalTrailersOnly(call_.call()));
          MaybeFinish(/*from_reaction=*/true);
        },
        &start_ops_, /*can_inline=*/false);
    start_ops_.RecvInitialMetadata(context_);
    start_ops_.set_core_cq_tag(&start_tag_);

    write_tag_.Set(
        call_.call(),
        [this](bool ok) {
          reactor_->OnWriteDone(ok);
          MaybeFinish(/*from_reaction=*/true);
        },
        &write_ops_, /*can_inline=*/false);
    write_ops_.set_core_cq_tag(&write_tag_);

    read_tag_.Set(
        call_.call(),
        [this](bool ok) {
          reactor_->OnReadDone(ok);
          MaybeFinish(/*from_reaction=*/true);
        },
        &read_ops_, /*can_inline=*/false);
    read_ops_.set_core_cq_tag(&read_tag_);

    // Also set up the Finish tag and op set.
    finish_tag_.Set(
        call_.call(),
        [this](bool /*ok*/) { MaybeFinish(/*from_reaction=*/true); },
        &finish_ops_,
        /*can_inline=*/false);
    finish_ops_.ClientRecvStatus(context_, &finish_status_);
    finish_ops_.set_core_cq_tag(&finish_tag_);
  }

  // MaybeFinish can be called from reactions or from user-initiated operations
  // like StartCall or RemoveHold. If this is the last operation or hold on this
  // object, it will invoke the OnDone reaction. If MaybeFinish was called from
  // a reaction, it can call OnDone directly. If not, it would need to schedule
  // OnDone onto an executor thread to avoid the possibility of deadlocking with
  // any locks in the user code that invoked it.
  void MaybeFinish(bool from_reaction) {
    if (GPR_UNLIKELY(callbacks_outstanding_.fetch_sub(
                         1, std::memory_order_acq_rel) == 1)) {
      grpc::Status s = std::move(finish_status_);
      auto* reactor = reactor_;
      auto* call = call_.call();
      this->~ClientCallbackReaderWriterImpl();
      if (GPR_LIKELY(from_reaction)) {
        grpc_call_unref(call);
        reactor->OnDone(s);
      } else {
        grpc_call_run_in_event_engine(
            call, [reactor, s = std::move(s)]() { reactor->OnDone(s); });
        grpc_call_unref(call);
      }
    }
  }

  grpc::ClientContext* const context_;
  grpc::internal::Call call_;
  ClientBidiReactor<Request, Response>* const reactor_;

  grpc::internal::CallOpSet<grpc::internal::CallOpSendInitialMetadata,
                            grpc::internal::CallOpRecvInitialMetadata>
      start_ops_;
  grpc::internal::CallbackWithSuccessTag start_tag_;
  const bool start_corked_;
  bool corked_write_needed_;  // no lock needed since only accessed in
                              // Write/WritesDone which cannot be concurrent

  grpc::internal::CallOpSet<grpc::internal::CallOpClientRecvStatus> finish_ops_;
  grpc::internal::CallbackWithSuccessTag finish_tag_;
  grpc::Status finish_status_;

  grpc::internal::CallOpSet<grpc::internal::CallOpSendInitialMetadata,
                            grpc::internal::CallOpSendMessage,
                            grpc::internal::CallOpClientSendClose>
      write_ops_;
  grpc::internal::CallbackWithSuccessTag write_tag_;

  grpc::internal::CallOpSet<grpc::internal::CallOpSendInitialMetadata,
                            grpc::internal::CallOpClientSendClose>
      writes_done_ops_;
  grpc::internal::CallbackWithSuccessTag writes_done_tag_;

  grpc::internal::CallOpSet<grpc::internal::CallOpRecvMessage<Response>>
      read_ops_;
  grpc::internal::CallbackWithSuccessTag read_tag_;

  struct StartCallBacklog {
    bool write_ops = false;
    bool writes_done_ops = false;
    bool read_ops = false;
  };
  StartCallBacklog backlog_ ABSL_GUARDED_BY(start_mu_);

  // Minimum of 3 callbacks to pre-register for start ops, StartCall, and finish
  std::atomic<intptr_t> callbacks_outstanding_{3};
  std::atomic_bool started_{false};
  grpc::internal::Mutex start_mu_;
};

template <class Request, class Response>
class ClientCallbackReaderWriterFactory {
 public:
  static void Create(grpc::ChannelInterface* channel,
                     const grpc::internal::RpcMethod& method,
                     grpc::ClientContext* context,
                     ClientBidiReactor<Request, Response>* reactor) {
    grpc::internal::Call call =
        channel->CreateCall(method, context, channel->CallbackCQ());

    grpc_call_ref(call.call());
    new (grpc_call_arena_alloc(
        call.call(), sizeof(ClientCallbackReaderWriterImpl<Request, Response>)))
        ClientCallbackReaderWriterImpl<Request, Response>(call, context,
                                                          reactor);
  }
};

template <class Response>
class ClientCallbackReaderImpl : public ClientCallbackReader<Response> {
 public:
  // always allocated against a call arena, no memory free required
  static void operator delete(void* /*ptr*/, std::size_t size) {
    ABSL_CHECK_EQ(size, sizeof(ClientCallbackReaderImpl));
  }

  // This operator should never be called as the memory should be freed as part
  // of the arena destruction. It only exists to provide a matching operator
  // delete to the operator new so that some compilers will not complain (see
  // https://github.com/grpc/grpc/issues/11301) Note at the time of adding this
  // there are no tests catching the compiler warning.
  static void operator delete(void*, void*) { ABSL_CHECK(false); }

  void StartCall() override {
    // This call initiates two batches, plus any backlog, each with a callback
    // 1. Send initial metadata (unless corked) + recv initial metadata
    // 2. Any backlog
    // 3. Recv trailing metadata

    start_tag_.Set(
        call_.call(),
        [this](bool ok) {
          reactor_->OnReadInitialMetadataDone(
              ok && !reactor_->InternalTrailersOnly(call_.call()));
          MaybeFinish(/*from_reaction=*/true);
        },
        &start_ops_, /*can_inline=*/false);
    start_ops_.SendInitialMetadata(&context_->send_initial_metadata_,
                                   context_->initial_metadata_flags());
    start_ops_.RecvInitialMetadata(context_);
    start_ops_.set_core_cq_tag(&start_tag_);
    call_.PerformOps(&start_ops_);

    // Also set up the read tag so it doesn't have to be set up each time
    read_tag_.Set(
        call_.call(),
        [this](bool ok) {
          reactor_->OnReadDone(ok);
          MaybeFinish(/*from_reaction=*/true);
        },
        &read_ops_, /*can_inline=*/false);
    read_ops_.set_core_cq_tag(&read_tag_);

    {
      grpc::internal::MutexLock lock(&start_mu_);
      if (backlog_.read_ops) {
        call_.PerformOps(&read_ops_);
      }
      started_.store(true, std::memory_order_release);
    }

    finish_tag_.Set(
        call_.call(),
        [this](bool /*ok*/) { MaybeFinish(/*from_reaction=*/true); },
        &finish_ops_, /*can_inline=*/false);
    finish_ops_.ClientRecvStatus(context_, &finish_status_);
    finish_ops_.set_core_cq_tag(&finish_tag_);
    call_.PerformOps(&finish_ops_);
  }

  void Read(Response* msg) override {
    read_ops_.RecvMessage(msg);
    callbacks_outstanding_.fetch_add(1, std::memory_order_relaxed);
    if (GPR_UNLIKELY(!started_.load(std::memory_order_acquire))) {
      grpc::internal::MutexLock lock(&start_mu_);
      if (GPR_LIKELY(!started_.load(std::memory_order_relaxed))) {
        backlog_.read_ops = true;
        return;
      }
    }
    call_.PerformOps(&read_ops_);
  }

  void AddHold(int holds) override {
    callbacks_outstanding_.fetch_add(holds, std::memory_order_relaxed);
  }
  void RemoveHold() override { MaybeFinish(/*from_reaction=*/false); }

 private:
  friend class ClientCallbackReaderFactory<Response>;

  template <class Request>
  ClientCallbackReaderImpl(grpc::internal::Call call,
                           grpc::ClientContext* context, Request* request,
                           ClientReadReactor<Response>* reactor)
      : context_(context), call_(call), reactor_(reactor) {
    this->BindReactor(reactor);
    // TODO(vjpai): don't assert
    ABSL_CHECK(start_ops_.SendMessagePtr(request).ok());
    start_ops_.ClientSendClose();
  }

  // MaybeFinish behaves as in ClientCallbackReaderWriterImpl.
  void MaybeFinish(bool from_reaction) {
    if (GPR_UNLIKELY(callbacks_outstanding_.fetch_sub(
                         1, std::memory_order_acq_rel) == 1)) {
      grpc::Status s = std::move(finish_status_);
      auto* reactor = reactor_;
      auto* call = call_.call();
      this->~ClientCallbackReaderImpl();
      if (GPR_LIKELY(from_reaction)) {
        grpc_call_unref(call);
        reactor->OnDone(s);
      } else {
        grpc_call_run_in_event_engine(
            call, [reactor, s = std::move(s)]() { reactor->OnDone(s); });
        grpc_call_unref(call);
      }
    }
  }

  grpc::ClientContext* const context_;
  grpc::internal::Call call_;
  ClientReadReactor<Response>* const reactor_;

  grpc::internal::CallOpSet<grpc::internal::CallOpSendInitialMetadata,
                            grpc::internal::CallOpSendMessage,
                            grpc::internal::CallOpClientSendClose,
                            grpc::internal::CallOpRecvInitialMetadata>
      start_ops_;
  grpc::internal::CallbackWithSuccessTag start_tag_;

  grpc::internal::CallOpSet<grpc::internal::CallOpClientRecvStatus> finish_ops_;
  grpc::internal::CallbackWithSuccessTag finish_tag_;
  grpc::Status finish_status_;

  grpc::internal::CallOpSet<grpc::internal::CallOpRecvMessage<Response>>
      read_ops_;
  grpc::internal::CallbackWithSuccessTag read_tag_;

  struct StartCallBacklog {
    bool read_ops = false;
  };
  StartCallBacklog backlog_ ABSL_GUARDED_BY(start_mu_);

  // Minimum of 2 callbacks to pre-register for start and finish
  std::atomic<intptr_t> callbacks_outstanding_{2};
  std::atomic_bool started_{false};
  grpc::internal::Mutex start_mu_;
};

template <class Response>
class ClientCallbackReaderFactory {
 public:
  template <class Request>
  static void Create(grpc::ChannelInterface* channel,
                     const grpc::internal::RpcMethod& method,
                     grpc::ClientContext* context, const Request* request,
                     ClientReadReactor<Response>* reactor) {
    grpc::internal::Call call =
        channel->CreateCall(method, context, channel->CallbackCQ());

    grpc_call_ref(call.call());
    new (grpc_call_arena_alloc(call.call(),
                               sizeof(ClientCallbackReaderImpl<Response>)))
        ClientCallbackReaderImpl<Response>(call, context, request, reactor);
  }
};

template <class Request>
class ClientCallbackWriterImpl : public ClientCallbackWriter<Request> {
 public:
  // always allocated against a call arena, no memory free required
  static void operator delete(void* /*ptr*/, std::size_t size) {
    ABSL_CHECK_EQ(size, sizeof(ClientCallbackWriterImpl));
  }

  // This operator should never be called as the memory should be freed as part
  // of the arena destruction. It only exists to provide a matching operator
  // delete to the operator new so that some compilers will not complain (see
  // https://github.com/grpc/grpc/issues/11301) Note at the time of adding this
  // there are no tests catching the compiler warning.
  static void operator delete(void*, void*) { ABSL_CHECK(false); }

  void StartCall() ABSL_LOCKS_EXCLUDED(start_mu_) override {
    // This call initiates two batches, plus any backlog, each with a callback
    // 1. Send initial metadata (unless corked) + recv initial metadata
    // 2. Any backlog
    // 3. Recv trailing metadata

    if (!start_corked_) {
      start_ops_.SendInitialMetadata(&context_->send_initial_metadata_,
                                     context_->initial_metadata_flags());
    }
    call_.PerformOps(&start_ops_);

    {
      grpc::internal::MutexLock lock(&start_mu_);

      if (backlog_.write_ops) {
        call_.PerformOps(&write_ops_);
      }
      if (backlog_.writes_done_ops) {
        call_.PerformOps(&writes_done_ops_);
      }
      call_.PerformOps(&finish_ops_);
      // The last thing in this critical section is to set started_ so that it
      // can be used lock-free as well.
      started_.store(true, std::memory_order_release);
    }
    // MaybeFinish outside the lock to make sure that destruction of this object
    // doesn't take place while holding the lock (which would cause the lock to
    // be released after destruction)
    this->MaybeFinish(/*from_reaction=*/false);
  }

  void Write(const Request* msg, grpc::WriteOptions options)
      ABSL_LOCKS_EXCLUDED(start_mu_) override {
    if (GPR_UNLIKELY(options.is_last_message())) {
      options.set_buffer_hint();
      write_ops_.ClientSendClose();
    }
    // TODO(vjpai): don't assert
    ABSL_CHECK(write_ops_.SendMessagePtr(msg, options).ok());
    callbacks_outstanding_.fetch_add(1, std::memory_order_relaxed);

    if (GPR_UNLIKELY(corked_write_needed_)) {
      write_ops_.SendInitialMetadata(&context_->send_initial_metadata_,
                                     context_->initial_metadata_flags());
      corked_write_needed_ = false;
    }

    if (GPR_UNLIKELY(!started_.load(std::memory_order_acquire))) {
      grpc::internal::MutexLock lock(&start_mu_);
      if (GPR_LIKELY(!started_.load(std::memory_order_relaxed))) {
        backlog_.write_ops = true;
        return;
      }
    }
    call_.PerformOps(&write_ops_);
  }

  void WritesDone() ABSL_LOCKS_EXCLUDED(start_mu_) override {
    writes_done_ops_.ClientSendClose();
    writes_done_tag_.Set(
        call_.call(),
        [this](bool ok) {
          reactor_->OnWritesDoneDone(ok);
          MaybeFinish(/*from_reaction=*/true);
        },
        &writes_done_ops_, /*can_inline=*/false);
    writes_done_ops_.set_core_cq_tag(&writes_done_tag_);
    callbacks_outstanding_.fetch_add(1, std::memory_order_relaxed);

    if (GPR_UNLIKELY(corked_write_needed_)) {
      writes_done_ops_.SendInitialMetadata(&context_->send_initial_metadata_,
                                           context_->initial_metadata_flags());
      corked_write_needed_ = false;
    }

    if (GPR_UNLIKELY(!started_.load(std::memory_order_acquire))) {
      grpc::internal::MutexLock lock(&start_mu_);
      if (GPR_LIKELY(!started_.load(std::memory_order_relaxed))) {
        backlog_.writes_done_ops = true;
        return;
      }
    }
    call_.PerformOps(&writes_done_ops_);
  }

  void AddHold(int holds) override {
    callbacks_outstanding_.fetch_add(holds, std::memory_order_relaxed);
  }
  void RemoveHold() override { MaybeFinish(/*from_reaction=*/false); }

 private:
  friend class ClientCallbackWriterFactory<Request>;

  template <class Response>
  ClientCallbackWriterImpl(grpc::internal::Call call,
                           grpc::ClientContext* context, Response* response,
                           ClientWriteReactor<Request>* reactor)
      : context_(context),
        call_(call),
        reactor_(reactor),
        start_corked_(context_->initial_metadata_corked_),
        corked_write_needed_(start_corked_) {
    this->BindReactor(reactor);

    // Set up the unchanging parts of the start and write tags and ops.
    start_tag_.Set(
        call_.call(),
        [this](bool ok) {
          reactor_->OnReadInitialMetadataDone(
              ok && !reactor_->InternalTrailersOnly(call_.call()));
          MaybeFinish(/*from_reaction=*/true);
        },
        &start_ops_, /*can_inline=*/false);
    start_ops_.RecvInitialMetadata(context_);
    start_ops_.set_core_cq_tag(&start_tag_);

    write_tag_.Set(
        call_.call(),
        [this](bool ok) {
          reactor_->OnWriteDone(ok);
          MaybeFinish(/*from_reaction=*/true);
        },
        &write_ops_, /*can_inline=*/false);
    write_ops_.set_core_cq_tag(&write_tag_);

    // Also set up the Finish tag and op set.
    finish_ops_.RecvMessage(response);
    finish_ops_.AllowNoMessage();
    finish_tag_.Set(
        call_.call(),
        [this](bool /*ok*/) { MaybeFinish(/*from_reaction=*/true); },
        &finish_ops_,
        /*can_inline=*/false);
    finish_ops_.ClientRecvStatus(context_, &finish_status_);
    finish_ops_.set_core_cq_tag(&finish_tag_);
  }

  // MaybeFinish behaves as in ClientCallbackReaderWriterImpl.
  void MaybeFinish(bool from_reaction) {
    if (GPR_UNLIKELY(callbacks_outstanding_.fetch_sub(
                         1, std::memory_order_acq_rel) == 1)) {
      grpc::Status s = std::move(finish_status_);
      auto* reactor = reactor_;
      auto* call = call_.call();
      this->~ClientCallbackWriterImpl();
      if (GPR_LIKELY(from_reaction)) {
        grpc_call_unref(call);
        reactor->OnDone(s);
      } else {
        grpc_call_run_in_event_engine(
            call, [reactor, s = std::move(s)]() { reactor->OnDone(s); });
        grpc_call_unref(call);
      }
    }
  }

  grpc::ClientContext* const context_;
  grpc::internal::Call call_;
  ClientWriteReactor<Request>* const reactor_;

  grpc::internal::CallOpSet<grpc::internal::CallOpSendInitialMetadata,
                            grpc::internal::CallOpRecvInitialMetadata>
      start_ops_;
  grpc::internal::CallbackWithSuccessTag start_tag_;
  const bool start_corked_;
  bool corked_write_needed_;  // no lock needed since only accessed in
                              // Write/WritesDone which cannot be concurrent

  grpc::internal::CallOpSet<grpc::internal::CallOpGenericRecvMessage,
                            grpc::internal::CallOpClientRecvStatus>
      finish_ops_;
  grpc::internal::CallbackWithSuccessTag finish_tag_;
  grpc::Status finish_status_;

  grpc::internal::CallOpSet<grpc::internal::CallOpSendInitialMetadata,
                            grpc::internal::CallOpSendMessage,
                            grpc::internal::CallOpClientSendClose>
      write_ops_;
  grpc::internal::CallbackWithSuccessTag write_tag_;

  grpc::internal::CallOpSet<grpc::internal::CallOpSendInitialMetadata,
                            grpc::internal::CallOpClientSendClose>
      writes_done_ops_;
  grpc::internal::CallbackWithSuccessTag writes_done_tag_;

  struct StartCallBacklog {
    bool write_ops = false;
    bool writes_done_ops = false;
  };
  StartCallBacklog backlog_ ABSL_GUARDED_BY(start_mu_);

  // Minimum of 3 callbacks to pre-register for start ops, StartCall, and finish
  std::atomic<intptr_t> callbacks_outstanding_{3};
  std::atomic_bool started_{false};
  grpc::internal::Mutex start_mu_;
};

template <class Request>
class ClientCallbackWriterFactory {
 public:
  template <class Response>
  static void Create(grpc::ChannelInterface* channel,
                     const grpc::internal::RpcMethod& method,
                     grpc::ClientContext* context, Response* response,
                     ClientWriteReactor<Request>* reactor) {
    grpc::internal::Call call =
        channel->CreateCall(method, context, channel->CallbackCQ());

    grpc_call_ref(call.call());
    new (grpc_call_arena_alloc(call.call(),
                               sizeof(ClientCallbackWriterImpl<Request>)))
        ClientCallbackWriterImpl<Request>(call, context, response, reactor);
  }
};

class ClientCallbackUnaryImpl final : public ClientCallbackUnary {
 public:
  // always allocated against a call arena, no memory free required
  static void operator delete(void* /*ptr*/, std::size_t size) {
    ABSL_CHECK_EQ(size, sizeof(ClientCallbackUnaryImpl));
  }

  // This operator should never be called as the memory should be freed as part
  // of the arena destruction. It only exists to provide a matching operator
  // delete to the operator new so that some compilers will not complain (see
  // https://github.com/grpc/grpc/issues/11301) Note at the time of adding this
  // there are no tests catching the compiler warning.
  static void operator delete(void*, void*) { ABSL_CHECK(false); }

  void StartCall() override {
    // This call initiates two batches, each with a callback
    // 1. Send initial metadata + write + writes done + recv initial metadata
    // 2. Read message, recv trailing metadata

    start_tag_.Set(
        call_.call(),
        [this](bool ok) {
          reactor_->OnReadInitialMetadataDone(
              ok && !reactor_->InternalTrailersOnly(call_.call()));
          MaybeFinish();
        },
        &start_ops_, /*can_inline=*/false);
    start_ops_.SendInitialMetadata(&context_->send_initial_metadata_,
                                   context_->initial_metadata_flags());
    start_ops_.RecvInitialMetadata(context_);
    start_ops_.set_core_cq_tag(&start_tag_);
    call_.PerformOps(&start_ops_);

    finish_tag_.Set(
        call_.call(), [this](bool /*ok*/) { MaybeFinish(); }, &finish_ops_,
        /*can_inline=*/false);
    finish_ops_.ClientRecvStatus(context_, &finish_status_);
    finish_ops_.set_core_cq_tag(&finish_tag_);
    call_.PerformOps(&finish_ops_);
  }

 private:
  friend class ClientCallbackUnaryFactory;

  template <class Request, class Response>
  ClientCallbackUnaryImpl(grpc::internal::Call call,
                          grpc::ClientContext* context, Request* request,
                          Response* response, ClientUnaryReactor* reactor)
      : context_(context), call_(call), reactor_(reactor) {
    this->BindReactor(reactor);
    // TODO(vjpai): don't assert
    ABSL_CHECK(start_ops_.SendMessagePtr(request).ok());
    start_ops_.ClientSendClose();
    finish_ops_.RecvMessage(response);
    finish_ops_.AllowNoMessage();
  }

  // In the unary case, MaybeFinish is only ever invoked from a
  // library-initiated reaction, so it will just directly call OnDone if this is
  // the last reaction for this RPC.
  void MaybeFinish() {
    if (GPR_UNLIKELY(callbacks_outstanding_.fetch_sub(
                         1, std::memory_order_acq_rel) == 1)) {
      grpc::Status s = std::move(finish_status_);
      auto* reactor = reactor_;
      auto* call = call_.call();
      this->~ClientCallbackUnaryImpl();
      grpc_call_unref(call);
      reactor->OnDone(s);
    }
  }

  grpc::ClientContext* const context_;
  grpc::internal::Call call_;
  ClientUnaryReactor* const reactor_;

  grpc::internal::CallOpSet<grpc::internal::CallOpSendInitialMetadata,
                            grpc::internal::CallOpSendMessage,
                            grpc::internal::CallOpClientSendClose,
                            grpc::internal::CallOpRecvInitialMetadata>
      start_ops_;
  grpc::internal::CallbackWithSuccessTag start_tag_;

  grpc::internal::CallOpSet<grpc::internal::CallOpGenericRecvMessage,
                            grpc::internal::CallOpClientRecvStatus>
      finish_ops_;
  grpc::internal::CallbackWithSuccessTag finish_tag_;
  grpc::Status finish_status_;

  // This call will have 2 callbacks: start and finish
  std::atomic<intptr_t> callbacks_outstanding_{2};
};

class ClientCallbackUnaryFactory {
 public:
  template <class Request, class Response, class BaseRequest = Request,
            class BaseResponse = Response>
  static void Create(grpc::ChannelInterface* channel,
                     const grpc::internal::RpcMethod& method,
                     grpc::ClientContext* context, const Request* request,
                     Response* response, ClientUnaryReactor* reactor) {
    grpc::internal::Call call =
        channel->CreateCall(method, context, channel->CallbackCQ());

    grpc_call_ref(call.call());

    new (grpc_call_arena_alloc(call.call(), sizeof(ClientCallbackUnaryImpl)))
        ClientCallbackUnaryImpl(call, context,
                                static_cast<const BaseRequest*>(request),
                                static_cast<BaseResponse*>(response), reactor);
  }
};

}  // namespace internal
}  // namespace grpc

#endif  // GRPCPP_SUPPORT_CLIENT_CALLBACK_H
