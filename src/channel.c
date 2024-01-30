/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 */

/*
 * Implements communication through a socket or any file handle.
 */

#include "vim.h"

#if defined(FEAT_JOB_CHANNEL) || defined(PROTO)

// TRUE when netbeans is running with a GUI.
#ifdef FEAT_GUI
# define CH_HAS_GUI (gui.in_use || gui.starting)
#endif

// Note: when making changes here also adjust configure.ac.
#ifdef MSWIN
// WinSock API is separated from C API, thus we can't use read(), write(),
// errno...
# define SOCK_ERRNO errno = WSAGetLastError()
# undef ECONNREFUSED
# define ECONNREFUSED WSAECONNREFUSED
# undef EWOULDBLOCK
# define EWOULDBLOCK WSAEWOULDBLOCK
# undef EINPROGRESS
# define EINPROGRESS WSAEINPROGRESS
# ifdef EINTR
#  undef EINTR
# endif
# define EINTR WSAEINTR
# define sock_write(sd, buf, len) send((SOCKET)sd, buf, len, 0)
# define sock_read(sd, buf, len) recv((SOCKET)sd, buf, len, 0)
# define sock_close(sd) closesocket((SOCKET)sd)
// Support for Unix-domain sockets was added in Windows SDK 17061.
# define UNIX_PATH_MAX 108
typedef struct sockaddr_un {
    ADDRESS_FAMILY sun_family;
    char sun_path[UNIX_PATH_MAX];
} SOCKADDR_UN, *PSOCKADDR_UN;
#else
# include <netdb.h>
# include <netinet/in.h>
# include <arpa/inet.h>
# include <sys/socket.h>
# include <sys/un.h>
# ifdef HAVE_LIBGEN_H
#  include <libgen.h>
# endif
# define SOCK_ERRNO
# define sock_write(sd, buf, len) write(sd, buf, len)
# define sock_read(sd, buf, len) read(sd, buf, len)
# define sock_close(sd) close(sd)
# define fd_read(fd, buf, len) read(fd, buf, len)
# define fd_write(sd, buf, len) write(sd, buf, len)
# define fd_close(sd) close(sd)
#endif

static void channel_read(channel_T *channel, ch_part_T part, char *func);
static ch_mode_T channel_get_mode(channel_T *channel, ch_part_T part);
static int channel_get_timeout(channel_T *channel, ch_part_T part);
static ch_part_T channel_part_send(channel_T *channel);
static ch_part_T channel_part_read(channel_T *channel);

#define FOR_ALL_CHANNELS(ch) \
    for ((ch) = first_channel; (ch) != NULL; (ch) = (ch)->ch_next)

// Whether we are inside channel_parse_messages() or another situation where it
// is safe to invoke callbacks.
static int safe_to_invoke_callback = 0;

#ifdef MSWIN
    static int
fd_read(sock_T fd, char *buf, size_t len)
{
    HANDLE h = (HANDLE)fd;
    DWORD nread;

    if (!ReadFile(h, buf, (DWORD)len, &nread, NULL))
	return -1;
    return (int)nread;
}

    static int
fd_write(sock_T fd, char *buf, size_t len)
{
    size_t	todo = len;
    HANDLE	h = (HANDLE)fd;
    DWORD	nwrite, size, done = 0;
    OVERLAPPED	ov;

    while (todo > 0)
    {
	if (todo > MAX_NAMED_PIPE_SIZE)
	    size = MAX_NAMED_PIPE_SIZE;
	else
	    size = (DWORD)todo;
	// If the pipe overflows while the job does not read the data,
	// WriteFile() will block forever. This abandons the write.
	CLEAR_FIELD(ov);
	nwrite = 0;
	if (!WriteFile(h, buf + done, size, &nwrite, &ov))
	{
	    DWORD err = GetLastError();

	    if (err != ERROR_IO_PENDING)
		return -1;
	    if (!GetOverlappedResult(h, &ov, &nwrite, FALSE))
		return -1;
	    FlushFileBuffers(h);
	}
	else if (nwrite == 0)
	    // WriteFile() returns TRUE but did not write anything. This causes
	    // a hang, so bail out.
	    break;
	todo -= nwrite;
	done += nwrite;
    }
    return (int)done;
}

    static void
fd_close(sock_T fd)
{
    HANDLE h = (HANDLE)fd;

    CloseHandle(h);
}
#endif

#ifdef MSWIN
# undef PERROR
# define PERROR(msg) (void)semsg("%s: %s", msg, strerror_win32(errno))

    static char *
strerror_win32(int eno)
{
    static LPVOID msgbuf = NULL;
    char_u *ptr;

    if (msgbuf)
    {
	LocalFree(msgbuf);
	msgbuf = NULL;
    }
    FormatMessage(
	FORMAT_MESSAGE_ALLOCATE_BUFFER |
	FORMAT_MESSAGE_FROM_SYSTEM |
	FORMAT_MESSAGE_IGNORE_INSERTS,
	NULL,
	eno,
	MAKELANGID(LANG_ENGLISH, SUBLANG_DEFAULT),
	(LPTSTR) &msgbuf,
	0,
	NULL);
    if (msgbuf != NULL)
	// chomp \r or \n
	for (ptr = (char_u *)msgbuf; *ptr; ptr++)
	    switch (*ptr)
	    {
		case '\r':
		    STRMOVE(ptr, ptr + 1);
		    ptr--;
		    break;
		case '\n':
		    if (*(ptr + 1) == '\0')
			*ptr = '\0';
		    else
			*ptr = ' ';
		    break;
	    }
    return msgbuf;
}
#endif

/*
 * The list of all allocated channels.
 */
static channel_T *first_channel = NULL;
static int next_ch_id = 0;

/*
 * Allocate a new channel.  The refcount is set to 1.
 * The channel isn't actually used until it is opened.
 * Returns NULL if out of memory.
 */
    channel_T *
add_channel(void)
{
    ch_part_T	part;
    channel_T	*channel = ALLOC_CLEAR_ONE(channel_T);

    if (channel == NULL)
	return NULL;

    channel->ch_id = next_ch_id++;
    ch_log(channel, "Created channel");

    for (part = PART_SOCK; part < PART_COUNT; ++part)
    {
	channel->ch_part[part].ch_fd = INVALID_FD;
#ifdef FEAT_GUI_X11
	channel->ch_part[part].ch_inputHandler = (XtInputId)NULL;
#endif
#ifdef FEAT_GUI_GTK
	channel->ch_part[part].ch_inputHandler = 0;
#endif
	channel->ch_part[part].ch_timeout = 2000;
    }

    if (first_channel != NULL)
    {
	first_channel->ch_prev = channel;
	channel->ch_next = first_channel;
    }
    first_channel = channel;

    channel->ch_refcount = 1;
    return channel;
}

    int
has_any_channel(void)
{
    return first_channel != NULL;
}

/*
 * Called when the refcount of a channel is zero.
 * Return TRUE if "channel" has a callback and the associated job wasn't
 * killed.
 */
    int
channel_still_useful(channel_T *channel)
{
    int has_sock_msg;
    int	has_out_msg;
    int	has_err_msg;

    // If the job was killed the channel is not expected to work anymore.
    if (channel->ch_job_killed && channel->ch_job == NULL)
	return FALSE;

    // If there is a close callback it may still need to be invoked.
    if (channel->ch_close_cb.cb_name != NULL)
	return TRUE;

    // If reading from or a buffer it's still useful.
    if (channel->ch_part[PART_IN].ch_bufref.br_buf != NULL)
	return TRUE;

    // If there is no callback then nobody can get readahead.  If the fd is
    // closed and there is no readahead then the callback won't be called.
    has_sock_msg = channel->ch_part[PART_SOCK].ch_fd != INVALID_FD
		|| channel->ch_part[PART_SOCK].ch_head.rq_next != NULL
		|| channel->ch_part[PART_SOCK].ch_json_head.jq_next != NULL;
    has_out_msg = channel->ch_part[PART_OUT].ch_fd != INVALID_FD
		  || channel->ch_part[PART_OUT].ch_head.rq_next != NULL
		  || channel->ch_part[PART_OUT].ch_json_head.jq_next != NULL;
    has_err_msg = channel->ch_part[PART_ERR].ch_fd != INVALID_FD
		  || channel->ch_part[PART_ERR].ch_head.rq_next != NULL
		  || channel->ch_part[PART_ERR].ch_json_head.jq_next != NULL;
    return (channel->ch_callback.cb_name != NULL && (has_sock_msg
		|| has_out_msg || has_err_msg))
	    || ((channel->ch_part[PART_OUT].ch_callback.cb_name != NULL
		       || channel->ch_part[PART_OUT].ch_bufref.br_buf != NULL)
		    && has_out_msg)
	    || ((channel->ch_part[PART_ERR].ch_callback.cb_name != NULL
		       || channel->ch_part[PART_ERR].ch_bufref.br_buf != NULL)
		    && has_err_msg);
}

/*
 * Return TRUE if "channel" is closeable (i.e. all readable fds are closed).
 */
    int
channel_can_close(channel_T *channel)
{
    return channel->ch_to_be_closed == 0;
}

/*
 * Close a channel and free all its resources.
 * The "channel" pointer remains valid.
 */
    static void
channel_free_contents(channel_T *channel)
{
    channel_close(channel, TRUE);
    channel_clear(channel);
    ch_log(channel, "Freeing channel");
}

/*
 * Unlink "channel" from the list of channels and free it.
 */
    static void
channel_free_channel(channel_T *channel)
{
    if (channel->ch_next != NULL)
	channel->ch_next->ch_prev = channel->ch_prev;
    if (channel->ch_prev == NULL)
	first_channel = channel->ch_next;
    else
	channel->ch_prev->ch_next = channel->ch_next;
    vim_free(channel);
}

    static void
channel_free(channel_T *channel)
{
    if (in_free_unref_items)
	return;

    if (safe_to_invoke_callback == 0)
	channel->ch_to_be_freed = TRUE;
    else
    {
	channel_free_contents(channel);
	channel_free_channel(channel);
    }
}

/*
 * Close a channel and free all its resources if there is no further action
 * possible, there is no callback to be invoked or the associated job was
 * killed.
 * Return TRUE if the channel was freed.
 */
    static int
channel_may_free(channel_T *channel)
{
    if (!channel_still_useful(channel))
    {
	channel_free(channel);
	return TRUE;
    }
    return FALSE;
}

/*
 * Decrement the reference count on "channel" and maybe free it when it goes
 * down to zero.  Don't free it if there is a pending action.
 * Returns TRUE when the channel is no longer referenced.
 */
    int
channel_unref(channel_T *channel)
{
    if (channel != NULL && --channel->ch_refcount <= 0)
	return channel_may_free(channel);
    return FALSE;
}

    int
free_unused_channels_contents(int copyID, int mask)
{
    int		did_free = FALSE;
    channel_T	*ch;

    // This is invoked from the garbage collector, which only runs at a safe
    // point.
    ++safe_to_invoke_callback;

    FOR_ALL_CHANNELS(ch)
	if (!channel_still_useful(ch)
				 && (ch->ch_copyID & mask) != (copyID & mask))
	{
	    // Free the channel and ordinary items it contains, but don't
	    // recurse into Lists, Dictionaries etc.
	    channel_free_contents(ch);
	    did_free = TRUE;
	}

    --safe_to_invoke_callback;
    return did_free;
}

    void
free_unused_channels(int copyID, int mask)
{
    channel_T	*ch;
    channel_T	*ch_next;

    for (ch = first_channel; ch != NULL; ch = ch_next)
    {
	ch_next = ch->ch_next;
	if (!channel_still_useful(ch)
				 && (ch->ch_copyID & mask) != (copyID & mask))
	    // Free the channel struct itself.
	    channel_free_channel(ch);
    }
}

#if defined(FEAT_GUI) || defined(PROTO)

# if defined(FEAT_GUI_X11) || defined(FEAT_GUI_GTK)
/*
 * Lookup the channel from the socket.  Set "partp" to the fd index.
 * Returns NULL when the socket isn't found.
 */
    static channel_T *
channel_fd2channel(sock_T fd, ch_part_T *partp)
{
    channel_T	*channel;
    ch_part_T	part;

    if (fd == INVALID_FD)
	return NULL;

    FOR_ALL_CHANNELS(channel)
    {
	for (part = PART_SOCK; part < PART_IN; ++part)
	    if (channel->ch_part[part].ch_fd == fd)
	    {
		*partp = part;
		return channel;
	    }
    }
    return NULL;
}

    static void
channel_read_fd(int fd)
{
    channel_T	*channel;
    ch_part_T	part;

    channel = channel_fd2channel(fd, &part);
    if (channel == NULL)
	ch_error(NULL, "Channel for fd %d not found", fd);
    else
	channel_read(channel, part, "channel_read_fd");
}
# endif

/*
 * Read a command from netbeans.
 */
# ifdef FEAT_GUI_X11
    static void
messageFromServerX11(XtPointer clientData,
		  int *unused1 UNUSED,
		  XtInputId *unused2 UNUSED)
{
    channel_read_fd((int)(long)clientData);
}
# endif

# ifdef FEAT_GUI_GTK
#  if GTK_CHECK_VERSION(3,0,0)
    static gboolean
messageFromServerGtk3(GIOChannel *unused1 UNUSED,
		  GIOCondition unused2 UNUSED,
		  gpointer clientData)
{
    channel_read_fd(GPOINTER_TO_INT(clientData));
    return TRUE; // Return FALSE instead in case the event source is to
		 // be removed after this function returns.
}
#  else
    static void
messageFromServerGtk2(gpointer clientData,
		  gint unused1 UNUSED,
		  GdkInputCondition unused2 UNUSED)
{
    channel_read_fd((int)(long)clientData);
}
#  endif
# endif

    static void
channel_gui_register_one(channel_T *channel, ch_part_T part UNUSED)
{
    if (!CH_HAS_GUI)
	return;

    // gets stuck in handling events for a not connected channel
    if (channel->ch_keep_open)
	return;

# ifdef FEAT_GUI_X11
    // Tell notifier we are interested in being called when there is input on
    // the editor connection socket.
    if (channel->ch_part[part].ch_inputHandler == (XtInputId)NULL)
    {
	ch_log(channel, "Registering part %s with fd %d",
		ch_part_names[part], channel->ch_part[part].ch_fd);

	channel->ch_part[part].ch_inputHandler = XtAppAddInput(
		(XtAppContext)app_context,
		channel->ch_part[part].ch_fd,
		(XtPointer)(XtInputReadMask + XtInputExceptMask),
		messageFromServerX11,
		(XtPointer)(long)channel->ch_part[part].ch_fd);
    }
# else
#  ifdef FEAT_GUI_GTK
    // Tell gdk we are interested in being called when there is input on the
    // editor connection socket.
    if (channel->ch_part[part].ch_inputHandler == 0)
    {
	ch_log(channel, "Registering part %s with fd %d",
		ch_part_names[part], channel->ch_part[part].ch_fd);
#   if GTK_CHECK_VERSION(3,0,0)
	GIOChannel *chnnl = g_io_channel_unix_new(
		(gint)channel->ch_part[part].ch_fd);

	channel->ch_part[part].ch_inputHandler = g_io_add_watch(
		chnnl,
		G_IO_IN|G_IO_HUP|G_IO_ERR|G_IO_PRI,
		messageFromServerGtk3,
		GINT_TO_POINTER(channel->ch_part[part].ch_fd));

	g_io_channel_unref(chnnl);
#   else
	channel->ch_part[part].ch_inputHandler = gdk_input_add(
		(gint)channel->ch_part[part].ch_fd,
		(GdkInputCondition)
			     ((int)GDK_INPUT_READ + (int)GDK_INPUT_EXCEPTION),
		messageFromServerGtk2,
		(gpointer)(long)channel->ch_part[part].ch_fd);
#   endif
    }
#  endif
# endif
}

    static void
channel_gui_register(channel_T *channel)
{
    if (channel->CH_SOCK_FD != INVALID_FD)
	channel_gui_register_one(channel, PART_SOCK);
    if (channel->CH_OUT_FD != INVALID_FD
	    && channel->CH_OUT_FD != channel->CH_SOCK_FD)
	channel_gui_register_one(channel, PART_OUT);
    if (channel->CH_ERR_FD != INVALID_FD
	    && channel->CH_ERR_FD != channel->CH_SOCK_FD
	    && channel->CH_ERR_FD != channel->CH_OUT_FD)
	channel_gui_register_one(channel, PART_ERR);
}

/*
 * Register any of our file descriptors with the GUI event handling system.
 * Called when the GUI has started.
 */
    void
channel_gui_register_all(void)
{
    channel_T *channel;

    FOR_ALL_CHANNELS(channel)
	channel_gui_register(channel);
}

    static void
channel_gui_unregister_one(channel_T *channel UNUSED, ch_part_T part UNUSED)
{
# ifdef FEAT_GUI_X11
    if (channel->ch_part[part].ch_inputHandler != (XtInputId)NULL)
    {
	ch_log(channel, "Unregistering part %s", ch_part_names[part]);
	XtRemoveInput(channel->ch_part[part].ch_inputHandler);
	channel->ch_part[part].ch_inputHandler = (XtInputId)NULL;
    }
# else
#  ifdef FEAT_GUI_GTK
    if (channel->ch_part[part].ch_inputHandler != 0)
    {
	ch_log(channel, "Unregistering part %s", ch_part_names[part]);
#   if GTK_CHECK_VERSION(3,0,0)
	g_source_remove(channel->ch_part[part].ch_inputHandler);
#   else
	gdk_input_remove(channel->ch_part[part].ch_inputHandler);
#   endif
	channel->ch_part[part].ch_inputHandler = 0;
    }
#  endif
# endif
}

    static void
channel_gui_unregister(channel_T *channel)
{
    ch_part_T	part;

    for (part = PART_SOCK; part < PART_IN; ++part)
	channel_gui_unregister_one(channel, part);
}

#endif  // FEAT_GUI

/*
 * For Unix we need to call connect() again after connect() failed.
 * On Win32 one time is sufficient.
 */
    static int
