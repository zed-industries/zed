#ifndef SNMP_API_H
#define SNMP_API_H

/*
 * snmp_api.h - API for access to snmp.
 *
 * Caution: when using this library in a multi-threaded application,
 * the values of global variables "snmp_errno" and "snmp_detail"
 * cannot be reliably determined.  Suggest using snmp_error()
 * to obtain the library error codes.
 */

#ifndef DONT_SHARE_ERROR_WITH_OTHER_THREADS
#define SET_SNMP_ERROR(x) snmp_errno=(x)
#else
#define SET_SNMP_ERROR(x)
#endif


#ifdef __cplusplus
extern "C" {
#endif

/***********************************************************
	Copyright 1989 by Carnegie Mellon University

                      All Rights Reserved

Permission to use, copy, modify, and distribute this software and its
documentation for any purpose and without fee is hereby granted,
provided that the above copyright notice appear in all copies and that
both that copyright notice and this permission notice appear in
supporting documentation, and that the name of CMU not be
used in advertising or publicity pertaining to distribution of the
software without specific, written prior permission.

CMU DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE, INCLUDING
ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO EVENT SHALL
CMU BE LIABLE FOR ANY SPECIAL, INDIRECT OR CONSEQUENTIAL DAMAGES OR
ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS,
WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION,
ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS
SOFTWARE.
******************************************************************/


struct variable_list;
struct timeval;


	/*
	 * Mimic size and alignment of 'struct sockaddr_storage' (see RFC 2553)
	 * But retain field names of traditional 'struct sockaddr'
	 */

#define _UCD_SS_MAXSIZE   92		/* <= sizeof( sockaddr_un ) */
#define _UCD_SS_ALIGNSIZE (sizeof (long))

#define _UCD_SS_PAD1SIZE  (_UCD_SS_ALIGNSIZE - sizeof( unsigned short ))
#define _UCD_SS_PAD2SIZE  (_UCD_SS_MAXSIZE - \
		(sizeof( unsigned short ) + _UCD_SS_PAD1SIZE + _UCD_SS_ALIGNSIZE ))

typedef struct {

#ifdef STRUCT_SOCKADDR_HAS_SA_UNION_SA_GENERIC_SA_FAMILY2
	/*
	 * Certain systems (notably Irix 6.x) have a non-traditional
	 *   socket structure, and #define the traditional field names.
	 * This local definition should reproduce this structure, and still
	 *    be large enough to handle any necessary Unix domain addresses.
	 */
  union {
   struct {
#ifdef _HAVE_SA_LEN
    unsigned char	sa_len2;
    unsigned char	sa_family2;
#else
    unsigned short	sa_family2;
#endif
    char		sa_data2[ _UCD_SS_PAD1SIZE ];
   } sa_generic;
    long		sa_align;
    char		sa_pad2[ _UCD_SS_PAD2SIZE ];
  } sa_union;

#else

#ifdef STRUCT_SOCKADDR_HAS_SA_LEN
    unsigned char	sa_len;
    unsigned char	sa_family;
#else
    unsigned short	sa_family;
#endif
    char		sa_data[ _UCD_SS_PAD1SIZE ];
    long		sa_align;
    char		sa_pad2[ _UCD_SS_PAD2SIZE ];
#endif

} snmp_ipaddr;

#define USM_AUTH_KU_LEN     32
#define USM_PRIV_KU_LEN     32

struct snmp_pdu {

	/*
	 * Protocol-version independent fields
	 */
    long    version;
    int	    command;	/* Type of this PDU */
    long    reqid;	/* Request id - note: not incremented on retries */
    long    msgid;      /* Message id for V3 messages
                         * note: incremented for each retry */
    long    transid;    /* Unique ID for incoming transactions */
    long    sessid;     /* Session id for AgentX messages */
    long    errstat;	/* Error status (non_repeaters in GetBulk) */
    long    errindex;	/* Error index (max_repetitions in GetBulk) */
    u_long  time;	/* Uptime */
    u_long  flags;

    int	    securityModel;
    int	    securityLevel;  /* noAuthNoPriv, authNoPriv, authPriv */
    int	    msgParseModel;

    snmp_ipaddr  address;	/* Address of peer or trap destination */

    struct variable_list *variables;


