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

#ifndef GRPC_SRC_CORE_LIB_SECURITY_CREDENTIALS_EXTERNAL_AWS_REQUEST_SIGNER_H
#define GRPC_SRC_CORE_LIB_SECURITY_CREDENTIALS_EXTERNAL_AWS_REQUEST_SIGNER_H

#include <grpc/support/port_platform.h>

#include <map>
#include <string>

#include "src/core/lib/iomgr/error.h"
#include "src/core/util/uri.h"

namespace grpc_core {

// Implements an AWS API request signer based on the AWS Signature Version 4
// signing process.
// https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html
// To retrieve the subject token in AwsExternalAccountCredentials, we need to
// sign an AWS request server and use the signed request as the subject token.
// This class is a utility to sign an AWS request.
class AwsRequestSigner {
 public:
  // Construct a signer with the necessary information to sign a request.
  // `access_key_id`, `secret_access_key` and `token` are the AWS credentials
  // required for signing. `method` and `url` are the HTTP method and url of the
  // request. `region` is the region of the AWS environment. `request_payload`
  // is the payload of the HTTP request. `additional_headers` are additional
  // headers to be inject into the request.
  AwsRequestSigner(std::string access_key_id, std::string secret_access_key,
                   std::string token, std::string method, std::string url,
                   std::string region, std::string request_payload,
                   std::map<std::string, std::string> additional_headers,
                   grpc_error_handle* error);

  // This method triggers the signing process then returns the headers of the
  // signed request as a map. In case there is an error, the input `error`
  // parameter will be updated and an empty map will be returned if there is
  // error.
  std::map<std::string, std::string> GetSignedRequestHeaders();

 private:
  std::string access_key_id_;
  std::string secret_access_key_;
  std::string token_;
  std::string method_;
  URI url_;
  std::string region_;
  std::string request_payload_;
  std::map<std::string, std::string> additional_headers_;

  std::string static_request_date_;
  std::map<std::string, std::string> request_headers_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_SECURITY_CREDENTIALS_EXTERNAL_AWS_REQUEST_SIGNER_H
