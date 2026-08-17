// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_DIRECTIVE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_DIRECTIVE_H_

#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class V8DirectiveType;

// Provides the JavaScript-exposed Directive base class used by
// window.fragmentDirective.items. This is the base interface for all fragment
// directive types.
// See: https://github.com/WICG/scroll-to-text-fragment/issues/160
// TODO(bokan): Update link once we have better public documentation.
class Directive : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  enum Type { kUnknown, kText, kSelector };

  explicit Directive(Type type);
  ~Directive() override;

  Type GetType() const;
  bool IsConsumed() { return consumed_; }
  void SetConsumed(bool consumed) { consumed_ = consumed; }
  void Trace(Visitor*) const override;

  // Web-exposed Directive interface.
  V8DirectiveType type() const;
  String toString() const;

 protected:
  // Override in subclasses to implement the toString() web-exposed method.
  virtual String ToStringImpl() const = 0;

 private:
  Type type_;
  bool consumed_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_DIRECTIVE_H_
