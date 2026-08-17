/*
 * Copyright 2008 by the Massachusetts Institute of Technology.
 * All Rights Reserved.
 *
 * Export of this software from the United States of America may
 *   require a specific license from the United States Government.
 *   It is the responsibility of any person or organization contemplating
 *   export to obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to use, copy, modify, and
 * distribute this software and its documentation for any purpose and
 * without fee is hereby granted, provided that the above copyright
 * notice appear in all copies and that both that copyright notice and
 * this permission notice appear in supporting documentation, and that
 * the name of M.I.T. not be used in advertising or publicity pertaining
 * to distribution of the software without specific, written prior
 * permission.  Furthermore if you modify this software you must label
 * your software as modified software and not distribute it in such a
 * fashion that it might be confused with the original M.I.T. software.
 * M.I.T. makes no representations about the suitability of
 * this software for any purpose.  It is provided "as is" without express
 * or implied warranty.
 */

#ifndef GSSAPI_EXT_H_
#define GSSAPI_EXT_H_

#include <gssapi/gssapi.h>

#ifdef __cplusplus
extern "C" {
#endif /* __cplusplus */

/*
 * Solaris extensions
 */
#ifndef _WIN32
OM_uint32 KRB5_CALLCONV
gss_pname_to_uid
	(OM_uint32 *minor,
         const gss_name_t name,
	 const gss_OID mech_type,
	 uid_t *uidOut);
#endif

/**
 * Provides a platform-specific name for a GSSAPI name as interpreted by a
 * given mechanism.
 *
 * @param [out] minor      Minor status code
 * @param [in] name        The gss name resulting from accept_sec_context
 * @param [in] mech_type   The mechanism that will be asked to map @a name to a
 *                         local name
 * @param [out] localname  Caller-allocated buffer to be filled in with the
 *                         local name on success
 */
OM_uint32 KRB5_CALLCONV
gss_localname
	(OM_uint32 *minor,
	 const gss_name_t name,
	 gss_const_OID mech_type,
	 gss_buffer_t localname);

/**
 * Determine whether a mechanism name is authorized to act as a username.
 *
 * @param [in] name      Mechanism name
 * @param [in] username  System username
 *
 * This is a simple wrapper around gss_authorize_localname().  It only supports
 * system usernames as local names, and cannot distinguish between lack of
 * authorization and other errors.
 *
 * @retval 1 @a name is authorized to act as @a username
 * @retval 0 @a name is not authorized or an error occurred
 */
int KRB5_CALLCONV
gss_userok(const gss_name_t name,
           const char *username);

/**
 *  Determine whether a mechanism name is authorized to act as a local name.
 *
 * @param [out] minor  Minor status code
 * @param [in] name    Mechanism name
 * @param [in] user    Local name
 *
 * @a name is a mechanism name, typically the result of a completed
 * gss_accept_sec_context().  @a user is an internal name representing a local
 * name, such as a name imported by gss_import_name() with an @a
 * input_name_type of @c GSS_C_NT_USER_NAME.
 *
 * @return Return GSS_S_COMPLETE if @a name is authorized to act as @a user,
 * GSS_S_UNAUTHORIZED if not, or an appropriate GSS error code if an error
 * occurred.
 *
 * @sa gss_userok
 */
OM_uint32 KRB5_CALLCONV
gss_authorize_localname(OM_uint32 *minor,
                        const gss_name_t name,
                        const gss_name_t user);

OM_uint32 KRB5_CALLCONV
gss_acquire_cred_with_password(
    OM_uint32 *,        /* minor_status */
    const gss_name_t,   /* desired_name */
    const gss_buffer_t, /* password */
    OM_uint32,          /* time_req */
    const gss_OID_set,  /* desired_mechs */
    gss_cred_usage_t,   /* cred_usage */
    gss_cred_id_t *,    /* output_cred_handle */
    gss_OID_set *,      /* actual_mechs */
    OM_uint32 *);       /* time_rec */

OM_uint32 KRB5_CALLCONV
gss_add_cred_with_password(
    OM_uint32 *,        /* minor_status */
    const gss_cred_id_t,/* input_cred_handle */
    const gss_name_t,   /* desired_name */
    const gss_OID,      /* desired_mech */
    const gss_buffer_t, /* password */
    gss_cred_usage_t,   /* cred_usage */
    OM_uint32,          /* initiator_time_req */
    OM_uint32,          /* acceptor_time_req */
    gss_cred_id_t *,    /* output_cred_handle */
    gss_OID_set *,      /* actual_mechs */
    OM_uint32 *,        /* initiator_time_rec */
    OM_uint32 *);       /* acceptor_time_rec */

/*
 * GGF extensions
 */
typedef struct gss_buffer_set_desc_struct {
    size_t count;
    gss_buffer_desc *elements;
} gss_buffer_set_desc, *gss_buffer_set_t;

#define GSS_C_NO_BUFFER_SET ((gss_buffer_set_t) 0)

OM_uint32 KRB5_CALLCONV gss_create_empty_buffer_set
	(OM_uint32 * /*minor_status*/,
	 gss_buffer_set_t * /*buffer_set*/);

OM_uint32 KRB5_CALLCONV gss_add_buffer_set_member
	(OM_uint32 * /*minor_status*/,
	 const gss_buffer_t /*member_buffer*/,
	 gss_buffer_set_t * /*buffer_set*/);

OM_uint32 KRB5_CALLCONV gss_release_buffer_set
	(OM_uint32 * /*minor_status*/,
	 gss_buffer_set_t * /*buffer_set*/);

OM_uint32 KRB5_CALLCONV gss_inquire_sec_context_by_oid
	(OM_uint32 * /*minor_status*/,
	 const gss_ctx_id_t /*context_handle*/,
	 const gss_OID /*desired_object*/,
	 gss_buffer_set_t * /*data_set*/);

OM_uint32 KRB5_CALLCONV gss_inquire_cred_by_oid
	(OM_uint32 * /*minor_status*/,
	 const gss_cred_id_t /*cred_handle*/,
	 const gss_OID /*desired_object*/,
	 gss_buffer_set_t * /*data_set*/);

OM_uint32 KRB5_CALLCONV gss_set_sec_context_option
	(OM_uint32 * /*minor_status*/,
	 gss_ctx_id_t * /*cred_handle*/,
	 const gss_OID /*desired_object*/,
	 const gss_buffer_t /*value*/);

/*
 * Export import cred extensions from GGF, but using Heimdal's signatures
 */
OM_uint32 KRB5_CALLCONV gss_export_cred
	(OM_uint32 * /* minor_status */,
	 gss_cred_id_t /* cred_handle */,
	 gss_buffer_t /* token */);

OM_uint32 KRB5_CALLCONV gss_import_cred
	(OM_uint32 * /* minor_status */,
	 gss_buffer_t /* token */,
	 gss_cred_id_t * /* cred_handle */);

/*
 * Heimdal extension
 */
OM_uint32 KRB5_CALLCONV gss_set_cred_option
	(OM_uint32 * /*minor_status*/,
	 gss_cred_id_t * /*cred*/,
	 const gss_OID /*desired_object*/,
	 const gss_buffer_t /*value*/);

/*
 * Call the given method on the given mechanism
 */
OM_uint32 KRB5_CALLCONV gssspi_mech_invoke
	(OM_uint32 * /*minor_status*/,
	 const gss_OID /*desired_mech*/,
	 const gss_OID /*desired_object*/,
	 gss_buffer_t /*value*/);

/*
 * AEAD extensions
 */

OM_uint32 KRB5_CALLCONV gss_wrap_aead
	(OM_uint32 * /*minor_status*/,
	 gss_ctx_id_t /*context_handle*/,
	 int /*conf_req_flag*/,
	 gss_qop_t /*qop_req*/,
	 gss_buffer_t /*input_assoc_buffer*/,
	 gss_buffer_t /*input_payload_buffer*/,
	 int * /*conf_state*/,
	 gss_buffer_t /*output_message_buffer*/);

OM_uint32 KRB5_CALLCONV gss_unwrap_aead
	(OM_uint32 * /*minor_status*/,
	 gss_ctx_id_t /*context_handle*/,
	 gss_buffer_t /*input_message_buffer*/,
	 gss_buffer_t /*input_assoc_buffer*/,
	 gss_buffer_t /*output_payload_buffer*/,
	 int * /*conf_state*/,
	 gss_qop_t * /*qop_state*/);

/*
 * SSPI extensions
 */
#define GSS_C_DCE_STYLE			0x1000
#define GSS_C_IDENTIFY_FLAG		0x2000
#define GSS_C_EXTENDED_ERROR_FLAG	0x4000

/*
 * Returns a buffer set with the first member containing the
 * session key for SSPI compatibility. The optional second
 * member contains an OID identifying the session key type.
 */
GSS_DLLIMP extern gss_OID GSS_C_INQ_SSPI_SESSION_KEY;

GSS_DLLIMP extern gss_OID GSS_C_INQ_NEGOEX_KEY;
GSS_DLLIMP extern gss_OID GSS_C_INQ_NEGOEX_VERIFY_KEY;

OM_uint32 KRB5_CALLCONV gss_complete_auth_token
	(OM_uint32 *minor_status,
	 const gss_ctx_id_t context_handle,
	 gss_buffer_t input_message_buffer);

typedef struct gss_iov_buffer_desc_struct {
    OM_uint32 type;
    gss_buffer_desc buffer;
} gss_iov_buffer_desc, *gss_iov_buffer_t;

#define GSS_C_NO_IOV_BUFFER		    ((gss_iov_buffer_t)0)

#define GSS_IOV_BUFFER_TYPE_EMPTY	    0
#define GSS_IOV_BUFFER_TYPE_DATA	    1	/* Packet data */
#define GSS_IOV_BUFFER_TYPE_HEADER	    2	/* Mechanism header */
#define GSS_IOV_BUFFER_TYPE_MECH_PARAMS	    3	/* Mechanism specific parameters */
#define GSS_IOV_BUFFER_TYPE_TRAILER	    7	/* Mechanism trailer */
#define GSS_IOV_BUFFER_TYPE_PADDING	    9	/* Padding */
#define GSS_IOV_BUFFER_TYPE_STREAM	    10	/* Complete wrap token */
#define GSS_IOV_BUFFER_TYPE_SIGN_ONLY	    11	/* Sign only packet data */
#define GSS_IOV_BUFFER_TYPE_MIC_TOKEN	    12	/* MIC token destination */

#define GSS_IOV_BUFFER_FLAG_MASK	    0xFFFF0000
#define GSS_IOV_BUFFER_FLAG_ALLOCATE	    0x00010000	/* indicates GSS should allocate */
#define GSS_IOV_BUFFER_FLAG_ALLOCATED	    0x00020000	/* indicates caller should free */

#define GSS_IOV_BUFFER_TYPE(_type)	    ((_type) & ~(GSS_IOV_BUFFER_FLAG_MASK))
#define GSS_IOV_BUFFER_FLAGS(_type)	    ((_type) & GSS_IOV_BUFFER_FLAG_MASK)

/*
 * Sign and optionally encrypt a sequence of buffers. The buffers
 * shall be ordered HEADER | DATA | PADDING | TRAILER. Suitable
 * space for the header, padding and trailer should be provided
 * by calling gss_wrap_iov_length(), or the ALLOCATE flag should
 * be set on those buffers.
 *
 * Encryption is in-place. SIGN_ONLY buffers are untouched. Only
 * a single PADDING buffer should be provided. The order of the
 * buffers in memory does not matter. Buffers in the IOV should
 * be arranged in the order above, and in the case of multiple
 * DATA buffers the sender and receiver should agree on the
 * order.
 *
 * With GSS_C_DCE_STYLE it is acceptable to not provide PADDING
 * and TRAILER, but the caller must guarantee the plaintext data
 * being encrypted is correctly padded, otherwise an error will
 * be returned.
 *
 * While applications that have knowledge of the underlying
 * cryptosystem may request a specific configuration of data
 * buffers, the only generally supported configurations are:
 *
 *  HEADER | DATA | PADDING | TRAILER
 *
 * which will emit GSS_Wrap() compatible tokens, and:
 *
 *  HEADER | SIGN_ONLY | DATA | PADDING | TRAILER
 *
 * for AEAD.
 *
 * The typical (special cased) usage for DCE is as follows:
 *
 *  SIGN_ONLY_1 | DATA | SIGN_ONLY_2 | HEADER
 */
OM_uint32 KRB5_CALLCONV gss_wrap_iov
(
    OM_uint32 *,	/* minor_status */
    gss_ctx_id_t,       /* context_handle */
    int,		/* conf_req_flag */
    gss_qop_t,		/* qop_req */
    int *,		/* conf_state */
    gss_iov_buffer_desc *,    /* iov */
    int);		/* iov_count */

/*
 * Verify and optionally decrypt a sequence of buffers. To process
 * a GSS-API message without separate buffer, pass STREAM | DATA.
 * Upon return DATA will contain the decrypted or integrity
 * protected message. Only a single DATA buffer may be provided
 * with this usage. DATA by default will point into STREAM, but if
 * the ALLOCATE flag is set a copy will be returned.
 *
 * Otherwise, decryption is in-place. SIGN_ONLY buffers are
 * untouched.
 */
OM_uint32 KRB5_CALLCONV gss_unwrap_iov
(
    OM_uint32 *,	/* minor_status */
    gss_ctx_id_t,       /* context_handle */
    int *,		/* conf_state */
    gss_qop_t *,	/* qop_state */
    gss_iov_buffer_desc *,    /* iov */
    int);		/* iov_count */

/*
 * Query HEADER, PADDING and TRAILER buffer lengths. DATA buffers
 * should be provided so the correct padding length can be determined.
 */
OM_uint32 KRB5_CALLCONV gss_wrap_iov_length
(
    OM_uint32 *,	/* minor_status */
    gss_ctx_id_t,	/* context_handle */
    int,		/* conf_req_flag */
    gss_qop_t,		/* qop_req */
    int *,		/* conf_state */
    gss_iov_buffer_desc *, /* iov */
    int);		/* iov_count */

/*
 * Produce a GSSAPI MIC token for a sequence of buffers.  All SIGN_ONLY and
 * DATA buffers will be signed, in the order they appear.  One MIC_TOKEN buffer
 * must be included for the result.  Suitable space should be provided for the
 * MIC_TOKEN buffer by calling gss_get_mic_iov_length, or the ALLOCATE flag
 * should be set on that buffer.  If the ALLOCATE flag is used, use
 * gss_release_iov_buffer to free the allocated buffer within the iov list when
 * it is no longer needed.
 */
OM_uint32 KRB5_CALLCONV gss_get_mic_iov
(
    OM_uint32 *,	/* minor_status */
    gss_ctx_id_t,	/* context_handle */
    gss_qop_t,		/* qop_req */
    gss_iov_buffer_desc *, /* iov */
    int);		/* iov_count */

/*
 * Query the MIC_TOKEN buffer length within the iov list.
 */
OM_uint32 KRB5_CALLCONV gss_get_mic_iov_length(
    OM_uint32 *,	/* minor_status */
    gss_ctx_id_t,	/* context_handle */
    gss_qop_t,		/* qop_req */
    gss_iov_buffer_desc *, /* iov */
    int);		/* iov_count */

/*
 * Verify the MIC_TOKEN buffer within the iov list against the SIGN_ONLY and
 * DATA buffers in the order they appear.  Return values are the same as for
 * gss_verify_mic.
 */
OM_uint32 KRB5_CALLCONV gss_verify_mic_iov
(
    OM_uint32 *,	/* minor_status */
    gss_ctx_id_t,	/* context_handle */
    gss_qop_t *,	/* qop_state */
    gss_iov_buffer_desc *, /* iov */
    int);		/* iov_count */

/*
 * Release buffers that have the ALLOCATED flag set.
 */
OM_uint32 KRB5_CALLCONV gss_release_iov_buffer
(
    OM_uint32 *,	/* minor_status */
    gss_iov_buffer_desc *, /* iov */
    int);		/* iov_count */

/*
 * Protocol transition
 */
OM_uint32 KRB5_CALLCONV
gss_acquire_cred_impersonate_name(
    OM_uint32 *,	    /* minor_status */
    const gss_cred_id_t,    /* impersonator_cred_handle */
    const gss_name_t,	    /* desired_name */
    OM_uint32,		    /* time_req */
    const gss_OID_set,	    /* desired_mechs */
    gss_cred_usage_t,	    /* cred_usage */
    gss_cred_id_t *,	    /* output_cred_handle */
    gss_OID_set *,	    /* actual_mechs */
    OM_uint32 *);	    /* time_rec */

OM_uint32 KRB5_CALLCONV
gss_add_cred_impersonate_name(
    OM_uint32 *,	    /* minor_status */
    gss_cred_id_t,	    /* input_cred_handle */
    const gss_cred_id_t,    /* impersonator_cred_handle */
    const gss_name_t,	    /* desired_name */
    const gss_OID,	    /* desired_mech */
    gss_cred_usage_t,	    /* cred_usage */
    OM_uint32,		    /* initiator_time_req */
    OM_uint32,		    /* acceptor_time_req */
    gss_cred_id_t *,	    /* output_cred_handle */
    gss_OID_set *,	    /* actual_mechs */
    OM_uint32 *,	    /* initiator_time_rec */
    OM_uint32 *);	    /* acceptor_time_rec */

/*
 * Naming extensions
 */
GSS_DLLIMP extern gss_buffer_t GSS_C_ATTR_LOCAL_LOGIN_USER;
GSS_DLLIMP extern gss_OID GSS_C_NT_COMPOSITE_EXPORT;

OM_uint32 KRB5_CALLCONV gss_display_name_ext
(
    OM_uint32 *,	/* minor_status */
    gss_name_t,		/* name */
    gss_OID,		/* display_as_name_type */
    gss_buffer_t	/* display_name */
);

OM_uint32 KRB5_CALLCONV gss_inquire_name
(
    OM_uint32 *,	/* minor_status */
    gss_name_t,		/* name */
    int *,		/* name_is_MN */
    gss_OID *,		/* MN_mech */
    gss_buffer_set_t *	/* attrs */
);

OM_uint32 KRB5_CALLCONV gss_get_name_attribute
(
    OM_uint32 *,	/* minor_status */
    gss_name_t,		/* name */
    gss_buffer_t,	/* attr */
    int *,		/* authenticated */
    int *,		/* complete */
    gss_buffer_t,	/* value */
    gss_buffer_t,	/* display_value */
    int *		/* more */
);

OM_uint32 KRB5_CALLCONV gss_set_name_attribute
(
    OM_uint32 *,	/* minor_status */
    gss_name_t,		/* name */
    int,		/* complete */
    gss_buffer_t,	/* attr */
    gss_buffer_t	/* value */
);

OM_uint32 KRB5_CALLCONV gss_delete_name_attribute
(
    OM_uint32 *,	/* minor_status */
    gss_name_t,		/* name */
    gss_buffer_t	/* attr */
);

OM_uint32 KRB5_CALLCONV gss_export_name_composite
(
    OM_uint32 *,	/* minor_status */
    gss_name_t,		/* name */
    gss_buffer_t	/* exp_composite_name */
);

typedef struct gss_any *gss_any_t;

OM_uint32 KRB5_CALLCONV gss_map_name_to_any
(
    OM_uint32 *,	/* minor_status */
    gss_name_t,		/* name */
    int,		/* authenticated */
    gss_buffer_t,	/* type_id */
    gss_any_t *		/* output */
);

OM_uint32 KRB5_CALLCONV gss_release_any_name_mapping
(
    OM_uint32 *,	/* minor_status */
    gss_name_t,		/* name */
    gss_buffer_t,	/* type_id */
    gss_any_t *		/* input */
);

/* draft-josefsson-gss-capsulate */
OM_uint32 KRB5_CALLCONV gss_encapsulate_token
(
    gss_const_buffer_t, /* input_token */
    gss_const_OID,      /* token_oid */
    gss_buffer_t        /* output_token */
);

OM_uint32 KRB5_CALLCONV gss_decapsulate_token
(
    gss_const_buffer_t, /* input_token */
    gss_const_OID,      /* token_oid */
    gss_buffer_t        /* output_token */
);

int KRB5_CALLCONV gss_oid_equal
(
    gss_const_OID,      /* first_oid */
    gss_const_OID       /* second_oid */
);

/* Credential store extensions */

struct gss_key_value_element_struct {
    const char *key;
    const char *value;
};
typedef struct gss_key_value_element_struct gss_key_value_element_desc;

struct gss_key_value_set_struct {
    OM_uint32 count;
    gss_key_value_element_desc *elements;
};
typedef struct gss_key_value_set_struct gss_key_value_set_desc;
typedef const gss_key_value_set_desc *gss_const_key_value_set_t;

#define GSS_C_NO_CRED_STORE ((gss_const_key_value_set_t) 0)

OM_uint32 KRB5_CALLCONV
gss_acquire_cred_from(
    OM_uint32 *,               /* minor_status */
    gss_name_t,                /* desired_name */
    OM_uint32,                 /* time_req */
    gss_OID_set,               /* desired_mechs */
    gss_cred_usage_t,          /* cred_usage */
    gss_const_key_value_set_t, /* cred_store */
    gss_cred_id_t *,           /* output_cred_handle */
    gss_OID_set *,             /* actual_mechs */
    OM_uint32 *);              /* time_rec */

OM_uint32 KRB5_CALLCONV
gss_add_cred_from(
    OM_uint32 *,               /* minor_status */
    gss_cred_id_t,             /* input_cred_handle */
    gss_name_t,                /* desired_name */
    gss_OID,                   /* desired_mech */
    gss_cred_usage_t,          /* cred_usage */
    OM_uint32,                 /* initiator_time_req */
    OM_uint32,                 /* acceptor_time_req */
    gss_const_key_value_set_t, /* cred_store */
    gss_cred_id_t *,           /* output_cred_handle */
    gss_OID_set *,             /* actual_mechs */
    OM_uint32 *,               /* initiator_time_rec */
    OM_uint32 *);              /* acceptor_time_rec */

OM_uint32 KRB5_CALLCONV
gss_store_cred_into(
    OM_uint32 *,               /* minor_status */
    gss_cred_id_t,             /* input_cred_handle */
    gss_cred_usage_t,          /* input_usage */
    gss_OID,                   /* desired_mech */
    OM_uint32,                 /* overwrite_cred */
    OM_uint32,                 /* default_cred */
    gss_const_key_value_set_t, /* cred_store */
    gss_OID_set *,             /* elements_stored */
    gss_cred_usage_t *);       /* cred_usage_stored */

/*
 * A mech can make itself negotiable via NegoEx (draft-zhu-negoex) by
 * implementing the following three SPIs, and also implementing
 * gss_inquire_sec_context_by_oid() and answering the GSS_C_INQ_NEGOEX_KEY and
 * GSS_C_INQ_NEGOEX_VERIFY_KEY OIDs.  The answer must be in two buffers: the
 * first contains the key contents, and the second contains the key enctype as
 * a four-byte little-endian integer.
 *
 * By default, NegoEx mechanisms will not be directly negotiated via SPNEGO.
 * If direct SPNEGO negotiation is required for interoperability, implement
 * gss_inquire_attrs_for_mech() and assert the GSS_C_MA_NEGOEX_AND_SPNEGO
 * attribute (along with any applicable RFC 5587 attributes).
 */

OM_uint32 KRB5_CALLCONV
gssspi_query_meta_data(
    OM_uint32 *minor_status,
    gss_const_OID mech_oid,
    gss_cred_id_t cred_handle,
    gss_ctx_id_t *context_handle,
    const gss_name_t targ_name,
    OM_uint32 req_flags,
    gss_buffer_t meta_data);

OM_uint32 KRB5_CALLCONV
gssspi_exchange_meta_data(
    OM_uint32 *minor_status,
    gss_const_OID mech_oid,
    gss_cred_id_t cred_handle,
    gss_ctx_id_t *context_handle,
    const gss_name_t targ_name,
    OM_uint32 req_flags,
    gss_const_buffer_t meta_data);

OM_uint32 KRB5_CALLCONV
gssspi_query_mechanism_info(
    OM_uint32 *minor_status,
    gss_const_OID mech_oid,
    unsigned char auth_scheme[16]);

GSS_DLLIMP extern gss_const_OID GSS_C_MA_NEGOEX_AND_SPNEGO;

#ifdef __cplusplus
}
#endif

/*
 * When used with gss_inquire_sec_context_by_oid(), return a buffer set with
 * the first member containing an unsigned 32-bit integer in network byte
 * order.  This is the Security Strength Factor (SSF) associated with the
 * secure channel established by the security context.  NOTE: This value is
 * made available solely as an indication for use by APIs like Cyrus SASL that
 * classify the strength of a secure channel via this number.  The strength of
 * a channel cannot necessarily be represented by a simple number.
 */
GSS_DLLIMP extern gss_OID GSS_C_SEC_CONTEXT_SASL_SSF;

#endif /* GSSAPI_EXT_H_ */
