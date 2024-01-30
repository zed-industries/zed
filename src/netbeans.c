/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *			Netbeans integration by David Weatherford
 *			Adopted for Win32 by Sergey Khorev
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 */

/*
 * Implements client side of org.netbeans.modules.emacs editor
 * integration protocol.  Be careful!  The protocol uses offsets
 * which are *between* characters, whereas vim uses line number
 * and column number which are *on* characters.
 * See ":help netbeans-protocol" for explanation.
 *
 * The Netbeans messages are received and queued in the gui event loop, or in
 * the select loop when Vim runs in a terminal. These messages are processed
 * by netbeans_parse_messages() which is invoked in the idle loop when Vim is
 * waiting for user input. The function netbeans_parse_messages() is also
 * called from the ":sleep" command, to allow the execution of test cases that
 * may not invoke the idle loop.
 */

#include "vim.h"

#if defined(FEAT_NETBEANS_INTG) || defined(PROTO)

#ifndef MSWIN
# include <netdb.h>
# ifdef HAVE_LIBGEN_H
#  include <libgen.h>
# endif
#endif

#include "version.h"

#define GUARDED		10000 // typenr for "guarded" annotation
#define GUARDEDOFFSET 1000000 // base for "guarded" sign id's
#define MAX_COLOR_LENGTH 32 // max length of color name in defineAnnoType

// The first implementation (working only with Netbeans) returned "1.1".  The
// protocol implemented here also supports A-A-P.
static char *ExtEdProtocolVersion = "2.5";

static long pos2off(buf_T *, pos_T *);
static pos_T *off2pos(buf_T *, long);
static pos_T *get_off_or_lnum(buf_T *buf, char_u **argp);
static long get_buf_size(buf_T *);
static int netbeans_keystring(char_u *keystr);
static void special_keys(char_u *args);

static int getConnInfo(char *file, char **host, char **port, char **password);

static void nb_init_graphics(void);
static void coloncmd(char *cmd, ...) ATTRIBUTE_FORMAT_PRINTF(1, 2);
static void nb_set_curbuf(buf_T *buf);
static void nb_parse_cmd(char_u *);
static int  nb_do_cmd(int, char_u *, int, int, char_u *);
static void nb_send(char *buf, char *fun);
static void nb_free(void);

#define NETBEANS_OPEN (channel_can_write_to(nb_channel))
static channel_T *nb_channel = NULL;

static int r_cmdno;			// current command number for reply
static int dosetvisible = FALSE;

/*
 * Include the debugging code if wanted.
 */
#ifdef NBDEBUG
# include "nbdebug.c"
#endif

static int needupdate = 0;
static int inAtomic = 0;

/*
 * Callback invoked when the channel is closed.
 */
    static void
nb_channel_closed(void)
{
    nb_channel = NULL;
}

/*
 * Close the connection and cleanup.
 * May be called when the socket was closed earlier.
 */
    static void
netbeans_close(void)
{
    if (NETBEANS_OPEN)
    {
	netbeans_send_disconnect();
	if (nb_channel != NULL)
	{
	    // Close the socket and remove the input handlers.
	    channel_close(nb_channel, TRUE);
	    channel_clear(nb_channel);
	}
	nb_channel = NULL;
    }

#ifdef FEAT_BEVAL_GUI
    bevalServers &= ~BEVAL_NETBEANS;
#endif

    needupdate = 0;
    inAtomic = 0;
    nb_free();

    // remove all signs and update the screen after gutter removal
    coloncmd(":sign unplace *");
    changed_window_setting();
    update_screen(UPD_CLEAR);
    setcursor();
    cursor_on();
    out_flush_cursor(TRUE, FALSE);
}

#define NB_DEF_HOST "localhost"
#define NB_DEF_ADDR "3219"
#define NB_DEF_PASS "changeme"

    static int
netbeans_connect(char *params, int doabort)
{
    int		port;
    char	buf[32];
    char	*hostname = NULL;
    char	*address = NULL;
    char	*password = NULL;
    char	*fname;
    char	*arg = NULL;

    if (*params == '=')
    {
	// "=fname": Read info from specified file.
	if (getConnInfo(params + 1, &hostname, &address, &password) == FAIL)
	    return FAIL;
    }
    else
    {
	if (*params == ':')
	    // ":<host>:<addr>:<password>": get info from argument
	    arg = params + 1;
	if (arg == NULL && (fname = getenv("__NETBEANS_CONINFO")) != NULL)
	{
	    // "": get info from file specified in environment
	    if (getConnInfo(fname, &hostname, &address, &password) == FAIL)
		return FAIL;
	}
	else
	{
	    if (arg != NULL)
	    {
		// ":<host>:<addr>:<password>": get info from argument
		hostname = arg;
		address = strchr(hostname, ':');
		if (address != NULL)
		{
		    *address++ = '\0';
		    password = strchr(address, ':');
		    if (password != NULL)
			*password++ = '\0';
		}
	    }

	    // Get the missing values from the environment.
	    if (hostname == NULL || *hostname == '\0')
		hostname = getenv("__NETBEANS_HOST");
	    if (address == NULL)
		address = getenv("__NETBEANS_SOCKET");
	    if (password == NULL)
		password = getenv("__NETBEANS_VIM_PASSWORD");

	    // Move values to allocated memory.
	    if (hostname != NULL)
		hostname = (char *)vim_strsave((char_u *)hostname);
	    if (address != NULL)
		address = (char *)vim_strsave((char_u *)address);
	    if (password != NULL)
		password = (char *)vim_strsave((char_u *)password);
	}
    }

    // Use the default when a value is missing.
    if (hostname == NULL || *hostname == '\0')
    {
	vim_free(hostname);
	hostname = (char *)vim_strsave((char_u *)NB_DEF_HOST);
    }
    if (address == NULL || *address == '\0')
    {
	vim_free(address);
	address = (char *)vim_strsave((char_u *)NB_DEF_ADDR);
    }
    if (password == NULL || *password == '\0')
    {
	vim_free(password);
	password = (char *)vim_strsave((char_u *)NB_DEF_PASS);
    }
    if (hostname != NULL && address != NULL && password != NULL)
    {
	port = atoi(address);
	nb_channel = channel_open(hostname, port, 3000, nb_channel_closed);
	if (nb_channel != NULL)
	{
	    // success
# ifdef FEAT_BEVAL_GUI
	    bevalServers |= BEVAL_NETBEANS;
# endif

	    // success, login
	    vim_snprintf(buf, sizeof(buf), "AUTH %s\n", password);
	    nb_send(buf, "netbeans_connect");

	    sprintf(buf, "0:version=0 \"%s\"\n", ExtEdProtocolVersion);
	    nb_send(buf, "externaleditor_version");
	}
    }

    if (nb_channel == NULL && doabort)
	getout(1);

    vim_free(hostname);
    vim_free(address);
    vim_free(password);
    return NETBEANS_OPEN ? OK : FAIL;
}

/*
 * Obtain the NetBeans hostname, port address and password from a file.
 * Return the strings in allocated memory.
 * Return FAIL if the file could not be read, OK otherwise (no matter what it
 * contains).
 */
    static int
getConnInfo(char *file, char **host, char **port, char **auth)
{
    FILE *fp;
    char_u buf[BUFSIZ];
    char_u *lp;
    char_u *nlp;
#ifdef UNIX
    stat_T	st;

    /*
     * For Unix only accept the file when it's not accessible by others.
     * The open will then fail if we don't own the file.
     */
    if (mch_stat(file, &st) == 0 && (st.st_mode & 0077) != 0)
    {
	nbdebug(("Wrong access mode for NetBeans connection info file: \"%s\"\n",
								       file));
	semsg(_(e_wrong_access_mode_for_netbeans_connection_info_file_str),
									file);
	return FAIL;
    }
#endif

    fp = mch_fopen(file, "r");
    if (fp == NULL)
    {
	nbdebug(("Cannot open NetBeans connection info file\n"));
	PERROR(e_cannot_open_netbeans_connection_info_file);
	return FAIL;
    }

    // Read the file. There should be one of each parameter
    while ((lp = (char_u *)fgets((char *)buf, BUFSIZ, fp)) != NULL)
    {
	if ((nlp = vim_strchr(lp, '\n')) != NULL)
	    *nlp = 0;	    // strip off the trailing newline

	if (STRNCMP(lp, "host=", 5) == 0)
	{
	    vim_free(*host);
	    *host = (char *)vim_strsave(&buf[5]);
	}
	else if (STRNCMP(lp, "port=", 5) == 0)
	{
	    vim_free(*port);
	    *port = (char *)vim_strsave(&buf[5]);
	}
	else if (STRNCMP(lp, "auth=", 5) == 0)
	{
	    vim_free(*auth);
	    *auth = (char *)vim_strsave(&buf[5]);
	}
    }
    fclose(fp);

    return OK;
}


struct keyqueue
{
    char_u	    *keystr;
    struct keyqueue *next;
    struct keyqueue *prev;
};

typedef struct keyqueue keyQ_T;

static keyQ_T keyHead; // dummy node, header for circular queue


/*
 * Queue up key commands sent from netbeans.
 * We store the string, because it may depend on the global mod_mask and
 * :nbkey doesn't have a key number.
 */
    static void
postpone_keycommand(char_u *keystr)
{
    keyQ_T *node;

    node = ALLOC_ONE(keyQ_T);
    if (node == NULL)
	return;  // out of memory, drop the key

    if (keyHead.next == NULL) // initialize circular queue
    {
	keyHead.next = &keyHead;
	keyHead.prev = &keyHead;
    }

    // insert node at tail of queue
    node->next = &keyHead;
    node->prev = keyHead.prev;
    keyHead.prev->next = node;
    keyHead.prev = node;

    node->keystr = vim_strsave(keystr);
}

/*
 * Handle any queued-up NetBeans keycommands to be send.
 */
    static void
handle_key_queue(void)
{
    int postponed = FALSE;

    while (!postponed && keyHead.next && keyHead.next != &keyHead)
    {
	// first, unlink the node
	keyQ_T *node = keyHead.next;
	keyHead.next = node->next;
	node->next->prev = node->prev;

	// Now, send the keycommand.  This may cause it to be postponed again
	// and change keyHead.
	if (node->keystr != NULL)
	    postponed = !netbeans_keystring(node->keystr);
	vim_free(node->keystr);

	// Finally, dispose of the node
	vim_free(node);
    }
}


/*
 * While there's still a command in the work queue, parse and execute it.
 */
    void
netbeans_parse_messages(void)
{
    readq_T	*node;
    char_u	*buffer;
    char_u	*p;
    int		own_node;

    while (nb_channel != NULL)
    {
	node = channel_peek(nb_channel, PART_SOCK);
	if (node == NULL)
	    break;	// nothing to read

	// Locate the end of the first line in the first buffer.
	p = channel_first_nl(node);
	if (p == NULL)
	{
	    // Command isn't complete.  If there is no following buffer,
	    // return (wait for more). If there is another buffer following,
	    // prepend the text to that buffer and delete this one.
	    if (channel_collapse(nb_channel, PART_SOCK, TRUE) == FAIL)
		return;
	    continue;
	}

	// There is a complete command at the start of the buffer.
	// Terminate it with a NUL.  When no more text is following unlink
	// the buffer.  Do this before executing, because new buffers can
	// be added while busy handling the command.
	*p++ = NUL;
	if (*p == NUL)
	{
	    own_node = TRUE;
	    buffer = channel_get(nb_channel, PART_SOCK, NULL);
	    // "node" is now invalid!
	}
	else
	{
	    own_node = FALSE;
	    buffer = node->rq_buffer;
	}

	// Now, parse and execute the commands.  This may set nb_channel to
	// NULL if the channel is closed.
	nb_parse_cmd(buffer);

	if (own_node)
	    // buffer finished, dispose of it
	    vim_free(buffer);
	else if (nb_channel != NULL)
	    // more follows, move it to the start
	    channel_consume(nb_channel, PART_SOCK, (int)(p - buffer));
    }
}

/*
 * Handle one NUL terminated command.
 *
 * format of a command from netbeans:
 *
 *    6:setTitle!84 "a.c"
 *
 *    bufno
 *     colon
 *      cmd
 *		!
 *		 cmdno
 *		    args
 *
 * for function calls, the ! is replaced by a /
 */
    static void
