/*
 *
 *  BlueZ - Bluetooth protocol stack for Linux
 *
 *  Copyright (C) 2001-2002  Nokia Corporation
 *  Copyright (C) 2002-2003  Maxim Krasnyansky <maxk@qualcomm.com>
 *  Copyright (C) 2002-2010  Marcel Holtmann <marcel@holtmann.org>
 *  Copyright (C) 2002-2003  Stephen Crane <steve.crane@rococosoft.com>
 *
 *
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program; if not, write to the Free Software
 *  Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
 *
 */

#ifndef __SDP_H
#define __SDP_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <bluetooth/bluetooth.h>

#define SDP_UNIX_PATH "/var/run/sdp"
#define SDP_RESPONSE_TIMEOUT	20
#define SDP_REQ_BUFFER_SIZE	2048
#define SDP_RSP_BUFFER_SIZE	65535
#define SDP_PDU_CHUNK_SIZE	1024

/*
 * All definitions are based on Bluetooth Assigned Numbers
 * of the Bluetooth Specification
 */
#define SDP_PSM		0x0001

/*
 * Protocol UUIDs
 */
#define SDP_UUID	0x0001
#define UDP_UUID	0x0002
#define RFCOMM_UUID	0x0003
#define TCP_UUID	0x0004
#define TCS_BIN_UUID	0x0005
#define TCS_AT_UUID	0x0006
#define ATT_UUID	0x0007
#define OBEX_UUID	0x0008
#define IP_UUID		0x0009
#define FTP_UUID	0x000a
#define HTTP_UUID	0x000c
#define WSP_UUID	0x000e
#define BNEP_UUID	0x000f
#define UPNP_UUID	0x0010
#define HIDP_UUID	0x0011
#define HCRP_CTRL_UUID	0x0012
#define HCRP_DATA_UUID	0x0014
#define HCRP_NOTE_UUID	0x0016
#define AVCTP_UUID	0x0017
#define AVDTP_UUID	0x0019
#define CMTP_UUID	0x001b
#define UDI_UUID	0x001d
#define MCAP_CTRL_UUID	0x001e
#define MCAP_DATA_UUID	0x001f
#define L2CAP_UUID	0x0100

/*
 * Service class identifiers of standard services and service groups
 */
