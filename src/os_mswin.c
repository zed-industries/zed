/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * os_mswin.c
 *
 * Routines for Win32.
 */

#include "vim.h"

#include <sys/types.h>
#include <signal.h>
#include <limits.h>
#ifndef PROTO
# include <process.h>
#endif

#undef chdir
#ifdef __GNUC__
# ifndef __MINGW32__
#  include <dirent.h>
# endif
#else
# include <direct.h>
#endif

#ifndef PROTO
# if !defined(FEAT_GUI_MSWIN)
#  include <shellapi.h>
# endif

# if defined(FEAT_PRINTER) && !defined(FEAT_POSTSCRIPT)
#  include <dlgs.h>
#  include <winspool.h>
#  include <commdlg.h>
# endif

#endif // PROTO

#ifdef __MINGW32__
# ifndef FROM_LEFT_1ST_BUTTON_PRESSED
#  define FROM_LEFT_1ST_BUTTON_PRESSED    0x0001
# endif
# ifndef RIGHTMOST_BUTTON_PRESSED
#  define RIGHTMOST_BUTTON_PRESSED	  0x0002
# endif
# ifndef FROM_LEFT_2ND_BUTTON_PRESSED
#  define FROM_LEFT_2ND_BUTTON_PRESSED    0x0004
# endif
# ifndef FROM_LEFT_3RD_BUTTON_PRESSED
#  define FROM_LEFT_3RD_BUTTON_PRESSED    0x0008
# endif
# ifndef FROM_LEFT_4TH_BUTTON_PRESSED
#  define FROM_LEFT_4TH_BUTTON_PRESSED    0x0010
# endif

/*
 * EventFlags
 */
# ifndef MOUSE_MOVED
#  define MOUSE_MOVED   0x0001
# endif
# ifndef DOUBLE_CLICK
#  define DOUBLE_CLICK  0x0002
# endif
#endif

/*
 * When generating prototypes for Win32 on Unix, these lines make the syntax
 * errors disappear.  They do not need to be correct.
 */
#ifdef PROTO
# define WINAPI
# define WINBASEAPI
typedef int BOOL;
typedef int CALLBACK;
typedef int COLORREF;
typedef int CONSOLE_CURSOR_INFO;
typedef int COORD;
typedef int DWORD;
typedef int ENUMLOGFONTW;
typedef int HANDLE;
typedef int HDC;
typedef int HFONT;
typedef int HICON;
typedef int HWND;
typedef int INPUT_RECORD;
typedef int INT_PTR;
typedef int KEY_EVENT_RECORD;
typedef int LOGFONTW;
typedef int LPARAM;
typedef int LPBOOL;
typedef int LPCSTR;
typedef int LPCWSTR;
typedef int LPDWORD;
typedef int LPSTR;
typedef int LPTSTR;
typedef int LPVOID;
typedef int LPWSTR;
typedef int LRESULT;
typedef int MOUSE_EVENT_RECORD;
typedef int NEWTEXTMETRICW;
typedef int PACL;
typedef int PRINTDLGW;
typedef int PSECURITY_DESCRIPTOR;
typedef int PSID;
typedef int SECURITY_INFORMATION;
typedef int SHORT;
typedef int SMALL_RECT;
typedef int TEXTMETRIC;
typedef int UINT;
typedef int WCHAR;
typedef int WNDENUMPROC;
typedef int WORD;
typedef int WPARAM;
typedef void VOID;
#endif

// Record all output and all keyboard & mouse input
// #define MCH_WRITE_DUMP

#ifdef MCH_WRITE_DUMP
FILE* fdDump = NULL;
#endif

#if !defined(FEAT_GUI_MSWIN) || defined(VIMDLL)
extern char g_szOrigTitle[];
#endif

#ifdef FEAT_GUI
extern HWND s_hwnd;
#else
static HWND s_hwnd = 0;	    // console window handle, set by GetConsoleHwnd()
#endif

#ifdef FEAT_JOB_CHANNEL
int WSInitialized = FALSE; // WinSock is initialized
#endif

// Don't generate prototypes here, because some systems do have these
// functions.
#if defined(__GNUC__) && !defined(PROTO)
# ifndef __MINGW32__
int _stricoll(char *a, char *b)
{
    // the ANSI-ish correct way is to use strxfrm():
    char a_buff[512], b_buff[512];  // file names, so this is enough on Win32
    strxfrm(a_buff, a, 512);
    strxfrm(b_buff, b, 512);
    return strcoll(a_buff, b_buff);
}

char * _fullpath(char *buf, char *fname, int len)
{
    LPTSTR toss;

    return (char *)GetFullPathName(fname, len, buf, &toss);
}
# endif

# if !defined(__MINGW32__) || (__GNUC__ < 4)
int _chdrive(int drive)
{
    char temp [3] = "-:";
    temp[0] = drive + 'A' - 1;
    return !SetCurrentDirectory(temp);
}
# endif
#endif


#ifndef PROTO
/*
 * Save the instance handle of the exe/dll.
 */
    void
SaveInst(HINSTANCE hInst)
{
    g_hinst = hInst;
}
#endif

#if defined(FEAT_GUI_MSWIN) || defined(PROTO)
/*
 * GUI version of mch_exit().
 * Shut down and exit with status `r'
 * Careful: mch_exit() may be called before mch_init()!
 */
    void
mch_exit_g(int r)
{
    exiting = TRUE;

    display_errors();

    ml_close_all(TRUE);		// remove all memfiles

# ifdef FEAT_OLE
    UninitOLE();
# endif
# ifdef FEAT_JOB_CHANNEL
    if (WSInitialized)
    {
	WSInitialized = FALSE;
	WSACleanup();
    }
# endif
# ifdef DYNAMIC_GETTEXT
    dyn_libintl_end();
# endif

    if (gui.in_use)
	gui_exit(r);

# ifdef EXITFREE
    free_all_mem();
# endif

    exit(r);
}

#endif // FEAT_GUI_MSWIN


/*
 * Init the tables for toupper() and tolower().
 */
    void
mch_early_init(void)
{
    int		i;

    PlatformId();

    // Init the tables for toupper() and tolower()
    for (i = 0; i < 256; ++i)
	toupper_tab[i] = tolower_tab[i] = i;
    CharUpperBuff((LPSTR)toupper_tab, 256);
    CharLowerBuff((LPSTR)tolower_tab, 256);
}


/*
 * Return TRUE if the input comes from a terminal, FALSE otherwise.
 */
    int
mch_input_isatty(void)
{
#ifdef FEAT_GUI_MSWIN
# ifdef VIMDLL
    if (gui.in_use)
# endif
	return TRUE;	    // GUI always has a tty
#endif
#if !defined(FEAT_GUI_MSWIN) || defined(VIMDLL)
    if (isatty(read_cmd_fd))
	return TRUE;
    return FALSE;
#endif
}

/*
 * mch_settitle(): set titlebar of our window
 */
    void
mch_settitle(
    char_u *title,
    char_u *icon UNUSED)
{
#ifdef FEAT_GUI_MSWIN
# ifdef VIMDLL
    if (gui.in_use)
# endif
    {
	gui_mch_settitle(title, icon);
	return;
    }
#endif
#if !defined(FEAT_GUI_MSWIN) || defined(VIMDLL)
    if (title != NULL)
    {
	WCHAR	*wp = enc_to_utf16(title, NULL);

	if (wp == NULL)
	    return;

	SetConsoleTitleW(wp);
	vim_free(wp);
	return;
    }
#endif
}


/*
 * Restore the window/icon title.
 * which is one of:
 *  SAVE_RESTORE_TITLE: Just restore title
 *  SAVE_RESTORE_ICON:  Just restore icon (which we don't have)
 *  SAVE_RESTORE_BOTH:  Restore title and icon (which we don't have)
 */
    void
mch_restore_title(int which UNUSED)
{
#if !defined(FEAT_GUI_MSWIN) || defined(VIMDLL)
# ifdef VIMDLL
    if (!gui.in_use)
# endif
	SetConsoleTitle(g_szOrigTitle);
#endif
}


/*
 * Return TRUE if we can restore the title (we can)
 */
    int
mch_can_restore_title(void)
{
    return TRUE;
}


/*
 * Return TRUE if we can restore the icon title (we can't)
 */
    int
mch_can_restore_icon(void)
{
    return FALSE;
}


/*
 * Get absolute file name into buffer "buf" of length "len" bytes,
 * turning all '/'s into '\\'s and getting the correct case of each component
 * of the file name.  Append a (back)slash to a directory name.
 * When 'shellslash' set do it the other way around.
 * Return OK or FAIL.
 */
    int
mch_FullName(
    char_u	*fname,
    char_u	*buf,
    int		len,
    int		force UNUSED)
{
    int		nResult = FAIL;
    WCHAR	*wname;
    WCHAR	wbuf[MAX_PATH];
    char_u	*cname = NULL;

    wname = enc_to_utf16(fname, NULL);
    if (wname != NULL && _wfullpath(wbuf, wname, MAX_PATH) != NULL)
    {
	cname = utf16_to_enc((short_u *)wbuf, NULL);
	if (cname != NULL)
	{
	    vim_strncpy(buf, cname, len - 1);
	    nResult = OK;
	}
    }
    vim_free(wname);
    vim_free(cname);

#ifdef USE_FNAME_CASE
    fname_case(buf, len);
#else
    slash_adjust(buf);
#endif

    return nResult;
}


/*
 * Return TRUE if "fname" does not depend on the current directory.
 */
    int
mch_isFullName(char_u *fname)
{
    // A name like "d:/foo" and "//server/share" is absolute.  "d:foo" is not.
    // Another way to check is to use mch_FullName() and see if the result is
    // the same as the name or mch_FullName() fails.  However, this has quite a
    // bit of overhead, so let's not do that.
    if (*fname == NUL)
	return FALSE;
    return ((ASCII_ISALPHA(fname[0]) && fname[1] == ':'
				      && (fname[2] == '/' || fname[2] == '\\'))
	    || (fname[0] == fname[1] && (fname[0] == '/' || fname[0] == '\\')));
}

/*
 * Replace all slashes by backslashes.
 * This used to be the other way around, but MS-DOS sometimes has problems
 * with slashes (e.g. in a command name).  We can't have mixed slashes and
 * backslashes, because comparing file names will not work correctly.  The
 * commands that use a file name should try to avoid the need to type a
 * backslash twice.
 * When 'shellslash' set do it the other way around.
 * When the path looks like a URL leave it unmodified.
 */
    void
slash_adjust(char_u *p)
{
    if (path_with_url(p))
	return;

    if (*p == '`')
    {
	size_t len = STRLEN(p);

	// don't replace backslash in backtick quoted strings
	if (len > 2 && *(p + len - 1) == '`')
	    return;
    }

    while (*p)
    {
	if (*p == psepcN)
	    *p = psepc;
	MB_PTR_ADV(p);
    }
}

// Use 64-bit stat functions.
#undef stat
#undef _stat
#undef _wstat
#undef _fstat
#define stat _stat64
#define _stat _stat64
#define _wstat _wstat64
#define _fstat _fstat64

    static int
read_reparse_point(const WCHAR *name, char_u *buf, DWORD *buf_len)
{
    HANDLE h;
    BOOL ok;

    h = CreateFileW(name, FILE_READ_ATTRIBUTES,
	    FILE_SHARE_READ | FILE_SHARE_WRITE, NULL,
	    OPEN_EXISTING,
	    FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT,
	    NULL);
    if (h == INVALID_HANDLE_VALUE)
	return FAIL;

    ok = DeviceIoControl(h, FSCTL_GET_REPARSE_POINT, NULL, 0, buf, *buf_len,
	    buf_len, NULL);
    CloseHandle(h);

    return ok ? OK : FAIL;
}

    static int
