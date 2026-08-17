/******************************************************************************


Copyright 1993, 1998  The Open Group

Permission to use, copy, modify, distribute, and sell this software and its
documentation for any purpose is hereby granted without fee, provided that
the above copyright notice appear in all copies and that both that
copyright notice and this permission notice appear in supporting
documentation.

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL THE
OPEN GROUP BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN
AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

Except as contained in this notice, the name of The Open Group shall not be
used in advertising or otherwise to promote the sale, use or other dealings
in this Software without prior written authorization from The Open Group.

Author: Ralph Mor, X Consortium
******************************************************************************/

#ifndef _ICECONN_H_
#define _ICECONN_H_

#include <X11/ICE/ICElib.h>

/*
 * Data structures for ICE connection object
 */

typedef struct _IceSavedReplyWait {
    IceReplyWaitInfo		*reply_wait;
    Bool			reply_ready;
    struct _IceSavedReplyWait	*next;
} _IceSavedReplyWait;

typedef struct _IcePingWait {
    IcePingReplyProc		ping_reply_proc;
    IcePointer			client_data;
    struct _IcePingWait 	*next;
} _IcePingWait;

typedef struct {
    char		*vendor;
    char		*release;
    int			version_count;
    IcePoVersionRec	*version_recs;
    int			auth_count;
    char		**auth_names;
    IcePoAuthProc	*auth_procs;
    IceIOErrorProc	io_error_proc;
} _IcePoProtocol;

typedef struct {
    char			*vendor;
    char			*release;
    int				version_count;
    IcePaVersionRec		*version_recs;
    IceProtocolSetupProc	protocol_setup_proc;
    IceProtocolActivateProc	protocol_activate_proc;
    int				auth_count;
    char			**auth_names;
    IcePaAuthProc		*auth_procs;
    IceHostBasedAuthProc	host_based_auth_proc;
    IceIOErrorProc		io_error_proc;
} _IcePaProtocol;

typedef struct {
    char		*protocol_name;
    _IcePoProtocol	*orig_client;
    _IcePaProtocol   	*accept_client;
} _IceProtocol;

typedef struct {
    Bool			in_use;
    int				my_opcode;
    _IceProtocol		*protocol;
    IcePointer			client_data;
    Bool			accept_flag;
    union {
	IcePaProcessMsgProc	accept_client;
	IcePoProcessMsgProc	orig_client;
    } process_msg_proc;
} _IceProcessMsgInfo;

typedef struct {
    int		his_version_index;
    int		my_version_index;
    char	*his_vendor;
    char	*his_release;
    char	my_auth_index;
    IcePointer 	my_auth_state;
    Bool	must_authenticate;
} _IceConnectToMeInfo;

typedef struct {
    int		his_opcode;
    int		my_opcode;
    int		his_version_index;
    int		my_version_index;
    char	*his_vendor;
    char	*his_release;
    char	my_auth_index;
    IcePointer 	my_auth_state;
    Bool	must_authenticate;
} _IceProtoSetupToMeInfo;

typedef struct {
    Bool 	auth_active;
    char	my_auth_index;
    IcePointer 	my_auth_state;
} _IceConnectToYouInfo;

typedef struct {
    int		my_opcode;
    int		my_auth_count;
    int		*my_auth_indices;
    Bool 	auth_active;
    char	my_auth_index;
    IcePointer	my_auth_state;
} _IceProtoSetupToYouInfo;


struct _IceConn {

    unsigned int io_ok : 1;		     /* did an IO error occur? */
    unsigned int swap : 1;  		     /* do we need to swap on reads? */
    unsigned int waiting_for_byteorder : 1;  /* waiting for a ByteOrder msg? */
    unsigned int skip_want_to_close : 1;     /* avoid shutdown negotiation? */
    unsigned int want_to_close : 1;	     /* did we send a WantToClose? */
    unsigned int free_asap : 1;		     /* free as soon as possible */
    unsigned int unused1 : 2;		     /* future use */
    unsigned int unused2 : 8;		     /* future use */

    IceConnectStatus connection_status; /* pending, accepted, rejected */

    unsigned char my_ice_version_index; /* which version are we using? */

    struct _XtransConnInfo *trans_conn; /* transport connection object */
    unsigned long send_sequence;     	/* Sequence # of last msg sent */
    unsigned long receive_sequence;    	/* Sequence # of last msg received */

    char *connection_string;		/* network connection string */
    char *vendor;			/* other client's vendor */
    char *release;			/* other client's release */

    char *inbuf;			/* Input buffer starting address */
    char *inbufptr;			/* Input buffer index pointer */
    char *inbufmax;			/* Input buffer maximum+1 address */

    char *outbuf;			/* Output buffer starting address */
    char *outbufptr;			/* Output buffer index pointer */
    char *outbufmax;			/* Output buffer maximum+1 address */

    char *scratch;			/* scratch buffer */
    unsigned long scratch_size;		/* scratch size */

    int dispatch_level;			/* IceProcessMessages dispatch level */

    IcePointer context;			/* context associated with caller
					   of IceOpenConnection */

    /*
     * Before we read a message, the major opcode of the message must be
     * mapped to our corresponding major opcode (the two clients can use
     * different opcodes for the same protocol).  In order to save space,
     * we keep track of the mininum and maximum major opcodes used by the
     * other client.  To get the information on how to process this message,
     * we do the following...
     *
     * processMsgInfo = iceConn->process_msg_info[
     *     message->majorOpcode - iceConn->his_min_opcode]
     *
     * Note that the number of elements in the iceConn->process_msg_info
     * array is always (iceConn->his_max_opcode - iceConn->his_min_opcode + 1).
     * We check process_msg_info->in_use to see if the opcode is being used.
     */

    _IceProcessMsgInfo		*process_msg_info;
    char 			his_min_opcode;   /* [1..255] */
    char			his_max_opcode;	  /* [1..255] */


    /*
     * Number of times this iceConn was returned in IceOpenConnection
     * or IceAcceptConnection.
     */

    unsigned char		open_ref_count;


    /*
     * Number of active protocols.
     */

    unsigned char		proto_ref_count;


    /*
     * If this ICE connection was created with IceAcceptConnection,
     * the listen_obj field is set to the listen object.  Otherwise,
     * the listen_obj field is NULL.
     */

    IceListenObj		listen_obj;




    /*
     * We need to keep track of all the replies we're waiting for.
     * Check the comments in process.c for how this works.
     */

    _IceSavedReplyWait		*saved_reply_waits;


    /*
     * We keep track of all Pings sent from the client.  When the Ping reply
     * arrives, we remove it from the list.
     */

    _IcePingWait		*ping_waits;


    /*
     * Some state for a client doing a Connection/Protocol Setup
     */

    _IceConnectToYouInfo	*connect_to_you;
    _IceProtoSetupToYouInfo	*protosetup_to_you;


    /*
     * Some state for a client receiving a Connection/Protocol Setup
     */

    _IceConnectToMeInfo		*connect_to_me;
    _IceProtoSetupToMeInfo	*protosetup_to_me;

};

#endif /* _ICECONN_H_ */