channel_connect(
	channel_T *channel,
	const struct sockaddr *server_addr,
	int server_addrlen,
	int *waittime)
{
    int		sd = -1;
#ifdef MSWIN
    u_long	val = 1;
#endif

    while (TRUE)
    {
	long	elapsed_msec = 0;
	int	waitnow;
	int	ret;

	if (sd >= 0)
	    sock_close(sd);
	sd = socket(server_addr->sa_family, SOCK_STREAM, 0);
	if (sd == -1)
	{
	    ch_error(channel, "in socket() in channel_connect().");
	    PERROR(_(e_socket_in_channel_connect));
	    return -1;
	}

	if (*waittime >= 0)
	{
	    // Make connect() non-blocking.
	    if (
#ifdef MSWIN
		ioctlsocket(sd, FIONBIO, &val) < 0
#else
		fcntl(sd, F_SETFL, O_NONBLOCK) < 0
#endif
	       )
	    {
		SOCK_ERRNO;
		ch_error(channel,
		      "channel_connect: Connect failed with errno %d", errno);
		sock_close(sd);
		return -1;
	    }
	}

	// Try connecting to the server.
	ch_log(channel, "Connecting...");

	ret = connect(sd, server_addr, server_addrlen);
	if (ret == 0)
	    // The connection could be established.
	    break;

	SOCK_ERRNO;
	if (*waittime < 0 || (errno != EWOULDBLOCK
		&& errno != ECONNREFUSED
#ifdef EINPROGRESS
		&& errno != EINPROGRESS
#endif
		))
	{
	    ch_error(channel,
		      "channel_connect: Connect failed with errno %d", errno);
	    PERROR(_(e_cannot_connect_to_port));
	    sock_close(sd);
	    return -1;
	}
	else if (errno == ECONNREFUSED)
	{
	    ch_error(channel, "channel_connect: Connection refused");
	    sock_close(sd);
	    return -1;
	}

	// Limit the waittime to 50 msec.  If it doesn't work within this
	// time we close the socket and try creating it again.
	waitnow = *waittime > 50 ? 50 : *waittime;

	// If connect() didn't finish then try using select() to wait for the
	// connection to be made. For Win32 always use select() to wait.
	{
	    struct timeval	tv;
	    fd_set		rfds;
	    fd_set		wfds;
#ifndef MSWIN
	    int			so_error = 0;
	    socklen_t		so_error_len = sizeof(so_error);
	    struct timeval	start_tv;
	    struct timeval	end_tv;
#endif
	    FD_ZERO(&rfds);
	    FD_SET(sd, &rfds);
	    FD_ZERO(&wfds);
	    FD_SET(sd, &wfds);

	    tv.tv_sec = waitnow / 1000;
	    tv.tv_usec = (waitnow % 1000) * 1000;
#ifndef MSWIN
	    gettimeofday(&start_tv, NULL);
#endif
	    ch_log(channel,
		      "Waiting for connection (waiting %d msec)...", waitnow);

	    ret = select(sd + 1, &rfds, &wfds, NULL, &tv);
	    if (ret < 0)
	    {
		SOCK_ERRNO;
		ch_error(channel,
		      "channel_connect: Connect failed with errno %d", errno);
		PERROR(_(e_cannot_connect_to_port));
		sock_close(sd);
		return -1;
	    }

#ifdef MSWIN
	    // On Win32: select() is expected to work and wait for up to
	    // "waitnow" msec for the socket to be open.
	    if (FD_ISSET(sd, &wfds))
		break;
	    elapsed_msec = waitnow;
	    if (*waittime > 1 && elapsed_msec < *waittime)
	    {
		*waittime -= elapsed_msec;
		continue;
	    }
#else
	    // On Linux-like systems: See socket(7) for the behavior
	    // After putting the socket in non-blocking mode, connect() will
	    // return EINPROGRESS, select() will not wait (as if writing is
	    // possible), need to use getsockopt() to check if the socket is
	    // actually able to connect.
	    // We detect a failure to connect when either read and write fds
	    // are set.  Use getsockopt() to find out what kind of failure.
	    if (FD_ISSET(sd, &rfds) || FD_ISSET(sd, &wfds))
	    {
		ret = getsockopt(sd,
			      SOL_SOCKET, SO_ERROR, &so_error, &so_error_len);
		if (ret < 0 || (so_error != 0
			&& so_error != EWOULDBLOCK
			&& so_error != ECONNREFUSED
# ifdef EINPROGRESS
			&& so_error != EINPROGRESS
# endif
			))
		{
		    ch_error(channel,
			    "channel_connect: Connect failed with errno %d",
			    so_error);
		    PERROR(_(e_cannot_connect_to_port));
		    sock_close(sd);
		    return -1;
		}
		else if (errno == ECONNREFUSED)
		{
		    ch_error(channel, "channel_connect: Connection refused");
		    sock_close(sd);
		    return -1;
		}
	    }

	    if (FD_ISSET(sd, &wfds) && so_error == 0)
		// Did not detect an error, connection is established.
		break;

	    gettimeofday(&end_tv, NULL);
	    elapsed_msec = (end_tv.tv_sec - start_tv.tv_sec) * 1000
				 + (end_tv.tv_usec - start_tv.tv_usec) / 1000;
#endif
	}

#ifndef MSWIN
	if (*waittime > 1 && elapsed_msec < *waittime)
	{
	    // The port isn't ready but we also didn't get an error.
	    // This happens when the server didn't open the socket
	    // yet.  Select() may return early, wait until the remaining
	    // "waitnow"  and try again.
	    waitnow -= elapsed_msec;
	    *waittime -= elapsed_msec;
	    if (waitnow > 0)
	    {
		mch_delay((long)waitnow, MCH_DELAY_IGNOREINPUT);
		ui_breakcheck();
		*waittime -= waitnow;
	    }
	    if (!got_int)
	    {
		if (*waittime <= 0)
		    // give it one more try
		    *waittime = 1;
		continue;
	    }
	    // we were interrupted, behave as if timed out
	}
#endif

	// We timed out.
	ch_error(channel, "Connection timed out");
	sock_close(sd);
	return -1;
    }

    if (*waittime >= 0)
    {
#ifdef MSWIN
	val = 0;
	ioctlsocket(sd, FIONBIO, &val);
#else
	(void)fcntl(sd, F_SETFL, 0);
#endif
    }

    return sd;
}

/*
 * Open a socket channel to the UNIX socket at "path".
 * Returns the channel for success.
 * Returns NULL for failure.
 */
    static channel_T *
channel_open_unix(
	const char *path,
	void (*nb_close_cb)(void))
{
    channel_T		*channel = NULL;
    int			sd = -1;
    size_t		path_len = STRLEN(path);
    struct sockaddr_un	server;
    size_t		server_len;
    int			waittime = -1;

    if (*path == NUL || path_len >= sizeof(server.sun_path))
    {
	semsg(_(e_invalid_argument_str), path);
	return NULL;
    }

    channel = add_channel();
    if (channel == NULL)
    {
	ch_error(NULL, "Cannot allocate channel.");
	return NULL;
    }

    CLEAR_FIELD(server);
    server.sun_family = AF_UNIX;
    STRNCPY(server.sun_path, path, sizeof(server.sun_path) - 1);

    ch_log(channel, "Trying to connect to %s", path);

    server_len = offsetof(struct sockaddr_un, sun_path) + path_len + 1;
    sd = channel_connect(channel, (struct sockaddr *)&server, (int)server_len,
								   &waittime);

    if (sd < 0)
    {
	channel_free(channel);
	return NULL;
    }

    ch_log(channel, "Connection made");

    channel->CH_SOCK_FD = (sock_T)sd;
    channel->ch_nb_close_cb = nb_close_cb;
    channel->ch_hostname = (char *)vim_strsave((char_u *)path);
    channel->ch_port = 0;
    channel->ch_to_be_closed |= (1U << PART_SOCK);

#ifdef FEAT_GUI
    channel_gui_register_one(channel, PART_SOCK);
#endif

    return channel;
}

/*
 * Open a socket channel to "hostname":"port".
 * "waittime" is the time in msec to wait for the connection.
 * When negative wait forever.
 * Returns the channel for success.
 * Returns NULL for failure.
 */
    channel_T *
channel_open(
	const char *hostname,
	int port,
	int waittime,
	void (*nb_close_cb)(void))
{
    int			sd = -1;
    channel_T		*channel = NULL;
#ifdef FEAT_IPV6
    int			err;
    struct addrinfo	hints;
    struct addrinfo	*res = NULL;
    struct addrinfo	*addr = NULL;
#else
    struct sockaddr_in	server;
    struct hostent	*host = NULL;
#endif

#ifdef MSWIN
    channel_init_winsock();
#endif

    channel = add_channel();
    if (channel == NULL)
    {
	ch_error(NULL, "Cannot allocate channel.");
	return NULL;
    }

    // Get the server internet address and put into addr structure fill in the
    // socket address structure and connect to server.
#ifdef FEAT_IPV6
    CLEAR_FIELD(hints);
    hints.ai_family = AF_UNSPEC;
    hints.ai_socktype = SOCK_STREAM;
# if defined(__ANDROID__)
    hints.ai_flags = AI_ADDRCONFIG;
# elif defined(AI_ADDRCONFIG) && defined(AI_V4MAPPED)
    hints.ai_flags = AI_ADDRCONFIG | AI_V4MAPPED;
# endif
    // Set port number manually in order to prevent name resolution services
    // from being invoked in the environment where AI_NUMERICSERV is not
    // defined.
    if ((err = getaddrinfo(hostname, NULL, &hints, &res)) != 0)
    {
	ch_error(channel, "in getaddrinfo() in channel_open()");
	semsg(_(e_getaddrinfo_in_channel_open_str), gai_strerror(err));
	channel_free(channel);
	return NULL;
    }

    for (addr = res; addr != NULL; addr = addr->ai_next)
    {
	const char  *dst = hostname;
# ifdef HAVE_INET_NTOP
	const void  *src = NULL;
	char	    buf[NUMBUFLEN];
# endif

	if (addr->ai_family == AF_INET6)
	{
	    struct sockaddr_in6 *sai = (struct sockaddr_in6 *)addr->ai_addr;

	    sai->sin6_port = htons(port);
# ifdef HAVE_INET_NTOP
	    src = &sai->sin6_addr;
# endif
	}
	else if (addr->ai_family == AF_INET)
	{
	    struct sockaddr_in *sai = (struct sockaddr_in *)addr->ai_addr;

	    sai->sin_port = htons(port);
# ifdef HAVE_INET_NTOP
	    src = &sai->sin_addr;
#endif
	}
# ifdef HAVE_INET_NTOP
	if (src != NULL)
	{
	    dst = inet_ntop(addr->ai_family, src, buf, sizeof(buf));
	    if (dst == NULL)
		dst = hostname;
	    else if (STRCMP(hostname, dst) != 0)
		ch_log(channel, "Resolved %s to %s", hostname, dst);
	}
# endif

	ch_log(channel, "Trying to connect to %s port %d", dst, port);

	// On Mac and Solaris a zero timeout almost never works.  Waiting for
	// one millisecond already helps a lot.  Later Mac systems (using IPv6)
	// need more time, 15 milliseconds appears to work well.
	// Let's do it for all systems, because we don't know why this is
	// needed.
	if (waittime == 0)
	    waittime = 15;

	sd = channel_connect(channel, addr->ai_addr, (int)addr->ai_addrlen,
								   &waittime);
	if (sd >= 0)
	    break;
    }

    freeaddrinfo(res);
#else
    CLEAR_FIELD(server);
    server.sin_family = AF_INET;
    server.sin_port = htons(port);
    if ((host = gethostbyname(hostname)) == NULL)
    {
	ch_error(channel, "in gethostbyname() in channel_open()");
	PERROR(_(e_gethostbyname_in_channel_open));
	channel_free(channel);
	return NULL;
    }
    {
	char *p;

	// When using host->h_addr_list[0] directly ubsan warns for it to not
	// be aligned.  First copy the pointer to avoid that.
	memcpy(&p, &host->h_addr_list[0], sizeof(p));
	memcpy((char *)&server.sin_addr, p, host->h_length);
    }

    ch_log(channel, "Trying to connect to %s port %d", hostname, port);

    // On Mac and Solaris a zero timeout almost never works.  At least wait one
    // millisecond.  Let's do it for all systems, because we don't know why
    // this is needed.
    if (waittime == 0)
	waittime = 1;

    sd = channel_connect(channel, (struct sockaddr *)&server, sizeof(server),
								   &waittime);
#endif

    if (sd < 0)
    {
	channel_free(channel);
	return NULL;
    }

    ch_log(channel, "Connection made");

    channel->CH_SOCK_FD = (sock_T)sd;
    channel->ch_nb_close_cb = nb_close_cb;
    channel->ch_hostname = (char *)vim_strsave((char_u *)hostname);
    channel->ch_port = port;
    channel->ch_to_be_closed |= (1U << PART_SOCK);

#ifdef FEAT_GUI
    channel_gui_register_one(channel, PART_SOCK);
#endif

    return channel;
}

    static void
free_set_callback(callback_T *cbp, callback_T *callback)
{
    free_callback(cbp);

    if (callback->cb_name != NULL && *callback->cb_name != NUL)
	copy_callback(cbp, callback);
    else
	cbp->cb_name = NULL;
}

/*
 * Prepare buffer "buf" for writing channel output to.
 */
	static void
prepare_buffer(buf_T *buf)
{
    buf_T *save_curbuf = curbuf;

    buf_copy_options(buf, BCO_ENTER);
    curbuf = buf;
#ifdef FEAT_QUICKFIX
    set_option_value_give_err((char_u *)"bt",
					    0L, (char_u *)"nofile", OPT_LOCAL);
    set_option_value_give_err((char_u *)"bh", 0L, (char_u *)"hide", OPT_LOCAL);
#endif
    if (curbuf->b_ml.ml_mfp == NULL)
	ml_open(curbuf);
    curbuf = save_curbuf;
}

/*
 * Find a buffer matching "name" or create a new one.
 * Returns NULL if there is something very wrong (error already reported).
 */
    static buf_T *
channel_find_buffer(char_u *name, int err, int msg)
{
    buf_T *buf = NULL;
    buf_T *save_curbuf = curbuf;

    if (name != NULL && *name != NUL)
    {
	buf = buflist_findname(name);
	if (buf == NULL)
	    buf = buflist_findname_exp(name);
    }

    if (buf != NULL)
	return buf;

    buf = buflist_new(name == NULL || *name == NUL ? NULL : name,
	    NULL, (linenr_T)0, BLN_LISTED | BLN_NEW);
    if (buf == NULL)
	return NULL;
    prepare_buffer(buf);

    curbuf = buf;
    if (msg)
	ml_replace(1, (char_u *)(err ? "Reading from channel error..."
		    : "Reading from channel output..."), TRUE);
    changed_bytes(1, 0);
    curbuf = save_curbuf;

    return buf;
}

/*
 * Set various properties from an "opt" argument.
 */
    static void
channel_set_options(channel_T *channel, jobopt_T *opt)
{
    ch_part_T	part;

    if (opt->jo_set & JO_MODE)
	for (part = PART_SOCK; part < PART_COUNT; ++part)
	    channel->ch_part[part].ch_mode = opt->jo_mode;
    if (opt->jo_set & JO_IN_MODE)
	channel->ch_part[PART_IN].ch_mode = opt->jo_in_mode;
    if (opt->jo_set & JO_OUT_MODE)
	channel->ch_part[PART_OUT].ch_mode = opt->jo_out_mode;
    if (opt->jo_set & JO_ERR_MODE)
	channel->ch_part[PART_ERR].ch_mode = opt->jo_err_mode;
    channel->ch_nonblock = opt->jo_noblock;

    if (opt->jo_set & JO_TIMEOUT)
	for (part = PART_SOCK; part < PART_COUNT; ++part)
	    channel->ch_part[part].ch_timeout = opt->jo_timeout;
    if (opt->jo_set & JO_OUT_TIMEOUT)
	channel->ch_part[PART_OUT].ch_timeout = opt->jo_out_timeout;
    if (opt->jo_set & JO_ERR_TIMEOUT)
	channel->ch_part[PART_ERR].ch_timeout = opt->jo_err_timeout;
    if (opt->jo_set & JO_BLOCK_WRITE)
	channel->ch_part[PART_IN].ch_block_write = 1;

    if (opt->jo_set & JO_CALLBACK)
	free_set_callback(&channel->ch_callback, &opt->jo_callback);
    if (opt->jo_set & JO_OUT_CALLBACK)
	free_set_callback(&channel->ch_part[PART_OUT].ch_callback,
							      &opt->jo_out_cb);
    if (opt->jo_set & JO_ERR_CALLBACK)
	free_set_callback(&channel->ch_part[PART_ERR].ch_callback,
							      &opt->jo_err_cb);
    if (opt->jo_set & JO_CLOSE_CALLBACK)
	free_set_callback(&channel->ch_close_cb, &opt->jo_close_cb);
    channel->ch_drop_never = opt->jo_drop_never;

    if ((opt->jo_set & JO_OUT_IO) && opt->jo_io[PART_OUT] == JIO_BUFFER)
    {
	buf_T *buf;

	// writing output to a buffer. Default mode is NL.
	if (!(opt->jo_set & JO_OUT_MODE))
	    channel->ch_part[PART_OUT].ch_mode = CH_MODE_NL;
	if (opt->jo_set & JO_OUT_BUF)
	{
	    buf = buflist_findnr(opt->jo_io_buf[PART_OUT]);
	    if (buf == NULL)
		semsg(_(e_buffer_nr_does_not_exist),
					       (long)opt->jo_io_buf[PART_OUT]);
	}
	else
	{
	    int msg = TRUE;

	    if (opt->jo_set2 & JO2_OUT_MSG)
		msg = opt->jo_message[PART_OUT];
	    buf = channel_find_buffer(opt->jo_io_name[PART_OUT], FALSE, msg);
	}
	if (buf != NULL)
	{
	    if (opt->jo_set & JO_OUT_MODIFIABLE)
		channel->ch_part[PART_OUT].ch_nomodifiable =
						!opt->jo_modifiable[PART_OUT];

	    if (!buf->b_p_ma && !channel->ch_part[PART_OUT].ch_nomodifiable)
	    {
		emsg(_(e_cannot_make_changes_modifiable_is_off));
	    }
	    else
	    {
		ch_log(channel, "writing out to buffer '%s'",
						       (char *)buf->b_ffname);
		set_bufref(&channel->ch_part[PART_OUT].ch_bufref, buf);
		// if the buffer was deleted or unloaded resurrect it
		if (buf->b_ml.ml_mfp == NULL)
		    prepare_buffer(buf);
	    }
	}
    }

    if ((opt->jo_set & JO_ERR_IO) && (opt->jo_io[PART_ERR] == JIO_BUFFER
	 || (opt->jo_io[PART_ERR] == JIO_OUT && (opt->jo_set & JO_OUT_IO)
				       && opt->jo_io[PART_OUT] == JIO_BUFFER)))
    {
	buf_T *buf;

	// writing err to a buffer. Default mode is NL.
	if (!(opt->jo_set & JO_ERR_MODE))
	    channel->ch_part[PART_ERR].ch_mode = CH_MODE_NL;
	if (opt->jo_io[PART_ERR] == JIO_OUT)
	    buf = channel->ch_part[PART_OUT].ch_bufref.br_buf;
	else if (opt->jo_set & JO_ERR_BUF)
	{
	    buf = buflist_findnr(opt->jo_io_buf[PART_ERR]);
	    if (buf == NULL)
		semsg(_(e_buffer_nr_does_not_exist),
					       (long)opt->jo_io_buf[PART_ERR]);
	}
	else
	{
	    int msg = TRUE;

	    if (opt->jo_set2 & JO2_ERR_MSG)
		msg = opt->jo_message[PART_ERR];
	    buf = channel_find_buffer(opt->jo_io_name[PART_ERR], TRUE, msg);
	}
	if (buf != NULL)
	{
	    if (opt->jo_set & JO_ERR_MODIFIABLE)
		channel->ch_part[PART_ERR].ch_nomodifiable =
						!opt->jo_modifiable[PART_ERR];
	    if (!buf->b_p_ma && !channel->ch_part[PART_ERR].ch_nomodifiable)
	    {
		emsg(_(e_cannot_make_changes_modifiable_is_off));
	    }
	    else
	    {
		ch_log(channel, "writing err to buffer '%s'",
						       (char *)buf->b_ffname);
		set_bufref(&channel->ch_part[PART_ERR].ch_bufref, buf);
		// if the buffer was deleted or unloaded resurrect it
		if (buf->b_ml.ml_mfp == NULL)
		    prepare_buffer(buf);
	    }
	}
    }

    channel->ch_part[PART_OUT].ch_io = opt->jo_io[PART_OUT];
    channel->ch_part[PART_ERR].ch_io = opt->jo_io[PART_ERR];
    channel->ch_part[PART_IN].ch_io = opt->jo_io[PART_IN];
}

/*
 * Implements ch_open().
 */
    static channel_T *