	/*
	 * SNMPv1 & SNMPv2c fields
	 */
    u_char  *community;		/* community for outgoing requests. */
    size_t  community_len;	/* Length of community name. */

	/*
	 * Trap information
	 */
    oid	    *enterprise;	/* System OID */
    size_t  enterprise_length;
    long    trap_type;		/* trap type */
    long    specific_type;	/* specific type */
    snmp_ipaddr	agent_addr;

	/*
	 * SNMPv3 fields
	 */
    u_char  *contextEngineID;	/* context snmpEngineID */
    size_t  contextEngineIDLen; /* Length of contextEngineID */
    char    *contextName;	/* authoritative contextName */
    size_t  contextNameLen;	/* Length of contextName */
    u_char  *securityEngineID;	/* authoritative snmpEngineID for security */
    size_t  securityEngineIDLen;/* Length of securityEngineID */
    char    *securityName;	/* on behalf of this principal */
    size_t  securityNameLen;	/* Length of securityName. */

	/*
	 * AgentX fields
	 *	(also uses SNMPv1 community field)
	 */
    int	    priority;
    int	    range_subid;

    void * securityStateRef;
};

struct snmp_session;
typedef int (*snmp_callback) (int, struct snmp_session *, int, struct snmp_pdu *, void *);

struct snmp_session {
	/*
	 * Protocol-version independent fields
	 */
    long  version;
    int	    retries;	/* Number of retries before timeout. */
    long    timeout;    /* Number of uS until first timeout, then exponential backoff */
    u_long  flags;
    struct  snmp_session *subsession;
    struct  snmp_session *next;

    char    *peername;	/* Domain name or dotted IP address of default peer */
    u_short remote_port;/* UDP port number of peer. */
    u_short local_port; /* My UDP port number, 0 for default, picked randomly */
    /* Authentication function or NULL if null authentication is used */
    u_char    *(*authenticator) (u_char *, size_t *, u_char *, size_t);
    snmp_callback callback; /* Function to interpret incoming data */
    /* Pointer to data that the callback function may consider important */
    void    *callback_magic;

    int     s_errno;        /* copy of system errno */
    int     s_snmp_errno;   /* copy of library errno */
    long    sessid;         /* Session id - AgentX only */

	/*
	 * SNMPv1 & SNMPv2c fields
	 */
    u_char  *community;	        /* community for outgoing requests. */
    size_t  community_len;      /* Length of community name. */

	/*
	 * SNMPv3 fields
	 */
    u_char  isAuthoritative;    /* are we the authoritative engine? */
    u_char  *contextEngineID;	/* authoritative snmpEngineID */
    size_t  contextEngineIDLen; /* Length of contextEngineID */
    u_int   engineBoots;        /* initial engineBoots for remote engine */
    u_int   engineTime;         /* initial engineTime for remote engine */
    char    *contextName;	/* authoritative contextName */
    size_t  contextNameLen;     /* Length of contextName */
    u_char  *securityEngineID;	/* authoritative snmpEngineID */
    size_t  securityEngineIDLen;  /* Length of contextEngineID */
    char    *securityName;	/* on behalf of this principal */
    size_t  securityNameLen;    /* Length of securityName. */
    oid     *securityAuthProto; /* auth protocol oid */
    size_t  securityAuthProtoLen; /* Length of auth protocol oid */
    u_char  securityAuthKey[USM_AUTH_KU_LEN];  /* Ku for auth protocol XXX */
    size_t  securityAuthKeyLen; /* Length of Ku for auth protocol */
    oid     *securityPrivProto; /* priv protocol oid */
    size_t  securityPrivProtoLen; /* Length of priv protocol oid */
    u_char  securityPrivKey[USM_PRIV_KU_LEN];  /* Ku for privacy protocol XXX */
    size_t  securityPrivKeyLen; /* Length of Ku for priv protocol */
    int	    securityModel;
    int	    securityLevel;  /* noAuthNoPriv, authNoPriv, authPriv */
};

/*
 * A list of all the outstanding requests for a particular session.
 */
