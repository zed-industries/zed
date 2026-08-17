// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_MESSAGING_WEB_MESSAGE_PORT_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_MESSAGING_WEB_MESSAGE_PORT_H_

#include <string>
#include <utility>

#include "base/memory/raw_ptr.h"
#include "base/task/sequenced_task_runner.h"
#include "mojo/public/cpp/bindings/connector.h"
#include "mojo/public/cpp/bindings/message.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/messaging/message_port_descriptor.h"

namespace blink {

// Defines a WebMessagePort, which is used by embedders to send and receive
// messages to and from Javascript content. This is a simplified version of the
// full MessagePort concept implemented by renderer/core/messaging/message_port.
// It is a lightweight wrapper of a Mojo message pipe, and provides
// functionality for sending and receiving messages that consist of text content
// and a vector of ports, automatically handling the message serialization and
// formatting. Note the Mojo pipe is bound in SINGLE_THREADED_SEND mode. If you
// need to send more complex message types then refer to transferable_message.*
// and string_message_codec.*.
//
// Intended embedder usage is as follows:
//
//  // Create a pair of ports. The two ends of the pipe are conjugates of each
//  // other.
//  std::pair<WebMessagePort, WebMessagePort> ports =
//      WebMessagePort::CreatePair();
//
//  // Keep one end for ourselves.
//  MessageReceiverImpl receiver;  // Implements MessageReceiver.
//  auto embedder_port = std::move(ports.first);
//  embedder_port.SetReceiver(&receiver, task_runner);
//
//  // Send the other end of the pipe to a WebContents. This will arrive in the
//  // main frame of that WebContents. See
//  // content/browser/public/message_port_provider.h for the API that allows
//  // this injection.
//  std::vector<MessagePortDescriptor> ports;
//  ports.emplace_back(ports.second.PassPort());
//  MessagePortProvider::PostMessageToFrame(
//      web_contents, ..., std::move(ports));
//
//  // The web contents can now talk back to us via |embedder_port|, and we can
//  // talk back directly to it over that same pipe.
//
// Note that some embedders provide "PostMessageToFrame" functions directly on
// their wrapped WebContents equivalents (Android and Cast, for example). Also
// note that for Android embedders, there are equivalent Java interfaces defined
// in org.chromium.content_public.browser.
//
// This is a move-only type, which makes it (almost) impossible to try to send a
// port across itself (which is illegal). This doesn't explicitly prevent you
// from sending a port's conjugate port to its conjugate, but note that the
// underlying impl will close the pipe with an error if you do that.
//
// This object is not thread safe, and is intended to be used from a single
// sequence. The sequence from which it is used does not have to be the same
// sequence that the bound receiver uses.
//
// Further note that a WebMessagePort is not "reusable". That is, once it has
// been bound via SetReceiver, it is no longer transmittable (can't be passed as
// a port in part of a Message). This is enforced via runtime CHECKs.
class BLINK_COMMON_EXPORT WebMessagePort : public mojo::MessageReceiver {
 public:
  // See below for definitions.
  struct Message;
  class MessageReceiver;

  WebMessagePort();
  WebMessagePort(const WebMessagePort&) = delete;
  WebMessagePort(WebMessagePort&&);
  WebMessagePort& operator=(const WebMessagePort&) = delete;
  WebMessagePort& operator=(WebMessagePort&&);
  ~WebMessagePort() override;

  // Factory function for creating two ends of a message channel. The two ports
  // are conjugates of each other.
  static std::pair<WebMessagePort, WebMessagePort> CreatePair();

  // Wraps one end of a message channel. |port|'s mojo pipe must
  // be paired, valid and not entangled.
  static WebMessagePort Create(MessagePortDescriptor port);

  // Sets a message receiver for this message port. Once bound any incoming
  // messages to this port will be routed to the provided |receiver| with
  // callbacks invoked on the provided |runner|. Note that if you set a receiver
  // *after* a pipe has already transitioned to being in error, you will not
  // receive an "OnPipeError" callback; you should instead manually check
  // "is_errored" before setting the receiver. Once a receiver has been set a
  // WebMessagePort is no longer transferable.
  void SetReceiver(MessageReceiver* receiver,
                   scoped_refptr<base::SequencedTaskRunner> runner);

  // Clears the message receiver for this message port. Without a receiver
  // incoming messages will be queued on the port until a receiver is set.
  // Note that it is possible that there are pending message tasks already
  // posted to the previous |receiver|, thus the previous |receiver| may
  // continue to be invoked after calling this.
  void ClearReceiver();

  // Returns true if this WebMessagePort currently has a receiver.
  bool HasReceiver() const { return receiver_; }

  // Returns the receiver to which this WebMessagePort is bound. This can
  // return nullptr if it has not been bound to a receiver.
  MessageReceiver* receiver() const { return receiver_; }

