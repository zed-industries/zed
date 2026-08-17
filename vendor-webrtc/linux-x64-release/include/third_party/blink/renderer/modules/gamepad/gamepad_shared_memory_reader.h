// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_GAMEPAD_GAMEPAD_SHARED_MEMORY_READER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_GAMEPAD_GAMEPAD_SHARED_MEMORY_READER_H_

#include "base/memory/raw_ptr.h"
#include "device/gamepad/public/mojom/gamepad.mojom-blink.h"
#include "device/gamepad/public/mojom/gamepad_hardware_buffer.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"

namespace base {
class ReadOnlySharedMemoryRegion;
}

namespace device {
template <class T>
class GamepadImpl;
using Gamepad = GamepadImpl<void>;
class Gamepads;
}  // namespace device

namespace blink {

class GamepadListener;
class LocalDOMWindow;

class GamepadSharedMemoryReader
    : public GarbageCollected<GamepadSharedMemoryReader>,
      public device::mojom::blink::GamepadObserver {
 public:
  explicit GamepadSharedMemoryReader(LocalDOMWindow&);
  ~GamepadSharedMemoryReader() override;
  void Trace(Visitor*) const;

  void SampleGamepads(device::Gamepads* gamepads);
  void Start(blink::GamepadListener* listener);
  void Stop();

  GamepadSharedMemoryReader(const GamepadSharedMemoryReader&) = delete;
  GamepadSharedMemoryReader& operator=(const GamepadSharedMemoryReader&) =
      delete;

 protected:
  void SendStartMessage();
  void SendStopMessage();

 private:
  // device::mojom::blink::GamepadObserver methods.
  void GamepadConnected(uint32_t index,
                        const device::Gamepad& gamepad) override;
  void GamepadDisconnected(uint32_t index,
                           const device::Gamepad& gamepad) override;

  base::ReadOnlySharedMemoryRegion renderer_shared_buffer_region_;
  base::ReadOnlySharedMemoryMapping renderer_shared_buffer_mapping_;
  raw_ptr<const device::GamepadHardwareBuffer> gamepad_hardware_buffer_ =
      nullptr;

  bool ever_interacted_with_ = false;

  HeapMojoReceiver<device::mojom::blink::GamepadObserver,
                   GamepadSharedMemoryReader>
      receiver_;
  HeapMojoRemote<device::mojom::blink::GamepadMonitor> gamepad_monitor_remote_;
  raw_ptr<blink::GamepadListener> listener_ = nullptr;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_GAMEPAD_GAMEPAD_SHARED_MEMORY_READER_H_
