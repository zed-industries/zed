/*
 * Copyright (C) 2006-2012 Free Software Foundation, Inc.
 *
 * Author: Nikos Mavrogiannopoulos
 *
 * This file is part of GnuTLS.
 *
 * The GnuTLS is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public License
 * as published by the Free Software Foundation; either version 2.1 of
 * the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>
 *
 */

#ifndef GNUTLS_GNUTLSXX_H
#define GNUTLS_GNUTLSXX_H

#include <exception>
#include <vector>
#include <gnutls/gnutls.h>

namespace gnutls {

	class noncopyable {
	      protected:
		noncopyable() {
		} ~noncopyable() {
	      } private:
		// These are non-implemented.
		 noncopyable(const noncopyable &);
		noncopyable & operator=(const noncopyable &);
	};


	class exception:public std::exception {
	      public:
		explicit exception(int x);
		const char *what() const throw();
		int get_code();
	      protected:
		int retcode;
	};


	class dh_params:private noncopyable {
	      public:
		dh_params();
		~dh_params();
		void import_raw(const gnutls_datum_t & prime,
				const gnutls_datum_t & generator);
		void import_pkcs3(const gnutls_datum_t & pkcs3_params,
				  gnutls_x509_crt_fmt_t format);
		void generate(unsigned int bits);

		void export_pkcs3(gnutls_x509_crt_fmt_t format,
				  unsigned char *params_data,
				  size_t * params_data_size);
		void export_raw(gnutls_datum_t & prime,
				gnutls_datum_t & generator);

		gnutls_dh_params_t get_params_t() const;
		 dh_params & operator=(const dh_params & src);
	      protected:
		 gnutls_dh_params_t params;
	};


	class rsa_params:private noncopyable {
	      public:
		rsa_params();
		~rsa_params();
		void import_raw(const gnutls_datum_t & m,
				const gnutls_datum_t & e,
				const gnutls_datum_t & d,
				const gnutls_datum_t & p,
				const gnutls_datum_t & q,
				const gnutls_datum_t & u);
		void import_pkcs1(const gnutls_datum_t & pkcs1_params,
				  gnutls_x509_crt_fmt_t format);
		void generate(unsigned int bits);

		void export_pkcs1(gnutls_x509_crt_fmt_t format,
				  unsigned char *params_data,
				  size_t * params_data_size);
		void export_raw(gnutls_datum_t & m, gnutls_datum_t & e,
				gnutls_datum_t & d, gnutls_datum_t & p,
				gnutls_datum_t & q, gnutls_datum_t & u);
		gnutls_rsa_params_t get_params_t() const;
		 rsa_params & operator=(const rsa_params & src);

	      protected:
		 gnutls_rsa_params_t params;
	};

	class session:private noncopyable {
	      protected:
		gnutls_session_t s;
	      public:
		explicit session(unsigned int);
		 virtual ~ session();

		gnutls_session_t ptr();
		int bye(gnutls_close_request_t how);
		int handshake();

		gnutls_alert_description_t get_alert() const;

		int send_alert(gnutls_alert_level_t level,
			       gnutls_alert_description_t desc);
		int send_appropriate_alert(int err);

		gnutls_cipher_algorithm_t get_cipher() const;
		gnutls_kx_algorithm_t get_kx() const;
		gnutls_mac_algorithm_t get_mac() const;
		gnutls_compression_method_t get_compression() const;
		gnutls_certificate_type_t get_certificate_type() const;

		// for the handshake
		void set_private_extensions(bool allow);

		gnutls_handshake_description_t get_handshake_last_out()
		    const;
		gnutls_handshake_description_t get_handshake_last_in()
		    const;

		ssize_t send(const void *data, size_t sizeofdata);
		ssize_t recv(void *data, size_t sizeofdata);

		bool get_record_direction() const;

		// maximum packet size
		size_t get_max_size() const;
		void set_max_size(size_t size);

		size_t check_pending() const;

		void prf(size_t label_size, const char *label,
			 int server_random_first,
			 size_t extra_size, const char *extra,
			 size_t outsize, char *out);

		void prf_raw(size_t label_size, const char *label,
			     size_t seed_size, const char *seed,
			     size_t outsize, char *out);

