// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_TEST_FAKE_GLES2_INTERFACE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_TEST_FAKE_GLES2_INTERFACE_H_

#include "gpu/command_buffer/client/gles2_interface_stub.h"

class FakeGLES2Interface : public gpu::gles2::GLES2InterfaceStub {
 public:
  // GLES2Interface implementation.
  GLenum GetGraphicsResetStatusKHR() override {
    return context_lost_ ? GL_INVALID_OPERATION : GL_NO_ERROR;
  }

  // Methods for tests.
  void SetIsContextLost(bool lost) { context_lost_ = lost; }

 private:
  bool context_lost_ = false;
};

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_TEST_FAKE_GLES2_INTERFACE_H_
