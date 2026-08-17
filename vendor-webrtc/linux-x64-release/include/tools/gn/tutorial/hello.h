// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_GN_TUTORIAL_HELLO_H_
#define TOOLS_GN_TUTORIAL_HELLO_H_

void Hello(const char* who);

#if defined(TWO_PEOPLE)
void Hello(const char* one, const char* two);
#endif

#endif  // TOOLS_GN_TUTORIAL_HELLO_H_
