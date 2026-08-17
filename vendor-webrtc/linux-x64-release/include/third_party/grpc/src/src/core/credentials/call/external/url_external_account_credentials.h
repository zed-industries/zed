//
// Copyright 2020 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

#ifndef GRPC_SRC_CORE_CREDENTIALS_CALL_EXTERNAL_URL_EXTERNAL_ACCOUNT_CREDENTIALS_H
#define GRPC_SRC_CORE_CREDENTIALS_CALL_EXTERNAL_URL_EXTERNAL_ACCOUNT_CREDENTIALS_H

#include <grpc/support/port_platform.h>

#include <functional>
#include <map>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "src/core/credentials/call/external/external_account_credentials.h"
#include "src/core/lib/iomgr/error.h"
#include "src/core/util/http_client/httpcli.h"
#include "src/core/util/orphanable.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/uri.h"

namespace grpc_core {

class UrlExternalAccountCredentials final : public ExternalAccountCredentials {
 public:
  static absl::StatusOr<RefCountedPtr<UrlExternalAccountCredentials>> Create(
      Options options, std::vector<std::string> scopes,
      std::shared_ptr<grpc_event_engine::experimental::EventEngine>
          event_engine = nullptr);

  UrlExternalAccountCredentials(
      Options options, std::vector<std::string> scopes,
      std::shared_ptr<grpc_event_engine::experimental::EventEngine>
          event_engine,
      grpc_error_handle* error);

  std::string debug_string() override;

  static UniqueTypeName Type();

  UniqueTypeName type() const override { return Type(); }

 private:
  OrphanablePtr<FetchBody> RetrieveSubjectToken(
      Timestamp deadline,
      absl::AnyInvocable<void(absl::StatusOr<std::string>)> on_done) override;

  absl::string_view CredentialSourceType() override;

  // Fields of credential source
  URI url_;
  std::string url_full_path_;
  std::map<std::string, std::string> headers_;
  std::string format_type_;
  std::string format_subject_token_field_name_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CREDENTIALS_CALL_EXTERNAL_URL_EXTERNAL_ACCOUNT_CREDENTIALS_H
