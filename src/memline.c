/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

// for debugging
// #define CHECK(c, s)	do { if (c) emsg((s)); } while (0)
#define CHECK(c, s)	do { /**/ } while (0)

/*
 * memline.c: Contains the functions for appending, deleting and changing the
 * text lines. The memfile functions are used to store the information in
 * blocks of memory, backed up by a file. The structure of the information is
 * a tree.  The root of the tree is a pointer block. The leaves of the tree
 * are data blocks. In between may be several layers of pointer blocks,
 * forming branches.
 *
 * Three types of blocks are used:
 * - Block nr 0 contains information for recovery
 * - Pointer blocks contain list of pointers to other blocks.
 * - Data blocks contain the actual text.
 *
 * Block nr 0 contains the block0 structure (see below).
 *
 * Block nr 1 is the first pointer block. It is the root of the tree.
 * Other pointer blocks are branches.
 *
 *  If a line is too big to fit in a single page, the block containing that
 *  line is made big enough to hold the line. It may span several pages.
 *  Otherwise all blocks are one page.
 *
 *  A data block that was filled when starting to edit a file and was not
 *  changed since then, can have a negative block number. This means that it
 *  has not yet been assigned a place in the file. When recovering, the lines
 *  in this data block can be read from the original file. When the block is
 *  changed (lines appended/deleted/changed) or when it is flushed it gets a
 *  positive number. Use mf_trans_del() to get the new number, before calling
 *  mf_get().
 */

#include "vim.h"

#ifndef UNIX		// it's in os_unix.h for Unix
# include <time.h>
#endif

#if defined(SASC) || defined(__amigaos4__)
# include <proto/dos.h>	    // for Open() and Close()
#endif

typedef struct block0		ZERO_BL;    // contents of the first block
typedef struct pointer_block	PTR_BL;	    // contents of a pointer block
typedef struct data_block	DATA_BL;    // contents of a data block
typedef struct pointer_entry	PTR_EN;	    // block/line-count pair

#define DATA_ID	       (('d' << 8) + 'a')   // data block id
#define PTR_ID	       (('p' << 8) + 't')   // pointer block id
#define BLOCK0_ID0     'b'		    // block 0 id 0
#define BLOCK0_ID1     '0'		    // block 0 id 1
#define BLOCK0_ID1_C0  'c'		    // block 0 id 1 'cm' 0
#define BLOCK0_ID1_C1  'C'		    // block 0 id 1 'cm' 1
#define BLOCK0_ID1_C2  'd'		    // block 0 id 1 'cm' 2
// BLOCK0_ID1_C3 and BLOCK0_ID1_C4 are for libsodium encryption.  However, for
// these the swapfile is disabled, thus they will not be used.  Added for
// consistency anyway.
#define BLOCK0_ID1_C3  'S'		    // block 0 id 1 'cm' 3
#define BLOCK0_ID1_C4  's'		    // block 0 id 1 'cm' 4

#if defined(FEAT_CRYPT)
static int id1_codes[] = {
    BLOCK0_ID1_C0,  // CRYPT_M_ZIP
    BLOCK0_ID1_C1,  // CRYPT_M_BF
    BLOCK0_ID1_C2,  // CRYPT_M_BF2
    BLOCK0_ID1_C3,  // CRYPT_M_SOD  - Unused!
    BLOCK0_ID1_C4,  // CRYPT_M_SOD2  - Unused!
};
#endif

/*
 * pointer to a block, used in a pointer block
 */
struct pointer_entry
{
    blocknr_T	pe_bnum;	// block number
    linenr_T	pe_line_count;	// number of lines in this branch
    linenr_T	pe_old_lnum;	// lnum for this block (for recovery)
    int		pe_page_count;	// number of pages in block pe_bnum
};

/*
 * A pointer block contains a list of branches in the tree.
 */
struct pointer_block
{
    short_u	pb_id;		// ID for pointer block: PTR_ID
    short_u	pb_count;	// number of pointers in this block
    short_u	pb_count_max;	// maximum value for pb_count
    PTR_EN	pb_pointer[1];	// list of pointers to blocks (actually longer)
				// followed by empty space until end of page
};

// Value for pb_count_max.
#define PB_COUNT_MAX(mfp) (short_u)(((mfp)->mf_page_size - offsetof(PTR_BL, pb_pointer)) / sizeof(PTR_EN))

/*
 * A data block is a leaf in the tree.
 *
 * The text of the lines is at the end of the block. The text of the first line
 * in the block is put at the end, the text of the second line in front of it,
 * etc. Thus the order of the lines is the opposite of the line number.
 */
struct data_block
{
    short_u	db_id;		// ID for data block: DATA_ID
    unsigned	db_free;	// free space available
    unsigned	db_txt_start;	// byte where text starts
    unsigned	db_txt_end;	// byte just after data block
    linenr_T	db_line_count;	// number of lines in this block
    unsigned	db_index[1];	// index for start of line (actually bigger)
				// followed by empty space up to db_txt_start
				// followed by the text in the lines until
				// end of page
};

/*
 * The low bits of db_index hold the actual index. The topmost bit is
 * used for the global command to be able to mark a line.
 * This method is not clean, but otherwise there would be at least one extra
 * byte used for each line.
 * The mark has to be in this place to keep it with the correct line when other
 * lines are inserted or deleted.
 */
#define DB_MARKED	((unsigned)1 << ((sizeof(unsigned) * 8) - 1))
#define DB_INDEX_MASK	(~DB_MARKED)

#define INDEX_SIZE  (sizeof(unsigned))	    // size of one db_index entry
#define HEADER_SIZE (offsetof(DATA_BL, db_index))  // size of data block header

#define B0_FNAME_SIZE_ORG	900	// what it was in older versions
#define B0_FNAME_SIZE_NOCRYPT	898	// 2 bytes used for other things
#define B0_FNAME_SIZE_CRYPT	890	// 10 bytes used for other things
#define B0_UNAME_SIZE		40
#define B0_HNAME_SIZE		40
/*
 * Restrict the numbers to 32 bits, otherwise most compilers will complain.
 * This won't detect a 64 bit machine that only swaps a byte in the top 32
 * bits, but that is crazy anyway.
 */
#define B0_MAGIC_LONG	0x30313233L
#define B0_MAGIC_INT	0x20212223L
#define B0_MAGIC_SHORT	0x10111213L
#define B0_MAGIC_CHAR	0x55

/*
 * Block zero holds all info about the swap file.
 *
 * NOTE: DEFINITION OF BLOCK 0 SHOULD NOT CHANGE! It would make all existing
 * swap files unusable!
 *
 * If size of block0 changes anyway, adjust MIN_SWAP_PAGE_SIZE in vim.h!!
 *
 * This block is built up of single bytes, to make it portable across
 * different machines. b0_magic_* is used to check the byte order and size of
 * variables, because the rest of the swap file is not portable.
 */
struct block0
{
    char_u	b0_id[2];	// id for block 0: BLOCK0_ID0 and BLOCK0_ID1,
				// BLOCK0_ID1_C0, BLOCK0_ID1_C1, etc.
    char_u	b0_version[10];	// Vim version string
    char_u	b0_page_size[4];// number of bytes per page
    char_u	b0_mtime[4];	// last modification time of file
    char_u	b0_ino[4];	// inode of b0_fname
    char_u	b0_pid[4];	// process id of creator (or 0)
    char_u	b0_uname[B0_UNAME_SIZE]; // name of user (uid if no name)
    char_u	b0_hname[B0_HNAME_SIZE]; // host name (if it has a name)
    char_u	b0_fname[B0_FNAME_SIZE_ORG]; // name of file being edited
    long	b0_magic_long;	// check for byte order of long
    int		b0_magic_int;	// check for byte order of int
    short	b0_magic_short;	// check for byte order of short
    char_u	b0_magic_char;	// check for last char
};

/*
 * Note: b0_dirty and b0_flags are put at the end of the file name.  For very
 * long file names in older versions of Vim they are invalid.
 * The 'fileencoding' comes before b0_flags, with a NUL in front.  But only
 * when there is room, for very long file names it's omitted.
 */
#define B0_DIRTY	0x55
#define b0_dirty	b0_fname[B0_FNAME_SIZE_ORG - 1]

/*
 * The b0_flags field is new in Vim 7.0.
 */
#define b0_flags	b0_fname[B0_FNAME_SIZE_ORG - 2]

/*
 * Crypt seed goes here, 8 bytes.  New in Vim 7.3.
 * Without encryption these bytes may be used for 'fenc'.
 */
#define b0_seed		b0_fname[B0_FNAME_SIZE_ORG - 2 - MF_SEED_LEN]

// The lowest two bits contain the fileformat.  Zero means it's not set
// (compatible with Vim 6.x), otherwise it's EOL_UNIX + 1, EOL_DOS + 1 or
// EOL_MAC + 1.
#define B0_FF_MASK	3

// Swap file is in directory of edited file.  Used to find the file from
// different mount points.
#define B0_SAME_DIR	4

// The 'fileencoding' is at the end of b0_fname[], with a NUL in front of it.
// When empty there is only the NUL.
#define B0_HAS_FENC	8

#define STACK_INCR	5	// nr of entries added to ml_stack at a time

/*
 * The line number where the first mark may be is remembered.
 * If it is 0 there are no marks at all.
 * (always used for the current buffer only, no buffer change possible while
 * executing a global command).
 */
static linenr_T	lowest_marked = 0;

/*
 * arguments for ml_find_line()
 */
#define ML_DELETE	0x11	    // delete line
#define ML_INSERT	0x12	    // insert line
#define ML_FIND		0x13	    // just find the line
#define ML_FLUSH	0x02	    // flush locked block
#define ML_SIMPLE(x)	((x) & 0x10)  // DEL, INS or FIND

// argument for ml_upd_block0()
typedef enum {
      UB_FNAME = 0	// update timestamp and filename
    , UB_SAME_DIR       // update the B0_SAME_DIR flag
    , UB_CRYPT		// update crypt key
} upd_block0_T;

#ifdef FEAT_CRYPT
static void ml_set_b0_crypt(buf_T *buf, ZERO_BL *b0p);
#endif
static void ml_upd_block0(buf_T *buf, upd_block0_T what);
static void set_b0_fname(ZERO_BL *, buf_T *buf);
static void set_b0_dir_flag(ZERO_BL *b0p, buf_T *buf);
static void add_b0_fenc(ZERO_BL *b0p, buf_T *buf);
static time_t swapfile_info(char_u *);
static int recov_file_names(char_u **, char_u *, int prepend_dot);
static char_u *findswapname(buf_T *, char_u **, char_u *);
static void ml_flush_line(buf_T *);
static bhdr_T *ml_new_data(memfile_T *, int, int);
static bhdr_T *ml_new_ptr(memfile_T *);
static bhdr_T *ml_find_line(buf_T *, linenr_T, int);
static int ml_add_stack(buf_T *);
static void ml_lineadd(buf_T *, int);
static int b0_magic_wrong(ZERO_BL *);
#ifdef CHECK_INODE
static int fnamecmp_ino(char_u *, char_u *, long);
#endif
static void long_to_char(long, char_u *);
static long char_to_long(char_u *);
#ifdef FEAT_CRYPT
static cryptstate_T *ml_crypt_prepare(memfile_T *mfp, off_T offset, int reading);
#endif
#ifdef FEAT_BYTEOFF
static void ml_updatechunk(buf_T *buf, long line, long len, int updtype);
#endif

/*
 * Open a new memline for "buf".
 *
 * Return FAIL for failure, OK otherwise.
 */
    int
ml_open(buf_T *buf)
{
    memfile_T	*mfp;
    bhdr_T	*hp = NULL;
    ZERO_BL	*b0p;
    PTR_BL	*pp;
    DATA_BL	*dp;

    /*
     * init fields in memline struct
     */
    buf->b_ml.ml_stack_size = 0; // no stack yet
    buf->b_ml.ml_stack = NULL;	// no stack yet
    buf->b_ml.ml_stack_top = 0;	// nothing in the stack
    buf->b_ml.ml_locked = NULL;	// no cached block
    buf->b_ml.ml_line_lnum = 0;	// no cached line
#ifdef FEAT_BYTEOFF
    buf->b_ml.ml_chunksize = NULL;
    buf->b_ml.ml_usedchunks = 0;
#endif

    if (cmdmod.cmod_flags & CMOD_NOSWAPFILE)
	buf->b_p_swf = FALSE;

    /*
     * When 'updatecount' is non-zero swap file may be opened later.
     */
    if (p_uc && buf->b_p_swf)
	buf->b_may_swap = TRUE;
    else
	buf->b_may_swap = FALSE;

    /*
     * Open the memfile.  No swap file is created yet.
     */
    mfp = mf_open(NULL, 0);
    if (mfp == NULL)
	goto error;

    buf->b_ml.ml_mfp = mfp;
#ifdef FEAT_CRYPT
    mfp->mf_buffer = buf;
#endif
    buf->b_ml.ml_flags = ML_EMPTY;
    buf->b_ml.ml_line_count = 1;

/*
 * fill block0 struct and write page 0
 */
    if ((hp = mf_new(mfp, FALSE, 1)) == NULL)
	goto error;
    if (hp->bh_bnum != 0)
    {
	iemsg(e_didnt_get_block_nr_zero);
	goto error;
    }
    b0p = (ZERO_BL *)(hp->bh_data);

    b0p->b0_id[0] = BLOCK0_ID0;
    b0p->b0_id[1] = BLOCK0_ID1;
    b0p->b0_magic_long = (long)B0_MAGIC_LONG;
    b0p->b0_magic_int = (int)B0_MAGIC_INT;
    b0p->b0_magic_short = (short)B0_MAGIC_SHORT;
    b0p->b0_magic_char = B0_MAGIC_CHAR;
    mch_memmove(b0p->b0_version, "VIM ", 4);
    STRNCPY(b0p->b0_version + 4, Version, 6);
    long_to_char((long)mfp->mf_page_size, b0p->b0_page_size);

#ifdef FEAT_SPELL
    if (!buf->b_spell)
#endif
    {
	b0p->b0_dirty = buf->b_changed ? B0_DIRTY : 0;
	b0p->b0_flags = get_fileformat(buf) + 1;
	set_b0_fname(b0p, buf);
	(void)get_user_name(b0p->b0_uname, B0_UNAME_SIZE);
	b0p->b0_uname[B0_UNAME_SIZE - 1] = NUL;
	mch_get_host_name(b0p->b0_hname, B0_HNAME_SIZE);
	b0p->b0_hname[B0_HNAME_SIZE - 1] = NUL;
	long_to_char(mch_get_pid(), b0p->b0_pid);
#ifdef FEAT_CRYPT
	ml_set_b0_crypt(buf, b0p);
#endif
    }

    /*
     * Always sync block number 0 to disk, so we can check the file name in
     * the swap file in findswapname(). Don't do this for a help files or
     * a spell buffer though.
     * Only works when there's a swapfile, otherwise it's done when the file
     * is created.
     */
    mf_put(mfp, hp, TRUE, FALSE);
    if (!buf->b_help && !B_SPELL(buf))
	(void)mf_sync(mfp, 0);

    /*
     * Fill in root pointer block and write page 1.
     */
    if ((hp = ml_new_ptr(mfp)) == NULL)
	goto error;
    if (hp->bh_bnum != 1)
    {
	iemsg(e_didnt_get_block_nr_one);
	goto error;
    }
    pp = (PTR_BL *)(hp->bh_data);
    pp->pb_count = 1;
    pp->pb_pointer[0].pe_bnum = 2;
    pp->pb_pointer[0].pe_page_count = 1;
    pp->pb_pointer[0].pe_old_lnum = 1;
    pp->pb_pointer[0].pe_line_count = 1;    // line count after insertion
    mf_put(mfp, hp, TRUE, FALSE);

    /*
     * Allocate first data block and create an empty line 1.
     */
    if ((hp = ml_new_data(mfp, FALSE, 1)) == NULL)
	goto error;
    if (hp->bh_bnum != 2)
    {
	iemsg(e_didnt_get_block_nr_two);
	goto error;
    }

    dp = (DATA_BL *)(hp->bh_data);
    dp->db_index[0] = --dp->db_txt_start;	// at end of block
    dp->db_free -= 1 + INDEX_SIZE;
    dp->db_line_count = 1;
    *((char_u *)dp + dp->db_txt_start) = NUL;	// empty line

    return OK;

error:
    if (mfp != NULL)
    {
	if (hp)
	    mf_put(mfp, hp, FALSE, FALSE);
	mf_close(mfp, TRUE);	    // will also free(mfp->mf_fname)
    }
    buf->b_ml.ml_mfp = NULL;
    return FAIL;
}

#if defined(FEAT_CRYPT) || defined(PROTO)
/*
 * Swapfile encryption is not supported by XChaCha20.  If this crypt method is
 * used then disable the swapfile, to avoid plain text being written to disk,
 * and return TRUE.
 * Otherwise return FALSE.
 */
    static int
crypt_may_close_swapfile(buf_T *buf, char_u *key, int method)
{
    if (crypt_method_is_sodium(method) && *key != NUL)
    {
	mf_close_file(buf, TRUE);
	buf->b_p_swf = FALSE;
	return TRUE;
    }
    return FALSE;
}

/*
 * Prepare encryption for "buf" for the current key and method.
 */
    static void
ml_set_mfp_crypt(buf_T *buf)
{
    if (*buf->b_p_key == NUL)
	return;

    int method_nr = crypt_get_method_nr(buf);

    if (method_nr > CRYPT_M_ZIP && method_nr < CRYPT_M_SOD)
    {
	// Generate a seed and store it in the memfile.
	sha2_seed(buf->b_ml.ml_mfp->mf_seed, MF_SEED_LEN, NULL, 0);
    }
# ifdef FEAT_SODIUM
    else if (crypt_method_is_sodium(method_nr))
	crypt_sodium_randombytes_buf(buf->b_ml.ml_mfp->mf_seed, MF_SEED_LEN);
# endif
}

/*
 * Prepare encryption for "buf" with block 0 "b0p".
 * Note: should not be called with libsodium encryption, since xchacha20 does
 * not support swapfile encryption.
 */
    static void
ml_set_b0_crypt(buf_T *buf, ZERO_BL *b0p)
{
    if (*buf->b_p_key == NUL)
	b0p->b0_id[1] = BLOCK0_ID1;
    else
    {
	int method_nr = crypt_get_method_nr(buf);

	b0p->b0_id[1] = id1_codes[method_nr];
	if (method_nr > CRYPT_M_ZIP && method_nr < CRYPT_M_SOD)
	{
	    // Generate a seed and store it in block 0 and in the memfile.
	    sha2_seed(&b0p->b0_seed, MF_SEED_LEN, NULL, 0);
	    mch_memmove(buf->b_ml.ml_mfp->mf_seed, &b0p->b0_seed, MF_SEED_LEN);
	}
    }
}

/*
 * Called after the crypt key or 'cryptmethod' was changed for "buf".
 * Will apply this to the swapfile.
 * "old_key" is the previous key.  It is equal to buf->b_p_key when
 * 'cryptmethod' is changed.
 * "old_cm" is the previous 'cryptmethod'.  It is equal to the current
 * 'cryptmethod' when 'key' is changed.
 */
    void
ml_set_crypt_key(
    buf_T	*buf,
    char_u	*old_key,
    char_u	*old_cm)
{
    memfile_T	*mfp = buf->b_ml.ml_mfp;
    bhdr_T	*hp;
    int		page_count;
    int		idx;
    long	error;
    infoptr_T	*ip;
    PTR_BL	*pp;
    DATA_BL	*dp;
    blocknr_T	bnum;
    int		top;
    int		old_method;

    if (mfp == NULL || mfp->mf_fd < 0)
	return;  // no memfile yet, nothing to do
    old_method = crypt_method_nr_from_name(old_cm);

#ifdef FEAT_CRYPT
    if (crypt_may_close_swapfile(buf, buf->b_p_key, crypt_get_method_nr(buf)))
	return;
#endif

    // First make sure the swapfile is in a consistent state, using the old
    // key and method.
    {
	char_u *new_key = buf->b_p_key;
	char_u *new_buf_cm = buf->b_p_cm;

	buf->b_p_key = old_key;
	buf->b_p_cm = old_cm;
	ml_preserve(buf, FALSE);
	buf->b_p_key = new_key;
	buf->b_p_cm = new_buf_cm;
    }

    // Set the key, method and seed to be used for reading, these must be the
    // old values.
    mfp->mf_old_key = old_key;
    mfp->mf_old_cm = old_method;
    if (old_method > 0 && *old_key != NUL)
	mch_memmove(mfp->mf_old_seed, mfp->mf_seed, MF_SEED_LEN);

    // Update block 0 with the crypt flag and may set a new seed.
    ml_upd_block0(buf, UB_CRYPT);

    if (mfp->mf_infile_count > 2)
    {
	/*
	 * Need to read back all data blocks from disk, decrypt them with the
	 * old key/method and mark them to be written. The algorithm is
	 * similar to what happens in ml_recover(), but we skip negative block
	 * numbers.
	 */
	ml_flush_line(buf);		    // flush buffered line
	(void)ml_find_line(buf, (linenr_T)0, ML_FLUSH); // flush locked block

	hp = NULL;
	bnum = 1;		// start with block 1
	page_count = 1;		// which is 1 page
	idx = 0;		// start with first index in block 1
	error = 0;
	buf->b_ml.ml_stack_top = 0;
	VIM_CLEAR(buf->b_ml.ml_stack);
	buf->b_ml.ml_stack_size = 0;	// no stack yet

	for ( ; !got_int; line_breakcheck())
	{
	    if (hp != NULL)
		mf_put(mfp, hp, FALSE, FALSE);	// release previous block

	    // get the block (pointer or data)
	    if ((hp = mf_get(mfp, bnum, page_count)) == NULL)
	    {
		if (bnum == 1)
		    break;
		++error;
	    }
	    else
	    {
		pp = (PTR_BL *)(hp->bh_data);
		if (pp->pb_id == PTR_ID)	// it is a pointer block
		{
		    if (pp->pb_count == 0)
		    {
			// empty block?
			++error;
		    }
		    else if (idx < (int)pp->pb_count)	// go a block deeper
		    {
			if (pp->pb_pointer[idx].pe_bnum < 0)
			{
			    // Skip data block with negative block number.
			    // Should not happen, because of the ml_preserve()
			    // above. Get same block again for next index.
			    ++idx;
			    continue;
			}

			// going one block deeper in the tree, new entry in
			// stack
			if ((top = ml_add_stack(buf)) < 0)
			{
			    ++error;
			    break;		    // out of memory
			}
			ip = &(buf->b_ml.ml_stack[top]);
			ip->ip_bnum = bnum;
			ip->ip_index = idx;

			bnum = pp->pb_pointer[idx].pe_bnum;
			page_count = pp->pb_pointer[idx].pe_page_count;
			idx = 0;
			continue;
		    }
		}
		else	    // not a pointer block
		{
		    dp = (DATA_BL *)(hp->bh_data);
		    if (dp->db_id != DATA_ID)	// block id wrong
			++error;
		    else
		    {
			// It is a data block, need to write it back to disk.
			mf_put(mfp, hp, TRUE, FALSE);
			hp = NULL;
		    }
		}
	    }

	    if (buf->b_ml.ml_stack_top == 0)	// finished
		break;

	    // go one block up in the tree
	    ip = &(buf->b_ml.ml_stack[--(buf->b_ml.ml_stack_top)]);
	    bnum = ip->ip_bnum;
	    idx = ip->ip_index + 1;	    // go to next index
	    page_count = 1;
	}
	if (hp != NULL)
	    mf_put(mfp, hp, FALSE, FALSE);  // release previous block

	if (error > 0)
	    emsg(_(e_error_while_updating_swap_file_crypt));
    }

    mfp->mf_old_key = NULL;
}
#endif

/*
 * ml_setname() is called when the file name of "buf" has been changed.
 * It may rename the swap file.
 */
    void
