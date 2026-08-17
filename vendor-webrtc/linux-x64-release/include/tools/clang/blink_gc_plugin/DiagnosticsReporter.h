// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_BLINK_GC_PLUGIN_DIAGNOSTICS_REPORTER_H_
#define TOOLS_BLINK_GC_PLUGIN_DIAGNOSTICS_REPORTER_H_

#include "CheckFieldsVisitor.h"
#include "CheckFinalizerVisitor.h"
#include "CheckForbiddenFieldsVisitor.h"
#include "CheckGCRootsVisitor.h"
#include "Config.h"
#include "clang/AST/AST.h"
#include "clang/AST/ASTConsumer.h"
#include "clang/Basic/Diagnostic.h"
#include "clang/Frontend/CompilerInstance.h"

class RecordInfo;

// All error/warning reporting methods under one roof.
//
class DiagnosticsReporter {
 public:
  explicit DiagnosticsReporter(clang::CompilerInstance&);

  bool hasErrorOccurred() const;
  clang::DiagnosticsEngine::Level getErrorLevel() const;

  void ClassMustLeftMostlyDeriveGC(RecordInfo* info);
  void ClassRequiresTraceMethod(RecordInfo* info);
  void BaseRequiresTracing(RecordInfo* derived,
                           clang::CXXMethodDecl* trace,
                           clang::CXXRecordDecl* base);
  void FieldsImproperlyTraced(RecordInfo* info,
                              clang::CXXMethodDecl* trace);
  void ClassContainsInvalidFields(
      RecordInfo* info,
      const CheckFieldsVisitor::Errors& errors);
  void ClassContainsGCRoots(RecordInfo* info,
                            const CheckGCRootsVisitor::Errors& errors);
  void ClassContainsGCRootRefs(RecordInfo* info,
                               const CheckGCRootsVisitor::Errors& errors);
  void ClassContainsForbiddenFields(
      RecordInfo* info,
      const CheckForbiddenFieldsVisitor::Errors& errors);
  void FinalizerAccessesFinalizedFields(
      clang::CXXMethodDecl* dtor,
      const CheckFinalizerVisitor::Errors& errors);
  void ClassMustDeclareGCMixinTraceMethod(RecordInfo* info);
  void OverriddenNonVirtualTrace(RecordInfo* info,
                                 clang::CXXMethodDecl* trace,
                                 clang::CXXMethodDecl* overridden);
  void MissingTraceDispatchMethod(RecordInfo* info);
  void VirtualAndManualDispatch(RecordInfo* info,
                                clang::CXXMethodDecl* dispatch);
  void MissingTraceDispatch(const clang::FunctionDecl* dispatch,
                            RecordInfo* receiver);
  void MissingFinalizeDispatch(const clang::FunctionDecl* dispatch,
                               RecordInfo* receiver);
  void StackAllocatedDerivesGarbageCollected(RecordInfo* info, BasePoint* base);
  void ClassOverridesNew(RecordInfo* info, clang::CXXMethodDecl* newop);
  void ClassDeclaresPureVirtualTrace(RecordInfo* info,
                                     clang::CXXMethodDecl* trace);
  void LeftMostBaseMustBePolymorphic(RecordInfo* derived,
                                     clang::CXXRecordDecl* base);
  void BaseClassMustDeclareVirtualTrace(RecordInfo* derived,
                                              clang::CXXRecordDecl* base);
  void ClassMustCRTPItself(const RecordInfo* derived,
                           const clang::CXXRecordDecl* base,
                           const clang::CXXBaseSpecifier* base_spec);
  void TraceMethodForStackAllocatedClass(RecordInfo* parent,
                                         clang::CXXMethodDecl* trace);
  void RedundantTraceDispatchMethod(RecordInfo* derived,
                                    clang::CXXRecordDecl* base);
  void RedundantFinalizeDispatchMethod(RecordInfo* derived,
                                       clang::CXXRecordDecl* base);

