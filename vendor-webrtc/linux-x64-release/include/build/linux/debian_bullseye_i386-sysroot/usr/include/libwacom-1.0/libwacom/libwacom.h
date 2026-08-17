/*
 * Copyright © 2011 Red Hat, Inc.
 *
 * Permission to use, copy, modify, distribute, and sell this software
 * and its documentation for any purpose is hereby granted without
 * fee, provided that the above copyright notice appear in all copies
 * and that both that copyright notice and this permission notice
 * appear in supporting documentation, and that the name of Red Hat
 * not be used in advertising or publicity pertaining to distribution
 * of the software without specific, written prior permission.  Red
 * Hat makes no representations about the suitability of this software
 * for any purpose.  It is provided "as is" without express or implied
 * warranty.
 *
 * THE AUTHORS DISCLAIM ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN
 * NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS
 * OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT,
 * NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
 * CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * Authors:
 *	Peter Hutterer (peter.hutterer@redhat.com)
 */


/** @cond hide_from_doxygen */
#ifndef _LIBWACOM_H_
#define _LIBWACOM_H_
/** @endcond */

#include <stdint.h>
#include <stdio.h>

#if defined(__GNUC__) && ((__GNUC__ * 100 + __GNUC_MINOR__) >= 301)
#define LIBWACOM_DEPRECATED  __attribute__((deprecated))
#else
#define LIBWACOM_DEPRECATED
#endif /* __GNUC__ */

/**
 @mainpage

 @section Introduction

 libwacom is a library to identify wacom tablets and their model-specific
 features. It provides easy access to information such as "is this a
 built-in on-screen tablet", "what is the size of this model", etc.

 @section Usage
 The usage of libwacom in an application could look like this:

 <pre>
      WacomDeviceDatabase *db;
      WacomDevice *device;
      WacomError *error;

      db = libwacom_database_new();
      error = libwacom_error_new();
      device = libwacom_new_from_path(db, "/dev/input/event0", WFALLBACK_NONE, error);
      if (!device)
           return; // should check for error here

      if (libwacom_device_is_builtin(device))
           printf("This is a built-in device\n");

      libwacom_destroy(device);
      libwacom_database_destroy(db);
 </pre>

 For a full API reference to see libwacom.h.

 @section Database

 libwacom comes with a database of models and their features in key-value
 format. If you cannot use libwacom, the files may be parsed directly. Note
 that the file format may change over time, especially in the beginning.
 */

/**
 @file libwacom.h
 */

typedef struct _WacomDevice WacomDevice;

typedef struct _WacomMatch WacomMatch;

typedef struct _WacomStylus WacomStylus;

typedef struct _WacomError WacomError;

typedef struct _WacomDeviceDatabase WacomDeviceDatabase;

#define WACOM_STYLUS_FALLBACK_ID 0xfffff
#define WACOM_ERASER_FALLBACK_ID 0xffffe

/**
 * Possible error codes.
 */
enum WacomErrorCode {
	WERROR_NONE,		/**< No error has occured */
	WERROR_BAD_ALLOC,	/**< Allocation error */
	WERROR_INVALID_PATH,	/**< A path specified is invalid */
	WERROR_INVALID_DB,	/**< The passed DB is invalid */
	WERROR_BAD_ACCESS,	/**< Invalid permissions to access the path */
	WERROR_UNKNOWN_MODEL,	/**< Unsupported/unknown device */
	WERROR_BUG_CALLER,	/**< A bug in the caller */
};

/**
 * Bus types for tablets.
 */
typedef enum {
	WBUSTYPE_UNKNOWN,	/**< Unknown/unsupported bus type */
	WBUSTYPE_USB,		/**< USB tablet */
	WBUSTYPE_SERIAL,	/**< Serial tablet */
	WBUSTYPE_BLUETOOTH,	/**< Bluetooth tablet */
	WBUSTYPE_I2C,		/**< I2C tablet */
} WacomBusType;

/**
 * Tablet integration.
 */
