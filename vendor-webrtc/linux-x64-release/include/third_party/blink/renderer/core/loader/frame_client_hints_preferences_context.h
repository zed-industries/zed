// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_FRAME_CLIENT_HINTS_PREFERENCES_CONTEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_FRAME_CLIENT_HINTS_PREFERENCES_CONTEXT_H_

#include "services/network/public/mojom/web_client_hints_types.mojom-shared.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/loader/fetch/client_hints_preferences.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace ukm {
class UkmRecorder;
}  // namespace ukm

namespace blink {

class LocalFrame;

class FrameClientHintsPreferencesContext final
    : public ClientHintsPreferences::Context {
  STACK_ALLOCATED();

 public:
  explicit FrameClientHintsPreferencesContext(LocalFrame*);

  ukm::SourceId GetUkmSourceId() override;
  ukm::UkmRecorder* GetUkmRecorder() override;
  void CountClientHints(network::mojom::WebClientHintsType) override;

 private:
  LocalFrame* frame_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_FRAME_CLIENT_HINTS_PREFERENCES_CONTEXT_H_