ml_setname(buf_T *buf)
{
    int		success = FALSE;
    memfile_T	*mfp;
    char_u	*fname;
    char_u	*dirp;
#if defined(MSWIN)
    char_u	*p;
#endif

    mfp = buf->b_ml.ml_mfp;
    if (mfp->mf_fd < 0)		    // there is no swap file yet
    {
	/*
	 * When 'updatecount' is 0 and 'noswapfile' there is no swap file.
	 * For help files we will make a swap file now.
	 */
	if (p_uc != 0 && (cmdmod.cmod_flags & CMOD_NOSWAPFILE) == 0)
	    ml_open_file(buf);	    // create a swap file
	return;
    }

    /*
     * Try all directories in the 'directory' option.
     */
    dirp = p_dir;
    for (;;)
    {
	if (*dirp == NUL)	    // tried all directories, fail
	    break;
	fname = findswapname(buf, &dirp, mfp->mf_fname);
						    // alloc's fname
	if (dirp == NULL)	    // out of memory
	    break;
	if (fname == NULL)	    // no file name found for this dir
	    continue;

#if defined(MSWIN)
	/*
	 * Set full pathname for swap file now, because a ":!cd dir" may
	 * change directory without us knowing it.
	 */
	p = FullName_save(fname, FALSE);
	vim_free(fname);
	fname = p;
	if (fname == NULL)
	    continue;
#endif
	// if the file name is the same we don't have to do anything
	if (fnamecmp(fname, mfp->mf_fname) == 0)
	{
	    vim_free(fname);
	    success = TRUE;
	    break;
	}
	// need to close the swap file before renaming
	if (mfp->mf_fd >= 0)
	{
	    close(mfp->mf_fd);
	    mfp->mf_fd = -1;
	}

	// try to rename the swap file
	if (vim_rename(mfp->mf_fname, fname) == 0)
	{
	    success = TRUE;
	    vim_free(mfp->mf_fname);
	    mfp->mf_fname = fname;
	    vim_free(mfp->mf_ffname);
#if defined(MSWIN)
	    mfp->mf_ffname = NULL;  // mf_fname is full pathname already
#else
	    mf_set_ffname(mfp);
#endif
	    ml_upd_block0(buf, UB_SAME_DIR);
	    break;
	}
	vim_free(fname);	    // this fname didn't work, try another
    }

    if (mfp->mf_fd == -1)	    // need to (re)open the swap file
    {
	mfp->mf_fd = mch_open((char *)mfp->mf_fname, O_RDWR | O_EXTRA, 0);
	if (mfp->mf_fd < 0)
	{
	    // could not (re)open the swap file, what can we do????
	    emsg(_(e_oops_lost_the_swap_file));
	    return;
	}
#ifdef HAVE_FD_CLOEXEC
	{
	    int fdflags = fcntl(mfp->mf_fd, F_GETFD);
	    if (fdflags >= 0 && (fdflags & FD_CLOEXEC) == 0)
		(void)fcntl(mfp->mf_fd, F_SETFD, fdflags | FD_CLOEXEC);
	}
#endif
    }
    if (!success)
	emsg(_(e_could_not_rename_swap_file));
}

/*
 * Open a file for the memfile for all buffers that are not readonly or have
 * been modified.
 * Used when 'updatecount' changes from zero to non-zero.
 */
    void
ml_open_files(void)
{
    buf_T	*buf;

    FOR_ALL_BUFFERS(buf)
	if (!buf->b_p_ro || buf->b_changed)
	    ml_open_file(buf);
}

/*
 * Open a swap file for an existing memfile, if there is no swap file yet.
 * If we are unable to find a file name, mf_fname will be NULL
 * and the memfile will be in memory only (no recovery possible).
 */
    void
ml_open_file(buf_T *buf)
{
    memfile_T	*mfp;
    char_u	*fname;
    char_u	*dirp;

    mfp = buf->b_ml.ml_mfp;
    if (mfp == NULL || mfp->mf_fd >= 0 || !buf->b_p_swf
				      || (cmdmod.cmod_flags & CMOD_NOSWAPFILE))
	return;		// nothing to do

#ifdef FEAT_SPELL
    // For a spell buffer use a temp file name.
    if (buf->b_spell)
    {
	fname = vim_tempname('s', FALSE);
	if (fname != NULL)
	    (void)mf_open_file(mfp, fname);	// consumes fname!
	buf->b_may_swap = FALSE;
	return;
    }
#endif

    /*
     * Try all directories in 'directory' option.
     */
    dirp = p_dir;
    for (;;)
    {
	if (*dirp == NUL)
	    break;
	// There is a small chance that between choosing the swap file name
	// and creating it, another Vim creates the file.  In that case the
	// creation will fail and we will use another directory.
	fname = findswapname(buf, &dirp, NULL); // allocates fname
	if (dirp == NULL)
	    break;  // out of memory
	if (fname == NULL)
	    continue;
	if (mf_open_file(mfp, fname) == OK)	// consumes fname!
	{
	    // don't sync yet in ml_sync_all()
	    mfp->mf_dirty = MF_DIRTY_YES_NOSYNC;
#if defined(MSWIN)
	    /*
	     * set full pathname for swap file now, because a ":!cd dir" may
	     * change directory without us knowing it.
	     */
	    mf_fullname(mfp);
#endif
	    ml_upd_block0(buf, UB_SAME_DIR);

	    // Flush block zero, so others can read it
	    if (mf_sync(mfp, MFS_ZERO) == OK)
	    {
		// Mark all blocks that should be in the swapfile as dirty.
		// Needed for when the 'swapfile' option was reset, so that
		// the swap file was deleted, and then on again.
		mf_set_dirty(mfp);
		break;
	    }
	    // Writing block 0 failed: close the file and try another dir
	    mf_close_file(buf, FALSE);
	}
    }

    if (*p_dir != NUL && mfp->mf_fname == NULL)
    {
	need_wait_return = TRUE;	// call wait_return() later
	++no_wait_return;
	(void)semsg(_(e_unable_to_open_swap_file_for_str_recovery_impossible),
		    buf_spname(buf) != NULL ? buf_spname(buf) : buf->b_fname);
	--no_wait_return;
    }

    // don't try to open a swap file again
    buf->b_may_swap = FALSE;
}

/*
 * If still need to create a swap file, and starting to edit a not-readonly
 * file, or reading into an existing buffer, create a swap file now.
 */
    void
check_need_swap(
    int	    newfile)		// reading file into new buffer
{
    int old_msg_silent = msg_silent; // might be reset by an E325 message

    if (curbuf->b_may_swap && (!curbuf->b_p_ro || !newfile))
	ml_open_file(curbuf);
    msg_silent = old_msg_silent;
}

/*
 * Close memline for buffer 'buf'.
 * If 'del_file' is TRUE, delete the swap file
 */
    void
ml_close(buf_T *buf, int del_file)
{
    if (buf->b_ml.ml_mfp == NULL)		// not open
	return;
    mf_close(buf->b_ml.ml_mfp, del_file);	// close the .swp file
    if (buf->b_ml.ml_line_lnum != 0
		      && (buf->b_ml.ml_flags & (ML_LINE_DIRTY | ML_ALLOCATED)))
	vim_free(buf->b_ml.ml_line_ptr);
    vim_free(buf->b_ml.ml_stack);
#ifdef FEAT_BYTEOFF
    VIM_CLEAR(buf->b_ml.ml_chunksize);
#endif
    buf->b_ml.ml_mfp = NULL;

    // Reset the "recovered" flag, give the ATTENTION prompt the next time
    // this buffer is loaded.
    buf->b_flags &= ~BF_RECOVERED;
}

/*
 * Close all existing memlines and memfiles.
 * Only used when exiting.
 * When 'del_file' is TRUE, delete the memfiles.
 * But don't delete files that were ":preserve"d when we are POSIX compatible.
 */
    void
ml_close_all(int del_file)
{
    buf_T	*buf;

    FOR_ALL_BUFFERS(buf)
	ml_close(buf, del_file && ((buf->b_flags & BF_PRESERVED) == 0
				 || vim_strchr(p_cpo, CPO_PRESERVE) == NULL));
#ifdef FEAT_SPELL
    spell_delete_wordlist();	// delete the internal wordlist
#endif
#ifdef TEMPDIRNAMES
    vim_deltempdir();		// delete created temp directory
#endif
}

/*
 * Close all memfiles for not modified buffers.
 * Only use just before exiting!
 */
    void
ml_close_notmod(void)
{
    buf_T	*buf;

    FOR_ALL_BUFFERS(buf)
	if (!bufIsChanged(buf))
	    ml_close(buf, TRUE);    // close all not-modified buffers
}

/*
 * Update the timestamp in the .swp file.
 * Used when the file has been written.
 */
    void
ml_timestamp(buf_T *buf)
{
    ml_upd_block0(buf, UB_FNAME);
}

/*
 * Return FAIL when the ID of "b0p" is wrong.
 */
    static int
ml_check_b0_id(ZERO_BL *b0p)
{
    if (b0p->b0_id[0] != BLOCK0_ID0
	    || (b0p->b0_id[1] != BLOCK0_ID1
		&& b0p->b0_id[1] != BLOCK0_ID1_C0
		&& b0p->b0_id[1] != BLOCK0_ID1_C1
		&& b0p->b0_id[1] != BLOCK0_ID1_C2
		&& b0p->b0_id[1] != BLOCK0_ID1_C3
		&& b0p->b0_id[1] != BLOCK0_ID1_C4)
	    )
	return FAIL;
    return OK;
}

/*
 * Update the timestamp or the B0_SAME_DIR flag of the .swp file.
 */
    static void
ml_upd_block0(buf_T *buf, upd_block0_T what)
{
    memfile_T	*mfp;
    bhdr_T	*hp;
    ZERO_BL	*b0p;

    mfp = buf->b_ml.ml_mfp;
    if (mfp == NULL)
	return;
    hp = mf_get(mfp, (blocknr_T)0, 1);
    if (hp == NULL)
    {
#ifdef FEAT_CRYPT
	// Possibly update the seed in the memfile before there is a block0.
	if (what == UB_CRYPT)
	    ml_set_mfp_crypt(buf);
#endif
	return;
    }

    b0p = (ZERO_BL *)(hp->bh_data);
    if (ml_check_b0_id(b0p) == FAIL)
	iemsg(e_ml_upd_block0_didnt_get_block_zero);
    else
    {
	if (what == UB_FNAME)
	    set_b0_fname(b0p, buf);
#ifdef FEAT_CRYPT
	else if (what == UB_CRYPT)
	    ml_set_b0_crypt(buf, b0p);
#endif
	else // what == UB_SAME_DIR
	    set_b0_dir_flag(b0p, buf);
    }
    mf_put(mfp, hp, TRUE, FALSE);
}

/*
 * Write file name and timestamp into block 0 of a swap file.
 * Also set buf->b_mtime.
 * Don't use NameBuff[]!!!
 */
    static void
set_b0_fname(ZERO_BL *b0p, buf_T *buf)
{
    stat_T	st;

    if (buf->b_ffname == NULL)
	b0p->b0_fname[0] = NUL;
    else
    {
#if defined(MSWIN) || defined(AMIGA)
	// Systems that cannot translate "~user" back into a path: copy the
	// file name unmodified.  Do use slashes instead of backslashes for
	// portability.
	vim_strncpy(b0p->b0_fname, buf->b_ffname, B0_FNAME_SIZE_CRYPT - 1);
# ifdef BACKSLASH_IN_FILENAME
	forward_slash(b0p->b0_fname);
# endif
#else
	size_t	flen, ulen;
	char_u	uname[B0_UNAME_SIZE];

	/*
	 * For a file under the home directory of the current user, we try to
	 * replace the home directory path with "~user". This helps when
	 * editing the same file on different machines over a network.
	 * First replace home dir path with "~/" with home_replace().
	 * Then insert the user name to get "~user/".
	 */
	home_replace(NULL, buf->b_ffname, b0p->b0_fname,
						   B0_FNAME_SIZE_CRYPT, TRUE);
	if (b0p->b0_fname[0] == '~')
	{
	    flen = STRLEN(b0p->b0_fname);
	    // If there is no user name or it is too long, don't use "~/"
	    if (get_user_name(uname, B0_UNAME_SIZE) == FAIL
		   || (ulen = STRLEN(uname)) + flen > B0_FNAME_SIZE_CRYPT - 1)
		vim_strncpy(b0p->b0_fname, buf->b_ffname,
						     B0_FNAME_SIZE_CRYPT - 1);
	    else
	    {
		mch_memmove(b0p->b0_fname + ulen + 1, b0p->b0_fname + 1, flen);
		mch_memmove(b0p->b0_fname + 1, uname, ulen);
	    }
	}
#endif
	if (mch_stat((char *)buf->b_ffname, &st) >= 0)
	{
	    long_to_char((long)st.st_mtime, b0p->b0_mtime);
#ifdef CHECK_INODE
	    long_to_char((long)st.st_ino, b0p->b0_ino);
#endif
	    buf_store_time(buf, &st, buf->b_ffname);
	    buf->b_mtime_read = buf->b_mtime;
	    buf->b_mtime_read_ns = buf->b_mtime_ns;
	}
	else
	{
	    long_to_char(0L, b0p->b0_mtime);
#ifdef CHECK_INODE
	    long_to_char(0L, b0p->b0_ino);
#endif
	    buf->b_mtime = 0;
	    buf->b_mtime_ns = 0;
	    buf->b_mtime_read = 0;
	    buf->b_mtime_read_ns = 0;
	    buf->b_orig_size = 0;
	    buf->b_orig_mode = 0;
	}
    }

    // Also add the 'fileencoding' if there is room.
    add_b0_fenc(b0p, curbuf);
}

/*
 * Update the B0_SAME_DIR flag of the swap file.  It's set if the file and the
 * swapfile for "buf" are in the same directory.
 * This is fail safe: if we are not sure the directories are equal the flag is
 * not set.
 */
    static void
set_b0_dir_flag(ZERO_BL *b0p, buf_T *buf)
{
    if (same_directory(buf->b_ml.ml_mfp->mf_fname, buf->b_ffname))
	b0p->b0_flags |= B0_SAME_DIR;
    else
	b0p->b0_flags &= ~B0_SAME_DIR;
}

/*
 * When there is room, add the 'fileencoding' to block zero.
 */
    static void
add_b0_fenc(
    ZERO_BL	*b0p,
    buf_T	*buf)
{
    int		n;
    int		size = B0_FNAME_SIZE_NOCRYPT;

#ifdef FEAT_CRYPT
    // Without encryption use the same offset as in Vim 7.2 to be compatible.
    // With encryption it's OK to move elsewhere, the swap file is not
    // compatible anyway.
    if (*buf->b_p_key != NUL)
	size = B0_FNAME_SIZE_CRYPT;
#endif

    n = (int)STRLEN(buf->b_p_fenc);
    if ((int)STRLEN(b0p->b0_fname) + n + 1 > size)
	b0p->b0_flags &= ~B0_HAS_FENC;
    else
    {
	mch_memmove((char *)b0p->b0_fname + size - n,
					    (char *)buf->b_p_fenc, (size_t)n);
	*(b0p->b0_fname + size - n - 1) = NUL;
	b0p->b0_flags |= B0_HAS_FENC;
    }
}

#if defined(HAVE_SYS_SYSINFO_H) && defined(HAVE_SYSINFO_UPTIME)
# include <sys/sysinfo.h>
#endif

#if defined(UNIX) || defined(MSWIN)
/*
 * Return TRUE if the process with number "b0p->b0_pid" is still running.
 * "swap_fname" is the name of the swap file, if it's from before a reboot then
 * the result is FALSE;
 */
    static int
swapfile_process_running(ZERO_BL *b0p, char_u *swap_fname UNUSED)
{
#if defined(HAVE_SYSINFO) && defined(HAVE_SYSINFO_UPTIME)
    stat_T	    st;
    struct sysinfo  sinfo;

    // If the system rebooted after when the swap file was written then the
    // process can't be running now.
    if (mch_stat((char *)swap_fname, &st) != -1
	    && sysinfo(&sinfo) == 0
	    && st.st_mtime < time(NULL) - (
#  ifdef FEAT_EVAL
		override_sysinfo_uptime >= 0 ? override_sysinfo_uptime :
#  endif
		sinfo.uptime))
	return FALSE;
# endif
    return mch_process_running(char_to_long(b0p->b0_pid));
}
#endif

/*
 * Try to recover curbuf from the .swp file.
 * If "checkext" is TRUE, check the extension and detect whether it is
 * a swap file.
 */
    void