wstat_symlink_aware(const WCHAR *name, stat_T *stp)
{
#if (defined(_MSC_VER) && (_MSC_VER < 1900)) || defined(__MINGW32__)
    // Work around for VC12 or earlier (and MinGW). _wstat() can't handle
    // symlinks properly.
    // VC9 or earlier: _wstat() doesn't support a symlink at all. It retrieves
    // status of a symlink itself.
    // VC10: _wstat() supports a symlink to a normal file, but it doesn't
    // support a symlink to a directory (always returns an error).
    // VC11 and VC12: _wstat() doesn't return an error for a symlink to a
    // directory, but it doesn't set S_IFDIR flag.
    // MinGW: Same as VC9.
    int			n;
    BOOL		is_symlink = FALSE;
    HANDLE		hFind, h;
    DWORD		attr = 0;
    WIN32_FIND_DATAW	findDataW;

    hFind = FindFirstFileW(name, &findDataW);
    if (hFind != INVALID_HANDLE_VALUE)
    {
	attr = findDataW.dwFileAttributes;
	if ((attr & FILE_ATTRIBUTE_REPARSE_POINT)
		&& (findDataW.dwReserved0 == IO_REPARSE_TAG_SYMLINK))
	    is_symlink = TRUE;
	FindClose(hFind);
    }
    if (is_symlink)
    {
	h = CreateFileW(name, FILE_READ_ATTRIBUTES,
		FILE_SHARE_READ | FILE_SHARE_WRITE, NULL,
		OPEN_EXISTING,
		(attr & FILE_ATTRIBUTE_DIRECTORY)
					    ? FILE_FLAG_BACKUP_SEMANTICS : 0,
		NULL);
	if (h != INVALID_HANDLE_VALUE)
	{
	    int	    fd;

	    fd = _open_osfhandle((intptr_t)h, _O_RDONLY);
	    n = _fstat(fd, (struct _stat *)stp);
	    if ((n == 0) && (attr & FILE_ATTRIBUTE_DIRECTORY))
		stp->st_mode = (stp->st_mode & ~S_IFREG) | S_IFDIR;
	    _close(fd);
	    return n;
	}
    }
#endif
    return _wstat(name, (struct _stat *)stp);
}

    char_u *
resolve_appexeclink(char_u *fname)
{
    DWORD		attr = 0;
    int			idx;
    WCHAR		*p, *end, *wname;
    // The buffer size is arbitrarily chosen to be "big enough" (TM), the
    // ceiling should be around 16k.
    char_u		buf[4096];
    DWORD		buf_len = sizeof(buf);
    REPARSE_DATA_BUFFER *rb = (REPARSE_DATA_BUFFER *)buf;

    wname = enc_to_utf16(fname, NULL);
    if (wname == NULL)
	return NULL;

    attr = GetFileAttributesW(wname);
    if (attr == INVALID_FILE_ATTRIBUTES ||
	    (attr & FILE_ATTRIBUTE_REPARSE_POINT) == 0)
    {
	vim_free(wname);
	return NULL;
    }

    // The applinks are similar to symlinks but with a huge difference: they can
    // only be executed, any other I/O operation on them is bound to fail with
    // ERROR_FILE_NOT_FOUND even though the file exists.
    if (read_reparse_point(wname, buf, &buf_len) == FAIL)
    {
	vim_free(wname);
	return NULL;
    }
    vim_free(wname);

    if (rb->ReparseTag != IO_REPARSE_TAG_APPEXECLINK)
	return NULL;

    // The (undocumented) reparse buffer contains a set of N null-terminated
    // Unicode strings, the application path is stored in the third one.
    if (rb->AppExecLinkReparseBuffer.StringCount < 3)
	return NULL;

    p = rb->AppExecLinkReparseBuffer.StringList;
    end = p + rb->ReparseDataLength / sizeof(WCHAR);
    for (idx = 0; p < end
	    && idx < (int)rb->AppExecLinkReparseBuffer.StringCount
	    && idx != 2; )
    {
	if (*p++ == L'\0')
	    ++idx;
    }

    return utf16_to_enc(p, NULL);
}

/*
 * stat() can't handle a trailing '/' or '\', remove it first.
 */
    int
vim_stat(const char *name, stat_T *stp)
{
    // WinNT and later can use _MAX_PATH wide characters for a pathname, which
    // means that the maximum pathname is _MAX_PATH * 3 bytes when 'enc' is
    // UTF-8.
    char_u	buf[_MAX_PATH * 3 + 1];
    char_u	*p;
    WCHAR	*wp;
    int		n;

    vim_strncpy((char_u *)buf, (char_u *)name, sizeof(buf) - 1);
    p = buf + STRLEN(buf);
    if (p > buf)
	MB_PTR_BACK(buf, p);

    // Remove trailing '\\' except root path.
    if (p > buf && (*p == '\\' || *p == '/') && p[-1] != ':')
	*p = NUL;

    if ((buf[0] == '\\' && buf[1] == '\\') || (buf[0] == '/' && buf[1] == '/'))
    {
	// UNC root path must be followed by '\\'.
	p = vim_strpbrk(buf + 2, (char_u *)"\\/");
	if (p != NULL)
	{
	    p = vim_strpbrk(p + 1, (char_u *)"\\/");
	    if (p == NULL)
		STRCAT(buf, "\\");
	}
    }

    wp = enc_to_utf16(buf, NULL);
    if (wp == NULL)
	return -1;

    n = wstat_symlink_aware(wp, stp);
    vim_free(wp);
    return n;
}

#if (defined(FEAT_GUI_MSWIN) && !defined(VIMDLL)) || defined(PROTO)
    void
mch_settmode(tmode_T tmode UNUSED)
{
    // nothing to do
}

    int
mch_get_shellsize(void)
{
    // never used
    return OK;
}

    void
mch_set_shellsize(void)
{
    // never used
}

/*
 * Rows and/or Columns has changed.
 */
    void
mch_new_shellsize(void)
{
    // never used
}

#endif

/*
 * We have no job control, so fake it by starting a new shell.
 */
    void
mch_suspend(void)
{
    suspend_shell();
}

#if defined(USE_MCH_ERRMSG) || defined(PROTO)

# ifdef display_errors
#  undef display_errors
# endif

/*
 * Display the saved error message(s).
 */
    void
display_errors(void)
{
# ifdef FEAT_GUI
    char_u *p;

#  ifdef VIMDLL
    if (gui.in_use || gui.starting)
#  endif
    {
	if (error_ga.ga_data != NULL)
	{
	    // avoid putting up a message box with blanks only
	    for (p = (char_u *)error_ga.ga_data; *p; ++p)
		if (!SAFE_isspace(*p))
		{
		    // Only use a dialog when not using --gui-dialog-file:
		    // write text to a file.
		    if (!gui_dialog_log((char_u *)"Errors", p))
			(void)gui_mch_dialog(
				     gui.starting ? VIM_INFO :
					     VIM_ERROR,
				     gui.starting ? (char_u *)_("Message") :
					     (char_u *)_("Error"),
					     p, (char_u *)_("&Ok"),
					1, NULL, FALSE);
		    break;
		}
	    ga_clear(&error_ga);
	}
	return;
    }
# endif
# if !defined(FEAT_GUI) || defined(VIMDLL)
    FlushFileBuffers(GetStdHandle(STD_ERROR_HANDLE));
# endif
}
#endif


/*
 * Return TRUE if "p" contain a wildcard that can be expanded by
 * dos_expandpath().
 */
    int
mch_has_exp_wildcard(char_u *p)
{
    for ( ; *p; MB_PTR_ADV(p))
    {
	if (vim_strchr((char_u *)"?*[", *p) != NULL
		|| (*p == '~' && p[1] != NUL))
	    return TRUE;
    }
    return FALSE;
}

/*
 * Return TRUE if "p" contain a wildcard or a "~1" kind of thing (could be a
 * shortened file name).
 */
    int
mch_has_wildcard(char_u *p)
{
    for ( ; *p; MB_PTR_ADV(p))
    {
	if (vim_strchr((char_u *)
#ifdef VIM_BACKTICK
				    "?*$[`"
#else
				    "?*$["
#endif
						, *p) != NULL
		|| (*p == '~' && p[1] != NUL))
	    return TRUE;
    }
    return FALSE;
}


/*
 * The normal _chdir() does not change the default drive.  This one does.
 * Returning 0 implies success; -1 implies failure.
 */
    int
mch_chdir(char *path)
{
    WCHAR   *p;
    int	    n;

    if (path[0] == NUL)		// just checking...
	return -1;

    if (p_verbose >= 5)
    {
	verbose_enter();
	smsg("chdir(%s)", path);
	verbose_leave();
    }
    if (SAFE_isalpha(path[0]) && path[1] == ':')	// has a drive name
    {
	// If we can change to the drive, skip that part of the path.  If we
	// can't then the current directory may be invalid, try using chdir()
	// with the whole path.
	if (_chdrive(TOLOWER_ASC(path[0]) - 'a' + 1) == 0)
	    path += 2;
    }

    if (*path == NUL)		// drive name only
	return 0;

    p = enc_to_utf16((char_u *)path, NULL);
    if (p == NULL)
	return -1;

    n = _wchdir(p);
    vim_free(p);
    return n;
}


#if defined(FEAT_GUI_MSWIN) && !defined(VIMDLL)
/*
 * return non-zero if a character is available
 */
    int
mch_char_avail(void)
{
    // never used
    return TRUE;
}

# if defined(FEAT_TERMINAL) || defined(PROTO)
/*
 * Check for any pending input or messages.
 */
    int
mch_check_messages(void)
{
    // TODO: check for messages
    return TRUE;
}
# endif
#endif


#if defined(FEAT_LIBCALL) || defined(PROTO)
/*
 * Call a DLL routine which takes either a string or int param
 * and returns an allocated string.
 * Return OK if it worked, FAIL if not.
 */
typedef LPTSTR (*MYSTRPROCSTR)(LPTSTR);
typedef LPTSTR (*MYINTPROCSTR)(int);
typedef int (*MYSTRPROCINT)(LPTSTR);
typedef int (*MYINTPROCINT)(int);

/*
 * Check if a pointer points to a valid NUL terminated string.
 * Return the length of the string, including terminating NUL.
 * Returns 0 for an invalid pointer, 1 for an empty string.
 */
    static size_t
check_str_len(char_u *str)
{
    SYSTEM_INFO			si;
    MEMORY_BASIC_INFORMATION	mbi;
    size_t			length = 0;
    size_t			i;
    const char_u		*p;

    // get page size
    GetSystemInfo(&si);

    // get memory information
    if (!VirtualQuery(str, &mbi, sizeof(mbi)))
	return 0;

    // pre cast these (typing savers)
    long_u dwStr = (long_u)str;
    long_u dwBaseAddress = (long_u)mbi.BaseAddress;

    // get start address of page that str is on
    long_u strPage = dwStr - (dwStr - dwBaseAddress) % si.dwPageSize;

    // get length from str to end of page
    long_u pageLength = si.dwPageSize - (dwStr - strPage);

    for (p = str; !IsBadReadPtr(p, (UINT)pageLength);
	    p += pageLength, pageLength = si.dwPageSize)
	for (i = 0; i < pageLength; ++i, ++length)
	    if (p[i] == NUL)
		return length + 1;

    return 0;
}

/*
 * Passed to do_in_runtimepath() to load a vim.ico file.
 */
    static void
mch_icon_load_cb(char_u *fname, void *cookie)
{
    HANDLE *h = (HANDLE *)cookie;

    *h = LoadImage(NULL,
		   (LPSTR)fname,
		   IMAGE_ICON,
		   64,
		   64,
		   LR_LOADFROMFILE | LR_LOADMAP3DCOLORS);
}

/*
 * Try loading an icon file from 'runtimepath'.
 */
    int
mch_icon_load(HANDLE *iconp)
{
    return do_in_runtimepath((char_u *)"bitmaps/vim.ico",
						  0, mch_icon_load_cb, iconp);
}

    int
