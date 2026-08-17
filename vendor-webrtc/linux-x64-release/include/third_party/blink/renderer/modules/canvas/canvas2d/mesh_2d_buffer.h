// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_MESH_2D_BUFFER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_MESH_2D_BUFFER_H_

#include <stdint.h>

#include <utility>

#include "base/memory/scoped_refptr.h"
#include "base/numerics/safe_conversions.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/bindings/v8_external_memory_accounter.h"
#include "v8/include/v8-isolate.h"

namespace cc {
template <typename T>
class RefCountedBuffer;
}  // namespace cc

namespace blink {

// JS wrapper for a retained mesh buffer.
//
// This is the base for vertex/uv/index JS buffers (SkPoint/SkPoint/uint16_t
// specializations, respectively).
//
// The actual data payload is stored in a RefcountedBuffer, which enables
// sharing with the rest of paint pipeline, and avoids deep copies during
// paint op recording.
template <typename T>
class Mesh2DBuffer : public ScriptWrappable {
 public:
  Mesh2DBuffer(const Mesh2DBuffer&) = delete;
  Mesh2DBuffer& operator=(const Mesh2DBuffer&) = delete;

  ~Mesh2DBuffer() override {
    external_memory_accounter_.Decrease(
        v8::Isolate::GetCurrent(),
        base::checked_cast<int64_t>(buffer_->data().size() * sizeof(T)));
  }

  scoped_refptr<cc::RefCountedBuffer<T>> GetBuffer() const { return buffer_; }

 protected:
  explicit Mesh2DBuffer(scoped_refptr<cc::RefCountedBuffer<T>> buffer)
      : buffer_(std::move(buffer)) {
    external_memory_accounter_.Increase(
        v8::Isolate::GetCurrent(),
        base::checked_cast<int64_t>(buffer_->data().size() * sizeof(T)));
  }

 private:
  scoped_refptr<cc::RefCountedBuffer<T>> buffer_;

  NO_UNIQUE_ADDRESS V8ExternalMemoryAccounterBase external_memory_accounter_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_MESH_2D_BUFFER_H_
