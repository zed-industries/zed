/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * findfile.c: Search for files in directories listed in 'path'
 */

#include "vim.h"

/*
 * File searching functions for 'path', 'tags' and 'cdpath' options.
 * External visible functions:
 * vim_findfile_init()		creates/initialises the search context
 * vim_findfile_free_visited()	free list of visited files/dirs of search
 *				context
 * vim_findfile()		find a file in the search context
 * vim_findfile_cleanup()	cleanup/free search context created by
 *				vim_findfile_init()
 *
 * All static functions and variables start with 'ff_'
 *
 * In general it works like this:
 * First you create yourself a search context by calling vim_findfile_init().
 * It is possible to give a search context from a previous call to
 * vim_findfile_init(), so it can be reused. After this you call vim_findfile()
 * until you are satisfied with the result or it returns NULL. On every call it
 * returns the next file which matches the conditions given to
 * vim_findfile_init(). If it doesn't find a next file it returns NULL.
 *
 * It is possible to call vim_findfile_init() again to reinitialise your search
 * with some new parameters. Don't forget to pass your old search context to
 * it, so it can reuse it and especially reuse the list of already visited
 * directories. If you want to delete the list of already visited directories
 * simply call vim_findfile_free_visited().
 *
 * When you are done call vim_findfile_cleanup() to free the search context.
 *
 * The function vim_findfile_init() has a long comment, which describes the
 * needed parameters.
 *
 *
 *
 * ATTENTION:
 * ==========
 *	Also we use an allocated search context here, these functions are NOT
 *	thread-safe!
 *
 *	To minimize parameter passing (or because I'm to lazy), only the
 *	external visible functions get a search context as a parameter. This is
 *	then assigned to a static global, which is used throughout the local
 *	functions.
 */

/*
 * type for the directory search stack
 */
typedef struct ff_stack
{
    struct ff_stack	*ffs_prev;

    // the fix part (no wildcards) and the part containing the wildcards
    // of the search path
    char_u		*ffs_fix_path;
    char_u		*ffs_wc_path;

    // files/dirs found in the above directory, matched by the first wildcard
    // of wc_part
    char_u		**ffs_filearray;
    int			ffs_filearray_size;
    int			ffs_filearray_cur;   // needed for partly handled dirs

    // to store status of partly handled directories
    // 0: we work on this directory for the first time
    // 1: this directory was partly searched in an earlier step
    int			ffs_stage;

    // How deep are we in the directory tree?
    // Counts backward from value of level parameter to vim_findfile_init
    int			ffs_level;

    // Did we already expand '**' to an empty string?
    int			ffs_star_star_empty;
} ff_stack_T;

/*
 * type for already visited directories or files.
 */
typedef struct ff_visited
{
    struct ff_visited	*ffv_next;

    // Visited directories are different if the wildcard string are
    // different. So we have to save it.
    char_u		*ffv_wc_path;

    // for unix use inode etc for comparison (needed because of links), else
    // use filename.
#ifdef UNIX
    int			ffv_dev_valid;	// ffv_dev and ffv_ino were set
    dev_t		ffv_dev;	// device number
    ino_t		ffv_ino;	// inode number
#endif
    // The memory for this struct is allocated according to the length of
    // ffv_fname.
    char_u		ffv_fname[1];	// actually longer
} ff_visited_T;

/*
 * We might have to manage several visited lists during a search.
 * This is especially needed for the tags option. If tags is set to:
 *      "./++/tags,./++/TAGS,++/tags"  (replace + with *)
 * So we have to do 3 searches:
 *   1) search from the current files directory downward for the file "tags"
 *   2) search from the current files directory downward for the file "TAGS"
 *   3) search from Vims current directory downwards for the file "tags"
 * As you can see, the first and the third search are for the same file, so for
 * the third search we can use the visited list of the first search. For the
 * second search we must start from a empty visited list.
 * The struct ff_visited_list_hdr is used to manage a linked list of already
 * visited lists.
 */
typedef struct ff_visited_list_hdr
{
    struct ff_visited_list_hdr	*ffvl_next;

    // the filename the attached visited list is for
    char_u			*ffvl_filename;

    ff_visited_T		*ffvl_visited_list;

} ff_visited_list_hdr_T;


/*
 * '**' can be expanded to several directory levels.
 * Set the default maximum depth.
 */
#define FF_MAX_STAR_STAR_EXPAND ((char_u)30)

/*
 * The search context:
 *   ffsc_stack_ptr:	the stack for the dirs to search
 *   ffsc_visited_list: the currently active visited list
 *   ffsc_dir_visited_list: the currently active visited list for search dirs
 *   ffsc_visited_lists_list: the list of all visited lists
 *   ffsc_dir_visited_lists_list: the list of all visited lists for search dirs
 *   ffsc_file_to_search:     the file to search for
 *   ffsc_start_dir:	the starting directory, if search path was relative
 *   ffsc_fix_path:	the fix part of the given path (without wildcards)
 *			Needed for upward search.
 *   ffsc_wc_path:	the part of the given path containing wildcards
 *   ffsc_level:	how many levels of dirs to search downwards
 *   ffsc_stopdirs_v:	array of stop directories for upward search
 *   ffsc_find_what:	FINDFILE_BOTH, FINDFILE_DIR or FINDFILE_FILE
 *   ffsc_tagfile:	searching for tags file, don't use 'suffixesadd'
 */
typedef struct ff_search_ctx_T
{
     ff_stack_T			*ffsc_stack_ptr;
     ff_visited_list_hdr_T	*ffsc_visited_list;
     ff_visited_list_hdr_T	*ffsc_dir_visited_list;
     ff_visited_list_hdr_T	*ffsc_visited_lists_list;
     ff_visited_list_hdr_T	*ffsc_dir_visited_lists_list;
     char_u			*ffsc_file_to_search;
     char_u			*ffsc_start_dir;
     char_u			*ffsc_fix_path;
     char_u			*ffsc_wc_path;
     int			ffsc_level;
     char_u			**ffsc_stopdirs_v;
     int			ffsc_find_what;
     int			ffsc_tagfile;
} ff_search_ctx_T;

// locally needed functions
static int ff_check_visited(ff_visited_T **, char_u *, char_u *);
static void vim_findfile_free_visited(void *search_ctx_arg);
static void vim_findfile_free_visited_list(ff_visited_list_hdr_T **list_headp);
static void ff_free_visited_list(ff_visited_T *vl);
static ff_visited_list_hdr_T* ff_get_visited_list(char_u *, ff_visited_list_hdr_T **list_headp);

static void ff_push(ff_search_ctx_T *search_ctx, ff_stack_T *stack_ptr);
static ff_stack_T *ff_pop(ff_search_ctx_T *search_ctx);
static void ff_clear(ff_search_ctx_T *search_ctx);
static void ff_free_stack_element(ff_stack_T *stack_ptr);
static ff_stack_T *ff_create_stack_element(char_u *, char_u *, int, int);
static int ff_path_in_stoplist(char_u *, int, char_u **);

static char_u	*ff_expand_buffer = NULL; // used for expanding filenames

#if 0
/*
 * if someone likes findfirst/findnext, here are the functions
 * NOT TESTED!!
 */

static void *ff_fn_search_context = NULL;

    char_u *
vim_findfirst(char_u *path, char_u *filename, int level)
{
    ff_fn_search_context =
	vim_findfile_init(path, filename, NULL, level, TRUE, FALSE,
		ff_fn_search_context, rel_fname);
    if (NULL == ff_fn_search_context)
	return NULL;
    else
	return vim_findnext()
}

    char_u *
vim_findnext(void)
{
    char_u *ret = vim_findfile(ff_fn_search_context);

    if (NULL == ret)
    {
	vim_findfile_cleanup(ff_fn_search_context);
	ff_fn_search_context = NULL;
    }
    return ret;
}
#endif

/*
 * Initialization routine for vim_findfile().
 *
 * Returns the newly allocated search context or NULL if an error occurred.
 *
 * Don't forget to clean up by calling vim_findfile_cleanup() if you are done
 * with the search context.
 *
 * Find the file 'filename' in the directory 'path'.
 * The parameter 'path' may contain wildcards. If so only search 'level'
 * directories deep. The parameter 'level' is the absolute maximum and is
 * not related to restricts given to the '**' wildcard. If 'level' is 100
 * and you use '**200' vim_findfile() will stop after 100 levels.
 *
 * 'filename' cannot contain wildcards!  It is used as-is, no backslashes to
 * escape special characters.
 *
 * If 'stopdirs' is not NULL and nothing is found downward, the search is
 * restarted on the next higher directory level. This is repeated until the
 * start-directory of a search is contained in 'stopdirs'. 'stopdirs' has the
 * format ";*<dirname>*\(;<dirname>\)*;\=$".
 *
 * If the 'path' is relative, the starting dir for the search is either VIM's
 * current dir or if the path starts with "./" the current files dir.
 * If the 'path' is absolute, the starting dir is that part of the path before
 * the first wildcard.
 *
 * Upward search is only done on the starting dir.
 *
 * If 'free_visited' is TRUE the list of already visited files/directories is
 * cleared. Set this to FALSE if you just want to search from another
 * directory, but want to be sure that no directory from a previous search is
 * searched again. This is useful if you search for a file at different places.
 * The list of visited files/dirs can also be cleared with the function
 * vim_findfile_free_visited().
 *
 * Set the parameter 'find_what' to FINDFILE_DIR if you want to search for
 * directories only, FINDFILE_FILE for files only, FINDFILE_BOTH for both.
 *
 * A search context returned by a previous call to vim_findfile_init() can be
 * passed in the parameter "search_ctx_arg".  This context is reused and
 * reinitialized with the new parameters.  The list of already visited
 * directories from this context is only deleted if the parameter
 * "free_visited" is true.  Be aware that the passed "search_ctx_arg" is freed
 * if the reinitialization fails.
 *
 * If you don't have a search context from a previous call "search_ctx_arg"
 * must be NULL.
 *
 * This function silently ignores a few errors, vim_findfile() will have
 * limited functionality then.
 */
    void *
