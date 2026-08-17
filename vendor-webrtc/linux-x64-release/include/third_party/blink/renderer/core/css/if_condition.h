// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_IF_CONDITION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_IF_CONDITION_H_

#include "third_party/blink/renderer/core/css/media_list.h"
#include "third_party/blink/renderer/core/css/media_query.h"
#include "third_party/blink/renderer/core/css/media_query_exp.h"
#include "third_party/blink/renderer/core/media_type_names.h"

namespace blink {

// https://drafts.csswg.org/css-values-5/#typedef-if-condition
class CORE_EXPORT IfCondition : public GarbageCollected<IfCondition> {
 public:
  virtual ~IfCondition() = default;
  virtual void Trace(Visitor*) const {}

  enum class Type {
    kStyle,
    kMedia,
    kSupports,
    kNot,
    kAnd,
    kOr,
    kUnknown,
    kElse
  };
  virtual Type GetType() const = 0;

  // These helper functions return nullptr if any argument is nullptr.
  static const IfCondition* Not(const IfCondition*);
  static const IfCondition* And(const IfCondition*, const IfCondition*);
  static const IfCondition* Or(const IfCondition*, const IfCondition*);
};

class IfConditionNot : public IfCondition {
 public:
  explicit IfConditionNot(const IfCondition* operand) : operand_(operand) {}
  void Trace(Visitor* visitor) const override;
  Type GetType() const override { return Type::kNot; }
  const IfCondition& Operand() const { return *operand_; }

 private:
  Member<const IfCondition> operand_;
};

class IfConditionAnd : public IfCondition {
 public:
  IfConditionAnd(const IfCondition* left, const IfCondition* right)
      : left_(left), right_(right) {}
  void Trace(Visitor*) const override;
  Type GetType() const override { return Type::kAnd; }
  const IfCondition& Left() const { return *left_; }
  const IfCondition& Right() const { return *right_; }

 private:
  Member<const IfCondition> left_;
  Member<const IfCondition> right_;
};

class IfConditionOr : public IfCondition {
 public:
  IfConditionOr(const IfCondition* left, const IfCondition* right)
      : left_(left), right_(right) {}
  void Trace(Visitor*) const override;
  Type GetType() const override { return Type::kOr; }
  const IfCondition& Left() const { return *left_; }
  const IfCondition& Right() const { return *right_; }

 private:
  Member<const IfCondition> left_;
  Member<const IfCondition> right_;
};

class CORE_EXPORT IfTestStyle : public IfCondition {
 public:
  explicit IfTestStyle(const MediaQueryExpNode* style_test)
      : style_test_(style_test) {}
  void Trace(Visitor*) const override;
  Type GetType() const override { return Type::kStyle; }
  const MediaQueryExpNode* GetMediaQueryExpNode() const { return style_test_; }

 private:
  Member<const MediaQueryExpNode> style_test_;
};

class CORE_EXPORT IfTestMedia : public IfCondition {
 public:
  explicit IfTestMedia(const MediaQueryExpNode* exp_node) {
    HeapVector<Member<const MediaQuery>> queries;
    queries.push_back(MakeGarbageCollected<MediaQuery>(
        MediaQuery::RestrictorType::kNone, media_type_names::kAll, exp_node));
    media_test_ = MakeGarbageCollected<MediaQuerySet>(std::move(queries));
  }
  void Trace(Visitor*) const override;
  Type GetType() const override { return Type::kMedia; }
  const MediaQuerySet* GetMediaQuerySet() const { return media_test_; }

 private:
  Member<const MediaQuerySet> media_test_;
};

class CORE_EXPORT IfTestSupports : public IfCondition {
 public:
  explicit IfTestSupports(bool result) : result_(result) {}
  void Trace(Visitor*) const override;
  Type GetType() const override { return Type::kSupports; }
  bool GetResult() const { return result_; }

 private:
  bool result_;
};

class CORE_EXPORT IfConditionUnknown : public IfCondition {
 public:
  explicit IfConditionUnknown(String string) : string_(string) {}
  void Trace(Visitor*) const override;
  Type GetType() const override { return Type::kUnknown; }
  String GetString() const { return string_; }

 private:
  String string_;
};

class CORE_EXPORT IfConditionElse : public IfCondition {
 public:
  explicit IfConditionElse() = default;
  void Trace(Visitor*) const override;
  Type GetType() const override { return Type::kElse; }
};

template <>
struct DowncastTraits<IfConditionNot> {
  static bool AllowFrom(const IfCondition& node) {
    return node.GetType() == IfCondition::Type::kNot;
  }
};

template <>
struct DowncastTraits<IfConditionAnd> {
  static bool AllowFrom(const IfCondition& node) {
    return node.GetType() == IfCondition::Type::kAnd;
  }
};

template <>
struct DowncastTraits<IfConditionOr> {
  static bool AllowFrom(const IfCondition& node) {
    return node.GetType() == IfCondition::Type::kOr;
  }
};

template <>
struct DowncastTraits<IfTestStyle> {
  static bool AllowFrom(const IfCondition& node) {
    return node.GetType() == IfCondition::Type::kStyle;
  }
};

template <>
struct DowncastTraits<IfTestMedia> {
  static bool AllowFrom(const IfCondition& node) {
    return node.GetType() == IfCondition::Type::kMedia;
  }
};

template <>
struct DowncastTraits<IfTestSupports> {
  static bool AllowFrom(const IfCondition& node) {
    return node.GetType() == IfCondition::Type::kSupports;
  }
};

template <>
struct DowncastTraits<IfConditionUnknown> {
  static bool AllowFrom(const IfCondition& node) {
    return node.GetType() == IfCondition::Type::kUnknown;
  }
};

template <>
struct DowncastTraits<IfConditionElse> {
  static bool AllowFrom(const IfCondition& node) {
    return node.GetType() == IfCondition::Type::kElse;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_IF_CONDITION_H_