#define SDP_SERVER_SVCLASS_ID		0x1000
#define BROWSE_GRP_DESC_SVCLASS_ID	0x1001
#define PUBLIC_BROWSE_GROUP		0x1002
#define SERIAL_PORT_SVCLASS_ID		0x1101
#define LAN_ACCESS_SVCLASS_ID		0x1102
#define DIALUP_NET_SVCLASS_ID		0x1103
#define IRMC_SYNC_SVCLASS_ID		0x1104
#define OBEX_OBJPUSH_SVCLASS_ID		0x1105
#define OBEX_FILETRANS_SVCLASS_ID	0x1106
#define IRMC_SYNC_CMD_SVCLASS_ID	0x1107
#define HEADSET_SVCLASS_ID		0x1108
#define CORDLESS_TELEPHONY_SVCLASS_ID	0x1109
#define AUDIO_SOURCE_SVCLASS_ID		0x110a
#define AUDIO_SINK_SVCLASS_ID		0x110b
#define AV_REMOTE_TARGET_SVCLASS_ID	0x110c
#define ADVANCED_AUDIO_SVCLASS_ID	0x110d
#define AV_REMOTE_SVCLASS_ID		0x110e
#define AV_REMOTE_CONTROLLER_SVCLASS_ID	0x110f
#define INTERCOM_SVCLASS_ID		0x1110
#define FAX_SVCLASS_ID			0x1111
#define HEADSET_AGW_SVCLASS_ID		0x1112
#define WAP_SVCLASS_ID			0x1113
#define WAP_CLIENT_SVCLASS_ID		0x1114
#define PANU_SVCLASS_ID			0x1115
#define NAP_SVCLASS_ID			0x1116
#define GN_SVCLASS_ID			0x1117
#define DIRECT_PRINTING_SVCLASS_ID	0x1118
#define REFERENCE_PRINTING_SVCLASS_ID	0x1119
#define IMAGING_SVCLASS_ID		0x111a
#define IMAGING_RESPONDER_SVCLASS_ID	0x111b
#define IMAGING_ARCHIVE_SVCLASS_ID	0x111c
#define IMAGING_REFOBJS_SVCLASS_ID	0x111d
#define HANDSFREE_SVCLASS_ID		0x111e
#define HANDSFREE_AGW_SVCLASS_ID	0x111f
#define DIRECT_PRT_REFOBJS_SVCLASS_ID	0x1120
#define REFLECTED_UI_SVCLASS_ID		0x1121
#define BASIC_PRINTING_SVCLASS_ID	0x1122
#define PRINTING_STATUS_SVCLASS_ID	0x1123
#define HID_SVCLASS_ID			0x1124
#define HCR_SVCLASS_ID			0x1125
#define HCR_PRINT_SVCLASS_ID		0x1126
#define HCR_SCAN_SVCLASS_ID		0x1127
#define CIP_SVCLASS_ID			0x1128
#define VIDEO_CONF_GW_SVCLASS_ID	0x1129
#define UDI_MT_SVCLASS_ID		0x112a
#define UDI_TA_SVCLASS_ID		0x112b
#define AV_SVCLASS_ID			0x112c
#define SAP_SVCLASS_ID			0x112d
#define PBAP_PCE_SVCLASS_ID		0x112e
#define PBAP_PSE_SVCLASS_ID		0x112f
#define PBAP_SVCLASS_ID			0x1130
#define MAP_MSE_SVCLASS_ID		0x1132
#define MAP_MCE_SVCLASS_ID		0x1133
#define MAP_SVCLASS_ID			0x1134
#define GNSS_SVCLASS_ID			0x1135
#define GNSS_SERVER_SVCLASS_ID		0x1136
#define MPS_SC_SVCLASS_ID		0x113A
#define MPS_SVCLASS_ID			0x113B
#define PNP_INFO_SVCLASS_ID		0x1200
#define GENERIC_NETWORKING_SVCLASS_ID	0x1201
#define GENERIC_FILETRANS_SVCLASS_ID	0x1202
#define GENERIC_AUDIO_SVCLASS_ID	0x1203
#define GENERIC_TELEPHONY_SVCLASS_ID	0x1204
#define UPNP_SVCLASS_ID			0x1205
#define UPNP_IP_SVCLASS_ID		0x1206
#define UPNP_PAN_SVCLASS_ID		0x1300
#define UPNP_LAP_SVCLASS_ID		0x1301
#define UPNP_L2CAP_SVCLASS_ID		0x1302
#define VIDEO_SOURCE_SVCLASS_ID		0x1303
#define VIDEO_SINK_SVCLASS_ID		0x1304
#define VIDEO_DISTRIBUTION_SVCLASS_ID	0x1305
#define HDP_SVCLASS_ID			0x1400
#define HDP_SOURCE_SVCLASS_ID		0x1401
#define HDP_SINK_SVCLASS_ID		0x1402
#define GENERIC_ACCESS_SVCLASS_ID	0x1800
#define GENERIC_ATTRIB_SVCLASS_ID	0x1801
#define APPLE_AGENT_SVCLASS_ID		0x2112

/*
 * Standard profile descriptor identifiers; note these
 * may be identical to some of the service classes defined above
 */
