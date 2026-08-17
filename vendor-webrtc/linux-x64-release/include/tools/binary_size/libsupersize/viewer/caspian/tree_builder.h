// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_BINARY_SIZE_LIBSUPERSIZE_VIEWER_CASPIAN_TREE_BUILDER_H_
#define TOOLS_BINARY_SIZE_LIBSUPERSIZE_VIEWER_CASPIAN_TREE_BUILDER_H_

#include <deque>
#include <functional>
#include <memory>
#include <string>
#include <string_view>
#include <unordered_map>
#include <vector>

#include "tools/binary_size/libsupersize/viewer/caspian/lens.h"
#include "tools/binary_size/libsupersize/viewer/caspian/model.h"

namespace caspian {
class TreeBuilder {
 public:
  using FilterFunc = std::function<bool(const GroupedPath&, const BaseSymbol&)>;

  explicit TreeBuilder(SizeInfo* size_info);
  explicit TreeBuilder(DeltaSizeInfo* size_info);
  ~TreeBuilder();
  void Build(std::unique_ptr<BaseLens> lens,
             char separator,
             bool method_count_mode,
             std::vector<FilterFunc> filters);
  TreeNode* Find(std::string_view path);
  Json::Value Open(const char* path);
  Json::Value GetAncestryById(uint32_t id);

 private:
  void AddFileEntry(GroupedPath source_path,
                    const std::vector<const BaseSymbol*>& symbols);

  TreeNode* GetOrMakeParentNode(TreeNode* child_node);

  void AttachToParent(TreeNode* child, TreeNode* parent);

  TreeNode* FindNodeById(int32_t id);

  ArtifactType ArtifactTypeFromChild(GroupedPath child_path) const;

  bool ShouldIncludeSymbol(const GroupedPath& id_path,
                           const BaseSymbol& symbol) const;

  // Merges dex method symbols into containers based on the class of the dex
  // method.
  void JoinDexMethodClasses(TreeNode* node);

  BaseSizeInfo* size_info_;
  bool diff_mode_;
  TreeNodeFactory tree_node_factory_;
  std::unique_ptr<TreeNode> root_;
  std::unordered_map<GroupedPath, TreeNode*> _parents;

  // Contained TreeNode hold lightweight string_views to fields in SizeInfo.
  // If grouping by component, this isn't possible: TreeNode id_paths are not
  // substrings of SizeInfo-owned strings. In that case, the strings are stored
  // in |owned_strings_|.
  // Deque is used for stability, to support string_view.
  std::deque<std::string> owned_strings_;
  std::unique_ptr<BaseLens> lens_;
  bool method_count_mode_;
  // The current path separator: '>' if grouping by component, '/' otherwise.
  // Note that we split paths on '/' no matter the value of separator, since
  // when grouping by component, paths look like Component>path/to/file.
  char sep_;
  std::vector<FilterFunc> filters_;
  std::vector<const BaseSymbol*> symbols_;
  std::unordered_map<std::string_view, DiffStatus> source_to_diff_status_;
};  // TreeBuilder
}  // namespace caspian
#endif  // TOOLS_BINARY_SIZE_LIBSUPERSIZE_VIEWER_CASPIAN_TREE_BUILDER_H_
