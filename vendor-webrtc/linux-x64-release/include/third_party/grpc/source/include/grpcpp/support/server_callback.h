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

#ifndef GRPCPP_SUPPORT_SERVER_CALLBACK_H
#define GRPCPP_SUPPORT_SERVER_CALLBACK_H

#include <grpc/impl/call.h>
#include <grpcpp/impl/call.h>
#include <grpcpp/impl/call_op_set.h>
#include <grpcpp/impl/sync.h>
#include <grpcpp/support/callback_common.h>
#include <grpcpp/support/config.h>
#include <grpcpp/support/message_allocator.h>
#include <grpcpp/support/status.h>

#include <atomic>
#include <functional>
#include <type_traits>

#include "absl/functional/any_invocable.h"

namespace grpc {

// Declare base class of all reactors as internal
namespace internal {

// Forward declarations
template <class Request, class Response>
class CallbackUnaryHandler;
template <class Request, class Response>
class CallbackClientStreamingHandler;
template <class Request, class Response>
class CallbackServerStreamingHandler;
template <class Request, class Response>
class CallbackBidiHandler;

class ServerReactor {
 public:
  virtual ~ServerReactor() = default;
  virtual void OnDone() = 0;
  virtual void OnCancel() = 0;

  // The following is not API. It is for internal use only and specifies whether
  // all reactions of this Reactor can be run without an extra executor
  // scheduling. This should only be used for internally-defined reactors with
  // trivial reactions.
  virtual bool InternalInlineable() { return false; }

 private:
  template <class Request, class Response>
  friend class CallbackUnaryHandler;
  template <class Request, class Response>
  friend class CallbackClientStreamingHandler;
  template <class Request, class Response>
  friend class CallbackServerStreamingHandler;
  template <class Request, class Response>
  friend class CallbackBidiHandler;
};

/// The base class of ServerCallbackUnary etc.
class ServerCallbackCall {
 public:
  virtual ~ServerCallbackCall() {}

  // This object is responsible for tracking when it is safe to call OnDone and
  // OnCancel. OnDone should not be called until the method handler is complete,
  // Finish has been called, the ServerContext CompletionOp (which tracks
  // cancellation or successful completion) has completed, and all outstanding
  // Read/Write actions have seen their reactions. OnCancel should not be called
  // until after the method handler is done and the RPC has completed with a
  // cancellation. This is tracked by counting how many of these conditions have
  // been met and calling OnCancel when none remain unmet.

  // Public versions of MaybeDone: one where we don't know the reactor in
  // advance (used for the ServerContext CompletionOp), and one for where we
  // know the inlineability of the OnDone reaction. You should set the inline
  // flag to true if either the Reactor is InternalInlineable() or if this
  // callback is already being forced to run dispatched to an executor
  // (typically because it contains additional work than just the MaybeDone).

  void MaybeDone() {
    if (GPR_UNLIKELY(Unref() == 1)) {
      ScheduleOnDone(reactor()->InternalInlineable());
    }
  }

  void MaybeDone(bool inline_ondone) {
    if (GPR_UNLIKELY(Unref() == 1)) {
      ScheduleOnDone(inline_ondone);
    }
  }

  // Fast version called with known reactor passed in, used from derived
  // classes, typically in non-cancel case
  void MaybeCallOnCancel(ServerReactor* reactor) {
    if (GPR_UNLIKELY(UnblockCancellation())) {
      CallOnCancel(reactor);
    }
  }

  // Slower version called from object that doesn't know the reactor a priori
  // (such as the ServerContext CompletionOp which is formed before the
  // reactor). This is used in cancel cases only, so it's ok to be slower and
  // invoke a virtual function.
  void MaybeCallOnCancel() {
    if (GPR_UNLIKELY(UnblockCancellation())) {
      CallOnCancel(reactor());
    }
  }

 protected:
  /// Increases the reference count
  void Ref() { callbacks_outstanding_.fetch_add(1, std::memory_order_relaxed); }

 private:
  virtual ServerReactor* reactor() = 0;

  virtual grpc_call* call() = 0;

  virtual void RunAsync(absl::AnyInvocable<void()> cb) {
    grpc_call_run_in_event_engine(call(), std::move(cb));
  }

