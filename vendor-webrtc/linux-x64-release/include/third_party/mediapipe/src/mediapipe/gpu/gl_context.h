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

#ifndef MEDIAPIPE_GPU_GL_CONTEXT_H_
#define MEDIAPIPE_GPU_GL_CONTEXT_H_

#include <atomic>
#include <cstdint>
#include <functional>
#include <memory>
#include <type_traits>

#include "absl/base/attributes.h"
#include "absl/base/thread_annotations.h"
#include "absl/container/flat_hash_map.h"
#include "absl/log/absl_check.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "absl/synchronization/mutex.h"
#include "mediapipe/framework/executor.h"
#include "mediapipe/framework/mediapipe_profiling.h"
#include "mediapipe/framework/port/status.h"
#include "mediapipe/framework/port/statusor.h"
#include "mediapipe/framework/port/threadpool.h"
#include "mediapipe/framework/timestamp.h"
#include "mediapipe/gpu/attachments.h"
#include "mediapipe/gpu/gl_base.h"
#include "mediapipe/gpu/gpu_buffer_format.h"

#ifdef __APPLE__
#include <CoreVideo/CoreVideo.h>

#include "mediapipe/objc/CFHolder.h"

#if TARGET_OS_OSX

#ifdef __OBJC__
@class NSOpenGLContext;
@class NSOpenGLPixelFormat;
#else
struct NSOpenGLContext;
struct NSOpenGLPixelFormat;
#endif  // __OBJC___

#else

#ifdef __OBJC__
@class EAGLSharegroup;
@class EAGLContext;
#else
struct EAGLSharegroup;
struct EAGLContext;
#endif  // __OBJC___

#endif  // TARGET_OS_OSX

#else

#endif  // __APPLE__

namespace mediapipe {

typedef std::function<void()> GlVoidFunction;
typedef std::function<absl::Status()> GlStatusFunction;

class GlContext;
// TODO: remove after glWaitSync crashes are resolved.
class GlSyncWrapper;

// Generic interface for synchronizing access to a shared resource from a
// different context. This is an abstract class to keep users from
// depending on its contents. The implementation may differ depending on
// the capabilities of the GL context.
class GlSyncPoint {
 public:
  explicit GlSyncPoint(const std::shared_ptr<GlContext>& gl_context)
      : gl_context_(gl_context) {}
  virtual ~GlSyncPoint() {}

  // Waits until the GPU has executed all commands up to the sync point.
  // This blocks the CPU, and ensures the commands are complete from the
  // point of view of all threads and contexts.
  virtual void Wait() = 0;

  // Ensures that the following commands on the current OpenGL context will
  // not be executed until the sync point has been reached.
  // This does not block the CPU, and only affects the current OpenGL context.
  virtual void WaitOnGpu() { Wait(); }

  // Returns whether the sync point has been reached. Does not block.
  virtual bool IsReady() = 0;

  // Returns the GlContext object associated with this sync point, if any.
  const std::shared_ptr<GlContext>& GetContext() { return gl_context_; }

 protected:
  std::shared_ptr<GlContext> gl_context_;
};

// Combines sync points for multiple contexts.
class GlMultiSyncPoint : public GlSyncPoint {
 public:
  GlMultiSyncPoint() : GlSyncPoint(nullptr) {}

  // Adds a new sync to the multisync.
  // If we already have a sync from the same context, overwrite it.
  // Commands on the same context are serialized, and we only care about
  // when the last one is done.
  void Add(std::shared_ptr<GlSyncPoint> new_sync);

  void Wait() override;
  void WaitOnGpu() override;
  bool IsReady() override;