#ifdef SNMP_NEED_REQUEST_LIST
struct request_list {
    struct request_list *next_request;
    long  request_id;	/* request id */
    long  message_id;	/* message id */
    snmp_callback callback; /* user callback per request (NULL if unused) */
    void   *cb_data;   /* user callback data per request (NULL if unused) */
    int	    retries;	/* Number of retries */
    u_long timeout;	/* length to wait for timeout */
    struct timeval time; /* Time this request was made */
    struct timeval expire;  /* time this request is due to expire */
    struct  snmp_session *session;
    struct snmp_pdu *pdu;   /* The pdu for this request
			       (saved so it can be retransmitted */
};
#endif /* SNMP_NEED_REQUEST_LIST */

/*
 * Set fields in session and pdu to the following to get a default or unconfigured value.
 */
#define SNMP_DEFAULT_COMMUNITY_LEN  0	/* to get a default community name */
#define SNMP_DEFAULT_RETRIES	    -1
#define SNMP_DEFAULT_TIMEOUT	    -1
#define SNMP_DEFAULT_REMPORT	    0
#define SNMP_DEFAULT_REQID	    -1
#define SNMP_DEFAULT_MSGID	    -1
#define SNMP_DEFAULT_ERRSTAT	    -1
#define SNMP_DEFAULT_ERRINDEX	    -1
#define SNMP_DEFAULT_ADDRESS	    0
#define SNMP_DEFAULT_PEERNAME	    NULL
#define SNMP_DEFAULT_ENTERPRISE_LENGTH	0
#define SNMP_DEFAULT_TIME	    0
#define SNMP_DEFAULT_VERSION	    -1
#define SNMP_DEFAULT_CONTEXT        ""
#define SNMP_DEFAULT_AUTH_PROTO     usmHMACMD5AuthProtocol
#define SNMP_DEFAULT_AUTH_PROTOLEN  USM_LENGTH_OID_TRANSFORM
#define SNMP_DEFAULT_PRIV_PROTO     usmDESPrivProtocol
#define SNMP_DEFAULT_PRIV_PROTOLEN  USM_LENGTH_OID_TRANSFORM

extern const char *snmp_api_errstring (int);
extern void snmp_perror (const char *);
extern void snmp_set_detail (const char *);

#define SNMP_MAX_MSG_SIZE          1472 /* ethernet MTU minus IP/UDP header */
#define SNMP_MAX_MSG_V3_HDRS       (4+3+4+7+7+3+7+16) /* fudge factor=16 */
#define SNMP_MAX_ENG_SIZE          32
#define SNMP_MAX_SEC_NAME_SIZE     256
#define SNMP_MAX_CONTEXT_SIZE      256
#define SNMP_SEC_PARAM_BUF_SIZE    256

/* set to one to ignore unauthenticated Reports */
#define SNMPV3_IGNORE_UNAUTH_REPORTS 0

/* authoritative engine definitions */
#define SNMP_SESS_NONAUTHORITATIVE 0 /* should be 0 to default to this */
#define SNMP_SESS_AUTHORITATIVE    1 /* don't learn engineIDs */
#define SNMP_SESS_UNKNOWNAUTH      2 /* sometimes (like NRs) */

/* to determine type of Report from varbind_list */
#define REPORT_STATS_LEN 9
#define REPORT_snmpUnknownSecurityModels_NUM 1
#define REPORT_snmpInvalidMsgs_NUM 2
#define REPORT_usmStatsUnsupportedSecLevels_NUM 1
#define REPORT_usmStatsNotInTimeWindows_NUM 2
#define REPORT_usmStatsUnknownUserNames_NUM 3
#define REPORT_usmStatsUnknownEngineIDs_NUM 4
#define REPORT_usmStatsWrongDigests_NUM 5
#define REPORT_usmStatsDecryptionErrors_NUM 6

#define SNMP_DETAIL_SIZE        512

#define SNMP_FLAGS_DONT_PROBE      0x100    /* don't probe for an engineID */
#define SNMP_FLAGS_STREAM_SOCKET   0x80
#define SNMP_FLAGS_LISTENING       0x40     /* Server stream sockets only */
#define SNMP_FLAGS_SUBSESSION      0x20
#define SNMP_FLAGS_STRIKE2         0x02
#define SNMP_FLAGS_STRIKE1         0x01

#define CLEAR_SNMP_STRIKE_FLAGS(x) \
	x &= ~(SNMP_FLAGS_STRIKE2|SNMP_FLAGS_STRIKE1)

	/*
	 * returns '1' if the session is to be regarded as dead,
	 * otherwise set the strike flags appropriately, and return 0
	 */
