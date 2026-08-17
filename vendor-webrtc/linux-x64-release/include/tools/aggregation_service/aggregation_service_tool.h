// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_AGGREGATION_SERVICE_AGGREGATION_SERVICE_TOOL_H_
#define TOOLS_AGGREGATION_SERVICE_AGGREGATION_SERVICE_TOOL_H_

#include <memory>
#include <string>
#include <string_view>
#include <vector>

#include "base/strings/string_split.h"
#include "base/values.h"
#include "tools/aggregation_service/aggregation_service_tool_network_initializer.h"
#include "url/gurl.h"

namespace base {
class FilePath;
}  // namespace base

namespace content {
class TestAggregationService;
}  // namespace content

namespace url {
class Origin;
}  // namespace url

namespace aggregation_service {

struct UrlKeyFile {
  UrlKeyFile(GURL url, std::string key_file);
  UrlKeyFile(const UrlKeyFile& other);
  UrlKeyFile& operator=(const UrlKeyFile& other);
  ~UrlKeyFile();

  GURL url;
  std::string key_file;
};

// This class is a wrapper for aggregation service tool.
class AggregationServiceTool {
 public:
  AggregationServiceTool();
  ~AggregationServiceTool();

  // Sets whether to disable the AggregatableReport's payload(s) being encrypted
  // after serialization.
  void SetDisablePayloadEncryption(bool should_disable);

  // Sets public keys to storage from the url-filename pairs and returns
  // whether it's successful.
  bool SetPublicKeys(const std::vector<UrlKeyFile>& key_files);

  // Construct an aggregatable report from the specified information and returns
  // a `base::Value::Dict` for its JSON representation. Empty
  // `base::Value::Dict` will be returned in case of error.
  base::Value::Dict AssembleReport(std::string operation_str,
                                   std::string bucket_str,
                                   std::string value_str,
                                   std::string aggregation_mode_str,
                                   url::Origin reporting_origin,
                                   std::vector<GURL> processing_urls,
                                   bool is_debug_mode_enabled,
                                   base::Value::Dict additional_fields,
                                   std::string api_version,
                                   std::string api_identifier);

  // Sends the contents of the aggregatable report to the specified reporting
  // url `url` and returns whether it's successful.
  bool SendReport(const base::Value& contents, const GURL& url);

  // Writes the contents of the aggregatable report to the specified file
  // `filename` and returns whether it's successful.
  bool WriteReportToFile(const base::Value& contents,
                         const base::FilePath& filename);

 private:
  bool SetPublicKeysFromFile(const GURL& url, std::string_view json_file_path);

  ToolNetworkInitializer network_initializer_;
  std::unique_ptr<content::TestAggregationService> agg_service_;
};

}  // namespace aggregation_service

#endif  // TOOLS_AGGREGATION_SERVICE_AGGREGATION_SERVICE_TOOL_H_