  // CallOnDone performs the work required at completion of the RPC: invoking
  // the OnDone function and doing all necessary cleanup. This function is only
  // ever invoked on a fully-Unref'fed ServerCallbackCall.
  virtual void CallOnDone() = 0;

  // If the OnDone reaction is inlineable, execute it inline. Otherwise send it
  // to an executor.
  void ScheduleOnDone(bool inline_ondone);

  // If the OnCancel reaction is inlineable, execute it inline. Otherwise send
  // it to an executor.
  void CallOnCancel(ServerReactor* reactor);

  // Implement the cancellation constraint counter. Return true if OnCancel
  // should be called, false otherwise.
  bool UnblockCancellation() {
    return on_cancel_conditions_remaining_.fetch_sub(
               1, std::memory_order_acq_rel) == 1;
  }

  /// Decreases the reference count and returns the previous value
  int Unref() {
    return callbacks_outstanding_.fetch_sub(1, std::memory_order_acq_rel);
  }

  std::atomic_int on_cancel_conditions_remaining_{2};
  std::atomic_int callbacks_outstanding_{
      3};  // reserve for start, Finish, and CompletionOp
};

template <class Request, class Response>
class DefaultMessageHolder : public MessageHolder<Request, Response> {
 public:
  DefaultMessageHolder() {
    this->set_request(&request_obj_);
    this->set_response(&response_obj_);
  }
  void Release() override {
    // the object is allocated in the call arena.
    this->~DefaultMessageHolder<Request, Response>();
  }

 private:
  Request request_obj_;
  Response response_obj_;
};

}  // namespace internal

// Forward declarations
class ServerUnaryReactor;
template <class Request>
class ServerReadReactor;
template <class Response>
class ServerWriteReactor;
template <class Request, class Response>
class ServerBidiReactor;

// NOTE: The actual call/stream object classes are provided as API only to
// support mocking. There are no implementations of these class interfaces in
// the API.
class ServerCallbackUnary : public internal::ServerCallbackCall {
 public:
  ~ServerCallbackUnary() override {}
  virtual void Finish(grpc::Status s) = 0;
  virtual void SendInitialMetadata() = 0;

 protected:
  // Use a template rather than explicitly specifying ServerUnaryReactor to
  // delay binding and avoid a circular forward declaration issue
  template <class Reactor>
  void BindReactor(Reactor* reactor) {
    reactor->InternalBindCall(this);
  }
};

template <class Request>
class ServerCallbackReader : public internal::ServerCallbackCall {
 public:
  ~ServerCallbackReader() override {}
  virtual void Finish(grpc::Status s) = 0;
  virtual void SendInitialMetadata() = 0;
  virtual void Read(Request* msg) = 0;

 protected:
  void BindReactor(ServerReadReactor<Request>* reactor) {
    reactor->InternalBindReader(this);
  }
};

template <class Response>
class ServerCallbackWriter : public internal::ServerCallbackCall {
 public:
  ~ServerCallbackWriter() override {}

  virtual void Finish(grpc::Status s) = 0;
  virtual void SendInitialMetadata() = 0;
  virtual void Write(const Response* msg, grpc::WriteOptions options) = 0;
  virtual void WriteAndFinish(const Response* msg, grpc::WriteOptions options,
                              grpc::Status s) = 0;

 protected:
  void BindReactor(ServerWriteReactor<Response>* reactor) {
    reactor->InternalBindWriter(this);
  }
};

template <class Request, class Response>
class ServerCallbackReaderWriter : public internal::ServerCallbackCall {
 public:
  ~ServerCallbackReaderWriter() override {}

  virtual void Finish(grpc::Status s) = 0;
  virtual void SendInitialMetadata() = 0;
  virtual void Read(Request* msg) = 0;
  virtual void Write(const Response* msg, grpc::WriteOptions options) = 0;
  virtual void WriteAndFinish(const Response* msg, grpc::WriteOptions options,
                              grpc::Status s) = 0;

