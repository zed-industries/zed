/* This is the gssapi.h prologue. */
/* no xom.h */
/* End of gssapi.h prologue. */
/* -*- mode: c; indent-tabs-mode: nil -*- */
/*
 * Copyright 1993 by OpenVision Technologies, Inc.
 *
 * Permission to use, copy, modify, distribute, and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appears in all copies and
 * that both that copyright notice and this permission notice appear in
 * supporting documentation, and that the name of OpenVision not be used
 * in advertising or publicity pertaining to distribution of the software
 * without specific, written prior permission. OpenVision makes no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied warranty.
 *
 * OPENVISION DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL OPENVISION BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF
 * USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
 * OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */

#ifndef _GSSAPI_H_
#define _GSSAPI_H_

/*
 * Determine platform-dependent configuration.
 */

#if defined(__MACH__) && defined(__APPLE__)
#       include <TargetConditionals.h>
#       if TARGET_RT_MAC_CFM
#               error "Use KfM 4.0 SDK headers for CFM compilation."
#       endif
#endif

#ifdef __cplusplus
extern "C" {
#endif /* __cplusplus */

#if defined(TARGET_OS_MAC) && TARGET_OS_MAC
#    pragma pack(push,2)
#endif

#if defined(_MSDOS) || defined(_WIN32)
#include <win-mac.h>
#endif

#ifndef KRB5_CALLCONV
#define KRB5_CALLCONV
#define KRB5_CALLCONV_C
#endif

#include <stdint.h>

/*
 * First, include stddef.h to get size_t defined.
 */
#include <stddef.h>

/*
 * POSIX says that sys/types.h is where size_t is defined.
 */
#include <sys/types.h>

/*
 * $Id$
 */

/*
 * First, define the three platform-dependent pointer types.
 */

struct gss_name_struct;
typedef struct gss_name_struct * gss_name_t;

struct gss_cred_id_struct;
typedef struct gss_cred_id_struct * gss_cred_id_t;

struct gss_ctx_id_struct;
typedef struct gss_ctx_id_struct * gss_ctx_id_t;

/*
 * The following type must be defined as the smallest natural unsigned integer
 * supported by the platform that has at least 32 bits of precision.
 */
typedef uint32_t gss_uint32;
typedef int32_t gss_int32;

#ifdef  OM_STRING
/*
 * We have included the xom.h header file.  Use the definition for
 * OM_object identifier.
 */
typedef OM_object_identifier    gss_OID_desc, *gss_OID;
#else   /* OM_STRING */
/*
 * We can't use X/Open definitions, so roll our own.
 */
typedef gss_uint32      OM_uint32;

typedef struct gss_OID_desc_struct {
    OM_uint32 length;
    void *elements;
} gss_OID_desc, *gss_OID;
#endif  /* OM_STRING */

typedef struct gss_OID_set_desc_struct  {
    size_t  count;
    gss_OID elements;
} gss_OID_set_desc, *gss_OID_set;

typedef struct gss_buffer_desc_struct {
    size_t length;
    void *value;
} gss_buffer_desc, *gss_buffer_t;

typedef struct gss_channel_bindings_struct {
    OM_uint32 initiator_addrtype;
    gss_buffer_desc initiator_address;
    OM_uint32 acceptor_addrtype;
    gss_buffer_desc acceptor_address;
    gss_buffer_desc application_data;
} *gss_channel_bindings_t;

/*
 * For now, define a QOP-type as an OM_uint32 (pending resolution of ongoing
 * discussions).
 */
typedef OM_uint32       gss_qop_t;
typedef int             gss_cred_usage_t;

/*
 * Flag bits for context-level services.
 */
#define GSS_C_DELEG_FLAG        1
#define GSS_C_MUTUAL_FLAG       2
#define GSS_C_REPLAY_FLAG       4
#define GSS_C_SEQUENCE_FLAG     8
#define GSS_C_CONF_FLAG         16
#define GSS_C_INTEG_FLAG        32
#define GSS_C_ANON_FLAG         64
#define GSS_C_PROT_READY_FLAG   128
#define GSS_C_TRANS_FLAG        256
#define GSS_C_DELEG_POLICY_FLAG 32768

/*
 * Credential usage options
 */
#define GSS_C_BOTH      0
#define GSS_C_INITIATE  1
#define GSS_C_ACCEPT    2

/*
 * Status code types for gss_display_status
 */
#define GSS_C_GSS_CODE  1
#define GSS_C_MECH_CODE 2

/*
 * The constant definitions for channel-bindings address families
 */
#define GSS_C_AF_UNSPEC     0
#define GSS_C_AF_LOCAL      1
#define GSS_C_AF_INET       2
#define GSS_C_AF_IMPLINK    3
#define GSS_C_AF_PUP        4
#define GSS_C_AF_CHAOS      5
#define GSS_C_AF_NS         6
#define GSS_C_AF_NBS        7
#define GSS_C_AF_ECMA       8
#define GSS_C_AF_DATAKIT    9
#define GSS_C_AF_CCITT      10
#define GSS_C_AF_SNA        11
#define GSS_C_AF_DECnet     12
#define GSS_C_AF_DLI        13
#define GSS_C_AF_LAT        14
#define GSS_C_AF_HYLINK     15
#define GSS_C_AF_APPLETALK  16
#define GSS_C_AF_BSC        17
#define GSS_C_AF_DSS        18
#define GSS_C_AF_OSI        19
#define GSS_C_AF_NETBIOS    20
#define GSS_C_AF_X25        21

#define GSS_C_AF_NULLADDR   255

/*
 * Various Null values.
 */
#define GSS_C_NO_NAME ((gss_name_t) 0)
#define GSS_C_NO_BUFFER ((gss_buffer_t) 0)
#define GSS_C_NO_OID ((gss_OID) 0)
#define GSS_C_NO_OID_SET ((gss_OID_set) 0)
#define GSS_C_NO_CONTEXT ((gss_ctx_id_t) 0)
#define GSS_C_NO_CREDENTIAL ((gss_cred_id_t) 0)
#define GSS_C_NO_CHANNEL_BINDINGS ((gss_channel_bindings_t) 0)
#define GSS_C_EMPTY_BUFFER {0, NULL}

/*
 * Some alternate names for a couple of the above values.  These are defined
 * for V1 compatibility.
 */
#define GSS_C_NULL_OID          GSS_C_NO_OID
#define GSS_C_NULL_OID_SET      GSS_C_NO_OID_SET

/*
 * Define the default Quality of Protection for per-message services.  Note
 * that an implementation that offers multiple levels of QOP may either reserve
 * a value (for example zero, as assumed here) to mean "default protection", or
 * alternatively may simply equate GSS_C_QOP_DEFAULT to a specific explicit
 * QOP value.  However a value of 0 should always be interpreted by a GSSAPI
 * implementation as a request for the default protection level.
 */
#define GSS_C_QOP_DEFAULT 0

/*
 * Expiration time of 2^32-1 seconds means infinite lifetime for a
 * credential or security context
 */
#define GSS_C_INDEFINITE ((OM_uint32) 0xfffffffful)


/* Major status codes */

#define GSS_S_COMPLETE 0

/*
 * Some "helper" definitions to make the status code macros obvious.
 */
#define GSS_C_CALLING_ERROR_OFFSET 24
#define GSS_C_ROUTINE_ERROR_OFFSET 16
#define GSS_C_SUPPLEMENTARY_OFFSET 0
#define GSS_C_CALLING_ERROR_MASK ((OM_uint32) 0377ul)
#define GSS_C_ROUTINE_ERROR_MASK ((OM_uint32) 0377ul)
#define GSS_C_SUPPLEMENTARY_MASK ((OM_uint32) 0177777ul)

/*
 * The macros that test status codes for error conditions.  Note that the
 * GSS_ERROR() macro has changed slightly from the V1 GSSAPI so that it now
 * evaluates its argument only once.
 */
#define GSS_CALLING_ERROR(x) \
  ((x) & (GSS_C_CALLING_ERROR_MASK << GSS_C_CALLING_ERROR_OFFSET))
#define GSS_ROUTINE_ERROR(x) \
  ((x) & (GSS_C_ROUTINE_ERROR_MASK << GSS_C_ROUTINE_ERROR_OFFSET))
#define GSS_SUPPLEMENTARY_INFO(x) \
  ((x) & (GSS_C_SUPPLEMENTARY_MASK << GSS_C_SUPPLEMENTARY_OFFSET))
#define GSS_ERROR(x) \
  ((x) & ((GSS_C_CALLING_ERROR_MASK << GSS_C_CALLING_ERROR_OFFSET) | \
          (GSS_C_ROUTINE_ERROR_MASK << GSS_C_ROUTINE_ERROR_OFFSET)))

/*
 * Now the actual status code definitions
 */

/*
 * Calling errors:
 */
#define GSS_S_CALL_INACCESSIBLE_READ \
                             (((OM_uint32) 1ul) << GSS_C_CALLING_ERROR_OFFSET)
#define GSS_S_CALL_INACCESSIBLE_WRITE \
                             (((OM_uint32) 2ul) << GSS_C_CALLING_ERROR_OFFSET)
#define GSS_S_CALL_BAD_STRUCTURE \
                             (((OM_uint32) 3ul) << GSS_C_CALLING_ERROR_OFFSET)

/*
 * Routine errors:
 */
#define GSS_S_BAD_MECH (((OM_uint32) 1ul) << GSS_C_ROUTINE_ERROR_OFFSET)
#define GSS_S_BAD_NAME (((OM_uint32) 2ul) << GSS_C_ROUTINE_ERROR_OFFSET)
#define GSS_S_BAD_NAMETYPE (((OM_uint32) 3ul) << GSS_C_ROUTINE_ERROR_OFFSET)
#define GSS_S_BAD_BINDINGS (((OM_uint32) 4ul) << GSS_C_ROUTINE_ERROR_OFFSET)
#define GSS_S_BAD_STATUS (((OM_uint32) 5ul) << GSS_C_ROUTINE_ERROR_OFFSET)
#define GSS_S_BAD_SIG (((OM_uint32) 6ul) << GSS_C_ROUTINE_ERROR_OFFSET)
#define GSS_S_BAD_MIC GSS_S_BAD_SIG
#define GSS_S_NO_CRED (((OM_uint32) 7ul) << GSS_C_ROUTINE_ERROR_OFFSET)
#define GSS_S_NO_CONTEXT (((OM_uint32) 8ul) << GSS_C_ROUTINE_ERROR_OFFSET)
#define GSS_S_DEFECTIVE_TOKEN (((OM_uint32) 9ul) << GSS_C_ROUTINE_ERROR_OFFSET)
#define GSS_S_DEFECTIVE_CREDENTIAL \
     (((OM_uint32) 10ul) << GSS_C_ROUTINE_ERROR_OFFSET)
#define GSS_S_CREDENTIALS_EXPIRED \
     (((OM_uint32) 11ul) << GSS_C_ROUTINE_ERROR_OFFSET)
#define GSS_S_CONTEXT_EXPIRED \
     (((OM_uint32) 12ul) << GSS_C_ROUTINE_ERROR_OFFSET)
#define GSS_S_FAILURE (((OM_uint32) 13ul) << GSS_C_ROUTINE_ERROR_OFFSET)
#define GSS_S_BAD_QOP (((OM_uint32) 14ul) << GSS_C_ROUTINE_ERROR_OFFSET)
#define GSS_S_UNAUTHORIZED (((OM_uint32) 15ul) << GSS_C_ROUTINE_ERROR_OFFSET)
#define GSS_S_UNAVAILABLE (((OM_uint32) 16ul) << GSS_C_ROUTINE_ERROR_OFFSET)
#define GSS_S_DUPLICATE_ELEMENT \
     (((OM_uint32) 17ul) << GSS_C_ROUTINE_ERROR_OFFSET)
#define GSS_S_NAME_NOT_MN \
     (((OM_uint32) 18ul) << GSS_C_ROUTINE_ERROR_OFFSET)
#define GSS_S_BAD_MECH_ATTR \
     (((OM_uint32) 19ul) << GSS_C_ROUTINE_ERROR_OFFSET)

/*
 * Supplementary info bits:
 */
#define GSS_S_CONTINUE_NEEDED (1 << (GSS_C_SUPPLEMENTARY_OFFSET + 0))
#define GSS_S_DUPLICATE_TOKEN (1 << (GSS_C_SUPPLEMENTARY_OFFSET + 1))
#define GSS_S_OLD_TOKEN (1 << (GSS_C_SUPPLEMENTARY_OFFSET + 2))
#define GSS_S_UNSEQ_TOKEN (1 << (GSS_C_SUPPLEMENTARY_OFFSET + 3))
#define GSS_S_GAP_TOKEN (1 << (GSS_C_SUPPLEMENTARY_OFFSET + 4))


/*
 * Finally, function prototypes for the GSSAPI routines.
 */

#if defined (_WIN32) && defined (_MSC_VER)
# ifdef GSS_DLL_FILE
#  define GSS_DLLIMP __declspec(dllexport)
# else
#  define GSS_DLLIMP __declspec(dllimport)
# endif
#else
# define GSS_DLLIMP
#endif

/* Reserved static storage for GSS_oids.  Comments are quotes from RFC 2744.
 *
 * The implementation must reserve static storage for a
 * gss_OID_desc object containing the value
 * {10, (void *)"\x2a\x86\x48\x86\xf7\x12\x01\x02\x01\x01"},
 * corresponding to an object-identifier value of
 * {iso(1) member-body(2) United States(840) mit(113554)
 * infosys(1) gssapi(2) generic(1) user_name(1)}.  The constant
 * GSS_C_NT_USER_NAME should be initialized to point
 * to that gss_OID_desc.
 */
GSS_DLLIMP extern gss_OID GSS_C_NT_USER_NAME;

/*
 * The implementation must reserve static storage for a
 * gss_OID_desc object containing the value
 * {10, (void *)"\x2a\x86\x48\x86\xf7\x12\x01\x02\x01\x02"},
 * corresponding to an object-identifier value of
 * {iso(1) member-body(2) United States(840) mit(113554)
 * infosys(1) gssapi(2) generic(1) machine_uid_name(2)}.
 * The constant GSS_C_NT_MACHINE_UID_NAME should be
 * initialized to point to that gss_OID_desc.
 */
GSS_DLLIMP extern gss_OID GSS_C_NT_MACHINE_UID_NAME;

/*
 * The implementation must reserve static storage for a
 * gss_OID_desc object containing the value
 * {10, (void *)"\x2a\x86\x48\x86\xf7\x12\x01\x02\x01\x03"},
 * corresponding to an object-identifier value of
 * {iso(1) member-body(2) United States(840) mit(113554)
 * infosys(1) gssapi(2) generic(1) string_uid_name(3)}.
 * The constant GSS_C_NT_STRING_UID_NAME should be
 * initialized to point to that gss_OID_desc.
 */
GSS_DLLIMP extern gss_OID GSS_C_NT_STRING_UID_NAME;

/*
 * The implementation must reserve static storage for a
 * gss_OID_desc object containing the value
 * {6, (void *)"\x2b\x06\x01\x05\x06\x02"},
 * corresponding to an object-identifier value of
 * {iso(1) org(3) dod(6) internet(1) security(5)
 * nametypes(6) gss-host-based-services(2)).  The constant
 * GSS_C_NT_HOSTBASED_SERVICE_X should be initialized to point
 * to that gss_OID_desc.  This is a deprecated OID value, and
 * implementations wishing to support hostbased-service names
 * should instead use the GSS_C_NT_HOSTBASED_SERVICE OID,
 * defined below, to identify such names;
 * GSS_C_NT_HOSTBASED_SERVICE_X should be accepted a synonym
 * for GSS_C_NT_HOSTBASED_SERVICE when presented as an input
 * parameter, but should not be emitted by GSS-API
 * implementations
 */
GSS_DLLIMP extern gss_OID GSS_C_NT_HOSTBASED_SERVICE_X;

/*
 * The implementation must reserve static storage for a
 * gss_OID_desc object containing the value
 * {10, (void *)"\x2a\x86\x48\x86\xf7\x12"
 *              "\x01\x02\x01\x04"}, corresponding to an
 * object-identifier value of {iso(1) member-body(2)
 * Unites States(840) mit(113554) infosys(1) gssapi(2)
 * generic(1) service_name(4)}.  The constant
 * GSS_C_NT_HOSTBASED_SERVICE should be initialized
 * to point to that gss_OID_desc.
 */
GSS_DLLIMP extern gss_OID GSS_C_NT_HOSTBASED_SERVICE;

/*
 * The implementation must reserve static storage for a
 * gss_OID_desc object containing the value
 * {6, (void *)"\x2b\x06\01\x05\x06\x03"},
 * corresponding to an object identifier value of
 * {1(iso), 3(org), 6(dod), 1(internet), 5(security),
 * 6(nametypes), 3(gss-anonymous-name)}.  The constant
 * and GSS_C_NT_ANONYMOUS should be initialized to point
 * to that gss_OID_desc.
 */
GSS_DLLIMP extern gss_OID GSS_C_NT_ANONYMOUS;


/*
 * The implementation must reserve static storage for a
 * gss_OID_desc object containing the value
 * {6, (void *)"\x2b\x06\x01\x05\x06\x04"},
 * corresponding to an object-identifier value of
 * {1(iso), 3(org), 6(dod), 1(internet), 5(security),
 * 6(nametypes), 4(gss-api-exported-name)}.  The constant
 * GSS_C_NT_EXPORT_NAME should be initialized to point
 * to that gss_OID_desc.
 */
GSS_DLLIMP extern gss_OID GSS_C_NT_EXPORT_NAME;

/* Function Prototypes */

OM_uint32 KRB5_CALLCONV
gss_acquire_cred(
    OM_uint32 *,        /* minor_status */
    gss_name_t,         /* desired_name */
    OM_uint32,          /* time_req */
    gss_OID_set,        /* desired_mechs */
    gss_cred_usage_t,   /* cred_usage */
    gss_cred_id_t *,    /* output_cred_handle */
    gss_OID_set *,      /* actual_mechs */
    OM_uint32 *);       /* time_rec */

OM_uint32 KRB5_CALLCONV
gss_release_cred(
    OM_uint32 *,        /* minor_status */
    gss_cred_id_t *);   /* cred_handle */

OM_uint32 KRB5_CALLCONV
gss_init_sec_context(
    OM_uint32 *,        /* minor_status */
    gss_cred_id_t,      /* claimant_cred_handle */
    gss_ctx_id_t *,     /* context_handle */
    gss_name_t,         /* target_name */
    gss_OID,            /* mech_type (used to be const) */
    OM_uint32,          /* req_flags */
    OM_uint32,          /* time_req */
    gss_channel_bindings_t,     /* input_chan_bindings */
    gss_buffer_t,       /* input_token */
    gss_OID *,          /* actual_mech_type */
    gss_buffer_t,       /* output_token */
    OM_uint32 *,        /* ret_flags */
    OM_uint32 *);       /* time_rec */

OM_uint32 KRB5_CALLCONV
gss_accept_sec_context(
    OM_uint32 *,                /* minor_status */
    gss_ctx_id_t *,             /* context_handle */
    gss_cred_id_t,              /* acceptor_cred_handle */
    gss_buffer_t,               /* input_token_buffer */
    gss_channel_bindings_t,     /* input_chan_bindings */
    gss_name_t *,               /* src_name */
    gss_OID *,                  /* mech_type */
    gss_buffer_t,               /* output_token */
    OM_uint32 *,                /* ret_flags */
    OM_uint32 *,                /* time_rec */
    gss_cred_id_t *);           /* delegated_cred_handle */

OM_uint32 KRB5_CALLCONV
gss_process_context_token(
    OM_uint32 *,        /* minor_status */
    gss_ctx_id_t,       /* context_handle */
    gss_buffer_t);      /* token_buffer */


OM_uint32 KRB5_CALLCONV
gss_delete_sec_context(
    OM_uint32 *,        /* minor_status */
    gss_ctx_id_t *,     /* context_handle */
    gss_buffer_t);      /* output_token */


OM_uint32 KRB5_CALLCONV
gss_context_time(
    OM_uint32 *,        /* minor_status */
    gss_ctx_id_t,       /* context_handle */
    OM_uint32 *);       /* time_rec */


/* New for V2 */
OM_uint32 KRB5_CALLCONV
gss_get_mic(
    OM_uint32 *,        /* minor_status */
    gss_ctx_id_t,       /* context_handle */
    gss_qop_t,          /* qop_req */
    gss_buffer_t,       /* message_buffer */
    gss_buffer_t);      /* message_token */


/* New for V2 */
OM_uint32 KRB5_CALLCONV
gss_verify_mic(OM_uint32 *,     /* minor_status */
               gss_ctx_id_t,    /* context_handle */
               gss_buffer_t,    /* message_buffer */
               gss_buffer_t,    /* message_token */
               gss_qop_t *      /* qop_state */
);

/* New for V2 */
OM_uint32 KRB5_CALLCONV
gss_wrap(
    OM_uint32 *,        /* minor_status */
    gss_ctx_id_t,       /* context_handle */
    int,                /* conf_req_flag */
    gss_qop_t,          /* qop_req */
    gss_buffer_t,       /* input_message_buffer */
    int *,              /* conf_state */
    gss_buffer_t);      /* output_message_buffer */


/* New for V2 */
OM_uint32 KRB5_CALLCONV
gss_unwrap(
    OM_uint32 *,        /* minor_status */
    gss_ctx_id_t,       /* context_handle */
    gss_buffer_t,       /* input_message_buffer */
    gss_buffer_t,       /* output_message_buffer */
    int *,              /* conf_state */
    gss_qop_t *);       /* qop_state */


OM_uint32 KRB5_CALLCONV
gss_display_status(
    OM_uint32 *,        /* minor_status */
    OM_uint32,          /* status_value */
    int,                /* status_type */
    gss_OID,            /* mech_type (used to be const) */
    OM_uint32 *,        /* message_context */
    gss_buffer_t);      /* status_string */


OM_uint32 KRB5_CALLCONV
gss_indicate_mechs(
    OM_uint32 *,        /* minor_status */
    gss_OID_set *);     /* mech_set */


OM_uint32 KRB5_CALLCONV
gss_compare_name(
    OM_uint32 *,        /* minor_status */
    gss_name_t,         /* name1 */
    gss_name_t,         /* name2 */
    int *);             /* name_equal */


OM_uint32 KRB5_CALLCONV
gss_display_name(
    OM_uint32 *,        /* minor_status */
    gss_name_t,         /* input_name */
    gss_buffer_t,       /* output_name_buffer */
    gss_OID *);         /* output_name_type */


OM_uint32 KRB5_CALLCONV
gss_import_name(
    OM_uint32 *,        /* minor_status */
    gss_buffer_t,       /* input_name_buffer */
    gss_OID,            /* input_name_type(used to be const) */
    gss_name_t *);      /* output_name */

OM_uint32 KRB5_CALLCONV
gss_release_name(
    OM_uint32 *,        /* minor_status */
    gss_name_t *);      /* input_name */

OM_uint32 KRB5_CALLCONV
gss_release_buffer(
    OM_uint32 *,        /* minor_status */
    gss_buffer_t);      /* buffer */

OM_uint32 KRB5_CALLCONV
gss_release_oid_set(
    OM_uint32 *,        /* minor_status */
    gss_OID_set *);     /* set */

OM_uint32 KRB5_CALLCONV
gss_inquire_cred(
    OM_uint32 *,        /* minor_status */
    gss_cred_id_t,      /* cred_handle */
    gss_name_t *,       /* name */
    OM_uint32 *,        /* lifetime */
    gss_cred_usage_t *, /* cred_usage */
    gss_OID_set *);     /* mechanisms */

/* Last argument new for V2 */
OM_uint32 KRB5_CALLCONV
gss_inquire_context(
    OM_uint32 *,        /* minor_status */
    gss_ctx_id_t,       /* context_handle */
    gss_name_t *,       /* src_name */
    gss_name_t *,       /* targ_name */
    OM_uint32 *,        /* lifetime_rec */
    gss_OID *,          /* mech_type */
    OM_uint32 *,        /* ctx_flags */
    int *,              /* locally_initiated */
    int *);             /* open */

/* New for V2 */
OM_uint32 KRB5_CALLCONV
gss_wrap_size_limit(
    OM_uint32 *,        /* minor_status */
    gss_ctx_id_t,       /* context_handle */
    int,                /* conf_req_flag */
    gss_qop_t,          /* qop_req */
    OM_uint32,          /* req_output_size */
    OM_uint32 *);       /* max_input_size */

/* New for V2 */
OM_uint32 KRB5_CALLCONV
gss_import_name_object(
    OM_uint32 *,        /* minor_status */
    void *,             /* input_name */
    gss_OID,            /* input_name_type */
    gss_name_t *);      /* output_name */

/* New for V2 */
OM_uint32 KRB5_CALLCONV
gss_export_name_object(
    OM_uint32 *,        /* minor_status */
    gss_name_t,         /* input_name */
    gss_OID,            /* desired_name_type */
    void **);           /* output_name */

/* New for V2 */
OM_uint32 KRB5_CALLCONV
gss_add_cred(
    OM_uint32 *,        /* minor_status */
    gss_cred_id_t,      /* input_cred_handle */
    gss_name_t,         /* desired_name */
    gss_OID,            /* desired_mech */
    gss_cred_usage_t,   /* cred_usage */
    OM_uint32,          /* initiator_time_req */
    OM_uint32,          /* acceptor_time_req */
    gss_cred_id_t *,    /* output_cred_handle */
    gss_OID_set *,      /* actual_mechs */
    OM_uint32 *,        /* initiator_time_rec */
    OM_uint32 *);       /* acceptor_time_rec */

/* New for V2 */
OM_uint32 KRB5_CALLCONV
gss_inquire_cred_by_mech(
    OM_uint32 *,                /* minor_status */
    gss_cred_id_t,              /* cred_handle */
    gss_OID,                    /* mech_type */
    gss_name_t *,               /* name */
    OM_uint32 *,                /* initiator_lifetime */
    OM_uint32 *,                /* acceptor_lifetime */
    gss_cred_usage_t *);        /* cred_usage */

/* New for V2 */
OM_uint32 KRB5_CALLCONV
gss_export_sec_context(
    OM_uint32 *,        /* minor_status */
    gss_ctx_id_t *,     /* context_handle */
    gss_buffer_t);      /* interprocess_token */

/* New for V2 */
OM_uint32 KRB5_CALLCONV
gss_import_sec_context(
    OM_uint32 *,        /* minor_status */
    gss_buffer_t,       /* interprocess_token */
    gss_ctx_id_t *);    /* context_handle */

/* New for V2 */
OM_uint32 KRB5_CALLCONV
gss_release_oid(
    OM_uint32 *,        /* minor_status */
    gss_OID *);         /* oid */

/* New for V2 */
OM_uint32 KRB5_CALLCONV
gss_create_empty_oid_set(
    OM_uint32 *,        /* minor_status */
    gss_OID_set *);     /* oid_set */

/* New for V2 */
OM_uint32 KRB5_CALLCONV
gss_add_oid_set_member(
    OM_uint32 *,        /* minor_status */
    gss_OID,            /* member_oid */
    gss_OID_set *);     /* oid_set */

/* New for V2 */
OM_uint32 KRB5_CALLCONV
gss_test_oid_set_member(
    OM_uint32 *,        /* minor_status */
    gss_OID,            /* member */
    gss_OID_set,        /* set */
    int *);             /* present */

/* New for V2 */
OM_uint32 KRB5_CALLCONV
gss_str_to_oid(
    OM_uint32 *,        /* minor_status */
    gss_buffer_t,       /* oid_str */
    gss_OID *);         /* oid */

/* New for V2 */
OM_uint32 KRB5_CALLCONV
gss_oid_to_str(
    OM_uint32 *,        /* minor_status */
    gss_OID,            /* oid */
    gss_buffer_t);      /* oid_str */

/* New for V2 */
OM_uint32 KRB5_CALLCONV
gss_inquire_names_for_mech(
    OM_uint32 *,        /* minor_status */
    gss_OID,            /* mechanism */
    gss_OID_set *);     /* name_types */

/* New for V2 */
OM_uint32 KRB5_CALLCONV
gss_inquire_mechs_for_name(
    OM_uint32 *,        /* minor_status */
    const gss_name_t,   /* input_name */
    gss_OID_set *);     /* mech_types */

/*
 * The following routines are obsolete variants of gss_get_mic, gss_wrap,
 * gss_verify_mic and gss_unwrap.  They should be provided by GSSAPI V2
 * implementations for backwards compatibility with V1 applications.  Distinct
 * entrypoints (as opposed to #defines) should be provided, to allow GSSAPI
 * V1 applications to link against GSSAPI V2 implementations.
 */
OM_uint32 KRB5_CALLCONV
gss_sign(
    OM_uint32 *,        /* minor_status */
    gss_ctx_id_t,       /* context_handle */
    int,                /* qop_req */
    gss_buffer_t,       /* message_buffer */
    gss_buffer_t);      /* message_token */

OM_uint32 KRB5_CALLCONV
gss_verify(
    OM_uint32 *,        /* minor_status */
    gss_ctx_id_t,       /* context_handle */
    gss_buffer_t,       /* message_buffer */
    gss_buffer_t,       /* token_buffer */
    int *);             /* qop_state */

OM_uint32 KRB5_CALLCONV
gss_seal(
    OM_uint32 *,        /* minor_status */
    gss_ctx_id_t,       /* context_handle */
    int,                /* conf_req_flag */
    int,                /* qop_req */
    gss_buffer_t,       /* input_message_buffer */
    int *,              /* conf_state */
    gss_buffer_t);      /* output_message_buffer */

OM_uint32 KRB5_CALLCONV
gss_unseal(
    OM_uint32 *,        /* minor_status */
    gss_ctx_id_t,       /* context_handle */
    gss_buffer_t,       /* input_message_buffer */
    gss_buffer_t,       /* output_message_buffer */
    int *,              /* conf_state */
    int *);             /* qop_state */

/* New for V2 */
OM_uint32 KRB5_CALLCONV
gss_export_name(
    OM_uint32  *,       /* minor_status */
    const gss_name_t,   /* input_name */
    gss_buffer_t);      /* exported_name */

/* New for V2 */
OM_uint32 KRB5_CALLCONV
gss_duplicate_name(
    OM_uint32  *,       /* minor_status */
    const gss_name_t,   /* input_name */
    gss_name_t *);      /* dest_name */

/* New for V2 */
OM_uint32 KRB5_CALLCONV
gss_canonicalize_name(
    OM_uint32  *,       /* minor_status */
    const gss_name_t,   /* input_name */
    const gss_OID,      /* mech_type */
    gss_name_t *);      /* output_name */

/* RFC 4401 */

#define GSS_C_PRF_KEY_FULL      0
#define GSS_C_PRF_KEY_PARTIAL   1

OM_uint32 KRB5_CALLCONV
gss_pseudo_random(
    OM_uint32 *,        /* minor_status */
    gss_ctx_id_t,       /* context */
    int,                /* prf_key */
    const gss_buffer_t, /* prf_in */
    ssize_t,            /* desired_output_len */
    gss_buffer_t);      /* prf_out */

OM_uint32 KRB5_CALLCONV
gss_store_cred(
    OM_uint32 *,        /* minor_status */
    const gss_cred_id_t,/* input_cred_handle */
    gss_cred_usage_t,   /* input_usage */
    const gss_OID,      /* desired_mech */
    OM_uint32,          /* overwrite_cred */
    OM_uint32,          /* default_cred */
    gss_OID_set *,      /* elements_stored */
    gss_cred_usage_t *);/* cred_usage_stored */

OM_uint32 KRB5_CALLCONV
gss_set_neg_mechs(
    OM_uint32 *,        /* minor_status */
    gss_cred_id_t,      /* cred_handle */
    const gss_OID_set); /* mech_set */

#if defined(TARGET_OS_MAC) && TARGET_OS_MAC
#    pragma pack(pop)
#endif

#ifdef __cplusplus
}
#endif