channel_open_func(typval_T *argvars)
{
    char_u	*address;
    char_u	*p;
    char	*rest;
    int		port = 0;
    int		is_ipv6 = FALSE;
    int		is_unix = FALSE;
    jobopt_T    opt;
    channel_T	*channel = NULL;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_opt_dict_arg(argvars, 1) == FAIL))
	return NULL;

    address = tv_get_string(&argvars[0]);
    if (argvars[1].v_type != VAR_UNKNOWN
	    && check_for_nonnull_dict_arg(argvars, 1) == FAIL)
	return NULL;

    if (*address == NUL)
    {
	semsg(_(e_invalid_argument_str), address);
	return NULL;
    }

    if (!STRNCMP(address, "unix:", 5))
    {
	is_unix = TRUE;
	address += 5;
    }
    else if (*address == '[')
    {
	// ipv6 address
	is_ipv6 = TRUE;
	p = vim_strchr(address + 1, ']');
	if (p == NULL || *++p != ':')
	{
	    semsg(_(e_invalid_argument_str), address);
	    return NULL;
	}
    }
    else
    {
	// ipv4 address
	p = vim_strchr(address, ':');
	if (p == NULL)
	{
	    semsg(_(e_invalid_argument_str), address);
	    return NULL;
	}
    }

    if (!is_unix)
    {
	port = strtol((char *)(p + 1), &rest, 10);
	if (port <= 0 || port >= 65536 || *rest != NUL)
	{
	    semsg(_(e_invalid_argument_str), address);
	    return NULL;
	}
	if (is_ipv6)
	{
	    // strip '[' and ']'
	    ++address;
	    *(p - 1) = NUL;
	}
	else
	    *p = NUL;
    }

    // parse options
    clear_job_options(&opt);
    opt.jo_mode = CH_MODE_JSON;
    opt.jo_timeout = 2000;
    if (get_job_options(&argvars[1], &opt,
	    JO_MODE_ALL + JO_CB_ALL + JO_TIMEOUT_ALL
		+ (is_unix? 0 : JO_WAITTIME), 0) == FAIL)
	goto theend;
    if (opt.jo_timeout < 0)
    {
	emsg(_(e_invalid_argument));
	goto theend;
    }

    if (is_unix)
	channel = channel_open_unix((char *)address, NULL);
    else
	channel = channel_open((char *)address, port, opt.jo_waittime, NULL);
    if (channel != NULL)
    {
	opt.jo_set = JO_ALL;
	channel_set_options(channel, &opt);
    }
theend:
    free_job_options(&opt);
    return channel;
}

    void
ch_close_part(channel_T *channel, ch_part_T part)
{
    sock_T *fd = &channel->ch_part[part].ch_fd;

    if (*fd == INVALID_FD)
	return;

    if (part == PART_SOCK)
	sock_close(*fd);
    else
    {
	// When using a pty the same FD is set on multiple parts, only
	// close it when the last reference is closed.
	if ((part == PART_IN || channel->CH_IN_FD != *fd)
		&& (part == PART_OUT || channel->CH_OUT_FD != *fd)
		&& (part == PART_ERR || channel->CH_ERR_FD != *fd))
	{
#ifdef MSWIN
	    if (channel->ch_named_pipe)
		DisconnectNamedPipe((HANDLE)fd);
#endif
	    fd_close(*fd);
	}
    }
    *fd = INVALID_FD;

    // channel is closed, may want to end the job if it was the last
    channel->ch_to_be_closed &= ~(1U << part);
}

    void
channel_set_pipes(channel_T *channel, sock_T in, sock_T out, sock_T err)
{
    if (in != INVALID_FD)
    {
	ch_close_part(channel, PART_IN);
	channel->CH_IN_FD = in;
# if defined(UNIX)
	// Do not end the job when all output channels are closed, wait until
	// the job ended.
	if (mch_isatty(in))
	    channel->ch_to_be_closed |= (1U << PART_IN);
# endif
    }
    if (out != INVALID_FD)
    {
# if defined(FEAT_GUI)
	channel_gui_unregister_one(channel, PART_OUT);
# endif
	ch_close_part(channel, PART_OUT);
	channel->CH_OUT_FD = out;
	channel->ch_to_be_closed |= (1U << PART_OUT);
# if defined(FEAT_GUI)
	channel_gui_register_one(channel, PART_OUT);
# endif
    }
    if (err != INVALID_FD)
    {
# if defined(FEAT_GUI)
	channel_gui_unregister_one(channel, PART_ERR);
# endif
	ch_close_part(channel, PART_ERR);
	channel->CH_ERR_FD = err;
	channel->ch_to_be_closed |= (1U << PART_ERR);
# if defined(FEAT_GUI)
	channel_gui_register_one(channel, PART_ERR);
# endif
    }
}

/*
 * Sets the job the channel is associated with and associated options.
 * This does not keep a refcount, when the job is freed ch_job is cleared.
 */
    void
channel_set_job(channel_T *channel, job_T *job, jobopt_T *options)
{
    channel->ch_job = job;

    channel_set_options(channel, options);

    if (job->jv_in_buf == NULL)
	return;

    chanpart_T *in_part = &channel->ch_part[PART_IN];

    set_bufref(&in_part->ch_bufref, job->jv_in_buf);
    ch_log(channel, "reading from buffer '%s'",
	    (char *)in_part->ch_bufref.br_buf->b_ffname);
    if (options->jo_set & JO_IN_TOP)
    {
	if (options->jo_in_top == 0 && !(options->jo_set & JO_IN_BOT))
	{
	    // Special mode: send last-but-one line when appending a line
	    // to the buffer.
	    in_part->ch_bufref.br_buf->b_write_to_channel = TRUE;
	    in_part->ch_buf_append = TRUE;
	    in_part->ch_buf_top =
		in_part->ch_bufref.br_buf->b_ml.ml_line_count + 1;
	}
	else
	    in_part->ch_buf_top = options->jo_in_top;
    }
    else
	in_part->ch_buf_top = 1;
    if (options->jo_set & JO_IN_BOT)
	in_part->ch_buf_bot = options->jo_in_bot;
    else
	in_part->ch_buf_bot = in_part->ch_bufref.br_buf->b_ml.ml_line_count;
}

/*
 * Set the callback for "channel"/"part" for the response with "id".
 */
    static void
channel_set_req_callback(
	channel_T   *channel,
	ch_part_T   part,
	callback_T  *callback,
	int	    id)
{
    cbq_T *head = &channel->ch_part[part].ch_cb_head;
    cbq_T *item = ALLOC_ONE(cbq_T);

    if (item == NULL)
	return;

    copy_callback(&item->cq_callback, callback);
    item->cq_seq_nr = id;
    item->cq_prev = head->cq_prev;
    head->cq_prev = item;
    item->cq_next = NULL;
    if (item->cq_prev == NULL)
	head->cq_next = item;
    else
	item->cq_prev->cq_next = item;
}

    static void
write_buf_line(buf_T *buf, linenr_T lnum, channel_T *channel)
{
    char_u  *line = ml_get_buf(buf, lnum, FALSE);
    int	    len = (int)STRLEN(line);
    char_u  *p;
    int	    i;

    // Need to make a copy to be able to append a NL.
    if ((p = alloc(len + 2)) == NULL)
	return;
    memcpy((char *)p, (char *)line, len);

    if (channel->ch_write_text_mode)
	p[len] = CAR;
    else
    {
	for (i = 0; i < len; ++i)
	    if (p[i] == NL)
		p[i] = NUL;

	p[len] = NL;
    }
    p[len + 1] = NUL;
    channel_send(channel, PART_IN, p, len + 1, "write_buf_line");
    vim_free(p);
}

/*
 * Return TRUE if "channel" can be written to.
 * Returns FALSE if the input is closed or the write would block.
 */
    static int
can_write_buf_line(channel_T *channel)
{
    chanpart_T *in_part = &channel->ch_part[PART_IN];

    if (in_part->ch_fd == INVALID_FD)
	return FALSE;  // pipe was closed

    // for testing: block every other attempt to write
    if (in_part->ch_block_write == 1)
	in_part->ch_block_write = -1;
    else if (in_part->ch_block_write == -1)
	in_part->ch_block_write = 1;

    // TODO: Win32 implementation, probably using WaitForMultipleObjects()
#ifndef MSWIN
    {
# if defined(HAVE_SELECT)
	struct timeval	tval;
	fd_set		wfds;
	int		ret;

	FD_ZERO(&wfds);
	FD_SET((int)in_part->ch_fd, &wfds);
	tval.tv_sec = 0;
	tval.tv_usec = 0;
	for (;;)
	{
	    ret = select((int)in_part->ch_fd + 1, NULL, &wfds, NULL, &tval);
#  ifdef EINTR
	    SOCK_ERRNO;
	    if (ret == -1 && errno == EINTR)
		continue;
#  endif
	    if (ret <= 0 || in_part->ch_block_write == 1)
	    {
		if (ret > 0)
		    ch_log(channel, "FAKED Input not ready for writing");
		else
		    ch_log(channel, "Input not ready for writing");
		return FALSE;
	    }
	    break;
	}
# else
	struct pollfd	fds;

	fds.fd = in_part->ch_fd;
	fds.events = POLLOUT;
	if (poll(&fds, 1, 0) <= 0)
	{
	    ch_log(channel, "Input not ready for writing");
	    return FALSE;
	}
	if (in_part->ch_block_write == 1)
	{
	    ch_log(channel, "FAKED Input not ready for writing");
	    return FALSE;
	}
# endif
    }
#endif
    return TRUE;
}

/*
 * Write any buffer lines to the input channel.
 */
    void
channel_write_in(channel_T *channel)
{
    chanpart_T *in_part = &channel->ch_part[PART_IN];
    linenr_T    lnum;
    buf_T	*buf = in_part->ch_bufref.br_buf;
    int		written = 0;

    if (buf == NULL || in_part->ch_buf_append)
	return;  // no buffer or using appending
    if (!bufref_valid(&in_part->ch_bufref) || buf->b_ml.ml_mfp == NULL)
    {
	// buffer was wiped out or unloaded
	ch_log(channel, "input buffer has been wiped out");
	in_part->ch_bufref.br_buf = NULL;
	return;
    }

    for (lnum = in_part->ch_buf_top; lnum <= in_part->ch_buf_bot
				   && lnum <= buf->b_ml.ml_line_count; ++lnum)
    {
	if (!can_write_buf_line(channel))
	    break;
	write_buf_line(buf, lnum, channel);
	++written;
    }

    if (written == 1)
	ch_log(channel, "written line %d to channel", (int)lnum - 1);
    else if (written > 1)
	ch_log(channel, "written %d lines to channel", written);

    in_part->ch_buf_top = lnum;
    if (lnum > buf->b_ml.ml_line_count || lnum > in_part->ch_buf_bot)
    {
#if defined(FEAT_TERMINAL)
	// Send CTRL-D or "eof_chars" to close stdin on MS-Windows.
	if (channel->ch_job != NULL)
	    term_send_eof(channel);
#endif

	// Writing is done, no longer need the buffer.
	in_part->ch_bufref.br_buf = NULL;
	ch_log(channel, "Finished writing all lines to channel");

	// Close the pipe/socket, so that the other side gets EOF.
	ch_close_part(channel, PART_IN);
    }
    else
	ch_log(channel, "Still %ld more lines to write",
				   (long)(buf->b_ml.ml_line_count - lnum + 1));
}

/*
 * Handle buffer "buf" being freed, remove it from any channels.
 */
    void
channel_buffer_free(buf_T *buf)
{
    channel_T	*channel;
    ch_part_T	part;

    FOR_ALL_CHANNELS(channel)
	for (part = PART_SOCK; part < PART_COUNT; ++part)
	{
	    chanpart_T  *ch_part = &channel->ch_part[part];

	    if (ch_part->ch_bufref.br_buf == buf)
	    {
		ch_log(channel, "%s buffer has been wiped out",
							  ch_part_names[part]);
		ch_part->ch_bufref.br_buf = NULL;
	    }
	}
}

/*
 * Write any lines waiting to be written to "channel".
 */
    static void
channel_write_input(channel_T *channel)
{
    chanpart_T	*in_part = &channel->ch_part[PART_IN];

    if (in_part->ch_writeque.wq_next != NULL)
	channel_send(channel, PART_IN, (char_u *)"", 0, "channel_write_input");
    else if (in_part->ch_bufref.br_buf != NULL)
    {
	if (in_part->ch_buf_append)
	    channel_write_new_lines(in_part->ch_bufref.br_buf);
	else
	    channel_write_in(channel);
    }
}

/*
 * Write any lines waiting to be written to a channel.
 */
    void
channel_write_any_lines(void)
{
    channel_T	*channel;

    FOR_ALL_CHANNELS(channel)
	channel_write_input(channel);
}

/*
 * Write appended lines above the last one in "buf" to the channel.
 */
    void
channel_write_new_lines(buf_T *buf)
{
    channel_T	*channel;
    int		found_one = FALSE;

    // There could be more than one channel for the buffer, loop over all of
    // them.
    FOR_ALL_CHANNELS(channel)
    {
	chanpart_T  *in_part = &channel->ch_part[PART_IN];
	linenr_T    lnum;
	int	    written = 0;

	if (in_part->ch_bufref.br_buf == buf && in_part->ch_buf_append)
	{
	    if (in_part->ch_fd == INVALID_FD)
		continue;  // pipe was closed
	    found_one = TRUE;
	    for (lnum = in_part->ch_buf_bot; lnum < buf->b_ml.ml_line_count;
								       ++lnum)
	    {
		if (!can_write_buf_line(channel))
		    break;
		write_buf_line(buf, lnum, channel);
		++written;
	    }

	    if (written == 1)
		ch_log(channel, "written line %d to channel", (int)lnum - 1);
	    else if (written > 1)
		ch_log(channel, "written %d lines to channel", written);
	    if (lnum < buf->b_ml.ml_line_count)
		ch_log(channel, "Still %ld more lines to write",
				       (long)(buf->b_ml.ml_line_count - lnum));

	    in_part->ch_buf_bot = lnum;
	}
    }
    if (!found_one)
	buf->b_write_to_channel = FALSE;
}

/*
 * Invoke the "callback" on channel "channel".
 * This does not redraw but sets channel_need_redraw;
 */
    static void
invoke_callback(channel_T *channel, callback_T *callback, typval_T *argv)
{
    typval_T	rettv;

    if (safe_to_invoke_callback == 0)
	iemsg("Invoking callback when it is not safe");

    argv[0].v_type = VAR_CHANNEL;
    argv[0].vval.v_channel = channel;

    call_callback(callback, -1, &rettv, 2, argv);
    clear_tv(&rettv);
    channel_need_redraw = TRUE;
}

/*
 * Return the first node from "channel"/"part" without removing it.
 * Returns NULL if there is nothing.
 */
    readq_T *
channel_peek(channel_T *channel, ch_part_T part)
{
    readq_T *head = &channel->ch_part[part].ch_head;

    return head->rq_next;
}

/*
 * Return a pointer to the first NL in "node".
 * Skips over NUL characters.
 * Returns NULL if there is no NL.
 */
    char_u *
channel_first_nl(readq_T *node)
{
    char_u  *buffer = node->rq_buffer;
    long_u  i;

    for (i = 0; i < node->rq_buflen; ++i)
	if (buffer[i] == NL)
	    return buffer + i;
    return NULL;
}

/*
 * Return the first buffer from channel "channel"/"part" and remove it.
 * The caller must free it.
 * Returns NULL if there is nothing.
 */
    char_u *
channel_get(channel_T *channel, ch_part_T part, int *outlen)
{
    readq_T *head = &channel->ch_part[part].ch_head;
    readq_T *node = head->rq_next;
    char_u *p;

    if (node == NULL)
	return NULL;
    if (outlen != NULL)
	*outlen += node->rq_buflen;
    // dispose of the node but keep the buffer
    p = node->rq_buffer;
    head->rq_next = node->rq_next;
    if (node->rq_next == NULL)
	head->rq_prev = NULL;
    else
	node->rq_next->rq_prev = NULL;
    vim_free(node);
    return p;
}

/*
 * Returns the whole buffer contents concatenated for "channel"/"part".
 * Replaces NUL bytes with NL.
 */
    static char_u *
channel_get_all(channel_T *channel, ch_part_T part, int *outlen)
{
    readq_T *head = &channel->ch_part[part].ch_head;
    readq_T *node;
    long_u  len = 0;
    char_u  *res;
    char_u  *p;

    // Concatenate everything into one buffer.
    for (node = head->rq_next; node != NULL; node = node->rq_next)
	len += node->rq_buflen;
    res = alloc(len + 1);
    if (res == NULL)
	return NULL;
    p = res;
    for (node = head->rq_next; node != NULL; node = node->rq_next)
    {
	mch_memmove(p, node->rq_buffer, node->rq_buflen);
	p += node->rq_buflen;
    }
    *p = NUL;

    // Free all buffers
    do
    {
	p = channel_get(channel, part, NULL);
	vim_free(p);
    } while (p != NULL);

    if (outlen != NULL)
    {
	// Returning the length, keep NUL characters.
	*outlen += len;
	return res;
    }

    // Turn all NUL into NL, so that the result can be used as a string.
    p = res;
    while (p < res + len)
    {
	if (*p == NUL)
	    *p = NL;
#ifdef MSWIN
	else if (*p == 0x1b)
	{
	    // crush the escape sequence OSC 0/1/2: ESC ]0;
	    if (p + 3 < res + len
		    && p[1] == ']'
		    && (p[2] == '0' || p[2] == '1' || p[2] == '2')
		    && p[3] == ';')
	    {
		// '\a' becomes a NL
		while (p < res + (len - 1) && *p != '\a')
		    ++p;
		// BEL is zero width characters, suppress display mistake
		// ConPTY (after 10.0.18317) requires advance checking
		if (p[-1] == NUL)
		    p[-1] = 0x07;
	    }
	}
#endif
	++p;
    }

    return res;
}

/*
 * Consume "len" bytes from the head of "node".
 * Caller must check these bytes are available.
 */
    void
channel_consume(channel_T *channel, ch_part_T part, int len)
{
    readq_T *head = &channel->ch_part[part].ch_head;
    readq_T *node = head->rq_next;
    char_u *buf = node->rq_buffer;

    mch_memmove(buf, buf + len, node->rq_buflen - len);
    node->rq_buflen -= len;
    node->rq_buffer[node->rq_buflen] = NUL;
}

/*
 * Collapses the first and second buffer for "channel"/"part".
 * Returns FAIL if nothing was done.
 * When "want_nl" is TRUE collapse more buffers until a NL is found.
 * When the channel part mode is "lsp", collapse all the buffers as the http
 * header and the JSON content can be present in multiple buffers.
 */
    int
channel_collapse(channel_T *channel, ch_part_T part, int want_nl)
{
    ch_mode_T	mode = channel->ch_part[part].ch_mode;
    readq_T	*head = &channel->ch_part[part].ch_head;
    readq_T	*node = head->rq_next;
    readq_T	*last_node;
    readq_T	*n;
    char_u	*newbuf;
    char_u	*p;
    long_u	len;

    if (node == NULL || node->rq_next == NULL)
	return FAIL;

    last_node = node->rq_next;
    len = node->rq_buflen + last_node->rq_buflen;
    if (want_nl || mode == CH_MODE_LSP)
	while (last_node->rq_next != NULL
		&& (mode == CH_MODE_LSP
		    || channel_first_nl(last_node) == NULL))
	{
	    last_node = last_node->rq_next;
	    len += last_node->rq_buflen;
	}

    p = newbuf = alloc(len + 1);
    if (newbuf == NULL)
	return FAIL;	    // out of memory
    mch_memmove(p, node->rq_buffer, node->rq_buflen);
    p += node->rq_buflen;
    vim_free(node->rq_buffer);
    node->rq_buffer = newbuf;
    for (n = node; n != last_node; )
    {
	n = n->rq_next;
	mch_memmove(p, n->rq_buffer, n->rq_buflen);
	p += n->rq_buflen;
	vim_free(n->rq_buffer);
    }
    *p = NUL;
    node->rq_buflen = (long_u)(p - newbuf);

    // dispose of the collapsed nodes and their buffers
    for (n = node->rq_next; n != last_node; )
    {
	n = n->rq_next;
	vim_free(n->rq_prev);
    }
    node->rq_next = last_node->rq_next;
    if (last_node->rq_next == NULL)
	head->rq_prev = node;
    else
	last_node->rq_next->rq_prev = node;
    vim_free(last_node);
    return OK;
}

