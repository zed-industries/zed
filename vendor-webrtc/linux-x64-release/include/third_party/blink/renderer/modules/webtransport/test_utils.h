// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Utilities for testing classes in this directory. They assume they are running
// inside a gtest test.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBTRANSPORT_TEST_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBTRANSPORT_TEST_UTILS_H_

#include "base/functional/callback.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/weak_ptr.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "mojo/public/cpp/bindings/receiver.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "mojo/public/cpp/system/data_pipe.h"
#include "mojo/public/cpp/system/message_pipe.h"
#include "services/network/public/mojom/web_transport.mojom-blink.h"
#include "third_party/blink/public/mojom/webtransport/web_transport_connector.mojom-blink.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "v8/include/v8.h"

namespace blink {

class BrowserInterfaceBrokerProxy;
class KURL;
class ReadableStream;
class ScriptState;
class V8TestingScope;
class WebTransport;

// Creates a mojo data pipe with the normal options for WebTransportTests.
// Returns true for success. Returns false and causes a test failure
// otherwise.
bool CreateDataPipeForWebTransportTests(
    mojo::ScopedDataPipeProducerHandle* producer,
    mojo::ScopedDataPipeConsumerHandle* consumer);

// Read the next value from a stream. It is expected that there is a value,
// ie. that |done| will be false. Fails the test if no value is available.
v8::Local<v8::Value> ReadValueFromStream(const V8TestingScope& scope,
                                         ReadableStream* stream);

// Helps create a WebTransport with a fake remote for use in tests. See for
// example ScopedWebTransport in bidirectional_stream_test.cc for how to use it.
class TestWebTransportCreator final
    : public mojom::blink::WebTransportConnector {
  DISALLOW_NEW();

 public:
  using CreateStubCallback = base::RepeatingCallback<void(
      mojo::PendingRemote<network::mojom::blink::WebTransport>&)>;

  TestWebTransportCreator();
  ~TestWebTransportCreator() override;

  // The |create_stub| callback should initialize a subclass of
  // network::mojom::blink::WebTransport and keep it alive for as long as the
  // WebTransport object should remain functional.
  void Init(ScriptState* script_state, CreateStubCallback create_stub);

  WebTransport* GetWebTransport() const { return web_transport_; }

  // Implementation of mojom::blink::WebTransportConnector.
  void Connect(
      const KURL&,
      Vector<network::mojom::blink::WebTransportCertificateFingerprintPtr>,
      mojo::PendingRemote<network::mojom::blink::WebTransportHandshakeClient>)
      override;

  void Reset();

 private:
  void BindConnector(mojo::ScopedMessagePipeHandle handle);

  // |browser_interface_broker_| is cached here because we need to use it in the
  // destructor. This means ScopedWebTransporHelper must always be destroyed
  // before the ExecutionContext that owns the BrowserInterfaceBrokerProxy.
  raw_ptr<const BrowserInterfaceBrokerProxy> browser_interface_broker_ =
      nullptr;
  Persistent<WebTransport> web_transport_;
  mojo::Remote<network::mojom::blink::WebTransportClient> client_remote_;
  mojo::Receiver<mojom::blink::WebTransportConnector> connector_receiver_{this};
  CreateStubCallback create_stub_;

  base::WeakPtrFactory<TestWebTransportCreator> weak_ptr_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBTRANSPORT_TEST_UTILS_H_
