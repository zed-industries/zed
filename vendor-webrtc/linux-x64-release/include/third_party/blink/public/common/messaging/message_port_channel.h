// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_MESSAGING_MESSAGE_PORT_CHANNEL_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_MESSAGING_MESSAGE_PORT_CHANNEL_H_

#include <vector>

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/public/common/common_export.h"

namespace blink {

class MessagePortDescriptor;

// MessagePortChannel corresponds to a HTML MessagePort. It is a thin wrapper
// around a MessagePortDescriptor and used to provide methods for reading and
// writing messages. Currently all reading and writing is handled separately
// by other code, so MessagePortChannel is nothing other than a ref-counted
// holder of a MessagePortDescriptor, and is in the process of being removed.
class BLINK_COMMON_EXPORT MessagePortChannel {
 public:
  ~MessagePortChannel();
  MessagePortChannel();

  // Shallow copy, resulting in multiple references to the same port.
  MessagePortChannel(const MessagePortChannel& other);
  MessagePortChannel& operator=(const MessagePortChannel& other);

  explicit MessagePortChannel(MessagePortDescriptor handle);

  const MessagePortDescriptor& GetHandle() const;
  MessagePortDescriptor ReleaseHandle() const;

  static std::vector<MessagePortDescriptor> ReleaseHandles(
      const std::vector<MessagePortChannel>& ports);
  static std::vector<MessagePortChannel> CreateFromHandles(
      std::vector<MessagePortDescriptor> handles);

 private:
  class State;
  mutable scoped_refptr<State> state_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_MESSAGING_MESSAGE_PORT_CHANNEL_H_
