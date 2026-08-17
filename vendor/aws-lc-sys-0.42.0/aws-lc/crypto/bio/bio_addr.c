// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <openssl/bio.h>

#if !defined(OPENSSL_NO_SOCK)

#include <openssl/mem.h>

#include "../internal.h"
#include "./internal.h"

BIO_ADDR *BIO_ADDR_new(void) {
  BIO_ADDR *ret = OPENSSL_zalloc(sizeof(BIO_ADDR));
  if (ret == NULL) {
    return NULL;
  }

  ret->sa.sa_family = AF_UNSPEC;
  return ret;
}

void BIO_ADDR_free(BIO_ADDR *ap) { OPENSSL_free(ap); }

int BIO_ADDR_copy(BIO_ADDR *dst, const BIO_ADDR *src) {
  GUARD_PTR(dst);
  GUARD_PTR(src);

  if (src->sa.sa_family == AF_UNSPEC) {
    BIO_ADDR_clear(dst);
    return 1;
  }

  if (src->sa.sa_family == AF_INET) {
    OPENSSL_memcpy(&dst->s_in, &src->sa, sizeof(dst->s_in));
    return 1;
  }
#ifdef AF_INET6
  if (src->sa.sa_family == AF_INET6) {
    OPENSSL_memcpy(&dst->s_in6, &src->sa, sizeof(dst->s_in6));
    return 1;
  }
#endif
#ifdef AWS_LC_HAS_AF_UNIX
  if (src->sa.sa_family == AF_UNIX) {
    OPENSSL_memcpy(&dst->s_un, &src->sa, sizeof(dst->s_un));
    return 1;
  }
#endif

  return 0;
}

BIO_ADDR *BIO_ADDR_dup(const BIO_ADDR *ap) {
  GUARD_PTR(ap);

  BIO_ADDR *ret = BIO_ADDR_new();
  if (ret == NULL) {
    return NULL;
  }

  if (1 != BIO_ADDR_copy(ret, ap)) {
    BIO_ADDR_free(ret);
    ret = NULL;
  }
  return ret;
}

void BIO_ADDR_clear(BIO_ADDR *ap) {
  if (ap == NULL) {
    return;
  }
  OPENSSL_cleanse(ap, sizeof(BIO_ADDR));
  ap->sa.sa_family = AF_UNSPEC;
}

int BIO_ADDR_family(const BIO_ADDR *ap) {
  GUARD_PTR(ap);

  return ap->sa.sa_family;
}

int BIO_ADDR_rawmake(BIO_ADDR *ap, int family, const void *where,
                     size_t wherelen, unsigned short port) {
  GUARD_PTR(ap);
  if (family == AF_INET) {
    if (wherelen != sizeof(struct in_addr)) {
      return 0;
    }
    OPENSSL_cleanse(&ap->s_in, sizeof(ap->s_in));
    ap->s_in.sin_family = family;
    ap->s_in.sin_port =  port;
    ap->s_in.sin_addr = *(struct in_addr *)where;
    return 1;
  }
#ifdef AF_INET6
  if (family == AF_INET6) {
    if (wherelen != sizeof(struct in6_addr)) {
      return 0;
    }
    OPENSSL_cleanse(&ap->s_in6, sizeof(ap->s_in6));
    ap->s_in6.sin6_family = family;
    ap->s_in6.sin6_port = port;
    ap->s_in6.sin6_addr = *(struct in6_addr *)where;
    return 1;
  }
#endif
#ifdef AWS_LC_HAS_AF_UNIX
  if (family == AF_UNIX) {
    GUARD_PTR(where);
    // wherelen is expected to be the length of the path string
    // not including the terminating NUL.
    if (wherelen >= sizeof(ap->s_un.sun_path)) {
      return 0;
    }
    OPENSSL_cleanse(&ap->s_un, sizeof(ap->s_un));
    ap->s_un.sun_family = family;
    OPENSSL_memcpy(ap->s_un.sun_path, where, wherelen);
    return 1;
  }
#endif

  return 0;
}

int BIO_ADDR_rawaddress(const BIO_ADDR *ap, void *p, size_t *l) {
  size_t len = 0;
  const void *addrptr = NULL;

  GUARD_PTR(ap);

  if (ap->sa.sa_family == AF_INET) {
    len = sizeof(ap->s_in.sin_addr);
    addrptr = &ap->s_in.sin_addr;
  }
#ifdef AF_INET6
  else if (ap->sa.sa_family == AF_INET6) {
    len = sizeof(ap->s_in6.sin6_addr);
    addrptr = &ap->s_in6.sin6_addr;
  }
#endif
#ifdef AWS_LC_HAS_AF_UNIX
  else if (ap->sa.sa_family == AF_UNIX) {
    // len does not include the null
    len = OPENSSL_strnlen(ap->s_un.sun_path, sizeof(ap->s_un.sun_path));
    addrptr = &ap->s_un.sun_path;
  }
#endif

  if (addrptr == NULL) {
    return 0;
  }
  if (l != NULL) {
    // AF_UNIX includes the trailing NUL written below so |*l| matches the
    // bytes written to |p| and reflects sockaddr_un's NUL-terminated path.
    // OpenSSL does not write this NUL; aws-lc intentionally does.
    *l = (ap->sa.sa_family == AF_UNIX) ? len + 1 : len;
  }

  if (p != NULL) {
    OPENSSL_memcpy(p, addrptr, len);
    if (ap->sa.sa_family == AF_UNIX) {
      // OpenSSL does not write the terminating NUL
      ((char *)p)[len] = '\0';
    }
  }

  return 1;
}

unsigned short BIO_ADDR_rawport(const BIO_ADDR *ap)
{
  GUARD_PTR(ap);

  if (ap->sa.sa_family == AF_INET) {
    return ap->s_in.sin_port;
  }
#ifdef AF_INET6
  if (ap->sa.sa_family == AF_INET6) {
    return ap->s_in6.sin6_port;
  }
#endif
  return 0;
}


#endif  // !defined(OPENSSL_NO_SOCK)
