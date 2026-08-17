// Copyright 2020 The MediaPipe Authors.
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

#ifndef MEDIAPIPE_FRAMEWORK_FORMATS_TENSOR_H_
#define MEDIAPIPE_FRAMEWORK_FORMATS_TENSOR_H_

#include <algorithm>
#include <cstdint>
#include <functional>
#include <initializer_list>
#include <memory>
#include <numeric>
#include <tuple>
#include <type_traits>
#include <utility>
#include <vector>

#include "absl/functional/any_invocable.h"
#include "absl/status/status.h"
#include "absl/synchronization/mutex.h"
#include "mediapipe/framework/formats/tensor/internal.h"
#include "mediapipe/framework/memory_manager.h"
// Exports MEDIAPIPE_TENSOR_USE_AHWB macro.
#include "mediapipe/framework/port.h"

#ifdef MEDIAPIPE_TENSOR_USE_AHWB
#include <EGL/egl.h>
#include <EGL/eglext.h>

#include <list>

#include "absl/log/absl_check.h"
#include "mediapipe/framework/formats/hardware_buffer.h"
#include "mediapipe/framework/formats/hardware_buffer_pool.h"
#include "mediapipe/framework/formats/tensor_ahwb_usage.h"
#include "mediapipe/framework/formats/unique_fd.h"
#endif  // MEDIAPIPE_TENSOR_USE_AHWB
#if MEDIAPIPE_OPENGL_ES_VERSION >= MEDIAPIPE_OPENGL_ES_30
#include "mediapipe/gpu/gl_base.h"
#include "mediapipe/gpu/gl_context.h"
#endif  // MEDIAPIPE_OPENGL_ES_VERSION >= MEDIAPIPE_OPENGL_ES_30

#if defined __has_builtin
#if __has_builtin(__builtin_LINE)
#define builtin_LINE __builtin_LINE
#endif
#if __has_builtin(__builtin_FILE)
#define builtin_FILE __builtin_FILE
#endif
#endif

#ifndef builtin_LINE
#define builtin_LINE() 0
#endif
#ifndef builtin_FILE
#define builtin_FILE() ""
#endif

#include "mediapipe/gpu/webgpu/webgpu_check.h"
#if MEDIAPIPE_USE_WEBGPU
#include <webgpu/webgpu_cpp.h>

#include "mediapipe/gpu/webgpu/webgpu_service.h"
#endif  // MEDIAPIPE_USE_WEBGPU

namespace mediapipe {
// Tensor is a container of multi-dimensional data that supports sharing the
// content across different backends and APIs, currently: CPU / Metal / OpenGL.
// Texture2DView is limited to 4 dimensions.
// The content is accessible through requesting device specific views.
// Acquiring a view guarantees that the content is not changed by another thread
// until the view is released.
//
// Tensor::MtlBufferView view = tensor.GetMtlBufferWriteView(mtl_device);
// mtl_device is used to create MTLBuffer
// id<MTLBuffer> buffer = view.buffer();
// For OpenGL the code below must be called by a thread with valid OpenGL ES
// context bound:
// GLuint buffer = view.buffer();
// Then the buffer can be bound to the GPU command buffer.
// ...binding the buffer to the command buffer...
// ...committing command buffer and releasing the view...
//
// The following request for the CPU view will be blocked until the GPU view is
// released and the GPU task is finished.
//
// auto view = tensor.GetCpuReadView();
// float* pointer = view.buffer<float>();
// ...reading the cpu memory...

struct MtlResources;
class Tensor {
  class View {
   public:
    // Non-copyable.
    View(const View&) = delete;
    View& operator=(const View&) = delete;

   protected:
    explicit View(std::unique_ptr<absl::MutexLock>&& lock)
        : lock_(std::move(lock)) {}
    std::unique_ptr<absl::MutexLock> lock_;
  };