nb_parse_cmd(char_u *cmd)
{
    char	*verb;
    char	*q;
    int		bufno;
    int		isfunc = -1;

    if (STRCMP(cmd, "DISCONNECT") == 0)
    {
	// We assume the server knows that we can safely exit!
	// Disconnect before exiting, Motif hangs in a Select error
	// message otherwise.
	netbeans_close();
	getout(0);
	// NOTREACHED
    }

    if (STRCMP(cmd, "DETACH") == 0)
    {
	buf_T	*buf;

	FOR_ALL_BUFFERS(buf)
	    buf->b_has_sign_column = FALSE;

	// The IDE is breaking the connection.
	netbeans_close();
	return;
    }

    bufno = strtol((char *)cmd, &verb, 10);

    if (*verb != ':')
    {
	nbdebug(("    missing colon: %s\n", cmd));
	semsg(_(e_missing_colon_str), cmd);
	return;
    }
    ++verb; // skip colon

    for (q = verb; *q; q++)
    {
	if (*q == '!')
	{
	    *q++ = NUL;
	    isfunc = 0;
	    break;
	}
	else if (*q == '/')
	{
	    *q++ = NUL;
	    isfunc = 1;
	    break;
	}
    }

    if (isfunc < 0)
    {
	nbdebug(("    missing ! or / in: %s\n", cmd));
	semsg(_(e_missing_bang_or_slash_in_str), cmd);
	return;
    }

    r_cmdno = strtol(q, &q, 10);

    q = (char *)skipwhite((char_u *)q);

    if (nb_do_cmd(bufno, (char_u *)verb, isfunc, r_cmdno, (char_u *)q) == FAIL)
    {
#ifdef NBDEBUG
	/*
	 * This happens because the ExtEd can send a command or 2 after
	 * doing a stopDocumentListen command. It doesn't harm anything
	 * so I'm disabling it except for debugging.
	 */
	nbdebug(("nb_parse_cmd: Command error for \"%s\"\n", cmd));
	emsg(e_bad_return_from_nb_do_cmd);
#endif
    }
}

struct nbbuf_struct
{
    buf_T		*bufp;
    unsigned int	 fireChanges:1;
    unsigned int	 initDone:1;
    unsigned int	 insertDone:1;
    unsigned int	 modified:1;
    int			 nbbuf_number;
    char		*displayname;
    int			*signmap;
    short_u		 signmaplen;
    short_u		 signmapused;
};

typedef struct nbbuf_struct nbbuf_T;

static nbbuf_T *buf_list = NULL;
static int buf_list_size = 0;	// size of buf_list
static int buf_list_used = 0;	// nr of entries in buf_list actually in use

static char **globalsignmap = NULL;
static int globalsignmaplen = 0;
static int globalsignmapused = 0;

static int  mapsigntype(nbbuf_T *, int localsigntype);
static void addsigntype(nbbuf_T *, int localsigntype, char_u *typeName,
			char_u *tooltip, char_u *glyphfile,
			char_u *fg, char_u *bg);
static void print_read_msg(nbbuf_T *buf);
static void print_save_msg(nbbuf_T *buf, off_T nchars);

static int curPCtype = -1;

/*
 * Free netbeans resources.
 */
    static void
nb_free(void)
{
    keyQ_T *key_node = keyHead.next;
    nbbuf_T buf;
    int i;

    // free the netbeans buffer list
    for (i = 0; i < buf_list_used; i++)
    {
	buf = buf_list[i];
	vim_free(buf.displayname);
	vim_free(buf.signmap);
	if (buf.bufp != NULL && buf_valid(buf.bufp))
	{
	    buf.bufp->b_netbeans_file = FALSE;
	    buf.bufp->b_was_netbeans_file = FALSE;
	}
    }
    VIM_CLEAR(buf_list);
    buf_list_size = 0;
    buf_list_used = 0;

    // free the queued key commands
    while (key_node != NULL && key_node != &keyHead)
    {
	keyQ_T *next = key_node->next;
	vim_free(key_node->keystr);
	vim_free(key_node);
	if (next == &keyHead)
	{
	    keyHead.next = &keyHead;
	    keyHead.prev = &keyHead;
	    break;
	}
	key_node = next;
    }

    // free the queued netbeans commands
    if (nb_channel != NULL)
	channel_clear(nb_channel);
}

/*
 * Get the Netbeans buffer number for the specified buffer.
 */
    static int
nb_getbufno(buf_T *bufp)
{
    int i;

    for (i = 0; i < buf_list_used; i++)
	if (buf_list[i].bufp == bufp)
	    return i;
    return -1;
}

/*
 * Is this a NetBeans-owned buffer?
 */
    int
isNetbeansBuffer(buf_T *bufp)
{
    return NETBEANS_OPEN && bufp->b_netbeans_file;
}

/*
 * NetBeans and Vim have different undo models. In Vim, the file isn't
 * changed if changes are undone via the undo command. In NetBeans, once
 * a change has been made the file is marked as modified until saved. It
 * doesn't matter if the change was undone.
 *
 * So this function is for the corner case where Vim thinks a buffer is
 * unmodified but NetBeans thinks it IS modified.
 */
    int
isNetbeansModified(buf_T *bufp)
{
    if (isNetbeansBuffer(bufp))
    {
	int bufno = nb_getbufno(bufp);

	if (bufno > 0)
	    return buf_list[bufno].modified;
	else
	    return FALSE;
    }
    else
	return FALSE;
}

/*
 * Given a Netbeans buffer number, return the netbeans buffer.
 * Returns NULL for 0 or a negative number. A 0 bufno means a
 * non-buffer related command has been sent.
 */
    static nbbuf_T *
nb_get_buf(int bufno)
{
    // find or create a buffer with the given number
    int incr;

    if (bufno <= 0)
	return NULL;

    if (!buf_list)
    {
	// initialize
	buf_list = alloc_clear(100 * sizeof(nbbuf_T));
	buf_list_size = 100;
    }
    if (bufno >= buf_list_used) // new
    {
	if (bufno >= buf_list_size) // grow list
	{
	    nbbuf_T	*t_buf_list = buf_list;
	    size_t	bufsize;

	    incr = bufno - buf_list_size + 90;
	    buf_list_size += incr;
	    bufsize = buf_list_size * sizeof(nbbuf_T);
	    if (bufsize == 0 || bufsize / sizeof(nbbuf_T)
						      != (size_t)buf_list_size)
	    {
		// list size overflow, bail out
		return NULL;
	    }
	    buf_list = vim_realloc(buf_list, bufsize);
	    if (buf_list == NULL)
	    {
		vim_free(t_buf_list);
		buf_list_size = 0;
		return NULL;
	    }
	    vim_memset(buf_list + buf_list_size - incr, 0,
						      incr * sizeof(nbbuf_T));
	}

	while (buf_list_used <= bufno)
	{
	    // Default is to fire text changes.
	    buf_list[buf_list_used].fireChanges = 1;
	    ++buf_list_used;
	}
    }

    return buf_list + bufno;
}

/*
 * Return the number of buffers that are modified.
 */
    static int
count_changed_buffers(void)
{
    buf_T	*bufp;
    int		n;

    n = 0;
    FOR_ALL_BUFFERS(bufp)
	if (bufp->b_changed)
	    ++n;
    return n;
}

/*
 * End the netbeans session.
 */
    void
netbeans_end(void)
{
    int i;
    static char buf[128];

    if (!NETBEANS_OPEN)
	return;

    for (i = 0; i < buf_list_used; i++)
    {
	if (!buf_list[i].bufp)
	    continue;
	if (netbeansForcedQuit)
	{
	    // mark as unmodified so NetBeans won't put up dialog on "killed"
	    sprintf(buf, "%d:unmodified=%d\n", i, r_cmdno);
	    nbdebug(("EVT: %s", buf));
	    nb_send(buf, "netbeans_end");
	}
	sprintf(buf, "%d:killed=%d\n", i, r_cmdno);
	nbdebug(("EVT: %s", buf));
	// nb_send(buf, "netbeans_end");    avoid "write failed" messages
	nb_send(buf, NULL);
	buf_list[i].bufp = NULL;
    }
}

/*
 * Send a message to netbeans.
 * When "fun" is NULL no error is given.
 */
    static void
nb_send(char *buf, char *fun)
{
    if (nb_channel != NULL)
	channel_send(nb_channel, PART_SOCK, (char_u *)buf,
						       (int)STRLEN(buf), fun);
}

/*
 * Some input received from netbeans requires a response. This function
 * handles a response with no information (except the command number).
 */
    static void
nb_reply_nil(int cmdno)
{
    char reply[32];

    nbdebug(("REP %d: <none>\n", cmdno));

    // Avoid printing an annoying error message.
    if (!NETBEANS_OPEN)
	return;

    sprintf(reply, "%d\n", cmdno);
    nb_send(reply, "nb_reply_nil");
}


/*
 * Send a response with text.
 * "result" must have been quoted already (using nb_quote()).
 */
    static void
nb_reply_text(int cmdno, char_u *result)
{
    char_u *reply;

    nbdebug(("REP %d: %s\n", cmdno, (char *)result));

    reply = alloc(STRLEN(result) + 32);
    sprintf((char *)reply, "%d %s\n", cmdno, (char *)result);
    nb_send((char *)reply, "nb_reply_text");

    vim_free(reply);
}


/*
 * Send a response with a number result code.
 */
    static void
nb_reply_nr(int cmdno, long result)
{
    char reply[32];

    nbdebug(("REP %d: %ld\n", cmdno, result));

    sprintf(reply, "%d %ld\n", cmdno, result);
    nb_send(reply, "nb_reply_nr");
}


/*
 * Encode newline, ret, backslash, double quote for transmission to NetBeans.
 */
    static char_u *
nb_quote(char_u *txt)
{
    char_u *buf = alloc(2 * STRLEN(txt) + 1);
    char_u *p = txt;
    char_u *q = buf;

    if (buf == NULL)
	return NULL;
    for (; *p; p++)
    {
	switch (*p)
	{
	    case '\"':
	    case '\\':
		*q++ = '\\'; *q++ = *p; break;
	 // case '\t':
	 //     *q++ = '\\'; *q++ = 't'; break;
	    case '\n':
		*q++ = '\\'; *q++ = 'n'; break;
	    case '\r':
		*q++ = '\\'; *q++ = 'r'; break;
	    default:
		*q++ = *p;
		break;
	}
    }
    *q = '\0';

    return buf;
}


/*
 * Remove top level double quotes; convert backslashed chars.
 * Returns an allocated string (NULL for failure).
 * If "endp" is not NULL it is set to the character after the terminating
 * quote.
 */
    static char *
nb_unquote(char_u *p, char_u **endp)
{
    char *result = 0;
    char *q;
    int done = 0;

    // result is never longer than input
    result = alloc_clear(STRLEN(p) + 1);
    if (result == NULL)
	return NULL;

    if (*p++ != '"')
    {
	nbdebug(("nb_unquote called with string that doesn't start with a quote!: %s\n",
			p));
	result[0] = NUL;
	return result;
    }

    for (q = result; !done && *p != NUL;)
    {
	switch (*p)
	{
	    case '"':
		/*
		 * Unbackslashed dquote marks the end, if first char was dquote.
		 */
		done = 1;
		break;

	    case '\\':
		++p;
		switch (*p)
		{
		    case '\\':	*q++ = '\\';	break;
		    case 'n':	*q++ = '\n';	break;
		    case 't':	*q++ = '\t';	break;
		    case 'r':	*q++ = '\r';	break;
		    case '"':	*q++ = '"';	break;
		    case NUL:	--p;		break;
		    // default: skip over illegal chars
		}
		++p;
		break;

	    default:
		*q++ = *p++;
	}
    }

    if (endp != NULL)
	*endp = p;

    return result;
}

/*
 * Remove from "first" byte to "last" byte (inclusive), at line "lnum" of the
 * current buffer.  Remove to end of line when "last" is MAXCOL.
 */
    static void
nb_partialremove(linenr_T lnum, colnr_T first, colnr_T last)
{
    char_u *oldtext, *newtext;
    int oldlen;
    int lastbyte = last;

    oldtext = ml_get(lnum);
    oldlen = (int)STRLEN(oldtext);
    if (first >= (colnr_T)oldlen || oldlen == 0)  // just in case
	return;
    if (lastbyte >= oldlen)
	lastbyte = oldlen - 1;
    newtext = alloc(oldlen - (int)(lastbyte - first));
    if (newtext == NULL)
	return;

    mch_memmove(newtext, oldtext, first);
    STRMOVE(newtext + first, oldtext + lastbyte + 1);
    nbdebug(("    NEW LINE %ld: %s\n", lnum, newtext));
    ml_replace(lnum, newtext, FALSE);
}

/*
 * Replace the "first" line with the concatenation of the "first" and
 * the "other" line. The "other" line is not removed.
 */
    static void
nb_joinlines(linenr_T first, linenr_T other)
{
    int len_first, len_other;
    char_u *p;

    len_first = (int)STRLEN(ml_get(first));
    len_other = (int)STRLEN(ml_get(other));
    p = alloc(len_first + len_other + 1);
    if (p == NULL)
	return;

    mch_memmove(p, ml_get(first), len_first);
    mch_memmove(p + len_first, ml_get(other), len_other + 1);
    ml_replace(first, p, FALSE);
}

