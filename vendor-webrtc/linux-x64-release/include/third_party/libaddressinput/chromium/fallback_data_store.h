// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_LIBADDRESSINPUT_CHROMIUM_FALLBACK_DATA_STORE_H_
#define THIRD_PARTY_LIBADDRESSINPUT_CHROMIUM_FALLBACK_DATA_STORE_H_

#include <string>

namespace autofill {

class FallbackDataStore {
 public:
  // Gets stale, but valid static data for |key|. Should only be used as a last
  // resort after attempts to check the local cache or the webserver have
  // failed.
  static bool Get(const std::string& key, std::string* data);
};

}  // namespace autofill

#endif  // THIRD_PARTY_LIBADDRESSINPUT_CHROMIUM_FALLBACK_DATA_STORE_H_