#define SET_SNMP_STRIKE_FLAGS(x) \
	((   x & SNMP_FLAGS_STRIKE2 ) ? 1 :				\
	 ((( x & SNMP_FLAGS_STRIKE1 ) ? ( x |= SNMP_FLAGS_STRIKE2 ) :	\
	                                ( x |= SNMP_FLAGS_STRIKE1 )),	\
	                                0))

/*
 * Error return values.
 *
 * SNMPERR_SUCCESS is the non-PDU "success" code.
 *
 * XXX	These should be merged with SNMP_ERR_* defines and confined
 *	to values < 0.  ???
 */
#define SNMPERR_SUCCESS			(0)  /* XXX  Non-PDU "success" code. */
#define SNMPERR_GENERR			(-1)
#define SNMPERR_BAD_LOCPORT		(-2)
#define SNMPERR_BAD_ADDRESS		(-3)
#define SNMPERR_BAD_SESSION		(-4)
#define SNMPERR_TOO_LONG		(-5)
#define SNMPERR_NO_SOCKET		(-6)
#define SNMPERR_V2_IN_V1		(-7)
#define SNMPERR_V1_IN_V2		(-8)
#define SNMPERR_BAD_REPEATERS		(-9)
#define SNMPERR_BAD_REPETITIONS		(-10)
#define SNMPERR_BAD_ASN1_BUILD		(-11)
#define SNMPERR_BAD_SENDTO		(-12)
#define SNMPERR_BAD_PARSE		(-13)
#define SNMPERR_BAD_VERSION		(-14)
#define SNMPERR_BAD_SRC_PARTY		(-15)
#define SNMPERR_BAD_DST_PARTY		(-16)
#define SNMPERR_BAD_CONTEXT		(-17)
#define SNMPERR_BAD_COMMUNITY		(-18)
#define SNMPERR_NOAUTH_DESPRIV		(-19)
#define SNMPERR_BAD_ACL			(-20)
#define SNMPERR_BAD_PARTY		(-21)
#define SNMPERR_ABORT			(-22)
#define SNMPERR_UNKNOWN_PDU		(-23)
#define SNMPERR_TIMEOUT 		(-24)
#define SNMPERR_BAD_RECVFROM 		(-25)
#define SNMPERR_BAD_ENG_ID 		(-26)
#define SNMPERR_BAD_SEC_NAME 		(-27)
#define SNMPERR_BAD_SEC_LEVEL 		(-28)
#define SNMPERR_ASN_PARSE_ERR           (-29)
#define SNMPERR_UNKNOWN_SEC_MODEL 	(-30)
#define SNMPERR_INVALID_MSG             (-31)
#define SNMPERR_UNKNOWN_ENG_ID          (-32)
#define SNMPERR_UNKNOWN_USER_NAME 	(-33)
#define SNMPERR_UNSUPPORTED_SEC_LEVEL 	(-34)
#define SNMPERR_AUTHENTICATION_FAILURE 	(-35)
#define SNMPERR_NOT_IN_TIME_WINDOW 	(-36)
#define SNMPERR_DECRYPTION_ERR          (-37)
#define SNMPERR_SC_GENERAL_FAILURE	(-38)
#define SNMPERR_SC_NOT_CONFIGURED	(-39)
#define SNMPERR_KT_NOT_AVAILABLE	(-40)
#define SNMPERR_UNKNOWN_REPORT          (-41)
#define SNMPERR_USM_GENERICERROR		(-42)
#define SNMPERR_USM_UNKNOWNSECURITYNAME		(-43)
#define SNMPERR_USM_UNSUPPORTEDSECURITYLEVEL	(-44)
#define SNMPERR_USM_ENCRYPTIONERROR		(-45)
#define SNMPERR_USM_AUTHENTICATIONFAILURE	(-46)
#define SNMPERR_USM_PARSEERROR			(-47)
#define SNMPERR_USM_UNKNOWNENGINEID		(-48)
#define SNMPERR_USM_NOTINTIMEWINDOW		(-49)
#define SNMPERR_USM_DECRYPTIONERROR		(-50)
#define SNMPERR_NOMIB			(-51)
#define SNMPERR_RANGE			(-52)
#define SNMPERR_MAX_SUBID		(-53)
#define SNMPERR_BAD_SUBID		(-54)
#define SNMPERR_LONG_OID		(-55)
#define SNMPERR_BAD_NAME		(-56)
#define SNMPERR_VALUE			(-57)
#define SNMPERR_UNKNOWN_OBJID		(-58)
#define SNMPERR_NULL_PDU		(-59)
#define SNMPERR_NO_VARS			(-60)
#define SNMPERR_VAR_TYPE		(-61)
#define SNMPERR_MALLOC			(-62)