#define SKIP_STOP 2
#define streq(a,b) (strcmp(a,b) == 0)

/*
 * Do the actual processing of a single netbeans command or function.
 * The difference between a command and function is that a function
 * gets a response (it's required) but a command does not.
 * For arguments see comment for nb_parse_cmd().
 */
    static int
nb_do_cmd(
    int		bufno,
    char_u	*cmd,
    int		func,
    int		cmdno,
    char_u	*args)	    // points to space before arguments or NUL
{
    int		do_update = 0;
    long	off = 0;
    nbbuf_T	*buf = nb_get_buf(bufno);
    static int	skip = 0;
    int		retval = OK;
    char	*cp;	    // for when a char pointer is needed

    nbdebug(("%s %d: (%d) %s %s\n", (func) ? "FUN" : "CMD", cmdno, bufno, cmd,
	STRCMP(cmd, "insert") == 0 ? "<text>" : (char *)args));

    if (func)
    {
// =====================================================================
	if (streq((char *)cmd, "getModified"))
	{
	    if (buf == NULL || buf->bufp == NULL)
		// Return the number of buffers that are modified.
		nb_reply_nr(cmdno, (long)count_changed_buffers());
	    else
		// Return whether the buffer is modified.
		nb_reply_nr(cmdno, (long)(buf->bufp->b_changed
					   || isNetbeansModified(buf->bufp)));
// =====================================================================
	}
	else if (streq((char *)cmd, "saveAndExit"))
	{
	    // Note: this will exit Vim if successful.
	    coloncmd(":confirm qall");

	    // We didn't exit: return the number of changed buffers.
	    nb_reply_nr(cmdno, (long)count_changed_buffers());
// =====================================================================
	}
	else if (streq((char *)cmd, "getCursor"))
	{
	    char_u text[200];

	    // Note: nb_getbufno() may return -1.  This indicates the IDE
	    // didn't assign a number to the current buffer in response to a
	    // fileOpened event.
	    sprintf((char *)text, "%d %ld %d %ld",
		    nb_getbufno(curbuf),
		    (long)curwin->w_cursor.lnum,
		    (int)curwin->w_cursor.col,
		    pos2off(curbuf, &curwin->w_cursor));
	    nb_reply_text(cmdno, text);
// =====================================================================
	}
	else if (streq((char *)cmd, "getAnno"))
	{
	    long linenum = 0;
#ifdef FEAT_SIGNS
	    if (buf == NULL || buf->bufp == NULL)
	    {
		nbdebug(("    Invalid buffer identifier in getAnno\n"));
		emsg(_(e_invalid_buffer_identifier_in_getanno));
		retval = FAIL;
	    }
	    else
	    {
		int serNum;

		cp = (char *)args;
		serNum = strtol(cp, &cp, 10);
		// If the sign isn't found linenum will be zero.
		linenum = (long)buf_findsign(buf->bufp, serNum, NULL);
	    }
#endif
	    nb_reply_nr(cmdno, linenum);
// =====================================================================
	}
	else if (streq((char *)cmd, "getLength"))
	{
	    long len = 0;

	    if (buf == NULL || buf->bufp == NULL)
	    {
		nbdebug(("    invalid buffer identifier in getLength\n"));
		emsg(_(e_invalid_buffer_identifier_in_getlength));
		retval = FAIL;
	    }
	    else
	    {
		len = get_buf_size(buf->bufp);
	    }
	    nb_reply_nr(cmdno, len);
// =====================================================================
	}
	else if (streq((char *)cmd, "getText"))
	{
	    long	len;
	    linenr_T	nlines;
	    char_u	*text = NULL;
	    linenr_T	lno = 1;
	    char_u	*p;
	    char_u	*line;

	    if (buf == NULL || buf->bufp == NULL)
	    {
		nbdebug(("    invalid buffer identifier in getText\n"));
		emsg(_(e_invalid_buffer_identifier_in_gettext));
		retval = FAIL;
	    }
	    else
	    {
		len = get_buf_size(buf->bufp);
		nlines = buf->bufp->b_ml.ml_line_count;
		text = alloc((len > 0) ? ((len + nlines) * 2) : 4);
		if (text == NULL)
		{
		    nbdebug(("    nb_do_cmd: getText has null text field\n"));
		    retval = FAIL;
		}
		else
		{
		    p = text;
		    *p++ = '\"';
		    for (; lno <= nlines ; lno++)
		    {
			line = nb_quote(ml_get_buf(buf->bufp, lno, FALSE));
			if (line != NULL)
			{
			    STRCPY(p, line);
			    p += STRLEN(line);
			    *p++ = '\\';
			    *p++ = 'n';
			    vim_free(line);
			}
		    }
		    *p++ = '\"';
		    *p = '\0';
		}
	    }
	    if (text == NULL)
		nb_reply_text(cmdno, (char_u *)"");
	    else
	    {
		nb_reply_text(cmdno, text);
		vim_free(text);
	    }
// =====================================================================
	}
	else if (streq((char *)cmd, "remove"))
	{
	    long count;
	    pos_T first, last;
	    pos_T *pos;
	    pos_T *next;
	    linenr_T del_from_lnum, del_to_lnum;  // lines to be deleted as a whole
	    int oldFire = netbeansFireChanges;
	    int oldSuppress = netbeansSuppressNoLines;
	    int wasChanged;

	    if (skip >= SKIP_STOP)
	    {
		nbdebug(("    Skipping %s command\n", (char *) cmd));
		nb_reply_nil(cmdno);
		return OK;
	    }

	    if (buf == NULL || buf->bufp == NULL)
	    {
		nbdebug(("    invalid buffer identifier in remove\n"));
		emsg(_(e_invalid_buffer_identifier_in_remove));
		retval = FAIL;
	    }
	    else
	    {
		netbeansFireChanges = FALSE;
		netbeansSuppressNoLines = TRUE;

		nb_set_curbuf(buf->bufp);
		wasChanged = buf->bufp->b_changed;
		cp = (char *)args;
		off = strtol(cp, &cp, 10);
		count = strtol(cp, &cp, 10);
		args = (char_u *)cp;
		// delete "count" chars, starting at "off"
		pos = off2pos(buf->bufp, off);
		if (!pos)
		{
		    nbdebug(("    !bad position\n"));
		    nb_reply_text(cmdno, (char_u *)"!bad position");
		    netbeansFireChanges = oldFire;
		    netbeansSuppressNoLines = oldSuppress;
		    return FAIL;
		}
		first = *pos;
		nbdebug(("    FIRST POS: line %ld, col %d\n",
						      first.lnum, first.col));
		pos = off2pos(buf->bufp, off+count-1);
		if (!pos)
		{
		    nbdebug(("    !bad count\n"));
		    nb_reply_text(cmdno, (char_u *)"!bad count");
		    netbeansFireChanges = oldFire;
		    netbeansSuppressNoLines = oldSuppress;
		    return FAIL;
		}
		last = *pos;
		nbdebug(("    LAST POS: line %ld, col %d\n",
							last.lnum, last.col));
		del_from_lnum = first.lnum;
		del_to_lnum = last.lnum;
		do_update = 1;

		// Get the position of the first byte after the deleted
		// section.  "next" is NULL when deleting to the end of the
		// file.
		next = off2pos(buf->bufp, off + count);

		// Remove part of the first line.
		if (first.col != 0
				|| (next != NULL && first.lnum == next->lnum))
		{
		    if (first.lnum != last.lnum
			    || (next != NULL && first.lnum != next->lnum))
		    {
			// remove to the end of the first line
			nb_partialremove(first.lnum, first.col,
							     (colnr_T)MAXCOL);
			if (first.lnum == last.lnum)
			{
			    // Partial line to remove includes the end of
			    // line.  Join the line with the next one, have
			    // the next line deleted below.
			    nb_joinlines(first.lnum, next->lnum);
			    del_to_lnum = next->lnum;
			}
		    }
		    else
		    {
			// remove within one line
			nb_partialremove(first.lnum, first.col, last.col);
		    }
		    ++del_from_lnum;  // don't delete the first line
		}

		// Remove part of the last line.
		if (first.lnum != last.lnum && next != NULL
			&& next->col != 0 && last.lnum == next->lnum)
		{
		    nb_partialremove(last.lnum, 0, last.col);
		    if (del_from_lnum > first.lnum)
		    {
			// Join end of last line to start of first line; last
			// line is deleted below.
			nb_joinlines(first.lnum, last.lnum);
		    }
		    else
			// First line is deleted as a whole, keep the last
			// line.
			--del_to_lnum;
		}

		// First is partial line; last line to remove includes
		// the end of line; join first line to line following last
		// line; line following last line is deleted below.
		if (first.lnum != last.lnum && del_from_lnum > first.lnum
			&& next != NULL && last.lnum != next->lnum)
		{
		    nb_joinlines(first.lnum, next->lnum);
		    del_to_lnum = next->lnum;
		}

		// Delete whole lines if there are any.
		if (del_to_lnum >= del_from_lnum)
		{
		    int i;

		    // delete signs from the lines being deleted
		    for (i = del_from_lnum; i <= del_to_lnum; i++)
		    {
			int id = buf_findsign_id(buf->bufp, (linenr_T)i, NULL);
			if (id > 0)
			{
			    nbdebug(("    Deleting sign %d on line %d\n",
								      id, i));
			    buf_delsign(buf->bufp, 0, id, NULL);
			}
			else
			{
			    nbdebug(("    No sign on line %d\n", i));
			}
		    }

		    nbdebug(("    Deleting lines %ld through %ld\n",
						 del_from_lnum, del_to_lnum));
		    curwin->w_cursor.lnum = del_from_lnum;
		    curwin->w_cursor.col = 0;
		    del_lines(del_to_lnum - del_from_lnum + 1, FALSE);
		}

		// Leave cursor at first deleted byte.
		curwin->w_cursor = first;
		check_cursor_lnum();
		buf->bufp->b_changed = wasChanged; // logically unchanged
		netbeansFireChanges = oldFire;
		netbeansSuppressNoLines = oldSuppress;

		u_blockfree(buf->bufp);
		u_clearall(buf->bufp);
	    }
	    nb_reply_nil(cmdno);
// =====================================================================
	}
	else if (streq((char *)cmd, "insert"))
	{
	    char_u	*to_free;

	    if (skip >= SKIP_STOP)
	    {
		nbdebug(("    Skipping %s command\n", (char *) cmd));
		nb_reply_nil(cmdno);
		return OK;
	    }

	    // get offset
	    cp = (char *)args;
	    off = strtol(cp, &cp, 10);
	    args = (char_u *)cp;

	    // get text to be inserted
	    args = skipwhite(args);
	    args = to_free = (char_u *)nb_unquote(args, NULL);
	    /*
	    nbdebug(("    CHUNK[%d]: %d bytes at offset %d\n",
		    buf->bufp->b_ml.ml_line_count, STRLEN(args), off));
	    */

	    if (buf == NULL || buf->bufp == NULL)
	    {
		nbdebug(("    invalid buffer identifier in insert\n"));
		emsg(_(e_invalid_buffer_identifier_in_insert));
		retval = FAIL;
	    }
	    else if (args != NULL)
	    {
		int	ff_detected = EOL_UNKNOWN;
		int	buf_was_empty = (buf->bufp->b_ml.ml_flags & ML_EMPTY);
		size_t	len = 0;
		int	added = 0;
		int	oldFire = netbeansFireChanges;
		int	old_b_changed;
		char_u	*nlp;
		linenr_T lnum;
		linenr_T lnum_start;
		pos_T	*pos;

		netbeansFireChanges = 0;

		// Jump to the buffer where we insert.  After this "curbuf"
		// can be used.
		nb_set_curbuf(buf->bufp);
		old_b_changed = curbuf->b_changed;

		// Convert the specified character offset into a lnum/col
		// position.
		pos = off2pos(curbuf, off);
		if (pos != NULL)
		{
		    if (pos->lnum <= 0)
			lnum_start = 1;
		    else
			lnum_start = pos->lnum;
		}
		else
		{
		    // If the given position is not found, assume we want
		    // the end of the file.  See setLocAndSize HACK.
		    if (buf_was_empty)
			lnum_start = 1;	    // above empty line
		    else
			lnum_start = curbuf->b_ml.ml_line_count + 1;
		}

		// "lnum" is the line where we insert: either append to it or
		// insert a new line above it.
		lnum = lnum_start;

		// Loop over the "\n" separated lines of the argument.
		do_update = 1;
		while (*args != NUL)
		{
		    nlp = vim_strchr(args, '\n');
		    if (nlp == NULL)
		    {
			// Incomplete line, probably truncated.  Next "insert"
			// command should append to this one.
			len = STRLEN(args);
		    }
		    else
		    {
			len = nlp - args;

			/*
			 * We need to detect EOL style, because the commands
			 * use a character offset.
			 */
			if (nlp > args && nlp[-1] == '\r')
			{
			    ff_detected = EOL_DOS;
			    --len;
			}
			else
			    ff_detected = EOL_UNIX;
		    }
		    args[len] = NUL;

		    if (lnum == lnum_start
			    && ((pos != NULL && pos->col > 0)
				|| (lnum == 1 && buf_was_empty)))
		    {
			char_u	*oldline = ml_get(lnum);
			char_u	*newline;
			int	col = pos == NULL ? 0 : pos->col;

			// Insert halfway a line.
			newline = alloc(STRLEN(oldline) + len + 1);
			if (newline != NULL)
			{
			    mch_memmove(newline, oldline, (size_t)col);
			    newline[col] = NUL;
			    STRCAT(newline, args);
			    STRCAT(newline, oldline + col);
			    ml_replace(lnum, newline, FALSE);
			}
		    }
		    else
		    {
			// Append a new line.  Not that we always do this,
			// also when the text doesn't end in a "\n".
			ml_append((linenr_T)(lnum - 1), args,
						   (colnr_T)(len + 1), FALSE);
			++added;
		    }

		    if (nlp == NULL)
			break;
		    ++lnum;
		    args = nlp + 1;
		}

		// Adjust the marks below the inserted lines.
		appended_lines_mark(lnum_start - 1, (long)added);

		/*
		 * When starting with an empty buffer set the fileformat.
		 * This is just guessing...
		 */
		if (buf_was_empty)
		{
		    if (ff_detected == EOL_UNKNOWN)
#if defined(MSWIN)
			ff_detected = EOL_DOS;
#else
			ff_detected = EOL_UNIX;
#endif
		    set_fileformat(ff_detected, OPT_LOCAL);
		    curbuf->b_start_ffc = *curbuf->b_p_ff;
		}

		/*
		 * XXX - GRP - Is the next line right? If I've inserted
		 * text the buffer has been updated but not written. Will
		 * netbeans guarantee to write it? Even if I do a :q! ?
		 */
		curbuf->b_changed = old_b_changed; // logically unchanged
		netbeansFireChanges = oldFire;

		// Undo info is invalid now...
		u_blockfree(curbuf);
		u_clearall(curbuf);
	    }
	    vim_free(to_free);
	    nb_reply_nil(cmdno); // or !error
	}
	else
	{
	    nbdebug(("UNIMPLEMENTED FUNCTION: %s\n", cmd));
	    nb_reply_nil(cmdno);
	    retval = FAIL;
	}
    }
    else // Not a function; no reply required.
    {
// =====================================================================
	if (streq((char *)cmd, "create"))
	{
	    // Create a buffer without a name.
	    if (buf == NULL)
	    {
		nbdebug(("    invalid buffer identifier in create\n"));
		emsg(_(e_invalid_buffer_identifier_in_create));
		return FAIL;
	    }
	    VIM_CLEAR(buf->displayname);

	    netbeansReadFile = 0; // don't try to open disk file
	    do_ecmd(0, NULL, 0, 0, ECMD_ONE, ECMD_HIDE + ECMD_OLDBUF, curwin);
	    netbeansReadFile = 1;
	    buf->bufp = curbuf;
	    maketitle();
	    buf->insertDone = FALSE;
#if defined(FEAT_MENU) && defined(FEAT_GUI)
	    if (gui.in_use)
		gui_update_menus(0);
#endif
// =====================================================================
	}
	else if (streq((char *)cmd, "insertDone"))
	{
	    if (buf == NULL || buf->bufp == NULL)
	    {
		nbdebug(("    invalid buffer identifier in insertDone\n"));
	    }
	    else
	    {
		buf->bufp->b_start_eol = *args == 'T';
		buf->insertDone = TRUE;
		args += 2;
		buf->bufp->b_p_ro = *args == 'T';
		print_read_msg(buf);
	    }
// =====================================================================
	}
	else if (streq((char *)cmd, "saveDone"))
	{
	    long savedChars = atol((char *)args);

	    if (buf == NULL || buf->bufp == NULL)
		nbdebug(("    invalid buffer identifier in saveDone\n"));
	    else
		print_save_msg(buf, savedChars);
// =====================================================================
	}
	else if (streq((char *)cmd, "startDocumentListen"))
	{
	    if (buf == NULL)
	    {
		nbdebug(("    invalid buffer identifier in startDocumentListen\n"));
		emsg(_(e_invalid_buffer_identifier_in_startdocumentlisten));
		return FAIL;
	    }
	    buf->fireChanges = 1;
// =====================================================================
	}
	else if (streq((char *)cmd, "stopDocumentListen"))
	{
	    if (buf == NULL)
	    {
		nbdebug(("    invalid buffer identifier in stopDocumentListen\n"));
		emsg(_(e_invalid_buffer_identifier_in_stopdocumentlisten));
		return FAIL;
	    }
	    buf->fireChanges = 0;
	    if (buf->bufp != NULL && buf->bufp->b_was_netbeans_file)
	    {
		if (!buf->bufp->b_netbeans_file)
		{
		    nbdebug((e_netbeans_connection_lost_for_buffer_nr,
							   buf->bufp->b_fnum));
		    semsg(_(e_netbeans_connection_lost_for_buffer_nr),
							   buf->bufp->b_fnum);
		}
		else
		{
		    // NetBeans uses stopDocumentListen when it stops editing
		    // a file.  It then expects the buffer in Vim to
		    // disappear.
		    do_bufdel(DOBUF_DEL, (char_u *)"", 1,
				  buf->bufp->b_fnum, buf->bufp->b_fnum, TRUE);
		    CLEAR_POINTER(buf);
		}
	    }
// =====================================================================
	}
	else if (streq((char *)cmd, "setTitle"))
	{
	    if (buf == NULL)
	    {
		nbdebug(("    invalid buffer identifier in setTitle\n"));
		emsg(_(e_invalid_buffer_identifier_in_settitle));
		return FAIL;
	    }
	    vim_free(buf->displayname);
	    buf->displayname = nb_unquote(args, NULL);
// =====================================================================
	}
	else if (streq((char *)cmd, "initDone"))
	{
	    if (buf == NULL || buf->bufp == NULL)
	    {
		nbdebug(("    invalid buffer identifier in initDone\n"));
		emsg(_(e_invalid_buffer_identifier_in_initdone));
		return FAIL;
	    }
	    do_update = 1;
	    buf->initDone = TRUE;
	    nb_set_curbuf(buf->bufp);
	    apply_autocmds(EVENT_BUFREADPOST, 0, 0, FALSE, buf->bufp);

	    // handle any postponed key commands
	    handle_key_queue();
// =====================================================================
	}
	else if (streq((char *)cmd, "setBufferNumber")
		|| streq((char *)cmd, "putBufferNumber"))
	{
	    char_u	*path;
	    buf_T	*bufp;

	    if (buf == NULL)
	    {
		nbdebug(("    invalid buffer identifier in setBufferNumber\n"));
		emsg(_(e_invalid_buffer_identifier_in_setbuffernumber));
		return FAIL;
	    }
	    path = (char_u *)nb_unquote(args, NULL);
	    if (path == NULL)
		return FAIL;
	    bufp = buflist_findname(path);
	    vim_free(path);
	    if (bufp == NULL)
	    {
		nbdebug(("    File %s not found in setBufferNumber\n", args));
		semsg(_(e_file_str_not_found_in_setbuffernumber), args);
		return FAIL;
	    }
	    buf->bufp = bufp;
	    buf->nbbuf_number = bufp->b_fnum;

	    // "setBufferNumber" has the side effect of jumping to the buffer
	    // (don't know why!).  Don't do that for "putBufferNumber".
	    if (*cmd != 'p')
		coloncmd(":buffer %d", bufp->b_fnum);
	    else
	    {
		buf->initDone = TRUE;

		// handle any postponed key commands
		handle_key_queue();
	    }

// =====================================================================
	}
	else if (streq((char *)cmd, "setFullName"))
	{
	    if (buf == NULL)
	    {
		nbdebug(("    invalid buffer identifier in setFullName\n"));
		emsg(_(e_invalid_buffer_identifier_in_setfullname));
		return FAIL;
	    }
	    vim_free(buf->displayname);
	    buf->displayname = nb_unquote(args, NULL);

	    netbeansReadFile = 0; // don't try to open disk file
	    do_ecmd(0, (char_u *)buf->displayname, 0, 0, ECMD_ONE,
					     ECMD_HIDE + ECMD_OLDBUF, curwin);
	    netbeansReadFile = 1;
	    buf->bufp = curbuf;
	    maketitle();
#if defined(FEAT_MENU) && defined(FEAT_GUI)
	    if (gui.in_use)
		gui_update_menus(0);
#endif
// =====================================================================
	}
	else if (streq((char *)cmd, "editFile"))
	{
	    if (buf == NULL)
	    {
		nbdebug(("    invalid buffer identifier in editFile\n"));
		emsg(_(e_invalid_buffer_identifier_in_editfile));
		return FAIL;
	    }
	    // Edit a file: like create + setFullName + read the file.
	    vim_free(buf->displayname);
	    buf->displayname = nb_unquote(args, NULL);
	    do_ecmd(0, (char_u *)buf->displayname, NULL, NULL, ECMD_ONE,
					     ECMD_HIDE + ECMD_OLDBUF, curwin);
	    buf->bufp = curbuf;
	    buf->initDone = TRUE;
	    do_update = 1;
	    maketitle();
#if defined(FEAT_MENU) && defined(FEAT_GUI)
	    if (gui.in_use)
		gui_update_menus(0);
#endif
// =====================================================================
	}
	else if (streq((char *)cmd, "setVisible"))
	{
	    if (buf == NULL || buf->bufp == NULL)
	    {
		nbdebug(("    invalid buffer identifier in setVisible\n"));
		// This message was commented out, probably because it can
		// happen when shutting down.
		if (p_verbose > 0)
		    emsg(_(e_invalid_buffer_identifier_in_setvisible));
		return FAIL;
	    }
	    if (streq((char *)args, "T") && buf->bufp != curbuf)
	    {
		exarg_T exarg;
		exarg.cmd = (char_u *)"goto";
		exarg.forceit = FALSE;
		dosetvisible = TRUE;
		goto_buffer(&exarg, DOBUF_FIRST, FORWARD, buf->bufp->b_fnum);
		do_update = 1;
		dosetvisible = FALSE;

#ifdef FEAT_GUI
		// Side effect!!!.
		if (gui.in_use)
		    gui_mch_set_foreground();
#endif
	    }
// =====================================================================
	}
	else if (streq((char *)cmd, "raise"))
	{
#ifdef FEAT_GUI
	    // Bring gvim to the foreground.
	    if (gui.in_use)
		gui_mch_set_foreground();
#endif
// =====================================================================
	}
	else if (streq((char *)cmd, "setModified"))
	{
	    int prev_b_changed;

	    if (buf == NULL || buf->bufp == NULL)
	    {
		nbdebug(("    invalid buffer identifier in setModified\n"));
		// This message was commented out, probably because it can
		// happen when shutting down.
		if (p_verbose > 0)
		    emsg(_(e_invalid_buffer_identifier_in_setmodified));
		return FAIL;
	    }
	    prev_b_changed = buf->bufp->b_changed;
	    if (streq((char *)args, "T"))
		buf->bufp->b_changed = TRUE;
	    else
	    {
		stat_T	st;

		// Assume NetBeans stored the file.  Reset the timestamp to
		// avoid "file changed" warnings.
		if (buf->bufp->b_ffname != NULL
			&& mch_stat((char *)buf->bufp->b_ffname, &st) >= 0)
		    buf_store_time(buf->bufp, &st, buf->bufp->b_ffname);
		buf->bufp->b_changed = FALSE;
	    }
	    buf->modified = buf->bufp->b_changed;
	    if (prev_b_changed != buf->bufp->b_changed)
	    {
		check_status(buf->bufp);
		redraw_tabline = TRUE;
		maketitle();
		update_screen(0);
	    }
// =====================================================================
	}
	else if (streq((char *)cmd, "setModtime"))
	{
	    if (buf == NULL || buf->bufp == NULL)
		nbdebug(("    invalid buffer identifier in setModtime\n"));
	    else
	    {
		buf->bufp->b_mtime = atoi((char *)args);
		buf->bufp->b_mtime_ns = 0;
	    }
// =====================================================================
	}
	else if (streq((char *)cmd, "setReadOnly"))
	{
	    if (buf == NULL || buf->bufp == NULL)
		nbdebug(("    invalid buffer identifier in setReadOnly\n"));
	    else if (streq((char *)args, "T"))
		buf->bufp->b_p_ro = TRUE;
	    else
		buf->bufp->b_p_ro = FALSE;
// =====================================================================
	}
	else if (streq((char *)cmd, "setMark"))
	{
	    // not yet
// =====================================================================
	}
	else if (streq((char *)cmd, "showBalloon"))
	{
#if defined(FEAT_BEVAL_GUI)
	    static char	*text = NULL;

	    /*
	     * Set up the Balloon Expression Evaluation area.
	     * Ignore 'ballooneval' here.
	     * The text pointer must remain valid for a while.
	     */
	    if (balloonEval != NULL)
	    {
		vim_free(text);
		text = nb_unquote(args, NULL);
		if (text != NULL)
		    gui_mch_post_balloon(balloonEval, (char_u *)text);
	    }
#endif
// =====================================================================
	}
	else if (streq((char *)cmd, "setDot"))
	{
	    pos_T *pos;
#ifdef NBDEBUG
	    char_u *s;
#endif

	    if (buf == NULL || buf->bufp == NULL)
	    {
		nbdebug(("    invalid buffer identifier in setDot\n"));
		emsg(_(e_invalid_buffer_identifier_in_setdot));
		return FAIL;
	    }

	    nb_set_curbuf(buf->bufp);

	    // Don't want Visual mode now.
	    if (VIsual_active)
		end_visual_mode();
#ifdef NBDEBUG
	    s = args;
#endif
	    pos = get_off_or_lnum(buf->bufp, &args);
	    if (pos)
	    {
		curwin->w_cursor = *pos;
		check_cursor();
#ifdef FEAT_FOLDING
		foldOpenCursor();
#endif
	    }
	    else
	    {
		nbdebug(("    BAD POSITION in setDot: %s\n", s));
	    }

	    // gui_update_cursor(TRUE, FALSE);
	    // update_curbuf(UPD_NOT_VALID);
	    update_topline();		// scroll to show the line
	    update_screen(UPD_VALID);
	    setcursor();
	    cursor_on();
	    out_flush_cursor(TRUE, FALSE);

	    // Quit a hit-return or more prompt.
	    if (State == MODE_HITRETURN || State == MODE_ASKMORE)
	    {
#ifdef FEAT_GUI_GTK
		if (gui.in_use && gtk_main_level() > 0)
		    gtk_main_quit();
#endif
	    }
// =====================================================================
	}
	else if (streq((char *)cmd, "close"))
	{
#ifdef NBDEBUG
	    char *name = "<NONE>";
#endif

	    if (buf == NULL)
	    {
		nbdebug(("    invalid buffer identifier in close\n"));
		emsg(_(e_invalid_buffer_identifier_in_close));
		return FAIL;
	    }

#ifdef NBDEBUG
	    if (buf->displayname != NULL)
		name = buf->displayname;
#endif
	    if (buf->bufp == NULL)
	    {
		nbdebug(("    invalid buffer identifier in close\n"));
		// This message was commented out, probably because it can
		// happen when shutting down.
		if (p_verbose > 0)
		    emsg(_(e_invalid_buffer_identifier_in_close));
	    }
	    nbdebug(("    CLOSE %d: %s\n", bufno, name));
#ifdef FEAT_GUI
	    need_mouse_correct = TRUE;
#endif
	    if (buf->bufp != NULL)
		do_buffer(DOBUF_WIPE, DOBUF_FIRST, FORWARD,
						     buf->bufp->b_fnum, TRUE);
	    buf->bufp = NULL;
	    buf->initDone = FALSE;
	    do_update = 1;
// =====================================================================
	}
	else if (streq((char *)cmd, "setStyle")) // obsolete...
	{
	    nbdebug(("    setStyle is obsolete!\n"));
// =====================================================================
	}
	else if (streq((char *)cmd, "setExitDelay"))
	{
	    // Only used in version 2.1.
// =====================================================================
	}
	else if (streq((char *)cmd, "defineAnnoType"))
	{
#ifdef FEAT_SIGNS
	    int typeNum;
	    char_u *typeName;
	    char_u *tooltip;
	    char_u *p;
	    char_u *glyphFile;
	    int parse_error = FALSE;
	    char_u *fg;
	    char_u *bg;

	    if (buf == NULL)
	    {
		nbdebug(("    invalid buffer identifier in defineAnnoType\n"));
		emsg(_(e_invalid_buffer_identifier_in_defineannotype));
		return FAIL;
	    }

	    cp = (char *)args;
	    typeNum = strtol(cp, &cp, 10);
	    args = (char_u *)cp;
	    args = skipwhite(args);
	    typeName = (char_u *)nb_unquote(args, &args);
	    args = skipwhite(args + 1);
	    tooltip = (char_u *)nb_unquote(args, &args);
	    args = skipwhite(args + 1);

	    p = (char_u *)nb_unquote(args, &args);
	    glyphFile = vim_strsave_escaped(p, escape_chars);
	    vim_free(p);

	    args = skipwhite(args + 1);
	    p = skiptowhite(args);
	    if (*p != NUL)
	    {
		*p = NUL;
		p = skipwhite(p + 1);
	    }
	    fg = vim_strsave(args);
	    bg = vim_strsave(p);
	    if (STRLEN(fg) > MAX_COLOR_LENGTH || STRLEN(bg) > MAX_COLOR_LENGTH)
	    {
		emsg(_(e_highlighting_color_name_too_long_in_defineAnnoType));
		VIM_CLEAR(typeName);
		parse_error = TRUE;
	    }
	    else if (typeName != NULL && tooltip != NULL && glyphFile != NULL)
		addsigntype(buf, typeNum, typeName, tooltip, glyphFile, fg, bg);

	    vim_free(typeName);
	    vim_free(fg);
	    vim_free(bg);
	    vim_free(tooltip);
	    vim_free(glyphFile);
	    if (parse_error)
		return FAIL;

#endif
// =====================================================================
	}
	else if (streq((char *)cmd, "addAnno"))
	{
#ifdef FEAT_SIGNS
	    int serNum;
	    int localTypeNum;
	    int typeNum;
	    pos_T *pos;

	    if (buf == NULL || buf->bufp == NULL)
	    {
		nbdebug(("    invalid buffer identifier in addAnno\n"));
		emsg(_(e_invalid_buffer_identifier_in_addanno));
		return FAIL;
	    }

	    do_update = 1;

	    cp = (char *)args;
	    serNum = strtol(cp, &cp, 10);

	    // Get the typenr specific for this buffer and convert it to
	    // the global typenumber, as used for the sign name.
	    localTypeNum = strtol(cp, &cp, 10);
	    args = (char_u *)cp;
	    typeNum = mapsigntype(buf, localTypeNum);

	    pos = get_off_or_lnum(buf->bufp, &args);

	    cp = (char *)args;
	    vim_ignored = (int)strtol(cp, &cp, 10);
	    args = (char_u *)cp;
# ifdef NBDEBUG
	    if (vim_ignored != -1)
		nbdebug(("    partial line annotation -- Not Yet Implemented!\n"));
# endif
	    if (serNum >= GUARDEDOFFSET)
	    {
		nbdebug(("    too many annotations! ignoring...\n"));
		return FAIL;
	    }
	    if (pos)
	    {
		coloncmd(":sign place %d line=%ld name=%d buffer=%d",
			   serNum, pos->lnum, typeNum, buf->bufp->b_fnum);
		if (typeNum == curPCtype)
		    coloncmd(":sign jump %d buffer=%d", serNum,
						       buf->bufp->b_fnum);
	    }
#endif
// =====================================================================
	}
	else if (streq((char *)cmd, "removeAnno"))
	{
#ifdef FEAT_SIGNS
	    int serNum;

	    if (buf == NULL || buf->bufp == NULL)
	    {
		nbdebug(("    invalid buffer identifier in removeAnno\n"));
		return FAIL;
	    }
	    do_update = 1;
	    cp = (char *)args;
	    serNum = strtol(cp, &cp, 10);
	    args = (char_u *)cp;
	    coloncmd(":sign unplace %d buffer=%d",
		     serNum, buf->bufp->b_fnum);
	    redraw_buf_later(buf->bufp, UPD_NOT_VALID);
#endif
// =====================================================================
	}
	else if (streq((char *)cmd, "moveAnnoToFront"))
	{
#ifdef FEAT_SIGNS
	    nbdebug(("    moveAnnoToFront: Not Yet Implemented!\n"));
#endif
// =====================================================================
	}
	else if (streq((char *)cmd, "guard") || streq((char *)cmd, "unguard"))
	{
	    int len;
	    pos_T first;
	    pos_T last;
	    pos_T *pos;
	    int un = (cmd[0] == 'u');
	    static int guardId = GUARDEDOFFSET;

	    if (skip >= SKIP_STOP)
	    {
		nbdebug(("    Skipping %s command\n", (char *) cmd));
		return OK;
	    }

	    nb_init_graphics();

	    if (buf == NULL || buf->bufp == NULL)
	    {
		nbdebug(("    invalid buffer identifier in %s command\n", cmd));
		return FAIL;
	    }
	    nb_set_curbuf(buf->bufp);
	    cp = (char *)args;
	    off = strtol(cp, &cp, 10);
	    len = strtol(cp, NULL, 10);
	    args = (char_u *)cp;
	    pos = off2pos(buf->bufp, off);
	    do_update = 1;
	    if (!pos)
		nbdebug(("    no such start pos in %s, %ld\n", cmd, off));
	    else
	    {
		first = *pos;
		pos = off2pos(buf->bufp, off + len - 1);
		if (pos != NULL && pos->col == 0)
		{
			/*
			 * In Java Swing the offset is a position between 2
			 * characters. If col == 0 then we really want the
			 * previous line as the end.
			 */
			pos = off2pos(buf->bufp, off + len - 2);
		}
		if (!pos)
		    nbdebug(("    no such end pos in %s, %ld\n",
			    cmd, off + len - 1));
		else
		{
		    long lnum;
		    last = *pos;
		    // set highlight for region
		    nbdebug(("    %sGUARD %ld,%d to %ld,%d\n", (un) ? "UN" : "",
			     first.lnum, first.col,
			     last.lnum, last.col));
#ifdef FEAT_SIGNS
		    for (lnum = first.lnum; lnum <= last.lnum; lnum++)
		    {
			if (un)
			{
			    // never used
			}
			else
			{
			    if (buf_findsigntype_id(buf->bufp, lnum,
				GUARDED) == 0)
			    {
				coloncmd(
				    ":sign place %d line=%ld name=%d buffer=%d",
				     guardId++, lnum, GUARDED,
				     buf->bufp->b_fnum);
			    }
			}
		    }
#endif
		    redraw_buf_later(buf->bufp, UPD_NOT_VALID);
		}
	    }
// =====================================================================
	}
	else if (streq((char *)cmd, "startAtomic"))
	{
	    inAtomic = 1;
// =====================================================================
	}
	else if (streq((char *)cmd, "endAtomic"))
	{
	    inAtomic = 0;
	    if (needupdate)
	    {
		do_update = 1;
		needupdate = 0;
	    }
// =====================================================================
	}
	else if (streq((char *)cmd, "save"))
	{
	    /*
	     * NOTE - This command is obsolete wrt NetBeans. It's left in
	     * only for historical reasons.
	     */
	    if (buf == NULL || buf->bufp == NULL)
	    {
		nbdebug(("    invalid buffer identifier in %s command\n", cmd));
		return FAIL;
	    }

	    // the following is taken from ex_cmds.c (do_wqall function)
	    if (bufIsChanged(buf->bufp))
	    {
		// Only write if the buffer can be written.
		if (p_write
			&& !buf->bufp->b_p_ro
			&& buf->bufp->b_ffname != NULL
			&& !bt_dontwrite(buf->bufp))
		{
		    bufref_T bufref;

		    set_bufref(&bufref, buf->bufp);
		    buf_write_all(buf->bufp, FALSE);
		    // an autocommand may have deleted the buffer
		    if (!bufref_valid(&bufref))
			buf->bufp = NULL;
		}
	    }
	    else
	    {
		nbdebug(("    Buffer has no changes!\n"));
	    }
// =====================================================================
	}
	else if (streq((char *)cmd, "netbeansBuffer"))
	{
	    if (buf == NULL || buf->bufp == NULL)
	    {
		nbdebug(("    invalid buffer identifier in %s command\n", cmd));
		return FAIL;
	    }
	    if (*args == 'T')
	    {
		buf->bufp->b_netbeans_file = TRUE;
		buf->bufp->b_was_netbeans_file = TRUE;
	    }
	    else
		buf->bufp->b_netbeans_file = FALSE;
// =====================================================================
	}
	else if (streq((char *)cmd, "specialKeys"))
	{
	    special_keys(args);
// =====================================================================
	}
	else if (streq((char *)cmd, "actionMenuItem"))
	{
	    // not used yet
// =====================================================================
	}
	else if (streq((char *)cmd, "version"))
	{
	    // not used yet
	}
	else
	{
	    nbdebug(("Unrecognised command: %s\n", cmd));
	}
	/*
	 * Unrecognized command is ignored.
	 */
    }
    if (inAtomic && do_update)
    {
	needupdate = 1;
	do_update = 0;
    }

    /*
     * Is this needed? I moved the netbeans_Xt_connect() later during startup
     * and it may no longer be necessary. If it's not needed then needupdate
     * and do_update can also be removed.
     */
    if (buf != NULL && buf->initDone && do_update)
    {
	update_screen(UPD_NOT_VALID);
	setcursor();
	cursor_on();
	out_flush_cursor(TRUE, FALSE);

	// Quit a hit-return or more prompt.
	if (State == MODE_HITRETURN || State == MODE_ASKMORE)
	{
#ifdef FEAT_GUI_GTK
	    if (gui.in_use && gtk_main_level() > 0)
		gtk_main_quit();
#endif
	}
    }

    return retval;
}


