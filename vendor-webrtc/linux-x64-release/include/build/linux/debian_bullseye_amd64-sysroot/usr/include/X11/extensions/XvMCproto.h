#ifndef _XVMCPROTO_H_
#define _XVMCPROTO_H_

#define xvmc_QueryVersion		0
#define xvmc_ListSurfaceTypes		1
#define xvmc_CreateContext		2
#define xvmc_DestroyContext		3
#define xvmc_CreateSurface		4
#define xvmc_DestroySurface		5
#define xvmc_CreateSubpicture		6
#define xvmc_DestroySubpicture		7
#define xvmc_ListSubpictureTypes	8
#define xvmc_GetDRInfo                  9
#define xvmc_LastRequest		xvmc_GetDRInfo

#define xvmcNumRequest			(xvmc_LastRequest + 1)


typedef struct {
  CARD32 surface_type_id;
  CARD16 chroma_format;
  CARD16 pad0;
  CARD16 max_width;
  CARD16 max_height;
  CARD16 subpicture_max_width;
  CARD16 subpicture_max_height;
  CARD32 mc_type;
  CARD32 flags;
} xvmcSurfaceInfo;
#define sz_xvmcSurfaceInfo 24;

typedef struct {
  CARD8 reqType;
  CARD8 xvmcReqType;
  CARD16 length;
} xvmcQueryVersionReq;
#define sz_xvmcQueryVersionReq 4;

typedef struct {
  BYTE type;  /* X_Reply */
  BYTE padb1;
  CARD16 sequenceNumber;
  CARD32 length;
  CARD32 major;
  CARD32 minor;
  CARD32 padl4;
  CARD32 padl5;
  CARD32 padl6;
  CARD32 padl7;
} xvmcQueryVersionReply;
#define sz_xvmcQueryVersionReply 32

typedef struct {
  CARD8 reqType;
  CARD8 xvmcReqType;
  CARD16 length;
  CARD32 port;
} xvmcListSurfaceTypesReq;
#define sz_xvmcListSurfaceTypesReq 8;

typedef struct {
  BYTE type;  /* X_Reply */
  BYTE padb1;
  CARD16 sequenceNumber;
  CARD32 length;
  CARD32 num;
  CARD32 padl3;
  CARD32 padl4;
  CARD32 padl5;
  CARD32 padl6;
  CARD32 padl7;
} xvmcListSurfaceTypesReply;
#define sz_xvmcListSurfaceTypesReply 32

typedef struct {
  CARD8 reqType;
  CARD8 xvmcReqType;
  CARD16 length;
  CARD32 context_id;
  CARD32 port;
  CARD32 surface_type_id;
  CARD16 width;
  CARD16 height;
  CARD32 flags;
} xvmcCreateContextReq;
#define sz_xvmcCreateContextReq 24;

typedef struct {
  BYTE type;  /* X_Reply */
  BYTE padb1;
  CARD16 sequenceNumber;
  CARD32 length;
  CARD16 width_actual;
  CARD16 height_actual;
  CARD32 flags_return;
  CARD32 padl4;
  CARD32 padl5;
  CARD32 padl6;
  CARD32 padl7;
} xvmcCreateContextReply;
#define sz_xvmcCreateContextReply 32

typedef struct {
  CARD8 reqType;
  CARD8 xvmcReqType;
  CARD16 length;
  CARD32 context_id;
} xvmcDestroyContextReq;
#define sz_xvmcDestroyContextReq 8;

typedef struct {
  CARD8 reqType;
  CARD8 xvmcReqType;
  CARD16 length;
  CARD32 surface_id;
  CARD32 context_id;
} xvmcCreateSurfaceReq;
#define sz_xvmcCreateSurfaceReq 12;

typedef struct {
  BYTE type;  /* X_Reply */
  BYTE padb1;
  CARD16 sequenceNumber;
  CARD32 length;
  CARD32 padl2;
  CARD32 padl3;
  CARD32 padl4;
  CARD32 padl5;
  CARD32 padl6;
  CARD32 padl7;
} xvmcCreateSurfaceReply;
#define sz_xvmcCreateSurfaceReply 32

typedef struct {
  CARD8 reqType;
  CARD8 xvmcReqType;
  CARD16 length;
  CARD32 surface_id;
} xvmcDestroySurfaceReq;
#define sz_xvmcDestroySurfaceReq 8;


typedef struct {
  CARD8 reqType;
  CARD8 xvmcReqType;
  CARD16 length;
  CARD32 subpicture_id;
  CARD32 context_id;
  CARD32 xvimage_id;
  CARD16 width;
  CARD16 height;
} xvmcCreateSubpictureReq;
#define sz_xvmcCreateSubpictureReq 20;

typedef struct {
  BYTE type;  /* X_Reply */
  BYTE padb1;
  CARD16 sequenceNumber;
  CARD32 length;
  CARD16 width_actual;
  CARD16 height_actual;
  CARD16 num_palette_entries;
  CARD16 entry_bytes;
  CARD8  component_order[4];
  CARD32 padl5;
  CARD32 padl6;
  CARD32 padl7;
} xvmcCreateSubpictureReply;
#define sz_xvmcCreateSubpictureReply 32

typedef struct {
  CARD8 reqType;
  CARD8 xvmcReqType;
  CARD16 length;
  CARD32 subpicture_id;
} xvmcDestroySubpictureReq;
#define sz_xvmcDestroySubpictureReq 8;

typedef struct {
  CARD8 reqType;
  CARD8 xvmcReqType;
  CARD16 length;
  CARD32 port;
  CARD32 surface_type_id;
} xvmcListSubpictureTypesReq;
#define sz_xvmcListSubpictureTypesReq 12;

typedef struct {
  BYTE type;  /* X_Reply */
  BYTE padb1;
  CARD16 sequenceNumber;
  CARD32 length;
  CARD32 num;
  CARD32 padl2;
  CARD32 padl3;
  CARD32 padl4;
  CARD32 padl5;
  CARD32 padl6;
} xvmcListSubpictureTypesReply;
#define sz_xvmcListSubpictureTypesReply 32

typedef struct {
  CARD8 reqType;
  CARD8 xvmcReqType;
  CARD16 length;
  CARD32 port;
  CARD32 shmKey;
  CARD32 magic;
} xvmcGetDRInfoReq;
#define sz_xvmcGetDRInfoReq 16;

typedef struct {
  BYTE type;  /* X_Reply */
  BYTE padb1;
  CARD16 sequenceNumber;
  CARD32 length;
  CARD32 major;
  CARD32 minor;
  CARD32 patchLevel;
  CARD32 nameLen;
  CARD32 busIDLen;
  CARD32 isLocal;
} xvmcGetDRInfoReply;
#define sz_xvmcGetDRInfoReply 32

#endif