 protected:
  void BindReactor(ServerBidiReactor<Request, Response>* reactor) {
    reactor->InternalBindStream(this);
  }
};

// The following classes are the reactor interfaces that are to be implemented
// by the user, returned as the output parameter of the method handler for a
// callback method. Note that none of the classes are pure; all reactions have a
// default empty reaction so that the user class only needs to override those
// reactions that it cares about. The reaction methods will be invoked by the
// library in response to the completion of various operations. Reactions must
// not include blocking operations (such as blocking I/O, starting synchronous
// RPCs, or waiting on condition variables). Reactions may be invoked
// concurrently, except that OnDone is called after all others (assuming proper
// API usage). The reactor may not be deleted until OnDone is called.

/// \a ServerBidiReactor is the interface for a bidirectional streaming RPC.
template <class Request, class Response>
class ServerBidiReactor : public internal::ServerReactor {
 public:
  // NOTE: Initializing stream_ as a constructor initializer rather than a
  //       default initializer because gcc-4.x requires a copy constructor for
  //       default initializing a templated member, which isn't ok for atomic.
  // TODO(vjpai): Switch to default constructor and default initializer when
  //              gcc-4.x is no longer supported
  ServerBidiReactor() : stream_(nullptr) {}
  ~ServerBidiReactor() override = default;

  /// Send any initial metadata stored in the RPC context. If not invoked,
  /// any initial metadata will be passed along with the first Write or the
  /// Finish (if there are no writes).
  void StartSendInitialMetadata() ABSL_LOCKS_EXCLUDED(stream_mu_) {
    ServerCallbackReaderWriter<Request, Response>* stream =
        stream_.load(std::memory_order_acquire);
    if (stream == nullptr) {
      grpc::internal::MutexLock l(&stream_mu_);
      stream = stream_.load(std::memory_order_relaxed);
      if (stream == nullptr) {
        backlog_.send_initial_metadata_wanted = true;
        return;
      }
    }
    stream->SendInitialMetadata();
  }

  /// Initiate a read operation.
  ///
  /// \param[out] req Where to eventually store the read message. Valid when
  ///                 the library calls OnReadDone
  void StartRead(Request* req) ABSL_LOCKS_EXCLUDED(stream_mu_) {
    ServerCallbackReaderWriter<Request, Response>* stream =
        stream_.load(std::memory_order_acquire);
    if (stream == nullptr) {
      grpc::internal::MutexLock l(&stream_mu_);
      stream = stream_.load(std::memory_order_relaxed);
      if (stream == nullptr) {
        backlog_.read_wanted = req;
        return;
      }
    }
    stream->Read(req);
  }

  /// Initiate a write operation.
  ///
  /// \param[in] resp The message to be written. The library does not take
  ///                 ownership but the caller must ensure that the message is
  ///                 not deleted or modified until OnWriteDone is called.
  void StartWrite(const Response* resp) {
    StartWrite(resp, grpc::WriteOptions());
  }

  /// Initiate a write operation with specified options.
  ///
  /// \param[in] resp The message to be written. The library does not take
  ///                 ownership but the caller must ensure that the message is
  ///                 not deleted or modified until OnWriteDone is called.
  /// \param[in] options The WriteOptions to use for writing this message
  void StartWrite(const Response* resp, grpc::WriteOptions options)
      ABSL_LOCKS_EXCLUDED(stream_mu_) {
    ServerCallbackReaderWriter<Request, Response>* stream =
        stream_.load(std::memory_order_acquire);
    if (stream == nullptr) {
      grpc::internal::MutexLock l(&stream_mu_);
      stream = stream_.load(std::memory_order_relaxed);
      if (stream == nullptr) {
        backlog_.write_wanted = resp;
        backlog_.write_options_wanted = options;
        return;
      }
    }
    stream->Write(resp, options);
  }

  /// Initiate a write operation with specified options and final RPC Status,
  /// which also causes any trailing metadata for this RPC to be sent out.
  /// StartWriteAndFinish is like merging StartWriteLast and Finish into a
  /// single step. A key difference, though, is that this operation doesn't have
  /// an OnWriteDone reaction - it is considered complete only when OnDone is
  /// available. An RPC can either have StartWriteAndFinish or Finish, but not
  /// both.
  ///
  /// \param[in] resp The message to be written. The library does not take
  ///                 ownership but the caller must ensure that the message is
  ///                 not deleted or modified until OnDone is called.
  /// \param[in] options The WriteOptions to use for writing this message
  /// \param[in] s The status outcome of this RPC
  void StartWriteAndFinish(const Response* resp, grpc::WriteOptions options,
                           grpc::Status s) ABSL_LOCKS_EXCLUDED(stream_mu_) {
    ServerCallbackReaderWriter<Request, Response>* stream =
        stream_.load(std::memory_order_acquire);
    if (stream == nullptr) {
      grpc::internal::MutexLock l(&stream_mu_);
      stream = stream_.load(std::memory_order_relaxed);
      if (stream == nullptr) {
        backlog_.write_and_finish_wanted = true;
        backlog_.write_wanted = resp;
        backlog_.write_options_wanted = options;
        backlog_.status_wanted = std::move(s);
        return;
      }
    }
    stream->WriteAndFinish(resp, options, std::move(s));
  }