mch_libcall(
    char_u	*libname,
    char_u	*funcname,
    char_u	*argstring,	// NULL when using a argint
    int		argint,
    char_u	**string_result,// NULL when using number_result
    int		*number_result)
{
    HINSTANCE		hinstLib;
    MYSTRPROCSTR	ProcAdd;
    MYINTPROCSTR	ProcAddI;
    char_u		*retval_str = NULL;
    int			retval_int = 0;
    size_t		len;

    BOOL fRunTimeLinkSuccess = FALSE;

    // Get a handle to the DLL module.
    hinstLib = vimLoadLib((char *)libname);

    // If the handle is valid, try to get the function address.
    if (hinstLib != NULL)
    {
# ifdef HAVE_TRY_EXCEPT
	__try
	{
# endif
	if (argstring != NULL)
	{
	    // Call with string argument
	    ProcAdd = (MYSTRPROCSTR)GetProcAddress(hinstLib, (LPCSTR)funcname);
	    if ((fRunTimeLinkSuccess = (ProcAdd != NULL)) != 0)
	    {
		if (string_result == NULL)
		    retval_int = ((MYSTRPROCINT)ProcAdd)((LPSTR)argstring);
		else
		    retval_str = (char_u *)(ProcAdd)((LPSTR)argstring);
	    }
	}
	else
	{
	    // Call with number argument
	    ProcAddI = (MYINTPROCSTR) GetProcAddress(hinstLib, (LPCSTR)funcname);
	    if ((fRunTimeLinkSuccess = (ProcAddI != NULL)) != 0)
	    {
		if (string_result == NULL)
		    retval_int = ((MYINTPROCINT)ProcAddI)(argint);
		else
		    retval_str = (char_u *)(ProcAddI)(argint);
	    }
	}

	// Save the string before we free the library.
	// Assume that a "1" result is an illegal pointer.
	if (string_result == NULL)
	    *number_result = retval_int;
	else if (retval_str != NULL
		&& (len = check_str_len(retval_str)) > 0)
	{
	    *string_result = alloc(len);
	    if (*string_result != NULL)
		mch_memmove(*string_result, retval_str, len);
	}

# ifdef HAVE_TRY_EXCEPT
	}
	__except(EXCEPTION_EXECUTE_HANDLER)
	{
	    if (GetExceptionCode() == EXCEPTION_STACK_OVERFLOW)
		_resetstkoflw();
	    fRunTimeLinkSuccess = 0;
	}
# endif

	// Free the DLL module.
	(void)FreeLibrary(hinstLib);
    }

    if (!fRunTimeLinkSuccess)
    {
	semsg(_(e_library_call_failed_for_str), funcname);
	return FAIL;
    }

    return OK;
}
#endif

/*
 * Debugging helper: expose the MCH_WRITE_DUMP stuff to other modules
 */
    void
DumpPutS(const char *psz UNUSED)
{
#ifdef MCH_WRITE_DUMP
    if (fdDump)
    {
	fputs(psz, fdDump);
	if (psz[strlen(psz) - 1] != '\n')
	    fputc('\n', fdDump);
	fflush(fdDump);
    }
#endif
}

#ifdef _DEBUG

void __cdecl
Trace(
    char *pszFormat,
    ...)
{
    CHAR szBuff[2048];
    va_list args;

    va_start(args, pszFormat);
    vsprintf(szBuff, pszFormat, args);
    va_end(args);

    OutputDebugString(szBuff);
}

#endif //_DEBUG

#if !defined(FEAT_GUI) || defined(VIMDLL) || defined(PROTO)
extern HWND g_hWnd;	// This is in os_win32.c.

/*
 * Showing the printer dialog is tricky since we have no GUI
 * window to parent it. The following routines are needed to
 * get the window parenting and Z-order to work properly.
 */
    static void
GetConsoleHwnd(void)
{
    // Skip if it's already set.
    if (s_hwnd != 0)
	return;

    // Window handle may have been found by init code (Windows NT only)
    if (g_hWnd != 0)
    {
	s_hwnd = g_hWnd;
	return;
    }

    s_hwnd = GetConsoleWindow();
}

/*
 * Console implementation of ":winpos".
 */
    int
mch_get_winpos(int *x, int *y)
{
    RECT  rect;

    GetConsoleHwnd();
    GetWindowRect(s_hwnd, &rect);
    *x = rect.left;
    *y = rect.top;
    return OK;
}

/*
 * Console implementation of ":winpos x y".
 */
    void
mch_set_winpos(int x, int y)
{
    GetConsoleHwnd();
    SetWindowPos(s_hwnd, NULL, x, y, 0, 0,
		 SWP_NOZORDER | SWP_NOSIZE | SWP_NOACTIVATE);
}
#endif

#if (defined(FEAT_PRINTER) && !defined(FEAT_POSTSCRIPT)) || defined(PROTO)

//=================================================================
// Win32 printer stuff

static HFONT		prt_font_handles[2][2][2];
static PRINTDLGW	prt_dlg;
static const int	boldface[2] = {FW_REGULAR, FW_BOLD};
static TEXTMETRIC	prt_tm;
static int		prt_line_height;
static int		prt_number_width;
static int		prt_left_margin;
static int		prt_right_margin;
static int		prt_top_margin;
static char_u		szAppName[] = TEXT("VIM");
static HWND		hDlgPrint;
static int		*bUserAbort = NULL;
static char_u		*prt_name = NULL;

// Defines which are also in vim.rc.
# define IDC_BOX1		400
# define IDC_PRINTTEXT1		401
# define IDC_PRINTTEXT2		402
# define IDC_PROGRESS		403

    static BOOL
vimSetDlgItemText(HWND hDlg, int nIDDlgItem, char_u *s)
{
    WCHAR   *wp;
    BOOL    ret;

    wp = enc_to_utf16(s, NULL);
    if (wp == NULL)
	return FALSE;

    ret = SetDlgItemTextW(hDlg, nIDDlgItem, wp);
    vim_free(wp);
    return ret;
}

/*
 * Convert BGR to RGB for Windows GDI calls
 */
    static COLORREF
swap_me(COLORREF colorref)
{
    int  temp;
    char *ptr = (char *)&colorref;

    temp = *(ptr);
    *(ptr ) = *(ptr + 2);
    *(ptr + 2) = temp;
    return colorref;
}

    static INT_PTR CALLBACK
PrintDlgProc(
	HWND hDlg,
	UINT message,
	WPARAM wParam UNUSED,
	LPARAM lParam UNUSED)
{
# ifdef FEAT_GETTEXT
    NONCLIENTMETRICS nm;
    static HFONT hfont;
# endif

    switch (message)
    {
	case WM_INITDIALOG:
# ifdef FEAT_GETTEXT
	    nm.cbSize = sizeof(NONCLIENTMETRICS);
	    if (SystemParametersInfo(
			SPI_GETNONCLIENTMETRICS,
			sizeof(NONCLIENTMETRICS),
			&nm,
			0))
	    {
		char buff[MAX_PATH];
		int i;

		// Translate the dialog texts
		hfont = CreateFontIndirect(&nm.lfMessageFont);
		for (i = IDC_PRINTTEXT1; i <= IDC_PROGRESS; i++)
		{
		    SendDlgItemMessage(hDlg, i, WM_SETFONT, (WPARAM)hfont, 1);
		    if (GetDlgItemText(hDlg,i, buff, sizeof(buff)))
			vimSetDlgItemText(hDlg,i, (char_u *)_(buff));
		}
		SendDlgItemMessage(hDlg, IDCANCEL,
						WM_SETFONT, (WPARAM)hfont, 1);
		if (GetDlgItemText(hDlg,IDCANCEL, buff, sizeof(buff)))
		    vimSetDlgItemText(hDlg,IDCANCEL, (char_u *)_(buff));
	    }
# endif
	    SetWindowText(hDlg, (LPCSTR)szAppName);
	    if (prt_name != NULL)
	    {
		vimSetDlgItemText(hDlg, IDC_PRINTTEXT2, (char_u *)prt_name);
		VIM_CLEAR(prt_name);
	    }
	    EnableMenuItem(GetSystemMenu(hDlg, FALSE), SC_CLOSE, MF_GRAYED);
# if !defined(FEAT_GUI) || defined(VIMDLL)
#  ifdef VIMDLL
	    if (!gui.in_use)
#  endif
		BringWindowToTop(s_hwnd);
# endif
	    return TRUE;

	case WM_COMMAND:
	    *bUserAbort = TRUE;
	    EnableWindow(GetParent(hDlg), TRUE);
	    DestroyWindow(hDlg);
	    hDlgPrint = NULL;
# ifdef FEAT_GETTEXT
	    DeleteObject(hfont);
# endif
	    return TRUE;
    }
    return FALSE;
}

    static BOOL CALLBACK
AbortProc(HDC hdcPrn UNUSED, int iCode UNUSED)
{
    MSG msg;

    while (!*bUserAbort && PeekMessageW(&msg, NULL, 0, 0, PM_REMOVE))
    {
	if (!hDlgPrint || !IsDialogMessageW(hDlgPrint, &msg))
	{
	    TranslateMessage(&msg);
	    DispatchMessageW(&msg);
	}
    }
    return !*bUserAbort;
}

# if !defined(FEAT_GUI) || defined(VIMDLL)

    static UINT_PTR CALLBACK
PrintHookProc(
	HWND hDlg,	// handle to dialog box
	UINT uiMsg,	// message identifier
	WPARAM wParam UNUSED,	// message parameter
	LPARAM lParam	// message parameter
	)
{
    HWND	hwndOwner;
    RECT	rc, rcDlg, rcOwner;
    PRINTDLGW	*pPD;

    if (uiMsg != WM_INITDIALOG)
	return FALSE;

    // Get the owner window and dialog box rectangles.
    if ((hwndOwner = GetParent(hDlg)) == NULL)
	hwndOwner = GetDesktopWindow();

    GetWindowRect(hwndOwner, &rcOwner);
    GetWindowRect(hDlg, &rcDlg);
    CopyRect(&rc, &rcOwner);

    // Offset the owner and dialog box rectangles so that
    // right and bottom values represent the width and
    // height, and then offset the owner again to discard
    // space taken up by the dialog box.

    OffsetRect(&rcDlg, -rcDlg.left, -rcDlg.top);
    OffsetRect(&rc, -rc.left, -rc.top);
    OffsetRect(&rc, -rcDlg.right, -rcDlg.bottom);

    // The new position is the sum of half the remaining
    // space and the owner's original position.

    SetWindowPos(hDlg,
	    HWND_TOP,
	    rcOwner.left + (rc.right / 2),
	    rcOwner.top + (rc.bottom / 2),
	    0, 0,		// ignores size arguments
	    SWP_NOSIZE);

    //  tackle the printdlg copiesctrl problem
    pPD = (PRINTDLGW *)lParam;
    pPD->nCopies = (WORD)pPD->lCustData;
    SetDlgItemInt( hDlg, edt3, pPD->nCopies, FALSE );
    //  Bring the window to top
    BringWindowToTop(GetParent(hDlg));
    SetForegroundWindow(hDlg);

    return FALSE;
}
# endif

    void
mch_print_cleanup(void)
{
    int pifItalic;
    int pifBold;
    int pifUnderline;

    for (pifBold = 0; pifBold <= 1; pifBold++)
	for (pifItalic = 0; pifItalic <= 1; pifItalic++)
	    for (pifUnderline = 0; pifUnderline <= 1; pifUnderline++)
		DeleteObject(prt_font_handles[pifBold][pifItalic][pifUnderline]);

    if (prt_dlg.hDC != NULL)
	DeleteDC(prt_dlg.hDC);
    if (!*bUserAbort)
	SendMessage(hDlgPrint, WM_COMMAND, 0, 0);
}

    static int
to_device_units(int idx, int dpi, int physsize, int offset, int def_number)
{
    int		ret = 0;
    int		u;
    int		nr;

    u = prt_get_unit(idx);
    if (u == PRT_UNIT_NONE)
    {
	u = PRT_UNIT_PERC;
	nr = def_number;
    }
    else
	nr = printer_opts[idx].number;

    switch (u)
    {
	case PRT_UNIT_PERC:
	    ret = (physsize * nr) / 100;
	    break;
	case PRT_UNIT_INCH:
	    ret = (nr * dpi);
	    break;
	case PRT_UNIT_MM:
	    ret = (nr * 10 * dpi) / 254;
	    break;
	case PRT_UNIT_POINT:
	    ret = (nr * 10 * dpi) / 720;
	    break;
    }

    if (ret < offset)
	return 0;
    else
	return ret - offset;
}

    static int
