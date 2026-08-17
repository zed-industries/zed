#ifndef CURLINC_HEADER_H
#define CURLINC_HEADER_H
/***************************************************************************
 *                                  _   _ ____  _
 *  Project                     ___| | | |  _ \| |
 *                             / __| | | | |_) | |
 *                            | (__| |_| |  _ <| |___
 *                             \___|\___/|_| \_\_____|
 *
 * Copyright (C) Daniel Stenberg, <daniel@haxx.se>, et al.
 *
 * This software is licensed as described in the file COPYING, which
 * you should have received as part of this distribution. The terms
 * are also available at https://curl.se/docs/copyright.html.
 *
 * You may opt to use, copy, modify, merge, publish, distribute and/or sell
 * copies of the Software, and permit persons to whom the Software is
 * furnished to do so, under the terms of the COPYING file.
 *
 * This software is distributed on an "AS IS" basis, WITHOUT WARRANTY OF ANY
 * KIND, either express or implied.
 *
 * SPDX-License-Identifier: curl
 *
 ***************************************************************************/

#ifdef  __cplusplus
extern "C" {
#endif

struct curl_header {
  char *name;    /* this might not use the same case */
  char *value;
  size_t amount; /* number of headers using this name  */
  size_t index;  /* ... of this instance, 0 or higher */
  unsigned int origin; /* see bits below */
  void *anchor; /* handle privately used by libcurl */
};

/* 'origin' bits */
#define CURLH_HEADER    (1<<0) /* plain server header */
#define CURLH_TRAILER   (1<<1) /* trailers */
#define CURLH_CONNECT   (1<<2) /* CONNECT headers */
#define CURLH_1XX       (1<<3) /* 1xx headers */
#define CURLH_PSEUDO    (1<<4) /* pseudo headers */

typedef enum {
  CURLHE_OK,
  CURLHE_BADINDEX,      /* header exists but not with this index */
  CURLHE_MISSING,       /* no such header exists */
  CURLHE_NOHEADERS,     /* no headers at all exist (yet) */
  CURLHE_NOREQUEST,     /* no request with this number was used */
  CURLHE_OUT_OF_MEMORY, /* out of memory while processing */
  CURLHE_BAD_ARGUMENT,  /* a function argument was not okay */
  CURLHE_NOT_BUILT_IN   /* if API was disabled in the build */
} CURLHcode;

CURL_EXTERN CURLHcode curl_easy_header(CURL *easy,
                                       const char *name,
                                       size_t index,
                                       unsigned int origin,
                                       int request,
                                       struct curl_header **hout);

CURL_EXTERN struct curl_header *curl_easy_nextheader(CURL *easy,
                                                     unsigned int origin,
                                                     int request,
                                                     struct curl_header *prev);

#ifdef __cplusplus
} /* end of extern "C" */
#endif

#endif /* CURLINC_HEADER_H */
