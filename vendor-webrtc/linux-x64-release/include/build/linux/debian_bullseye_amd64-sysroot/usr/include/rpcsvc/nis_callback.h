/*
 * Copyright (c) 2010, Oracle America, Inc.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 *       notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 *       copyright notice, this list of conditions and the following
 *       disclaimer in the documentation and/or other materials
 *       provided with the distribution.
 *     * Neither the name of the "Oracle America, Inc." nor the names of its
 *       contributors may be used to endorse or promote products derived
 *       from this software without specific prior written permission.
 *
 *   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 *   "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 *   LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 *   FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 *   COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
 *   INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 *   DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE
 *   GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 *   INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
 *   WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 *   NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 *   OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
#ifndef _RPCSVC_NIS_CALLBACK_H
#define _RPCSVC_NIS_CALLBACK_H 1

#include <rpc/rpc.h>
#include <rpcsvc/nis.h>

#ifdef  __cplusplus
extern "C" {
#endif

typedef nis_object *obj_p;

struct cback_data {
	struct {
		u_int entries_len;
		obj_p *entries_val;
	} entries;
};
typedef struct cback_data cback_data;

#define CB_PROG 100302
#define CB_VERS 1

#define CBPROC_RECEIVE 1
extern  bool_t * cbproc_receive_1 (cback_data *, CLIENT *);
extern  bool_t * cbproc_receive_1_svc (cback_data *, struct svc_req *);

#define CBPROC_FINISH 2
extern  void * cbproc_finish_1 (void *, CLIENT *);
extern  void * cbproc_finish_1_svc (void *, struct svc_req *);

#define CBPROC_ERROR 3
extern  void * cbproc_error_1 (nis_error *, CLIENT *);
extern  void * cbproc_error_1_svc (nis_error *, struct svc_req *);
extern int cb_prog_1_freeresult (SVCXPRT *, xdrproc_t, caddr_t);

/* the xdr functions */

extern  bool_t xdr_obj_p (XDR *, obj_p*);
extern  bool_t xdr_cback_data (XDR *, cback_data*);

#ifdef  __cplusplus
}
#endif

#endif /* !_RPCVSC_NIS_CALLBACK_H */
