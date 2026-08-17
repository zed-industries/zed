//
//
// Copyright 2015 gRPC authors.
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
//

#ifndef GRPC_SRC_CORE_UTIL_HTTP_CLIENT_HTTPCLI_H
#define GRPC_SRC_CORE_UTIL_HTTP_CLIENT_HTTPCLI_H

#include <grpc/event_engine/event_engine.h>
#include <grpc/grpc.h>
#include <grpc/slice.h>
#include <grpc/support/port_platform.h>
#include <stddef.h>

#include <functional>
#include <memory>
#include <optional>
#include <vector>

#include "absl/base/thread_annotations.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "src/core/handshaker/handshaker.h"
#include "src/core/lib/iomgr/closure.h"
#include "src/core/lib/iomgr/endpoint.h"
#include "src/core/lib/iomgr/error.h"
#include "src/core/lib/iomgr/exec_ctx.h"
#include "src/core/lib/iomgr/iomgr_fwd.h"
#include "src/core/lib/iomgr/iomgr_internal.h"
#include "src/core/lib/iomgr/polling_entity.h"
#include "src/core/lib/iomgr/resolve_address.h"
#include "src/core/lib/resource_quota/resource_quota.h"
#include "src/core/util/debug_location.h"
#include "src/core/util/http_client/parser.h"
#include "src/core/util/orphanable.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/sync.h"
#include "src/core/util/time.h"
#include "src/core/util/uri.h"

// User agent this library reports
#define GRPC_HTTPCLI_USER_AGENT "grpc-httpcli/0.0"

// override functions return 1 if they handled the request, 0 otherwise
typedef int (*grpc_httpcli_get_override)(const grpc_http_request* request,
                                         const grpc_core::URI& uri,
                                         grpc_core::Timestamp deadline,
                                         grpc_closure* on_complete,
                                         grpc_http_response* response);
typedef int (*grpc_httpcli_post_override)(const grpc_http_request* request,
                                          const grpc_core::URI& uri,
                                          absl::string_view body,
                                          grpc_core::Timestamp deadline,
                                          grpc_closure* on_complete,
                                          grpc_http_response* response);
typedef int (*grpc_httpcli_put_override)(const grpc_http_request* request,
                                         const grpc_core::URI& uri,
                                         absl::string_view body,
                                         grpc_core::Timestamp deadline,
                                         grpc_closure* on_complete,
                                         grpc_http_response* response);

namespace grpc_core {

// Tracks an in-progress GET or POST request. Calling \a Start()
// begins async work and calling \a Orphan() arranges for async work
// to be completed as sooon as possible (possibly aborting the request
// if it's in flight).
// TODO(ctiller): allow caching and capturing multiple requests for the
//                same content and combining them
class HttpRequest : public InternallyRefCounted<HttpRequest> {
 public:
  // Asynchronously perform a HTTP GET.
  // 'uri' is the target to make the request to. The scheme field is used to
  //  determine the port number. The authority field is the target host. The
  //  path field determines the path of the request. No other fields are used.
  // 'args' are optional channel args for the request.
  // 'pollent' indicates a grpc_polling_entity that is interested in the result
  //   of the get - work on this entity may be used to progress the get
  //   operation
  // 'request' contains request parameters - these are caller owned and
  //   can be destroyed once the call returns
  // 'deadline' contains a deadline for the request (or gpr_inf_future)
  // 'on_done' is a callback to report results to
  // 'channel_creds' are used to configurably secure the connection.
  //   For insecure requests, use grpc_insecure_credentials_create.
  //   For secure requests, use CreateHttpRequestSSLCredentials().
  //   nullptr is treated as insecure credentials.
  //   TODO(yihuaz): disallow nullptr as a value after unsecure builds
  //   are removed.
  GRPC_MUST_USE_RESULT static OrphanablePtr<HttpRequest> Get(
      URI uri, const grpc_channel_args* args, grpc_polling_entity* pollent,
      const grpc_http_request* request, Timestamp deadline,
      grpc_closure* on_done, grpc_http_response* response,
      RefCountedPtr<grpc_channel_credentials> channel_creds);

