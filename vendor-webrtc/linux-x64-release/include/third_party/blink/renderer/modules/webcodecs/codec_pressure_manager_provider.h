// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_CODEC_PRESSURE_MANAGER_PROVIDER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_CODEC_PRESSURE_MANAGER_PROVIDER_H_

#include "base/task/sequenced_task_runner.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class CodecPressureManager;

// Simple supplement to lazily create a single CodecPressureManager for encoders
// or decoders, per ExecutionContext.
class MODULES_EXPORT CodecPressureManagerProvider
    : public GarbageCollected<CodecPressureManagerProvider>,
      public Supplement<ExecutionContext> {
 public:
  static const char kSupplementName[];

  // Gets or creates the CodecPressureManagerProvider.
  static CodecPressureManagerProvider& From(ExecutionContext&);
  explicit CodecPressureManagerProvider(ExecutionContext&);

  // Disable copy and assign.
  CodecPressureManagerProvider& operator=(const CodecPressureManagerProvider&) =
      delete;
  CodecPressureManagerProvider(const CodecPressureManagerProvider&) = delete;

  void Trace(Visitor*) const override;

  CodecPressureManager* GetDecoderPressureManager();
  CodecPressureManager* GetEncoderPressureManager();

 private:
  scoped_refptr<base::SequencedTaskRunner> GetTaskRunner();

  Member<CodecPressureManager> decoder_pressure_manager_;
  Member<CodecPressureManager> encoder_pressure_manager_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_CODEC_PRESSURE_MANAGER_PROVIDER_H_
