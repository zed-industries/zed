/* -*- Mode: C; tab-width: 8; indent-tabs-mode: t; c-basic-offset: 8 -*-
 *
 * Copyright (C) 2010-2015 Richard Hughes <richard@hughsie.com>
 *
 * Licensed under the GNU Lesser General Public License Version 2.1
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301 USA
 */

#if !defined (__COLORD_H_INSIDE__) && !defined (CD_COMPILATION)
#error "Only <colord.h> can be included directly."
#endif

#ifndef __CD_ENUM_H
#define __CD_ENUM_H

#include <glib-object.h>

G_BEGIN_DECLS

/**
 * CdDeviceKind:
 *
 * The device type.
 **/
typedef enum {
	CD_DEVICE_KIND_UNKNOWN,			/* Since: 0.1.0 */
	CD_DEVICE_KIND_DISPLAY,			/* Since: 0.1.0 */
	CD_DEVICE_KIND_SCANNER,			/* Since: 0.1.0 */
	CD_DEVICE_KIND_PRINTER,			/* Since: 0.1.0 */
	CD_DEVICE_KIND_CAMERA,			/* Since: 0.1.0 */
	CD_DEVICE_KIND_WEBCAM,			/* Since: 0.1.0 */
	/*< private >*/
	CD_DEVICE_KIND_LAST
} CdDeviceKind;

/**
 * CdProfileKind:
 *
 * The profile type.
 **/
typedef enum {
	CD_PROFILE_KIND_UNKNOWN,		/* Since: 0.1.1 */
	CD_PROFILE_KIND_INPUT_DEVICE,		/* Since: 0.1.1 */
	CD_PROFILE_KIND_DISPLAY_DEVICE,		/* Since: 0.1.1 */
	CD_PROFILE_KIND_OUTPUT_DEVICE,		/* Since: 0.1.1 */
	CD_PROFILE_KIND_DEVICELINK,		/* Since: 0.1.1 */
	CD_PROFILE_KIND_COLORSPACE_CONVERSION,	/* Since: 0.1.1 */
	CD_PROFILE_KIND_ABSTRACT,		/* Since: 0.1.1 */
	CD_PROFILE_KIND_NAMED_COLOR,		/* Since: 0.1.1 */
	/*< private >*/
	CD_PROFILE_KIND_LAST
} CdProfileKind;

/**
 * CdObjectScope:
 *
 * The options type.
 **/
typedef enum {
	CD_OBJECT_SCOPE_UNKNOWN,		/* Since: 0.1.4 */
	CD_OBJECT_SCOPE_NORMAL,			/* Since: 0.1.4 */
	CD_OBJECT_SCOPE_TEMP,			/* Since: 0.1.4 */
	CD_OBJECT_SCOPE_DISK,			/* Since: 0.1.4 */
	/*< private >*/
	CD_OBJECT_SCOPE_LAST
} CdObjectScope;

/**
 * CdRenderingIntent:
 * @CD_RENDERING_INTENT_UNKNOWN:		Unknown rendering intent
 * @CD_RENDERING_INTENT_PERCEPTUAL:		Used for photos as it maintains contrast
 * @CD_RENDERING_INTENT_RELATIVE_COLORIMETRIC:	Used for graphic design and named colors
 * @CD_RENDERING_INTENT_SATURATION:		Used for business charts as it maintains saturation without dithering
 * @CD_RENDERING_INTENT_ABSOLUTE_COLORIMETRIC:	Used when a specific color is required
 *
 * The rendering intent.
 **/
typedef enum {
	CD_RENDERING_INTENT_UNKNOWN,			/* Since: 0.1.5 */
	CD_RENDERING_INTENT_PERCEPTUAL,			/* Since: 0.1.5 */
	CD_RENDERING_INTENT_RELATIVE_COLORIMETRIC,	/* Since: 0.1.5 */
	CD_RENDERING_INTENT_SATURATION,			/* Since: 0.1.5 */
	CD_RENDERING_INTENT_ABSOLUTE_COLORIMETRIC,	/* Since: 0.1.5 */
	/*< private >*/
	CD_RENDERING_INTENT_LAST
} CdRenderingIntent;

/**
 * CdPixelFormat:
 *
 * The pixel format of an image.
 * NOTE: these values are the same as the lcms2 AOTTTTTUYFPXSEEECCCCBBB type.
 **/
typedef guint32 CdPixelFormat;

#define	CD_PIXEL_FORMAT_UNKNOWN		0x00000000	/* Since: 1.0.0 */
#define	CD_PIXEL_FORMAT_ARGB32		0x00044099	/* Since: 1.0.0 */
#define	CD_PIXEL_FORMAT_RGB24		0x00040019	/* Since: 1.0.0 */
#define	CD_PIXEL_FORMAT_CMYK32		0x00060021	/* Since: 1.0.0 */
#define	CD_PIXEL_FORMAT_BGRA32		0x00044499	/* Since: 1.0.0 */
#define	CD_PIXEL_FORMAT_RGBA32		0x00040099	/* Since: 1.1.8 */

/**
 * CdColorspace:
 *
 * The known colorspace.
 **/
