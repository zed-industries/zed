// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_COMPUTE_PRESSURE_PRESSURE_OBSERVER_TEST_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_COMPUTE_PRESSURE_PRESSURE_OBSERVER_TEST_UTILS_H_

#include "mojo/public/cpp/bindings/associated_remote.h"
#include "mojo/public/cpp/bindings/receiver.h"
#include "services/device/public/mojom/pressure_manager.mojom-blink.h"
#include "services/device/public/mojom/pressure_update.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/compute_pressure/web_pressure_manager.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_binding_for_testing.h"

namespace blink {

class ExceptionState;
class LocalDOMWindow;
class ScriptState;

class FakePressureService final : public mojom::blink::WebPressureManager {
 public:
  FakePressureService();
  ~FakePressureService() override;

  void BindRequest(mojo::ScopedMessagePipeHandle handle);

  void SendUpdate(device::mojom::blink::PressureUpdatePtr update);

  // mojom::blink::WebPressureManager implementation.
  void AddClient(
      device::mojom::blink::PressureSource source,
      mojo::PendingAssociatedRemote<device::mojom::blink::PressureClient>
          client,
      AddClientCallback callback) override;

  void Reset() { manager_receiver_.reset(); }

 private:
  void OnConnectionError();

  mojo::AssociatedRemote<device::mojom::blink::PressureClient> client_remote_;
  mojo::Receiver<mojom::blink::WebPressureManager> manager_receiver_{this};
};

class ComputePressureTestingContext final {
  STACK_ALLOCATED();

 public:
  explicit ComputePressureTestingContext(
      FakePressureService* mock_pressure_service);

  ~ComputePressureTestingContext();

  ScriptState* GetScriptState();
  ExceptionState& GetExceptionState();

 private:
  LocalDOMWindow* DomWindow();
  V8TestingScope testing_scope_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_COMPUTE_PRESSURE_PRESSURE_OBSERVER_TEST_UTILS_H_
