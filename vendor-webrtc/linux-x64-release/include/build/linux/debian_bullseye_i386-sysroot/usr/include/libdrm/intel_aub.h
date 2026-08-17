/*
 * Copyright © 2010 Intel Corporation
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the next
 * paragraph) shall be included in all copies or substantial portions of the
 * Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
 * IN THE SOFTWARE.
 *
 * Authors:
 *    Eric Anholt <eric@anholt.net>
 *
 */

/** @file intel_aub.h
 *
 * The AUB file is a file format used by Intel's internal simulation
 * and other validation tools.  It can be used at various levels by a
 * driver to input state to the simulated hardware or a replaying
 * debugger.
 *
 * We choose to dump AUB files using the trace block format for ease
 * of implementation -- dump out the blocks of memory as plain blobs
 * and insert ring commands to execute the batchbuffer blob.
 */

#ifndef _INTEL_AUB_H
#define _INTEL_AUB_H

#define AUB_MI_NOOP			(0)
#define AUB_MI_BATCH_BUFFER_START 	(0x31 << 23)
#define AUB_PIPE_CONTROL		(0x7a000002)

/* DW0: instruction type. */

#define CMD_AUB			(7 << 29)

#define CMD_AUB_HEADER		(CMD_AUB | (1 << 23) | (0x05 << 16))
/* DW1 */
# define AUB_HEADER_MAJOR_SHIFT		24
# define AUB_HEADER_MINOR_SHIFT		16

#define CMD_AUB_TRACE_HEADER_BLOCK (CMD_AUB | (1 << 23) | (0x41 << 16))
#define CMD_AUB_DUMP_BMP           (CMD_AUB | (1 << 23) | (0x9e << 16))

/* DW1 */
#define AUB_TRACE_OPERATION_MASK	0x000000ff
#define AUB_TRACE_OP_COMMENT		0x00000000
#define AUB_TRACE_OP_DATA_WRITE		0x00000001
#define AUB_TRACE_OP_COMMAND_WRITE	0x00000002
#define AUB_TRACE_OP_MMIO_WRITE		0x00000003
// operation = TRACE_DATA_WRITE, Type
#define AUB_TRACE_TYPE_MASK		0x0000ff00
#define AUB_TRACE_TYPE_NOTYPE		(0 << 8)
#define AUB_TRACE_TYPE_BATCH		(1 << 8)
#define AUB_TRACE_TYPE_VERTEX_BUFFER	(5 << 8)
#define AUB_TRACE_TYPE_2D_MAP		(6 << 8)
#define AUB_TRACE_TYPE_CUBE_MAP		(7 << 8)
#define AUB_TRACE_TYPE_VOLUME_MAP	(9 << 8)
#define AUB_TRACE_TYPE_1D_MAP		(10 << 8)
#define AUB_TRACE_TYPE_CONSTANT_BUFFER	(11 << 8)
#define AUB_TRACE_TYPE_CONSTANT_URB	(12 << 8)
#define AUB_TRACE_TYPE_INDEX_BUFFER	(13 << 8)
#define AUB_TRACE_TYPE_GENERAL		(14 << 8)
#define AUB_TRACE_TYPE_SURFACE		(15 << 8)


// operation = TRACE_COMMAND_WRITE, Type =
#define AUB_TRACE_TYPE_RING_HWB		(1 << 8)
#define AUB_TRACE_TYPE_RING_PRB0	(2 << 8)
#define AUB_TRACE_TYPE_RING_PRB1	(3 << 8)
#define AUB_TRACE_TYPE_RING_PRB2	(4 << 8)

// Address space
#define AUB_TRACE_ADDRESS_SPACE_MASK	0x00ff0000
#define AUB_TRACE_MEMTYPE_GTT		(0 << 16)
#define AUB_TRACE_MEMTYPE_LOCAL		(1 << 16)
#define AUB_TRACE_MEMTYPE_NONLOCAL	(2 << 16)
#define AUB_TRACE_MEMTYPE_PCI		(3 << 16)
#define AUB_TRACE_MEMTYPE_GTT_ENTRY     (4 << 16)

/* DW2 */