vim_findfile_init(
    char_u	*path,
    char_u	*filename,
    char_u	*stopdirs UNUSED,
    int		level,
    int		free_visited,
    int		find_what,
    void	*search_ctx_arg,
    int		tagfile,	// expanding names of tags files
    char_u	*rel_fname)	// file name to use for "."
{
    char_u		*wc_part;
    ff_stack_T		*sptr;
    ff_search_ctx_T	*search_ctx;

    // If a search context is given by the caller, reuse it, else allocate a
    // new one.
    if (search_ctx_arg != NULL)
	search_ctx = search_ctx_arg;
    else
    {
	search_ctx = ALLOC_CLEAR_ONE(ff_search_ctx_T);
	if (search_ctx == NULL)
	    goto error_return;
    }
    search_ctx->ffsc_find_what = find_what;
    search_ctx->ffsc_tagfile = tagfile;

    // clear the search context, but NOT the visited lists
    ff_clear(search_ctx);

    // clear visited list if wanted
    if (free_visited == TRUE)
	vim_findfile_free_visited(search_ctx);
    else
    {
	// Reuse old visited lists. Get the visited list for the given
	// filename. If no list for the current filename exists, creates a new
	// one.
	search_ctx->ffsc_visited_list = ff_get_visited_list(filename,
					&search_ctx->ffsc_visited_lists_list);
	if (search_ctx->ffsc_visited_list == NULL)
	    goto error_return;
	search_ctx->ffsc_dir_visited_list = ff_get_visited_list(filename,
				    &search_ctx->ffsc_dir_visited_lists_list);
	if (search_ctx->ffsc_dir_visited_list == NULL)
	    goto error_return;
    }

    if (ff_expand_buffer == NULL)
    {
	ff_expand_buffer = alloc(MAXPATHL);
	if (ff_expand_buffer == NULL)
	    goto error_return;
    }

    // Store information on starting dir now if path is relative.
    // If path is absolute, we do that later.
    if (path[0] == '.'
	    && (vim_ispathsep(path[1]) || path[1] == NUL)
	    && (!tagfile || vim_strchr(p_cpo, CPO_DOTTAG) == NULL)
	    && rel_fname != NULL)
    {
	int	len = (int)(gettail(rel_fname) - rel_fname);

	if (!vim_isAbsName(rel_fname) && len + 1 < MAXPATHL)
	{
	    // Make the start dir an absolute path name.
	    vim_strncpy(ff_expand_buffer, rel_fname, len);
	    search_ctx->ffsc_start_dir = FullName_save(ff_expand_buffer, FALSE);
	}
	else
	    search_ctx->ffsc_start_dir = vim_strnsave(rel_fname, len);
	if (search_ctx->ffsc_start_dir == NULL)
	    goto error_return;
	if (*++path != NUL)
	    ++path;
    }
    else if (*path == NUL || !vim_isAbsName(path))
    {
#ifdef BACKSLASH_IN_FILENAME
	// "c:dir" needs "c:" to be expanded, otherwise use current dir
	if (*path != NUL && path[1] == ':')
	{
	    char_u  drive[3];

	    drive[0] = path[0];
	    drive[1] = ':';
	    drive[2] = NUL;
	    if (vim_FullName(drive, ff_expand_buffer, MAXPATHL, TRUE) == FAIL)
		goto error_return;
	    path += 2;
	}
	else
#endif
	if (mch_dirname(ff_expand_buffer, MAXPATHL) == FAIL)
	    goto error_return;

	search_ctx->ffsc_start_dir = vim_strsave(ff_expand_buffer);
	if (search_ctx->ffsc_start_dir == NULL)
	    goto error_return;

#ifdef BACKSLASH_IN_FILENAME
	// A path that starts with "/dir" is relative to the drive, not to the
	// directory (but not for "//machine/dir").  Only use the drive name.
	if ((*path == '/' || *path == '\\')
		&& path[1] != path[0]
		&& search_ctx->ffsc_start_dir[1] == ':')
	    search_ctx->ffsc_start_dir[2] = NUL;
#endif
    }

    /*
     * If stopdirs are given, split them into an array of pointers.
     * If this fails (mem allocation), there is no upward search at all or a
     * stop directory is not recognized -> continue silently.
     * If stopdirs just contains a ";" or is empty,
     * search_ctx->ffsc_stopdirs_v will only contain a  NULL pointer. This
     * is handled as unlimited upward search.  See function
     * ff_path_in_stoplist() for details.
     */
    if (stopdirs != NULL)
    {
	char_u	*walker = stopdirs;
	int	dircount;

	while (*walker == ';')
	    walker++;

	dircount = 1;
	search_ctx->ffsc_stopdirs_v = ALLOC_ONE(char_u *);

	if (search_ctx->ffsc_stopdirs_v != NULL)
	{
	    do
	    {
		char_u	*helper;
		void	*ptr;

		helper = walker;
		ptr = vim_realloc(search_ctx->ffsc_stopdirs_v,
					   (dircount + 1) * sizeof(char_u *));
		if (ptr)
		    search_ctx->ffsc_stopdirs_v = ptr;
		else
		    // ignore, keep what we have and continue
		    break;
		walker = vim_strchr(walker, ';');
		if (walker)
		{
		    search_ctx->ffsc_stopdirs_v[dircount-1] =
					 vim_strnsave(helper, walker - helper);
		    walker++;
		}
		else
		    // this might be "", which means ascent till top
		    // of directory tree.
		    search_ctx->ffsc_stopdirs_v[dircount-1] =
							  vim_strsave(helper);

		dircount++;

	    } while (walker != NULL);
	    search_ctx->ffsc_stopdirs_v[dircount-1] = NULL;
	}
    }

    search_ctx->ffsc_level = level;

    /*
     * split into:
     *  -fix path
     *  -wildcard_stuff (might be NULL)
     */
    wc_part = vim_strchr(path, '*');
    if (wc_part != NULL)
    {
	int	llevel;
	int	len;
	char	*errpt;

	// save the fix part of the path
	search_ctx->ffsc_fix_path = vim_strnsave(path, wc_part - path);

	/*
	 * copy wc_path and add restricts to the '**' wildcard.
	 * The octet after a '**' is used as a (binary) counter.
	 * So '**3' is transposed to '**^C' ('^C' is ASCII value 3)
	 * or '**76' is transposed to '**N'( 'N' is ASCII value 76).
	 * If no restrict is given after '**' the default is used.
	 * Due to this technique the path looks awful if you print it as a
	 * string.
	 */
	len = 0;
	while (*wc_part != NUL)
	{
	    if (len + 5 >= MAXPATHL)
	    {
		emsg(_(e_path_too_long_for_completion));
		break;
	    }
	    if (STRNCMP(wc_part, "**", 2) == 0)
	    {
		ff_expand_buffer[len++] = *wc_part++;
		ff_expand_buffer[len++] = *wc_part++;

		llevel = strtol((char *)wc_part, &errpt, 10);
		if ((char_u *)errpt != wc_part && llevel > 0 && llevel < 255)
		    ff_expand_buffer[len++] = llevel;
		else if ((char_u *)errpt != wc_part && llevel == 0)
		    // restrict is 0 -> remove already added '**'
		    len -= 2;
		else
		    ff_expand_buffer[len++] = FF_MAX_STAR_STAR_EXPAND;
		wc_part = (char_u *)errpt;
		if (*wc_part != NUL && !vim_ispathsep(*wc_part))
		{
		    semsg(_(e_invalid_path_number_must_be_at_end_of_path_or_be_followed_by_str), PATHSEPSTR);
		    goto error_return;
		}
	    }
	    else
		ff_expand_buffer[len++] = *wc_part++;
	}
	ff_expand_buffer[len] = NUL;
	search_ctx->ffsc_wc_path = vim_strsave(ff_expand_buffer);

	if (search_ctx->ffsc_wc_path == NULL)
	    goto error_return;
    }
    else
	search_ctx->ffsc_fix_path = vim_strsave(path);

    if (search_ctx->ffsc_start_dir == NULL)
    {
	// store the fix part as startdir.
	// This is needed if the parameter path is fully qualified.
	search_ctx->ffsc_start_dir = vim_strsave(search_ctx->ffsc_fix_path);
	if (search_ctx->ffsc_start_dir == NULL)
	    goto error_return;
	search_ctx->ffsc_fix_path[0] = NUL;
    }

    // create an absolute path
    if (STRLEN(search_ctx->ffsc_start_dir)
			  + STRLEN(search_ctx->ffsc_fix_path) + 3 >= MAXPATHL)
    {
	emsg(_(e_path_too_long_for_completion));
	goto error_return;
    }
    STRCPY(ff_expand_buffer, search_ctx->ffsc_start_dir);
    add_pathsep(ff_expand_buffer);
    {
	int    eb_len = (int)STRLEN(ff_expand_buffer);
	char_u *buf = alloc(eb_len
				+ (int)STRLEN(search_ctx->ffsc_fix_path) + 1);

	STRCPY(buf, ff_expand_buffer);
	STRCPY(buf + eb_len, search_ctx->ffsc_fix_path);
	if (mch_isdir(buf))
	{
	    STRCAT(ff_expand_buffer, search_ctx->ffsc_fix_path);
	    add_pathsep(ff_expand_buffer);
	}
	else
	{
	    char_u *p =  gettail(search_ctx->ffsc_fix_path);
	    char_u *wc_path = NULL;
	    char_u *temp = NULL;
	    int    len = 0;

	    if (p > search_ctx->ffsc_fix_path)
	    {
		// do not add '..' to the path and start upwards searching
		len = (int)(p - search_ctx->ffsc_fix_path) - 1;
		if ((len >= 2
			&& STRNCMP(search_ctx->ffsc_fix_path, "..", 2) == 0)
			&& (len == 2
				   || search_ctx->ffsc_fix_path[2] == PATHSEP))
		{
		    vim_free(buf);
		    goto error_return;
		}
		STRNCAT(ff_expand_buffer, search_ctx->ffsc_fix_path, len);
		add_pathsep(ff_expand_buffer);
	    }
	    else
		len = (int)STRLEN(search_ctx->ffsc_fix_path);

	    if (search_ctx->ffsc_wc_path != NULL)
	    {
		wc_path = vim_strsave(search_ctx->ffsc_wc_path);
		temp = alloc(STRLEN(search_ctx->ffsc_wc_path)
				 + STRLEN(search_ctx->ffsc_fix_path + len)
				 + 1);
		if (temp == NULL || wc_path == NULL)
		{
		    vim_free(buf);
		    vim_free(temp);
		    vim_free(wc_path);
		    goto error_return;
		}

		STRCPY(temp, search_ctx->ffsc_fix_path + len);
		STRCAT(temp, search_ctx->ffsc_wc_path);
		vim_free(search_ctx->ffsc_wc_path);
		vim_free(wc_path);
		search_ctx->ffsc_wc_path = temp;
	    }
	}
	vim_free(buf);
    }

    sptr = ff_create_stack_element(ff_expand_buffer,
					   search_ctx->ffsc_wc_path, level, 0);

    if (sptr == NULL)
	goto error_return;

    ff_push(search_ctx, sptr);

    search_ctx->ffsc_file_to_search = vim_strsave(filename);
    if (search_ctx->ffsc_file_to_search == NULL)
	goto error_return;

    return search_ctx;

error_return:
    /*
     * We clear the search context now!
     * Even when the caller gave us a (perhaps valid) context we free it here,
     * as we might have already destroyed it.
     */
    vim_findfile_cleanup(search_ctx);
    return NULL;
}