/*
 * Store "buf[len]" on "channel"/"part".
 * When "prepend" is TRUE put in front, otherwise append at the end.
 * Returns OK or FAIL.
 */
    static int
channel_save(channel_T *channel, ch_part_T part, char_u *buf, int len,
						      int prepend, char *lead)
{
    readq_T *node;
    readq_T *head = &channel->ch_part[part].ch_head;
    char_u  *p;
    int	    i;

    node = ALLOC_ONE(readq_T);
    if (node == NULL)
	return FAIL;	    // out of memory
    // A NUL is added at the end, because netbeans code expects that.
    // Otherwise a NUL may appear inside the text.
    node->rq_buffer = alloc(len + 1);
    if (node->rq_buffer == NULL)
    {
	vim_free(node);
	return FAIL;	    // out of memory
    }

    if (channel->ch_part[part].ch_mode == CH_MODE_NL)
    {
	// Drop any CR before a NL.
	p = node->rq_buffer;
	for (i = 0; i < len; ++i)
	    if (buf[i] != CAR || i + 1 >= len || buf[i + 1] != NL)
		*p++ = buf[i];
	*p = NUL;
	node->rq_buflen = (long_u)(p - node->rq_buffer);
    }
    else
    {
	mch_memmove(node->rq_buffer, buf, len);
	node->rq_buffer[len] = NUL;
	node->rq_buflen = (long_u)len;
    }

    if (prepend)
    {
	// prepend node to the head of the queue
	node->rq_next = head->rq_next;
	node->rq_prev = NULL;
	if (head->rq_next == NULL)
	    head->rq_prev = node;
	else
	    head->rq_next->rq_prev = node;
	head->rq_next = node;
    }
    else
    {
	// append node to the tail of the queue
	node->rq_next = NULL;
	node->rq_prev = head->rq_prev;
	if (head->rq_prev == NULL)
	    head->rq_next = node;
	else
	    head->rq_prev->rq_next = node;
	head->rq_prev = node;
    }

    if (ch_log_active() && lead != NULL)
	ch_log_literal(lead, channel, part, buf, len);

    return OK;
}

/*
 * Try to fill the buffer of "reader".
 * Returns FALSE when nothing was added.
 */
    static int
channel_fill(js_read_T *reader)
{
    channel_T	*channel = (channel_T *)reader->js_cookie;
    ch_part_T	part = reader->js_cookie_arg;
    char_u	*next = channel_get(channel, part, NULL);
    int		keeplen;
    int		addlen;
    char_u	*p;

    if (next == NULL)
	return FALSE;

    keeplen = reader->js_end - reader->js_buf;
    if (keeplen > 0)
    {
	// Prepend unused text.
	addlen = (int)STRLEN(next);
	p = alloc(keeplen + addlen + 1);
	if (p == NULL)
	{
	    vim_free(next);
	    return FALSE;
	}
	mch_memmove(p, reader->js_buf, keeplen);
	mch_memmove(p + keeplen, next, addlen + 1);
	vim_free(next);
	next = p;
    }

    vim_free(reader->js_buf);
    reader->js_buf = next;
    return TRUE;
}

/*
 * Process the HTTP header in a Language Server Protocol (LSP) message.
 *
 * The message format is described in the LSP specification:
 * https://microsoft.github.io/language-server-protocol/specification
 *
 * It has the following two fields:
 *
 *	Content-Length: ...
 *	Content-Type: application/vscode-jsonrpc; charset=utf-8
 *
 * Each field ends with "\r\n". The header ends with an additional "\r\n".
 *
 * Returns OK if a valid header is received and FAIL if some fields in the
 * header are not correct. Returns MAYBE if a partial header is received and
 * need to wait for more data to arrive.
 */
    static int
channel_process_lsp_http_hdr(js_read_T *reader)
{
    char_u	*line_start;
    char_u	*p;
    int_u	hdr_len;
    int		payload_len = -1;
    int_u	jsbuf_len;

    // We find the end once, to avoid calling strlen() many times.
    jsbuf_len = (int_u)STRLEN(reader->js_buf);
    reader->js_end = reader->js_buf + jsbuf_len;

    p = reader->js_buf;

    // Process each line in the header till an empty line is read (header
    // separator).
    while (TRUE)
    {
	line_start = p;
	while (*p != NUL && *p != '\n')
	    p++;
	if (*p == NUL)			// partial header
	    return MAYBE;
	p++;

	// process the content length field (if present)
	if ((p - line_start > 16)
		&& STRNICMP(line_start, "Content-Length: ", 16) == 0)
	{
	    errno = 0;
	    payload_len = strtol((char *)line_start + 16, NULL, 10);
	    if (errno == ERANGE || payload_len < 0)
		// invalid length, discard the payload
		return FAIL;
	}

	if ((p - line_start) == 2 && line_start[0] == '\r' &&
		line_start[1] == '\n')
	    // reached the empty line
	    break;
    }

    if (payload_len == -1)
	// Content-Length field is not present in the header
	return FAIL;

    hdr_len = p - reader->js_buf;

    // if the entire payload is not received, wait for more data to arrive
    if (jsbuf_len < hdr_len + payload_len)
	return MAYBE;

    reader->js_used += hdr_len;
    // recalculate the end based on the length read from the header.
    reader->js_end = reader->js_buf + hdr_len + payload_len;

    return OK;
}

/*
 * Use the read buffer of "channel"/"part" and parse a JSON message that is
 * complete.  The messages are added to the queue.
 * Return TRUE if there is more to read.
 */
    static int
channel_parse_json(channel_T *channel, ch_part_T part)
{
    js_read_T	reader;
    typval_T	listtv;
    jsonq_T	*item;
    chanpart_T	*chanpart = &channel->ch_part[part];
    jsonq_T	*head = &chanpart->ch_json_head;
    int		status = OK;
    int		ret;

    if (channel_peek(channel, part) == NULL)
	return FALSE;

    reader.js_buf = channel_get(channel, part, NULL);
    reader.js_used = 0;
    reader.js_fill = channel_fill;
    reader.js_cookie = channel;
    reader.js_cookie_arg = part;

    if (chanpart->ch_mode == CH_MODE_LSP)
	status = channel_process_lsp_http_hdr(&reader);

    // When a message is incomplete we wait for a short while for more to
    // arrive.  After the delay drop the input, otherwise a truncated string
    // or list will make us hang.
    // Do not generate error messages, they will be written in a channel log.
    if (status == OK)
    {
	++emsg_silent;
	status = json_decode(&reader, &listtv,
				chanpart->ch_mode == CH_MODE_JS ? JSON_JS : 0);
	--emsg_silent;
    }
    if (status == OK)
    {
	// Only accept the response when it is a list with at least two
	// items.
	if (chanpart->ch_mode == CH_MODE_LSP && listtv.v_type != VAR_DICT)
	{
	    ch_error(channel, "Did not receive a LSP dict, discarding");
	    clear_tv(&listtv);
	}
	else if (chanpart->ch_mode != CH_MODE_LSP
	      && (listtv.v_type != VAR_LIST || listtv.vval.v_list->lv_len < 2))
	{
	    if (listtv.v_type != VAR_LIST)
		ch_error(channel, "Did not receive a list, discarding");
	    else
		ch_error(channel, "Expected list with two items, got %d",
						  listtv.vval.v_list->lv_len);
	    clear_tv(&listtv);
	}
	else
	{
	    item = ALLOC_ONE(jsonq_T);
	    if (item == NULL)
		clear_tv(&listtv);
	    else
	    {
		item->jq_no_callback = FALSE;
		item->jq_value = alloc_tv();
		if (item->jq_value == NULL)
		{
		    vim_free(item);
		    clear_tv(&listtv);
		}
		else
		{
		    *item->jq_value = listtv;
		    item->jq_prev = head->jq_prev;
		    head->jq_prev = item;
		    item->jq_next = NULL;
		    if (item->jq_prev == NULL)
			head->jq_next = item;
		    else
			item->jq_prev->jq_next = item;
		}
	    }
	}
    }

    if (status == OK)
	chanpart->ch_wait_len = 0;
    else if (status == MAYBE)
    {
	size_t buflen = STRLEN(reader.js_buf);

	if (chanpart->ch_wait_len < buflen)
	{
	    // First time encountering incomplete message or after receiving
	    // more (but still incomplete): set a deadline of 100 msec.
	    ch_log(channel,
		    "Incomplete message (%d bytes) - wait 100 msec for more",
		    (int)buflen);
	    reader.js_used = 0;
	    chanpart->ch_wait_len = buflen;
#ifdef MSWIN
	    chanpart->ch_deadline = GetTickCount() + 100L;
#else
	    gettimeofday(&chanpart->ch_deadline, NULL);
	    chanpart->ch_deadline.tv_usec += 100 * 1000;
	    if (chanpart->ch_deadline.tv_usec > 1000 * 1000)
	    {
		chanpart->ch_deadline.tv_usec -= 1000 * 1000;
		++chanpart->ch_deadline.tv_sec;
	    }
#endif
	}
	else
	{
	    int timeout;
#ifdef MSWIN
	    timeout = GetTickCount() > chanpart->ch_deadline;
#else
	    {
		struct timeval now_tv;

		gettimeofday(&now_tv, NULL);
		timeout = now_tv.tv_sec > chanpart->ch_deadline.tv_sec
		      || (now_tv.tv_sec == chanpart->ch_deadline.tv_sec
			   && now_tv.tv_usec > chanpart->ch_deadline.tv_usec);
	    }
#endif
	    if (timeout)
	    {
		status = FAIL;
		chanpart->ch_wait_len = 0;
		ch_log(channel, "timed out");
	    }
	    else
	    {
		reader.js_used = 0;
		ch_log(channel, "still waiting on incomplete message");
	    }
	}
    }

    if (status == FAIL)
    {
	ch_error(channel, "Decoding failed - discarding input");
	ret = FALSE;
	chanpart->ch_wait_len = 0;
    }
    else if (reader.js_buf[reader.js_used] != NUL)
    {
	// Put the unread part back into the channel.
	channel_save(channel, part, reader.js_buf + reader.js_used,
			(int)(reader.js_end - reader.js_buf) - reader.js_used,
								  TRUE, NULL);
	ret = status == MAYBE ? FALSE: TRUE;
    }
    else
	ret = FALSE;

    vim_free(reader.js_buf);
    return ret;
}

/*
 * Remove "node" from the queue that it is in.  Does not free it.
 */
    static void
remove_cb_node(cbq_T *head, cbq_T *node)
{
    if (node->cq_prev == NULL)
	head->cq_next = node->cq_next;
    else
	node->cq_prev->cq_next = node->cq_next;
    if (node->cq_next == NULL)
	head->cq_prev = node->cq_prev;
    else
	node->cq_next->cq_prev = node->cq_prev;
}

/*
 * Remove "node" from the queue that it is in and free it.
 * Caller should have freed or used node->jq_value.
 */
    static void
remove_json_node(jsonq_T *head, jsonq_T *node)
{
    if (node->jq_prev == NULL)
	head->jq_next = node->jq_next;
    else
	node->jq_prev->jq_next = node->jq_next;
    if (node->jq_next == NULL)
	head->jq_prev = node->jq_prev;
    else
	node->jq_next->jq_prev = node->jq_prev;
    vim_free(node);
}

/*
 * Add "id" to the list of JSON message IDs we are waiting on.
 */
    static void
channel_add_block_id(chanpart_T *chanpart, int id)
{
    garray_T *gap = &chanpart->ch_block_ids;

    if (gap->ga_growsize == 0)
	ga_init2(gap, sizeof(int), 10);
    if (ga_grow(gap, 1) == OK)
    {
	((int *)gap->ga_data)[gap->ga_len] = id;
	++gap->ga_len;
    }
}

/*
 * Remove "id" from the list of JSON message IDs we are waiting on.
 */
    static void
channel_remove_block_id(chanpart_T *chanpart, int id)
{
    garray_T	*gap = &chanpart->ch_block_ids;
    int		i;

    for (i = 0; i < gap->ga_len; ++i)
	if (((int *)gap->ga_data)[i] == id)
	{
	    --gap->ga_len;
	    if (i < gap->ga_len)
	    {
		int *p = ((int *)gap->ga_data) + i;

		mch_memmove(p, p + 1, (gap->ga_len - i) * sizeof(int));
	    }
	    return;
	}
    siemsg("channel_remove_block_id(): cannot find id %d", id);
}

/*
 * Return TRUE if "id" is in the list of JSON message IDs we are waiting on.
 */
    static int
channel_has_block_id(chanpart_T *chanpart, int id)
{
    garray_T	*gap = &chanpart->ch_block_ids;
    int		i;

    for (i = 0; i < gap->ga_len; ++i)
	if (((int *)gap->ga_data)[i] == id)
	    return TRUE;
    return FALSE;
}

/*
 * Get a message from the JSON queue for channel "channel".
 * When "id" is positive it must match the first number in the list.
 * When "id" is zero or negative jut get the first message.  But not one
 * in the ch_block_ids list.
 * When "without_callback" is TRUE also get messages that were pushed back.
 * Return OK when found and return the value in "rettv".
 * Return FAIL otherwise.
 */
    static int
channel_get_json(
	channel_T   *channel,
	ch_part_T   part,
	int	    id,
	int	    without_callback,
	typval_T    **rettv)
{
    jsonq_T   *head = &channel->ch_part[part].ch_json_head;
    jsonq_T   *item = head->jq_next;

    while (item != NULL)
    {
	list_T	    *l;
	typval_T    *tv;

	if (channel->ch_part[part].ch_mode != CH_MODE_LSP)
	{
	    l = item->jq_value->vval.v_list;
	    CHECK_LIST_MATERIALIZE(l);
	    tv = &l->lv_first->li_tv;
	}
	else
	{
	    dict_T	*d;
	    dictitem_T	*di;

	    // LSP message payload is a JSON-RPC dict.
	    // For RPC requests and responses, the 'id' item will be present.
	    // For notifications, it will not be present.
	    if (id > 0)
	    {
		if (item->jq_value->v_type != VAR_DICT)
		    goto nextitem;
		d = item->jq_value->vval.v_dict;
		if (d == NULL)
		    goto nextitem;
		// When looking for a response message from the LSP server,
		// ignore new LSP request and notification messages.  LSP
		// request and notification messages have the "method" field in
		// the header and the response messages do not have this field.
		if (dict_has_key(d, "method"))
		    goto nextitem;
		di = dict_find(d, (char_u *)"id", -1);
		if (di == NULL)
		    goto nextitem;
		tv = &di->di_tv;
	    }
	    else
		tv = item->jq_value;
	}

	if ((without_callback || !item->jq_no_callback)
	    && ((id > 0 && tv->v_type == VAR_NUMBER && tv->vval.v_number == id)
	      || (id <= 0 && (tv->v_type != VAR_NUMBER
		 || tv->vval.v_number == 0
		 || !channel_has_block_id(
				&channel->ch_part[part], tv->vval.v_number)))))
	{
	    *rettv = item->jq_value;
	    if (tv->v_type == VAR_NUMBER)
		ch_log(channel, "Getting JSON message %ld",
						      (long)tv->vval.v_number);
	    remove_json_node(head, item);
	    return OK;
	}
nextitem:
	item = item->jq_next;
    }
    return FAIL;
}

/*
 * Put back "rettv" into the JSON queue, there was no callback for it.
 * Takes over the values in "rettv".
 */
    static void
channel_push_json(channel_T *channel, ch_part_T part, typval_T *rettv)
{
    jsonq_T   *head = &channel->ch_part[part].ch_json_head;
    jsonq_T   *item = head->jq_next;
    jsonq_T   *newitem;

    if (head->jq_prev != NULL && head->jq_prev->jq_no_callback)
	// last item was pushed back, append to the end
	item = NULL;
    else while (item != NULL && item->jq_no_callback)
	// append after the last item that was pushed back
	item = item->jq_next;

    newitem = ALLOC_ONE(jsonq_T);
    if (newitem == NULL)
    {
	clear_tv(rettv);
	return;
    }

    newitem->jq_value = alloc_tv();
    if (newitem->jq_value == NULL)
    {
	vim_free(newitem);
	clear_tv(rettv);
	return;
    }

    newitem->jq_no_callback = FALSE;
    *newitem->jq_value = *rettv;
    if (item == NULL)
    {
	// append to the end
	newitem->jq_prev = head->jq_prev;
	head->jq_prev = newitem;
	newitem->jq_next = NULL;
	if (newitem->jq_prev == NULL)
	    head->jq_next = newitem;
	else
	    newitem->jq_prev->jq_next = newitem;
    }
    else
    {
	// append after "item"
	newitem->jq_prev = item;
	newitem->jq_next = item->jq_next;
	item->jq_next = newitem;
	if (newitem->jq_next == NULL)
	    head->jq_prev = newitem;
	else
	    newitem->jq_next->jq_prev = newitem;
    }
}

#define CH_JSON_MAX_ARGS 4

/*
 * Execute a command received over "channel"/"part"
 * "argv[0]" is the command string.
 * "argv[1]" etc. have further arguments, type is VAR_UNKNOWN if missing.
 */
    static void