  void NoteManualDispatchMethod(clang::CXXMethodDecl* dispatch);
  void NoteBaseRequiresTracing(BasePoint* base);
  void NoteFieldRequiresTracing(RecordInfo* holder, clang::FieldDecl* field);
  void NoteFieldShouldNotBeTraced(RecordInfo* holder, clang::FieldDecl* field);
  void NotePartObjectContainsGCRoot(FieldPoint* point);
  void NotePartObjectContainsGCRootRef(FieldPoint* point);
  void NoteFieldContainsGCRoot(FieldPoint* point);
  void NoteFieldContainsGCRootRef(FieldPoint* point);
  void NoteField(FieldPoint* point, unsigned note);
  void NoteField(clang::FieldDecl* field, unsigned note);
  void NoteOverriddenNonVirtualTrace(clang::CXXMethodDecl* overridden);

  // Used by FindBadPatterns.
  void UniquePtrUsedWithGC(const clang::Expr* expr,
                           const clang::FunctionDecl* bad_function,
                           const clang::CXXRecordDecl* gc_type);
  void OptionalDeclUsedWithGC(const clang::Decl* decl,
                              const clang::CXXRecordDecl* optional,
                              const clang::CXXRecordDecl* gc_type);
  void OptionalNewExprUsedWithGC(const clang::Expr* expr,
                                 const clang::CXXRecordDecl* optional,
                                 const clang::CXXRecordDecl* gc_type);
  void OptionalDeclUsedWithMember(const clang::Decl* decl,
                                  const clang::CXXRecordDecl* optional,
                                  const clang::CXXRecordDecl* member);
  void OptionalNewExprUsedWithMember(const clang::Expr* expr,
                                     const clang::CXXRecordDecl* optional,
                                     const clang::CXXRecordDecl* member);
  void RawPtrOrRefDeclUsedWithGC(const clang::Decl* decl,
                                 const clang::CXXRecordDecl* optional,
                                 const clang::CXXRecordDecl* gc_type);
  void RawPtrOrRefNewExprUsedWithGC(const clang::Expr* expr,
                                    const clang::CXXRecordDecl* optional,
                                    const clang::CXXRecordDecl* gc_type);
  void VariantUsedWithGC(const clang::Expr* expr,
                         const clang::CXXRecordDecl* variant,
                         const clang::CXXRecordDecl* gc_type);
  void CollectionOfGCed(const clang::Decl* decl,
                        const clang::CXXRecordDecl* collection,
                        const clang::CXXRecordDecl* gc_type);
  void CollectionOfGCed(const clang::Expr* expr,
                        const clang::CXXRecordDecl* collection,
                        const clang::CXXRecordDecl* gc_type);
  void CollectionOfMembers(const clang::Decl* decl,
                           const clang::CXXRecordDecl* collection,
                           const clang::CXXRecordDecl* gc_type);
  void CollectionOfMembers(const clang::Expr* expr,
                           const clang::CXXRecordDecl* collection,
                           const clang::CXXRecordDecl* gc_type);
  void MemberOnStack(const clang::VarDecl* var);
  void AdditionalPadding(const clang::RecordDecl* var, size_t padding);
  void WeakPtrToGCed(const clang::Decl* decl,
                     const clang::CXXRecordDecl* weak_ptr,
                     const clang::CXXRecordDecl* gc_type);
  void GCedField(const clang::FieldDecl* field,
                 const clang::CXXRecordDecl* gctype);
  void GCedVar(const clang::VarDecl* var, const clang::CXXRecordDecl* gctype);

 private:
  clang::DiagnosticBuilder ReportDiagnostic(
      clang::SourceLocation location,
      unsigned diag_id);

  void ReportMissingDispatchMethod(RecordInfo* info, unsigned error);
  void ReportMissingDispatch(const clang::FunctionDecl* dispatch,
                             RecordInfo* receiver,
                             unsigned error);

  clang::CompilerInstance& instance_;
  clang::DiagnosticsEngine& diagnostic_;

