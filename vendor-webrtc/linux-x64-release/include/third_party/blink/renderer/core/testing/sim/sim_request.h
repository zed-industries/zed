// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_SIM_SIM_REQUEST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_SIM_SIM_REQUEST_H_

#include <optional>

#include "third_party/blink/public/platform/web_security_origin.h"
#include "third_party/blink/public/platform/web_url_error.h"
#include "third_party/blink/public/platform/web_url_response.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class SimNetwork;
class StaticDataNavigationBodyLoader;
class URLLoaderClient;

// Simulates a single request for a resource from the server. Requires a
// SimNetwork to have been created first. Use the Write(), Finish() and
// Complete() methods to simulate the response from the server.
// Note that all requests must be finished.
class SimRequestBase {
 public:
  // Additional params which can be passed to the SimRequest.
  struct Params {
    Params() : response_http_status(200) {}

    // Redirect the request to |redirect_url|. Don't call Start() or Complete()
    // if |redirect_url| is non-empty.
    String redirect_url;

    // Referrer URL that should be included in response.
    String referrer;

    // The origin of the request used to load the main resource.
    WebSecurityOrigin requestor_origin;

    WTF::HashMap<String, String> response_http_headers;

    // The HTTP status code of the response. |response_http_status| is ignored
    // if |redirect_url| is non-empty, since a redirect implies a 302 status
    // code.
    int response_http_status;
  };

  // Write a chunk of the response body.
  void Write(const String& data);
  void Write(const Vector<char>& data);

  // Finish the response, this is as if the server closed the connection.
  // If |navigation_body_loader| already finished, skip calling Finish on it.
  void Finish(bool body_loader_finished = false);

  // Shorthand to complete a request (start/write/finish) sequence in order.
  void Complete(const String& data = String());
  void Complete(const Vector<char>& data);

  const KURL& GetURL() const { return url_; }

 protected:
  SimRequestBase(KURL url,
                 String mime_type,
                 bool start_immediately,
                 Params params = Params());
  ~SimRequestBase();

  void StartInternal();
  void ServePending();

 private:
  friend class SimNetwork;

  void Reset();

  // Internal function to write a chunk of the response body
  void WriteInternal(base::span<const char>);

  // Used by SimNetwork.
  void DidReceiveResponse(URLLoaderClient*, const WebURLResponse&);
  void DidFail(const WebURLError&);
  void UsedForNavigation(StaticDataNavigationBodyLoader*);

  const KURL url_;
  String redirect_url_;
  String mime_type_;
  String referrer_;
  WebSecurityOrigin requestor_origin_;
  const bool start_immediately_;
  bool started_ = false;
  WebURLResponse response_;
  std::optional<WebURLError> error_;
  URLLoaderClient* client_ = nullptr;
  unsigned total_encoded_data_length_ = 0;
  WTF::HashMap<String, String> response_http_headers_;
  int response_http_status_;
  StaticDataNavigationBodyLoader* navigation_body_loader_ = nullptr;
};

// This request can be used as a main resource request for navigation.
// It does not allow starting asynchronously, because that's not how
// navigations work in reality.
// TODO(dgozman): rename this to SimNavigationRequest or something.
class SimRequest final : public SimRequestBase {
 public:
  SimRequest(KURL url, String mime_type, Params params = Params());
  SimRequest(String url, String mime_type, Params params = Params());
  ~SimRequest();
};

// This request can be started asynchronously, suited for simulating
// delayed load of subresources.
class SimSubresourceRequest final : public SimRequestBase {
 public:
  SimSubresourceRequest(KURL url, String mime_type, Params params = Params());
  SimSubresourceRequest(String url, String mime_type, Params params = Params());

  ~SimSubresourceRequest();

  // Starts the response from the server, this is as if the headers and 200 OK
  // reply had been received but no response body yet.
  void Start();
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_SIM_SIM_REQUEST_H_