typedef enum {
	CD_COLORSPACE_UNKNOWN,			/* Since: 0.1.1 */
	CD_COLORSPACE_XYZ,			/* Since: 0.1.1 */
	CD_COLORSPACE_LAB,			/* Since: 0.1.1 */
	CD_COLORSPACE_LUV,			/* Since: 0.1.1 */
	CD_COLORSPACE_YCBCR,			/* Since: 0.1.1 */
	CD_COLORSPACE_YXY,			/* Since: 0.1.1 */
	CD_COLORSPACE_RGB,			/* Since: 0.1.1 */
	CD_COLORSPACE_GRAY,			/* Since: 0.1.1 */
	CD_COLORSPACE_HSV,			/* Since: 0.1.1 */
	CD_COLORSPACE_CMYK,			/* Since: 0.1.1 */
	CD_COLORSPACE_CMY,			/* Since: 0.1.1 */
	/*< private >*/
	CD_COLORSPACE_LAST
} CdColorspace;

/**
 * CdDeviceMode:
 *
 * The device mode.
 **/
typedef enum {
	CD_DEVICE_MODE_UNKNOWN,			/* Since: 0.1.2 */
	CD_DEVICE_MODE_PHYSICAL,		/* Since: 0.1.2 */
	CD_DEVICE_MODE_VIRTUAL,			/* Since: 0.1.2 */
	/*< private >*/
	CD_DEVICE_MODE_LAST
} CdDeviceMode;

/**
 * CdDeviceRelation:
 *
 * The device to profile relationship.
 **/
typedef enum {
	CD_DEVICE_RELATION_UNKNOWN,		/* Since: 0.1.3 */
	CD_DEVICE_RELATION_SOFT,		/* Since: 0.1.3 */
	CD_DEVICE_RELATION_HARD,		/* Since: 0.1.3 */
	/*< private >*/
	CD_DEVICE_RELATION_LAST
} CdDeviceRelation;

/**
 * CdSensorKind:
 *
 * The sensor type.
 **/
typedef enum {
	CD_SENSOR_KIND_UNKNOWN,			/* Since: 0.1.6 */
	CD_SENSOR_KIND_DUMMY,			/* Since: 0.1.6 */
	CD_SENSOR_KIND_HUEY,			/* Since: 0.1.6 */
	CD_SENSOR_KIND_COLOR_MUNKI_PHOTO,	/* Since: 0.1.6 */
	CD_SENSOR_KIND_SPYDER,			/* Since: 0.1.6, but not used since 0.1.16 */
	CD_SENSOR_KIND_DTP20,			/* Since: 0.1.6 */
	CD_SENSOR_KIND_DTP22,			/* Since: 0.1.6 */
	CD_SENSOR_KIND_DTP41,			/* Since: 0.1.6 */
	CD_SENSOR_KIND_DTP51,			/* Since: 0.1.6 */
	CD_SENSOR_KIND_DTP94,			/* Since: 0.1.6 */
	CD_SENSOR_KIND_SPECTRO_SCAN,		/* Since: 0.1.6 */
	CD_SENSOR_KIND_I1_PRO,			/* Since: 0.1.6 */
	CD_SENSOR_KIND_COLORIMTRE_HCFR,		/* Since: 0.1.6 */
	CD_SENSOR_KIND_I1_DISPLAY3,		/* Since: 0.1.14 */
	CD_SENSOR_KIND_COLORHUG,		/* Since: 0.1.15 */
	CD_SENSOR_KIND_SPYDER2,			/* Since: 0.1.16 */
	CD_SENSOR_KIND_SPYDER3,			/* Since: 0.1.16 */
	CD_SENSOR_KIND_COLORHUG_PLUS,		/* Since: 0.1.24 */
	CD_SENSOR_KIND_I1_DISPLAY1,		/* Since: 0.1.25 */
	CD_SENSOR_KIND_I1_DISPLAY2,		/* Since: 0.1.25 */
	CD_SENSOR_KIND_DTP92,			/* Since: 0.1.25 */
	CD_SENSOR_KIND_I1_MONITOR,		/* Since: 0.1.25 */
	CD_SENSOR_KIND_SPYDER4,			/* Since: 0.1.26 */
	CD_SENSOR_KIND_COLOR_MUNKI_SMILE,	/* Since: 0.1.27 */
	CD_SENSOR_KIND_COLORHUG2,		/* Since: 1.2.2 */
	CD_SENSOR_KIND_SPYDER5,			/* Since: 1.2.11 */
	CD_SENSOR_KIND_SPARK,			/* Since: 1.2.11 */
	CD_SENSOR_KIND_SPYDERX,                 /* Since: 1.4.5 */
	/*< private >*/
	CD_SENSOR_KIND_LAST
} CdSensorKind;

/* renamed due to trademark issue */
#define CD_SENSOR_KIND_COLORHUG_SPECTRO		CD_SENSOR_KIND_COLORHUG_PLUS

/**
 * CdSensorCap:
 *
 * The sensor capabilities, i.e. things the sensor can do.
 **/