  // Returns the task runner to which this WebMessagePort is bound. This can
  // return nullptr if the port is not bound to a receiver.
  base::SequencedTaskRunner* GetTaskRunner() const;

  // Returns true if its safe to post a message to this message port. That is,
  // a receiver has been set and the pipe is open and not in an error state.
  bool CanPostMessage() const;

  // Transmits a |message| over this port. If the port is in a state such that
  // "CanPostMessage" returns false then the message is dropped and this will
  // return false. Returns true if the message was actually accepted to be sent.
  // Note that this does not guarantee delivery, as the other end of the pipe
  // could be closed before the message is processed on the remote end.
  bool PostMessage(Message&& message);

  // Returns true if this port is bound to a valid message pipe.
  bool IsValid() const;

  // Returns true if this WebMessagePort has been closed.
  bool is_closed() const { return is_closed_; }

  // Returns true if this WebMessagePort has experienced an error.
  bool is_errored() const { return is_errored_; }

  // Returns true if this WebMessagePort is transferable as part of a
  // Message. This is true for a brand new WebMessagePort, but becomes false
  // if SetReceiver is ever called.
  bool is_transferable() const { return is_transferable_; }

  // Closes this message port. This also clears the receiver, if it is set.
  // After calling this "is_closed" will return true, "is_transferable" will
  // return false, and "is_errored" will retain the state it had before the pipe
  // was closed. This function can be called at any time, and repeatedly.
  void Close();

  // Reset this WebMessagePort to a completely default state. Similar to
  // close, but also resets the "is_closed", "is_errored" and "is_transferable"
  // states. Can be called at any time, and repeatedly.
  void Reset();

  // Passes out the underlying port descriptor. This port will be reset after
  // calling this (all of "IsValid", "is_closed" and "is_errored" will return
  // false). This can only be called if "is_transferable()" returns true.
  MessagePortDescriptor PassPort();

  // Maintains a static agent cluster ID that is used as the cluster ID in
  // casees where the embedder is the one calling PostMessage.
  static const base::UnguessableToken& GetEmbedderAgentClusterID();

 private:
  // Creates a message port that wraps the provided |port|. This provided |port|
  // must be valid. This is private as it should only be called by message
  // deserialization code, or the CreatePair factory.
  explicit WebMessagePort(MessagePortDescriptor&& port);

  void Take(WebMessagePort&& other);
  void OnPipeError();
  void CloseIfNecessary();

  // mojo::MessageReceiver implementation:
  bool Accept(mojo::Message* mojo_message) override;

  MessagePortDescriptor port_;
  std::unique_ptr<mojo::Connector> connector_;
  bool is_closed_ = true;
  bool is_errored_ = false;
  bool is_transferable_ = false;
  raw_ptr<MessageReceiver> receiver_ = nullptr;
};

// A very simple message format. This is a subset of a TransferableMessage, as
// many of the fields in the full message type aren't appropriate for messages
// originating from the embedder.
struct BLINK_COMMON_EXPORT WebMessagePort::Message {
  Message();
  Message(const Message&) = delete;
  Message(Message&&);
  Message& operator=(const Message&) = delete;
  Message& operator=(Message&&);
  ~Message();

  // Creates a message with the given |data|.
  explicit Message(const std::u16string& data);

  // Creates a message with the given collection of |ports| to be transferred.
  explicit Message(std::vector<WebMessagePort> ports);

  // Creates a message with a single |port| to be transferred.
  explicit Message(WebMessagePort&& port);

  // Creates a message with |data| and a collection of |ports| to be
  // transferred.
  Message(const std::u16string& data, std::vector<WebMessagePort> ports);

  // Creates a message with |data| and a single |port| to be transferred.
  Message(const std::u16string& data, WebMessagePort port);

  // A UTF-16 message.
  std::u16string data;

  // Other message ports that are to be transmitted as part of this message.
  std::vector<WebMessagePort> ports;
};

// Interface to be implemented by receivers.
class BLINK_COMMON_EXPORT WebMessagePort::MessageReceiver {
 public:
  MessageReceiver();
  MessageReceiver(const MessageReceiver&) = delete;
  MessageReceiver(MessageReceiver&&) = delete;
  MessageReceiver& operator=(const MessageReceiver&) = delete;
  MessageReceiver& operator=(MessageReceiver&&) = delete;
  virtual ~MessageReceiver();

  // Called for each incoming |message|. Returns false if the message could not
  // be successfully handled, in which case the pipe should be torn-down and
  // OnPipeError() invoked.
  virtual bool OnMessage(Message message);

  // Invoked when the underlying pipe has experienced an error.
  virtual void OnPipeError() {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_MESSAGING_WEB_MESSAGE_PORT_H_