ml_recover(int checkext)
{
    buf_T	*buf = NULL;
    memfile_T	*mfp = NULL;
    char_u	*fname;
    char_u	*fname_used = NULL;
    bhdr_T	*hp = NULL;
    ZERO_BL	*b0p;
    int		b0_ff;
    char_u	*b0_fenc = NULL;
#ifdef FEAT_CRYPT
    int		b0_cm = -1;
#endif
    PTR_BL	*pp;
    DATA_BL	*dp;
    infoptr_T	*ip;
    blocknr_T	bnum;
    int		page_count;
    stat_T	org_stat, swp_stat;
    int		len;
    int		directly;
    linenr_T	lnum;
    char_u	*p;
    int		i;
    long	error;
    int		cannot_open;
    linenr_T	line_count;
    int		has_error;
    int		idx;
    int		top;
    int		txt_start;
    off_T	size;
    int		called_from_main;
    int		serious_error = TRUE;
    long	mtime;
    int		attr;
    int		orig_file_status = NOTDONE;

    recoverymode = TRUE;
    called_from_main = (curbuf->b_ml.ml_mfp == NULL);
    attr = HL_ATTR(HLF_E);

    /*
     * If the file name ends in ".s[a-w][a-z]" we assume this is the swap file.
     * Otherwise a search is done to find the swap file(s).
     */
    fname = curbuf->b_fname;
    if (fname == NULL)		    // When there is no file name
	fname = (char_u *)"";
    len = (int)STRLEN(fname);
    if (checkext && len >= 4 &&
#if defined(VMS)
	    STRNICMP(fname + len - 4, "_s", 2)
#else
	    STRNICMP(fname + len - 4, ".s", 2)
#endif
						== 0
		&& vim_strchr((char_u *)"abcdefghijklmnopqrstuvw",
					   TOLOWER_ASC(fname[len - 2])) != NULL
		&& ASCII_ISALPHA(fname[len - 1]))
    {
	directly = TRUE;
	fname_used = vim_strsave(fname); // make a copy for mf_open()
    }
    else
    {
	directly = FALSE;

	// count the number of matching swap files
	len = recover_names(fname, FALSE, NULL, 0, NULL);
	if (len == 0)		    // no swap files found
	{
	    semsg(_(e_no_swap_file_found_for_str), fname);
	    goto theend;
	}
	if (len == 1)		    // one swap file found, use it
	    i = 1;
	else			    // several swap files found, choose
	{
	    // list the names of the swap files
	    (void)recover_names(fname, TRUE, NULL, 0, NULL);
	    msg_putchar('\n');
	    msg_puts(_("Enter number of swap file to use (0 to quit): "));
	    i = get_number(FALSE, NULL);
	    if (i < 1 || i > len)
		goto theend;
	}
	// get the swap file name that will be used
	(void)recover_names(fname, FALSE, NULL, i, &fname_used);
    }
    if (fname_used == NULL)
	goto theend;			// out of memory

    // When called from main() still need to initialize storage structure
    if (called_from_main && ml_open(curbuf) == FAIL)
	getout(1);

    /*
     * Allocate a buffer structure for the swap file that is used for recovery.
     * Only the memline and crypt information in it are really used.
     */
    buf = ALLOC_ONE(buf_T);
    if (buf == NULL)
	goto theend;

    /*
     * init fields in memline struct
     */
    buf->b_ml.ml_stack_size = 0;	// no stack yet
    buf->b_ml.ml_stack = NULL;		// no stack yet
    buf->b_ml.ml_stack_top = 0;		// nothing in the stack
    buf->b_ml.ml_line_lnum = 0;		// no cached line
    buf->b_ml.ml_locked = NULL;		// no locked block
    buf->b_ml.ml_flags = 0;
#ifdef FEAT_CRYPT
    buf->b_p_key = empty_option;
    buf->b_p_cm = empty_option;
#endif

    /*
     * open the memfile from the old swap file
     */
    p = vim_strsave(fname_used); // save "fname_used" for the message:
				 // mf_open() will consume "fname_used"!
    mfp = mf_open(fname_used, O_RDONLY);
    fname_used = p;
    if (mfp == NULL || mfp->mf_fd < 0)
    {
	if (fname_used != NULL)
	    semsg(_(e_cannot_open_str), fname_used);
	goto theend;
    }
    buf->b_ml.ml_mfp = mfp;
#ifdef FEAT_CRYPT
    mfp->mf_buffer = buf;
#endif

    /*
     * The page size set in mf_open() might be different from the page size
     * used in the swap file, we must get it from block 0.  But to read block
     * 0 we need a page size.  Use the minimal size for block 0 here, it will
     * be set to the real value below.
     */
    mfp->mf_page_size = MIN_SWAP_PAGE_SIZE;

    /*
     * try to read block 0
     */
    if ((hp = mf_get(mfp, (blocknr_T)0, 1)) == NULL)
    {
	msg_start();
	msg_puts_attr(_("Unable to read block 0 from "), attr | MSG_HIST);
	msg_outtrans_attr(mfp->mf_fname, attr | MSG_HIST);
	msg_puts_attr(_("\nMaybe no changes were made or Vim did not update the swap file."),
		attr | MSG_HIST);
	msg_end();
	goto theend;
    }
    b0p = (ZERO_BL *)(hp->bh_data);
    if (STRNCMP(b0p->b0_version, "VIM 3.0", 7) == 0)
    {
	msg_start();
	msg_outtrans_attr(mfp->mf_fname, MSG_HIST);
	msg_puts_attr(_(" cannot be used with this version of Vim.\n"),
								    MSG_HIST);
	msg_puts_attr(_("Use Vim version 3.0.\n"), MSG_HIST);
	msg_end();
	goto theend;
    }
    if (ml_check_b0_id(b0p) == FAIL)
    {
	semsg(_(e_str_does_not_look_like_vim_swap_file), mfp->mf_fname);
	goto theend;
    }
    if (b0_magic_wrong(b0p))
    {
	msg_start();
	msg_outtrans_attr(mfp->mf_fname, attr | MSG_HIST);
#if defined(MSWIN)
	if (STRNCMP(b0p->b0_hname, "PC ", 3) == 0)
	    msg_puts_attr(_(" cannot be used with this version of Vim.\n"),
							     attr | MSG_HIST);
	else
#endif
	    msg_puts_attr(_(" cannot be used on this computer.\n"),
							     attr | MSG_HIST);
	msg_puts_attr(_("The file was created on "), attr | MSG_HIST);
	// avoid going past the end of a corrupted hostname
	b0p->b0_fname[0] = NUL;
	msg_puts_attr((char *)b0p->b0_hname, attr | MSG_HIST);
	msg_puts_attr(_(",\nor the file has been damaged."), attr | MSG_HIST);
	msg_end();
	goto theend;
    }

#ifdef FEAT_CRYPT
    for (i = 0; i < (int)ARRAY_LENGTH(id1_codes); ++i)
	if (id1_codes[i] == b0p->b0_id[1])
	    b0_cm = i;
    if (b0_cm > 0)
	mch_memmove(mfp->mf_seed, &b0p->b0_seed, MF_SEED_LEN);
    crypt_set_cm_option(buf, b0_cm < 0 ? 0 : b0_cm);
#else
    if (b0p->b0_id[1] != BLOCK0_ID1)
    {
	semsg(_(e_str_is_encrypted_and_this_version_of_vim_does_not_support_encryption), mfp->mf_fname);
	goto theend;
    }
#endif

    /*
     * If we guessed the wrong page size, we have to recalculate the
     * highest block number in the file.
     */
    if (mfp->mf_page_size != (unsigned)char_to_long(b0p->b0_page_size))
    {
	unsigned previous_page_size = mfp->mf_page_size;

	mf_new_page_size(mfp, (unsigned)char_to_long(b0p->b0_page_size));
	if (mfp->mf_page_size < previous_page_size)
	{
	    msg_start();
	    msg_outtrans_attr(mfp->mf_fname, attr | MSG_HIST);
	    msg_puts_attr(_(" has been damaged (page size is smaller than minimum value).\n"),
			attr | MSG_HIST);
	    msg_end();
	    goto theend;
	}
	if ((size = vim_lseek(mfp->mf_fd, (off_T)0L, SEEK_END)) <= 0)
	    mfp->mf_blocknr_max = 0;	    // no file or empty file
	else
	    mfp->mf_blocknr_max = (blocknr_T)(size / mfp->mf_page_size);
	mfp->mf_infile_count = mfp->mf_blocknr_max;

	// need to reallocate the memory used to store the data
	p = alloc(mfp->mf_page_size);
	if (p == NULL)
	    goto theend;
	mch_memmove(p, hp->bh_data, previous_page_size);
	vim_free(hp->bh_data);
	hp->bh_data = p;
	b0p = (ZERO_BL *)(hp->bh_data);
    }

    /*
     * If .swp file name given directly, use name from swap file for buffer.
     */
    if (directly)
    {
	expand_env(b0p->b0_fname, NameBuff, MAXPATHL);
	if (setfname(curbuf, NameBuff, NULL, TRUE) == FAIL)
	    goto theend;
    }

    home_replace(NULL, mfp->mf_fname, NameBuff, MAXPATHL, TRUE);
    smsg(_("Using swap file \"%s\""), NameBuff);

    if (buf_spname(curbuf) != NULL)
	vim_strncpy(NameBuff, buf_spname(curbuf), MAXPATHL - 1);
    else
	home_replace(NULL, curbuf->b_ffname, NameBuff, MAXPATHL, TRUE);
    smsg(_("Original file \"%s\""), NameBuff);
    msg_putchar('\n');

    /*
     * check date of swap file and original file
     */
    mtime = char_to_long(b0p->b0_mtime);
    if (curbuf->b_ffname != NULL
	    && mch_stat((char *)curbuf->b_ffname, &org_stat) != -1
	    && ((mch_stat((char *)mfp->mf_fname, &swp_stat) != -1
		    && org_stat.st_mtime > swp_stat.st_mtime)
		|| org_stat.st_mtime != mtime))
	emsg(_(e_warning_original_file_may_have_been_changed));
    out_flush();

    // Get the 'fileformat' and 'fileencoding' from block zero.
    b0_ff = (b0p->b0_flags & B0_FF_MASK);
    if (b0p->b0_flags & B0_HAS_FENC)
    {
	int fnsize = B0_FNAME_SIZE_NOCRYPT;

#ifdef FEAT_CRYPT
	// Use the same size as in add_b0_fenc().
	if (b0p->b0_id[1] != BLOCK0_ID1)
	    fnsize = B0_FNAME_SIZE_CRYPT;
#endif
	for (p = b0p->b0_fname + fnsize; p > b0p->b0_fname && p[-1] != NUL; --p)
	    ;
	b0_fenc = vim_strnsave(p, b0p->b0_fname + fnsize - p);
    }

    mf_put(mfp, hp, FALSE, FALSE);	// release block 0
    hp = NULL;

    /*
     * Now that we are sure that the file is going to be recovered, clear the
     * contents of the current buffer.
     */
    while (!(curbuf->b_ml.ml_flags & ML_EMPTY))
	ml_delete((linenr_T)1);

    /*
     * Try reading the original file to obtain the values of 'fileformat',
     * 'fileencoding', etc.  Ignore errors.  The text itself is not used.
     * When the file is encrypted the user is asked to enter the key.
     */
    if (curbuf->b_ffname != NULL)
	orig_file_status = readfile(curbuf->b_ffname, NULL, (linenr_T)0,
			      (linenr_T)0, (linenr_T)MAXLNUM, NULL, READ_NEW);

#ifdef FEAT_CRYPT
    if (b0_cm >= 0)
    {
	// Need to ask the user for the crypt key.  If this fails we continue
	// without a key, will probably get garbage text.
	if (*curbuf->b_p_key != NUL)
	{
	    smsg(_("Swap file is encrypted: \"%s\""), fname_used);
	    msg_puts(_("\nIf you entered a new crypt key but did not write the text file,"));
	    msg_puts(_("\nenter the new crypt key."));
	    msg_puts(_("\nIf you wrote the text file after changing the crypt key press enter"));
	    msg_puts(_("\nto use the same key for text file and swap file"));
	}
	else
	    smsg(_(need_key_msg), fname_used);
	buf->b_p_key = crypt_get_key(FALSE, FALSE);
	if (buf->b_p_key == NULL)
	    buf->b_p_key = curbuf->b_p_key;
	else if (*buf->b_p_key == NUL)
	{
	    vim_free(buf->b_p_key);
	    buf->b_p_key = curbuf->b_p_key;
	}
	if (buf->b_p_key == NULL)
	    buf->b_p_key = empty_option;
    }
#endif

    // Use the 'fileformat' and 'fileencoding' as stored in the swap file.
    if (b0_ff != 0)
	set_fileformat(b0_ff - 1, OPT_LOCAL);
    if (b0_fenc != NULL)
    {
	set_option_value_give_err((char_u *)"fenc", 0L, b0_fenc, OPT_LOCAL);
	vim_free(b0_fenc);
    }
    unchanged(curbuf, TRUE, TRUE);

    bnum = 1;		// start with block 1
    page_count = 1;	// which is 1 page
    lnum = 0;		// append after line 0 in curbuf
    line_count = 0;
    idx = 0;		// start with first index in block 1
    error = 0;
    buf->b_ml.ml_stack_top = 0;
    buf->b_ml.ml_stack = NULL;
    buf->b_ml.ml_stack_size = 0;	// no stack yet

    if (curbuf->b_ffname == NULL)
	cannot_open = TRUE;
    else
	cannot_open = FALSE;

    serious_error = FALSE;
    for ( ; !got_int; line_breakcheck())
    {
	if (hp != NULL)
	    mf_put(mfp, hp, FALSE, FALSE);	// release previous block

	/*
	 * get block
	 */
	if ((hp = mf_get(mfp, bnum, page_count)) == NULL)
	{
	    if (bnum == 1)
	    {
		semsg(_(e_unable_to_read_block_one_from_str), mfp->mf_fname);
		goto theend;
	    }
	    ++error;
	    ml_append(lnum++, (char_u *)_("???MANY LINES MISSING"),
							    (colnr_T)0, TRUE);
	}
	else		// there is a block
	{
	    pp = (PTR_BL *)(hp->bh_data);
	    if (pp->pb_id == PTR_ID)		// it is a pointer block
	    {
		int ptr_block_error = FALSE;
		if (pp->pb_count_max != PB_COUNT_MAX(mfp))
		{
		    ptr_block_error = TRUE;
		    pp->pb_count_max = PB_COUNT_MAX(mfp);
		}
		if (pp->pb_count > pp->pb_count_max)
		{
		    ptr_block_error = TRUE;
		    pp->pb_count = pp->pb_count_max;
		}
		if (ptr_block_error)
		    emsg(_(e_warning_pointer_block_corrupted));

		// check line count when using pointer block first time
		if (idx == 0 && line_count != 0)
		{
		    for (i = 0; i < (int)pp->pb_count; ++i)
			line_count -= pp->pb_pointer[i].pe_line_count;
		    if (line_count != 0)
		    {
			++error;
			ml_append(lnum++, (char_u *)_("???LINE COUNT WRONG"),
							    (colnr_T)0, TRUE);
		    }
		}

		if (pp->pb_count == 0)
		{
		    ml_append(lnum++, (char_u *)_("???EMPTY BLOCK"),
							    (colnr_T)0, TRUE);
		    ++error;
		}
		else if (idx < (int)pp->pb_count)	// go a block deeper
		{
		    if (pp->pb_pointer[idx].pe_bnum < 0)
		    {
			/*
			 * Data block with negative block number.
			 * Try to read lines from the original file.
			 * This is slow, but it works.
			 */
			if (!cannot_open)
			{
			    line_count = pp->pb_pointer[idx].pe_line_count;
			    if (readfile(curbuf->b_ffname, NULL, lnum,
					pp->pb_pointer[idx].pe_old_lnum - 1,
					line_count, NULL, 0) != OK)
				cannot_open = TRUE;
			    else
				lnum += line_count;
			}
			if (cannot_open)
			{
			    ++error;
			    ml_append(lnum++, (char_u *)_("???LINES MISSING"),
							    (colnr_T)0, TRUE);
			}
			++idx;	    // get same block again for next index
			continue;
		    }

		    /*
		     * going one block deeper in the tree
		     */
		    if ((top = ml_add_stack(buf)) < 0)	// new entry in stack
		    {
			++error;
			break;		    // out of memory
		    }
		    ip = &(buf->b_ml.ml_stack[top]);
		    ip->ip_bnum = bnum;
		    ip->ip_index = idx;

		    bnum = pp->pb_pointer[idx].pe_bnum;
		    line_count = pp->pb_pointer[idx].pe_line_count;
		    page_count = pp->pb_pointer[idx].pe_page_count;
		    idx = 0;
		    continue;
		}
	    }
	    else	    // not a pointer block
	    {
		dp = (DATA_BL *)(hp->bh_data);
		if (dp->db_id != DATA_ID)	// block id wrong
		{
		    if (bnum == 1)
		    {
			semsg(_(e_block_one_id_wrong_str_not_swp_file),
							       mfp->mf_fname);
			goto theend;
		    }
		    ++error;
		    ml_append(lnum++, (char_u *)_("???BLOCK MISSING"),
							    (colnr_T)0, TRUE);
		}
		else
		{
		    /*
		     * It is a data block.
		     * Append all the lines in this block.
		     */
		    has_error = FALSE;

		    // Check the length of the block.
		    // If wrong, use the length given in the pointer block.
		    if (page_count * mfp->mf_page_size != dp->db_txt_end)
		    {
			ml_append(lnum++, (char_u *)_("??? from here until ???END lines may be messed up"),
							    (colnr_T)0, TRUE);
			++error;
			has_error = TRUE;
			dp->db_txt_end = page_count * mfp->mf_page_size;
		    }

		    // Make sure there is a NUL at the end of the block so we
		    // don't go over the end when copying text.
		    *((char_u *)dp + dp->db_txt_end - 1) = NUL;

		    // Check the number of lines in the block.
		    // If wrong, use the count in the data block.
		    if (line_count != dp->db_line_count)
		    {
			ml_append(lnum++, (char_u *)_("??? from here until ???END lines may have been inserted/deleted"),
							    (colnr_T)0, TRUE);
			++error;
			has_error = TRUE;
		    }

		    int did_questions = FALSE;
		    for (i = 0; i < dp->db_line_count; ++i)
		    {
			if ((char_u *)&(dp->db_index[i])
					    >= (char_u *)dp + dp->db_txt_start)
			{
			    // line count must be wrong
			    ++error;
			    ml_append(lnum++,
				    (char_u *)_("??? lines may be missing"),
							     (colnr_T)0, TRUE);
			    break;
			}

			txt_start = (dp->db_index[i] & DB_INDEX_MASK);
			if (txt_start <= (int)HEADER_SIZE
					  || txt_start >= (int)dp->db_txt_end)
			{
			    ++error;
			    // avoid lots of lines with "???"
			    if (did_questions)
				continue;
			    did_questions = TRUE;
			    p = (char_u *)"???";
			}
			else
			{
			    did_questions = FALSE;
			    p = (char_u *)dp + txt_start;
			}
			ml_append(lnum++, p, (colnr_T)0, TRUE);
		    }
		    if (has_error)
			ml_append(lnum++, (char_u *)_("???END"),
							    (colnr_T)0, TRUE);
		}
	    }
	}

	if (buf->b_ml.ml_stack_top == 0)	// finished
	    break;

	/*
	 * go one block up in the tree
	 */
	ip = &(buf->b_ml.ml_stack[--(buf->b_ml.ml_stack_top)]);
	bnum = ip->ip_bnum;
	idx = ip->ip_index + 1;	    // go to next index
	page_count = 1;
    }

    /*
     * Compare the buffer contents with the original file.  When they differ
     * set the 'modified' flag.
     * Lines 1 - lnum are the new contents.
     * Lines lnum + 1 to ml_line_count are the original contents.
     * Line ml_line_count + 1 in the dummy empty line.
     */
    if (orig_file_status != OK || curbuf->b_ml.ml_line_count != lnum * 2 + 1)
    {
	// Recovering an empty file results in two lines and the first line is
	// empty.  Don't set the modified flag then.
	if (!(curbuf->b_ml.ml_line_count == 2 && *ml_get(1) == NUL))
	{
	    changed_internal();
	    ++CHANGEDTICK(curbuf);
	}
    }
    else
    {
	for (idx = 1; idx <= lnum; ++idx)
	{
	    // Need to copy one line, fetching the other one may flush it.
	    p = vim_strsave(ml_get(idx));
	    i = STRCMP(p, ml_get(idx + lnum));
	    vim_free(p);
	    if (i != 0)
	    {
		changed_internal();
		++CHANGEDTICK(curbuf);
		break;
	    }
	}
    }

    /*
     * Delete the lines from the original file and the dummy line from the
     * empty buffer.  These will now be after the last line in the buffer.
     */
    while (curbuf->b_ml.ml_line_count > lnum
				       && !(curbuf->b_ml.ml_flags & ML_EMPTY))
	ml_delete(curbuf->b_ml.ml_line_count);
    curbuf->b_flags |= BF_RECOVERED;
    check_cursor();

    recoverymode = FALSE;
    if (got_int)
	emsg(_(e_recovery_interrupted));
    else if (error)
    {
	++no_wait_return;
	msg(">>>>>>>>>>>>>");
	emsg(_(e_errors_detected_while_recovering_look_for_lines_starting_with_questions));
	--no_wait_return;
	msg(_("See \":help E312\" for more information."));
	msg(">>>>>>>>>>>>>");
    }
    else
    {
	if (curbuf->b_changed)
	{
	    msg(_("Recovery completed. You should check if everything is OK."));
	    msg_puts(_("\n(You might want to write out this file under another name\n"));
	    msg_puts(_("and run diff with the original file to check for changes)"));
	}
	else
	    msg(_("Recovery completed. Buffer contents equals file contents."));
	msg_puts(_("\nYou may want to delete the .swp file now."));
#if defined(UNIX) || defined(MSWIN)
	if (swapfile_process_running(b0p, fname_used))
	{
	    // Warn there could be an active Vim on the same file, the user may
	    // want to kill it.
	    msg_puts(_("\nNote: process STILL RUNNING: "));
	    msg_outnum(char_to_long(b0p->b0_pid));
	}
#endif
	msg_puts("\n\n");
	cmdline_row = msg_row;
    }
#ifdef FEAT_CRYPT
    if (*buf->b_p_key != NUL && STRCMP(curbuf->b_p_key, buf->b_p_key) != 0)
    {
	msg_puts(_("Using crypt key from swap file for the text file.\n"));
	set_option_value_give_err((char_u *)"key", 0L, buf->b_p_key, OPT_LOCAL);
    }
#endif
    redraw_curbuf_later(UPD_NOT_VALID);

theend:
    vim_free(fname_used);
    recoverymode = FALSE;
    if (mfp != NULL)
    {
	if (hp != NULL)
	    mf_put(mfp, hp, FALSE, FALSE);
	mf_close(mfp, FALSE);	    // will also vim_free(mfp->mf_fname)
    }
    if (buf != NULL)
    {
#ifdef FEAT_CRYPT
	if (buf->b_p_key != curbuf->b_p_key)
	    free_string_option(buf->b_p_key);
	free_string_option(buf->b_p_cm);
#endif
	vim_free(buf->b_ml.ml_stack);
	vim_free(buf);
    }
    if (serious_error && called_from_main)
	ml_close(curbuf, TRUE);
    else
    {
	apply_autocmds(EVENT_BUFREADPOST, NULL, curbuf->b_fname, FALSE, curbuf);
	apply_autocmds(EVENT_BUFWINENTER, NULL, curbuf->b_fname, FALSE, curbuf);
    }
}

/*
 * Find the names of swap files in current directory and the directory given
 * with the 'directory' option.
 *
 * Used to:
 * - list the swap files for "vim -r"
 * - count the number of swap files when recovering
 * - list the swap files when recovering
 * - list the swap files for swapfilelist()
 * - find the name of the n'th swap file when recovering
 */
    int
recover_names(
    char_u	*fname,		// base for swap file name
    int		do_list,	// when TRUE, list the swap file names
    list_T	*ret_list UNUSED, // when not NULL add file names to it
    int		nr,		// when non-zero, return nr'th swap file name
    char_u	**fname_out)	// result when "nr" > 0
{
    int		num_names;
    char_u	*(names[6]);
    char_u	*tail;
    char_u	*p;
    int		num_files;
    int		file_count = 0;
    char_u	**files;
    char_u	*dirp;
    char_u	*dir_name;
    char_u	*fname_res = NULL;
#ifdef HAVE_READLINK
    char_u	fname_buf[MAXPATHL];
#endif

    if (fname != NULL)
    {
#ifdef HAVE_READLINK
	// Expand symlink in the file name, because the swap file is created
	// with the actual file instead of with the symlink.
	if (resolve_symlink(fname, fname_buf) == OK)
	    fname_res = fname_buf;
	else
#endif
	    fname_res = fname;
    }

    if (do_list)
    {
	// use msg() to start the scrolling properly
	msg(_("Swap files found:"));
	msg_putchar('\n');
    }

    /*
     * Do the loop for every directory in 'directory'.
     * First allocate some memory to put the directory name in.
     */
    dir_name = alloc(STRLEN(p_dir) + 1);
    dirp = p_dir;
    while (dir_name != NULL && *dirp)
    {
	/*
	 * Isolate a directory name from *dirp and put it in dir_name (we know
	 * it is large enough, so use 31000 for length).
	 * Advance dirp to next directory name.
	 */
	(void)copy_option_part(&dirp, dir_name, 31000, ",");

	if (dir_name[0] == '.' && dir_name[1] == NUL)	// check current dir
	{
	    if (fname == NULL)
	    {
#ifdef VMS
		names[0] = vim_strsave((char_u *)"*_sw%");
#else
		names[0] = vim_strsave((char_u *)"*.sw?");
#endif
#if defined(UNIX) || defined(MSWIN)
		// For Unix names starting with a dot are special.  MS-Windows
		// supports this too, on some file systems.
		names[1] = vim_strsave((char_u *)".*.sw?");
		names[2] = vim_strsave((char_u *)".sw?");
		num_names = 3;
#else
# ifdef VMS
		names[1] = vim_strsave((char_u *)".*_sw%");
		num_names = 2;
# else
		num_names = 1;
# endif
#endif
	    }
	    else
		num_names = recov_file_names(names, fname_res, TRUE);
	}
	else			    // check directory dir_name
	{
	    if (fname == NULL)
	    {
#ifdef VMS
		names[0] = concat_fnames(dir_name, (char_u *)"*_sw%", TRUE);
#else
		names[0] = concat_fnames(dir_name, (char_u *)"*.sw?", TRUE);
#endif
#if defined(UNIX) || defined(MSWIN)
		// For Unix names starting with a dot are special.  MS-Windows
		// supports this too, on some file systems.
		names[1] = concat_fnames(dir_name, (char_u *)".*.sw?", TRUE);
		names[2] = concat_fnames(dir_name, (char_u *)".sw?", TRUE);
		num_names = 3;
#else
# ifdef VMS
		names[1] = concat_fnames(dir_name, (char_u *)".*_sw%", TRUE);
		num_names = 2;
# else
		num_names = 1;
# endif
#endif
	    }
	    else
	    {
#if defined(UNIX) || defined(MSWIN)
		int	len = (int)STRLEN(dir_name);

		p = dir_name + len;
		if (after_pathsep(dir_name, p) && len > 1 && p[-1] == p[-2])
		{
		    // Ends with '//', Use Full path for swap name
		    tail = make_percent_swname(dir_name, fname_res);
		}
		else
#endif
		{
		    tail = gettail(fname_res);
		    tail = concat_fnames(dir_name, tail, TRUE);
		}
		if (tail == NULL)
		    num_names = 0;
		else
		{
		    num_names = recov_file_names(names, tail, FALSE);
		    vim_free(tail);
		}
	    }
	}

	// check for out-of-memory
	for (int i = 0; i < num_names; ++i)
	{
	    if (names[i] == NULL)
	    {
		for (i = 0; i < num_names; ++i)
		    vim_free(names[i]);
		num_names = 0;
	    }
	}
	if (num_names == 0)
	    num_files = 0;
	else if (expand_wildcards(num_names, names, &num_files, &files,
			    EW_NOTENV|EW_KEEPALL|EW_FILE|EW_SILENT) == FAIL)
	    num_files = 0;

	/*
	 * When no swap file found, wildcard expansion might have failed (e.g.
	 * not able to execute the shell).
	 * Try finding a swap file by simply adding ".swp" to the file name.
	 */
	if (*dirp == NUL && file_count + num_files == 0 && fname != NULL)
	{
	    stat_T	    st;
	    char_u	    *swapname;

	    swapname = modname(fname_res,
#if defined(VMS)
			       (char_u *)"_swp", FALSE
#else
			       (char_u *)".swp", TRUE
#endif
			      );
	    if (swapname != NULL)
	    {
		if (mch_stat((char *)swapname, &st) != -1)    // It exists!
		{
		    files = ALLOC_ONE(char_u *);
		    if (files != NULL)
		    {
			files[0] = swapname;
			swapname = NULL;
			num_files = 1;
		    }
		}
		vim_free(swapname);
	    }
	}

	/*
	 * Remove swapfile name of the current buffer, it must be ignored.
	 * But keep it for swapfilelist().
	 */
	if (curbuf->b_ml.ml_mfp != NULL
			       && (p = curbuf->b_ml.ml_mfp->mf_fname) != NULL
			       && ret_list == NULL)
	{
	    for (int i = 0; i < num_files; ++i)
		// Do not expand wildcards, on windows would try to expand
		// "%tmp%" in "%tmp%file".
		if (fullpathcmp(p, files[i], TRUE, FALSE) & FPC_SAME)
		{
		    // Remove the name from files[i].  Move further entries
		    // down.  When the array becomes empty free it here, since
		    // FreeWild() won't be called below.
		    vim_free(files[i]);
		    if (--num_files == 0)
			vim_free(files);
		    else
			for ( ; i < num_files; ++i)
			    files[i] = files[i + 1];
		}
	}
	if (nr > 0)
	{
	    file_count += num_files;
	    if (nr <= file_count)
	    {
		*fname_out = vim_strsave(
				      files[nr - 1 + num_files - file_count]);
		dirp = (char_u *)"";		    // stop searching
	    }
	}
	else if (do_list)
	{
	    if (dir_name[0] == '.' && dir_name[1] == NUL)
	    {
		if (fname == NULL)
		    msg_puts(_("   In current directory:\n"));
		else
		    msg_puts(_("   Using specified name:\n"));
	    }
	    else
	    {
		msg_puts(_("   In directory "));
		msg_home_replace(dir_name);
		msg_puts(":\n");
	    }

	    if (num_files)
	    {
		for (int i = 0; i < num_files; ++i)
		{
		    // print the swap file name
		    msg_outnum((long)++file_count);
		    msg_puts(".    ");
		    msg_puts((char *)gettail(files[i]));
		    msg_putchar('\n');
		    (void)swapfile_info(files[i]);
		}
	    }
	    else
		msg_puts(_("      -- none --\n"));
	    out_flush();
	}
#ifdef FEAT_EVAL
	else if (ret_list != NULL)
	{
	    for (int i = 0; i < num_files; ++i)
	    {
		char_u *name = concat_fnames(dir_name, files[i], TRUE);
		if (name != NULL)
		{
		    list_append_string(ret_list, name, -1);
		    vim_free(name);
		}
	    }
	}
#endif
	else
	    file_count += num_files;

	for (int i = 0; i < num_names; ++i)
	    vim_free(names[i]);
	if (num_files > 0)
	    FreeWild(num_files, files);
    }
    vim_free(dir_name);
    return file_count;
}

#if defined(UNIX) || defined(MSWIN) || defined(PROTO)
/*
 * Need _very_ long file names.
 * Append the full path to name with path separators made into percent
 * signs, to "dir". An unnamed buffer is handled as "" (<currentdir>/"")
 * The last character in "dir" must be an extra slash or backslash, it is
 * removed.
 */
    char_u *
make_percent_swname(char_u *dir, char_u *name)
{
    char_u *d = NULL, *s, *f;

    f = fix_fname(name != NULL ? name : (char_u *)"");
    if (f == NULL)
	return NULL;

    s = alloc(STRLEN(f) + 1);
    if (s != NULL)
    {
	STRCPY(s, f);
	for (d = s; *d != NUL; MB_PTR_ADV(d))
	    if (vim_ispathsep(*d))
		*d = '%';

	dir[STRLEN(dir) - 1] = NUL;  // remove one trailing slash
	d = concat_fnames(dir, s, TRUE);
	vim_free(s);
    }
    vim_free(f);
    return d;
}
#endif

#if (defined(UNIX) || defined(VMS) || defined(MSWIN)) \
	&& (defined(FEAT_GUI_DIALOG) || defined(FEAT_CON_DIALOG))
# define HAVE_PROCESS_STILL_RUNNING
static int process_still_running;
#endif

#if defined(FEAT_EVAL) || defined(PROTO)
/*
 * Return information found in swapfile "fname" in dictionary "d".
 * This is used by the swapinfo() function.
 */
    void
get_b0_dict(char_u *fname, dict_T *d)
{
    int fd;
    struct block0 b0;

    if ((fd = mch_open((char *)fname, O_RDONLY | O_EXTRA, 0)) >= 0)
    {
	if (read_eintr(fd, &b0, sizeof(b0)) == sizeof(b0))
	{
	    if (ml_check_b0_id(&b0) == FAIL)
		dict_add_string(d, "error", (char_u *)"Not a swap file");
	    else if (b0_magic_wrong(&b0))
		dict_add_string(d, "error", (char_u *)"Magic number mismatch");
	    else
	    {
		// we have swap information
		dict_add_string_len(d, "version", b0.b0_version, 10);
		dict_add_string_len(d, "user", b0.b0_uname, B0_UNAME_SIZE);
		dict_add_string_len(d, "host", b0.b0_hname, B0_HNAME_SIZE);
		dict_add_string_len(d, "fname", b0.b0_fname, B0_FNAME_SIZE_ORG);

		dict_add_number(d, "pid", char_to_long(b0.b0_pid));
		dict_add_number(d, "mtime", char_to_long(b0.b0_mtime));
		dict_add_number(d, "dirty", b0.b0_dirty ? 1 : 0);
# ifdef CHECK_INODE
		dict_add_number(d, "inode", char_to_long(b0.b0_ino));
# endif
	    }
	}
	else
	    dict_add_string(d, "error", (char_u *)"Cannot read file");
	close(fd);
    }
    else
	dict_add_string(d, "error", (char_u *)"Cannot open file");
}
#endif

/*
 * Give information about an existing swap file.
 * Returns timestamp (0 when unknown).
 */
    static time_t
