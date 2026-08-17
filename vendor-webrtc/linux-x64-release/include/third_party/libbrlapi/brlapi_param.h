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

#ifndef BRLAPI_INCLUDED_PARAM
#define BRLAPI_INCLUDED_PARAM

#ifdef __cplusplus
extern "C" {
#endif /* __cplusplus */

#include "brlapi_keycodes.h"

/** \ingroup brlapi_parameterManagement
 *
 * @{ */

typedef enum {
//Connection Parameters
  BRLAPI_PARAM_SERVER_VERSION = 0,		/**< Version of the server: uint32_t */
  BRLAPI_PARAM_CLIENT_PRIORITY = 1,		/**< Priority of the client: uint32_t (from 0 through 100, default is 50) */

//Device Parameters
  BRLAPI_PARAM_DRIVER_NAME = 2,			/**< Full name of the driver: string */
  BRLAPI_PARAM_DRIVER_CODE = 3,			/**< Code (short name) of the driver: string */
  BRLAPI_PARAM_DRIVER_VERSION = 4,		/**< Version of the driver: string */
  BRLAPI_PARAM_DEVICE_MODEL = 5,		/**< Model of the device: string */
  BRLAPI_PARAM_DEVICE_CELL_SIZE = 31,		/**< Number of dots in a cell: uint8_t */
  BRLAPI_PARAM_DISPLAY_SIZE = 6,		/**< Dimensions of the braille display: { uint32_t columns; uint32_t rows; } */
  BRLAPI_PARAM_DEVICE_IDENTIFIER = 7,		/**< Identifier of the device: string */
  BRLAPI_PARAM_DEVICE_SPEED = 8,		/**< Speed of the device: uint32_t */
  BRLAPI_PARAM_DEVICE_ONLINE = 9,		/**< Device is online: boolean */
/* TODO: status area */

//Input Parameters
  BRLAPI_PARAM_RETAIN_DOTS = 10,		/**< Pass dot combinations (rather than characters): boolean */

//Braille Rendering Parameters
  BRLAPI_PARAM_COMPUTER_BRAILLE_CELL_SIZE = 11,	/**< Number of dots used to render a computer braille character: uint8_t (8 or 6) */
  BRLAPI_PARAM_LITERARY_BRAILLE = 12,		/**< Whether braille is literary (rather than computer): boolean */
  BRLAPI_PARAM_CURSOR_DOTS = 13,		/**< Representation of the cursor: uint8_t (ISO 11548-1) */
  BRLAPI_PARAM_CURSOR_BLINK_PERIOD = 14,	/**< Blinking period of the cursor: uint32_t (milliseconds) */
  BRLAPI_PARAM_CURSOR_BLINK_PERCENTAGE = 15,	/**< Portion of the blinking period that the cursor is visible: uint8_t (from 0 through 100) */
  BRLAPI_PARAM_RENDERED_CELLS = 16,		/**< Cells rendered by the client: uint8_t[] (ISO 11548-1), one cell per element */

//Navigation Parameters
  BRLAPI_PARAM_SKIP_IDENTICAL_LINES = 17,	/**< Whether to skip identical screen lines: boolean */
  BRLAPI_PARAM_AUDIBLE_ALERTS = 18,		/**< Whether to use audible alerts: boolean */

//Clipboard Parameters
  BRLAPI_PARAM_CLIPBOARD_CONTENT = 19,		/**< Content of the clipboard: UTF-8 string */

//TTY Mode Parameters
  BRLAPI_PARAM_BOUND_COMMAND_CODES = 20,	/**< Commands bound for the device:
						  * uint64_t[], one command code per element */
  BRLAPI_PARAM_COMMAND_SHORT_NAME = 21,		/**< Short name for a command
						  * (specified via the subparam argument):
						  * string (usually a few characters) */
  BRLAPI_PARAM_COMMAND_LONG_NAME = 22,		/**< Long name for a command
						  * (specified via the subparam argument):
						  * string (usually a few words) */

//Raw Mode Parameters
  BRLAPI_PARAM_DEVICE_KEY_CODES = 23,		/**< Keys defined for the device:
						  * uint64_t[], one key code per element */
  BRLAPI_PARAM_KEY_SHORT_NAME = 24,		/**< Short name for a key
						  * (specified via the subparam argument):
						  * string (usually a few characters) */
  BRLAPI_PARAM_KEY_LONG_NAME = 25,		/**< Long name for a key
						  * (specified via the subparam argument):
						  * string (usually a few words) */

//Braille Translation Parameters
  BRLAPI_PARAM_COMPUTER_BRAILLE_ROWS_MASK = 26,	/**< Set of Unicode rows that are defined for computer braille
						  * (from U+0000 through U+10FFFF):
						  * uint8_t[544], one bit per row, eight rows per element */
  BRLAPI_PARAM_COMPUTER_BRAILLE_ROW_CELLS = 27,	/**< Computer braille cells for a Unicode row
						  * (specified via the subparam argument):
						  * uint8_t[256] (ISO 11548-1), one cell per element */
  BRLAPI_PARAM_COMPUTER_BRAILLE_TABLE = 28,	/**< Name of the computer braille table: string */
  BRLAPI_PARAM_LITERARY_BRAILLE_TABLE = 29,	/**< Name of the literary braille table: string */
  BRLAPI_PARAM_MESSAGE_LOCALE = 30,		/**< Locale to use for messages: string */
/* TODO: dot-to-unicode as well */

 /* TODO: help strings */

  BRLAPI_PARAM_COUNT = 32 /** Number of parameters */
} brlapi_param_t;

/* brlapi_param_subparam_t */
/** Type to be used for specifying a sub-parameter */
typedef uint64_t brlapi_param_subparam_t;

/* brlapi_param_bool_t */
/** Type to be used for boolean parameters */
typedef uint8_t brlapi_param_bool_t;

/* brlapi_param_serverVersion_t */
/** Type to be used for BRLAPI_PARAM_SERVER_VERSION */
typedef uint32_t brlapi_param_serverVersion_t;

/* brlapi_param_clientPriority_t */
/** Type to be used for BRLAPI_PARAM_CLIENT_PRIORITY */
typedef uint32_t brlapi_param_clientPriority_t;

/* BRLAPI_PARAM_CLIENT_PRIORITY_DEFAULT */
/** Default value for BRLAPI_PARAM_CLIENT_PRIORITY */
#define BRLAPI_PARAM_CLIENT_PRIORITY_DEFAULT 50

/* brlapi_param_driverName_t */
/** Type to be used for BRLAPI_PARAM_DRIVER_NAME */
typedef char *brlapi_param_driverName_t;

/* brlapi_param_driverCode_t */
/** Type to be used for BRLAPI_PARAM_DRIVER_CODE */
typedef char *brlapi_param_driverCode_t;

/* brlapi_param_driverVersion_t */
/** Type to be used for BRLAPI_PARAM_DRIVER_VERSION */
typedef char *brlapi_param_driverVersion_t;

/* brlapi_param_deviceModel_t */
/** Type to be used for BRLAPI_PARAM_DEVICE_MODEL */
typedef char *brlapi_param_deviceModel_t;

/* brlapi_param_deviceCellSize_t */
/** Type to be used for BRLAPI_PARAM_DEVICE_CELL_SIZE */
typedef uint8_t brlapi_param_deviceCellSize_t;

/* brlapi_param_displaySize_t */
/** Type to be used for BRLAPI_PARAM_DISPLAY_SIZE */
typedef struct {
  uint32_t columns;
  uint32_t rows;
} brlapi_param_displaySize_t;

/* brlapi_param_deviceIdentifier_t */
/** Type to be used for BRLAPI_PARAM_DEVICE_IDENTIFIER */
typedef char *brlapi_param_deviceIdentifier_t;

/* brlapi_param_deviceSpeed_t */
/** Type to be used for BRLAPI_PARAM_DEVICE_SPEED */
typedef uint32_t brlapi_param_deviceSpeed_t;

/* brlapi_param_deviceOnline_t */
/** Type to be used for BRLAPI_PARAM_DEVICE_ONLINE */
typedef brlapi_param_bool_t brlapi_param_deviceOnline_t;

/* brlapi_param_retainDots_t */
/** Type to be used for BRLAPI_PARAM_RETAIN_DOTS */
typedef brlapi_param_bool_t brlapi_param_retainDots_t;

/* brlapi_param_computerBrailleCellSize_t */
/** Type to be used for BRLAPI_PARAM_COMPUTER_BRAILLE_CELL_SIZE */
typedef uint8_t brlapi_param_computerBrailleCellSize_t;

/* brlapi_param_literaryBraille_t */
/** Type to be used for BRLAPI_PARAM_LITERARY_BRAILLE */
typedef brlapi_param_bool_t brlapi_param_literaryBraille_t;

/* brlapi_param_cursorDots_t */
/** Type to be used for BRLAPI_PARAM_CURSOR_DOTS */
typedef uint8_t brlapi_param_cursorDots_t;

/* brlapi_param_cursorBlinkPeriod_t */
/** Type to be used for BRLAPI_PARAM_CURSOR_BLINK_PERIOD */
typedef uint32_t brlapi_param_cursorBlinkPeriod_t;

/* brlapi_param_cursorBlinkPercentage_t */
/** Type to be used for BRLAPI_PARAM_CURSOR_BLINK_PERCENTAGE */
typedef uint8_t brlapi_param_cursorBlinkPercentage_t;

/* brlapi_param_renderedCells_t */
/** Type to be used for BRLAPI_PARAM_RENDERED_CELLS */
typedef uint8_t *brlapi_param_renderedCells_t;

/* brlapi_param_skipIdenticalLines_t */
/** Type to be used for BRLAPI_PARAM_SKIP_IDENTICAL_LINES */
typedef brlapi_param_bool_t brlapi_param_skipIdenticalLines_t;

/* brlapi_param_audibleAlerts_t */
/** Type to be used for BRLAPI_PARAM_AUDIBLE_ALERTS */
typedef brlapi_param_bool_t brlapi_param_audibleAlerts_t;

/* brlapi_param_clipboardContent_t */
/** Type to be used for BRLAPI_PARAM_CLIPBOARD_CONTENT */
typedef char *brlapi_param_clipboardContent_t;

/* brlapi_param_commandCode_t */
/** Type to be used for BRLAPI_PARAM_BOUND_COMMAND_CODES */
typedef brlapi_keyCode_t brlapi_param_commandCode_t;

/* brlapi_param_commandShortName_t */
/** Type to be used for BRLAPI_PARAM_COMMAND_SHORT_NAME */
typedef char *brlapi_param_commandShortName_t;

/* brlapi_param_commandLongName_t */
/** Type to be used for BRLAPI_PARAM_COMMAND_LONG_NAME */
typedef char *brlapi_param_commandLongName_t;

/* brlapi_param_keyCode_t */
/** Type to be used for BRLAPI_PARAM_DEVICE_KEY_CODES */
typedef brlapi_keyCode_t brlapi_param_keyCode_t;

/* brlapi_param_keyShortName_t */
/** Type to be used for BRLAPI_PARAM_KEY_SHORT_NAME */
typedef char *brlapi_param_keyShortName_t;

/* brlapi_param_keyLongName_t */
/** Type to be used for BRLAPI_PARAM_KEY_LONG_NAME */
typedef char *brlapi_param_keyLongName_t;

/* brlapi_param_computerBrailleRowsMask_t */
/** Type to be used for BRLAPI_PARAM_COMPUTER_BRAILLE_ROWS_MASK */
typedef uint8_t brlapi_param_computerBrailleRowsMask_t[544];

/* brlapi_param_computerBrailleRowCells_t */
/** Type to be used for BRLAPI_PARAM_COMPUTER_BRAILLE_ROW_CELLS */
typedef struct {
  uint8_t cells[0X100];
  uint8_t defined[0X100 / 8];
} brlapi_param_computerBrailleRowCells_t;

/* brlapi_param_computerBrailleTable_t */
/** Type to be used for BRLAPI_PARAM_COMPUTER_BRAILLE_TABLE */
typedef char *brlapi_param_computerBrailleTable_t;

/* brlapi_param_literaryBrailleTable_t */
/** Type to be used for BRLAPI_PARAM_LITERARY_BRAILLE_TABLE */
typedef char *brlapi_param_literaryBrailleTable_t;

/* brlapi_param_messageLocale_t */
/** Type to be used for BRLAPI_PARAM_MESSAGE_LOCALE      */
typedef char *brlapi_param_messageLocale_t;

/** Enumeration of parameter value types */
typedef enum {
  BRLAPI_PARAM_TYPE_STRING,	/**< Parameter is a string of UTF-8 characters */
  BRLAPI_PARAM_TYPE_BOOLEAN,	/**< Parameter is one or more booleans represented by a uint8_t */
  BRLAPI_PARAM_TYPE_UINT8,	/**< Parameter is one or more 8-bit unsigned integers */
  BRLAPI_PARAM_TYPE_UINT16,	/**< Parameter is one or more 16-bit unsigned integers */
  BRLAPI_PARAM_TYPE_UINT32,	/**< Parameter is one or more 32-bit unsigned integers */
  BRLAPI_PARAM_TYPE_UINT64,	/**< Parameter is one or more 64-bit unsigned integers */
  BRLAPI_PARAM_TYPE_KEYCODE = BRLAPI_PARAM_TYPE_UINT64,	/**< Parameter is one or more key codes */
} brlapi_param_type_t;

/** Structure that describes the properties of a parameter */
typedef struct {
  brlapi_param_type_t type;	/**< Type of the parameter's value */
  uint16_t count;		/**< Number of elements in the parameter's value */
  uint8_t isArray;		/**< Whether the parameter contains several values, or always only one */
  uint8_t hasSubparam;		/**< Parameter uses the subparam argument */
} brlapi_param_properties_t;

/** Enumeration of parameter types */
/* brlapi_getParameterProperties */
/** Return a description of the properties of a parameter
 *
 * \param parameter is the parameter whose properties describion shall be returned.
 *
 * \return a pointer to the description of the properties of the parameter.
 */
extern const brlapi_param_properties_t *brlapi_getParameterProperties(brlapi_param_t parameter);

/** @} */

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif /* BRLAPI_INCLUDED_PARAM */
