// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_LIBADDRESSINPUT_CHROMIUM_CHROME_STORAGE_IMPL_H_
#define THIRD_PARTY_LIBADDRESSINPUT_CHROMIUM_CHROME_STORAGE_IMPL_H_

#include <memory>
#include <string>
#include <vector>

#include "base/scoped_observation.h"
#include "components/prefs/pref_store.h"
#include "third_party/libaddressinput/src/cpp/include/libaddressinput/storage.h"

class WriteablePrefStore;

namespace autofill {

// An implementation of the Storage interface which passes through to an
// underlying WriteablePrefStore.
class ChromeStorageImpl : public ::i18n::addressinput::Storage,
                          public PrefStore::Observer {
 public:
  // |store| must outlive |this|.
  explicit ChromeStorageImpl(WriteablePrefStore* store);

  ChromeStorageImpl(const ChromeStorageImpl&) = delete;
  ChromeStorageImpl& operator=(const ChromeStorageImpl&) = delete;

  virtual ~ChromeStorageImpl();

  // ::i18n::addressinput::Storage implementation.
  void Put(const std::string& key, std::string* data) override;
  void Get(const std::string& key, const Callback& data_ready) const override;

  // PrefStore::Observer implementation.
  void OnInitializationCompleted(bool succeeded) override;

 private:
  struct Request {
    Request(const std::string& key, const Callback& callback);

    std::string key;
    const Callback& callback;
  };

  // Non-const version of Get().
  void DoGet(const std::string& key, const Callback& data_ready);

  WriteablePrefStore* backing_store_;  // weak

  // Get requests that haven't yet been serviced.
  std::vector<std::unique_ptr<Request>> outstanding_requests_;

  base::ScopedObservation<PrefStore, PrefStore::Observer> scoped_observation_{
      this};
};

}  // namespace autofill

#endif  // THIRD_PARTY_LIBADDRESSINPUT_CHROMIUM_CHROME_STORAGE_IMPL_H_