/*
 * If "buf" is not the current buffer try changing to a window that edits this
 * buffer.  If there is no such window then close the current buffer and set
 * the current buffer as "buf".
 */
    static void
nb_set_curbuf(buf_T *buf)
{
    if (curbuf == buf)
	return;

    if (buf_jump_open_win(buf) != NULL)
	return;
    if ((swb_flags & SWB_USETAB) && buf_jump_open_tab(buf) != NULL)
	return;
    set_curbuf(buf, DOBUF_GOTO);
}

/*
 * Process a vim colon command.
 */
    static void
coloncmd(char *cmd, ...)
{
    char buf[1024];
    va_list ap;

    va_start(ap, cmd);
    vim_vsnprintf(buf, sizeof(buf), cmd, ap);
    va_end(ap);

    nbdebug(("    COLONCMD %s\n", buf));

    do_cmdline((char_u *)buf, NULL, NULL, DOCMD_NOWAIT | DOCMD_KEYTYPED);

    setcursor();		// restore the cursor position
    out_flush_cursor(TRUE, FALSE);
}


/*
 * Parse the specialKeys argument and issue the appropriate map commands.
 */
    static void
special_keys(char_u *args)
{
    char *save_str = nb_unquote(args, NULL);
    char *tok = strtok(save_str, " ");
    char *sep;
#define KEYBUFLEN 64
    char keybuf[KEYBUFLEN];
    char cmdbuf[256];

    while (tok != NULL)
    {
	int i = 0;

	if ((sep = strchr(tok, '-')) != NULL)
	{
	    *sep = NUL;
	    while (*tok)
	    {
		switch (*tok)
		{
		    case 'A':
		    case 'M':
		    case 'C':
		    case 'S':
			keybuf[i++] = *tok;
			keybuf[i++] = '-';
			break;
		}
		tok++;
	    }
	    tok++;
	}

	if (strlen(tok) + i < KEYBUFLEN)
	{
	    strcpy(&keybuf[i], tok);
	    vim_snprintf(cmdbuf, sizeof(cmdbuf),
				 "<silent><%s> :nbkey %s<CR>", keybuf, keybuf);
	    do_map(MAPTYPE_MAP, (char_u *)cmdbuf, MODE_NORMAL, FALSE);
	}
	tok = strtok(NULL, " ");
    }
    vim_free(save_str);
}

    void
