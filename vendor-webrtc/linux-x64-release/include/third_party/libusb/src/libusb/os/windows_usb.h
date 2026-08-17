/*
 * Windows backend for libusbx 1.0
 * Copyright © 2009-2012 Pete Batard <pete@akeo.ie>
 * With contributions from Michael Plante, Orin Eman et al.
 * Parts of this code adapted from libusb-win32-v1 by Stephan Meyer
 * Major code testing contribution by Xiaofan Chen
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
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

#pragma once

#include "windows_common.h"

#if defined(_MSC_VER)
// disable /W4 MSVC warnings that are benign
#pragma warning(disable:4127) // conditional expression is constant
#pragma warning(disable:4100) // unreferenced formal parameter
#pragma warning(disable:4214) // bit field types other than int
#pragma warning(disable:4201) // nameless struct/union
#endif

// Missing from MSVC6 setupapi.h
#if !defined(SPDRP_ADDRESS)
#define SPDRP_ADDRESS	28
#endif
#if !defined(SPDRP_INSTALL_STATE)
#define SPDRP_INSTALL_STATE	34
#endif

#if defined(__CYGWIN__ )
#define _stricmp stricmp
// cygwin produces a warning unless these prototypes are defined
extern int _snprintf(char *buffer, size_t count, const char *format, ...);
extern char *_strdup(const char *strSource);
// _beginthreadex is MSVCRT => unavailable for cygwin. Fallback to using CreateThread
#define _beginthreadex(a, b, c, d, e, f) CreateThread(a, b, (LPTHREAD_START_ROUTINE)c, d, e, f)
#endif

#define MAX_CTRL_BUFFER_LENGTH      4096
#define MAX_USB_DEVICES             256
#define MAX_USB_STRING_LENGTH       128
#define MAX_GUID_STRING_LENGTH      40
#define MAX_PATH_LENGTH             128
#define MAX_KEY_LENGTH              256
#define LIST_SEPARATOR              ';'
#define HTAB_SIZE                   1021

#if !defined(GUID_DEVINTERFACE_LIBUSB0_FILTER)
const GUID GUID_DEVINTERFACE_LIBUSB0_FILTER = { 0xF9F3FF14, 0xAE21, 0x48A0, {0x8A, 0x25, 0x80, 0x11, 0xA7, 0xA9, 0x31, 0xD9} };
#endif


/*
 * Multiple USB API backend support
 */
#define USB_API_UNSUPPORTED 0
#define USB_API_HUB         1
#define USB_API_COMPOSITE   2
#define USB_API_WINUSBX     3
#define USB_API_MAX         4
// The following is used to indicate if the composite extra props have already been set.
#define USB_API_SET         (1<<USB_API_MAX) 

// Sub-APIs for WinUSB-like driver APIs (WinUSB, libusbK, libusb-win32 through the libusbK DLL)
// Must have the same values as the KUSB_DRVID enum from libusbk.h
#define SUB_API_NOTSET      -1
#define SUB_API_LIBUSBK     0
#define SUB_API_LIBUSB0     1
#define SUB_API_WINUSB      2
#define SUB_API_MAX         3

#define WINUSBX_DRV_NAMES   { "libusbK", "libusb0", "WinUSB"}

struct windows_usb_api_backend {
	const uint8_t id;
	const char* designation;
	const char **driver_name_list; // Driver name, without .sys, e.g. "usbccgp"
	const uint8_t nb_driver_names;
	int (*init)(int sub_api, struct libusb_context *ctx);
	int (*exit)(int sub_api);
	int (*open)(int sub_api, struct libusb_device_handle *dev_handle);
	void (*close)(int sub_api, struct libusb_device_handle *dev_handle);
	int (*configure_endpoints)(int sub_api, struct libusb_device_handle *dev_handle, int iface);
	int (*claim_interface)(int sub_api, struct libusb_device_handle *dev_handle, int iface);
	int (*set_interface_altsetting)(int sub_api, struct libusb_device_handle *dev_handle, int iface, int altsetting);
	int (*release_interface)(int sub_api, struct libusb_device_handle *dev_handle, int iface);
	int (*clear_halt)(int sub_api, struct libusb_device_handle *dev_handle, unsigned char endpoint);
	int (*reset_device)(int sub_api, struct libusb_device_handle *dev_handle);
	int (*submit_bulk_transfer)(int sub_api, struct usbi_transfer *itransfer);
	int (*submit_iso_transfer)(int sub_api, struct usbi_transfer *itransfer);
	int (*submit_control_transfer)(int sub_api, struct usbi_transfer *itransfer);
	int (*abort_control)(int sub_api, struct usbi_transfer *itransfer);
	int (*abort_transfers)(int sub_api, struct usbi_transfer *itransfer);
	int (*copy_transfer_data)(int sub_api, struct usbi_transfer *itransfer, uint32_t io_size);
};

