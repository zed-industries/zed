// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_PLUGINS_TYPEPREDICATEUTIL_H_
#define TOOLS_CLANG_PLUGINS_TYPEPREDICATEUTIL_H_

#include <map>
#include <memory>
#include <optional>
#include <set>
#include <string>
#include <vector>

#include "clang/AST/Decl.h"
#include "clang/AST/DeclTemplate.h"
#include "clang/AST/Type.h"
#include "llvm/ADT/ScopeExit.h"

enum class InductionRule : unsigned {
  kNone = 0,
  kPointerPointee = (1 << 0),
  kObjCPointerPointee = (1 << 1),
  kReferencePointee = (1 << 2),
  kArrayElement = (1 << 3),
  kUnqualifiedDesugaredType = (1 << 4),
  kBaseClass = (1 << 5),
  kVirtualBaseClass = (1 << 6),
  kField = (1 << 7),
  kTemplateArgument = (1 << 8),
};

constexpr InductionRule operator|(InductionRule a, InductionRule b) {
  return static_cast<InductionRule>(static_cast<unsigned>(a) |
                                    static_cast<unsigned>(b));
}
constexpr InductionRule operator&(InductionRule a, InductionRule b) {
  return static_cast<InductionRule>(static_cast<unsigned>(a) &
                                    static_cast<unsigned>(b));
}

// Represents a match result |verdict_|.
// - MatchResult::kNoMatch: no match found against |type|.
// - MatchResult::kMatch: a match found against |type|.
// - MatchResult::kUndetermined: This denotes the result
//   is not yet determined, due to cross references.
// Holds some additional information to tell reasons.
class MatchResult {
 public:
  enum Verdict {
    kMatch,
    kNoMatch,
    // This denotes the match status is not yet determined.
    kUndetermined,
  };

  explicit MatchResult(const clang::Type* type) : type_(type) {}
  explicit MatchResult(const clang::Type* type, Verdict verdict)
      : type_(type), verdict_(verdict) {}

  const clang::Type* type() const { return type_; }

  Verdict verdict() const { return this->verdict_; }

  std::shared_ptr<MatchResult> source() const { return this->source_; }

  std::optional<clang::SourceLocation> source_loc() const {
    return this->source_loc_;
  }

 private:
  template <InductionRule Rules>
  friend class TypePredicate;

  // Merges a sub verdict into this type's verdict.
  //
  // | this   \ sub  | kNoMatch      | kUndetermined | kMatch |
  // +---------------+---------------+---------------+--------+
  // | kNoMatch      | kNoMatch      | kUndetermined | kMatch |
  // | kUndetermined | kUndetermined | kUndetermined | kMatch |
  // | kMatch        | kMatch        | kMatch        | kMatch |
  Verdict MergeSubResult(
      std::shared_ptr<MatchResult> sub,
      std::optional<clang::SourceLocation> loc = std::nullopt) {
    if (sub->verdict_ == kMatch && this->verdict_ != kMatch) {
      this->verdict_ = kMatch;
      this->source_ = std::move(sub);
      this->source_loc_ = loc;
    } else if (sub->verdict_ == kUndetermined && this->verdict_ == kNoMatch) {
      this->verdict_ = kUndetermined;
      this->source_ = std::move(sub);
      this->source_loc_ = loc;
    }
    return this->verdict_;
  }

  // |type_| is considered to be |verdict_|.
  // Optionally, the result contains a reason for the verdict, |source_|.
  // There can be multiple reasons (e.g. |type_| has multiple matching
  // members), but only one of them is stored. The relation between |type_|
  // and |source_| is optionally shown at |source_loc_|.
  const clang::Type* type_;
  Verdict verdict_ = kNoMatch;
  std::shared_ptr<MatchResult> source_;
  std::optional<clang::SourceLocation> source_loc_;
};

// Determines there is a match against |type| or not.
// A type is considered match if |IsBaseMatch| returns true or
// reach such |type| by applying InductionRule recursively.
template <InductionRule Rules>
class TypePredicate {
 public:
  virtual ~TypePredicate() = default;
  bool Matches(const clang::Type* type) const {
    return GetMatchResult(type)->verdict_ == MatchResult::kMatch;
  }

