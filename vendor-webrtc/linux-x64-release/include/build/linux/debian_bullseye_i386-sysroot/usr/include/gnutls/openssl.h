/*
 * Copyright (C) 2004-2012 Free Software Foundation, Inc.
 * Copyright (c) 2002 Andrew McDonald <andrew@mcdonald.org.uk>
 *
 * This file is part of GnuTLS-EXTRA.
 *
 * GnuTLS-extra is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License as
 * published by the Free Software Foundation; either version 3 of the
 * License, or (at your option) any later version.
 *
 * GnuTLS-extra is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with GnuTLS-EXTRA; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA
 * 02110-1301, USA.
 *
 */

/* WARNING: Error functions aren't currently thread-safe */

/* This file contains prototypes about the OpenSSL compatibility layer
 * in GnuTLS.  GnuTLS is not a complete replacement of OPENSSL so this
 * compatibility layer only supports limited OpenSSL functionality.
 *
 * New programs should avoid using this compatibility layer, and use
 * the native GnuTLS API directly.
 */

#ifndef GNUTLS_OPENSSL_H
#define GNUTLS_OPENSSL_H

#ifdef __cplusplus
extern "C" {
#endif

#include <gnutls/gnutls.h>

/* Extra definitions that no longer exist in gnutls.
 */
#define GNUTLS_X509_CN_SIZE 256
#define GNUTLS_X509_C_SIZE 3
#define GNUTLS_X509_O_SIZE 256
#define GNUTLS_X509_OU_SIZE 256
#define GNUTLS_X509_L_SIZE 256
#define GNUTLS_X509_S_SIZE 256
#define GNUTLS_X509_EMAIL_SIZE 256

	typedef struct {
		char common_name[GNUTLS_X509_CN_SIZE];
		char country[GNUTLS_X509_C_SIZE];
		char organization[GNUTLS_X509_O_SIZE];
		char organizational_unit_name[GNUTLS_X509_OU_SIZE];
		char locality_name[GNUTLS_X509_L_SIZE];
		char state_or_province_name[GNUTLS_X509_S_SIZE];
		char email[GNUTLS_X509_EMAIL_SIZE];
	} gnutls_x509_dn;


#define OPENSSL_VERSION_NUMBER (0x0090604F)
#define SSLEAY_VERSION_NUMBER OPENSSL_VERSION_NUMBER
#define OPENSSL_VERSION_TEXT ("GNUTLS " GNUTLS_VERSION " ")

#define SSL_ERROR_NONE        (0)
#define SSL_ERROR_SSL         (1)
#define SSL_ERROR_WANT_READ   (2)
#define SSL_ERROR_WANT_WRITE  (3)
#define SSL_ERROR_SYSCALL     (5)
#define SSL_ERROR_ZERO_RETURN (6)

#define SSL_FILETYPE_PEM (GNUTLS_X509_FMT_PEM)

#define SSL_VERIFY_NONE (0)

#define SSL_ST_OK (1)

#define X509_V_ERR_CERT_NOT_YET_VALID          (1)
#define X509_V_ERR_CERT_HAS_EXPIRED            (2)
#define X509_V_ERR_DEPTH_ZERO_SELF_SIGNED_CERT (3)

#define SSL_OP_ALL (0x000FFFFF)
#define SSL_OP_NO_TLSv1 (0x0400000)

#define SSL_MODE_ENABLE_PARTIAL_WRITE (0x1)
#define SSL_MODE_ACCEPT_MOVING_WRITE_BUFFER (0x2)
#define SSL_MODE_AUTO_RETRY (0x4)

#undef X509_NAME
#undef X509
	typedef gnutls_x509_dn X509_NAME;
	typedef gnutls_datum_t X509;

	typedef struct _SSL SSL;

	typedef struct {
		char priority_string[256];
		unsigned int connend;
	} SSL_METHOD;

	typedef struct {
		gnutls_protocol_t version;
		gnutls_cipher_algorithm_t cipher;
		gnutls_kx_algorithm_t kx;
		gnutls_mac_algorithm_t mac;
		gnutls_compression_method_t compression;
		gnutls_certificate_type_t cert;
	} SSL_CIPHER;

	typedef struct _BIO {
		gnutls_transport_ptr_t fd;
	} BIO;

	typedef struct {
		SSL *ssl;
		int error;
		const gnutls_datum_t *cert_list;
#define current_cert cert_list
	} X509_STORE_CTX;

#define X509_STORE_CTX_get_current_cert(ctx) ((ctx)->current_cert)

	typedef struct _SSL_CTX {
		SSL_METHOD *method;
		char *certfile;
		int certfile_type;
		char *keyfile;
		int keyfile_type;
		unsigned long options;

		int (*verify_callback) (int, X509_STORE_CTX *);
		int verify_mode;

	} SSL_CTX;

	struct _SSL {
		gnutls_session_t gnutls_state;

		gnutls_certificate_client_credentials gnutls_cred;

		SSL_CTX *ctx;
		SSL_CIPHER ciphersuite;

		int last_error;
		int shutdown;
		int state;
		unsigned long options;

		int (*verify_callback) (int, X509_STORE_CTX *);
		int verify_mode;

		gnutls_transport_ptr_t rfd;
		gnutls_transport_ptr_t wfd;
	};

#define rbio gnutls_state

	typedef struct {
		void *handle;
	} MD_CTX;

	struct rsa_st;
	typedef struct rsa_st RSA;

#define MD5_CTX MD_CTX
#define RIPEMD160_CTX MD_CTX

#define OpenSSL_add_ssl_algorithms()  SSL_library_init()
#define SSLeay_add_ssl_algorithms()   SSL_library_init()
#define SSLeay_add_all_algorithms()   OpenSSL_add_all_algorithms()

#define SSL_get_cipher_name(ssl) SSL_CIPHER_get_name(SSL_get_current_cipher(ssl))
#define SSL_get_cipher(ssl) SSL_get_cipher_name(ssl)
#define SSL_get_cipher_bits(ssl,bp) SSL_CIPHER_get_bits(SSL_get_current_cipher(ssl),(bp))
#define SSL_get_cipher_version(ssl) SSL_CIPHER_get_version(SSL_get_current_cipher(ssl))


/* Library initialisation functions */

	int SSL_library_init(void);
	void OpenSSL_add_all_algorithms(void);


/* SSL_CTX structure handling */

	SSL_CTX *SSL_CTX_new(SSL_METHOD * method);
	void SSL_CTX_free(SSL_CTX * ctx);
	int SSL_CTX_set_default_verify_paths(SSL_CTX * ctx);
	int SSL_CTX_use_certificate_file(SSL_CTX * ctx,
					 const char *certfile, int type);
	int SSL_CTX_use_PrivateKey_file(SSL_CTX * ctx, const char *keyfile,
					int type);
	void SSL_CTX_set_verify(SSL_CTX * ctx, int verify_mode,
				int (*verify_callback) (int,
							X509_STORE_CTX *));
	unsigned long SSL_CTX_set_options(SSL_CTX * ctx,
					  unsigned long options);
	long SSL_CTX_set_mode(SSL_CTX * ctx, long mode);
	int SSL_CTX_set_cipher_list(SSL_CTX * ctx, const char *list);


/* SSL_CTX statistics */

	long SSL_CTX_sess_number(SSL_CTX * ctx);
	long SSL_CTX_sess_connect(SSL_CTX * ctx);
	long SSL_CTX_sess_connect_good(SSL_CTX * ctx);
	long SSL_CTX_sess_connect_renegotiate(SSL_CTX * ctx);
	long SSL_CTX_sess_accept(SSL_CTX * ctx);
	long SSL_CTX_sess_accept_good(SSL_CTX * ctx);
	long SSL_CTX_sess_accept_renegotiate(SSL_CTX * ctx);
	long SSL_CTX_sess_hits(SSL_CTX * ctx);
	long SSL_CTX_sess_misses(SSL_CTX * ctx);
	long SSL_CTX_sess_timeouts(SSL_CTX * ctx);


/* SSL structure handling */

	SSL *SSL_new(SSL_CTX * ctx);
	void SSL_free(SSL * ssl);
	void SSL_load_error_strings(void);
	int SSL_get_error(SSL * ssl, int ret);
	int SSL_set_fd(SSL * ssl, int fd);
	int SSL_set_rfd(SSL * ssl, int fd);
	int SSL_set_wfd(SSL * ssl, int fd);
	void SSL_set_bio(SSL * ssl, BIO * rbio, BIO * wbio);
	void SSL_set_connect_state(SSL * ssl);
	int SSL_pending(SSL * ssl);
	void SSL_set_verify(SSL * ssl, int verify_mode,
			    int (*verify_callback) (int,
						    X509_STORE_CTX *));
	const X509 *SSL_get_peer_certificate(SSL * ssl);

/* SSL connection open/close/read/write functions */

	int SSL_connect(SSL * ssl);
	int SSL_accept(SSL * ssl);
	int SSL_shutdown(SSL * ssl);
	int SSL_read(SSL * ssl, void *buf, int len);
	int SSL_write(SSL * ssl, const void *buf, int len);

	int SSL_want(SSL * ssl);

#define SSL_NOTHING (1)
#define SSL_WRITING (2)
#define SSL_READING (3)
#define SSL_X509_LOOKUP (4)

#define SSL_want_nothing(s) (SSL_want(s) == SSL_NOTHING)
#define SSL_want_read(s) (SSL_want(s) == SSL_READING)
#define SSL_want_write(s) (SSL_want(s) == SSL_WRITING)
#define SSL_want_x509_lookup(s) (SSL_want(s) == SSL_X509_LOOKUP)


/* SSL_METHOD functions */

	SSL_METHOD *SSLv23_client_method(void);
	SSL_METHOD *SSLv23_server_method(void);
	SSL_METHOD *SSLv3_client_method(void);
	SSL_METHOD *SSLv3_server_method(void);
	SSL_METHOD *TLSv1_client_method(void);
	SSL_METHOD *TLSv1_server_method(void);


/* SSL_CIPHER functions */

	SSL_CIPHER *SSL_get_current_cipher(SSL * ssl);
	const char *SSL_CIPHER_get_name(SSL_CIPHER * cipher);
	int SSL_CIPHER_get_bits(SSL_CIPHER * cipher, int *bits);
	const char *SSL_CIPHER_get_version(SSL_CIPHER * cipher);
	char *SSL_CIPHER_description(SSL_CIPHER * cipher, char *buf,
				     int size);


/* X509 functions */

	X509_NAME *X509_get_subject_name(const X509 * cert);
	X509_NAME *X509_get_issuer_name(const X509 * cert);
	char *X509_NAME_oneline(gnutls_x509_dn * name, char *buf, int len);
	void X509_free(const X509 * cert);


/* BIO functions */

	void BIO_get_fd(gnutls_session_t gnutls_state, int *fd);
	BIO *BIO_new_socket(int sock, int close_flag);

/* error handling */

	unsigned long ERR_get_error(void);
	const char *ERR_error_string(unsigned long e, char *buf);


/* RAND functions */

	int RAND_status(void);
	void RAND_seed(const void *buf, int num);
	int RAND_bytes(unsigned char *buf, int num);
	int RAND_pseudo_bytes(unsigned char *buf, int num);
	const char *RAND_file_name(char *buf, size_t len);
	int RAND_load_file(const char *name, long maxbytes);
	int RAND_write_file(const char *name);

	int RAND_egd_bytes(const char *path, int bytes);
#define RAND_egd(p) RAND_egd_bytes((p), 255)

/* message digest functions */

#define MD5_DIGEST_LENGTH 16

	void MD5_Init(MD5_CTX * ctx);
	void MD5_Update(MD5_CTX * ctx, const void *buf, int len);
	void MD5_Final(unsigned char *md, MD5_CTX * ctx);
	unsigned char *MD5(const unsigned char *buf, unsigned long len,
			   unsigned char *md);

	void RIPEMD160_Init(RIPEMD160_CTX * ctx);
	void RIPEMD160_Update(RIPEMD160_CTX * ctx, const void *buf,
			      int len);
	void RIPEMD160_Final(unsigned char *md, RIPEMD160_CTX * ctx);
	unsigned char *RIPEMD160(const unsigned char *buf,
				 unsigned long len, unsigned char *md);

#ifdef __cplusplus
}
#endif
#endif
