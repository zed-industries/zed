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

#ifndef GRPCPP_SUPPORT_INTERCEPTOR_H
#define GRPCPP_SUPPORT_INTERCEPTOR_H

#include <grpc/impl/grpc_types.h>
#include <grpcpp/impl/metadata_map.h>
#include <grpcpp/support/byte_buffer.h>
#include <grpcpp/support/config.h>
#include <grpcpp/support/string_ref.h>

#include <map>
#include <memory>
#include <string>

namespace grpc {

class ChannelInterface;
class Status;

namespace experimental {

/// An enumeration of different possible points at which the \a Intercept
/// method of the \a Interceptor interface may be called. Any given call
/// to \a Intercept will include one or more of these hook points, and
/// each hook point makes certain types of information available to the
/// interceptor.
/// In these enumeration names, PRE_SEND means that an interception has taken
/// place between the time the application provided a certain type of data
/// (e.g., initial metadata, status) and the time that that data goes to the
/// other side. POST_SEND means that the data has been committed for going to
/// the other side (even if it has not yet been received at the other side).
/// PRE_RECV means an interception between the time that a certain
/// operation has been requested and it is available. POST_RECV means that a
/// result is available but has not yet been passed back to the application.
/// A batch of interception points will only contain either PRE or POST hooks
/// but not both types. For example, a batch with PRE_SEND hook points will not
/// contain POST_RECV or POST_SEND ops. Likewise, a batch with POST_* ops can
/// not contain PRE_* ops.
enum class InterceptionHookPoints {
  /// The first three in this list are for clients and servers
  PRE_SEND_INITIAL_METADATA,
  PRE_SEND_MESSAGE,
  POST_SEND_MESSAGE,
  PRE_SEND_STATUS,  // server only
  PRE_SEND_CLOSE,   // client only: WritesDone for stream; after write in unary
  /// The following three are for hijacked clients only. A batch with PRE_RECV_*
  /// hook points will never contain hook points of other types.
  PRE_RECV_INITIAL_METADATA,
  PRE_RECV_MESSAGE,
  PRE_RECV_STATUS,
  /// The following two are for all clients and servers
  POST_RECV_INITIAL_METADATA,
  POST_RECV_MESSAGE,
  POST_RECV_STATUS,  // client only
  POST_RECV_CLOSE,   // server only
  /// This is a special hook point available to both clients and servers when
  /// TryCancel() is performed.
  ///  - No other hook points will be present along with this.
  ///  - It is illegal for an interceptor to block/delay this operation.
  ///  - ALL interceptors see this hook point irrespective of whether the
  ///    RPC was hijacked or not.
  PRE_SEND_CANCEL,
  NUM_INTERCEPTION_HOOKS
};

/// Class that is passed as an argument to the \a Intercept method
/// of the application's \a Interceptor interface implementation. It has five
/// purposes:
///   1. Indicate which hook points are present at a specific interception
///   2. Allow an interceptor to inform the library that an RPC should
///      continue to the next stage of its processing (which may be another
///      interceptor or the main path of the library)
///   3. Allow an interceptor to hijack the processing of the RPC (only for
///      client-side RPCs with PRE_SEND_INITIAL_METADATA) so that it does not
///      proceed with normal processing beyond that stage
///   4. Access the relevant fields of an RPC at each interception point
///   5. Set some fields of an RPC at each interception point, when possible
class InterceptorBatchMethods {
 public:
  virtual ~InterceptorBatchMethods() {}
  /// Determine whether the current batch has an interception hook point
  /// of type \a type
  virtual bool QueryInterceptionHookPoint(InterceptionHookPoints type) = 0;
  /// Signal that the interceptor is done intercepting the current batch of the
  /// RPC. Every interceptor must either call Proceed or Hijack on each
  /// interception. In most cases, only Proceed will be used. Explicit use of
  /// Proceed is what enables interceptors to delay the processing of RPCs
  /// while they perform other work.
  /// Proceed is a no-op if the batch contains PRE_SEND_CANCEL. Simply returning
  /// from the Intercept method does the job of continuing the RPC in this case.
  /// This is because PRE_SEND_CANCEL is always in a separate batch and is not
  /// allowed to be delayed.
  virtual void Proceed() = 0;
  /// Indicate that the interceptor has hijacked the RPC (only valid if the
  /// batch contains send_initial_metadata on the client side). Later
  /// interceptors in the interceptor list will not be called. Later batches
  /// on the same RPC will go through interception, but only up to the point
  /// of the hijacking interceptor.
  virtual void Hijack() = 0;

  /// Send Message Methods
  /// GetSerializedSendMessage and GetSendMessage/ModifySendMessage are the
  /// available methods to view and modify the request payload. An interceptor
  /// can access the payload in either serialized form or non-serialized form
  /// but not both at the same time.
  /// gRPC performs serialization in a lazy manner, which means
  /// that a call to GetSerializedSendMessage will result in a serialization
  /// operation if the payload stored is not in the serialized form already; the
  /// non-serialized form will be lost and GetSendMessage will no longer return
  /// a valid pointer, and this will remain true for later interceptors too.
  /// This can change however if ModifySendMessage is used to replace the
  /// current payload. Note that ModifySendMessage requires a new payload
  /// message in the non-serialized form. This will overwrite the existing
  /// payload irrespective of whether it had been serialized earlier. Also note
  /// that gRPC Async API requires early serialization of the payload which
  /// means that the payload would be available in the serialized form only
  /// unless an interceptor replaces the payload with ModifySendMessage.