		/* if you just want some defaults, use the following.
		 */
		void set_priority(const char *prio, const char **err_pos);
		void set_priority(gnutls_priority_t p);

		gnutls_protocol_t get_protocol_version() const;

		// for resuming sessions
		void set_data(const void *session_data,
			      size_t session_data_size);
		void get_data(void *session_data,
			      size_t * session_data_size) const;
		void get_data(gnutls_session_t session,
			      gnutls_datum_t & data) const;
		void get_id(void *session_id,
			    size_t * session_id_size) const;

		bool is_resumed() const;

		void set_max_handshake_packet_length(size_t max);

		void clear_credentials();
		void set_credentials(const class credentials & cred);

		void set_transport_ptr(gnutls_transport_ptr_t ptr);
		void set_transport_ptr(gnutls_transport_ptr_t recv_ptr,
				       gnutls_transport_ptr_t send_ptr);
		gnutls_transport_ptr_t get_transport_ptr() const;
		void get_transport_ptr(gnutls_transport_ptr_t & recv_ptr,
				       gnutls_transport_ptr_t & send_ptr)
		    const;

		void set_transport_lowat(size_t num);
		void set_transport_push_function(gnutls_push_func
						 push_func);
		void set_transport_vec_push_function(gnutls_vec_push_func
						     vec_push_func);
		void set_transport_pull_function(gnutls_pull_func
						 pull_func);
		void set_transport_pull_timeout_function (gnutls_pull_timeout_func pull_timeout_func);

		void set_user_ptr(void *ptr);
		void *get_user_ptr() const;

		void send_openpgp_cert(gnutls_openpgp_crt_status_t status);

		gnutls_credentials_type_t get_auth_type() const;
		gnutls_credentials_type_t get_server_auth_type() const;
		gnutls_credentials_type_t get_client_auth_type() const;

		// informational stuff
		void set_dh_prime_bits(unsigned int bits);
		unsigned int get_dh_secret_bits() const;
		unsigned int get_dh_peers_public_bits() const;
		unsigned int get_dh_prime_bits() const;
		void get_dh_group(gnutls_datum_t & gen,
				  gnutls_datum_t & prime) const;
		void get_dh_pubkey(gnutls_datum_t & raw_key) const;
		void get_rsa_export_pubkey(gnutls_datum_t & exponent,
					   gnutls_datum_t & modulus) const;
		unsigned int get_rsa_export_modulus_bits() const;

		void get_our_certificate(gnutls_datum_t & cert) const;
		bool get_peers_certificate(std::vector < gnutls_datum_t >
					   &out_certs) const;
		bool get_peers_certificate(const gnutls_datum_t ** certs,
					   unsigned int *certs_size) const;

		time_t get_peers_certificate_activation_time() const;
		time_t get_peers_certificate_expiration_time() const;
		void verify_peers_certificate(unsigned int &status) const;

	};

// interface for databases
	class DB:private noncopyable {
	      public:
		virtual ~ DB() = 0;
		virtual bool store(const gnutls_datum_t & key,
				   const gnutls_datum_t & data) = 0;
		virtual bool retrieve(const gnutls_datum_t & key,
				      gnutls_datum_t & data) = 0;
		virtual bool remove(const gnutls_datum_t & key) = 0;
	};

	class server_session:public session {
	      public:
		server_session();
		explicit server_session(int flags);
		~server_session();
		void db_remove() const;

		void set_db_cache_expiration(unsigned int seconds);
		void set_db(const DB & db);

		// returns true if session is expired
		bool db_check_entry(const gnutls_datum_t & session_data) const;

		// server side only
		const char *get_srp_username() const;
		const char *get_psk_username() const;

		void get_server_name(void *data, size_t * data_length,
				     unsigned int *type,
				     unsigned int indx) const;

		int rehandshake();
		void set_certificate_request(gnutls_certificate_request_t);
	};

	class client_session:public session {
	      public:
		client_session();
		explicit client_session(int flags);
		~client_session();

		void set_verify_cert(const char *hostname, unsigned flags);
		void set_server_name(gnutls_server_name_type_t type,
				     const void *name, size_t name_length);

		bool get_request_status();
	};