swapfile_info(char_u *fname)
{
    stat_T	    st;
    int		    fd;
    struct block0   b0;
#ifdef UNIX
    char_u	    uname[B0_UNAME_SIZE];
#endif

    // print the swap file date
    if (mch_stat((char *)fname, &st) != -1)
    {
#ifdef UNIX
	// print name of owner of the file
	if (mch_get_uname(st.st_uid, uname, B0_UNAME_SIZE) == OK)
	{
	    msg_puts(_("          owned by: "));
	    msg_outtrans(uname);
	    msg_puts(_("   dated: "));
	}
	else
#endif
	    msg_puts(_("             dated: "));
	msg_puts(get_ctime(st.st_mtime, TRUE));
    }
    else
	st.st_mtime = 0;

    /*
     * print the original file name
     */
    fd = mch_open((char *)fname, O_RDONLY | O_EXTRA, 0);
    if (fd >= 0)
    {
	if (read_eintr(fd, &b0, sizeof(b0)) == sizeof(b0))
	{
	    if (STRNCMP(b0.b0_version, "VIM 3.0", 7) == 0)
	    {
		msg_puts(_("         [from Vim version 3.0]"));
	    }
	    else if (ml_check_b0_id(&b0) == FAIL)
	    {
		msg_puts(_("         [does not look like a Vim swap file]"));
	    }
	    else
	    {
		msg_puts(_("         file name: "));
		if (b0.b0_fname[0] == NUL)
		    msg_puts(_("[No Name]"));
		else
		    msg_outtrans(b0.b0_fname);

		msg_puts(_("\n          modified: "));
		msg_puts(b0.b0_dirty ? _("YES") : _("no"));

		if (*(b0.b0_uname) != NUL)
		{
		    msg_puts(_("\n         user name: "));
		    msg_outtrans(b0.b0_uname);
		}

		if (*(b0.b0_hname) != NUL)
		{
		    if (*(b0.b0_uname) != NUL)
			msg_puts(_("   host name: "));
		    else
			msg_puts(_("\n         host name: "));
		    msg_outtrans(b0.b0_hname);
		}

		if (char_to_long(b0.b0_pid) != 0L)
		{
		    msg_puts(_("\n        process ID: "));
		    msg_outnum(char_to_long(b0.b0_pid));
#if defined(UNIX) || defined(MSWIN)
		    if (swapfile_process_running(&b0, fname))
		    {
			msg_puts(_(" (STILL RUNNING)"));
# ifdef HAVE_PROCESS_STILL_RUNNING
			process_still_running = TRUE;
# endif
		    }
#endif
		}

		if (b0_magic_wrong(&b0))
		{
#if defined(MSWIN)
		    if (STRNCMP(b0.b0_hname, "PC ", 3) == 0)
			msg_puts(_("\n         [not usable with this version of Vim]"));
		    else
#endif
			msg_puts(_("\n         [not usable on this computer]"));
		}
	    }
	}
	else
	    msg_puts(_("         [cannot be read]"));
	close(fd);
    }
    else
	msg_puts(_("         [cannot be opened]"));
    msg_putchar('\n');

    return st.st_mtime;
}

/*
 * Return TRUE if the swap file looks OK and there are no changes, thus it can
 * be safely deleted.
 */
    static int
swapfile_unchanged(char_u *fname)
{
    stat_T	    st;
    int		    fd;
    struct block0   b0;
    int		    ret = TRUE;

    // must be able to stat the swap file
    if (mch_stat((char *)fname, &st) == -1)
	return FALSE;

    // must be able to read the first block
    fd = mch_open((char *)fname, O_RDONLY | O_EXTRA, 0);
    if (fd < 0)
	return FALSE;
    if (read_eintr(fd, &b0, sizeof(b0)) != sizeof(b0))
    {
	close(fd);
	return FALSE;
    }

    // the ID and magic number must be correct
    if (ml_check_b0_id(&b0) == FAIL|| b0_magic_wrong(&b0))
	ret = FALSE;

    // must be unchanged
    if (b0.b0_dirty)
	ret = FALSE;

#if defined(UNIX) || defined(MSWIN)
    // Host name must be known and must equal the current host name, otherwise
    // comparing pid is meaningless.
    if (*(b0.b0_hname) == NUL)
    {
	ret = FALSE;
    }
    else
    {
	char_u	    hostname[B0_HNAME_SIZE];

	mch_get_host_name(hostname, B0_HNAME_SIZE);
	hostname[B0_HNAME_SIZE - 1] = NUL;
	b0.b0_hname[B0_HNAME_SIZE - 1] = NUL; // in case of corruption
	if (STRICMP(b0.b0_hname, hostname) != 0)
	    ret = FALSE;
    }

    // process must be known and not be running
    if (char_to_long(b0.b0_pid) == 0L || swapfile_process_running(&b0, fname))
	ret = FALSE;
#endif

    // We do not check the user, it should be irrelevant for whether the swap
    // file is still useful.

    close(fd);
    return ret;
}

    static int
recov_file_names(char_u **names, char_u *path, int prepend_dot)
{
    int		num_names;

    /*
     * (Win32 and Win64) never short names, but do prepend a dot.
     * (Not MS-DOS or Win32 or Win64) maybe short name, maybe not: Try both.
     * Only use the short name if it is different.
     */
    char_u	*p;
    int		i;
# ifndef MSWIN
    int	    shortname = curbuf->b_shortname;

    curbuf->b_shortname = FALSE;
# endif

    num_names = 0;

    /*
     * May also add the file name with a dot prepended, for swap file in same
     * dir as original file.
     */
    if (prepend_dot)
    {
	names[num_names] = modname(path, (char_u *)".sw?", TRUE);
	if (names[num_names] == NULL)
	    goto end;
	++num_names;
    }

    /*
     * Form the normal swap file name pattern by appending ".sw?".
     */
#ifdef VMS
    names[num_names] = concat_fnames(path, (char_u *)"_sw%", FALSE);
#else
    names[num_names] = concat_fnames(path, (char_u *)".sw?", FALSE);
#endif
    if (names[num_names] == NULL)
	goto end;
    if (num_names >= 1)	    // check if we have the same name twice
    {
	p = names[num_names - 1];
	i = (int)STRLEN(names[num_names - 1]) - (int)STRLEN(names[num_names]);
	if (i > 0)
	    p += i;	    // file name has been expanded to full path

	if (STRCMP(p, names[num_names]) != 0)
	    ++num_names;
	else
	    vim_free(names[num_names]);
    }
    else
	++num_names;

# ifndef MSWIN
    /*
     * Also try with 'shortname' set, in case the file is on a DOS filesystem.
     */
    curbuf->b_shortname = TRUE;
#ifdef VMS
    names[num_names] = modname(path, (char_u *)"_sw%", FALSE);
#else
    names[num_names] = modname(path, (char_u *)".sw?", FALSE);
#endif
    if (names[num_names] == NULL)
	goto end;

    /*
     * Remove the one from 'shortname', if it's the same as with 'noshortname'.
     */
    p = names[num_names];
    i = STRLEN(names[num_names]) - STRLEN(names[num_names - 1]);
    if (i > 0)
	p += i;		// file name has been expanded to full path
    if (STRCMP(names[num_names - 1], p) == 0)
	vim_free(names[num_names]);
    else
	++num_names;
# endif

end:
# ifndef MSWIN
    curbuf->b_shortname = shortname;
# endif

    return num_names;
}

/*
 * sync all memlines
 *
 * If 'check_file' is TRUE, check if original file exists and was not changed.
 * If 'check_char' is TRUE, stop syncing when character becomes available, but
 * always sync at least one block.
 */
    void
ml_sync_all(int check_file, int check_char)
{
    buf_T		*buf;
    stat_T		st;

    FOR_ALL_BUFFERS(buf)
    {
	if (buf->b_ml.ml_mfp == NULL
		|| buf->b_ml.ml_mfp->mf_fname == NULL
		|| buf->b_ml.ml_mfp->mf_fd < 0)
	    continue;			    // no file

#ifdef FEAT_CRYPT
	if (crypt_may_close_swapfile(buf, buf->b_p_key,
						     crypt_get_method_nr(buf)))
	    continue;
#endif

	ml_flush_line(buf);		    // flush buffered line
					    // flush locked block
	(void)ml_find_line(buf, (linenr_T)0, ML_FLUSH);
	if (bufIsChanged(buf) && check_file && mf_need_trans(buf->b_ml.ml_mfp)
						     && buf->b_ffname != NULL)
	{
	    /*
	     * If the original file does not exist anymore or has been changed
	     * call ml_preserve() to get rid of all negative numbered blocks.
	     */
	    if (mch_stat((char *)buf->b_ffname, &st) == -1
		    || st.st_mtime != buf->b_mtime_read
#ifdef ST_MTIM_NSEC
		    || st.ST_MTIM_NSEC != buf->b_mtime_read_ns
#endif
		    || st.st_size != buf->b_orig_size)
	    {
		ml_preserve(buf, FALSE);
		did_check_timestamps = FALSE;
		need_check_timestamps = TRUE;	// give message later
	    }
	}
	if (buf->b_ml.ml_mfp->mf_dirty == MF_DIRTY_YES)
	{
	    (void)mf_sync(buf->b_ml.ml_mfp, (check_char ? MFS_STOP : 0)
					| (bufIsChanged(buf) ? MFS_FLUSH : 0));
	    if (check_char && ui_char_avail())	// character available now
		break;
	}
    }
}

/*
 * sync one buffer, including negative blocks
 *
 * after this all the blocks are in the swap file
 *
 * Used for the :preserve command and when the original file has been
 * changed or deleted.
 *
 * when message is TRUE the success of preserving is reported
 */
    void
ml_preserve(buf_T *buf, int message)
{
    bhdr_T	*hp;
    linenr_T	lnum;
    memfile_T	*mfp = buf->b_ml.ml_mfp;
    int		status;
    int		got_int_save = got_int;

    if (mfp == NULL || mfp->mf_fname == NULL)
    {
	if (message)
	    emsg(_(e_cannot_preserve_there_is_no_swap_file));
	return;
    }
#ifdef FEAT_CRYPT
    if (crypt_may_close_swapfile(buf, buf->b_p_key, crypt_get_method_nr(buf)))
	return;
#endif

    // We only want to stop when interrupted here, not when interrupted
    // before.
    got_int = FALSE;

    ml_flush_line(buf);				    // flush buffered line
    (void)ml_find_line(buf, (linenr_T)0, ML_FLUSH); // flush locked block
    status = mf_sync(mfp, MFS_ALL | MFS_FLUSH);

    // stack is invalid after mf_sync(.., MFS_ALL)
    buf->b_ml.ml_stack_top = 0;

    /*
     * Some of the data blocks may have been changed from negative to
     * positive block number. In that case the pointer blocks need to be
     * updated.
     *
     * We don't know in which pointer block the references are, so we visit
     * all data blocks until there are no more translations to be done (or
     * we hit the end of the file, which can only happen in case a write fails,
     * e.g. when file system if full).
     * ml_find_line() does the work by translating the negative block numbers
     * when getting the first line of each data block.
     */
    if (mf_need_trans(mfp) && !got_int)
    {
	lnum = 1;
	while (mf_need_trans(mfp) && lnum <= buf->b_ml.ml_line_count)
	{
	    hp = ml_find_line(buf, lnum, ML_FIND);
	    if (hp == NULL)
	    {
		status = FAIL;
		goto theend;
	    }
	    CHECK(buf->b_ml.ml_locked_low != lnum, "low != lnum");
	    lnum = buf->b_ml.ml_locked_high + 1;
	}
	(void)ml_find_line(buf, (linenr_T)0, ML_FLUSH);	// flush locked block
	// sync the updated pointer blocks
	if (mf_sync(mfp, MFS_ALL | MFS_FLUSH) == FAIL)
	    status = FAIL;
	buf->b_ml.ml_stack_top = 0;	    // stack is invalid now
    }
theend:
    got_int |= got_int_save;

    if (message)
    {
	if (status == OK)
	    msg(_("File preserved"));
	else
	    emsg(_(e_preserve_failed));
    }
}

/*
 * NOTE: The pointer returned by the ml_get_*() functions only remains valid
 * until the next call!
 *  line1 = ml_get(1);
 *  line2 = ml_get(2);	// line1 is now invalid!
 * Make a copy of the line if necessary.
 */
/*
 * Return a pointer to a (read-only copy of a) line.
 *
 * On failure an error message is given and IObuff is returned (to avoid
 * having to check for error everywhere).
 */
    char_u  *
ml_get(linenr_T lnum)
{
    return ml_get_buf(curbuf, lnum, FALSE);
}

/*
 * Return pointer to position "pos".
 */
    char_u *
ml_get_pos(pos_T *pos)
{
    return (ml_get_buf(curbuf, pos->lnum, FALSE) + pos->col);
}

/*
 * Return pointer to cursor line.
 */
    char_u *
ml_get_curline(void)
{
    return ml_get_buf(curbuf, curwin->w_cursor.lnum, FALSE);
}

/*
 * Return pointer to cursor position.
 */
    char_u *
ml_get_cursor(void)
{
    return (ml_get_buf(curbuf, curwin->w_cursor.lnum, FALSE) +
							curwin->w_cursor.col);
}

/*
 * Return a pointer to a line in a specific buffer
 *
 * "will_change": if TRUE mark the buffer dirty (chars in the line will be
 * changed)
 */
    char_u  *
ml_get_buf(
    buf_T	*buf,
    linenr_T	lnum,
    int		will_change)		// line will be changed
{
    bhdr_T	*hp;
    DATA_BL	*dp;
    static int	recursive = 0;
    static char_u questions[4];

    if (lnum > buf->b_ml.ml_line_count)	// invalid line number
    {
	if (recursive == 0)
	{
	    // Avoid giving this message for a recursive call, may happen when
	    // the GUI redraws part of the text.
	    ++recursive;
	    siemsg(e_ml_get_invalid_lnum_nr, lnum);
	    --recursive;
	}
	ml_flush_line(buf);
errorret:
	STRCPY(questions, "???");
	buf->b_ml.ml_line_len = 4;
	buf->b_ml.ml_line_lnum = lnum;
	return questions;
    }
    if (lnum <= 0)			// pretend line 0 is line 1
	lnum = 1;

    if (buf->b_ml.ml_mfp == NULL)	// there are no lines
    {
	buf->b_ml.ml_line_len = 1;
	return (char_u *)"";
    }

    /*
     * See if it is the same line as requested last time.
     * Otherwise may need to flush last used line.
     * Don't use the last used line when 'swapfile' is reset, need to load all
     * blocks.
     */
    if (buf->b_ml.ml_line_lnum != lnum || mf_dont_release)
    {
	unsigned    start, end;
	colnr_T	    len;
	int	    idx;

	ml_flush_line(buf);

	/*
	 * Find the data block containing the line.
	 * This also fills the stack with the blocks from the root to the data
	 * block and releases any locked block.
	 */
	if ((hp = ml_find_line(buf, lnum, ML_FIND)) == NULL)
	{
	    if (recursive == 0)
	    {
		// Avoid giving this message for a recursive call, may happen
		// when the GUI redraws part of the text.
		++recursive;
		get_trans_bufname(buf);
		shorten_dir(NameBuff);
		siemsg(e_ml_get_cannot_find_line_nr_in_buffer_nr_str,
						  lnum, buf->b_fnum, NameBuff);
		--recursive;
	    }
	    goto errorret;
	}

	dp = (DATA_BL *)(hp->bh_data);

	idx = lnum - buf->b_ml.ml_locked_low;
	start = ((dp->db_index[idx]) & DB_INDEX_MASK);
	// The text ends where the previous line starts.  The first line ends
	// at the end of the block.
	if (idx == 0)
	    end = dp->db_txt_end;
	else
	    end = ((dp->db_index[idx - 1]) & DB_INDEX_MASK);
	len = end - start;

	buf->b_ml.ml_line_ptr = (char_u *)dp + start;
	buf->b_ml.ml_line_len = len;
	buf->b_ml.ml_line_lnum = lnum;
	buf->b_ml.ml_flags &= ~(ML_LINE_DIRTY | ML_ALLOCATED);
    }
    if (will_change)
    {
	buf->b_ml.ml_flags |= (ML_LOCKED_DIRTY | ML_LOCKED_POS);
#ifdef FEAT_EVAL
	if (ml_get_alloc_lines && (buf->b_ml.ml_flags & ML_ALLOCATED))
	    // can't make the change in the data block
	    buf->b_ml.ml_flags |= ML_LINE_DIRTY;
#endif
    }

#ifdef FEAT_EVAL
    if (ml_get_alloc_lines
		 && (buf->b_ml.ml_flags & (ML_LINE_DIRTY | ML_ALLOCATED)) == 0)
    {
	char_u *p = alloc(buf->b_ml.ml_line_len);

	// make sure the text is in allocated memory
	if (p != NULL)
	{
	    memmove(p, buf->b_ml.ml_line_ptr, buf->b_ml.ml_line_len);
	    buf->b_ml.ml_line_ptr = p;
	    buf->b_ml.ml_flags |= ML_ALLOCATED;
	    if (will_change)
		// can't make the change in the data block
		buf->b_ml.ml_flags |= ML_LINE_DIRTY;
	}
    }
#endif
    return buf->b_ml.ml_line_ptr;
}

/*
 * Check if a line that was just obtained by a call to ml_get
 * is in allocated memory.
 * This ignores ML_ALLOCATED to get the same behavior as without the test
 * override.
 */
    int
ml_line_alloced(void)
{
    return (curbuf->b_ml.ml_flags & ML_LINE_DIRTY);
}

#ifdef FEAT_PROP_POPUP
/*
 * Add text properties that continue from the previous line.
 */
    static void
add_text_props_for_append(
	    buf_T	*buf,
	    linenr_T	lnum,
	    char_u	**line,
	    int		*len,
	    char_u	**tofree)
{
    int		round;
    int		new_prop_count = 0;
    int		count;
    int		n;
    char_u	*props;
    int		new_len = 0;  // init for gcc
    char_u	*new_line = NULL;
    textprop_T	prop;

    // Make two rounds:
    // 1. calculate the extra space needed
    // 2. allocate the space and fill it
    for (round = 1; round <= 2; ++round)
    {
	if (round == 2)
	{
	    if (new_prop_count == 0)
		return;  // nothing to do
	    new_len = *len + new_prop_count * sizeof(textprop_T);
	    new_line = alloc(new_len);
	    if (new_line == NULL)
		return;
	    mch_memmove(new_line, *line, *len);
	    new_prop_count = 0;
	}

	// Get the line above to find any props that continue in the next
	// line.
	count = get_text_props(buf, lnum, &props, FALSE);
	for (n = 0; n < count; ++n)
	{
	    mch_memmove(&prop, props + n * sizeof(textprop_T),
							   sizeof(textprop_T));
	    if (prop.tp_flags & TP_FLAG_CONT_NEXT)
	    {
		if (round == 2)
		{
		    prop.tp_flags |= TP_FLAG_CONT_PREV;
		    prop.tp_col = 1;
		    prop.tp_len = *len;  // not exactly the right length
		    mch_memmove(new_line + *len + new_prop_count
			      * sizeof(textprop_T), &prop, sizeof(textprop_T));
		}
		++new_prop_count;
	    }
	}
    }
    *line = new_line;
    *tofree = new_line;
    *len = new_len;
}
#endif

    static int