ex_nbclose(exarg_T *eap UNUSED)
{
    netbeans_close();
}

    void
ex_nbkey(exarg_T *eap)
{
    (void)netbeans_keystring(eap->arg);
}

    void
ex_nbstart(
    exarg_T	*eap)
{
#ifdef FEAT_GUI
# if !defined(FEAT_GUI_X11) && !defined(FEAT_GUI_GTK)  \
		&& !defined(FEAT_GUI_MSWIN)
    if (gui.in_use)
    {
	emsg(_(e_netbeans_is_not_supported_with_this_GUI));
	return;
    }
# endif
#endif
    netbeans_open((char *)eap->arg, FALSE);
}

/*
 * Initialize highlights and signs for use by netbeans  (mostly obsolete)
 */
    static void
nb_init_graphics(void)
{
    static int did_init = FALSE;

    if (did_init)
	return;

    coloncmd(":highlight NBGuarded guibg=Cyan guifg=Black"
	    " ctermbg=LightCyan ctermfg=Black");
    coloncmd(":sign define %d linehl=NBGuarded", GUARDED);

    did_init = TRUE;
}

/*
 * Convert key to netbeans name.  This uses the global "mod_mask".
 */
    static void
netbeans_keyname(int key, char *buf)
{
    char *name = 0;
    char namebuf[2];
    int ctrl  = 0;
    int shift = 0;
    int alt   = 0;

    if (mod_mask & MOD_MASK_CTRL)
	ctrl = 1;
    if (mod_mask & MOD_MASK_SHIFT)
	shift = 1;
    if (mod_mask & MOD_MASK_ALT)
	alt = 1;


    switch (key)
    {
	case K_F1:		name = "F1";		break;
	case K_S_F1:	name = "F1";	shift = 1;	break;
	case K_F2:		name = "F2";		break;
	case K_S_F2:	name = "F2";	shift = 1;	break;
	case K_F3:		name = "F3";		break;
	case K_S_F3:	name = "F3";	shift = 1;	break;
	case K_F4:		name = "F4";		break;
	case K_S_F4:	name = "F4";	shift = 1;	break;
	case K_F5:		name = "F5";		break;
	case K_S_F5:	name = "F5";	shift = 1;	break;
	case K_F6:		name = "F6";		break;
	case K_S_F6:	name = "F6";	shift = 1;	break;
	case K_F7:		name = "F7";		break;
	case K_S_F7:	name = "F7";	shift = 1;	break;
	case K_F8:		name = "F8";		break;
	case K_S_F8:	name = "F8";	shift = 1;	break;
	case K_F9:		name = "F9";		break;
	case K_S_F9:	name = "F9";	shift = 1;	break;
	case K_F10:		name = "F10";		break;
	case K_S_F10:	name = "F10";	shift = 1;	break;
	case K_F11:		name = "F11";		break;
	case K_S_F11:	name = "F11";	shift = 1;	break;
	case K_F12:		name = "F12";		break;
	case K_S_F12:	name = "F12";	shift = 1;	break;
	default:
			if (key >= ' ' && key <= '~')
			{
			    // Allow ASCII characters.
			    name = namebuf;
			    namebuf[0] = key;
			    namebuf[1] = NUL;
			}
			else
			    name = "X";
			break;
    }

    buf[0] = '\0';
    if (ctrl)
	strcat(buf, "C");
    if (shift)
	strcat(buf, "S");
    if (alt)
	strcat(buf, "M"); // META
    if (ctrl || shift || alt)
	strcat(buf, "-");
    strcat(buf, name);
}

