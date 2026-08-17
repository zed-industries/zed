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
/* from @(#)ypupdate_prot.x	1.3 91/03/11 TIRPC 1.0 */

#ifndef __RPCSVC_YPUPD_H__
#define __RPCSVC_YPUPD_H__

#include <rpc/rpc.h>

#define MAXMAPNAMELEN 255
#define MAXYPDATALEN 1023
#define MAXERRMSGLEN 255

#ifdef  __cplusplus
extern "C" {
#endif

typedef struct {
	u_int yp_buf_len;
	char *yp_buf_val;
} yp_buf;

extern  bool_t xdr_yp_buf (XDR *, yp_buf*);

struct ypupdate_args {
	char *mapname;
	yp_buf key;
	yp_buf datum;
};
typedef struct ypupdate_args ypupdate_args;

extern  bool_t xdr_ypupdate_args (XDR *, ypupdate_args*);

struct ypdelete_args {
	char *mapname;
	yp_buf key;
};
typedef struct ypdelete_args ypdelete_args;

extern  bool_t xdr_ypdelete_args (XDR *, ypdelete_args*);

#define YPU_PROG 100028
#define YPU_VERS 1

#define YPU_CHANGE 1
extern  u_int * ypu_change_1 (ypupdate_args *, CLIENT *);
extern  u_int * ypu_change_1_svc (ypupdate_args *, struct svc_req *);
#define YPU_INSERT 2
extern  u_int * ypu_insert_1 (ypupdate_args *, CLIENT *);
extern  u_int * ypu_insert_1_svc (ypupdate_args *, struct svc_req *);
#define YPU_DELETE 3
extern  u_int * ypu_delete_1 (ypdelete_args *, CLIENT *);
extern  u_int * ypu_delete_1_svc (ypdelete_args *, struct svc_req *);
#define YPU_STORE 4
extern  u_int * ypu_store_1 (ypupdate_args *, CLIENT *);
extern  u_int * ypu_store_1_svc (ypupdate_args *, struct svc_req *);

#ifdef  __cplusplus
}
#endif

#endif /* !__RPCSVC_YPUPD_H__ */