  /// Inform system of a planned write operation with specified options, but
  /// allow the library to schedule the actual write coalesced with the writing
  /// of trailing metadata (which takes place on a Finish call).
  ///
  /// \param[in] resp The message to be written. The library does not take
  ///                 ownership but the caller must ensure that the message is
  ///                 not deleted or modified until OnWriteDone is called.
  /// \param[in] options The WriteOptions to use for writing this message
  void StartWriteLast(const Response* resp, grpc::WriteOptions options) {
    StartWrite(resp, options.set_last_message());
  }

  /// Indicate that the stream is to be finished and the trailing metadata and
  /// RPC status are to be sent. Every RPC MUST be finished using either Finish
  /// or StartWriteAndFinish (but not both), even if the RPC is already
  /// cancelled.
  ///
  /// \param[in] s The status outcome of this RPC
  void Finish(grpc::Status s) ABSL_LOCKS_EXCLUDED(stream_mu_) {
    ServerCallbackReaderWriter<Request, Response>* stream =
        stream_.load(std::memory_order_acquire);
    if (stream == nullptr) {
      grpc::internal::MutexLock l(&stream_mu_);
      stream = stream_.load(std::memory_order_relaxed);
      if (stream == nullptr) {
        backlog_.finish_wanted = true;
        backlog_.status_wanted = std::move(s);
        return;
      }
    }
    stream->Finish(std::move(s));
  }

  /// Notifies the application that an explicit StartSendInitialMetadata
  /// operation completed. Not used when the sending of initial metadata
  /// piggybacks onto the first write.
  ///
  /// \param[in] ok Was it successful? If false, no further write-side operation
  ///               will succeed.
  virtual void OnSendInitialMetadataDone(bool /*ok*/) {}

  /// Notifies the application that a StartRead operation completed.
  ///
  /// \param[in] ok Was it successful? If false, no further read-side operation
  ///               will succeed.
  virtual void OnReadDone(bool /*ok*/) {}

  /// Notifies the application that a StartWrite (or StartWriteLast) operation
  /// completed.
  ///
  /// \param[in] ok Was it successful? If false, no further write-side operation
  ///               will succeed.
  virtual void OnWriteDone(bool /*ok*/) {}

  /// Notifies the application that all operations associated with this RPC
  /// have completed. This is an override (from the internal base class) but
  /// still abstract, so derived classes MUST override it to be instantiated.
  void OnDone() override = 0;

  /// Notifies the application that this RPC has been cancelled. This is an
  /// override (from the internal base class) but not final, so derived classes
  /// should override it if they want to take action.
  void OnCancel() override {}

 private:
  friend class ServerCallbackReaderWriter<Request, Response>;
  // May be overridden by internal implementation details. This is not a public
  // customization point.
  virtual void InternalBindStream(
      ServerCallbackReaderWriter<Request, Response>* stream) {
    grpc::internal::MutexLock l(&stream_mu_);

    if (GPR_UNLIKELY(backlog_.send_initial_metadata_wanted)) {
      stream->SendInitialMetadata();
    }
    if (GPR_UNLIKELY(backlog_.read_wanted != nullptr)) {
      stream->Read(backlog_.read_wanted);
    }
    if (GPR_UNLIKELY(backlog_.write_and_finish_wanted)) {
      stream->WriteAndFinish(backlog_.write_wanted,
                             std::move(backlog_.write_options_wanted),
                             std::move(backlog_.status_wanted));
    } else {
      if (GPR_UNLIKELY(backlog_.write_wanted != nullptr)) {
        stream->Write(backlog_.write_wanted,
                      std::move(backlog_.write_options_wanted));
      }
      if (GPR_UNLIKELY(backlog_.finish_wanted)) {
        stream->Finish(std::move(backlog_.status_wanted));
      }
    }
    // Set stream_ last so that other functions can use it lock-free
    stream_.store(stream, std::memory_order_release);
  }