prt_get_cpl(void)
{
    int		hr;
    int		phyw;
    int		dvoff;
    int		rev_offset;
    int		dpi;

    GetTextMetrics(prt_dlg.hDC, &prt_tm);
    prt_line_height = prt_tm.tmHeight + prt_tm.tmExternalLeading;

    hr	    = GetDeviceCaps(prt_dlg.hDC, HORZRES);
    phyw    = GetDeviceCaps(prt_dlg.hDC, PHYSICALWIDTH);
    dvoff   = GetDeviceCaps(prt_dlg.hDC, PHYSICALOFFSETX);
    dpi	    = GetDeviceCaps(prt_dlg.hDC, LOGPIXELSX);

    rev_offset = phyw - (dvoff + hr);

    prt_left_margin = to_device_units(OPT_PRINT_LEFT, dpi, phyw, dvoff, 10);
    if (prt_use_number())
    {
	prt_number_width = PRINT_NUMBER_WIDTH * prt_tm.tmAveCharWidth;
	prt_left_margin += prt_number_width;
    }
    else
	prt_number_width = 0;

    prt_right_margin = hr - to_device_units(OPT_PRINT_RIGHT, dpi, phyw,
							       rev_offset, 5);

    return (prt_right_margin - prt_left_margin) / prt_tm.tmAveCharWidth;
}

    static int
prt_get_lpp(void)
{
    int vr;
    int phyw;
    int dvoff;
    int rev_offset;
    int	bottom_margin;
    int	dpi;

    vr	    = GetDeviceCaps(prt_dlg.hDC, VERTRES);
    phyw    = GetDeviceCaps(prt_dlg.hDC, PHYSICALHEIGHT);
    dvoff   = GetDeviceCaps(prt_dlg.hDC, PHYSICALOFFSETY);
    dpi	    = GetDeviceCaps(prt_dlg.hDC, LOGPIXELSY);

    rev_offset = phyw - (dvoff + vr);

    prt_top_margin = to_device_units(OPT_PRINT_TOP, dpi, phyw, dvoff, 5);

    // adjust top margin if there is a header
    prt_top_margin += prt_line_height * prt_header_height();

    bottom_margin = vr - to_device_units(OPT_PRINT_BOT, dpi, phyw,
							       rev_offset, 5);

    return (bottom_margin - prt_top_margin) / prt_line_height;
}

    int
mch_print_init(prt_settings_T *psettings, char_u *jobname, int forceit)
{
    static HGLOBAL	stored_dm = NULL;
    static HGLOBAL	stored_devn = NULL;
    static int		stored_nCopies = 1;
    static int		stored_nFlags = 0;

    LOGFONTW		fLogFont;
    int			pifItalic;
    int			pifBold;
    int			pifUnderline;

    DEVMODEW		*mem;
    DEVNAMES		*devname;
    int			i;
    DWORD		err;

    bUserAbort = &(psettings->user_abort);
    CLEAR_FIELD(prt_dlg);
    prt_dlg.lStructSize = sizeof(PRINTDLGW);
# if !defined(FEAT_GUI) || defined(VIMDLL)
#  ifdef VIMDLL
    if (!gui.in_use)
#  endif
	GetConsoleHwnd();	    // get value of s_hwnd
# endif
    prt_dlg.hwndOwner = s_hwnd;
    prt_dlg.Flags = PD_NOPAGENUMS | PD_NOSELECTION | PD_RETURNDC;
    if (!forceit)
    {
	prt_dlg.hDevMode = stored_dm;
	prt_dlg.hDevNames = stored_devn;
	prt_dlg.lCustData = stored_nCopies; // work around bug in print dialog
# if !defined(FEAT_GUI) || defined(VIMDLL)
#  ifdef VIMDLL
	if (!gui.in_use)
#  endif
	{
	    /*
	     * Use hook to prevent console window being sent to back
	     */
	    prt_dlg.lpfnPrintHook = PrintHookProc;
	    prt_dlg.Flags |= PD_ENABLEPRINTHOOK;
	}
# endif
	prt_dlg.Flags |= stored_nFlags;
    }

    /*
     * If bang present, return default printer setup with no dialog
     * never show dialog if we are running over telnet
     */
    if (forceit
# if !defined(FEAT_GUI) || defined(VIMDLL)
#  ifdef VIMDLL
	    || (!gui.in_use && !term_console)
#  else
	    || !term_console
#  endif
# endif
	    )
    {
	prt_dlg.Flags |= PD_RETURNDEFAULT;
	/*
	 * MSDN suggests setting the first parameter to WINSPOOL for
	 * NT, but NULL appears to work just as well.
	 */
	if (*p_pdev != NUL)
	    prt_dlg.hDC = CreateDC(NULL, (LPCSTR)p_pdev, NULL, NULL);
	else
	{
	    prt_dlg.Flags |= PD_RETURNDEFAULT;
	    if (PrintDlgW(&prt_dlg) == 0)
		goto init_fail_dlg;
	}
    }
    else if (PrintDlgW(&prt_dlg) == 0)
	goto init_fail_dlg;
    else
    {
	/*
	 * keep the previous driver context
	 */
	stored_dm = prt_dlg.hDevMode;
	stored_devn = prt_dlg.hDevNames;
	stored_nFlags = prt_dlg.Flags;
	stored_nCopies = prt_dlg.nCopies;
    }

    if (prt_dlg.hDC == NULL)
    {
	emsg(_(e_printer_selection_failed));
	mch_print_cleanup();
	return FALSE;
    }

    // Not all printer drivers report the support of color (or grey) in the
    // same way.  Let's set has_color if there appears to be some way to print
    // more than B&W.
    i = GetDeviceCaps(prt_dlg.hDC, NUMCOLORS);
    psettings->has_color = (GetDeviceCaps(prt_dlg.hDC, BITSPIXEL) > 1
				   || GetDeviceCaps(prt_dlg.hDC, PLANES) > 1
				   || i > 2 || i == -1);

    // Ensure all font styles are baseline aligned
    SetTextAlign(prt_dlg.hDC, TA_BASELINE|TA_LEFT);

    /*
     * On some windows systems the nCopies parameter is not
     * passed back correctly. It must be retrieved from the
     * hDevMode struct.
     */
    mem = (DEVMODEW *)GlobalLock(prt_dlg.hDevMode);
    if (mem != NULL)
    {
	if (mem->dmCopies != 1)
	    stored_nCopies = mem->dmCopies;
	if ((mem->dmFields & DM_DUPLEX) && (mem->dmDuplex & ~DMDUP_SIMPLEX))
	    psettings->duplex = TRUE;
	if ((mem->dmFields & DM_COLOR) && (mem->dmColor & DMCOLOR_COLOR))
	    psettings->has_color = TRUE;
    }
    GlobalUnlock(prt_dlg.hDevMode);

    devname = (DEVNAMES *)GlobalLock(prt_dlg.hDevNames);
    if (devname != 0)
    {
	WCHAR	*wprinter_name = (WCHAR *)devname + devname->wDeviceOffset;
	WCHAR	*wport_name = (WCHAR *)devname + devname->wOutputOffset;
	char_u	*text = (char_u *)_("to %s on %s");
	char_u  *printer_name = utf16_to_enc(wprinter_name, NULL);
	char_u	*port_name = utf16_to_enc(wport_name, NULL);

	if (printer_name != NULL && port_name != NULL)
	    prt_name = alloc(STRLEN(printer_name)
					   + STRLEN(port_name) + STRLEN(text));
	if (prt_name != NULL)
	    wsprintf((char *)prt_name, (const char *)text,
		    printer_name, port_name);
	vim_free(printer_name);
	vim_free(port_name);
    }
    GlobalUnlock(prt_dlg.hDevNames);

    /*
     * Initialise the font according to 'printfont'
     */
    CLEAR_FIELD(fLogFont);
    if (get_logfont(&fLogFont, p_pfn, prt_dlg.hDC, TRUE) == FAIL)
    {
	semsg(_(e_unknown_printer_font_str), p_pfn);
	mch_print_cleanup();
	return FALSE;
    }

    for (pifBold = 0; pifBold <= 1; pifBold++)
	for (pifItalic = 0; pifItalic <= 1; pifItalic++)
	    for (pifUnderline = 0; pifUnderline <= 1; pifUnderline++)
	    {
		fLogFont.lfWeight =  boldface[pifBold];
		fLogFont.lfItalic = pifItalic;
		fLogFont.lfUnderline = pifUnderline;
		prt_font_handles[pifBold][pifItalic][pifUnderline]
					      = CreateFontIndirectW(&fLogFont);
	    }

    SetBkMode(prt_dlg.hDC, OPAQUE);
    SelectObject(prt_dlg.hDC, prt_font_handles[0][0][0]);

    /*
     * Fill in the settings struct
     */
    psettings->chars_per_line = prt_get_cpl();
    psettings->lines_per_page = prt_get_lpp();
    if (prt_dlg.Flags & PD_USEDEVMODECOPIESANDCOLLATE)
    {
	psettings->n_collated_copies = (prt_dlg.Flags & PD_COLLATE)
						    ? prt_dlg.nCopies : 1;
	psettings->n_uncollated_copies = (prt_dlg.Flags & PD_COLLATE)
						    ? 1 : prt_dlg.nCopies;

	if (psettings->n_collated_copies == 0)
	    psettings->n_collated_copies = 1;

	if (psettings->n_uncollated_copies == 0)
	    psettings->n_uncollated_copies = 1;
    }
    else
    {
	psettings->n_collated_copies = 1;
	psettings->n_uncollated_copies = 1;
    }

    psettings->jobname = jobname;

    return TRUE;

init_fail_dlg:
    err = CommDlgExtendedError();
    if (err)
    {
	char_u *buf;

	// I suspect FormatMessage() doesn't work for values returned by
	// CommDlgExtendedError().  What does?
	FormatMessage(FORMAT_MESSAGE_ALLOCATE_BUFFER |
		FORMAT_MESSAGE_FROM_SYSTEM |
		FORMAT_MESSAGE_IGNORE_INSERTS,
		NULL, err, 0, (LPTSTR)(&buf), 0, NULL);
	semsg(_(e_print_error_str),
		buf == NULL ? (char_u *)_("Unknown") : buf);
	LocalFree((LPVOID)(buf));
    }
    else
	msg_clr_eos(); // Maybe canceled

    mch_print_cleanup();
    return FALSE;
}


    int
mch_print_begin(prt_settings_T *psettings)
{
    int			ret = 0;
    char		szBuffer[300];
    WCHAR		*wp;

    hDlgPrint = CreateDialog(g_hinst, TEXT("PrintDlgBox"),
					     prt_dlg.hwndOwner, PrintDlgProc);
    SetAbortProc(prt_dlg.hDC, AbortProc);
    wsprintf(szBuffer, _("Printing '%s'"), gettail(psettings->jobname));
    vimSetDlgItemText(hDlgPrint, IDC_PRINTTEXT1, (char_u *)szBuffer);

    wp = enc_to_utf16(psettings->jobname, NULL);
    if (wp != NULL)
    {
	DOCINFOW	di;

	CLEAR_FIELD(di);
	di.cbSize = sizeof(di);
	di.lpszDocName = wp;
	ret = StartDocW(prt_dlg.hDC, &di);
	vim_free(wp);
    }

# ifdef FEAT_GUI
    // Give focus back to main window (when using MDI).
#  ifdef VIMDLL
    if (gui.in_use)
#  endif
	SetFocus(s_hwnd);
# endif

    return (ret > 0);
}

    void
mch_print_end(prt_settings_T *psettings UNUSED)
{
    EndDoc(prt_dlg.hDC);
    if (!*bUserAbort)
	SendMessage(hDlgPrint, WM_COMMAND, 0, 0);
}

    int
mch_print_end_page(void)
{
    return (EndPage(prt_dlg.hDC) > 0);
}

    int
mch_print_begin_page(char_u *msg)
{
    if (msg != NULL)
	vimSetDlgItemText(hDlgPrint, IDC_PROGRESS, msg);
    return (StartPage(prt_dlg.hDC) > 0);
}

    int
mch_print_blank_page(void)
{
    return (mch_print_begin_page(NULL) ? (mch_print_end_page()) : FALSE);
}

static int prt_pos_x = 0;
static int prt_pos_y = 0;

    void
