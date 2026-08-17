// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_TESTING_TEST_RESPONSE_GENERATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_TESTING_TEST_RESPONSE_GENERATOR_H_

#include <stdint.h>

#include "third_party/blink/public/platform/web_url.h"
#include "third_party/blink/public/platform/web_url_error.h"
#include "third_party/blink/public/platform/web_url_response.h"

namespace blink {

// Generates WebURLErrors and WebURLResponses suitable for testing purposes.
class TestResponseGenerator {
 public:
  enum Flags {
    kNormal = 0,
    kNoAcceptRanges = 1 << 0,   // Don't include Accept-Ranges in 206 response.
    kNoContentRange = 1 << 1,   // Don't include Content-Range in 206 response.
    kNoContentLength = 1 << 2,  // Don't include Content-Length in 206 response.
    kNoContentRangeInstanceSize = 1 << 3,  // Content-Range: N-M/* in 206.
  };

  // Build an HTTP response generator for the given URL. |content_length| is
  // used to generate Content-Length and Content-Range headers.
  TestResponseGenerator(const KURL& url, int64_t content_length);
  TestResponseGenerator(const TestResponseGenerator&) = delete;
  TestResponseGenerator& operator=(const TestResponseGenerator&) = delete;

  // Generates a WebURLError object.
  WebURLError GenerateError();

  // Generates a regular HTTP 200 response.
  WebURLResponse Generate200();

  // Generates a regular HTTP 206 response starting from |first_byte_offset|
  // until the end of the resource.
  WebURLResponse Generate206(int64_t first_byte_offset);

  // Generates a custom HTTP 206 response starting from |first_byte_offset|
  // until the end of the resource. You can tweak what gets included in the
  // headers via |flags|.
  WebURLResponse Generate206(int64_t first_byte_offset, Flags flags);

  // Generates a regular HTTP 206 response starting from |first_byte_offset|
  // until |last_byte_offset|.
  WebURLResponse GeneratePartial206(int64_t first_byte_offset,
                                    int64_t last_byte_offset);

  // Generates a custom HTTP 206 response starting from |first_byte_offset|
  // until |last_byte_offset|. You can tweak what gets included in the
  // headers via |flags|.
  WebURLResponse GeneratePartial206(int64_t first_byte_offset,
                                    int64_t last_byte_offset,
                                    Flags flags);

  // Generates a regular HTTP 404 response.
  WebURLResponse Generate404();

  // Generate a HTTP response with specified code.
  WebURLResponse GenerateResponse(int code);

  // Generates a file:// response starting from |first_byte_offset| until the
  // end of the resource.
  //
  // If |first_byte_offset| is negative a response containing no content length
  // will be returned.
  WebURLResponse GenerateFileResponse(int64_t first_byte_offset);

  int64_t content_length() { return content_length_; }

 private:
  WebURL url_;
  int64_t content_length_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_TESTING_TEST_RESPONSE_GENERATOR_H_