typedef enum {
	WACOM_DEVICE_INTEGRATED_NONE    = 0,
	WACOM_DEVICE_INTEGRATED_DISPLAY = (1 << 0),
	WACOM_DEVICE_INTEGRATED_SYSTEM  = (1 << 1)
} WacomIntegrationFlags;

/**
 * Classes of devices.
 *
 * @deprecated This enum should no longer be used. The classes are not
 * fine-grained or reliable enough to be useful.
 */
typedef enum {
	WCLASS_UNKNOWN,		/**< Unknown/unsupported device class */
	WCLASS_INTUOS3,		/**< Any Intuos3 series */
	WCLASS_INTUOS4,		/**< Any Intuos4 series */
	WCLASS_INTUOS5,		/**< Any Intuos5 series */
	WCLASS_CINTIQ,		/**< Any Cintiq device */
	WCLASS_BAMBOO,		/**< Any Bamboo device */
	WCLASS_GRAPHIRE,	/**< Any Graphire device */
	WCLASS_ISDV4,		/**< Any serial ISDV4 device */
	WCLASS_INTUOS,		/**< Any Intuos series */
	WCLASS_INTUOS2,		/**< Any Intuos2 series */
	WCLASS_PEN_DISPLAYS,	/**< Any "interactive pen display" */
	WCLASS_REMOTE,		/**< Any Wacom Remote */
} WacomClass;

/**
 * Class of stylus
 */
typedef enum {
	WSTYLUS_UNKNOWN,
	WSTYLUS_GENERAL,
	WSTYLUS_INKING,
	WSTYLUS_AIRBRUSH,
	WSTYLUS_CLASSIC,
	WSTYLUS_MARKER,
	WSTYLUS_STROKE,
	WSTYLUS_PUCK,
	WSTYLUS_3D,
	WSTYLUS_MOBILE,
} WacomStylusType;

/**
 * Type of eraser on a stylus
 */
typedef enum {
	WACOM_ERASER_UNKNOWN,
	WACOM_ERASER_NONE,      /**< No eraser is present on the stylus */
	WACOM_ERASER_INVERT,	/**< Eraser is a separate tool on the opposite end of the stylus */
	WACOM_ERASER_BUTTON,	/**< Eraser is a button alongside any other stylus buttons */
} WacomEraserType;

/**
 * Capabilities of the various tablet buttons
 */
typedef enum {
	WACOM_BUTTON_NONE                   = 0,
	WACOM_BUTTON_POSITION_LEFT          = (1 << 1),
	WACOM_BUTTON_POSITION_RIGHT         = (1 << 2),
	WACOM_BUTTON_POSITION_TOP           = (1 << 3),
	WACOM_BUTTON_POSITION_BOTTOM        = (1 << 4),
	WACOM_BUTTON_RING_MODESWITCH        = (1 << 5),
	WACOM_BUTTON_RING2_MODESWITCH       = (1 << 6),
	WACOM_BUTTON_TOUCHSTRIP_MODESWITCH  = (1 << 7),
	WACOM_BUTTON_TOUCHSTRIP2_MODESWITCH = (1 << 8),
	WACOM_BUTTON_OLED                   = (1 << 9),
	WACOM_BUTTON_MODESWITCH             = (WACOM_BUTTON_RING_MODESWITCH | WACOM_BUTTON_RING2_MODESWITCH | WACOM_BUTTON_TOUCHSTRIP_MODESWITCH | WACOM_BUTTON_TOUCHSTRIP2_MODESWITCH),
	WACOM_BUTTON_DIRECTION              = (WACOM_BUTTON_POSITION_LEFT | WACOM_BUTTON_POSITION_RIGHT | WACOM_BUTTON_POSITION_TOP | WACOM_BUTTON_POSITION_BOTTOM),
	WACOM_BUTTON_RINGS_MODESWITCH       = (WACOM_BUTTON_RING_MODESWITCH | WACOM_BUTTON_RING2_MODESWITCH),
	WACOM_BUTTON_TOUCHSTRIPS_MODESWITCH = (WACOM_BUTTON_TOUCHSTRIP_MODESWITCH | WACOM_BUTTON_TOUCHSTRIP2_MODESWITCH),
} WacomButtonFlags;

