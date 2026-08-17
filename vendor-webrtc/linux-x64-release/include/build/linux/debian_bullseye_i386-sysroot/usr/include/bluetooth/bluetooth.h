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

#ifndef __BLUETOOTH_H
#define __BLUETOOTH_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include <endian.h>
#include <byteswap.h>
#include <netinet/in.h>

#ifndef AF_BLUETOOTH
#define AF_BLUETOOTH	31
#define PF_BLUETOOTH	AF_BLUETOOTH
#endif

#define BTPROTO_L2CAP	0
#define BTPROTO_HCI	1
#define BTPROTO_SCO	2
#define BTPROTO_RFCOMM	3
#define BTPROTO_BNEP	4
#define BTPROTO_CMTP	5
#define BTPROTO_HIDP	6
#define BTPROTO_AVDTP	7

#define SOL_HCI		0
#define SOL_L2CAP	6
#define SOL_SCO		17
#define SOL_RFCOMM	18

#ifndef SOL_BLUETOOTH
#define SOL_BLUETOOTH	274
#endif

#define BT_SECURITY	4
struct bt_security {
	uint8_t level;
	uint8_t key_size;
};
#define BT_SECURITY_SDP		0
#define BT_SECURITY_LOW		1
#define BT_SECURITY_MEDIUM	2
#define BT_SECURITY_HIGH	3
#define BT_SECURITY_FIPS	4

#define BT_DEFER_SETUP	7

#define BT_FLUSHABLE	8

#define BT_FLUSHABLE_OFF	0
#define BT_FLUSHABLE_ON		1

#define BT_POWER		9
struct bt_power {
	uint8_t force_active;
};
#define BT_POWER_FORCE_ACTIVE_OFF 0
#define BT_POWER_FORCE_ACTIVE_ON  1

#define BT_CHANNEL_POLICY	10

/* BR/EDR only (default policy)
 *   AMP controllers cannot be used.
 *   Channel move requests from the remote device are denied.
 *   If the L2CAP channel is currently using AMP, move the channel to BR/EDR.
 */
#define BT_CHANNEL_POLICY_BREDR_ONLY		0

/* BR/EDR Preferred
 *   Allow use of AMP controllers.
 *   If the L2CAP channel is currently on AMP, move it to BR/EDR.
 *   Channel move requests from the remote device are allowed.
 */
#define BT_CHANNEL_POLICY_BREDR_PREFERRED	1

/* AMP Preferred
 *   Allow use of AMP controllers
 *   If the L2CAP channel is currently on BR/EDR and AMP controller
 *     resources are available, initiate a channel move to AMP.
 *   Channel move requests from the remote device are allowed.
 *   If the L2CAP socket has not been connected yet, try to create
 *     and configure the channel directly on an AMP controller rather
 *     than BR/EDR.
 */
#define BT_CHANNEL_POLICY_AMP_PREFERRED		2

#define BT_VOICE		11
struct bt_voice {
	uint16_t setting;
};

#define BT_SNDMTU		12
#define BT_RCVMTU		13

#define BT_VOICE_TRANSPARENT			0x0003
#define BT_VOICE_CVSD_16BIT			0x0060

#define BT_PHY			14

#define BT_PHY_BR_1M_1SLOT	0x00000001
#define BT_PHY_BR_1M_3SLOT	0x00000002
#define BT_PHY_BR_1M_5SLOT	0x00000004
#define BT_PHY_EDR_2M_1SLOT	0x00000008
#define BT_PHY_EDR_2M_3SLOT	0x00000010
#define BT_PHY_EDR_2M_5SLOT	0x00000020
#define BT_PHY_EDR_3M_1SLOT	0x00000040
#define BT_PHY_EDR_3M_3SLOT	0x00000080
#define BT_PHY_EDR_3M_5SLOT	0x00000100
#define BT_PHY_LE_1M_TX		0x00000200
#define BT_PHY_LE_1M_RX		0x00000400
#define BT_PHY_LE_2M_TX		0x00000800
#define BT_PHY_LE_2M_RX		0x00001000
#define BT_PHY_LE_CODED_TX	0x00002000
#define BT_PHY_LE_CODED_RX	0x00004000

#define BT_MODE			15

#define BT_MODE_BASIC		0x00
#define BT_MODE_ERTM		0x01
#define BT_MODE_STREAMING	0x02
#define BT_MODE_LE_FLOWCTL	0x03
#define BT_MODE_EXT_FLOWCTL	0x04