#if defined(FEAT_BEVAL) || defined(PROTO)
/*
 * Function to be called for balloon evaluation.  Grabs the text under the
 * cursor and sends it to the debugger for evaluation.  The debugger should
 * respond with a showBalloon command when there is a useful result.
 */
    void
netbeans_beval_cb(
	BalloonEval	*beval,
	int		 state UNUSED)
{
    win_T	*wp;
    char_u	*text;
    linenr_T	lnum;
    int		col;
    char	*buf;
    char_u	*p;

    // Don't do anything when 'ballooneval' is off, messages scrolled the
    // windows up or we have no connection.
    if (!can_use_beval() || !NETBEANS_OPEN)
	return;

    if (get_beval_info(beval, TRUE, &wp, &lnum, &text, &col) != OK)
	return;

    // Send debugger request.  Only when the text is of reasonable
    // length.
    if (text != NULL && text[0] != NUL && STRLEN(text) < MAXPATHL)
    {
	buf = alloc(MAXPATHL * 2 + 25);
	if (buf != NULL)
	{
	    p = nb_quote(text);
	    if (p != NULL)
	    {
		vim_snprintf(buf, MAXPATHL * 2 + 25,
			"0:balloonText=%d \"%s\"\n", r_cmdno, p);
		vim_free(p);
	    }
	    nbdebug(("EVT: %s", buf));
	    nb_send(buf, "netbeans_beval_cb");
	    vim_free(buf);
	}
    }
    vim_free(text);
}
#endif

/*
 * Return TRUE when the netbeans connection is active.
 */
    int
netbeans_active(void)
{
    return NETBEANS_OPEN;
}

/*
 * Tell netbeans that the window was opened, ready for commands.
 */
    void
netbeans_open(char *params, int doabort)
{
    char *cmd = "0:startupDone=0\n";

    if (NETBEANS_OPEN)
    {
	emsg(_(e_netbeans_already_connected));
	return;
    }

    if (netbeans_connect(params, doabort) != OK)
	return;

    nbdebug(("EVT: %s", cmd));
    nb_send(cmd, "netbeans_startup_done");

    // update the screen after having added the gutter
    changed_window_setting();
    update_screen(UPD_CLEAR);
    setcursor();
    cursor_on();
    out_flush_cursor(TRUE, FALSE);
}

/*
 * Tell netbeans that we're exiting. This should be called right
 * before calling exit.
 */
    void
netbeans_send_disconnect(void)
{
    char buf[128];

    if (NETBEANS_OPEN)
    {
	sprintf(buf, "0:disconnect=%d\n", r_cmdno);
	nbdebug(("EVT: %s", buf));
	nb_send(buf, "netbeans_disconnect");
    }
}

#if defined(FEAT_EVAL) || defined(PROTO)
    int
set_ref_in_nb_channel(int copyID)
{
    int abort = FALSE;
    typval_T tv;

    if (nb_channel == NULL)
	return FALSE;

    tv.v_type = VAR_CHANNEL;
    tv.vval.v_channel = nb_channel;
    abort = set_ref_in_item(&tv, copyID, NULL, NULL);
    return abort;
}
#endif

#if defined(FEAT_GUI_X11) || defined(FEAT_GUI_MSWIN) || defined(PROTO)
/*
 * Tell netbeans that the window was moved or resized.
 */
    void
netbeans_frame_moved(int new_x, int new_y)
{
    char buf[128];

    if (!NETBEANS_OPEN)
	return;

    sprintf(buf, "0:geometry=%d %d %d %d %d\n",
		    r_cmdno, (int)Columns, (int)Rows, new_x, new_y);
    // nbdebug(("EVT: %s", buf)); happens too many times during a move
    nb_send(buf, "netbeans_frame_moved");
}
#endif

/*
 * Tell netbeans the user opened or activated a file.
 */
    void
netbeans_file_activated(buf_T *bufp)
{
    int bufno = nb_getbufno(bufp);
    nbbuf_T *bp = nb_get_buf(bufno);
    char    buffer[2*MAXPATHL];
    char_u  *q;

    if (!NETBEANS_OPEN || !bufp->b_netbeans_file || dosetvisible)
	return;

    q = nb_quote(bufp->b_ffname);
    if (q == NULL || bp == NULL)
	return;

    vim_snprintf(buffer, sizeof(buffer),  "%d:fileOpened=%d \"%s\" %s %s\n",
	    bufno,
	    bufno,
	    (char *)q,
	    "T",  // open in NetBeans
	    "F"); // modified

    vim_free(q);
    nbdebug(("EVT: %s", buffer));

    nb_send(buffer, "netbeans_file_opened");
}

/*
 * Tell netbeans the user opened a file.
 */
    void
netbeans_file_opened(buf_T *bufp)
{
    int bufno = nb_getbufno(bufp);
    char    buffer[2*MAXPATHL];
    char_u  *q;
    nbbuf_T *bp = nb_get_buf(nb_getbufno(bufp));
    int	    bnum;

    if (!NETBEANS_OPEN)
	return;

    q = nb_quote(bufp->b_ffname);
    if (q == NULL)
	return;
    if (bp != NULL)
	bnum = bufno;
    else
	bnum = 0;

    vim_snprintf(buffer, sizeof(buffer), "%d:fileOpened=%d \"%s\" %s %s\n",
	    bnum,
	    0,
	    (char *)q,
	    "T",  // open in NetBeans
	    "F"); // modified

    vim_free(q);
    nbdebug(("EVT: %s", buffer));

    nb_send(buffer, "netbeans_file_opened");
    if (p_acd && vim_chdirfile(bufp->b_ffname, "auto") == OK)
    {
	last_chdir_reason = "netbeans";
	shorten_fnames(TRUE);
    }
}

/*
 * Tell netbeans that a file was deleted or wiped out.
 */
    void