  grpc::internal::Mutex stream_mu_;
  // TODO(vjpai): Make stream_or_backlog_ into an std::variant once C++17 or
  // ABSL is supported since stream and backlog are mutually exclusive in this
  // class. Do likewise with the  remaining reactor classes and their backlogs
  // as well.
  std::atomic<ServerCallbackReaderWriter<Request, Response>*> stream_{nullptr};
  struct PreBindBacklog {
    bool send_initial_metadata_wanted = false;
    bool write_and_finish_wanted = false;
    bool finish_wanted = false;
    Request* read_wanted = nullptr;
    const Response* write_wanted = nullptr;
    grpc::WriteOptions write_options_wanted;
    grpc::Status status_wanted;
  };
  PreBindBacklog backlog_ ABSL_GUARDED_BY(stream_mu_);
};

/// \a ServerReadReactor is the interface for a client-streaming RPC.
template <class Request>
class ServerReadReactor : public internal::ServerReactor {
 public:
  ServerReadReactor() : reader_(nullptr) {}
  ~ServerReadReactor() override = default;

  /// The following operation initiations are exactly like ServerBidiReactor.
  void StartSendInitialMetadata() ABSL_LOCKS_EXCLUDED(reader_mu_) {
    ServerCallbackReader<Request>* reader =
        reader_.load(std::memory_order_acquire);
    if (reader == nullptr) {
      grpc::internal::MutexLock l(&reader_mu_);
      reader = reader_.load(std::memory_order_relaxed);
      if (reader == nullptr) {
        backlog_.send_initial_metadata_wanted = true;
        return;
      }
    }
    reader->SendInitialMetadata();
  }
  void StartRead(Request* req) ABSL_LOCKS_EXCLUDED(reader_mu_) {
    ServerCallbackReader<Request>* reader =
        reader_.load(std::memory_order_acquire);
    if (reader == nullptr) {
      grpc::internal::MutexLock l(&reader_mu_);
      reader = reader_.load(std::memory_order_relaxed);
      if (reader == nullptr) {
        backlog_.read_wanted = req;
        return;
      }
    }
    reader->Read(req);
  }
  void Finish(grpc::Status s) ABSL_LOCKS_EXCLUDED(reader_mu_) {
    ServerCallbackReader<Request>* reader =
        reader_.load(std::memory_order_acquire);
    if (reader == nullptr) {
      grpc::internal::MutexLock l(&reader_mu_);
      reader = reader_.load(std::memory_order_relaxed);
      if (reader == nullptr) {
        backlog_.finish_wanted = true;
        backlog_.status_wanted = std::move(s);
        return;
      }
    }
    reader->Finish(std::move(s));
  }

  /// The following notifications are exactly like ServerBidiReactor.
  virtual void OnSendInitialMetadataDone(bool /*ok*/) {}
  virtual void OnReadDone(bool /*ok*/) {}
  void OnDone() override = 0;
  void OnCancel() override {}

 private:
  friend class ServerCallbackReader<Request>;

  // May be overridden by internal implementation details. This is not a public
  // customization point.
  virtual void InternalBindReader(ServerCallbackReader<Request>* reader)
      ABSL_LOCKS_EXCLUDED(reader_mu_) {
    grpc::internal::MutexLock l(&reader_mu_);

    if (GPR_UNLIKELY(backlog_.send_initial_metadata_wanted)) {
      reader->SendInitialMetadata();
    }
    if (GPR_UNLIKELY(backlog_.read_wanted != nullptr)) {
      reader->Read(backlog_.read_wanted);
    }
    if (GPR_UNLIKELY(backlog_.finish_wanted)) {
      reader->Finish(std::move(backlog_.status_wanted));
    }
    // Set reader_ last so that other functions can use it lock-free
    reader_.store(reader, std::memory_order_release);
  }

  grpc::internal::Mutex reader_mu_;
  std::atomic<ServerCallbackReader<Request>*> reader_{nullptr};
  struct PreBindBacklog {
    bool send_initial_metadata_wanted = false;
    bool finish_wanted = false;
    Request* read_wanted = nullptr;
    grpc::Status status_wanted;
  };
  PreBindBacklog backlog_ ABSL_GUARDED_BY(reader_mu_);
};

/// \a ServerWriteReactor is the interface for a server-streaming RPC.
template <class Response>
class ServerWriteReactor : public internal::ServerReactor {
 public:
  ServerWriteReactor() : writer_(nullptr) {}
  ~ServerWriteReactor() override = default;