/* Connection and socket states */
enum {
	BT_CONNECTED = 1, /* Equal to TCP_ESTABLISHED to make net code happy */
	BT_OPEN,
	BT_BOUND,
	BT_LISTEN,
	BT_CONNECT,
	BT_CONNECT2,
	BT_CONFIG,
	BT_DISCONN,
	BT_CLOSED
};

/* Byte order conversions */
#if __BYTE_ORDER == __LITTLE_ENDIAN
#define htobs(d)  (d)
#define htobl(d)  (d)
#define htobll(d) (d)
#define btohs(d)  (d)
#define btohl(d)  (d)
#define btohll(d) (d)
#elif __BYTE_ORDER == __BIG_ENDIAN
#define htobs(d)  bswap_16(d)
#define htobl(d)  bswap_32(d)
#define htobll(d) bswap_64(d)
#define btohs(d)  bswap_16(d)
#define btohl(d)  bswap_32(d)
#define btohll(d) bswap_64(d)
#else
#error "Unknown byte order"
#endif

/* Bluetooth unaligned access */
#define bt_get_unaligned(ptr)			\
__extension__ ({				\
	struct __attribute__((packed)) {	\
		__typeof__(*(ptr)) __v;		\
	} *__p = (__typeof__(__p)) (ptr);	\
	__p->__v;				\
})

#define bt_put_unaligned(val, ptr)		\
do {						\
	struct __attribute__((packed)) {	\
		__typeof__(*(ptr)) __v;		\
	} *__p = (__typeof__(__p)) (ptr);	\
	__p->__v = (val);			\
} while(0)

#if __BYTE_ORDER == __LITTLE_ENDIAN
static inline uint64_t bt_get_le64(const void *ptr)
{
	return bt_get_unaligned((const uint64_t *) ptr);
}

static inline uint64_t bt_get_be64(const void *ptr)
{
	return bswap_64(bt_get_unaligned((const uint64_t *) ptr));
}

static inline uint32_t bt_get_le32(const void *ptr)
{
	return bt_get_unaligned((const uint32_t *) ptr);
}

static inline uint32_t bt_get_be32(const void *ptr)
{
	return bswap_32(bt_get_unaligned((const uint32_t *) ptr));
}

static inline uint16_t bt_get_le16(const void *ptr)
{
	return bt_get_unaligned((const uint16_t *) ptr);
}

static inline uint16_t bt_get_be16(const void *ptr)
{
	return bswap_16(bt_get_unaligned((const uint16_t *) ptr));
}

static inline void bt_put_le64(uint64_t val, const void *ptr)
{
	bt_put_unaligned(val, (uint64_t *) ptr);
}

static inline void bt_put_be64(uint64_t val, const void *ptr)
{
	bt_put_unaligned(bswap_64(val), (uint64_t *) ptr);
}

static inline void bt_put_le32(uint32_t val, const void *ptr)
{
	bt_put_unaligned(val, (uint32_t *) ptr);
}

static inline void bt_put_be32(uint32_t val, const void *ptr)
{
	bt_put_unaligned(bswap_32(val), (uint32_t *) ptr);
}

static inline void bt_put_le16(uint16_t val, const void *ptr)
{
	bt_put_unaligned(val, (uint16_t *) ptr);
}

static inline void bt_put_be16(uint16_t val, const void *ptr)
{
	bt_put_unaligned(bswap_16(val), (uint16_t *) ptr);
}

#elif __BYTE_ORDER == __BIG_ENDIAN
static inline uint64_t bt_get_le64(const void *ptr)
{
	return bswap_64(bt_get_unaligned((const uint64_t *) ptr));
}

static inline uint64_t bt_get_be64(const void *ptr)
{
	return bt_get_unaligned((const uint64_t *) ptr);
}

static inline uint32_t bt_get_le32(const void *ptr)
{
	return bswap_32(bt_get_unaligned((const uint32_t *) ptr));
}

static inline uint32_t bt_get_be32(const void *ptr)
{
	return bt_get_unaligned((const uint32_t *) ptr);
}

static inline uint16_t bt_get_le16(const void *ptr)
{
	return bswap_16(bt_get_unaligned((const uint16_t *) ptr));
}