/**
 * Axis type for a stylus. Note that x/y is implied.
 */
typedef enum {
	WACOM_AXIS_TYPE_NONE                = 0,
	/** Tilt in x and y direction */
	WACOM_AXIS_TYPE_TILT                = (1 << 1),
	/** Rotation in the z-axis */
	WACOM_AXIS_TYPE_ROTATION_Z          = (1 << 2),
	/** Distance to surface */
	WACOM_AXIS_TYPE_DISTANCE            = (1 << 3),
	/** Tip pressure */
	WACOM_AXIS_TYPE_PRESSURE            = (1 << 4),
	/** A absolute-position slider like the wheel on the airbrush */
	WACOM_AXIS_TYPE_SLIDER              = (1 << 5),
} WacomAxisTypeFlags;

typedef enum {
	WFALLBACK_NONE = 0,
	WFALLBACK_GENERIC = 1
} WacomFallbackFlags;

typedef enum {
	WCOMPARE_NORMAL		= 0,		/**< compare the device only */
	WCOMPARE_MATCHES	= (1 << 1),	/**< compare all possible matches too */
} WacomCompareFlags;

typedef enum {
	WACOM_STATUS_LED_UNAVAILABLE	= -1,
	WACOM_STATUS_LED_RING		= 0,
	WACOM_STATUS_LED_RING2		= 1,
	WACOM_STATUS_LED_TOUCHSTRIP	= 2,
	WACOM_STATUS_LED_TOUCHSTRIP2	= 3
} WacomStatusLEDs;

/**
 * Allocate a new structure for error reporting.
 *
 * @return A newly allocated error structure or NULL if the allocation
 * failed.
 */
WacomError* libwacom_error_new(void);

/**
 * Free the error and associated memory.
 * Resets error to NULL.
 *
 * @param error A reference to a error struct.
 * @see libwacom_error_new
 */
void libwacom_error_free(WacomError **error);

/**
 * @return The code for this error.
 */
enum WacomErrorCode libwacom_error_get_code(WacomError *error);

/**
 * @return A human-readable message for this error
 */
const char* libwacom_error_get_message(WacomError *error);

/**
 * Loads the Tablet and Stylus databases, to be used
 * in libwacom_new_*() functions.
 *
 * @return A new database or NULL on error.
 */
WacomDeviceDatabase* libwacom_database_new(void);

/**
 * Loads the Tablet and Stylus databases, to be used
 * in libwacom_new_*() functions, from the prefix
 * path passes. This is only useful for diagnostics
 * applications.
 *
 * @return A new database or NULL on error.
 */
WacomDeviceDatabase* libwacom_database_new_for_path(const char *datadir);

/**
 * Free all memory used by the database.
 *
 * @param db A Tablet and Stylus database.
 */
void libwacom_database_destroy(WacomDeviceDatabase *db);

/**
 * Create a new device reference from the given device path.
 * In case of error, NULL is returned and the error is set to the
 * appropriate value.
 *
 * @param db A device database
 * @param path A device path in the form of e.g. /dev/input/event0
 * @param fallback Whether we should create a generic if model is unknown
 * @param error If not NULL, set to the error if any occurs
 *
 * @return A new reference to this device or NULL on errror.
 */
WacomDevice* libwacom_new_from_path(const WacomDeviceDatabase *db, const char *path, WacomFallbackFlags fallback, WacomError *error);

/**
 * Create a new device reference from the given vendor/product IDs.
 * In case of error, NULL is returned and the error is set to the
 * appropriate value.
 *
 * @note The term "usbid" is misleading, this function will return
 * devices with matching ids on the USB, Bluetooth or i2c bus.
 *
 * @param db A device database
 * @param vendor_id The vendor ID of the device
 * @param product_id The product ID of the device
 * @param error If not NULL, set to the error if any occurs
 *
 * @return A new reference to this device or NULL on errror.
 */
WacomDevice* libwacom_new_from_usbid(const WacomDeviceDatabase *db, int vendor_id, int product_id, WacomError *error);