ml_append_int(
    buf_T	*buf,
    linenr_T	lnum,		// append after this line (can be 0)
    char_u	*line_arg,	// text of the new line
    colnr_T	len_arg,	// length of line, including NUL, or 0
    int		flags)		// ML_APPEND_ flags
{
    char_u	*line = line_arg;
    colnr_T	len = len_arg;
    int		i;
    int		line_count;	// number of indexes in current block
    int		offset;
    int		from, to;
    int		space_needed;	// space needed for new line
    int		page_size;
    int		page_count;
    int		db_idx;		// index for lnum in data block
    bhdr_T	*hp;
    memfile_T	*mfp;
    DATA_BL	*dp;
    PTR_BL	*pp;
    infoptr_T	*ip;
#ifdef FEAT_PROP_POPUP
    char_u	*tofree = NULL;
# ifdef FEAT_BYTEOFF
    colnr_T	text_len = 0;	// text len with NUL without text properties
# endif
#endif
    int		ret = FAIL;

    if (lnum > buf->b_ml.ml_line_count || buf->b_ml.ml_mfp == NULL)
	return FAIL;  // lnum out of range

    if (lowest_marked && lowest_marked > lnum)
	lowest_marked = lnum + 1;

    if (len == 0)
    {
	len = (colnr_T)STRLEN(line) + 1;	// space needed for the text
#if defined(FEAT_PROP_POPUP) && defined(FEAT_BYTEOFF)
	text_len = len;
#endif
    }
#if defined(FEAT_PROP_POPUP) && defined(FEAT_BYTEOFF)
    else if (curbuf->b_has_textprop)
	// "len" may include text properties, get the length of the text.
	text_len = (colnr_T)STRLEN(line) + 1;
    else
	text_len = len;
#endif

#ifdef FEAT_PROP_POPUP
    if (curbuf->b_has_textprop && lnum > 0
			     && !(flags & (ML_APPEND_UNDO | ML_APPEND_NOPROP)))
	// Add text properties that continue from the previous line.
	add_text_props_for_append(buf, lnum, &line, &len, &tofree);
#endif

    space_needed = len + INDEX_SIZE;	// space needed for text + index

    mfp = buf->b_ml.ml_mfp;
    page_size = mfp->mf_page_size;

/*
 * find the data block containing the previous line
 * This also fills the stack with the blocks from the root to the data block
 * This also releases any locked block.
 */
    if ((hp = ml_find_line(buf, lnum == 0 ? (linenr_T)1 : lnum,
							  ML_INSERT)) == NULL)
	goto theend;

    buf->b_ml.ml_flags &= ~ML_EMPTY;

    if (lnum == 0)		// got line one instead, correct db_idx
	db_idx = -1;		// careful, it is negative!
    else
	db_idx = lnum - buf->b_ml.ml_locked_low;
		// get line count before the insertion
    line_count = buf->b_ml.ml_locked_high - buf->b_ml.ml_locked_low;

    dp = (DATA_BL *)(hp->bh_data);

/*
 * If
 * - there is not enough room in the current block
 * - appending to the last line in the block
 * - not appending to the last line in the file
 * insert in front of the next block.
 */
    if ((int)dp->db_free < space_needed && db_idx == line_count - 1
					    && lnum < buf->b_ml.ml_line_count)
    {
	/*
	 * Now that the line is not going to be inserted in the block that we
	 * expected, the line count has to be adjusted in the pointer blocks
	 * by using ml_locked_lineadd.
	 */
	--(buf->b_ml.ml_locked_lineadd);
	--(buf->b_ml.ml_locked_high);
	if ((hp = ml_find_line(buf, lnum + 1, ML_INSERT)) == NULL)
	    goto theend;

	db_idx = -1;		    // careful, it is negative!
		    // get line count before the insertion
	line_count = buf->b_ml.ml_locked_high - buf->b_ml.ml_locked_low;
	CHECK(buf->b_ml.ml_locked_low != lnum + 1, "locked_low != lnum + 1");

	dp = (DATA_BL *)(hp->bh_data);
    }

    ++buf->b_ml.ml_line_count;

    if ((int)dp->db_free >= space_needed)	// enough room in data block
    {
	/*
	 * Insert the new line in an existing data block, or in the data block
	 * allocated above.
	 */
	dp->db_txt_start -= len;
	dp->db_free -= space_needed;
	++(dp->db_line_count);

	/*
	 * move the text of the lines that follow to the front
	 * adjust the indexes of the lines that follow
	 */
	if (line_count > db_idx + 1)	    // if there are following lines
	{
	    /*
	     * Offset is the start of the previous line.
	     * This will become the character just after the new line.
	     */
	    if (db_idx < 0)
		offset = dp->db_txt_end;
	    else
		offset = ((dp->db_index[db_idx]) & DB_INDEX_MASK);
	    mch_memmove((char *)dp + dp->db_txt_start,
					  (char *)dp + dp->db_txt_start + len,
				 (size_t)(offset - (dp->db_txt_start + len)));
	    for (i = line_count - 1; i > db_idx; --i)
		dp->db_index[i + 1] = dp->db_index[i] - len;
	    dp->db_index[db_idx + 1] = offset - len;
	}
	else
	    // add line at the end (which is the start of the text)
	    dp->db_index[db_idx + 1] = dp->db_txt_start;

	/*
	 * copy the text into the block
	 */
	mch_memmove((char *)dp + dp->db_index[db_idx + 1], line, (size_t)len);
	if (flags & ML_APPEND_MARK)
	    dp->db_index[db_idx + 1] |= DB_MARKED;

	/*
	 * Mark the block dirty.
	 */
	buf->b_ml.ml_flags |= ML_LOCKED_DIRTY;
	if (!(flags & ML_APPEND_NEW))
	    buf->b_ml.ml_flags |= ML_LOCKED_POS;
    }
    else	    // not enough space in data block
    {
	long	    line_count_left, line_count_right;
	int	    page_count_left, page_count_right;
	bhdr_T	    *hp_left;
	bhdr_T	    *hp_right;
	bhdr_T	    *hp_new;
	int	    lines_moved;
	int	    data_moved = 0;	    // init to shut up gcc
	int	    total_moved = 0;	    // init to shut up gcc
	DATA_BL	    *dp_right, *dp_left;
	int	    stack_idx;
	int	    in_left;
	int	    lineadd;
	blocknr_T   bnum_left, bnum_right;
	linenr_T    lnum_left, lnum_right;
	int	    pb_idx;
	PTR_BL	    *pp_new;

	/*
	 * There is not enough room, we have to create a new data block and
	 * copy some lines into it.
	 * Then we have to insert an entry in the pointer block.
	 * If this pointer block also is full, we go up another block, and so
	 * on, up to the root if necessary.
	 * The line counts in the pointer blocks have already been adjusted by
	 * ml_find_line().
	 *
	 * We are going to allocate a new data block. Depending on the
	 * situation it will be put to the left or right of the existing
	 * block.  If possible we put the new line in the left block and move
	 * the lines after it to the right block. Otherwise the new line is
	 * also put in the right block. This method is more efficient when
	 * inserting a lot of lines at one place.
	 */
	if (db_idx < 0)		// left block is new, right block is existing
	{
	    lines_moved = 0;
	    in_left = TRUE;
	    // space_needed does not change
	}
	else			// left block is existing, right block is new
	{
	    lines_moved = line_count - db_idx - 1;
	    if (lines_moved == 0)
		in_left = FALSE;	// put new line in right block
					// space_needed does not change
	    else
	    {
		data_moved = ((dp->db_index[db_idx]) & DB_INDEX_MASK) -
							    dp->db_txt_start;
		total_moved = data_moved + lines_moved * INDEX_SIZE;
		if ((int)dp->db_free + total_moved >= space_needed)
		{
		    in_left = TRUE;	// put new line in left block
		    space_needed = total_moved;
		}
		else
		{
		    in_left = FALSE;	    // put new line in right block
		    space_needed += total_moved;
		}
	    }
	}

	page_count = ((space_needed + HEADER_SIZE) + page_size - 1) / page_size;
	if ((hp_new = ml_new_data(mfp, flags & ML_APPEND_NEW, page_count))
								       == NULL)
	{
			// correct line counts in pointer blocks
	    --(buf->b_ml.ml_locked_lineadd);
	    --(buf->b_ml.ml_locked_high);
	    goto theend;
	}
	if (db_idx < 0)		// left block is new
	{
	    hp_left = hp_new;
	    hp_right = hp;
	    line_count_left = 0;
	    line_count_right = line_count;
	}
	else			// right block is new
	{
	    hp_left = hp;
	    hp_right = hp_new;
	    line_count_left = line_count;
	    line_count_right = 0;
	}
	dp_right = (DATA_BL *)(hp_right->bh_data);
	dp_left = (DATA_BL *)(hp_left->bh_data);
	bnum_left = hp_left->bh_bnum;
	bnum_right = hp_right->bh_bnum;
	page_count_left = hp_left->bh_page_count;
	page_count_right = hp_right->bh_page_count;

	/*
	 * May move the new line into the right/new block.
	 */
	if (!in_left)
	{
	    dp_right->db_txt_start -= len;
	    dp_right->db_free -= len + INDEX_SIZE;
	    dp_right->db_index[0] = dp_right->db_txt_start;
	    if (flags & ML_APPEND_MARK)
		dp_right->db_index[0] |= DB_MARKED;

	    mch_memmove((char *)dp_right + dp_right->db_txt_start,
							   line, (size_t)len);
	    ++line_count_right;
	}
	/*
	 * may move lines from the left/old block to the right/new one.
	 */
	if (lines_moved)
	{
	    dp_right->db_txt_start -= data_moved;
	    dp_right->db_free -= total_moved;
	    mch_memmove((char *)dp_right + dp_right->db_txt_start,
			(char *)dp_left + dp_left->db_txt_start,
			(size_t)data_moved);
	    offset = dp_right->db_txt_start - dp_left->db_txt_start;
	    dp_left->db_txt_start += data_moved;
	    dp_left->db_free += total_moved;

	    /*
	     * update indexes in the new block
	     */
	    for (to = line_count_right, from = db_idx + 1;
					 from < line_count_left; ++from, ++to)
		dp_right->db_index[to] = dp->db_index[from] + offset;
	    line_count_right += lines_moved;
	    line_count_left -= lines_moved;
	}

	/*
	 * May move the new line into the left (old or new) block.
	 */
	if (in_left)
	{
	    dp_left->db_txt_start -= len;
	    dp_left->db_free -= len + INDEX_SIZE;
	    dp_left->db_index[line_count_left] = dp_left->db_txt_start;
	    if (flags & ML_APPEND_MARK)
		dp_left->db_index[line_count_left] |= DB_MARKED;
	    mch_memmove((char *)dp_left + dp_left->db_txt_start,
							   line, (size_t)len);
	    ++line_count_left;
	}

	if (db_idx < 0)		// left block is new
	{
	    lnum_left = lnum + 1;
	    lnum_right = 0;
	}
	else			// right block is new
	{
	    lnum_left = 0;
	    if (in_left)
		lnum_right = lnum + 2;
	    else
		lnum_right = lnum + 1;
	}
	dp_left->db_line_count = line_count_left;
	dp_right->db_line_count = line_count_right;

	/*
	 * release the two data blocks
	 * The new one (hp_new) already has a correct blocknumber.
	 * The old one (hp, in ml_locked) gets a positive blocknumber if
	 * we changed it and we are not editing a new file.
	 */
	if (lines_moved || in_left)
	    buf->b_ml.ml_flags |= ML_LOCKED_DIRTY;
	if (!(flags & ML_APPEND_NEW) && db_idx >= 0 && in_left)
	    buf->b_ml.ml_flags |= ML_LOCKED_POS;
	mf_put(mfp, hp_new, TRUE, FALSE);

	/*
	 * flush the old data block
	 * set ml_locked_lineadd to 0, because the updating of the
	 * pointer blocks is done below
	 */
	lineadd = buf->b_ml.ml_locked_lineadd;
	buf->b_ml.ml_locked_lineadd = 0;
	ml_find_line(buf, (linenr_T)0, ML_FLUSH);   // flush data block

	/*
	 * update pointer blocks for the new data block
	 */
	for (stack_idx = buf->b_ml.ml_stack_top - 1; stack_idx >= 0;
								  --stack_idx)
	{
	    ip = &(buf->b_ml.ml_stack[stack_idx]);
	    pb_idx = ip->ip_index;
	    if ((hp = mf_get(mfp, ip->ip_bnum, 1)) == NULL)
		goto theend;
	    pp = (PTR_BL *)(hp->bh_data);   // must be pointer block
	    if (pp->pb_id != PTR_ID)
	    {
		iemsg(e_pointer_block_id_wrong_three);
		mf_put(mfp, hp, FALSE, FALSE);
		goto theend;
	    }
	    /*
	     * TODO: If the pointer block is full and we are adding at the end
	     * try to insert in front of the next block
	     */
	    // block not full, add one entry
	    if (pp->pb_count < pp->pb_count_max)
	    {
		if (pb_idx + 1 < (int)pp->pb_count)
		    mch_memmove(&pp->pb_pointer[pb_idx + 2],
				&pp->pb_pointer[pb_idx + 1],
			(size_t)(pp->pb_count - pb_idx - 1) * sizeof(PTR_EN));
		++pp->pb_count;
		pp->pb_pointer[pb_idx].pe_line_count = line_count_left;
		pp->pb_pointer[pb_idx].pe_bnum = bnum_left;
		pp->pb_pointer[pb_idx].pe_page_count = page_count_left;
		pp->pb_pointer[pb_idx + 1].pe_line_count = line_count_right;
		pp->pb_pointer[pb_idx + 1].pe_bnum = bnum_right;
		pp->pb_pointer[pb_idx + 1].pe_page_count = page_count_right;

		if (lnum_left != 0)
		    pp->pb_pointer[pb_idx].pe_old_lnum = lnum_left;
		if (lnum_right != 0)
		    pp->pb_pointer[pb_idx + 1].pe_old_lnum = lnum_right;

		mf_put(mfp, hp, TRUE, FALSE);
		buf->b_ml.ml_stack_top = stack_idx + 1;	    // truncate stack

		if (lineadd)
		{
		    --(buf->b_ml.ml_stack_top);
		    // fix line count for rest of blocks in the stack
		    ml_lineadd(buf, lineadd);
							// fix stack itself
		    buf->b_ml.ml_stack[buf->b_ml.ml_stack_top].ip_high +=
								      lineadd;
		    ++(buf->b_ml.ml_stack_top);
		}

		/*
		 * We are finished, break the loop here.
		 */
		break;
	    }
	    // pointer block full
	    /*
	     * split the pointer block
	     * allocate a new pointer block
	     * move some of the pointer into the new block
	     * prepare for updating the parent block
	     */
	    for (;;)	// do this twice when splitting block 1
	    {
		hp_new = ml_new_ptr(mfp);
		if (hp_new == NULL)	    // TODO: try to fix tree
		    goto theend;
		pp_new = (PTR_BL *)(hp_new->bh_data);

		if (hp->bh_bnum != 1)
		    break;

		/*
		 * if block 1 becomes full the tree is given an extra level
		 * The pointers from block 1 are moved into the new block.
		 * block 1 is updated to point to the new block
		 * then continue to split the new block
		 */
		mch_memmove(pp_new, pp, (size_t)page_size);
		pp->pb_count = 1;
		pp->pb_pointer[0].pe_bnum = hp_new->bh_bnum;
		pp->pb_pointer[0].pe_line_count = buf->b_ml.ml_line_count;
		pp->pb_pointer[0].pe_old_lnum = 1;
		pp->pb_pointer[0].pe_page_count = 1;
		mf_put(mfp, hp, TRUE, FALSE);   // release block 1
		hp = hp_new;		// new block is to be split
		pp = pp_new;
		CHECK(stack_idx != 0, _("stack_idx should be 0"));
		ip->ip_index = 0;
		++stack_idx;	// do block 1 again later
	    }
	    /*
	     * move the pointers after the current one to the new block
	     * If there are none, the new entry will be in the new block.
	     */
	    total_moved = pp->pb_count - pb_idx - 1;
	    if (total_moved)
	    {
		mch_memmove(&pp_new->pb_pointer[0],
				&pp->pb_pointer[pb_idx + 1],
				(size_t)(total_moved) * sizeof(PTR_EN));
		pp_new->pb_count = total_moved;
		pp->pb_count -= total_moved - 1;
		pp->pb_pointer[pb_idx + 1].pe_bnum = bnum_right;
		pp->pb_pointer[pb_idx + 1].pe_line_count = line_count_right;
		pp->pb_pointer[pb_idx + 1].pe_page_count = page_count_right;
		if (lnum_right)
		    pp->pb_pointer[pb_idx + 1].pe_old_lnum = lnum_right;
	    }
	    else
	    {
		pp_new->pb_count = 1;
		pp_new->pb_pointer[0].pe_bnum = bnum_right;
		pp_new->pb_pointer[0].pe_line_count = line_count_right;
		pp_new->pb_pointer[0].pe_page_count = page_count_right;
		pp_new->pb_pointer[0].pe_old_lnum = lnum_right;
	    }
	    pp->pb_pointer[pb_idx].pe_bnum = bnum_left;
	    pp->pb_pointer[pb_idx].pe_line_count = line_count_left;
	    pp->pb_pointer[pb_idx].pe_page_count = page_count_left;
	    if (lnum_left)
		pp->pb_pointer[pb_idx].pe_old_lnum = lnum_left;
	    lnum_left = 0;
	    lnum_right = 0;

	    /*
	     * recompute line counts
	     */
	    line_count_right = 0;
	    for (i = 0; i < (int)pp_new->pb_count; ++i)
		line_count_right += pp_new->pb_pointer[i].pe_line_count;
	    line_count_left = 0;
	    for (i = 0; i < (int)pp->pb_count; ++i)
		line_count_left += pp->pb_pointer[i].pe_line_count;

	    bnum_left = hp->bh_bnum;
	    bnum_right = hp_new->bh_bnum;
	    page_count_left = 1;
	    page_count_right = 1;
	    mf_put(mfp, hp, TRUE, FALSE);
	    mf_put(mfp, hp_new, TRUE, FALSE);

	}

	/*
	 * Safety check: fallen out of for loop?
	 */
	if (stack_idx < 0)
	{
	    iemsg(e_updated_too_many_blocks);
	    buf->b_ml.ml_stack_top = 0;	// invalidate stack
	}
    }

#ifdef FEAT_BYTEOFF
    // The line was inserted below 'lnum'
    ml_updatechunk(buf, lnum + 1,
# ifdef FEAT_PROP_POPUP
	    (long)text_len
# else
	    (long)len
# endif
	    , ML_CHNK_ADDLINE);
#endif

#ifdef FEAT_NETBEANS_INTG
    if (netbeans_active())
    {
	if (STRLEN(line) > 0)
	    netbeans_inserted(buf, lnum+1, (colnr_T)0, line, (int)STRLEN(line));
	netbeans_inserted(buf, lnum+1, (colnr_T)STRLEN(line),
							   (char_u *)"\n", 1);
    }
#endif
#ifdef FEAT_JOB_CHANNEL
    if (buf->b_write_to_channel)
	channel_write_new_lines(buf);
#endif
    ret = OK;

theend:
#ifdef FEAT_PROP_POPUP
    vim_free(tofree);
#endif
    return ret;
}

/*
 * Flush any pending change and call ml_append_int()
 */
    static int
ml_append_flush(
    buf_T	*buf,
    linenr_T	lnum,		// append after this line (can be 0)
    char_u	*line,		// text of the new line
    colnr_T	len,		// length of line, including NUL, or 0
    int		flags)		// ML_APPEND_ flags
{
    if (lnum > buf->b_ml.ml_line_count)
	return FAIL;  // lnum out of range

    if (buf->b_ml.ml_line_lnum != 0)
	// This may also invoke ml_append_int().
	ml_flush_line(buf);

#ifdef FEAT_EVAL
    // When inserting above recorded changes: flush the changes before changing
    // the text.  Then flush the cached line, it may become invalid.
    may_invoke_listeners(buf, lnum + 1, lnum + 1, 1);
    if (buf->b_ml.ml_line_lnum != 0)
	ml_flush_line(buf);
#endif

    return ml_append_int(buf, lnum, line, len, flags);
}

/*
 * Append a line after lnum (may be 0 to insert a line in front of the file).
 * "line" does not need to be allocated, but can't be another line in a
 * buffer, unlocking may make it invalid.
 *
 * "newfile": TRUE when starting to edit a new file, meaning that pe_old_lnum
 *		will be set for recovery
 * Check: The caller of this function should probably also call
 * appended_lines().
 *
 * return FAIL for failure, OK otherwise
 */
    int
ml_append(
    linenr_T	lnum,		// append after this line (can be 0)
    char_u	*line,		// text of the new line
    colnr_T	len,		// length of new line, including NUL, or 0
    int		newfile)	// flag, see above
{
    return ml_append_flags(lnum, line, len, newfile ? ML_APPEND_NEW : 0);
}

    int
ml_append_flags(
    linenr_T	lnum,		// append after this line (can be 0)
    char_u	*line,		// text of the new line
    colnr_T	len,		// length of new line, including NUL, or 0
    int		flags)		// ML_APPEND_ values
{
    // When starting up, we might still need to create the memfile
    if (curbuf->b_ml.ml_mfp == NULL && open_buffer(FALSE, NULL, 0) == FAIL)
	return FAIL;
    return ml_append_flush(curbuf, lnum, line, len, flags);
}


#if defined(FEAT_SPELL) || defined(FEAT_QUICKFIX) || defined(FEAT_PROP_POPUP) \
	|| defined(PROTO)
/*
 * Like ml_append() but for an arbitrary buffer.  The buffer must already have
 * a memline.
 */
    int
ml_append_buf(
    buf_T	*buf,
    linenr_T	lnum,		// append after this line (can be 0)
    char_u	*line,		// text of the new line
    colnr_T	len,		// length of new line, including NUL, or 0
    int		newfile)	// flag, see above
{
    if (buf->b_ml.ml_mfp == NULL)
	return FAIL;
    return ml_append_flush(buf, lnum, line, len, newfile ? ML_APPEND_NEW : 0);
}
#endif

/*
 * Replace line "lnum", with buffering, in current buffer.
 *
 * If "copy" is TRUE, make a copy of the line, otherwise the line has been
 * copied to allocated memory already.
 * If "copy" is FALSE the "line" may be freed to add text properties!
 * Do not use it after calling ml_replace().
 *
 * Check: The caller of this function should probably also call
 * changed_lines(), unless update_screen(UPD_NOT_VALID) is used.
 *
 * return FAIL for failure, OK otherwise
 */
    int
ml_replace(linenr_T lnum, char_u *line, int copy)
{
    colnr_T len = -1;

    if (line != NULL)
	len = (colnr_T)STRLEN(line);
    return ml_replace_len(lnum, line, len, FALSE, copy);
}

/*
 * Replace a line for the current buffer.  Like ml_replace() with:
 * "len_arg" is the length of the text, excluding NUL.
 * If "has_props" is TRUE then "line_arg" includes the text properties and
 * "len_arg" includes the NUL of the text.
 * When "copy" is TRUE copy the text into allocated memory, otherwise
 * "line_arg" must be allocated and will be consumed here.
 */
    int
ml_replace_len(
	linenr_T    lnum,
	char_u	    *line_arg,
	colnr_T	    len_arg,
	int	    has_props,
	int	    copy)
{
    char_u *line = line_arg;
    colnr_T len = len_arg;

    if (line == NULL)		// just checking...
	return FAIL;

    // When starting up, we might still need to create the memfile
    if (curbuf->b_ml.ml_mfp == NULL && open_buffer(FALSE, NULL, 0) == FAIL)
	return FAIL;

    if (!has_props)
	++len;  // include the NUL after the text
    if (copy)
    {
	// copy the line to allocated memory
#ifdef FEAT_PROP_POPUP
	if (has_props)
	    line = vim_memsave(line, len);
	else
#endif
	    line = vim_strnsave(line, len - 1);
	if (line == NULL)
	    return FAIL;
    }

#ifdef FEAT_NETBEANS_INTG
    if (netbeans_active())
    {
	netbeans_removed(curbuf, lnum, 0, (long)STRLEN(ml_get(lnum)));
	netbeans_inserted(curbuf, lnum, 0, line, (int)STRLEN(line));
    }
#endif
    if (curbuf->b_ml.ml_line_lnum != lnum)
    {
	// another line is buffered, flush it
	ml_flush_line(curbuf);

#ifdef FEAT_PROP_POPUP
	if (curbuf->b_has_textprop && !has_props)
	    // Need to fetch the old line to copy over any text properties.
	    ml_get_buf(curbuf, lnum, TRUE);
#endif
    }

#ifdef FEAT_PROP_POPUP
    if (curbuf->b_has_textprop && !has_props)
    {
	size_t	oldtextlen = STRLEN(curbuf->b_ml.ml_line_ptr) + 1;

	if (oldtextlen < (size_t)curbuf->b_ml.ml_line_len)
	{
	    char_u *newline;
	    size_t textproplen = curbuf->b_ml.ml_line_len - oldtextlen;

	    // Need to copy over text properties, stored after the text.
	    newline = alloc(len + (int)textproplen);
	    if (newline != NULL)
	    {
		mch_memmove(newline, line, len);
		mch_memmove(newline + len, curbuf->b_ml.ml_line_ptr
						    + oldtextlen, textproplen);
		vim_free(line);
		line = newline;
		len += (colnr_T)textproplen;
	    }
	}
    }
#endif

    if (curbuf->b_ml.ml_flags & (ML_LINE_DIRTY | ML_ALLOCATED))
	vim_free(curbuf->b_ml.ml_line_ptr);	// free allocated line

    curbuf->b_ml.ml_line_ptr = line;
    curbuf->b_ml.ml_line_len = len;
    curbuf->b_ml.ml_line_lnum = lnum;
    curbuf->b_ml.ml_flags = (curbuf->b_ml.ml_flags | ML_LINE_DIRTY) & ~ML_EMPTY;

    return OK;
}

#ifdef FEAT_PROP_POPUP
/*
 * Adjust text properties in line "lnum" for a deleted line.
 * When "above" is true this is the line above the deleted line, otherwise this
 * is the line below the deleted line.
 * "del_props[del_props_len]" are the properties of the deleted line.
 */
    static void
adjust_text_props_for_delete(
	buf_T	    *buf,
	linenr_T    lnum,
	char_u	    *del_props,
	int	    del_props_len,
	int	    above)
{
    int		did_get_line = FALSE;
    int		done_del;
    int		done_this;
    textprop_T	prop_del;
    bhdr_T	*hp;
    DATA_BL	*dp;
    int		idx;
    int		line_start;
    long	line_size;
    int		this_props_len = 0;
    char_u	*text;
    size_t	textlen;
    int		found;

    for (done_del = 0; done_del < del_props_len; done_del += sizeof(textprop_T))
    {
	mch_memmove(&prop_del, del_props + done_del, sizeof(textprop_T));
	if ((above && (prop_del.tp_flags & TP_FLAG_CONT_PREV)
		    && !(prop_del.tp_flags & TP_FLAG_CONT_NEXT))
		|| (!above && (prop_del.tp_flags & TP_FLAG_CONT_NEXT)
		    && !(prop_del.tp_flags & TP_FLAG_CONT_PREV)))
	{
	    if (!did_get_line)
	    {
		did_get_line = TRUE;
		if ((hp = ml_find_line(buf, lnum, ML_FIND)) == NULL)
		    return;

		dp = (DATA_BL *)(hp->bh_data);
		idx = lnum - buf->b_ml.ml_locked_low;
		line_start = ((dp->db_index[idx]) & DB_INDEX_MASK);
		if (idx == 0)		// first line in block, text at the end
		    line_size = dp->db_txt_end - line_start;
		else
		    line_size = ((dp->db_index[idx - 1]) & DB_INDEX_MASK)
								  - line_start;
		text = (char_u *)dp + line_start;
		textlen = STRLEN(text) + 1;
		if ((long)textlen >= line_size)
		{
		    if (above)
			internal_error("no text property above deleted line");
		    else
			internal_error("no text property below deleted line");
		    return;
		}
		this_props_len = line_size - (int)textlen;
	    }

	    found = FALSE;
	    for (done_this = 0; done_this < this_props_len;
					       done_this += sizeof(textprop_T))
	    {
		int	    flag = above ? TP_FLAG_CONT_NEXT
							   : TP_FLAG_CONT_PREV;
		textprop_T  prop_this;

		mch_memmove(&prop_this, text + textlen + done_this,
							   sizeof(textprop_T));
		if ((prop_this.tp_flags & flag)
			&& prop_del.tp_id == prop_this.tp_id
			&& prop_del.tp_type == prop_this.tp_type)
		{
		    found = TRUE;
		    prop_this.tp_flags &= ~flag;
		    mch_memmove(text + textlen + done_this, &prop_this,
							   sizeof(textprop_T));
		    break;
		}
	    }
	    if (!found)
	    {
		if (above)
		    internal_error("text property above deleted line not found");
		else
		    internal_error("text property below deleted line not found");
	    }

	    buf->b_ml.ml_flags |= (ML_LOCKED_DIRTY | ML_LOCKED_POS);
	}
    }
}
#endif

/*
 * Delete line "lnum" in the current buffer.
 * When "flags" has ML_DEL_MESSAGE may give a "No lines in buffer" message.
 * When "flags" has ML_DEL_UNDO this is called from undo.
 *
 * return FAIL for failure, OK otherwise
 */
    static int
