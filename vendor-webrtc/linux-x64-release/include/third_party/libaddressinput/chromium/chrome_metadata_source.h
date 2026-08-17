// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_LIBADDRESSINPUT_CHROMIUM_CHROME_METADATA_SOURCE_H_
#define THIRD_PARTY_LIBADDRESSINPUT_CHROMIUM_CHROME_METADATA_SOURCE_H_

#include <list>
#include <memory>
#include <string>

#include "base/memory/scoped_refptr.h"
#include "third_party/libaddressinput/src/cpp/include/libaddressinput/source.h"

namespace network {
class SharedURLLoaderFactory;
class SimpleURLLoader;
}

namespace autofill {

// A class for downloading rules to let libaddressinput validate international
// addresses.
class ChromeMetadataSource : public ::i18n::addressinput::Source {
 public:
  ChromeMetadataSource(
      const std::string& validation_data_url,
      scoped_refptr<network::SharedURLLoaderFactory> url_loader_factory);

  ChromeMetadataSource(const ChromeMetadataSource&) = delete;
  ChromeMetadataSource& operator=(const ChromeMetadataSource&) = delete;

  virtual ~ChromeMetadataSource();

  // ::i18n::addressinput::Source:
  virtual void Get(const std::string& key,
                   const Callback& downloaded) const override;

 private:
  struct Request {
    Request(const std::string& key,
            std::unique_ptr<network::SimpleURLLoader> loader,
            const Callback& callback);

    std::string key;
    // The data that's received.
    std::string data;
    // The object that manages retrieving the data.
    std::unique_ptr<network::SimpleURLLoader> loader;
    const Callback& callback;
  };

  using RequestList = std::list<std::unique_ptr<Request>>;

  // Non-const method actually implementing Get().
  void Download(const std::string& key, const Callback& downloaded);

  void OnSimpleLoaderComplete(RequestList::iterator it,
                              std::unique_ptr<std::string> response_body);

  const std::string validation_data_url_;
  scoped_refptr<network::SharedURLLoaderFactory> url_loader_factory_;

  // Holds all pending requests and their URL loaders.
  RequestList requests_;
};

}  // namespace autofill

#endif  // THIRD_PARTY_LIBADDRESSINPUT_CHROMIUM_CHROME_METADATA_SOURCE_H_
