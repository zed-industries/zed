// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_GEOMETRY_AS_JSON_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_GEOMETRY_AS_JSON_H_

#include <memory>

#include "third_party/blink/renderer/platform/json/json_values.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "ui/gfx/geometry/rect_f.h"

namespace blink {

template <typename T>
static std::unique_ptr<JSONArray> RectAsJSONArray(const T& rect) {
  gfx::RectF rect_f(rect);
  auto array = std::make_unique<JSONArray>();
  array->PushDouble(rect.x());
  array->PushDouble(rect.y());
  array->PushDouble(rect.width());
  array->PushDouble(rect.height());
  return array;
}

template <typename T>
std::unique_ptr<JSONArray> PointAsJSONArray(const T& point) {
  gfx::PointF point_f(point);
  auto array = std::make_unique<JSONArray>();
  array->PushDouble(point_f.x());
  array->PushDouble(point_f.y());
  return array;
}

template <typename T>
std::unique_ptr<JSONArray> VectorAsJSONArray(const T& vector) {
  gfx::Vector2dF vector_f(vector);
  auto array = std::make_unique<JSONArray>();
  array->PushDouble(vector_f.x());
  array->PushDouble(vector_f.y());
  return array;
}

inline std::unique_ptr<JSONArray> Point3AsJSONArray(const gfx::Point3F& point) {
  auto array = std::make_unique<JSONArray>();
  array->PushDouble(point.x());
  array->PushDouble(point.y());
  if (point.z())
    array->PushDouble(point.z());
  return array;
}

template <typename T>
std::unique_ptr<JSONArray> SizeAsJSONArray(const T& size) {
  gfx::SizeF size_f(size);
  auto array = std::make_unique<JSONArray>();
  array->PushDouble(size_f.width());
  array->PushDouble(size_f.height());
  return array;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_GEOMETRY_AS_JSON_H_
