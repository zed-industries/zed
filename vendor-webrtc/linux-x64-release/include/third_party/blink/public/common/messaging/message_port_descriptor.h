// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_MESSAGING_MESSAGE_PORT_DESCRIPTOR_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_MESSAGING_MESSAGE_PORT_DESCRIPTOR_H_

#include "base/dcheck_is_on.h"
#include "base/unguessable_token.h"
#include "mojo/public/cpp/system/message_pipe.h"
#include "third_party/blink/public/common/common_export.h"

namespace blink {

class ExecutionContext;

// Defines a message port descriptor, which is a mojo::MessagePipeHandle and
// some associated state which follows the handle around as it is passed from
// one execution context to another. This can be serialized into a
// mojom::MessagePortDescriptor using MessagePortDescriptorStructTraits. Since
// this class uses only POD and Mojo types the same serialization logic is fine
// for use both inside and outside of Blink. This type is move-only so that only
// a single representation of an endpoint can exist at any moment in time.
// This class is not thread-safe, but a MessagePortDescriptor is only ever owned
// by a single thread at a time.
//
// A MessagePortDescriptor should never be created in isolation, but rather they
// should only be created in pairs via MessagePortDescriptorPair.
//
// To enforce that a Mojo pipe isn't left dangling, MessagePortDescriptors
// enforce that they are only destroyed while holding onto their pipe.
//
// This class is intended to be used as follows:
//
//   MessagePortDescriptorPair port_pair;
//   MessagePortDescriptor port0 = port_pair.TakePort0();
//   MessagePortDescriptor port1 = port_pair.TakePort1();
//
//   ... pass around the port descriptors in TransferableMessages as desired ...
//
//   // Pass this into a MessagePortChannel for reference-counted safe-keeping.
//   MessagePortChannel channel0(port0);
//
//   // Entangle into a MessagePort for use in sending messages.
//   MessagePort message_port;
//   message_port.Entangle(channel0)
//   message_port.postMessage(...);
//   channel0 = message_port.Disentangle();
//
// Note that there is a Java wrapper to this class implemented by
// org.chromium.content.browser.AppWebMessagePortDescriptor.
class BLINK_COMMON_EXPORT MessagePortDescriptor {
 public:
  // Delegate used to provide information about the state of message ports.
  // See full class declaration below.
  class InstrumentationDelegate;

  // Allows setting a singleton instrumentation delegate. This is not
  // thread-safe and should be set in early Blink initialization.
  static void SetInstrumentationDelegate(InstrumentationDelegate* delegate);

  MessagePortDescriptor();

  // Disallow copying, and enforce move-only semantics.
  MessagePortDescriptor(const MessagePortDescriptor& message_port) = delete;
  MessagePortDescriptor(MessagePortDescriptor&& message_port);
  MessagePortDescriptor& operator=(const MessagePortDescriptor& message_port) =
      delete;
  MessagePortDescriptor& operator=(MessagePortDescriptor&& message_port);

  ~MessagePortDescriptor();

  // Simple accessors.
  const mojo::ScopedMessagePipeHandle& handle() const { return handle_; }
  const base::UnguessableToken& id() const { return id_; }
  uint64_t sequence_number() const { return sequence_number_; }

  // Helper accessor for getting the underlying Mojo handle. Makes tests a
  // little easier to write.
  MojoHandle GetMojoHandleForTesting() const;

  // Returns true if this is a valid descriptor.
  bool IsValid() const;

  // Returns true if this descriptor is currently entangled (meaning that the
  // handle has been vended out via "TakeHandleToEntangle*").
  bool IsEntangled() const;

  // Returns true if this is a default initialized descriptor.
  bool IsDefault() const;

  // Resets the descriptor, closing the pipe if this is a valid descriptor.
  // After calling this "IsDefault" will return true. It is not valid to call
  // this on a descriptor whose handle is currently entangled (taken via
  // "TakeHandleToEntangle*" but not yet returned) or in the process of being
  // serialized.
  void Reset();

  // These are only meant to be used for serialization, and as such the values
  // should always be non-default initialized when they are called. These should
  // only be called for descriptors that actually host non-default values. If
  // you start serializing an object by calling any of the
  // "TakeFooForSerialization" functions it is expected (and enforced by
  // DCHECKs) that you will call all of them. Don't use these unless you really
  // need to!
  void InitializeFromSerializedValues(mojo::ScopedMessagePipeHandle handle,
                                      const base::UnguessableToken& id,
                                      uint64_t sequence_number);
  mojo::ScopedMessagePipeHandle TakeHandleForSerialization();
  base::UnguessableToken TakeIdForSerialization();
  uint64_t TakeSequenceNumberForSerialization();

  // The following functions are only intended to be used by classes that
  // implemented message port endpoints, like blink::MessagePort (for internal
  // use from content and blink), blink::WebMessagePort (for embedder use from
  // C++ code) and org.chromium.content.browser.AppWebMessagePort
  // (implementation of org.chromium.content_public.browser.MessagePort, which
  // is intended for embedder use in Java code).

  // Intended for use by MessagePort, for binding/unbinding the handle to/from a
  // mojo::Connector. The handle must be bound directly to a mojo::Connector in
  // order for messages to be sent or received. MessagePort::Entangle is passed
  // a MessagePortDescriptor, and takes the handle from the descriptor in order
  // to bind it to the mojo::Connector. Similarly, MessagePort::Disentangle
  // takes the handle back from the mojo::Connector, gives it back to the
  // MessagePortDescriptor, and releases the MessagePortDescriptor to the
  // caller. See MessagePort::Entangle and MessagePort::Disentangle.
  mojo::ScopedMessagePipeHandle TakeHandleToEntangle(
      ExecutionContext* execution_context);