#define SNMPERR_MAX			(-62)

#define non_repeaters	errstat
#define max_repetitions errindex


struct variable_list {
    struct variable_list *next_variable;    /* NULL for last variable */
    oid	    *name;  /* Object identifier of variable */
    size_t  name_length;    /* number of subid's in name */
    u_char  type;   /* ASN type of variable */
    union { /* value of variable */
	long	*integer;
	u_char	*string;
	oid	*objid;
	u_char  *bitstring;
	struct counter64 *counter64;
#ifdef OPAQUE_SPECIAL_TYPES
	float   *floatVal;
	double	*doubleVal;
/*	t_union *unionVal; */
#endif /* OPAQUE_SPECIAL_TYPES */
    } val;
    size_t	    val_len;
    oid name_loc[MAX_OID_LEN];  /* 90 percentile < 24. */
    u_char buf[40];             /* 90 percentile < 40. */
    void *data;			/* (Opaque) hook for additional data */
    int  index;
};



/*
 * struct snmp_session *snmp_open(session)
 *	struct snmp_session *session;
 *
 * Sets up the session with the snmp_session information provided
 * by the user.  Then opens and binds the necessary UDP port.
 * A handle to the created session is returned (this is different than
 * the pointer passed to snmp_open()).  On any error, NULL is returned
 * and snmp_errno is set to the appropriate error code.
 */
struct snmp_session *snmp_open (struct snmp_session *);

/*
 * int snmp_close(session)
 *     struct snmp_session *session;
 *
 * Close the input session.  Frees all data allocated for the session,
 * dequeues any pending requests, and closes any sockets allocated for
 * the session.  Returns 0 on error, 1 otherwise.
 *
 * snmp_close_sessions() does the same thing for all open sessions
 */
int snmp_close (struct snmp_session *);
int snmp_close_sessions (void);


/*
 * int snmp_send(session, pdu)
 *     struct snmp_session *session;
 *     struct snmp_pdu	*pdu;
 *
 * Sends the input pdu on the session after calling snmp_build to create
 * a serialized packet.  If necessary, set some of the pdu data from the
 * session defaults.  Add a request corresponding to this pdu to the list
 * of outstanding requests on this session, then send the pdu.
 * Returns the request id of the generated packet if applicable, otherwise 1.
 * On any error, 0 is returned.
 * The pdu is freed by snmp_send() unless a failure occured.
 */
int snmp_send (struct snmp_session *, struct snmp_pdu *);

/*
 * int snmp_async_send(session, pdu, callback, cb_data)
 *     struct snmp_session *session;
 *     struct snmp_pdu	*pdu;
 *     snmp_callback callback;
 *     void   *cb_data;
 *
 * Sends the input pdu on the session after calling snmp_build to create
 * a serialized packet.  If necessary, set some of the pdu data from the
 * session defaults.  Add a request corresponding to this pdu to the list
 * of outstanding requests on this session and store callback and data,
 * then send the pdu.
 * Returns the request id of the generated packet if applicable, otherwise 1.
 * On any error, 0 is returned.
 * The pdu is freed by snmp_send() unless a failure occured.
 */
int snmp_async_send (struct snmp_session *, struct snmp_pdu *,
                         snmp_callback, void *);


/*
 * void snmp_read(fdset)
 *     fd_set  *fdset;
 *
 * Checks to see if any of the fd's set in the fdset belong to
 * snmp.  Each socket with it's fd set has a packet read from it
 * and snmp_parse is called on the packet received.  The resulting pdu
 * is passed to the callback routine for that session.  If the callback
 * routine returns successfully, the pdu and it's request are deleted.
 */
void snmp_read (fd_set *);