 public:
  // No resources are allocated here.
  enum class ElementType {
    kNone,
    kFloat16,
    kFloat32,
    kUInt8,
    kInt8,
    kInt32,
    kInt64,
    kChar,
    kBool
  };
  struct Shape {
    Shape() = default;
    Shape(std::initializer_list<int> dimensions) : dims(dimensions) {}
    Shape(const std::vector<int>& dimensions) : dims(dimensions) {}
    Shape(std::initializer_list<int> dimensions, bool is_dynamic)
        : dims(dimensions), is_dynamic(is_dynamic) {}
    Shape(const std::vector<int>& dimensions, bool is_dynamic)
        : dims(dimensions), is_dynamic(is_dynamic) {}
    int num_elements() const {
      return std::accumulate(dims.begin(), dims.end(), 1,
                             std::multiplies<int>());
    }
    std::vector<int> dims;
    // The Tensor has dynamic rather than static shape so the TFLite interpreter
    // needs to be reallocated. Only relevant for CPU.
    bool is_dynamic = false;
  };
  // Quantization parameters corresponding to the zero_point and scale value
  // made available by TfLite quantized (uint8/int8) tensors.
  struct QuantizationParameters {
    QuantizationParameters() = default;
    QuantizationParameters(float scale, int zero_point)
        : scale(scale), zero_point(zero_point) {}
    float scale = 1.0f;
    int zero_point = 0;
  };

  // memory_alignment is an optional argument to tell the API to allocate
  // a buffer that is padded to multiples of memory_alignment bytes.
  // memory_alignment must be power of 2, i.e. 2, 4, 8, 16, 64, etc.
  // If memory_alignment is 0, then the buffer will not be padded.
  // Note that memory_alignment is only applied to CPU storage (includes AHWBs).
  Tensor(ElementType element_type, const Shape& shape,
         MemoryManager* memory_manager = nullptr, int memory_alignment = 0);
  Tensor(ElementType element_type, const Shape& shape,
         const QuantizationParameters& quantization_parameters,
         MemoryManager* memory_manager = nullptr, int memory_alignment = 0);

  // Non-copyable.
  Tensor(const Tensor&) = delete;
  Tensor& operator=(const Tensor&) = delete;
  // Move-only.
  Tensor(Tensor&& src);
  Tensor& operator=(Tensor&&);
  ~Tensor();

  template <typename T>
  class CpuView : public View {
   public:
    template <typename P>
    auto buffer() const {
      // const and non-const return  type selection.
      return static_cast<typename std::tuple_element<
          std::is_const<T>::value, std::tuple<P*, const P*>>::type>(buffer_);
    }
    CpuView(CpuView&& src) : View(std::move(src.lock_)) {
      buffer_ = std::exchange(src.buffer_, nullptr);
      release_callback_ = std::exchange(src.release_callback_, nullptr);
    }
    ~CpuView() {
      if (release_callback_) release_callback_();
    }

   protected:
    friend class Tensor;
    CpuView(T* buffer, std::unique_ptr<absl::MutexLock>&& lock,
            absl::AnyInvocable<void()> release_callback = nullptr)
        : View(std::move(lock)),
          buffer_(buffer),
          release_callback_(std::move(release_callback)) {}
    T* buffer_;
    absl::AnyInvocable<void()> release_callback_;
  };
  using CpuReadView = CpuView<const void>;
  CpuReadView GetCpuReadView() const;
  using CpuWriteView = CpuView<void>;
  CpuWriteView GetCpuWriteView(
      uint64_t source_location_hash =
          tensor_internal::FnvHash64(builtin_FILE(), builtin_LINE())) const;

#ifdef MEDIAPIPE_TENSOR_USE_AHWB
  using FinishingFunc = std::function<bool(bool)>;

  class AHardwareBufferView : public View {
   public:
    // Returns the AHardwareBuffer handle. Note that writes to the handle may be
    // pending. To enable synchronized read access, a fence FD can be obtained
    // from GetWriteCompleteFenceFd().
    AHardwareBuffer* handle() const {
      return hardware_buffer_->GetAHardwareBuffer();
    }
    AHardwareBufferView(AHardwareBufferView&& src)
        : View(std::move(src.lock_)) {
      hardware_buffer_ = std::move(src.hardware_buffer_);
      write_complete_fence_fd_ =
          std::exchange(src.write_complete_fence_fd_, nullptr);
      ahwb_usage_ = std::exchange(src.ahwb_usage_, nullptr);
      is_write_view_ = src.is_write_view_;
    }

    // Returns a file descriptor fence that signals the end of a pending write
    // operation.
    // Note that the provided file descriptor is valid only during the lifetime
    // of the view and must be duplicated if used outside of the view.
    int GetWriteCompleteFenceFd() const {
      ABSL_CHECK(!is_write_view_)
          << "AHWB write view can't return write complete fence FD'";
      return write_complete_fence_fd_->Get();
    }

