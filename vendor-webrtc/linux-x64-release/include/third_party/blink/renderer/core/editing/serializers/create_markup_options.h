// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SERIALIZERS_CREATE_MARKUP_OPTIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SERIALIZERS_CREATE_MARKUP_OPTIONS_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class Node;

enum AbsoluteURLs { kDoNotResolveURLs, kResolveAllURLs, kResolveNonLocalURLs };

class CORE_EXPORT CreateMarkupOptions final {
  STACK_ALLOCATED();

 public:
  class CORE_EXPORT Builder;

  CreateMarkupOptions() = default;

  const Node* ConstrainingAncestor() const { return constraining_ancestor_; }
  AbsoluteURLs ShouldResolveURLs() const { return should_resolve_urls_; }
  bool ShouldAnnotateForInterchange() const {
    return should_annotate_for_interchange_;
  }
  bool ShouldConvertBlocksToInlines() const {
    return should_convert_blocks_to_inlines_;
  }
  bool IsForMarkupSanitization() const { return is_for_markup_sanitization_; }
  bool IgnoresCSSTextTransformsForRenderedText() const {
    return ignores_css_text_transforms_for_rendered_text;
  }

 private:
  const Node* constraining_ancestor_ = nullptr;
  AbsoluteURLs should_resolve_urls_ = kDoNotResolveURLs;
  bool should_annotate_for_interchange_ = false;
  bool should_convert_blocks_to_inlines_ = false;
  bool is_for_markup_sanitization_ = false;
  bool ignores_css_text_transforms_for_rendered_text = false;
};

class CORE_EXPORT CreateMarkupOptions::Builder final {
  STACK_ALLOCATED();

 public:
  Builder() = default;
  explicit Builder(const CreateMarkupOptions& options) : data_(options) {}

  CreateMarkupOptions Build() const { return data_; }

  Builder& SetConstrainingAncestor(const Node* node);
  Builder& SetShouldResolveURLs(AbsoluteURLs absolute_urls);
  Builder& SetShouldAnnotateForInterchange(bool annotate_for_interchange);
  Builder& SetShouldConvertBlocksToInlines(bool convert_blocks_for_inlines);
  Builder& SetIsForMarkupSanitization(bool is_for_sanitization);
  Builder& SetIgnoresCSSTextTransformsForRenderedText(
      bool text_without_transforms);

 private:
  CreateMarkupOptions data_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SERIALIZERS_CREATE_MARKUP_OPTIONS_H_