  std::shared_ptr<MatchResult> GetMatchResult(
      const clang::Type* type,
      std::set<const clang::Type*>* visited = nullptr) const {
    // Retrieve a "base" type to reduce recursion depth.
    const clang::Type* raw_type = GetBaseType(type);
    if (!raw_type || !raw_type->isRecordType()) {
      // |TypePredicate| does not support followings:
      // - function type
      // - enum type
      // - builtin type
      // - complex type
      // - obj-C types
      // - using type
      // - typeof type
      return std::make_shared<MatchResult>(type);  // No match.
    }

    // Use a memoized result if exists.
    auto iter = cache_.find(type);
    if (iter != cache_.end()) {
      return iter->second;
    }

    // This performs DFS on a directed graph composed of |Type*|.
    // Avoid searching for visited nodes by managing |visited|, as this can lead
    // to infinite loops in the presence of self-references and
    // cross-references. Since finding a match for |Type* x| is equivalent to
    // being able to reach from node |Type* x| to node |Type* y| where
    // |IsBaseCase(y)|, there is no need to look up visited nodes again.
    bool root = visited == nullptr;
    if (root) {
      // Will be deleted as a part of |clean_up()|.
      visited = new std::set<const clang::Type*>();
    } else if (visited->count(type)) {
      // This type is already visited but not memoized,
      // therefore this node is reached by following cross-references from
      // ancestors. The verdict of this node cannot be determined without
      // waiting for computation in its ancestors.
      return std::make_shared<MatchResult>(raw_type,
                                           MatchResult::kUndetermined);
    }
    visited->insert(type);

    auto match = std::make_shared<MatchResult>(raw_type);

    // Clean-up: this lambda is called automatically at the scope exit.
    const auto clean_up =
        llvm::make_scope_exit([this, &visited, &raw_type, &root, &match] {
          if (root) {
            delete visited;
          }
          // Memoize the result if finalized.
          if (match->verdict_ != MatchResult::kUndetermined) {
            this->cache_.insert({raw_type, match});
          }
        });

    // Base case.
    if (IsBaseMatch(raw_type)) {
      match->verdict_ = MatchResult::kMatch;
      return match;
    }

    const clang::RecordDecl* decl = raw_type->getAsRecordDecl();
    assert(decl);

    // Check member fields
    if constexpr ((Rules & InductionRule::kField) != InductionRule::kNone) {
      for (const auto& field : decl->fields()) {
        match->MergeSubResult(
            GetMatchResult(field->getType().getTypePtrOrNull(), visited),
            field->getBeginLoc());

        // Verdict finalized: early return.
        if (match->verdict_ == MatchResult::kMatch) {
          return match;
        }
      }
    }

    const auto* cxx_decl = clang::dyn_cast<clang::CXXRecordDecl>(decl);
    if (cxx_decl && cxx_decl->hasDefinition()) {
      // Check base classes
      if constexpr ((Rules & InductionRule::kBaseClass) !=
                    InductionRule::kNone) {
        for (const auto& base_specifier : cxx_decl->bases()) {
          match->MergeSubResult(
              GetMatchResult(base_specifier.getType().getTypePtr(), visited),
              base_specifier.getBeginLoc());

          // Verdict finalized: early return.
          if (match->verdict_ == MatchResult::kMatch) {
            return match;
          }
        }
      }

      // Check virtual base classes
      if constexpr ((Rules & InductionRule::kVirtualBaseClass) !=
                    InductionRule::kNone) {
        for (const auto& base_specifier : cxx_decl->vbases()) {
          match->MergeSubResult(
              GetMatchResult(base_specifier.getType().getTypePtr(), visited),
              base_specifier.getBeginLoc());

          // Verdict finalized: early return.
          if (match->verdict_ == MatchResult::kMatch) {
            return match;
          }
        }
      }
    }

    // Check template parameters.
    if constexpr ((Rules & InductionRule::kTemplateArgument) !=
                  InductionRule::kNone) {
      if (auto* field_record_template =
              clang::dyn_cast<clang::ClassTemplateSpecializationDecl>(decl)) {
        const auto& template_args = field_record_template->getTemplateArgs();
        for (unsigned i = 0; i < template_args.size(); i++) {
          if (template_args[i].getKind() != clang::TemplateArgument::Type) {
            continue;
          }
          match->MergeSubResult(
              GetMatchResult(template_args[i].getAsType().getTypePtrOrNull(),
                             visited),
              field_record_template->getTemplateKeywordLoc());

          // Verdict finalized: early return.
          if (match->verdict_ == MatchResult::kMatch) {
            return match;
          }
        }
      }
    }

    // All reachable types have been traversed but the root type has not
    // been marked as a match; therefore it must be no match.
    if (root && match->verdict_ == MatchResult::kUndetermined) {
      match->verdict_ = MatchResult::kNoMatch;
    }
    return match;
  }

 private:
  const clang::Type* GetBaseType(const clang::Type* type) const {
    using clang::dyn_cast;

    const clang::Type* last_type = nullptr;
    while (type && type != last_type) {
      last_type = type;

      // Unwrap type aliases.
      if constexpr ((Rules & InductionRule::kUnqualifiedDesugaredType) !=
                    InductionRule::kNone) {
        type = type->getUnqualifiedDesugaredType();
      }

      // Unwrap pointers.
      if constexpr ((Rules & InductionRule::kPointerPointee) !=
                    InductionRule::kNone) {
        while (type && type->isPointerType()) {
          type = type->getPointeeType().getTypePtr();
        }
      }

      // Unwrap ObjC pointers.
      if constexpr ((Rules & InductionRule::kObjCPointerPointee) !=
                    InductionRule::kNone) {
        while (type && type->isObjCObjectPointerType()) {
          type = type->getPointeeType().getTypePtr();
        }
      }

      // Unwrap array.
      if constexpr ((Rules & InductionRule::kArrayElement) !=
                    InductionRule::kNone) {
        while (const auto* array_type = dyn_cast<clang::ArrayType>(type)) {
          type = array_type->getElementType().getTypePtr();
        }
      }

      // Unwrap reference.
      if constexpr ((Rules & InductionRule::kReferencePointee) !=
                    InductionRule::kNone) {
        if (const auto* ref_type = dyn_cast<clang::ReferenceType>(type)) {
          type = ref_type->getPointeeType().getTypePtrOrNull();
        }
      }
    }
    return type;
  }

  virtual bool IsBaseMatch(const clang::Type* type) const { return false; }

  // Cache to efficiently determine match.
  mutable std::map<const clang::Type*, std::shared_ptr<MatchResult>> cache_;
};

#endif  // TOOLS_CLANG_PLUGINS_TYPEPREDICATEUTIL_H_
