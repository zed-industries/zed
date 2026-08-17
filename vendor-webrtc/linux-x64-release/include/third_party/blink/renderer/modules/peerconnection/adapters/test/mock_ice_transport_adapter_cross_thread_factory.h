// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_ADAPTERS_TEST_MOCK_ICE_TRANSPORT_ADAPTER_CROSS_THREAD_FACTORY_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_ADAPTERS_TEST_MOCK_ICE_TRANSPORT_ADAPTER_CROSS_THREAD_FACTORY_H_

#include "base/check_op.h"
#include "testing/gmock/include/gmock/gmock.h"
#include "third_party/blink/renderer/modules/peerconnection/adapters/ice_transport_adapter_cross_thread_factory.h"

namespace blink {

class MockIceTransportAdapterCrossThreadFactory
    : public IceTransportAdapterCrossThreadFactory {
 public:
  MockIceTransportAdapterCrossThreadFactory(
      std::unique_ptr<MockIceTransportAdapter> mock_adapter,
      IceTransportAdapter::Delegate** delegate_out)
      : mock_adapter_(std::move(mock_adapter)), delegate_out_(delegate_out) {
    if (delegate_out) {
      // Ensure the caller has not left the delegate_out value floating.
      DCHECK_EQ(nullptr, *delegate_out);
    }
  }

  // IceTransportAdapterCrossThreadFactory overrides.
  void InitializeOnMainThread(LocalFrame&) override {}
  std::unique_ptr<IceTransportAdapter> ConstructOnWorkerThread(
      IceTransportAdapter::Delegate* delegate) override {
    DCHECK(mock_adapter_);
    if (delegate_out_) {
      *delegate_out_ = delegate;
    }
    return std::move(mock_adapter_);
  }

 private:
  std::unique_ptr<MockIceTransportAdapter> mock_adapter_;
  IceTransportAdapter::Delegate** delegate_out_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_ADAPTERS_TEST_MOCK_ICE_TRANSPORT_ADAPTER_CROSS_THREAD_FACTORY_H_