 private:
  std::vector<std::shared_ptr<GlSyncPoint>> syncs_;
};

// TODO: remove.
typedef std::shared_ptr<GlSyncPoint> GlSyncToken;

#if defined(__EMSCRIPTEN__)
typedef EMSCRIPTEN_WEBGL_CONTEXT_HANDLE PlatformGlContext;
constexpr PlatformGlContext kPlatformGlContextNone = 0;
#elif HAS_EGL
typedef EGLContext PlatformGlContext;
constexpr PlatformGlContext kPlatformGlContextNone = EGL_NO_CONTEXT;
#elif HAS_EAGL
typedef EAGLContext* PlatformGlContext;
constexpr PlatformGlContext kPlatformGlContextNone = nil;
#elif HAS_NSGL
typedef NSOpenGLContext* PlatformGlContext;
constexpr PlatformGlContext kPlatformGlContextNone = nil;
#endif  //  defined(__EMSCRIPTEN__)

// This class provides a common API for creating and managing GL contexts.
//
// It handles the following responsibilities:
// - Providing a cross-platform interface over platform-specific APIs like EGL
//   and EAGL.
// - Managing the interaction between threads and GL contexts.
// - Managing synchronization between different GL contexts.
//
class GlContext : public std::enable_shared_from_this<GlContext> {
 public:
  using StatusOrGlContext = absl::StatusOr<std::shared_ptr<GlContext>>;
  // Creates a GlContext.
  //
  // The first argument (which can be a GlContext, or a platform-specific type)
  // indicates a context with which to share resources (e.g. textures).
  // Resources will be shared amongst all contexts linked in this way. You can
  // pass null if sharing is not desired.
  //
  // If create_thread is true, the context will create a thread and run all
  // OpenGL tasks on it.
  static StatusOrGlContext Create(std::nullptr_t nullp, bool create_thread);
  static StatusOrGlContext Create(const GlContext& share_context,
                                  bool create_thread);
  static StatusOrGlContext Create(PlatformGlContext share_context,
                                  bool create_thread);
#if HAS_EAGL
  static StatusOrGlContext Create(EAGLSharegroup* sharegroup,
                                  bool create_thread);
#endif  // HAS_EAGL

  // Returns the GlContext that is current on this thread. May return nullptr.
  static std::shared_ptr<GlContext> GetCurrent();

  GlContext(const GlContext&) = delete;
  GlContext& operator=(const GlContext&) = delete;
  ~GlContext();

  // Initializes this GlContext with the graph tracing and profiling interface.
  // Also initializes the GlProfilingHelper object for this GlContext if the
  // GlProfilingHelper is uninitialized. This ensures that the GlProfilingHelper
  // is unique to and only initialized once per GlContext object.
  void SetProfilingContext(
      std::shared_ptr<mediapipe::ProfilingContext> profiling_context);

  // Executes a function in the GL context. Waits for the
  // function's execution to be complete before returning to the caller.
  absl::Status Run(GlStatusFunction gl_func, int node_id = -1,
                   Timestamp input_timestamp = Timestamp::Unset());

  // Like Run, but does not wait.
  void RunWithoutWaiting(GlVoidFunction gl_func);

  // Returns a synchronization token for this GlContext.
  std::shared_ptr<GlSyncPoint> CreateSyncToken();

  // If another part of the framework calls glFinish, it should call this
  // method to let the context know that it has done so. The context can use
  // that information to avoid inserting additional glFinish calls in some
  // cases.
  void GlFinishCalled();

  // Ensures that the changes to shared resources covered by the token are
  // visible in the current context.
  // This should only be called outside a job.
  void WaitSyncToken(const std::shared_ptr<GlSyncPoint>& token);

  // Checks whether the token's sync point has been reached. Returns true
  // iff WaitSyncToken would not have to wait.
  // This is thread-safe.
  bool SyncTokenIsReady(const std::shared_ptr<GlSyncPoint>& token);

#if defined(__EMSCRIPTEN__)
  // Returns the EMSCRIPTEN_WEBGL_CONTEXT_HANDLE for our context.
  EMSCRIPTEN_WEBGL_CONTEXT_HANDLE webgl_context() const { return context_; }
  EmscriptenWebGLContextAttributes webgl_attributes() const { return attrs_; }
#elif HAS_EGL
  // Returns the EGLDisplay used by our context.
  EGLDisplay egl_display() const { return display_; }