/**
 * Create a new device reference from the given name.
 * In case of error, NULL is returned and the error is set to the
 * appropriate value.
 *
 * @param db A device database
 * @param name The name identifying the device
 * @param error If not NULL, set to the error if any occurs
 *
 * @return A new reference to this device or NULL on error.
 */
WacomDevice* libwacom_new_from_name(const WacomDeviceDatabase *db, const char *name, WacomError *error);

/**
 * Returns the list of devices in the given database.
 *
 * @param db A device database
 * @param error If not NULL, set to the error if any occurs
 *
 * @return A NULL terminated list of pointers to all the devices inside the
 * database.
 * The content of the list is owned by the database and should not be
 * modified or freed. Use free() to free the list.
 */
WacomDevice** libwacom_list_devices_from_database(const  WacomDeviceDatabase *db, WacomError *error);

/**
 * Print the description of this device to the given file.
 *
 * @param fd The file descriptor to print to
 * @param device The device to print the description for.
 */
void libwacom_print_device_description (int fd, const WacomDevice *device);


/**
 * Remove the device and free all memory and references to it.
 *
 * @param device The device to delete
 */
void libwacom_destroy(WacomDevice *device);

/**
 * Compare the two devices for equal-ness.
 *
 * @param a The first device
 * @param b The second device
 * @param flags Flags to dictate what constitutes a match
 *
 * @return 0 if the devices are identical, nonzero otherwise
 */
int libwacom_compare(const WacomDevice *a, const WacomDevice *b, WacomCompareFlags flags);

/**
 * @param device The tablet to query
 * @return The class of the device
 *
 * @deprecated This function should no longer be used. The classes are not
 * fine-grained or reliable enough to be useful.
 */
LIBWACOM_DEPRECATED
WacomClass libwacom_get_class(const WacomDevice *device);

/**
 * @param device The tablet to query
 * @return The human-readable name for this device
 */
const char* libwacom_get_name(const WacomDevice *device);

/**
 * @param device The tablet to query
 * @return The vendor-specific model name (e.g. CTE-650 for a Bamboo Fun), or NULL if none is set
 */
const char* libwacom_get_model_name(const WacomDevice *device);

/**
 * @param device The tablet to query
 * @return The full filename including path to the SVG layout of the device
 * if available, or NULL otherwise
 */
const char* libwacom_get_layout_filename(const WacomDevice *device);

/**
 * @param device The tablet to query
 * @return The numeric vendor ID for this device
 *
 * @bug The return value is a signed int but libwacom_match_get_vendor_id()
 * returns an unsigned int. This may cause compiler warnings, but the
 * effective range for vendor IDs is 16-bit only anyway.
 */
int libwacom_get_vendor_id(const WacomDevice *device);

/**
 * @param device The tablet to query
 * @return The current match string used for this device (if set) or the first
 * match string in the tablet definition.
 */
const char* libwacom_get_match(const WacomDevice *device);

/**
 * @param device The tablet to query
 * @return A pointer to the null-terminated list of possible matches for this device. Do not
 * modify this pointer or any content!
 */
const WacomMatch** libwacom_get_matches(const WacomDevice *device);

/**
 * Return the match string of the paired device for this device. A paired
 * device is a device with a different match string but that shares the
 * physical device with this device.
 *
 * If the return value is NULL, no device is paired with this device or all
 * paired devices have the same WacomMatch as this device.
 *
 * The returned device may not be a libwacom device itself.
 *
 * @param device The tablet to query
 * @return A pointer to paired device for this device. Do not
 * modify this pointer or any content!
 */
const WacomMatch* libwacom_get_paired_device(const WacomDevice *device);

/**
 * @param device The tablet to query
 * @return The numeric product ID for this device
 *
 * @bug The return value is a signed int but libwacom_match_get_product_id()
 * returns an unsigned int. This may cause compiler warning, but the
 * effective range for product IDs is 16-bit only anyway.
 */
int libwacom_get_product_id(const WacomDevice *device);

/**
 * Retrieve the width of the device. This is the width of the usable area as
 * advertised, not the total size of the physical tablet. For e.g. an
 * Intuos4 6x9 this will return 9.
 *
 * @param device The tablet to query
 * @return The width of this device in inches
 */