    // TODO: verify if multiple functions can be specified.
    void SetReadingFinishedFunc(FinishingFunc&& func) {
      ABSL_CHECK(!is_write_view_)
          << "AHWB write view can't accept 'reading finished callback'";
      ABSL_CHECK(ahwb_usage_->is_complete_fn == nullptr)
          << "AHWB reading finished callback is already set.";
      ahwb_usage_->is_complete_fn = std::move(func);
    }

    // TODO: verify if multiple functions can be specified.
    void SetWritingFinishedFD(int fd, FinishingFunc func = nullptr) {
      ABSL_CHECK(is_write_view_)
          << "AHWB read view can't accept 'writing finished file descriptor'";
      ABSL_CHECK(!write_complete_fence_fd_->IsValid())
          << "AHWB write complete fence FD is already set.";
      ABSL_CHECK(ahwb_usage_->is_complete_fn == nullptr)
          << "AHWB write finished callback is already set.";
      *write_complete_fence_fd_ = UniqueFd(fd);
      ahwb_usage_->is_complete_fn = std::move(func);
    }

    // Passed `callback` is invoked when the tensor is being released.
    // TODO: rename to Add* or set a single callback only.
    void SetReleaseCallback(absl::AnyInvocable<void()> callback) {
      ahwb_usage_->release_callbacks.push_back(std::move(callback));
    }

   protected:
    friend class Tensor;
    AHardwareBufferView(HardwareBuffer* hardware_buffer,
                        UniqueFd* write_complete_fence_fd,
                        TensorAhwbUsage* ahwb_usage,
                        std::unique_ptr<absl::MutexLock>&& lock,
                        bool is_write_view)
        : View(std::move(lock)),
          hardware_buffer_(hardware_buffer),
          write_complete_fence_fd_(write_complete_fence_fd),
          ahwb_usage_(ahwb_usage),
          is_write_view_(is_write_view) {}

    HardwareBuffer* hardware_buffer_ = nullptr;
    UniqueFd* write_complete_fence_fd_ = nullptr;
    TensorAhwbUsage* ahwb_usage_ = nullptr;
    bool is_write_view_ = false;
  };

  AHardwareBufferView GetAHardwareBufferReadView() const;
  AHardwareBufferView GetAHardwareBufferWriteView() const;
#endif  // MEDIAPIPE_TENSOR_USE_AHWB

#if MEDIAPIPE_OPENGL_ES_VERSION >= MEDIAPIPE_OPENGL_ES_30
  // TODO: Use GlTextureView instead.
  // Only float32 textures are supported with 1/2/3/4 depths.
  // OpenGlTexture2dView currently only supports BHWC memory layout.
  class OpenGlTexture2dView : public View {
   public:
    GLuint name() const { return name_; }
    OpenGlTexture2dView(OpenGlTexture2dView&& src)
        : View(std::move(src.lock_)) {
      name_ = std::exchange(src.name_, GL_INVALID_INDEX);
    }
    // To fit a tensor into a texture two layouts are used:
    // 1. Aligned. Width of the texture = tensor_width * num_slices, where slice
    //    is a group of 4 depth values. Tensor depth is padded to 4.
    // 2. Linearized. If texture width or height with the layout 1. is greater
    //    than the GPU supports then all tensor values are packed into a texture
    //    with fixed width calculated by this method.
    // Must be called with the valid GL context bound to the current thread.
    enum class Layout { kAligned, kLinearized };
    static Layout GetLayoutDimensions(const Tensor::Shape& shape, int* width,
                                      int* height);

   protected:
    friend class Tensor;
    OpenGlTexture2dView(GLuint name, std::unique_ptr<absl::MutexLock>&& lock)
        : View(std::move(lock)), name_(name) {}
    GLuint name_;
  };
  // A valid OpenGL context must be bound to the calling thread due to possible
  // GPU resource allocation.
  OpenGlTexture2dView GetOpenGlTexture2dReadView() const;
  OpenGlTexture2dView GetOpenGlTexture2dWriteView() const;
#endif  // MEDIAPIPE_OPENGL_ES_VERSION >= MEDIAPIPE_OPENGL_ES_30

#if MEDIAPIPE_USE_WEBGPU
  class WebGpuTexture2dView : public View {
   public:
    WebGpuTexture2dView(WebGpuTexture2dView&& src)
        : View(std::move(src.lock_)) {  // Only moves the View portion of src.
      name_ = std::exchange(src.name_, nullptr);
    }

    wgpu::Texture name() const { return name_; }

   protected:
    friend class Tensor;

