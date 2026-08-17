/*
 * Copyright (C) 2012, 2013 Adobe Systems Incorporated. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDER "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY,
 * OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR
 * TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF
 * THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_PATH_2D_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_PATH_2D_H_

#include <cmath>

#include "base/check.h"
#include "base/compiler_specific.h"
#include "base/notreached.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_union_path2d_string.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/geometry/dom_matrix_read_only.h"
#include "third_party/blink/renderer/core/svg/svg_path_utilities.h"
#include "third_party/blink/renderer/modules/canvas/canvas2d/canvas_path.h"
#include "third_party/blink/renderer/modules/canvas/canvas2d/identifiability_study_helper.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/geometry/path.h"
#include "third_party/blink/renderer/platform/geometry/path_builder.h"
#include "third_party/blink/renderer/platform/heap/forward.h"  // IWYU pragma: keep (blink::Visitor)
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

// IWYU pragma: no_include "third_party/blink/renderer/platform/heap/visitor.h"

namespace blink {

class DOMMatrix2DInit;
class ExceptionState;
class ScriptState;

class MODULES_EXPORT Path2D final : public ScriptWrappable, public CanvasPath {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static Path2D* Create(ScriptState* script_state,
                        const V8UnionPath2DOrString* path) {
    DCHECK(path);
    switch (path->GetContentType()) {
      case V8UnionPath2DOrString::ContentType::kPath2D:
        return MakeGarbageCollected<Path2D>(script_state, path->GetAsPath2D());
      case V8UnionPath2DOrString::ContentType::kString:
        return MakeGarbageCollected<Path2D>(script_state, path->GetAsString());
    }
    NOTREACHED();
  }
  static Path2D* Create(ScriptState* script_state) {
    return MakeGarbageCollected<Path2D>(script_state);
  }
  static Path2D* Create(ScriptState* script_state, const Path& path) {
    return MakeGarbageCollected<Path2D>(script_state, path);
  }

  void addPath(Path2D* path,
               DOMMatrix2DInit* transform,
               ExceptionState& exception_state) {
    DOMMatrixReadOnly* matrix =
        DOMMatrixReadOnly::fromMatrix2D(transform, exception_state);
    if (!matrix || !std::isfinite(matrix->m11()) ||
        !std::isfinite(matrix->m12()) || !std::isfinite(matrix->m21()) ||
        !std::isfinite(matrix->m22()) || !std::isfinite(matrix->m41()) ||
        !std::isfinite(matrix->m42()))
      return;
    GetModifiablePath().AddPath(path->GetPath(), matrix->GetAffineTransform());
    if (identifiability_study_helper_.ShouldUpdateBuilder()) [[unlikely]] {
      identifiability_study_helper_.UpdateBuilder(CanvasOps::kAddPath,
                                                  path->GetIdentifiableToken());
    }
  }

  void Trace(Visitor*) const override;

  ExecutionContext* GetTopExecutionContext() const override {
    return context_.Get();
  }

  explicit Path2D(ScriptState* script_state)
      : context_(ExecutionContext::From(script_state)) {
    identifiability_study_helper_.SetExecutionContext(context_.Get());
    GetModifiablePath().SetIsVolatile(false);
  }
  Path2D(ScriptState* script_state, const Path& path)
      : CanvasPath(path), context_(ExecutionContext::From(script_state)) {
    identifiability_study_helper_.SetExecutionContext(context_.Get());
    GetModifiablePath().SetIsVolatile(false);
  }
  Path2D(ScriptState* script_state, Path2D* path)
      : Path2D(script_state, path->GetPath()) {}
  Path2D(ScriptState* script_state, const String& path_data)
      : context_(ExecutionContext::From(script_state)) {
    identifiability_study_helper_.SetExecutionContext(context_.Get());
    GetModifiablePath() = PathBuilder(BuildPathFromString(path_data));
    GetModifiablePath().SetIsVolatile(false);
  }

  Path2D(const Path2D&) = delete;
  Path2D& operator=(const Path2D&) = delete;

  ~Path2D() override = default;

 private:
  Member<ExecutionContext> context_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_PATH_2D_H_
