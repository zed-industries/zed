// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIASTREAM_AEC_DUMP_AGENT_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIASTREAM_AEC_DUMP_AGENT_IMPL_H_

#include "base/memory/raw_ptr.h"
#include "mojo/public/cpp/bindings/receiver.h"
#include "third_party/blink/public/mojom/mediastream/aec_dump.mojom-blink.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

// An instance of this class connects to the browser process to register for
// notifications to start / stop writing to a dump file.
class PLATFORM_EXPORT AecDumpAgentImpl : public mojom::blink::AecDumpAgent {
 public:
  class Delegate {
   public:
    virtual void OnStartDump(base::File file) = 0;
    virtual void OnStopDump() = 0;
  };

  // This may fail in unit tests, in which case a null object is returned.
  static std::unique_ptr<AecDumpAgentImpl> Create(Delegate* delegate);

  AecDumpAgentImpl(const AecDumpAgentImpl&) = delete;
  AecDumpAgentImpl& operator=(const AecDumpAgentImpl&) = delete;
  ~AecDumpAgentImpl() override;

  // AecDumpAgent methods:
  void Start(base::File dump_file) override;
  void Stop() override;

 private:
  explicit AecDumpAgentImpl(
      Delegate* delegate,
      mojo::PendingReceiver<mojom::blink::AecDumpAgent> receiver);

  raw_ptr<Delegate> delegate_;
  mojo::Receiver<mojom::blink::AecDumpAgent> receiver_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIASTREAM_AEC_DUMP_AGENT_IMPL_H_
