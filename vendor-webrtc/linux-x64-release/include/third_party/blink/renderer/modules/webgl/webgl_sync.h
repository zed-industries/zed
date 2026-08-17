// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL_SYNC_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL_SYNC_H_

#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/modules/webgl/webgl_object.h"
#include "third_party/blink/renderer/platform/scheduler/public/post_cancellable_task.h"

namespace gpu {
namespace gles2 {
class GLES2Interface;
}
}  // namespace gpu

namespace blink {

class WebGL2RenderingContextBase;

class WebGLSync : public WebGLObject {
  DEFINE_WRAPPERTYPEINFO();

 public:
  ~WebGLSync() override;

  void UpdateCache(gpu::gles2::GLES2Interface*);
  GLint GetCachedResult(GLenum pname);
  bool IsSignaled() const;

 protected:
  WebGLSync(WebGL2RenderingContextBase*, GLuint, GLenum object_type);

  void DeleteObjectImpl(gpu::gles2::GLES2Interface*) override;

 private:
  void ScheduleAllowCacheUpdate();
  void AllowCacheUpdate();

  bool allow_cache_update_ = false;
  // Initialized in cpp file to avoid including gl3.h in this header.
  GLint sync_status_;

  GLenum object_type_;

  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;
  TaskHandle task_handle_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL_SYNC_H_