/*
 * void
 * snmp_free_pdu(pdu)
 *     struct snmp_pdu *pdu;
 *
 * Frees the pdu and any malloc'd data associated with it.
 */
void snmp_free_pdu (struct snmp_pdu *);

void snmp_free_var (struct variable_list *); /* frees just this one */

void snmp_free_varbind(struct variable_list *var); /* frees all in list */

/*
 * int snmp_select_info(numfds, fdset, timeout, block)
 * int *numfds;
 * fd_set   *fdset;
 * struct timeval *timeout;
 * int *block;
 *
 * Returns info about what snmp requires from a select statement.
 * numfds is the number of fds in the list that are significant.
 * All file descriptors opened for SNMP are OR'd into the fdset.
 * If activity occurs on any of these file descriptors, snmp_read
 * should be called with that file descriptor set.
 *
 * The timeout is the latest time that SNMP can wait for a timeout.  The
 * select should be done with the minimum time between timeout and any other
 * timeouts necessary.  This should be checked upon each invocation of select.
 * If a timeout is received, snmp_timeout should be called to check if the
 * timeout was for SNMP.  (snmp_timeout is idempotent)
 *
 * Block is 1 if the select is requested to block indefinitely, rather than
 * time out.  If block is input as 1, the timeout value will be treated as
 * undefined, but it must be available for setting in snmp_select_info.  On
 * return, if block is true, the value of timeout will be undefined.
 *
 * snmp_select_info returns the number of open sockets.  (i.e. The number
 * of sessions open)
 */
int snmp_select_info (int *, fd_set *, struct timeval *, int *);



/*
 * void snmp_timeout();
 *
 * snmp_timeout should be called whenever the timeout from snmp_select_info
 * expires, but it is idempotent, so snmp_timeout can be polled (probably a
 * cpu expensive proposition).  snmp_timeout checks to see if any of the
 * sessions have an outstanding request that has timed out.  If it finds one
 * (or more), and that pdu has more retries available, a new packet is formed
 * from the pdu and is resent.  If there are no more retries available, the
 * callback for the session is used to alert the user of the timeout.
 */

void snmp_timeout (void);


/*
 * This routine must be supplied by the application:
 *
 * u_char *authenticator(pdu, length, community, community_len)
 * u_char *pdu;		The rest of the PDU to be authenticated
 * int *length;		The length of the PDU (updated by the authenticator)
 * u_char *community;	The community name to authenticate under.
 * int	community_len	The length of the community name.
 *
 * Returns the authenticated pdu, or NULL if authentication failed.
 * If null authentication is used, the authenticator in snmp_session can be
 * set to NULL(0).
 */



/*
 * This routine must be supplied by the application:
 *
 * int callback(operation, session, reqid, pdu, magic)
 * int operation;
 * struct snmp_session *session;    The session authenticated under.
 * int reqid;			    The request id of this pdu (0 for TRAP)
 * struct snmp_pdu *pdu;	    The pdu information.
 * void *magic			    A link to the data for this routine.
 *
 * Returns 1 if request was successful, 0 if it should be kept pending.
 * Any data in the pdu must be copied because it will be freed elsewhere.
 * Operations are defined below:
 */

#define RECEIVED_MESSAGE   1
#define TIMED_OUT	   2
#define SEND_FAILED	   3

long snmp_get_next_msgid(void);
long snmp_get_next_reqid(void);
long snmp_get_next_sessid(void);
long snmp_get_next_transid(void);
/* provide for backwards compatibility */
void snmp_set_dump_packet(int);
int snmp_get_dump_packet(void);
void snmp_set_quick_print(int);
int snmp_get_quick_print(void);
void snmp_set_suffix_only(int);
int snmp_get_suffix_only(void);
void snmp_set_full_objid(int);
int snmp_get_full_objid(void);
void snmp_set_random_access(int);
int snmp_get_random_access(void);

