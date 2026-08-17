// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// no-include-guard-because-multiply-included

// Declares the BarMessageListener class for a given interface Bar. In ipcz
// message parlance an interface is a collection of related messages. This class
// routes a generic message dispatches to generated virtual methods named for
// the messages they receive. e.g. a DoStuff message is routed (based on message
// ID) to OnDoStuff().
//
// ipcz objects may override this interface to receive messages from some
// transport endpoint which they control. For example a NodeLink implements
// the Node interface (see node_messages.h and node_message_generator.h) by
// subclassing generated NodeMessageListener class and implementing all its
// methods.
//
// Note that listeners separate message receipt (OnMessage) from message
// dispatch (DispatchMessage). By default, OnMessage() simply forwards to
// DispatchMessage(), but the split allows subclasses to override this behavior,
// for example to defer dispatch in some cases.
//
// All raw transport messages are fully validated and deserialized before
// hitting OnMessage(), so implementations do not need to do any protocol-level
// validation of their own.

#define IPCZ_MSG_BEGIN_INTERFACE(name)                             \
  class name##MessageListener : public DriverTransport::Listener { \
   public:                                                         \
    virtual ~name##MessageListener() = default;                    \
    virtual bool OnMessage(Message& message);                      \
                                                                   \
   protected:                                                      \
    virtual bool DispatchMessage(Message& message);

#define IPCZ_MSG_END_INTERFACE()                                      \
 private:                                                             \
  bool OnTransportMessage(const DriverTransport::RawMessage& message, \
                          const DriverTransport& transport,           \
                          IpczDriverHandle envelope) final;           \
  void OnTransportError() override {}                                 \
  }                                                                   \
  ;

#define IPCZ_MSG_ID(x)

#define IPCZ_MSG_BEGIN(name, id_decl) \
  virtual bool On##name(name&) {      \
    return false;                     \
  }

#define IPCZ_MSG_END()
#define IPCZ_MSG_BEGIN_VERSION(version)
#define IPCZ_MSG_END_VERSION(version)
#define IPCZ_MSG_PARAM(type, name)
#define IPCZ_MSG_PARAM_ARRAY(type, name)
#define IPCZ_MSG_PARAM_DRIVER_OBJECT(name)
#define IPCZ_MSG_PARAM_DRIVER_OBJECT_ARRAY(name)