  /// Returns a modifiable ByteBuffer holding the serialized form of the message
  /// that is going to be sent. Valid for PRE_SEND_MESSAGE interceptions.
  /// A return value of nullptr indicates that this ByteBuffer is not valid.
  virtual ByteBuffer* GetSerializedSendMessage() = 0;

  /// Returns a non-modifiable pointer to the non-serialized form of the message
  /// to be sent. Valid for PRE_SEND_MESSAGE interceptions. A return value of
  /// nullptr indicates that this field is not valid.
  virtual const void* GetSendMessage() = 0;

  /// Overwrites the message to be sent with \a message. \a message should be in
  /// the non-serialized form expected by the method. Valid for PRE_SEND_MESSAGE
  /// interceptions. Note that the interceptor is responsible for maintaining
  /// the life of the message till it is serialized or it receives the
  /// POST_SEND_MESSAGE interception point, whichever happens earlier. The
  /// modifying interceptor may itself force early serialization by calling
  /// GetSerializedSendMessage.
  virtual void ModifySendMessage(const void* message) = 0;

  /// Checks whether the SEND MESSAGE op succeeded. Valid for POST_SEND_MESSAGE
  /// interceptions.
  virtual bool GetSendMessageStatus() = 0;

  /// Returns a modifiable multimap of the initial metadata to be sent. Valid
  /// for PRE_SEND_INITIAL_METADATA interceptions. A value of nullptr indicates
  /// that this field is not valid.
  virtual std::multimap<std::string, std::string>* GetSendInitialMetadata() = 0;

  /// Returns the status to be sent. Valid for PRE_SEND_STATUS interceptions.
  virtual Status GetSendStatus() = 0;

  /// Overwrites the status with \a status. Valid for PRE_SEND_STATUS
  /// interceptions.
  virtual void ModifySendStatus(const Status& status) = 0;

  /// Returns a modifiable multimap of the trailing metadata to be sent. Valid
  /// for PRE_SEND_STATUS interceptions. A value of nullptr indicates
  /// that this field is not valid.
  virtual std::multimap<std::string, std::string>*
  GetSendTrailingMetadata() = 0;

  /// Returns a pointer to the modifiable received message. Note that the
  /// message is already deserialized but the type is not set; the interceptor
  /// should static_cast to the appropriate type before using it. This is valid
  /// for PRE_RECV_MESSAGE and POST_RECV_MESSAGE interceptions; nullptr for not
  /// valid
  virtual void* GetRecvMessage() = 0;

  /// Returns a modifiable multimap of the received initial metadata.
  /// Valid for PRE_RECV_INITIAL_METADATA and POST_RECV_INITIAL_METADATA
  /// interceptions; nullptr if not valid
  virtual std::multimap<grpc::string_ref, grpc::string_ref>*
  GetRecvInitialMetadata() = 0;

  /// Returns a modifiable view of the received status on PRE_RECV_STATUS and
  /// POST_RECV_STATUS interceptions; nullptr if not valid.
  virtual Status* GetRecvStatus() = 0;

  /// Returns a modifiable multimap of the received trailing metadata on
  /// PRE_RECV_STATUS and POST_RECV_STATUS interceptions; nullptr if not valid
  virtual std::multimap<grpc::string_ref, grpc::string_ref>*
  GetRecvTrailingMetadata() = 0;

  /// Gets an intercepted channel. When a call is started on this interceptor,
  /// only interceptors after the current interceptor are created from the
  /// factory objects registered with the channel. This allows calls to be
  /// started from interceptors without infinite regress through the interceptor
  /// list.
  virtual std::unique_ptr<ChannelInterface> GetInterceptedChannel() = 0;

  /// On a hijacked RPC, an interceptor can decide to fail a PRE_RECV_MESSAGE
  /// op. This would be a signal to the reader that there will be no more
  /// messages, or the stream has failed or been cancelled.
  virtual void FailHijackedRecvMessage() = 0;

  /// On a hijacked RPC/ to-be hijacked RPC, this can be called to fail a SEND
  /// MESSAGE op
  virtual void FailHijackedSendMessage() = 0;
};

/// Interface for an interceptor. Interceptor authors must create a class
/// that derives from this parent class.
class Interceptor {
 public:
  virtual ~Interceptor() {}

  /// The one public method of an Interceptor interface. Override this to
  /// trigger the desired actions at the hook points described above.
  virtual void Intercept(InterceptorBatchMethods* methods) = 0;
};

}  // namespace experimental
}  // namespace grpc

#endif  // GRPCPP_SUPPORT_INTERCEPTOR_H