  // Returns the EGLConfig used to create our context.
  EGLConfig egl_config() const { return config_; }

  // Returns our EGLContext.
  EGLContext egl_context() const { return context_; }
#elif HAS_EAGL
  EAGLContext* eagl_context() const { return context_; }
  CVOpenGLESTextureCacheRef cv_texture_cache() const { return *texture_cache_; }
#elif HAS_NSGL
  NSOpenGLContext* nsgl_context() const { return context_; }
  NSOpenGLPixelFormat* nsgl_pixel_format() const { return pixel_format_; }
  CVOpenGLTextureCacheRef cv_texture_cache() const { return *texture_cache_; }
#endif  // HAS_EGL

  // Returns whatever the current platform's native context handle is.
  // Prefer the explicit *_context methods above, unless you're going to use
  // this in a context that you are sure will work with whatever definition of
  // PlatformGlContext is in use.
  PlatformGlContext native_context() const { return context_; }

  // Check if the context is current on this thread. Mainly for test purposes.
  bool IsCurrent() const;

  GLint gl_major_version() const { return gl_major_version_; }
  GLint gl_minor_version() const { return gl_minor_version_; }

  static bool ParseGlVersion(absl::string_view version_string, GLint* major,
                             GLint* minor);

  // Returns a GlVersion code used with GpuBufferFormat.
  // TODO: make this more generally applicable.
  GlVersion GetGlVersion() const;

  // Simple query for GL extension support; only valid after GlContext has
  // finished its initialization successfully.
  bool HasGlExtension(absl::string_view extension) const;

  int64_t gl_finish_count() { return gl_finish_count_; }

  // Used by GlFinishSyncPoint. The count_to_pass cannot exceed the current
  // gl_finish_count_ (but it can be equal).
  void WaitForGlFinishCountPast(int64_t count_to_pass);

  // Convenience version of Run for arguments with a void result type.
  // Waits for the function to finish executing before returning.
  //
  // Implementation note: we cannot use a std::function<void(void)> argument
  // here, because that would break passing in a lambda that returns a status;
  // e.g.:
  //   RunInGlContext([]() -> absl::Status { ... });
  //
  // The reason is that std::function<void(...)> allows the implicit conversion
  // of a callable with any result type, as long as the argument types match.
  // As a result, the above lambda would be implicitly convertible to both
  // std::function<absl::Status(void)> and std::function<void(void)>, and
  // the invocation would be ambiguous.
  //
  // Therefore, instead of using std::function<void(void)>, we use a template
  // that only accepts arguments with a void result type.
  template <typename T,
            typename = typename std::enable_if<std::is_void<
                typename std::invoke_result<T>::type>::value>::type>
  void Run(T f) {
    Run([f] {
      f();
      return absl::OkStatus();
    }).IgnoreError();
  }

  // Sets default texture filtering parameters.
  void SetStandardTextureParams(GLenum target, GLint internal_format);

  using AttachmentBase = internal::AttachmentBase<GlContext>;
  template <class T>
  using Attachment = internal::Attachment<GlContext, T>;

  // TOOD: const result?
  template <class T>
  T& GetCachedAttachment(const Attachment<T>& attachment) {
    ABSL_DCHECK(IsCurrent());
    internal::AttachmentPtr<void>& entry = attachments_[&attachment];
    if (entry == nullptr) {
      entry = attachment.factory()(*this);
    }
    return *static_cast<T*>(entry.get());
  }

  // Returns true if any GL context, including external contexts not managed by
  // the GlContext class, is current.
  static bool IsAnyContextCurrent();

  // Returns the current native context, whether managed by this class or not.
  // Useful as a cross-platform way to get the current PlatformGlContext.
  static PlatformGlContext GetCurrentNativeContext();