  // Intended for use by WebMessagePort and the corresponding
  // org.chromium.content.browser.AppWebMessagePort, which are the interfaces
  // that embedders use for communicating with hosted content.
  mojo::ScopedMessagePipeHandle TakeHandleToEntangleWithEmbedder();

  // Returns a handle that was previously taken for entangling via
  // "TakeHandleToEntangle*". Passing an invalid handle indicates that the
  // handle was forcibly closed due to error while vended out by the descriptor.
  // TODO(chrisha): Close the loop and move the connector inside of the
  // descriptor, by making TakeHandleToEntangle vend a bound Connector.
  void GiveDisentangledHandle(mojo::ScopedMessagePipeHandle handle);

 private:
  // For access to NotifyPeer and the following constructor.
  friend class MessagePortDescriptorPair;

  // Creates a new MessagePortDescriptor that wraps the provided brand new
  // handle. A unique id and starting sequence number will be generated. Only
  // meant to be called from MessagePortDescriptorPair.
  explicit MessagePortDescriptor(mojo::ScopedMessagePipeHandle handle);

  // Helper functions for forwarding notifications to the
  // InstrumentationDelegate if it exists.
  void NotifyAttached(ExecutionContext* execution_context);
  void NotifyAttachedToEmbedder();
  void NotifyDetached();
  void NotifyDestroyed();

  // Checks that the serialization state of the object is valid. Only
  // meaningful in DCHECK builds.
  void EnsureNotSerialized() const;
  void EnsureValidSerializationState() const;

  static constexpr size_t kInvalidSequenceNumber = 0;
  static constexpr size_t kFirstValidSequenceNumber = 1;

  // The handle to the underlying pipe.
  mojo::ScopedMessagePipeHandle handle_;

#if DCHECK_IS_ON()
  // Keeps track of serialization status. An object will explode if
  // serialization is only ever half-completed.
  struct SerializationState {
    bool took_handle_for_serialization_ : 1;
    bool took_id_for_serialization_ : 1;
    bool took_sequence_number_for_serialization_ : 1;
  } serialization_state_ = {};
#endif

  // The randomly generated ID of this message handle. The ID follows this
  // MessagePortDescriptor around as it is passed between execution contexts,
  // and allows for bookkeeping across the various contexts. When an execution
  // context entangles itself with a handle, it will report back to the browser
  // the ID of the handle and the execution context as well. This allows for the
  // browser to know who is talking to who.
  base::UnguessableToken id_;

  // The sequence number of the instrumentation message related to this handle.
  // Since these messages can arrive out of order in the browser process, this
  // is used to reorder them so that consistent state can be maintained. This
  // will never be zero for a valid port descriptor.
  uint64_t sequence_number_ = kInvalidSequenceNumber;
};

// Defines a wrapped mojo::MessagePipe, containing 2 MessagePortDescriptors that
// are peers of each other.
class BLINK_COMMON_EXPORT MessagePortDescriptorPair {
 public:
  MessagePortDescriptorPair();
  ~MessagePortDescriptorPair();

  const MessagePortDescriptor& port0() const { return port0_; }
  const MessagePortDescriptor& port1() const { return port1_; }

  MessagePortDescriptor TakePort0() { return std::move(port0_); }
  MessagePortDescriptor TakePort1() { return std::move(port1_); }

 private:
  MessagePortDescriptor port0_;
  MessagePortDescriptor port1_;
};

// A delegate used for instrumenting operations on message handle descriptors.
// These messages allow the observing entity to follow the handle endpoints as
// they travel from one execution context to another, and to know when they are
// bound. If no instrumentation delegate is provided the instrumentation is
// disabled. The implementation needs to be thread-safe.
//
// Note that the sequence numbers associated with each port will never be
// reused, and increment by exactly one for each message received. This allows
// the receiver to order incoming messages and know if a message is missing.
// NotifyMessagePortsCreated is special in that it introduces new ports and
// starts new sequence numbers. The next message received (either a PortAttached
// or PortDestroyed message) will use the subsequent sequence number.
class BLINK_COMMON_EXPORT MessagePortDescriptor::InstrumentationDelegate {
 public:
  virtual ~InstrumentationDelegate() = default;

  // Notifies the instrumentation that a pair of matching ports was created.
  virtual void NotifyMessagePortPairCreated(
      const base::UnguessableToken& port0_id,
      const base::UnguessableToken& port1_id) = 0;

  // Notifies the instrumentation that a handle has been attached to an
  // execution context. Note that |execution_context| should never be null, but
  // it is valid for "execution_context->IsContextDestroyed()" to return true.
  // Further note that this should only ever be called by blink::MessagePort.
  // All other contexts should be calling NotifyMessagePortAttachedToEmbedder.
  virtual void NotifyMessagePortAttached(
      const base::UnguessableToken& port_id,
      uint64_t sequence_number,
      ExecutionContext* execution_context) = 0;

  // Notifies the instrumentation that a handle has been attached to an
  // embedder.
  virtual void NotifyMessagePortAttachedToEmbedder(
      const base::UnguessableToken& port_id,
      uint64_t sequence_number) = 0;

  // Notifies the instrumentation that a handle has been detached from an
  // execution context.
  virtual void NotifyMessagePortDetached(const base::UnguessableToken& port_id,
                                         uint64_t sequence_number) = 0;

  // Notifies the instrumentation that a handle has been destroyed.
  virtual void NotifyMessagePortDestroyed(const base::UnguessableToken& port_id,
                                          uint64_t sequence_number) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_MESSAGING_MESSAGE_PORT_DESCRIPTOR_H_