int snmp_oid_compare (const oid *, size_t, const oid *, size_t);
void init_snmp (const char *);
u_char *snmp_pdu_build (struct snmp_pdu *, u_char *, size_t *);
#ifdef USE_REVERSE_ASNENCODING
u_char *snmp_pdu_rbuild (struct snmp_pdu *, u_char *, size_t *);
#endif
int snmpv3_parse(struct snmp_pdu *, u_char *, size_t *, u_char  **, struct snmp_session *);
int snmpv3_dparse(struct snmp_pdu *, u_char *, size_t *, u_char  **, int);
int snmpv3_packet_build(struct snmp_pdu *pdu, u_char *packet, size_t *out_length, u_char *pdu_data, size_t pdu_data_len);
int snmpv3_packet_rbuild(struct snmp_pdu *pdu, u_char *packet, size_t *out_length, u_char *pdu_data, size_t pdu_data_len);
int snmpv3_make_report(struct snmp_pdu *pdu, int error);
int snmpv3_get_report_type(struct snmp_pdu *pdu);
int snmp_pdu_parse(struct snmp_pdu *pdu, u_char *data, size_t *length);
int snmp_pdu_dparse(struct snmp_pdu *pdu, u_char *data, size_t *length, int);
u_char* snmpv3_scopedPDU_parse(struct snmp_pdu *pdu, u_char *cp, size_t *length);
u_char* snmpv3_scopedPDU_dparse(struct snmp_pdu *pdu, u_char *cp, size_t *length, int);
void snmp_store(const char *type);
void snmp_shutdown(const char *type);
struct variable_list *snmp_pdu_add_variable (struct snmp_pdu *, oid *, size_t, u_char, u_char *, size_t);
struct variable_list *snmp_varlist_add_variable(struct variable_list **varlist,
	oid *name, size_t name_length, u_char type, u_char *value, size_t len);
int hex_to_binary (const char *, u_char *);
int ascii_to_binary (const char *, u_char *);
int snmp_add_var (struct snmp_pdu *, oid*, size_t, char, const char *);
oid  *snmp_duplicate_objid(oid *objToCopy, size_t);
u_int snmp_increment_statistic(int which);
u_int snmp_increment_statistic_by(int which, int count);
u_int snmp_get_statistic(int which);
void  snmp_init_statistics(void);
int create_user_from_session(struct snmp_session *session);

/* extended open */
struct snmp_session *snmp_open_ex (struct snmp_session *,
  int (*fpre_parse) (struct snmp_session *, snmp_ipaddr),
  int (*fparse) (struct snmp_session *, struct snmp_pdu *, u_char *, size_t),
  int (*fpost_parse) (struct snmp_session *, struct snmp_pdu *, int),
  int (*fbuild) (struct snmp_session *, struct snmp_pdu *, u_char *, size_t *),
  int (*fcheck) (u_char *, size_t)
);

/* provided for backwards compatability.  Don't use these functions.
   See snmp_debug.h and snmp_debug.c instead.
*/
#if HAVE_STDARG_H
void DEBUGP (const char *, ...);
#else
void DEBUGP (va_alist);
#endif
void DEBUGPOID(oid *, size_t);
void snmp_set_do_debugging (int);
int snmp_get_do_debugging (void);

#ifdef CMU_COMPATIBLE
extern int snmp_dump_packet;
extern int quick_print;
#endif

size_t snmp_socket_length   (int family);

/*
 * snmp_error - return error data
 * Inputs :  address of errno, address of snmp_errno, address of string
 * Caller must free the string returned after use.
 */
void snmp_error (struct snmp_session *, int *, int *, char **);
/*
 * single session API.
 *
 * These functions perform similar actions as snmp_XX functions,
 * but operate on a single session only.
 *
 * Synopsis:

	void * sessp;
	struct snmp_session session, *ss;
	struct snmp_pdu *pdu, *response;

	snmp_sess_init(&session);
	session.retries = ...
	session.remote_port = ...
	sessp = snmp_sess_open(&session);
	ss = snmp_sess_session(sessp);
	if (ss == NULL)
		exit(1);
	...
	if (ss->community) free(ss->community);
	ss->community = strdup(gateway);
	ss->community_len = strlen(gateway);
	...
	snmp_sess_synch_response(sessp, pdu, &response);
	...
	snmp_sess_close(sessp);

 * See also:
 * snmp_sess_synch_response, in snmp_client.h.

 * Notes:
 *  1. Invoke snmp_sess_session after snmp_sess_open.
 *  2. snmp_sess_session return value is an opaque pointer.
 *  3. Do NOT free memory returned by snmp_sess_session.
 *  4. Replace snmp_send(ss,pdu) with snmp_sess_send(sessp,pdu)
 */

