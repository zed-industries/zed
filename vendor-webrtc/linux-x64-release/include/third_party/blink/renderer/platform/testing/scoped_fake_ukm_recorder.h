// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_SCOPED_FAKE_UKM_RECORDER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_SCOPED_FAKE_UKM_RECORDER_H_

#include "components/ukm/test_ukm_recorder.h"
#include "mojo/public/cpp/bindings/receiver.h"

namespace blink {

// Class used to mock the UKM recorder living in the browser process and being
// served UKM events over Mojo. When instantiated, this class will register
// itself as the Mojo interface for the UkmRecorderInterface in the
// BrowserInterfaceBroker. It will then forward UKM logging events to a
// TestUkmRecorder instance it owns and make it available for validation.
// Consumers of this class should make sure to instantiate it before any other
// instance takes a dependency on that mojo interface.
class ScopedFakeUkmRecorder : public ukm::mojom::UkmRecorderInterface,
                              public ukm::mojom::UkmRecorderFactory {
 public:
  explicit ScopedFakeUkmRecorder();
  ~ScopedFakeUkmRecorder() override;

  // ukm::mojom::UkmRecorderInterface:
  void AddEntry(ukm::mojom::UkmEntryPtr entry) override;
  void UpdateSourceURL(int64_t source_id, const std::string& url) override;

  // ukm::mojom::UkmRecorderFactory:
  void CreateUkmRecorder(
      mojo::PendingReceiver<ukm::mojom::UkmRecorderInterface> receiver,
      mojo::PendingRemote<ukm::mojom::UkmRecorderClientInterface> client_remote)
      override;

  void ResetRecorder();
  void SetHandle(mojo::ScopedMessagePipeHandle handle);

  ukm::TestUkmRecorder* recorder() { return recorder_.get(); }

 private:
  std::unique_ptr<mojo::Receiver<ukm::mojom::UkmRecorderFactory>> receiver_;
  std::unique_ptr<mojo::Receiver<ukm::mojom::UkmRecorderInterface>>
      interface_receiver_;
  std::unique_ptr<ukm::TestUkmRecorder> recorder_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_SCOPED_FAKE_UKM_RECORDER_H_