  /// The following operation initiations are exactly like ServerBidiReactor.
  void StartSendInitialMetadata() ABSL_LOCKS_EXCLUDED(writer_mu_) {
    ServerCallbackWriter<Response>* writer =
        writer_.load(std::memory_order_acquire);
    if (writer == nullptr) {
      grpc::internal::MutexLock l(&writer_mu_);
      writer = writer_.load(std::memory_order_relaxed);
      if (writer == nullptr) {
        backlog_.send_initial_metadata_wanted = true;
        return;
      }
    }
    writer->SendInitialMetadata();
  }
  void StartWrite(const Response* resp) {
    StartWrite(resp, grpc::WriteOptions());
  }
  void StartWrite(const Response* resp, grpc::WriteOptions options)
      ABSL_LOCKS_EXCLUDED(writer_mu_) {
    ServerCallbackWriter<Response>* writer =
        writer_.load(std::memory_order_acquire);
    if (writer == nullptr) {
      grpc::internal::MutexLock l(&writer_mu_);
      writer = writer_.load(std::memory_order_relaxed);
      if (writer == nullptr) {
        backlog_.write_wanted = resp;
        backlog_.write_options_wanted = options;
        return;
      }
    }
    writer->Write(resp, options);
  }
  void StartWriteAndFinish(const Response* resp, grpc::WriteOptions options,
                           grpc::Status s) ABSL_LOCKS_EXCLUDED(writer_mu_) {
    ServerCallbackWriter<Response>* writer =
        writer_.load(std::memory_order_acquire);
    if (writer == nullptr) {
      grpc::internal::MutexLock l(&writer_mu_);
      writer = writer_.load(std::memory_order_relaxed);
      if (writer == nullptr) {
        backlog_.write_and_finish_wanted = true;
        backlog_.write_wanted = resp;
        backlog_.write_options_wanted = options;
        backlog_.status_wanted = std::move(s);
        return;
      }
    }
    writer->WriteAndFinish(resp, options, std::move(s));
  }
  void StartWriteLast(const Response* resp, grpc::WriteOptions options) {
    StartWrite(resp, options.set_last_message());
  }
  void Finish(grpc::Status s) ABSL_LOCKS_EXCLUDED(writer_mu_) {
    ServerCallbackWriter<Response>* writer =
        writer_.load(std::memory_order_acquire);
    if (writer == nullptr) {
      grpc::internal::MutexLock l(&writer_mu_);
      writer = writer_.load(std::memory_order_relaxed);
      if (writer == nullptr) {
        backlog_.finish_wanted = true;
        backlog_.status_wanted = std::move(s);
        return;
      }
    }
    writer->Finish(std::move(s));
  }

  /// The following notifications are exactly like ServerBidiReactor.
  virtual void OnSendInitialMetadataDone(bool /*ok*/) {}
  virtual void OnWriteDone(bool /*ok*/) {}
  void OnDone() override = 0;
  void OnCancel() override {}

 private:
  friend class ServerCallbackWriter<Response>;
  // May be overridden by internal implementation details. This is not a public
  // customization point.
  virtual void InternalBindWriter(ServerCallbackWriter<Response>* writer)
      ABSL_LOCKS_EXCLUDED(writer_mu_) {
    grpc::internal::MutexLock l(&writer_mu_);

    if (GPR_UNLIKELY(backlog_.send_initial_metadata_wanted)) {
      writer->SendInitialMetadata();
    }
    if (GPR_UNLIKELY(backlog_.write_and_finish_wanted)) {
      writer->WriteAndFinish(backlog_.write_wanted,
                             std::move(backlog_.write_options_wanted),
                             std::move(backlog_.status_wanted));
    } else {
      if (GPR_UNLIKELY(backlog_.write_wanted != nullptr)) {
        writer->Write(backlog_.write_wanted,
                      std::move(backlog_.write_options_wanted));
      }
      if (GPR_UNLIKELY(backlog_.finish_wanted)) {
        writer->Finish(std::move(backlog_.status_wanted));
      }
    }
    // Set writer_ last so that other functions can use it lock-free
    writer_.store(writer, std::memory_order_release);
  }