netbeans_file_killed(buf_T *bufp)
{
    int		bufno = nb_getbufno(bufp);
    nbbuf_T	*nbbuf = nb_get_buf(bufno);
    char	buffer[2*MAXPATHL];

    if (!NETBEANS_OPEN || bufno == -1)
	return;

    nbdebug(("netbeans_file_killed:\n"));
    nbdebug(("    Killing bufno: %d", bufno));

    sprintf(buffer, "%d:killed=%d\n", bufno, r_cmdno);

    nbdebug(("EVT: %s", buffer));

    nb_send(buffer, "netbeans_file_killed");

    if (nbbuf != NULL)
	nbbuf->bufp = NULL;
}

/*
 * Get a pointer to the Netbeans buffer for Vim buffer "bufp".
 * Return NULL if there is no such buffer or changes are not to be reported.
 * Otherwise store the buffer number in "*bufnop".
 */
    static nbbuf_T *
nb_bufp2nbbuf_fire(buf_T *bufp, int *bufnop)
{
    int		bufno;
    nbbuf_T	*nbbuf;

    if (!NETBEANS_OPEN || !netbeansFireChanges)
	return NULL;		// changes are not reported at all

    bufno = nb_getbufno(bufp);
    if (bufno <= 0)
	return NULL;		// file is not known to NetBeans

    nbbuf = nb_get_buf(bufno);
    if (nbbuf != NULL && !nbbuf->fireChanges)
	return NULL;		// changes in this buffer are not reported

    *bufnop = bufno;
    return nbbuf;
}

/*
 * Tell netbeans the user inserted some text.
 */
    void
netbeans_inserted(
    buf_T	*bufp,
    linenr_T	linenr,
    colnr_T	col,
    char_u	*txt,
    int		newlen)
{
    char_u	*buf;
    int		bufno;
    nbbuf_T	*nbbuf;
    pos_T	pos;
    long	off;
    char_u	*p;
    char_u	*newtxt;

    if (!NETBEANS_OPEN)
	return;

    nbbuf = nb_bufp2nbbuf_fire(bufp, &bufno);
    if (nbbuf == NULL)
	return;

    // Don't mark as modified for initial read
    if (nbbuf->insertDone)
	nbbuf->modified = 1;

    // send the "insert" EVT
    newtxt = alloc(newlen + 1);
    vim_strncpy(newtxt, txt, newlen);

    // Note: this may make "txt" invalid
    pos.lnum = linenr;
    pos.col = col;
    off = pos2off(bufp, &pos);

    p = nb_quote(newtxt);
    if (p != NULL)
    {
	buf = alloc(128 + 2*newlen);
	sprintf((char *)buf, "%d:insert=%d %ld \"%s\"\n",
						      bufno, r_cmdno, off, p);
	nbdebug(("EVT: %s", buf));
	nb_send((char *)buf, "netbeans_inserted");
	vim_free(p);
	vim_free(buf);
    }
    vim_free(newtxt);
}

/*
 * Tell netbeans some bytes have been removed.
 */
    void
netbeans_removed(
    buf_T	*bufp,
    linenr_T	linenr,
    colnr_T	col,
    long	len)
{
    char_u	buf[128];
    int		bufno;
    nbbuf_T	*nbbuf;
    pos_T	pos;
    long	off;

    if (!NETBEANS_OPEN)
	return;

    nbbuf = nb_bufp2nbbuf_fire(bufp, &bufno);
    if (nbbuf == NULL)
	return;

    if (len < 0)
    {
	nbdebug(("Negative len %ld in netbeans_removed()!\n", len));
	return;
    }

    nbbuf->modified = 1;

    pos.lnum = linenr;
    pos.col = col;

    off = pos2off(bufp, &pos);

    sprintf((char *)buf, "%d:remove=%d %ld %ld\n", bufno, r_cmdno, off, len);
    nbdebug(("EVT: %s", buf));
    nb_send((char *)buf, "netbeans_removed");
}

/*
 * Send netbeans an unmodified command.
 */
    void
netbeans_unmodified(buf_T *bufp UNUSED)
{
    // This is a no-op, because NetBeans considers a buffer modified
    // even when all changes have been undone.
}

/*
 * Send a button release event back to netbeans. It's up to netbeans
 * to decide what to do (if anything) with this event.
 */
    void
netbeans_button_release(int button)
{
    char	buf[128];
    int		bufno;

    if (!NETBEANS_OPEN)
	return;

    bufno = nb_getbufno(curbuf);

    if (bufno < 0 || curwin == NULL || curwin->w_buffer != curbuf)
	return;

    int col = mouse_col - curwin->w_wincol
	- ((curwin->w_p_nu || curwin->w_p_rnu) ? 9 : 1);
    long off = pos2off(curbuf, &curwin->w_cursor);

    // sync the cursor position
    sprintf(buf, "%d:newDotAndMark=%d %ld %ld\n", bufno, r_cmdno, off, off);
    nbdebug(("EVT: %s", buf));
    nb_send(buf, "netbeans_button_release[newDotAndMark]");

    sprintf(buf, "%d:buttonRelease=%d %d %ld %d\n", bufno, r_cmdno,
	    button, (long)curwin->w_cursor.lnum, col);
    nbdebug(("EVT: %s", buf));
    nb_send(buf, "netbeans_button_release");
}


/*
 * Send a keypress event back to netbeans. This usually simulates some
 * kind of function key press. This function operates on a key code.
 * Return TRUE when the key was sent, FALSE when the command has been
 * postponed.
 */
    int
netbeans_keycommand(int key)
{
    char	keyName[60];

    netbeans_keyname(key, keyName);
    return netbeans_keystring((char_u *)keyName);
}


/*
 * Send a keypress event back to netbeans. This usually simulates some
 * kind of function key press. This function operates on a key string.
 * Return TRUE when the key was sent, FALSE when the command has been
 * postponed.
 */
    static int
netbeans_keystring(char_u *keyName)
{
    char	buf[2*MAXPATHL];
    int		bufno = nb_getbufno(curbuf);
    long	off;
    char_u	*q;

    if (!NETBEANS_OPEN)
	return TRUE;

    if (bufno == -1)
    {
	nbdebug(("got keycommand for non-NetBeans buffer, opening...\n"));
	q = curbuf->b_ffname == NULL ? (char_u *)""
						 : nb_quote(curbuf->b_ffname);
	if (q == NULL)
	    return TRUE;
	vim_snprintf(buf, sizeof(buf), "0:fileOpened=%d \"%s\" %s %s\n", 0,
		q,
		"T",  // open in NetBeans
		"F"); // modified
	if (curbuf->b_ffname != NULL)
	    vim_free(q);
	nbdebug(("EVT: %s", buf));
	nb_send(buf, "netbeans_keycommand");

	postpone_keycommand(keyName);
	return FALSE;
    }

    // sync the cursor position
    off = pos2off(curbuf, &curwin->w_cursor);
    sprintf(buf, "%d:newDotAndMark=%d %ld %ld\n", bufno, r_cmdno, off, off);
    nbdebug(("EVT: %s", buf));
    nb_send(buf, "netbeans_keycommand");

    // To work on Win32 you must apply patch to ExtEditor module
    // from ExtEdCaret.java.diff - make EVT_newDotAndMark handler
    // more synchronous

    // now send keyCommand event
    vim_snprintf(buf, sizeof(buf), "%d:keyCommand=%d \"%s\"\n",
						     bufno, r_cmdno, keyName);
    nbdebug(("EVT: %s", buf));
    nb_send(buf, "netbeans_keycommand");

    // New: do both at once and include the lnum/col.
    vim_snprintf(buf, sizeof(buf), "%d:keyAtPos=%d \"%s\" %ld %ld/%ld\n",
	    bufno, r_cmdno, keyName,
		off, (long)curwin->w_cursor.lnum, (long)curwin->w_cursor.col);
    nbdebug(("EVT: %s", buf));
    nb_send(buf, "netbeans_keycommand");
    return TRUE;
}


/*
 * Send a save event to netbeans.
 */
    void
netbeans_save_buffer(buf_T *bufp)
{
    char_u	buf[64];
    int		bufno;
    nbbuf_T	*nbbuf;

    if (!NETBEANS_OPEN)
	return;

    nbbuf = nb_bufp2nbbuf_fire(bufp, &bufno);
    if (nbbuf == NULL)
	return;

    nbbuf->modified = 0;

    sprintf((char *)buf, "%d:save=%d\n", bufno, r_cmdno);
    nbdebug(("EVT: %s", buf));
    nb_send((char *)buf, "netbeans_save_buffer");
}


/*
 * Send remove command to netbeans (this command has been turned off).
 */
    void
netbeans_deleted_all_lines(buf_T *bufp)
{
    char_u	buf[64];
    int		bufno;
    nbbuf_T	*nbbuf;

    if (!NETBEANS_OPEN)
	return;

    nbbuf = nb_bufp2nbbuf_fire(bufp, &bufno);
    if (nbbuf == NULL)
	return;

    // Don't mark as modified for initial read
    if (nbbuf->insertDone)
	nbbuf->modified = 1;

    sprintf((char *)buf, "%d:remove=%d 0 -1\n", bufno, r_cmdno);
    nbdebug(("EVT(suppressed): %s", buf));
//     nb_send(buf, "netbeans_deleted_all_lines");
}


/*
 * See if the lines are guarded. The top and bot parameters are from
 * u_savecommon(), these are the line above the change and the line below the
 * change.
 */
    int
netbeans_is_guarded(linenr_T top, linenr_T bot)
{
    sign_entry_T	*p;
    int			lnum;

    if (!NETBEANS_OPEN)
	return FALSE;

    FOR_ALL_SIGNS_IN_BUF(curbuf, p)
	if (p->se_id >= GUARDEDOFFSET)
	    for (lnum = top + 1; lnum < bot; lnum++)
		if (lnum == p->se_lnum)
		    return TRUE;

    return FALSE;
}

#if defined(FEAT_GUI_X11) || defined(PROTO)
/*
 * We have multiple signs to draw at the same location. Draw the
 * multi-sign indicator instead. This is the Motif version.
 */
    void
netbeans_draw_multisign_indicator(int row)
{
    int i;
    int y;
    int x;

    if (!NETBEANS_OPEN)
	return;

    x = 0;
    y = row * gui.char_height + 2;

    for (i = 0; i < gui.char_height - 3; i++)
	XDrawPoint(gui.dpy, gui.wid, gui.text_gc, x+2, y++);

    XDrawPoint(gui.dpy, gui.wid, gui.text_gc, x+0, y);
    XDrawPoint(gui.dpy, gui.wid, gui.text_gc, x+2, y);
    XDrawPoint(gui.dpy, gui.wid, gui.text_gc, x+4, y++);
    XDrawPoint(gui.dpy, gui.wid, gui.text_gc, x+1, y);
    XDrawPoint(gui.dpy, gui.wid, gui.text_gc, x+2, y);
    XDrawPoint(gui.dpy, gui.wid, gui.text_gc, x+3, y++);
    XDrawPoint(gui.dpy, gui.wid, gui.text_gc, x+2, y);
}
#endif // FEAT_GUI_X11

#if defined(FEAT_GUI_GTK) && !defined(PROTO)
/*
 * We have multiple signs to draw at the same location. Draw the
 * multi-sign indicator instead. This is the GTK/Gnome version.
 */
    void
netbeans_draw_multisign_indicator(int row)
{
    int i;
    int y;
    int x;
#if GTK_CHECK_VERSION(3,0,0)
    cairo_t *cr = NULL;
#else
    GdkDrawable *drawable = gui.drawarea->window;
#endif

    if (!NETBEANS_OPEN)
	return;

#if GTK_CHECK_VERSION(3,0,0)
    cr = cairo_create(gui.surface);
    cairo_set_source_rgba(cr,
	    gui.fgcolor->red, gui.fgcolor->green, gui.fgcolor->blue,
	    gui.fgcolor->alpha);
#endif

    x = 0;
    y = row * gui.char_height + 2;

    for (i = 0; i < gui.char_height - 3; i++)
#if GTK_CHECK_VERSION(3,0,0)
	cairo_rectangle(cr, x+2, y++, 1, 1);
#else
	gdk_draw_point(drawable, gui.text_gc, x+2, y++);
#endif

#if GTK_CHECK_VERSION(3,0,0)
    cairo_rectangle(cr, x+0, y, 1, 1);
    cairo_rectangle(cr, x+2, y, 1, 1);
    cairo_rectangle(cr, x+4, y++, 1, 1);
    cairo_rectangle(cr, x+1, y, 1, 1);
    cairo_rectangle(cr, x+2, y, 1, 1);
    cairo_rectangle(cr, x+3, y++, 1, 1);
    cairo_rectangle(cr, x+2, y, 1, 1);
#else
    gdk_draw_point(drawable, gui.text_gc, x+0, y);
    gdk_draw_point(drawable, gui.text_gc, x+2, y);
    gdk_draw_point(drawable, gui.text_gc, x+4, y++);
    gdk_draw_point(drawable, gui.text_gc, x+1, y);
    gdk_draw_point(drawable, gui.text_gc, x+2, y);
    gdk_draw_point(drawable, gui.text_gc, x+3, y++);
    gdk_draw_point(drawable, gui.text_gc, x+2, y);
#endif

#if GTK_CHECK_VERSION(3,0,0)
    cairo_destroy(cr);
#endif
}
#endif // FEAT_GUI_GTK