channel_exe_cmd(channel_T *channel, ch_part_T part, typval_T *argv)
{
    char_u  *cmd = argv[0].vval.v_string;
    char_u  *arg;
    int	    options = channel->ch_part[part].ch_mode == CH_MODE_JS
								 ? JSON_JS : 0;

    if (argv[1].v_type != VAR_STRING)
    {
	ch_error(channel, "received command with non-string argument");
	if (p_verbose > 2)
	    emsg(_(e_received_command_with_non_string_argument));
	return;
    }
    arg = argv[1].vval.v_string;
    if (arg == NULL)
	arg = (char_u *)"";

    if (STRCMP(cmd, "ex") == 0)
    {
	int	called_emsg_before = called_emsg;
	char_u	*p = arg;
	int	do_emsg_silent;

	ch_log(channel, "Executing ex command '%s'", (char *)arg);
	do_emsg_silent = !checkforcmd(&p, "echoerr", 5);
	if (do_emsg_silent)
	    ++emsg_silent;
	do_cmdline_cmd(arg);
	if (do_emsg_silent)
	    --emsg_silent;
	if (called_emsg > called_emsg_before)
	    ch_log(channel, "Ex command error: '%s'",
					  (char *)get_vim_var_str(VV_ERRMSG));
    }
    else if (STRCMP(cmd, "normal") == 0)
    {
	exarg_T ea;

	ch_log(channel, "Executing normal command '%s'", (char *)arg);
	CLEAR_FIELD(ea);
	ea.arg = arg;
	ea.addr_count = 0;
	ea.forceit = TRUE; // no mapping
	ex_normal(&ea);
    }
    else if (STRCMP(cmd, "redraw") == 0)
    {
	ch_log(channel, "redraw");
	redraw_cmd(*arg != NUL);
	showruler(FALSE);
	setcursor();
	out_flush_cursor(TRUE, FALSE);
    }
    else if (STRCMP(cmd, "expr") == 0 || STRCMP(cmd, "call") == 0)
    {
	int is_call = cmd[0] == 'c';
	int id_idx = is_call ? 3 : 2;

	if (argv[id_idx].v_type != VAR_UNKNOWN
					 && argv[id_idx].v_type != VAR_NUMBER)
	{
	    ch_error(channel, "last argument for expr/call must be a number");
	    if (p_verbose > 2)
		emsg(_(e_last_argument_for_expr_call_must_be_number));
	}
	else if (is_call && argv[2].v_type != VAR_LIST)
	{
	    ch_error(channel, "third argument for call must be a list");
	    if (p_verbose > 2)
		emsg(_(e_third_argument_for_call_must_be_list));
	}
	else
	{
	    typval_T	*tv = NULL;
	    typval_T	res_tv;
	    typval_T	err_tv;
	    char_u	*json = NULL;

	    // Don't pollute the display with errors.
	    // Do generate the errors so that try/catch works.
	    ++emsg_silent;
	    if (!is_call)
	    {
		ch_log(channel, "Evaluating expression '%s'", (char *)arg);
		tv = eval_expr(arg, NULL);
	    }
	    else
	    {
		ch_log(channel, "Calling '%s'", (char *)arg);
		if (func_call(arg, &argv[2], NULL, NULL, &res_tv) == OK)
		    tv = &res_tv;
	    }

	    if (argv[id_idx].v_type == VAR_NUMBER)
	    {
		int id = argv[id_idx].vval.v_number;

		if (tv != NULL)
		    json = json_encode_nr_expr(id, tv, options | JSON_NL);
		if (tv == NULL || (json != NULL && *json == NUL))
		{
		    // If evaluation failed or the result can't be encoded
		    // then return the string "ERROR".
		    vim_free(json);
		    err_tv.v_type = VAR_STRING;
		    err_tv.vval.v_string = (char_u *)"ERROR";
		    json = json_encode_nr_expr(id, &err_tv, options | JSON_NL);
		}
		if (json != NULL)
		{
		    channel_send(channel,
				 part == PART_SOCK ? PART_SOCK : PART_IN,
				 json, (int)STRLEN(json), (char *)cmd);
		    vim_free(json);
		}
	    }
	    --emsg_silent;
	    if (tv == &res_tv)
		clear_tv(tv);
	    else
		free_tv(tv);
	}
    }
    else if (p_verbose > 2)
    {
	ch_error(channel, "Received unknown command: %s", (char *)cmd);
	semsg(_(e_received_unknown_command_str), cmd);
    }
}

/*
 * Invoke the callback at "cbhead".
 * Does not redraw but sets channel_need_redraw.
 */
    static void
invoke_one_time_callback(
	channel_T   *channel,
	cbq_T	    *cbhead,
	cbq_T	    *item,
	typval_T    *argv)
{
    ch_log(channel, "Invoking one-time callback %s",
					    (char *)item->cq_callback.cb_name);
    // Remove the item from the list first, if the callback
    // invokes ch_close() the list will be cleared.
    remove_cb_node(cbhead, item);
    invoke_callback(channel, &item->cq_callback, argv);
    free_callback(&item->cq_callback);
    vim_free(item);
}

    static void
append_to_buffer(buf_T *buffer, char_u *msg, channel_T *channel, ch_part_T part)
{
    aco_save_T	aco;
    linenr_T    lnum = buffer->b_ml.ml_line_count;
    int		save_write_to = buffer->b_write_to_channel;
    chanpart_T  *ch_part = &channel->ch_part[part];
    int		save_p_ma = buffer->b_p_ma;
    int		empty = (buffer->b_ml.ml_flags & ML_EMPTY) ? 1 : 0;

    if (!buffer->b_p_ma && !ch_part->ch_nomodifiable)
    {
	if (!ch_part->ch_nomod_error)
	{
	    ch_error(channel, "Buffer is not modifiable, cannot append");
	    ch_part->ch_nomod_error = TRUE;
	}
	return;
    }

    // If the buffer is also used as input insert above the last
    // line. Don't write these lines.
    if (save_write_to)
    {
	--lnum;
	buffer->b_write_to_channel = FALSE;
    }

    // Append to the buffer
    ch_log(channel, "appending line %d to buffer %s",
				       (int)lnum + 1 - empty, buffer->b_fname);

    buffer->b_p_ma = TRUE;

    // Set curbuf to "buffer", temporarily.
    aucmd_prepbuf(&aco, buffer);
    if (curbuf != buffer)
    {
	// Could not find a window for this buffer, the following might cause
	// trouble, better bail out.
	return;
    }

    u_sync(TRUE);
    // ignore undo failure, undo is not very useful here
    vim_ignored = u_save(lnum - empty, lnum + 1);

    if (empty)
    {
	// The buffer is empty, replace the first (dummy) line.
	ml_replace(lnum, msg, TRUE);
	lnum = 0;
    }
    else
	ml_append(lnum, msg, 0, FALSE);
    appended_lines_mark(lnum, 1L);

    // reset notion of buffer
    aucmd_restbuf(&aco);

    if (ch_part->ch_nomodifiable)
	buffer->b_p_ma = FALSE;
    else
	buffer->b_p_ma = save_p_ma;

    if (buffer->b_nwindows > 0)
    {
	win_T	*wp;

	FOR_ALL_WINDOWS(wp)
	{
	    if (wp->w_buffer == buffer)
	    {
		int move_cursor = save_write_to
			    ? wp->w_cursor.lnum == lnum + 1
			    : (wp->w_cursor.lnum == lnum
				&& wp->w_cursor.col == 0);

		// If the cursor is at or above the new line, move it one line
		// down.  If the topline is outdated update it now.
		if (move_cursor || wp->w_topline > buffer->b_ml.ml_line_count)
		{
		    win_T *save_curwin = curwin;

		    if (move_cursor)
			++wp->w_cursor.lnum;
		    curwin = wp;
		    curbuf = curwin->w_buffer;
		    scroll_cursor_bot(0, FALSE);
		    curwin = save_curwin;
		    curbuf = curwin->w_buffer;
		}
	    }
	}
	redraw_buf_and_status_later(buffer, UPD_VALID);
	channel_need_redraw = TRUE;
    }

    if (save_write_to)
    {
	channel_T *ch;

	// Find channels reading from this buffer and adjust their
	// next-to-read line number.
	buffer->b_write_to_channel = TRUE;
	FOR_ALL_CHANNELS(ch)
	{
	    chanpart_T  *in_part = &ch->ch_part[PART_IN];

	    if (in_part->ch_bufref.br_buf == buffer)
		in_part->ch_buf_bot = buffer->b_ml.ml_line_count;
	}
    }
}

    static void
drop_messages(channel_T *channel, ch_part_T part)
{
    char_u *msg;

    while ((msg = channel_get(channel, part, NULL)) != NULL)
    {
	ch_log(channel, "Dropping message '%s'", (char *)msg);
	vim_free(msg);
    }
}

/*
 * Return TRUE if for "channel" / "part" ch_json_head should be used.
 */
    static int
channel_use_json_head(channel_T *channel, ch_part_T part)
{
    ch_mode_T	ch_mode = channel->ch_part[part].ch_mode;

    return ch_mode == CH_MODE_JSON || ch_mode == CH_MODE_JS
						     || ch_mode == CH_MODE_LSP;
}

/*
 * Invoke a callback for "channel"/"part" if needed.
 * This does not redraw but sets channel_need_redraw when redraw is needed.
 * Return TRUE when a message was handled, there might be another one.
 */
    static int
may_invoke_callback(channel_T *channel, ch_part_T part)
{
    char_u	*msg = NULL;
    typval_T	*listtv = NULL;
    typval_T	argv[CH_JSON_MAX_ARGS];
    int		seq_nr = -1;
    chanpart_T	*ch_part = &channel->ch_part[part];
    ch_mode_T	ch_mode = ch_part->ch_mode;
    cbq_T	*cbhead = &ch_part->ch_cb_head;
    cbq_T	*cbitem;
    callback_T	*callback = NULL;
    buf_T	*buffer = NULL;
    char_u	*p;
    int		called_otc;		// one time callbackup

    if (channel->ch_nb_close_cb != NULL)
	// this channel is handled elsewhere (netbeans)
	return FALSE;

    // Use a message-specific callback, part callback or channel callback
    for (cbitem = cbhead->cq_next; cbitem != NULL; cbitem = cbitem->cq_next)
	if (cbitem->cq_seq_nr == 0)
	    break;
    if (cbitem != NULL)
	callback = &cbitem->cq_callback;
    else if (ch_part->ch_callback.cb_name != NULL)
	callback = &ch_part->ch_callback;
    else if (channel->ch_callback.cb_name != NULL)
	callback = &channel->ch_callback;

    buffer = ch_part->ch_bufref.br_buf;
    if (buffer != NULL && (!bufref_valid(&ch_part->ch_bufref)
					       || buffer->b_ml.ml_mfp == NULL))
    {
	// buffer was wiped out or unloaded
	ch_log(channel, "%s buffer has been wiped out", ch_part_names[part]);
	ch_part->ch_bufref.br_buf = NULL;
	buffer = NULL;
    }

    if (channel_use_json_head(channel, part))
    {
	listitem_T	*item;
	int		argc = 0;

	// Get any json message in the queue.
	if (channel_get_json(channel, part, -1, FALSE, &listtv) == FAIL)
	{
	    if (ch_mode == CH_MODE_LSP)
		// In the "lsp" mode, the http header and the json payload may
		// be received in multiple messages. So concatenate all the
		// received messages.
		(void)channel_collapse(channel, part, FALSE);

	    // Parse readahead, return when there is still no message.
	    channel_parse_json(channel, part);
	    if (channel_get_json(channel, part, -1, FALSE, &listtv) == FAIL)
		return FALSE;
	}

	if (ch_mode == CH_MODE_LSP)
	{
	    dict_T	*d = listtv->vval.v_dict;
	    dictitem_T	*di;

	    seq_nr = 0;
	    if (d != NULL)
	    {
		di = dict_find(d, (char_u *)"id", -1);
		if (di != NULL && di->di_tv.v_type == VAR_NUMBER)
		    seq_nr = di->di_tv.vval.v_number;
	    }

	    argv[1] = *listtv;
	}
	else
	{
	    for (item = listtv->vval.v_list->lv_first;
		    item != NULL && argc < CH_JSON_MAX_ARGS;
		    item = item->li_next)
		argv[argc++] = item->li_tv;
	    while (argc < CH_JSON_MAX_ARGS)
		argv[argc++].v_type = VAR_UNKNOWN;

	    if (argv[0].v_type == VAR_STRING)
	    {
		// ["cmd", arg] or ["cmd", arg, arg] or ["cmd", arg, arg, arg]
		channel_exe_cmd(channel, part, argv);
		free_tv(listtv);
		return TRUE;
	    }

	    if (argv[0].v_type != VAR_NUMBER)
	    {
		ch_error(channel,
			"Dropping message with invalid sequence number type");
		free_tv(listtv);
		return FALSE;
	    }
	    seq_nr = argv[0].vval.v_number;
	}
    }
    else if (channel_peek(channel, part) == NULL)
    {
	// nothing to read on RAW or NL channel
	return FALSE;
    }
    else
    {
	// If there is no callback or buffer drop the message.
	if (callback == NULL && buffer == NULL)
	{
	    // If there is a close callback it may use ch_read() to get the
	    // messages.
	    if (channel->ch_close_cb.cb_name == NULL && !channel->ch_drop_never)
		drop_messages(channel, part);
	    return FALSE;
	}

	if (ch_mode == CH_MODE_NL)
	{
	    char_u  *nl = NULL;
	    char_u  *buf;
	    readq_T *node;

	    // See if we have a message ending in NL in the first buffer.  If
	    // not try to concatenate the first and the second buffer.
	    while (TRUE)
	    {
		node = channel_peek(channel, part);
		nl = channel_first_nl(node);
		if (nl != NULL)
		    break;
		if (channel_collapse(channel, part, TRUE) == FAIL)
		{
		    if (ch_part->ch_fd == INVALID_FD && node->rq_buflen > 0)
			break;
		    return FALSE; // incomplete message
		}
	    }
	    buf = node->rq_buffer;

	    // Convert NUL to NL, the internal representation.
	    for (p = buf; (nl == NULL || p < nl)
					    && p < buf + node->rq_buflen; ++p)
		if (*p == NUL)
		    *p = NL;

	    if (nl == NULL)
	    {
		// get the whole buffer, drop the NL
		msg = channel_get(channel, part, NULL);
	    }
	    else if (nl + 1 == buf + node->rq_buflen)
	    {
		// get the whole buffer
		msg = channel_get(channel, part, NULL);
		*nl = NUL;
	    }
	    else
	    {
		// Copy the message into allocated memory (excluding the NL)
		// and remove it from the buffer (including the NL).
		msg = vim_strnsave(buf, nl - buf);
		channel_consume(channel, part, (int)(nl - buf) + 1);
	    }
	}
	else
	{
	    // For a raw channel we don't know where the message ends, just
	    // get everything we have.
	    // Convert NUL to NL, the internal representation.
	    msg = channel_get_all(channel, part, NULL);
	}

	if (msg == NULL)
	    return FALSE; // out of memory (and avoids Coverity warning)

	argv[1].v_type = VAR_STRING;
	argv[1].vval.v_string = msg;
    }

    called_otc = FALSE;
    if (seq_nr > 0)
    {
	// JSON or JS or LSP mode: invoke the one-time callback with the
	// matching nr
	int lsp_req_msg = FALSE;

	// Don't use a LSP server request message with the same sequence number
	// as the client request message as the response message.
	if (ch_mode == CH_MODE_LSP && argv[1].v_type == VAR_DICT
		&& dict_has_key(argv[1].vval.v_dict, "method"))
	    lsp_req_msg = TRUE;

	if (!lsp_req_msg)
	{
	    for (cbitem = cbhead->cq_next; cbitem != NULL;
		    cbitem = cbitem->cq_next)
	    {
		if (cbitem->cq_seq_nr == seq_nr)
		{
		    invoke_one_time_callback(channel, cbhead, cbitem, argv);
		    called_otc = TRUE;
		    break;
		}
	    }
	}
    }

    if (seq_nr > 0 && (ch_mode != CH_MODE_LSP || called_otc))
    {
	if (!called_otc)
	{
	    // If the 'drop' channel attribute is set to 'never' or if
	    // ch_evalexpr() is waiting for this response message, then don't
	    // drop this message.
	    if (channel->ch_drop_never)
	    {
		// message must be read with ch_read()
		channel_push_json(channel, part, listtv);

		// Change the type to avoid the value being freed.
		listtv->v_type = VAR_NUMBER;
		free_tv(listtv);
		listtv = NULL;
	    }
	    else
		ch_log(channel, "Dropping message %d without callback",
								       seq_nr);
	}
    }
    else if (callback != NULL || buffer != NULL)
    {
	if (buffer != NULL)
	{
	    if (msg == NULL)
		// JSON or JS mode: re-encode the message.
		msg = json_encode(listtv, ch_mode);
	    if (msg != NULL)
	    {
#ifdef FEAT_TERMINAL
		if (buffer->b_term != NULL)
		    write_to_term(buffer, msg, channel);
		else
#endif
		    append_to_buffer(buffer, msg, channel, part);
	    }
	}

	if (callback != NULL)
	{
	    if (cbitem != NULL)
		invoke_one_time_callback(channel, cbhead, cbitem, argv);
	    else
	    {
		// invoke the channel callback
		ch_log(channel, "Invoking channel callback %s",
						    (char *)callback->cb_name);
		invoke_callback(channel, callback, argv);
	    }
	}
    }
    else
	ch_log(channel, "Dropping message %d", seq_nr);

    if (listtv != NULL)
	free_tv(listtv);
    vim_free(msg);

    return TRUE;
}

#if defined(FEAT_NETBEANS_INTG) || defined(PROTO)
/*
 * Return TRUE when channel "channel" is open for writing to.
 * Also returns FALSE or invalid "channel".
 */
    int
channel_can_write_to(channel_T *channel)
{
    return channel != NULL && (channel->CH_SOCK_FD != INVALID_FD
			  || channel->CH_IN_FD != INVALID_FD);
}
#endif

/*
 * Return TRUE when channel "channel" is open for reading or writing.
 * Also returns FALSE for invalid "channel".
 */
    int
channel_is_open(channel_T *channel)
{
    return channel != NULL && (channel->CH_SOCK_FD != INVALID_FD
			  || channel->CH_IN_FD != INVALID_FD
			  || channel->CH_OUT_FD != INVALID_FD
			  || channel->CH_ERR_FD != INVALID_FD);
}

/*
 * Return a pointer indicating the readahead.  Can only be compared between
 * calls.  Returns NULL if there is no readahead.
 */
    static void *
channel_readahead_pointer(channel_T *channel, ch_part_T part)
{
    if (channel_use_json_head(channel, part))
    {
	jsonq_T   *head = &channel->ch_part[part].ch_json_head;

	if (head->jq_next == NULL)
	    // Parse json from readahead, there might be a complete message to
	    // process.
	    channel_parse_json(channel, part);

	return head->jq_next;
    }
    return channel_peek(channel, part);
}

/*
 * Return TRUE if "channel" has JSON or other typeahead.
 */
    static int
channel_has_readahead(channel_T *channel, ch_part_T part)
{
    return channel_readahead_pointer(channel, part) != NULL;
}

/*
 * Return a string indicating the status of the channel.
 * If "req_part" is not negative check that part.
 */
    static char *
channel_status(channel_T *channel, int req_part)
{
    ch_part_T part;
    int has_readahead = FALSE;

    if (channel == NULL)
	 return "fail";
    if (req_part == PART_OUT)
    {
	if (channel->CH_OUT_FD != INVALID_FD)
	    return "open";
	if (channel_has_readahead(channel, PART_OUT))
	    has_readahead = TRUE;
    }
    else if (req_part == PART_ERR)
    {
	if (channel->CH_ERR_FD != INVALID_FD)
	    return "open";
	if (channel_has_readahead(channel, PART_ERR))
	    has_readahead = TRUE;
    }
    else
    {
	if (channel_is_open(channel))
	    return "open";
	for (part = PART_SOCK; part < PART_IN; ++part)
	    if (channel_has_readahead(channel, part))
	    {
		has_readahead = TRUE;
		break;
	    }
    }

    if (has_readahead)
	return "buffered";
    return "closed";
}

    static void
channel_part_info(channel_T *channel, dict_T *dict, char *name, ch_part_T part)
{
    chanpart_T *chanpart = &channel->ch_part[part];
    char	namebuf[20];  // longest is "sock_timeout"
    size_t	tail;
    char	*status;
    char	*s = "";

    vim_strncpy((char_u *)namebuf, (char_u *)name, 4);
    STRCAT(namebuf, "_");
    tail = STRLEN(namebuf);

    STRCPY(namebuf + tail, "status");
    if (chanpart->ch_fd != INVALID_FD)
	status = "open";
    else if (channel_has_readahead(channel, part))
	status = "buffered";
    else
	status = "closed";
    dict_add_string(dict, namebuf, (char_u *)status);

    STRCPY(namebuf + tail, "mode");
    switch (chanpart->ch_mode)
    {
	case CH_MODE_NL: s = "NL"; break;
	case CH_MODE_RAW: s = "RAW"; break;
	case CH_MODE_JSON: s = "JSON"; break;
	case CH_MODE_JS: s = "JS"; break;
	case CH_MODE_LSP: s = "LSP"; break;
    }
    dict_add_string(dict, namebuf, (char_u *)s);

    STRCPY(namebuf + tail, "io");
    if (part == PART_SOCK)
	s = "socket";
    else switch (chanpart->ch_io)
    {
	case JIO_NULL: s = "null"; break;
	case JIO_PIPE: s = "pipe"; break;
	case JIO_FILE: s = "file"; break;
	case JIO_BUFFER: s = "buffer"; break;
	case JIO_OUT: s = "out"; break;
    }
    dict_add_string(dict, namebuf, (char_u *)s);

    STRCPY(namebuf + tail, "timeout");
    dict_add_number(dict, namebuf, chanpart->ch_timeout);
}

    static void