  grpc::internal::Mutex writer_mu_;
  std::atomic<ServerCallbackWriter<Response>*> writer_{nullptr};
  struct PreBindBacklog {
    bool send_initial_metadata_wanted = false;
    bool write_and_finish_wanted = false;
    bool finish_wanted = false;
    const Response* write_wanted = nullptr;
    grpc::WriteOptions write_options_wanted;
    grpc::Status status_wanted;
  };
  PreBindBacklog backlog_ ABSL_GUARDED_BY(writer_mu_);
};

class ServerUnaryReactor : public internal::ServerReactor {
 public:
  ServerUnaryReactor() : call_(nullptr) {}
  ~ServerUnaryReactor() override = default;

  /// StartSendInitialMetadata is exactly like ServerBidiReactor.
  void StartSendInitialMetadata() ABSL_LOCKS_EXCLUDED(call_mu_) {
    ServerCallbackUnary* call = call_.load(std::memory_order_acquire);
    if (call == nullptr) {
      grpc::internal::MutexLock l(&call_mu_);
      call = call_.load(std::memory_order_relaxed);
      if (call == nullptr) {
        backlog_.send_initial_metadata_wanted = true;
        return;
      }
    }
    call->SendInitialMetadata();
  }
  /// Finish is similar to ServerBidiReactor except for one detail.
  /// If the status is non-OK, any message will not be sent. Instead,
  /// the client will only receive the status and any trailing metadata.
  void Finish(grpc::Status s) ABSL_LOCKS_EXCLUDED(call_mu_) {
    ServerCallbackUnary* call = call_.load(std::memory_order_acquire);
    if (call == nullptr) {
      grpc::internal::MutexLock l(&call_mu_);
      call = call_.load(std::memory_order_relaxed);
      if (call == nullptr) {
        backlog_.finish_wanted = true;
        backlog_.status_wanted = std::move(s);
        return;
      }
    }
    call->Finish(std::move(s));
  }

  /// The following notifications are exactly like ServerBidiReactor.
  virtual void OnSendInitialMetadataDone(bool /*ok*/) {}
  void OnDone() override = 0;
  void OnCancel() override {}

 private:
  friend class ServerCallbackUnary;
  // May be overridden by internal implementation details. This is not a public
  // customization point.
  virtual void InternalBindCall(ServerCallbackUnary* call)
      ABSL_LOCKS_EXCLUDED(call_mu_) {
    grpc::internal::MutexLock l(&call_mu_);

    if (GPR_UNLIKELY(backlog_.send_initial_metadata_wanted)) {
      call->SendInitialMetadata();
    }
    if (GPR_UNLIKELY(backlog_.finish_wanted)) {
      call->Finish(std::move(backlog_.status_wanted));
    }
    // Set call_ last so that other functions can use it lock-free
    call_.store(call, std::memory_order_release);
  }

  grpc::internal::Mutex call_mu_;
  std::atomic<ServerCallbackUnary*> call_{nullptr};
  struct PreBindBacklog {
    bool send_initial_metadata_wanted = false;
    bool finish_wanted = false;
    grpc::Status status_wanted;
  };
  PreBindBacklog backlog_ ABSL_GUARDED_BY(call_mu_);
};

namespace internal {

template <class Base>
class FinishOnlyReactor : public Base {
 public:
  explicit FinishOnlyReactor(grpc::Status s) { this->Finish(std::move(s)); }
  void OnDone() override { this->~FinishOnlyReactor(); }
};

using UnimplementedUnaryReactor = FinishOnlyReactor<ServerUnaryReactor>;
template <class Request>
using UnimplementedReadReactor = FinishOnlyReactor<ServerReadReactor<Request>>;
template <class Response>
using UnimplementedWriteReactor =
    FinishOnlyReactor<ServerWriteReactor<Response>>;
template <class Request, class Response>
using UnimplementedBidiReactor =
    FinishOnlyReactor<ServerBidiReactor<Request, Response>>;

}  // namespace internal

// TODO(vjpai): Remove namespace experimental when last known users are migrated
// off.
namespace experimental {

template <class Request, class Response>
using ServerBidiReactor = ::grpc::ServerBidiReactor<Request, Response>;

}  // namespace experimental

}  // namespace grpc

#endif  // GRPCPP_SUPPORT_SERVER_CALLBACK_H