typedef enum {
	CD_SENSOR_CAP_UNKNOWN,			/* Since: 0.1.6 */
	CD_SENSOR_CAP_LCD,			/* Since: 0.1.6 */
	CD_SENSOR_CAP_CRT,			/* Since: 0.1.6 */
	CD_SENSOR_CAP_PRINTER,			/* Since: 0.1.6 */
	CD_SENSOR_CAP_SPOT,			/* Since: 0.1.6 */
	CD_SENSOR_CAP_PROJECTOR,		/* Since: 0.1.6 */
	CD_SENSOR_CAP_AMBIENT,			/* Since: 0.1.6 */
	CD_SENSOR_CAP_CALIBRATION,		/* Since: 0.1.6 (hint: raw measurement) */
	CD_SENSOR_CAP_LED,			/* Since: 0.1.17 */
	CD_SENSOR_CAP_PLASMA,			/* Since: 0.1.29 */
	CD_SENSOR_CAP_LCD_CCFL,			/* Since: 0.1.31 */
	CD_SENSOR_CAP_LCD_RGB_LED,		/* Since: 0.1.31 */
	CD_SENSOR_CAP_LCD_WHITE_LED,		/* Since: 0.1.31 */
	CD_SENSOR_CAP_WIDE_GAMUT_LCD_CCFL,	/* Since: 0.1.31 */
	CD_SENSOR_CAP_WIDE_GAMUT_LCD_RGB_LED,	/* Since: 0.1.31 */
	CD_SENSOR_CAP_SPECTRAL,			/* Since: 1.3.1 */
	CD_SENSOR_CAP_CALIBRATION_DARK,		/* Since: 1.3.1 */
	CD_SENSOR_CAP_CALIBRATION_IRRADIANCE,	/* Since: 1.3.1 */
	/*< private >*/
	CD_SENSOR_CAP_LAST
} CdSensorCap;

/**
 * CdSensorState:
 *
 * The state of the sensor.
 **/
typedef enum {
	CD_SENSOR_STATE_UNKNOWN,		/* Since: 0.1.6 */
	CD_SENSOR_STATE_STARTING,		/* Since: 0.1.6 */
	CD_SENSOR_STATE_IDLE,			/* Since: 0.1.6 */
	CD_SENSOR_STATE_MEASURING,		/* Since: 0.1.6 */
	CD_SENSOR_STATE_BUSY,			/* Since: 0.1.19 */
	/*< private >*/
	CD_SENSOR_STATE_LAST
} CdSensorState;

/**
 * CdStandardSpace:
 *
 * A standard colorspace
 **/
typedef enum {
	CD_STANDARD_SPACE_UNKNOWN,		/* Since: 0.1.6 */
	CD_STANDARD_SPACE_SRGB,			/* Since: 0.1.6 */
	CD_STANDARD_SPACE_ADOBE_RGB,		/* Since: 0.1.6 */
	CD_STANDARD_SPACE_PROPHOTO_RGB,		/* Since: 0.1.6 */
	/*< private >*/
	CD_STANDARD_SPACE_LAST
} CdStandardSpace;

/**
 * CdProfileWarning:
 * @CD_PROFILE_WARNING_NONE: No error is found
 * @CD_PROFILE_WARNING_DESCRIPTION_MISSING: The description is missing or of zero length
 * @CD_PROFILE_WARNING_COPYRIGHT_MISSING: The copyright is missing or of zero length
 * @CD_PROFILE_WARNING_VCGT_NON_MONOTONIC: The video card gamma table is not monotonic
 * @CD_PROFILE_WARNING_SCUM_DOT: Lab 100, 0, 0 does not map to RGB 255,255,255
 * @CD_PROFILE_WARNING_GRAY_AXIS_INVALID: There is significant a/b for gray
 * @CD_PROFILE_WARNING_GRAY_AXIS_NON_MONOTONIC: The gray ramp is not monotonic
 * @CD_PROFILE_WARNING_PRIMARIES_INVALID: One or more of the primaries are invalid
 * @CD_PROFILE_WARNING_PRIMARIES_NON_ADDITIVE: The primaries to not add to give D50 white
 * @CD_PROFILE_WARNING_PRIMARIES_UNLIKELY: One or more of the primaries are outside of ROMM RGB
 * @CD_PROFILE_WARNING_WHITEPOINT_INVALID: RGB 255,255,255 does not return D50
 * @CD_PROFILE_WARNING_WHITEPOINT_UNLIKELY: Whitepoint is outside of usual range
 *
 * The warning about the profile. Profiles with warnings can still be
 * used, but may be of limited use.
 **/