    WebGpuTexture2dView(wgpu::Texture name,
                        std::unique_ptr<absl::MutexLock>&& lock)
        : View(std::move(lock)), name_(name) {}

    wgpu::Texture name_;
  };

  WebGpuTexture2dView GetWebGpuTexture2dReadView(
      const WebGpuService& service) const;
  WebGpuTexture2dView GetWebGpuTexture2dWriteView(
      const WebGpuService& service) const;
#endif  // MEDIAPIPE_USE_WEBGPU

#if MEDIAPIPE_OPENGL_ES_VERSION >= MEDIAPIPE_OPENGL_ES_31
  class OpenGlBufferView : public View {
   public:
    GLuint name() const { return name_; }

    OpenGlBufferView(OpenGlBufferView&& src) : View(std::move(src.lock_)) {
      name_ = std::exchange(src.name_, GL_INVALID_INDEX);
      ssbo_read_ = std::exchange(src.ssbo_read_, nullptr);
    }
    ~OpenGlBufferView() {
      if (ssbo_read_) {
        // TODO: update tensor to properly handle cases when
        // multiple views were requested multiple sync fence may be needed.
        *ssbo_read_ = glFenceSync(GL_SYNC_GPU_COMMANDS_COMPLETE, 0);
      }
    }

   protected:
    friend class Tensor;

    OpenGlBufferView(GLuint name, std::unique_ptr<absl::MutexLock>&& lock,
                     GLsync* ssbo_read)
        : View(std::move(lock)), name_(name), ssbo_read_(ssbo_read) {}
    GLuint name_;
    GLsync* ssbo_read_;
  };
  // A valid OpenGL context must be bound to the calling thread due to possible
  // GPU resource allocation.
  OpenGlBufferView GetOpenGlBufferReadView() const;
  OpenGlBufferView GetOpenGlBufferWriteView(
      uint64_t source_location_hash =
          tensor_internal::FnvHash64(builtin_FILE(), builtin_LINE())) const;
#endif  // MEDIAPIPE_OPENGL_ES_VERSION >= MEDIAPIPE_OPENGL_ES_31

  const Shape& shape() const { return shape_; }
  ElementType element_type() const { return element_type_; }
  const QuantizationParameters& quantization_parameters() const {
    return quantization_parameters_;
  }
  int element_size() const {
    switch (element_type_) {
      case ElementType::kNone:
        return 0;
      case ElementType::kFloat16:
        return 2;
      case ElementType::kFloat32:
        return sizeof(float);
      case ElementType::kUInt8:
        return 1;
      case ElementType::kInt8:
        return 1;
      case ElementType::kInt32:
        return sizeof(int32_t);
      case ElementType::kInt64:
        return sizeof(int64_t);
      case ElementType::kChar:
        return sizeof(char);
      case ElementType::kBool:
        return sizeof(bool);
    }
  }
  int bytes() const { return shape_.num_elements() * element_size(); }

  bool ready_on_cpu() const {
    return valid_ & (kValidAHardwareBuffer | kValidCpu);
  }
  bool ready_on_gpu() const {
    return valid_ &
           (kValidMetalBuffer | kValidOpenGlBuffer | kValidWebGpuTexture2d |
            kValidAHardwareBuffer | kValidOpenGlTexture2d);
  }
  bool ready_as_metal_buffer() const { return valid_ & kValidMetalBuffer; }
  bool ready_as_opengl_buffer() const {
    return valid_ & (kValidAHardwareBuffer | kValidOpenGlBuffer);
  }
  bool ready_as_opengl_texture_2d() const {
    return valid_ & kValidOpenGlTexture2d;
  }
  bool ready_as_ahwb() const { return use_ahwb_; }
  bool ready_as_webgpu_texture_2d() const {
    return valid_ & kValidWebGpuTexture2d;
  }

 private:
  friend class MtlBufferView;
  void Move(Tensor*);
  void Invalidate();
  absl::Status ReadBackGpuToCpu() const;

  ElementType element_type_;
  Shape shape_;
  QuantizationParameters quantization_parameters_;
  int memory_alignment_ = 0;