mch_print_start_line(int margin, int page_line)
{
    if (margin)
	prt_pos_x = -prt_number_width;
    else
	prt_pos_x = 0;
    prt_pos_y = page_line * prt_line_height
				 + prt_tm.tmAscent + prt_tm.tmExternalLeading;
}

    int
mch_print_text_out(char_u *p, int len)
{
    SIZE	sz;
    WCHAR	*wp;
    int		wlen = len;
    int		ret = FALSE;

    wp = enc_to_utf16(p, &wlen);
    if (wp == NULL)
	return FALSE;

    TextOutW(prt_dlg.hDC, prt_pos_x + prt_left_margin,
	    prt_pos_y + prt_top_margin, wp, wlen);
    GetTextExtentPoint32W(prt_dlg.hDC, wp, wlen, &sz);
    vim_free(wp);
    prt_pos_x += (sz.cx - prt_tm.tmOverhang);
    // This is wrong when printing spaces for a TAB.
    if (p[len] != NUL)
    {
	wlen = mb_ptr2len(p + len);
	wp = enc_to_utf16(p + len, &wlen);
	if (wp != NULL)
	{
	    GetTextExtentPoint32W(prt_dlg.hDC, wp, 1, &sz);
	    ret = (prt_pos_x + prt_left_margin + sz.cx > prt_right_margin);
	    vim_free(wp);
	}
    }
    return ret;
}

    void
mch_print_set_font(int iBold, int iItalic, int iUnderline)
{
    SelectObject(prt_dlg.hDC, prt_font_handles[iBold][iItalic][iUnderline]);
}

    void
mch_print_set_bg(long_u bgcol)
{
    SetBkColor(prt_dlg.hDC, GetNearestColor(prt_dlg.hDC,
						   swap_me((COLORREF)bgcol)));
    /*
     * With a white background we can draw characters transparent, which is
     * good for italic characters that overlap to the next char cell.
     */
    if (bgcol == 0xffffffUL)
	SetBkMode(prt_dlg.hDC, TRANSPARENT);
    else
	SetBkMode(prt_dlg.hDC, OPAQUE);
}

    void
mch_print_set_fg(long_u fgcol)
{
    SetTextColor(prt_dlg.hDC, GetNearestColor(prt_dlg.hDC,
						   swap_me((COLORREF)fgcol)));
}

#endif // FEAT_PRINTER && !FEAT_POSTSCRIPT



#if defined(FEAT_SHORTCUT) || defined(PROTO)
# ifndef PROTO
#  include <shlobj.h>
# endif

# define is_path_sep(c)	    ((c) == L'\\' || (c) == L'/')

    static int
is_reparse_point_included(LPCWSTR fname)
{
    LPCWSTR	p = fname, q;
    WCHAR	buf[MAX_PATH];
    DWORD	attr;

    if (SAFE_isalpha(p[0]) && p[1] == L':' && is_path_sep(p[2]))
	p += 3;
    else if (is_path_sep(p[0]) && is_path_sep(p[1]))
	p += 2;

    while (*p != L'\0')
    {
	q = wcspbrk(p, L"\\/");
	if (q == NULL)
	    p = q = fname + wcslen(fname);
	else
	    p = q + 1;
	if (q - fname >= MAX_PATH)
	    return FALSE;
	wcsncpy(buf, fname, q - fname);
	buf[q - fname] = L'\0';
	attr = GetFileAttributesW(buf);
	if (attr != INVALID_FILE_ATTRIBUTES
		&& (attr & FILE_ATTRIBUTE_REPARSE_POINT) != 0)
	    return TRUE;
    }
    return FALSE;
}

/*
 * Return the resolved file path, NULL if "fname" is an AppExecLink reparse
 * point, already fully resolved, or it doesn't exists.
 */
    char_u *
resolve_reparse_point(char_u *fname)
{
    HANDLE	    h = INVALID_HANDLE_VALUE;
    DWORD	    size;
    WCHAR	    *p, *wp;
    char_u	    *rfname = NULL;
    WCHAR	    *buff = NULL;

    p = enc_to_utf16(fname, NULL);
    if (p == NULL)
	goto fail;

    if (!is_reparse_point_included(p))
    {
	vim_free(p);
	goto fail;
    }

    h = CreateFileW(p, 0, 0, NULL, OPEN_EXISTING,
	    FILE_FLAG_BACKUP_SEMANTICS, NULL);
    vim_free(p);

    if (h == INVALID_HANDLE_VALUE)
	goto fail;

    size = GetFinalPathNameByHandleW(h, NULL, 0, 0);
    if (size == 0)
	goto fail;
    buff = ALLOC_MULT(WCHAR, size);
    if (buff == NULL)
	goto fail;
    if (GetFinalPathNameByHandleW(h, buff, size, 0) == 0)
	goto fail;

    if (wcsncmp(buff, L"\\\\?\\UNC\\", 8) == 0)
    {
	buff[6] = L'\\';
	wp = buff + 6;
    }
    else if (wcsncmp(buff, L"\\\\?\\", 4) == 0)
	wp = buff + 4;
    else
	wp = buff;

    rfname = utf16_to_enc(wp, NULL);

fail:
    if (h != INVALID_HANDLE_VALUE)
	CloseHandle(h);
    if (buff != NULL)
	vim_free(buff);

    return rfname;
}

/*
 * When "fname" is the name of a shortcut (*.lnk) resolve the file it points
 * to and return that name in allocated memory.
 * Otherwise NULL is returned.
 */
    static char_u *
resolve_shortcut(char_u *fname)
{
    HRESULT		hr;
    IShellLink		*psl = NULL;
    IPersistFile	*ppf = NULL;
    OLECHAR		wsz[MAX_PATH];
    char_u		*rfname = NULL;
    int			len;
    IShellLinkW		*pslw = NULL;
    WIN32_FIND_DATAW	ffdw; // we get those free of charge

    // Check if the file name ends in ".lnk". Avoid calling
    // CoCreateInstance(), it's quite slow.
    if (fname == NULL)
	return rfname;
    len = (int)STRLEN(fname);
    if (len <= 4 || STRNICMP(fname + len - 4, ".lnk", 4) != 0)
	return rfname;

    CoInitialize(NULL);

    // create a link manager object and request its interface
    hr = CoCreateInstance(
	    &CLSID_ShellLink, NULL, CLSCTX_INPROC_SERVER,
	    &IID_IShellLinkW, (void**)&pslw);
    if (hr == S_OK)
    {
	WCHAR	*p = enc_to_utf16(fname, NULL);

	if (p != NULL)
	{
	    // Get a pointer to the IPersistFile interface.
	    hr = pslw->lpVtbl->QueryInterface(
		    pslw, &IID_IPersistFile, (void**)&ppf);
	    if (hr != S_OK)
		goto shortcut_errorw;

	    // "load" the name and resolve the link
	    hr = ppf->lpVtbl->Load(ppf, p, STGM_READ);
	    if (hr != S_OK)
		goto shortcut_errorw;
# if 0  // This makes Vim wait a long time if the target does not exist.
	    hr = pslw->lpVtbl->Resolve(pslw, NULL, SLR_NO_UI);
	    if (hr != S_OK)
		goto shortcut_errorw;
# endif

	    // Get the path to the link target.
	    ZeroMemory(wsz, MAX_PATH * sizeof(WCHAR));
	    hr = pslw->lpVtbl->GetPath(pslw, wsz, MAX_PATH, &ffdw, 0);
	    if (hr == S_OK && wsz[0] != NUL)
		rfname = utf16_to_enc(wsz, NULL);

shortcut_errorw:
	    vim_free(p);
	}
    }

    // Release all interface pointers (both belong to the same object)
    if (ppf != NULL)
	ppf->lpVtbl->Release(ppf);
    if (psl != NULL)
	psl->lpVtbl->Release(psl);
    if (pslw != NULL)
	pslw->lpVtbl->Release(pslw);

    CoUninitialize();
    return rfname;
}

    char_u *
mch_resolve_path(char_u *fname, int reparse_point)
{
    char_u  *path = resolve_shortcut(fname);

    if (path == NULL && reparse_point)
	path = resolve_reparse_point(fname);
    return path;
}
#endif

#if (defined(FEAT_EVAL) && (!defined(FEAT_GUI) || defined(VIMDLL))) || defined(PROTO)
/*
 * Bring ourselves to the foreground.  Does work if the OS doesn't allow it.
 */
    void
win32_set_foreground(void)
{
    GetConsoleHwnd();	    // get value of s_hwnd
    if (s_hwnd != 0)
	SetForegroundWindow(s_hwnd);
}
#endif

#if defined(FEAT_CLIENTSERVER) || defined(PROTO)
/*
 * Client-server code for Vim
 *
 * Originally written by Paul Moore
 */

// In order to handle inter-process messages, we need to have a window. But
// the functions in this module can be called before the main GUI window is
// created (and may also be called in the console version, where there is no
// GUI window at all).
//
// So we create a hidden window, and arrange to destroy it on exit.
HWND message_window = 0;	    // window that's handling messages

# define VIM_CLASSNAME      "VIM_MESSAGES"
# define VIM_CLASSNAME_LEN  (sizeof(VIM_CLASSNAME) - 1)

// Timeout for sending a message to another Vim instance.  Normally this works
// instantly, but it may hang when the other Vim instance is halted.
# define SENDMESSAGE_TIMEOUT	(5 * 1000)

// Communication is via WM_COPYDATA messages. The message type is sent in
// the dwData parameter. Types are defined here.
# define COPYDATA_KEYS		0
# define COPYDATA_REPLY		1
# define COPYDATA_EXPR		10
# define COPYDATA_RESULT	11
# define COPYDATA_ERROR_RESULT	12
# define COPYDATA_ENCODING	20

// This is a structure containing a server HWND and its name.
struct server_id
{
    HWND hwnd;
    char_u *name;
};

// Last received 'encoding' that the client uses.
static char_u	*client_enc = NULL;

/*
 * Tell the other side what encoding we are using.
 * Return -1 if timeout happens.  Other errors are ignored.
 */
    static int
serverSendEnc(HWND target)
{
    COPYDATASTRUCT data;

    data.dwData = COPYDATA_ENCODING;
    data.cbData = (DWORD)STRLEN(p_enc) + 1;
    data.lpData = p_enc;
    if (SendMessageTimeout(target, WM_COPYDATA,
	    (WPARAM)message_window, (LPARAM)&data,
	    SMTO_ABORTIFHUNG, SENDMESSAGE_TIMEOUT, NULL) == 0)
	return -1;
    return 0;
}

/*
 * Clean up on exit. This destroys the hidden message window.
 */
    static void
CleanUpMessaging(void)
{
    if (message_window == 0)
	return;

    DestroyWindow(message_window);
    message_window = 0;
}

static int save_reply(HWND server, char_u *reply, int expr);

/*
 * The window procedure for the hidden message window.
 * It handles callback messages and notifications from servers.
 * In order to process these messages, it is necessary to run a
 * message loop. Code which may run before the main message loop
 * is started (in the GUI) is careful to pump messages when it needs
 * to. Features which require message delivery during normal use will
 * not work in the console version - this basically means those
 * features which allow Vim to act as a server, rather than a client.
 */
    static LRESULT CALLBACK