/* XXXX these are not part of the GSSAPI C bindings!  (but should be) */

#define GSS_CALLING_ERROR_FIELD(x) \
   (((x) >> GSS_C_CALLING_ERROR_OFFSET) & GSS_C_CALLING_ERROR_MASK)
#define GSS_ROUTINE_ERROR_FIELD(x) \
   (((x) >> GSS_C_ROUTINE_ERROR_OFFSET) & GSS_C_ROUTINE_ERROR_MASK)
#define GSS_SUPPLEMENTARY_INFO_FIELD(x) \
   (((x) >> GSS_C_SUPPLEMENTARY_OFFSET) & GSS_C_SUPPLEMENTARY_MASK)

/* XXXX This is a necessary evil until the spec is fixed */
#define GSS_S_CRED_UNAVAIL GSS_S_FAILURE

/*
 * RFC 5587
 */
typedef const gss_buffer_desc *gss_const_buffer_t;
typedef const struct gss_channel_bindings_struct *gss_const_channel_bindings_t;
typedef const struct gss_ctx_id_struct *gss_const_ctx_id_t;
typedef const struct gss_cred_id_struct *gss_const_cred_id_t;
typedef const struct gss_name_struct *gss_const_name_t;
typedef const gss_OID_desc *gss_const_OID;
typedef const gss_OID_set_desc *gss_const_OID_set;