  unsigned diag_class_must_left_mostly_derive_gc_;
  unsigned diag_class_requires_trace_method_;
  unsigned diag_base_requires_tracing_;
  unsigned diag_fields_require_tracing_;
  unsigned diag_fields_improperly_traced_;
  unsigned diag_class_contains_invalid_fields_;
  unsigned diag_class_contains_gc_root_;
  unsigned diag_class_contains_gc_root_ref_;
  unsigned diag_finalizer_accesses_finalized_field_;
  unsigned diag_overridden_non_virtual_trace_;
  unsigned diag_missing_trace_dispatch_method_;
  unsigned diag_virtual_and_manual_dispatch_;
  unsigned diag_missing_trace_dispatch_;
  unsigned diag_missing_finalize_dispatch_;
  unsigned diag_stack_allocated_derives_gc_;
  unsigned diag_class_overrides_new_;
  unsigned diag_class_declares_pure_virtual_trace_;
  unsigned diag_left_most_base_must_be_polymorphic_;
  unsigned diag_base_class_must_declare_virtual_trace_;
  unsigned diag_class_must_crtp_itself_;
  unsigned diag_weak_ptr_to_gc_managed_class_;
  unsigned diag_gced_field_;
  unsigned diag_gced_var_;

  unsigned diag_base_requires_tracing_note_;
  unsigned diag_field_requires_tracing_note_;
  unsigned diag_field_should_not_be_traced_note_;
  unsigned diag_raw_ptr_to_gc_managed_class_note_;
  unsigned diag_ref_ptr_to_gc_managed_class_note_;
  unsigned diag_reference_ptr_to_gc_managed_class_note_;
  unsigned diag_unique_ptr_to_gc_managed_class_note_;
  unsigned diag_raw_ptr_to_traceable_class_note_;
  unsigned diag_ref_ptr_to_traceable_class_note_;
  unsigned diag_reference_ptr_to_traceable_class_note_;
  unsigned diag_unique_ptr_to_traceable_class_note_;
  unsigned diag_member_to_gc_unmanaged_class_note_;
  unsigned diag_stack_allocated_field_note_;
  unsigned diag_member_in_unmanaged_class_note_;
  unsigned diag_ptr_to_member_in_unmanaged_class_note_;
  unsigned diag_part_object_to_gc_derived_class_note_;
  unsigned diag_part_object_contains_gc_root_note_;
  unsigned diag_part_object_contains_gc_root_ref_note_;
  unsigned diag_field_contains_gc_root_note_;
  unsigned diag_field_contains_gc_root_ref_note_;
  unsigned diag_finalized_field_note_;
  unsigned diag_overridden_non_virtual_trace_note_;
  unsigned diag_manual_dispatch_method_note_;
  unsigned diag_iterator_to_gc_managed_collection_note_;
  unsigned diag_trace_method_of_stack_allocated_parent_;
  unsigned diag_member_in_stack_allocated_class_;
  unsigned diag_member_on_stack_;
  unsigned diag_additional_padding_;
  unsigned diag_part_object_in_unmanaged_;
  unsigned diag_task_runner_timer_in_gc_class_note;
  unsigned diag_forbidden_field_part_object_class_note;
  unsigned diag_mojo_remote_in_gc_class_note;
  unsigned diag_mojo_receiver_in_gc_class_note;
  unsigned diag_mojo_associated_remote_in_gc_class_note;
  unsigned diag_mojo_associated_receiver_in_gc_class_note;
  unsigned diag_redundant_trace_dispatch_method_;
  unsigned diag_redundant_finalize_dispatch_method_;

  unsigned diag_unique_ptr_used_with_gc_;
  unsigned diag_optional_decl_used_with_gc_;
  unsigned diag_optional_new_expr_used_with_gc_;
  unsigned diag_optional_decl_used_with_member_;
  unsigned diag_optional_new_expr_used_with_member_;
  unsigned diag_raw_ptr_or_ref_decl_used_with_gc_;
  unsigned diag_raw_ptr_or_ref_new_expr_used_with_gc_;
  unsigned diag_variant_used_with_gc_;
  unsigned diag_collection_of_gced_;
  unsigned diag_collection_of_members_;
};

#endif // TOOLS_BLINK_GC_PLUGIN_DIAGNOSTICS_REPORTER_H_