typedef enum {
	/* FIXME: next API break, add CD_PROFILE_WARNING_UNKNOWN */
	CD_PROFILE_WARNING_NONE,			/* Since: 0.1.25 */
	CD_PROFILE_WARNING_DESCRIPTION_MISSING,		/* Since: 0.1.25 */
	CD_PROFILE_WARNING_COPYRIGHT_MISSING,		/* Since: 0.1.25 */
	CD_PROFILE_WARNING_VCGT_NON_MONOTONIC,		/* Since: 0.1.25 */
	CD_PROFILE_WARNING_SCUM_DOT,			/* Since: 0.1.25 */
	CD_PROFILE_WARNING_GRAY_AXIS_INVALID,		/* Since: 0.1.25 */
	CD_PROFILE_WARNING_GRAY_AXIS_NON_MONOTONIC,	/* Since: 0.1.25 */
	CD_PROFILE_WARNING_PRIMARIES_INVALID,		/* Since: 0.1.25 */
	CD_PROFILE_WARNING_PRIMARIES_NON_ADDITIVE,	/* Since: 0.1.25 */
	CD_PROFILE_WARNING_PRIMARIES_UNLIKELY,		/* Since: 0.1.25 */
	CD_PROFILE_WARNING_WHITEPOINT_INVALID,		/* Since: 0.1.25 */
	CD_PROFILE_WARNING_WHITEPOINT_UNLIKELY,		/* Since: 0.1.34 */
	/*< private >*/
	CD_PROFILE_WARNING_LAST
} CdProfileWarning;

/**
 * CdProfileQuality:
 * @CD_PROFILE_QUALITY_LOW: Low quality profile, fast
 * @CD_PROFILE_QUALITY_MEDIUM: Medium quality profile
 * @CD_PROFILE_QUALITY_HIGH: High quality profile, slow
 *
 * The quality of the profile produced through calibration.
 **/
typedef enum {
	/* FIXME: next API break, add CD_PROFILE_QUALITY_UNKNOWN */
	CD_PROFILE_QUALITY_LOW,				/* Since: 0.1.27 */
	CD_PROFILE_QUALITY_MEDIUM,			/* Since: 0.1.27 */
	CD_PROFILE_QUALITY_HIGH,			/* Since: 0.1.27 */
	/*< private >*/
	CD_PROFILE_QUALITY_LAST
} CdProfileQuality;

/**
 * CdSensorError:
 * @CD_SENSOR_ERROR_NO_SUPPORT:		This action is unsupported on this hardware
 * @CD_SENSOR_ERROR_NO_DATA:		The sensor provided no data
 * @CD_SENSOR_ERROR_INTERNAL:		An internal error occurred
 * @CD_SENSOR_ERROR_ALREADY_LOCKED:	The sensor is already locked
 * @CD_SENSOR_ERROR_NOT_LOCKED:		The sensor is not locked
 * @CD_SENSOR_ERROR_IN_USE:		The sensor is already in use
 * @CD_SENSOR_ERROR_FAILED_TO_AUTHENTICATE:	Authentication failed
 * @CD_SENSOR_ERROR_REQUIRED_POSITION_CALIBRATE:	The sensor needs to be in the calibrate position
 * @CD_SENSOR_ERROR_REQUIRED_POSITION_SURFACE:		The sensor needs to be in the surface position
 * @CD_SENSOR_ERROR_REQUIRED_DARK_CALIBRATION:		The sensor needs dark calibration
 * @CD_SENSOR_ERROR_REQUIRED_IRRADIANCE_CALIBRATION:	The sensor needs irradiance calibration
 *
 * The sensor error code.
 *
 * Since: 0.1.26
 **/
typedef enum {
	CD_SENSOR_ERROR_NO_SUPPORT,			/* Since: 0.1.26 */
	CD_SENSOR_ERROR_NO_DATA,			/* Since: 0.1.26 */
	CD_SENSOR_ERROR_INTERNAL,			/* Since: 0.1.26 */
	CD_SENSOR_ERROR_ALREADY_LOCKED,			/* Since: 0.1.26 */
	CD_SENSOR_ERROR_NOT_LOCKED,			/* Since: 0.1.26 */
	CD_SENSOR_ERROR_IN_USE,				/* Since: 0.1.26 */
	CD_SENSOR_ERROR_FAILED_TO_AUTHENTICATE,		/* Since: 0.1.26 */
	CD_SENSOR_ERROR_REQUIRED_POSITION_CALIBRATE,	/* Since: 0.1.26 */
	CD_SENSOR_ERROR_REQUIRED_POSITION_SURFACE,	/* Since: 0.1.26 */
	CD_SENSOR_ERROR_REQUIRED_DARK_CALIBRATION,	/* Since: 1.2.13 */
	CD_SENSOR_ERROR_REQUIRED_IRRADIANCE_CALIBRATION, /* Since: 1.1.1 */
	/*< private >*/
	CD_SENSOR_ERROR_LAST
} CdSensorError;

/**
 * CdProfileError:
 * @CD_PROFILE_ERROR_INTERNAL: 		An internal error occurred
 * @CD_PROFILE_ERROR_ALREADY_INSTALLED: The profile is already installed
 * @CD_PROFILE_ERROR_FAILED_TO_WRITE: 	The profile could not be written
 * @CD_PROFILE_ERROR_FAILED_TO_PARSE: 	The profile could not be parsed
 * @CD_PROFILE_ERROR_FAILED_TO_READ: 	The profile could not be read
 * @CD_PROFILE_ERROR_FAILED_TO_AUTHENTICATE:	Authentication failed
 * @CD_PROFILE_ERROR_PROPERTY_INVALID:	One or more of the properties was invalid
 * @CD_PROFILE_ERROR_FAILED_TO_GET_UID:	Failed to get UID for sender
 *
 * Errors that can be thrown
 */