ml_delete_int(buf_T *buf, linenr_T lnum, int flags)
{
    bhdr_T	*hp;
    memfile_T	*mfp;
    DATA_BL	*dp;
    PTR_BL	*pp;
    infoptr_T	*ip;
    int		count;	    // number of entries in block
    int		idx;
    int		stack_idx;
    int		text_start;
    int		line_start;
    long	line_size;
    int		i;
    int		ret = FAIL;
#ifdef FEAT_PROP_POPUP
    char_u	*textprop_save = NULL;
    long	textprop_len = 0;
#endif

    if (lowest_marked && lowest_marked > lnum)
	lowest_marked--;

/*
 * If the file becomes empty the last line is replaced by an empty line.
 */
    if (buf->b_ml.ml_line_count == 1)	    // file becomes empty
    {
	if ((flags & ML_DEL_MESSAGE)
#ifdef FEAT_NETBEANS_INTG
		&& !netbeansSuppressNoLines
#endif
	   )
	    set_keep_msg((char_u *)_(no_lines_msg), 0);

	// FEAT_BYTEOFF already handled in there, don't worry 'bout it below
	i = ml_replace((linenr_T)1, (char_u *)"", TRUE);
	buf->b_ml.ml_flags |= ML_EMPTY;

	return i;
    }

/*
 * Find the data block containing the line.
 * This also fills the stack with the blocks from the root to the data block.
 * This also releases any locked block..
 */
    mfp = buf->b_ml.ml_mfp;
    if (mfp == NULL)
	return FAIL;

    if ((hp = ml_find_line(buf, lnum, ML_DELETE)) == NULL)
	return FAIL;

    dp = (DATA_BL *)(hp->bh_data);
    // compute line count before the delete
    count = (long)(buf->b_ml.ml_locked_high)
					- (long)(buf->b_ml.ml_locked_low) + 2;
    idx = lnum - buf->b_ml.ml_locked_low;

    --buf->b_ml.ml_line_count;

    line_start = ((dp->db_index[idx]) & DB_INDEX_MASK);
    if (idx == 0)		// first line in block, text at the end
	line_size = dp->db_txt_end - line_start;
    else
	line_size = ((dp->db_index[idx - 1]) & DB_INDEX_MASK) - line_start;

#ifdef FEAT_NETBEANS_INTG
    if (netbeans_active())
	netbeans_removed(buf, lnum, 0, line_size);
#endif
#ifdef FEAT_PROP_POPUP
    // If there are text properties compute their byte length.
    // if needed make a copy, so that we can update properties in preceding and
    // following lines.
    if (buf->b_has_textprop)
    {
	size_t	textlen = STRLEN((char_u *)dp + line_start) + 1;

	textprop_len = line_size - (long)textlen;
	if (!(flags & (ML_DEL_UNDO | ML_DEL_NOPROP)) && textprop_len > 0)
	    textprop_save = vim_memsave((char_u *)dp + line_start + textlen,
								 textprop_len);
    }
#endif

/*
 * special case: If there is only one line in the data block it becomes empty.
 * Then we have to remove the entry, pointing to this data block, from the
 * pointer block. If this pointer block also becomes empty, we go up another
 * block, and so on, up to the root if necessary.
 * The line counts in the pointer blocks have already been adjusted by
 * ml_find_line().
 */
    if (count == 1)
    {
	mf_free(mfp, hp);	// free the data block
	buf->b_ml.ml_locked = NULL;

	for (stack_idx = buf->b_ml.ml_stack_top - 1; stack_idx >= 0;
								  --stack_idx)
	{
	    buf->b_ml.ml_stack_top = 0;	    // stack is invalid when failing
	    ip = &(buf->b_ml.ml_stack[stack_idx]);
	    idx = ip->ip_index;
	    if ((hp = mf_get(mfp, ip->ip_bnum, 1)) == NULL)
		goto theend;
	    pp = (PTR_BL *)(hp->bh_data);   // must be pointer block
	    if (pp->pb_id != PTR_ID)
	    {
		iemsg(e_pointer_block_id_wrong_four);
		mf_put(mfp, hp, FALSE, FALSE);
		goto theend;
	    }
	    count = --(pp->pb_count);
	    if (count == 0)	    // the pointer block becomes empty!
		mf_free(mfp, hp);
	    else
	    {
		if (count != idx)	// move entries after the deleted one
		    mch_memmove(&pp->pb_pointer[idx], &pp->pb_pointer[idx + 1],
				      (size_t)(count - idx) * sizeof(PTR_EN));
		mf_put(mfp, hp, TRUE, FALSE);

		buf->b_ml.ml_stack_top = stack_idx;	// truncate stack
		// fix line count for rest of blocks in the stack
		if (buf->b_ml.ml_locked_lineadd != 0)
		{
		    ml_lineadd(buf, buf->b_ml.ml_locked_lineadd);
		    buf->b_ml.ml_stack[buf->b_ml.ml_stack_top].ip_high +=
						  buf->b_ml.ml_locked_lineadd;
		}
		++(buf->b_ml.ml_stack_top);

		break;
	    }
	}
	CHECK(stack_idx < 0, _("deleted block 1?"));
    }
    else
    {
	/*
	 * delete the text by moving the next lines forwards
	 */
	text_start = dp->db_txt_start;
	mch_memmove((char *)dp + text_start + line_size,
		  (char *)dp + text_start, (size_t)(line_start - text_start));

	/*
	 * delete the index by moving the next indexes backwards
	 * Adjust the indexes for the text movement.
	 */
	for (i = idx; i < count - 1; ++i)
	    dp->db_index[i] = dp->db_index[i + 1] + line_size;

	dp->db_free += line_size + INDEX_SIZE;
	dp->db_txt_start += line_size;
	--(dp->db_line_count);

	/*
	 * mark the block dirty and make sure it is in the file (for recovery)
	 */
	buf->b_ml.ml_flags |= (ML_LOCKED_DIRTY | ML_LOCKED_POS);
    }

#ifdef FEAT_BYTEOFF
    ml_updatechunk(buf, lnum, line_size
# ifdef FEAT_PROP_POPUP
					- textprop_len
# endif
						    , ML_CHNK_DELLINE);
#endif
    ret = OK;

theend:
#ifdef FEAT_PROP_POPUP
    if (textprop_save != NULL)
    {
	// Adjust text properties in the line above and below.
	if (lnum > 1)
	    adjust_text_props_for_delete(buf, lnum - 1, textprop_save,
						      (int)textprop_len, TRUE);
	if (lnum <= buf->b_ml.ml_line_count)
	    adjust_text_props_for_delete(buf, lnum, textprop_save,
						     (int)textprop_len, FALSE);
    }
    vim_free(textprop_save);
#endif
    return ret;
}

/*
 * Delete line "lnum" in the current buffer.
 * When "message" is TRUE may give a "No lines in buffer" message.
 *
 * Check: The caller of this function should probably also call
 * deleted_lines() after this.
 *
 * return FAIL for failure, OK otherwise
 */
    int
ml_delete(linenr_T lnum)
{
    return ml_delete_flags(lnum, 0);
}

/*
 * Like ml_delete() but using flags (see ml_delete_int()).
 */
    int
ml_delete_flags(linenr_T lnum, int flags)
{
    ml_flush_line(curbuf);
    if (lnum < 1 || lnum > curbuf->b_ml.ml_line_count)
	return FAIL;

#ifdef FEAT_EVAL
    // When inserting above recorded changes: flush the changes before changing
    // the text.
    may_invoke_listeners(curbuf, lnum, lnum + 1, -1);
#endif

    return ml_delete_int(curbuf, lnum, flags);
}

/*
 * set the DB_MARKED flag for line 'lnum'
 */
    void
ml_setmarked(linenr_T lnum)
{
    bhdr_T    *hp;
    DATA_BL *dp;
				    // invalid line number
    if (lnum < 1 || lnum > curbuf->b_ml.ml_line_count
					       || curbuf->b_ml.ml_mfp == NULL)
	return;			    // give error message?

    if (lowest_marked == 0 || lowest_marked > lnum)
	lowest_marked = lnum;

    /*
     * find the data block containing the line
     * This also fills the stack with the blocks from the root to the data block
     * This also releases any locked block.
     */
    if ((hp = ml_find_line(curbuf, lnum, ML_FIND)) == NULL)
	return;		    // give error message?

    dp = (DATA_BL *)(hp->bh_data);
    dp->db_index[lnum - curbuf->b_ml.ml_locked_low] |= DB_MARKED;
    curbuf->b_ml.ml_flags |= ML_LOCKED_DIRTY;
}

/*
 * find the first line with its DB_MARKED flag set
 */
    linenr_T
ml_firstmarked(void)
{
    bhdr_T	*hp;
    DATA_BL	*dp;
    linenr_T	lnum;
    int		i;

    if (curbuf->b_ml.ml_mfp == NULL)
	return (linenr_T) 0;

    /*
     * The search starts with lowest_marked line. This is the last line where
     * a mark was found, adjusted by inserting/deleting lines.
     */
    for (lnum = lowest_marked; lnum <= curbuf->b_ml.ml_line_count; )
    {
	/*
	 * Find the data block containing the line.
	 * This also fills the stack with the blocks from the root to the data
	 * block This also releases any locked block.
	 */
	if ((hp = ml_find_line(curbuf, lnum, ML_FIND)) == NULL)
	    return (linenr_T)0;		    // give error message?

	dp = (DATA_BL *)(hp->bh_data);

	for (i = lnum - curbuf->b_ml.ml_locked_low;
			    lnum <= curbuf->b_ml.ml_locked_high; ++i, ++lnum)
	    if ((dp->db_index[i]) & DB_MARKED)
	    {
		(dp->db_index[i]) &= DB_INDEX_MASK;
		curbuf->b_ml.ml_flags |= ML_LOCKED_DIRTY;
		lowest_marked = lnum + 1;
		return lnum;
	    }
    }

    return (linenr_T) 0;
}

/*
 * clear all DB_MARKED flags
 */
    void
ml_clearmarked(void)
{
    bhdr_T	*hp;
    DATA_BL	*dp;
    linenr_T	lnum;
    int		i;

    if (curbuf->b_ml.ml_mfp == NULL)	    // nothing to do
	return;

    /*
     * The search starts with line lowest_marked.
     */
    for (lnum = lowest_marked; lnum <= curbuf->b_ml.ml_line_count; )
    {
	/*
	 * Find the data block containing the line.
	 * This also fills the stack with the blocks from the root to the data
	 * block and releases any locked block.
	 */
	if ((hp = ml_find_line(curbuf, lnum, ML_FIND)) == NULL)
	    return;		// give error message?

	dp = (DATA_BL *)(hp->bh_data);

	for (i = lnum - curbuf->b_ml.ml_locked_low;
			    lnum <= curbuf->b_ml.ml_locked_high; ++i, ++lnum)
	    if ((dp->db_index[i]) & DB_MARKED)
	    {
		(dp->db_index[i]) &= DB_INDEX_MASK;
		curbuf->b_ml.ml_flags |= ML_LOCKED_DIRTY;
	    }
    }

    lowest_marked = 0;
}

/*
 * flush ml_line if necessary
 */
    static void
ml_flush_line(buf_T *buf)
{
    bhdr_T	*hp;
    DATA_BL	*dp;
    linenr_T	lnum;
    char_u	*new_line;
    char_u	*old_line;
    colnr_T	new_len;
    int		old_len;
    int		extra;
    int		idx;
    int		start;
    int		count;
    int		i;
    static int  entered = FALSE;

    if (buf->b_ml.ml_line_lnum == 0 || buf->b_ml.ml_mfp == NULL)
	return;		// nothing to do

    if (buf->b_ml.ml_flags & ML_LINE_DIRTY)
    {
	// This code doesn't work recursively, but Netbeans may call back here
	// when obtaining the cursor position.
	if (entered)
	    return;
	entered = TRUE;

	lnum = buf->b_ml.ml_line_lnum;
	new_line = buf->b_ml.ml_line_ptr;

	hp = ml_find_line(buf, lnum, ML_FIND);
	if (hp == NULL)
	    siemsg(e_cannot_find_line_nr, lnum);
	else
	{
	    dp = (DATA_BL *)(hp->bh_data);
	    idx = lnum - buf->b_ml.ml_locked_low;
	    start = ((dp->db_index[idx]) & DB_INDEX_MASK);
	    old_line = (char_u *)dp + start;
	    if (idx == 0)	// line is last in block
		old_len = dp->db_txt_end - start;
	    else		// text of previous line follows
		old_len = (dp->db_index[idx - 1] & DB_INDEX_MASK) - start;
	    new_len = buf->b_ml.ml_line_len;
	    extra = new_len - old_len;	    // negative if lines gets smaller

	    /*
	     * if new line fits in data block, replace directly
	     */
	    if ((int)dp->db_free >= extra)
	    {
#if defined(FEAT_BYTEOFF) && defined(FEAT_PROP_POPUP)
		int old_prop_len = 0;
		if (buf->b_has_textprop)
		    old_prop_len = old_len - (int)STRLEN(old_line) - 1;
#endif
		// if the length changes and there are following lines
		count = buf->b_ml.ml_locked_high - buf->b_ml.ml_locked_low + 1;
		if (extra != 0 && idx < count - 1)
		{
		    // move text of following lines
		    mch_memmove((char *)dp + dp->db_txt_start - extra,
				(char *)dp + dp->db_txt_start,
				(size_t)(start - dp->db_txt_start));

		    // adjust pointers of this and following lines
		    for (i = idx + 1; i < count; ++i)
			dp->db_index[i] -= extra;
		}
		dp->db_index[idx] -= extra;

		// adjust free space
		dp->db_free -= extra;
		dp->db_txt_start -= extra;

		// copy new line into the data block
		mch_memmove(old_line - extra, new_line, (size_t)new_len);
		buf->b_ml.ml_flags |= (ML_LOCKED_DIRTY | ML_LOCKED_POS);
#if defined(FEAT_BYTEOFF) && defined(FEAT_PROP_POPUP)
		// The else case is already covered by the insert and delete
		if (buf->b_has_textprop)
		{
		    // Do not count the size of any text properties.
		    extra += old_prop_len;
		    extra -= new_len - (int)STRLEN(new_line) - 1;
		}
		if (extra != 0)
		    ml_updatechunk(buf, lnum, (long)extra, ML_CHNK_UPDLINE);
#endif
	    }
	    else
	    {
		/*
		 * Cannot do it in one data block: Delete and append.
		 * Append first, because ml_delete_int() cannot delete the
		 * last line in a buffer, which causes trouble for a buffer
		 * that has only one line.
		 * Don't forget to copy the mark!
		 */
		// How about handling errors???
		(void)ml_append_int(buf, lnum, new_line, new_len,
			 ((dp->db_index[idx] & DB_MARKED) ? ML_APPEND_MARK : 0)
#ifdef FEAT_PROP_POPUP
			     | ML_APPEND_NOPROP
#endif
			 );
		(void)ml_delete_int(buf, lnum, ML_DEL_NOPROP);
	    }
	}
	vim_free(new_line);

	entered = FALSE;
    }
    else if (buf->b_ml.ml_flags & ML_ALLOCATED)
	vim_free(buf->b_ml.ml_line_ptr);

    buf->b_ml.ml_flags &= ~(ML_LINE_DIRTY | ML_ALLOCATED);
    buf->b_ml.ml_line_lnum = 0;
}

/*
 * create a new, empty, data block
 */
    static bhdr_T *
ml_new_data(memfile_T *mfp, int negative, int page_count)
{
    bhdr_T	*hp;
    DATA_BL	*dp;

    if ((hp = mf_new(mfp, negative, page_count)) == NULL)
	return NULL;

    dp = (DATA_BL *)(hp->bh_data);
    dp->db_id = DATA_ID;
    dp->db_txt_start = dp->db_txt_end = page_count * mfp->mf_page_size;
    dp->db_free = dp->db_txt_start - HEADER_SIZE;
    dp->db_line_count = 0;

    return hp;
}

/*
 * create a new, empty, pointer block
 */
    static bhdr_T *
ml_new_ptr(memfile_T *mfp)
{
    bhdr_T	*hp;
    PTR_BL	*pp;

    if ((hp = mf_new(mfp, FALSE, 1)) == NULL)
	return NULL;

    pp = (PTR_BL *)(hp->bh_data);
    pp->pb_id = PTR_ID;
    pp->pb_count = 0;
    pp->pb_count_max = PB_COUNT_MAX(mfp);

    return hp;
}

/*
 * Lookup line 'lnum' in a memline.
 *
 *   action: if ML_DELETE or ML_INSERT the line count is updated while searching
 *	     if ML_FLUSH only flush a locked block
 *	     if ML_FIND just find the line
 *
 * If the block was found it is locked and put in ml_locked.
 * The stack is updated to lead to the locked block. The ip_high field in
 * the stack is updated to reflect the last line in the block AFTER the
 * insert or delete, also if the pointer block has not been updated yet. But
 * if ml_locked != NULL ml_locked_lineadd must be added to ip_high.
 *
 * return: NULL for failure, pointer to block header otherwise
 */
    static bhdr_T *
ml_find_line(buf_T *buf, linenr_T lnum, int action)
{
    DATA_BL	*dp;
    PTR_BL	*pp;
    infoptr_T	*ip;
    bhdr_T	*hp;
    memfile_T	*mfp;
    linenr_T	t;
    blocknr_T	bnum, bnum2;
    int		dirty;
    linenr_T	low, high;
    int		top;
    int		page_count;
    int		idx;

    mfp = buf->b_ml.ml_mfp;

    /*
     * If there is a locked block check if the wanted line is in it.
     * If not, flush and release the locked block.
     * Don't do this for ML_INSERT_SAME, because the stack need to be updated.
     * Don't do this for ML_FLUSH, because we want to flush the locked block.
     * Don't do this when 'swapfile' is reset, we want to load all the blocks.
     */
    if (buf->b_ml.ml_locked)
    {
	if (ML_SIMPLE(action)
		&& buf->b_ml.ml_locked_low <= lnum
		&& buf->b_ml.ml_locked_high >= lnum
		&& !mf_dont_release)
	{
	    // remember to update pointer blocks and stack later
	    if (action == ML_INSERT)
	    {
		++(buf->b_ml.ml_locked_lineadd);
		++(buf->b_ml.ml_locked_high);
	    }
	    else if (action == ML_DELETE)
	    {
		--(buf->b_ml.ml_locked_lineadd);
		--(buf->b_ml.ml_locked_high);
	    }
	    return (buf->b_ml.ml_locked);
	}

	mf_put(mfp, buf->b_ml.ml_locked, buf->b_ml.ml_flags & ML_LOCKED_DIRTY,
					    buf->b_ml.ml_flags & ML_LOCKED_POS);
	buf->b_ml.ml_locked = NULL;

	/*
	 * If lines have been added or deleted in the locked block, need to
	 * update the line count in pointer blocks.
	 */
	if (buf->b_ml.ml_locked_lineadd != 0)
	    ml_lineadd(buf, buf->b_ml.ml_locked_lineadd);
    }

    if (action == ML_FLUSH)	    // nothing else to do
	return NULL;

    bnum = 1;			    // start at the root of the tree
    page_count = 1;
    low = 1;
    high = buf->b_ml.ml_line_count;

    if (action == ML_FIND)	// first try stack entries
    {
	for (top = buf->b_ml.ml_stack_top - 1; top >= 0; --top)
	{
	    ip = &(buf->b_ml.ml_stack[top]);
	    if (ip->ip_low <= lnum && ip->ip_high >= lnum)
	    {
		bnum = ip->ip_bnum;
		low = ip->ip_low;
		high = ip->ip_high;
		buf->b_ml.ml_stack_top = top;	// truncate stack at prev entry
		break;
	    }
	}
	if (top < 0)
	    buf->b_ml.ml_stack_top = 0;		// not found, start at the root
    }
    else	// ML_DELETE or ML_INSERT
	buf->b_ml.ml_stack_top = 0;	// start at the root

/*
 * search downwards in the tree until a data block is found
 */
    for (;;)
    {
	if ((hp = mf_get(mfp, bnum, page_count)) == NULL)
	    goto error_noblock;

	/*
	 * update high for insert/delete
	 */
	if (action == ML_INSERT)
	    ++high;
	else if (action == ML_DELETE)
	    --high;

	dp = (DATA_BL *)(hp->bh_data);
	if (dp->db_id == DATA_ID)	// data block
	{
	    buf->b_ml.ml_locked = hp;
	    buf->b_ml.ml_locked_low = low;
	    buf->b_ml.ml_locked_high = high;
	    buf->b_ml.ml_locked_lineadd = 0;
	    buf->b_ml.ml_flags &= ~(ML_LOCKED_DIRTY | ML_LOCKED_POS);
	    return hp;
	}

	pp = (PTR_BL *)(dp);		// must be pointer block
	if (pp->pb_id != PTR_ID)
	{
	    iemsg(e_pointer_block_id_wrong);
	    goto error_block;
	}

	if ((top = ml_add_stack(buf)) < 0)	// add new entry to stack
	    goto error_block;
	ip = &(buf->b_ml.ml_stack[top]);
	ip->ip_bnum = bnum;
	ip->ip_low = low;
	ip->ip_high = high;
	ip->ip_index = -1;		// index not known yet

	dirty = FALSE;
	for (idx = 0; idx < (int)pp->pb_count; ++idx)
	{
	    t = pp->pb_pointer[idx].pe_line_count;
	    CHECK(t == 0, _("pe_line_count is zero"));
	    if ((low += t) > lnum)
	    {
		ip->ip_index = idx;
		bnum = pp->pb_pointer[idx].pe_bnum;
		page_count = pp->pb_pointer[idx].pe_page_count;
		high = low - 1;
		low -= t;

		/*
		 * a negative block number may have been changed
		 */
		if (bnum < 0)
		{
		    bnum2 = mf_trans_del(mfp, bnum);
		    if (bnum != bnum2)
		    {
			bnum = bnum2;
			pp->pb_pointer[idx].pe_bnum = bnum;
			dirty = TRUE;
		    }
		}

		break;
	    }
	}
	if (idx >= (int)pp->pb_count)	    // past the end: something wrong!
	{
	    if (lnum > buf->b_ml.ml_line_count)
		siemsg(e_line_number_out_of_range_nr_past_the_end,
					      lnum - buf->b_ml.ml_line_count);

	    else
		siemsg(e_line_count_wrong_in_block_nr, bnum);
	    goto error_block;
	}
	if (action == ML_DELETE)
	{
	    pp->pb_pointer[idx].pe_line_count--;
	    dirty = TRUE;
	}
	else if (action == ML_INSERT)
	{
	    pp->pb_pointer[idx].pe_line_count++;
	    dirty = TRUE;
	}
	mf_put(mfp, hp, dirty, FALSE);
    }

error_block:
    mf_put(mfp, hp, FALSE, FALSE);
error_noblock:
    /*
     * If action is ML_DELETE or ML_INSERT we have to correct the tree for
     * the incremented/decremented line counts, because there won't be a line
     * inserted/deleted after all.
     */
    if (action == ML_DELETE)
	ml_lineadd(buf, 1);
    else if (action == ML_INSERT)
	ml_lineadd(buf, -1);
    buf->b_ml.ml_stack_top = 0;
    return NULL;
}

/*
 * add an entry to the info pointer stack
 *
 * return -1 for failure, number of the new entry otherwise
 */
    static int
ml_add_stack(buf_T *buf)
{
    int		top;
    infoptr_T	*newstack;

    top = buf->b_ml.ml_stack_top;

    // may have to increase the stack size
    if (top == buf->b_ml.ml_stack_size)
    {
	CHECK(top > 0, _("Stack size increases")); // more than 5 levels???

	newstack = ALLOC_MULT(infoptr_T, buf->b_ml.ml_stack_size + STACK_INCR);
	if (newstack == NULL)
	    return -1;
	if (top > 0)
	    mch_memmove(newstack, buf->b_ml.ml_stack,
					     (size_t)top * sizeof(infoptr_T));
	vim_free(buf->b_ml.ml_stack);
	buf->b_ml.ml_stack = newstack;
	buf->b_ml.ml_stack_size += STACK_INCR;
    }

    buf->b_ml.ml_stack_top++;
    return top;
}

/*
 * Update the pointer blocks on the stack for inserted/deleted lines.
 * The stack itself is also updated.
 *
 * When a insert/delete line action fails, the line is not inserted/deleted,
 * but the pointer blocks have already been updated. That is fixed here by
 * walking through the stack.
 *
 * Count is the number of lines added, negative if lines have been deleted.
 */
    static void
ml_lineadd(buf_T *buf, int count)
{
    int		idx;
    infoptr_T	*ip;
    PTR_BL	*pp;
    memfile_T	*mfp = buf->b_ml.ml_mfp;
    bhdr_T	*hp;

    for (idx = buf->b_ml.ml_stack_top - 1; idx >= 0; --idx)
    {
	ip = &(buf->b_ml.ml_stack[idx]);
	if ((hp = mf_get(mfp, ip->ip_bnum, 1)) == NULL)
	    break;
	pp = (PTR_BL *)(hp->bh_data);	// must be pointer block
	if (pp->pb_id != PTR_ID)
	{
	    mf_put(mfp, hp, FALSE, FALSE);
	    iemsg(e_pointer_block_id_wrong_two);
	    break;
	}
	pp->pb_pointer[ip->ip_index].pe_line_count += count;
	ip->ip_high += count;
	mf_put(mfp, hp, TRUE, FALSE);
    }
}

#if defined(HAVE_READLINK) || defined(PROTO)
/*
 * Resolve a symlink in the last component of a file name.
 * Note that f_resolve() does it for every part of the path, we don't do that
 * here.
 * If it worked returns OK and the resolved link in "buf[MAXPATHL]".
 * Otherwise returns FAIL.
 */
    int
resolve_symlink(char_u *fname, char_u *buf)
{
    char_u	tmp[MAXPATHL];
    int		ret;
    int		depth = 0;

    if (fname == NULL)
	return FAIL;

    // Put the result so far in tmp[], starting with the original name.
    vim_strncpy(tmp, fname, MAXPATHL - 1);

    for (;;)
    {
	// Limit symlink depth to 100, catch recursive loops.
	if (++depth == 100)
	{
	    semsg(_(e_symlink_loop_for_str), fname);
	    return FAIL;
	}

	ret = readlink((char *)tmp, (char *)buf, MAXPATHL - 1);
	if (ret <= 0)
	{
	    if (errno == EINVAL || errno == ENOENT)
	    {
		// Found non-symlink or not existing file, stop here.
		// When at the first level use the unmodified name, skip the
		// call to vim_FullName().
		if (depth == 1)
		    return FAIL;

		// Use the resolved name in tmp[].
		break;
	    }

	    // There must be some error reading links, use original name.
	    return FAIL;
	}
	buf[ret] = NUL;

	/*
	 * Check whether the symlink is relative or absolute.
	 * If it's relative, build a new path based on the directory
	 * portion of the filename (if any) and the path the symlink
	 * points to.
	 */
	if (mch_isFullName(buf))
	    STRCPY(tmp, buf);
	else
	{
	    char_u *tail;

	    tail = gettail(tmp);
	    if (STRLEN(tail) + STRLEN(buf) >= MAXPATHL)
		return FAIL;
	    STRCPY(tail, buf);
	}
    }

    /*
     * Try to resolve the full name of the file so that the swapfile name will
     * be consistent even when opening a relative symlink from different
     * working directories.
     */
    return vim_FullName(tmp, buf, MAXPATHL, TRUE);
}
#endif