int libwacom_get_width(const WacomDevice *device);

/**
 * Retrieve the height of the device. This is the height of the usable area as
 * advertised, not the total size of the physical tablet. For e.g. an
 * Intuos4 6x9 this will return 6.
 *
 * @param device The tablet to query
 * @return The width of this device in inches
 */
int libwacom_get_height(const WacomDevice *device);

/**
 * @param device The tablet to query
 * @return non-zero if the device supports styli or zero otherwise
 */
int libwacom_has_stylus(const WacomDevice *device);

/**
 * @param device The tablet to query
 * @return non-zero if the device supports touch or zero otherwise
 */
int libwacom_has_touch(const WacomDevice *device);

/**
 * Tablet buttons are numbered 'A' through to 'A' + number of buttons.
 *
 * @param device The tablet to query
 * @return The number of buttons on the tablet
 */
int libwacom_get_num_buttons(const WacomDevice *device);

/**
 * @param device The tablet to query
 * @param num_styli Return location for the number of listed styli
 * @return an array of Styli IDs supported by the device
 */
const int *libwacom_get_supported_styli(const WacomDevice *device, int *num_styli);

/**
 * @param device The tablet to query
 * @return non-zero if the device has a touch ring or zero otherwise
 */
int libwacom_has_ring(const WacomDevice *device);

/**
 * @param device The tablet to query
 * @return non-zero if the device has a second touch ring or zero otherwise
 */
int libwacom_has_ring2(const WacomDevice *device);

/**
 * @param device The tablet to query
 * @return non-zero if the device has a touch switch or zero otherwise
 */
int libwacom_has_touchswitch(const WacomDevice *device);

/**
 * @param device The tablet to query
 * @return the number of modes for the touchring if it has a mode switch
 */
int libwacom_get_ring_num_modes(const WacomDevice *device);

/**
 * @param device The tablet to query
 * @return the number of modes for the second touchring if it has a mode switch
 */
int libwacom_get_ring2_num_modes(const WacomDevice *device);

/**
 * @param device The tablet to query
 * @return the number of touch strips on the tablet
 * otherwise
 */
int libwacom_get_num_strips(const WacomDevice *device);

/**
 * @param device The tablet to query
 * @return the number of modes for each of the touchstrips if any
 */
int libwacom_get_strips_num_modes(const WacomDevice *device);

/**
 * @param device The tablet to query
 * @param num_leds Return location for the number of supported status LEDs
 * @return an array of status LEDs supported by the device
 */
const WacomStatusLEDs *libwacom_get_status_leds(const WacomDevice *device, int *num_leds);

/**
 * @param device The tablet to query
 * @param button The ID of the button to check for, between 'A' and 'Z'
 * @return the status LED group id to use
 * or -1 if no LED is available for the given tablet / button
 */
int libwacom_get_button_led_group (const WacomDevice *device,
				   char               button);

/**
 * @param device The tablet to query
 * @return non-zero if the device is built into the screen (ie a screen tablet)
 * or zero if the device is an external tablet
 * @deprecated 0.7 Use libwacom_get_integration_flags() instead.
 */
int libwacom_is_builtin(const WacomDevice *device) LIBWACOM_DEPRECATED;

/**
 * @param device The tablet to query
 * @return non-zero if the device can be used left-handed
 * (rotated 180 degrees)
 */
int libwacom_is_reversible(const WacomDevice *device);

/**
 * @param device The tablet to query
 * @return the integration flags for the device
 */
WacomIntegrationFlags libwacom_get_integration_flags (const WacomDevice *device);

/**
 * @param device The tablet to query
 * @return The bustype of this device.
 */
WacomBusType libwacom_get_bustype(const WacomDevice *device);

/**
 * @param device The tablet to query
 * @param button The ID of the button to check for, between 'A' and 'Z'
 * @return a WacomButtonFlags with information about the button
 */
WacomButtonFlags libwacom_get_button_flag(const WacomDevice *device,
					  char               button);