/*
 * Get the stopdir string.  Check that ';' is not escaped.
 */
    char_u *
vim_findfile_stopdir(char_u *buf)
{
    char_u	*r_ptr = buf;

    while (*r_ptr != NUL && *r_ptr != ';')
    {
	if (r_ptr[0] == '\\' && r_ptr[1] == ';')
	{
	    // Overwrite the escape char,
	    // use STRLEN(r_ptr) to move the trailing '\0'.
	    STRMOVE(r_ptr, r_ptr + 1);
	    r_ptr++;
	}
	r_ptr++;
    }
    if (*r_ptr == ';')
    {
	*r_ptr = 0;
	r_ptr++;
    }
    else if (*r_ptr == NUL)
	r_ptr = NULL;
    return r_ptr;
}

/*
 * Clean up the given search context. Can handle a NULL pointer.
 */
    void
vim_findfile_cleanup(void *ctx)
{
    if (ctx == NULL)
	return;

    vim_findfile_free_visited(ctx);
    ff_clear(ctx);
    vim_free(ctx);
}

/*
 * Find a file in a search context.
 * The search context was created with vim_findfile_init() above.
 * Return a pointer to an allocated file name or NULL if nothing found.
 * To get all matching files call this function until you get NULL.
 *
 * If the passed search_context is NULL, NULL is returned.
 *
 * The search algorithm is depth first. To change this replace the
 * stack with a list (don't forget to leave partly searched directories on the
 * top of the list).
 */
    char_u *
vim_findfile(void *search_ctx_arg)
{
    char_u	*file_path;
    char_u	*rest_of_wildcards;
    char_u	*path_end = NULL;
    ff_stack_T	*stackp;
    int		len;
    int		i;
    char_u	*p;
    char_u	*suf;
    ff_search_ctx_T *search_ctx;

    if (search_ctx_arg == NULL)
	return NULL;

    search_ctx = (ff_search_ctx_T *)search_ctx_arg;

    /*
     * filepath is used as buffer for various actions and as the storage to
     * return a found filename.
     */
    if ((file_path = alloc(MAXPATHL)) == NULL)
	return NULL;

    // store the end of the start dir -- needed for upward search
    if (search_ctx->ffsc_start_dir != NULL)
	path_end = &search_ctx->ffsc_start_dir[
					  STRLEN(search_ctx->ffsc_start_dir)];

    // upward search loop
    for (;;)
    {
	// downward search loop
	for (;;)
	{
	    // check if user wants to stop the search
	    ui_breakcheck();
	    if (got_int)
		break;

	    // get directory to work on from stack
	    stackp = ff_pop(search_ctx);
	    if (stackp == NULL)
		break;

	    /*
	     * TODO: decide if we leave this test in
	     *
	     * GOOD: don't search a directory(-tree) twice.
	     * BAD:  - check linked list for every new directory entered.
	     *       - check for double files also done below
	     *
	     * Here we check if we already searched this directory.
	     * We already searched a directory if:
	     * 1) The directory is the same.
	     * 2) We would use the same wildcard string.
	     *
	     * Good if you have links on same directory via several ways
	     *  or you have selfreferences in directories (e.g. SuSE Linux 6.3:
	     *  /etc/rc.d/init.d is linked to /etc/rc.d -> endless loop)
	     *
	     * This check is only needed for directories we work on for the
	     * first time (hence stackp->ff_filearray == NULL)
	     */
	    if (stackp->ffs_filearray == NULL
		    && ff_check_visited(&search_ctx->ffsc_dir_visited_list
							  ->ffvl_visited_list,
			    stackp->ffs_fix_path, stackp->ffs_wc_path) == FAIL)
	    {
#ifdef FF_VERBOSE
		if (p_verbose >= 5)
		{
		    verbose_enter_scroll();
		    smsg("Already Searched: %s (%s)",
				   stackp->ffs_fix_path, stackp->ffs_wc_path);
		    // don't overwrite this either
		    msg_puts("\n");
		    verbose_leave_scroll();
		}
#endif
		ff_free_stack_element(stackp);
		continue;
	    }
#ifdef FF_VERBOSE
	    else if (p_verbose >= 5)
	    {
		verbose_enter_scroll();
		smsg("Searching: %s (%s)",
				   stackp->ffs_fix_path, stackp->ffs_wc_path);
		// don't overwrite this either
		msg_puts("\n");
		verbose_leave_scroll();
	    }
#endif

	    // check depth
	    if (stackp->ffs_level <= 0)
	    {
		ff_free_stack_element(stackp);
		continue;
	    }

	    file_path[0] = NUL;

	    /*
	     * If no filearray till now expand wildcards
	     * The function expand_wildcards() can handle an array of paths
	     * and all possible expands are returned in one array. We use this
	     * to handle the expansion of '**' into an empty string.
	     */
	    if (stackp->ffs_filearray == NULL)
	    {
		char_u *dirptrs[2];

		// we use filepath to build the path expand_wildcards() should
		// expand.
		dirptrs[0] = file_path;
		dirptrs[1] = NULL;

		// if we have a start dir copy it in
		if (!vim_isAbsName(stackp->ffs_fix_path)
						&& search_ctx->ffsc_start_dir)
		{
		    if (STRLEN(search_ctx->ffsc_start_dir) + 1 < MAXPATHL)
		    {
			STRCPY(file_path, search_ctx->ffsc_start_dir);
			add_pathsep(file_path);
		    }
		    else
		    {
			ff_free_stack_element(stackp);
			goto fail;
		    }
		}

		// append the fix part of the search path
		if (STRLEN(file_path) + STRLEN(stackp->ffs_fix_path) + 1
								    < MAXPATHL)
		{
		    STRCAT(file_path, stackp->ffs_fix_path);
		    add_pathsep(file_path);
		}
		else
		{
		    ff_free_stack_element(stackp);
		    goto fail;
		}

		rest_of_wildcards = stackp->ffs_wc_path;
		if (*rest_of_wildcards != NUL)
		{
		    len = (int)STRLEN(file_path);
		    if (STRNCMP(rest_of_wildcards, "**", 2) == 0)
		    {
			// pointer to the restrict byte
			// The restrict byte is not a character!
			p = rest_of_wildcards + 2;

			if (*p > 0)
			{
			    (*p)--;
			    if (len + 1 < MAXPATHL)
				file_path[len++] = '*';
			    else
			    {
				ff_free_stack_element(stackp);
				goto fail;
			    }
			}

			if (*p == 0)
			{
			    // remove '**<numb> from wildcards
			    STRMOVE(rest_of_wildcards, rest_of_wildcards + 3);
			}
			else
			    rest_of_wildcards += 3;

			if (stackp->ffs_star_star_empty == 0)
			{
			    // if not done before, expand '**' to empty
			    stackp->ffs_star_star_empty = 1;
			    dirptrs[1] = stackp->ffs_fix_path;
			}
		    }

		    /*
		     * Here we copy until the next path separator or the end of
		     * the path. If we stop at a path separator, there is
		     * still something else left. This is handled below by
		     * pushing every directory returned from expand_wildcards()
		     * on the stack again for further search.
		     */
		    while (*rest_of_wildcards
			    && !vim_ispathsep(*rest_of_wildcards))
			if (len + 1 < MAXPATHL)
			    file_path[len++] = *rest_of_wildcards++;
			else
			{
			    ff_free_stack_element(stackp);
			    goto fail;
			}

		    file_path[len] = NUL;
		    if (vim_ispathsep(*rest_of_wildcards))
			rest_of_wildcards++;
		}

		/*
		 * Expand wildcards like "*" and "$VAR".
		 * If the path is a URL don't try this.
		 */
		if (path_with_url(dirptrs[0]))
		{
		    stackp->ffs_filearray = ALLOC_ONE(char_u *);
		    if (stackp->ffs_filearray != NULL
			    && (stackp->ffs_filearray[0]
				= vim_strsave(dirptrs[0])) != NULL)
			stackp->ffs_filearray_size = 1;
		    else
			stackp->ffs_filearray_size = 0;
		}
		else
		    // Add EW_NOTWILD because the expanded path may contain
		    // wildcard characters that are to be taken literally.
		    // This is a bit of a hack.
		    expand_wildcards((dirptrs[1] == NULL) ? 1 : 2, dirptrs,
			    &stackp->ffs_filearray_size,
			    &stackp->ffs_filearray,
			    EW_DIR|EW_ADDSLASH|EW_SILENT|EW_NOTWILD);

		stackp->ffs_filearray_cur = 0;
		stackp->ffs_stage = 0;
	    }
	    else
		rest_of_wildcards = &stackp->ffs_wc_path[
						 STRLEN(stackp->ffs_wc_path)];

	    if (stackp->ffs_stage == 0)
	    {
		// this is the first time we work on this directory
		if (*rest_of_wildcards == NUL)
		{
		    /*
		     * We don't have further wildcards to expand, so we have to
		     * check for the final file now.
		     */
		    for (i = stackp->ffs_filearray_cur;
					  i < stackp->ffs_filearray_size; ++i)
		    {
			if (!path_with_url(stackp->ffs_filearray[i])
				      && !mch_isdir(stackp->ffs_filearray[i]))
			    continue;   // not a directory

			// prepare the filename to be checked for existence
			// below
			if (STRLEN(stackp->ffs_filearray[i]) + 1
				+ STRLEN(search_ctx->ffsc_file_to_search)
								    < MAXPATHL)
			{
			    STRCPY(file_path, stackp->ffs_filearray[i]);
			    add_pathsep(file_path);
			    STRCAT(file_path, search_ctx->ffsc_file_to_search);
			}
			else
			{
			    ff_free_stack_element(stackp);
			    goto fail;
			}

			/*
			 * Try without extra suffix and then with suffixes
			 * from 'suffixesadd'.
			 */
			len = (int)STRLEN(file_path);
			if (search_ctx->ffsc_tagfile)
			    suf = (char_u *)"";
			else
			    suf = curbuf->b_p_sua;
			for (;;)
			{
			    // if file exists and we didn't already find it
			    if ((path_with_url(file_path)
				  || (mch_getperm(file_path) >= 0
				      && (search_ctx->ffsc_find_what
							      == FINDFILE_BOTH
					  || ((search_ctx->ffsc_find_what
							      == FINDFILE_DIR)
						   == mch_isdir(file_path)))))
#ifndef FF_VERBOSE
				    && (ff_check_visited(
					    &search_ctx->ffsc_visited_list
							   ->ffvl_visited_list,
					    file_path, (char_u *)"") == OK)
#endif
			       )
			    {
#ifdef FF_VERBOSE
				if (ff_check_visited(
					    &search_ctx->ffsc_visited_list
							   ->ffvl_visited_list,
					      file_path, (char_u *)"") == FAIL)
				{
				    if (p_verbose >= 5)
				    {
					verbose_enter_scroll();
					smsg("Already: %s",
								   file_path);
					// don't overwrite this either
					msg_puts("\n");
					verbose_leave_scroll();
				    }
				    continue;
				}
#endif

				// push dir to examine rest of subdirs later
				stackp->ffs_filearray_cur = i + 1;
				ff_push(search_ctx, stackp);

				if (!path_with_url(file_path))
				    simplify_filename(file_path);
				if (mch_dirname(ff_expand_buffer, MAXPATHL)
									== OK)
				{
				    p = shorten_fname(file_path,
							    ff_expand_buffer);
				    if (p != NULL)
					STRMOVE(file_path, p);
				}
#ifdef FF_VERBOSE
				if (p_verbose >= 5)
				{
				    verbose_enter_scroll();
				    smsg("HIT: %s", file_path);
				    // don't overwrite this either
				    msg_puts("\n");
				    verbose_leave_scroll();
				}
#endif
				return file_path;
			    }

			    // Not found or found already, try next suffix.
			    if (*suf == NUL)
				break;
			    copy_option_part(&suf, file_path + len,
							 MAXPATHL - len, ",");
			}
		    }
		}
		else
		{
		    /*
		     * still wildcards left, push the directories for further
		     * search
		     */
		    for (i = stackp->ffs_filearray_cur;
					  i < stackp->ffs_filearray_size; ++i)
		    {
			if (!mch_isdir(stackp->ffs_filearray[i]))
			    continue;	// not a directory

			ff_push(search_ctx,
				ff_create_stack_element(
						     stackp->ffs_filearray[i],
						     rest_of_wildcards,
						     stackp->ffs_level - 1, 0));
		    }
		}
		stackp->ffs_filearray_cur = 0;
		stackp->ffs_stage = 1;
	    }

	    /*
	     * if wildcards contains '**' we have to descent till we reach the
	     * leaves of the directory tree.
	     */
	    if (STRNCMP(stackp->ffs_wc_path, "**", 2) == 0)
	    {
		for (i = stackp->ffs_filearray_cur;
					  i < stackp->ffs_filearray_size; ++i)
		{
		    if (fnamecmp(stackp->ffs_filearray[i],
						   stackp->ffs_fix_path) == 0)
			continue; // don't repush same directory
		    if (!mch_isdir(stackp->ffs_filearray[i]))
			continue;   // not a directory
		    ff_push(search_ctx,
			    ff_create_stack_element(stackp->ffs_filearray[i],
				stackp->ffs_wc_path, stackp->ffs_level - 1, 1));
		}
	    }

	    // we are done with the current directory
	    ff_free_stack_element(stackp);

	}

	// If we reached this, we didn't find anything downwards.
	// Let's check if we should do an upward search.
	if (search_ctx->ffsc_start_dir
		&& search_ctx->ffsc_stopdirs_v != NULL && !got_int)
	{
	    ff_stack_T  *sptr;

	    // is the last starting directory in the stop list?
	    if (ff_path_in_stoplist(search_ctx->ffsc_start_dir,
		       (int)(path_end - search_ctx->ffsc_start_dir),
		       search_ctx->ffsc_stopdirs_v) == TRUE)
		break;

	    // cut of last dir
	    while (path_end > search_ctx->ffsc_start_dir
						  && vim_ispathsep(*path_end))
		path_end--;
	    while (path_end > search_ctx->ffsc_start_dir
					      && !vim_ispathsep(path_end[-1]))
		path_end--;
	    *path_end = 0;
	    path_end--;

	    if (*search_ctx->ffsc_start_dir == 0)
		break;

	    if (STRLEN(search_ctx->ffsc_start_dir) + 1
		    + STRLEN(search_ctx->ffsc_fix_path) < MAXPATHL)
	    {
		STRCPY(file_path, search_ctx->ffsc_start_dir);
		add_pathsep(file_path);
		STRCAT(file_path, search_ctx->ffsc_fix_path);
	    }
	    else
		goto fail;

	    // create a new stack entry
	    sptr = ff_create_stack_element(file_path,
		    search_ctx->ffsc_wc_path, search_ctx->ffsc_level, 0);
	    if (sptr == NULL)
		break;
	    ff_push(search_ctx, sptr);
	}
	else
	    break;
    }

fail:
    vim_free(file_path);
    return NULL;
}

