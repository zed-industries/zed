// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL_TRANSFORM_FEEDBACK_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL_TRANSFORM_FEEDBACK_H_

#include "third_party/blink/renderer/modules/webgl/webgl_object.h"
#include "third_party/blink/renderer/modules/webgl/webgl_program.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"

namespace blink {

class WebGL2RenderingContextBase;
class WebGLBuffer;

class WebGLTransformFeedback : public WebGLObject {
  DEFINE_WRAPPERTYPEINFO();

 public:
  enum class TFType {
    kDefault,
    kUser,
  };

  explicit WebGLTransformFeedback(
      WebGL2RenderingContextBase*,
      TFType,
      GLint max_transform_feedback_separate_attribs);
  ~WebGLTransformFeedback() override;

  bool IsDefaultObject() const { return type_ == TFType::kDefault; }

  GLenum GetTarget() const { return target_; }
  void SetTarget(GLenum);

  bool HasEverBeenBound() const { return HasObject() && target_; }

  WebGLProgram* GetProgram() const { return program_.Get(); }
  void SetProgram(WebGLProgram*);

  // These are the indexed bind points for transform feedback buffers.
  // Returns false if index is out of range and the caller should
  // synthesize a GL error.
  bool SetBoundIndexedTransformFeedbackBuffer(GLuint index, WebGLBuffer*);
  bool GetBoundIndexedTransformFeedbackBuffer(GLuint index,
                                              WebGLBuffer** outBuffer) const;
  bool HasBoundIndexedTransformFeedbackBuffer(const WebGLBuffer* buffer) {
    return bound_indexed_transform_feedback_buffers_.Contains(buffer);
  }
  bool HasEnoughBuffers(GLuint num_required) const;

  bool UsesBuffer(WebGLBuffer*);
  void UnbindBuffer(WebGLBuffer*);

  void Trace(Visitor*) const override;

  bool active() const { return active_; }
  bool paused() const { return paused_; }
  const HeapVector<Member<WebGLBuffer>>&
  bound_indexed_transform_feedback_buffers() const {
    return bound_indexed_transform_feedback_buffers_;
  }

  void SetActive(bool active) {
    active_ = active;
    DCHECK(active_ || !paused_);
  }
  void SetPaused(bool paused) {
    paused_ = paused;
    DCHECK(active_ || !paused_);
  }

  bool ValidateProgramForResume(WebGLProgram*) const;

 private:
  void DispatchDetached(gpu::gles2::GLES2Interface*);
  void DeleteObjectImpl(gpu::gles2::GLES2Interface*) override;

  TFType type_;
  GLenum target_;

  HeapVector<Member<WebGLBuffer>> bound_indexed_transform_feedback_buffers_;

  Member<WebGLProgram> program_;
  unsigned program_link_count_;
  bool active_;
  bool paused_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL_TRANSFORM_FEEDBACK_H_
