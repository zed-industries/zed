/*
 * Copyright (C) 2014 Free Software Foundation, Inc.
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

/* Prototypes for direct handling of extension data */

#ifndef GNUTLS_X509_EXT_H
#define GNUTLS_X509_EXT_H

#include <gnutls/gnutls.h>
#include <gnutls/x509.h>

/* *INDENT-OFF* */
#ifdef __cplusplus
extern "C" {
#endif
/* *INDENT-ON* */

typedef struct gnutls_subject_alt_names_st *gnutls_subject_alt_names_t;

int gnutls_subject_alt_names_init(gnutls_subject_alt_names_t *);
void gnutls_subject_alt_names_deinit(gnutls_subject_alt_names_t sans);
int gnutls_subject_alt_names_get(gnutls_subject_alt_names_t sans, unsigned int seq,
				 unsigned int *san_type, gnutls_datum_t * san,
				 gnutls_datum_t * othername_oid);
int gnutls_subject_alt_names_set(gnutls_subject_alt_names_t sans,
				 unsigned int san_type,
				 const gnutls_datum_t * san,
				 const char* othername_oid);


int gnutls_x509_ext_import_subject_alt_names(const gnutls_datum_t * ext,
					 gnutls_subject_alt_names_t,
					 unsigned int flags);
int gnutls_x509_ext_export_subject_alt_names(gnutls_subject_alt_names_t,
					  gnutls_datum_t * ext);

/* They are exactly the same */
#define gnutls_x509_ext_import_issuer_alt_name gnutls_x509_ext_import_subject_alt_name
#define gnutls_x509_ext_export_issuer_alt_name gnutls_x509_ext_export_subject_alt_name

typedef struct gnutls_x509_crl_dist_points_st *gnutls_x509_crl_dist_points_t;

int gnutls_x509_crl_dist_points_init(gnutls_x509_crl_dist_points_t *);
void gnutls_x509_crl_dist_points_deinit(gnutls_x509_crl_dist_points_t);
int gnutls_x509_crl_dist_points_get(gnutls_x509_crl_dist_points_t, unsigned int seq,
				  unsigned int *type,
				  gnutls_datum_t *dist, unsigned int *reason_flags);
int gnutls_x509_crl_dist_points_set(gnutls_x509_crl_dist_points_t,
				 gnutls_x509_subject_alt_name_t type,
				 const gnutls_datum_t *dist, unsigned int reason_flags);

int gnutls_x509_ext_import_crl_dist_points(const gnutls_datum_t * ext,
					gnutls_x509_crl_dist_points_t dp,
					unsigned int flags);
int gnutls_x509_ext_export_crl_dist_points(gnutls_x509_crl_dist_points_t dp,
					gnutls_datum_t * ext);

int gnutls_x509_ext_import_name_constraints(const gnutls_datum_t * ext,
					 gnutls_x509_name_constraints_t nc,
					 unsigned int flags);
int gnutls_x509_ext_export_name_constraints(gnutls_x509_name_constraints_t nc,
					 gnutls_datum_t * ext);

typedef struct gnutls_x509_aia_st *gnutls_x509_aia_t;

int gnutls_x509_aia_init(gnutls_x509_aia_t *);
void gnutls_x509_aia_deinit(gnutls_x509_aia_t);
int gnutls_x509_aia_get(gnutls_x509_aia_t aia, unsigned int seq,
			gnutls_datum_t *oid,
			unsigned *san_type,
			gnutls_datum_t *san);
int gnutls_x509_aia_set(gnutls_x509_aia_t aia,
			const char *oid,
			unsigned san_type,
			const gnutls_datum_t * san);

int gnutls_x509_ext_import_aia(const gnutls_datum_t * ext,
				gnutls_x509_aia_t,
				unsigned int flags);
int gnutls_x509_ext_export_aia(gnutls_x509_aia_t aia,
					      gnutls_datum_t * ext);

int gnutls_x509_ext_import_subject_key_id(const gnutls_datum_t * ext,
				       gnutls_datum_t * id);
int gnutls_x509_ext_export_subject_key_id(const gnutls_datum_t * id,
				       gnutls_datum_t * ext);

typedef struct gnutls_x509_aki_st *gnutls_x509_aki_t;

int gnutls_x509_ext_export_authority_key_id(gnutls_x509_aki_t,
					 gnutls_datum_t * ext);
int gnutls_x509_ext_import_authority_key_id(const gnutls_datum_t * ext,
					 gnutls_x509_aki_t,
					 unsigned int flags);

int gnutls_x509_othername_to_virtual(const char *oid, 
				     const gnutls_datum_t *othername,
				     unsigned int *virt_type,
				     gnutls_datum_t *virt);

int gnutls_x509_aki_init(gnutls_x509_aki_t *);
int gnutls_x509_aki_get_id(gnutls_x509_aki_t, gnutls_datum_t *id);
int gnutls_x509_aki_get_cert_issuer(gnutls_x509_aki_t aki, unsigned int seq,
				 unsigned int *san_type, gnutls_datum_t * san,
				 gnutls_datum_t *othername_oid,
				 gnutls_datum_t *serial);
int gnutls_x509_aki_set_id(gnutls_x509_aki_t aki, const gnutls_datum_t *id);
int gnutls_x509_aki_set_cert_issuer(gnutls_x509_aki_t aki, 
				 unsigned int san_type, 
				 const gnutls_datum_t * san,
				 const char *othername_oid,
				 const gnutls_datum_t * serial);
void gnutls_x509_aki_deinit(gnutls_x509_aki_t);

int gnutls_x509_ext_import_private_key_usage_period(const gnutls_datum_t * ext,
						 time_t * activation,
						 time_t * expiration);
int gnutls_x509_ext_export_private_key_usage_period(time_t activation,
						 time_t expiration,
						 gnutls_datum_t * ext);

int gnutls_x509_ext_import_basic_constraints(const gnutls_datum_t * ext,
					  unsigned int *ca, int *pathlen);
int gnutls_x509_ext_export_basic_constraints(unsigned int ca, int pathlen,
					  gnutls_datum_t * ext);

typedef struct gnutls_x509_key_purposes_st *gnutls_x509_key_purposes_t;

int gnutls_x509_key_purpose_init(gnutls_x509_key_purposes_t *p);
void gnutls_x509_key_purpose_deinit(gnutls_x509_key_purposes_t p);
int gnutls_x509_key_purpose_set(gnutls_x509_key_purposes_t p, const char *oid);
int gnutls_x509_key_purpose_get(gnutls_x509_key_purposes_t p, unsigned idx, gnutls_datum_t *oid);

int gnutls_x509_ext_import_key_purposes(const gnutls_datum_t * ext,
				     gnutls_x509_key_purposes_t,
				     unsigned int flags);
int gnutls_x509_ext_export_key_purposes(gnutls_x509_key_purposes_t,
				     gnutls_datum_t * ext);


int gnutls_x509_ext_import_key_usage(const gnutls_datum_t * ext,
				  unsigned int *key_usage);
int gnutls_x509_ext_export_key_usage(unsigned int key_usage,
				  gnutls_datum_t * ext);

int gnutls_x509_ext_import_inhibit_anypolicy(const gnutls_datum_t * ext,
				  unsigned int *skipcerts);
int gnutls_x509_ext_export_inhibit_anypolicy(unsigned int skipcerts,
				  gnutls_datum_t * ext);

int gnutls_x509_ext_import_proxy(const gnutls_datum_t * ext, int *pathlen,
			      char **policyLanguage, char **policy,
			      size_t * sizeof_policy);
int gnutls_x509_ext_export_proxy(int pathLenConstraint, const char *policyLanguage,
			      const char *policy, size_t sizeof_policy,
			      gnutls_datum_t * ext);

typedef struct gnutls_x509_policies_st *gnutls_x509_policies_t;

int gnutls_x509_policies_init(gnutls_x509_policies_t *);
void gnutls_x509_policies_deinit(gnutls_x509_policies_t);

int gnutls_x509_policies_get(gnutls_x509_policies_t policies, unsigned int seq,
				 struct gnutls_x509_policy_st *policy);
int gnutls_x509_policies_set(gnutls_x509_policies_t policies,
				 const struct gnutls_x509_policy_st *policy);

int gnutls_x509_ext_import_policies(const gnutls_datum_t * ext, gnutls_x509_policies_t
				 policies,
				 unsigned int flags);
int gnutls_x509_ext_export_policies(gnutls_x509_policies_t policies,
				 gnutls_datum_t * ext);

int gnutls_x509_ext_import_tlsfeatures(const gnutls_datum_t * ext,
									   gnutls_x509_tlsfeatures_t,
									   unsigned int flags);

int gnutls_x509_ext_export_tlsfeatures(gnutls_x509_tlsfeatures_t f,
					  gnutls_datum_t * ext);

int gnutls_x509_tlsfeatures_add(gnutls_x509_tlsfeatures_t f, unsigned int feature);

/* *INDENT-OFF* */
#ifdef __cplusplus
}
#endif
/* *INDENT-ON* */

#endif /* GNUTLS_X509_EXT_H */