/*
 * Free the list of lists of visited files and directories
 * Can handle it if the passed search_context is NULL;
 */
    static void
vim_findfile_free_visited(void *search_ctx_arg)
{
    ff_search_ctx_T *search_ctx;

    if (search_ctx_arg == NULL)
	return;

    search_ctx = (ff_search_ctx_T *)search_ctx_arg;
    vim_findfile_free_visited_list(&search_ctx->ffsc_visited_lists_list);
    vim_findfile_free_visited_list(&search_ctx->ffsc_dir_visited_lists_list);
}

    static void
vim_findfile_free_visited_list(ff_visited_list_hdr_T **list_headp)
{
    ff_visited_list_hdr_T *vp;

    while (*list_headp != NULL)
    {
	vp = (*list_headp)->ffvl_next;
	ff_free_visited_list((*list_headp)->ffvl_visited_list);

	vim_free((*list_headp)->ffvl_filename);
	vim_free(*list_headp);
	*list_headp = vp;
    }
    *list_headp = NULL;
}

    static void
ff_free_visited_list(ff_visited_T *vl)
{
    ff_visited_T *vp;

    while (vl != NULL)
    {
	vp = vl->ffv_next;
	vim_free(vl->ffv_wc_path);
	vim_free(vl);
	vl = vp;
    }
    vl = NULL;
}

/*
 * Returns the already visited list for the given filename. If none is found it
 * allocates a new one.
 */
    static ff_visited_list_hdr_T*
ff_get_visited_list(
    char_u			*filename,
    ff_visited_list_hdr_T	**list_headp)
{
    ff_visited_list_hdr_T  *retptr = NULL;

    // check if a visited list for the given filename exists
    if (*list_headp != NULL)
    {
	retptr = *list_headp;
	while (retptr != NULL)
	{
	    if (fnamecmp(filename, retptr->ffvl_filename) == 0)
	    {
#ifdef FF_VERBOSE
		if (p_verbose >= 5)
		{
		    verbose_enter_scroll();
		    smsg("ff_get_visited_list: FOUND list for %s",
								    filename);
		    // don't overwrite this either
		    msg_puts("\n");
		    verbose_leave_scroll();
		}
#endif
		return retptr;
	    }
	    retptr = retptr->ffvl_next;
	}
    }

#ifdef FF_VERBOSE
    if (p_verbose >= 5)
    {
	verbose_enter_scroll();
	smsg("ff_get_visited_list: new list for %s", filename);
	// don't overwrite this either
	msg_puts("\n");
	verbose_leave_scroll();
    }
#endif

    /*
     * if we reach this we didn't find a list and we have to allocate new list
     */
    retptr = ALLOC_ONE(ff_visited_list_hdr_T);
    if (retptr == NULL)
	return NULL;

    retptr->ffvl_visited_list = NULL;
    retptr->ffvl_filename = vim_strsave(filename);
    if (retptr->ffvl_filename == NULL)
    {
	vim_free(retptr);
	return NULL;
    }
    retptr->ffvl_next = *list_headp;
    *list_headp = retptr;

    return retptr;
}

/*
 * check if two wildcard paths are equal. Returns TRUE or FALSE.
 * They are equal if:
 *  - both paths are NULL
 *  - they have the same length
 *  - char by char comparison is OK
 *  - the only differences are in the counters behind a '**', so
 *    '**\20' is equal to '**\24'
 */
    static int
ff_wc_equal(char_u *s1, char_u *s2)
{
    int		i, j;
    int		c1 = NUL;
    int		c2 = NUL;
    int		prev1 = NUL;
    int		prev2 = NUL;

    if (s1 == s2)
	return TRUE;

    if (s1 == NULL || s2 == NULL)
	return FALSE;

    for (i = 0, j = 0; s1[i] != NUL && s2[j] != NUL;)
    {
	c1 = PTR2CHAR(s1 + i);
	c2 = PTR2CHAR(s2 + j);

	if ((p_fic ? MB_TOLOWER(c1) != MB_TOLOWER(c2) : c1 != c2)
		&& (prev1 != '*' || prev2 != '*'))
	    return FALSE;
	prev2 = prev1;
	prev1 = c1;

	i += mb_ptr2len(s1 + i);
	j += mb_ptr2len(s2 + j);
    }
    return s1[i] == s2[j];
}

/*
 * maintains the list of already visited files and dirs
 * returns FAIL if the given file/dir is already in the list
 * returns OK if it is newly added
 *
 * TODO: What to do on memory allocation problems?
 *	 -> return TRUE - Better the file is found several times instead of
 *	    never.
 */
    static int
ff_check_visited(
    ff_visited_T	**visited_list,
    char_u		*fname,
    char_u		*wc_path)
{
    ff_visited_T	*vp;
#ifdef UNIX
    stat_T		st;
    int			url = FALSE;
#endif

    // For a URL we only compare the name, otherwise we compare the
    // device/inode (unix) or the full path name (not Unix).
    if (path_with_url(fname))
    {
	vim_strncpy(ff_expand_buffer, fname, MAXPATHL - 1);
#ifdef UNIX
	url = TRUE;
#endif
    }
    else
    {
	ff_expand_buffer[0] = NUL;
#ifdef UNIX
	if (mch_stat((char *)fname, &st) < 0)
#else
	if (vim_FullName(fname, ff_expand_buffer, MAXPATHL, TRUE) == FAIL)
#endif
	    return FAIL;
    }

    // check against list of already visited files
    for (vp = *visited_list; vp != NULL; vp = vp->ffv_next)
    {
	if (
#ifdef UNIX
		!url ? (vp->ffv_dev_valid && vp->ffv_dev == st.st_dev
						  && vp->ffv_ino == st.st_ino)
		     :
#endif
		fnamecmp(vp->ffv_fname, ff_expand_buffer) == 0
	   )
	{
	    // are the wildcard parts equal
	    if (ff_wc_equal(vp->ffv_wc_path, wc_path) == TRUE)
		// already visited
		return FAIL;
	}
    }

    /*
     * New file/dir.  Add it to the list of visited files/dirs.
     */
    vp = alloc(
	     offsetof(ff_visited_T, ffv_fname) + STRLEN(ff_expand_buffer) + 1);
    if (vp == NULL)
	return OK;

#ifdef UNIX
    if (!url)
    {
	vp->ffv_dev_valid = TRUE;
	vp->ffv_ino = st.st_ino;
	vp->ffv_dev = st.st_dev;
	vp->ffv_fname[0] = NUL;
    }
    else
    {
	vp->ffv_dev_valid = FALSE;
#endif
	STRCPY(vp->ffv_fname, ff_expand_buffer);
#ifdef UNIX
    }
#endif
    if (wc_path != NULL)
	vp->ffv_wc_path = vim_strsave(wc_path);
    else
	vp->ffv_wc_path = NULL;

    vp->ffv_next = *visited_list;
    *visited_list = vp;

    return OK;
}