void   snmp_sess_init       (struct snmp_session *);
void * snmp_sess_open       (struct snmp_session *);
struct snmp_session * snmp_sess_session    (void *);

/* use return value from snmp_sess_open as void * parameter */

int    snmp_sess_send       (void *, struct snmp_pdu *);
int    snmp_sess_async_send (void *, struct snmp_pdu *,
                                         snmp_callback, void *);
int    snmp_sess_select_info (void *, int *, fd_set *,
                                         struct timeval *, int *);
int    snmp_sess_read       (void *, fd_set *);
void   snmp_sess_timeout    (void *);
int    snmp_sess_close      (void *);

void   snmp_sess_error      (void *, int *, int *, char **);
void   snmp_sess_perror     (const char *prog_string, struct snmp_session *ss);

/* end single session API */

/* generic statistic counters */

/* snmpv3 statistics */

/* mpd stats */
#define   STAT_SNMPUNKNOWNSECURITYMODELS     0
#define   STAT_SNMPINVALIDMSGS               1
#define   STAT_SNMPUNKNOWNPDUHANDLERS        2
#define   STAT_MPD_STATS_START               STAT_SNMPUNKNOWNSECURITYMODELS
#define   STAT_MPD_STATS_END                 STAT_SNMPUNKNOWNPDUHANDLERS

/* usm stats */
#define   STAT_USMSTATSUNSUPPORTEDSECLEVELS  3
#define   STAT_USMSTATSNOTINTIMEWINDOWS      4
#define   STAT_USMSTATSUNKNOWNUSERNAMES      5
#define   STAT_USMSTATSUNKNOWNENGINEIDS      6
#define   STAT_USMSTATSWRONGDIGESTS          7
#define   STAT_USMSTATSDECRYPTIONERRORS      8
#define   STAT_USM_STATS_START               STAT_USMSTATSUNSUPPORTEDSECLEVELS
#define   STAT_USM_STATS_END                 STAT_USMSTATSDECRYPTIONERRORS

/* snmp counters */
#define  STAT_SNMPINPKTS                     9
#define  STAT_SNMPOUTPKTS                    10
#define  STAT_SNMPINBADVERSIONS              11
#define  STAT_SNMPINBADCOMMUNITYNAMES        12
#define  STAT_SNMPINBADCOMMUNITYUSES         13
#define  STAT_SNMPINASNPARSEERRS             14
/* #define  STAT_SNMPINBADTYPES		     15 */
#define  STAT_SNMPINTOOBIGS                  16
#define  STAT_SNMPINNOSUCHNAMES              17
#define  STAT_SNMPINBADVALUES                18
#define  STAT_SNMPINREADONLYS                19
#define  STAT_SNMPINGENERRS                  20
#define  STAT_SNMPINTOTALREQVARS             21
#define  STAT_SNMPINTOTALSETVARS             22
#define  STAT_SNMPINGETREQUESTS              23
#define  STAT_SNMPINGETNEXTS                 24
#define  STAT_SNMPINSETREQUESTS              25
#define  STAT_SNMPINGETRESPONSES             26
#define  STAT_SNMPINTRAPS                    27
#define  STAT_SNMPOUTTOOBIGS                 28
#define  STAT_SNMPOUTNOSUCHNAMES             29
#define  STAT_SNMPOUTBADVALUES               30
/* #define  STAT_SNMPOUTREADONLYS	     31 */
#define  STAT_SNMPOUTGENERRS                 32
#define  STAT_SNMPOUTGETREQUESTS             33
#define  STAT_SNMPOUTGETNEXTS                34
#define  STAT_SNMPOUTSETREQUESTS             35
#define  STAT_SNMPOUTGETRESPONSES            36
#define  STAT_SNMPOUTTRAPS                   37
/* AUTHTRAPENABLE			     38 */
#define  STAT_SNMPSILENTDROPS		     39
#define  STAT_SNMPPROXYDROPS		     40
#define  STAT_SNMP_STATS_START               STAT_SNMPINPKTS
#define  STAT_SNMP_STATS_END                 STAT_SNMPOUTTRAPS

#define  MAX_STATS                           41

#ifdef __cplusplus
}
#endif

#endif /* SNMP_API_H */