  // Asynchronously perform a HTTP POST.
  // 'uri' is the target to make the request to. The scheme field is used to
  //  determine the port number. The authority field is the target host. The
  //  path field determines the path of the request. No other fields are used.
  // 'args' are optional channel args for the request.
  // 'pollent' indicates a grpc_polling_entity that is interested in the result
  //   of the post - work on this entity may be used to progress the post
  //   operation
  // 'request' contains request parameters - these are caller owned and can be
  //   destroyed once the call returns
  // 'deadline' contains a deadline for the request (or gpr_inf_future)
  // 'on_done' is a callback to report results to
  // 'channel_creds' are used to configurably secure the connection.
  //   For insecure requests, use grpc_insecure_credentials_create.
  //   For secure requests, use CreateHttpRequestSSLCredentials().
  //   nullptr is treated as insecure credentials.
  //   TODO(apolcyn): disallow nullptr as a value after unsecure builds
  //   are removed.
  // Does not support ?var1=val1&var2=val2 in the path.
  GRPC_MUST_USE_RESULT static OrphanablePtr<HttpRequest> Post(
      URI uri, const grpc_channel_args* args, grpc_polling_entity* pollent,
      const grpc_http_request* request, Timestamp deadline,
      grpc_closure* on_done, grpc_http_response* response,
      RefCountedPtr<grpc_channel_credentials> channel_creds);

  // Asynchronously perform a HTTP PUT.
  // 'uri' is the target to make the request to. The scheme field is used to
  //  determine the port number. The authority field is the target host. The
  //  path field determines the path of the request. No other fields are used.
  // 'args' are optional channel args for the request.
  // 'pollent' indicates a grpc_polling_entity that is interested in the result
  //   of the post - work on this entity may be used to progress the post
  //   operation
  // 'request' contains request parameters - these are caller owned and can be
  //   destroyed once the call returns
  // 'deadline' contains a deadline for the request (or gpr_inf_future)
  // 'on_done' is a callback to report results to
  // 'channel_creds' are used to configurably secure the connection.
  //   For insecure requests, use grpc_insecure_credentials_create.
  //   For secure requests, use CreateHttpRequestSSLCredentials().
  //   nullptr is treated as insecure credentials.
  //   TODO(apolcyn): disallow nullptr as a value after unsecure builds
  //   are removed.
  // Does not support ?var1=val1&var2=val2 in the path.
  GRPC_MUST_USE_RESULT static OrphanablePtr<HttpRequest> Put(
      URI uri, const grpc_channel_args* args, grpc_polling_entity* pollent,
      const grpc_http_request* request, Timestamp deadline,
      grpc_closure* on_done, grpc_http_response* response,
      RefCountedPtr<grpc_channel_credentials> channel_creds);

  HttpRequest(URI uri, const grpc_slice& request_text,
              grpc_http_response* response, Timestamp deadline,
              const grpc_channel_args* channel_args, grpc_closure* on_done,
              grpc_polling_entity* pollent, const char* name,
              std::optional<std::function<bool()>> test_only_generate_response,
              RefCountedPtr<grpc_channel_credentials> channel_creds);

  ~HttpRequest() override;

  void Start();

  void Orphan() override;

  static void SetOverride(grpc_httpcli_get_override get,
                          grpc_httpcli_post_override post,
                          grpc_httpcli_put_override put);

  static void TestOnlySetOnHandshakeDoneIntercept(
      void (*intercept)(HttpRequest* req));

 private:
  void Finish(grpc_error_handle error) ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_) {
    grpc_polling_entity_del_from_pollset_set(pollent_, pollset_set_);
    ExecCtx::Run(DEBUG_LOCATION, on_done_, error);
  }