/*
 * If the mouse is clicked in the gutter of a line with multiple
 * annotations, cycle through the set of signs.
 */
    void
netbeans_gutter_click(linenr_T lnum)
{
    sign_entry_T	*p;

    if (!NETBEANS_OPEN)
	return;

    FOR_ALL_SIGNS_IN_BUF(curbuf, p)
    {
	if (p->se_lnum == lnum && p->se_next && p->se_next->se_lnum == lnum)
	{
	    sign_entry_T *tail;

	    // remove "p" from list, reinsert it at the tail of the sublist
	    if (p->se_prev)
		p->se_prev->se_next = p->se_next;
	    else
		curbuf->b_signlist = p->se_next;
	    p->se_next->se_prev = p->se_prev;
	    // now find end of sublist and insert p
	    for (tail = p->se_next;
		  tail->se_next && tail->se_next->se_lnum == lnum
				       && tail->se_next->se_id < GUARDEDOFFSET;
		  tail = tail->se_next)
		;
	    // tail now points to last entry with same lnum (except
	    // that "guarded" annotations are always last)
	    p->se_next = tail->se_next;
	    if (tail->se_next)
		tail->se_next->se_prev = p;
	    p->se_prev = tail;
	    tail->se_next = p;
	    update_debug_sign(curbuf, lnum);
	    break;
	}
    }
}

/*
 * Add a sign of the requested type at the requested location.
 *
 * Reverse engineering:
 * Apparently an annotation is defined the first time it is used in a buffer.
 * When the same annotation is used in two buffers, the second time we do not
 * need to define a new sign name but reuse the existing one.  But since the
 * ID number used in the second buffer starts counting at one again, a mapping
 * is made from the ID specifically for the buffer to the global sign name
 * (which is a number).
 *
 * globalsignmap[]	stores the signs that have been defined globally.
 * buf->signmapused[]	maps buffer-local annotation IDs to an index in
 *			globalsignmap[].
 */
    static void
addsigntype(
    nbbuf_T	*buf,
    int		typeNum,
    char_u	*typeName,
    char_u	*tooltip UNUSED,
    char_u	*glyphFile,
    char_u	*fg,
    char_u	*bg)
{
    int i, j;
    int use_fg = (*fg && STRCMP(fg, "none") != 0);
    int use_bg = (*bg && STRCMP(bg, "none") != 0);

    for (i = 0; i < globalsignmapused; i++)
	if (STRCMP(typeName, globalsignmap[i]) == 0)
	    break;

    if (i == globalsignmapused) // not found; add it to global map
    {
	nbdebug(("DEFINEANNOTYPE(%d,%s,%s,%s,%s,%s)\n",
			    typeNum, typeName, tooltip, glyphFile, fg, bg));
	if (use_fg || use_bg)
	{
	    char fgbuf[2 * (8 + MAX_COLOR_LENGTH) + 1];
	    char bgbuf[2 * (8 + MAX_COLOR_LENGTH) + 1];
	    char *ptr;
	    int value;

	    value = strtol((char *)fg, &ptr, 10);
	    if (ptr != (char *)fg)
		sprintf(fgbuf, "guifg=#%06x", value & 0xFFFFFF);
	    else
		sprintf(fgbuf, "guifg=%s ctermfg=%s", fg, fg);

	    value = strtol((char *)bg, &ptr, 10);
	    if (ptr != (char *)bg)
		sprintf(bgbuf, "guibg=#%06x", value & 0xFFFFFF);
	    else
		sprintf(bgbuf, "guibg=%s ctermbg=%s", bg, bg);

	    coloncmd(":highlight NB_%s %s %s", typeName, (use_fg) ? fgbuf : "",
		     (use_bg) ? bgbuf : "");
	    if (*glyphFile == NUL)
		// no glyph, line highlighting only
		coloncmd(":sign define %d linehl=NB_%s", i + 1, typeName);
	    else if (vim_strsize(glyphFile) <= 2)
		// one- or two-character glyph name, use as text glyph with
		// texthl
		coloncmd(":sign define %d text=%s texthl=NB_%s", i + 1,
							 glyphFile, typeName);
	    else
		// glyph, line highlighting
		coloncmd(":sign define %d icon=%s linehl=NB_%s", i + 1,
							 glyphFile, typeName);
	}
	else
	    // glyph, no line highlighting
	    coloncmd(":sign define %d icon=%s", i + 1, glyphFile);

	if (STRCMP(typeName,"CurrentPC") == 0)
	    curPCtype = typeNum;

	if (globalsignmapused == globalsignmaplen)
	{
	    if (globalsignmaplen == 0) // first allocation
	    {
		globalsignmaplen = 20;
		globalsignmap = ALLOC_CLEAR_MULT(char *, globalsignmaplen);
	    }
	    else    // grow it
	    {
		int incr;
		int oldlen = globalsignmaplen;
		char **t_globalsignmap = globalsignmap;

		globalsignmaplen *= 2;
		incr = globalsignmaplen - oldlen;
		globalsignmap = vim_realloc(globalsignmap,
					   globalsignmaplen * sizeof(char *));
		if (globalsignmap == NULL)
		{
		    vim_free(t_globalsignmap);
		    globalsignmaplen = 0;
		    return;
		}
		vim_memset(globalsignmap + oldlen, 0, incr * sizeof(char *));
	    }
	}

	globalsignmap[i] = (char *)vim_strsave(typeName);
	globalsignmapused = i + 1;
    }

    // check local map; should *not* be found!
    for (j = 0; j < buf->signmapused; j++)
	if (buf->signmap[j] == i + 1)
	    return;

    // add to local map
    if (buf->signmapused == buf->signmaplen)
    {
	if (buf->signmaplen == 0) // first allocation
	{
	    buf->signmaplen = 5;
	    buf->signmap = ALLOC_CLEAR_MULT(int, buf->signmaplen);
	}
	else    // grow it
	{
	    int incr;
	    int oldlen = buf->signmaplen;
	    int *t_signmap = buf->signmap;

	    buf->signmaplen *= 2;
	    incr = buf->signmaplen - oldlen;
	    buf->signmap = vim_realloc(buf->signmap,
					       buf->signmaplen * sizeof(int));
	    if (buf->signmap == NULL)
	    {
		vim_free(t_signmap);
		buf->signmaplen = 0;
		return;
	    }
	    vim_memset(buf->signmap + oldlen, 0, incr * sizeof(int));
	}
    }

    buf->signmap[buf->signmapused++] = i + 1;

}


/*
 * See if we have the requested sign type in the buffer.
 */
    static int
mapsigntype(nbbuf_T *buf, int localsigntype)
{
    if (--localsigntype >= 0 && localsigntype < buf->signmapused)
	return buf->signmap[localsigntype];

    return 0;
}


/*
 * Compute length of buffer, don't print anything.
 */
    static long
get_buf_size(buf_T *bufp)
{
    linenr_T	lnum;
    long	char_count = 0;
    int		eol_size;
    long	last_check = 100000L;

    if (bufp->b_ml.ml_flags & ML_EMPTY)
	return 0;

    if (get_fileformat(bufp) == EOL_DOS)
	eol_size = 2;
    else
	eol_size = 1;
    for (lnum = 1; lnum <= bufp->b_ml.ml_line_count; ++lnum)
    {
	char_count += (long)STRLEN(ml_get_buf(bufp, lnum, FALSE))
	    + eol_size;
	// Check for a CTRL-C every 100000 characters
	if (char_count > last_check)
	{
	    ui_breakcheck();
	    if (got_int)
		return char_count;
	    last_check = char_count + 100000L;
	}
    }
    // Correction for when last line doesn't have an EOL.
    if (!bufp->b_p_eol && (bufp->b_p_bin || !bufp->b_p_fixeol))
	char_count -= eol_size;

    return char_count;
}

/*
 * Convert character offset to lnum,col
 */
    static pos_T *
off2pos(buf_T *buf, long offset)
{
    linenr_T	 lnum;
    static pos_T pos;

    pos.lnum = 0;
    pos.col = 0;
    pos.coladd = 0;

    if (!(buf->b_ml.ml_flags & ML_EMPTY))
    {
	if ((lnum = ml_find_line_or_offset(buf, (linenr_T)0, &offset)) < 0)
	    return NULL;
	pos.lnum = lnum;
	pos.col = offset;
    }

    return &pos;
}

/*
 * Convert an argument in the form "1234" to an offset and compute the
 * lnum/col from it.  Convert an argument in the form "123/12" directly to a
 * lnum/col.
 * "argp" is advanced to after the argument.
 * Return a pointer to the position, NULL if something is wrong.
 */
    static pos_T *
get_off_or_lnum(buf_T *buf, char_u **argp)
{
    static pos_T	mypos;
    long		off;

    off = strtol((char *)*argp, (char **)argp, 10);
    if (**argp == '/')
    {
	mypos.lnum = (linenr_T)off;
	++*argp;
	mypos.col = strtol((char *)*argp, (char **)argp, 10);
	mypos.coladd = 0;
	return &mypos;
    }
    return off2pos(buf, off);
}


/*
 * Convert (lnum,col) to byte offset in the file.
 */
    static long
pos2off(buf_T *buf, pos_T *pos)
{
    long	 offset = 0;

    if (buf->b_ml.ml_flags & ML_EMPTY)
	return 0;

    if ((offset = ml_find_line_or_offset(buf, pos->lnum, 0)) < 0)
	return 0;
    offset += pos->col;

    return offset;
}


/*
 * This message is printed after NetBeans opens a new file. It's
 * similar to the message readfile() uses, but since NetBeans
 * doesn't normally call readfile, we do our own.
 */
    static void
print_read_msg(nbbuf_T *buf)
{
    int	    lnum = buf->bufp->b_ml.ml_line_count;
    off_T   nchars = buf->bufp->b_orig_size;
    char_u  c;

    msg_add_fname(buf->bufp, buf->bufp->b_ffname);
    c = FALSE;

    if (buf->bufp->b_p_ro)
    {
	STRCAT(IObuff, shortmess(SHM_RO) ? _("[RO]") : _("[readonly]"));
	c = TRUE;
    }
    if (!buf->bufp->b_start_eol)
    {
	STRCAT(IObuff, shortmess(SHM_LAST) ? _("[noeol]")
					       : _("[Incomplete last line]"));
	c = TRUE;
    }
    msg_add_lines(c, (long)lnum, nchars);

    // Now display it
    VIM_CLEAR(keep_msg);
    msg_scrolled_ign = TRUE;
    msg_trunc_attr((char *)IObuff, FALSE, 0);
    msg_scrolled_ign = FALSE;
}


/*
 * Print a message after NetBeans writes the file. This message should be
 * identical to the standard message a non-netbeans user would see when
 * writing a file.
 */
    static void
print_save_msg(nbbuf_T *buf, off_T nchars)
{
    char_u	c;
    char_u	*p;

    if (nchars >= 0)
    {
	// put fname in IObuff with quotes
	msg_add_fname(buf->bufp, buf->bufp->b_ffname);
	c = FALSE;

	msg_add_lines(c, buf->bufp->b_ml.ml_line_count,
						buf->bufp->b_orig_size);

	VIM_CLEAR(keep_msg);
	msg_scrolled_ign = TRUE;
	p = (char_u *)msg_trunc_attr((char *)IObuff, FALSE, 0);
	if ((msg_scrolled && !need_wait_return) || !buf->initDone)
	{
	    // Need to repeat the message after redrawing when:
	    // - When reading from stdin (the screen will be cleared next).
	    // - When restart_edit is set (otherwise there will be a delay
	    //   before redrawing).
	    // - When the screen was scrolled but there is no wait-return
	    //   prompt.
	    set_keep_msg(p, 0);
	}
	msg_scrolled_ign = FALSE;
	// add_to_input_buf((char_u *)"\f", 1);
    }
    else
    {
	char msgbuf[IOSIZE];

	vim_snprintf(msgbuf, IOSIZE,
		       _(e_is_read_only_add_bang_to_override), IObuff);
	nbdebug(("    %s\n", msgbuf));
	emsg(msgbuf);
    }
}

#endif // defined(FEAT_NETBEANS_INTG)
