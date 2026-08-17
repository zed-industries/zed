// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_COMPILE_HINTS_CONSUMER_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_COMPILE_HINTS_CONSUMER_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_compile_hints_common.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/bloom_filter.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"

namespace blink::v8_compile_hints {

class CORE_EXPORT V8CrowdsourcedCompileHintsConsumer
    : public GarbageCollected<V8CrowdsourcedCompileHintsConsumer> {
 public:
  V8CrowdsourcedCompileHintsConsumer() = default;

  V8CrowdsourcedCompileHintsConsumer(
      const V8CrowdsourcedCompileHintsConsumer&) = delete;
  V8CrowdsourcedCompileHintsConsumer& operator=(
      const V8CrowdsourcedCompileHintsConsumer&) = delete;

  // Set the compile hints data based on raw memory containing int64_t:s.
  void SetData(base::span<const int64_t> memory);

  static bool CompileHintCallback(int position,
                                  void* raw_data_and_script_name_hash);

  // For the compile hints callback, we need an object which stays alive and
  // doesn't move while the background thread is holding a pointer to it.
  // `DataAndScriptNameHash` is such an object. The actual data (the Bloom
  // filter) is in a `Data` object shared by multiple `DataAndScriptNameHash`
  // objects.
  class Data : public WTF::ThreadSafeRefCounted<Data> {
   public:
    Data() = default;

    Data(const Data&) = delete;
    Data& operator=(const Data&) = delete;

   private:
    friend class V8CrowdsourcedCompileHintsConsumer;
    WTF::BloomFilter<kBloomFilterKeySize> bloom_;
  };

  class DataAndScriptNameHash {
   public:
    DataAndScriptNameHash(const scoped_refptr<Data>& data,
                          uint32_t script_name_hash)
        : data_(data), script_name_hash_(script_name_hash) {}

    DataAndScriptNameHash(const DataAndScriptNameHash&) = delete;
    DataAndScriptNameHash& operator=(const DataAndScriptNameHash&) = delete;

   private:
    friend class V8CrowdsourcedCompileHintsConsumer;
    scoped_refptr<Data> data_;
    uint32_t script_name_hash_;
  };

  std::unique_ptr<DataAndScriptNameHash> GetDataWithScriptNameHash(
      uint32_t script_name_hash);

  void Trace(Visitor*) const {}

  bool HasData() const { return data_.get() != nullptr; }

 private:
  scoped_refptr<Data> data_;
};

}  // namespace blink::v8_compile_hints

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_COMPILE_HINTS_CONSUMER_H_