/**
 * @param device The tablet to query
 * @param button The ID of the button to check for, between 'A' and 'Z'
 * @return The evdev event code sent when the button is pressed or 0 if
 * unknown.
 */
int libwacom_get_button_evdev_code(const WacomDevice *device,
				   char               button);

/**
 * Get the WacomStylus for the given tool ID.
 *
 * @param db A Tablet and Stylus database.
 * @param id The Tool ID for this stylus
 * @return A WacomStylus representing the stylus. Do not free.
 */
const WacomStylus *libwacom_stylus_get_for_id (const WacomDeviceDatabase *db, int id);

/**
 * @param stylus The stylus to query
 * @return the ID of the tool
 */
int         libwacom_stylus_get_id (const WacomStylus *stylus);

/**
 * @param stylus The stylus to query
 * @return The name of the stylus
 */
const char *libwacom_stylus_get_name (const WacomStylus *stylus);

/**
 * @param stylus The stylus to query
 * @param num_paired_ids The length of the returned list
 * @return The list of other IDs paired to this stylus
 */
const int *libwacom_stylus_get_paired_ids(const WacomStylus *stylus, int *num_paired_ids);

/**
 * @param stylus The stylus to query
 * @return The number of buttons on the stylus
 */
int         libwacom_stylus_get_num_buttons (const WacomStylus *stylus);

/**
 * Check if the given stylus is paired with a separate eraser.
 *
 * If this function returns @c true then the tool described by the given
 * WacomStylus is paired with a separate eraser tool. The actual eraser
 * tool may be located by iterating over the list of paired styli.
 *
 * @param stylus The stylus to query
 * @return Whether the stylus is paired with an eraser
 * @see libwacom_stylus_get_paired_ids
 * @see libwacom_stylus_is_eraser
 */
int         libwacom_stylus_has_eraser (const WacomStylus *stylus);

/**
 * Check if the given stylus may act like an eraser.
 *
 * If this function returns @c true then the tool described by the given
 * WacomStylus may act like an eraser. Such a tool may be dedicated to
 * sending just eraser events (and paired with a separate tool for "tip"
 * events) or capable of sending both both tip and eraser events.
 *
 * @param stylus The stylus to query
 * @return Whether the stylus can act as an eraser
 * @see libwacom_stylus_get_eraser_type
 * @see libwacom_stylus_has_eraser
 */
int         libwacom_stylus_is_eraser (const WacomStylus *stylus);

/**
 * @param stylus The stylus to query
 * @return Whether the stylus has a lens
 */
int         libwacom_stylus_has_lens (const WacomStylus *stylus);

/**
 * @param stylus The stylus to query
 * @return Whether the stylus has a relative mouse wheel
 */
int         libwacom_stylus_has_wheel (const WacomStylus *stylus);

/**
 * @param stylus The stylus to query
 * @return The flags specifying the list of absolute axes
 */
WacomAxisTypeFlags libwacom_stylus_get_axes (const WacomStylus *stylus);

/**
 * @param stylus The stylus to query
 * @return The type of stylus
 */
WacomStylusType libwacom_stylus_get_type (const WacomStylus *stylus);

/**
 * @param stylus The stylus to query
 * @return The type of eraser on the stylus
 */
WacomEraserType libwacom_stylus_get_eraser_type (const WacomStylus *stylus);

/**
 * Print the description of this stylus to the given file.
 *
 * @param fd The file descriptor
 * @param stylus The stylus to print the description for.
 */
void libwacom_print_stylus_description (int fd, const WacomStylus *stylus);

const char *libwacom_match_get_name(const WacomMatch *match);
WacomBusType libwacom_match_get_bustype(const WacomMatch *match);
uint32_t libwacom_match_get_product_id(const WacomMatch *match);
uint32_t libwacom_match_get_vendor_id(const WacomMatch *match);
const char* libwacom_match_get_match_string(const WacomMatch *match);

/** @cond hide_from_doxygen */
#endif /* _LIBWACOM_H_ */
/** @endcond */

/* vim: set noexpandtab tabstop=8 shiftwidth=8: */