Messaging_WndProc(HWND hwnd, UINT msg, WPARAM wParam, LPARAM lParam)
{
    if (msg == WM_COPYDATA)
    {
	// This is a message from another Vim. The dwData member of the
	// COPYDATASTRUCT determines the type of message:
	//   COPYDATA_ENCODING:
	//	The encoding that the client uses. Following messages will
	//	use this encoding, convert if needed.
	//   COPYDATA_KEYS:
	//	A key sequence. We are a server, and a client wants these keys
	//	adding to the input queue.
	//   COPYDATA_REPLY:
	//	A reply. We are a client, and a server has sent this message
	//	in response to a request.  (server2client())
	//   COPYDATA_EXPR:
	//	An expression. We are a server, and a client wants us to
	//	evaluate this expression.
	//   COPYDATA_RESULT:
	//	A reply. We are a client, and a server has sent this message
	//	in response to a COPYDATA_EXPR.
	//   COPYDATA_ERROR_RESULT:
	//	A reply. We are a client, and a server has sent this message
	//	in response to a COPYDATA_EXPR that failed to evaluate.
	COPYDATASTRUCT	*data = (COPYDATASTRUCT*)lParam;
	HWND		sender = (HWND)wParam;
	COPYDATASTRUCT	reply;
	char_u		*res;
	int		retval;
	DWORD_PTR	dwret = 0;
	char_u		*str;
	char_u		*tofree;

	switch (data->dwData)
	{
	case COPYDATA_ENCODING:
	    // Remember the encoding that the client uses.
	    vim_free(client_enc);
	    client_enc = enc_canonize((char_u *)data->lpData);
	    return 1;

	case COPYDATA_KEYS:
	    // Remember who sent this, for <client>
	    clientWindow = sender;

	    // Add the received keys to the input buffer.  The loop waiting
	    // for the user to do something should check the input buffer.
	    str = serverConvert(client_enc, (char_u *)data->lpData, &tofree);
	    server_to_input_buf(str);
	    vim_free(tofree);

# ifdef FEAT_GUI
	    // Wake up the main GUI loop.
#  ifdef VIMDLL
	    if (gui.in_use)
#  endif
		if (s_hwnd != 0)
		    PostMessage(s_hwnd, WM_NULL, 0, 0);
# endif
	    return 1;

	case COPYDATA_EXPR:
	    // Remember who sent this, for <client>
	    clientWindow = sender;

	    str = serverConvert(client_enc, (char_u *)data->lpData, &tofree);
	    res = eval_client_expr_to_string(str);

	    if (res == NULL)
	    {
		char	*err = _(e_invalid_expression_received);
		size_t	len = STRLEN(str) + STRLEN(err) + 5;

		res = alloc(len);
		if (res != NULL)
		    vim_snprintf((char *)res, len, "%s: \"%s\"", err, str);
		reply.dwData = COPYDATA_ERROR_RESULT;
	    }
	    else
		reply.dwData = COPYDATA_RESULT;
	    reply.lpData = res;
	    reply.cbData = (DWORD)STRLEN(res) + 1;

	    if (serverSendEnc(sender) < 0)
		retval = -1;
	    else
	    {
		if (SendMessageTimeout(sender, WM_COPYDATA,
			(WPARAM)message_window, (LPARAM)&reply,
			SMTO_ABORTIFHUNG, SENDMESSAGE_TIMEOUT, &dwret) == 0)
		    retval = -1;
		else
		    retval = (int)dwret;
	    }
	    vim_free(tofree);
	    vim_free(res);
	    return retval;

	case COPYDATA_REPLY:
	case COPYDATA_RESULT:
	case COPYDATA_ERROR_RESULT:
	    if (data->lpData != NULL)
	    {
		str = serverConvert(client_enc, (char_u *)data->lpData,
								     &tofree);
		if (tofree == NULL)
		    str = vim_strsave(str);
		if (save_reply(sender, str,
			   (data->dwData == COPYDATA_REPLY ?  0 :
			   (data->dwData == COPYDATA_RESULT ? 1 :
							      2))) == FAIL)
		    vim_free(str);
		else if (data->dwData == COPYDATA_REPLY)
		{
		    char_u	winstr[30];

		    sprintf((char *)winstr, PRINTF_HEX_LONG_U, (long_u)sender);
		    apply_autocmds(EVENT_REMOTEREPLY, winstr, str,
								TRUE, curbuf);
		}
	    }
	    return 1;
	}

	return 0;
    }

    else if (msg == WM_ACTIVATE && wParam == WA_ACTIVE)
    {
	// When the message window is activated (brought to the foreground),
	// this actually applies to the text window.
# if !defined(FEAT_GUI) || defined(VIMDLL)
#  ifdef VIMDLL
	if (!gui.in_use)
#  endif
	    GetConsoleHwnd();	    // get value of s_hwnd
# endif
	if (s_hwnd != 0)
	{
	    SetForegroundWindow(s_hwnd);
	    return 0;
	}
    }

    return DefWindowProc(hwnd, msg, wParam, lParam);
}

/*
 * Initialise the message handling process.  This involves creating a window
 * to handle messages - the window will not be visible.
 */
    void
serverInitMessaging(void)
{
    WNDCLASS wndclass;

    // Clean up on exit
    atexit(CleanUpMessaging);

    // Register a window class - we only really care
    // about the window procedure
    wndclass.style = 0;
    wndclass.lpfnWndProc = Messaging_WndProc;
    wndclass.cbClsExtra = 0;
    wndclass.cbWndExtra = 0;
    wndclass.hInstance = g_hinst;
    wndclass.hIcon = NULL;
    wndclass.hCursor = NULL;
    wndclass.hbrBackground = NULL;
    wndclass.lpszMenuName = NULL;
    wndclass.lpszClassName = VIM_CLASSNAME;
    RegisterClass(&wndclass);

    // Create the message window. It will be hidden, so the details don't
    // matter.  Don't use WS_OVERLAPPEDWINDOW, it will make a shortcut remove
    // focus from gvim.
    message_window = CreateWindow(VIM_CLASSNAME, "",
			 WS_POPUPWINDOW | WS_CAPTION,
			 CW_USEDEFAULT, CW_USEDEFAULT,
			 100, 100, NULL, NULL,
			 g_hinst, NULL);
}

// Used by serverSendToVim() to find an alternate server name.
static char_u *altname_buf_ptr = NULL;

/*
 * Get the title of the window "hwnd", which is the Vim server name, in
 * "name[namelen]" and return the length.
 * Returns zero if window "hwnd" is not a Vim server.
 */
    static int
getVimServerName(HWND hwnd, char *name, int namelen)
{
    int		len;
    char	buffer[VIM_CLASSNAME_LEN + 1];

    // Ignore windows which aren't Vim message windows
    len = GetClassName(hwnd, buffer, sizeof(buffer));
    if (len != VIM_CLASSNAME_LEN || STRCMP(buffer, VIM_CLASSNAME) != 0)
	return 0;

    // Get the title of the window
    return GetWindowText(hwnd, name, namelen);
}

    static BOOL CALLBACK
enumWindowsGetServer(HWND hwnd, LPARAM lparam)
{
    struct	server_id *id = (struct server_id *)lparam;
    char	server[MAX_PATH];

    // Get the title of the window
    if (getVimServerName(hwnd, server, sizeof(server)) == 0)
	return TRUE;

    // If this is the server we're looking for, return its HWND
    if (STRICMP(server, id->name) == 0)
    {
	id->hwnd = hwnd;
	return FALSE;
    }

    // If we are looking for an alternate server, remember this name.
    if (altname_buf_ptr != NULL
	    && STRNICMP(server, id->name, STRLEN(id->name)) == 0
	    && vim_isdigit(server[STRLEN(id->name)]))
    {
	STRCPY(altname_buf_ptr, server);
	altname_buf_ptr = NULL;	    // don't use another name
    }

    // Otherwise, keep looking
    return TRUE;
}

    static BOOL CALLBACK
enumWindowsGetNames(HWND hwnd, LPARAM lparam)
{
    garray_T	*ga = (garray_T *)lparam;
    char	server[MAX_PATH];

    // Get the title of the window
    if (getVimServerName(hwnd, server, sizeof(server)) == 0)
	return TRUE;

    // Add the name to the list
    ga_concat(ga, (char_u *)server);
    ga_concat(ga, (char_u *)"\n");
    return TRUE;
}

struct enum_windows_s
{
    WNDENUMPROC lpEnumFunc;
    LPARAM      lParam;
};

    static BOOL CALLBACK
enum_windows_child(HWND hwnd, LPARAM lParam)
{
    struct enum_windows_s *ew = (struct enum_windows_s *)lParam;

    return (ew->lpEnumFunc)(hwnd, ew->lParam);
}

    static BOOL CALLBACK
enum_windows_toplevel(HWND hwnd, LPARAM lParam)
{
    struct enum_windows_s *ew = (struct enum_windows_s *)lParam;

    if ((ew->lpEnumFunc)(hwnd, ew->lParam))
	return TRUE;
    return EnumChildWindows(hwnd, enum_windows_child, lParam);
}

/*
 * Enumerate all windows including children.
 */
    static BOOL
enum_windows(WNDENUMPROC lpEnumFunc, LPARAM lParam)
{
    struct enum_windows_s ew;

    ew.lpEnumFunc = lpEnumFunc;
    ew.lParam = lParam;
    return EnumWindows(enum_windows_toplevel, (LPARAM)&ew);
}

    static HWND
findServer(char_u *name)
{
    struct server_id id;

    id.name = name;
    id.hwnd = 0;

    enum_windows(enumWindowsGetServer, (LPARAM)(&id));

    return id.hwnd;
}

    void
serverSetName(char_u *name)
{
    char_u	*ok_name;
    HWND	hwnd = 0;
    int		i = 0;
    char_u	*p;

    // Leave enough space for a 9-digit suffix to ensure uniqueness!
    ok_name = alloc(STRLEN(name) + 10);

    STRCPY(ok_name, name);
    p = ok_name + STRLEN(name);

    for (;;)
    {
	// This is inefficient - we're doing an EnumWindows loop for each
	// possible name. It would be better to grab all names in one go,
	// and scan the list each time...
	hwnd = findServer(ok_name);
	if (hwnd == 0)
	    break;

	++i;
	if (i >= 1000)
	    break;

	sprintf((char *)p, "%d", i);
    }

    if (hwnd != 0)
	vim_free(ok_name);
    else
    {
	// Remember the name
	serverName = ok_name;
	need_maketitle = TRUE;	// update Vim window title later

	// Update the message window title
	SetWindowText(message_window, (LPCSTR)ok_name);

# ifdef FEAT_EVAL
	// Set the servername variable
	set_vim_var_string(VV_SEND_SERVER, serverName, -1);
# endif
    }
}

    char_u *
serverGetVimNames(void)
{
    garray_T ga;

    ga_init2(&ga, 1, 100);

    enum_windows(enumWindowsGetNames, (LPARAM)(&ga));
    ga_append(&ga, NUL);

    return ga.ga_data;
}

    int
serverSendReply(
    char_u	*name,		// Where to send.
    char_u	*reply)		// What to send.
{
    HWND	target;
    COPYDATASTRUCT data;
    long_u	n = 0;
    DWORD_PTR	dwret = 0;

    // The "name" argument is a magic cookie obtained from expand("<client>").
    // It should be of the form 0xXXXXX - i.e. a C hex literal, which is the
    // value of the client's message window HWND.
    sscanf((char *)name, SCANF_HEX_LONG_U, &n);
    if (n == 0)
	return -1;

    target = (HWND)n;
    if (!IsWindow(target))
	return -1;

    data.dwData = COPYDATA_REPLY;
    data.cbData = (DWORD)STRLEN(reply) + 1;
    data.lpData = reply;

    if (serverSendEnc(target) < 0)
	return -1;
    if (SendMessageTimeout(target, WM_COPYDATA,
		(WPARAM)message_window, (LPARAM)&data,
		SMTO_ABORTIFHUNG, SENDMESSAGE_TIMEOUT, &dwret) == 0)
	return -1;
    return dwret ? 0 : -1;
}

    int