channel_info(channel_T *channel, dict_T *dict)
{
    dict_add_number(dict, "id", channel->ch_id);
    dict_add_string(dict, "status", (char_u *)channel_status(channel, -1));

    if (channel->ch_hostname != NULL)
    {
	if (channel->ch_port)
	{
	    dict_add_string(dict, "hostname", (char_u *)channel->ch_hostname);
	    dict_add_number(dict, "port", channel->ch_port);
	}
	else
	    // Unix-domain socket.
	    dict_add_string(dict, "path", (char_u *)channel->ch_hostname);
	channel_part_info(channel, dict, "sock", PART_SOCK);
    }
    else
    {
	channel_part_info(channel, dict, "out", PART_OUT);
	channel_part_info(channel, dict, "err", PART_ERR);
	channel_part_info(channel, dict, "in", PART_IN);
    }
}

/*
 * Close channel "channel".
 * Trigger the close callback if "invoke_close_cb" is TRUE.
 * Does not clear the buffers.
 */
    void
channel_close(channel_T *channel, int invoke_close_cb)
{
    ch_log(channel, "Closing channel");

#ifdef FEAT_GUI
    channel_gui_unregister(channel);
#endif

    ch_close_part(channel, PART_SOCK);
    ch_close_part(channel, PART_IN);
    ch_close_part(channel, PART_OUT);
    ch_close_part(channel, PART_ERR);

    if (invoke_close_cb)
    {
	ch_part_T	part;

#ifdef FEAT_TERMINAL
	// let the terminal know it is closing to avoid getting stuck
	term_channel_closing(channel);
#endif
	// Invoke callbacks and flush buffers before the close callback.
	if (channel->ch_close_cb.cb_name != NULL)
	    ch_log(channel,
		     "Invoking callbacks and flushing buffers before closing");
	for (part = PART_SOCK; part < PART_IN; ++part)
	{
	    if (channel->ch_close_cb.cb_name != NULL
			    || channel->ch_part[part].ch_bufref.br_buf != NULL)
	    {
		// Increment the refcount to avoid the channel being freed
		// halfway.
		++channel->ch_refcount;
		if (channel->ch_close_cb.cb_name == NULL)
		    ch_log(channel, "flushing %s buffers before closing",
							  ch_part_names[part]);
		while (may_invoke_callback(channel, part))
		    ;
		--channel->ch_refcount;
	    }
	}

	if (channel->ch_close_cb.cb_name != NULL)
	{
	      typval_T	argv[1];
	      typval_T	rettv;

	      // Increment the refcount to avoid the channel being freed
	      // halfway.
	      ++channel->ch_refcount;
	      ch_log(channel, "Invoking close callback %s",
					 (char *)channel->ch_close_cb.cb_name);
	      argv[0].v_type = VAR_CHANNEL;
	      argv[0].vval.v_channel = channel;
	      call_callback(&channel->ch_close_cb, -1, &rettv, 1, argv);
	      clear_tv(&rettv);
	      channel_need_redraw = TRUE;

	      // the callback is only called once
	      free_callback(&channel->ch_close_cb);

	      if (channel_need_redraw)
	      {
		  channel_need_redraw = FALSE;
		  redraw_after_callback(TRUE, FALSE);
	      }

	      if (!channel->ch_drop_never)
		  // any remaining messages are useless now
		  for (part = PART_SOCK; part < PART_IN; ++part)
		      drop_messages(channel, part);

	      --channel->ch_refcount;
	}
    }

    channel->ch_nb_close_cb = NULL;

#ifdef FEAT_TERMINAL
    term_channel_closed(channel);
#endif
}

/*
 * Close the "in" part channel "channel".
 */
    static void
channel_close_in(channel_T *channel)
{
    ch_close_part(channel, PART_IN);
}

    static void
remove_from_writeque(writeq_T *wq, writeq_T *entry)
{
    ga_clear(&entry->wq_ga);
    wq->wq_next = entry->wq_next;
    if (wq->wq_next == NULL)
	wq->wq_prev = NULL;
    else
	wq->wq_next->wq_prev = NULL;
    vim_free(entry);
}

/*
 * Clear the read buffer on "channel"/"part".
 */
    static void
channel_clear_one(channel_T *channel, ch_part_T part)
{
    chanpart_T *ch_part = &channel->ch_part[part];
    jsonq_T *json_head = &ch_part->ch_json_head;
    cbq_T   *cb_head = &ch_part->ch_cb_head;

    while (channel_peek(channel, part) != NULL)
	vim_free(channel_get(channel, part, NULL));

    while (cb_head->cq_next != NULL)
    {
	cbq_T *node = cb_head->cq_next;

	remove_cb_node(cb_head, node);
	free_callback(&node->cq_callback);
	vim_free(node);
    }

    while (json_head->jq_next != NULL)
    {
	free_tv(json_head->jq_next->jq_value);
	remove_json_node(json_head, json_head->jq_next);
    }

    free_callback(&ch_part->ch_callback);
    ga_clear(&ch_part->ch_block_ids);

    while (ch_part->ch_writeque.wq_next != NULL)
	remove_from_writeque(&ch_part->ch_writeque,
						 ch_part->ch_writeque.wq_next);
}

/*
 * Clear all the read buffers on "channel".
 */
    void
channel_clear(channel_T *channel)
{
    ch_log(channel, "Clearing channel");
    VIM_CLEAR(channel->ch_hostname);
    channel_clear_one(channel, PART_SOCK);
    channel_clear_one(channel, PART_OUT);
    channel_clear_one(channel, PART_ERR);
    channel_clear_one(channel, PART_IN);
    free_callback(&channel->ch_callback);
    free_callback(&channel->ch_close_cb);
}

#if defined(EXITFREE) || defined(PROTO)
    void
channel_free_all(void)
{
    channel_T *channel;

    ch_log(NULL, "channel_free_all()");
    FOR_ALL_CHANNELS(channel)
	channel_clear(channel);
}
#endif


// Sent when the netbeans channel is found closed when reading.
#define DETACH_MSG_RAW "DETACH\n"

// Buffer size for reading incoming messages.
#define MAXMSGSIZE 4096

/*
 * Check if there are remaining data that should be written for "in_part".
 */
    static int
is_channel_write_remaining(chanpart_T *in_part)
{
    buf_T *buf = in_part->ch_bufref.br_buf;

    if (in_part->ch_writeque.wq_next != NULL)
	return TRUE;
    if (buf == NULL)
	return FALSE;
    return in_part->ch_buf_append
	    ? (in_part->ch_buf_bot < buf->b_ml.ml_line_count)
	    : (in_part->ch_buf_top <= in_part->ch_buf_bot
			    && in_part->ch_buf_top <= buf->b_ml.ml_line_count);
}

#if defined(HAVE_SELECT)
/*
 * Add write fds where we are waiting for writing to be possible.
 */
    static int
channel_fill_wfds(int maxfd_arg, fd_set *wfds)
{
    int		maxfd = maxfd_arg;
    channel_T	*ch;

    FOR_ALL_CHANNELS(ch)
    {
	chanpart_T  *in_part = &ch->ch_part[PART_IN];

	if (in_part->ch_fd != INVALID_FD
		&& is_channel_write_remaining(in_part))
	{
	    FD_SET((int)in_part->ch_fd, wfds);
	    if ((int)in_part->ch_fd >= maxfd)
		maxfd = (int)in_part->ch_fd + 1;
	}
    }
    return maxfd;
}
#else
/*
 * Add write fds where we are waiting for writing to be possible.
 */
    static int
channel_fill_poll_write(int nfd_in, struct pollfd *fds)
{
    int		nfd = nfd_in;
    channel_T	*ch;

    FOR_ALL_CHANNELS(ch)
    {
	chanpart_T  *in_part = &ch->ch_part[PART_IN];

	if (in_part->ch_fd != INVALID_FD
		&& is_channel_write_remaining(in_part))
	{
	    in_part->ch_poll_idx = nfd;
	    fds[nfd].fd = in_part->ch_fd;
	    fds[nfd].events = POLLOUT;
	    ++nfd;
	}
	else
	    in_part->ch_poll_idx = -1;
    }
    return nfd;
}
#endif

typedef enum {
    CW_READY,
    CW_NOT_READY,
    CW_ERROR
} channel_wait_result;

/*
 * Check for reading from "fd" with "timeout" msec.
 * Return CW_READY when there is something to read.
 * Return CW_NOT_READY when there is nothing to read.
 * Return CW_ERROR when there is an error.
 */
    static channel_wait_result
channel_wait(channel_T *channel, sock_T fd, int timeout)
{
    if (timeout > 0)
	ch_log(channel, "Waiting for up to %d msec", timeout);

# ifdef MSWIN
    if (fd != channel->CH_SOCK_FD)
    {
	DWORD	nread;
	int	sleep_time;
	DWORD	deadline = GetTickCount() + timeout;
	int	delay = 1;

	// reading from a pipe, not a socket
	while (TRUE)
	{
	    int r = PeekNamedPipe((HANDLE)fd, NULL, 0, NULL, &nread, NULL);

	    if (r && nread > 0)
		return CW_READY;

	    if (channel->ch_named_pipe)
	    {
		DisconnectNamedPipe((HANDLE)fd);
		ConnectNamedPipe((HANDLE)fd, NULL);
	    }
	    else if (r == 0)
		return CW_ERROR;

	    // perhaps write some buffer lines
	    channel_write_any_lines();

	    sleep_time = deadline - GetTickCount();
	    if (sleep_time <= 0)
		break;
	    // Wait for a little while.  Very short at first, up to 10 msec
	    // after looping a few times.
	    if (sleep_time > delay)
		sleep_time = delay;
	    Sleep(sleep_time);
	    delay = delay * 2;
	    if (delay > 10)
		delay = 10;
	}
    }
    else
#endif
    {
#if defined(HAVE_SELECT)
	struct timeval	tval;
	fd_set		rfds;
	fd_set		wfds;
	int		ret;
	int		maxfd;

	tval.tv_sec = timeout / 1000;
	tval.tv_usec = (timeout % 1000) * 1000;
	for (;;)
	{
	    FD_ZERO(&rfds);
	    FD_SET((int)fd, &rfds);

	    // Write lines to a pipe when a pipe can be written to.  Need to
	    // set this every time, some buffers may be done.
	    maxfd = (int)fd + 1;
	    FD_ZERO(&wfds);
	    maxfd = channel_fill_wfds(maxfd, &wfds);

	    ret = select(maxfd, &rfds, &wfds, NULL, &tval);
# ifdef EINTR
	    SOCK_ERRNO;
	    if (ret == -1 && errno == EINTR)
		continue;
# endif
	    if (ret > 0)
	    {
		if (FD_ISSET(fd, &rfds))
		    return CW_READY;
		channel_write_any_lines();
		continue;
	    }
	    break;
	}
#else
	for (;;)
	{
	    struct pollfd   fds[MAX_OPEN_CHANNELS + 1];
	    int		    nfd = 1;

	    fds[0].fd = fd;
	    fds[0].events = POLLIN;
	    nfd = channel_fill_poll_write(nfd, fds);
	    if (poll(fds, nfd, timeout) > 0)
	    {
		if (fds[0].revents & POLLIN)
		    return CW_READY;
		channel_write_any_lines();
		continue;
	    }
	    break;
	}
#endif
    }
    return CW_NOT_READY;
}

    static void
ch_close_part_on_error(
	channel_T *channel, ch_part_T part, int is_err, char *func)
{
    char	msg[] = "%s(): Read %s from ch_part[%d], closing";

    if (is_err)
	// Do not call emsg(), most likely the other end just exited.
	ch_error(channel, msg, func, "error", part);
    else
	ch_log(channel, msg, func, "EOF", part);

    // Queue a "DETACH" netbeans message in the command queue in order to
    // terminate the netbeans session later. Do not end the session here
    // directly as we may be running in the context of a call to
    // netbeans_parse_messages():
    //	netbeans_parse_messages
    //	    -> autocmd triggered while processing the netbeans cmd
    //		-> ui_breakcheck
    //		    -> gui event loop or select loop
    //			-> channel_read()
    // Only send "DETACH" for a netbeans channel.
    if (channel->ch_nb_close_cb != NULL)
	channel_save(channel, PART_SOCK, (char_u *)DETACH_MSG_RAW,
			      (int)STRLEN(DETACH_MSG_RAW), FALSE, "PUT ");

    // When reading is not possible close this part of the channel.  Don't
    // close the channel yet, there may be something to read on another part.
    // When stdout and stderr use the same FD we get the error only on one of
    // them, also close the other.
    if (part == PART_OUT || part == PART_ERR)
    {
	ch_part_T other = part == PART_OUT ? PART_ERR : PART_OUT;

	if (channel->ch_part[part].ch_fd == channel->ch_part[other].ch_fd)
	    ch_close_part(channel, other);
    }
    ch_close_part(channel, part);

#ifdef FEAT_GUI
    // Stop listening to GUI events right away.
    channel_gui_unregister_one(channel, part);
#endif
}

    static void
channel_close_now(channel_T *channel)
{
    ch_log(channel, "Closing channel because all readable fds are closed");
    if (channel->ch_nb_close_cb != NULL)
	(*channel->ch_nb_close_cb)();
    channel_close(channel, TRUE);
}

/*
 * Read from channel "channel" for as long as there is something to read.
 * "part" is PART_SOCK, PART_OUT or PART_ERR.
 * The data is put in the read queue.  No callbacks are invoked here.
 */
    static void
channel_read(channel_T *channel, ch_part_T part, char *func)
{
    static char_u	*buf = NULL;
    int			len = 0;
    int			readlen = 0;
    sock_T		fd;
    int			use_socket = FALSE;

    fd = channel->ch_part[part].ch_fd;
    if (fd == INVALID_FD)
    {
	ch_error(channel, "channel_read() called while %s part is closed",
							  ch_part_names[part]);
	return;
    }
    use_socket = fd == channel->CH_SOCK_FD;

    // Allocate a buffer to read into.
    if (buf == NULL)
    {
	buf = alloc(MAXMSGSIZE);
	if (buf == NULL)
	    return;	// out of memory!
    }

    // Keep on reading for as long as there is something to read.
    // Use select() or poll() to avoid blocking on a message that is exactly
    // MAXMSGSIZE long.
    for (;;)
    {
	if (channel_wait(channel, fd, 0) != CW_READY)
	    break;
	if (use_socket)
	    len = sock_read(fd, (char *)buf, MAXMSGSIZE);
	else
	    len = fd_read(fd, (char *)buf, MAXMSGSIZE);
	if (len <= 0)
	    break;	// error or nothing more to read

	// Store the read message in the queue.
	channel_save(channel, part, buf, len, FALSE, "RECV ");
	readlen += len;
    }

    // Reading a disconnection (readlen == 0), or an error.
    if (readlen <= 0)
    {
	if (!channel->ch_keep_open)
	    ch_close_part_on_error(channel, part, (len < 0), func);
    }
#if defined(CH_HAS_GUI) && defined(FEAT_GUI_GTK)
    else if (CH_HAS_GUI && gtk_main_level() > 0)
	// signal the main loop that there is something to read
	gtk_main_quit();
#endif
}

/*
 * Read from RAW or NL "channel"/"part".  Blocks until there is something to
 * read or the timeout expires.
 * When "raw" is TRUE don't block waiting on a NL.
 * Does not trigger timers or handle messages.
 * Returns what was read in allocated memory.
 * Returns NULL in case of error or timeout.
 */
    static char_u *
channel_read_block(
	channel_T *channel, ch_part_T part, int timeout, int raw, int *outlen)
{
    char_u	*buf;
    char_u	*msg;
    ch_mode_T	mode = channel->ch_part[part].ch_mode;
    sock_T	fd = channel->ch_part[part].ch_fd;
    char_u	*nl;
    readq_T	*node;

    ch_log(channel, "Blocking %s read, timeout: %d msec",
				  mode == CH_MODE_RAW ? "RAW" : "NL", timeout);

    while (TRUE)
    {
	node = channel_peek(channel, part);
	if (node != NULL)
	{
	    if (mode == CH_MODE_RAW || (mode == CH_MODE_NL
					   && channel_first_nl(node) != NULL))
		// got a complete message
		break;
	    if (channel_collapse(channel, part, mode == CH_MODE_NL) == OK)
		continue;
	    // If not blocking or nothing more is coming then return what we
	    // have.
	    if (raw || fd == INVALID_FD)
		break;
	}

	// Wait for up to the channel timeout.
	if (fd == INVALID_FD)
	    return NULL;
	if (channel_wait(channel, fd, timeout) != CW_READY)
	{
	    ch_log(channel, "Timed out");
	    return NULL;
	}
	channel_read(channel, part, "channel_read_block");
    }

    // We have a complete message now.
    if (mode == CH_MODE_RAW || outlen != NULL)
    {
	msg = channel_get_all(channel, part, outlen);
    }
    else
    {
	char_u *p;

	buf = node->rq_buffer;
	nl = channel_first_nl(node);

	// Convert NUL to NL, the internal representation.
	for (p = buf; (nl == NULL || p < nl) && p < buf + node->rq_buflen; ++p)
	    if (*p == NUL)
		*p = NL;

	if (nl == NULL)
	{
	    // must be a closed channel with missing NL
	    msg = channel_get(channel, part, NULL);
	}
	else if (nl + 1 == buf + node->rq_buflen)
	{
	    // get the whole buffer
	    msg = channel_get(channel, part, NULL);
	    *nl = NUL;
	}
	else
	{
	    // Copy the message into allocated memory and remove it from the
	    // buffer.
	    msg = vim_strnsave(buf, nl - buf);
	    channel_consume(channel, part, (int)(nl - buf) + 1);
	}
    }
    if (ch_log_active())
	ch_log(channel, "Returning %d bytes", (int)STRLEN(msg));
    return msg;
}

static int channel_blocking_wait = 0;

/*
 * Return TRUE if in a blocking wait that might trigger callbacks.
 */
    int
channel_in_blocking_wait(void)
{
    return channel_blocking_wait > 0;
}

/*
 * Read one JSON message with ID "id" from "channel"/"part" and store the
 * result in "rettv".
 * When "id" is -1 accept any message;
 * Blocks until the message is received or the timeout is reached.
 * In corner cases this can be called recursively, that is why ch_block_ids is
 * a list.
 */
    static int