#define SDP_SERVER_PROFILE_ID		SDP_SERVER_SVCLASS_ID
#define BROWSE_GRP_DESC_PROFILE_ID	BROWSE_GRP_DESC_SVCLASS_ID
#define SERIAL_PORT_PROFILE_ID		SERIAL_PORT_SVCLASS_ID
#define LAN_ACCESS_PROFILE_ID		LAN_ACCESS_SVCLASS_ID
#define DIALUP_NET_PROFILE_ID		DIALUP_NET_SVCLASS_ID
#define IRMC_SYNC_PROFILE_ID		IRMC_SYNC_SVCLASS_ID
#define OBEX_OBJPUSH_PROFILE_ID		OBEX_OBJPUSH_SVCLASS_ID
#define OBEX_FILETRANS_PROFILE_ID	OBEX_FILETRANS_SVCLASS_ID
#define IRMC_SYNC_CMD_PROFILE_ID	IRMC_SYNC_CMD_SVCLASS_ID
#define HEADSET_PROFILE_ID		HEADSET_SVCLASS_ID
#define CORDLESS_TELEPHONY_PROFILE_ID	CORDLESS_TELEPHONY_SVCLASS_ID
#define AUDIO_SOURCE_PROFILE_ID		AUDIO_SOURCE_SVCLASS_ID
#define AUDIO_SINK_PROFILE_ID		AUDIO_SINK_SVCLASS_ID
#define AV_REMOTE_TARGET_PROFILE_ID	AV_REMOTE_TARGET_SVCLASS_ID
#define ADVANCED_AUDIO_PROFILE_ID	ADVANCED_AUDIO_SVCLASS_ID
#define AV_REMOTE_PROFILE_ID		AV_REMOTE_SVCLASS_ID
#define INTERCOM_PROFILE_ID		INTERCOM_SVCLASS_ID
#define FAX_PROFILE_ID			FAX_SVCLASS_ID
#define HEADSET_AGW_PROFILE_ID		HEADSET_AGW_SVCLASS_ID
#define WAP_PROFILE_ID			WAP_SVCLASS_ID
#define WAP_CLIENT_PROFILE_ID		WAP_CLIENT_SVCLASS_ID
#define PANU_PROFILE_ID			PANU_SVCLASS_ID
#define NAP_PROFILE_ID			NAP_SVCLASS_ID
#define GN_PROFILE_ID			GN_SVCLASS_ID
#define DIRECT_PRINTING_PROFILE_ID	DIRECT_PRINTING_SVCLASS_ID
#define REFERENCE_PRINTING_PROFILE_ID	REFERENCE_PRINTING_SVCLASS_ID
#define IMAGING_PROFILE_ID		IMAGING_SVCLASS_ID
#define IMAGING_RESPONDER_PROFILE_ID	IMAGING_RESPONDER_SVCLASS_ID
#define IMAGING_ARCHIVE_PROFILE_ID	IMAGING_ARCHIVE_SVCLASS_ID
#define IMAGING_REFOBJS_PROFILE_ID	IMAGING_REFOBJS_SVCLASS_ID
#define HANDSFREE_PROFILE_ID		HANDSFREE_SVCLASS_ID
#define HANDSFREE_AGW_PROFILE_ID	HANDSFREE_AGW_SVCLASS_ID
#define DIRECT_PRT_REFOBJS_PROFILE_ID	DIRECT_PRT_REFOBJS_SVCLASS_ID
#define REFLECTED_UI_PROFILE_ID		REFLECTED_UI_SVCLASS_ID
#define BASIC_PRINTING_PROFILE_ID	BASIC_PRINTING_SVCLASS_ID
#define PRINTING_STATUS_PROFILE_ID	PRINTING_STATUS_SVCLASS_ID
#define HID_PROFILE_ID			HID_SVCLASS_ID
#define HCR_PROFILE_ID			HCR_SCAN_SVCLASS_ID
#define HCR_PRINT_PROFILE_ID		HCR_PRINT_SVCLASS_ID
#define HCR_SCAN_PROFILE_ID		HCR_SCAN_SVCLASS_ID
#define CIP_PROFILE_ID			CIP_SVCLASS_ID
#define VIDEO_CONF_GW_PROFILE_ID	VIDEO_CONF_GW_SVCLASS_ID
#define UDI_MT_PROFILE_ID		UDI_MT_SVCLASS_ID
#define UDI_TA_PROFILE_ID		UDI_TA_SVCLASS_ID
#define AV_PROFILE_ID			AV_SVCLASS_ID
#define SAP_PROFILE_ID			SAP_SVCLASS_ID
#define PBAP_PCE_PROFILE_ID		PBAP_PCE_SVCLASS_ID
#define PBAP_PSE_PROFILE_ID		PBAP_PSE_SVCLASS_ID
#define PBAP_PROFILE_ID			PBAP_SVCLASS_ID
#define MAP_PROFILE_ID			MAP_SVCLASS_ID
#define PNP_INFO_PROFILE_ID		PNP_INFO_SVCLASS_ID
#define GENERIC_NETWORKING_PROFILE_ID	GENERIC_NETWORKING_SVCLASS_ID
#define GENERIC_FILETRANS_PROFILE_ID	GENERIC_FILETRANS_SVCLASS_ID
#define GENERIC_AUDIO_PROFILE_ID	GENERIC_AUDIO_SVCLASS_ID
#define GENERIC_TELEPHONY_PROFILE_ID	GENERIC_TELEPHONY_SVCLASS_ID
#define UPNP_PROFILE_ID			UPNP_SVCLASS_ID
#define UPNP_IP_PROFILE_ID		UPNP_IP_SVCLASS_ID
#define UPNP_PAN_PROFILE_ID		UPNP_PAN_SVCLASS_ID
#define UPNP_LAP_PROFILE_ID		UPNP_LAP_SVCLASS_ID
#define UPNP_L2CAP_PROFILE_ID		UPNP_L2CAP_SVCLASS_ID
#define VIDEO_SOURCE_PROFILE_ID		VIDEO_SOURCE_SVCLASS_ID
#define VIDEO_SINK_PROFILE_ID		VIDEO_SINK_SVCLASS_ID
#define VIDEO_DISTRIBUTION_PROFILE_ID	VIDEO_DISTRIBUTION_SVCLASS_ID
#define HDP_PROFILE_ID			HDP_SVCLASS_ID
#define HDP_SOURCE_PROFILE_ID		HDP_SOURCE_SVCLASS_ID
#define HDP_SINK_PROFILE_ID		HDP_SINK_SVCLASS_ID
#define GENERIC_ACCESS_PROFILE_ID	GENERIC_ACCESS_SVCLASS_ID
#define GENERIC_ATTRIB_PROFILE_ID	GENERIC_ATTRIB_SVCLASS_ID
#define APPLE_AGENT_PROFILE_ID		APPLE_AGENT_SVCLASS_ID
#define MPS_PROFILE_ID			MPS_SC_SVCLASS_ID

