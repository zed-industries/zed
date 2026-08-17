// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_CARDBOARD_CONFIG_OPENGL_ES2_CUSTOM_BINDINGS_H_
#define THIRD_PARTY_CARDBOARD_CONFIG_OPENGL_ES2_CUSTOM_BINDINGS_H_

#include "ui/gl/gl_bindings.h"

#define glBindFramebuffer glBindFramebufferEXT
#define glDeleteBuffers glDeleteBuffersARB
#define glGenBuffers glGenBuffersARB

#endif  // THIRD_PARTY_CARDBOARD_CONFIG_OPENGL_ES2_CUSTOM_BINDINGS_H_
