// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_LIBADDRESSINPUT_CHROMIUM_STORAGE_TEST_RUNNER_H_
#define THIRD_PARTY_LIBADDRESSINPUT_CHROMIUM_STORAGE_TEST_RUNNER_H_

#include "third_party/libaddressinput/src/cpp/include/libaddressinput/storage.h"

#include <memory>
#include <string>

namespace autofill {

// A test sutie for ::i18n::addressinput::Storage.
class StorageTestRunner {
 public:
  // Does not take ownership of |storage|.
  explicit StorageTestRunner(::i18n::addressinput::Storage* storage);

  StorageTestRunner(const StorageTestRunner&) = delete;
  StorageTestRunner& operator=(const StorageTestRunner&) = delete;

  ~StorageTestRunner();

  // Runs all the tests from the standard test suite.
  void RunAllTests();

 private:
  void ClearValues();
  std::unique_ptr<::i18n::addressinput::Storage::Callback> BuildCallback();
  void OnDataReady(bool success, const std::string& key, std::string* data);

  // Test suite.
  void GetWithoutPutReturnsEmptyData();
  void GetReturnsWhatWasPut();
  void SecondPutOverwritesData();

  ::i18n::addressinput::Storage* storage_;  // weak
  bool success_;
  std::string key_;
  std::string data_;
};

}  // namespace autofill

#endif  // THIRD_PARTY_LIBADDRESSINPUT_CHROMIUM_STORAGE_TEST_RUNNER_H_