  void AppendError(grpc_error_handle error) ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_);

  void DoRead() ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_) {
    Ref().release();  // ref held by pending read
    grpc_endpoint_read(ep_.get(), &incoming_, &on_read_, /*urgent=*/true,
                       /*min_progress_size=*/1);
  }

  static void OnRead(void* user_data, grpc_error_handle error) {
    HttpRequest* req = static_cast<HttpRequest*>(user_data);
    ExecCtx::Run(DEBUG_LOCATION,
                 &req->continue_on_read_after_schedule_on_exec_ctx_, error);
  }

  // Needed since OnRead may be called inline from grpc_endpoint_read
  static void ContinueOnReadAfterScheduleOnExecCtx(void* user_data,
                                                   grpc_error_handle error) {
    RefCountedPtr<HttpRequest> req(static_cast<HttpRequest*>(user_data));
    MutexLock lock(&req->mu_);
    req->OnReadInternal(error);
  }

  void OnReadInternal(grpc_error_handle error)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_);

  void OnWritten() ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_) { DoRead(); }

  static void DoneWrite(void* arg, grpc_error_handle error) {
    HttpRequest* req = static_cast<HttpRequest*>(arg);
    ExecCtx::Run(DEBUG_LOCATION,
                 &req->continue_done_write_after_schedule_on_exec_ctx_, error);
  }

  // Needed since DoneWrite may be called inline from grpc_endpoint_write
  static void ContinueDoneWriteAfterScheduleOnExecCtx(void* arg,
                                                      grpc_error_handle error);

  void StartWrite() ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_);

  void OnHandshakeDone(absl::StatusOr<HandshakerArgs*> result);

  void DoHandshake(
      const grpc_event_engine::experimental::EventEngine::ResolvedAddress& addr)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_);

  void NextAddress(grpc_error_handle error) ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_);

  void OnResolved(
      absl::StatusOr<std::vector<
          grpc_event_engine::experimental::EventEngine::ResolvedAddress>>
          addresses_or);

  const URI uri_;
  const grpc_slice request_text_;
  const Timestamp deadline_;
  const grpc_channel_args* channel_args_;
  RefCountedPtr<grpc_channel_credentials> channel_creds_;
  grpc_closure on_read_;
  grpc_closure continue_on_read_after_schedule_on_exec_ctx_;
  grpc_closure done_write_;
  grpc_closure continue_done_write_after_schedule_on_exec_ctx_;
  OrphanablePtr<grpc_endpoint> ep_;
  grpc_closure* on_done_;
  ResourceQuotaRefPtr resource_quota_;
  grpc_polling_entity* pollent_;
  grpc_pollset_set* pollset_set_;
  const std::optional<std::function<bool()>> test_only_generate_response_;
  Mutex mu_;
  RefCountedPtr<HandshakeManager> handshake_mgr_ ABSL_GUARDED_BY(mu_);
  bool cancelled_ ABSL_GUARDED_BY(mu_) = false;
  grpc_http_parser parser_ ABSL_GUARDED_BY(mu_);
  std::vector<grpc_event_engine::experimental::EventEngine::ResolvedAddress>
      addresses_ ABSL_GUARDED_BY(mu_);
  size_t next_address_ ABSL_GUARDED_BY(mu_) = 0;
  int have_read_byte_ ABSL_GUARDED_BY(mu_) = 0;
  grpc_iomgr_object iomgr_obj_ ABSL_GUARDED_BY(mu_);
  grpc_slice_buffer incoming_ ABSL_GUARDED_BY(mu_);
  grpc_slice_buffer outgoing_ ABSL_GUARDED_BY(mu_);
  grpc_error_handle overall_error_ ABSL_GUARDED_BY(mu_) = absl::OkStatus();
  // TODO(yijiem): remove these once event_engine_dns_non_client_channel
  // experiment is fully enabled.
  bool use_event_engine_dns_resolver_;
  std::shared_ptr<DNSResolver> resolver_;
  std::optional<DNSResolver::TaskHandle> dns_request_handle_
      ABSL_GUARDED_BY(mu_) = DNSResolver::kNullHandle;
  absl::StatusOr<std::unique_ptr<
      grpc_event_engine::experimental::EventEngine::DNSResolver>>
      ee_resolver_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_HTTP_CLIENT_HTTPCLI_H