static inline uint16_t bt_get_be16(const void *ptr)
{
	return bt_get_unaligned((const uint16_t *) ptr);
}

static inline void bt_put_le64(uint64_t val, const void *ptr)
{
	bt_put_unaligned(bswap_64(val), (uint64_t *) ptr);
}

static inline void bt_put_be64(uint64_t val, const void *ptr)
{
	bt_put_unaligned(val, (uint64_t *) ptr);
}

static inline void bt_put_le32(uint32_t val, const void *ptr)
{
	bt_put_unaligned(bswap_32(val), (uint32_t *) ptr);
}

static inline void bt_put_be32(uint32_t val, const void *ptr)
{
	bt_put_unaligned(val, (uint32_t *) ptr);
}

static inline void bt_put_le16(uint16_t val, const void *ptr)
{
	bt_put_unaligned(bswap_16(val), (uint16_t *) ptr);
}

static inline void bt_put_be16(uint16_t val, const void *ptr)
{
	bt_put_unaligned(val, (uint16_t *) ptr);
}
#else
#error "Unknown byte order"
#endif

/* BD Address */
typedef struct {
	uint8_t b[6];
} __attribute__((packed)) bdaddr_t;

/* BD Address type */
#define BDADDR_BREDR           0x00
#define BDADDR_LE_PUBLIC       0x01
#define BDADDR_LE_RANDOM       0x02

#define BDADDR_ANY   (&(bdaddr_t) {{0, 0, 0, 0, 0, 0}})
#define BDADDR_ALL   (&(bdaddr_t) {{0xff, 0xff, 0xff, 0xff, 0xff, 0xff}})
#define BDADDR_LOCAL (&(bdaddr_t) {{0, 0, 0, 0xff, 0xff, 0xff}})

/* Copy, swap, convert BD Address */
static inline int bacmp(const bdaddr_t *ba1, const bdaddr_t *ba2)
{
	return memcmp(ba1, ba2, sizeof(bdaddr_t));
}
static inline void bacpy(bdaddr_t *dst, const bdaddr_t *src)
{
	memcpy(dst, src, sizeof(bdaddr_t));
}

void baswap(bdaddr_t *dst, const bdaddr_t *src);
bdaddr_t *strtoba(const char *str);
char *batostr(const bdaddr_t *ba);
int ba2str(const bdaddr_t *ba, char *str);
int ba2strlc(const bdaddr_t *ba, char *str);
int str2ba(const char *str, bdaddr_t *ba);
int ba2oui(const bdaddr_t *ba, char *oui);
int bachk(const char *str);

int baprintf(const char *format, ...);
int bafprintf(FILE *stream, const char *format, ...);
int basprintf(char *str, const char *format, ...);
int basnprintf(char *str, size_t size, const char *format, ...);

void *bt_malloc(size_t size);
void bt_free(void *ptr);

int bt_error(uint16_t code);
const char *bt_compidtostr(int id);

typedef struct {
	uint8_t data[16];
} uint128_t;

static inline void bswap_128(const void *src, void *dst)
{
	const uint8_t *s = (const uint8_t *) src;
	uint8_t *d = (uint8_t *) dst;
	int i;

	for (i = 0; i < 16; i++)
		d[15 - i] = s[i];
}

#if __BYTE_ORDER == __BIG_ENDIAN

#define ntoh64(x) (x)

static inline void ntoh128(const uint128_t *src, uint128_t *dst)
{
	memcpy(dst, src, sizeof(uint128_t));
}

static inline void btoh128(const uint128_t *src, uint128_t *dst)
{
	bswap_128(src, dst);
}

#else

static inline uint64_t ntoh64(uint64_t n)
{
	uint64_t h;
	uint64_t tmp = ntohl(n & 0x00000000ffffffff);

	h = ntohl(n >> 32);
	h |= tmp << 32;

	return h;
}

static inline void ntoh128(const uint128_t *src, uint128_t *dst)
{
	bswap_128(src, dst);
}

static inline void btoh128(const uint128_t *src, uint128_t *dst)
{
	memcpy(dst, src, sizeof(uint128_t));
}

#endif

#define hton64(x)     ntoh64(x)
#define hton128(x, y) ntoh128(x, y)
#define htob128(x, y) btoh128(x, y)

#ifdef __cplusplus
}
#endif

#endif /* __BLUETOOTH_H */
