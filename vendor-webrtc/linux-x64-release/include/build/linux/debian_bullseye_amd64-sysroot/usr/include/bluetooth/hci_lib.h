/*
 *
 *  BlueZ - Bluetooth protocol stack for Linux
 *
 *  Copyright (C) 2000-2001  Qualcomm Incorporated
 *  Copyright (C) 2002-2003  Maxim Krasnyansky <maxk@qualcomm.com>
 *  Copyright (C) 2002-2010  Marcel Holtmann <marcel@holtmann.org>
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

#ifndef __HCI_LIB_H
#define __HCI_LIB_H

#ifdef __cplusplus
extern "C" {
#endif

struct hci_request {
	uint16_t ogf;
	uint16_t ocf;
	int      event;
	void     *cparam;
	int      clen;
	void     *rparam;
	int      rlen;
};

struct hci_version {
	uint16_t manufacturer;
	uint8_t  hci_ver;
	uint16_t hci_rev;
	uint8_t  lmp_ver;
	uint16_t lmp_subver;
};

int hci_open_dev(int dev_id);
int hci_close_dev(int dd);
int hci_send_cmd(int dd, uint16_t ogf, uint16_t ocf, uint8_t plen, void *param);
int hci_send_req(int dd, struct hci_request *req, int timeout);

int hci_create_connection(int dd, const bdaddr_t *bdaddr, uint16_t ptype, uint16_t clkoffset, uint8_t rswitch, uint16_t *handle, int to);
int hci_disconnect(int dd, uint16_t handle, uint8_t reason, int to);

int hci_inquiry(int dev_id, int len, int num_rsp, const uint8_t *lap, inquiry_info **ii, long flags);
int hci_devinfo(int dev_id, struct hci_dev_info *di);
int hci_devba(int dev_id, bdaddr_t *bdaddr);
int hci_devid(const char *str);

int hci_read_local_name(int dd, int len, char *name, int to);
int hci_write_local_name(int dd, const char *name, int to);
int hci_read_remote_name(int dd, const bdaddr_t *bdaddr, int len, char *name, int to);
int hci_read_remote_name_with_clock_offset(int dd, const bdaddr_t *bdaddr, uint8_t pscan_rep_mode, uint16_t clkoffset, int len, char *name, int to);
int hci_read_remote_name_cancel(int dd, const bdaddr_t *bdaddr, int to);
int hci_read_remote_version(int dd, uint16_t handle, struct hci_version *ver, int to);
int hci_read_remote_features(int dd, uint16_t handle, uint8_t *features, int to);
int hci_read_remote_ext_features(int dd, uint16_t handle, uint8_t page, uint8_t *max_page, uint8_t *features, int to);
int hci_read_clock_offset(int dd, uint16_t handle, uint16_t *clkoffset, int to);
int hci_read_local_version(int dd, struct hci_version *ver, int to);
int hci_read_local_commands(int dd, uint8_t *commands, int to);
int hci_read_local_features(int dd, uint8_t *features, int to);
int hci_read_local_ext_features(int dd, uint8_t page, uint8_t *max_page, uint8_t *features, int to);
int hci_read_bd_addr(int dd, bdaddr_t *bdaddr, int to);
int hci_read_class_of_dev(int dd, uint8_t *cls, int to);
int hci_write_class_of_dev(int dd, uint32_t cls, int to);
int hci_read_voice_setting(int dd, uint16_t *vs, int to);
int hci_write_voice_setting(int dd, uint16_t vs, int to);
int hci_read_current_iac_lap(int dd, uint8_t *num_iac, uint8_t *lap, int to);
int hci_write_current_iac_lap(int dd, uint8_t num_iac, uint8_t *lap, int to);
int hci_read_stored_link_key(int dd, bdaddr_t *bdaddr, uint8_t all, int to);
int hci_write_stored_link_key(int dd, bdaddr_t *bdaddr, uint8_t *key, int to);
int hci_delete_stored_link_key(int dd, bdaddr_t *bdaddr, uint8_t all, int to);
int hci_authenticate_link(int dd, uint16_t handle, int to);
int hci_encrypt_link(int dd, uint16_t handle, uint8_t encrypt, int to);
int hci_change_link_key(int dd, uint16_t handle, int to);
int hci_switch_role(int dd, bdaddr_t *bdaddr, uint8_t role, int to);
int hci_park_mode(int dd, uint16_t handle, uint16_t max_interval, uint16_t min_interval, int to);
int hci_exit_park_mode(int dd, uint16_t handle, int to);
int hci_read_inquiry_scan_type(int dd, uint8_t *type, int to);
int hci_write_inquiry_scan_type(int dd, uint8_t type, int to);
int hci_read_inquiry_mode(int dd, uint8_t *mode, int to);
int hci_write_inquiry_mode(int dd, uint8_t mode, int to);
int hci_read_afh_mode(int dd, uint8_t *mode, int to);
int hci_write_afh_mode(int dd, uint8_t mode, int to);
int hci_read_ext_inquiry_response(int dd, uint8_t *fec, uint8_t *data, int to);
int hci_write_ext_inquiry_response(int dd, uint8_t fec, uint8_t *data, int to);
int hci_read_simple_pairing_mode(int dd, uint8_t *mode, int to);
int hci_write_simple_pairing_mode(int dd, uint8_t mode, int to);
int hci_read_local_oob_data(int dd, uint8_t *hash, uint8_t *randomizer, int to);
int hci_read_inq_response_tx_power_level(int dd, int8_t *level, int to);
int hci_read_inquiry_transmit_power_level(int dd, int8_t *level, int to);
int hci_write_inquiry_transmit_power_level(int dd, int8_t level, int to);
int hci_read_transmit_power_level(int dd, uint16_t handle, uint8_t type, int8_t *level, int to);
int hci_read_link_policy(int dd, uint16_t handle, uint16_t *policy, int to);
int hci_write_link_policy(int dd, uint16_t handle, uint16_t policy, int to);
int hci_read_link_supervision_timeout(int dd, uint16_t handle, uint16_t *timeout, int to);
int hci_write_link_supervision_timeout(int dd, uint16_t handle, uint16_t timeout, int to);
int hci_set_afh_classification(int dd, uint8_t *map, int to);
int hci_read_link_quality(int dd, uint16_t handle, uint8_t *link_quality, int to);
int hci_read_rssi(int dd, uint16_t handle, int8_t *rssi, int to);
int hci_read_afh_map(int dd, uint16_t handle, uint8_t *mode, uint8_t *map, int to);
int hci_read_clock(int dd, uint16_t handle, uint8_t which, uint32_t *clock, uint16_t *accuracy, int to);

int hci_le_set_scan_enable(int dev_id, uint8_t enable, uint8_t filter_dup, int to);
int hci_le_set_scan_parameters(int dev_id, uint8_t type, uint16_t interval,
					uint16_t window, uint8_t own_type,
					uint8_t filter, int to);
int hci_le_set_advertise_enable(int dev_id, uint8_t enable, int to);
int hci_le_create_conn(int dd, uint16_t interval, uint16_t window,
		uint8_t initiator_filter, uint8_t peer_bdaddr_type,
		bdaddr_t peer_bdaddr, uint8_t own_bdaddr_type,
		uint16_t min_interval, uint16_t max_interval,
		uint16_t latency, uint16_t supervision_timeout,
		uint16_t min_ce_length, uint16_t max_ce_length,
		uint16_t *handle, int to);
int hci_le_conn_update(int dd, uint16_t handle, uint16_t min_interval,
			uint16_t max_interval, uint16_t latency,
			uint16_t supervision_timeout, int to);
int hci_le_add_white_list(int dd, const bdaddr_t *bdaddr, uint8_t type, int to);
int hci_le_rm_white_list(int dd, const bdaddr_t *bdaddr, uint8_t type, int to);
int hci_le_read_white_list_size(int dd, uint8_t *size, int to);
int hci_le_clear_white_list(int dd, int to);
int hci_le_add_resolving_list(int dd, const bdaddr_t *bdaddr, uint8_t type,
				uint8_t *peer_irk, uint8_t *local_irk, int to);
int hci_le_rm_resolving_list(int dd, const bdaddr_t *bdaddr, uint8_t type, int to);
int hci_le_clear_resolving_list(int dd, int to);
int hci_le_read_resolving_list_size(int dd, uint8_t *size, int to);
int hci_le_set_address_resolution_enable(int dev_id, uint8_t enable, int to);
int hci_le_read_remote_features(int dd, uint16_t handle, uint8_t *features, int to);

int hci_for_each_dev(int flag, int(*func)(int dd, int dev_id, long arg), long arg);
int hci_get_route(bdaddr_t *bdaddr);

char *hci_bustostr(int bus);
char *hci_typetostr(int type);
char *hci_dtypetostr(int type);
char *hci_dflagstostr(uint32_t flags);
char *hci_ptypetostr(unsigned int ptype);
int hci_strtoptype(char *str, unsigned int *val);
char *hci_scoptypetostr(unsigned int ptype);
int hci_strtoscoptype(char *str, unsigned int *val);
char *hci_lptostr(unsigned int ptype);
int hci_strtolp(char *str, unsigned int *val);
char *hci_lmtostr(unsigned int ptype);
int hci_strtolm(char *str, unsigned int *val);

char *hci_cmdtostr(unsigned int cmd);
char *hci_commandstostr(uint8_t *commands, char *pref, int width);

char *hci_vertostr(unsigned int ver);
int hci_strtover(char *str, unsigned int *ver);
char *lmp_vertostr(unsigned int ver);
int lmp_strtover(char *str, unsigned int *ver);
char *pal_vertostr(unsigned int ver);
int pal_strtover(char *str, unsigned int *ver);

char *lmp_featurestostr(uint8_t *features, char *pref, int width);

static inline void hci_set_bit(int nr, void *addr)
{
	*((uint32_t *) addr + (nr >> 5)) |= (1 << (nr & 31));
}

static inline void hci_clear_bit(int nr, void *addr)
{
	*((uint32_t *) addr + (nr >> 5)) &= ~(1 << (nr & 31));
}

static inline int hci_test_bit(int nr, void *addr)
{
	return *((uint32_t *) addr + (nr >> 5)) & (1 << (nr & 31));
}

/* HCI filter tools */
static inline void hci_filter_clear(struct hci_filter *f)
{
	memset(f, 0, sizeof(*f));
}
static inline void hci_filter_set_ptype(int t, struct hci_filter *f)
{
	hci_set_bit((t == HCI_VENDOR_PKT) ? 0 : (t & HCI_FLT_TYPE_BITS), &f->type_mask);
}
static inline void hci_filter_clear_ptype(int t, struct hci_filter *f)
{
	hci_clear_bit((t == HCI_VENDOR_PKT) ? 0 : (t & HCI_FLT_TYPE_BITS), &f->type_mask);
}
static inline int hci_filter_test_ptype(int t, struct hci_filter *f)
{
	return hci_test_bit((t == HCI_VENDOR_PKT) ? 0 : (t & HCI_FLT_TYPE_BITS), &f->type_mask);
}
static inline void hci_filter_all_ptypes(struct hci_filter *f)
{
	memset((void *) &f->type_mask, 0xff, sizeof(f->type_mask));
}
static inline void hci_filter_set_event(int e, struct hci_filter *f)
{
	hci_set_bit((e & HCI_FLT_EVENT_BITS), &f->event_mask);
}
static inline void hci_filter_clear_event(int e, struct hci_filter *f)
{
	hci_clear_bit((e & HCI_FLT_EVENT_BITS), &f->event_mask);
}
static inline int hci_filter_test_event(int e, struct hci_filter *f)
{
	return hci_test_bit((e & HCI_FLT_EVENT_BITS), &f->event_mask);
}
static inline void hci_filter_all_events(struct hci_filter *f)
{
	memset((void *) f->event_mask, 0xff, sizeof(f->event_mask));
}
static inline void hci_filter_set_opcode(int opcode, struct hci_filter *f)
{
	f->opcode = opcode;
}
static inline void hci_filter_clear_opcode(struct hci_filter *f)
{
	f->opcode = 0;
}
static inline int hci_filter_test_opcode(int opcode, struct hci_filter *f)
{
	return (f->opcode == opcode);
}

#ifdef __cplusplus
}
#endif

#endif /* __HCI_LIB_H */