/*
 * Compatibility macros for the old MDP acronym
 */
#define MDP_SVCLASS_ID			HDP_SVCLASS_ID
#define MDP_SOURCE_SVCLASS_ID		HDP_SOURCE_SVCLASS_ID
#define MDP_SINK_SVCLASS_ID		HDP_SINK_SVCLASS_ID
#define MDP_PROFILE_ID			HDP_PROFILE_ID
#define MDP_SOURCE_PROFILE_ID		HDP_SOURCE_PROFILE_ID
#define MDP_SINK_PROFILE_ID		HDP_SINK_PROFILE_ID

/*
 * Attribute identifier codes
 */
#define SDP_SERVER_RECORD_HANDLE		0x0000

/*
 * Possible values for attribute-id are listed below.
 * See SDP Spec, section "Service Attribute Definitions" for more details.
 */
#define SDP_ATTR_RECORD_HANDLE			0x0000
#define SDP_ATTR_SVCLASS_ID_LIST		0x0001
#define SDP_ATTR_RECORD_STATE			0x0002
#define SDP_ATTR_SERVICE_ID			0x0003
#define SDP_ATTR_PROTO_DESC_LIST		0x0004
#define SDP_ATTR_BROWSE_GRP_LIST		0x0005
#define SDP_ATTR_LANG_BASE_ATTR_ID_LIST		0x0006
#define SDP_ATTR_SVCINFO_TTL			0x0007
#define SDP_ATTR_SERVICE_AVAILABILITY		0x0008
#define SDP_ATTR_PFILE_DESC_LIST		0x0009
#define SDP_ATTR_DOC_URL			0x000a
#define SDP_ATTR_CLNT_EXEC_URL			0x000b
#define SDP_ATTR_ICON_URL			0x000c
#define SDP_ATTR_ADD_PROTO_DESC_LIST		0x000d

