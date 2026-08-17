// Copyright 1999-2016 The OpenSSL Project Authors. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// This file contains a table of "standard" extensions

#if defined(__cplusplus)
extern "C" {
#endif

extern const X509V3_EXT_METHOD v3_bcons, v3_nscert, v3_key_usage, v3_ext_ku;
extern const X509V3_EXT_METHOD v3_info, v3_sinfo;
extern const X509V3_EXT_METHOD v3_ns_ia5_list[], v3_alt[], v3_skey_id,
    v3_akey_id;
extern const X509V3_EXT_METHOD v3_crl_num, v3_crl_reason, v3_crl_invdate;
extern const X509V3_EXT_METHOD v3_delta_crl, v3_cpols, v3_crld, v3_freshest_crl;
extern const X509V3_EXT_METHOD v3_ocsp_nonce, v3_ocsp_accresp, v3_ocsp_acutoff;
extern const X509V3_EXT_METHOD v3_ocsp_crlid, v3_ocsp_nocheck,
    v3_ocsp_serviceloc;
extern const X509V3_EXT_METHOD v3_crl_hold;
extern const X509V3_EXT_METHOD v3_policy_mappings, v3_policy_constraints;
extern const X509V3_EXT_METHOD v3_name_constraints, v3_inhibit_anyp, v3_idp;
extern const X509V3_EXT_METHOD v3_addr, v3_asid;

// This table will be searched using OBJ_bsearch so it *must* kept in order
// of the ext_nid values.

// TODO(fork): OCSP support
#define OPENSSL_NO_OCSP

static const X509V3_EXT_METHOD *const standard_exts[] = {
    &v3_nscert,
    &v3_ns_ia5_list[0],
    &v3_ns_ia5_list[1],
    &v3_ns_ia5_list[2],
    &v3_ns_ia5_list[3],
    &v3_ns_ia5_list[4],
    &v3_ns_ia5_list[5],
    &v3_ns_ia5_list[6],
    &v3_skey_id,
    &v3_key_usage,
    &v3_alt[0],
    &v3_alt[1],
    &v3_bcons,
    &v3_crl_num,
    &v3_cpols,
    &v3_akey_id,
    &v3_crld,
    &v3_ext_ku,
    &v3_delta_crl,
    &v3_crl_reason,
    &v3_crl_invdate,
    &v3_info,
#ifndef OPENSSL_NO_OCSP
    &v3_ocsp_nonce,
    &v3_ocsp_crlid,
    &v3_ocsp_accresp,
    &v3_ocsp_acutoff,
    &v3_ocsp_serviceloc,
#endif
    &v3_ocsp_nocheck,
    &v3_sinfo,
    &v3_policy_constraints,
#ifndef OPENSSL_NO_OCSP
    &v3_crl_hold,
#endif
    &v3_name_constraints,
    &v3_policy_mappings,
    &v3_inhibit_anyp,
    &v3_idp,
    &v3_alt[2],
    &v3_freshest_crl,
};

// Number of standard extensions

#define STANDARD_EXTENSION_COUNT \
  (sizeof(standard_exts) / sizeof(X509V3_EXT_METHOD *))

#if defined(__cplusplus)
}  // extern C
#endif