channel_read_json_block(
	channel_T   *channel,
	ch_part_T   part,
	int	    timeout_arg,
	int	    id,
	typval_T    **rettv)
{
    int		more;
    sock_T	fd;
    int		timeout;
    chanpart_T	*chanpart = &channel->ch_part[part];
    ch_mode_T	mode = channel->ch_part[part].ch_mode;
    int		retval = FAIL;

    ch_log(channel, "Blocking read JSON for id %d", id);
    ++channel_blocking_wait;

    if (id >= 0)
	channel_add_block_id(chanpart, id);

    for (;;)
    {
	if (mode == CH_MODE_LSP)
	    // In the "lsp" mode, the http header and the json payload may be
	    // received in multiple messages. So concatenate all the received
	    // messages.
	    (void)channel_collapse(channel, part, FALSE);

	more = channel_parse_json(channel, part);

	// search for message "id"
	if (channel_get_json(channel, part, id, TRUE, rettv) == OK)
	{
	    ch_log(channel, "Received JSON for id %d", id);
	    retval = OK;
	    break;
	}

	if (!more)
	{
	    void *prev_readahead_ptr = channel_readahead_pointer(channel, part);
	    void *readahead_ptr;

	    // Handle any other messages in the queue.  If done some more
	    // messages may have arrived.
	    if (channel_parse_messages())
		continue;

	    // channel_parse_messages() may fill the queue with new data to
	    // process.  Only loop when the readahead changed, otherwise we
	    // would busy-loop.
	    readahead_ptr = channel_readahead_pointer(channel, part);
	    if (readahead_ptr != NULL && readahead_ptr != prev_readahead_ptr)
		continue;

	    // Wait for up to the timeout.  If there was an incomplete message
	    // use the deadline for that.
	    timeout = timeout_arg;
	    if (chanpart->ch_wait_len > 0)
	    {
#ifdef MSWIN
		timeout = chanpart->ch_deadline - GetTickCount() + 1;
#else
		{
		    struct timeval now_tv;

		    gettimeofday(&now_tv, NULL);
		    timeout = (chanpart->ch_deadline.tv_sec
						       - now_tv.tv_sec) * 1000
			+ (chanpart->ch_deadline.tv_usec
						     - now_tv.tv_usec) / 1000
			+ 1;
		}
#endif
		if (timeout < 0)
		{
		    // Something went wrong, channel_parse_json() didn't
		    // discard message.  Cancel waiting.
		    chanpart->ch_wait_len = 0;
		    timeout = timeout_arg;
		}
		else if (timeout > timeout_arg)
		    timeout = timeout_arg;
	    }
	    fd = chanpart->ch_fd;
	    if (fd == INVALID_FD
			    || channel_wait(channel, fd, timeout) != CW_READY)
	    {
		if (timeout == timeout_arg)
		{
		    if (fd != INVALID_FD)
			ch_log(channel, "Timed out on id %d", id);
		    break;
		}
	    }
	    else
		channel_read(channel, part, "channel_read_json_block");
	}
    }
    if (id >= 0)
	channel_remove_block_id(chanpart, id);
    --channel_blocking_wait;

    return retval;
}

/*
 * Get the channel from the argument.
 * Returns NULL if the handle is invalid.
 * When "check_open" is TRUE check that the channel can be used.
 * When "reading" is TRUE "check_open" considers typeahead useful.
 * "part" is used to check typeahead, when PART_COUNT use the default part.
 */
    channel_T *
get_channel_arg(typval_T *tv, int check_open, int reading, ch_part_T part)
{
    channel_T	*channel = NULL;
    int		has_readahead = FALSE;

    if (tv->v_type == VAR_JOB)
    {
	if (tv->vval.v_job != NULL)
	    channel = tv->vval.v_job->jv_channel;
    }
    else if (tv->v_type == VAR_CHANNEL)
    {
	channel = tv->vval.v_channel;
    }
    else
    {
	semsg(_(e_invalid_argument_str), tv_get_string(tv));
	return NULL;
    }
    if (channel != NULL && reading)
	has_readahead = channel_has_readahead(channel,
		       part != PART_COUNT ? part : channel_part_read(channel));

    if (check_open && (channel == NULL || (!channel_is_open(channel)
					     && !(reading && has_readahead))))
    {
	emsg(_(e_not_an_open_channel));
	return NULL;
    }
    return channel;
}

/*
 * Common for ch_read() and ch_readraw().
 */
    static void
common_channel_read(typval_T *argvars, typval_T *rettv, int raw, int blob)
{
    channel_T	*channel;
    ch_part_T	part = PART_COUNT;
    jobopt_T	opt;
    int		mode;
    int		timeout;
    int		id = -1;
    typval_T	*listtv = NULL;

    // return an empty string by default
    rettv->v_type = VAR_STRING;
    rettv->vval.v_string = NULL;

    if (in_vim9script()
	    && (check_for_chan_or_job_arg(argvars, 0) == FAIL
		|| check_for_opt_dict_arg(argvars, 1) == FAIL))
	return;

    clear_job_options(&opt);
    if (get_job_options(&argvars[1], &opt, JO_TIMEOUT + JO_PART + JO_ID, 0)
								      == FAIL)
	goto theend;

    if (opt.jo_set & JO_PART)
	part = opt.jo_part;
    channel = get_channel_arg(&argvars[0], TRUE, TRUE, part);
    if (channel == NULL)
	goto theend;

    if (part == PART_COUNT)
	part = channel_part_read(channel);
    mode = channel_get_mode(channel, part);
    timeout = channel_get_timeout(channel, part);
    if (opt.jo_set & JO_TIMEOUT)
	timeout = opt.jo_timeout;

    if (blob)
    {
	int	    outlen = 0;
	char_u  *p = channel_read_block(channel, part,
		timeout, TRUE, &outlen);
	if (p != NULL)
	{
	    blob_T	*b = blob_alloc();

	    if (b != NULL)
	    {
		b->bv_ga.ga_len = outlen;
		if (ga_grow(&b->bv_ga, outlen) == FAIL)
		    blob_free(b);
		else
		{
		    memcpy(b->bv_ga.ga_data, p, outlen);
		    rettv_blob_set(rettv, b);
		}
	    }
	    vim_free(p);
	}
    }
    else if (raw || mode == CH_MODE_RAW || mode == CH_MODE_NL)
	rettv->vval.v_string = channel_read_block(channel, part,
		timeout, raw, NULL);
    else
    {
	if (opt.jo_set & JO_ID)
	    id = opt.jo_id;
	channel_read_json_block(channel, part, timeout, id, &listtv);
	if (listtv != NULL)
	{
	    *rettv = *listtv;
	    vim_free(listtv);
	}
	else
	{
	    rettv->v_type = VAR_SPECIAL;
	    rettv->vval.v_number = VVAL_NONE;
	}
    }

theend:
    free_job_options(&opt);
}

#if defined(MSWIN) || defined(__HAIKU__) || defined(FEAT_GUI) || defined(PROTO)
/*
 * Check the channels for anything that is ready to be read.
 * The data is put in the read queue.
 * if "only_keep_open" is TRUE only check channels where ch_keep_open is set.
 */
    void
channel_handle_events(int only_keep_open)
{
    channel_T	*channel;
    ch_part_T	part;
    sock_T	fd;

    FOR_ALL_CHANNELS(channel)
    {
	if (only_keep_open && !channel->ch_keep_open)
	    continue;

	// check the socket and pipes
	for (part = PART_SOCK; part < PART_IN; ++part)
	{
	    fd = channel->ch_part[part].ch_fd;
	    if (fd == INVALID_FD)
		continue;

	    int r = channel_wait(channel, fd, 0);

	    if (r == CW_READY)
		channel_read(channel, part, "channel_handle_events");
	    else if (r == CW_ERROR)
		ch_close_part_on_error(channel, part, TRUE,
			"channel_handle_events");
	}

# ifdef __HAIKU__
	// Workaround for Haiku: Since select/poll cannot detect EOF from tty,
	// should close fds when the job has finished if 'channel' connects to
	// the pty.
	if (channel->ch_job != NULL)
	{
	    job_T *job = channel->ch_job;

	    if (job->jv_tty_out != NULL && job->jv_status == JOB_FINISHED)
		for (part = PART_SOCK; part < PART_COUNT; ++part)
		    ch_close_part(channel, part);
	}
# endif
    }
}
#endif

# if defined(FEAT_GUI) || defined(PROTO)
/*
 * Return TRUE when there is any channel with a keep_open flag.
 */
    int
channel_any_keep_open(void)
{
    channel_T	*channel;

    FOR_ALL_CHANNELS(channel)
	if (channel->ch_keep_open)
	    return TRUE;
    return FALSE;
}
# endif

/*
 * Set "channel"/"part" to non-blocking.
 * Only works for sockets and pipes.
 */
    void
channel_set_nonblock(channel_T *channel, ch_part_T part)
{
    chanpart_T *ch_part = &channel->ch_part[part];
    int		fd = ch_part->ch_fd;

    if (fd == INVALID_FD)
	return;

#ifdef MSWIN
    u_long	val = 1;

    ioctlsocket(fd, FIONBIO, &val);
#else
    (void)fcntl(fd, F_SETFL, O_NONBLOCK);
#endif
    ch_part->ch_nonblocking = TRUE;
}

/*
 * Write "buf" (NUL terminated string) to "channel"/"part".
 * When "fun" is not NULL an error message might be given.
 * Return FAIL or OK.
 */
    int
channel_send(
	channel_T *channel,
	ch_part_T part,
	char_u	  *buf_arg,
	int	  len_arg,
	char	  *fun)
{
    int		res;
    sock_T	fd;
    chanpart_T	*ch_part = &channel->ch_part[part];
    int		did_use_queue = FALSE;

    fd = ch_part->ch_fd;
    if (fd == INVALID_FD)
    {
	if (!channel->ch_error && fun != NULL)
	{
	    ch_error(channel, "%s(): write while not connected", fun);
	    semsg(_(e_str_write_while_not_connected), fun);
	}
	channel->ch_error = TRUE;
	return FAIL;
    }

    if (channel->ch_nonblock && !ch_part->ch_nonblocking)
	channel_set_nonblock(channel, part);

    if (ch_log_active())
    {
	ch_log_literal("SEND ", channel, part, buf_arg, len_arg);
	did_repeated_msg = 0;
    }

    for (;;)
    {
	writeq_T    *wq = &ch_part->ch_writeque;
	char_u	    *buf;
	int	    len;

	if (wq->wq_next != NULL)
	{
	    // first write what was queued
	    buf = wq->wq_next->wq_ga.ga_data;
	    len = wq->wq_next->wq_ga.ga_len;
	    did_use_queue = TRUE;
	}
	else
	{
	    if (len_arg == 0)
		// nothing to write, called from channel_select_check()
		return OK;
	    buf = buf_arg;
	    len = len_arg;
	}

	if (part == PART_SOCK)
	    res = sock_write(fd, (char *)buf, len);
	else
	{
	    res = fd_write(fd, (char *)buf, len);
#ifdef MSWIN
	    if (channel->ch_named_pipe && res < 0)
	    {
		DisconnectNamedPipe((HANDLE)fd);
		ConnectNamedPipe((HANDLE)fd, NULL);
	    }
#endif
	}
	if (res < 0 && (errno == EWOULDBLOCK
#ifdef EAGAIN
			|| errno == EAGAIN
#endif
		    ))
	    res = 0; // nothing got written

	if (res >= 0 && ch_part->ch_nonblocking)
	{
	    writeq_T *entry = wq->wq_next;

	    if (did_use_queue)
		ch_log(channel, "Sent %d bytes now", res);
	    if (res == len)
	    {
		// Wrote all the buf[len] bytes.
		if (entry != NULL)
		{
		    // Remove the entry from the write queue.
		    remove_from_writeque(wq, entry);
		    continue;
		}
		if (did_use_queue)
		    ch_log(channel, "Write queue empty");
	    }
	    else
	    {
		// Wrote only buf[res] bytes, can't write more now.
		if (entry != NULL)
		{
		    if (res > 0)
		    {
			// Remove the bytes that were written.
			mch_memmove(entry->wq_ga.ga_data,
				    (char *)entry->wq_ga.ga_data + res,
				    len - res);
			entry->wq_ga.ga_len -= res;
		    }
		    buf = buf_arg;
		    len = len_arg;
		}
		else
		{
		    buf += res;
		    len -= res;
		}
		ch_log(channel, "Adding %d bytes to the write queue", len);

		// Append the not written bytes of the argument to the write
		// buffer.  Limit entries to 4000 bytes.
		if (wq->wq_prev != NULL
			&& wq->wq_prev->wq_ga.ga_len + len < 4000)
		{
		    writeq_T *last = wq->wq_prev;

		    // append to the last entry
		    if (len > 0 && ga_grow(&last->wq_ga, len) == OK)
		    {
			mch_memmove((char *)last->wq_ga.ga_data
							  + last->wq_ga.ga_len,
				    buf, len);
			last->wq_ga.ga_len += len;
		    }
		}
		else
		{
		    writeq_T *last = ALLOC_ONE(writeq_T);

		    if (last != NULL)
		    {
			last->wq_prev = wq->wq_prev;
			last->wq_next = NULL;
			if (wq->wq_prev == NULL)
			    wq->wq_next = last;
			else
			    wq->wq_prev->wq_next = last;
			wq->wq_prev = last;
			ga_init2(&last->wq_ga, 1, 1000);
			if (len > 0 && ga_grow(&last->wq_ga, len) == OK)
			{
			    mch_memmove(last->wq_ga.ga_data, buf, len);
			    last->wq_ga.ga_len = len;
			}
		    }
		}
	    }
	}
	else if (res != len)
	{
	    if (!channel->ch_error && fun != NULL)
	    {
		ch_error(channel, "%s(): write failed", fun);
		semsg(_(e_str_write_failed), fun);
	    }
	    channel->ch_error = TRUE;
	    return FAIL;
	}

	channel->ch_error = FALSE;
	return OK;
    }
}

/*
 * Common for "ch_sendexpr()" and "ch_sendraw()".
 * Returns the channel if the caller should read the response.
 * Sets "part_read" to the read fd.
 * Otherwise returns NULL.
 */
    static channel_T *
send_common(
	typval_T    *argvars,
	char_u	    *text,
	int	    len,
	int	    id,
	int	    eval,
	jobopt_T    *opt,
	char	    *fun,
	ch_part_T   *part_read)
{
    channel_T	*channel;
    ch_part_T	part_send;

    clear_job_options(opt);
    channel = get_channel_arg(&argvars[0], TRUE, FALSE, 0);
    if (channel == NULL)
	return NULL;
    part_send = channel_part_send(channel);
    *part_read = channel_part_read(channel);

    if (get_job_options(&argvars[2], opt, JO_CALLBACK + JO_TIMEOUT, 0) == FAIL)
	return NULL;

    // Set the callback. An empty callback means no callback and not reading
    // the response. With "ch_evalexpr()" and "ch_evalraw()" a callback is not
    // allowed.
    if (opt->jo_callback.cb_name != NULL && *opt->jo_callback.cb_name != NUL)
    {
	if (eval)
	{
	    semsg(_(e_cannot_use_callback_with_str), fun);
	    return NULL;
	}
	channel_set_req_callback(channel, *part_read, &opt->jo_callback, id);
    }

    if (channel_send(channel, part_send, text, len, fun) == OK
					   && opt->jo_callback.cb_name == NULL)
	return channel;
    return NULL;
}

/*
 * common for "ch_evalexpr()" and "ch_sendexpr()"
 */
    static void
ch_expr_common(typval_T *argvars, typval_T *rettv, int eval)
{
    char_u	*text;
    typval_T	*listtv;
    channel_T	*channel;
    int		id;
    ch_mode_T	ch_mode;
    ch_part_T	part_send;
    ch_part_T	part_read;
    jobopt_T    opt;
    int		timeout;
    int		callback_present = FALSE;

    // return an empty string by default
    rettv->v_type = VAR_STRING;
    rettv->vval.v_string = NULL;

    if (in_vim9script()
	    && (check_for_chan_or_job_arg(argvars, 0) == FAIL
		|| check_for_opt_dict_arg(argvars, 2) == FAIL))
	return;

    channel = get_channel_arg(&argvars[0], TRUE, FALSE, 0);
    if (channel == NULL)
	return;
    part_send = channel_part_send(channel);

    ch_mode = channel_get_mode(channel, part_send);
    if (ch_mode == CH_MODE_RAW || ch_mode == CH_MODE_NL)
    {
	emsg(_(e_cannot_use_evalexpr_sendexpr_with_raw_or_nl_channel));
	return;
    }

    if (ch_mode == CH_MODE_LSP)
    {
	dict_T		*d;
	dictitem_T	*di;

	// return an empty dict by default
	if (rettv_dict_alloc(rettv) == FAIL)
	    return;

	if (check_for_dict_arg(argvars, 1) == FAIL)
	    return;

	d = argvars[1].vval.v_dict;
	di = dict_find(d, (char_u *)"id", -1);
	if (di != NULL && di->di_tv.v_type != VAR_NUMBER)
	{
	    // only number type is supported for the 'id' item
	    semsg(_(e_invalid_value_for_argument_str), "id");
	    return;
	}

	if (argvars[2].v_type == VAR_DICT)
	    if (dict_has_key(argvars[2].vval.v_dict, "callback"))
		callback_present = TRUE;

	if (eval || callback_present)
	{
	    // When evaluating an expression or sending an expression with a
	    // callback, always assign a generated ID
	    id = ++channel->ch_last_msg_id;
	    if (di == NULL)
		dict_add_number(d, "id", id);
	    else
		di->di_tv.vval.v_number = id;
	}
	else
	{
	    // When sending an expression, if the message has an 'id' item,
	    // then use it.
	    id = 0;
	    if (di != NULL)
		id = di->di_tv.vval.v_number;
	}
	if (!dict_has_key(d, "jsonrpc"))
	    dict_add_string(d, "jsonrpc", (char_u *)"2.0");
	text = json_encode_lsp_msg(&argvars[1]);
    }
    else
    {
	id = ++channel->ch_last_msg_id;
	text = json_encode_nr_expr(id, &argvars[1],
			      (ch_mode == CH_MODE_JS ? JSON_JS : 0) | JSON_NL);
    }
    if (text == NULL)
	return;

    channel = send_common(argvars, text, (int)STRLEN(text), id, eval, &opt,
			    eval ? "ch_evalexpr" : "ch_sendexpr", &part_read);
    vim_free(text);
    if (channel != NULL && eval)
    {
	if (opt.jo_set & JO_TIMEOUT)
	    timeout = opt.jo_timeout;
	else
	    timeout = channel_get_timeout(channel, part_read);
	if (channel_read_json_block(channel, part_read, timeout, id, &listtv)
									== OK)
	{
	    if (ch_mode == CH_MODE_LSP)
	    {
		*rettv = *listtv;
		// Change the type to avoid the value being freed.
		listtv->v_type = VAR_NUMBER;
		free_tv(listtv);
	    }
	    else
	    {
		list_T *list = listtv->vval.v_list;

		// Move the item from the list and then change the type to
		// avoid the value being freed.
		*rettv = list->lv_u.mat.lv_last->li_tv;
		list->lv_u.mat.lv_last->li_tv.v_type = VAR_NUMBER;
		free_tv(listtv);
	    }
	}
    }
    free_job_options(&opt);
    if (ch_mode == CH_MODE_LSP && !eval && callback_present)
    {
	// if ch_sendexpr() is used to send a LSP message and a callback
	// function is specified, then return the generated identifier for the
	// message.  The user can use this to cancel the request (if needed).
	if (rettv->vval.v_dict != NULL)
	    dict_add_number(rettv->vval.v_dict, "id", id);
    }
}

/*
 * common for "ch_evalraw()" and "ch_sendraw()"
 */
    static void
