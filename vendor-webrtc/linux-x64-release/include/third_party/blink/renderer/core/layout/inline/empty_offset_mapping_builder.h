// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_EMPTY_OFFSET_MAPPING_BUILDER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_EMPTY_OFFSET_MAPPING_BUILDER_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class LayoutText;

// A mock class providing all APIs of an offset mapping builder, but not doing
// anything. For templates functions/classes that can optionally create an
// offset mapping, this mock class is passed to create an instantiation that
// does not create any offset mapping.
class EmptyOffsetMappingBuilder {
  STACK_ALLOCATED();

 public:
  class SourceNodeScope {
   public:
    SourceNodeScope(EmptyOffsetMappingBuilder*, const void*) {}
    ~SourceNodeScope() = default;
  };

  EmptyOffsetMappingBuilder() = default;
  EmptyOffsetMappingBuilder(const EmptyOffsetMappingBuilder&) = delete;
  EmptyOffsetMappingBuilder& operator=(const EmptyOffsetMappingBuilder&) =
      delete;
  void AppendIdentityMapping(unsigned) {}
  void RevertIdentityMapping1() {}
  void AppendCollapsedMapping(unsigned) {}
  void AppendVariableMapping(unsigned, unsigned) {}
  void CollapseTrailingSpace(unsigned) {}
  void Composite(const EmptyOffsetMappingBuilder&) {}
  void Concatenate(const EmptyOffsetMappingBuilder&) {}
  void RestoreTrailingCollapsibleSpace(const LayoutText&, unsigned) {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_EMPTY_OFFSET_MAPPING_BUILDER_H_