extern const struct windows_usb_api_backend usb_api_backend[USB_API_MAX];

#define PRINT_UNSUPPORTED_API(fname)              \
	usbi_dbg("unsupported API call for '"         \
		#fname "' (unrecognized device driver)"); \
	return LIBUSB_ERROR_NOT_SUPPORTED;

/*
 * private structures definition
 * with inline pseudo constructors/destructors
 */

#define LIBUSB_REQ_RECIPIENT(request_type) ((request_type) & 0x1F)
#define LIBUSB_REQ_TYPE(request_type) ((request_type) & (0x03 << 5))
#define LIBUSB_REQ_IN(request_type) ((request_type) & LIBUSB_ENDPOINT_IN)
#define LIBUSB_REQ_OUT(request_type) (!LIBUSB_REQ_IN(request_type))

struct windows_device_priv {
	uint8_t depth;						// distance to HCD
	uint8_t port;						// port number on the hub
	uint8_t active_config;
	struct libusb_device *parent_dev;	// access to parent is required for usermode ops
	struct windows_usb_api_backend const *apib;
	char *path;							// device interface path
	int sub_api;						// for WinUSB-like APIs
	struct {
		char *path;						// each interface needs a device interface path,
		struct windows_usb_api_backend const *apib; // an API backend (multiple drivers support),
		int sub_api;
		int8_t nb_endpoints;			// and a set of endpoint addresses (USB_MAXENDPOINTS)
		uint8_t *endpoint;
		bool restricted_functionality;	// indicates if the interface functionality is restricted
										// by Windows (eg. HID keyboards or mice cannot do R/W)
	} usb_interface[USB_MAXINTERFACES];
	USB_DEVICE_DESCRIPTOR dev_descriptor;
	unsigned char **config_descriptor;	// list of pointers to the cached config descriptors
};

static inline struct windows_device_priv *_device_priv(struct libusb_device *dev) {
	return (struct windows_device_priv *)dev->os_priv;
}

static inline void windows_device_priv_init(libusb_device* dev) {
	struct windows_device_priv* p = _device_priv(dev);
	int i;
	p->depth = 0;
	p->port = 0;
	p->parent_dev = NULL;
	p->path = NULL;
	p->apib = &usb_api_backend[USB_API_UNSUPPORTED];
	p->sub_api = SUB_API_NOTSET;
	p->active_config = 0;
	p->config_descriptor = NULL;
	memset(&(p->dev_descriptor), 0, sizeof(USB_DEVICE_DESCRIPTOR));
	for (i=0; i<USB_MAXINTERFACES; i++) {
		p->usb_interface[i].path = NULL;
		p->usb_interface[i].apib = &usb_api_backend[USB_API_UNSUPPORTED];
		p->usb_interface[i].sub_api = SUB_API_NOTSET;
		p->usb_interface[i].nb_endpoints = 0;
		p->usb_interface[i].endpoint = NULL;
		p->usb_interface[i].restricted_functionality = false;
	}
}

static inline void windows_device_priv_release(libusb_device* dev) {
	struct windows_device_priv* p = _device_priv(dev);
	int i;
	safe_free(p->path);
	if ((dev->num_configurations > 0) && (p->config_descriptor != NULL)) {
		for (i=0; i < dev->num_configurations; i++)
			safe_free(p->config_descriptor[i]);
	}
	safe_free(p->config_descriptor);
	for (i=0; i<USB_MAXINTERFACES; i++) {
		safe_free(p->usb_interface[i].path);
		safe_free(p->usb_interface[i].endpoint);
	}
}

struct interface_handle_t {
	HANDLE dev_handle; // WinUSB needs an extra handle for the file
	HANDLE api_handle; // used by the API to communicate with the device
};

struct windows_device_handle_priv {
	int active_interface;
	struct interface_handle_t interface_handle[USB_MAXINTERFACES];
	int autoclaim_count[USB_MAXINTERFACES]; // For auto-release
};

static inline struct windows_device_handle_priv *_device_handle_priv(
	struct libusb_device_handle *handle)
{
	return (struct windows_device_handle_priv *) handle->os_priv;
}

// used for async polling functions
struct windows_transfer_priv {
	struct winfd pollable_fd;
	uint8_t interface_number;
};

// used to match a device driver (including filter drivers) against a supported API
struct driver_lookup {
	char list[MAX_KEY_LENGTH+1];// REG_MULTI_SZ list of services (driver) names
	const DWORD reg_prop;		// SPDRP registry key to use to retreive list
	const char* designation;	// internal designation (for debug output)
};

/*
 * Windows DDK API definitions. Most of it copied from MinGW's includes
 */
typedef DWORD DEVNODE, DEVINST;
typedef DEVNODE *PDEVNODE, *PDEVINST;
typedef DWORD RETURN_TYPE;
typedef RETURN_TYPE CONFIGRET;

#define USB_GET_NODE_INFORMATION                258
#define USB_GET_DESCRIPTOR_FROM_NODE_CONNECTION 260
#define USB_GET_NODE_CONNECTION_NAME            261
#define USB_GET_HUB_CAPABILITIES                271
#if !defined(USB_GET_NODE_CONNECTION_INFORMATION_EX)
#define USB_GET_NODE_CONNECTION_INFORMATION_EX  274
#endif
#if !defined(USB_GET_HUB_CAPABILITIES_EX)
#define USB_GET_HUB_CAPABILITIES_EX             276
#endif

#ifndef METHOD_BUFFERED
#define METHOD_BUFFERED                         0
#endif
#ifndef FILE_ANY_ACCESS
#define FILE_ANY_ACCESS                         0x00000000
#endif
#ifndef FILE_DEVICE_UNKNOWN
#define FILE_DEVICE_UNKNOWN                     0x00000022
#endif
#ifndef FILE_DEVICE_USB
#define FILE_DEVICE_USB                         FILE_DEVICE_UNKNOWN
#endif

#ifndef CTL_CODE
#define CTL_CODE(DeviceType, Function, Method, Access)( \
  ((DeviceType) << 16) | ((Access) << 14) | ((Function) << 2) | (Method))
