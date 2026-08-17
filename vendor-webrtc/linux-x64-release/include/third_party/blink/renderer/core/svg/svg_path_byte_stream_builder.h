/*
 * Copyright (C) Research In Motion Limited 2010. All rights reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_PATH_BYTE_STREAM_BUILDER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_PATH_BYTE_STREAM_BUILDER_H_

#include "base/memory/stack_allocated.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/svg/svg_path_consumer.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class SVGPathByteStream;

class CORE_EXPORT SVGPathByteStreamBuilder final : public SVGPathConsumer {
  STACK_ALLOCATED();

 public:
  SVGPathByteStreamBuilder();

  void ReserveInitialCapacity(unsigned capacity) {
    result_.ReserveInitialCapacity(capacity);
  }

  // Returns a copy of the byte stream.
  SVGPathByteStream CopyByteStream();

  void EmitSegment(const PathSegmentData&) override;

 private:
  class CoalescingBuffer;

  // To minimize allocations a Vector with on-stack allocation is used for
  // the stream. CopyByteStream() returns a Vector with a capacity just big
  // enough for the data.
  using SVGPathByteStreamBuilderStorage = Vector<unsigned char, 1024>;

  SVGPathByteStreamBuilderStorage result_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_PATH_BYTE_STREAM_BUILDER_H_
