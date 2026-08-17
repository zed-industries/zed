/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_WIN_SCOPED_GDI_HANDLE_H_
#define MODULES_DESKTOP_CAPTURE_WIN_SCOPED_GDI_HANDLE_H_

#include <windows.h>

namespace webrtc {
namespace win {

// Scoper for GDI objects.
template <class T, class Traits>
class ScopedGDIObject {
 public:
  ScopedGDIObject() : handle_(NULL) {}
  explicit ScopedGDIObject(T object) : handle_(object) {}

  ~ScopedGDIObject() { Traits::Close(handle_); }

  ScopedGDIObject(const ScopedGDIObject&) = delete;
  ScopedGDIObject& operator=(const ScopedGDIObject&) = delete;

  T Get() { return handle_; }

  void Set(T object) {
    if (handle_ && object != handle_)
      Traits::Close(handle_);
    handle_ = object;
  }

  ScopedGDIObject& operator=(T object) {
    Set(object);
    return *this;
  }

  T release() {
    T object = handle_;
    handle_ = NULL;
    return object;
  }

  operator T() { return handle_; }

 private:
  T handle_;
};

// The traits class that uses DeleteObject() to close a handle.
template <typename T>
class DeleteObjectTraits {
 public:
  DeleteObjectTraits() = delete;
  DeleteObjectTraits(const DeleteObjectTraits&) = delete;
  DeleteObjectTraits& operator=(const DeleteObjectTraits&) = delete;

  // Closes the handle.
  static void Close(T handle) {
    if (handle)
      DeleteObject(handle);
  }
};

// The traits class that uses DestroyCursor() to close a handle.
class DestroyCursorTraits {
 public:
  DestroyCursorTraits() = delete;
  DestroyCursorTraits(const DestroyCursorTraits&) = delete;
  DestroyCursorTraits& operator=(const DestroyCursorTraits&) = delete;

  // Closes the handle.
  static void Close(HCURSOR handle) {
    if (handle)
      DestroyCursor(handle);
  }
};

typedef ScopedGDIObject<HBITMAP, DeleteObjectTraits<HBITMAP> > ScopedBitmap;
typedef ScopedGDIObject<HCURSOR, DestroyCursorTraits> ScopedCursor;

}  // namespace win
}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_WIN_SCOPED_GDI_HANDLE_H_