typedef enum
{
	CD_PROFILE_ERROR_INTERNAL,			/* Since: 0.1.26 */
	CD_PROFILE_ERROR_ALREADY_INSTALLED,		/* Since: 0.1.26 */
	CD_PROFILE_ERROR_FAILED_TO_WRITE,		/* Since: 0.1.26 */
	CD_PROFILE_ERROR_FAILED_TO_PARSE,		/* Since: 0.1.26 */
	CD_PROFILE_ERROR_FAILED_TO_READ,		/* Since: 0.1.26 */
	CD_PROFILE_ERROR_FAILED_TO_AUTHENTICATE,	/* Since: 0.1.26 */
	CD_PROFILE_ERROR_PROPERTY_INVALID,		/* Since: 0.1.31 */
	CD_PROFILE_ERROR_FAILED_TO_GET_UID,		/* Since: 1.2.1 */
	/*< private >*/
	CD_PROFILE_ERROR_LAST
} CdProfileError;

/**
 * CdDeviceError:
 * @CD_DEVICE_ERROR_INTERNAL:		An internal error occurred
 * @CD_DEVICE_ERROR_PROFILE_DOES_NOT_EXIST:	The profile does not exist
 * @CD_DEVICE_ERROR_PROFILE_ALREADY_ADDED:	The profile has already been added
 * @CD_DEVICE_ERROR_PROFILING:		The device is being profiled
 * @CD_DEVICE_ERROR_NOTHING_MATCHED:	Nothing matched the search term
 * @CD_DEVICE_ERROR_FAILED_TO_INHIBIT:	Cound not inhibit device
 * @CD_DEVICE_ERROR_FAILED_TO_UNINHIBIT:	Cound not uninhibit device
 * @CD_DEVICE_ERROR_FAILED_TO_AUTHENTICATE:	Authentication failed
 * @CD_DEVICE_ERROR_NOT_ENABLED:	The device has been disabled
 *
 * Errors that can be thrown
 */
typedef enum
{
	CD_DEVICE_ERROR_INTERNAL,			/* Since: 0.1.26 */
	CD_DEVICE_ERROR_PROFILE_DOES_NOT_EXIST,		/* Since: 0.1.26 */
	CD_DEVICE_ERROR_PROFILE_ALREADY_ADDED,		/* Since: 0.1.26 */
	CD_DEVICE_ERROR_PROFILING,			/* Since: 0.1.26 */
	CD_DEVICE_ERROR_NOTHING_MATCHED,		/* Since: 0.1.26 */
	CD_DEVICE_ERROR_FAILED_TO_INHIBIT,		/* Since: 0.1.26 */
	CD_DEVICE_ERROR_FAILED_TO_UNINHIBIT,		/* Since: 0.1.26 */
	CD_DEVICE_ERROR_FAILED_TO_AUTHENTICATE,		/* Since: 0.1.26 */
	CD_DEVICE_ERROR_NOT_ENABLED,			/* Since: 0.1.26 */
	/*< private >*/
	CD_DEVICE_ERROR_LAST
} CdDeviceError;

/**
 * CdClientError:
 * @CD_CLIENT_ERROR_INTERNAL:		An internal error occurred
 * @CD_CLIENT_ERROR_ALREADY_EXISTS:	The profile already exists
 * @CD_CLIENT_ERROR_FAILED_TO_AUTHENTICATE:	Authentication failed
 * @CD_CLIENT_ERROR_NOT_SUPPORTED:	Feature not supported
 * @CD_CLIENT_ERROR_NOT_FOUND:		Profile or device not found
 * @CD_CLIENT_ERROR_INPUT_INVALID:	One or more of the parameters is invalid
 * @CD_CLIENT_ERROR_FILE_INVALID:	The file if invalid
 *
 * Errors that can be thrown
 */
typedef enum {
	CD_CLIENT_ERROR_INTERNAL,			/* Since: 0.1.26 */
	CD_CLIENT_ERROR_ALREADY_EXISTS,			/* Since: 0.1.26 */
	CD_CLIENT_ERROR_FAILED_TO_AUTHENTICATE,		/* Since: 0.1.26 */
	CD_CLIENT_ERROR_NOT_SUPPORTED,			/* Since: 0.1.26 */
	CD_CLIENT_ERROR_NOT_FOUND,			/* Since: 0.1.26 */
	CD_CLIENT_ERROR_INPUT_INVALID,			/* Since: 0.1.26 */
	CD_CLIENT_ERROR_FILE_INVALID,			/* Since: 0.1.26 */
	/*< private >*/
	CD_CLIENT_ERROR_LAST
} CdClientError;

/* defined in org.freedesktop.ColorManager.xml */
#define CD_CLIENT_PROPERTY_DAEMON_VERSION	"DaemonVersion"		/* Since: 0.1.0 */
#define CD_CLIENT_PROPERTY_SYSTEM_VENDOR	"SystemVendor"		/* Since: 1.0.2 */
#define CD_CLIENT_PROPERTY_SYSTEM_MODEL		"SystemModel"		/* Since: 1.0.2 */

