/*
 * libbrlapi - A library providing access to braille terminals for applications.
 *
 * Copyright (C) 2002-2020 by
 *   Samuel Thibault <Samuel.Thibault@ens-lyon.org>
 *   Sébastien Hinderer <Sebastien.Hinderer@ens-lyon.org>
 *
 * libbrlapi comes with ABSOLUTELY NO WARRANTY.
 *
 * This is free software, placed under the terms of the
 * GNU Lesser General Public License, as published by the Free Software
 * Foundation; either version 2.1 of the License, or (at your option) any
 * later version. Please see the file LICENSE-LGPL for details.
 *
 * Web Page: http://brltty.app/
 *
 * This software is maintained by Dave Mielke <dave@mielke.cc>.
 */

/** \file
 */

#ifndef BRLAPI_INCLUDED_KEYCODES
#define BRLAPI_INCLUDED_KEYCODES

#ifdef __cplusplus
extern "C" {
#endif /* __cplusplus */

/** \defgroup brlapi_keycodes Types and Defines for \e BrlAPI Key Codes
 *
 * Key codes are unsigned 64 bit integers.  This 64-bit space is split into 3
 * parts:
 *
 * - bits 63-32 (BRLAPI_KEY_FLAGS_MASK), flags: bits 39-32 are standard X
 * modifiers (shift, control, meta, ...). Other flags are used for some commands,
 * see documentation of BRLAPI_KEY_FLG_* for their respective uses.
 * - bits 31-29 (BRLAPI_KEY_TYPE_MASK), key type: either BRLAPI_KEY_TYPE_CMD for
 * braille commands, or BRLAPI_KEY_TYPE_SYM for standard X keysyms.
 * - bits 28-0 (BRLAPI_KEY_CODE_MASK), key code: for braille commands, see
 * BRLAPI_KEY_CMD_* ; for standard X keysyms, this is the keysym value, see
 * X11 documentation, a complete list is probably available on your system in
 * /usr/include/X11/keysymdef.h
 *
 * The third part is itself split into two parts: a command number and a command
 * value.  The relative sizes of these parts vary according to the key type.
 *
 * For a braille command, bits 28-16 (BRLAPI_KEY_CMD_BLK_MASK) hold the braille
 * command number, while bits 15-0 (BRLAPI_KEY_CMD_ARG_MASK) hold the command
 * value.
 *
 * For a X keysym, if it is a unicode keysym (0x1uvwxyz), then the command
 * number part is 0x1000000 and the value part is 0xuvwxyz. Else, the command
 * part is held by bits 28-8 and the value part is held by bits 7-0. This
 * permits to easily handle usual cases like 0x00xy (latin1), 0x01xy (latin2),
 * XK_Backspace (0xff08, backspace), XK_Tab (0xff09, tab), ...
 *
 * For instance, if key == 0x0000000020010008,
 * - (key & BRLAPI_KEY_TYPE_MASK) == BRLAPI_KEY_TYPE_CMD, so it's a braille
 * command
 * - (key & BRLAPI_KEY_CMD_BLK_MASK) == BRLAPI_KEY_CMD_ROUTE, so it's the
 * braille route command.
 * - (key & BRLAPI_KEY_CMD_ARG_MASK) == 8, so the highlighted cell is the 9th
 * one (cells are numbered from 0)
 * - (key & BRLAPI_KEY_FLAGS_MASK) == 0, so no modifier key was pressed during
 * the command, and no particular flag applies to the command.
 *
 * if key == 0x000000010000FF09,
 * - (key & BRLAPI_KEY_TYPE_MASK) == BRLAPI_KEY_TYPE_SYM, so it's a keysym
 * - (key & BRLAPI_KEY_CODE_MASK) == XK_Tab, so it's the tab key.
 * BRLAPI_KEY_SYM_TAB can also be used here, as well as a few other
 * BRLAPI_KEY_SYM_* constants which are provided to avoid having to include
 * X11/keysymdef.h
 * - (key & BRLAPI_KEY_FLAGS_MASK) == BRLAPI_KEY_FLG_SHIFT, so the shift
 * modifier was pressed during the command.
 *
 * in the X11 standard some keysyms are directly unicode, for instance if
 * key == 0x0000000001001EA0,
 * - (key & BRLAPI_KEY_TYPE_MASK) == BRLAPI_KEY_TYPE_SYM, so it's a keysym
 * - (key & BRLAPI_KEY_SYM_UNICODE) != 0 so it's a unicode keysym, whose value
 * is key & (BRLAPI_KEY_SYM_UNICODE-1).  Of course, one can also consider
 * (key & BRLAPI_KEY_CODE_MASK) == XK_Abelowdot
 * - (key & BRLAPI_KEY_FLAGS_MASK) == 0, so no modifier key was pressed during
 * the command, and no particular flag applies to the command.
 *
 * The brlapi_expandKeyCode() function may be used for splitting key codes into
 * these parts.
 * @{
 */
typedef uint64_t brlapi_keyCode_t;

/** Define a brlapi_keyCode_t constant */
#define BRLAPI_KEYCODE_C(value) UINT64_C(value)

/** Hexadecimal print format for brlapi_keyCode_t */
#define BRLAPI_PRIxKEYCODE PRIx64

/** Unsigned decimal print format for brlapi_keyCode_t */
#define BRLAPI_PRIuKEYCODE PRIu64

/** Brlapi_keyCode_t's biggest value
 *
 * As defined in \c <stdint.h> */
#define BRLAPI_KEY_MAX UINT64_C(0XFFFFFFFFFFFFFFFF)

/**
 * Mask for flags of brlapi_keyCode_t
 */
#define BRLAPI_KEY_FLAGS_MASK		UINT64_C(0XFFFFFFFF00000000)
/** Shift for flags of brlapi_keyCode_t */
#define BRLAPI_KEY_FLAGS_SHIFT		32

#define BRLAPI_KEY_FLG(v)		((brlapi_keyCode_t)(v) << BRLAPI_KEY_FLAGS_SHIFT)
/** Standard X modifiers */
/** Mod1 modifier (AKA meta) */
#define BRLAPI_KEY_FLG_MOD1		BRLAPI_KEY_FLG(0x00000008)
/** Mod2 modifier (usually numlock) */
#define BRLAPI_KEY_FLG_MOD2		BRLAPI_KEY_FLG(0x00000010)
/** Mod3 modifier */
#define BRLAPI_KEY_FLG_MOD3		BRLAPI_KEY_FLG(0x00000020)
/** Mod4 modifier */
#define BRLAPI_KEY_FLG_MOD4		BRLAPI_KEY_FLG(0x00000040)
/** Mod5 modifier (usually Alt-Gr) */
#define BRLAPI_KEY_FLG_MOD5		BRLAPI_KEY_FLG(0x00000080)


/**
 * Mask for type of brlapi_keyCode_t
 */
#define BRLAPI_KEY_TYPE_MASK		UINT64_C(0X00000000E0000000)
/** Shift for type of brlapi_keyCode_t */
#define BRLAPI_KEY_TYPE_SHIFT		29
/** Braille command brlapi_keyCode_t */
#define BRLAPI_KEY_TYPE_CMD		UINT64_C(0X0000000020000000)
/** X Keysym brlapi_keyCode_t */
#define BRLAPI_KEY_TYPE_SYM		UINT64_C(0X0000000000000000)

/**
 * Mask for code of brlapi_keyCode_t
 */
#define BRLAPI_KEY_CODE_MASK		UINT64_C(0X000000001FFFFFFF)
/** Shift for code of brlapi_keyCode_t */
#define BRLAPI_KEY_CODE_SHIFT		0

/** Mask for braille command type */
#define BRLAPI_KEY_CMD_BLK_MASK		UINT64_C(0X1FFF0000)
/** Shift for braille command type */
#define BRLAPI_KEY_CMD_BLK_SHIFT	16
/** Mask for braille command value */
#define BRLAPI_KEY_CMD_ARG_MASK		UINT64_C(0X0000FFFF)
/** Shift for braille command value */
#define BRLAPI_KEY_CMD_ARG_SHIFT	0
#define BRLAPI_KEY_CMD(v)		((v) << BRLAPI_KEY_CMD_BLK_SHIFT)

/** Standard X keysyms */
#define BRLAPI_KEY_SYM_BACKSPACE	UINT64_C(0X0000FF08)
#define BRLAPI_KEY_SYM_TAB		UINT64_C(0X0000FF09)
#define BRLAPI_KEY_SYM_LINEFEED		UINT64_C(0X0000FF0D)
#define BRLAPI_KEY_SYM_ESCAPE		UINT64_C(0X0000FF1B)
#define BRLAPI_KEY_SYM_HOME		UINT64_C(0X0000FF50)
#define BRLAPI_KEY_SYM_LEFT		UINT64_C(0X0000FF51)
#define BRLAPI_KEY_SYM_UP		UINT64_C(0X0000FF52)
#define BRLAPI_KEY_SYM_RIGHT		UINT64_C(0X0000FF53)
#define BRLAPI_KEY_SYM_DOWN		UINT64_C(0X0000FF54)
#define BRLAPI_KEY_SYM_PAGE_UP		UINT64_C(0X0000FF55)
#define BRLAPI_KEY_SYM_PAGE_DOWN	UINT64_C(0X0000FF56)
#define BRLAPI_KEY_SYM_END		UINT64_C(0X0000FF57)
#define BRLAPI_KEY_SYM_INSERT		UINT64_C(0X0000FF63)
#define BRLAPI_KEY_SYM_FUNCTION		UINT64_C(0X0000FFBE)
#define BRLAPI_KEY_SYM_DELETE		UINT64_C(0X0000FFFF)
#define BRLAPI_KEY_SYM_UNICODE		UINT64_C(0X01000000)

/**
 * Flag for a raw keycode press vs release
 *
 * When brlapi_enterTtyMode() has been called with a driver name,
 * brlapi_readKey() and brlapi_readKeyWithTimeout() will return
 * driver-specific key codes except for the common BRLAPI_DRV_KEY_PRESS flag
 * which indicates that it's a key press (as opposed to a release) event.
 */
#define BRLAPI_DRV_KEY_PRESS BRLAPI_KEYCODE_C(0X8000000000000000)

/** @} */

#include "brlapi_constants.h"

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif /* BRLAPI_INCLUDED_KEYCODES */
