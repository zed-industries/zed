/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_LINUX_X11_X_ATOM_CACHE_H_
#define MODULES_DESKTOP_CAPTURE_LINUX_X11_X_ATOM_CACHE_H_

#include <X11/X.h>
#include <X11/Xlib.h>

namespace webrtc {

// A cache of Atom. Each Atom object is created on demand.
class XAtomCache final {
 public:
  explicit XAtomCache(::Display* display);
  ~XAtomCache();

  ::Display* display() const;

  Atom WmState();
  Atom WindowType();
  Atom WindowTypeNormal();
  Atom IccProfile();

 private:
  // If |*atom| is None, this function uses XInternAtom() to retrieve an Atom.
  Atom CreateIfNotExist(Atom* atom, const char* name);

  ::Display* const display_;
  Atom wm_state_ = None;
  Atom window_type_ = None;
  Atom window_type_normal_ = None;
  Atom icc_profile_ = None;
};

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_LINUX_X11_X_ATOM_CACHE_H_
