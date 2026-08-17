// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_BLINK_GC_PLUGIN_CHECK_FIELDS_VISITOR_H_
#define TOOLS_BLINK_GC_PLUGIN_CHECK_FIELDS_VISITOR_H_

#include <vector>

#include "BlinkGCPluginOptions.h"
#include "Edge.h"

class FieldPoint;

// This visitor checks that the fields of a class are "well formed".
// - unique_ptr, scoped_refptr and WeakPtr must not point to a GC derived type.
// - Part objects must not be a GC derived type.
// - An on-heap class must never contain GC roots.
// - Only stack-allocated types may point to stack-allocated types.

class CheckFieldsVisitor : public RecursiveEdgeVisitor {
 public:
  enum Error {
    kRawPtrToGCManaged,
    kRefPtrToGCManaged,
    kReferencePtrToGCManaged,
    kUniquePtrToGCManaged,
    kMemberToGCUnmanaged,
    kMemberInUnmanaged,
    kPtrToMemberInUnmanaged,
    kPtrFromHeapToStack,
    kGCDerivedPartObject,
    kIteratorToGCManaged,
    kMemberInStackAllocated,
    kTraceablePartObjectInUnmanaged,
    kRawPtrToTraceable,
    kRefPtrToTraceable,
    kReferencePtrToTraceable,
    kUniquePtrToTraceable,
  };

  using Errors = std::vector<std::pair<FieldPoint*, Error>>;

  explicit CheckFieldsVisitor(const BlinkGCPluginOptions&);

  Errors& invalid_fields();

  bool ContainsInvalidFields(RecordInfo* info);

  void AtMember(Member*) override;
  void AtWeakMember(WeakMember*) override;
  void AtValue(Value*) override;
  void AtCollection(Collection*) override;
  void AtIterator(Iterator*) override;

 private:
  const BlinkGCPluginOptions& options_;

  FieldPoint* current_;
  bool stack_allocated_host_;
  bool managed_host_;
  Errors invalid_fields_;
};

#endif  // TOOLS_BLINK_GC_PLUGIN_CHECK_FIELDS_VISITOR_H_