serverSendToVim(
    char_u	 *name,			// Where to send.
    char_u	 *cmd,			// What to send.
    char_u	 **result,		// Result of eval'ed expression
    void	 *ptarget,		// HWND of server
    int		 asExpr,		// Expression or keys?
    int		 timeout,		// timeout in seconds or zero
    int		 silent)		// don't complain about no server
{
    HWND	target;
    COPYDATASTRUCT data;
    char_u	*retval = NULL;
    int		retcode = 0;
    DWORD_PTR	dwret = 0;
    char_u	altname_buf[MAX_PATH];

    // Execute locally if no display or target is ourselves
    if (serverName != NULL && STRICMP(name, serverName) == 0)
	return sendToLocalVim(cmd, asExpr, result);

    // If the server name does not end in a digit then we look for an
    // alternate name.  e.g. when "name" is GVIM then we may find GVIM2.
    if (STRLEN(name) > 1 && !vim_isdigit(name[STRLEN(name) - 1]))
	altname_buf_ptr = altname_buf;
    altname_buf[0] = NUL;
    target = findServer(name);
    altname_buf_ptr = NULL;
    if (target == 0 && altname_buf[0] != NUL)
	// Use another server name we found.
	target = findServer(altname_buf);

    if (target == 0)
    {
	if (!silent)
	    semsg(_(e_no_registered_server_named_str), name);
	return -1;
    }

    if (ptarget)
	*(HWND *)ptarget = target;

    data.dwData = asExpr ? COPYDATA_EXPR : COPYDATA_KEYS;
    data.cbData = (DWORD)STRLEN(cmd) + 1;
    data.lpData = cmd;

    if (serverSendEnc(target) < 0)
	return -1;
    if (SendMessageTimeout(target, WM_COPYDATA,
		(WPARAM)message_window, (LPARAM)&data,
		SMTO_ABORTIFHUNG, SENDMESSAGE_TIMEOUT, &dwret) == 0)
	return -1;
    if (dwret == 0)
	return -1;

    if (asExpr)
	retval = serverGetReply(target, &retcode, TRUE, TRUE, timeout);

    if (result == NULL)
	vim_free(retval);
    else
	*result = retval; // Caller assumes responsibility for freeing

    return retcode;
}

/*
 * Bring the server to the foreground.
 */
    void
serverForeground(char_u *name)
{
    HWND	target = findServer(name);

    if (target != 0)
	SetForegroundWindow(target);
}

// Replies from server need to be stored until the client picks them up via
// remote_read(). So we maintain a list of server-id/reply pairs.
// Note that there could be multiple replies from one server pending if the
// client is slow picking them up.
// We just store the replies in a simple list. When we remove an entry, we
// move list entries down to fill the gap.
// The server ID is simply the HWND.
typedef struct
{
    HWND	server;		// server window
    char_u	*reply;		// reply string
    int		expr_result;	// 0 for REPLY, 1 for RESULT 2 for error
} reply_T;

static garray_T reply_list = {0, 0, sizeof(reply_T), 5, 0};

# define REPLY_ITEM(i) ((reply_T *)(reply_list.ga_data) + (i))
# define REPLY_COUNT (reply_list.ga_len)

// Flag which is used to wait for a reply
static int reply_received = 0;

/*
 * Store a reply.  "reply" must be allocated memory (or NULL).
 */
    static int
save_reply(HWND server, char_u *reply, int expr)
{
    reply_T *rep;

    if (ga_grow(&reply_list, 1) == FAIL)
	return FAIL;

    rep = REPLY_ITEM(REPLY_COUNT);
    rep->server = server;
    rep->reply = reply;
    rep->expr_result = expr;
    if (rep->reply == NULL)
	return FAIL;

    ++REPLY_COUNT;
    reply_received = 1;
    return OK;
}

/*
 * Get a reply from server "server".
 * When "expr_res" is non NULL, get the result of an expression, otherwise a
 * server2client() message.
 * When non NULL, point to return code. 0 => OK, -1 => ERROR
 * If "remove" is TRUE, consume the message, the caller must free it then.
 * if "wait" is TRUE block until a message arrives (or the server exits).
 */
    char_u *
serverGetReply(HWND server, int *expr_res, int remove, int wait, int timeout)
{
    int		i;
    char_u	*reply;
    reply_T	*rep;
    int		did_process = FALSE;
    time_t	start;
    time_t	now;

    // When waiting, loop until the message waiting for is received.
    time(&start);
    for (;;)
    {
	// Reset this here, in case a message arrives while we are going
	// through the already received messages.
	reply_received = 0;

	for (i = 0; i < REPLY_COUNT; ++i)
	{
	    rep = REPLY_ITEM(i);
	    if (rep->server == server
		    && ((rep->expr_result != 0) == (expr_res != NULL)))
	    {
		// Save the values we've found for later
		reply = rep->reply;
		if (expr_res != NULL)
		    *expr_res = rep->expr_result == 1 ? 0 : -1;

		if (remove)
		{
		    // Move the rest of the list down to fill the gap
		    mch_memmove(rep, rep + 1,
				     (REPLY_COUNT - i - 1) * sizeof(reply_T));
		    --REPLY_COUNT;
		}

		// Return the reply to the caller, who takes on responsibility
		// for freeing it if "remove" is TRUE.
		return reply;
	    }
	}

	// If we got here, we didn't find a reply. Return immediately if the
	// "wait" parameter isn't set.
	if (!wait)
	{
	    // Process pending messages once. Without this, looping on
	    // remote_peek() would never get the reply.
	    if (!did_process)
	    {
		did_process = TRUE;
		serverProcessPendingMessages();
		continue;
	    }
	    break;
	}

	// We need to wait for a reply. Enter a message loop until the
	// "reply_received" flag gets set.

	// Loop until we receive a reply
	while (reply_received == 0)
	{
# ifdef FEAT_TIMERS
	    // TODO: use the return value to decide how long to wait.
	    check_due_timer();
# endif
	    time(&now);
	    if (timeout > 0 && (now - start) >= timeout)
		break;

	    // Wait for a SendMessage() call to us.  This could be the reply
	    // we are waiting for.  Use a timeout of a second, to catch the
	    // situation that the server died unexpectedly.
	    MsgWaitForMultipleObjects(0, NULL, TRUE, 1000, QS_ALLINPUT);

	    // If the server has died, give up
	    if (!IsWindow(server))
		return NULL;

	    serverProcessPendingMessages();
	}
    }

    return NULL;
}

/*
 * Process any messages in the Windows message queue.
 */
    void
serverProcessPendingMessages(void)
{
    MSG msg;

    while (PeekMessageW(&msg, NULL, 0, 0, PM_REMOVE))
    {
	TranslateMessage(&msg);
	DispatchMessageW(&msg);
    }
}

#endif // FEAT_CLIENTSERVER

#if defined(FEAT_GUI) || (defined(FEAT_PRINTER) && !defined(FEAT_POSTSCRIPT)) \
	|| defined(PROTO)

struct charset_pair
{
    char	*name;
    BYTE	charset;
};

static struct charset_pair
charset_pairs[] =
{
    {"ANSI",		ANSI_CHARSET},
    {"CHINESEBIG5",	CHINESEBIG5_CHARSET},
    {"DEFAULT",		DEFAULT_CHARSET},
    {"HANGEUL",		HANGEUL_CHARSET},
    {"OEM",		OEM_CHARSET},
    {"SHIFTJIS",	SHIFTJIS_CHARSET},
    {"SYMBOL",		SYMBOL_CHARSET},
    {"ARABIC",		ARABIC_CHARSET},
    {"BALTIC",		BALTIC_CHARSET},
    {"EASTEUROPE",	EASTEUROPE_CHARSET},
    {"GB2312",		GB2312_CHARSET},
    {"GREEK",		GREEK_CHARSET},
    {"HEBREW",		HEBREW_CHARSET},
    {"JOHAB",		JOHAB_CHARSET},
    {"MAC",		MAC_CHARSET},
    {"RUSSIAN",		RUSSIAN_CHARSET},
    {"THAI",		THAI_CHARSET},
    {"TURKISH",		TURKISH_CHARSET},
# ifdef VIETNAMESE_CHARSET
    {"VIETNAMESE",	VIETNAMESE_CHARSET},
# endif
    {NULL,		0}
};

struct quality_pair
{
    char	*name;
    DWORD	quality;
};

static struct quality_pair
quality_pairs[] = {
# ifdef CLEARTYPE_QUALITY
    {"CLEARTYPE",	CLEARTYPE_QUALITY},
# endif
# ifdef ANTIALIASED_QUALITY
    {"ANTIALIASED",	ANTIALIASED_QUALITY},
# endif
# ifdef NONANTIALIASED_QUALITY
    {"NONANTIALIASED",	NONANTIALIASED_QUALITY},
# endif
# ifdef PROOF_QUALITY
    {"PROOF",		PROOF_QUALITY},
# endif
# ifdef DRAFT_QUALITY
    {"DRAFT",		DRAFT_QUALITY},
# endif
    {"DEFAULT",		DEFAULT_QUALITY},
    {NULL,		0}
};

/*
 * Convert a charset ID to a name.
 * Return NULL when not recognized.
 */
    char *
charset_id2name(int id)
{
    struct charset_pair *cp;

    for (cp = charset_pairs; cp->name != NULL; ++cp)
	if ((BYTE)id == cp->charset)
	    break;
    return cp->name;
}

/*
 * Convert a quality ID to a name.
 * Return NULL when not recognized.
 */
    char *
quality_id2name(DWORD id)
{
    struct quality_pair *qp;

    for (qp = quality_pairs; qp->name != NULL; ++qp)
	if (id == qp->quality)
	    break;
    return qp->name;
}

// The default font height in 100% scaling (96dpi).
// (-12 in 96dpi equates to roughly 9pt)
#define DEFAULT_FONT_HEIGHT	(-12)

static const LOGFONTW s_lfDefault =
{
    DEFAULT_FONT_HEIGHT,
    0, 0, 0, FW_NORMAL, FALSE, FALSE, FALSE, DEFAULT_CHARSET,
    OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS,
    PROOF_QUALITY, FIXED_PITCH | FF_DONTCARE,
    L""	// Default font name will be set later based on current language.
};

// This will be initialized when set_default_logfont() is called first time.
// The value will be based on the system DPI.
int current_font_height = 0;		// also used in gui_w32.c

/*
 * Convert a string representing a point size into pixels. The string should
 * be a positive decimal number, with an optional decimal point (eg, "12", or
 * "10.5"). The pixel value is returned, and a pointer to the next unconverted
 * character is stored in *end. The flag "vertical" says whether this
 * calculation is for a vertical (height) size or a horizontal (width) one.
 */
    static int
points_to_pixels(WCHAR *str, WCHAR **end, int vertical, long_i pprinter_dc)
{
    int		pixels;
    int		points = 0;
    int		divisor = 0;
    HWND	hwnd = (HWND)0;
    HDC		hdc;
    HDC		printer_dc = (HDC)pprinter_dc;

    while (*str != NUL)
    {
	if (*str == L'.' && divisor == 0)
	{
	    // Start keeping a divisor, for later
	    divisor = 1;
	}
	else
	{
	    if (!VIM_ISDIGIT(*str))
		break;

	    points *= 10;
	    points += *str - L'0';
	    divisor *= 10;
	}
	++str;
    }

    if (divisor == 0)
	divisor = 1;

    if (printer_dc == NULL)
    {
	hwnd = GetDesktopWindow();
	hdc = GetWindowDC(hwnd);
    }
    else
	hdc = printer_dc;

    pixels = MulDiv(points,
		    GetDeviceCaps(hdc, vertical ? LOGPIXELSY : LOGPIXELSX),
		    72 * divisor);

    if (printer_dc == NULL)
	ReleaseDC(hwnd, hdc);

    *end = str;
    return pixels;
}

/*
 * Convert pixel into point size. This is a reverse of points_to_pixels.
 */
    static double
pixels_to_points(int pixels, int vertical, long_i pprinter_dc)
{
    double	points = 0;
    HWND	hwnd = (HWND)0;
    HDC		hdc;
    HDC		printer_dc = (HDC)pprinter_dc;

    if (printer_dc == NULL)
    {
	hwnd = GetDesktopWindow();
	hdc = GetWindowDC(hwnd);
    }
    else
	hdc = printer_dc;

    points = pixels * 72.0 / GetDeviceCaps(hdc, vertical ? LOGPIXELSY : LOGPIXELSX);
    if (printer_dc == NULL)
	ReleaseDC(hwnd, hdc);

    return points;
}

    static int CALLBACK
