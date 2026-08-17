// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_BLINK_GC_PLUGIN_CHECK_GC_ROOTS_VISITOR_H_
#define TOOLS_BLINK_GC_PLUGIN_CHECK_GC_ROOTS_VISITOR_H_

#include <set>
#include <vector>

#include "Edge.h"
#include "RecordInfo.h"

struct BlinkGCPluginOptions;

// This visitor checks that the fields of a class and the fields of
// its part objects don't define GC roots.
class CheckGCRootsVisitor : public RecursiveEdgeVisitor {
 public:
  typedef std::vector<FieldPoint*> RootPath;
  typedef std::set<RecordInfo*> VisitingSet;
  typedef std::vector<RootPath> Errors;

  explicit CheckGCRootsVisitor(const BlinkGCPluginOptions&);

  Errors& gc_roots();
  Errors& gc_root_refs();

  bool ContainsGCRoots(RecordInfo* info);

  void VisitValue(Value* edge) override;
  void VisitUniquePtr(UniquePtr*) override;
  void VisitRawPtr(RawPtr*) override;
  void VisitRefPtr(RefPtr*) override;
  void VisitPersistent(Persistent* edge) override;
  void VisitCollection(Collection* edge) override;

 private:
  RootPath current_;
  VisitingSet visiting_set_;
  Errors gc_roots_;
  Errors gc_root_refs_;
  bool is_ref_ = false;

  bool should_check_unique_ptrs_;
};

#endif  // TOOLS_BLINK_GC_PLUGIN_CHECK_GC_ROOTS_VISITOR_H_