/*
 * create stack element from given path pieces
 */
    static ff_stack_T *
ff_create_stack_element(
    char_u	*fix_part,
    char_u	*wc_part,
    int		level,
    int		star_star_empty)
{
    ff_stack_T	*new;

    new = ALLOC_ONE(ff_stack_T);
    if (new == NULL)
	return NULL;

    new->ffs_prev	   = NULL;
    new->ffs_filearray	   = NULL;
    new->ffs_filearray_size = 0;
    new->ffs_filearray_cur  = 0;
    new->ffs_stage	   = 0;
    new->ffs_level	   = level;
    new->ffs_star_star_empty = star_star_empty;

    // the following saves NULL pointer checks in vim_findfile
    if (fix_part == NULL)
	fix_part = (char_u *)"";
    new->ffs_fix_path = vim_strsave(fix_part);

    if (wc_part == NULL)
	wc_part  = (char_u *)"";
    new->ffs_wc_path = vim_strsave(wc_part);

    if (new->ffs_fix_path == NULL || new->ffs_wc_path == NULL)
    {
	ff_free_stack_element(new);
	new = NULL;
    }

    return new;
}

/*
 * Push a dir on the directory stack.
 */
    static void
ff_push(ff_search_ctx_T *search_ctx, ff_stack_T *stack_ptr)
{
    // check for NULL pointer, not to return an error to the user, but
    // to prevent a crash
    if (stack_ptr == NULL)
	return;

    stack_ptr->ffs_prev = search_ctx->ffsc_stack_ptr;
    search_ctx->ffsc_stack_ptr = stack_ptr;
}

/*
 * Pop a dir from the directory stack.
 * Returns NULL if stack is empty.
 */
    static ff_stack_T *
ff_pop(ff_search_ctx_T *search_ctx)
{
    ff_stack_T  *sptr;

    sptr = search_ctx->ffsc_stack_ptr;
    if (search_ctx->ffsc_stack_ptr != NULL)
	search_ctx->ffsc_stack_ptr = search_ctx->ffsc_stack_ptr->ffs_prev;

    return sptr;
}

/*
 * free the given stack element
 */
    static void
ff_free_stack_element(ff_stack_T *stack_ptr)
{
    // vim_free handles possible NULL pointers
    vim_free(stack_ptr->ffs_fix_path);
    vim_free(stack_ptr->ffs_wc_path);

    if (stack_ptr->ffs_filearray != NULL)
	FreeWild(stack_ptr->ffs_filearray_size, stack_ptr->ffs_filearray);

    vim_free(stack_ptr);
}

/*
 * Clear the search context, but NOT the visited list.
 */
    static void
ff_clear(ff_search_ctx_T *search_ctx)
{
    ff_stack_T   *sptr;

    // clear up stack
    while ((sptr = ff_pop(search_ctx)) != NULL)
	ff_free_stack_element(sptr);

    vim_free(search_ctx->ffsc_file_to_search);
    vim_free(search_ctx->ffsc_start_dir);
    vim_free(search_ctx->ffsc_fix_path);
    vim_free(search_ctx->ffsc_wc_path);

    if (search_ctx->ffsc_stopdirs_v != NULL)
    {
	int  i = 0;

	while (search_ctx->ffsc_stopdirs_v[i] != NULL)
	{
	    vim_free(search_ctx->ffsc_stopdirs_v[i]);
	    i++;
	}
	vim_free(search_ctx->ffsc_stopdirs_v);
    }
    search_ctx->ffsc_stopdirs_v = NULL;

    // reset everything
    search_ctx->ffsc_file_to_search = NULL;
    search_ctx->ffsc_start_dir = NULL;
    search_ctx->ffsc_fix_path = NULL;
    search_ctx->ffsc_wc_path = NULL;
    search_ctx->ffsc_level = 0;
}

/*
 * check if the given path is in the stopdirs
 * returns TRUE if yes else FALSE
 */
    static int
ff_path_in_stoplist(char_u *path, int path_len, char_u **stopdirs_v)
{
    int		i = 0;

    // eat up trailing path separators, except the first
    while (path_len > 1 && vim_ispathsep(path[path_len - 1]))
	path_len--;

    // if no path consider it as match
    if (path_len == 0)
	return TRUE;

    for (i = 0; stopdirs_v[i] != NULL; i++)
    {
	if ((int)STRLEN(stopdirs_v[i]) > path_len)
	{
	    // match for parent directory. So '/home' also matches
	    // '/home/rks'. Check for PATHSEP in stopdirs_v[i], else
	    // '/home/r' would also match '/home/rks'
	    if (fnamencmp(stopdirs_v[i], path, path_len) == 0
		    && vim_ispathsep(stopdirs_v[i][path_len]))
		return TRUE;
	}
	else
	{
	    if (fnamecmp(stopdirs_v[i], path) == 0)
		return TRUE;
	}
    }
    return FALSE;
}

/*
 * Find the file name "ptr[len]" in the path.  Also finds directory names.
 *
 * On the first call set the parameter 'first' to TRUE to initialize
 * the search.  For repeating calls to FALSE.
 *
 * Repeating calls will return other files called 'ptr[len]' from the path.
 *
 * Only on the first call 'ptr' and 'len' are used.  For repeating calls they
 * don't need valid values.
 *
 * If nothing found on the first call the option FNAME_MESS will issue the
 * message:
 *	    'Can't find file "<file>" in path'
 * On repeating calls:
 *	    'No more file "<file>" found in path'
 *
 * options:
 * FNAME_MESS	    give error message when not found
 *
 * Uses NameBuff[]!
 *
 * Returns an allocated string for the file name.  NULL for error.
 *
 */
    char_u *
find_file_in_path(
    char_u	*ptr,		// file name
    int		len,		// length of file name
    int		options,
    int		first,		// use count'th matching file name
    char_u	*rel_fname,	// file name searching relative to
    char_u	**file_to_find,	// in/out: modified copy of file name
    char	**search_ctx)	// in/out: state of the search
{
    return find_file_in_path_option(ptr, len, options, first,
	    *curbuf->b_p_path == NUL ? p_path : curbuf->b_p_path,
	    FINDFILE_BOTH, rel_fname, curbuf->b_p_sua,
	    file_to_find, search_ctx);
}

# if defined(EXITFREE) || defined(PROTO)
    void
free_findfile(void)
{
    VIM_CLEAR(ff_expand_buffer);
}
# endif

/*
 * Find the directory name "ptr[len]" in the path.
 *
 * options:
 * FNAME_MESS	    give error message when not found
 * FNAME_UNESC	    unescape backslashes.
 *
 * Uses NameBuff[]!
 *
 * Returns an allocated string for the file name.  NULL for error.
 */
    char_u *
find_directory_in_path(
    char_u	*ptr,		// file name
    int		len,		// length of file name
    int		options,
    char_u	*rel_fname,	// file name searching relative to
    char_u	**file_to_find,	// in/out: modified copy of file name
    char	**search_ctx)	// in/out: state of the search
{
    return find_file_in_path_option(ptr, len, options, TRUE, p_cdpath,
				       FINDFILE_DIR, rel_fname, (char_u *)"",
				       file_to_find, search_ctx);
}

    char_u *
