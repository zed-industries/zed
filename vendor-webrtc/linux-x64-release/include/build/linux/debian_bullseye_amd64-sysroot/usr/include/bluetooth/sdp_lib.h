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

#ifndef __SDP_LIB_H
#define __SDP_LIB_H

#include <sys/socket.h>
#include <bluetooth/bluetooth.h>
#include <bluetooth/hci.h>

#ifdef __cplusplus
extern "C" {
#endif

/*
 * SDP lists
 */
typedef void(*sdp_list_func_t)(void *, void *);
typedef void(*sdp_free_func_t)(void *);
typedef int (*sdp_comp_func_t)(const void *, const void *);

sdp_list_t *sdp_list_append(sdp_list_t *list, void *d);
sdp_list_t *sdp_list_remove(sdp_list_t *list, void *d);
sdp_list_t *sdp_list_insert_sorted(sdp_list_t *list, void *data, sdp_comp_func_t f);
void        sdp_list_free(sdp_list_t *list, sdp_free_func_t f);

static inline int sdp_list_len(const sdp_list_t *list)
{
	int n = 0;
	for (; list; list = list->next)
		n++;
	return n;
}

static inline sdp_list_t *sdp_list_find(sdp_list_t *list, void *u, sdp_comp_func_t f)
{
	for (; list; list = list->next)
		if (f(list->data, u) == 0)
			return list;
	return NULL;
}

static inline void sdp_list_foreach(sdp_list_t *list, sdp_list_func_t f, void *u)
{
	for (; list; list = list->next)
		f(list->data, u);
}

/*
 * Values of the flags parameter to sdp_record_register
 */
#define SDP_RECORD_PERSIST	0x01
#define SDP_DEVICE_RECORD	0x02

/*
 * Values of the flags parameter to sdp_connect
 */
#define SDP_RETRY_IF_BUSY	0x01
#define SDP_WAIT_ON_CLOSE	0x02
#define SDP_NON_BLOCKING	0x04
#define SDP_LARGE_MTU		0x08

/*
 * a session with an SDP server
 */
typedef struct {
	int sock;
	int state;
	int local;
	int flags;
	uint16_t tid;	/* Current transaction ID */
	void *priv;
} sdp_session_t;

typedef enum {
	/*
	 *  Attributes are specified as individual elements
	 */
	SDP_ATTR_REQ_INDIVIDUAL = 1,
	/*
	 *  Attributes are specified as a range
	 */
	SDP_ATTR_REQ_RANGE
} sdp_attrreq_type_t;

/*
 * 	When the pdu_id(type) is a sdp error response, check the status value
 * 	to figure out the error reason. For status values 0x0001-0x0006 check
 * 	Bluetooth SPEC. If the status is 0xffff, call sdp_get_error function
 * 	to get the real reason:
 * 	    - wrong transaction ID(EPROTO)
 * 	    - wrong PDU id or(EPROTO)
 * 	    - I/O error
 */
typedef void sdp_callback_t(uint8_t type, uint16_t status, uint8_t *rsp, size_t size, void *udata);

/*
 * create an L2CAP connection to a Bluetooth device
 *
 * INPUT:
 *
 *  bdaddr_t *src:
 *	Address of the local device to use to make the connection
 *	(or BDADDR_ANY)
 *
 *  bdaddr_t *dst:
 *    Address of the SDP server device
 */
sdp_session_t *sdp_connect(const bdaddr_t *src, const bdaddr_t *dst, uint32_t flags);
int sdp_close(sdp_session_t *session);
int sdp_get_socket(const sdp_session_t *session);

/*
 * SDP transaction: functions for asynchronous search.
 */
sdp_session_t *sdp_create(int sk, uint32_t flags);
int sdp_get_error(sdp_session_t *session);
int sdp_process(sdp_session_t *session);
int sdp_set_notify(sdp_session_t *session, sdp_callback_t *func, void *udata);

int sdp_service_search_async(sdp_session_t *session, const sdp_list_t *search, uint16_t max_rec_num);
int sdp_service_attr_async(sdp_session_t *session, uint32_t handle, sdp_attrreq_type_t reqtype, const sdp_list_t *attrid_list);
int sdp_service_search_attr_async(sdp_session_t *session, const sdp_list_t *search, sdp_attrreq_type_t reqtype, const sdp_list_t *attrid_list);

uint16_t sdp_gen_tid(sdp_session_t *session);

/*
 * find all devices in the piconet
 */
int sdp_general_inquiry(inquiry_info *ii, int dev_num, int duration, uint8_t *found);

/* flexible extraction of basic attributes - Jean II */
int sdp_get_int_attr(const sdp_record_t *rec, uint16_t attr, int *value);
int sdp_get_string_attr(const sdp_record_t *rec, uint16_t attr, char *value, int valuelen);

/*
 * Basic sdp data functions
 */
sdp_data_t *sdp_data_alloc(uint8_t dtd, const void *value);
sdp_data_t *sdp_data_alloc_with_length(uint8_t dtd, const void *value, uint32_t length);
void sdp_data_free(sdp_data_t *data);
sdp_data_t *sdp_data_get(const sdp_record_t *rec, uint16_t attr_id);

sdp_data_t *sdp_seq_alloc(void **dtds, void **values, int len);
sdp_data_t *sdp_seq_alloc_with_length(void **dtds, void **values, int *length, int len);
sdp_data_t *sdp_seq_append(sdp_data_t *seq, sdp_data_t *data);

int sdp_attr_add(sdp_record_t *rec, uint16_t attr, sdp_data_t *data);
void sdp_attr_remove(sdp_record_t *rec, uint16_t attr);
void sdp_attr_replace(sdp_record_t *rec, uint16_t attr, sdp_data_t *data);
int sdp_set_uuidseq_attr(sdp_record_t *rec, uint16_t attr, sdp_list_t *seq);
int sdp_get_uuidseq_attr(const sdp_record_t *rec, uint16_t attr, sdp_list_t **seqp);

/*
 * NOTE that none of the functions below will update the SDP server,
 * unless the {register, update}sdp_record_t() function is invoked.
 * All functions which return an integer value, return 0 on success
 * or -1 on failure.
 */

/*
 * Create an attribute and add it to the service record's attribute list.
 * This consists of the data type descriptor of the attribute,
 * the value of the attribute and the attribute identifier.
 */
int sdp_attr_add_new(sdp_record_t *rec, uint16_t attr, uint8_t dtd, const void *p);

/*
 * Set the information attributes of the service record.
 * The set of attributes comprises service name, description
 * and provider name
 */
void sdp_set_info_attr(sdp_record_t *rec, const char *name, const char *prov, const char *desc);

/*
 * Set the ServiceClassID attribute to the sequence specified by seq.
 * Note that the identifiers need to be in sorted order from the most
 * specific to the most generic service class that this service
 * conforms to.
 */
static inline int sdp_set_service_classes(sdp_record_t *rec, sdp_list_t *seq)
{
	return sdp_set_uuidseq_attr(rec, SDP_ATTR_SVCLASS_ID_LIST, seq);
}

/*
 * Get the service classes to which the service conforms.
 *
 * When set, the list contains elements of ServiceClassIdentifer(uint16_t)
 * ordered from most specific to most generic
 */
static inline int sdp_get_service_classes(const sdp_record_t *rec, sdp_list_t **seqp)
{
	return sdp_get_uuidseq_attr(rec, SDP_ATTR_SVCLASS_ID_LIST, seqp);
}

/*
 * Set the BrowseGroupList attribute to the list specified by seq.
 *
 * A service can belong to one or more service groups
 * and the list comprises such group identifiers (UUIDs)
 */
static inline int sdp_set_browse_groups(sdp_record_t *rec, sdp_list_t *seq)
{
	return sdp_set_uuidseq_attr(rec, SDP_ATTR_BROWSE_GRP_LIST, seq);
}

/*
 * Set the access protocols of the record to those specified in proto
 */
int sdp_set_access_protos(sdp_record_t *rec, const sdp_list_t *proto);

/*
 * Set the additional access protocols of the record to those specified in proto
 */
int sdp_set_add_access_protos(sdp_record_t *rec, const sdp_list_t *proto);

/*
 * Get protocol port (i.e. PSM for L2CAP, Channel for RFCOMM)
 */
int sdp_get_proto_port(const sdp_list_t *list, int proto);

/*
 * Get protocol descriptor.
 */
sdp_data_t *sdp_get_proto_desc(sdp_list_t *list, int proto);

/*
 * Set the LanguageBase attributes to the values specified in list
 * (a linked list of sdp_lang_attr_t objects, one for each language in
 * which user-visible attributes are present).
 */
int sdp_set_lang_attr(sdp_record_t *rec, const sdp_list_t *list);

/*
 * Set the ServiceInfoTimeToLive attribute of the service.
 * This is the number of seconds that this record is guaranteed
 * not to change after being obtained by a client.
 */
static inline int sdp_set_service_ttl(sdp_record_t *rec, uint32_t ttl)
{
	return sdp_attr_add_new(rec, SDP_ATTR_SVCINFO_TTL, SDP_UINT32, &ttl);
}

/*
 * Set the ServiceRecordState attribute of a service. This is
 * guaranteed to change if there is any kind of modification to
 * the record.
 */
static inline int sdp_set_record_state(sdp_record_t *rec, uint32_t state)
{
	return sdp_attr_add_new(rec, SDP_ATTR_RECORD_STATE, SDP_UINT32, &state);
}

/*
 * Set the ServiceID attribute of a service.
 */
void sdp_set_service_id(sdp_record_t *rec, uuid_t uuid);

/*
 * Set the GroupID attribute of a service
 */
void sdp_set_group_id(sdp_record_t *rec, uuid_t grouuuid);

/*
 * Set the ServiceAvailability attribute of a service.
 *
 * Note that this represents the relative availability
 * of the service: 0x00 means completely unavailable;
 * 0xFF means maximum availability.
 */
static inline int sdp_set_service_avail(sdp_record_t *rec, uint8_t avail)
{
	return sdp_attr_add_new(rec, SDP_ATTR_SERVICE_AVAILABILITY, SDP_UINT8, &avail);
}

/*
 * Set the profile descriptor list attribute of a record.
 *
 * Each element in the list is an object of type
 * sdp_profile_desc_t which is a definition of the
 * Bluetooth profile that this service conforms to.
 */
int sdp_set_profile_descs(sdp_record_t *rec, const sdp_list_t *desc);

/*
 * Set URL attributes of a record.
 *
 * ClientExecutableURL: a URL to a client's platform specific (WinCE,
 * PalmOS) executable code that can be used to access this service.
 *
 * DocumentationURL: a URL pointing to service documentation
 *
 * IconURL: a URL to an icon that can be used to represent this service.
 *
 * Note: pass NULL for any URLs that you don't want to set or remove
 */
void sdp_set_url_attr(sdp_record_t *rec, const char *clientExecURL, const char *docURL, const char *iconURL);

/*
 * a service search request.
 *
 *  INPUT :
 *
 *    sdp_list_t *search
 *      list containing elements of the search
 *      pattern. Each entry in the list is a UUID
 *      of the service to be searched
 *
 *    uint16_t max_rec_num
 *       An integer specifying the maximum number of
 *       entries that the client can handle in the response.
 *
 *  OUTPUT :
 *
 *    int return value
 *      0
 *        The request completed successfully. This does not
 *        mean the requested services were found
 *      -1
 *        The request completed unsuccessfully
 *
 *    sdp_list_t *rsp_list
 *      This variable is set on a successful return if there are
 *      non-zero service handles. It is a singly linked list of
 *      service record handles (uint16_t)
 */
int sdp_service_search_req(sdp_session_t *session, const sdp_list_t *search, uint16_t max_rec_num, sdp_list_t **rsp_list);

/*
 *  a service attribute request.
 *
 *  INPUT :
 *
 *    uint32_t handle
 *      The handle of the service for which the attribute(s) are
 *      requested
 *
 *    sdp_attrreq_type_t reqtype
 *      Attribute identifiers are 16 bit unsigned integers specified
 *      in one of 2 ways described below :
 *      SDP_ATTR_REQ_INDIVIDUAL - 16bit individual identifiers
 *         They are the actual attribute identifiers in ascending order
 *
 *      SDP_ATTR_REQ_RANGE - 32bit identifier range
 *         The high-order 16bits is the start of range
 *         the low-order 16bits are the end of range
 *         0x0000 to 0xFFFF gets all attributes
 *
 *    sdp_list_t *attrid_list
 *      Singly linked list containing attribute identifiers desired.
 *      Every element is either a uint16_t(attrSpec = SDP_ATTR_REQ_INDIVIDUAL)
 *      or a uint32_t(attrSpec=SDP_ATTR_REQ_RANGE)
 *
 *  OUTPUT :
 *    int return value
 *      0
 *        The request completed successfully. This does not
 *        mean the requested services were found
 *      -1
 *        The request completed unsuccessfully due to a timeout
 */
sdp_record_t *sdp_service_attr_req(sdp_session_t *session, uint32_t handle, sdp_attrreq_type_t reqtype, const sdp_list_t *attrid_list);

/*
 *  This is a service search request combined with the service
 *  attribute request. First a service class match is done and
 *  for matching service, requested attributes are extracted
 *
 *  INPUT :
 *
 *    sdp_list_t *search
 *      Singly linked list containing elements of the search
 *      pattern. Each entry in the list is a UUID(DataTypeSDP_UUID16)
 *      of the service to be searched
 *
 *    AttributeSpecification attrSpec
 *      Attribute identifiers are 16 bit unsigned integers specified
 *      in one of 2 ways described below :
 *      SDP_ATTR_REQ_INDIVIDUAL - 16bit individual identifiers
 *         They are the actual attribute identifiers in ascending order
 *
 *      SDP_ATTR_REQ_RANGE - 32bit identifier range
 *         The high-order 16bits is the start of range
 *         the low-order 16bits are the end of range
 *         0x0000 to 0xFFFF gets all attributes
 *
 *    sdp_list_t *attrid_list
 *      Singly linked list containing attribute identifiers desired.
 *      Every element is either a uint16_t(attrSpec = SDP_ATTR_REQ_INDIVIDUAL)
 *      or a uint32_t(attrSpec=SDP_ATTR_REQ_RANGE)
 *
 *  OUTPUT :
 *    int return value
 *      0
 *        The request completed successfully. This does not
 *        mean the requested services were found
 *      -1
 *        The request completed unsuccessfully due to a timeout
 *
 *    sdp_list_t *rsp_list
 *      This variable is set on a successful return to point to
 *      service(s) found. Each element of this list is of type
 *      sdp_record_t *.
 */
int sdp_service_search_attr_req(sdp_session_t *session, const sdp_list_t *search, sdp_attrreq_type_t reqtype, const sdp_list_t *attrid_list, sdp_list_t **rsp_list);

/*
 * Allocate/free a service record and its attributes
 */
sdp_record_t *sdp_record_alloc(void);
void sdp_record_free(sdp_record_t *rec);

/*
 * Register a service record.
 *
 * Note: It is the responsbility of the Service Provider to create the
 * record first and set its attributes using setXXX() methods.
 *
 * The service provider must then call sdp_record_register() to make
 * the service record visible to SDP clients.  This function returns 0
 * on success or -1 on failure (and sets errno).
 */
int sdp_device_record_register_binary(sdp_session_t *session, bdaddr_t *device, uint8_t *data, uint32_t size, uint8_t flags, uint32_t *handle);
int sdp_device_record_register(sdp_session_t *session, bdaddr_t *device, sdp_record_t *rec, uint8_t flags);
int sdp_record_register(sdp_session_t *session, sdp_record_t *rec, uint8_t flags);

/*
 * Unregister a service record.
 */
int sdp_device_record_unregister_binary(sdp_session_t *session, bdaddr_t *device, uint32_t handle);
int sdp_device_record_unregister(sdp_session_t *session, bdaddr_t *device, sdp_record_t *rec);
int sdp_record_unregister(sdp_session_t *session, sdp_record_t *rec);

/*
 * Update an existing service record.  (Calling this function
 * before a previous call to sdp_record_register() will result
 * in an error.)
 */
int sdp_device_record_update_binary(sdp_session_t *session, bdaddr_t *device, uint32_t handle, uint8_t *data, uint32_t size);
int sdp_device_record_update(sdp_session_t *session, bdaddr_t *device, const sdp_record_t *rec);
int sdp_record_update(sdp_session_t *sess, const sdp_record_t *rec);

void sdp_record_print(const sdp_record_t *rec);

/*
 * UUID functions
 */
uuid_t *sdp_uuid16_create(uuid_t *uuid, uint16_t data);
uuid_t *sdp_uuid32_create(uuid_t *uuid, uint32_t data);
uuid_t *sdp_uuid128_create(uuid_t *uuid, const void *data);
int sdp_uuid16_cmp(const void *p1, const void *p2);
int sdp_uuid128_cmp(const void *p1, const void *p2);
int sdp_uuid_cmp(const void *p1, const void *p2);
uuid_t *sdp_uuid_to_uuid128(const uuid_t *uuid);
void sdp_uuid16_to_uuid128(uuid_t *uuid128, const uuid_t *uuid16);
void sdp_uuid32_to_uuid128(uuid_t *uuid128, const uuid_t *uuid32);
int sdp_uuid128_to_uuid(uuid_t *uuid);
int sdp_uuid_to_proto(uuid_t *uuid);
int sdp_uuid_extract(const uint8_t *buffer, int bufsize, uuid_t *uuid, int *scanned);
void sdp_uuid_print(const uuid_t *uuid);

#define MAX_LEN_UUID_STR 37
#define MAX_LEN_PROTOCOL_UUID_STR 8
#define MAX_LEN_SERVICECLASS_UUID_STR 28
#define MAX_LEN_PROFILEDESCRIPTOR_UUID_STR 28

int sdp_uuid2strn(const uuid_t *uuid, char *str, size_t n);
int sdp_proto_uuid2strn(const uuid_t *uuid, char *str, size_t n);
int sdp_svclass_uuid2strn(const uuid_t *uuid, char *str, size_t n);
int sdp_profile_uuid2strn(const uuid_t *uuid, char *str, size_t n);

/*
 * In all the sdp_get_XXX(handle, XXX *xxx) functions below,
 * the XXX * is set to point to the value, should it exist
 * and 0 is returned. If the value does not exist, -1 is
 * returned and errno set to ENODATA.
 *
 * In all the methods below, the memory management rules are
 * simple. Don't free anything! The pointer returned, in the
 * case of constructed types, is a pointer to the contents
 * of the sdp_record_t.
 */

/*
 * Get the access protocols from the service record
 */
int sdp_get_access_protos(const sdp_record_t *rec, sdp_list_t **protos);

/*
 * Get the additional access protocols from the service record
 */
int sdp_get_add_access_protos(const sdp_record_t *rec, sdp_list_t **protos);

/*
 * Extract the list of browse groups to which the service belongs.
 * When set, seqp contains elements of GroupID (uint16_t)
 */
static inline int sdp_get_browse_groups(const sdp_record_t *rec, sdp_list_t **seqp)
{
	return sdp_get_uuidseq_attr(rec, SDP_ATTR_BROWSE_GRP_LIST, seqp);
}

/*
 * Extract language attribute meta-data of the service record.
 * For each language in the service record, LangSeq has a struct of type
 * sdp_lang_attr_t.
 */
int sdp_get_lang_attr(const sdp_record_t *rec, sdp_list_t **langSeq);

/*
 * Extract the Bluetooth profile descriptor sequence from a record.
 * Each element in the list is of type sdp_profile_desc_t
 * which contains the UUID of the profile and its version number
 * (encoded as major and minor in the high-order 8bits
 * and low-order 8bits respectively of the uint16_t)
 */
int sdp_get_profile_descs(const sdp_record_t *rec, sdp_list_t **profDesc);

/*
 * Extract SDP server version numbers
 *
 * Note: that this is an attribute of the SDP server only and
 * contains a list of uint16_t each of which represent the
 * major and minor SDP version numbers supported by this server
 */
int sdp_get_server_ver(const sdp_record_t *rec, sdp_list_t **pVnumList);

int sdp_get_service_id(const sdp_record_t *rec, uuid_t *uuid);
int sdp_get_group_id(const sdp_record_t *rec, uuid_t *uuid);
int sdp_get_record_state(const sdp_record_t *rec, uint32_t *svcRecState);
int sdp_get_service_avail(const sdp_record_t *rec, uint8_t *svcAvail);
int sdp_get_service_ttl(const sdp_record_t *rec, uint32_t *svcTTLInfo);
int sdp_get_database_state(const sdp_record_t *rec, uint32_t *svcDBState);

static inline int sdp_get_service_name(const sdp_record_t *rec, char *str, int len)
{
	return sdp_get_string_attr(rec, SDP_ATTR_SVCNAME_PRIMARY, str, len);
}

static inline int sdp_get_service_desc(const sdp_record_t *rec, char *str, int len)
{
	return sdp_get_string_attr(rec, SDP_ATTR_SVCDESC_PRIMARY, str, len);
}

static inline int sdp_get_provider_name(const sdp_record_t *rec, char *str, int len)
{
	return sdp_get_string_attr(rec, SDP_ATTR_PROVNAME_PRIMARY, str, len);
}

static inline int sdp_get_doc_url(const sdp_record_t *rec, char *str, int len)
{
	return sdp_get_string_attr(rec, SDP_ATTR_DOC_URL, str, len);
}

static inline int sdp_get_clnt_exec_url(const sdp_record_t *rec, char *str, int len)
{
	return sdp_get_string_attr(rec, SDP_ATTR_CLNT_EXEC_URL, str, len);
}

static inline int sdp_get_icon_url(const sdp_record_t *rec, char *str, int len)
{
	return sdp_get_string_attr(rec, SDP_ATTR_ICON_URL, str, len);
}

/*
 * Set the supported features
 * sf should be a list of list with each feature data
 * Returns 0 on success -1 on fail
 */
int sdp_set_supp_feat(sdp_record_t *rec, const sdp_list_t *sf);

/*
 * Get the supported features
 * seqp is set to a list of list with each feature data
 * Returns 0 on success, if an error occurred -1 is returned and errno is set
 */
int sdp_get_supp_feat(const sdp_record_t *rec, sdp_list_t **seqp);

sdp_record_t *sdp_extract_pdu(const uint8_t *pdata, int bufsize, int *scanned);
sdp_record_t *sdp_copy_record(sdp_record_t *rec);

void sdp_data_print(sdp_data_t *data);
void sdp_print_service_attr(sdp_list_t *alist);

int sdp_attrid_comp_func(const void *key1, const void *key2);

void sdp_set_seq_len(uint8_t *ptr, uint32_t length);
void sdp_set_attrid(sdp_buf_t *pdu, uint16_t id);
void sdp_append_to_pdu(sdp_buf_t *dst, sdp_data_t *d);
void sdp_append_to_buf(sdp_buf_t *dst, uint8_t *data, uint32_t len);

int sdp_gen_pdu(sdp_buf_t *pdu, sdp_data_t *data);
int sdp_gen_record_pdu(const sdp_record_t *rec, sdp_buf_t *pdu);

int sdp_extract_seqtype(const uint8_t *buf, int bufsize, uint8_t *dtdp, int *size);

sdp_data_t *sdp_extract_attr(const uint8_t *pdata, int bufsize, int *extractedLength, sdp_record_t *rec);

void sdp_pattern_add_uuid(sdp_record_t *rec, uuid_t *uuid);
void sdp_pattern_add_uuidseq(sdp_record_t *rec, sdp_list_t *seq);

int sdp_send_req_w4_rsp(sdp_session_t *session, uint8_t *req, uint8_t *rsp, uint32_t reqsize, uint32_t *rspsize);

void sdp_add_lang_attr(sdp_record_t *rec);

#ifdef __cplusplus
}
#endif

#endif /* __SDP_LIB_H */
