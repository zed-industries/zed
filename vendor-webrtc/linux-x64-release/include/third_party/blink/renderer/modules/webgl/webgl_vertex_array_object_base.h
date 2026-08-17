// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL_VERTEX_ARRAY_OBJECT_BASE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL_VERTEX_ARRAY_OBJECT_BASE_H_

#include "third_party/blink/renderer/modules/webgl/webgl_buffer.h"
#include "third_party/blink/renderer/modules/webgl/webgl_object.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class WebGLVertexArrayObjectBase : public WebGLObject {
 public:
  enum VaoType {
    kVaoTypeDefault,
    kVaoTypeUser,
  };

  ~WebGLVertexArrayObjectBase() override;

  bool IsDefaultObject() const { return type_ == kVaoTypeDefault; }

  bool HasEverBeenBound() const { return Object() && has_ever_been_bound_; }
  void SetHasEverBeenBound() { has_ever_been_bound_ = true; }

  WebGLBuffer* BoundElementArrayBuffer() const {
    return bound_element_array_buffer_.Get();
  }
  void SetElementArrayBuffer(WebGLBuffer*);

  WebGLBuffer* GetArrayBufferForAttrib(GLuint);
  void SetArrayBufferForAttrib(GLuint, WebGLBuffer*);
  void SetAttribEnabled(GLuint, bool);
  bool IsAllEnabledAttribBufferBound() const {
    return is_all_enabled_attrib_buffer_bound_;
  }
  bool HasArrayBuffer(const WebGLBuffer* buffer) {
    return array_buffer_list_.Contains(buffer);
  }
  void UnbindBuffer(WebGLBuffer*);

  void Trace(Visitor*) const override;

 protected:
  WebGLVertexArrayObjectBase(WebGLRenderingContextBase*,
                             VaoType,
                             GLint max_vertex_attribs);

 private:
  void DispatchDetached(gpu::gles2::GLES2Interface*);
  void DeleteObjectImpl(gpu::gles2::GLES2Interface*) override;

  void UpdateAttribBufferBoundStatus();

  VaoType type_;
  bool has_ever_been_bound_;
  Member<WebGLBuffer> bound_element_array_buffer_;
  HeapVector<Member<WebGLBuffer>> array_buffer_list_;
  Vector<bool> attrib_enabled_;
  bool is_all_enabled_attrib_buffer_bound_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL_VERTEX_ARRAY_OBJECT_BASE_H_