  // Creates a synchronization token for the current, non-GlContext-owned
  // context. This can be passed to MediaPipe so it can synchronize with the
  // commands issued in the external context up to this point.
  // Note: if the current context does not support sync fences, this calls
  // glFinish and returns nullptr.
  // TODO: return GlNopSyncPoint instead?
  static std::shared_ptr<GlSyncPoint> CreateSyncTokenForCurrentExternalContext(
      const std::shared_ptr<GlContext>& delegate_graph_context);

  // These are used for testing specific SyncToken implementations. Do not use
  // outside of tests.
  enum class SyncTokenTypeForTest {
    kGlFinish,
  };
  std::shared_ptr<GlSyncPoint> TestOnly_CreateSpecificSyncToken(
      SyncTokenTypeForTest type);

 private:
  // TODO: remove after glWaitSync crashes are resolved.
  friend GlSyncWrapper;

  GlContext();

  bool ShouldUseFenceSync() const;

#if defined(__EMSCRIPTEN__)
  absl::Status CreateContext(EMSCRIPTEN_WEBGL_CONTEXT_HANDLE share_context);
  absl::Status CreateContextInternal(
      EMSCRIPTEN_WEBGL_CONTEXT_HANDLE share_context, int webgl_version);

  EMSCRIPTEN_WEBGL_CONTEXT_HANDLE context_ = 0;
  EmscriptenWebGLContextAttributes attrs_;
#elif HAS_EGL
  absl::Status CreateContext(EGLContext share_context);
  absl::Status CreateContextInternal(EGLContext share_context, int gl_version);

  EGLDisplay display_ = EGL_NO_DISPLAY;
  EGLConfig config_;
  EGLSurface surface_ = EGL_NO_SURFACE;
  EGLContext context_ = EGL_NO_CONTEXT;
#elif HAS_EAGL
  absl::Status CreateContext(EAGLSharegroup* sharegroup);

  EAGLContext* context_;
  CFHolder<CVOpenGLESTextureCacheRef> texture_cache_;
#elif HAS_NSGL
  absl::Status CreateContext(NSOpenGLContext* share_context);

  NSOpenGLContext* context_;
  NSOpenGLPixelFormat* pixel_format_;
  CFHolder<CVOpenGLTextureCacheRef> texture_cache_;
#endif  // defined(__EMSCRIPTEN__)

  class DedicatedThread;

  // A context binding represents the minimal set of information needed to make
  // a context current on a thread. Its contents depend on the platform.
  struct ContextBinding {
    // The context_object is null if this binding refers to a context not
    // managed by GlContext.
    std::weak_ptr<GlContext> context_object;
#if defined(__EMSCRIPTEN__)
    EMSCRIPTEN_WEBGL_CONTEXT_HANDLE context = 0;
#elif HAS_EGL
    EGLDisplay display = EGL_NO_DISPLAY;
    EGLSurface draw_surface = EGL_NO_SURFACE;
    EGLSurface read_surface = EGL_NO_SURFACE;
    EGLContext context = EGL_NO_CONTEXT;
#elif HAS_EAGL
    EAGLContext* context = nullptr;
#elif HAS_NSGL
    NSOpenGLContext* context = nullptr;
#endif  // HAS_EGL
  };

  absl::Status FinishInitialization(bool create_thread);

  // This wraps a thread_local.
  static std::weak_ptr<GlContext>& CurrentContext();

  static absl::Status SwitchContext(ContextBinding* saved_context,
                                    const ContextBinding& new_context);

  absl::Status EnterContext(ContextBinding* saved_context);
  absl::Status ExitContext(const ContextBinding* saved_context);
  void DestroyContext();

  bool HasContext() const;

  // This function clears out any tripped gl Errors and just logs them. This
  // is used by code that needs to check glGetError() to know if it succeeded,
  // but can't rely on the existing state to be 'clean'.
  void ForceClearExistingGlErrors();

