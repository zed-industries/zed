// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_MESH_2D_UV_BUFFER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_MESH_2D_UV_BUFFER_H_

#include <utility>

#include "base/memory/scoped_refptr.h"
#include "cc/paint/refcounted_buffer.h"
#include "third_party/blink/renderer/modules/canvas/canvas2d/mesh_2d_buffer.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/skia/include/core/SkPoint.h"

namespace blink {

class MODULES_EXPORT Mesh2DUVBuffer final : public Mesh2DBuffer<SkPoint> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit Mesh2DUVBuffer(scoped_refptr<cc::RefCountedBuffer<SkPoint>> buf)
      : Mesh2DBuffer(std::move(buf)) {}
  ~Mesh2DUVBuffer() override = default;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_MESH_2D_UV_BUFFER_H_