OM_uint32 KRB5_CALLCONV
gss_indicate_mechs_by_attrs(
    OM_uint32 *,        /* minor_status */
    gss_const_OID_set,  /* desired_mech_attrs */
    gss_const_OID_set,  /* except_mech_attrs */
    gss_const_OID_set,  /* critical_mech_attrs */
    gss_OID_set *);     /* mechs */

OM_uint32 KRB5_CALLCONV
gss_inquire_attrs_for_mech(
    OM_uint32 *,        /* minor_status */
    gss_const_OID,      /* mech */
    gss_OID_set *,      /* mech_attrs */
    gss_OID_set *);     /* known_mech_attrs */

OM_uint32 KRB5_CALLCONV
gss_display_mech_attr(
    OM_uint32 *,        /* minor_status */
    gss_const_OID,      /* mech_attr */
    gss_buffer_t,       /* name */
    gss_buffer_t,       /* short_desc */
    gss_buffer_t);      /* long_desc */

GSS_DLLIMP extern gss_const_OID GSS_C_MA_MECH_CONCRETE;
GSS_DLLIMP extern gss_const_OID GSS_C_MA_MECH_PSEUDO;
GSS_DLLIMP extern gss_const_OID GSS_C_MA_MECH_COMPOSITE;
GSS_DLLIMP extern gss_const_OID GSS_C_MA_MECH_NEGO;
GSS_DLLIMP extern gss_const_OID GSS_C_MA_MECH_GLUE;
GSS_DLLIMP extern gss_const_OID GSS_C_MA_NOT_MECH;
GSS_DLLIMP extern gss_const_OID GSS_C_MA_DEPRECATED;
GSS_DLLIMP extern gss_const_OID GSS_C_MA_NOT_DFLT_MECH;
GSS_DLLIMP extern gss_const_OID GSS_C_MA_ITOK_FRAMED;
GSS_DLLIMP extern gss_const_OID GSS_C_MA_AUTH_INIT;
GSS_DLLIMP extern gss_const_OID GSS_C_MA_AUTH_TARG;
GSS_DLLIMP extern gss_const_OID GSS_C_MA_AUTH_INIT_INIT;
GSS_DLLIMP extern gss_const_OID GSS_C_MA_AUTH_TARG_INIT;
GSS_DLLIMP extern gss_const_OID GSS_C_MA_AUTH_INIT_ANON;
GSS_DLLIMP extern gss_const_OID GSS_C_MA_AUTH_TARG_ANON;
GSS_DLLIMP extern gss_const_OID GSS_C_MA_DELEG_CRED;
GSS_DLLIMP extern gss_const_OID GSS_C_MA_INTEG_PROT;
GSS_DLLIMP extern gss_const_OID GSS_C_MA_CONF_PROT;
GSS_DLLIMP extern gss_const_OID GSS_C_MA_MIC;
GSS_DLLIMP extern gss_const_OID GSS_C_MA_WRAP;
GSS_DLLIMP extern gss_const_OID GSS_C_MA_PROT_READY;
GSS_DLLIMP extern gss_const_OID GSS_C_MA_REPLAY_DET;
GSS_DLLIMP extern gss_const_OID GSS_C_MA_OOS_DET;
GSS_DLLIMP extern gss_const_OID GSS_C_MA_CBINDINGS;
GSS_DLLIMP extern gss_const_OID GSS_C_MA_PFS;
GSS_DLLIMP extern gss_const_OID GSS_C_MA_COMPRESS;
GSS_DLLIMP extern gss_const_OID GSS_C_MA_CTX_TRANS;

/*
 * RFC 5801
 */
OM_uint32 KRB5_CALLCONV
gss_inquire_saslname_for_mech(
    OM_uint32 *,        /* minor_status */
    const gss_OID,      /* desired_mech */
    gss_buffer_t,       /* sasl_mech_name */
    gss_buffer_t,       /* mech_name */
    gss_buffer_t        /* mech_description */
);

OM_uint32 KRB5_CALLCONV
gss_inquire_mech_for_saslname(
    OM_uint32 *,        /* minor_status */
    const gss_buffer_t, /* sasl_mech_name */
    gss_OID *           /* mech_type */
);

#endif /* _GSSAPI_H_ */