find_file_in_path_option(
    char_u	*ptr,		// file name
    int		len,		// length of file name
    int		options,
    int		first,		// use count'th matching file name
    char_u	*path_option,	// p_path or p_cdpath
    int		find_what,	// FINDFILE_FILE, _DIR or _BOTH
    char_u	*rel_fname,	// file name we are looking relative to.
    char_u	*suffixes,	// list of suffixes, 'suffixesadd' option
    char_u	**file_to_find,	// in/out: modified copy of file name
    char	**search_ctx_arg) // in/out: state of the search
{
    ff_search_ctx_T	**search_ctx = (ff_search_ctx_T **)search_ctx_arg;
    static char_u	*dir;
    static int		did_findfile_init = FALSE;
    char_u		save_char;
    char_u		*file_name = NULL;
    char_u		*buf = NULL;
    int			rel_to_curdir;
# ifdef AMIGA
    struct Process	*proc = (struct Process *)FindTask(0L);
    APTR		save_winptr = proc->pr_WindowPtr;

    // Avoid a requester here for a volume that doesn't exist.
    proc->pr_WindowPtr = (APTR)-1L;
# endif

    if (first == TRUE)
    {
	if (len == 0)
	    return NULL;

	// copy file name into NameBuff, expanding environment variables
	save_char = ptr[len];
	ptr[len] = NUL;
	expand_env_esc(ptr, NameBuff, MAXPATHL, FALSE, TRUE, NULL);
	ptr[len] = save_char;

	vim_free(*file_to_find);
	*file_to_find = vim_strsave(NameBuff);
	if (*file_to_find == NULL)	// out of memory
	{
	    file_name = NULL;
	    goto theend;
	}
	if (options & FNAME_UNESC)
	{
	    // Change all "\ " to " ".
	    for (ptr = *file_to_find; *ptr != NUL; ++ptr)
		if (ptr[0] == '\\' && ptr[1] == ' ')
		    mch_memmove(ptr, ptr + 1, STRLEN(ptr));
	}
    }

    rel_to_curdir = ((*file_to_find)[0] == '.'
		    && ((*file_to_find)[1] == NUL
			|| vim_ispathsep((*file_to_find)[1])
			|| ((*file_to_find)[1] == '.'
			    && ((*file_to_find)[2] == NUL
				|| vim_ispathsep((*file_to_find)[2])))));
    if (vim_isAbsName(*file_to_find)
	    // "..", "../path", "." and "./path": don't use the path_option
	    || rel_to_curdir
# if defined(MSWIN)
	    // handle "\tmp" as absolute path
	    || vim_ispathsep((*file_to_find)[0])
	    // handle "c:name" as absolute path
	    || ((*file_to_find)[0] != NUL && (*file_to_find)[1] == ':')
# endif
# ifdef AMIGA
	    // handle ":tmp" as absolute path
	    || (*file_to_find)[0] == ':'
# endif
       )
    {
	/*
	 * Absolute path, no need to use "path_option".
	 * If this is not a first call, return NULL.  We already returned a
	 * filename on the first call.
	 */
	if (first == TRUE)
	{
	    int		l;
	    int		run;

	    if (path_with_url(*file_to_find))
	    {
		file_name = vim_strsave(*file_to_find);
		goto theend;
	    }

	    // When FNAME_REL flag given first use the directory of the file.
	    // Otherwise or when this fails use the current directory.
	    for (run = 1; run <= 2; ++run)
	    {
		l = (int)STRLEN(*file_to_find);
		if (run == 1
			&& rel_to_curdir
			&& (options & FNAME_REL)
			&& rel_fname != NULL
			&& STRLEN(rel_fname) + l < MAXPATHL)
		{
		    STRCPY(NameBuff, rel_fname);
		    STRCPY(gettail(NameBuff), *file_to_find);
		    l = (int)STRLEN(NameBuff);
		}
		else
		{
		    STRCPY(NameBuff, *file_to_find);
		    run = 2;
		}

		// When the file doesn't exist, try adding parts of
		// 'suffixesadd'.
		buf = suffixes;
		for (;;)
		{
		    if (mch_getperm(NameBuff) >= 0
			     && (find_what == FINDFILE_BOTH
				 || ((find_what == FINDFILE_DIR)
						    == mch_isdir(NameBuff))))
		    {
			file_name = vim_strsave(NameBuff);
			goto theend;
		    }
		    if (*buf == NUL)
			break;
		    copy_option_part(&buf, NameBuff + l, MAXPATHL - l, ",");
		}
	    }
	}
    }
    else
    {
	/*
	 * Loop over all paths in the 'path' or 'cdpath' option.
	 * When "first" is set, first setup to the start of the option.
	 * Otherwise continue to find the next match.
	 */
	if (first == TRUE)
	{
	    // vim_findfile_free_visited can handle a possible NULL pointer
	    vim_findfile_free_visited(*search_ctx);
	    dir = path_option;
	    did_findfile_init = FALSE;
	}

	for (;;)
	{
	    if (did_findfile_init)
	    {
		file_name = vim_findfile(*search_ctx);
		if (file_name != NULL)
		    break;

		did_findfile_init = FALSE;
	    }
	    else
	    {
		char_u  *r_ptr;

		if (dir == NULL || *dir == NUL)
		{
		    // We searched all paths of the option, now we can
		    // free the search context.
		    vim_findfile_cleanup(*search_ctx);
		    *search_ctx = NULL;
		    break;
		}

		if ((buf = alloc(MAXPATHL)) == NULL)
		    break;

		// copy next path
		buf[0] = 0;
		copy_option_part(&dir, buf, MAXPATHL, " ,");

		// get the stopdir string
		r_ptr = vim_findfile_stopdir(buf);
		*search_ctx = vim_findfile_init(buf, *file_to_find,
					    r_ptr, 100, FALSE, find_what,
					   *search_ctx, FALSE, rel_fname);
		if (*search_ctx != NULL)
		    did_findfile_init = TRUE;
		vim_free(buf);
	    }
	}
    }
    if (file_name == NULL && (options & FNAME_MESS))
    {
	if (first == TRUE)
	{
	    if (find_what == FINDFILE_DIR)
		semsg(_(e_cant_find_directory_str_in_cdpath), *file_to_find);
	    else
		semsg(_(e_cant_find_file_str_in_path), *file_to_find);
	}
	else
	{
	    if (find_what == FINDFILE_DIR)
		semsg(_(e_no_more_directory_str_found_in_cdpath),
								*file_to_find);
	    else
		semsg(_(e_no_more_file_str_found_in_path), *file_to_find);
	}
    }

theend:
# ifdef AMIGA
    proc->pr_WindowPtr = save_winptr;
# endif
    return file_name;
}

/*
 * Get the file name at the cursor.
 * If Visual mode is active, use the selected text if it's in one line.
 * Returns the name in allocated memory, NULL for failure.
 */
    char_u *
grab_file_name(long count, linenr_T *file_lnum)
{
    int options = FNAME_MESS|FNAME_EXP|FNAME_REL|FNAME_UNESC;

    if (VIsual_active)
    {
	int	len;
	char_u	*ptr;

	if (get_visual_text(NULL, &ptr, &len) == FAIL)
	    return NULL;
	// Only recognize ":123" here
	if (file_lnum != NULL && ptr[len] == ':' && SAFE_isdigit(ptr[len + 1]))
	{
	    char_u *p = ptr + len + 1;

	    *file_lnum = getdigits(&p);
	}
	return find_file_name_in_path(ptr, len, options,
						     count, curbuf->b_ffname);
    }
    return file_name_at_cursor(options | FNAME_HYP, count, file_lnum);
}

/*
 * Return the file name under or after the cursor.
 *
 * The 'path' option is searched if the file name is not absolute.
 * The string returned has been alloc'ed and should be freed by the caller.
 * NULL is returned if the file name or file is not found.
 *
 * options:
 * FNAME_MESS	    give error messages
 * FNAME_EXP	    expand to path
 * FNAME_HYP	    check for hypertext link
 * FNAME_INCL	    apply "includeexpr"
 */
    char_u *
file_name_at_cursor(int options, long count, linenr_T *file_lnum)
{
    return file_name_in_line(ml_get_curline(),
		      curwin->w_cursor.col, options, count, curbuf->b_ffname,
		      file_lnum);
}

/*
 * Return the name of the file under or after ptr[col].
 * Otherwise like file_name_at_cursor().
 */
    char_u *
file_name_in_line(
    char_u	*line,
    int		col,
    int		options,
    long	count,
    char_u	*rel_fname,	// file we are searching relative to
    linenr_T	*file_lnum)	// line number after the file name
{
    char_u	*ptr;
    int		len;
    int		in_type = TRUE;
    int		is_url = FALSE;

    /*
     * search forward for what could be the start of a file name
     */
    ptr = line + col;
    while (*ptr != NUL && !vim_isfilec(*ptr))
	MB_PTR_ADV(ptr);
    if (*ptr == NUL)		// nothing found
    {
	if (options & FNAME_MESS)
	    emsg(_(e_no_file_name_under_cursor));
	return NULL;
    }

    /*
     * Search backward for first char of the file name.
     * Go one char back to ":" before "//" even when ':' is not in 'isfname'.
     */
    while (ptr > line)
    {
	if (has_mbyte && (len = (*mb_head_off)(line, ptr - 1)) > 0)
	    ptr -= len + 1;
	else if (vim_isfilec(ptr[-1])
		|| ((options & FNAME_HYP) && path_is_url(ptr - 1)))
	    --ptr;
	else
	    break;
    }

    /*
     * Search forward for the last char of the file name.
     * Also allow "://" when ':' is not in 'isfname'.
     */
    len = 0;
    while (vim_isfilec(ptr[len]) || (ptr[len] == '\\' && ptr[len + 1] == ' ')
		 || ((options & FNAME_HYP) && path_is_url(ptr + len))
		 || (is_url && vim_strchr((char_u *)":?&=", ptr[len]) != NULL))
    {
	// After type:// we also include :, ?, & and = as valid characters, so
	// that http://google.com:8080?q=this&that=ok works.
	if ((ptr[len] >= 'A' && ptr[len] <= 'Z')
				       || (ptr[len] >= 'a' && ptr[len] <= 'z'))
	{
	    if (in_type && path_is_url(ptr + len + 1))
		is_url = TRUE;
	}
	else
	    in_type = FALSE;

	if (ptr[len] == '\\')
	    // Skip over the "\" in "\ ".
	    ++len;
	if (has_mbyte)
	    len += (*mb_ptr2len)(ptr + len);
	else
	    ++len;
    }

    /*
     * If there is trailing punctuation, remove it.
     * But don't remove "..", could be a directory name.
     */
    if (len > 2 && vim_strchr((char_u *)".,:;!", ptr[len - 1]) != NULL
						       && ptr[len - 2] != '.')
	--len;

    if (file_lnum != NULL)
    {
	char_u *p;
	char	*line_english = " line ";
	char	*line_transl = _(line_msg);

	// Get the number after the file name and a separator character.
	// Also accept " line 999" with and without the same translation as
	// used in last_set_msg().
	p = ptr + len;
	if (STRNCMP(p, line_english, STRLEN(line_english)) == 0)
	    p += STRLEN(line_english);
	else if (STRNCMP(p, line_transl, STRLEN(line_transl)) == 0)
	    p += STRLEN(line_transl);
	else
	    p = skipwhite(p);
	if (*p != NUL)
	{
	    if (!SAFE_isdigit(*p))
		++p;		    // skip the separator
	    p = skipwhite(p);
	    if (SAFE_isdigit(*p))
		*file_lnum = (int)getdigits(&p);
	}
    }

    return find_file_name_in_path(ptr, len, options, count, rel_fname);
}

# if defined(FEAT_FIND_ID) && defined(FEAT_EVAL)
    static char_u *
eval_includeexpr(char_u *ptr, int len)
{
    char_u	*res;
    sctx_T	save_sctx = current_sctx;

    set_vim_var_string(VV_FNAME, ptr, len);
    current_sctx = curbuf->b_p_script_ctx[BV_INEX];

    res = eval_to_string_safe(curbuf->b_p_inex,
	    was_set_insecurely((char_u *)"includeexpr", OPT_LOCAL),
								   TRUE, TRUE);

    set_vim_var_string(VV_FNAME, NULL, 0);
    current_sctx = save_sctx;
    return res;
}
# endif

/*
 * Return the name of the file ptr[len] in 'path'.
 * Otherwise like file_name_at_cursor().
 */
    char_u *
find_file_name_in_path(
    char_u	*ptr,
    int		len,
    int		options,
    long	count,
    char_u	*rel_fname)	// file we are searching relative to
{
    char_u	*file_name;
    int		c;
# if defined(FEAT_FIND_ID) && defined(FEAT_EVAL)
    char_u	*tofree = NULL;
# endif

    if (len == 0)
	return NULL;

# if defined(FEAT_FIND_ID) && defined(FEAT_EVAL)
    if ((options & FNAME_INCL) && *curbuf->b_p_inex != NUL)
    {
	tofree = eval_includeexpr(ptr, len);
	if (tofree != NULL)
	{
	    ptr = tofree;
	    len = (int)STRLEN(ptr);
	}
    }
# endif

    if (options & FNAME_EXP)
    {
	char_u	*file_to_find = NULL;
	char	*search_ctx = NULL;

	file_name = find_file_in_path(ptr, len, options & ~FNAME_MESS,
				  TRUE, rel_fname, &file_to_find, &search_ctx);

# if defined(FEAT_FIND_ID) && defined(FEAT_EVAL)
	/*
	 * If the file could not be found in a normal way, try applying
	 * 'includeexpr' (unless done already).
	 */
	if (file_name == NULL
		&& !(options & FNAME_INCL) && *curbuf->b_p_inex != NUL)
	{
	    tofree = eval_includeexpr(ptr, len);
	    if (tofree != NULL)
	    {
		ptr = tofree;
		len = (int)STRLEN(ptr);
		file_name = find_file_in_path(ptr, len, options & ~FNAME_MESS,
				  TRUE, rel_fname, &file_to_find, &search_ctx);
	    }
	}
# endif
	if (file_name == NULL && (options & FNAME_MESS))
	{
	    c = ptr[len];
	    ptr[len] = NUL;
	    semsg(_(e_cant_find_file_str_in_path_2), ptr);
	    ptr[len] = c;
	}

	// Repeat finding the file "count" times.  This matters when it
	// appears several times in the path.
	while (file_name != NULL && --count > 0)
	{
	    vim_free(file_name);
	    file_name = find_file_in_path(ptr, len, options, FALSE, rel_fname,
						   &file_to_find, &search_ctx);
	}

	vim_free(file_to_find);
	vim_findfile_cleanup(search_ctx);
    }
    else
	file_name = vim_strnsave(ptr, len);

# if defined(FEAT_FIND_ID) && defined(FEAT_EVAL)
    vim_free(tofree);
# endif

    return file_name;
}

