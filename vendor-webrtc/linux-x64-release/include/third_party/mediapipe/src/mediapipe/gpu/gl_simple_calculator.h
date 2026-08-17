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

#ifndef MEDIAPIPE_GPU_GL_SIMPLE_CALCULATOR_H_
#define MEDIAPIPE_GPU_GL_SIMPLE_CALCULATOR_H_

#include <utility>  // for declval

#include "mediapipe/framework/calculator_framework.h"
#include "mediapipe/framework/port/status.h"
#include "mediapipe/gpu/gl_calculator_helper.h"

namespace mediapipe {

// This class saves some boilerplate for the common case of processing one
// input stream of frames and outputting it as one output stream of frames,
// processed using OpenGL.
//
// If you use tags, both input and output streams should be tagged as VIDEO.
// Otherwise, this will use the first input and output.
//
// Subclasses should define at least:
// - GlSetup(), which is called once (on the first frame) and should set up any
//   GL objects the calculator will reuse throughout its life.
// - GlRender(), which is called for each frame.
// - A destructor, to destroy the objects created in GlSetup.
// Note that when GlSetup and GlRender are called, the GL context has already
// been set, but in the destructor it has not. The destructor should use the
// RunInGlContext() helper to make sure it is doing the destruction in the right
// GL context.
//
// Additionally, you can define a GlBind() method, which will be called to
// enable shader programs, bind any additional textures you may need, etc.
// If your calculator shares a context with other calculators, GlBind() will be
// called before each GlRender(); if it has its own context, it will be called
// only once.

class GlSimpleCalculator : public CalculatorBase {
 public:
  GlSimpleCalculator() : initialized_(false) {}
  GlSimpleCalculator(const GlSimpleCalculator&) = delete;
  GlSimpleCalculator& operator=(const GlSimpleCalculator&) = delete;
  ~GlSimpleCalculator() override = default;

  static absl::Status GetContract(CalculatorContract* cc);
  absl::Status Open(CalculatorContext* cc) override;
  absl::Status Process(CalculatorContext* cc) override;
  absl::Status Close(CalculatorContext* cc) override;

  // This method is called once on the first frame. Use it to setup any objects
  // that will be reused throughout the calculator's life.
  virtual absl::Status GlSetup() = 0;

  // You can use this optional method to do any pre-rendering setup that needs
  // to be redone after the context has been used by another calculator.
  // If your context is not shared, it will only be called once.
  virtual absl::Status GlBind() { return absl::OkStatus(); }

  // Do your rendering here. The source and destination textures have already
  // been created and bound for you.
  // - src: source texture (contains input frame); already bound to GL_TEXTURE1.
  // - dst: destination texture (write output frame here); already bound to
  //        GL_TEXTURE0 and attached to the framebuffer.
  virtual absl::Status GlRender(const GlTexture& src, const GlTexture& dst) = 0;

  // The method is called to delete all the programs.
  virtual absl::Status GlTeardown() = 0;

  // You can override this method to compute the size of the destination
  // texture. By default, it will take the same size as the source texture.
  virtual void GetOutputDimensions(int src_width, int src_height,
                                   int* dst_width, int* dst_height) {
    *dst_width = src_width;
    *dst_height = src_height;
  }

  // Override this to output a different type of buffer.
  virtual GpuBufferFormat GetOutputFormat() { return GpuBufferFormat::kBGRA32; }

 protected:
  // Forward invocations of RunInGlContext to the helper.
  // The decltype part just says that this method returns whatever type the
  // helper method invocation returns. In C++14 we could remove it and use
  // return type deduction, i.e.:
  //   template <typename F> auto RunInGlContext(F&& f) { ... }
  template <typename F>
  auto RunInGlContext(F&& f)
      -> decltype(std::declval<GlCalculatorHelper>().RunInGlContext(f)) {
    return helper_.RunInGlContext(std::forward<F>(f));
  }

  GlCalculatorHelper helper_;
  bool initialized_;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_GPU_GL_SIMPLE_CALCULATOR_H_
