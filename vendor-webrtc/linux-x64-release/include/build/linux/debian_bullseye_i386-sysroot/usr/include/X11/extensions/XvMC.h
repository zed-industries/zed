#ifndef _XVMC_H_
#define _XVMC_H_

#include <X11/X.h>
#include <X11/extensions/Xv.h>

#define XvMCName "XVideo-MotionCompensation"
#define XvMCNumEvents 0
#define XvMCNumErrors 3
#define XvMCVersion 1
#define XvMCRevision 1

#define XvMCBadContext          0
#define XvMCBadSurface          1
#define XvMCBadSubpicture       2

/* Chroma formats */
#define XVMC_CHROMA_FORMAT_420          0x00000001
#define XVMC_CHROMA_FORMAT_422          0x00000002
#define XVMC_CHROMA_FORMAT_444          0x00000003

/* XvMCSurfaceInfo Flags */
#define XVMC_OVERLAID_SURFACE                   0x00000001
#define XVMC_BACKEND_SUBPICTURE                 0x00000002
#define XVMC_SUBPICTURE_INDEPENDENT_SCALING     0x00000004
#define XVMC_INTRA_UNSIGNED                     0x00000008

/* Motion Compensation types */
#define XVMC_MOCOMP                     0x00000000
#define XVMC_IDCT                       0x00010000

#define XVMC_MPEG_1                     0x00000001
#define XVMC_MPEG_2                     0x00000002
#define XVMC_H263                       0x00000003
#define XVMC_MPEG_4                     0x00000004

#define XVMC_MB_TYPE_MOTION_FORWARD     0x02
#define XVMC_MB_TYPE_MOTION_BACKWARD    0x04
#define XVMC_MB_TYPE_PATTERN            0x08
#define XVMC_MB_TYPE_INTRA              0x10

#define XVMC_PREDICTION_FIELD           0x01
#define XVMC_PREDICTION_FRAME           0x02
#define XVMC_PREDICTION_DUAL_PRIME      0x03
#define XVMC_PREDICTION_16x8            0x02
#define XVMC_PREDICTION_4MV             0x04

#define XVMC_SELECT_FIRST_FORWARD       0x01
#define XVMC_SELECT_FIRST_BACKWARD      0x02
#define XVMC_SELECT_SECOND_FORWARD      0x04
#define XVMC_SELECT_SECOND_BACKWARD     0x08

#define XVMC_DCT_TYPE_FRAME             0x00
#define XVMC_DCT_TYPE_FIELD             0x01

#define XVMC_TOP_FIELD          0x00000001
#define XVMC_BOTTOM_FIELD       0x00000002
#define XVMC_FRAME_PICTURE      (XVMC_TOP_FIELD | XVMC_BOTTOM_FIELD)

#define XVMC_SECOND_FIELD       0x00000004

#define XVMC_DIRECT             0x00000001

#define XVMC_RENDERING          0x00000001
#define XVMC_DISPLAYING         0x00000002


typedef struct {
   int surface_type_id;
   int chroma_format;
   unsigned short max_width;
   unsigned short max_height;
   unsigned short subpicture_max_width;
   unsigned short subpicture_max_height;
   int mc_type;
   int flags;
} XvMCSurfaceInfo;

typedef struct {
   XID context_id;
   int surface_type_id;
   unsigned short width;
   unsigned short height;
   XvPortID port;
   int flags;
   void * privData;  /* private to the library */
} XvMCContext;

typedef struct {
  XID surface_id;
  XID context_id;
  int surface_type_id;
  unsigned short width;
  unsigned short height;
  void *privData;  /* private to the library */
} XvMCSurface;

typedef struct {
  XID subpicture_id;
  XID context_id;
  int xvimage_id;
  unsigned short width;
  unsigned short height;
  int num_palette_entries;
  int entry_bytes;
  char component_order[4];
  void *privData;    /* private to the library */
} XvMCSubpicture;

typedef struct {
  unsigned int num_blocks;
  XID context_id;
  void *privData;
  short *blocks;
} XvMCBlockArray;

typedef struct {
   unsigned short x;
   unsigned short y;
   unsigned char macroblock_type;
   unsigned char motion_type;
   unsigned char motion_vertical_field_select;
   unsigned char dct_type;
   short PMV[2][2][2];
   unsigned int index;
   unsigned short coded_block_pattern;
   unsigned short pad0;
} XvMCMacroBlock;


typedef struct {
  unsigned int num_blocks;
  XID context_id;
  void *privData;
  XvMCMacroBlock *macro_blocks;
} XvMCMacroBlockArray;

#endif
