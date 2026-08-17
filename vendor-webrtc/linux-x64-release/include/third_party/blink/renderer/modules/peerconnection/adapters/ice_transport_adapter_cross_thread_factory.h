// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_ADAPTERS_ICE_TRANSPORT_ADAPTER_CROSS_THREAD_FACTORY_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_ADAPTERS_ICE_TRANSPORT_ADAPTER_CROSS_THREAD_FACTORY_H_

#include "third_party/blink/renderer/modules/peerconnection/adapters/ice_transport_adapter.h"

namespace blink {

class LocalFrame;

// This class creates a single concrete instance of an IceTransportAdapter with
// a hook to allow creating dependencies on the main thread (the
// IceTransportAdapter is created on the worker thread).
//
// Callers must call InitializeOnMainThread() before ConstructOnWorkerThread().
class IceTransportAdapterCrossThreadFactory {
 public:
  virtual ~IceTransportAdapterCrossThreadFactory() = default;

  // Construct any dependencies on the main thread. Can only be called once.
  virtual void InitializeOnMainThread(LocalFrame&) = 0;

  // Construct the IceTransportAdapter instance with the given delegate. Can
  // only be called once.
  virtual std::unique_ptr<IceTransportAdapter> ConstructOnWorkerThread(
      IceTransportAdapter::Delegate* delegate) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_ADAPTERS_ICE_TRANSPORT_ADAPTER_CROSS_THREAD_FACTORY_H_