#define SDP_ATTR_GROUP_ID			0x0200
#define SDP_ATTR_IP_SUBNET			0x0200
#define SDP_ATTR_VERSION_NUM_LIST		0x0200
#define SDP_ATTR_SUPPORTED_FEATURES_LIST	0x0200
#define SDP_ATTR_GOEP_L2CAP_PSM			0x0200
#define SDP_ATTR_SVCDB_STATE			0x0201

#define SDP_ATTR_MPSD_SCENARIOS			0x0200
#define SDP_ATTR_MPMD_SCENARIOS			0x0201
#define SDP_ATTR_MPS_DEPENDENCIES		0x0202

#define SDP_ATTR_SERVICE_VERSION		0x0300
#define SDP_ATTR_EXTERNAL_NETWORK		0x0301
#define SDP_ATTR_SUPPORTED_DATA_STORES_LIST	0x0301
#define SDP_ATTR_DATA_EXCHANGE_SPEC		0x0301
#define SDP_ATTR_NETWORK			0x0301
#define SDP_ATTR_FAX_CLASS1_SUPPORT		0x0302
#define SDP_ATTR_REMOTE_AUDIO_VOLUME_CONTROL	0x0302
#define SDP_ATTR_MCAP_SUPPORTED_PROCEDURES	0x0302
#define SDP_ATTR_FAX_CLASS20_SUPPORT		0x0303
#define SDP_ATTR_SUPPORTED_FORMATS_LIST		0x0303
#define SDP_ATTR_FAX_CLASS2_SUPPORT		0x0304
#define SDP_ATTR_AUDIO_FEEDBACK_SUPPORT		0x0305
#define SDP_ATTR_NETWORK_ADDRESS		0x0306
#define SDP_ATTR_WAP_GATEWAY			0x0307
#define SDP_ATTR_HOMEPAGE_URL			0x0308
#define SDP_ATTR_WAP_STACK_TYPE			0x0309
#define SDP_ATTR_SECURITY_DESC			0x030a
#define SDP_ATTR_NET_ACCESS_TYPE		0x030b
#define SDP_ATTR_MAX_NET_ACCESSRATE		0x030c
#define SDP_ATTR_IP4_SUBNET			0x030d
#define SDP_ATTR_IP6_SUBNET			0x030e
#define SDP_ATTR_SUPPORTED_CAPABILITIES		0x0310
#define SDP_ATTR_SUPPORTED_FEATURES		0x0311
#define SDP_ATTR_SUPPORTED_FUNCTIONS		0x0312
#define SDP_ATTR_TOTAL_IMAGING_DATA_CAPACITY	0x0313
#define SDP_ATTR_SUPPORTED_REPOSITORIES		0x0314
#define SDP_ATTR_MAS_INSTANCE_ID		0x0315
#define SDP_ATTR_SUPPORTED_MESSAGE_TYPES	0x0316
#define SDP_ATTR_PBAP_SUPPORTED_FEATURES	0x0317
#define SDP_ATTR_MAP_SUPPORTED_FEATURES		0x0317

#define SDP_ATTR_SPECIFICATION_ID		0x0200
#define SDP_ATTR_VENDOR_ID			0x0201
#define SDP_ATTR_PRODUCT_ID			0x0202
#define SDP_ATTR_VERSION			0x0203
#define SDP_ATTR_PRIMARY_RECORD			0x0204
#define SDP_ATTR_VENDOR_ID_SOURCE		0x0205