/* defined in metadata-spec.txt */
#define CD_PROFILE_METADATA_STANDARD_SPACE	"STANDARD_space"	/* Since: 0.1.8 */
#define CD_PROFILE_METADATA_EDID_MD5		"EDID_md5"		/* Since: 0.1.8 */
#define CD_PROFILE_METADATA_EDID_MODEL		"EDID_model"		/* Since: 0.1.8 */
#define CD_PROFILE_METADATA_EDID_SERIAL		"EDID_serial"		/* Since: 0.1.8 */
#define CD_PROFILE_METADATA_EDID_MNFT		"EDID_mnft"		/* Since: 0.1.8 */
#define CD_PROFILE_METADATA_EDID_VENDOR		"EDID_manufacturer"	/* Since: 0.1.8 */
#define CD_PROFILE_METADATA_FILE_CHECKSUM	"FILE_checksum"		/* Since: 0.1.8 */
#define CD_PROFILE_METADATA_CMF_PRODUCT		"CMF_product"		/* Since: 0.1.9 */
#define CD_PROFILE_METADATA_CMF_BINARY		"CMF_binary"		/* Since: 0.1.9 */
#define CD_PROFILE_METADATA_CMF_VERSION		"CMF_version"		/* Since: 0.1.9 */
#define CD_PROFILE_METADATA_DATA_SOURCE		"DATA_source"		/* Since: 0.1.9 */
#define CD_PROFILE_METADATA_DATA_SOURCE_EDID	"edid"			/* Since: 0.1.9 */
#define CD_PROFILE_METADATA_DATA_SOURCE_CALIB	"calib"			/* Since: 0.1.9 */
#define CD_PROFILE_METADATA_DATA_SOURCE_STANDARD "standard"		/* Since: 0.1.14 */
#define CD_PROFILE_METADATA_DATA_SOURCE_TEST	"test"			/* Since: 0.1.14 */
#define CD_PROFILE_METADATA_MAPPING_FORMAT	"MAPPING_format"	/* Since: 0.1.9 */
#define CD_PROFILE_METADATA_MAPPING_QUALIFIER	"MAPPING_qualifier"	/* Since: 0.1.9 */
#define CD_PROFILE_METADATA_MAPPING_DEVICE_ID	"MAPPING_device_id"	/* Since: 0.1.9 */
#define CD_PROFILE_METADATA_ACCURACY_DE76_AVG	"ACCURACY_dE76_avg"	/* Since: 0.1.16 */
#define CD_PROFILE_METADATA_ACCURACY_DE76_MAX	"ACCURACY_dE76_max"	/* Since: 0.1.16 */
#define CD_PROFILE_METADATA_ACCURACY_DE76_RMS	"ACCURACY_dE76_rms"	/* Since: 0.1.16 */
#define CD_PROFILE_METADATA_MEASUREMENT_DEVICE	"MEASUREMENT_device"	/* Since: 0.1.16 */
#define CD_PROFILE_METADATA_SCREEN_SURFACE	"SCREEN_surface"	/* Since: 0.1.16 */
#define CD_PROFILE_METADATA_SCREEN_SURFACE_MATTE 	"matte"		/* Since: 0.1.16 */
#define CD_PROFILE_METADATA_SCREEN_SURFACE_GLOSSY	"glossy"	/* Since: 0.1.16 */
#define CD_PROFILE_METADATA_SCREEN_BRIGHTNESS	"SCREEN_brightness"	/* Since: 0.1.17 */
#define CD_PROFILE_METADATA_CONNECTION_TYPE	"CONNECTION_type"	/* Since: 0.1.16 */
#define CD_PROFILE_METADATA_CONNECTION_TYPE_INTERNAL	"internal"	/* Since: 0.1.16 */
#define CD_PROFILE_METADATA_CONNECTION_TYPE_VGA		"vga"		/* Since: 0.1.16 */
#define CD_PROFILE_METADATA_CONNECTION_TYPE_DVI		"dvi"		/* Since: 0.1.16 */
#define CD_PROFILE_METADATA_CONNECTION_TYPE_HDMI	"hdmi"		/* Since: 0.1.16 */
#define CD_PROFILE_METADATA_CONNECTION_TYPE_DISPLAYPORT	"displayport"	/* Since: 0.1.16 */
#define CD_PROFILE_METADATA_LICENSE		"License"		/* Since: 0.1.25 */
#define CD_PROFILE_METADATA_QUALITY		"Quality"		/* Since: 0.1.27 */
#define CD_PROFILE_METADATA_QUALITY_LOW		"low"			/* Since: 0.1.27 */
#define CD_PROFILE_METADATA_QUALITY_MEDIUM	"medium"		/* Since: 0.1.27 */
#define CD_PROFILE_METADATA_QUALITY_HIGH	"high"			/* Since: 0.1.27 */

