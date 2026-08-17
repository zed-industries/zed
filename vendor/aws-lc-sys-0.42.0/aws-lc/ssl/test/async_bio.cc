// Copyright (c) 2014, Google Inc.
// SPDX-License-Identifier: ISC

#include "async_bio.h"

#include <errno.h>
#include <string.h>

#include <openssl/bio.h>
#include <openssl/mem.h>

#include "../../crypto/internal.h"


namespace {

extern const BIO_METHOD g_async_bio_method;

struct AsyncBio {
  bool datagram;
  bool enforce_write_quota;
  size_t read_quota;
  size_t write_quota;
};

AsyncBio *GetData(BIO *bio) {
  if (bio->method != &g_async_bio_method) {
    return NULL;
  }
  return (AsyncBio *)bio->ptr;
}

static int AsyncWrite(BIO *bio, const char *in, int inl) {
  AsyncBio *a = GetData(bio);
  if (a == NULL || bio->next_bio == NULL) {
    return 0;
  }

  if (!a->enforce_write_quota) {
    return BIO_write(bio->next_bio, in, inl);
  }

  BIO_clear_retry_flags(bio);

  if (a->write_quota == 0) {
    BIO_set_retry_write(bio);
    errno = EAGAIN;
    return -1;
  }

  if (!a->datagram && static_cast<size_t>(inl) > a->write_quota) {
    inl = static_cast<int>(a->write_quota);
  }
  int ret = BIO_write(bio->next_bio, in, inl);
  if (ret <= 0) {
    BIO_copy_next_retry(bio);
  } else {
    a->write_quota -= (a->datagram ? 1 : ret);
  }
  return ret;
}

static int AsyncRead(BIO *bio, char *out, int outl) {
  AsyncBio *a = GetData(bio);
  if (a == NULL || bio->next_bio == NULL) {
    return 0;
  }

  BIO_clear_retry_flags(bio);

  if (a->read_quota == 0) {
    BIO_set_retry_read(bio);
    errno = EAGAIN;
    return -1;
  }

  if (!a->datagram && static_cast<size_t>(outl) > a->read_quota) {
    outl = static_cast<int>(a->read_quota);
  }
  int ret = BIO_read(bio->next_bio, out, outl);
  if (ret <= 0) {
    BIO_copy_next_retry(bio);
  } else {
    a->read_quota -= (a->datagram ? 1 : ret);
  }
  return ret;
}

static long AsyncCtrl(BIO *bio, int cmd, long num, void *ptr) {
  if (bio->next_bio == NULL) {
    return 0;
  }
  BIO_clear_retry_flags(bio);
  long ret = BIO_ctrl(bio->next_bio, cmd, num, ptr);
  BIO_copy_next_retry(bio);
  return ret;
}

static int AsyncNew(BIO *bio) {
  AsyncBio *a = (AsyncBio *)OPENSSL_zalloc(sizeof(*a));
  if (a == NULL) {
    return 0;
  }
  a->enforce_write_quota = true;
  bio->init = 1;
  bio->ptr = (char *)a;
  return 1;
}

static int AsyncFree(BIO *bio) {
  if (bio == NULL) {
    return 0;
  }

  OPENSSL_free(bio->ptr);
  bio->ptr = NULL;
  bio->init = 0;
  bio->flags = 0;
  return 1;
}

static long AsyncCallbackCtrl(BIO *bio, int cmd, bio_info_cb fp) {
  if (bio->next_bio == NULL) {
    return 0;
  }
  return BIO_callback_ctrl(bio->next_bio, cmd, fp);
}

const BIO_METHOD g_async_bio_method = {
  BIO_TYPE_FILTER,
  "async bio",
  AsyncWrite,
  AsyncRead,
  NULL /* puts */,
  NULL /* gets */,
  AsyncCtrl,
  AsyncNew,
  AsyncFree,
  AsyncCallbackCtrl,
};

}  // namespace

bssl::UniquePtr<BIO> AsyncBioCreate() {
  return bssl::UniquePtr<BIO>(BIO_new(&g_async_bio_method));
}

bssl::UniquePtr<BIO> AsyncBioCreateDatagram() {
  bssl::UniquePtr<BIO> ret(BIO_new(&g_async_bio_method));
  if (!ret) {
    return nullptr;
  }
  GetData(ret.get())->datagram = true;
  return ret;
}

void AsyncBioAllowRead(BIO *bio, size_t count) {
  AsyncBio *a = GetData(bio);
  if (a == NULL) {
    return;
  }
  a->read_quota += count;
}

void AsyncBioAllowWrite(BIO *bio, size_t count) {
  AsyncBio *a = GetData(bio);
  if (a == NULL) {
    return;
  }
  a->write_quota += count;
}

void AsyncBioEnforceWriteQuota(BIO *bio, bool enforce) {
  AsyncBio *a = GetData(bio);
  if (a == NULL) {
    return;
  }
  a->enforce_write_quota = enforce;
}