/**
 * aub_state_struct_type enum values are encoded with the top 16 bits
 * representing the type to be delivered to the .aub file, and the bottom 16
 * bits representing the subtype.  This macro performs the encoding.
 */
#define ENCODE_SS_TYPE(type, subtype) (((type) << 16) | (subtype))

enum aub_state_struct_type {
   AUB_TRACE_VS_STATE =			ENCODE_SS_TYPE(AUB_TRACE_TYPE_GENERAL, 1),
   AUB_TRACE_GS_STATE =			ENCODE_SS_TYPE(AUB_TRACE_TYPE_GENERAL, 2),
   AUB_TRACE_CLIP_STATE =		ENCODE_SS_TYPE(AUB_TRACE_TYPE_GENERAL, 3),
   AUB_TRACE_SF_STATE =			ENCODE_SS_TYPE(AUB_TRACE_TYPE_GENERAL, 4),
   AUB_TRACE_WM_STATE =			ENCODE_SS_TYPE(AUB_TRACE_TYPE_GENERAL, 5),
   AUB_TRACE_CC_STATE =			ENCODE_SS_TYPE(AUB_TRACE_TYPE_GENERAL, 6),
   AUB_TRACE_CLIP_VP_STATE =		ENCODE_SS_TYPE(AUB_TRACE_TYPE_GENERAL, 7),
   AUB_TRACE_SF_VP_STATE =		ENCODE_SS_TYPE(AUB_TRACE_TYPE_GENERAL, 8),
   AUB_TRACE_CC_VP_STATE =		ENCODE_SS_TYPE(AUB_TRACE_TYPE_GENERAL, 0x9),
   AUB_TRACE_SAMPLER_STATE =		ENCODE_SS_TYPE(AUB_TRACE_TYPE_GENERAL, 0xa),
   AUB_TRACE_KERNEL_INSTRUCTIONS =	ENCODE_SS_TYPE(AUB_TRACE_TYPE_GENERAL, 0xb),
   AUB_TRACE_SCRATCH_SPACE =		ENCODE_SS_TYPE(AUB_TRACE_TYPE_GENERAL, 0xc),
   AUB_TRACE_SAMPLER_DEFAULT_COLOR =	ENCODE_SS_TYPE(AUB_TRACE_TYPE_GENERAL, 0xd),

   AUB_TRACE_SCISSOR_STATE =		ENCODE_SS_TYPE(AUB_TRACE_TYPE_GENERAL, 0x15),
   AUB_TRACE_BLEND_STATE =		ENCODE_SS_TYPE(AUB_TRACE_TYPE_GENERAL, 0x16),
   AUB_TRACE_DEPTH_STENCIL_STATE =	ENCODE_SS_TYPE(AUB_TRACE_TYPE_GENERAL, 0x17),

   AUB_TRACE_VERTEX_BUFFER =		ENCODE_SS_TYPE(AUB_TRACE_TYPE_VERTEX_BUFFER, 0),
   AUB_TRACE_BINDING_TABLE =		ENCODE_SS_TYPE(AUB_TRACE_TYPE_SURFACE, 0x100),
   AUB_TRACE_SURFACE_STATE =		ENCODE_SS_TYPE(AUB_TRACE_TYPE_SURFACE, 0x200),
   AUB_TRACE_VS_CONSTANTS =		ENCODE_SS_TYPE(AUB_TRACE_TYPE_CONSTANT_BUFFER, 0),
   AUB_TRACE_WM_CONSTANTS =		ENCODE_SS_TYPE(AUB_TRACE_TYPE_CONSTANT_BUFFER, 1),
};

#undef ENCODE_SS_TYPE

/**
 * Decode a aub_state_struct_type value to determine the type that should be
 * stored in the .aub file.
 */
static inline uint32_t AUB_TRACE_TYPE(enum aub_state_struct_type ss_type)
{
   return (ss_type & 0xFFFF0000) >> 16;
}

/**
 * Decode a state_struct_type value to determine the subtype that should be
 * stored in the .aub file.
 */
static inline uint32_t AUB_TRACE_SUBTYPE(enum aub_state_struct_type ss_type)
{
   return ss_type & 0xFFFF;
}

/* DW3: address */
/* DW4: len */

#endif /* _INTEL_AUB_H */
