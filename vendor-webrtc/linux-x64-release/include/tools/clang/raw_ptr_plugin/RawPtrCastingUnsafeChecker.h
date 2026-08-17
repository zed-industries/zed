// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_RAW_PTR_PLUGIN_RAWPTRCASTINGUNSAFECHECKER_H_
#define TOOLS_CLANG_RAW_PTR_PLUGIN_RAWPTRCASTINGUNSAFECHECKER_H_

#include <string>

#include "TypePredicateUtil.h"
#include "clang/AST/Type.h"

// Determines a type is "casting unsafe" or not.
// A type is considered "casting unsafe" if ref counting can be broken as a
// result of casting. We determine "casting unsafe" types by applying rules here
// recursively.
class CastingUnsafePredicate
    : public TypePredicate<
          // Pointers, references and arrays to "casting unsafe" element(s) are
          // "casting unsafe"
          InductionRule::kPointerPointee | InductionRule::kObjCPointerPointee |
          InductionRule::kReferencePointee | InductionRule::kArrayElement |
          // Typedef, using aliases to "casting unsafe" type are "casting
          // unsafe"
          InductionRule::kUnqualifiedDesugaredType |
          // Classes and structs having a "casting unsafe" base class are
          // "casting unsafe"
          InductionRule::kBaseClass | InductionRule::kVirtualBaseClass |
          // Classes and structs having a "casting unsafe" member are "casting
          // unsafe"
          InductionRule::kField> {
  // Base case: |raw_ptr<T>| and |raw_ref<T>| are never safe to cast to/from.
  // When implemented as BackupRefPtr, assignment needs to go through the C++
  // operators to ensure the refcount is properly maintained.
  bool IsBaseMatch(const clang::Type* type) const override {
    const auto* decl = type->getAsRecordDecl();
    if (!decl) {
      return false;
    }
    const std::string record_name = decl->getQualifiedNameAsString();
    return record_name == "base::raw_ptr" || record_name == "base::raw_ref";
  }
};

#endif  // TOOLS_CLANG_RAW_PTR_PLUGIN_RAWPTRCASTINGUNSAFECHECKER_H_
