// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_GPU_GL_CONTEXT_INTERNAL_H_
#define MEDIAPIPE_GPU_GL_CONTEXT_INTERNAL_H_

#include "absl/synchronization/mutex.h"
#ifdef __APPLE__
#if TARGET_OS_OSX
#import <AppKit/NSOpenGL.h>
#else
#import <OpenGLES/EAGL.h>
#endif  // TARGET_OS_OSX
#endif  // __APPLE__

#if MEDIAPIPE_DISABLE_PTHREADS
#include <thread>
#endif

#include "mediapipe/gpu/gl_context.h"

namespace mediapipe {

class GlContext::DedicatedThread {
 public:
  DedicatedThread();
  ~DedicatedThread();
  DedicatedThread(const DedicatedThread&) = delete;
  DedicatedThread& operator=(DedicatedThread) = delete;

  absl::Status Run(GlStatusFunction gl_func);
  void RunWithoutWaiting(GlVoidFunction gl_func);

  bool IsCurrentThread();

  void SelfDestruct();

 private:
  void ThreadBody();

  using Job = std::function<void(void)>;
  Job GetJob();
  void PutJob(Job job);

  absl::Mutex mutex_;
  // Used to wait for a job's completion.
  absl::CondVar gl_job_done_cv_ ABSL_GUARDED_BY(mutex_);

#if !MEDIAPIPE_DISABLE_PTHREADS
  static void* ThreadBody(void* instance);

  pthread_t gl_thread_id_;
#else
  std::thread gl_thread_;
#endif

  std::deque<Job> jobs_ ABSL_GUARDED_BY(mutex_);
  absl::CondVar has_jobs_cv_ ABSL_GUARDED_BY(mutex_);

  bool self_destruct_ = false;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_GPU_GL_CONTEXT_INTERNAL_H_