#endif

#define IOCTL_USB_GET_HUB_CAPABILITIES \
  CTL_CODE(FILE_DEVICE_USB, USB_GET_HUB_CAPABILITIES, METHOD_BUFFERED, FILE_ANY_ACCESS)

#define IOCTL_USB_GET_DESCRIPTOR_FROM_NODE_CONNECTION \
  CTL_CODE(FILE_DEVICE_USB, USB_GET_DESCRIPTOR_FROM_NODE_CONNECTION, METHOD_BUFFERED, FILE_ANY_ACCESS)

#define IOCTL_USB_GET_ROOT_HUB_NAME \
  CTL_CODE(FILE_DEVICE_USB, HCD_GET_ROOT_HUB_NAME, METHOD_BUFFERED, FILE_ANY_ACCESS)

#define IOCTL_USB_GET_NODE_INFORMATION \
  CTL_CODE(FILE_DEVICE_USB, USB_GET_NODE_INFORMATION, METHOD_BUFFERED, FILE_ANY_ACCESS)

#define IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX \
  CTL_CODE(FILE_DEVICE_USB, USB_GET_NODE_CONNECTION_INFORMATION_EX, METHOD_BUFFERED, FILE_ANY_ACCESS)

#define IOCTL_USB_GET_NODE_CONNECTION_ATTRIBUTES \
  CTL_CODE(FILE_DEVICE_USB, USB_GET_NODE_CONNECTION_ATTRIBUTES, METHOD_BUFFERED, FILE_ANY_ACCESS)

#define IOCTL_USB_GET_NODE_CONNECTION_NAME \
  CTL_CODE(FILE_DEVICE_USB, USB_GET_NODE_CONNECTION_NAME, METHOD_BUFFERED, FILE_ANY_ACCESS)

// Most of the structures below need to be packed
#pragma pack(push, 1)

typedef struct USB_CONFIGURATION_DESCRIPTOR_SHORT {
  struct {
    ULONG ConnectionIndex;
    struct {
      UCHAR bmRequest;
      UCHAR bRequest;
      USHORT wValue;
      USHORT wIndex;
      USHORT wLength;
    } SetupPacket;
  } req;
  USB_CONFIGURATION_DESCRIPTOR data;
} USB_CONFIGURATION_DESCRIPTOR_SHORT;

typedef struct USB_ROOT_HUB_NAME_FIXED {
	ULONG ActualLength;
	WCHAR RootHubName[MAX_PATH_LENGTH];
} USB_ROOT_HUB_NAME_FIXED;

#pragma pack(pop)
