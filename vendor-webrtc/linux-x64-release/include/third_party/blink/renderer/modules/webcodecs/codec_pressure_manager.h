// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_CODEC_PRESSURE_MANAGER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_CODEC_PRESSURE_MANAGER_H_

#include "base/sequence_checker.h"
#include "base/task/sequenced_task_runner.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/webcodecs/codec_pressure_gauge.h"
#include "third_party/blink/renderer/modules/webcodecs/reclaimable_codec.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {
class CodecPressureGauge;

// Keeps track of the number of pressuring codecs in an ExecutionContext, and
// notifies these codecs when a pressure threshold has been crossed.
class MODULES_EXPORT CodecPressureManager
    : public GarbageCollected<CodecPressureManager> {
  USING_PRE_FINALIZER(CodecPressureManager, UnregisterManager);

 public:
  CodecPressureManager(ReclaimableCodec::CodecType,
                       scoped_refptr<base::SequencedTaskRunner> task_runner);

  // Disable copy and assign.
  CodecPressureManager(const CodecPressureManager&) = delete;
  CodecPressureManager& operator=(const CodecPressureManager&) = delete;

  // Adds or removes a codec, potentially causing global pressure flags to be
  // set/unset.
  void AddCodec(ReclaimableCodec*);
  void RemoveCodec(ReclaimableCodec*);

  // Called when a codec is disposed without having called RemoveCodec() first.
  // This can happen when a codec is garbage collected without having been
  // closed or reclaimed first.
  void OnCodecDisposed(ReclaimableCodec*);

  size_t pressure_for_testing() { return local_codec_pressure_; }

  size_t is_global_pressure_exceeded_for_testing() {
    return global_pressure_exceeded_;
  }

  void Trace(Visitor*) const;

 private:
  friend class CodecPressureManagerTest;

  using CodecSet = HeapHashSet<WeakMember<ReclaimableCodec>>;

  // Returns the right CodecPressureGauge based off of |codec_type_|.
  CodecPressureGauge& GetCodecPressureGauge();

  // Pre-finalizer.
  void UnregisterManager();

  void OnGlobalPressureThresholdChanged(bool pressure_threshold_exceeded);

  bool manager_registered_ = true;
  bool global_pressure_exceeded_ = false;

  // Track |local_codec_pressure_| manually instead of using
  // |codecs_with_pressure_.size()|, since the GC can silently remove elements
  // from |codecs_with_pressure_|.
  size_t local_codec_pressure_ = 0u;
  CodecSet codecs_with_pressure_;

  CodecPressureGauge::PressureCallbackId pressure_callback_id_;

  ReclaimableCodec::CodecType codec_type_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_CODEC_PRESSURE_MANAGER_H_