  // The flags describe the current source of truth resource type.
  enum {
    kValidNone = 0,
    kValidCpu = 1 << 0,
    kValidMetalBuffer = 1 << 1,
    kValidOpenGlBuffer = 1 << 2,
    kValidOpenGlTexture2d = 1 << 3,
    kValidWebGpuTexture2d = 1 << 4,
    kValidAHardwareBuffer = 1 << 5,
  };
  // A list of resource which are currently allocated and synchronized between
  // each-other: valid_ = kValidCpu | kValidMetalBuffer;
  mutable int valid_ = 0;
  // The mutex is locked by Get*View and is kept by all Views.
  mutable absl::Mutex view_mutex_;

  mutable void* cpu_buffer_ = nullptr;
  absl::Status AllocateCpuBuffer() const;
  void FreeCpuBuffer() const;
  // Forward declaration of the MtlResources provides compile-time verification
  // of ODR if this header includes any actual code that uses MtlResources.
  mutable std::unique_ptr<MtlResources> mtl_resources_;

#if MEDIAPIPE_USE_WEBGPU
  mutable wgpu::Device webgpu_device_;
  mutable wgpu::Texture webgpu_texture2d_;
#endif  // MEDIAPIPE_USE_WEBGPU
#ifdef MEDIAPIPE_TENSOR_USE_AHWB
  mutable std::shared_ptr<HardwareBuffer> ahwb_;

  // Allocates and pools HardwareBuffer instances. Holding the shared_ptr to the
  // pool ensures it outlives the internal ahwb_.
  std::shared_ptr<HardwareBufferPool> hardware_buffer_pool_;

  // Signals when GPU finished writing into SSBO so AHWB can be used then. Or
  // signals when writing into AHWB has been finished so GPU can read from SSBO.
  // Sync and FD are bound together.
  mutable EGLSyncKHR fence_sync_ = EGL_NO_SYNC_KHR;

  // Filehandle to signal when the writing into the AHWB has been finished.
  mutable UniqueFd write_complete_fence_fd_;

  // Reading from SSBO has been finished so SSBO can be released.
  mutable GLsync ssbo_read_ = 0;

  // Keeps track of current AHWB usages (e.g. multiple reads - two inference
  // calculators use the same input tensor and import buffer by FD which results
  // in two buffer handles that must be released.)
  mutable std::list<TensorAhwbUsage> ahwb_usages_;

  absl::Status AllocateAHardwareBuffer() const;

  void CreateEglSyncAndFd() const;
#endif  // MEDIAPIPE_TENSOR_USE_AHWB

  // Use Ahwb for other views: OpenGL / CPU buffer.
  mutable bool use_ahwb_ = false;
  mutable uint64_t ahwb_tracking_key_ = 0;
  // Expects the target SSBO to be already bound.
  bool AllocateAhwbMapToSsbo() const;
  bool InsertAhwbToSsboFence() const;
  void MoveAhwbStuff(Tensor* src);
  void ReleaseAhwbStuff();
  void* MapAhwbToCpuRead() const;
  void* MapAhwbToCpuWrite() const;
  void MoveCpuOrSsboToAhwb() const;
  // Set current tracking key, set "use ahwb" if the key is already marked.
  void TrackAhwbUsage(uint64_t key) const;

#if MEDIAPIPE_OPENGL_ES_VERSION >= MEDIAPIPE_OPENGL_ES_30
  mutable std::shared_ptr<mediapipe::GlContext> gl_context_;
  mutable GLuint opengl_texture2d_ = GL_INVALID_INDEX;
  mutable GLuint frame_buffer_ = GL_INVALID_INDEX;
  mutable int texture_width_;
  mutable int texture_height_;
#ifdef __EMSCRIPTEN__
  mutable bool texture_is_half_float_ = false;
#endif  // __EMSCRIPTEN__
  void AllocateOpenGlTexture2d() const;
#if MEDIAPIPE_OPENGL_ES_VERSION >= MEDIAPIPE_OPENGL_ES_31
  mutable GLuint opengl_buffer_ = GL_INVALID_INDEX;
  void AllocateOpenGlBuffer() const;
#endif  // MEDIAPIPE_OPENGL_ES_VERSION >= MEDIAPIPE_OPENGL_ES_31
  bool NeedsHalfFloatRenderTarget() const;
#endif  // MEDIAPIPE_OPENGL_ES_VERSION >= MEDIAPIPE_OPENGL_ES_30
};

int BhwcBatchFromShape(const Tensor::Shape& shape);
int BhwcHeightFromShape(const Tensor::Shape& shape);
int BhwcWidthFromShape(const Tensor::Shape& shape);
int BhwcDepthFromShape(const Tensor::Shape& shape);

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_FORMATS_TENSOR_H_
