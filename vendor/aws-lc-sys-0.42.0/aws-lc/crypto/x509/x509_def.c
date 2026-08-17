// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/x509.h>

// TODO(fork): cleanup

#define OPENSSLDIR "/etc/ssl"

#define X509_CERT_AREA OPENSSLDIR
#define X509_CERT_DIR OPENSSLDIR "/certs"
#define X509_CERT_FILE OPENSSLDIR "/cert.pem"
#define X509_PRIVATE_DIR OPENSSLDIR "/private"
#define X509_CERT_DIR_EVP "SSL_CERT_DIR"
#define X509_CERT_FILE_EVP "SSL_CERT_FILE"

const char *X509_get_default_private_dir(void) { return X509_PRIVATE_DIR; }

const char *X509_get_default_cert_area(void) { return X509_CERT_AREA; }

const char *X509_get_default_cert_dir(void) { return X509_CERT_DIR; }

const char *X509_get_default_cert_file(void) { return X509_CERT_FILE; }

const char *X509_get_default_cert_dir_env(void) { return X509_CERT_DIR_EVP; }

const char *X509_get_default_cert_file_env(void) {
  return X509_CERT_FILE_EVP;
}