#define SDP_ATTR_HID_DEVICE_RELEASE_NUMBER	0x0200
#define SDP_ATTR_HID_PARSER_VERSION		0x0201
#define SDP_ATTR_HID_DEVICE_SUBCLASS		0x0202
#define SDP_ATTR_HID_COUNTRY_CODE		0x0203
#define SDP_ATTR_HID_VIRTUAL_CABLE		0x0204
#define SDP_ATTR_HID_RECONNECT_INITIATE		0x0205
#define SDP_ATTR_HID_DESCRIPTOR_LIST		0x0206
#define SDP_ATTR_HID_LANG_ID_BASE_LIST		0x0207
#define SDP_ATTR_HID_SDP_DISABLE		0x0208
#define SDP_ATTR_HID_BATTERY_POWER		0x0209
#define SDP_ATTR_HID_REMOTE_WAKEUP		0x020a
#define SDP_ATTR_HID_PROFILE_VERSION		0x020b
#define SDP_ATTR_HID_SUPERVISION_TIMEOUT	0x020c
#define SDP_ATTR_HID_NORMALLY_CONNECTABLE	0x020d
#define SDP_ATTR_HID_BOOT_DEVICE		0x020e

/*
 * These identifiers are based on the SDP spec stating that
 * "base attribute id of the primary (universal) language must be 0x0100"
 *
 * Other languages should have their own offset; e.g.:
 * #define XXXLangBase yyyy
 * #define AttrServiceName_XXX	0x0000+XXXLangBase
 */
#define SDP_PRIMARY_LANG_BASE		0x0100

#define SDP_ATTR_SVCNAME_PRIMARY	0x0000 + SDP_PRIMARY_LANG_BASE
#define SDP_ATTR_SVCDESC_PRIMARY	0x0001 + SDP_PRIMARY_LANG_BASE
#define SDP_ATTR_PROVNAME_PRIMARY	0x0002 + SDP_PRIMARY_LANG_BASE

/*
 * The Data representation in SDP PDUs (pps 339, 340 of BT SDP Spec)
 * These are the exact data type+size descriptor values
 * that go into the PDU buffer.
 *
 * The datatype (leading 5bits) + size descriptor (last 3 bits)
 * is 8 bits. The size descriptor is critical to extract the
 * right number of bytes for the data value from the PDU.
 *
 * For most basic types, the datatype+size descriptor is
 * straightforward. However for constructed types and strings,
 * the size of the data is in the next "n" bytes following the
 * 8 bits (datatype+size) descriptor. Exactly what the "n" is
 * specified in the 3 bits of the data size descriptor.
 *
 * TextString and URLString can be of size 2^{8, 16, 32} bytes
 * DataSequence and DataSequenceAlternates can be of size 2^{8, 16, 32}
 * The size are computed post-facto in the API and are not known apriori
 */
#define SDP_DATA_NIL		0x00
#define SDP_UINT8		0x08
#define SDP_UINT16		0x09
#define SDP_UINT32		0x0A
#define SDP_UINT64		0x0B
#define SDP_UINT128		0x0C
#define SDP_INT8		0x10
#define SDP_INT16		0x11
#define SDP_INT32		0x12
#define SDP_INT64		0x13
#define SDP_INT128		0x14
#define SDP_UUID_UNSPEC		0x18
#define SDP_UUID16		0x19
#define SDP_UUID32		0x1A
#define SDP_UUID128		0x1C
#define SDP_TEXT_STR_UNSPEC	0x20
#define SDP_TEXT_STR8		0x25
#define SDP_TEXT_STR16		0x26
#define SDP_TEXT_STR32		0x27
#define SDP_BOOL		0x28
#define SDP_SEQ_UNSPEC		0x30
#define SDP_SEQ8		0x35
#define SDP_SEQ16		0x36
#define SDP_SEQ32		0x37
#define SDP_ALT_UNSPEC		0x38
#define SDP_ALT8		0x3D
#define SDP_ALT16		0x3E
#define SDP_ALT32		0x3F
#define SDP_URL_STR_UNSPEC	0x40
#define SDP_URL_STR8		0x45
#define SDP_URL_STR16		0x46
#define SDP_URL_STR32		0x47

/*
 * The PDU identifiers of SDP packets between client and server
 */
#define SDP_ERROR_RSP		0x01
#define SDP_SVC_SEARCH_REQ	0x02
#define SDP_SVC_SEARCH_RSP	0x03
#define SDP_SVC_ATTR_REQ	0x04
#define SDP_SVC_ATTR_RSP	0x05
#define SDP_SVC_SEARCH_ATTR_REQ	0x06
#define SDP_SVC_SEARCH_ATTR_RSP	0x07

