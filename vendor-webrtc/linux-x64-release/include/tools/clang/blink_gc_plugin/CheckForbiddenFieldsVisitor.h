// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_BLINK_GC_PLUGIN_CHECKFORBIDDENFIELDSVISITOR_H_
#define TOOLS_CLANG_BLINK_GC_PLUGIN_CHECKFORBIDDENFIELDSVISITOR_H_

#include <set>
#include <vector>

#include "Edge.h"
#include "RecordInfo.h"

// This visitor checks that the fields of a class and the fields of
// its embedded objects don't define GC roots.
class CheckForbiddenFieldsVisitor : public RecursiveEdgeVisitor {
 public:
  enum class Error {
    kTaskRunnerInGCManaged,
    kMojoRemoteInGCManaged,
    kMojoReceiverInGCManaged,
    kMojoAssociatedRemoteInGCManaged,
    kMojoAssociatedReceiverInGCManaged,
  };

  using RootPath = std::vector<FieldPoint*>;
  using VisitingSet = std::set<RecordInfo*>;
  using Errors = std::vector<std::pair<RootPath, Error>>;

  explicit CheckForbiddenFieldsVisitor();

  // The forbidden fields found across the call(s) to
  // `ContainsForbiddenFields`.
  Errors& forbidden_fields();

  // Checks whether the record recursively contains forbidden fields (either the
  // record itself or an embedded object).
  // Returns whether forbidden fields were found.
  bool ContainsForbiddenFields(RecordInfo* info);

  void VisitValue(Value* edge) override;
  void VisitArrayEdge(ArrayEdge* edge) override;

 private:
  bool ContainsForbiddenFieldsInternal(RecordInfo* info);
  bool ContainsInvalidFieldTypes(Value* edge);

  // The current path that we are working on.
  // This permits to find the path back to the GCed type when handling embedded
  // objects.
  RootPath current_;

  // The currently visited nodes. This is used to handle cyclic dependencies
  // between the embedded objects we are going through.
  VisitingSet visiting_set_;

  // The actual fields that were found while inspecting the record.
  Errors forbidden_fields_;
};

#endif  // TOOLS_CLANG_BLINK_GC_PLUGIN_CHECKFORBIDDENFIELDSVISITOR_H_