  // Returns true if there were any GL errors. Note that this may be a no-op
  // for performance reasons in some contexts (specifically Emscripten opt).
  bool CheckForGlErrors();

  // Same as `CheckForGLErrors()` but with the option of forcing the check
  // even if we would otherwise skip for performance reasons.
  bool CheckForGlErrors(bool force);

  void LogUncheckedGlErrors(bool had_gl_errors);
  absl::Status GetGlExtensions();
  absl::Status GetGlExtensionsCompat();

  // Make the context current, run gl_func, and restore the previous context.
  // Internal helper only; callers should use Run or RunWithoutWaiting instead,
  // which delegates to the dedicated thread if required.
  absl::Status SwitchContextAndRun(GlStatusFunction gl_func);

  // The following ContextBinding functions have platform-specific
  // implementations.

  // A binding that can be used to make this GlContext current.
  ContextBinding ThisContextBinding();
  // Fill in platform-specific fields. Must _not_ set context_obj.
  ContextBinding ThisContextBindingPlatform();
  // Fills in a ContextBinding with platform-specific information about which
  // context is current on this thread.
  static void GetCurrentContextBinding(ContextBinding* binding);
  // Makes the context described by new_context current on this thread.
  static absl::Status SetCurrentContextBinding(
      const ContextBinding& new_binding);

  // If not null, a dedicated thread used to execute tasks on this context.
  // Used on Android due to expensive context switching on some configurations.
  std::unique_ptr<DedicatedThread> thread_;

  GLint gl_major_version_ = 0;
  GLint gl_minor_version_ = 0;

  // glGetString and glGetStringi both return pointers to static strings,
  // so we should be fine storing the extension pieces as string_view's.
  std::set<absl::string_view> gl_extensions_;

  // Used by SetStandardTextureParams. Do we want several of these bools, or a
  // better mechanism?
  bool can_linear_filter_float_textures_;

  absl::flat_hash_map<const AttachmentBase*, internal::AttachmentPtr<void>>
      attachments_;

  // Number of glFinish calls completed on the GL thread.
  // Changes should be guarded by mutex_. However, we use simple atomic
  // loads for efficiency on the fast path.
  std::atomic<int64_t> gl_finish_count_ = 0;
  std::atomic<int64_t> gl_finish_count_target_ = 0;

  GlContext* context_waiting_on_ ABSL_GUARDED_BY(mutex_) = nullptr;

  // This mutex is held by a thread while this GL context is current on that
  // thread. Since it may be held for extended periods of time, it should not
  // be used for other pieces of status.
  absl::Mutex context_use_mutex_;

  // This mutex is used to guard a few different members and condition
  // variables. It should only be held for a short time.
  absl::Mutex mutex_;
  absl::CondVar wait_for_gl_finish_cv_ ABSL_GUARDED_BY(mutex_);

  std::unique_ptr<mediapipe::GlProfilingHelper> profiling_helper_ = nullptr;

  bool destructing_ = false;
};

// A framebuffer that the framework can use to attach textures for rendering
// etc.
// This could just be a member of GlContext, but it serves as a basic example
// of an attachment.
ABSL_CONST_INIT extern const GlContext::Attachment<GLuint> kUtilityFramebuffer;

// For backward compatibility. TODO: migrate remaining callers.
ABSL_DEPRECATED(
    "Prefer passing an explicit GlVersion argument (use "
    "GlContext::GetGlVersion)")
const GlTextureInfo& GlTextureInfoForGpuBufferFormat(GpuBufferFormat format,
                                                     int plane);

namespace internal_gl_context {

struct OpenGlVersion {
  int major;
  int minor;
};

bool IsOpenGlVersionSameOrAbove(const OpenGlVersion& version,
                                const OpenGlVersion& expected_version);

}  // namespace internal_gl_context

}  // namespace mediapipe

#endif  // MEDIAPIPE_GPU_GL_CONTEXT_H_