/* defined in org.freedesktop.ColorManager.Profile.xml */
#define CD_PROFILE_PROPERTY_FILENAME		"Filename"		/* Since: 0.1.8 */
#define CD_PROFILE_PROPERTY_QUALIFIER		"Qualifier"		/* Since: 0.1.8 */
#define CD_PROFILE_PROPERTY_FORMAT		"Format"		/* Since: 0.1.8 */
#define CD_PROFILE_PROPERTY_COLORSPACE		"Colorspace"		/* Since: 0.1.8 */
#define CD_PROFILE_PROPERTY_TITLE		"Title"			/* Since: 0.1.8 */
#define CD_PROFILE_PROPERTY_KIND		"Kind"			/* Since: 0.1.8 */
#define CD_PROFILE_PROPERTY_CREATED		"Created"		/* Since: 0.1.8 */
#define CD_PROFILE_PROPERTY_HAS_VCGT		"HasVcgt"		/* Since: 0.1.8 */
#define CD_PROFILE_PROPERTY_IS_SYSTEM_WIDE	"IsSystemWide"		/* Since: 0.1.8 */
#define CD_PROFILE_PROPERTY_METADATA		"Metadata"		/* Since: 0.1.8 */
#define CD_PROFILE_PROPERTY_ID			"ProfileId"		/* Since: 0.1.8 */
#define CD_PROFILE_PROPERTY_SCOPE		"Scope"			/* Since: 0.1.10 */
#define CD_PROFILE_PROPERTY_OWNER		"Owner"			/* Since: 0.1.13 */
#define CD_PROFILE_PROPERTY_WARNINGS		"Warnings"		/* Since: 0.1.25 */

/* defined in metadata-spec.txt */
#define CD_DEVICE_METADATA_XRANDR_NAME		"XRANDR_name"		/* Since: 0.1.8 */
#define CD_DEVICE_METADATA_OUTPUT_EDID_MD5	"OutputEdidMd5"		/* Since: 0.1.34 */
#define CD_DEVICE_METADATA_OUTPUT_PRIORITY	"OutputPriority"	/* Since: 0.1.25 */
#define CD_DEVICE_METADATA_OUTPUT_PRIORITY_PRIMARY	"primary"	/* Since: 0.1.25 */
#define CD_DEVICE_METADATA_OUTPUT_PRIORITY_SECONDARY	"secondary"	/* Since: 0.1.25 */
#define CD_DEVICE_METADATA_OWNER_CMDLINE	"OwnerCmdline"		/* Since: 0.1.29 */

/* defined in org.freedesktop.ColorManager.Device.xml */
#define CD_DEVICE_PROPERTY_MODEL		"Model"			/* Since: 0.1.8 */
#define CD_DEVICE_PROPERTY_KIND			"Kind"			/* Since: 0.1.8 */
#define CD_DEVICE_PROPERTY_VENDOR		"Vendor"		/* Since: 0.1.8 */
#define CD_DEVICE_PROPERTY_SERIAL		"Serial"		/* Since: 0.1.8 */
#define CD_DEVICE_PROPERTY_COLORSPACE		"Colorspace"		/* Since: 0.1.8 */
#define CD_DEVICE_PROPERTY_FORMAT		"Format"		/* Since: 0.1.9 */
#define CD_DEVICE_PROPERTY_MODE			"Mode"			/* Since: 0.1.8 */
#define CD_DEVICE_PROPERTY_PROFILES		"Profiles"		/* Since: 0.1.8 */
#define CD_DEVICE_PROPERTY_CREATED		"Created"		/* Since: 0.1.8 */
#define CD_DEVICE_PROPERTY_MODIFIED		"Modified"		/* Since: 0.1.8 */
#define CD_DEVICE_PROPERTY_METADATA		"Metadata"		/* Since: 0.1.8 */
#define CD_DEVICE_PROPERTY_ID			"DeviceId"		/* Since: 0.1.8 */
#define CD_DEVICE_PROPERTY_SCOPE		"Scope"			/* Since: 0.1.9 */
#define CD_DEVICE_PROPERTY_OWNER		"Owner"			/* Since: 0.1.13 */
#define CD_DEVICE_PROPERTY_SEAT			"Seat"			/* Since: 0.1.24 */
#define CD_DEVICE_PROPERTY_PROFILING_INHIBITORS	"ProfilingInhibitors"	/* Since: 0.1.18 */
#define CD_DEVICE_PROPERTY_ENABLED		"Enabled"		/* Since: 0.1.26 */
#define CD_DEVICE_PROPERTY_EMBEDDED		"Embedded"		/* Since: 0.1.27 */

/* defined in org.freedesktop.ColorManager.Sensor.xml */
#define CD_SENSOR_PROPERTY_ID			"SensorId"		/* Since: 0.1.26 */
#define CD_SENSOR_PROPERTY_KIND			"Kind"			/* Since: 0.1.26 */
#define CD_SENSOR_PROPERTY_STATE		"State"			/* Since: 0.1.26 */
#define CD_SENSOR_PROPERTY_MODE			"Mode"			/* Since: 0.1.26 */
#define CD_SENSOR_PROPERTY_SERIAL		"Serial"		/* Since: 0.1.26 */
#define CD_SENSOR_PROPERTY_MODEL		"Model"			/* Since: 0.1.26 */
#define CD_SENSOR_PROPERTY_VENDOR		"Vendor"		/* Since: 0.1.26 */
#define CD_SENSOR_PROPERTY_NATIVE		"Native"		/* Since: 0.1.26 */
#define CD_SENSOR_PROPERTY_LOCKED		"Locked"		/* Since: 0.1.26 */
#define CD_SENSOR_PROPERTY_CAPABILITIES		"Capabilities"		/* Since: 0.1.26 */
#define CD_SENSOR_PROPERTY_OPTIONS		"Options"		/* Since: 0.1.26 */
#define CD_SENSOR_PROPERTY_EMBEDDED		"Embedded"		/* Since: 0.1.26 */
#define CD_SENSOR_PROPERTY_METADATA		"Metadata"		/* Since: 0.1.28 */