/*
 * Return the end of the directory name, on the first path
 * separator:
 * "/path/file", "/path/dir/", "/path//dir", "/file"
 *	 ^	       ^	     ^	      ^
 */
    static char_u *
gettail_dir(char_u *fname)
{
    char_u	*dir_end = fname;
    char_u	*next_dir_end = fname;
    int		look_for_sep = TRUE;
    char_u	*p;

    for (p = fname; *p != NUL; )
    {
	if (vim_ispathsep(*p))
	{
	    if (look_for_sep)
	    {
		next_dir_end = p;
		look_for_sep = FALSE;
	    }
	}
	else
	{
	    if (!look_for_sep)
		dir_end = next_dir_end;
	    look_for_sep = TRUE;
	}
	MB_PTR_ADV(p);
    }
    return dir_end;
}

/*
 * return TRUE if 'c' is a path list separator.
 */
    int
vim_ispathlistsep(int c)
{
# ifdef UNIX
    return (c == ':');
# else
    return (c == ';');	// might not be right for every system...
# endif
}

/*
 * Moves "*psep" back to the previous path separator in "path".
 * Returns FAIL is "*psep" ends up at the beginning of "path".
 */
    static int
find_previous_pathsep(char_u *path, char_u **psep)
{
    // skip the current separator
    if (*psep > path && vim_ispathsep(**psep))
	--*psep;

    // find the previous separator
    while (*psep > path)
    {
	if (vim_ispathsep(**psep))
	    return OK;
	MB_PTR_BACK(path, *psep);
    }

    return FAIL;
}

/*
 * Returns TRUE if "maybe_unique" is unique wrt other_paths in "gap".
 * "maybe_unique" is the end portion of "((char_u **)gap->ga_data)[i]".
 */
    static int
is_unique(char_u *maybe_unique, garray_T *gap, int i)
{
    int	    j;
    int	    candidate_len;
    int	    other_path_len;
    char_u  **other_paths = (char_u **)gap->ga_data;
    char_u  *rival;

    for (j = 0; j < gap->ga_len; j++)
    {
	if (j == i)
	    continue;  // don't compare it with itself

	candidate_len = (int)STRLEN(maybe_unique);
	other_path_len = (int)STRLEN(other_paths[j]);
	if (other_path_len < candidate_len)
	    continue;  // it's different when it's shorter

	rival = other_paths[j] + other_path_len - candidate_len;
	if (fnamecmp(maybe_unique, rival) == 0
		&& (rival == other_paths[j] || vim_ispathsep(*(rival - 1))))
	    return FALSE;  // match
    }

    return TRUE;  // no match found
}

/*
 * Split the 'path' option into an array of strings in garray_T.  Relative
 * paths are expanded to their equivalent fullpath.  This includes the "."
 * (relative to current buffer directory) and empty path (relative to current
 * directory) notations.
 *
 * TODO: handle upward search (;) and path limiter (**N) notations by
 * expanding each into their equivalent path(s).
 */
    static void
expand_path_option(char_u *curdir, garray_T *gap)
{
    char_u	*path_option = *curbuf->b_p_path == NUL
						  ? p_path : curbuf->b_p_path;
    char_u	*buf;
    char_u	*p;
    int		len;

    if ((buf = alloc(MAXPATHL)) == NULL)
	return;

    while (*path_option != NUL)
    {
	copy_option_part(&path_option, buf, MAXPATHL, " ,");

	if (buf[0] == '.' && (buf[1] == NUL || vim_ispathsep(buf[1])))
	{
	    // Relative to current buffer:
	    // "/path/file" + "." -> "/path/"
	    // "/path/file"  + "./subdir" -> "/path/subdir"
	    if (curbuf->b_ffname == NULL)
		continue;
	    p = gettail(curbuf->b_ffname);
	    len = (int)(p - curbuf->b_ffname);
	    if (len + (int)STRLEN(buf) >= MAXPATHL)
		continue;
	    if (buf[1] == NUL)
		buf[len] = NUL;
	    else
		STRMOVE(buf + len, buf + 2);
	    mch_memmove(buf, curbuf->b_ffname, len);
	    simplify_filename(buf);
	}
	else if (buf[0] == NUL)
	    // relative to current directory
	    STRCPY(buf, curdir);
	else if (path_with_url(buf))
	    // URL can't be used here
	    continue;
	else if (!mch_isFullName(buf))
	{
	    // Expand relative path to their full path equivalent
	    len = (int)STRLEN(curdir);
	    if (len + (int)STRLEN(buf) + 3 > MAXPATHL)
		continue;
	    STRMOVE(buf + len + 1, buf);
	    STRCPY(buf, curdir);
	    buf[len] = PATHSEP;
	    simplify_filename(buf);
	}

	if (ga_grow(gap, 1) == FAIL)
	    break;

# if defined(MSWIN)
	// Avoid the path ending in a backslash, it fails when a comma is
	// appended.
	len = (int)STRLEN(buf);
	if (buf[len - 1] == '\\')
	    buf[len - 1] = '/';
# endif

	p = vim_strsave(buf);
	if (p == NULL)
	    break;
	((char_u **)gap->ga_data)[gap->ga_len++] = p;
    }

    vim_free(buf);
}

/*
 * Returns a pointer to the file or directory name in "fname" that matches the
 * longest path in "ga"p, or NULL if there is no match. For example:
 *
 *    path: /foo/bar/baz
 *   fname: /foo/bar/baz/quux.txt
 * returns:		 ^this
 */
    static char_u *
get_path_cutoff(char_u *fname, garray_T *gap)
{
    int	    i;
    int	    maxlen = 0;
    char_u  **path_part = (char_u **)gap->ga_data;
    char_u  *cutoff = NULL;

    for (i = 0; i < gap->ga_len; i++)
    {
	int j = 0;

	while ((fname[j] == path_part[i][j]
# if defined(MSWIN)
		|| (vim_ispathsep(fname[j]) && vim_ispathsep(path_part[i][j]))
# endif
			     ) && fname[j] != NUL && path_part[i][j] != NUL)
	    j++;
	if (j > maxlen)
	{
	    maxlen = j;
	    cutoff = &fname[j];
	}
    }

    // skip to the file or directory name
    if (cutoff != NULL)
	while (vim_ispathsep(*cutoff))
	    MB_PTR_ADV(cutoff);

    return cutoff;
}

/*
 * Sorts, removes duplicates and modifies all the fullpath names in "gap" so
 * that they are unique with respect to each other while conserving the part
 * that matches the pattern. Beware, this is at least O(n^2) wrt "gap->ga_len".
 */
    void
uniquefy_paths(garray_T *gap, char_u *pattern)
{
    int		i;
    int		len;
    char_u	**fnames = (char_u **)gap->ga_data;
    int		sort_again = FALSE;
    char_u	*pat;
    char_u      *file_pattern;
    char_u	*curdir;
    regmatch_T	regmatch;
    garray_T	path_ga;
    char_u	**in_curdir = NULL;
    char_u	*short_name;

    remove_duplicates(gap);
    ga_init2(&path_ga, sizeof(char_u *), 1);

    /*
     * We need to prepend a '*' at the beginning of file_pattern so that the
     * regex matches anywhere in the path. FIXME: is this valid for all
     * possible patterns?
     */
    len = (int)STRLEN(pattern);
    file_pattern = alloc(len + 2);
    if (file_pattern == NULL)
	return;
    file_pattern[0] = '*';
    file_pattern[1] = NUL;
    STRCAT(file_pattern, pattern);
    pat = file_pat_to_reg_pat(file_pattern, NULL, NULL, TRUE);
    vim_free(file_pattern);
    if (pat == NULL)
	return;

    regmatch.rm_ic = TRUE;		// always ignore case
    regmatch.regprog = vim_regcomp(pat, RE_MAGIC + RE_STRING);
    vim_free(pat);
    if (regmatch.regprog == NULL)
	return;

    if ((curdir = alloc(MAXPATHL)) == NULL)
	goto theend;
    mch_dirname(curdir, MAXPATHL);
    expand_path_option(curdir, &path_ga);

    in_curdir = ALLOC_CLEAR_MULT(char_u *, gap->ga_len);
    if (in_curdir == NULL)
	goto theend;

    for (i = 0; i < gap->ga_len && !got_int; i++)
    {
	char_u	    *path = fnames[i];
	int	    is_in_curdir;
	char_u	    *dir_end = gettail_dir(path);
	char_u	    *pathsep_p;
	char_u	    *path_cutoff;

	len = (int)STRLEN(path);
	is_in_curdir = fnamencmp(curdir, path, dir_end - path) == 0
					     && curdir[dir_end - path] == NUL;
	if (is_in_curdir)
	    in_curdir[i] = vim_strsave(path);

	// Shorten the filename while maintaining its uniqueness
	path_cutoff = get_path_cutoff(path, &path_ga);

	// Don't assume all files can be reached without path when search
	// pattern starts with star star slash, so only remove path_cutoff
	// when possible.
	if (pattern[0] == '*' && pattern[1] == '*'
		&& vim_ispathsep_nocolon(pattern[2])
		&& path_cutoff != NULL
		&& vim_regexec(&regmatch, path_cutoff, (colnr_T)0)
		&& is_unique(path_cutoff, gap, i))
	{
	    sort_again = TRUE;
	    mch_memmove(path, path_cutoff, STRLEN(path_cutoff) + 1);
	}
	else
	{
	    // Here all files can be reached without path, so get shortest
	    // unique path.  We start at the end of the path.
	    pathsep_p = path + len - 1;

	    while (find_previous_pathsep(path, &pathsep_p))
		if (vim_regexec(&regmatch, pathsep_p + 1, (colnr_T)0)
			&& is_unique(pathsep_p + 1, gap, i)
			&& path_cutoff != NULL && pathsep_p + 1 >= path_cutoff)
		{
		    sort_again = TRUE;
		    mch_memmove(path, pathsep_p + 1, STRLEN(pathsep_p));
		    break;
		}
	}

	if (mch_isFullName(path))
	{
	    /*
	     * Last resort: shorten relative to curdir if possible.
	     * 'possible' means:
	     * 1. It is under the current directory.
	     * 2. The result is actually shorter than the original.
	     *
	     *	    Before		  curdir	After
	     *	    /foo/bar/file.txt	  /foo/bar	./file.txt
	     *	    c:\foo\bar\file.txt   c:\foo\bar	.\file.txt
	     *	    /file.txt		  /		/file.txt
	     *	    c:\file.txt		  c:\		.\file.txt
	     */
	    short_name = shorten_fname(path, curdir);
	    if (short_name != NULL && short_name > path + 1
# if defined(MSWIN)
		    // On windows,
		    //	    shorten_fname("c:\a\a.txt", "c:\a\b")
		    // returns "\a\a.txt", which is not really the short
		    // name, hence:
		    && !vim_ispathsep(*short_name)
# endif
		)
	    {
		STRCPY(path, ".");
		add_pathsep(path);
		STRMOVE(path + STRLEN(path), short_name);
	    }
	}
	ui_breakcheck();
    }

    // Shorten filenames in /in/current/directory/{filename}
    for (i = 0; i < gap->ga_len && !got_int; i++)
    {
	char_u *rel_path;
	char_u *path = in_curdir[i];

	if (path == NULL)
	    continue;

	// If the {filename} is not unique, change it to ./{filename}.
	// Else reduce it to {filename}
	short_name = shorten_fname(path, curdir);
	if (short_name == NULL)
	    short_name = path;
	if (is_unique(short_name, gap, i))
	{
	    STRCPY(fnames[i], short_name);
	    continue;
	}

	rel_path = alloc(STRLEN(short_name) + STRLEN(PATHSEPSTR) + 2);
	if (rel_path == NULL)
	    goto theend;
	STRCPY(rel_path, ".");
	add_pathsep(rel_path);
	STRCAT(rel_path, short_name);

	vim_free(fnames[i]);
	fnames[i] = rel_path;
	sort_again = TRUE;
	ui_breakcheck();
    }

theend:
    vim_free(curdir);
    if (in_curdir != NULL)
    {
	for (i = 0; i < gap->ga_len; i++)
	    vim_free(in_curdir[i]);
	vim_free(in_curdir);
    }
    ga_clear_strings(&path_ga);
    vim_regfree(regmatch.regprog);

    if (sort_again)
	remove_duplicates(gap);
}