/*
 * Some additions to support service registration.
 * These are outside the scope of the Bluetooth specification
 */
#define SDP_SVC_REGISTER_REQ	0x75
#define SDP_SVC_REGISTER_RSP	0x76
#define SDP_SVC_UPDATE_REQ	0x77
#define SDP_SVC_UPDATE_RSP	0x78
#define SDP_SVC_REMOVE_REQ	0x79
#define SDP_SVC_REMOVE_RSP	0x80

/*
 * SDP Error codes
 */
#define SDP_INVALID_VERSION		0x0001
#define SDP_INVALID_RECORD_HANDLE	0x0002
#define SDP_INVALID_SYNTAX		0x0003
#define SDP_INVALID_PDU_SIZE		0x0004
#define SDP_INVALID_CSTATE		0x0005

/*
 * SDP PDU
 */
typedef struct {
	uint8_t  pdu_id;
	uint16_t tid;
	uint16_t plen;
} __attribute__ ((packed)) sdp_pdu_hdr_t;

/*
 * Common definitions for attributes in the SDP.
 * Should the type of any of these change, you need only make a change here.
 */

typedef struct {
	uint8_t type;
	union {
		uint16_t  uuid16;
		uint32_t  uuid32;
		uint128_t uuid128;
	} value;
} uuid_t;

#define SDP_IS_UUID(x) ((x) == SDP_UUID16 || (x) == SDP_UUID32 || \
							(x) == SDP_UUID128)
#define SDP_IS_ALT(x)  ((x) == SDP_ALT8 || (x) == SDP_ALT16 || (x) == SDP_ALT32)
#define SDP_IS_SEQ(x)  ((x) == SDP_SEQ8 || (x) == SDP_SEQ16 || (x) == SDP_SEQ32)
#define SDP_IS_TEXT_STR(x) ((x) == SDP_TEXT_STR8 || (x) == SDP_TEXT_STR16 || \
							(x) == SDP_TEXT_STR32)

typedef struct _sdp_list sdp_list_t;
struct _sdp_list {
	sdp_list_t *next;
	void *data;
};

/*
 * User-visible strings can be in many languages
 * in addition to the universal language.
 *
 * Language meta-data includes language code in ISO639
 * followed by the encoding format. The third field in this
 * structure is the attribute offset for the language.
 * User-visible strings in the specified language can be
 * obtained at this offset.
 */
typedef struct {
	uint16_t code_ISO639;
	uint16_t encoding;
	uint16_t base_offset;
} sdp_lang_attr_t;

/*
 * Profile descriptor is the Bluetooth profile metadata. If a
 * service conforms to a well-known profile, then its profile
 * identifier (UUID) is an attribute of the service. In addition,
 * if the profile has a version number it is specified here.
 */
typedef struct {
	uuid_t uuid;
	uint16_t version;
} sdp_profile_desc_t;

typedef struct {
	uint8_t major;
	uint8_t minor;
} sdp_version_t;

typedef struct {
	uint8_t *data;
	uint32_t data_size;
	uint32_t buf_size;
} sdp_buf_t;

typedef struct {
	uint32_t handle;

	/* Search pattern: a sequence of all UUIDs seen in this record */
	sdp_list_t *pattern;
	sdp_list_t *attrlist;

	/* Main service class for Extended Inquiry Response */
	uuid_t svclass;
} sdp_record_t;

typedef struct sdp_data_struct sdp_data_t;
struct sdp_data_struct {
	uint8_t dtd;
	uint16_t attrId;
	union {
		int8_t    int8;
		int16_t   int16;
		int32_t   int32;
		int64_t   int64;
		uint128_t int128;
		uint8_t   uint8;
		uint16_t  uint16;
		uint32_t  uint32;
		uint64_t  uint64;
		uint128_t uint128;
		uuid_t    uuid;
		char     *str;
		sdp_data_t *dataseq;
	} val;
	sdp_data_t *next;
	int unitSize;
};

#ifdef __cplusplus
}
#endif

#endif /* __SDP_H */