ch_raw_common(typval_T *argvars, typval_T *rettv, int eval)
{
    char_u	buf[NUMBUFLEN];
    char_u	*text;
    int		len;
    channel_T	*channel;
    ch_part_T	part_read;
    jobopt_T    opt;
    int		timeout;

    // return an empty string by default
    rettv->v_type = VAR_STRING;
    rettv->vval.v_string = NULL;

    if (in_vim9script()
	    && (check_for_chan_or_job_arg(argvars, 0) == FAIL
		|| check_for_string_or_blob_arg(argvars, 1) == FAIL
		|| check_for_opt_dict_arg(argvars, 2) == FAIL))
	return;

    if (argvars[1].v_type == VAR_BLOB)
    {
	text = argvars[1].vval.v_blob->bv_ga.ga_data;
	len = argvars[1].vval.v_blob->bv_ga.ga_len;
    }
    else
    {
	text = tv_get_string_buf(&argvars[1], buf);
	len = (int)STRLEN(text);
    }
    channel = send_common(argvars, text, len, 0, eval, &opt,
			      eval ? "ch_evalraw" : "ch_sendraw", &part_read);
    if (channel != NULL && eval)
    {
	if (opt.jo_set & JO_TIMEOUT)
	    timeout = opt.jo_timeout;
	else
	    timeout = channel_get_timeout(channel, part_read);
	rettv->vval.v_string = channel_read_block(channel, part_read,
							timeout, TRUE, NULL);
    }
    free_job_options(&opt);
}

#define KEEP_OPEN_TIME 20  // msec

#if (defined(UNIX) && !defined(HAVE_SELECT)) || defined(PROTO)
/*
 * Add open channels to the poll struct.
 * Return the adjusted struct index.
 * The type of "fds" is hidden to avoid problems with the function proto.
 */
    int
channel_poll_setup(int nfd_in, void *fds_in, int *towait)
{
    int		nfd = nfd_in;
    channel_T	*channel;
    struct	pollfd *fds = fds_in;
    ch_part_T	part;

    FOR_ALL_CHANNELS(channel)
    {
	for (part = PART_SOCK; part < PART_IN; ++part)
	{
	    chanpart_T	*ch_part = &channel->ch_part[part];

	    if (ch_part->ch_fd != INVALID_FD)
	    {
		if (channel->ch_keep_open)
		{
		    // For unknown reason poll() returns immediately for a
		    // keep-open channel.  Instead of adding it to the fds add
		    // a short timeout and check, like polling.
		    if (*towait < 0 || *towait > KEEP_OPEN_TIME)
			*towait = KEEP_OPEN_TIME;
		}
		else
		{
		    ch_part->ch_poll_idx = nfd;
		    fds[nfd].fd = ch_part->ch_fd;
		    fds[nfd].events = POLLIN;
		    nfd++;
		}
	    }
	    else
		channel->ch_part[part].ch_poll_idx = -1;
	}
    }

    nfd = channel_fill_poll_write(nfd, fds);

    return nfd;
}

/*
 * The type of "fds" is hidden to avoid problems with the function proto.
 */
    int
channel_poll_check(int ret_in, void *fds_in)
{
    int		ret = ret_in;
    channel_T	*channel;
    struct	pollfd *fds = fds_in;
    ch_part_T	part;
    int		idx;
    chanpart_T	*in_part;

    FOR_ALL_CHANNELS(channel)
    {
	for (part = PART_SOCK; part < PART_IN; ++part)
	{
	    idx = channel->ch_part[part].ch_poll_idx;

	    if (ret > 0 && idx != -1 && (fds[idx].revents & POLLIN))
	    {
		channel_read(channel, part, "channel_poll_check");
		--ret;
	    }
	    else if (channel->ch_part[part].ch_fd != INVALID_FD
						      && channel->ch_keep_open)
	    {
		// polling a keep-open channel
		channel_read(channel, part, "channel_poll_check_keep_open");
	    }
	}

	in_part = &channel->ch_part[PART_IN];
	idx = in_part->ch_poll_idx;
	if (ret > 0 && idx != -1 && (fds[idx].revents & POLLOUT))
	{
	    channel_write_input(channel);
	    --ret;
	}
    }

    return ret;
}
#endif // UNIX && !HAVE_SELECT

#if (!defined(MSWIN) && defined(HAVE_SELECT)) || defined(PROTO)

/*
 * The "fd_set" type is hidden to avoid problems with the function proto.
 */
    int
channel_select_setup(
	int maxfd_in,
	void *rfds_in,
	void *wfds_in,
	struct timeval *tv,
	struct timeval **tvp)
{
    int		maxfd = maxfd_in;
    channel_T	*channel;
    fd_set	*rfds = rfds_in;
    fd_set	*wfds = wfds_in;
    ch_part_T	part;

    FOR_ALL_CHANNELS(channel)
    {
	for (part = PART_SOCK; part < PART_IN; ++part)
	{
	    sock_T fd = channel->ch_part[part].ch_fd;

	    if (fd != INVALID_FD)
	    {
		if (channel->ch_keep_open)
		{
		    // For unknown reason select() returns immediately for a
		    // keep-open channel.  Instead of adding it to the rfds add
		    // a short timeout and check, like polling.
		    if (*tvp == NULL || tv->tv_sec > 0
					|| tv->tv_usec > KEEP_OPEN_TIME * 1000)
		    {
			*tvp = tv;
			tv->tv_sec = 0;
			tv->tv_usec = KEEP_OPEN_TIME * 1000;
		    }
		}
		else
		{
		    FD_SET((int)fd, rfds);
		    if (maxfd < (int)fd)
			maxfd = (int)fd;
		}
	    }
	}
    }

    maxfd = channel_fill_wfds(maxfd, wfds);

    return maxfd;
}

/*
 * The "fd_set" type is hidden to avoid problems with the function proto.
 */
    int
channel_select_check(int ret_in, void *rfds_in, void *wfds_in)
{
    int		ret = ret_in;
    channel_T	*channel;
    fd_set	*rfds = rfds_in;
    fd_set	*wfds = wfds_in;
    ch_part_T	part;
    chanpart_T	*in_part;

    FOR_ALL_CHANNELS(channel)
    {
	for (part = PART_SOCK; part < PART_IN; ++part)
	{
	    sock_T fd = channel->ch_part[part].ch_fd;

	    if (ret > 0 && fd != INVALID_FD && FD_ISSET(fd, rfds))
	    {
		channel_read(channel, part, "channel_select_check");
		FD_CLR(fd, rfds);
		--ret;
	    }
	    else if (fd != INVALID_FD && channel->ch_keep_open)
	    {
		// polling a keep-open channel
		channel_read(channel, part, "channel_select_check_keep_open");
	    }
	}

	in_part = &channel->ch_part[PART_IN];
	if (ret > 0 && in_part->ch_fd != INVALID_FD
					    && FD_ISSET(in_part->ch_fd, wfds))
	{
	    // Clear the flag first, ch_fd may change in channel_write_input().
	    FD_CLR(in_part->ch_fd, wfds);
	    channel_write_input(channel);
	    --ret;
	}

# ifdef __HAIKU__
	// Workaround for Haiku: Since select/poll cannot detect EOF from tty,
	// should close fds when the job has finished if 'channel' connects to
	// the pty.
	if (channel->ch_job != NULL)
	{
	    job_T *job = channel->ch_job;

	    if (job->jv_tty_out != NULL && job->jv_status == JOB_FINISHED)
		for (part = PART_SOCK; part < PART_COUNT; ++part)
		    ch_close_part(channel, part);
	}
# endif
    }

    return ret;
}
#endif // !MSWIN && HAVE_SELECT

/*
 * Execute queued up commands.
 * Invoked from the main loop when it's safe to execute received commands,
 * and during a blocking wait for ch_evalexpr().
 * Return TRUE when something was done.
 */
    int
channel_parse_messages(void)
{
    channel_T	*channel = first_channel;
    int		ret = FALSE;
    int		r;
    ch_part_T	part = PART_SOCK;
    static int	recursive = 0;
#ifdef ELAPSED_FUNC
    elapsed_T	start_tv;
#endif

    // The code below may invoke callbacks, which might call us back.
    // In a recursive call channels will not be closed.
    ++recursive;
    ++safe_to_invoke_callback;

#ifdef ELAPSED_FUNC
    ELAPSED_INIT(start_tv);
#endif

    // Only do this message when another message was given, otherwise we get
    // lots of them.
    if ((did_repeated_msg & REPEATED_MSG_LOOKING) == 0)
    {
	ch_log(NULL, "looking for messages on channels");
	// now we should also give the message for SafeState
	did_repeated_msg = REPEATED_MSG_LOOKING;
    }
    while (channel != NULL)
    {
	if (recursive == 1)
	{
	    if (channel_can_close(channel))
	    {
		channel->ch_to_be_closed = (1U << PART_COUNT);
		channel_close_now(channel);
		// channel may have been freed, start over
		channel = first_channel;
		continue;
	    }
	    if (channel->ch_to_be_freed || channel->ch_killing)
	    {
		channel_free_contents(channel);
		if (channel->ch_job != NULL)
		    channel->ch_job->jv_channel = NULL;

		// free the channel and then start over
		channel_free_channel(channel);
		channel = first_channel;
		continue;
	    }
	    if (channel->ch_refcount == 0 && !channel_still_useful(channel))
	    {
		// channel is no longer useful, free it
		channel_free(channel);
		channel = first_channel;
		part = PART_SOCK;
		continue;
	    }
	}

	if (channel->ch_part[part].ch_fd != INVALID_FD
				      || channel_has_readahead(channel, part))
	{
	    // Increase the refcount, in case the handler causes the channel
	    // to be unreferenced or closed.
	    ++channel->ch_refcount;
	    r = may_invoke_callback(channel, part);
	    if (r == OK)
		ret = TRUE;
	    if (channel_unref(channel) || (r == OK
#ifdef ELAPSED_FUNC
			// Limit the time we loop here to 100 msec, otherwise
			// Vim becomes unresponsive when the callback takes
			// more than a bit of time.
			&& ELAPSED_FUNC(start_tv) < 100L
#endif
			))
	    {
		// channel was freed or something was done, start over
		channel = first_channel;
		part = PART_SOCK;
		continue;
	    }
	}
	if (part < PART_ERR)
	    ++part;
	else
	{
	    channel = channel->ch_next;
	    part = PART_SOCK;
	}
    }

    if (channel_need_redraw)
    {
	channel_need_redraw = FALSE;
	redraw_after_callback(TRUE, FALSE);
    }

    --safe_to_invoke_callback;
    --recursive;

    return ret;
}

/*
 * Return TRUE if any channel has readahead.  That means we should not block on
 * waiting for input.
 */
    int
channel_any_readahead(void)
{
    channel_T	*channel = first_channel;
    ch_part_T	part = PART_SOCK;

    while (channel != NULL)
    {
	if (channel_has_readahead(channel, part))
	    return TRUE;
	if (part < PART_ERR)
	    ++part;
	else
	{
	    channel = channel->ch_next;
	    part = PART_SOCK;
	}
    }
    return FALSE;
}

/*
 * Mark references to lists used in channels.
 */
    int
set_ref_in_channel(int copyID)
{
    int		abort = FALSE;
    channel_T	*channel;
    typval_T	tv;

    for (channel = first_channel; !abort && channel != NULL;
						   channel = channel->ch_next)
	if (channel_still_useful(channel))
	{
	    tv.v_type = VAR_CHANNEL;
	    tv.vval.v_channel = channel;
	    abort = abort || set_ref_in_item(&tv, copyID, NULL, NULL);
	}
    return abort;
}

/*
 * Return the "part" to write to for "channel".
 */
    static ch_part_T
channel_part_send(channel_T *channel)
{
    if (channel->CH_SOCK_FD == INVALID_FD)
	return PART_IN;
    return PART_SOCK;
}

/*
 * Return the default "part" to read from for "channel".
 */
    static ch_part_T
channel_part_read(channel_T *channel)
{
    if (channel->CH_SOCK_FD == INVALID_FD)
	return PART_OUT;
    return PART_SOCK;
}

/*
 * Return the mode of "channel"/"part"
 * If "channel" is invalid returns CH_MODE_JSON.
 */
    static ch_mode_T
channel_get_mode(channel_T *channel, ch_part_T part)
{
    if (channel == NULL)
	return CH_MODE_JSON;
    return channel->ch_part[part].ch_mode;
}

/*
 * Return the timeout of "channel"/"part"
 */
    static int
channel_get_timeout(channel_T *channel, ch_part_T part)
{
    return channel->ch_part[part].ch_timeout;
}

/*
 * "ch_canread()" function
 */
    void
f_ch_canread(typval_T *argvars, typval_T *rettv)
{
    channel_T *channel;

    rettv->vval.v_number = 0;
    if (in_vim9script() && check_for_chan_or_job_arg(argvars, 0) == FAIL)
	return;

    channel = get_channel_arg(&argvars[0], FALSE, FALSE, 0);
    if (channel != NULL)
	rettv->vval.v_number = channel_has_readahead(channel, PART_SOCK)
			    || channel_has_readahead(channel, PART_OUT)
			    || channel_has_readahead(channel, PART_ERR);
}

/*
 * "ch_close()" function
 */
    void
f_ch_close(typval_T *argvars, typval_T *rettv UNUSED)
{
    channel_T *channel;

    if (in_vim9script() && check_for_chan_or_job_arg(argvars, 0) == FAIL)
	return;

    channel = get_channel_arg(&argvars[0], TRUE, FALSE, 0);
    if (channel != NULL)
    {
	channel_close(channel, FALSE);
	channel_clear(channel);
    }
}

/*
 * "ch_close()" function
 */
    void
f_ch_close_in(typval_T *argvars, typval_T *rettv UNUSED)
{
    channel_T *channel;

    if (in_vim9script() && check_for_chan_or_job_arg(argvars, 0) == FAIL)
	return;

    channel = get_channel_arg(&argvars[0], TRUE, FALSE, 0);
    if (channel != NULL)
	channel_close_in(channel);
}

/*
 * "ch_getbufnr()" function
 */
    void
f_ch_getbufnr(typval_T *argvars, typval_T *rettv)
{
    channel_T	*channel;

    rettv->vval.v_number = -1;

    if (in_vim9script()
	    && (check_for_chan_or_job_arg(argvars, 0) == FAIL
		|| check_for_string_arg(argvars, 1) == FAIL))
	return;

    channel = get_channel_arg(&argvars[0], FALSE, FALSE, 0);
    if (channel == NULL)
	return;

    char_u	*what = tv_get_string(&argvars[1]);
    int		part;
    if (STRCMP(what, "err") == 0)
	part = PART_ERR;
    else if (STRCMP(what, "out") == 0)
	part = PART_OUT;
    else if (STRCMP(what, "in") == 0)
	part = PART_IN;
    else
	part = PART_SOCK;
    if (channel->ch_part[part].ch_bufref.br_buf != NULL)
	rettv->vval.v_number =
	    channel->ch_part[part].ch_bufref.br_buf->b_fnum;
}

/*
 * "ch_getjob()" function
 */
    void
f_ch_getjob(typval_T *argvars, typval_T *rettv)
{
    channel_T *channel;

    if (in_vim9script() && check_for_chan_or_job_arg(argvars, 0) == FAIL)
	return;

    channel = get_channel_arg(&argvars[0], FALSE, FALSE, 0);
    if (channel == NULL)
	return;

    rettv->v_type = VAR_JOB;
    rettv->vval.v_job = channel->ch_job;
    if (channel->ch_job != NULL)
	++channel->ch_job->jv_refcount;
}

/*
 * "ch_info()" function
 */
    void
f_ch_info(typval_T *argvars, typval_T *rettv UNUSED)
{
    channel_T *channel;

    if (in_vim9script() && check_for_chan_or_job_arg(argvars, 0) == FAIL)
	return;

    channel = get_channel_arg(&argvars[0], FALSE, FALSE, 0);
    if (channel != NULL && rettv_dict_alloc(rettv) == OK)
	channel_info(channel, rettv->vval.v_dict);
}

/*
 * "ch_open()" function
 */
    void
f_ch_open(typval_T *argvars, typval_T *rettv)
{
    rettv->v_type = VAR_CHANNEL;
    if (check_restricted() || check_secure())
	return;
    rettv->vval.v_channel = channel_open_func(argvars);
}

/*
 * "ch_read()" function
 */
    void
f_ch_read(typval_T *argvars, typval_T *rettv)
{
    common_channel_read(argvars, rettv, FALSE, FALSE);
}

/*
 * "ch_readblob()" function
 */
    void
f_ch_readblob(typval_T *argvars, typval_T *rettv)
{
    common_channel_read(argvars, rettv, TRUE, TRUE);
}

/*
 * "ch_readraw()" function
 */
    void
f_ch_readraw(typval_T *argvars, typval_T *rettv)
{
    common_channel_read(argvars, rettv, TRUE, FALSE);
}

/*
 * "ch_evalexpr()" function
 */
    void
f_ch_evalexpr(typval_T *argvars, typval_T *rettv)
{
    ch_expr_common(argvars, rettv, TRUE);
}

/*
 * "ch_sendexpr()" function
 */
    void
f_ch_sendexpr(typval_T *argvars, typval_T *rettv)
{
    ch_expr_common(argvars, rettv, FALSE);
}

/*
 * "ch_evalraw()" function
 */
    void
f_ch_evalraw(typval_T *argvars, typval_T *rettv)
{
    ch_raw_common(argvars, rettv, TRUE);
}

/*
 * "ch_sendraw()" function
 */
    void
f_ch_sendraw(typval_T *argvars, typval_T *rettv)
{
    ch_raw_common(argvars, rettv, FALSE);
}

/*
 * "ch_setoptions()" function
 */
    void
f_ch_setoptions(typval_T *argvars, typval_T *rettv UNUSED)
{
    channel_T	*channel;
    jobopt_T	opt;

    if (in_vim9script()
	    && (check_for_chan_or_job_arg(argvars, 0) == FAIL
		|| check_for_dict_arg(argvars, 1) == FAIL))
	return;

    channel = get_channel_arg(&argvars[0], FALSE, FALSE, 0);
    if (channel == NULL)
	return;
    clear_job_options(&opt);
    if (get_job_options(&argvars[1], &opt,
			    JO_CB_ALL + JO_TIMEOUT_ALL + JO_MODE_ALL, 0) == OK)
	channel_set_options(channel, &opt);
    free_job_options(&opt);
}

/*
 * "ch_status()" function
 */
    void
f_ch_status(typval_T *argvars, typval_T *rettv)
{
    channel_T	*channel;
    jobopt_T	opt;
    int		part = -1;

    // return an empty string by default
    rettv->v_type = VAR_STRING;
    rettv->vval.v_string = NULL;

    if (in_vim9script()
	    && (check_for_chan_or_job_arg(argvars, 0) == FAIL
		|| check_for_opt_dict_arg(argvars, 1) == FAIL))
	return;

    channel = get_channel_arg(&argvars[0], FALSE, FALSE, 0);

    if (argvars[1].v_type != VAR_UNKNOWN)
    {
	clear_job_options(&opt);
	if (get_job_options(&argvars[1], &opt, JO_PART, 0) == OK
						     && (opt.jo_set & JO_PART))
	    part = opt.jo_part;
    }

    rettv->vval.v_string = vim_strsave((char_u *)channel_status(channel, part));
}

/*
 * Get a string with information about the channel in "varp" in "buf".
 * "buf" must be at least NUMBUFLEN long.
 */
    char_u *
channel_to_string_buf(typval_T *varp, char_u *buf)
{
    channel_T *channel = varp->vval.v_channel;
    char      *status = channel_status(channel, -1);

    if (channel == NULL)
	vim_snprintf((char *)buf, NUMBUFLEN, "channel %s", status);
    else
	vim_snprintf((char *)buf, NUMBUFLEN,
				      "channel %d %s", channel->ch_id, status);
    return buf;
}

#endif // FEAT_JOB_CHANNEL
