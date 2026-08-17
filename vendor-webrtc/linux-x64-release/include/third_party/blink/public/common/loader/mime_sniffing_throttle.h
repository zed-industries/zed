// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_MIME_SNIFFING_THROTTLE_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_MIME_SNIFFING_THROTTLE_H_

#include "base/memory/weak_ptr.h"
#include "base/task/sequenced_task_runner.h"
#include "services/network/public/mojom/url_response_head.mojom-forward.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/loader/url_loader_throttle.h"

namespace blink {

// Throttle for mime type sniffing. This may intercept the request and
// modify the response's mime type in the response head.
class BLINK_COMMON_EXPORT MimeSniffingThrottle : public URLLoaderThrottle {
 public:
  // |task_runner| is used to bind the right task runner for handling incoming
  // IPC in MimeSniffingLoader. |task_runner| is supposed to be bound to the
  // current sequence.
  explicit MimeSniffingThrottle(
      scoped_refptr<base::SequencedTaskRunner> task_runner);
  ~MimeSniffingThrottle() override;

  // Implements blink::URLLoaderThrottle.
  void DetachFromCurrentSequence() override;
  void WillProcessResponse(const GURL& response_url,
                           network::mojom::URLResponseHead* response_head,
                           bool* defer) override;
  const char* NameForLoggingWillProcessResponse() override;

  // Called from MimeSniffingURLLoader once mime type is ready.
  void ResumeWithNewResponseHead(
      network::mojom::URLResponseHeadPtr new_response_head,
      mojo::ScopedDataPipeConsumerHandle body);

 private:
  scoped_refptr<base::SequencedTaskRunner> task_runner_;
  base::WeakPtrFactory<MimeSniffingThrottle> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_MIME_SNIFFING_THROTTLE_H_