/* defined in metadata-spec.txt */
#define CD_SENSOR_METADATA_IMAGE_ATTACH		"ImageAttach"		/* Since: 0.1.28 */
#define CD_SENSOR_METADATA_IMAGE_CALIBRATE	"ImageCalibrate"	/* Since: 0.1.28 */
#define CD_SENSOR_METADATA_IMAGE_SCREEN		"ImageScreen"		/* Since: 0.1.28 */

/* convenience functions as it's easy to forget the bitwise operators */
#define cd_bitfield_add(bitfield,tmp)		do { ((bitfield) |= (cd_bitfield_value(tmp))); } while (0)
#define cd_bitfield_remove(bitfield,tmp)	do { ((bitfield) &= ~(cd_bitfield_value(tmp))); } while (0)
#define cd_bitfield_contain(bitfield,tmp)	(((bitfield) & (cd_bitfield_value(tmp))) > 0)
#define cd_bitfield_value(tmp)			((guint64) 1 << (tmp))

guint64		 cd_bitfield_from_enums			(gint			 value, ...);
const gchar	*cd_device_kind_to_string		(CdDeviceKind		 kind_enum);
CdDeviceKind	 cd_device_kind_from_string		(const gchar		*kind);
const gchar	*cd_profile_kind_to_string		(CdProfileKind		 profile_kind);
CdProfileKind	 cd_profile_kind_from_string		(const gchar		*profile_kind);
CdRenderingIntent cd_rendering_intent_from_string	(const gchar		*rendering_intent);
const gchar	*cd_rendering_intent_to_string		(CdRenderingIntent	 rendering_intent);
CdPixelFormat	 cd_pixel_format_from_string		(const gchar		*pixel_format);
const gchar	*cd_pixel_format_to_string		(CdPixelFormat		 pixel_format);
const gchar	*cd_colorspace_to_string		(CdColorspace		 colorspace);
CdColorspace	 cd_colorspace_from_string		(const gchar		*colorspace);
const gchar	*cd_device_mode_to_string		(CdDeviceMode		 device_mode);
CdDeviceMode	 cd_device_mode_from_string		(const gchar		*device_mode);
const gchar	*cd_device_relation_to_string		(CdDeviceRelation	 device_relation);
CdDeviceRelation cd_device_relation_from_string		(const gchar		*device_relation);
const gchar	*cd_object_scope_to_string		(CdObjectScope		 object_scope);
CdObjectScope	 cd_object_scope_from_string		(const gchar		*object_scope);
const gchar	*cd_sensor_kind_to_string		(CdSensorKind		 sensor_kind);
CdSensorKind	 cd_sensor_kind_from_string		(const gchar		*sensor_kind);
const gchar	*cd_sensor_state_to_string		(CdSensorState		 sensor_state);
CdSensorState	 cd_sensor_state_from_string		(const gchar		*sensor_state);
const gchar	*cd_sensor_cap_to_string		(CdSensorCap		 sensor_cap);
CdSensorCap	 cd_sensor_cap_from_string		(const gchar		*sensor_cap);
const gchar	*cd_standard_space_to_string		(CdStandardSpace	 standard_space);
CdStandardSpace	 cd_standard_space_from_string		(const gchar		*standard_space);
const gchar	*cd_profile_warning_to_string		(CdProfileWarning	 kind_enum);
CdProfileWarning cd_profile_warning_from_string		(const gchar		*type);
const gchar	*cd_profile_quality_to_string		(CdProfileQuality	 quality_enum);
CdProfileQuality cd_profile_quality_from_string		(const gchar		*quality);
CdProfileKind	 cd_device_kind_to_profile_kind		(CdDeviceKind		 device_kind);

const gchar	*cd_sensor_error_to_string		(CdSensorError		 error_enum);
CdSensorError	 cd_sensor_error_from_string		(const gchar		*error_desc);
const gchar	*cd_profile_error_to_string		(CdProfileError		 error_enum);
CdProfileError	 cd_profile_error_from_string		(const gchar		*error_desc);
const gchar	*cd_device_error_to_string		(CdDeviceError		 error_enum);
CdDeviceError	 cd_device_error_from_string		(const gchar		*error_desc);
const gchar	*cd_client_error_to_string		(CdClientError		 error_enum);
CdClientError	 cd_client_error_from_string		(const gchar		*error_desc);

G_END_DECLS

#endif /* __CD_ENUM_H */