/*
 * Make swap file name out of the file name and a directory name.
 * Returns pointer to allocated memory or NULL.
 */
    char_u *
makeswapname(
    char_u	*fname,
    char_u	*ffname UNUSED,
    buf_T	*buf,
    char_u	*dir_name)
{
    char_u	*r, *s;
    char_u	*fname_res = fname;
#ifdef HAVE_READLINK
    char_u	fname_buf[MAXPATHL];

    // Expand symlink in the file name, so that we put the swap file with the
    // actual file instead of with the symlink.
    if (resolve_symlink(fname, fname_buf) == OK)
	fname_res = fname_buf;
#endif

#if defined(UNIX) || defined(MSWIN)  // Need _very_ long file names
    int		len = (int)STRLEN(dir_name);

    s = dir_name + len;
    if (after_pathsep(dir_name, s) && len > 1 && s[-1] == s[-2])
    {			       // Ends with '//', Use Full path
	r = NULL;
	if ((s = make_percent_swname(dir_name, fname_res)) != NULL)
	{
	    r = modname(s, (char_u *)".swp", FALSE);
	    vim_free(s);
	}
	return r;
    }
#endif

    r = buf_modname(
	    (buf->b_p_sn || buf->b_shortname),
	    fname_res,
	    (char_u *)
#if defined(VMS)
	    "_swp",
#else
	    ".swp",
#endif
	    // Prepend a '.' to the swap file name for the current directory.
	    dir_name[0] == '.' && dir_name[1] == NUL);
    if (r == NULL)	    // out of memory
	return NULL;

    s = get_file_in_dir(r, dir_name);
    vim_free(r);
    return s;
}

/*
 * Get file name to use for swap file or backup file.
 * Use the name of the edited file "fname" and an entry in the 'dir' or 'bdir'
 * option "dname".
 * - If "dname" is ".", return "fname" (swap file in dir of file).
 * - If "dname" starts with "./", insert "dname" in "fname" (swap file
 *   relative to dir of file).
 * - Otherwise, prepend "dname" to the tail of "fname" (swap file in specific
 *   dir).
 *
 * The return value is an allocated string and can be NULL.
 */
    char_u *
get_file_in_dir(
    char_u  *fname,
    char_u  *dname)	// don't use "dirname", it is a global for Alpha
{
    char_u	*t;
    char_u	*tail;
    char_u	*retval;
    int		save_char;

    tail = gettail(fname);

    if (dname[0] == '.' && dname[1] == NUL)
	retval = vim_strsave(fname);
    else if (dname[0] == '.' && vim_ispathsep(dname[1]))
    {
	if (tail == fname)	    // no path before file name
	    retval = concat_fnames(dname + 2, tail, TRUE);
	else
	{
	    save_char = *tail;
	    *tail = NUL;
	    t = concat_fnames(fname, dname + 2, TRUE);
	    *tail = save_char;
	    if (t == NULL)	    // out of memory
		retval = NULL;
	    else
	    {
		retval = concat_fnames(t, tail, TRUE);
		vim_free(t);
	    }
	}
    }
    else
	retval = concat_fnames(dname, tail, TRUE);

#ifdef MSWIN
    if (retval != NULL)
	for (t = gettail(retval); *t != NUL; MB_PTR_ADV(t))
	    if (*t == ':')
		*t = '%';
#endif

    return retval;
}

/*
 * Print the ATTENTION message: info about an existing swap file.
 */
    static void
attention_message(
    buf_T   *buf,	// buffer being edited
    char_u  *fname)	// swap file name
{
    stat_T	st;
    time_t	swap_mtime;

    ++no_wait_return;
    (void)emsg(_(e_attention));
    msg_puts(_("\nFound a swap file by the name \""));
    msg_home_replace(fname);
    msg_puts("\"\n");
    swap_mtime = swapfile_info(fname);
    msg_puts(_("While opening file \""));
    msg_outtrans(buf->b_fname);
    msg_puts("\"\n");
    if (mch_stat((char *)buf->b_fname, &st) == -1)
    {
	msg_puts(_("      CANNOT BE FOUND"));
    }
    else
    {
	msg_puts(_("             dated: "));
	msg_puts(get_ctime(st.st_mtime, TRUE));
	if (swap_mtime != 0 && st.st_mtime > swap_mtime)
	    msg_puts(_("      NEWER than swap file!\n"));
    }
    // Some of these messages are long to allow translation to
    // other languages.
    msg_puts(_("\n(1) Another program may be editing the same file.  If this is the case,\n    be careful not to end up with two different instances of the same\n    file when making changes.  Quit, or continue with caution.\n"));
    msg_puts(_("(2) An edit session for this file crashed.\n"));
    msg_puts(_("    If this is the case, use \":recover\" or \"vim -r "));
    msg_outtrans(buf->b_fname);
    msg_puts(_("\"\n    to recover the changes (see \":help recovery\").\n"));
    msg_puts(_("    If you did this already, delete the swap file \""));
    msg_outtrans(fname);
    msg_puts(_("\"\n    to avoid this message.\n"));
    cmdline_row = msg_row;
    --no_wait_return;
}

typedef enum {
    SEA_CHOICE_NONE = 0,
    SEA_CHOICE_READONLY = 1,
    SEA_CHOICE_EDIT = 2,
    SEA_CHOICE_RECOVER = 3,
    SEA_CHOICE_DELETE = 4,
    SEA_CHOICE_QUIT = 5,
    SEA_CHOICE_ABORT = 6
} sea_choice_T;

#if defined(FEAT_EVAL)
/*
 * Trigger the SwapExists autocommands.
 * Returns a value for equivalent to do_dialog().
 */
    static sea_choice_T
do_swapexists(buf_T *buf, char_u *fname)
{
    set_vim_var_string(VV_SWAPNAME, fname, -1);
    set_vim_var_string(VV_SWAPCHOICE, NULL, -1);

    // Trigger SwapExists autocommands with <afile> set to the file being
    // edited.  Disallow changing directory here.
    ++allbuf_lock;
    apply_autocmds(EVENT_SWAPEXISTS, buf->b_fname, NULL, FALSE, NULL);
    --allbuf_lock;

    set_vim_var_string(VV_SWAPNAME, NULL, -1);

    switch (*get_vim_var_str(VV_SWAPCHOICE))
    {
	case 'o': return SEA_CHOICE_READONLY;
	case 'e': return SEA_CHOICE_EDIT;
	case 'r': return SEA_CHOICE_RECOVER;
	case 'd': return SEA_CHOICE_DELETE;
	case 'q': return SEA_CHOICE_QUIT;
	case 'a': return SEA_CHOICE_ABORT;
    }

    return SEA_CHOICE_NONE;
}
#endif

/*
 * Find out what name to use for the swap file for buffer 'buf'.
 *
 * Several names are tried to find one that does not exist
 * Returns the name in allocated memory or NULL.
 * When out of memory "dirp" is set to NULL.
 *
 * Note: If BASENAMELEN is not correct, you will get error messages for
 *	 not being able to open the swap or undo file
 * Note: May trigger SwapExists autocmd, pointers may change!
 */
    static char_u *
findswapname(
    buf_T	*buf,
    char_u	**dirp,		// pointer to list of directories
    char_u	*old_fname)	// don't give warning for this file name
{
    char_u	*fname;
    int		n;
    char_u	*dir_name;
#ifdef AMIGA
    BPTR	fh;
#endif
    int		r;
    char_u	*buf_fname = buf->b_fname;

#if !defined(UNIX)
# define CREATE_DUMMY_FILE
    FILE	*dummyfd = NULL;

# ifdef MSWIN
    if (buf_fname != NULL && !mch_isFullName(buf_fname)
				       && vim_strchr(gettail(buf_fname), ':'))
    {
	char_u *t;

	buf_fname = vim_strsave(buf_fname);
	if (buf_fname == NULL)
	    buf_fname = buf->b_fname;
	else
	    for (t = gettail(buf_fname); *t != NUL; MB_PTR_ADV(t))
		if (*t == ':')
		    *t = '%';
    }
# endif

    /*
     * If we start editing a new file, e.g. "test.doc", which resides on an
     * MSDOS compatible filesystem, it is possible that the file
     * "test.doc.swp" which we create will be exactly the same file. To avoid
     * this problem we temporarily create "test.doc".  Don't do this when the
     * check below for an 8.3 file name is used.
     */
    if (!(buf->b_p_sn || buf->b_shortname) && buf_fname != NULL
					     && mch_getperm(buf_fname) < 0)
	dummyfd = mch_fopen((char *)buf_fname, "w");
#endif

    /*
     * Isolate a directory name from *dirp and put it in dir_name.
     * First allocate some memory to put the directory name in.
     */
    dir_name = alloc(STRLEN(*dirp) + 1);
    if (dir_name == NULL)
	*dirp = NULL;
    else
	(void)copy_option_part(dirp, dir_name, 31000, ",");

    /*
     * we try different names until we find one that does not exist yet
     */
    if (dir_name == NULL)	    // out of memory
	fname = NULL;
    else
	fname = makeswapname(buf_fname, buf->b_ffname, buf, dir_name);

    for (;;)
    {
	if (fname == NULL)	// must be out of memory
	    break;
	if ((n = (int)STRLEN(fname)) == 0)	// safety check
	{
	    VIM_CLEAR(fname);
	    break;
	}
#if defined(UNIX)
/*
 * Some systems have a MS-DOS compatible filesystem that use 8.3 character
 * file names. If this is the first try and the swap file name does not fit in
 * 8.3, detect if this is the case, set shortname and try again.
 */
	if (fname[n - 2] == 'w' && fname[n - 1] == 'p'
					&& !(buf->b_p_sn || buf->b_shortname))
	{
	    char_u	    *tail;
	    char_u	    *fname2;
	    stat_T	    s1, s2;
	    int		    f1, f2;
	    int		    created1 = FALSE, created2 = FALSE;
	    int		    same = FALSE;

	    /*
	     * Check if swapfile name does not fit in 8.3:
	     * It either contains two dots, is longer than 8 chars, or starts
	     * with a dot.
	     */
	    tail = gettail(buf_fname);
	    if (       vim_strchr(tail, '.') != NULL
		    || STRLEN(tail) > (size_t)8
		    || *gettail(fname) == '.')
	    {
		fname2 = alloc(n + 2);
		if (fname2 != NULL)
		{
		    STRCPY(fname2, fname);
		    // if fname == "xx.xx.swp",	    fname2 = "xx.xx.swx"
		    // if fname == ".xx.swp",	    fname2 = ".xx.swpx"
		    // if fname == "123456789.swp", fname2 = "12345678x.swp"
		    if (vim_strchr(tail, '.') != NULL)
			fname2[n - 1] = 'x';
		    else if (*gettail(fname) == '.')
		    {
			fname2[n] = 'x';
			fname2[n + 1] = NUL;
		    }
		    else
			fname2[n - 5] += 1;
		    /*
		     * may need to create the files to be able to use mch_stat()
		     */
		    f1 = mch_open((char *)fname, O_RDONLY | O_EXTRA, 0);
		    if (f1 < 0)
		    {
			f1 = mch_open_rw((char *)fname,
					       O_RDWR|O_CREAT|O_EXCL|O_EXTRA);
			created1 = TRUE;
		    }
		    if (f1 >= 0)
		    {
			f2 = mch_open((char *)fname2, O_RDONLY | O_EXTRA, 0);
			if (f2 < 0)
			{
			    f2 = mch_open_rw((char *)fname2,
					       O_RDWR|O_CREAT|O_EXCL|O_EXTRA);
			    created2 = TRUE;
			}
			if (f2 >= 0)
			{
			    /*
			     * Both files exist now. If mch_stat() returns the
			     * same device and inode they are the same file.
			     */
			    if (mch_fstat(f1, &s1) != -1
				    && mch_fstat(f2, &s2) != -1
				    && s1.st_dev == s2.st_dev
				    && s1.st_ino == s2.st_ino)
				same = TRUE;
			    close(f2);
			    if (created2)
				mch_remove(fname2);
			}
			close(f1);
			if (created1)
			    mch_remove(fname);
		    }
		    vim_free(fname2);
		    if (same)
		    {
			buf->b_shortname = TRUE;
			vim_free(fname);
			fname = makeswapname(buf_fname, buf->b_ffname,
							       buf, dir_name);
			continue;	// try again with b_shortname set
		    }
		}
	    }
	}
#endif
	/*
	 * check if the swapfile already exists
	 */
	if (mch_getperm(fname) < 0)	// it does not exist
	{
#ifdef HAVE_LSTAT
	    stat_T	sb;

	    /*
	     * Extra security check: When a swap file is a symbolic link, this
	     * is most likely a symlink attack.
	     */
	    if (mch_lstat((char *)fname, &sb) < 0)
#else
# ifdef AMIGA
	    fh = Open((UBYTE *)fname, (long)MODE_NEWFILE);
	    /*
	     * on the Amiga mch_getperm() will return -1 when the file exists
	     * but is being used by another program. This happens if you edit
	     * a file twice.
	     */
	    if (fh != (BPTR)NULL)	// can open file, OK
	    {
		Close(fh);
		mch_remove(fname);
		break;
	    }
	    if (IoErr() != ERROR_OBJECT_IN_USE
					    && IoErr() != ERROR_OBJECT_EXISTS)
# endif
#endif
		break;
	}

	/*
	 * A file name equal to old_fname is OK to use.
	 */
	if (old_fname != NULL && fnamecmp(fname, old_fname) == 0)
	    break;

	/*
	 * get here when file already exists
	 */
	if (fname[n - 2] == 'w' && fname[n - 1] == 'p')	// first try
	{
	    /*
	     * on MS-DOS compatible filesystems (e.g. messydos) file.doc.swp
	     * and file.doc are the same file. To guess if this problem is
	     * present try if file.doc.swx exists. If it does, we set
	     * buf->b_shortname and try file_doc.swp (dots replaced by
	     * underscores for this file), and try again. If it doesn't we
	     * assume that "file.doc.swp" already exists.
	     */
	    if (!(buf->b_p_sn || buf->b_shortname))	// not tried yet
	    {
		fname[n - 1] = 'x';
		r = mch_getperm(fname);		// try "file.swx"
		fname[n - 1] = 'p';
		if (r >= 0)		    // "file.swx" seems to exist
		{
		    buf->b_shortname = TRUE;
		    vim_free(fname);
		    fname = makeswapname(buf_fname, buf->b_ffname,
							       buf, dir_name);
		    continue;	    // try again with '.' replaced with '_'
		}
	    }
	    /*
	     * If we get here the ".swp" file really exists.
	     * Give an error message, unless recovering, no file name, we are
	     * viewing a help file or when the path of the file is different
	     * (happens when all .swp files are in one directory).
	     */
	    if (!recoverymode && buf_fname != NULL
				&& !buf->b_help
				&& !(buf->b_flags & (BF_DUMMY | BF_NO_SEA)))
	    {
		int		fd;
		struct block0	b0;
		int		differ = FALSE;

		/*
		 * Try to read block 0 from the swap file to get the original
		 * file name (and inode number).
		 */
		fd = mch_open((char *)fname, O_RDONLY | O_EXTRA, 0);
		if (fd >= 0)
		{
		    if (read_eintr(fd, &b0, sizeof(b0)) == sizeof(b0))
		    {
			/*
			 * If the swapfile has the same directory as the
			 * buffer don't compare the directory names, they can
			 * have a different mountpoint.
			 */
			if (b0.b0_flags & B0_SAME_DIR)
			{
			    if (fnamecmp(gettail(buf->b_ffname),
						   gettail(b0.b0_fname)) != 0
				    || !same_directory(fname, buf->b_ffname))
			    {
#ifdef CHECK_INODE
				// Symlinks may point to the same file even
				// when the name differs, need to check the
				// inode too.
				expand_env(b0.b0_fname, NameBuff, MAXPATHL);
				if (fnamecmp_ino(buf->b_ffname, NameBuff,
						     char_to_long(b0.b0_ino)))
#endif
				    differ = TRUE;
			    }
			}
			else
			{
			    /*
			     * The name in the swap file may be
			     * "~user/path/file".  Expand it first.
			     */
			    expand_env(b0.b0_fname, NameBuff, MAXPATHL);
#ifdef CHECK_INODE
			    if (fnamecmp_ino(buf->b_ffname, NameBuff,
						     char_to_long(b0.b0_ino)))
				differ = TRUE;
#else
			    if (fnamecmp(NameBuff, buf->b_ffname) != 0)
				differ = TRUE;
#endif
			}
		    }
		    close(fd);
		}

		// give the ATTENTION message when there is an old swap file
		// for the current file, and the buffer was not recovered.
		if (differ == FALSE && !(curbuf->b_flags & BF_RECOVERED)
			&& vim_strchr(p_shm, SHM_ATTENTION) == NULL)
		{
		    sea_choice_T choice = SEA_CHOICE_NONE;
		    stat_T	 st;
#ifdef CREATE_DUMMY_FILE
		    int		 did_use_dummy = FALSE;

		    // Avoid getting a warning for the file being created
		    // outside of Vim, it was created at the start of this
		    // function.  Delete the file now, because Vim might exit
		    // here if the window is closed.
		    if (dummyfd != NULL)
		    {
			fclose(dummyfd);
			dummyfd = NULL;
			mch_remove(buf_fname);
			did_use_dummy = TRUE;
		    }
#endif

#ifdef HAVE_PROCESS_STILL_RUNNING
		    process_still_running = FALSE;
#endif
		    // It's safe to delete the swap file if all these are true:
		    // - the edited file exists
		    // - the swap file has no changes and looks OK
		    if (mch_stat((char *)buf->b_fname, &st) == 0
						  && swapfile_unchanged(fname))
		    {
			choice = SEA_CHOICE_DELETE;
			if (p_verbose > 0)
			    verb_msg(_("Found a swap file that is not useful, deleting it"));
		    }

#if defined(FEAT_EVAL)
		    /*
		     * If there is an SwapExists autocommand and we can handle
		     * the response, trigger it.  It may return 0 to ask the
		     * user anyway.
		     */
		    if (choice == SEA_CHOICE_NONE
			    && swap_exists_action != SEA_NONE
			    && has_autocmd(EVENT_SWAPEXISTS, buf_fname, buf))
			choice = do_swapexists(buf, fname);
#endif

		    if (choice == SEA_CHOICE_NONE
					 && swap_exists_action == SEA_READONLY)
		    {
			// always open readonly.
			choice = SEA_CHOICE_READONLY;
		    }

		    if (choice == SEA_CHOICE_NONE)
		    {
#ifdef FEAT_GUI
			// If we are supposed to start the GUI but it wasn't
			// completely started yet, start it now.  This makes
			// the messages displayed in the Vim window when
			// loading a session from the .gvimrc file.
			if (gui.starting && !gui.in_use)
			    gui_start(NULL);
#endif
			// Show info about the existing swap file.
			attention_message(buf, fname);

			// We don't want a 'q' typed at the more-prompt
			// interrupt loading a file.
			got_int = FALSE;

			// If vimrc has "simalt ~x" we don't want it to
			// interfere with the prompt here.
			flush_buffers(FLUSH_TYPEAHEAD);
		    }

#if defined(FEAT_GUI_DIALOG) || defined(FEAT_CON_DIALOG)
		    if (swap_exists_action != SEA_NONE
						  && choice == SEA_CHOICE_NONE)
		    {
			char_u	*name;
			int	dialog_result;

			name = alloc(STRLEN(fname)
				+ STRLEN(_("Swap file \""))
				+ STRLEN(_("\" already exists!")) + 5);
			if (name != NULL)
			{
			    STRCPY(name, _("Swap file \""));
			    home_replace(NULL, fname, name + STRLEN(name),
								  1000, TRUE);
			    STRCAT(name, _("\" already exists!"));
			}
			dialog_result = do_dialog(VIM_WARNING,
				    (char_u *)_("VIM - ATTENTION"),
				    name == NULL
					?  (char_u *)_("Swap file already exists!")
					: name,
# ifdef HAVE_PROCESS_STILL_RUNNING
				    process_still_running
					? (char_u *)_("&Open Read-Only\n&Edit anyway\n&Recover\n&Quit\n&Abort") :
# endif
					(char_u *)_("&Open Read-Only\n&Edit anyway\n&Recover\n&Delete it\n&Quit\n&Abort"), 1, NULL, FALSE);

# ifdef HAVE_PROCESS_STILL_RUNNING
			if (process_still_running && dialog_result >= 4)
			    // compensate for missing "Delete it" button
			    dialog_result++;
# endif
			choice = dialog_result;
			vim_free(name);

			// pretend screen didn't scroll, need redraw anyway
			msg_scrolled = 0;
			redraw_all_later(UPD_NOT_VALID);
		    }
#endif

		    switch (choice)
		    {
			case SEA_CHOICE_READONLY:
			    buf->b_p_ro = TRUE;
			    break;
			case SEA_CHOICE_EDIT:
			    break;
			case SEA_CHOICE_RECOVER:
			    swap_exists_action = SEA_RECOVER;
			    break;
			case SEA_CHOICE_DELETE:
			    mch_remove(fname);
			    break;
			case SEA_CHOICE_QUIT:
			    swap_exists_action = SEA_QUIT;
			    break;
			case SEA_CHOICE_ABORT:
			    swap_exists_action = SEA_QUIT;
			    got_int = TRUE;
			    break;
			case SEA_CHOICE_NONE:
			    msg_puts("\n");
			    if (msg_silent == 0)
				// call wait_return() later
				need_wait_return = TRUE;
			    break;
		    }

		    // If the file was deleted this fname can be used.
		    if (choice != SEA_CHOICE_NONE && mch_getperm(fname) < 0)
			break;

#ifdef CREATE_DUMMY_FILE
		    // Going to try another name, need the dummy file again.
		    if (did_use_dummy)
			dummyfd = mch_fopen((char *)buf_fname, "w");
#endif
		}
	    }
	}

	/*
	 * Change the ".swp" extension to find another file that can be used.
	 * First decrement the last char: ".swo", ".swn", etc.
	 * If that still isn't enough decrement the last but one char: ".svz"
	 * Can happen when editing many "No Name" buffers.
	 */
	if (fname[n - 1] == 'a')	// ".s?a"
	{
	    if (fname[n - 2] == 'a')    // ".saa": tried enough, give up
	    {
		emsg(_(e_too_many_swap_files_found));
		VIM_CLEAR(fname);
		break;
	    }
	    --fname[n - 2];		// ".svz", ".suz", etc.
	    fname[n - 1] = 'z' + 1;
	}
	--fname[n - 1];			// ".swo", ".swn", etc.
    }

    vim_free(dir_name);
#ifdef CREATE_DUMMY_FILE
    if (dummyfd != NULL)	// file has been created temporarily
    {
	fclose(dummyfd);
	mch_remove(buf_fname);
    }
#endif
#ifdef MSWIN
    if (buf_fname != buf->b_fname)
	vim_free(buf_fname);
#endif
    return fname;
}

    static int
b0_magic_wrong(ZERO_BL *b0p)
{
    return (b0p->b0_magic_long != (long)B0_MAGIC_LONG
	    || b0p->b0_magic_int != (int)B0_MAGIC_INT
	    || b0p->b0_magic_short != (short)B0_MAGIC_SHORT
	    || b0p->b0_magic_char != B0_MAGIC_CHAR);
}

#ifdef CHECK_INODE
/*
 * Compare current file name with file name from swap file.
 * Try to use inode numbers when possible.
 * Return non-zero when files are different.
 *
 * When comparing file names a few things have to be taken into consideration:
 * - When working over a network the full path of a file depends on the host.
 *   We check the inode number if possible.  It is not 100% reliable though,
 *   because the device number cannot be used over a network.
 * - When a file does not exist yet (editing a new file) there is no inode
 *   number.
 * - The file name in a swap file may not be valid on the current host.  The
 *   "~user" form is used whenever possible to avoid this.
 *
 * This is getting complicated, let's make a table:
 *
 *		ino_c  ino_s  fname_c  fname_s	differ =
 *
 * both files exist -> compare inode numbers:
 *		!= 0   != 0	X	 X	ino_c != ino_s
 *
 * inode number(s) unknown, file names available -> compare file names
 *		== 0	X	OK	 OK	fname_c != fname_s
 *		 X     == 0	OK	 OK	fname_c != fname_s
 *
 * current file doesn't exist, file for swap file exist, file name(s) not
 * available -> probably different
 *		== 0   != 0    FAIL	 X	TRUE
 *		== 0   != 0	X	FAIL	TRUE
 *
 * current file exists, inode for swap unknown, file name(s) not
 * available -> probably different
 *		!= 0   == 0    FAIL	 X	TRUE
 *		!= 0   == 0	X	FAIL	TRUE
 *
 * current file doesn't exist, inode for swap unknown, one file name not
 * available -> probably different
 *		== 0   == 0    FAIL	 OK	TRUE
 *		== 0   == 0	OK	FAIL	TRUE
 *
 * current file doesn't exist, inode for swap unknown, both file names not
 * available -> compare file names
 *		== 0   == 0    FAIL	FAIL	fname_c != fname_s
 *
 * Note that when the ino_t is 64 bits, only the last 32 will be used.  This
 * can't be changed without making the block 0 incompatible with 32 bit
 * versions.
 */

    static int
