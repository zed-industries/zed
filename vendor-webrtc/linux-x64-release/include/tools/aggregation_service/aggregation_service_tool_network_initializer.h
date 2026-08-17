// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_AGGREGATION_SERVICE_AGGREGATION_SERVICE_TOOL_NETWORK_INITIALIZER_H_
#define TOOLS_AGGREGATION_SERVICE_AGGREGATION_SERVICE_TOOL_NETWORK_INITIALIZER_H_

#include <memory>

#include "base/memory/scoped_refptr.h"
#include "content/public/test/content_test_suite_base.h"
#include "content/public/test/test_content_client_initializer.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "services/data_decoder/public/cpp/test_support/in_process_data_decoder.h"
#include "services/network/network_context.h"
#include "services/network/network_service.h"
#include "services/network/public/cpp/shared_url_loader_factory.h"
#include "services/network/public/mojom/url_loader_factory.mojom.h"

namespace aggregation_service {

// This class is responsible for initializing network states, including state
// for processing network responses. The object should be kept alive for the
// duration of network usage. Inherits from `ContentTestSuiteBase` to ensure the
// `ContentBrowserClient` is initialized.
class ToolNetworkInitializer : public content::ContentTestSuiteBase {
 public:
  ToolNetworkInitializer();
  ToolNetworkInitializer(const ToolNetworkInitializer& other) = delete;
  ToolNetworkInitializer& operator=(const ToolNetworkInitializer& other) =
      delete;
  ~ToolNetworkInitializer() override;

  scoped_refptr<network::SharedURLLoaderFactory> shared_url_loader_factory() {
    return shared_url_loader_factory_;
  }

 private:
  // Needed for the call to `content::GetCertVerifierParams()`
  content::TestContentClientInitializer test_content_initializer_;

  std::unique_ptr<network::NetworkService> network_service_;
  std::unique_ptr<network::NetworkContext> network_context_;
  mojo::Remote<network::mojom::URLLoaderFactory> url_loader_factory_;
  scoped_refptr<network::SharedURLLoaderFactory> shared_url_loader_factory_;

  // Used to process JSON network responses.
  std::unique_ptr<data_decoder::test::InProcessDataDecoder>
      in_process_data_decoder_;
};

}  // namespace aggregation_service

#endif  // TOOLS_AGGREGATION_SERVICE_AGGREGATION_SERVICE_TOOL_NETWORK_INITIALIZER_H_