font_enumproc(
    ENUMLOGFONTW    *elf,
    NEWTEXTMETRICW  *ntm UNUSED,
    DWORD	    type UNUSED,
    LPARAM	    lparam)
{
    // Return value:
    //	  0 = terminate now (monospace & ANSI)
    //	  1 = continue, still no luck...
    //	  2 = continue, but we have an acceptable LOGFONTW
    //	      (monospace, not ANSI)
    // We use these values, as EnumFontFamilies returns 1 if the
    // callback function is never called. So, we check the return as
    // 0 = perfect, 2 = OK, 1 = no good...
    // It's not pretty, but it works!

    LOGFONTW *lf = (LOGFONTW *)(lparam);

# ifndef FEAT_PROPORTIONAL_FONTS
    // Ignore non-monospace fonts without further ado
    if ((ntm->tmPitchAndFamily & 1) != 0)
	return 1;
# endif

    // Remember this LOGFONTW as a "possible"
    *lf = elf->elfLogFont;

    // Terminate the scan as soon as we find an ANSI font
    if (lf->lfCharSet == ANSI_CHARSET
	    || lf->lfCharSet == OEM_CHARSET
	    || lf->lfCharSet == DEFAULT_CHARSET)
	return 0;

    // Continue the scan - we have a non-ANSI font
    return 2;
}

    static int
init_logfont(LOGFONTW *lf)
{
    int		n;
    HWND	hwnd = GetDesktopWindow();
    HDC		hdc = GetWindowDC(hwnd);

    n = EnumFontFamiliesW(hdc,
			 lf->lfFaceName,
			 (FONTENUMPROCW)font_enumproc,
			 (LPARAM)lf);

    ReleaseDC(hwnd, hdc);

    // If we couldn't find a usable font, return failure
    if (n == 1)
	return FAIL;

    // Tidy up the rest of the LOGFONTW structure. We set to a basic
    // font - get_logfont() sets bold, italic, etc based on the user's
    // input.
    lf->lfHeight = current_font_height;
    lf->lfWidth = 0;
    lf->lfItalic = FALSE;
    lf->lfUnderline = FALSE;
    lf->lfStrikeOut = FALSE;
    lf->lfWeight = FW_NORMAL;

    // Return success
    return OK;
}

/*
 * Call back for EnumFontFamiliesW in expand_font_enumproc.
 *
 */
    static int CALLBACK
expand_font_enumproc(
    ENUMLOGFONTW    *elf,
    NEWTEXTMETRICW  *ntm UNUSED,
    DWORD	    type UNUSED,
    LPARAM	    lparam)
{
    LOGFONTW *lf = (LOGFONTW*)elf;

# ifndef FEAT_PROPORTIONAL_FONTS
    // Ignore non-monospace fonts without further ado
    if ((ntm->tmPitchAndFamily & 1) != 0)
	return 1;
# endif

    // Filter only on ANSI. Otherwise will see a lot of random fonts that we
    // usually don't want.
    if (lf->lfCharSet != ANSI_CHARSET)
        return 1;

    int (*add_match)(char_u *) = (int (*)(char_u *))lparam;

    WCHAR *faceNameW = lf->lfFaceName;
    char_u *faceName = utf16_to_enc(faceNameW, NULL);
    if (!faceName)
	return 0;

    add_match(faceName);
    vim_free(faceName);

    return 1;
}

/*
 * Cmdline expansion for setting 'guifont'. Will enumerate through all
 * monospace fonts for completion. If used after ':', will expand to possible
 * font configuration options like font sizes.
 *
 * This function has "gui" in its name because in some platforms (GTK) font
 * handling is done by the GUI code, whereas in Windows it's part of the
 * platform code.
 */
    void
gui_mch_expand_font(optexpand_T *args, void *param UNUSED, int (*add_match)(char_u *val))
{
    expand_T	    *xp = args->oe_xp;
    if (xp->xp_pattern > args->oe_set_arg && *(xp->xp_pattern-1) == ':')
    {
	char buf[30];

	// Always fill in with the current font size as first option for
	// convenience. We simply round to the closest integer for simplicity.
        int font_height = (int)round(
		pixels_to_points(-current_font_height, TRUE, (long_i)NULL));
	vim_snprintf(buf, ARRAY_LENGTH(buf), "h%d", font_height);
	add_match((char_u *)buf);

	// Note: Keep this in sync with get_logfont(). Don't include 'c' and
	// 'q' as we fill in all the values below.
	static char *(p_gfn_win_opt_values[]) = {
	    "h" , "w" , "W" , "b" , "i" , "u" , "s"};
	for (size_t i = 0; i < ARRAY_LENGTH(p_gfn_win_opt_values); i++)
	    add_match((char_u *)p_gfn_win_opt_values[i]);

	struct charset_pair *cp;
	for (cp = charset_pairs; cp->name != NULL; ++cp)
	{
	    vim_snprintf(buf, ARRAY_LENGTH(buf), "c%s", cp->name);
	    add_match((char_u *)buf);
	}
	struct quality_pair *qp;
	for (qp = quality_pairs; qp->name != NULL; ++qp)
	{
	    vim_snprintf(buf, ARRAY_LENGTH(buf), "q%s", qp->name);
	    add_match((char_u *)buf);
	}
	return;
    }

    HWND	hwnd = GetDesktopWindow();
    HDC		hdc = GetWindowDC(hwnd);

    EnumFontFamiliesW(hdc,
	    NULL,
	    (FONTENUMPROCW)expand_font_enumproc,
	    (LPARAM)add_match);

    ReleaseDC(hwnd, hdc);
}

/*
 * Compare a UTF-16 string and an ASCII string literally.
 * Only works all the code points are inside ASCII range.
 */
    static int
utf16ascncmp(const WCHAR *w, const char *p, size_t n)
{
    size_t i;

    for (i = 0; i < n; i++)
    {
	if (w[i] == 0 || w[i] != p[i])
	    return w[i] - p[i];
    }
    return 0;
}

/*
 * Equivalent of GetDpiForSystem().
 */
    UINT WINAPI
vimGetDpiForSystem(void)
{
    HWND hwnd = GetDesktopWindow();
    HDC hdc = GetWindowDC(hwnd);
    UINT dpi = GetDeviceCaps(hdc, LOGPIXELSY);
    ReleaseDC(hwnd, hdc);
    return dpi;
}

/*
 * Set default logfont based on current language.
 */
    static void
set_default_logfont(LOGFONTW *lf)
{
    // Default font name for current language on MS-Windows.
    // If not translated, falls back to "Consolas".
    // This must be a fixed-pitch font.
    const char *defaultfontname = N_("DefaultFontNameForWindows");
    char *fontname = _(defaultfontname);

    if (strcmp(fontname, defaultfontname) == 0)
	fontname = "Consolas";

    *lf = s_lfDefault;
    lf->lfHeight = DEFAULT_FONT_HEIGHT * (int)vimGetDpiForSystem() / 96;
    if (current_font_height == 0)
	current_font_height = lf->lfHeight;

    WCHAR *wfontname = enc_to_utf16((char_u*)fontname, NULL);
    if (wfontname != NULL)
    {
	wcscpy_s(lf->lfFaceName, LF_FACESIZE, wfontname);
	vim_free(wfontname);
    }
}

/*
 * Get font info from "name" into logfont "lf".
 * Return OK for a valid name, FAIL otherwise.
 */
    int
get_logfont(
    LOGFONTW	*lf,
    char_u	*name,
    HDC		printer_dc,
    int		verbose)
{
    WCHAR	*p;
    int		i;
    int		ret = FAIL;
    static LOGFONTW *lastlf = NULL;
    WCHAR	*wname;

    set_default_logfont(lf);
    if (name == NULL)
	return OK;

    wname = enc_to_utf16(name, NULL);
    if (wname == NULL)
	return FAIL;

    if (wcscmp(wname, L"*") == 0)
    {
# if defined(FEAT_GUI_MSWIN)
	CHOOSEFONTW	cf;
	// if name is "*", bring up std font dialog:
	CLEAR_FIELD(cf);
	cf.lStructSize = sizeof(cf);
	cf.hwndOwner = s_hwnd;
	cf.Flags = CF_SCREENFONTS | CF_FIXEDPITCHONLY | CF_INITTOLOGFONTSTRUCT;
	if (lastlf != NULL)
	    *lf = *lastlf;
	cf.lpLogFont = lf;
	cf.nFontType = 0 ; //REGULAR_FONTTYPE;
	if (ChooseFontW(&cf))
	    ret = OK;
# endif
	goto theend;
    }

    /*
     * Split name up, it could be <name>:h<height>:w<width> etc.
     */
    for (p = wname; *p && *p != L':'; p++)
    {
	if (p - wname + 1 >= LF_FACESIZE)
	    goto theend;			// Name too long
	lf->lfFaceName[p - wname] = *p;
    }
    if (p != wname)
	lf->lfFaceName[p - wname] = NUL;

    // First set defaults
    lf->lfHeight = DEFAULT_FONT_HEIGHT * (int)vimGetDpiForSystem() / 96;
    lf->lfWidth = 0;
    lf->lfWeight = FW_NORMAL;
    lf->lfItalic = FALSE;
    lf->lfUnderline = FALSE;
    lf->lfStrikeOut = FALSE;

    /*
     * If the font can't be found, try replacing '_' by ' '.
     */
    if (init_logfont(lf) == FAIL)
    {
	int	did_replace = FALSE;

	for (i = 0; lf->lfFaceName[i]; ++i)
	    if (lf->lfFaceName[i] == L'_')
	    {
		lf->lfFaceName[i] = L' ';
		did_replace = TRUE;
	    }
	if (!did_replace || init_logfont(lf) == FAIL)
	    goto theend;
    }

    while (*p == L':')
	p++;

    // Set the values found after ':'
    while (*p)
    {
	switch (*p++)
	{
	    // Note: Keep this in sync with gui_mch_expand_font().
	    case L'h':
		lf->lfHeight = - points_to_pixels(p, &p, TRUE, (long_i)printer_dc);
		break;
	    case L'w':
		lf->lfWidth = points_to_pixels(p, &p, FALSE, (long_i)printer_dc);
		break;
	    case L'W':
		lf->lfWeight = wcstol(p, &p, 10);
		break;
	    case L'b':
		lf->lfWeight = FW_BOLD;
		break;
	    case L'i':
		lf->lfItalic = TRUE;
		break;
	    case L'u':
		lf->lfUnderline = TRUE;
		break;
	    case L's':
		lf->lfStrikeOut = TRUE;
		break;
	    case L'c':
		{
		    struct charset_pair *cp;

		    for (cp = charset_pairs; cp->name != NULL; ++cp)
			if (utf16ascncmp(p, cp->name, strlen(cp->name)) == 0)
			{
			    lf->lfCharSet = cp->charset;
			    p += strlen(cp->name);
			    break;
			}
		    if (cp->name == NULL && verbose)
		    {
			char_u *s = utf16_to_enc(p, NULL);

			semsg(_(e_illegal_str_name_str_in_font_name_str),
							   "charset", s, name);
			vim_free(s);
			break;
		    }
		    break;
		}
	    case L'q':
		{
		    struct quality_pair *qp;

		    for (qp = quality_pairs; qp->name != NULL; ++qp)
			if (utf16ascncmp(p, qp->name, strlen(qp->name)) == 0)
			{
			    lf->lfQuality = qp->quality;
			    p += strlen(qp->name);
			    break;
			}
		    if (qp->name == NULL && verbose)
		    {
			char_u *s = utf16_to_enc(p, NULL);
			semsg(_(e_illegal_str_name_str_in_font_name_str),
							   "quality", s, name);
			vim_free(s);
			break;
		    }
		    break;
		}
	    default:
		if (verbose)
		    semsg(_(e_illegal_char_nr_in_font_name_str), p[-1], name);
		goto theend;
	}
	while (*p == L':')
	    p++;
    }
    ret = OK;

theend:
    // ron: init lastlf
    if (ret == OK && printer_dc == NULL)
    {
	vim_free(lastlf);
	lastlf = ALLOC_ONE(LOGFONTW);
	if (lastlf != NULL)
	    mch_memmove(lastlf, lf, sizeof(LOGFONTW));
    }
    vim_free(wname);

    return ret;
}

#endif // defined(FEAT_GUI) || defined(FEAT_PRINTER)

#if defined(FEAT_JOB_CHANNEL) || defined(PROTO)
/*
 * Initialize the Winsock dll.
 */
    void
channel_init_winsock(void)
{
    WSADATA wsaData;
    int wsaerr;

    if (WSInitialized)
	return;

    wsaerr = WSAStartup(MAKEWORD(2, 2), &wsaData);
    if (wsaerr == 0)
	WSInitialized = TRUE;
}
#endif