/*
 * Calls globpath() with 'path' values for the given pattern and stores the
 * result in "gap".
 * Returns the total number of matches.
 */
    int
expand_in_path(
    garray_T	*gap,
    char_u	*pattern,
    int		flags)		// EW_* flags
{
    char_u	*curdir;
    garray_T	path_ga;
    char_u	*paths = NULL;
    int		glob_flags = 0;

    if ((curdir = alloc(MAXPATHL)) == NULL)
	return 0;
    mch_dirname(curdir, MAXPATHL);

    ga_init2(&path_ga, sizeof(char_u *), 1);
    expand_path_option(curdir, &path_ga);
    vim_free(curdir);
    if (path_ga.ga_len == 0)
	return 0;

    paths = ga_concat_strings(&path_ga, ",");
    ga_clear_strings(&path_ga);
    if (paths == NULL)
	return 0;

    if (flags & EW_ICASE)
	glob_flags |= WILD_ICASE;
    if (flags & EW_ADDSLASH)
	glob_flags |= WILD_ADD_SLASH;
    globpath(paths, pattern, gap, glob_flags, FALSE);
    vim_free(paths);

    return gap->ga_len;
}


/*
 * Converts a file name into a canonical form. It simplifies a file name into
 * its simplest form by stripping out unneeded components, if any.  The
 * resulting file name is simplified in place and will either be the same
 * length as that supplied, or shorter.
 */
    void
simplify_filename(char_u *filename)
{
#ifndef AMIGA	    // Amiga doesn't have "..", it uses "/"
    int		components = 0;
    char_u	*p, *tail, *start;
    int		stripping_disabled = FALSE;
    int		relative = TRUE;

    p = filename;
# ifdef BACKSLASH_IN_FILENAME
    if (p[0] != NUL && p[1] == ':')	    // skip "x:"
	p += 2;
# endif

    if (vim_ispathsep(*p))
    {
	relative = FALSE;
	do
	    ++p;
	while (vim_ispathsep(*p));
    }
    start = p;	    // remember start after "c:/" or "/" or "///"
#ifdef UNIX
    // Posix says that "//path" is unchanged but "///path" is "/path".
    if (start > filename + 2)
    {
	STRMOVE(filename + 1, p);
	start = p = filename + 1;
    }
#endif

    do
    {
	// At this point "p" is pointing to the char following a single "/"
	// or "p" is at the "start" of the (absolute or relative) path name.
# ifdef VMS
	// VMS allows device:[path] - don't strip the [ in directory
	if ((*p == '[' || *p == '<') && p > filename && p[-1] == ':')
	{
	    // :[ or :< composition: vms directory component
	    ++components;
	    p = getnextcomp(p + 1);
	}
	// allow remote calls as host"user passwd"::device:[path]
	else if (p[0] == ':' && p[1] == ':' && p > filename && p[-1] == '"' )
	{
	    // ":: composition: vms host/passwd component
	    ++components;
	    p = getnextcomp(p + 2);
	}
	else
# endif
	  if (vim_ispathsep(*p))
	    STRMOVE(p, p + 1);		// remove duplicate "/"
	else if (p[0] == '.' && (vim_ispathsep(p[1]) || p[1] == NUL))
	{
	    if (p == start && relative)
		p += 1 + (p[1] != NUL);	// keep single "." or leading "./"
	    else
	    {
		// Strip "./" or ".///".  If we are at the end of the file name
		// and there is no trailing path separator, either strip "/." if
		// we are after "start", or strip "." if we are at the beginning
		// of an absolute path name .
		tail = p + 1;
		if (p[1] != NUL)
		    while (vim_ispathsep(*tail))
			MB_PTR_ADV(tail);
		else if (p > start)
		    --p;		// strip preceding path separator
		STRMOVE(p, tail);
	    }
	}
	else if (p[0] == '.' && p[1] == '.' &&
	    (vim_ispathsep(p[2]) || p[2] == NUL))
	{
	    // Skip to after ".." or "../" or "..///".
	    tail = p + 2;
	    while (vim_ispathsep(*tail))
		MB_PTR_ADV(tail);

	    if (components > 0)		// strip one preceding component
	    {
		int		do_strip = FALSE;
		char_u		saved_char;
		stat_T		st;

		// Don't strip for an erroneous file name.
		if (!stripping_disabled)
		{
		    // If the preceding component does not exist in the file
		    // system, we strip it.  On Unix, we don't accept a symbolic
		    // link that refers to a non-existent file.
		    saved_char = p[-1];
		    p[-1] = NUL;
# ifdef UNIX
		    if (mch_lstat((char *)filename, &st) < 0)
# else
			if (mch_stat((char *)filename, &st) < 0)
# endif
			    do_strip = TRUE;
		    p[-1] = saved_char;

		    --p;
		    // Skip back to after previous '/'.
		    while (p > start && !after_pathsep(start, p))
			MB_PTR_BACK(start, p);

		    if (!do_strip)
		    {
			// If the component exists in the file system, check
			// that stripping it won't change the meaning of the
			// file name.  First get information about the
			// unstripped file name.  This may fail if the component
			// to strip is not a searchable directory (but a regular
			// file, for instance), since the trailing "/.." cannot
			// be applied then.  We don't strip it then since we
			// don't want to replace an erroneous file name by
			// a valid one, and we disable stripping of later
			// components.
			saved_char = *tail;
			*tail = NUL;
			if (mch_stat((char *)filename, &st) >= 0)
			    do_strip = TRUE;
			else
			    stripping_disabled = TRUE;
			*tail = saved_char;
# ifdef UNIX
			if (do_strip)
			{
			    stat_T	new_st;

			    // On Unix, the check for the unstripped file name
			    // above works also for a symbolic link pointing to
			    // a searchable directory.  But then the parent of
			    // the directory pointed to by the link must be the
			    // same as the stripped file name.  (The latter
			    // exists in the file system since it is the
			    // component's parent directory.)
			    if (p == start && relative)
				(void)mch_stat(".", &new_st);
			    else
			    {
				saved_char = *p;
				*p = NUL;
				(void)mch_stat((char *)filename, &new_st);
				*p = saved_char;
			    }

			    if (new_st.st_ino != st.st_ino ||
				new_st.st_dev != st.st_dev)
			    {
				do_strip = FALSE;
				// We don't disable stripping of later
				// components since the unstripped path name is
				// still valid.
			    }
			}
# endif
		    }
		}

		if (!do_strip)
		{
		    // Skip the ".." or "../" and reset the counter for the
		    // components that might be stripped later on.
		    p = tail;
		    components = 0;
		}
		else
		{
		    // Strip previous component.  If the result would get empty
		    // and there is no trailing path separator, leave a single
		    // "." instead.  If we are at the end of the file name and
		    // there is no trailing path separator and a preceding
		    // component is left after stripping, strip its trailing
		    // path separator as well.
		    if (p == start && relative && tail[-1] == '.')
		    {
			*p++ = '.';
			*p = NUL;
		    }
		    else
		    {
			if (p > start && tail[-1] == '.')
			    --p;
			STRMOVE(p, tail);	// strip previous component
		    }

		    --components;
		}
	    }
	    else if (p == start && !relative)	// leading "/.." or "/../"
		STRMOVE(p, tail);		// strip ".." or "../"
	    else
	    {
		if (p == start + 2 && p[-2] == '.')	// leading "./../"
		{
		    STRMOVE(p - 2, p);			// strip leading "./"
		    tail -= 2;
		}
		p = tail;		// skip to char after ".." or "../"
	    }
	}
	else
	{
	    ++components;		// simple path component
	    p = getnextcomp(p);
	}
    } while (*p != NUL);
#endif // !AMIGA
}

#if defined(FEAT_EVAL) || defined(PROTO)
/*
 * "simplify()" function
 */
    void
f_simplify(typval_T *argvars, typval_T *rettv)
{
    char_u	*p;

    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;

    p = tv_get_string_strict(&argvars[0]);
    rettv->vval.v_string = vim_strsave(p);
    simplify_filename(rettv->vval.v_string);	// simplify in place
    rettv->v_type = VAR_STRING;
}
#endif // FEAT_EVAL
