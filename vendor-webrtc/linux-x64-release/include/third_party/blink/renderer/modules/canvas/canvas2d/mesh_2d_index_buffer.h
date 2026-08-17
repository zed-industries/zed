// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_MESH_2D_INDEX_BUFFER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_MESH_2D_INDEX_BUFFER_H_

#include <stdint.h>

#include <utility>

#include "base/memory/scoped_refptr.h"
#include "cc/paint/refcounted_buffer.h"
#include "third_party/blink/renderer/modules/canvas/canvas2d/mesh_2d_buffer.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class MODULES_EXPORT Mesh2DIndexBuffer final : public Mesh2DBuffer<uint16_t> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit Mesh2DIndexBuffer(scoped_refptr<cc::RefCountedBuffer<uint16_t>> buf)
      : Mesh2DBuffer(std::move(buf)) {}
  ~Mesh2DIndexBuffer() override = default;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_MESH_2D_INDEX_BUFFER_H_