fnamecmp_ino(
    char_u	*fname_c,	    // current file name
    char_u	*fname_s,	    // file name from swap file
    long	ino_block0)
{
    stat_T	st;
    ino_t	ino_c = 0;	    // ino of current file
    ino_t	ino_s;		    // ino of file from swap file
    char_u	buf_c[MAXPATHL];    // full path of fname_c
    char_u	buf_s[MAXPATHL];    // full path of fname_s
    int		retval_c;	    // flag: buf_c valid
    int		retval_s;	    // flag: buf_s valid

    if (mch_stat((char *)fname_c, &st) == 0)
	ino_c = (ino_t)st.st_ino;

    /*
     * First we try to get the inode from the file name, because the inode in
     * the swap file may be outdated.  If that fails (e.g. this path is not
     * valid on this machine), use the inode from block 0.
     */
    if (mch_stat((char *)fname_s, &st) == 0)
	ino_s = (ino_t)st.st_ino;
    else
	ino_s = (ino_t)ino_block0;

    if (ino_c && ino_s)
	return (ino_c != ino_s);

    /*
     * One of the inode numbers is unknown, try a forced vim_FullName() and
     * compare the file names.
     */
    retval_c = vim_FullName(fname_c, buf_c, MAXPATHL, TRUE);
    retval_s = vim_FullName(fname_s, buf_s, MAXPATHL, TRUE);
    if (retval_c == OK && retval_s == OK)
	return STRCMP(buf_c, buf_s) != 0;

    /*
     * Can't compare inodes or file names, guess that the files are different,
     * unless both appear not to exist at all, then compare with the file name
     * in the swap file.
     */
    if (ino_s == 0 && ino_c == 0 && retval_c == FAIL && retval_s == FAIL)
	return STRCMP(fname_c, fname_s) != 0;
    return TRUE;
}
#endif // CHECK_INODE

/*
 * Move a long integer into a four byte character array.
 * Used for machine independency in block zero.
 */
    static void
long_to_char(long n, char_u *s)
{
    s[0] = (char_u)(n & 0xff);
    n = (unsigned)n >> 8;
    s[1] = (char_u)(n & 0xff);
    n = (unsigned)n >> 8;
    s[2] = (char_u)(n & 0xff);
    n = (unsigned)n >> 8;
    s[3] = (char_u)(n & 0xff);
}

    static long
char_to_long(char_u *s)
{
    long    retval;

    retval = s[3];
    retval <<= 8;
    retval |= s[2];
    retval <<= 8;
    retval |= s[1];
    retval <<= 8;
    retval |= s[0];

    return retval;
}

/*
 * Set the flags in the first block of the swap file:
 * - file is modified or not: buf->b_changed
 * - 'fileformat'
 * - 'fileencoding'
 */
    void
ml_setflags(buf_T *buf)
{
    bhdr_T	*hp;
    ZERO_BL	*b0p;

    if (!buf->b_ml.ml_mfp)
	return;
    for (hp = buf->b_ml.ml_mfp->mf_used_last; hp != NULL; hp = hp->bh_prev)
    {
	if (hp->bh_bnum == 0)
	{
	    b0p = (ZERO_BL *)(hp->bh_data);
	    b0p->b0_dirty = buf->b_changed ? B0_DIRTY : 0;
	    b0p->b0_flags = (b0p->b0_flags & ~B0_FF_MASK)
						  | (get_fileformat(buf) + 1);
	    add_b0_fenc(b0p, buf);
	    hp->bh_flags |= BH_DIRTY;
	    mf_sync(buf->b_ml.ml_mfp, MFS_ZERO);
	    break;
	}
    }
}

#if defined(FEAT_CRYPT) || defined(PROTO)
/*
 * If "data" points to a data block encrypt the text in it and return a copy
 * in allocated memory.  Return NULL when out of memory.
 * Otherwise return "data".
 */
    char_u *
ml_encrypt_data(
    memfile_T	*mfp,
    char_u	*data,
    off_T	offset,
    unsigned	size)
{
    DATA_BL	*dp = (DATA_BL *)data;
    char_u	*head_end;
    char_u	*text_start;
    char_u	*new_data;
    int		text_len;
    cryptstate_T *state;

    if (dp->db_id != DATA_ID)
	return data;

    state = ml_crypt_prepare(mfp, offset, FALSE);
    if (state == NULL)
	return data;

    new_data = alloc(size);
    if (new_data == NULL)
	return NULL;
    head_end = (char_u *)(&dp->db_index[dp->db_line_count]);
    text_start = (char_u *)dp + dp->db_txt_start;
    text_len = size - dp->db_txt_start;

    // Copy the header and the text.
    mch_memmove(new_data, dp, head_end - (char_u *)dp);

    // Encrypt the text.
    crypt_encode(state, text_start, text_len, new_data + dp->db_txt_start,
									FALSE);
    crypt_free_state(state);

    // Clear the gap.
    if (head_end < text_start)
	vim_memset(new_data + (head_end - data), 0, text_start - head_end);

    return new_data;
}

/*
 * Decrypt the text in "data" if it points to an encrypted data block.
 */
    void
ml_decrypt_data(
    memfile_T	*mfp,
    char_u	*data,
    off_T	offset,
    unsigned	size)
{
    DATA_BL	*dp = (DATA_BL *)data;
    char_u	*head_end;
    char_u	*text_start;
    int		text_len;
    cryptstate_T *state;

    if (dp->db_id != DATA_ID)
	return;

    head_end = (char_u *)(&dp->db_index[dp->db_line_count]);
    text_start = (char_u *)dp + dp->db_txt_start;
    text_len = dp->db_txt_end - dp->db_txt_start;

    if (head_end > text_start || dp->db_txt_start > size
	    || dp->db_txt_end > size)
	return;  // data was messed up

    state = ml_crypt_prepare(mfp, offset, TRUE);
    if (state == NULL)
	return;

    // Decrypt the text in place.
    crypt_decode_inplace(state, text_start, text_len, FALSE);
    crypt_free_state(state);
}

/*
 * Prepare for encryption/decryption, using the key, seed and offset.
 * Return an allocated cryptstate_T *.
 * Note: Encryption not supported for SODIUM
 */
    static cryptstate_T *
ml_crypt_prepare(memfile_T *mfp, off_T offset, int reading)
{
    buf_T	*buf = mfp->mf_buffer;
    char_u	salt[50];
    int		method_nr;
    char_u	*key;
    crypt_arg_T arg;

    CLEAR_FIELD(arg);
    if (reading && mfp->mf_old_key != NULL)
    {
	// Reading back blocks with the previous key/method/seed.
	method_nr = mfp->mf_old_cm;
	key = mfp->mf_old_key;
	arg.cat_seed = mfp->mf_old_seed;
    }
    else
    {
	method_nr = crypt_get_method_nr(buf);
	key = buf->b_p_key;
	arg.cat_seed = mfp->mf_seed;
    }

    if (*key == NUL)
	return NULL;

    if (crypt_may_close_swapfile(buf, key, method_nr))
	return NULL;

    if (method_nr == CRYPT_M_ZIP)
    {
	// For PKzip: Append the offset to the key, so that we use a different
	// key for every block.
	vim_snprintf((char *)salt, sizeof(salt), "%s%ld", key, (long)offset);
	arg.cat_seed = NULL;
	arg.cat_init_from_file = FALSE;

	return crypt_create(method_nr, salt, &arg);
    }

    // Using blowfish or better: add salt and seed. We use the byte offset
    // of the block for the salt.
    vim_snprintf((char *)salt, sizeof(salt), "%ld", (long)offset);

    arg.cat_salt = salt;
    arg.cat_salt_len = (int)STRLEN(salt);
    arg.cat_seed_len = MF_SEED_LEN;
    arg.cat_add_len = 0;
    arg.cat_add = NULL;
    arg.cat_init_from_file = FALSE;

    return crypt_create(method_nr, key, &arg);
}

#endif


#if defined(FEAT_BYTEOFF) || defined(PROTO)

#define MLCS_MAXL 800	// max no of lines in chunk
#define MLCS_MINL 400   // should be half of MLCS_MAXL

/*
 * Keep information for finding byte offset of a line, updtype may be one of:
 * ML_CHNK_ADDLINE: Add len to parent chunk, possibly splitting it
 *	   Careful: ML_CHNK_ADDLINE may cause ml_find_line() to be called.
 * ML_CHNK_DELLINE: Subtract len from parent chunk, possibly deleting it
 * ML_CHNK_UPDLINE: Add len to parent chunk, as a signed entity.
 */
    static void
ml_updatechunk(
    buf_T	*buf,
    linenr_T	line,
    long	len,
    int		updtype)
{
    static buf_T	*ml_upd_lastbuf = NULL;
    static linenr_T	ml_upd_lastline;
    static linenr_T	ml_upd_lastcurline;
    static int		ml_upd_lastcurix;

    linenr_T		curline = ml_upd_lastcurline;
    int			curix = ml_upd_lastcurix;
    long		size;
    chunksize_T		*curchnk;
    int			rest;
    bhdr_T		*hp;
    DATA_BL		*dp;

    if (buf->b_ml.ml_usedchunks == -1 || len == 0)
	return;
    if (buf->b_ml.ml_chunksize == NULL)
    {
	buf->b_ml.ml_chunksize = ALLOC_MULT(chunksize_T, 100);
	if (buf->b_ml.ml_chunksize == NULL)
	{
	    buf->b_ml.ml_usedchunks = -1;
	    return;
	}
	buf->b_ml.ml_numchunks = 100;
	buf->b_ml.ml_usedchunks = 1;
	buf->b_ml.ml_chunksize[0].mlcs_numlines = 1;
	buf->b_ml.ml_chunksize[0].mlcs_totalsize = 1;
    }

    if (updtype == ML_CHNK_UPDLINE && buf->b_ml.ml_line_count == 1)
    {
	/*
	 * First line in empty buffer from ml_flush_line() -- reset
	 */
	buf->b_ml.ml_usedchunks = 1;
	buf->b_ml.ml_chunksize[0].mlcs_numlines = 1;
	buf->b_ml.ml_chunksize[0].mlcs_totalsize = (long)buf->b_ml.ml_line_len;
	return;
    }

    /*
     * Find chunk that our line belongs to, curline will be at start of the
     * chunk.
     */
    if (buf != ml_upd_lastbuf || line != ml_upd_lastline + 1
	    || updtype != ML_CHNK_ADDLINE)
    {
	for (curline = 1, curix = 0;
	     curix < buf->b_ml.ml_usedchunks - 1
	     && line >= curline + buf->b_ml.ml_chunksize[curix].mlcs_numlines;
	     curix++)
	    curline += buf->b_ml.ml_chunksize[curix].mlcs_numlines;
    }
    else if (curix < buf->b_ml.ml_usedchunks - 1
	      && line >= curline + buf->b_ml.ml_chunksize[curix].mlcs_numlines)
    {
	// Adjust cached curix & curline
	curline += buf->b_ml.ml_chunksize[curix].mlcs_numlines;
	curix++;
    }
    curchnk = buf->b_ml.ml_chunksize + curix;

    if (updtype == ML_CHNK_DELLINE)
	len = -len;
    curchnk->mlcs_totalsize += len;
    if (updtype == ML_CHNK_ADDLINE)
    {
	curchnk->mlcs_numlines++;

	// May resize here so we don't have to do it in both cases below
	if (buf->b_ml.ml_usedchunks + 1 >= buf->b_ml.ml_numchunks)
	{
	    chunksize_T *t_chunksize = buf->b_ml.ml_chunksize;

	    buf->b_ml.ml_numchunks = buf->b_ml.ml_numchunks * 3 / 2;
	    buf->b_ml.ml_chunksize = vim_realloc(buf->b_ml.ml_chunksize,
			    sizeof(chunksize_T) * buf->b_ml.ml_numchunks);
	    if (buf->b_ml.ml_chunksize == NULL)
	    {
		// Hmmmm, Give up on offset for this buffer
		vim_free(t_chunksize);
		buf->b_ml.ml_usedchunks = -1;
		return;
	    }
	}

	if (buf->b_ml.ml_chunksize[curix].mlcs_numlines >= MLCS_MAXL)
	{
	    int	    count;	    // number of entries in block
	    int	    idx;
	    int	    end_idx;
	    int	    text_end;
	    int	    linecnt;

	    mch_memmove(buf->b_ml.ml_chunksize + curix + 1,
			buf->b_ml.ml_chunksize + curix,
			(buf->b_ml.ml_usedchunks - curix) *
			sizeof(chunksize_T));
	    // Compute length of first half of lines in the split chunk
	    size = 0;
	    linecnt = 0;
	    while (curline < buf->b_ml.ml_line_count
			&& linecnt < MLCS_MINL)
	    {
		if ((hp = ml_find_line(buf, curline, ML_FIND)) == NULL)
		{
		    buf->b_ml.ml_usedchunks = -1;
		    return;
		}
		dp = (DATA_BL *)(hp->bh_data);
		count = (long)(buf->b_ml.ml_locked_high) -
			(long)(buf->b_ml.ml_locked_low) + 1;
		idx = curline - buf->b_ml.ml_locked_low;
		curline = buf->b_ml.ml_locked_high + 1;

		// compute index of last line to use in this MEMLINE
		rest = count - idx;
		if (linecnt + rest > MLCS_MINL)
		{
		    end_idx = idx + MLCS_MINL - linecnt - 1;
		    linecnt = MLCS_MINL;
		}
		else
		{
		    end_idx = count - 1;
		    linecnt += rest;
		}
#ifdef FEAT_PROP_POPUP
		if (buf->b_has_textprop)
		{
		    int i;

		    // We cannot use the text pointers to get the text length,
		    // the text prop info would also be counted.  Go over the
		    // lines.
		    for (i = end_idx; i < idx; ++i)
			size += (int)STRLEN((char_u *)dp
				      + (dp->db_index[i] & DB_INDEX_MASK)) + 1;
		}
		else
#endif
		{
		    if (idx == 0) // first line in block, text at the end
			text_end = dp->db_txt_end;
		    else
			text_end = ((dp->db_index[idx - 1]) & DB_INDEX_MASK);
		    size += text_end
				   - ((dp->db_index[end_idx]) & DB_INDEX_MASK);
		}
	    }
	    buf->b_ml.ml_chunksize[curix].mlcs_numlines = linecnt;
	    buf->b_ml.ml_chunksize[curix + 1].mlcs_numlines -= linecnt;
	    buf->b_ml.ml_chunksize[curix].mlcs_totalsize = size;
	    buf->b_ml.ml_chunksize[curix + 1].mlcs_totalsize -= size;
	    buf->b_ml.ml_usedchunks++;
	    ml_upd_lastbuf = NULL;   // Force recalc of curix & curline
	    return;
	}
	else if (buf->b_ml.ml_chunksize[curix].mlcs_numlines >= MLCS_MINL
		     && curix == buf->b_ml.ml_usedchunks - 1
		     && buf->b_ml.ml_line_count - line <= 1)
	{
	    /*
	     * We are in the last chunk and it is cheap to create a new one
	     * after this. Do it now to avoid the loop above later on
	     */
	    curchnk = buf->b_ml.ml_chunksize + curix + 1;
	    buf->b_ml.ml_usedchunks++;
	    if (line == buf->b_ml.ml_line_count)
	    {
		curchnk->mlcs_numlines = 0;
		curchnk->mlcs_totalsize = 0;
	    }
	    else
	    {
		/*
		 * Line is just prior to last, move count for last
		 * This is the common case  when loading a new file
		 */
		hp = ml_find_line(buf, buf->b_ml.ml_line_count, ML_FIND);
		if (hp == NULL)
		{
		    buf->b_ml.ml_usedchunks = -1;
		    return;
		}
		dp = (DATA_BL *)(hp->bh_data);
		if (dp->db_line_count == 1)
		    rest = dp->db_txt_end - dp->db_txt_start;
		else
		    rest =
			((dp->db_index[dp->db_line_count - 2]) & DB_INDEX_MASK)
			- dp->db_txt_start;
		curchnk->mlcs_totalsize = rest;
		curchnk->mlcs_numlines = 1;
		curchnk[-1].mlcs_totalsize -= rest;
		curchnk[-1].mlcs_numlines -= 1;
	    }
	}
    }
    else if (updtype == ML_CHNK_DELLINE)
    {
	curchnk->mlcs_numlines--;
	ml_upd_lastbuf = NULL;   // Force recalc of curix & curline
	if (curix < buf->b_ml.ml_usedchunks - 1
		&& curchnk->mlcs_numlines + curchnk[1].mlcs_numlines
								  <= MLCS_MINL)
	{
	    curix++;
	    curchnk = buf->b_ml.ml_chunksize + curix;
	}
	else if (curix == 0 && curchnk->mlcs_numlines <= 0)
	{
	    buf->b_ml.ml_usedchunks--;
	    mch_memmove(buf->b_ml.ml_chunksize, buf->b_ml.ml_chunksize + 1,
			buf->b_ml.ml_usedchunks * sizeof(chunksize_T));
	    return;
	}
	else if (curix == 0 || (curchnk->mlcs_numlines > 10
		    && curchnk->mlcs_numlines + curchnk[-1].mlcs_numlines
								  > MLCS_MINL))
	{
	    return;
	}

	// Collapse chunks
	curchnk[-1].mlcs_numlines += curchnk->mlcs_numlines;
	curchnk[-1].mlcs_totalsize += curchnk->mlcs_totalsize;
	buf->b_ml.ml_usedchunks--;
	if (curix < buf->b_ml.ml_usedchunks)
	    mch_memmove(buf->b_ml.ml_chunksize + curix,
			buf->b_ml.ml_chunksize + curix + 1,
			(buf->b_ml.ml_usedchunks - curix) *
			sizeof(chunksize_T));
	return;
    }
    ml_upd_lastbuf = buf;
    ml_upd_lastline = line;
    ml_upd_lastcurline = curline;
    ml_upd_lastcurix = curix;
}

/*
 * Find offset for line or line with offset.
 * Find line with offset if "lnum" is 0; return remaining offset in offp
 * Find offset of line if "lnum" > 0
 * return -1 if information is not available
 */
    long
ml_find_line_or_offset(buf_T *buf, linenr_T lnum, long *offp)
{
    linenr_T	curline;
    int		curix;
    long	size;
    bhdr_T	*hp;
    DATA_BL	*dp;
    int		count;		// number of entries in block
    int		idx;
    int		start_idx;
    int		text_end;
    long	offset;
    int		len;
    int		ffdos = (get_fileformat(buf) == EOL_DOS);
    int		extra = 0;

    // take care of cached line first
    ml_flush_line(curbuf);

    if (buf->b_ml.ml_usedchunks == -1
	    || buf->b_ml.ml_chunksize == NULL
	    || lnum < 0)
	return -1;

    if (offp == NULL)
	offset = 0;
    else
	offset = *offp;
    if (lnum == 0 && offset <= 0)
	return 1;   // Not a "find offset" and offset 0 _must_ be in line 1
    /*
     * Find the last chunk before the one containing our line. Last chunk is
     * special because it will never qualify.
     */
    curline = 1;
    curix = size = 0;
    while (curix < buf->b_ml.ml_usedchunks - 1
	    && ((lnum != 0
	     && lnum >= curline + buf->b_ml.ml_chunksize[curix].mlcs_numlines)
		|| (offset != 0
	       && offset > size + buf->b_ml.ml_chunksize[curix].mlcs_totalsize
		 + (long)ffdos * buf->b_ml.ml_chunksize[curix].mlcs_numlines)))
    {
	curline += buf->b_ml.ml_chunksize[curix].mlcs_numlines;
	size += buf->b_ml.ml_chunksize[curix].mlcs_totalsize;
	if (offset && ffdos)
	    size += buf->b_ml.ml_chunksize[curix].mlcs_numlines;
	curix++;
    }

    while ((lnum != 0 && curline < lnum) || (offset != 0 && size < offset))
    {
#ifdef FEAT_PROP_POPUP
	size_t textprop_total = 0;
#endif

	if (curline > buf->b_ml.ml_line_count
		|| (hp = ml_find_line(buf, curline, ML_FIND)) == NULL)
	    return -1;
	dp = (DATA_BL *)(hp->bh_data);
	count = (long)(buf->b_ml.ml_locked_high) -
		(long)(buf->b_ml.ml_locked_low) + 1;
	start_idx = idx = curline - buf->b_ml.ml_locked_low;
	if (idx == 0)  // first line in block, text at the end
	    text_end = dp->db_txt_end;
	else
	    text_end = ((dp->db_index[idx - 1]) & DB_INDEX_MASK);
	// Compute index of last line to use in this MEMLINE
	if (lnum != 0)
	{
	    if (curline + (count - idx) >= lnum)
		idx += lnum - curline - 1;
	    else
		idx = count - 1;
	}
	else
	{
	    extra = 0;
	    for (;;)
	    {
#ifdef FEAT_PROP_POPUP
		size_t textprop_size = 0;

		if (buf->b_has_textprop)
		{
		    char_u *l1, *l2;

		    // compensate for the extra bytes taken by textprops
		    l1 = (char_u *)dp + ((dp->db_index[idx]) & DB_INDEX_MASK);
		    l2 = (char_u *)dp + (idx == 0 ? dp->db_txt_end
				  : ((dp->db_index[idx - 1]) & DB_INDEX_MASK));
		    textprop_size = (l2 - l1) - (STRLEN(l1) + 1);
		}
#endif
		if (!(offset >= size
			+ text_end - (int)((dp->db_index[idx]) & DB_INDEX_MASK)
#ifdef FEAT_PROP_POPUP
			- (long)(textprop_total + textprop_size)
#endif
			+ ffdos))
		    break;

		if (ffdos)
		    size++;
#ifdef FEAT_PROP_POPUP
		textprop_total += textprop_size;
#endif
		if (idx == count - 1)
		{
		    extra = 1;
		    break;
		}
		idx++;
	    }
	}
#ifdef FEAT_PROP_POPUP
	if (buf->b_has_textprop && lnum != 0)
	{
	    int i;

	    // cannot use the db_index pointer, need to get the actual text
	    // lengths.
	    len = 0;
	    for (i = start_idx; i <= idx; ++i)
	    {
		char_u *p = (char_u *)dp + ((dp->db_index[i]) & DB_INDEX_MASK);
		len += (int)STRLEN(p) + 1;
	    }
	}
	else
#endif
	    len = text_end - ((dp->db_index[idx]) & DB_INDEX_MASK)
#ifdef FEAT_PROP_POPUP
				- (long)textprop_total
#endif
				;
	size += len;
	if (offset != 0 && size >= offset)
	{
	    if (size + ffdos == offset)
		*offp = 0;
	    else if (idx == start_idx)
		*offp = offset - size + len;
	    else
		*offp = offset - size + len
		     - (text_end - ((dp->db_index[idx - 1]) & DB_INDEX_MASK))
#ifdef FEAT_PROP_POPUP
		     + (long)textprop_total
#endif
		     ;
	    curline += idx - start_idx + extra;
	    if (curline > buf->b_ml.ml_line_count)
		return -1;	// exactly one byte beyond the end
	    return curline;
	}
	curline = buf->b_ml.ml_locked_high + 1;
    }

    if (lnum != 0)
    {
	// Count extra CR characters.
	if (ffdos)
	    size += lnum - 1;

	// Don't count the last line break if 'noeol' and ('bin' or
	// 'nofixeol').
	if ((!buf->b_p_fixeol || buf->b_p_bin) && !buf->b_p_eol
					   && lnum > buf->b_ml.ml_line_count)
	    size -= ffdos + 1;
    }

    return size;
}

/*
 * Goto byte in buffer with offset 'cnt'.
 */
    void
goto_byte(long cnt)
{
    long	boff = cnt;
    linenr_T	lnum;

    ml_flush_line(curbuf);	// cached line may be dirty
    setpcmark();
    if (boff)
	--boff;
    lnum = ml_find_line_or_offset(curbuf, (linenr_T)0, &boff);
    if (lnum < 1)	// past the end
    {
	curwin->w_cursor.lnum = curbuf->b_ml.ml_line_count;
	curwin->w_curswant = MAXCOL;
	coladvance((colnr_T)MAXCOL);
    }
    else
    {
	curwin->w_cursor.lnum = lnum;
	curwin->w_cursor.col = (colnr_T)boff;
	curwin->w_cursor.coladd = 0;
	curwin->w_set_curswant = TRUE;
    }
    check_cursor();

    // Make sure the cursor is on the first byte of a multi-byte char.
    if (has_mbyte)
	mb_adjust_cursor();
}
#endif