	class credentials:private noncopyable {
	      public:
		virtual ~ credentials() {
		} gnutls_credentials_type_t get_type() const;
	      protected:
		friend class session;
		explicit credentials(gnutls_credentials_type_t t);
		void *ptr() const;
		void set_ptr(void *ptr);
		gnutls_credentials_type_t type;
	      private:
		void *cred;
	};

	class certificate_credentials:public credentials {
	      public:
		~certificate_credentials();
		certificate_credentials();

		void free_keys();
		void free_cas();
		void free_ca_names();
		void free_crls();

		void set_dh_params(const dh_params & params);
		void set_rsa_export_params(const rsa_params & params);
		void set_verify_flags(unsigned int flags);
		void set_verify_limits(unsigned int max_bits,
				       unsigned int max_depth);

		void set_x509_trust_file(const char *cafile,
					 gnutls_x509_crt_fmt_t type);
		void set_x509_trust(const gnutls_datum_t & CA,
				    gnutls_x509_crt_fmt_t type);

		void set_x509_trust(gnutls_x509_crt_t * ca_list,
				    int ca_list_size);

		void set_x509_crl_file(const char *crlfile,
				       gnutls_x509_crt_fmt_t type);
		void set_x509_crl(const gnutls_datum_t & CRL,
				  gnutls_x509_crt_fmt_t type);
		void set_x509_crl(gnutls_x509_crl_t * crl_list,
				  int crl_list_size);

		void set_x509_key_file(const char *certfile,
				       const char *KEYFILE,
				       gnutls_x509_crt_fmt_t type);
		void set_x509_key(const gnutls_datum_t & CERT,
				  const gnutls_datum_t & KEY,
				  gnutls_x509_crt_fmt_t type);

		void set_x509_key(gnutls_x509_crt_t * cert_list,
				  int cert_list_size,
				  gnutls_x509_privkey_t key);


		void set_simple_pkcs12_file(const char *pkcs12file,
					    gnutls_x509_crt_fmt_t type,
					    const char *password);

		void set_retrieve_function
		    (gnutls_certificate_retrieve_function * func);

	      protected:
		 gnutls_certificate_credentials_t cred;
	};

	class certificate_server_credentials:public certificate_credentials {
	      public:
		void set_params_function(gnutls_params_function * func);
	};

	class certificate_client_credentials:public certificate_credentials {
	      public:
	};




	class anon_server_credentials:public credentials {
	      public:
		anon_server_credentials();
		~anon_server_credentials();
		void set_dh_params(const dh_params & params);
		void set_params_function(gnutls_params_function * func);
	      protected:
		 gnutls_anon_server_credentials_t cred;
	};

	class anon_client_credentials:public credentials {
	      public:
		anon_client_credentials();
		~anon_client_credentials();
	      protected:
		gnutls_anon_client_credentials_t cred;
	};


	class srp_server_credentials:public credentials {
	      public:
		srp_server_credentials();
		~srp_server_credentials();
		void set_credentials_file(const char *password_file,
					  const char *password_conf_file);
		void set_credentials_function
		    (gnutls_srp_server_credentials_function * func);
	      protected:
		 gnutls_srp_server_credentials_t cred;
	};

	class srp_client_credentials:public credentials {
	      public:
		srp_client_credentials();
		~srp_client_credentials();
		void set_credentials(const char *username,
				     const char *password);
		void set_credentials_function
		    (gnutls_srp_client_credentials_function * func);
	      protected:
		 gnutls_srp_client_credentials_t cred;
	};


	class psk_server_credentials:public credentials {
	      public:
		psk_server_credentials();
		~psk_server_credentials();
		void set_credentials_file(const char *password_file);
		void set_credentials_function
		    (gnutls_psk_server_credentials_function * func);
		void set_dh_params(const dh_params & params);
		void set_params_function(gnutls_params_function * func);
	      protected:
		 gnutls_psk_server_credentials_t cred;
	};

	class psk_client_credentials:public credentials {
	      public:
		psk_client_credentials();
		~psk_client_credentials();
		void set_credentials(const char *username,
				     const gnutls_datum_t & key,
				     gnutls_psk_key_flags flags);
		void set_credentials_function
		    (gnutls_psk_client_credentials_function * func);
	      protected:
		 gnutls_psk_client_credentials_t cred;
	};


}				/* namespace */

#endif /* GNUTLS_GNUTLSXX_H */
