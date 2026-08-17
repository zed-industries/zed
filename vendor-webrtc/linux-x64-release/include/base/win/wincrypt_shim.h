// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_WINCRYPT_SHIM_H_
#define BASE_WIN_WINCRYPT_SHIM_H_

// Any Chromium headers which want to `#include <wincrypt.h>` should instead
// #include this header.

#include <windows.h>

#include <wincrypt.h>

// wincrypt.h defines macros which conflict with OpenSSL's types. See
// http://msdn.microsoft.com/en-us/library/windows/desktop/aa378145(v=vs.85).aspx
#undef PKCS7_SIGNER_INFO
#undef X509_CERT_PAIR
#undef X509_EXTENSIONS
#undef X509_NAME

#define WINCRYPT_PKCS7_SIGNER_INFO ((LPCSTR)500)
#define WINCRYPT_X509_CERT_PAIR ((LPCSTR)53)
#define WINCRYPT_X509_EXTENSIONS ((LPCSTR)5)
#define WINCRYPT_X509_NAME ((LPCSTR)7)

#endif  // BASE_WIN_WINCRYPT_SHIM_H_
