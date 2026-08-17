/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_WIN_SCOPED_COM_INITIALIZER_H_
#define RTC_BASE_WIN_SCOPED_COM_INITIALIZER_H_

#include <comdef.h>

namespace webrtc {

// Initializes COM in the constructor (STA or MTA), and uninitializes COM in the
// destructor. Taken from base::win::ScopedCOMInitializer.
//
// WARNING: This should only be used once per thread, ideally scoped to a
// similar lifetime as the thread itself.  You should not be using this in
// random utility functions that make COM calls; instead ensure that these
// functions are running on a COM-supporting thread!
// See https://msdn.microsoft.com/en-us/library/ms809971.aspx for details.
class ScopedCOMInitializer {
 public:
  // Enum value provided to initialize the thread as an MTA instead of STA.
  // There are two types of apartments, Single Threaded Apartments (STAs)
  // and Multi Threaded Apartments (MTAs). Within a given process there can
  // be multiple STA’s but there is only one MTA. STA is typically used by
  // "GUI applications" and MTA by "worker threads" with no UI message loop.
  enum SelectMTA { kMTA };

  // Constructor for STA initialization.
  ScopedCOMInitializer();

  // Constructor for MTA initialization.
  explicit ScopedCOMInitializer(SelectMTA mta);

  ~ScopedCOMInitializer();

  ScopedCOMInitializer(const ScopedCOMInitializer&) = delete;
  ScopedCOMInitializer& operator=(const ScopedCOMInitializer&) = delete;

  bool Succeeded() { return SUCCEEDED(hr_); }

 private:
  void Initialize(COINIT init);

  HRESULT hr_;
};

}  // namespace webrtc

#endif  // RTC_BASE_WIN_SCOPED_COM_INITIALIZER_H_
