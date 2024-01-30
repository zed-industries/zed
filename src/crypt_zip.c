/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * crypt_zip.c: Zip encryption support.
 */
#include "vim.h"

#if defined(FEAT_CRYPT) || defined(PROTO)
/*
 * Optional encryption support.
 * Mohsin Ahmed, mosh@sasi.com, 98-09-24
 * Based on zip/crypt sources.
 *
 * NOTE FOR USA: Since 2000 exporting this code from the USA is allowed to
 * most countries.  There are a few exceptions, but that still should not be a
 * problem since this code was originally created in Europe and India.
 */

// Need a type that should be 32 bits. 64 also works but wastes space.
typedef unsigned int u32_T;	// int is at least 32 bits

// The state of encryption, referenced by cryptstate_T.
typedef struct {
    u32_T keys[3];
} zip_state_T;


static u32_T crc_32_table[256];

/*
 * Fill the CRC table, if not done already.
 */
    static void
make_crc_tab(void)
{
    u32_T	s, t, v;
    static int	done = FALSE;

    if (done)
	return;
    for (t = 0; t < 256; t++)
    {
	v = t;
	for (s = 0; s < 8; s++)
	    v = (v >> 1) ^ ((v & 1) * (u32_T)0xedb88320L);
	crc_32_table[t] = v;
    }
    done = TRUE;
}

#define CRC32(c, b) (crc_32_table[((int)(c) ^ (b)) & 0xff] ^ ((c) >> 8))

/*
 * Return the next byte in the pseudo-random sequence.
 */
#define DECRYPT_BYTE_ZIP(keys, t) \
{ \
    short_u temp = (short_u)keys[2] | 2; \
    t = (int)(((unsigned)(temp * (temp ^ 1U)) >> 8) & 0xff); \
}

/*
 * Update the encryption keys with the next byte of plain text.
 */
#define UPDATE_KEYS_ZIP(keys, c) do { \
    keys[0] = CRC32(keys[0], (c)); \
    keys[1] += keys[0] & 0xff; \
    keys[1] = keys[1] * 134775813L + 1; \
    keys[2] = CRC32(keys[2], (int)(keys[1] >> 24)); \
} while (0)

/*
 * Initialize for encryption/decryption.
 */
    int
crypt_zip_init(
    cryptstate_T    *state,
    char_u	    *key,
    crypt_arg_T     *arg UNUSED)
{
    char_u	*p;
    zip_state_T	*zs;

    zs = ALLOC_ONE(zip_state_T);
    if (zs == NULL)
	return FAIL;
    state->method_state = zs;

    make_crc_tab();
    zs->keys[0] = 305419896L;
    zs->keys[1] = 591751049L;
    zs->keys[2] = 878082192L;
    for (p = key; *p != NUL; ++p)
	UPDATE_KEYS_ZIP(zs->keys, (int)*p);

    return OK;
}

/*
 * Encrypt "from[len]" into "to[len]".
 * "from" and "to" can be equal to encrypt in place.
 */
    void
crypt_zip_encode(
    cryptstate_T *state,
    char_u	*from,
    size_t	len,
    char_u	*to,
    int		last UNUSED)
{
    zip_state_T *zs = state->method_state;
    size_t	i;
    int		ztemp, t;

    for (i = 0; i < len; ++i)
    {
	ztemp = from[i];
	DECRYPT_BYTE_ZIP(zs->keys, t);
	UPDATE_KEYS_ZIP(zs->keys, ztemp);
	to[i] = t ^ ztemp;
    }
}

/*
 * Decrypt "from[len]" into "to[len]".
 */
    void
crypt_zip_decode(
    cryptstate_T *state,
    char_u	*from,
    size_t	len,
    char_u	*to,
    int		last UNUSED)
{
    zip_state_T *zs = state->method_state;
    size_t	i;
    short_u	temp;

    for (i = 0; i < len; ++i)
    {
	temp = (short_u)zs->keys[2] | 2;
	temp = (int)(((unsigned)(temp * (temp ^ 1U)) >> 8) & 0xff);
	UPDATE_KEYS_ZIP(zs->keys, to[i] = from[i] ^ temp);
    }
}

#endif // FEAT_CRYPT
