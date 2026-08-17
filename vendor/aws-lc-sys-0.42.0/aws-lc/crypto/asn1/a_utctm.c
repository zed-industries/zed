// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/asn1.h>
#include <openssl/bytestring.h>
#include <openssl/err.h>
#include <openssl/mem.h>

#include <stdlib.h>
#include <string.h>
#include <time.h>

#include "internal.h"

int asn1_utctime_to_tm(struct tm *tm, const ASN1_UTCTIME *d,
                       int allow_timezone_offset) {
  if (d->type != V_ASN1_UTCTIME) {
    return 0;
  }
  CBS cbs;
  CBS_init(&cbs, d->data, (size_t)d->length);
  if (!CBS_parse_utc_time(&cbs, tm, allow_timezone_offset)) {
    return 0;
  }
  return 1;
}

int ASN1_UTCTIME_check(const ASN1_UTCTIME *d) {
  return asn1_utctime_to_tm(NULL, d, /*allow_timezone_offset=*/1);
}

int ASN1_UTCTIME_set_string(ASN1_UTCTIME *s, const char *str) {
  // Although elsewhere we allow timezone offsets with UTCTime, to be compatible
  // with some existing misissued certificates, this function is used to
  // construct new certificates and can be stricter.
  size_t len = strlen(str);
  CBS cbs;
  CBS_init(&cbs, (const uint8_t *)str, len);
  if (!CBS_parse_utc_time(&cbs, /*out_tm=*/NULL,
                          /*allow_timezone_offset=*/0)) {
    return 0;
  }
  if (s != NULL) {
    if (!ASN1_STRING_set(s, str, len)) {
      return 0;
    }
    s->type = V_ASN1_UTCTIME;
  }
  return 1;
}

ASN1_UTCTIME *ASN1_UTCTIME_set(ASN1_UTCTIME *s, int64_t posix_time) {
  return ASN1_UTCTIME_adj(s, posix_time, 0, 0);
}

ASN1_UTCTIME *ASN1_UTCTIME_adj(ASN1_UTCTIME *s, int64_t posix_time, int offset_day,
                               long offset_sec) {
  struct tm data;
  if (!OPENSSL_posix_to_tm(posix_time, &data)) {
    return NULL;
  }

  if (offset_day || offset_sec) {
    if (!OPENSSL_gmtime_adj(&data, offset_day, offset_sec)) {
      return NULL;
    }
  }

  if (data.tm_year < 50 || data.tm_year >= 150) {
    return NULL;
  }

  char buf[14];
  int ret = snprintf(buf, sizeof(buf), "%02d%02d%02d%02d%02d%02dZ",
                     data.tm_year % 100, data.tm_mon + 1, data.tm_mday,
                     data.tm_hour, data.tm_min, data.tm_sec);
  if (ret != (int)(sizeof(buf) - 1)) {
    abort();  // |snprintf| should neither truncate nor write fewer bytes.
  }

  int free_s = 0;
  if (s == NULL) {
    free_s = 1;
    s = ASN1_UTCTIME_new();
    if (s == NULL) {
      return NULL;
    }
  }

  if (!ASN1_STRING_set(s, buf, strlen(buf))) {
    if (free_s) {
      ASN1_UTCTIME_free(s);
    }
    return NULL;
  }
  s->type = V_ASN1_UTCTIME;
  return s;
}

int ASN1_UTCTIME_cmp_time_t(const ASN1_UTCTIME *s, time_t t) {
  struct tm stm, ttm;
  int day, sec;

  if (!asn1_utctime_to_tm(&stm, s, /*allow_timezone_offset=*/1)) {
    return -2;
  }

  if (!OPENSSL_posix_to_tm(t, &ttm)) {
    return -2;
  }

  if (!OPENSSL_gmtime_diff(&day, &sec, &ttm, &stm)) {
    return -2;
  }

  if (day > 0) {
    return 1;
  }
  if (day < 0) {
    return -1;
  }
  if (sec > 0) {
    return 1;
  }
  if (sec < 0) {
    return -1;
  }
  return 0;
}
