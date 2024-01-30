/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */
/*
 * os_win32.c
 *
 * Used for both the console version and the Win32 GUI.  A lot of code is for
 * the console version only, so there is a lot of "#ifndef FEAT_GUI_MSWIN".
 *
 * Win32 (Windows NT and Windows 95) system-dependent routines.
 * Portions lifted from the Win32 SDK samples, the MSDOS-dependent code,
 * NetHack 3.1.3, GNU Emacs 19.30, and Vile 5.5.
 *
 * George V. Reilly <george@reilly.org> wrote most of this.
 * Roger Knobbe <rogerk@wonderware.com> did the initial port of Vim 3.0.
 */

#include "vim.h"

#ifdef FEAT_MZSCHEME
# include "if_mzsch.h"
#endif

#include <sys/types.h>
#include <signal.h>
#include <limits.h>

// cproto fails on missing include files
#ifndef PROTO
# include <process.h>
# include <winternl.h>
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
#endif

#ifdef FEAT_JOB_CHANNEL
# include <tlhelp32.h>
#endif

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

// Record all output and all keyboard & mouse input
// #define MCH_WRITE_DUMP

#ifdef MCH_WRITE_DUMP
FILE* fdDump = NULL;
#endif

/*
 * When generating prototypes for Win32 on Unix, these lines make the syntax
 * errors disappear.  They do not need to be correct.
 */
#ifdef PROTO
# define WINAPI
typedef char * LPCSTR;
typedef char * LPWSTR;
typedef int ACCESS_MASK;
typedef int BOOL;
typedef int BOOLEAN;
typedef int CALLBACK;
typedef int COLORREF;
typedef int CONSOLE_CURSOR_INFO;
typedef int COORD;
typedef int DWORD;
typedef int HANDLE;
typedef int LPHANDLE;
typedef int HDC;
typedef int HFONT;
typedef int HICON;
typedef int HINSTANCE;
typedef int HWND;
typedef int INPUT_RECORD;
typedef int INT;
typedef int KEY_EVENT_RECORD;
typedef int LOGFONT;
typedef int LPBOOL;
typedef int LPCTSTR;
typedef int LPDWORD;
typedef int LPSTR;
typedef int LPTSTR;
typedef int LPVOID;
typedef int MOUSE_EVENT_RECORD;
typedef int PACL;
typedef int PDWORD;
typedef int PHANDLE;
typedef int PRINTDLG;
typedef int PSECURITY_DESCRIPTOR;
typedef int PSID;
typedef int SECURITY_INFORMATION;
typedef int SHORT;
typedef int SMALL_RECT;
typedef int TEXTMETRIC;
typedef int TOKEN_INFORMATION_CLASS;
typedef int TRUSTEE;
typedef int WORD;
typedef int WCHAR;
typedef void VOID;
typedef int BY_HANDLE_FILE_INFORMATION;
typedef int SE_OBJECT_TYPE;
typedef int PSNSECINFO;
typedef int PSNSECINFOW;
typedef int STARTUPINFO;
typedef int PROCESS_INFORMATION;
typedef int LPSECURITY_ATTRIBUTES;
# define __stdcall // empty
#endif

#if !defined(FEAT_GUI_MSWIN) || defined(VIMDLL)
// Win32 Console handles for input and output
static HANDLE g_hConIn  = INVALID_HANDLE_VALUE;
static HANDLE g_hConOut = INVALID_HANDLE_VALUE;

// Win32 Screen buffer,coordinate,console I/O information
static SMALL_RECT g_srScrollRegion;
// This is explicitly initialised to work around a LTCG issue on Windows ARM64
// (at least of 19.39.33321).  This pushes this into the `.data` rather than
// `.bss` which corrects code generation in `write_chars` (#13453).
static COORD	  g_coord = {0, 0};  // 0-based, but external coords are 1-based

// The attribute of the screen when the editor was started
static WORD  g_attrDefault = 7;  // lightgray text on black background
static WORD  g_attrCurrent;

static int g_fCBrkPressed = FALSE;  // set by ctrl-break interrupt
static int g_fCtrlCPressed = FALSE; // set when ctrl-C or ctrl-break detected
static int g_fForceExit = FALSE;    // set when forcefully exiting

static void scroll(unsigned cLines);
static void set_scroll_region(unsigned left, unsigned top,
			      unsigned right, unsigned bottom);
static void set_scroll_region_tb(unsigned top, unsigned bottom);
static void set_scroll_region_lr(unsigned left, unsigned right);
static void insert_lines(unsigned cLines);
static void delete_lines(unsigned cLines);
static void gotoxy(unsigned x, unsigned y);
static void standout(void);
static int s_cursor_visible = TRUE;
static int did_create_conin = FALSE;
// The 'input_record_buffer' is an internal dynamic fifo queue of MS-Windows
// console INPUT_RECORD events that are normally read from the console input
// buffer.  This provides an injection point for testing the low-level handling
// of INPUT_RECORDs.
typedef struct input_record_buffer_node_S
{
    INPUT_RECORD ir;
    struct input_record_buffer_node_S *next;
} input_record_buffer_node_T;
typedef struct input_record_buffer_S
{
    input_record_buffer_node_T *head;
    input_record_buffer_node_T *tail;
    int length;
} input_record_buffer_T;
static input_record_buffer_T input_record_buffer;
static int read_input_record_buffer(INPUT_RECORD* irEvents, int nMaxLength);
static int write_input_record_buffer(INPUT_RECORD* irEvents, int nLength);
#endif
#ifdef FEAT_GUI_MSWIN
static int s_dont_use_vimrun = TRUE;
static int need_vimrun_warning = FALSE;
static char *vimrun_path = "vimrun ";
#endif

static int win32_getattrs(char_u *name);
static int win32_setattrs(char_u *name, int attrs);
static int win32_set_archive(char_u *name);

static int conpty_working = 0;
static int conpty_type = 0;
static int conpty_stable = 0;
static int conpty_fix_type = 0;
static void vtp_flag_init();

#if !defined(FEAT_GUI_MSWIN) || defined(VIMDLL)
static int vtp_working = 0;
static void vtp_init();
static void vtp_exit();
static void vtp_sgr_bulk(int arg);
static void vtp_sgr_bulks(int argc, int *argv);

static int wt_working = 0;
static void wt_init(void);

static int g_color_index_bg = 0;
static int g_color_index_fg = 7;

# ifdef FEAT_TERMGUICOLORS
static guicolor_T save_console_bg_rgb;
static guicolor_T save_console_fg_rgb;
static guicolor_T store_console_bg_rgb;
static guicolor_T store_console_fg_rgb;
static int default_console_color_bg = 0x000000; // black
static int default_console_color_fg = 0xc0c0c0; // white
#  define USE_VTP		(vtp_working && is_term_win32() \
						 && (p_tgc || t_colors >= 256))
#  define USE_WT		(wt_working)
# else
#  define USE_VTP		0
#  define USE_WT		0
# endif

static void set_console_color_rgb(void);
static void reset_console_color_rgb(void);
static void restore_console_color_rgb(void);
#endif  // !FEAT_GUI_MSWIN || VIMDLL

// This flag is newly created from Windows 10
#ifndef ENABLE_VIRTUAL_TERMINAL_PROCESSING
# define ENABLE_VIRTUAL_TERMINAL_PROCESSING 0x0004
#endif

#if !defined(FEAT_GUI_MSWIN) || defined(VIMDLL)
static int suppress_winsize = 1;	// don't fiddle with console
#endif

static WCHAR *exe_pathw = NULL;

static BOOL win8_or_later = FALSE;
static BOOL win10_22H2_or_later = FALSE;
#if !defined(FEAT_GUI_MSWIN) || defined(VIMDLL)
static BOOL use_alternate_screen_buffer = FALSE;
#endif

/*
 * Get version number including build number
 */
typedef BOOL (WINAPI *PfnRtlGetVersion)(LPOSVERSIONINFOW);
#define MAKE_VER(major, minor, build) \
    (((major) << 24) | ((minor) << 16) | (build))

    static DWORD
get_build_number(void)
{
    OSVERSIONINFOW	osver;
    HMODULE		hNtdll;
    PfnRtlGetVersion	pRtlGetVersion;
    DWORD		ver = MAKE_VER(0, 0, 0);

    osver.dwOSVersionInfoSize = sizeof(OSVERSIONINFOW);
    hNtdll = GetModuleHandle("ntdll.dll");
    if (hNtdll == NULL)
	return ver;

    pRtlGetVersion =
	(PfnRtlGetVersion)GetProcAddress(hNtdll, "RtlGetVersion");
    pRtlGetVersion(&osver);
    ver = MAKE_VER(min(osver.dwMajorVersion, 255),
	    min(osver.dwMinorVersion, 255),
	    min(osver.dwBuildNumber, 32767));
    return ver;
}

#if !defined(FEAT_GUI_MSWIN) || defined(VIMDLL)
    static BOOL
is_ambiwidth_event(
    INPUT_RECORD *ir)
{
    return ir->EventType == KEY_EVENT
		&& ir->Event.KeyEvent.bKeyDown
		&& ir->Event.KeyEvent.wRepeatCount == 1
		&& ir->Event.KeyEvent.wVirtualKeyCode == 0x12
		&& ir->Event.KeyEvent.wVirtualScanCode == 0x38
		&& ir->Event.KeyEvent.uChar.UnicodeChar == 0
		&& ir->Event.KeyEvent.dwControlKeyState == 2;
}

    static void
make_ambiwidth_event(
    INPUT_RECORD *down,
    INPUT_RECORD *up)
{
    down->Event.KeyEvent.wVirtualKeyCode = 0;
    down->Event.KeyEvent.wVirtualScanCode = 0;
    down->Event.KeyEvent.uChar.UnicodeChar
				    = up->Event.KeyEvent.uChar.UnicodeChar;
    down->Event.KeyEvent.dwControlKeyState = 0;
}

/*
 * Version of ReadConsoleInput() that works with IME.
 * Works around problems on Windows 8.
 */
    static BOOL
read_console_input(
    HANDLE	    hInput,
    INPUT_RECORD    *lpBuffer,
    int		    nLength,
    LPDWORD	    lpEvents)
{
    enum
    {
	IRSIZE = 10
    };
    static INPUT_RECORD s_irCache[IRSIZE];
    static DWORD s_dwIndex = 0;
    static DWORD s_dwMax = 0;
    DWORD dwEvents;
    int head;
    int tail;
    int i;
    static INPUT_RECORD s_irPseudo;

    if (s_dwMax == 0 && input_record_buffer.length > 0)
    {
	dwEvents = read_input_record_buffer(s_irCache, IRSIZE);
	s_dwIndex = 0;
	s_dwMax = dwEvents;
    }

    if (nLength == -2)
	return (s_dwMax > 0) ? TRUE : FALSE;

    if (!win8_or_later)
    {
	if (nLength == -1)
	    return PeekConsoleInputW(hInput, lpBuffer, 1, lpEvents);
	return ReadConsoleInputW(hInput, lpBuffer, 1, &dwEvents);
    }

    if (s_dwMax == 0)
    {
	if (!vtp_working && nLength == -1)
	    return PeekConsoleInputW(hInput, lpBuffer, 1, lpEvents);
	GetNumberOfConsoleInputEvents(hInput, &dwEvents);
	if (dwEvents == 0 && nLength == -1)
	    return PeekConsoleInputW(hInput, lpBuffer, 1, lpEvents);
	ReadConsoleInputW(hInput, s_irCache, IRSIZE, &dwEvents);
	s_dwIndex = 0;
	s_dwMax = dwEvents;
	if (dwEvents == 0)
	{
	    *lpEvents = 0;
	    return TRUE;
	}

	for (i = s_dwIndex; i < (int)s_dwMax - 1; ++i)
	    if (is_ambiwidth_event(&s_irCache[i]))
		make_ambiwidth_event(&s_irCache[i], &s_irCache[i + 1]);

	if (s_dwMax > 1)
	{
	    head = 0;
	    tail = s_dwMax - 1;
	    while (head != tail)
	    {
		if (s_irCache[head].EventType == WINDOW_BUFFER_SIZE_EVENT
			&& s_irCache[head + 1].EventType
						  == WINDOW_BUFFER_SIZE_EVENT)
		{
		    // Remove duplicate event to avoid flicker.
		    for (i = head; i < tail; ++i)
			s_irCache[i] = s_irCache[i + 1];
		    --tail;
		    continue;
		}
		head++;
	    }
	    s_dwMax = tail + 1;
	}
    }

    if (s_irCache[s_dwIndex].EventType == KEY_EVENT)
    {
	if (s_irCache[s_dwIndex].Event.KeyEvent.wRepeatCount > 1)
	{
	    s_irPseudo = s_irCache[s_dwIndex];
	    s_irPseudo.Event.KeyEvent.wRepeatCount = 1;
	    s_irCache[s_dwIndex].Event.KeyEvent.wRepeatCount--;
	    *lpBuffer = s_irPseudo;
	    *lpEvents = 1;
	    return TRUE;
	}
    }

    *lpBuffer = s_irCache[s_dwIndex];
    if (!(nLength == -1 || nLength == -2) && ++s_dwIndex >= s_dwMax)
	s_dwMax = 0;
    *lpEvents = 1;
    return TRUE;
}

/*
 * Version of PeekConsoleInput() that works with IME.
 */
    static BOOL
peek_console_input(
    HANDLE	    hInput,
    INPUT_RECORD    *lpBuffer,
    DWORD	    nLength UNUSED,
    LPDWORD	    lpEvents)
{
    return read_console_input(hInput, lpBuffer, -1, lpEvents);
}

# ifdef FEAT_CLIENTSERVER
    static DWORD
msg_wait_for_multiple_objects(
    DWORD    nCount,
    LPHANDLE pHandles,
    BOOL     fWaitAll,
    DWORD    dwMilliseconds,
    DWORD    dwWakeMask)
{
    if (read_console_input(NULL, NULL, -2, NULL))
	return WAIT_OBJECT_0;
    return MsgWaitForMultipleObjects(nCount, pHandles, fWaitAll,
				     dwMilliseconds, dwWakeMask);
}
# endif

# ifndef FEAT_CLIENTSERVER
    static DWORD
wait_for_single_object(
    HANDLE hHandle,
    DWORD dwMilliseconds)
{
    if (read_console_input(NULL, NULL, -2, NULL))
	return WAIT_OBJECT_0;
    return WaitForSingleObject(hHandle, dwMilliseconds);
}
# endif
#endif   // !FEAT_GUI_MSWIN || VIMDLL

    void
mch_get_exe_name(void)
{
    // Maximum length of $PATH is more than MAXPATHL.  8191 is often mentioned
    // as the maximum length that works.  Add 1 for a NUL byte and 5 for
    // "PATH=".
#define MAX_ENV_PATH_LEN (8191 + 1 + 5)
    WCHAR	temp[MAX_ENV_PATH_LEN];
    WCHAR	buf[MAX_PATH];
    int		updated = FALSE;
    static int	enc_prev = -1;

    if (exe_name == NULL || exe_pathw == NULL || enc_prev != enc_codepage)
    {
	// store the name of the executable, may be used for $VIM
	GetModuleFileNameW(NULL, buf, MAX_PATH);
	if (*buf != NUL)
	{
	    if (enc_codepage == -1)
		enc_codepage = GetACP();
	    vim_free(exe_name);
	    exe_name = utf16_to_enc(buf, NULL);
	    enc_prev = enc_codepage;

	    WCHAR *wp = wcsrchr(buf, '\\');
	    if (wp != NULL)
		*wp = NUL;
	    vim_free(exe_pathw);
	    exe_pathw = _wcsdup(buf);
	    updated = TRUE;
	}
    }

    if (exe_pathw == NULL || !updated)
	return;

    // Append our starting directory to $PATH, so that when doing
    // "!xxd" it's found in our starting directory.  Needed because
    // SearchPath() also looks there.
    WCHAR *p = _wgetenv(L"PATH");
    if (p == NULL || wcslen(p) + wcslen(exe_pathw) + 2 + 5 < MAX_ENV_PATH_LEN)
    {
	wcscpy(temp, L"PATH=");

	if (p == NULL || *p == NUL)
	    wcscat(temp, exe_pathw);
	else
	{
	    wcscat(temp, p);

	    // Check if exe_path is already included in $PATH.
	    if (wcsstr(temp, exe_pathw) == NULL)
	    {
		// Append ';' if $PATH doesn't end with it.
		size_t len = wcslen(temp);
		if (temp[len - 1] != L';')
		    wcscat(temp, L";");

		wcscat(temp, exe_pathw);
	    }
	}
	_wputenv(temp);
#ifdef libintl_wputenv
	libintl_wputenv(temp);
#endif
    }
}

/*
 * Unescape characters in "p" that appear in "escaped".
 */
    static void
unescape_shellxquote(char_u *p, char_u *escaped)
{
    int	    l = (int)STRLEN(p);
    int	    n;

    while (*p != NUL)
    {
	if (*p == '^' && vim_strchr(escaped, p[1]) != NULL)
	    mch_memmove(p, p + 1, l--);
	n = (*mb_ptr2len)(p);
	p += n;
	l -= n;
    }
}

/*
 * Load library "name".
 */
    HINSTANCE
vimLoadLib(const char *name)
{
    HINSTANCE	dll = NULL;

    // No need to load any library when registering OLE.
    if (found_register_arg)
	return NULL;

    // NOTE: Do not use mch_dirname() and mch_chdir() here, they may call
    // vimLoadLib() recursively, which causes a stack overflow.
    if (exe_pathw == NULL)
    {
	mch_get_exe_name();
	if (exe_pathw == NULL)
	    return NULL;
    }

    WCHAR old_dirw[MAXPATHL];

    if (GetCurrentDirectoryW(MAXPATHL, old_dirw) == 0)
	return NULL;

    // Change directory to where the executable is, both to make
    // sure we find a .dll there and to avoid looking for a .dll
    // in the current directory.
    SetCurrentDirectoryW(exe_pathw);
    dll = LoadLibrary(name);
    SetCurrentDirectoryW(old_dirw);
    return dll;
}

#if defined(VIMDLL) || defined(PROTO)
/*
 * Check if the current executable file is for the GUI subsystem.
 */
    int
mch_is_gui_executable(void)
{
    PBYTE		pImage = (PBYTE)GetModuleHandle(NULL);
    PIMAGE_DOS_HEADER	pDOS = (PIMAGE_DOS_HEADER)pImage;
    PIMAGE_NT_HEADERS	pPE;

    if (pDOS->e_magic != IMAGE_DOS_SIGNATURE)
	return FALSE;
    pPE = (PIMAGE_NT_HEADERS)(pImage + pDOS->e_lfanew);
    if (pPE->Signature != IMAGE_NT_SIGNATURE)
	return FALSE;
    if (pPE->OptionalHeader.Subsystem == IMAGE_SUBSYSTEM_WINDOWS_GUI)
	return TRUE;
    return FALSE;
}
#endif

#if defined(DYNAMIC_ICONV) || defined(DYNAMIC_GETTEXT) \
    || defined(FEAT_PYTHON3) || defined(PROTO)
/*
 * Get related information about 'funcname' which is imported by 'hInst'.
 * If 'info' is 0, return the function address.
 * If 'info' is 1, return the module name which the function is imported from.
 * If 'info' is 2, hook the function with 'ptr', and return the original
 * function address.
 */
    static void *
get_imported_func_info(HINSTANCE hInst, const char *funcname, int info,
	const void *ptr)
{
    PBYTE			pImage = (PBYTE)hInst;
    PIMAGE_DOS_HEADER		pDOS = (PIMAGE_DOS_HEADER)hInst;
    PIMAGE_NT_HEADERS		pPE;
    PIMAGE_IMPORT_DESCRIPTOR	pImpDesc;
    PIMAGE_THUNK_DATA		pIAT;	    // Import Address Table
    PIMAGE_THUNK_DATA		pINT;	    // Import Name Table
    PIMAGE_IMPORT_BY_NAME	pImpName;
    DWORD			ImpVA;

    if (pDOS->e_magic != IMAGE_DOS_SIGNATURE)
	return NULL;
    pPE = (PIMAGE_NT_HEADERS)(pImage + pDOS->e_lfanew);
    if (pPE->Signature != IMAGE_NT_SIGNATURE)
	return NULL;

    ImpVA = pPE->OptionalHeader
		.DataDirectory[IMAGE_DIRECTORY_ENTRY_IMPORT].VirtualAddress;
    if (ImpVA == 0)
	return NULL;	// No Import Table
    pImpDesc = (PIMAGE_IMPORT_DESCRIPTOR)(pImage + ImpVA);

    for (; pImpDesc->FirstThunk; ++pImpDesc)
    {
	if (!pImpDesc->OriginalFirstThunk)
	    continue;
	pIAT = (PIMAGE_THUNK_DATA)(pImage + pImpDesc->FirstThunk);
	pINT = (PIMAGE_THUNK_DATA)(pImage + pImpDesc->OriginalFirstThunk);
	for (; pIAT->u1.Function; ++pIAT, ++pINT)
	{
	    if (IMAGE_SNAP_BY_ORDINAL(pINT->u1.Ordinal))
		continue;
	    pImpName = (PIMAGE_IMPORT_BY_NAME)(pImage
					+ (UINT_PTR)(pINT->u1.AddressOfData));
	    if (strcmp((char *)pImpName->Name, funcname) == 0)
	    {
		void *original;
		DWORD old, new = PAGE_READWRITE;

		switch (info)
		{
		    case 0:
			return (void *)pIAT->u1.Function;
		    case 1:
			return (void *)(pImage + pImpDesc->Name);
		    case 2:
			original = (void *)pIAT->u1.Function;
			VirtualProtect(&pIAT->u1.Function, sizeof(void *),
				new, &old);
			pIAT->u1.Function = (UINT_PTR)ptr;
			VirtualProtect(&pIAT->u1.Function, sizeof(void *),
				old, &new);
			return original;
		    default:
			return NULL;
		}
	    }
	}
    }
    return NULL;
}

/*
 * Get the module handle which 'funcname' in 'hInst' is imported from.
 */
    HINSTANCE
find_imported_module_by_funcname(HINSTANCE hInst, const char *funcname)
{
    char    *modulename;

    modulename = (char *)get_imported_func_info(hInst, funcname, 1, NULL);
    if (modulename != NULL)
	return GetModuleHandleA(modulename);
    return NULL;
}

/*
 * Get the address of 'funcname' which is imported by 'hInst' DLL.
 */
    void *
get_dll_import_func(HINSTANCE hInst, const char *funcname)
{
    return get_imported_func_info(hInst, funcname, 0, NULL);
}

/*
 * Hook the function named 'funcname' which is imported by 'hInst' DLL,
 * and return the original function address.
 */
    void *
hook_dll_import_func(HINSTANCE hInst, const char *funcname, const void *hook)
{
    return get_imported_func_info(hInst, funcname, 2, hook);
}
#endif

#if defined(FEAT_PYTHON3) || defined(PROTO)
/*
 * Check if the specified DLL is a function forwarder.
 * If yes, return the instance of the forwarded DLL.
 * If no, return the specified DLL.
 * If error, return NULL.
 * This assumes that the DLL forwards all the function to a single DLL.
 */
    HINSTANCE
get_forwarded_dll(HINSTANCE hInst)
{
    PBYTE			pImage = (PBYTE)hInst;
    PIMAGE_DOS_HEADER		pDOS = (PIMAGE_DOS_HEADER)hInst;
    PIMAGE_NT_HEADERS		pPE;
    PIMAGE_EXPORT_DIRECTORY	pExpDir;
    DWORD			ExpVA;
    DWORD			ExpSize;
    LPDWORD			pFunctionTable;

    if (pDOS->e_magic != IMAGE_DOS_SIGNATURE)
	return NULL;
    pPE = (PIMAGE_NT_HEADERS)(pImage + pDOS->e_lfanew);
    if (pPE->Signature != IMAGE_NT_SIGNATURE)
	return NULL;

    ExpVA = pPE->OptionalHeader
		.DataDirectory[IMAGE_DIRECTORY_ENTRY_EXPORT].VirtualAddress;
    ExpSize = pPE->OptionalHeader
		.DataDirectory[IMAGE_DIRECTORY_ENTRY_EXPORT].Size;
    if (ExpVA == 0)
	return hInst;	// No Export Directory
    pExpDir = (PIMAGE_EXPORT_DIRECTORY)(pImage + ExpVA);
    pFunctionTable = (LPDWORD)(pImage + pExpDir->AddressOfFunctions);

    if (pExpDir->NumberOfNames == 0)
	return hInst;	// No export names.

    // Check only the first entry.
    if ((pFunctionTable[0] < ExpVA) || (pFunctionTable[0] >= ExpVA + ExpSize))
	// The first entry is not a function forwarder.
	return hInst;

    // The first entry is a function forwarder.
    // The name is represented as "DllName.FunctionName".
    const char *name = (const char *)(pImage + pFunctionTable[0]);
    const char *p = strchr(name, '.');
    if (p == NULL)
	return hInst;

    // Extract DllName.
    char buf[MAX_PATH];
    if (p - name + 1 > sizeof(buf))
	return NULL;
    strncpy(buf, name, p - name);
    buf[p - name] = '\0';
    return GetModuleHandleA(buf);
}
#endif

#if defined(DYNAMIC_GETTEXT) || defined(PROTO)
# ifndef GETTEXT_DLL
#  define GETTEXT_DLL "libintl.dll"
#  define GETTEXT_DLL_ALT1 "libintl-8.dll"
#  define GETTEXT_DLL_ALT2 "intl.dll"
# endif
// Dummy functions
static char *null_libintl_gettext(const char *);
static char *null_libintl_ngettext(const char *, const char *, unsigned long n);
static char *null_libintl_textdomain(const char *);
static char *null_libintl_bindtextdomain(const char *, const char *);
static char *null_libintl_bind_textdomain_codeset(const char *, const char *);
static int null_libintl_wputenv(const wchar_t *);

static HINSTANCE hLibintlDLL = NULL;
char *(*dyn_libintl_gettext)(const char *) = null_libintl_gettext;
char *(*dyn_libintl_ngettext)(const char *, const char *, unsigned long n)
						= null_libintl_ngettext;
char *(*dyn_libintl_textdomain)(const char *) = null_libintl_textdomain;
char *(*dyn_libintl_bindtextdomain)(const char *, const char *)
						= null_libintl_bindtextdomain;
char *(*dyn_libintl_bind_textdomain_codeset)(const char *, const char *)
				       = null_libintl_bind_textdomain_codeset;
int (*dyn_libintl_wputenv)(const wchar_t *) = null_libintl_wputenv;

    int
dyn_libintl_init(void)
{
    int i;
    static struct
    {
	char	    *name;
	FARPROC	    *ptr;
    } libintl_entry[] =
    {
	{"gettext", (FARPROC*)&dyn_libintl_gettext},
	{"ngettext", (FARPROC*)&dyn_libintl_ngettext},
	{"textdomain", (FARPROC*)&dyn_libintl_textdomain},
	{"bindtextdomain", (FARPROC*)&dyn_libintl_bindtextdomain},
	{NULL, NULL}
    };
    HINSTANCE hmsvcrt;

    // No need to initialize twice.
    if (hLibintlDLL != NULL)
	return 1;
    // Load gettext library (libintl.dll and other names).
    hLibintlDLL = vimLoadLib(GETTEXT_DLL);
# ifdef GETTEXT_DLL_ALT1
    if (!hLibintlDLL)
	hLibintlDLL = vimLoadLib(GETTEXT_DLL_ALT1);
# endif
# ifdef GETTEXT_DLL_ALT2
    if (!hLibintlDLL)
	hLibintlDLL = vimLoadLib(GETTEXT_DLL_ALT2);
# endif
    if (!hLibintlDLL)
    {
	if (p_verbose > 0)
	{
	    verbose_enter();
	    semsg(_(e_could_not_load_library_str_str), GETTEXT_DLL, GetWin32Error());
	    verbose_leave();
	}
	return 0;
    }
    for (i = 0; libintl_entry[i].name != NULL
					 && libintl_entry[i].ptr != NULL; ++i)
    {
	if ((*libintl_entry[i].ptr = GetProcAddress(hLibintlDLL,
					      libintl_entry[i].name)) == NULL)
	{
	    dyn_libintl_end();
	    if (p_verbose > 0)
	    {
		verbose_enter();
		semsg(_(e_could_not_load_library_function_str), libintl_entry[i].name);
		verbose_leave();
	    }
	    return 0;
	}
    }

    // The bind_textdomain_codeset() function is optional.
    dyn_libintl_bind_textdomain_codeset = (char *(*)(const char *, const char *))
			GetProcAddress(hLibintlDLL, "bind_textdomain_codeset");
    if (dyn_libintl_bind_textdomain_codeset == NULL)
	dyn_libintl_bind_textdomain_codeset =
					 null_libintl_bind_textdomain_codeset;

    // _wputenv() function for the libintl.dll is optional.
    hmsvcrt = find_imported_module_by_funcname(hLibintlDLL, "getenv");
    if (hmsvcrt != NULL)
	dyn_libintl_wputenv = (int (*)(const wchar_t *))
					GetProcAddress(hmsvcrt, "_wputenv");
    if (dyn_libintl_wputenv == NULL || dyn_libintl_wputenv == _wputenv)
	dyn_libintl_wputenv = null_libintl_wputenv;

    return 1;
}

    void
dyn_libintl_end(void)
{
    if (hLibintlDLL)
	FreeLibrary(hLibintlDLL);
    hLibintlDLL			= NULL;
    dyn_libintl_gettext		= null_libintl_gettext;
    dyn_libintl_ngettext	= null_libintl_ngettext;
    dyn_libintl_textdomain	= null_libintl_textdomain;
    dyn_libintl_bindtextdomain	= null_libintl_bindtextdomain;
    dyn_libintl_bind_textdomain_codeset = null_libintl_bind_textdomain_codeset;
    dyn_libintl_wputenv		= null_libintl_wputenv;
}

    static char *
null_libintl_gettext(const char *msgid)
{
    return (char*)msgid;
}

    static char *
null_libintl_ngettext(
	const char *msgid,
	const char *msgid_plural,
	unsigned long n)
{
    return (char *)(n == 1 ? msgid : msgid_plural);
}

    static char *
null_libintl_bindtextdomain(
	const char *domainname UNUSED,
	const char *dirname UNUSED)
{
    return NULL;
}

    static char *
null_libintl_bind_textdomain_codeset(
	const char *domainname UNUSED,
	const char *codeset UNUSED)
{
    return NULL;
}

    static char *
null_libintl_textdomain(const char *domainname UNUSED)
{
    return NULL;
}

    static int
null_libintl_wputenv(const wchar_t *envstring UNUSED)
{
    return 0;
}

#endif // DYNAMIC_GETTEXT

// This symbol is not defined in older versions of the SDK or Visual C++

#ifndef VER_PLATFORM_WIN32_WINDOWS
# define VER_PLATFORM_WIN32_WINDOWS 1
#endif

#ifdef HAVE_ACL
# ifndef PROTO
#  include <aclapi.h>
# endif
# ifndef PROTECTED_DACL_SECURITY_INFORMATION
#  define PROTECTED_DACL_SECURITY_INFORMATION	0x80000000L
# endif
#endif

#ifdef HAVE_ACL
/*
 * Enables or disables the specified privilege.
 */
    static BOOL
win32_enable_privilege(LPTSTR lpszPrivilege, BOOL bEnable)
{
    BOOL		bResult;
    LUID		luid;
    HANDLE		hToken;
    TOKEN_PRIVILEGES	tokenPrivileges;

    if (!OpenProcessToken(GetCurrentProcess(),
		TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY, &hToken))
	return FALSE;

    if (!LookupPrivilegeValue(NULL, lpszPrivilege, &luid))
    {
	CloseHandle(hToken);
	return FALSE;
    }

    tokenPrivileges.PrivilegeCount	     = 1;
    tokenPrivileges.Privileges[0].Luid       = luid;
    tokenPrivileges.Privileges[0].Attributes = bEnable ?
						    SE_PRIVILEGE_ENABLED : 0;

    bResult = AdjustTokenPrivileges(hToken, FALSE, &tokenPrivileges,
	    sizeof(TOKEN_PRIVILEGES), NULL, NULL);

    CloseHandle(hToken);

    return bResult && GetLastError() == ERROR_SUCCESS;
}
#endif

#ifdef _MSC_VER
// Suppress the deprecation warning for using GetVersionEx().
// It is needed for implementing "windowsversion()".
# pragma warning(push)
# pragma warning(disable: 4996)
#endif
/*
 * Set "win8_or_later" and fill in "windowsVersion" if possible.
 */
    void
PlatformId(void)
{
    static int done = FALSE;

    if (done)
	return;

    OSVERSIONINFO ovi;

    ovi.dwOSVersionInfoSize = sizeof(ovi);
    GetVersionEx(&ovi);

#ifdef FEAT_EVAL
    vim_snprintf(windowsVersion, sizeof(windowsVersion), "%d.%d",
	    (int)ovi.dwMajorVersion, (int)ovi.dwMinorVersion);
#endif
    if ((ovi.dwMajorVersion == 6 && ovi.dwMinorVersion >= 2)
	    || ovi.dwMajorVersion > 6)
	win8_or_later = TRUE;

    if ((ovi.dwMajorVersion == 10 && ovi.dwBuildNumber >= 19045)
	    || ovi.dwMajorVersion > 10)
	win10_22H2_or_later = TRUE;

#ifdef HAVE_ACL
    // Enable privilege for getting or setting SACLs.
    win32_enable_privilege(SE_SECURITY_NAME, TRUE);
#endif
    done = TRUE;
}
#ifdef _MSC_VER
# pragma warning(pop)
#endif

#if !defined(FEAT_GUI_MSWIN) || defined(VIMDLL)

# define SHIFT  (SHIFT_PRESSED)
# define CTRL   (RIGHT_CTRL_PRESSED | LEFT_CTRL_PRESSED)
# define ALT    (RIGHT_ALT_PRESSED  | LEFT_ALT_PRESSED)
# define ALT_GR (RIGHT_ALT_PRESSED  | LEFT_CTRL_PRESSED)


// When uChar.AsciiChar is 0, then we need to look at wVirtualKeyCode.
// We map function keys to their ANSI terminal equivalents, as produced
// by ANSI.SYS, for compatibility with the MS-DOS version of Vim.  Any
// ANSI key with a value >= '\300' is nonstandard, but provided anyway
// so that the user can have access to all SHIFT-, CTRL-, and ALT-
// combinations of function/arrow/etc keys.

static const struct
{
    WORD    wVirtKey;
    BOOL    fAnsiKey;
    int	    chAlone;
    int	    chShift;
    int	    chCtrl;
    int	    chAlt;
} VirtKeyMap[] =
{
//    Key	ANSI	alone	shift	ctrl	    alt
    { VK_ESCAPE,FALSE,	ESC,	ESC,	ESC,	    ESC,    },

    { VK_F1,	TRUE,	';',	'T',	'^',	    'h', },
    { VK_F2,	TRUE,	'<',	'U',	'_',	    'i', },
    { VK_F3,	TRUE,	'=',	'V',	'`',	    'j', },
    { VK_F4,	TRUE,	'>',	'W',	'a',	    'k', },
    { VK_F5,	TRUE,	'?',	'X',	'b',	    'l', },
    { VK_F6,	TRUE,	'@',	'Y',	'c',	    'm', },
    { VK_F7,	TRUE,	'A',	'Z',	'd',	    'n', },
    { VK_F8,	TRUE,	'B',	'[',	'e',	    'o', },
    { VK_F9,	TRUE,	'C',	'\\',	'f',	    'p', },
    { VK_F10,	TRUE,	'D',	']',	'g',	    'q', },
    { VK_F11,	TRUE,	'\205',	'\207',	'\211',	    '\213', },
    { VK_F12,	TRUE,	'\206',	'\210',	'\212',	    '\214', },

    { VK_HOME,	TRUE,	'G',	'\302',	'w',	    '\303', },
    { VK_UP,	TRUE,	'H',	'\304',	'\305',	    '\306', },
    { VK_PRIOR,	TRUE,	'I',	'\307',	'\204',	    '\310', }, // PgUp
    { VK_LEFT,	TRUE,	'K',	'\311',	's',	    '\312', },
    { VK_RIGHT,	TRUE,	'M',	'\313',	't',	    '\314', },
    { VK_END,	TRUE,	'O',	'\315',	'u',	    '\316', },
    { VK_DOWN,	TRUE,	'P',	'\317',	'\320',	    '\321', },
    { VK_NEXT,	TRUE,	'Q',	'\322',	'v',	    '\323', }, // PgDn
    { VK_INSERT,TRUE,	'R',	'\324',	'\325',	    '\326', },
    { VK_DELETE,TRUE,	'S',	'\327',	'\330',	    '\331', },
    { VK_BACK,	TRUE,	'x',	'y',	'z',	    '{', }, // Backspace

    { VK_SNAPSHOT,TRUE,	0,	0,	0,	    'r', }, // PrtScrn

# if 0
    // Most people don't have F13-F20, but what the hell...
    { VK_F13,	TRUE,	'\332',	'\333',	'\334',	    '\335', },
    { VK_F14,	TRUE,	'\336',	'\337',	'\340',	    '\341', },
    { VK_F15,	TRUE,	'\342',	'\343',	'\344',	    '\345', },
    { VK_F16,	TRUE,	'\346',	'\347',	'\350',	    '\351', },
    { VK_F17,	TRUE,	'\352',	'\353',	'\354',	    '\355', },
    { VK_F18,	TRUE,	'\356',	'\357',	'\360',	    '\361', },
    { VK_F19,	TRUE,	'\362',	'\363',	'\364',	    '\365', },
    { VK_F20,	TRUE,	'\366',	'\367',	'\370',	    '\371', },
# endif
    { VK_ADD,	TRUE,   'N',    'N',    'N',	'N',	}, // keyp '+'
    { VK_SUBTRACT, TRUE,'J',	'J',    'J',	'J',	}, // keyp '-'
 // { VK_DIVIDE,   TRUE,'N',	'N',    'N',	'N',	}, // keyp '/'
    { VK_MULTIPLY, TRUE,'7',	'7',    '7',	'7',	}, // keyp '*'

    { VK_NUMPAD0,TRUE,  '\332',	'\333',	'\334',	    '\335', },
    { VK_NUMPAD1,TRUE,  '\336',	'\337',	'\340',	    '\341', },
    { VK_NUMPAD2,TRUE,  '\342',	'\343',	'\344',	    '\345', },
    { VK_NUMPAD3,TRUE,  '\346',	'\347',	'\350',	    '\351', },
    { VK_NUMPAD4,TRUE,  '\352',	'\353',	'\354',	    '\355', },
    { VK_NUMPAD5,TRUE,  '\356',	'\357',	'\360',	    '\361', },
    { VK_NUMPAD6,TRUE,  '\362',	'\363',	'\364',	    '\365', },
    { VK_NUMPAD7,TRUE,  '\366',	'\367',	'\370',	    '\371', },
    { VK_NUMPAD8,TRUE,  '\372',	'\373',	'\374',	    '\375', },
    // Sorry, out of number space! <negri>
    { VK_NUMPAD9,TRUE,  '\376',	'\377',	'|',	    '}', },
};


/*
 * The return code indicates key code size.
 */
    static int
win32_kbd_patch_key(
    KEY_EVENT_RECORD *pker)
{
    UINT uMods = pker->dwControlKeyState;
    static int s_iIsDead = 0;
    static WORD awAnsiCode[2];
    static BYTE abKeystate[256];


    if (s_iIsDead == 2)
    {
	pker->uChar.UnicodeChar = (WCHAR) awAnsiCode[1];
	s_iIsDead = 0;
	return 1;
    }

    // check if it already has a valid unicode character.
    if (pker->uChar.UnicodeChar != 0)
	return 1;

    CLEAR_FIELD(abKeystate);

    // Clear any pending dead keys
    ToUnicode(VK_SPACE, MapVirtualKey(VK_SPACE, 0), abKeystate, awAnsiCode, 2, 0);

    if (uMods & SHIFT_PRESSED)
	abKeystate[VK_SHIFT] = 0x80;
    if (uMods & CAPSLOCK_ON)
	abKeystate[VK_CAPITAL] = 1;

    if ((uMods & ALT_GR) == ALT_GR)
    {
	abKeystate[VK_CONTROL] = abKeystate[VK_LCONTROL] =
	    abKeystate[VK_MENU] = abKeystate[VK_RMENU] = 0x80;
    }

    s_iIsDead = ToUnicode(pker->wVirtualKeyCode, pker->wVirtualScanCode,
			abKeystate, awAnsiCode, 2, 0);

    if (s_iIsDead > 0)
	pker->uChar.UnicodeChar = (WCHAR) awAnsiCode[0];

    return s_iIsDead;
}

static BOOL g_fJustGotFocus = FALSE;

/*
 * Decode a KEY_EVENT into one or two keystrokes
 */
    static BOOL
decode_key_event(
    KEY_EVENT_RECORD	*pker,
    WCHAR		*pch,
    WCHAR		*pch2,
    int			*pmodifiers,
    BOOL		fDoPost UNUSED)
{
    int i;
    const int nModifs = pker->dwControlKeyState & (SHIFT | ALT | CTRL);

    *pch = *pch2 = NUL;
    g_fJustGotFocus = FALSE;

    // ignore key up events
    if (!pker->bKeyDown)
	return FALSE;

    // ignore some keystrokes
    switch (pker->wVirtualKeyCode)
    {
    // modifiers
    case VK_SHIFT:
    case VK_CONTROL:
    case VK_MENU:   // Alt key
	return FALSE;

    default:
	break;
    }

    // Shift-TAB
    if (pker->wVirtualKeyCode == VK_TAB && (nModifs & SHIFT_PRESSED))
    {
	*pch = K_NUL;
	*pch2 = '\017';
	return TRUE;
    }

    for (i = ARRAY_LENGTH(VirtKeyMap);  --i >= 0;  )
    {
	if (VirtKeyMap[i].wVirtKey == pker->wVirtualKeyCode)
	{
	    *pch = VirtKeyMap[i].chAlone;
	    if ((nModifs & SHIFT) != 0)
		*pch = VirtKeyMap[i].chShift;
	    else if ((nModifs & CTRL) != 0 && (nModifs & ~CTRL) == 0)
		*pch = VirtKeyMap[i].chCtrl;
	    else if ((nModifs & ALT) != 0)
		*pch = VirtKeyMap[i].chAlt;

	    if (*pch != 0)
	    {
		if (VirtKeyMap[i].fAnsiKey)
		{
		    *pch2 = *pch;
		    *pch = K_NUL;
		    if (pmodifiers)
		    {
			if (pker->wVirtualKeyCode >= VK_F1
			    && pker->wVirtualKeyCode <= VK_F12)
			{
			    if ((nModifs & ALT) != 0)
			    {
				*pmodifiers |= MOD_MASK_ALT;
				if ((nModifs & SHIFT) == 0)
				    *pch2 = VirtKeyMap[i].chAlone;
			    }
			    if ((nModifs & CTRL) != 0)
			    {
				*pmodifiers |= MOD_MASK_CTRL;
				if ((nModifs & SHIFT) == 0)
				    *pch2 = VirtKeyMap[i].chAlone;
			    }
			}
			else if (pker->wVirtualKeyCode >= VK_END
				&& pker->wVirtualKeyCode <= VK_DOWN)
			{
			    // (0x23 - 0x28): VK_END, VK_HOME,
			    // VK_LEFT, VK_UP, VK_RIGHT, VK_DOWN

			    *pmodifiers = 0;
			    *pch2 = VirtKeyMap[i].chAlone;
			    if ((nModifs & SHIFT) != 0
						    && (nModifs & ~SHIFT) == 0)
			    {
				*pch2 = VirtKeyMap[i].chShift;
			    }
			    if ((nModifs & CTRL) != 0
						     && (nModifs & ~CTRL) == 0)
			    {
				*pch2 = VirtKeyMap[i].chCtrl;
				if (pker->wVirtualKeyCode == VK_UP
				    || pker->wVirtualKeyCode == VK_DOWN)
				{
				    *pmodifiers |= MOD_MASK_CTRL;
				    *pch2 = VirtKeyMap[i].chAlone;
				}
			    }
			    if ((nModifs & SHIFT) != 0
						      && (nModifs & CTRL) != 0)
			    {
				*pmodifiers |= MOD_MASK_CTRL;
				*pch2 = VirtKeyMap[i].chShift;
			    }
			    if ((nModifs & ALT) != 0)
			    {
				*pch2 = VirtKeyMap[i].chAlt;
				*pmodifiers |= MOD_MASK_ALT;
				if ((nModifs & ~ALT) == 0)
				{
				    *pch2 = VirtKeyMap[i].chAlone;
				}
				else if ((nModifs & SHIFT) != 0)
				{
				    *pch2 = VirtKeyMap[i].chShift;
				}
				else if ((nModifs & CTRL) != 0)
				{
				    if (pker->wVirtualKeyCode == VK_UP
					|| pker->wVirtualKeyCode == VK_DOWN)
				    {
					*pmodifiers |= MOD_MASK_CTRL;
					*pch2 = VirtKeyMap[i].chAlone;
				    }
				    else
				    {
					*pch2 = VirtKeyMap[i].chCtrl;
				    }
				}
			    }
			}
			else
			{
			    *pch2 = VirtKeyMap[i].chAlone;
			    if ((nModifs & SHIFT) != 0)
				*pmodifiers |= MOD_MASK_SHIFT;
			    if ((nModifs & CTRL) != 0)
				*pmodifiers |= MOD_MASK_CTRL;
			    if ((nModifs & ALT) != 0)
				*pmodifiers |= MOD_MASK_ALT;
			}
		    }
		}

		return TRUE;
	    }
	}
    }

    i = win32_kbd_patch_key(pker);

    if (i < 0)
	*pch = NUL;
    else
    {
	*pch = (i > 0) ? pker->uChar.UnicodeChar : NUL;

	if (pmodifiers != NULL)
	{
	    // Pass on the ALT key as a modifier, but only when not combined
	    // with CTRL (which is ALTGR).
	    if ((nModifs & ALT) != 0 && (nModifs & CTRL) == 0)
		*pmodifiers |= MOD_MASK_ALT;

	    // Pass on SHIFT only for special keys, because we don't know when
	    // it's already included with the character.
	    if ((nModifs & SHIFT) != 0 && *pch <= 0x20)
		*pmodifiers |= MOD_MASK_SHIFT;

	    // Pass on CTRL only for non-special keys, because we don't know
	    // when it's already included with the character.  And not when
	    // combined with ALT (which is ALTGR).
	    if ((nModifs & CTRL) != 0 && (nModifs & ALT) == 0
					       && *pch >= 0x20 && *pch < 0x80)
		*pmodifiers |= MOD_MASK_CTRL;
	}
    }

    return (*pch != NUL);
}

# if defined(FEAT_EVAL)
    static int
encode_key_event(dict_T *args, INPUT_RECORD *ir)
{
    static int s_dwMods = 0;

    char_u *action = dict_get_string(args, "event", TRUE);
    if (action && (STRICMP(action, "keydown") == 0
					|| STRICMP(action, "keyup") == 0))
    {
	BOOL isKeyDown = STRICMP(action, "keydown") == 0;
	WORD vkCode = dict_get_number_def(args, "keycode", 0);
	if (vkCode <= 0 || vkCode >= 0xFF)
	{
	    semsg(_(e_invalid_argument_nr), (long)vkCode);
	    return FALSE;
	}

	ir->EventType = KEY_EVENT;
	KEY_EVENT_RECORD ker;
	ZeroMemory(&ker, sizeof(ker));
	ker.bKeyDown = isKeyDown;
	ker.wRepeatCount = 1;
	ker.wVirtualScanCode = 0;
	ker.dwControlKeyState = 0;
	int mods = (int)dict_get_number(args, "modifiers");
	// Encode the win32 console key modifiers from Vim keyboard modifiers.
	if (mods)
	{
	    // If "modifiers" is explicitly set in the args, then we reset any
	    // remembered modifier key state that may have been set from
	    // earlier mod-key-down events, even if they are not yet unset by
	    // earlier mod-key-up events.
	    s_dwMods = 0;
	    if (mods & MOD_MASK_SHIFT)
		ker.dwControlKeyState |= SHIFT_PRESSED;
	    if (mods & MOD_MASK_CTRL)
		ker.dwControlKeyState |= LEFT_CTRL_PRESSED;
	    if (mods & MOD_MASK_ALT)
		ker.dwControlKeyState |= LEFT_ALT_PRESSED;
	}

	if (vkCode == VK_LSHIFT || vkCode == VK_RSHIFT || vkCode == VK_SHIFT)
	{
	    if (isKeyDown)
		s_dwMods |= SHIFT_PRESSED;
	    else
		s_dwMods &= ~SHIFT_PRESSED;
	}
	else if (vkCode == VK_LCONTROL || vkCode == VK_CONTROL)
	{
	    if (isKeyDown)
		s_dwMods |= LEFT_CTRL_PRESSED;
	    else
		s_dwMods &= ~LEFT_CTRL_PRESSED;
	}
	else if (vkCode == VK_RCONTROL)
	{
	    if (isKeyDown)
		s_dwMods |= RIGHT_CTRL_PRESSED;
	    else
		s_dwMods &= ~RIGHT_CTRL_PRESSED;
	}
	else if (vkCode == VK_LMENU || vkCode == VK_MENU)
	{
	    if (isKeyDown)
		s_dwMods |= LEFT_ALT_PRESSED;
	    else
		s_dwMods &= ~LEFT_ALT_PRESSED;
	}
	else if (vkCode == VK_RMENU)
	{
	    if (isKeyDown)
		s_dwMods |= RIGHT_ALT_PRESSED;
	    else
		s_dwMods &= ~RIGHT_ALT_PRESSED;
	}
	ker.dwControlKeyState |= s_dwMods;
	ker.wVirtualKeyCode = vkCode;
	ker.uChar.UnicodeChar = 0;
	ir->Event.KeyEvent = ker;
	vim_free(action);
    }
    else
    {
	if (action == NULL)
	{
	    semsg(_(e_missing_argument_str), "event");
	}
	else
	{
	    semsg(_(e_invalid_value_for_argument_str_str), "event", action);
	    vim_free(action);
	}
	return FALSE;
    }
    return TRUE;
}
# endif  // FEAT_EVAL
#endif // !FEAT_GUI_MSWIN || VIMDLL


/*
 * For the GUI the mouse handling is in gui_w32.c.
 */
#if defined(FEAT_GUI_MSWIN) && !defined(VIMDLL)
    void
mch_setmouse(int on UNUSED)
{
}
#else  // !FEAT_GUI_MSWIN || VIMDLL
static int g_fMouseAvail = FALSE;   // mouse present
static int g_fMouseActive = FALSE;  // mouse enabled
static int g_nMouseClick = -1;	    // mouse status
static int g_xMouse;		    // mouse x coordinate
static int g_yMouse;		    // mouse y coordinate
static DWORD g_cmodein = 0;	    // Original console input mode
static DWORD g_cmodeout = 0;	    // Original console output mode

/*
 * Enable or disable mouse input
 */
    void
mch_setmouse(int on)
{
    DWORD cmodein;

# ifdef VIMDLL
    if (gui.in_use)
	return;
# endif
    if (!g_fMouseAvail)
	return;

    g_fMouseActive = on;
    GetConsoleMode(g_hConIn, &cmodein);

    if (g_fMouseActive)
    {
	cmodein |= ENABLE_MOUSE_INPUT;
	cmodein &= ~ENABLE_QUICK_EDIT_MODE;
    }
    else
    {
	cmodein &= ~ENABLE_MOUSE_INPUT;
	cmodein |= g_cmodein & ENABLE_QUICK_EDIT_MODE;
    }

    SetConsoleMode(g_hConIn, cmodein | ENABLE_EXTENDED_FLAGS);
}


# if defined(FEAT_BEVAL_TERM) || defined(PROTO)
/*
 * Called when 'balloonevalterm' changed.
 */
    void
mch_bevalterm_changed(void)
{
    mch_setmouse(g_fMouseActive);
}
# endif

/*
 * Win32 console mouse scroll event handler.
 * Console version of the _OnMouseWheel() function in gui_w32.c
 *
 * This encodes the mouse scroll direction and keyboard modifiers into
 * g_nMouseClick, and the mouse position into g_xMouse and g_yMouse
 *
 * The direction of the scroll is decoded from two fields of the win32 console
 * mouse event record;
 *    1. The orientation - vertical or horizontal flag - from dwEventFlags
 *    2. The sign - positive or negative (aka delta flag) - from dwButtonState
 *
 * When scroll orientation is HORIZONTAL
 *    -  If the high word of the dwButtonState member contains a positive
 *	 value, the wheel was rotated to the right.
 *    -  Otherwise, the wheel was rotated to the left.
 * When scroll orientation is VERTICAL
 *    -  If the high word of the dwButtonState member contains a positive value,
 *       the wheel was rotated forward, away from the user.
 *    -  Otherwise, the wheel was rotated backward, toward the user.
 */
    static void
decode_mouse_wheel(MOUSE_EVENT_RECORD *pmer)
{
    int	    horizontal = (pmer->dwEventFlags == MOUSE_HWHEELED);
    int	    zDelta = pmer->dwButtonState;

    g_xMouse = pmer->dwMousePosition.X;
    g_yMouse = pmer->dwMousePosition.Y;

# ifdef FEAT_PROP_POPUP
    int lcol = g_xMouse;
    int lrow = g_yMouse;
    win_T *wp = mouse_find_win(&lrow, &lcol, FIND_POPUP);
    if (wp != NULL && popup_is_popup(wp))
    {
	g_nMouseClick = -1;
	cmdarg_T cap;
	oparg_T oa;
	CLEAR_FIELD(cap);
	clear_oparg(&oa);
	cap.oap = &oa;
	if (horizontal)
	{
	    cap.arg = zDelta < 0 ? MSCR_LEFT : MSCR_RIGHT;
	    cap.cmdchar = zDelta < 0 ? K_MOUSELEFT : K_MOUSERIGHT;
	}
	else
	{
	    cap.cmdchar = zDelta < 0 ? K_MOUSEUP : K_MOUSEDOWN;
	    cap.arg = zDelta < 0 ? MSCR_UP : MSCR_DOWN;
	}

	// Mouse hovers over popup window, scroll it if possible.
	mouse_row = wp->w_winrow;
	mouse_col = wp->w_wincol;
	nv_mousescroll(&cap);
	update_screen(0);
	setcursor();
	out_flush();
	return;
    }
# endif
    mouse_col = g_xMouse;
    mouse_row = g_yMouse;

    char_u modifiers = 0;
    char_u direction = 0;

    // Decode the direction into an event that Vim can process
    if (horizontal)
	direction = zDelta >= 0 ? KE_MOUSELEFT : KE_MOUSERIGHT;
    else
	direction = zDelta >= 0 ? KE_MOUSEDOWN : KE_MOUSEUP;

    // Decode the win32 console key modifiers into Vim mouse modifiers.
    if (pmer->dwControlKeyState & SHIFT_PRESSED)
	modifiers |= MOD_MASK_SHIFT; // MOUSE_SHIFT;
    if (pmer->dwControlKeyState & (RIGHT_CTRL_PRESSED | LEFT_CTRL_PRESSED))
	modifiers |= MOD_MASK_CTRL; // MOUSE_CTRL;
    if (pmer->dwControlKeyState & (RIGHT_ALT_PRESSED  | LEFT_ALT_PRESSED))
	modifiers |= MOD_MASK_ALT; // MOUSE_ALT;

    // add (bitwise or) the scroll direction and the key modifier chars
    // together.
    g_nMouseClick = ((direction << 8) | modifiers);

    return;
}

/*
 * Decode a MOUSE_EVENT.  If it's a valid event, return MOUSE_LEFT,
 * MOUSE_MIDDLE, or MOUSE_RIGHT for a click; MOUSE_DRAG for a mouse
 * move with a button held down; and MOUSE_RELEASE after a MOUSE_DRAG
 * or a MOUSE_LEFT, _MIDDLE, or _RIGHT.  We encode the button type,
 * the number of clicks, and the Shift/Ctrl/Alt modifiers in g_nMouseClick,
 * and we return the mouse position in g_xMouse and g_yMouse.
 *
 * Every MOUSE_LEFT, _MIDDLE, or _RIGHT will be followed by zero or more
 * MOUSE_DRAGs and one MOUSE_RELEASE.  MOUSE_RELEASE will be followed only
 * by MOUSE_LEFT, _MIDDLE, or _RIGHT.
 *
 * For multiple clicks, we send, say, MOUSE_LEFT/1 click, MOUSE_RELEASE,
 * MOUSE_LEFT/2 clicks, MOUSE_RELEASE, MOUSE_LEFT/3 clicks, MOUSE_RELEASE, ....
 *
 * Windows will send us MOUSE_MOVED notifications whenever the mouse
 * moves, even if it stays within the same character cell.  We ignore
 * all MOUSE_MOVED messages if the position hasn't really changed, and
 * we ignore all MOUSE_MOVED messages where no button is held down (i.e.,
 * we're only interested in MOUSE_DRAG).
 *
 * All of this is complicated by the code that fakes MOUSE_MIDDLE on
 * 2-button mouses by pressing the left & right buttons simultaneously.
 * In practice, it's almost impossible to click both at the same time,
 * so we need to delay a little.  Also, we tend not to get MOUSE_RELEASE
 * in such cases, if the user is clicking quickly.
 */
    static BOOL
decode_mouse_event(
    MOUSE_EVENT_RECORD *pmer)
{
    static int s_nOldButton = -1;
    static int s_nOldMouseClick = -1;
    static int s_xOldMouse = -1;
    static int s_yOldMouse = -1;
    static linenr_T s_old_topline = 0;
# ifdef FEAT_DIFF
    static int s_old_topfill = 0;
# endif
    static int s_cClicks = 1;
    static BOOL s_fReleased = TRUE;
    static DWORD s_dwLastClickTime = 0;
    static BOOL s_fNextIsMiddle = FALSE;

    static DWORD cButtons = 0;	// number of buttons supported

    const DWORD LEFT = FROM_LEFT_1ST_BUTTON_PRESSED;
    const DWORD MIDDLE = FROM_LEFT_2ND_BUTTON_PRESSED;
    const DWORD RIGHT = RIGHTMOST_BUTTON_PRESSED;
    const DWORD LEFT_RIGHT = LEFT | RIGHT;

    int nButton;

    if (cButtons == 0 && !GetNumberOfConsoleMouseButtons(&cButtons))
	cButtons = 2;

    if (!g_fMouseAvail || !g_fMouseActive)
    {
	g_nMouseClick = -1;
	return FALSE;
    }

    // get a spurious MOUSE_EVENT immediately after receiving focus; ignore
    if (g_fJustGotFocus)
    {
	g_fJustGotFocus = FALSE;
	return FALSE;
    }

    // If there is an unprocessed mouse click drop this one.
    if (g_nMouseClick != -1)
	return TRUE;

    if (pmer->dwEventFlags == MOUSE_WHEELED
				       || pmer->dwEventFlags == MOUSE_HWHEELED)
    {
	decode_mouse_wheel(pmer);
	return TRUE;  // we now should have a mouse scroll in g_nMouseClick
    }

    nButton = -1;
    g_xMouse = pmer->dwMousePosition.X;
    g_yMouse = pmer->dwMousePosition.Y;

    if (pmer->dwEventFlags == MOUSE_MOVED)
    {
	// Ignore MOUSE_MOVED events if (x, y) hasn't changed.	(We get these
	// events even when the mouse moves only within a char cell.)
	if (s_xOldMouse == g_xMouse && s_yOldMouse == g_yMouse)
	    return FALSE;
    }

    // If no buttons are pressed...
    if ((pmer->dwButtonState & ((1 << cButtons) - 1)) == 0)
    {
	nButton = MOUSE_RELEASE;

	// If the last thing returned was MOUSE_RELEASE, ignore this
	if (s_fReleased)
	{
# ifdef FEAT_BEVAL_TERM
	    // do return mouse move events when we want them
	    if (p_bevalterm)
		nButton = MOUSE_DRAG;
	    else
# endif
		return FALSE;
	}

	s_fReleased = TRUE;
    }
    else    // one or more buttons pressed
    {
	// on a 2-button mouse, hold down left and right buttons
	// simultaneously to get MIDDLE.

	if (cButtons == 2 && s_nOldButton != MOUSE_DRAG)
	{
	    DWORD dwLR = (pmer->dwButtonState & LEFT_RIGHT);

	    // if either left or right button only is pressed, see if the
	    // next mouse event has both of them pressed
	    if (dwLR == LEFT || dwLR == RIGHT)
	    {
		for (;;)
		{
		    // wait a short time for next input event
		    if (WaitForSingleObject(g_hConIn, p_mouset / 3)
							     != WAIT_OBJECT_0)
			break;
		    else
		    {
			DWORD cRecords = 0;
			INPUT_RECORD ir;
			MOUSE_EVENT_RECORD* pmer2 = &ir.Event.MouseEvent;

			peek_console_input(g_hConIn, &ir, 1, &cRecords);

			if (cRecords == 0 || ir.EventType != MOUSE_EVENT
				|| !(pmer2->dwButtonState & LEFT_RIGHT))
			    break;
			else
			{
			    if (pmer2->dwEventFlags != MOUSE_MOVED)
			    {
				read_console_input(g_hConIn, &ir, 1, &cRecords);

				return decode_mouse_event(pmer2);
			    }
			    else if (s_xOldMouse == pmer2->dwMousePosition.X &&
				     s_yOldMouse == pmer2->dwMousePosition.Y)
			    {
				// throw away spurious mouse move
				read_console_input(g_hConIn, &ir, 1, &cRecords);

				// are there any more mouse events in queue?
				peek_console_input(g_hConIn, &ir, 1, &cRecords);

				if (cRecords==0 || ir.EventType != MOUSE_EVENT)
				    break;
			    }
			    else
				break;
			}
		    }
		}
	    }
	}

	if (s_fNextIsMiddle)
	{
	    nButton = (pmer->dwEventFlags == MOUSE_MOVED)
		? MOUSE_DRAG : MOUSE_MIDDLE;
	    s_fNextIsMiddle = FALSE;
	}
	else if (cButtons == 2	&&
	    ((pmer->dwButtonState & LEFT_RIGHT) == LEFT_RIGHT))
	{
	    nButton = MOUSE_MIDDLE;

	    if (! s_fReleased && pmer->dwEventFlags != MOUSE_MOVED)
	    {
		s_fNextIsMiddle = TRUE;
		nButton = MOUSE_RELEASE;
	    }
	}
	else if ((pmer->dwButtonState & LEFT) == LEFT)
	    nButton = MOUSE_LEFT;
	else if ((pmer->dwButtonState & MIDDLE) == MIDDLE)
	    nButton = MOUSE_MIDDLE;
	else if ((pmer->dwButtonState & RIGHT) == RIGHT)
	    nButton = MOUSE_RIGHT;

	if (! s_fReleased && ! s_fNextIsMiddle
		&& nButton != s_nOldButton && s_nOldButton != MOUSE_DRAG)
	    return FALSE;

	s_fReleased = s_fNextIsMiddle;
    }

    if (pmer->dwEventFlags == 0 || pmer->dwEventFlags == DOUBLE_CLICK)
    {
	// button pressed or released, without mouse moving
	if (nButton != -1 && nButton != MOUSE_RELEASE)
	{
	    DWORD dwCurrentTime = GetTickCount();

	    if (s_xOldMouse != g_xMouse
		    || s_yOldMouse != g_yMouse
		    || s_nOldButton != nButton
		    || s_old_topline != curwin->w_topline
# ifdef FEAT_DIFF
		    || s_old_topfill != curwin->w_topfill
# endif
		    || (int)(dwCurrentTime - s_dwLastClickTime) > p_mouset)
	    {
		s_cClicks = 1;
	    }
	    else if (++s_cClicks > 4)
	    {
		s_cClicks = 1;
	    }

	    s_dwLastClickTime = dwCurrentTime;
	}
    }
    else if (pmer->dwEventFlags == MOUSE_MOVED)
    {
	if (nButton != -1 && nButton != MOUSE_RELEASE)
	    nButton = MOUSE_DRAG;

	s_cClicks = 1;
    }

    if (nButton == -1)
	return FALSE;

    if (nButton != MOUSE_RELEASE)
	s_nOldButton = nButton;

    g_nMouseClick = nButton;

    if (pmer->dwControlKeyState & SHIFT_PRESSED)
	g_nMouseClick |= MOUSE_SHIFT;
    if (pmer->dwControlKeyState & (RIGHT_CTRL_PRESSED | LEFT_CTRL_PRESSED))
	g_nMouseClick |= MOUSE_CTRL;
    if (pmer->dwControlKeyState & (RIGHT_ALT_PRESSED  | LEFT_ALT_PRESSED))
	g_nMouseClick |= MOUSE_ALT;

    if (nButton != MOUSE_DRAG && nButton != MOUSE_RELEASE)
	SET_NUM_MOUSE_CLICKS(g_nMouseClick, s_cClicks);

    // only pass on interesting (i.e., different) mouse events
    if (s_xOldMouse == g_xMouse
	    && s_yOldMouse == g_yMouse
	    && s_nOldMouseClick == g_nMouseClick)
    {
	g_nMouseClick = -1;
	return FALSE;
    }

    s_xOldMouse = g_xMouse;
    s_yOldMouse = g_yMouse;
    s_old_topline = curwin->w_topline;
# ifdef FEAT_DIFF
    s_old_topfill = curwin->w_topfill;
# endif
    s_nOldMouseClick = g_nMouseClick;

    return TRUE;
}

# ifdef FEAT_EVAL
    static int
encode_mouse_event(dict_T *args, INPUT_RECORD *ir)
{
    int		button;
    int		row;
    int		col;
    int		repeated_click;
    int_u	mods = 0;
    int		move;

    if (!dict_has_key(args, "row") || !dict_has_key(args, "col"))
	return FALSE;

    // Note: "move" is optional, requires fewer arguments
    move = (int)dict_get_bool(args, "move", FALSE);
    if (!move && (!dict_has_key(args, "button")
	    || !dict_has_key(args, "multiclick")
	    || !dict_has_key(args, "modifiers")))
	return FALSE;

    row = (int)dict_get_number(args, "row") - 1;
    col = (int)dict_get_number(args, "col") - 1;

    ir->EventType = MOUSE_EVENT;
    MOUSE_EVENT_RECORD mer;
    ZeroMemory(&mer, sizeof(mer));
    mer.dwMousePosition.X  = col;
    mer.dwMousePosition.Y  = row;

    if (move)
    {
	mer.dwButtonState = 0;
	mer.dwEventFlags = MOUSE_MOVED;
    }
    else
    {
	button = (int)dict_get_number(args, "button");
	repeated_click = (int)dict_get_number(args, "multiclick");
	mods = (int)dict_get_number(args, "modifiers");
	// Reset the scroll values to known values.
	// XXX: Remove this when/if the scroll step is made configurable.
	mouse_set_hor_scroll_step(6);
	mouse_set_vert_scroll_step(3);

	switch (button)
	{
	    case MOUSE_LEFT:
		mer.dwButtonState = FROM_LEFT_1ST_BUTTON_PRESSED;
		mer.dwEventFlags = repeated_click == 1 ? DOUBLE_CLICK : 0;
		break;
	    case MOUSE_MIDDLE:
		mer.dwButtonState = FROM_LEFT_2ND_BUTTON_PRESSED;
		mer.dwEventFlags = repeated_click == 1 ? DOUBLE_CLICK : 0;
		break;
	    case MOUSE_RIGHT:
		mer.dwButtonState = RIGHTMOST_BUTTON_PRESSED;
		mer.dwEventFlags = repeated_click == 1 ? DOUBLE_CLICK : 0;
		break;
	    case MOUSE_RELEASE:
		// umm?  Assume Left Release?
		mer.dwEventFlags = 0;

	    case MOUSE_MOVE:
		mer.dwButtonState = 0;
		mer.dwEventFlags = MOUSE_MOVED;
		break;
	    case MOUSE_X1:
		mer.dwButtonState = FROM_LEFT_3RD_BUTTON_PRESSED;
		break;
	    case MOUSE_X2:
		mer.dwButtonState = FROM_LEFT_4TH_BUTTON_PRESSED;
		break;
	    case MOUSE_4:  // KE_MOUSEDOWN;
		mer.dwButtonState = -1;
		mer.dwEventFlags = MOUSE_WHEELED;
		break;
	    case MOUSE_5:  // KE_MOUSEUP;
		mer.dwButtonState = +1;
		mer.dwEventFlags = MOUSE_WHEELED;
		break;
	    case MOUSE_6:  // KE_MOUSELEFT;
		mer.dwButtonState = -1;
		mer.dwEventFlags = MOUSE_HWHEELED;
		break;
	    case MOUSE_7:  // KE_MOUSERIGHT;
		mer.dwButtonState = +1;
		mer.dwEventFlags = MOUSE_HWHEELED;
		break;
	    default:
		semsg(_(e_invalid_argument_str), "button");
		return FALSE;
	}
    }

    mer.dwControlKeyState = 0;
    if (mods != 0)
    {
	// Encode the win32 console key modifiers from Vim MOUSE modifiers.
	if (mods & MOUSE_SHIFT)
	    mer.dwControlKeyState |= SHIFT_PRESSED;
	if (mods & MOUSE_CTRL)
	    mer.dwControlKeyState |= LEFT_CTRL_PRESSED;
	if (mods & MOUSE_ALT)
	    mer.dwControlKeyState |= LEFT_ALT_PRESSED;
    }
    ir->Event.MouseEvent = mer;
    return TRUE;
}
# endif  // FEAT_EVAL

    static int
write_input_record_buffer(INPUT_RECORD* irEvents, int nLength)
{
    int nCount = 0;
    while (nCount < nLength)
    {
	input_record_buffer.length++;
	input_record_buffer_node_T *event_node =
				    malloc(sizeof(input_record_buffer_node_T));
	event_node->ir = irEvents[nCount++];
	event_node->next = NULL;
	if (input_record_buffer.tail == NULL)
	{
	    input_record_buffer.head = event_node;
	    input_record_buffer.tail = event_node;
	}
	else
	{
	    input_record_buffer.tail->next = event_node;
	    input_record_buffer.tail = input_record_buffer.tail->next;
	}
    }
    return nCount;
}

    static int
read_input_record_buffer(INPUT_RECORD* irEvents, int nMaxLength)
{
    int nCount = 0;
    while (nCount < nMaxLength && input_record_buffer.head != NULL)
    {
	input_record_buffer.length--;
	input_record_buffer_node_T *pop_head = input_record_buffer.head;
	irEvents[nCount++] = pop_head->ir;
	input_record_buffer.head = pop_head->next;
	vim_free(pop_head);
	if (input_record_buffer.length == 0)
	    input_record_buffer.tail = NULL;
    }
    return nCount;
}
#endif // !FEAT_GUI_MSWIN || VIMDLL

#ifdef FEAT_EVAL
/*
 * The 'test_mswin_event' function is for testing Vim's low-level handling of
 * user input events.  ie, this manages the encoding of INPUT_RECORD events
 * so that we have a way to test how Vim decodes INPUT_RECORD events in Windows
 * consoles.
 *
 * The 'test_mswin_event' function is based on 'test_gui_event'.  In fact, when
 * the Windows GUI is running, the arguments; 'event' and 'args', are the same.
 * So, it acts as an alias for 'test_gui_event' for the Windows GUI.
 *
 * When the Windows console is running, the arguments; 'event' and 'args', are
 * a subset of what 'test_gui_event' handles, ie, only "key" and "mouse"
 * events are encoded as INPUT_RECORD events.
 *
 * Note: INPUT_RECORDs are only used by the Windows console, not the GUI.  The
 * GUI sends MSG structs instead.
 */
    int
test_mswin_event(char_u *event, dict_T *args)
{
    int lpEventsWritten = 0;

# if defined(VIMDLL) || defined(FEAT_GUI_MSWIN)
    if (gui.in_use)
	return test_gui_w32_sendevent(event, args);
# endif

# if defined(VIMDLL) || !defined(FEAT_GUI_MSWIN)

// Currently implemented event record types are; KEY_EVENT and MOUSE_EVENT
// Potentially could also implement: FOCUS_EVENT and WINDOW_BUFFER_SIZE_EVENT
// Maybe also:  MENU_EVENT

    INPUT_RECORD ir;
    BOOL input_encoded = FALSE;
    BOOL execute = FALSE;
    if (STRCMP(event, "key") == 0)
    {
	execute = dict_get_bool(args, "execute", FALSE);
	if (dict_has_key(args, "event"))
	    input_encoded = encode_key_event(args, &ir);
	else if (!execute)
	{
	    semsg(_(e_missing_argument_str), "event");
	    return FALSE;
	}
    }
    else if (STRCMP(event, "mouse") == 0)
    {
	execute = TRUE;
	input_encoded = encode_mouse_event(args, &ir);
    }
    else
    {
	semsg(_(e_invalid_value_for_argument_str_str), "event", event);
	return FALSE;
    }

    // Ideally, WriteConsoleInput would be used to inject these low-level
    // events.  But, this doesn't work well in the CI test environment.  So
    // implementing an input_record_buffer instead.
    if (input_encoded)
	lpEventsWritten = write_input_record_buffer(&ir, 1);

    // Set flags to execute the event, ie. like feedkeys mode X.
    if (execute)
    {
	int save_msg_scroll = msg_scroll;
	// Avoid a 1 second delay when the keys start Insert mode.
	msg_scroll = FALSE;
	ch_log(NULL, "test_mswin_event() executing");
	exec_normal(TRUE, TRUE, TRUE);
	msg_scroll |= save_msg_scroll;
    }

# endif
    return lpEventsWritten;
}
#endif // FEAT_EVAL

#ifdef MCH_CURSOR_SHAPE
/*
 * Set the shape of the cursor.
 * 'thickness' can be from 1 (thin) to 99 (block)
 */
    static void
mch_set_cursor_shape(int thickness)
{
    if (vtp_working)
    {
	if (*T_CSI == NUL)
	{
	    // If 't_SI' is not set, use the default cursor styles.
	    if (thickness < 50)
		vtp_printf("\033[3 q");	// underline
	    else
		vtp_printf("\033[0 q");	// default
	}
    }
    else
    {
	CONSOLE_CURSOR_INFO ConsoleCursorInfo;
	ConsoleCursorInfo.dwSize = thickness;
	ConsoleCursorInfo.bVisible = s_cursor_visible;

	SetConsoleCursorInfo(g_hConOut, &ConsoleCursorInfo);
	if (s_cursor_visible)
	    SetConsoleCursorPosition(g_hConOut, g_coord);
    }
}

    void
mch_update_cursor(void)
{
    int		idx;
    int		thickness;

# ifdef VIMDLL
    if (gui.in_use)
	return;
# endif

    /*
     * How the cursor is drawn depends on the current mode.
     */
    idx = get_shape_idx(FALSE);

    if (shape_table[idx].shape == SHAPE_BLOCK)
	thickness = 99;	// 100 doesn't work on W95
    else
	thickness = shape_table[idx].percentage;
    mch_set_cursor_shape(thickness);
}
#endif

#if !defined(FEAT_GUI_MSWIN) || defined(VIMDLL)
/*
 * Handle FOCUS_EVENT.
 */
    static void
handle_focus_event(INPUT_RECORD ir)
{
    g_fJustGotFocus = ir.Event.FocusEvent.bSetFocus;
    ui_focus_change((int)g_fJustGotFocus);
}

static void ResizeConBuf(HANDLE hConsole, COORD coordScreen);

/*
 * Wait until console input from keyboard or mouse is available,
 * or the time is up.
 * When "ignore_input" is TRUE even wait when input is available.
 * Return TRUE if something is available FALSE if not.
 */
    static int
WaitForChar(long msec, int ignore_input)
{
    DWORD	    dwNow = 0, dwEndTime = 0;
    INPUT_RECORD    ir;
    DWORD	    cRecords;
    WCHAR	    ch, ch2;
# ifdef FEAT_TIMERS
    int		    tb_change_cnt = typebuf.tb_change_cnt;
# endif

    if (msec > 0)
	// Wait until the specified time has elapsed.
	dwEndTime = GetTickCount() + msec;
    else if (msec < 0)
	// Wait forever.
	dwEndTime = INFINITE;

    // We need to loop until the end of the time period, because
    // we might get multiple unusable mouse events in that time.
    for (;;)
    {
	// Only process messages when waiting.
	if (msec != 0)
	{
# ifdef MESSAGE_QUEUE
	    parse_queued_messages();
# endif
# ifdef FEAT_MZSCHEME
	    mzvim_check_threads();
# endif
# ifdef FEAT_CLIENTSERVER
	    serverProcessPendingMessages();
# endif
	}

	if (g_nMouseClick != -1
# ifdef FEAT_CLIENTSERVER
		|| (!ignore_input && input_available())
# endif
	   )
	    return TRUE;

	if (msec > 0)
	{
	    // If the specified wait time has passed, return.  Beware that
	    // GetTickCount() may wrap around (overflow).
	    dwNow = GetTickCount();
	    if ((int)(dwNow - dwEndTime) >= 0)
		break;
	}
	if (msec != 0)
	{
	    DWORD dwWaitTime = dwEndTime - dwNow;

	    // Don't wait for more than 11 msec to avoid dropping characters,
	    // check channel while waiting for input and handle a callback from
	    // 'balloonexpr'.
	    if (dwWaitTime > 11)
		dwWaitTime = 11;

# ifdef FEAT_MZSCHEME
	    if (mzthreads_allowed() && p_mzq > 0 && (long)dwWaitTime > p_mzq)
		dwWaitTime = p_mzq; // don't wait longer than 'mzquantum'
# endif
# ifdef FEAT_TIMERS
	    // When waiting very briefly don't trigger timers.
	    if (dwWaitTime > 10)
	    {
		long	due_time;

		// Trigger timers and then get the time in msec until the next
		// one is due.  Wait up to that time.
		due_time = check_due_timer();
		if (typebuf.tb_change_cnt != tb_change_cnt)
		{
		    // timer may have used feedkeys().
		    return FALSE;
		}
		if (due_time > 0 && dwWaitTime > (DWORD)due_time)
		    dwWaitTime = due_time;
	    }
# endif
	    if (
# ifdef FEAT_CLIENTSERVER
		    // Wait for either an event on the console input or a
		    // message in the client-server window.
		    msg_wait_for_multiple_objects(1, &g_hConIn, FALSE,
				  dwWaitTime, QS_SENDMESSAGE) != WAIT_OBJECT_0
# else
		    wait_for_single_object(g_hConIn, dwWaitTime)
							      != WAIT_OBJECT_0
# endif
		    )
		continue;
	}

	cRecords = 0;
	peek_console_input(g_hConIn, &ir, 1, &cRecords);

# ifdef FEAT_MBYTE_IME
	// May have to redraw if the cursor ends up in the wrong place.
	// Only when not peeking.
	if (State == MODE_CMDLINE && msg_row == Rows - 1 && msec != 0)
	{
	    CONSOLE_SCREEN_BUFFER_INFO csbi;

	    if (GetConsoleScreenBufferInfo(g_hConOut, &csbi))
	    {
		if (csbi.dwCursorPosition.Y != msg_row)
		{
		    // The screen is now messed up, must redraw the command
		    // line and later all the windows.
		    redraw_all_later(UPD_CLEAR);
		    compute_cmdrow();
		    redrawcmd();
		}
	    }
	}
# endif

	if (cRecords > 0)
	{
	    if (ir.EventType == KEY_EVENT && ir.Event.KeyEvent.bKeyDown)
	    {
# ifdef FEAT_MBYTE_IME
		// Windows IME sends two '\n's with only one 'ENTER'.  First:
		// wVirtualKeyCode == 13. second: wVirtualKeyCode == 0
		if (ir.Event.KeyEvent.uChar.UnicodeChar == 0
			&& ir.Event.KeyEvent.wVirtualKeyCode == 13)
		{
		    read_console_input(g_hConIn, &ir, 1, &cRecords);
		    continue;
		}
# endif
		if (decode_key_event(&ir.Event.KeyEvent, &ch, &ch2,
								 NULL, FALSE))
		    return TRUE;
	    }

	    read_console_input(g_hConIn, &ir, 1, &cRecords);

	    if (ir.EventType == FOCUS_EVENT)
		handle_focus_event(ir);
	    else if (ir.EventType == WINDOW_BUFFER_SIZE_EVENT)
	    {
		COORD dwSize = ir.Event.WindowBufferSizeEvent.dwSize;

		// Only call shell_resized() when the size actually changed to
		// avoid the screen is cleared.
		if (dwSize.X != Columns || dwSize.Y != Rows)
		{
		    CONSOLE_SCREEN_BUFFER_INFO csbi;
		    GetConsoleScreenBufferInfo(g_hConOut, &csbi);
		    dwSize.X = csbi.srWindow.Right - csbi.srWindow.Left + 1;
		    dwSize.Y = csbi.srWindow.Bottom - csbi.srWindow.Top + 1;
		    if (dwSize.X != Columns || dwSize.Y != Rows)
		    {
			ResizeConBuf(g_hConOut, dwSize);
			shell_resized();
		    }
		}
	    }
	    else if (ir.EventType == MOUSE_EVENT
		    && decode_mouse_event(&ir.Event.MouseEvent))
		return TRUE;
	}
	else if (msec == 0)
	    break;
    }

# ifdef FEAT_CLIENTSERVER
    // Something might have been received while we were waiting.
    if (input_available())
	return TRUE;
# endif

    return FALSE;
}

/*
 * return non-zero if a character is available
 */
    int
mch_char_avail(void)
{
# ifdef VIMDLL
    if (gui.in_use)
	return TRUE;
# endif
    return WaitForChar(0L, FALSE);
}

# if defined(FEAT_TERMINAL) || defined(PROTO)
/*
 * Check for any pending input or messages.
 */
    int
mch_check_messages(void)
{
#  ifdef VIMDLL
    if (gui.in_use)
	return TRUE;
#  endif
    return WaitForChar(0L, TRUE);
}
# endif

/*
 * Create the console input.  Used when reading stdin doesn't work.
 */
    static void
create_conin(void)
{
    g_hConIn =	CreateFile("CONIN$", GENERIC_READ|GENERIC_WRITE,
			FILE_SHARE_READ|FILE_SHARE_WRITE,
			(LPSECURITY_ATTRIBUTES) NULL,
			OPEN_EXISTING, 0, (HANDLE)NULL);
    did_create_conin = TRUE;
}

/*
 * Get a keystroke or a mouse event, use a blocking wait.
 */
    static WCHAR
tgetch(int *pmodifiers, WCHAR *pch2)
{
    WCHAR ch;

    for (;;)
    {
	INPUT_RECORD ir;
	DWORD cRecords = 0;

# ifdef FEAT_CLIENTSERVER
	(void)WaitForChar(-1L, FALSE);
	if (input_available())
	    return 0;
	if (g_nMouseClick != -1)
	    return 0;
# endif
	if (read_console_input(g_hConIn, &ir, 1, &cRecords) == 0)
	{
	    if (did_create_conin)
		read_error_exit();
	    create_conin();
	    continue;
	}

	if (ir.EventType == KEY_EVENT)
	{
	    if (decode_key_event(&ir.Event.KeyEvent, &ch, pch2,
							    pmodifiers, TRUE))
		return ch;
	}
	else if (ir.EventType == FOCUS_EVENT)
	    handle_focus_event(ir);
	else if (ir.EventType == WINDOW_BUFFER_SIZE_EVENT)
	    shell_resized();
	else if (ir.EventType == MOUSE_EVENT)
	{
	    if (decode_mouse_event(&ir.Event.MouseEvent))
		return 0;
	}
    }
}
#endif // !FEAT_GUI_MSWIN


/*
 * mch_inchar(): low-level input function.
 * Get one or more characters from the keyboard or the mouse.
 * If time == 0, do not wait for characters.
 * If time == n, wait a short time for characters.
 * If time == -1, wait forever for characters.
 * Returns the number of characters read into buf.
 */
    int
mch_inchar(
    char_u	*buf UNUSED,
    int		maxlen UNUSED,
    long	time UNUSED,
    int		tb_change_cnt UNUSED)
{
#if !defined(FEAT_GUI_MSWIN) || defined(VIMDLL)

    int		len;
    int		c;
# ifdef VIMDLL
// Extra space for maximum three CSIs. E.g. U+1B6DB -> 0xF0 0x9B 0x9B 0x9B.
#  define TYPEAHEADSPACE    6
# else
#  define TYPEAHEADSPACE    0
# endif
# define TYPEAHEADLEN	    (20 + TYPEAHEADSPACE)
    static char_u   typeahead[TYPEAHEADLEN];	// previously typed bytes.
    static int	    typeaheadlen = 0;

# ifdef VIMDLL
    if (gui.in_use)
	return 0;
# endif

    // First use any typeahead that was kept because "buf" was too small.
    if (typeaheadlen > 0)
	goto theend;

    if (time >= 0)
    {
	if (!WaitForChar(time, FALSE))     // no character available
	    return 0;
    }
    else    // time == -1, wait forever
    {
	mch_set_winsize_now();	// Allow winsize changes from now on

	/*
	 * If there is no character available within 2 seconds (default)
	 * write the autoscript file to disk.  Or cause the CursorHold event
	 * to be triggered.
	 */
	if (!WaitForChar(p_ut, FALSE))
	{
	    if (trigger_cursorhold() && maxlen >= 3)
	    {
		buf[0] = K_SPECIAL;
		buf[1] = KS_EXTRA;
		buf[2] = (int)KE_CURSORHOLD;
		return 3;
	    }
	    before_blocking();
	}
    }

    /*
     * Try to read as many characters as there are, until the buffer is full.
     */

    // we will get at least one key. Get more if they are available.
    g_fCBrkPressed = FALSE;

# ifdef MCH_WRITE_DUMP
    if (fdDump)
	fputc('[', fdDump);
# endif

    // Keep looping until there is something in the typeahead buffer and more
    // to get and still room in the buffer (up to two bytes for a char and
    // three bytes for a modifier).
    while ((typeaheadlen == 0 || WaitForChar(0L, FALSE))
			  && typeaheadlen + 5 + TYPEAHEADSPACE <= TYPEAHEADLEN)
    {
	if (typebuf_changed(tb_change_cnt))
	{
	    // "buf" may be invalid now if a client put something in the
	    // typeahead buffer and "buf" is in the typeahead buffer.
	    typeaheadlen = 0;
	    break;
	}
	if (g_nMouseClick != -1)
	{
# ifdef MCH_WRITE_DUMP
	    if (fdDump)
		fprintf(fdDump, "{%02x @ %d, %d}",
			g_nMouseClick, g_xMouse, g_yMouse);
# endif
	    char_u modifiers = ((char_u *)(&g_nMouseClick))[0];
	    char_u scroll_dir = ((char_u *)(&g_nMouseClick))[1];

	    if (scroll_dir == KE_MOUSEDOWN
		    || scroll_dir == KE_MOUSEUP
		    || scroll_dir == KE_MOUSELEFT
		    || scroll_dir == KE_MOUSERIGHT)
	    {
		if (modifiers > 0)
		{
		    // use K_SPECIAL instead of CSI to make mappings work
		    typeahead[typeaheadlen++] = K_SPECIAL;
		    typeahead[typeaheadlen++] = KS_MODIFIER;
		    typeahead[typeaheadlen++] = modifiers;
		}
		typeahead[typeaheadlen++] = CSI;
		typeahead[typeaheadlen++] = KS_EXTRA;
		typeahead[typeaheadlen++] = scroll_dir;
	    }
	    else
	    {
		typeahead[typeaheadlen++] = ESC + 128;
		typeahead[typeaheadlen++] = 'M';
		typeahead[typeaheadlen++] = g_nMouseClick;
	    }

	    // Pass the pointer coordinates of the mouse event in 2 bytes,
	    // allowing for > 223 columns.  Both for click and scroll events.
	    // This is the same as what is used for the GUI.
	    typeahead[typeaheadlen++] = (char_u)(g_xMouse / 128 + ' ' + 1);
	    typeahead[typeaheadlen++] = (char_u)(g_xMouse % 128 + ' ' + 1);
	    typeahead[typeaheadlen++] = (char_u)(g_yMouse / 128 + ' ' + 1);
	    typeahead[typeaheadlen++] = (char_u)(g_yMouse % 128 + ' ' + 1);

	    g_nMouseClick = -1;
	}
	else
	{
	    WCHAR	ch2 = NUL;
	    int		modifiers = 0;

	    c = tgetch(&modifiers, &ch2);

	    c = simplify_key(c, &modifiers);

	    // Some chars need adjustment when the Ctrl modifier is used.
	    ++no_reduce_keys;
	    c = may_adjust_key_for_ctrl(modifiers, c);
	    --no_reduce_keys;

	    // remove the SHIFT modifier for keys where it's already included,
	    // e.g., '(' and '*'
	    modifiers = may_remove_shift_modifier(modifiers, c);

	    if (typebuf_changed(tb_change_cnt))
	    {
		// "buf" may be invalid now if a client put something in the
		// typeahead buffer and "buf" is in the typeahead buffer.
		typeaheadlen = 0;
		break;
	    }

	    if (c == Ctrl_C && ctrl_c_interrupts)
	    {
# if defined(FEAT_CLIENTSERVER)
		trash_input_buf();
# endif
		got_int = TRUE;
	    }

	    if (g_nMouseClick == -1)
	    {
		int	n = 1;

		if (ch2 == NUL)
		{
		    int	    i, j;
		    char_u  *p;
		    WCHAR   ch[2];

		    ch[0] = c;
		    if (c >= 0xD800 && c <= 0xDBFF)	// High surrogate
		    {
			ch[1] = tgetch(&modifiers, &ch2);
			n++;
		    }
		    p = utf16_to_enc(ch, &n);
		    if (p != NULL)
		    {
			for (i = 0, j = 0; i < n; i++)
			{
			    typeahead[typeaheadlen + j++] = p[i];
# ifdef VIMDLL
			    if (p[i] == CSI)
			    {
				typeahead[typeaheadlen + j++] = KS_EXTRA;
				typeahead[typeaheadlen + j++] = KE_CSI;
			    }
# endif
			}
			n = j;
			vim_free(p);
		    }
		}
		else
		{
		    typeahead[typeaheadlen] = c;
# ifdef VIMDLL
		    if (c == CSI)
		    {
			typeahead[typeaheadlen + 1] = KS_EXTRA;
			typeahead[typeaheadlen + 2] = KE_CSI;
			n = 3;
		    }
# endif
		}
		if (ch2 != NUL)
		{
		    if (c == K_NUL)
		    {
			switch (ch2)
			{
			case (WCHAR)'\324': // SHIFT+Insert
			case (WCHAR)'\325': // CTRL+Insert
			case (WCHAR)'\327': // SHIFT+Delete
			case (WCHAR)'\330': // CTRL+Delete
			    typeahead[typeaheadlen + n] = (char_u)ch2;
			    n++;
			    break;

			default:
			    typeahead[typeaheadlen + n] = 3;
			    typeahead[typeaheadlen + n + 1] = (char_u)ch2;
			    n += 2;
			    break;
			}
		    }
		    else
		    {
			typeahead[typeaheadlen + n] = 3;
			typeahead[typeaheadlen + n + 1] = (char_u)ch2;
			n += 2;
		    }
		}

		// Use the ALT key to set the 8th bit of the character
		// when it's one byte, the 8th bit isn't set yet and not
		// using a double-byte encoding (would become a lead
		// byte).
		if ((modifiers & MOD_MASK_ALT)
			&& n == 1
			&& (typeahead[typeaheadlen] & 0x80) == 0
			&& !enc_dbcs
		   )
		{
		    n = (*mb_char2bytes)(typeahead[typeaheadlen] | 0x80,
						    typeahead + typeaheadlen);
		    modifiers &= ~MOD_MASK_ALT;
		}

		if (modifiers != 0)
		{
		    // Prepend modifiers to the character.
		    mch_memmove(typeahead + typeaheadlen + 3,
						 typeahead + typeaheadlen, n);
		    typeahead[typeaheadlen++] = K_SPECIAL;
		    typeahead[typeaheadlen++] = (char_u)KS_MODIFIER;
		    typeahead[typeaheadlen++] =  modifiers;
		}

		typeaheadlen += n;

# ifdef MCH_WRITE_DUMP
		if (fdDump)
		    fputc(c, fdDump);
# endif
	    }
	}
    }

# ifdef MCH_WRITE_DUMP
    if (fdDump)
    {
	fputs("]\n", fdDump);
	fflush(fdDump);
    }
# endif

theend:
    // Move typeahead to "buf", as much as fits.
    len = 0;
    while (len < maxlen && typeaheadlen > 0)
    {
	buf[len++] = typeahead[0];
	mch_memmove(typeahead, typeahead + 1, --typeaheadlen);
    }
# ifdef FEAT_EVAL
    if (len > 0)
    {
	buf[len] = NUL;
	ch_log(NULL, "raw key input: \"%s\"", buf);
    }
# endif
    return len;

#else // FEAT_GUI_MSWIN
    return 0;
#endif // FEAT_GUI_MSWIN
}

#ifndef PROTO
# ifndef __MINGW32__
#  include <shellapi.h>	// required for FindExecutable()
# endif
#endif

/*
 * Return TRUE if "name" is an executable file, FALSE if not or it doesn't exist.
 * When returning TRUE and "path" is not NULL save the path and set "*path" to
 * the allocated memory.
 * TODO: Should somehow check if it's really executable.
 */
    static int
executable_file(char *name, char_u **path)
{
    int attrs = win32_getattrs((char_u *)name);

    // The file doesn't exist or is a folder.
    if (attrs == -1 || (attrs & FILE_ATTRIBUTE_DIRECTORY))
	return FALSE;
    // Check if the file is an AppExecLink, a special alias used by Windows
    // Store for its apps.
    if (attrs & FILE_ATTRIBUTE_REPARSE_POINT)
    {
	char_u	*res = resolve_appexeclink((char_u *)name);
	if (res == NULL)
	    res = resolve_reparse_point((char_u *)name);
	if (res == NULL)
	    return FALSE;
	// The path is already absolute.
	if (path != NULL)
	    *path = res;
	else
	    vim_free(res);
    }
    else if (path != NULL)
	*path = FullName_save((char_u *)name, FALSE);
    return TRUE;
}

/*
 * If "use_path" is TRUE: Return TRUE if "name" is in $PATH.
 * If "use_path" is FALSE: Return TRUE if "name" exists.
 * If "use_pathext" is TRUE search "name" with extensions in $PATHEXT.
 * When returning TRUE and "path" is not NULL save the path and set "*path" to
 * the allocated memory.
 */
    static int
executable_exists(char *name, char_u **path, int use_path, int use_pathext)
{
    // WinNT and later can use _MAX_PATH wide characters for a pathname, which
    // means that the maximum pathname is _MAX_PATH * 3 bytes when 'enc' is
    // UTF-8.
    char_u	buf[_MAX_PATH * 3];
    size_t	len = STRLEN(name);
    size_t	tmplen;
    char_u	*p, *e, *e2;
    char_u	*pathbuf = NULL;
    char_u	*pathext = NULL;
    char_u	*pathextbuf = NULL;
    char_u	*shname = NULL;
    int		noext = FALSE;
    int		retval = FALSE;

    if (len >= sizeof(buf))	// safety check
	return FALSE;

    // Using the name directly when a Unix-shell like 'shell'.
    shname = gettail(p_sh);
    if (strstr((char *)shname, "sh") != NULL &&
	!(strstr((char *)shname, "powershell") != NULL
				    || strstr((char *)shname, "pwsh") != NULL))
	noext = TRUE;

    if (use_pathext)
    {
	pathext = mch_getenv("PATHEXT");
	if (pathext == NULL)
	    pathext = (char_u *)".com;.exe;.bat;.cmd";

	if (noext == FALSE)
	{
	    /*
	     * Loop over all extensions in $PATHEXT.
	     * Check "name" ends with extension.
	     */
	    p = pathext;
	    while (*p)
	    {
		if (p[0] == ';'
			    || (p[0] == '.' && (p[1] == NUL || p[1] == ';')))
		{
		    // Skip empty or single ".".
		    ++p;
		    continue;
		}
		e = vim_strchr(p, ';');
		if (e == NULL)
		    e = p + STRLEN(p);
		tmplen = e - p;

		if (_strnicoll(name + len - tmplen, (char *)p, tmplen) == 0)
		{
		    noext = TRUE;
		    break;
		}

		p = e;
	    }
	}
    }

    // Prepend single "." to pathext, it means no extension added.
    if (pathext == NULL)
	pathext = (char_u *)".";
    else if (noext == TRUE)
    {
	if (pathextbuf == NULL)
	    pathextbuf = alloc(STRLEN(pathext) + 3);
	if (pathextbuf == NULL)
	{
	    retval = FALSE;
	    goto theend;
	}
	STRCPY(pathextbuf, ".;");
	STRCAT(pathextbuf, pathext);
	pathext = pathextbuf;
    }

    // Use $PATH when "use_path" is TRUE and "name" is basename.
    if (use_path && gettail((char_u *)name) == (char_u *)name)
    {
	p = mch_getenv("PATH");
	if (p != NULL)
	{
	    pathbuf = alloc(STRLEN(p) + 3);
	    if (pathbuf == NULL)
	    {
		retval = FALSE;
		goto theend;
	    }

	    if (mch_getenv("NoDefaultCurrentDirectoryInExePath") == NULL)
		STRCPY(pathbuf, ".;");
	    else
		*pathbuf = NUL;
	    STRCAT(pathbuf, p);
	}
    }

    /*
     * Walk through all entries in $PATH to check if "name" exists there and
     * is an executable file.
     */
    p = (pathbuf != NULL) ? pathbuf : (char_u *)".";
    while (*p)
    {
	if (*p == ';') // Skip empty entry
	{
	    ++p;
	    continue;
	}
	e = vim_strchr(p, ';');
	if (e == NULL)
	    e = p + STRLEN(p);

	if (e - p + len + 2 > sizeof(buf))
	{
	    retval = FALSE;
	    goto theend;
	}
	// A single "." that means current dir.
	if (e - p == 1 && *p == '.')
	    STRCPY(buf, name);
	else
	{
	    vim_strncpy(buf, p, e - p);
	    add_pathsep(buf);
	    STRCAT(buf, name);
	}
	tmplen = STRLEN(buf);

	/*
	 * Loop over all extensions in $PATHEXT.
	 * Check "name" with extension added.
	 */
	p = pathext;
	while (*p)
	{
	    if (*p == ';')
	    {
		// Skip empty entry
		++p;
		continue;
	    }
	    e2 = vim_strchr(p, (int)';');
	    if (e2 == NULL)
		e2 = p + STRLEN(p);

	    if (!(p[0] == '.' && (p[1] == NUL || p[1] == ';')))
	    {
		// Not a single "." that means no extension is added.
		if (e2 - p + tmplen + 1 > sizeof(buf))
		{
		    retval = FALSE;
		    goto theend;
		}
		vim_strncpy(buf + tmplen, p, e2 - p);
	    }
	    if (executable_file((char *)buf, path))
	    {
		retval = TRUE;
		goto theend;
	    }

	    p = e2;
	}

	p = e;
    }

theend:
    free(pathextbuf);
    free(pathbuf);
    return retval;
}

#if (defined(__MINGW32__) && __MSVCRT_VERSION__ >= 0x800) || \
	(defined(_MSC_VER) && _MSC_VER >= 1400)
/*
 * Bad parameter handler.
 *
 * Certain MS CRT functions will intentionally crash when passed invalid
 * parameters to highlight possible security holes.  Setting this function as
 * the bad parameter handler will prevent the crash.
 *
 * In debug builds the parameters contain CRT information that might help track
 * down the source of a problem, but in non-debug builds the arguments are all
 * NULL/0.  Debug builds will also produce assert dialogs from the CRT, it is
 * worth allowing these to make debugging of issues easier.
 */
    static void
bad_param_handler(const wchar_t *expression UNUSED,
    const wchar_t *function UNUSED,
    const wchar_t *file UNUSED,
    unsigned int line UNUSED,
    uintptr_t pReserved UNUSED)
{
}

# define SET_INVALID_PARAM_HANDLER \
	((void)_set_invalid_parameter_handler(bad_param_handler))
#else
# define SET_INVALID_PARAM_HANDLER
#endif

#ifdef FEAT_GUI_MSWIN

/*
 * GUI version of mch_init().
 */
    static void
mch_init_g(void)
{
# ifndef __MINGW32__
    extern int _fmode;
# endif

    // Silently handle invalid parameters to CRT functions
    SET_INVALID_PARAM_HANDLER;

    // Let critical errors result in a failure, not in a dialog box.  Required
    // for the timestamp test to work on removed floppies.
    SetErrorMode(SEM_FAILCRITICALERRORS);

    _fmode = O_BINARY;		// we do our own CR-LF translation

    // Specify window size.  Is there a place to get the default from?
    Rows = 25;
    Columns = 80;

    // Look for 'vimrun'
    {
	char_u vimrun_location[_MAX_PATH + 4];

	// First try in same directory as gvim.exe
	STRCPY(vimrun_location, exe_name);
	STRCPY(gettail(vimrun_location), "vimrun.exe");
	if (mch_getperm(vimrun_location) >= 0)
	{
	    if (*skiptowhite(vimrun_location) != NUL)
	    {
		// Enclose path with white space in double quotes.
		mch_memmove(vimrun_location + 1, vimrun_location,
						 STRLEN(vimrun_location) + 1);
		*vimrun_location = '"';
		STRCPY(gettail(vimrun_location), "vimrun\" ");
	    }
	    else
		STRCPY(gettail(vimrun_location), "vimrun ");

	    vimrun_path = (char *)vim_strsave(vimrun_location);
	    s_dont_use_vimrun = FALSE;
	}
	else if (executable_exists("vimrun.exe", NULL, TRUE, FALSE))
	    s_dont_use_vimrun = FALSE;

	// Don't give the warning for a missing vimrun.exe right now, but only
	// when vimrun was supposed to be used.  Don't bother people that do
	// not need vimrun.exe.
	if (s_dont_use_vimrun)
	    need_vimrun_warning = TRUE;
    }

    /*
     * If "finstr.exe" doesn't exist, use "grep -n" for 'grepprg'.
     * Otherwise the default "findstr /n" is used.
     */
    if (!executable_exists("findstr.exe", NULL, TRUE, FALSE))
	set_option_value_give_err((char_u *)"grepprg",
						    0, (char_u *)"grep -n", 0);

# ifdef FEAT_CLIPBOARD
    win_clip_init();
# endif

    vtp_flag_init();
}


#endif // FEAT_GUI_MSWIN

#if !defined(FEAT_GUI_MSWIN) || defined(VIMDLL)

# define SRWIDTH(sr) ((sr).Right - (sr).Left + 1)
# define SRHEIGHT(sr) ((sr).Bottom - (sr).Top + 1)

/*
 * ClearConsoleBuffer()
 * Description:
 *  Clears the entire contents of the console screen buffer, using the
 *  specified attribute.
 * Returns:
 *  TRUE on success
 */
    static BOOL
ClearConsoleBuffer(WORD wAttribute)
{
    CONSOLE_SCREEN_BUFFER_INFO csbi;
    COORD coord;
    DWORD NumCells, dummy;

    if (!GetConsoleScreenBufferInfo(g_hConOut, &csbi))
	return FALSE;

    NumCells = csbi.dwSize.X * csbi.dwSize.Y;
    coord.X = 0;
    coord.Y = 0;
    if (!FillConsoleOutputCharacter(g_hConOut, ' ', NumCells,
	    coord, &dummy))
	return FALSE;
    if (!FillConsoleOutputAttribute(g_hConOut, wAttribute, NumCells,
	    coord, &dummy))
	return FALSE;

    return TRUE;
}

/*
 * FitConsoleWindow()
 * Description:
 *  Checks if the console window will fit within given buffer dimensions.
 *  Also, if requested, will shrink the window to fit.
 * Returns:
 *  TRUE on success
 */
    static BOOL
FitConsoleWindow(
    COORD dwBufferSize,
    BOOL WantAdjust)
{
    CONSOLE_SCREEN_BUFFER_INFO csbi;
    COORD dwWindowSize;
    BOOL NeedAdjust = FALSE;

    if (!GetConsoleScreenBufferInfo(g_hConOut, &csbi))
	return FALSE;

    /*
     * A buffer resize will fail if the current console window does
     * not lie completely within that buffer.  To avoid this, we might
     * have to move and possibly shrink the window.
     */
    if (csbi.srWindow.Right >= dwBufferSize.X)
    {
	dwWindowSize.X = SRWIDTH(csbi.srWindow);
	if (dwWindowSize.X > dwBufferSize.X)
	    dwWindowSize.X = dwBufferSize.X;
	csbi.srWindow.Right = dwBufferSize.X - 1;
	csbi.srWindow.Left = dwBufferSize.X - dwWindowSize.X;
	NeedAdjust = TRUE;
    }
    if (csbi.srWindow.Bottom >= dwBufferSize.Y)
    {
	dwWindowSize.Y = SRHEIGHT(csbi.srWindow);
	if (dwWindowSize.Y > dwBufferSize.Y)
	    dwWindowSize.Y = dwBufferSize.Y;
	csbi.srWindow.Bottom = dwBufferSize.Y - 1;
	csbi.srWindow.Top = dwBufferSize.Y - dwWindowSize.Y;
	NeedAdjust = TRUE;
    }
    if (NeedAdjust && WantAdjust)
    {
	if (!SetConsoleWindowInfo(g_hConOut, TRUE, &csbi.srWindow))
	    return FALSE;
    }
    return TRUE;
}

typedef struct ConsoleBufferStruct
{
    BOOL			IsValid;
    CONSOLE_SCREEN_BUFFER_INFO	Info;
    PCHAR_INFO			Buffer;
    COORD			BufferSize;
    PSMALL_RECT			Regions;
    int				NumRegions;
} ConsoleBuffer;

/*
 * SaveConsoleBuffer()
 * Description:
 *  Saves important information about the console buffer, including the
 *  actual buffer contents.  The saved information is suitable for later
 *  restoration by RestoreConsoleBuffer().
 * Returns:
 *  TRUE if all information was saved; FALSE otherwise
 *  If FALSE, still sets cb->IsValid if buffer characteristics were saved.
 */
    static BOOL
SaveConsoleBuffer(
    ConsoleBuffer *cb)
{
    DWORD NumCells;
    COORD BufferCoord;
    SMALL_RECT ReadRegion;
    WORD Y, Y_incr;
    int i, numregions;

    if (cb == NULL)
	return FALSE;

    if (!GetConsoleScreenBufferInfo(g_hConOut, &cb->Info))
    {
	cb->IsValid = FALSE;
	return FALSE;
    }
    cb->IsValid = TRUE;

    // VTP uses alternate screen buffer.
    // No need to save buffer contents for restoration.
    if (use_alternate_screen_buffer)
	return TRUE;

    /*
     * Allocate a buffer large enough to hold the entire console screen
     * buffer.  If this ConsoleBuffer structure has already been initialized
     * with a buffer of the correct size, then just use that one.
     */
    if (!cb->IsValid || cb->Buffer == NULL ||
	    cb->BufferSize.X != cb->Info.dwSize.X ||
	    cb->BufferSize.Y != cb->Info.dwSize.Y)
    {
	cb->BufferSize.X = cb->Info.dwSize.X;
	cb->BufferSize.Y = cb->Info.dwSize.Y;
	NumCells = cb->BufferSize.X * cb->BufferSize.Y;
	vim_free(cb->Buffer);
	cb->Buffer = ALLOC_MULT(CHAR_INFO, NumCells);
	if (cb->Buffer == NULL)
	    return FALSE;
    }

    /*
     * We will now copy the console screen buffer into our buffer.
     * ReadConsoleOutput() seems to be limited as far as how much you
     * can read at a time.  Empirically, this number seems to be about
     * 12000 cells (rows * columns).  Start at position (0, 0) and copy
     * in chunks until it is all copied.  The chunks will all have the
     * same horizontal characteristics, so initialize them now.  The
     * height of each chunk will be (12000 / width).
     */
    BufferCoord.X = 0;
    ReadRegion.Left = 0;
    ReadRegion.Right = cb->Info.dwSize.X - 1;
    Y_incr = 12000 / cb->Info.dwSize.X;

    numregions = (cb->Info.dwSize.Y + Y_incr - 1) / Y_incr;
    if (cb->Regions == NULL || numregions != cb->NumRegions)
    {
	cb->NumRegions = numregions;
	vim_free(cb->Regions);
	cb->Regions = ALLOC_MULT(SMALL_RECT, cb->NumRegions);
	if (cb->Regions == NULL)
	{
	    VIM_CLEAR(cb->Buffer);
	    return FALSE;
	}
    }

    for (i = 0, Y = 0; i < cb->NumRegions; i++, Y += Y_incr)
    {
	/*
	 * Read into position (0, Y) in our buffer.
	 */
	BufferCoord.Y = Y;
	/*
	 * Read the region whose top left corner is (0, Y) and whose bottom
	 * right corner is (width - 1, Y + Y_incr - 1).  This should define
	 * a region of size width by Y_incr.  Don't worry if this region is
	 * too large for the remaining buffer; it will be cropped.
	 */
	ReadRegion.Top = Y;
	ReadRegion.Bottom = Y + Y_incr - 1;
	if (!ReadConsoleOutputW(g_hConOut,	// output handle
		cb->Buffer,			// our buffer
		cb->BufferSize,			// dimensions of our buffer
		BufferCoord,			// offset in our buffer
		&ReadRegion))			// region to save
	{
	    VIM_CLEAR(cb->Buffer);
	    VIM_CLEAR(cb->Regions);
	    return FALSE;
	}
	cb->Regions[i] = ReadRegion;
    }

    return TRUE;
}

/*
 * RestoreConsoleBuffer()
 * Description:
 *  Restores important information about the console buffer, including the
 *  actual buffer contents, if desired.  The information to restore is in
 *  the same format used by SaveConsoleBuffer().
 * Returns:
 *  TRUE on success
 */
    static BOOL
RestoreConsoleBuffer(
    ConsoleBuffer   *cb,
    BOOL	    RestoreScreen)
{
    COORD BufferCoord;
    SMALL_RECT WriteRegion;
    int i;

    // VTP uses alternate screen buffer.
    // No need to restore buffer contents.
    if (use_alternate_screen_buffer)
	return TRUE;

    if (cb == NULL || !cb->IsValid)
	return FALSE;

    /*
     * Before restoring the buffer contents, clear the current buffer, and
     * restore the cursor position and window information.  Doing this now
     * prevents old buffer contents from "flashing" onto the screen.
     */
    if (RestoreScreen)
	ClearConsoleBuffer(cb->Info.wAttributes);

    FitConsoleWindow(cb->Info.dwSize, TRUE);
    if (!SetConsoleScreenBufferSize(g_hConOut, cb->Info.dwSize))
	return FALSE;
    if (!SetConsoleTextAttribute(g_hConOut, cb->Info.wAttributes))
	return FALSE;

    if (!RestoreScreen)
    {
	/*
	 * No need to restore the screen buffer contents, so we're done.
	 */
	return TRUE;
    }

    if (!SetConsoleCursorPosition(g_hConOut, cb->Info.dwCursorPosition))
	return FALSE;
    if (!SetConsoleWindowInfo(g_hConOut, TRUE, &cb->Info.srWindow))
	return FALSE;

    /*
     * Restore the screen buffer contents.
     */
    if (cb->Buffer != NULL)
    {
	for (i = 0; i < cb->NumRegions; i++)
	{
	    BufferCoord.X = cb->Regions[i].Left;
	    BufferCoord.Y = cb->Regions[i].Top;
	    WriteRegion = cb->Regions[i];
	    if (!WriteConsoleOutputW(g_hConOut,	// output handle
			cb->Buffer,		// our buffer
			cb->BufferSize,		// dimensions of our buffer
			BufferCoord,		// offset in our buffer
			&WriteRegion))		// region to restore
		return FALSE;
	}
    }

    return TRUE;
}

# define FEAT_RESTORE_ORIG_SCREEN
# ifdef FEAT_RESTORE_ORIG_SCREEN
static ConsoleBuffer g_cbOrig = { 0 };
# endif
static ConsoleBuffer g_cbNonTermcap = { 0 };
static ConsoleBuffer g_cbTermcap = { 0 };

char g_szOrigTitle[256] = { 0 };
HWND g_hWnd = NULL;	// also used in os_mswin.c
static HICON g_hOrigIconSmall = NULL;
static HICON g_hOrigIcon = NULL;
static HICON g_hVimIcon = NULL;
static BOOL g_fCanChangeIcon = FALSE;

/*
 * GetConsoleIcon()
 * Description:
 *  Attempts to retrieve the small icon and/or the big icon currently in
 *  use by a given window.
 * Returns:
 *  TRUE on success
 */
    static BOOL
GetConsoleIcon(
    HWND	hWnd,
    HICON	*phIconSmall,
    HICON	*phIcon)
{
    if (hWnd == NULL)
	return FALSE;

    if (phIconSmall != NULL)
	*phIconSmall = (HICON)SendMessage(hWnd, WM_GETICON,
					       (WPARAM)ICON_SMALL, (LPARAM)0);
    if (phIcon != NULL)
	*phIcon = (HICON)SendMessage(hWnd, WM_GETICON,
						 (WPARAM)ICON_BIG, (LPARAM)0);
    return TRUE;
}

/*
 * SetConsoleIcon()
 * Description:
 *  Attempts to change the small icon and/or the big icon currently in
 *  use by a given window.
 * Returns:
 *  TRUE on success
 */
    static BOOL
SetConsoleIcon(
    HWND    hWnd,
    HICON   hIconSmall,
    HICON   hIcon)
{
    if (hWnd == NULL)
	return FALSE;

    if (hIconSmall != NULL)
	SendMessage(hWnd, WM_SETICON,
			    (WPARAM)ICON_SMALL, (LPARAM)hIconSmall);
    if (hIcon != NULL)
	SendMessage(hWnd, WM_SETICON,
			    (WPARAM)ICON_BIG, (LPARAM) hIcon);
    return TRUE;
}

/*
 * SaveConsoleTitleAndIcon()
 * Description:
 *  Saves the current console window title in g_szOrigTitle, for later
 *  restoration.  Also, attempts to obtain a handle to the console window,
 *  and use it to save the small and big icons currently in use by the
 *  console window.  This is not always possible on some versions of Windows;
 *  nor is it possible when running Vim remotely using Telnet (since the
 *  console window the user sees is owned by a remote process).
 */
    static void
SaveConsoleTitleAndIcon(void)
{
    // Save the original title.
    if (!GetConsoleTitle(g_szOrigTitle, sizeof(g_szOrigTitle)))
	return;

    /*
     * Obtain a handle to the console window using GetConsoleWindow() from
     * KERNEL32.DLL; we need to handle in order to change the window icon.
     * This function only exists on NT-based Windows, starting with Windows
     * 2000.  On older operating systems, we can't change the window icon
     * anyway.
     */
    g_hWnd = GetConsoleWindow();
    if (g_hWnd == NULL)
	return;

    // Save the original console window icon.
    GetConsoleIcon(g_hWnd, &g_hOrigIconSmall, &g_hOrigIcon);
    if (g_hOrigIconSmall == NULL || g_hOrigIcon == NULL)
	return;

    // Extract the first icon contained in the Vim executable.
    if (
# ifdef FEAT_LIBCALL
	    mch_icon_load((HANDLE *)&g_hVimIcon) == FAIL ||
# endif
	    g_hVimIcon == NULL)
	g_hVimIcon = ExtractIcon(NULL, (LPCSTR)exe_name, 0);
    if (g_hVimIcon != NULL)
	g_fCanChangeIcon = TRUE;
}

static int g_fWindInitCalled = FALSE;
static int g_fTermcapMode = FALSE;
static CONSOLE_CURSOR_INFO g_cci;

/*
 * non-GUI version of mch_init().
 */
    static void
mch_init_c(void)
{
# ifndef FEAT_RESTORE_ORIG_SCREEN
    CONSOLE_SCREEN_BUFFER_INFO csbi;
# endif
# ifndef __MINGW32__
    extern int _fmode;
# endif

    // Silently handle invalid parameters to CRT functions
    SET_INVALID_PARAM_HANDLER;

    // Let critical errors result in a failure, not in a dialog box.  Required
    // for the timestamp test to work on removed floppies.
    SetErrorMode(SEM_FAILCRITICALERRORS);

    _fmode = O_BINARY;		// we do our own CR-LF translation
    out_flush();

    // Obtain handles for the standard Console I/O devices
    if (read_cmd_fd == 0)
	g_hConIn =  GetStdHandle(STD_INPUT_HANDLE);
    else
	create_conin();
    g_hConOut = GetStdHandle(STD_OUTPUT_HANDLE);

    wt_init();
    vtp_flag_init();
# ifdef FEAT_RESTORE_ORIG_SCREEN
    // Save the initial console buffer for later restoration
    SaveConsoleBuffer(&g_cbOrig);
    g_attrCurrent = g_attrDefault = g_cbOrig.Info.wAttributes;
# else
    // Get current text attributes
    GetConsoleScreenBufferInfo(g_hConOut, &csbi);
    g_attrCurrent = g_attrDefault = csbi.wAttributes;
# endif
    if (cterm_normal_fg_color == 0)
	cterm_normal_fg_color = (g_attrCurrent & 0xf) + 1;
    if (cterm_normal_bg_color == 0)
	cterm_normal_bg_color = ((g_attrCurrent >> 4) & 0xf) + 1;

    // Fg and Bg color index number at startup
    g_color_index_fg = g_attrDefault & 0xf;
    g_color_index_bg = (g_attrDefault >> 4) & 0xf;

    // set termcap codes to current text attributes
    update_tcap(g_attrCurrent);

    GetConsoleCursorInfo(g_hConOut, &g_cci);
    GetConsoleMode(g_hConIn,  &g_cmodein);
    GetConsoleMode(g_hConOut, &g_cmodeout);

    SaveConsoleTitleAndIcon();
    /*
     * Set both the small and big icons of the console window to Vim's icon.
     * Note that Vim presently only has one size of icon (32x32), but it
     * automatically gets scaled down to 16x16 when setting the small icon.
     */
    if (g_fCanChangeIcon)
	SetConsoleIcon(g_hWnd, g_hVimIcon, g_hVimIcon);

    ui_get_shellsize();

    vtp_init();
    // Switch to a new alternate screen buffer.
    if (use_alternate_screen_buffer)
	vtp_printf("\033[?1049h");

# ifdef MCH_WRITE_DUMP
    fdDump = fopen("dump", "wt");

    if (fdDump)
    {
	time_t t;

	time(&t);
	fputs(ctime(&t), fdDump);
	fflush(fdDump);
    }
# endif

    g_fWindInitCalled = TRUE;

    g_fMouseAvail = GetSystemMetrics(SM_MOUSEPRESENT);

# ifdef FEAT_CLIPBOARD
    win_clip_init();
# endif
}

/*
 * non-GUI version of mch_exit().
 * Shut down and exit with status `r'
 * Careful: mch_exit() may be called before mch_init()!
 */
    static void
mch_exit_c(int r)
{
    exiting = TRUE;

    vtp_exit();

    stoptermcap();
    if (g_fWindInitCalled)
	settmode(TMODE_COOK);

    ml_close_all(TRUE);		// remove all memfiles

    if (g_fWindInitCalled)
    {
	mch_restore_title(SAVE_RESTORE_BOTH);
	/*
	 * Restore both the small and big icons of the console window to
	 * what they were at startup.  Don't do this when the window is
	 * closed, Vim would hang here.
	 */
	if (g_fCanChangeIcon && !g_fForceExit)
	    SetConsoleIcon(g_hWnd, g_hOrigIconSmall, g_hOrigIcon);

# ifdef MCH_WRITE_DUMP
	if (fdDump)
	{
	    time_t t;

	    time(&t);
	    fputs(ctime(&t), fdDump);
	    fclose(fdDump);
	}
	fdDump = NULL;
# endif
    }

    SetConsoleCursorInfo(g_hConOut, &g_cci);
    SetConsoleMode(g_hConIn,  g_cmodein | ENABLE_EXTENDED_FLAGS);
    SetConsoleMode(g_hConOut, g_cmodeout);

# ifdef DYNAMIC_GETTEXT
    dyn_libintl_end();
# endif

    exit(r);
}
#endif // !FEAT_GUI_MSWIN

    void
mch_init(void)
{
#ifdef VIMDLL
    if (gui.starting)
	mch_init_g();
    else
	mch_init_c();
#elif defined(FEAT_GUI_MSWIN)
    mch_init_g();
#else
    mch_init_c();
#endif
}

    void
mch_exit(int r)
{
#ifdef FEAT_NETBEANS_INTG
    netbeans_send_disconnect();
#endif

#ifdef VIMDLL
    if (gui.in_use || gui.starting)
	mch_exit_g(r);
    else
	mch_exit_c(r);
#elif defined(FEAT_GUI_MSWIN)
    mch_exit_g(r);
#else
    mch_exit_c(r);
#endif
}

/*
 * Do we have an interactive window?
 */
    int
mch_check_win(
    int argc UNUSED,
    char **argv UNUSED)
{
    mch_get_exe_name();

#if defined(FEAT_GUI_MSWIN) && !defined(VIMDLL)
    return OK;	    // GUI always has a tty
#else
# ifdef VIMDLL
    if (gui.in_use)
	return OK;
# endif
    if (isatty(1))
	return OK;
    return FAIL;
#endif
}

/*
 * Set the case of the file name, if it already exists.
 * When "len" is > 0, also expand short to long filenames.
 */
    void
fname_case(
    char_u	*name,
    int		len)
{
    int	    flen;
    WCHAR   *p;
    WCHAR   buf[_MAX_PATH + 1];

    flen = (int)STRLEN(name);
    if (flen == 0)
	return;

    slash_adjust(name);

    p = enc_to_utf16(name, NULL);
    if (p == NULL)
	return;

    if (GetLongPathNameW(p, buf, _MAX_PATH))
    {
	char_u	*q = utf16_to_enc(buf, NULL);

	if (q != NULL)
	{
	    if (len > 0 || flen >= (int)STRLEN(q))
		vim_strncpy(name, q, (len > 0) ? len - 1 : flen);
	    vim_free(q);
	}
    }
    vim_free(p);
}


/*
 * Insert user name in s[len].
 */
    int
mch_get_user_name(
    char_u  *s,
    int	    len)
{
    WCHAR wszUserName[256 + 1];	// UNLEN is 256
    DWORD wcch = ARRAY_LENGTH(wszUserName);

    if (GetUserNameW(wszUserName, &wcch))
    {
	char_u  *p = utf16_to_enc(wszUserName, NULL);

	if (p != NULL)
	{
	    vim_strncpy(s, p, len - 1);
	    vim_free(p);
	    return OK;
	}
    }
    s[0] = NUL;
    return FAIL;
}


/*
 * Insert host name in s[len].
 */
    void
mch_get_host_name(
    char_u	*s,
    int		len)
{
    WCHAR wszHostName[256 + 1];
    DWORD wcch = ARRAY_LENGTH(wszHostName);

    if (!GetComputerNameW(wszHostName, &wcch))
	return;

    char_u  *p = utf16_to_enc(wszHostName, NULL);
    if (p == NULL)
	return;

    vim_strncpy(s, p, len - 1);
    vim_free(p);
}


/*
 * return process ID
 */
    long
mch_get_pid(void)
{
    return (long)GetCurrentProcessId();
}

/*
 * return TRUE if process "pid" is still running
 */
    int
mch_process_running(long pid)
{
    HANDLE  hProcess = OpenProcess(PROCESS_QUERY_INFORMATION, 0, (DWORD)pid);
    DWORD   status = 0;
    int	    ret = FALSE;

    if (hProcess == NULL)
	return FALSE;  // might not have access
    if (GetExitCodeProcess(hProcess, &status) )
	ret = status == STILL_ACTIVE;
    CloseHandle(hProcess);
    return ret;
}

/*
 * Get name of current directory into buffer 'buf' of length 'len' bytes.
 * Return OK for success, FAIL for failure.
 */
    int
mch_dirname(
    char_u	*buf,
    int		len)
{
    WCHAR   wbuf[_MAX_PATH + 1];

    /*
     * Originally this was:
     *    return (getcwd(buf, len) != NULL ? OK : FAIL);
     * But the Win32s known bug list says that getcwd() doesn't work
     * so use the Win32 system call instead. <Negri>
     */
    if (GetCurrentDirectoryW(_MAX_PATH, wbuf) == 0)
	return FAIL;

    WCHAR   wcbuf[_MAX_PATH + 1];
    char_u  *p = NULL;

    if (GetLongPathNameW(wbuf, wcbuf, _MAX_PATH) != 0)
    {
	p = utf16_to_enc(wcbuf, NULL);
	if (STRLEN(p) >= (size_t)len)
	{
	    // long path name is too long, fall back to short one
	    VIM_CLEAR(p);
	}
    }
    if (p == NULL)
	p = utf16_to_enc(wbuf, NULL);

    if (p == NULL)
	return FAIL;

    vim_strncpy(buf, p, len - 1);
    vim_free(p);
    return OK;
}

/*
 * Get file permissions for "name".
 * Return mode_t or -1 for error.
 */
    long
mch_getperm(char_u *name)
{
    stat_T	st;
    int		n;

    n = mch_stat((char *)name, &st);
    return n == 0 ? (long)(unsigned short)st.st_mode : -1L;
}


/*
 * Set file permission for "name" to "perm".
 *
 * Return FAIL for failure, OK otherwise.
 */
    int
mch_setperm(char_u *name, long perm)
{
    long	n;
    WCHAR	*p;

    p = enc_to_utf16(name, NULL);
    if (p == NULL)
	return FAIL;

    n = _wchmod(p, perm);
    vim_free(p);
    if (n == -1)
	return FAIL;

    win32_set_archive(name);

    return OK;
}

/*
 * Set hidden flag for "name".
 */
    void
mch_hide(char_u *name)
{
    int attrs = win32_getattrs(name);
    if (attrs == -1)
	return;

    attrs |= FILE_ATTRIBUTE_HIDDEN;
    win32_setattrs(name, attrs);
}

/*
 * Return TRUE if file "name" exists and is hidden.
 */
    int
mch_ishidden(char_u *name)
{
    int f = win32_getattrs(name);

    if (f == -1)
	return FALSE;		    // file does not exist at all

    return (f & FILE_ATTRIBUTE_HIDDEN) != 0;
}

/*
 * return TRUE if "name" is a directory
 * return FALSE if "name" is not a directory or upon error
 */
    int
mch_isdir(char_u *name)
{
    int f = win32_getattrs(name);

    if (f == -1)
	return FALSE;		    // file does not exist at all

    return (f & FILE_ATTRIBUTE_DIRECTORY) != 0;
}

/*
 * return TRUE if "name" is a directory, NOT a symlink to a directory
 * return FALSE if "name" is not a directory
 * return FALSE for error
 */
    int
mch_isrealdir(char_u *name)
{
    return mch_isdir(name) && !mch_is_symbolic_link(name);
}

/*
 * Create directory "name".
 * Return 0 on success, -1 on error.
 */
    int
mch_mkdir(char_u *name)
{
    WCHAR   *p;
    int	    retval;

    p = enc_to_utf16(name, NULL);
    if (p == NULL)
	return -1;
    retval = _wmkdir(p);
    vim_free(p);
    return retval;
}

/*
 * Delete directory "name".
 * Return 0 on success, -1 on error.
 */
    int
mch_rmdir(char_u *name)
{
    WCHAR   *p;
    int	    retval;

    p = enc_to_utf16(name, NULL);
    if (p == NULL)
	return -1;
    retval = _wrmdir(p);
    vim_free(p);
    return retval;
}

/*
 * Return TRUE if file "fname" has more than one link.
 */
    int
mch_is_hard_link(char_u *fname)
{
    BY_HANDLE_FILE_INFORMATION info;

    return win32_fileinfo(fname, &info) == FILEINFO_OK
						   && info.nNumberOfLinks > 1;
}

/*
 * Return TRUE if "name" is a symbolic link (or a junction).
 */
    int
mch_is_symbolic_link(char_u *name)
{
    HANDLE		hFind;
    int			res = FALSE;
    DWORD		fileFlags = 0, reparseTag = 0;
    WCHAR		*wn;
    WIN32_FIND_DATAW	findDataW;

    wn = enc_to_utf16(name, NULL);
    if (wn == NULL)
	return FALSE;

    hFind = FindFirstFileW(wn, &findDataW);
    vim_free(wn);
    if (hFind != INVALID_HANDLE_VALUE)
    {
	fileFlags = findDataW.dwFileAttributes;
	reparseTag = findDataW.dwReserved0;
	FindClose(hFind);
    }

    if ((fileFlags & FILE_ATTRIBUTE_REPARSE_POINT)
	    && (reparseTag == IO_REPARSE_TAG_SYMLINK
		|| reparseTag == IO_REPARSE_TAG_MOUNT_POINT))
	res = TRUE;

    return res;
}

/*
 * Return TRUE if file "fname" has more than one link or if it is a symbolic
 * link.
 */
    int
mch_is_linked(char_u *fname)
{
    if (mch_is_hard_link(fname) || mch_is_symbolic_link(fname))
	return TRUE;
    return FALSE;
}

/*
 * Get the by-handle-file-information for "fname".
 * Returns FILEINFO_OK when OK.
 * Returns FILEINFO_ENC_FAIL when enc_to_utf16() failed.
 * Returns FILEINFO_READ_FAIL when CreateFile() failed.
 * Returns FILEINFO_INFO_FAIL when GetFileInformationByHandle() failed.
 */
    int
win32_fileinfo(char_u *fname, BY_HANDLE_FILE_INFORMATION *info)
{
    HANDLE	hFile;
    int		res = FILEINFO_READ_FAIL;
    WCHAR	*wn;

    wn = enc_to_utf16(fname, NULL);
    if (wn == NULL)
	return FILEINFO_ENC_FAIL;

    hFile = CreateFileW(wn,	// file name
	    GENERIC_READ,	// access mode
	    FILE_SHARE_READ | FILE_SHARE_WRITE,	// share mode
	    NULL,		// security descriptor
	    OPEN_EXISTING,	// creation disposition
	    FILE_FLAG_BACKUP_SEMANTICS,	// file attributes
	    NULL);		// handle to template file
    vim_free(wn);

    if (hFile == INVALID_HANDLE_VALUE)
	return FILEINFO_READ_FAIL;

    if (GetFileInformationByHandle(hFile, info) != 0)
	res = FILEINFO_OK;
    else
	res = FILEINFO_INFO_FAIL;
    CloseHandle(hFile);

    return res;
}

/*
 * get file attributes for `name'
 * -1 : error
 * else FILE_ATTRIBUTE_* defined in winnt.h
 */
    static int
win32_getattrs(char_u *name)
{
    int		attr;
    WCHAR	*p;

    p = enc_to_utf16(name, NULL);
    if (p == NULL)
	return INVALID_FILE_ATTRIBUTES;

    attr = GetFileAttributesW(p);
    vim_free(p);

    return attr;
}

/*
 * set file attributes for `name' to `attrs'
 *
 * return -1 for failure, 0 otherwise
 */
    static int
win32_setattrs(char_u *name, int attrs)
{
    int	    res;
    WCHAR   *p;

    p = enc_to_utf16(name, NULL);
    if (p == NULL)
	return -1;

    res = SetFileAttributesW(p, attrs);
    vim_free(p);

    return res ? 0 : -1;
}

/*
 * Set archive flag for "name".
 */
    static int
win32_set_archive(char_u *name)
{
    int attrs = win32_getattrs(name);
    if (attrs == -1)
	return -1;

    attrs |= FILE_ATTRIBUTE_ARCHIVE;
    return win32_setattrs(name, attrs);
}

/*
 * Return TRUE if file or directory "name" is writable (not readonly).
 * Strange semantics of Win32: a readonly directory is writable, but you can't
 * delete a file.  Let's say this means it is writable.
 */
    int
mch_writable(char_u *name)
{
    int attrs = win32_getattrs(name);

    return (attrs != -1 && (!(attrs & FILE_ATTRIBUTE_READONLY)
			  || (attrs & FILE_ATTRIBUTE_DIRECTORY)));
}

/*
 * Return TRUE if "name" can be executed, FALSE if not.
 * If "use_path" is FALSE only check if "name" is executable.
 * When returning TRUE and "path" is not NULL save the path and set "*path" to
 * the allocated memory.
 */
    int
mch_can_exe(char_u *name, char_u **path, int use_path UNUSED)
{
    return executable_exists((char *)name, path, TRUE, TRUE);
}

/*
 * Check what "name" is:
 * NODE_NORMAL: file or directory (or doesn't exist)
 * NODE_WRITABLE: writable device, socket, fifo, etc.
 * NODE_OTHER: non-writable things
 */
    int
mch_nodetype(char_u *name)
{
    HANDLE	hFile;
    int		type;
    WCHAR	*wn;

    // We can't open a file with a name "\\.\con" or "\\.\prn" and trying to
    // read from it later will cause Vim to hang.  Thus return NODE_WRITABLE
    // here.
    if (STRNCMP(name, "\\\\.\\", 4) == 0)
	return NODE_WRITABLE;

    wn = enc_to_utf16(name, NULL);
    if (wn == NULL)
	return NODE_NORMAL;

    hFile = CreateFileW(wn,	    // file name
	    GENERIC_WRITE,	    // access mode
	    0,			    // share mode
	    NULL,		    // security descriptor
	    OPEN_EXISTING,	    // creation disposition
	    0,			    // file attributes
	    NULL);		    // handle to template file
    vim_free(wn);
    if (hFile == INVALID_HANDLE_VALUE)
	return NODE_NORMAL;

    type = GetFileType(hFile);
    CloseHandle(hFile);
    if (type == FILE_TYPE_CHAR)
	return NODE_WRITABLE;
    if (type == FILE_TYPE_DISK)
	return NODE_NORMAL;
    return NODE_OTHER;
}

#ifdef HAVE_ACL
struct my_acl
{
    PSECURITY_DESCRIPTOR    pSecurityDescriptor;
    PSID		    pSidOwner;
    PSID		    pSidGroup;
    PACL		    pDacl;
    PACL		    pSacl;
};
#endif

/*
 * Return a pointer to the ACL of file "fname" in allocated memory.
 * Return NULL if the ACL is not available for whatever reason.
 */
    vim_acl_T
mch_get_acl(char_u *fname)
{
#ifndef HAVE_ACL
    return (vim_acl_T)NULL;
#else
    struct my_acl   *p = NULL;
    DWORD   err;

    p = ALLOC_CLEAR_ONE(struct my_acl);
    if (p != NULL)
    {
	WCHAR	*wn;

	wn = enc_to_utf16(fname, NULL);
	if (wn == NULL)
	{
	    vim_free(p);
	    return NULL;
	}

	// Try to retrieve the entire security descriptor.
	err = GetNamedSecurityInfoW(
		wn,			// Abstract filename
		SE_FILE_OBJECT,		// File Object
		OWNER_SECURITY_INFORMATION |
		GROUP_SECURITY_INFORMATION |
		DACL_SECURITY_INFORMATION |
		SACL_SECURITY_INFORMATION,
		&p->pSidOwner,		// Ownership information.
		&p->pSidGroup,		// Group membership.
		&p->pDacl,		// Discretionary information.
		&p->pSacl,		// For auditing purposes.
		&p->pSecurityDescriptor);
	if (err == ERROR_ACCESS_DENIED ||
		err == ERROR_PRIVILEGE_NOT_HELD)
	{
	    // Retrieve only DACL.
	    (void)GetNamedSecurityInfoW(
		    wn,
		    SE_FILE_OBJECT,
		    DACL_SECURITY_INFORMATION,
		    NULL,
		    NULL,
		    &p->pDacl,
		    NULL,
		    &p->pSecurityDescriptor);
	}
	if (p->pSecurityDescriptor == NULL)
	{
	    mch_free_acl((vim_acl_T)p);
	    p = NULL;
	}
	vim_free(wn);
    }

    return (vim_acl_T)p;
#endif
}

#ifdef HAVE_ACL
/*
 * Check if "acl" contains inherited ACE.
 */
    static BOOL
is_acl_inherited(PACL acl)
{
    DWORD   i;
    ACL_SIZE_INFORMATION    acl_info;
    PACCESS_ALLOWED_ACE	    ace;

    acl_info.AceCount = 0;
    GetAclInformation(acl, &acl_info, sizeof(acl_info), AclSizeInformation);
    for (i = 0; i < acl_info.AceCount; i++)
    {
	GetAce(acl, i, (LPVOID *)&ace);
	if (ace->Header.AceFlags & INHERITED_ACE)
	    return TRUE;
    }
    return FALSE;
}
#endif

/*
 * Set the ACL of file "fname" to "acl" (unless it's NULL).
 * Errors are ignored.
 * This must only be called with "acl" equal to what mch_get_acl() returned.
 */
    void
mch_set_acl(char_u *fname, vim_acl_T acl)
{
#ifdef HAVE_ACL
    struct my_acl   *p = (struct my_acl *)acl;
    SECURITY_INFORMATION    sec_info = 0;
    WCHAR	    *wn;

    if (p == NULL)
	return;

    wn = enc_to_utf16(fname, NULL);
    if (wn == NULL)
	return;

    // Set security flags
    if (p->pSidOwner)
	sec_info |= OWNER_SECURITY_INFORMATION;
    if (p->pSidGroup)
	sec_info |= GROUP_SECURITY_INFORMATION;
    if (p->pDacl)
    {
	sec_info |= DACL_SECURITY_INFORMATION;
	// Do not inherit its parent's DACL.
	// If the DACL is inherited, Cygwin permissions would be changed.
	if (!is_acl_inherited(p->pDacl))
	    sec_info |= PROTECTED_DACL_SECURITY_INFORMATION;
    }
    if (p->pSacl)
	sec_info |= SACL_SECURITY_INFORMATION;

    (void)SetNamedSecurityInfoW(
	    wn,			// Abstract filename
	    SE_FILE_OBJECT,	// File Object
	    sec_info,
	    p->pSidOwner,	// Ownership information.
	    p->pSidGroup,	// Group membership.
	    p->pDacl,		// Discretionary information.
	    p->pSacl		// For auditing purposes.
	    );
    vim_free(wn);
#endif
}

    void
mch_free_acl(vim_acl_T acl)
{
#ifdef HAVE_ACL
    struct my_acl   *p = (struct my_acl *)acl;

    if (p != NULL)
    {
	LocalFree(p->pSecurityDescriptor);	// Free the memory just in case
	vim_free(p);
    }
#endif
}

#if !defined(FEAT_GUI_MSWIN) || defined(VIMDLL)

/*
 * handler for ctrl-break, ctrl-c interrupts, and fatal events.
 */
    static BOOL WINAPI
handler_routine(
    DWORD dwCtrlType)
{
    INPUT_RECORD ir;
    DWORD out;

    switch (dwCtrlType)
    {
    case CTRL_C_EVENT:
	if (ctrl_c_interrupts)
	    g_fCtrlCPressed = TRUE;
	return TRUE;

    case CTRL_BREAK_EVENT:
	g_fCBrkPressed	= TRUE;
	ctrl_break_was_pressed = TRUE;
	// ReadConsoleInput is blocking, send a key event to continue.
	ir.EventType = KEY_EVENT;
	ir.Event.KeyEvent.bKeyDown = TRUE;
	ir.Event.KeyEvent.wRepeatCount = 1;
	ir.Event.KeyEvent.wVirtualKeyCode = VK_CANCEL;
	ir.Event.KeyEvent.wVirtualScanCode = 0;
	ir.Event.KeyEvent.dwControlKeyState = 0;
	ir.Event.KeyEvent.uChar.UnicodeChar = 0;
	WriteConsoleInput(g_hConIn, &ir, 1, &out);
	return TRUE;

    // fatal events: shut down gracefully
    case CTRL_CLOSE_EVENT:
    case CTRL_LOGOFF_EVENT:
    case CTRL_SHUTDOWN_EVENT:
	windgoto((int)Rows - 1, 0);
	g_fForceExit = TRUE;

	vim_snprintf((char *)IObuff, IOSIZE, _("Vim: Caught %s event\n"),
		(dwCtrlType == CTRL_CLOSE_EVENT
		     ? _("close")
		     : dwCtrlType == CTRL_LOGOFF_EVENT
			 ? _("logoff")
			 : _("shutdown")));
# ifdef DEBUG
	OutputDebugString(IObuff);
# endif

	preserve_exit();	// output IObuff, preserve files and exit

	return TRUE;		// not reached

    default:
	return FALSE;
    }
}


/*
 * set the tty in (raw) ? "raw" : "cooked" mode
 */
    void
mch_settmode(tmode_T tmode)
{
    DWORD cmodein;
    DWORD cmodeout;
    BOOL bEnableHandler;

# ifdef VIMDLL
    if (gui.in_use)
	return;
# endif
    GetConsoleMode(g_hConIn, &cmodein);
    GetConsoleMode(g_hConOut, &cmodeout);
    if (tmode == TMODE_RAW)
    {
	cmodein &= ~(ENABLE_LINE_INPUT | ENABLE_PROCESSED_INPUT |
		     ENABLE_ECHO_INPUT);
	if (g_fMouseActive)
	{
	    cmodein |= ENABLE_MOUSE_INPUT;
	    cmodein &= ~ENABLE_QUICK_EDIT_MODE;
	}
	else
	{
	    cmodein |= g_cmodein & ENABLE_QUICK_EDIT_MODE;
	}
	cmodeout &= ~(
# ifdef FEAT_TERMGUICOLORS
	    // Do not turn off the ENABLE_PROCESSED_OUTPUT flag when using
	    // VTP.
	    ((vtp_working) ? 0 : ENABLE_PROCESSED_OUTPUT) |
# else
	    ENABLE_PROCESSED_OUTPUT |
# endif
	    ENABLE_WRAP_AT_EOL_OUTPUT);
	bEnableHandler = TRUE;
    }
    else // cooked
    {
	cmodein |= (ENABLE_LINE_INPUT | ENABLE_PROCESSED_INPUT |
		    ENABLE_ECHO_INPUT);
	cmodeout |= (ENABLE_PROCESSED_OUTPUT | ENABLE_WRAP_AT_EOL_OUTPUT);
	bEnableHandler = FALSE;
    }
    SetConsoleMode(g_hConIn, cmodein | ENABLE_EXTENDED_FLAGS);
    SetConsoleMode(g_hConOut, cmodeout);
    SetConsoleCtrlHandler(handler_routine, bEnableHandler);

# ifdef MCH_WRITE_DUMP
    if (fdDump)
    {
	fprintf(fdDump, "mch_settmode(%s, in = %x, out = %x)\n",
		tmode == TMODE_RAW ? "raw" :
				    tmode == TMODE_COOK ? "cooked" : "normal",
		cmodein, cmodeout);
	fflush(fdDump);
    }
# endif
}


/*
 * Get the size of the current window in `Rows' and `Columns'
 * Return OK when size could be determined, FAIL otherwise.
 */
    int
mch_get_shellsize(void)
{
    CONSOLE_SCREEN_BUFFER_INFO csbi;

# ifdef VIMDLL
    if (gui.in_use)
	return OK;
# endif
    if (!g_fTermcapMode && g_cbTermcap.IsValid)
    {
	/*
	 * For some reason, we are trying to get the screen dimensions
	 * even though we are not in termcap mode.  The 'Rows' and 'Columns'
	 * variables are really intended to mean the size of Vim screen
	 * while in termcap mode.
	 */
	Rows = g_cbTermcap.Info.dwSize.Y;
	Columns = g_cbTermcap.Info.dwSize.X;
    }
    else if (GetConsoleScreenBufferInfo(g_hConOut, &csbi))
    {
	Rows = csbi.srWindow.Bottom - csbi.srWindow.Top + 1;
	Columns = csbi.srWindow.Right - csbi.srWindow.Left + 1;
    }
    else
    {
	Rows = 25;
	Columns = 80;
    }
    return OK;
}

/*
 * Resize console buffer to 'COORD'
 */
    static void
ResizeConBuf(
    HANDLE  hConsole,
    COORD   coordScreen)
{
    if (use_alternate_screen_buffer)
	return;

    if (!SetConsoleScreenBufferSize(hConsole, coordScreen))
    {
# ifdef MCH_WRITE_DUMP
	if (fdDump)
	{
	    fprintf(fdDump, "SetConsoleScreenBufferSize failed: %lx\n",
		    GetLastError());
	    fflush(fdDump);
	}
# endif
    }
}

/*
 * Resize console window size to 'srWindowRect'
 */
    static void
ResizeWindow(
    HANDLE     hConsole,
    SMALL_RECT srWindowRect)
{
    if (!SetConsoleWindowInfo(hConsole, TRUE, &srWindowRect))
    {
# ifdef MCH_WRITE_DUMP
	if (fdDump)
	{
	    fprintf(fdDump, "SetConsoleWindowInfo failed: %lx\n",
		    GetLastError());
	    fflush(fdDump);
	}
# endif
    }
}

/*
 * Set a console window to `xSize' * `ySize'
 */
    static void
ResizeConBufAndWindow(
    HANDLE  hConsole,
    int	    xSize,
    int	    ySize)
{
    CONSOLE_SCREEN_BUFFER_INFO csbi;	// hold current console buffer info
    SMALL_RECT	    srWindowRect;	// hold the new console size
    COORD	    coordScreen;
    COORD	    cursor;
    static int	    resized = FALSE;

# ifdef MCH_WRITE_DUMP
    if (fdDump)
    {
	fprintf(fdDump, "ResizeConBufAndWindow(%d, %d)\n", xSize, ySize);
	fflush(fdDump);
    }
# endif

    // get the largest size we can size the console window to
    coordScreen = GetLargestConsoleWindowSize(hConsole);

    // define the new console window size and scroll position
    srWindowRect.Left = srWindowRect.Top = (SHORT) 0;
    srWindowRect.Right =  (SHORT) (min(xSize, coordScreen.X) - 1);
    srWindowRect.Bottom = (SHORT) (min(ySize, coordScreen.Y) - 1);

    if (GetConsoleScreenBufferInfo(g_hConOut, &csbi))
    {
	int sx, sy;

	sx = csbi.srWindow.Right - csbi.srWindow.Left + 1;
	sy = csbi.srWindow.Bottom - csbi.srWindow.Top + 1;
	if (sy < ySize || sx < xSize)
	{
	    /*
	     * Increasing number of lines/columns, do buffer first.
	     * Use the maximal size in x and y direction.
	     */
	    if (sy < ySize)
		coordScreen.Y = ySize;
	    else
		coordScreen.Y = sy;
	    if (sx < xSize)
		coordScreen.X = xSize;
	    else
		coordScreen.X = sx;
	    SetConsoleScreenBufferSize(hConsole, coordScreen);
	}
    }

    // define the new console buffer size
    coordScreen.X = xSize;
    coordScreen.Y = ySize;

    // In the new console call API, only the first time in reverse order
    if (!vtp_working || resized)
    {
	ResizeWindow(hConsole, srWindowRect);
	ResizeConBuf(hConsole, coordScreen);
    }
    else
    {
	// Workaround for a Windows 10 bug
	cursor.X = srWindowRect.Left;
	cursor.Y = srWindowRect.Top;
	SetConsoleCursorPosition(hConsole, cursor);

	ResizeConBuf(hConsole, coordScreen);
	ResizeWindow(hConsole, srWindowRect);
	resized = TRUE;
    }
}


/*
 * Set the console window to `Rows' * `Columns'
 */
    void
mch_set_shellsize(void)
{
    COORD coordScreen;

# ifdef VIMDLL
    if (gui.in_use)
	return;
# endif
    // Don't change window size while still starting up
    if (suppress_winsize != 0)
    {
	suppress_winsize = 2;
	return;
    }

    if (term_console)
    {
	coordScreen = GetLargestConsoleWindowSize(g_hConOut);

	// Clamp Rows and Columns to reasonable values
	if (Rows > coordScreen.Y)
	    Rows = coordScreen.Y;
	if (Columns > coordScreen.X)
	    Columns = coordScreen.X;

	ResizeConBufAndWindow(g_hConOut, Columns, Rows);
    }
}

/*
 * Rows and/or Columns has changed.
 */
    void
mch_new_shellsize(void)
{
# ifdef VIMDLL
    if (gui.in_use)
	return;
# endif
    set_scroll_region(0, 0, Columns - 1, Rows - 1);
}


/*
 * Called when started up, to set the winsize that was delayed.
 */
    void
mch_set_winsize_now(void)
{
    if (suppress_winsize == 2)
    {
	suppress_winsize = 0;
	mch_set_shellsize();
	shell_resized();
    }
    suppress_winsize = 0;
}
#endif // FEAT_GUI_MSWIN

    static BOOL
vim_create_process(
    char		*cmd,
    BOOL		inherit_handles,
    DWORD		flags,
    STARTUPINFO		*si,
    PROCESS_INFORMATION *pi,
    LPVOID		*env,
    char		*cwd)
{
    BOOL	ret = FALSE;
    WCHAR	*wcmd, *wcwd = NULL;

    wcmd = enc_to_utf16((char_u *)cmd, NULL);
    if (wcmd == NULL)
	return FALSE;
    if (cwd != NULL)
    {
	wcwd = enc_to_utf16((char_u *)cwd, NULL);
	if (wcwd == NULL)
	    goto theend;
    }

    ret = CreateProcessW(
	    NULL,		// Executable name
	    wcmd,		// Command to execute
	    NULL,		// Process security attributes
	    NULL,		// Thread security attributes
	    inherit_handles,	// Inherit handles
	    flags,		// Creation flags
	    env,		// Environment
	    wcwd,		// Current directory
	    (LPSTARTUPINFOW)si,	// Startup information
	    pi);		// Process information
theend:
    vim_free(wcmd);
    vim_free(wcwd);
    return ret;
}


    static HINSTANCE
vim_shell_execute(
    char *cmd,
    INT	 n_show_cmd)
{
    HINSTANCE	ret;
    WCHAR	*wcmd;

    wcmd = enc_to_utf16((char_u *)cmd, NULL);
    if (wcmd == NULL)
	return (HINSTANCE) 0;

    ret = ShellExecuteW(NULL, NULL, wcmd, NULL, NULL, n_show_cmd);
    vim_free(wcmd);
    return ret;
}


#if defined(FEAT_GUI_MSWIN) || defined(PROTO)

/*
 * Specialised version of system() for Win32 GUI mode.
 * This version proceeds as follows:
 *    1. Create a console window for use by the subprocess
 *    2. Run the subprocess (it gets the allocated console by default)
 *    3. Wait for the subprocess to terminate and get its exit code
 *    4. Prompt the user to press a key to close the console window
 */
    static int
mch_system_classic(char *cmd, int options)
{
    STARTUPINFO		si;
    PROCESS_INFORMATION pi;
    DWORD		ret = 0;
    HWND		hwnd = GetFocus();

    si.cb = sizeof(si);
    si.lpReserved = NULL;
    si.lpDesktop = NULL;
    si.lpTitle = NULL;
    si.dwFlags = STARTF_USESHOWWINDOW;
    /*
     * It's nicer to run a filter command in a minimized window.
     * Don't activate the window to keep focus on Vim.
     */
    if (options & SHELL_DOOUT)
	si.wShowWindow = SW_SHOWMINNOACTIVE;
    else
	si.wShowWindow = SW_SHOWNORMAL;
    si.cbReserved2 = 0;
    si.lpReserved2 = NULL;

    // Now, run the command
    vim_create_process(cmd, FALSE,
	    CREATE_DEFAULT_ERROR_MODE |	CREATE_NEW_CONSOLE,
	    &si, &pi, NULL, NULL);

    // Wait for the command to terminate before continuing
    {
# ifdef FEAT_GUI
	int	    delay = 1;

	// Keep updating the window while waiting for the shell to finish.
	for (;;)
	{
	    MSG	msg;

	    if (PeekMessageW(&msg, (HWND)NULL, 0, 0, PM_REMOVE))
	    {
		TranslateMessage(&msg);
		DispatchMessageW(&msg);
		delay = 1;
		continue;
	    }
	    if (WaitForSingleObject(pi.hProcess, delay) != WAIT_TIMEOUT)
		break;

	    // We start waiting for a very short time and then increase it, so
	    // that we respond quickly when the process is quick, and don't
	    // consume too much overhead when it's slow.
	    if (delay < 50)
		delay += 10;
	}
# else
	WaitForSingleObject(pi.hProcess, INFINITE);
# endif

	// Get the command exit code
	GetExitCodeProcess(pi.hProcess, &ret);
    }

    // Close the handles to the subprocess, so that it goes away
    CloseHandle(pi.hThread);
    CloseHandle(pi.hProcess);

    // Try to get input focus back.  Doesn't always work though.
    PostMessage(hwnd, WM_SETFOCUS, 0, 0);

    return ret;
}

/*
 * Thread launched by the gui to send the current buffer data to the
 * process. This way avoid to hang up vim totally if the children
 * process take a long time to process the lines.
 */
    static unsigned int __stdcall
sub_process_writer(LPVOID param)
{
    HANDLE	    g_hChildStd_IN_Wr = param;
    linenr_T	    lnum = curbuf->b_op_start.lnum;
    DWORD	    len = 0;
    DWORD	    l;
    char_u	    *lp = ml_get(lnum);
    char_u	    *s;
    int		    written = 0;

    for (;;)
    {
	l = (DWORD)STRLEN(lp + written);
	if (l == 0)
	    len = 0;
	else if (lp[written] == NL)
	{
	    // NL -> NUL translation
	    WriteFile(g_hChildStd_IN_Wr, "", 1, &len, NULL);
	}
	else
	{
	    s = vim_strchr(lp + written, NL);
	    WriteFile(g_hChildStd_IN_Wr, (char *)lp + written,
		      s == NULL ? l : (DWORD)(s - (lp + written)),
		      &len, NULL);
	}
	if (len == l)
	{
	    // Finished a line, add a NL, unless this line should not have
	    // one.
	    if (lnum != curbuf->b_op_end.lnum
		|| (!curbuf->b_p_bin
		    && curbuf->b_p_fixeol)
		|| (lnum != curbuf->b_no_eol_lnum
		    && (lnum != curbuf->b_ml.ml_line_count
			|| curbuf->b_p_eol)))
	    {
		WriteFile(g_hChildStd_IN_Wr, "\n", 1,
						  (LPDWORD)&vim_ignored, NULL);
	    }

	    ++lnum;
	    if (lnum > curbuf->b_op_end.lnum)
		break;

	    lp = ml_get(lnum);
	    written = 0;
	}
	else if (len > 0)
	    written += len;
    }

    // finished all the lines, close pipe
    CloseHandle(g_hChildStd_IN_Wr);
    return 0;
}


# define BUFLEN 100	// length for buffer, stolen from unix version

/*
 * This function read from the children's stdout and write the
 * data on screen or in the buffer accordingly.
 */
    static void
dump_pipe(int	    options,
	  HANDLE    g_hChildStd_OUT_Rd,
	  garray_T  *ga,
	  char_u    buffer[],
	  DWORD	    *buffer_off)
{
    DWORD	availableBytes = 0;
    DWORD	i;
    int		ret;
    DWORD	len;
    DWORD	toRead;

    // we query the pipe to see if there is any data to read
    // to avoid to perform a blocking read
    ret = PeekNamedPipe(g_hChildStd_OUT_Rd, // pipe to query
			NULL,		    // optional buffer
			0,		    // buffer size
			NULL,		    // number of read bytes
			&availableBytes,    // available bytes total
			NULL);		    // byteLeft

    // We got real data in the pipe, read it
    while (ret != 0 && availableBytes > 0)
    {
	toRead = (DWORD)(BUFLEN - *buffer_off);
	toRead = availableBytes < toRead ? availableBytes : toRead;
	ReadFile(g_hChildStd_OUT_Rd, buffer + *buffer_off, toRead , &len, NULL);

	// If we haven't read anything, there is a problem
	if (len == 0)
	    break;

	availableBytes -= len;

	if (options & SHELL_READ)
	{
	    // Do NUL -> NL translation, append NL separated
	    // lines to the current buffer.
	    for (i = 0; i < len; ++i)
	    {
		if (buffer[i] == NL)
		    append_ga_line(ga);
		else if (buffer[i] == NUL)
		    ga_append(ga, NL);
		else
		    ga_append(ga, buffer[i]);
	    }
	}
	else if (has_mbyte)
	{
	    int		l;
	    int		c;
	    char_u	*p;

	    len += *buffer_off;
	    buffer[len] = NUL;

	    // Check if the last character in buffer[] is
	    // incomplete, keep these bytes for the next
	    // round.
	    for (p = buffer; p < buffer + len; p += l)
	    {
		l = MB_CPTR2LEN(p);
		if (l == 0)
		    l = 1;  // NUL byte?
		else if (MB_BYTE2LEN(*p) != l)
		    break;
	    }
	    if (p == buffer)	// no complete character
	    {
		// avoid getting stuck at an illegal byte
		if (len >= 12)
		    ++p;
		else
		{
		    *buffer_off = len;
		    return;
		}
	    }
	    c = *p;
	    *p = NUL;
	    msg_puts((char *)buffer);
	    if (p < buffer + len)
	    {
		*p = c;
		*buffer_off = (DWORD)((buffer + len) - p);
		mch_memmove(buffer, p, *buffer_off);
		return;
	    }
	    *buffer_off = 0;
	}
	else
	{
	    buffer[len] = NUL;
	    msg_puts((char *)buffer);
	}

	windgoto(msg_row, msg_col);
	cursor_on();
	out_flush();
    }
}

/*
 * Version of system to use for windows NT > 5.0 (Win2K), use pipe
 * for communication and doesn't open any new window.
 */
    static int
mch_system_piped(char *cmd, int options)
{
    STARTUPINFO		si;
    PROCESS_INFORMATION pi;
    DWORD		ret = 0;

    HANDLE g_hChildStd_IN_Rd = NULL;
    HANDLE g_hChildStd_IN_Wr = NULL;
    HANDLE g_hChildStd_OUT_Rd = NULL;
    HANDLE g_hChildStd_OUT_Wr = NULL;

    char_u	buffer[BUFLEN + 1]; // reading buffer + size
    DWORD	len;

    // buffer used to receive keys
    char_u	ta_buf[BUFLEN + 1];	// TypeAHead
    int		ta_len = 0;		// valid bytes in ta_buf[]

    DWORD	i;
    int		noread_cnt = 0;
    garray_T	ga;
    int		delay = 1;
    DWORD	buffer_off = 0;	// valid bytes in buffer[]
    char	*p = NULL;

    SECURITY_ATTRIBUTES saAttr;

    // Set the bInheritHandle flag so pipe handles are inherited.
    saAttr.nLength = sizeof(SECURITY_ATTRIBUTES);
    saAttr.bInheritHandle = TRUE;
    saAttr.lpSecurityDescriptor = NULL;

    if ( ! CreatePipe(&g_hChildStd_OUT_Rd, &g_hChildStd_OUT_Wr, &saAttr, 0)
	// Ensure the read handle to the pipe for STDOUT is not inherited.
       || ! SetHandleInformation(g_hChildStd_OUT_Rd, HANDLE_FLAG_INHERIT, 0)
	// Create a pipe for the child process's STDIN.
       || ! CreatePipe(&g_hChildStd_IN_Rd, &g_hChildStd_IN_Wr, &saAttr, 0)
	// Ensure the write handle to the pipe for STDIN is not inherited.
       || ! SetHandleInformation(g_hChildStd_IN_Wr, HANDLE_FLAG_INHERIT, 0) )
    {
	CloseHandle(g_hChildStd_IN_Rd);
	CloseHandle(g_hChildStd_IN_Wr);
	CloseHandle(g_hChildStd_OUT_Rd);
	CloseHandle(g_hChildStd_OUT_Wr);
	msg_puts(_("\nCannot create pipes\n"));
    }

    si.cb = sizeof(si);
    si.lpReserved = NULL;
    si.lpDesktop = NULL;
    si.lpTitle = NULL;
    si.dwFlags = STARTF_USESHOWWINDOW | STARTF_USESTDHANDLES;

    // set-up our file redirection
    si.hStdError = g_hChildStd_OUT_Wr;
    si.hStdOutput = g_hChildStd_OUT_Wr;
    si.hStdInput = g_hChildStd_IN_Rd;
    si.wShowWindow = SW_HIDE;
    si.cbReserved2 = 0;
    si.lpReserved2 = NULL;

    if (options & SHELL_READ)
	ga_init2(&ga, 1, BUFLEN);

    if (cmd != NULL)
    {
	p = (char *)vim_strsave((char_u *)cmd);
	if (p != NULL)
	    unescape_shellxquote((char_u *)p, p_sxe);
	else
	    p = cmd;
    }

    // Now, run the command.
    // About "Inherit handles" being TRUE: this command can be litigious,
    // handle inheritance was deactivated for pending temp file, but, if we
    // deactivate it, the pipes don't work for some reason.
     vim_create_process(p, TRUE, CREATE_DEFAULT_ERROR_MODE,
	     &si, &pi, NULL, NULL);

    if (p != cmd)
	vim_free(p);

    // Close our unused side of the pipes
    CloseHandle(g_hChildStd_IN_Rd);
    CloseHandle(g_hChildStd_OUT_Wr);

    if (options & SHELL_WRITE)
    {
	HANDLE thread = (HANDLE)
	 _beginthreadex(NULL,  // security attributes
			0,     // default stack size
			sub_process_writer, // function to be executed
			g_hChildStd_IN_Wr,  // parameter
			0,		 // creation flag, start immediately
			NULL);		    // we don't care about thread id
	CloseHandle(thread);
	g_hChildStd_IN_Wr = NULL;
    }

    // Keep updating the window while waiting for the shell to finish.
    for (;;)
    {
	MSG	msg;

	if (PeekMessageW(&msg, (HWND)NULL, 0, 0, PM_REMOVE))
	{
	    TranslateMessage(&msg);
	    DispatchMessageW(&msg);
	}

	// write pipe information in the window
	if ((options & (SHELL_READ|SHELL_WRITE))
# ifdef FEAT_GUI
		|| gui.in_use
# endif
	    )
	{
	    len = 0;
	    if (!(options & SHELL_EXPAND)
		&& ((options &
			(SHELL_READ|SHELL_WRITE|SHELL_COOKED))
		    != (SHELL_READ|SHELL_WRITE|SHELL_COOKED)
# ifdef FEAT_GUI
		    || gui.in_use
# endif
		    )
		&& (ta_len > 0 || noread_cnt > 4))
	    {
		if (ta_len == 0)
		{
		    // Get extra characters when we don't have any.  Reset the
		    // counter and timer.
		    noread_cnt = 0;
		    len = ui_inchar(ta_buf, BUFLEN, 10L, 0);
		}
		if (ta_len > 0 || len > 0)
		{
		    /*
		     * For pipes: Check for CTRL-C: send interrupt signal to
		     * child.
		     */
		    if (len == 1 && cmd != NULL)
		    {
			if (ta_buf[ta_len] == Ctrl_C)
			{
			    // Learn what exit code is expected, for
			// now put 9 as SIGKILL
			    TerminateProcess(pi.hProcess, 9);
			}
		    }

		    /*
		     * Check for CTRL-D: EOF, close pipe to child.
		     * Ctrl_D may be decorated by _OnChar()
		     */
		    if ((len == 1 || len == 4 ) && cmd != NULL)
		    {
			if (ta_buf[0] == Ctrl_D
			    || (ta_buf[0] == CSI
				&& ta_buf[1] == KS_MODIFIER
				&& ta_buf[3] == Ctrl_D))
			{
			    CloseHandle(g_hChildStd_IN_Wr);
			    g_hChildStd_IN_Wr = NULL;
			    len = 0;
			}
		    }

		    len = term_replace_keycodes(ta_buf, ta_len, len);

		    /*
		     * For pipes: echo the typed characters.  For a pty this
		     * does not seem to work.
		     */
		    for (i = ta_len; i < ta_len + len; ++i)
		    {
			if (ta_buf[i] == '\n' || ta_buf[i] == '\b')
			    msg_putchar(ta_buf[i]);
			else if (has_mbyte)
			{
			    int l = (*mb_ptr2len)(ta_buf + i);

			    msg_outtrans_len(ta_buf + i, l);
			    i += l - 1;
			}
			else
			    msg_outtrans_len(ta_buf + i, 1);
		    }
		    windgoto(msg_row, msg_col);
		    out_flush();

		    ta_len += len;

		    /*
		     * Write the characters to the child, unless EOF has been
		     * typed for pipes.  Write one character at a time, to
		     * avoid losing too much typeahead.  When writing buffer
		     * lines, drop the typed characters (only check for
		     * CTRL-C).
		     */
		    if (options & SHELL_WRITE)
			ta_len = 0;
		    else if (g_hChildStd_IN_Wr != NULL)
		    {
			WriteFile(g_hChildStd_IN_Wr, (char*)ta_buf,
				    1, &len, NULL);
			// if we are typing in, we want to keep things reactive
			delay = 1;
			if (len > 0)
			{
			    ta_len -= len;
			    mch_memmove(ta_buf, ta_buf + len, ta_len);
			}
		    }
		}
	    }
	}

	if (ta_len)
	    ui_inchar_undo(ta_buf, ta_len);

	if (WaitForSingleObject(pi.hProcess, delay) != WAIT_TIMEOUT)
	{
	    dump_pipe(options, g_hChildStd_OUT_Rd, &ga, buffer, &buffer_off);
	    break;
	}

	++noread_cnt;
	dump_pipe(options, g_hChildStd_OUT_Rd, &ga, buffer, &buffer_off);

	// We start waiting for a very short time and then increase it, so
	// that we respond quickly when the process is quick, and don't
	// consume too much overhead when it's slow.
	if (delay < 50)
	    delay += 10;
    }

    // Close the pipe
    CloseHandle(g_hChildStd_OUT_Rd);
    if (g_hChildStd_IN_Wr != NULL)
	CloseHandle(g_hChildStd_IN_Wr);

    WaitForSingleObject(pi.hProcess, INFINITE);

    // Get the command exit code
    GetExitCodeProcess(pi.hProcess, &ret);

    if (options & SHELL_READ)
    {
	if (ga.ga_len > 0)
	{
	    append_ga_line(&ga);
	    // remember that the NL was missing
	    curbuf->b_no_eol_lnum = curwin->w_cursor.lnum;
	}
	else
	    curbuf->b_no_eol_lnum = 0;
	ga_clear(&ga);
    }

    // Close the handles to the subprocess, so that it goes away
    CloseHandle(pi.hThread);
    CloseHandle(pi.hProcess);

    return ret;
}

    static int
mch_system_g(char *cmd, int options)
{
    // if we can pipe and the shelltemp option is off
    if (!p_stmp)
	return mch_system_piped(cmd, options);
    else
	return mch_system_classic(cmd, options);
}
#endif

#if !defined(FEAT_GUI_MSWIN) || defined(VIMDLL)
    static int
mch_system_c(char *cmd, int options UNUSED)
{
    int		ret;
    WCHAR	*wcmd;
    char_u	*buf;
    size_t	len;

    // If the command starts and ends with double quotes, enclose the command
    // in parentheses.
    len = STRLEN(cmd);
    if (len >= 2 && cmd[0] == '"' && cmd[len - 1] == '"')
    {
	len += 3;
	buf = alloc(len);
	if (buf == NULL)
	    return -1;
	vim_snprintf((char *)buf, len, "(%s)", cmd);
	wcmd = enc_to_utf16(buf, NULL);
	free(buf);
    }
    else
	wcmd = enc_to_utf16((char_u *)cmd, NULL);

    if (wcmd == NULL)
	return -1;

    ret = _wsystem(wcmd);
    vim_free(wcmd);
    return ret;
}

#endif

    static int
mch_system(char *cmd, int options)
{
#ifdef VIMDLL
    if (gui.in_use || gui.starting)
	return mch_system_g(cmd, options);
    else
	return mch_system_c(cmd, options);
#elif defined(FEAT_GUI_MSWIN)
    return mch_system_g(cmd, options);
#else
    return mch_system_c(cmd, options);
#endif
}

#if defined(FEAT_GUI) && defined(FEAT_TERMINAL)
/*
 * Use a terminal window to run a shell command in.
 */
    static int
mch_call_shell_terminal(
    char_u	*cmd,
    int		options UNUSED)	// SHELL_*, see vim.h
{
    jobopt_T	opt;
    char_u	*newcmd = NULL;
    typval_T	argvar[2];
    long_u	cmdlen;
    int		retval = -1;
    buf_T	*buf;
    job_T	*job;
    aco_save_T	aco;
    oparg_T	oa;		// operator arguments

    if (cmd == NULL)
	cmdlen = STRLEN(p_sh) + 1;
    else
	cmdlen = STRLEN(p_sh) + STRLEN(p_shcf) + STRLEN(cmd) + 10;
    newcmd = alloc(cmdlen);
    if (newcmd == NULL)
	return 255;
    if (cmd == NULL)
    {
	STRCPY(newcmd, p_sh);
	ch_log(NULL, "starting terminal to run a shell");
    }
    else
    {
	vim_snprintf((char *)newcmd, cmdlen, "%s %s %s", p_sh, p_shcf, cmd);
	ch_log(NULL, "starting terminal for system command '%s'", cmd);
    }

    init_job_options(&opt);

    argvar[0].v_type = VAR_STRING;
    argvar[0].vval.v_string = newcmd;
    argvar[1].v_type = VAR_UNKNOWN;
    buf = term_start(argvar, NULL, &opt, TERM_START_SYSTEM);
    if (buf == NULL)
    {
	vim_free(newcmd);
	return 255;
    }

    job = term_getjob(buf->b_term);
    ++job->jv_refcount;

    // Find a window to make "buf" curbuf.
    aucmd_prepbuf(&aco, buf);
    if (curbuf == buf)
    {
	// Only do this when a window was found for "buf".
	clear_oparg(&oa);
	while (term_use_loop())
	{
	    if (oa.op_type == OP_NOP && oa.regname == NUL && !VIsual_active)
	    {
		// If terminal_loop() returns OK we got a key that is handled
		// in Normal model. We don't do redrawing anyway.
		if (terminal_loop(TRUE) == OK)
		    normal_cmd(&oa, TRUE);
	    }
	    else
		normal_cmd(&oa, TRUE);
	}
	retval = job->jv_exitval;
	ch_log(NULL, "system command finished");

	job_unref(job);

	// restore curwin/curbuf and a few other things
	aucmd_restbuf(&aco);
    }

    wait_return(TRUE);
    do_buffer(DOBUF_WIPE, DOBUF_FIRST, FORWARD, buf->b_fnum, TRUE);

    vim_free(newcmd);
    return retval;
}
#endif

/*
 * Either execute a command by calling the shell or start a new shell
 */
    int
mch_call_shell(
    char_u  *cmd,
    int	    options)	// SHELL_*, see vim.h
{
    int		x = 0;
    int		tmode = cur_tmode;
    WCHAR	szShellTitle[512];

#ifdef FEAT_EVAL
    ch_log(NULL, "executing shell command: %s", cmd);
#endif
    // Change the title to reflect that we are in a subshell.
    if (GetConsoleTitleW(szShellTitle, ARRAY_LENGTH(szShellTitle) - 4) > 0)
    {
	if (cmd == NULL)
	    wcscat(szShellTitle, L" :sh");
	else
	{
	    WCHAR *wn = enc_to_utf16((char_u *)cmd, NULL);

	    if (wn != NULL)
	    {
		wcscat(szShellTitle, L" - !");
		if ((wcslen(szShellTitle) + wcslen(wn) <
			    ARRAY_LENGTH(szShellTitle)))
		    wcscat(szShellTitle, wn);
		SetConsoleTitleW(szShellTitle);
		vim_free(wn);
	    }
	}
    }

    out_flush();

#ifdef MCH_WRITE_DUMP
    if (fdDump)
    {
	fprintf(fdDump, "mch_call_shell(\"%s\", %d)\n", cmd, options);
	fflush(fdDump);
    }
#endif
#if defined(FEAT_GUI) && defined(FEAT_TERMINAL)
    // TODO: make the terminal window work with input or output redirected.
    if (
# ifdef VIMDLL
	    gui.in_use &&
# endif
	    vim_strchr(p_go, GO_TERMINAL) != NULL
	 && (options & (SHELL_FILTER|SHELL_DOOUT|SHELL_WRITE|SHELL_READ)) == 0)
    {
	char_u	*cmdbase = cmd;

	if (cmdbase != NULL)
	    // Skip a leading quote and (.
	    while (*cmdbase == '"' || *cmdbase == '(')
		++cmdbase;

	// Check the command does not begin with "start "
	if (cmdbase == NULL || STRNICMP(cmdbase, "start", 5) != 0
						   || !VIM_ISWHITE(cmdbase[5]))
	{
	    // Use a terminal window to run the command in.
	    x = mch_call_shell_terminal(cmd, options);
	    resettitle();
	    return x;
	}
    }
#endif

    /*
     * Catch all deadly signals while running the external command, because a
     * CTRL-C, Ctrl-Break or illegal instruction  might otherwise kill us.
     */
    mch_signal(SIGINT, SIG_IGN);
#if defined(__GNUC__) && !defined(__MINGW32__)
    mch_signal(SIGKILL, SIG_IGN);
#else
    mch_signal(SIGBREAK, SIG_IGN);
#endif
    mch_signal(SIGILL, SIG_IGN);
    mch_signal(SIGFPE, SIG_IGN);
    mch_signal(SIGSEGV, SIG_IGN);
    mch_signal(SIGTERM, SIG_IGN);
    mch_signal(SIGABRT, SIG_IGN);

    if (options & SHELL_COOKED)
	settmode(TMODE_COOK);	// set to normal mode

    if (cmd == NULL)
    {
	x = mch_system((char *)p_sh, options);
    }
    else
    {
	// we use "command" or "cmd" to start the shell; slow but easy
	char_u	*newcmd = NULL;
	char_u	*cmdbase = cmd;
	long_u	cmdlen;

	// Skip a leading ", ( and "(.
	if (*cmdbase == '"' )
	    ++cmdbase;
	if (*cmdbase == '(')
	    ++cmdbase;

	if ((STRNICMP(cmdbase, "start", 5) == 0) && VIM_ISWHITE(cmdbase[5]))
	{
	    STARTUPINFO		si;
	    PROCESS_INFORMATION	pi;
	    DWORD		flags = CREATE_NEW_CONSOLE;
	    INT			n_show_cmd = SW_SHOWNORMAL;
	    char_u		*p;

	    ZeroMemory(&si, sizeof(si));
	    si.cb = sizeof(si);
	    si.lpReserved = NULL;
	    si.lpDesktop = NULL;
	    si.lpTitle = NULL;
	    si.dwFlags = 0;
	    si.cbReserved2 = 0;
	    si.lpReserved2 = NULL;

	    cmdbase = skipwhite(cmdbase + 5);
	    if ((STRNICMP(cmdbase, "/min", 4) == 0)
		    && VIM_ISWHITE(cmdbase[4]))
	    {
		cmdbase = skipwhite(cmdbase + 4);
		si.dwFlags = STARTF_USESHOWWINDOW;
		si.wShowWindow = SW_SHOWMINNOACTIVE;
		n_show_cmd = SW_SHOWMINNOACTIVE;
	    }
	    else if ((STRNICMP(cmdbase, "/b", 2) == 0)
		    && VIM_ISWHITE(cmdbase[2]))
	    {
		cmdbase = skipwhite(cmdbase + 2);
		flags = CREATE_NO_WINDOW;
		si.dwFlags = STARTF_USESTDHANDLES;
		si.hStdInput = CreateFile("\\\\.\\NUL",	// File name
		    GENERIC_READ,			// Access flags
		    0,					// Share flags
		    NULL,				// Security att.
		    OPEN_EXISTING,			// Open flags
		    FILE_ATTRIBUTE_NORMAL,		// File att.
		    NULL);				// Temp file
		si.hStdOutput = si.hStdInput;
		si.hStdError = si.hStdInput;
	    }

	    // Remove a trailing ", ) and )" if they have a match
	    // at the start of the command.
	    if (cmdbase > cmd)
	    {
		p = cmdbase + STRLEN(cmdbase);
		if (p > cmdbase && p[-1] == '"' && *cmd == '"')
		    *--p = NUL;
		if (p > cmdbase && p[-1] == ')'
			&& (*cmd =='(' || cmd[1] == '('))
		    *--p = NUL;
	    }

	    newcmd = cmdbase;
	    unescape_shellxquote(cmdbase, p_sxe);

	    /*
	     * If creating new console, arguments are passed to the
	     * 'cmd.exe' as-is. If it's not, arguments are not treated
	     * correctly for current 'cmd.exe'. So unescape characters in
	     * shellxescape except '|' for avoiding to be treated as
	     * argument to them. Pass the arguments to sub-shell.
	     */
	    if (flags != CREATE_NEW_CONSOLE)
	    {
		char_u	*subcmd;
		char_u	*cmd_shell = mch_getenv("COMSPEC");

		if (cmd_shell == NULL || *cmd_shell == NUL)
		    cmd_shell = (char_u *)default_shell();

		subcmd = vim_strsave_escaped_ext(cmdbase,
			(char_u *)"|", '^', FALSE);
		if (subcmd != NULL)
		{
		    // make "cmd.exe /c arguments"
		    cmdlen = STRLEN(cmd_shell) + STRLEN(subcmd) + 5;
		    newcmd = alloc(cmdlen);
		    if (newcmd != NULL)
			vim_snprintf((char *)newcmd, cmdlen, "%s /c %s",
						       cmd_shell, subcmd);
		    else
			newcmd = cmdbase;
		    vim_free(subcmd);
		}
	    }

	    /*
	     * Now, start the command as a process, so that it doesn't
	     * inherit our handles which causes unpleasant dangling swap
	     * files if we exit before the spawned process
	     */
	    if (vim_create_process((char *)newcmd, FALSE, flags,
			&si, &pi, NULL, NULL))
		x = 0;
	    else if (vim_shell_execute((char *)newcmd, n_show_cmd)
							       > (HINSTANCE)32)
		x = 0;
	    else
	    {
		x = -1;
#ifdef FEAT_GUI_MSWIN
# ifdef VIMDLL
		if (gui.in_use)
# endif
		    emsg(_(e_command_not_found));
#endif
	    }

	    if (newcmd != cmdbase)
		vim_free(newcmd);

	    if (si.dwFlags == STARTF_USESTDHANDLES && si.hStdInput != NULL)
	    {
		// Close the handle to \\.\NUL created above.
		CloseHandle(si.hStdInput);
	    }
	    // Close the handles to the subprocess, so that it goes away
	    CloseHandle(pi.hThread);
	    CloseHandle(pi.hProcess);
	}
	else
	{
	    cmdlen =
#ifdef FEAT_GUI_MSWIN
		((gui.in_use || gui.starting) ?
		    (!s_dont_use_vimrun && p_stmp ?
			STRLEN(vimrun_path) : STRLEN(p_sh) + STRLEN(p_shcf))
		    : 0) +
#endif
		STRLEN(p_sh) + STRLEN(p_shcf) + STRLEN(cmd) + 10;

	    newcmd = alloc(cmdlen);
	    if (newcmd != NULL)
	    {
#if defined(FEAT_GUI_MSWIN)
		if (
# ifdef VIMDLL
		    (gui.in_use || gui.starting) &&
# endif
		    need_vimrun_warning)
		{
		    char *msg = _("VIMRUN.EXE not found in your $PATH.\n"
			"External commands will not pause after completion.\n"
			"See  :help win32-vimrun  for more information.");
		    char *title = _("Vim Warning");
		    WCHAR *wmsg = enc_to_utf16((char_u *)msg, NULL);
		    WCHAR *wtitle = enc_to_utf16((char_u *)title, NULL);

		    if (wmsg != NULL && wtitle != NULL)
			MessageBoxW(NULL, wmsg, wtitle, MB_ICONWARNING);
		    vim_free(wmsg);
		    vim_free(wtitle);
		    need_vimrun_warning = FALSE;
		}
		if (
# ifdef VIMDLL
		    (gui.in_use || gui.starting) &&
# endif
		    !s_dont_use_vimrun && p_stmp)
		    // Use vimrun to execute the command.  It opens a console
		    // window, which can be closed without killing Vim.
		    vim_snprintf((char *)newcmd, cmdlen, "%s%s%s %s %s",
			    vimrun_path,
			    (msg_silent != 0 || (options & SHELL_DOOUT))
								 ? "-s " : "",
			    p_sh, p_shcf, cmd);
		else if (
# ifdef VIMDLL
			(gui.in_use || gui.starting) &&
# endif
			s_dont_use_vimrun && STRCMP(p_shcf, "/c") == 0)
		    // workaround for the case that "vimrun" does not exist
		    vim_snprintf((char *)newcmd, cmdlen, "%s %s %s %s %s",
					   p_sh, p_shcf, p_sh, p_shcf, cmd);
		else
#endif
		    vim_snprintf((char *)newcmd, cmdlen, "%s %s %s",
							   p_sh, p_shcf, cmd);
		x = mch_system((char *)newcmd, options);
		vim_free(newcmd);
	    }
	}
    }

    if (tmode == TMODE_RAW)
    {
	// The shell may have messed with the mode, always set it.
	cur_tmode = TMODE_UNKNOWN;
	settmode(TMODE_RAW);	// set to raw mode
    }

    // Print the return value, unless "vimrun" was used.
    if (x != 0 && !(options & SHELL_SILENT) && !emsg_silent
#if defined(FEAT_GUI_MSWIN)
	    && ((gui.in_use || gui.starting) ?
		((options & SHELL_DOOUT) || s_dont_use_vimrun || !p_stmp) : 1)
#endif
	    )
    {
	smsg(_("shell returned %d"), x);
	msg_putchar('\n');
    }
    resettitle();

    mch_signal(SIGINT, SIG_DFL);
#if defined(__GNUC__) && !defined(__MINGW32__)
    mch_signal(SIGKILL, SIG_DFL);
#else
    mch_signal(SIGBREAK, SIG_DFL);
#endif
    mch_signal(SIGILL, SIG_DFL);
    mch_signal(SIGFPE, SIG_DFL);
    mch_signal(SIGSEGV, SIG_DFL);
    mch_signal(SIGTERM, SIG_DFL);
    mch_signal(SIGABRT, SIG_DFL);

    return x;
}

#if defined(FEAT_JOB_CHANNEL) || defined(PROTO)
    static HANDLE
job_io_file_open(
	char_u *fname,
	DWORD dwDesiredAccess,
	DWORD dwShareMode,
	LPSECURITY_ATTRIBUTES lpSecurityAttributes,
	DWORD dwCreationDisposition,
	DWORD dwFlagsAndAttributes)
{
    HANDLE h;
    WCHAR *wn;

    wn = enc_to_utf16(fname, NULL);
    if (wn == NULL)
	return INVALID_HANDLE_VALUE;

    h = CreateFileW(wn, dwDesiredAccess, dwShareMode,
	    lpSecurityAttributes, dwCreationDisposition,
	    dwFlagsAndAttributes, NULL);
    vim_free(wn);
    return h;
}

/*
 * Turn the dictionary "env" into a NUL separated list that can be used as the
 * environment argument of vim_create_process().
 */
    void
win32_build_env(dict_T *env, garray_T *gap, int is_terminal)
{
    hashitem_T	*hi;
    long_u	todo = env != NULL ? env->dv_hashtab.ht_used : 0;
    LPVOID	base = GetEnvironmentStringsW();

    // for last \0
    if (ga_grow(gap, 1) == FAIL)
	return;

    if (env != NULL)
    {
	FOR_ALL_HASHTAB_ITEMS(&env->dv_hashtab, hi, todo)
	{
	    if (!HASHITEM_EMPTY(hi))
	    {
		typval_T *item = &dict_lookup(hi)->di_tv;
		WCHAR   *wkey = enc_to_utf16((char_u *)hi->hi_key, NULL);
		WCHAR   *wval = enc_to_utf16(tv_get_string(item), NULL);
		--todo;
		if (wkey != NULL && wval != NULL)
		{
		    size_t	n;
		    size_t	lkey = wcslen(wkey);
		    size_t	lval = wcslen(wval);

		    if (ga_grow(gap, (int)(lkey + lval + 2)) == FAIL)
			continue;
		    for (n = 0; n < lkey; n++)
			*((WCHAR*)gap->ga_data + gap->ga_len++) = wkey[n];
		    *((WCHAR*)gap->ga_data + gap->ga_len++) = L'=';
		    for (n = 0; n < lval; n++)
			*((WCHAR*)gap->ga_data + gap->ga_len++) = wval[n];
		    *((WCHAR*)gap->ga_data + gap->ga_len++) = L'\0';
		}
		vim_free(wkey);
		vim_free(wval);
	    }
	}
    }

    if (base)
    {
	WCHAR	*p = (WCHAR*) base;

	// for last \0
	if (ga_grow(gap, 1) == FAIL)
	    return;

	while (*p != 0 || *(p + 1) != 0)
	{
	    if (ga_grow(gap, 1) == OK)
		*((WCHAR*)gap->ga_data + gap->ga_len++) = *p;
	    p++;
	}
	FreeEnvironmentStringsW(base);
	*((WCHAR*)gap->ga_data + gap->ga_len++) = L'\0';
    }

# if defined(FEAT_CLIENTSERVER) || defined(FEAT_TERMINAL)
    {
#  ifdef FEAT_CLIENTSERVER
	char_u	*servername = get_vim_var_str(VV_SEND_SERVER);
	size_t	servername_len = STRLEN(servername);
#  endif
#  ifdef FEAT_TERMINAL
	char_u	*version = get_vim_var_str(VV_VERSION);
	size_t	version_len = STRLEN(version);
#  endif
	// size of "VIM_SERVERNAME=" and value,
	// plus "VIM_TERMINAL=" and value,
	// plus two terminating NULs
	size_t	n = 0
#  ifdef FEAT_CLIENTSERVER
		    + 15 + servername_len
#  endif
#  ifdef FEAT_TERMINAL
		    + 13 + version_len + 2
#  endif
		    ;

	if (ga_grow(gap, (int)n) == OK)
	{
#  ifdef FEAT_CLIENTSERVER
	    for (n = 0; n < 15; n++)
		*((WCHAR*)gap->ga_data + gap->ga_len++) =
		    (WCHAR)"VIM_SERVERNAME="[n];
	    for (n = 0; n < servername_len; n++)
		*((WCHAR*)gap->ga_data + gap->ga_len++) =
		    (WCHAR)servername[n];
	    *((WCHAR*)gap->ga_data + gap->ga_len++) = L'\0';
#  endif
#  ifdef FEAT_TERMINAL
	    if (is_terminal)
	    {
		for (n = 0; n < 13; n++)
		    *((WCHAR*)gap->ga_data + gap->ga_len++) =
			(WCHAR)"VIM_TERMINAL="[n];
		for (n = 0; n < version_len; n++)
		    *((WCHAR*)gap->ga_data + gap->ga_len++) =
			(WCHAR)version[n];
		*((WCHAR*)gap->ga_data + gap->ga_len++) = L'\0';
	    }
#  endif
	}
    }
# endif
}

/*
 * Create a pair of pipes.
 * Return TRUE for success, FALSE for failure.
 */
    static BOOL
create_pipe_pair(HANDLE handles[2])
{
    static LONG		s;
    char		name[64];
    SECURITY_ATTRIBUTES sa;

    sprintf(name, "\\\\?\\pipe\\vim-%08lx-%08lx",
	    GetCurrentProcessId(),
	    InterlockedIncrement(&s));

    // Create named pipe. Max size of named pipe is 65535.
    handles[1] = CreateNamedPipe(
	    name,
	    PIPE_ACCESS_OUTBOUND | FILE_FLAG_OVERLAPPED,
	    PIPE_TYPE_BYTE | PIPE_NOWAIT,
	    1, MAX_NAMED_PIPE_SIZE, 0, 0, NULL);

    if (handles[1] == INVALID_HANDLE_VALUE)
	return FALSE;

    sa.nLength = sizeof(sa);
    sa.bInheritHandle = TRUE;
    sa.lpSecurityDescriptor = NULL;

    handles[0] = CreateFile(name,
	    FILE_GENERIC_READ,
	    FILE_SHARE_READ, &sa,
	    OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, 0);

    if (handles[0] == INVALID_HANDLE_VALUE)
    {
	CloseHandle(handles[1]);
	return FALSE;
    }

    return TRUE;
}

    void
mch_job_start(char *cmd, job_T *job, jobopt_T *options)
{
    STARTUPINFO		si;
    PROCESS_INFORMATION	pi;
    HANDLE		jo;
    SECURITY_ATTRIBUTES saAttr;
    channel_T		*channel = NULL;
    HANDLE		ifd[2];
    HANDLE		ofd[2];
    HANDLE		efd[2];
    garray_T		ga;

    int		use_null_for_in = options->jo_io[PART_IN] == JIO_NULL;
    int		use_null_for_out = options->jo_io[PART_OUT] == JIO_NULL;
    int		use_null_for_err = options->jo_io[PART_ERR] == JIO_NULL;
    int		use_file_for_in = options->jo_io[PART_IN] == JIO_FILE;
    int		use_file_for_out = options->jo_io[PART_OUT] == JIO_FILE;
    int		use_file_for_err = options->jo_io[PART_ERR] == JIO_FILE;
    int		use_out_for_err = options->jo_io[PART_ERR] == JIO_OUT;

    if (use_out_for_err && use_null_for_out)
	use_null_for_err = TRUE;

    ifd[0] = INVALID_HANDLE_VALUE;
    ifd[1] = INVALID_HANDLE_VALUE;
    ofd[0] = INVALID_HANDLE_VALUE;
    ofd[1] = INVALID_HANDLE_VALUE;
    efd[0] = INVALID_HANDLE_VALUE;
    efd[1] = INVALID_HANDLE_VALUE;
    ga_init2(&ga, sizeof(wchar_t), 500);

    jo = CreateJobObject(NULL, NULL);
    if (jo == NULL)
    {
	job->jv_status = JOB_FAILED;
	goto failed;
    }

    if (options->jo_env != NULL)
	win32_build_env(options->jo_env, &ga, FALSE);

    ZeroMemory(&pi, sizeof(pi));
    ZeroMemory(&si, sizeof(si));
    si.cb = sizeof(si);
    si.dwFlags |= STARTF_USESHOWWINDOW;
    si.wShowWindow = SW_HIDE;

    saAttr.nLength = sizeof(SECURITY_ATTRIBUTES);
    saAttr.bInheritHandle = TRUE;
    saAttr.lpSecurityDescriptor = NULL;

    if (use_file_for_in)
    {
	char_u *fname = options->jo_io_name[PART_IN];

	ifd[0] = job_io_file_open(fname, GENERIC_READ,
		FILE_SHARE_READ | FILE_SHARE_WRITE,
		&saAttr, OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL);
	if (ifd[0] == INVALID_HANDLE_VALUE)
	{
	    semsg(_(e_cant_open_file_str), fname);
	    goto failed;
	}
    }
    else if (!use_null_for_in
	    && (!create_pipe_pair(ifd)
		|| !SetHandleInformation(ifd[1], HANDLE_FLAG_INHERIT, 0)))
	goto failed;

    if (use_file_for_out)
    {
	char_u *fname = options->jo_io_name[PART_OUT];

	ofd[1] = job_io_file_open(fname, GENERIC_WRITE,
		FILE_SHARE_READ | FILE_SHARE_WRITE,
		&saAttr, CREATE_ALWAYS, FILE_ATTRIBUTE_NORMAL);
	if (ofd[1] == INVALID_HANDLE_VALUE)
	{
	    semsg(_(e_cant_open_file_str), fname);
	    goto failed;
	}
    }
    else if (!use_null_for_out &&
	    (!CreatePipe(&ofd[0], &ofd[1], &saAttr, 0)
	    || !SetHandleInformation(ofd[0], HANDLE_FLAG_INHERIT, 0)))
	goto failed;

    if (use_file_for_err)
    {
	char_u *fname = options->jo_io_name[PART_ERR];

	efd[1] = job_io_file_open(fname, GENERIC_WRITE,
		FILE_SHARE_READ | FILE_SHARE_WRITE,
		&saAttr, CREATE_ALWAYS, FILE_ATTRIBUTE_NORMAL);
	if (efd[1] == INVALID_HANDLE_VALUE)
	{
	    semsg(_(e_cant_open_file_str), fname);
	    goto failed;
	}
    }
    else if (!use_out_for_err && !use_null_for_err &&
	    (!CreatePipe(&efd[0], &efd[1], &saAttr, 0)
	    || !SetHandleInformation(efd[0], HANDLE_FLAG_INHERIT, 0)))
	goto failed;

    si.dwFlags |= STARTF_USESTDHANDLES;
    si.hStdInput = ifd[0];
    si.hStdOutput = ofd[1];
    si.hStdError = use_out_for_err ? ofd[1] : efd[1];

    if (!use_null_for_in || !use_null_for_out || !use_null_for_err)
    {
	if (options->jo_set & JO_CHANNEL)
	{
	    channel = options->jo_channel;
	    if (channel != NULL)
		++channel->ch_refcount;
	}
	else
	    channel = add_channel();
	if (channel == NULL)
	    goto failed;
    }

    if (!vim_create_process(cmd, TRUE,
	    CREATE_SUSPENDED |
	    CREATE_DEFAULT_ERROR_MODE |
	    CREATE_NEW_PROCESS_GROUP |
	    CREATE_UNICODE_ENVIRONMENT |
	    CREATE_NEW_CONSOLE,
	    &si, &pi,
	    ga.ga_data,
	    (char *)options->jo_cwd))
    {
	CloseHandle(jo);
	job->jv_status = JOB_FAILED;
	goto failed;
    }

    ga_clear(&ga);

    if (!AssignProcessToJobObject(jo, pi.hProcess))
    {
	// if failing, switch the way to terminate
	// process with TerminateProcess.
	CloseHandle(jo);
	jo = NULL;
    }
    ResumeThread(pi.hThread);
    CloseHandle(pi.hThread);
    job->jv_proc_info = pi;
    job->jv_job_object = jo;
    job->jv_status = JOB_STARTED;

    CloseHandle(ifd[0]);
    CloseHandle(ofd[1]);
    if (!use_out_for_err && !use_null_for_err)
	CloseHandle(efd[1]);

    job->jv_channel = channel;
    if (channel != NULL)
    {
	channel_set_pipes(channel,
		      use_file_for_in || use_null_for_in
					      ? INVALID_FD : (sock_T)ifd[1],
		      use_file_for_out || use_null_for_out
					     ? INVALID_FD : (sock_T)ofd[0],
		      use_out_for_err || use_file_for_err || use_null_for_err
					    ? INVALID_FD : (sock_T)efd[0]);
	channel_set_job(channel, job, options);
    }
    return;

failed:
    CloseHandle(ifd[0]);
    CloseHandle(ofd[0]);
    CloseHandle(efd[0]);
    CloseHandle(ifd[1]);
    CloseHandle(ofd[1]);
    CloseHandle(efd[1]);
    channel_unref(channel);
    ga_clear(&ga);
}

    char *
mch_job_status(job_T *job)
{
    DWORD dwExitCode = 0;

    if (!GetExitCodeProcess(job->jv_proc_info.hProcess, &dwExitCode)
	    || dwExitCode != STILL_ACTIVE)
    {
	job->jv_exitval = (int)dwExitCode;
	if (job->jv_status < JOB_ENDED)
	{
	    ch_log(job->jv_channel, "Job ended");
	    job->jv_status = JOB_ENDED;
	}
	return "dead";
    }
    return "run";
}

    job_T *
mch_detect_ended_job(job_T *job_list)
{
    HANDLE jobHandles[MAXIMUM_WAIT_OBJECTS];
    job_T *jobArray[MAXIMUM_WAIT_OBJECTS];
    job_T *job = job_list;

    while (job != NULL)
    {
	DWORD n;
	DWORD result;

	for (n = 0; n < MAXIMUM_WAIT_OBJECTS
				       && job != NULL; job = job->jv_next)
	{
	    if (job->jv_status == JOB_STARTED)
	    {
		jobHandles[n] = job->jv_proc_info.hProcess;
		jobArray[n] = job;
		++n;
	    }
	}
	if (n == 0)
	    continue;
	result = WaitForMultipleObjects(n, jobHandles, FALSE, 0);
	if (result >= WAIT_OBJECT_0 && result < WAIT_OBJECT_0 + n)
	{
	    job_T *wait_job = jobArray[result - WAIT_OBJECT_0];

	    if (STRCMP(mch_job_status(wait_job), "dead") == 0)
		return wait_job;
	}
    }
    return NULL;
}

    static BOOL
terminate_all(HANDLE process, int code)
{
    PROCESSENTRY32  pe;
    HANDLE	    h = INVALID_HANDLE_VALUE;
    DWORD	    pid = GetProcessId(process);

    if (pid != 0)
    {
	h = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
	if (h != INVALID_HANDLE_VALUE)
	{
	    pe.dwSize = sizeof(PROCESSENTRY32);
	    if (!Process32First(h, &pe))
		goto theend;

	    do
	    {
		if (pe.th32ParentProcessID == pid)
		{
		    HANDLE ph = OpenProcess(
				  PROCESS_ALL_ACCESS, FALSE, pe.th32ProcessID);
		    if (ph != NULL)
		    {
			terminate_all(ph, code);
			CloseHandle(ph);
		    }
		}
	    } while (Process32Next(h, &pe));

	    CloseHandle(h);
	}
    }

theend:
    return TerminateProcess(process, code);
}

/*
 * Send a (deadly) signal to "job".
 * Return FAIL if it didn't work.
 */
    int
mch_signal_job(job_T *job, char_u *how)
{
    int ret;

    if (STRCMP(how, "term") == 0 || STRCMP(how, "kill") == 0 || *how == NUL)
    {
	// deadly signal
	if (job->jv_job_object != NULL)
	{
	    if (job->jv_channel != NULL && job->jv_channel->ch_anonymous_pipe)
		job->jv_channel->ch_killing = TRUE;
	    return TerminateJobObject(job->jv_job_object, (UINT)-1) ? OK : FAIL;
	}
	return terminate_all(job->jv_proc_info.hProcess, -1) ? OK : FAIL;
    }

    if (!AttachConsole(job->jv_proc_info.dwProcessId))
	return FAIL;
    ret = GenerateConsoleCtrlEvent(
		STRCMP(how, "int") == 0 ? CTRL_C_EVENT : CTRL_BREAK_EVENT,
		job->jv_proc_info.dwProcessId)
	    ? OK : FAIL;
    FreeConsole();
    return ret;
}

/*
 * Clear the data related to "job".
 */
    void
mch_clear_job(job_T *job)
{
    if (job->jv_status == JOB_FAILED)
	return;

    if (job->jv_job_object != NULL)
	CloseHandle(job->jv_job_object);
    CloseHandle(job->jv_proc_info.hProcess);
}
#endif


#if !defined(FEAT_GUI_MSWIN) || defined(VIMDLL)

/*
 * Start termcap mode
 */
    static void
termcap_mode_start(void)
{
    DWORD cmodein;

    if (g_fTermcapMode)
	return;

    SaveConsoleBuffer(&g_cbNonTermcap);

    if (g_cbTermcap.IsValid)
    {
	/*
	 * We've been in termcap mode before.  Restore certain screen
	 * characteristics, including the buffer size and the window
	 * size.  Since we will be redrawing the screen, we don't need
	 * to restore the actual contents of the buffer.
	 */
	RestoreConsoleBuffer(&g_cbTermcap, FALSE);
	reset_console_color_rgb();
	SetConsoleWindowInfo(g_hConOut, TRUE, &g_cbTermcap.Info.srWindow);
	Rows = g_cbTermcap.Info.dwSize.Y;
	Columns = g_cbTermcap.Info.dwSize.X;
    }
    else
    {
	/*
	 * This is our first time entering termcap mode.  Clear the console
	 * screen buffer, and resize the buffer to match the current window
	 * size.  We will use this as the size of our editing environment.
	 */
	ClearConsoleBuffer(g_attrCurrent);
	set_console_color_rgb();
	ResizeConBufAndWindow(g_hConOut, Columns, Rows);
    }

    resettitle();

    GetConsoleMode(g_hConIn, &cmodein);
    if (g_fMouseActive)
    {
	cmodein |= ENABLE_MOUSE_INPUT;
	cmodein &= ~ENABLE_QUICK_EDIT_MODE;
    }
    else
    {
	cmodein &= ~ENABLE_MOUSE_INPUT;
	cmodein |= g_cmodein & ENABLE_QUICK_EDIT_MODE;
    }
    cmodein |= ENABLE_WINDOW_INPUT;
    SetConsoleMode(g_hConIn, cmodein | ENABLE_EXTENDED_FLAGS);

    redraw_later_clear();
    g_fTermcapMode = TRUE;
}


/*
 * End termcap mode
 */
    static void
termcap_mode_end(void)
{
    DWORD cmodein;
    ConsoleBuffer *cb;
    COORD coord;
    DWORD dwDummy;

    if (!g_fTermcapMode)
	return;

    SaveConsoleBuffer(&g_cbTermcap);

    GetConsoleMode(g_hConIn, &cmodein);
    cmodein &= ~(ENABLE_MOUSE_INPUT | ENABLE_WINDOW_INPUT);
    cmodein |= g_cmodein & ENABLE_QUICK_EDIT_MODE;
    SetConsoleMode(g_hConIn, cmodein | ENABLE_EXTENDED_FLAGS);

# ifdef FEAT_RESTORE_ORIG_SCREEN
    cb = exiting ? &g_cbOrig : &g_cbNonTermcap;
# else
    cb = &g_cbNonTermcap;
# endif
    RestoreConsoleBuffer(cb, p_rs);
    restore_console_color_rgb();

    // Switch back to main screen buffer.
    if (exiting && use_alternate_screen_buffer)
	vtp_printf("\033[?1049l");

    if (!USE_WT && (p_rs || exiting))
    {
	/*
	 * Clear anything that happens to be on the current line.
	 */
	coord.X = 0;
	coord.Y = (SHORT) (p_rs ? cb->Info.dwCursorPosition.Y : (Rows - 1));
	FillConsoleOutputCharacter(g_hConOut, ' ',
		cb->Info.dwSize.X, coord, &dwDummy);
	/*
	 * The following is just for aesthetics.  If we are exiting without
	 * restoring the screen, then we want to have a prompt string
	 * appear at the bottom line.  However, the command interpreter
	 * seems to always advance the cursor one line before displaying
	 * the prompt string, which causes the screen to scroll.  To
	 * counter this, move the cursor up one line before exiting.
	 */
	if (exiting && !p_rs)
	    coord.Y--;
	/*
	 * Position the cursor at the leftmost column of the desired row.
	 */
	SetConsoleCursorPosition(g_hConOut, coord);
    }
    SetConsoleCursorInfo(g_hConOut, &g_cci);
    g_fTermcapMode = FALSE;
}
#endif // !FEAT_GUI_MSWIN || VIMDLL


#if defined(FEAT_GUI_MSWIN) && !defined(VIMDLL)
    void
mch_write(
    char_u  *s UNUSED,
    int	    len UNUSED)
{
    // never used
}

#else

/*
 * clear `n' chars, starting from `coord'
 */
    static void
clear_chars(
    COORD coord,
    DWORD n)
{
    if (!vtp_working)
    {
	DWORD dwDummy;

	FillConsoleOutputCharacter(g_hConOut, ' ', n, coord, &dwDummy);
	FillConsoleOutputAttribute(g_hConOut, g_attrCurrent, n, coord,
								     &dwDummy);
    }
    else
    {
	set_console_color_rgb();
	gotoxy(coord.X + 1, coord.Y + 1);
	vtp_printf("\033[%dX", n);
    }
}


/*
 * Clear the screen
 */
    static void
clear_screen(void)
{
    g_coord.X = g_coord.Y = 0;

    if (!vtp_working)
	clear_chars(g_coord, Rows * Columns);
    else
    {
	set_console_color_rgb();
	gotoxy(1, 1);
	vtp_printf("\033[2J");
    }
}


/*
 * Clear to end of display
 */
    static void
clear_to_end_of_display(void)
{
    COORD save = g_coord;

    if (!vtp_working)
	clear_chars(g_coord, (Rows - g_coord.Y - 1)
					   * Columns + (Columns - g_coord.X));
    else
    {
	set_console_color_rgb();
	gotoxy(g_coord.X + 1, g_coord.Y + 1);
	vtp_printf("\033[0J");

	gotoxy(save.X + 1, save.Y + 1);
	g_coord = save;
    }
}


/*
 * Clear to end of line
 */
    static void
clear_to_end_of_line(void)
{
    COORD save = g_coord;

    if (!vtp_working)
	clear_chars(g_coord, Columns - g_coord.X);
    else
    {
	set_console_color_rgb();
	gotoxy(g_coord.X + 1, g_coord.Y + 1);
	vtp_printf("\033[0K");

	gotoxy(save.X + 1, save.Y + 1);
	g_coord = save;
    }
}


/*
 * Scroll the scroll region up by `cLines' lines
 */
    static void
scroll(unsigned cLines)
{
    COORD oldcoord = g_coord;

    gotoxy(g_srScrollRegion.Left + 1, g_srScrollRegion.Top + 1);
    delete_lines(cLines);

    g_coord = oldcoord;
}


/*
 * Set the scroll region
 */
    static void
set_scroll_region(
    unsigned left,
    unsigned top,
    unsigned right,
    unsigned bottom)
{
    if (left >= right
	    || top >= bottom
	    || right > (unsigned) Columns - 1
	    || bottom > (unsigned) Rows - 1)
	return;

    g_srScrollRegion.Left =   left;
    g_srScrollRegion.Top =    top;
    g_srScrollRegion.Right =  right;
    g_srScrollRegion.Bottom = bottom;
}

    static void
set_scroll_region_tb(
    unsigned top,
    unsigned bottom)
{
    if (top >= bottom || bottom > (unsigned)Rows - 1)
	return;

    g_srScrollRegion.Top = top;
    g_srScrollRegion.Bottom = bottom;
}

    static void
set_scroll_region_lr(
    unsigned left,
    unsigned right)
{
    if (left >= right || right > (unsigned)Columns - 1)
	return;

    g_srScrollRegion.Left = left;
    g_srScrollRegion.Right = right;
}


/*
 * Insert `cLines' lines at the current cursor position
 */
    static void
insert_lines(unsigned cLines)
{
    SMALL_RECT	    source, clip;
    COORD	    dest;
    CHAR_INFO	    fill;

    gotoxy(g_srScrollRegion.Left + 1, g_srScrollRegion.Top + 1);

    dest.X = g_srScrollRegion.Left;
    dest.Y = g_coord.Y + cLines;

    source.Left   = g_srScrollRegion.Left;
    source.Top	  = g_coord.Y;
    source.Right  = g_srScrollRegion.Right;
    source.Bottom = g_srScrollRegion.Bottom - cLines;

    clip.Left   = g_srScrollRegion.Left;
    clip.Top    = g_coord.Y;
    clip.Right  = g_srScrollRegion.Right;
    clip.Bottom = g_srScrollRegion.Bottom;

    fill.Char.AsciiChar = ' ';
    if (!USE_VTP)
	fill.Attributes = g_attrCurrent;
    else
	fill.Attributes = g_attrDefault;

    set_console_color_rgb();

    ScrollConsoleScreenBuffer(g_hConOut, &source, &clip, dest, &fill);

    // Here we have to deal with a win32 console flake: If the scroll
    // region looks like abc and we scroll c to a and fill with d we get
    // cbd... if we scroll block c one line at a time to a, we get cdd...
    // vim expects cdd consistently... So we have to deal with that
    // here... (this also occurs scrolling the same way in the other
    // direction).

    if (source.Bottom < dest.Y)
    {
	COORD coord;
	int   i;

	coord.X = source.Left;
	for (i = clip.Top; i < dest.Y; ++i)
	{
	    coord.Y = i;
	    clear_chars(coord, source.Right - source.Left + 1);
	}
    }

    if (vtp_working)
    {
	COORD coord;
	int i;

	coord.X = source.Left;
	for (i = source.Top; i < dest.Y; ++i)
	{
	    coord.Y = i;
	    clear_chars(coord, source.Right - source.Left + 1);
	}
    }
}


/*
 * Delete `cLines' lines at the current cursor position
 */
    static void
delete_lines(unsigned cLines)
{
    SMALL_RECT	    source, clip;
    COORD	    dest;
    CHAR_INFO	    fill;
    int		    nb;

    gotoxy(g_srScrollRegion.Left + 1, g_srScrollRegion.Top + 1);

    dest.X = g_srScrollRegion.Left;
    dest.Y = g_coord.Y;

    source.Left   = g_srScrollRegion.Left;
    source.Top	  = g_coord.Y + cLines;
    source.Right  = g_srScrollRegion.Right;
    source.Bottom = g_srScrollRegion.Bottom;

    clip.Left   = g_srScrollRegion.Left;
    clip.Top    = g_coord.Y;
    clip.Right  = g_srScrollRegion.Right;
    clip.Bottom = g_srScrollRegion.Bottom;

    fill.Char.AsciiChar = ' ';
    if (!vtp_working)
	fill.Attributes = g_attrCurrent;
    else
	fill.Attributes = g_attrDefault;

    set_console_color_rgb();

    ScrollConsoleScreenBuffer(g_hConOut, &source, &clip, dest, &fill);

    // Here we have to deal with a win32 console flake; See insert_lines()
    // above.

    nb = dest.Y + (source.Bottom - source.Top) + 1;

    if (nb < source.Top)
    {
	COORD coord;
	int   i;

	coord.X = source.Left;
	for (i = nb; i < clip.Bottom; ++i)
	{
	    coord.Y = i;
	    clear_chars(coord, source.Right - source.Left + 1);
	}
    }

    if (vtp_working)
    {
	COORD coord;
	int i;

	coord.X = source.Left;
	for (i = nb; i <= source.Bottom; ++i)
	{
	    coord.Y = i;
	    clear_chars(coord, source.Right - source.Left + 1);
	}
    }
}


/*
 * Set the cursor position to (x,y) (1-based).
 */
    static void
gotoxy(
    unsigned x,
    unsigned y)
{
    if (x < 1 || x > (unsigned)Columns || y < 1 || y > (unsigned)Rows)
	return;

    if (!USE_VTP)
    {
	// There are reports of double-width characters not displayed
	// correctly.  This workaround should fix it, similar to how it's done
	// for VTP.
	g_coord.X = 0;
	SetConsoleCursorPosition(g_hConOut, g_coord);

	// external cursor coords are 1-based; internal are 0-based
	g_coord.X = x - 1;
	g_coord.Y = y - 1;
	SetConsoleCursorPosition(g_hConOut, g_coord);
    }
    else
    {
	// Move the cursor to the left edge of the screen to prevent screen
	// destruction.  Insider build bug.  Always enabled because it's cheap
	// and avoids mistakes with recognizing the build.
	vtp_printf("\033[%d;%dH", g_coord.Y + 1, 1);

	vtp_printf("\033[%d;%dH", y, x);

	g_coord.X = x - 1;
	g_coord.Y = y - 1;
    }
}


/*
 * Set the current text attribute = (foreground | background)
 * See ../runtime/doc/os_win32.txt for the numbers.
 */
    static void
textattr(WORD wAttr)
{
    g_attrCurrent = wAttr & 0xff;

    SetConsoleTextAttribute(g_hConOut, wAttr);
}


    static void
textcolor(WORD wAttr)
{
    g_attrCurrent = (g_attrCurrent & 0xf0) + (wAttr & 0x0f);

    if (!vtp_working)
	SetConsoleTextAttribute(g_hConOut, g_attrCurrent);
    else
	vtp_sgr_bulk(wAttr);
}


    static void
textbackground(WORD wAttr)
{
    g_attrCurrent = (g_attrCurrent & 0x0f) + ((wAttr & 0x0f) << 4);

    if (!vtp_working)
	SetConsoleTextAttribute(g_hConOut, g_attrCurrent);
    else
	vtp_sgr_bulk(wAttr);
}


/*
 * restore the default text attribute (whatever we started with)
 */
    static void
normvideo(void)
{
    if (!vtp_working)
	textattr(g_attrDefault);
    else
	vtp_sgr_bulk(0);
}


static WORD g_attrPreStandout = 0;

/*
 * Make the text standout, by brightening it
 */
    static void
standout(void)
{
    g_attrPreStandout = g_attrCurrent;

    textattr((WORD) (g_attrCurrent|FOREGROUND_INTENSITY|BACKGROUND_INTENSITY));
}


/*
 * Turn off standout mode
 */
    static void
standend(void)
{
    if (g_attrPreStandout)
	textattr(g_attrPreStandout);

    g_attrPreStandout = 0;
}


/*
 * Set normal fg/bg color, based on T_ME.  Called when t_me has been set.
 */
    void
mch_set_normal_colors(void)
{
    char_u	*p;
    int		n;

    cterm_normal_fg_color = (g_attrDefault & 0xf) + 1;
    cterm_normal_bg_color = ((g_attrDefault >> 4) & 0xf) + 1;
    if (
# ifdef FEAT_TERMGUICOLORS
	!p_tgc &&
# endif
	T_ME[0] == ESC && T_ME[1] == '|')
    {
	p = T_ME + 2;
	n = getdigits(&p);
	if (*p == 'm' && n > 0)
	{
	    cterm_normal_fg_color = (n & 0xf) + 1;
	    cterm_normal_bg_color = ((n >> 4) & 0xf) + 1;
	}
    }
# ifdef FEAT_TERMGUICOLORS
    cterm_normal_fg_gui_color = INVALCOLOR;
    cterm_normal_bg_gui_color = INVALCOLOR;
# endif
}


/*
 * visual bell: flash the screen
 */
    static void
visual_bell(void)
{
    COORD   coordOrigin = {0, 0};
    WORD    attrFlash = ~g_attrCurrent & 0xff;

    DWORD   dwDummy;
    LPWORD  oldattrs = NULL;

# ifdef FEAT_TERMGUICOLORS
    if (!(p_tgc || t_colors >= 256))
# endif
    {
	oldattrs = ALLOC_MULT(WORD, Rows * Columns);
	if (oldattrs == NULL)
	    return;
	ReadConsoleOutputAttribute(g_hConOut, oldattrs, Rows * Columns,
			       coordOrigin, &dwDummy);
    }

    FillConsoleOutputAttribute(g_hConOut, attrFlash, Rows * Columns,
			       coordOrigin, &dwDummy);

    Sleep(15);	    // wait for 15 msec

    if (oldattrs != NULL)
    {
	WriteConsoleOutputAttribute(g_hConOut, oldattrs, Rows * Columns,
				coordOrigin, &dwDummy);
	vim_free(oldattrs);
    }
}


/*
 * Make the cursor visible or invisible
 */
    static void
cursor_visible(BOOL fVisible)
{
    s_cursor_visible = fVisible;

    if (vtp_working)
	vtp_printf("\033[?25%c", fVisible ? 'h' : 'l');

# ifdef MCH_CURSOR_SHAPE
    mch_update_cursor();
# endif
}


/*
 * Write "cbToWrite" bytes in `pchBuf' to the screen.
 * Returns the number of bytes actually written (at least one).
 */
    static DWORD
write_chars(
    char_u *pchBuf,
    DWORD  cbToWrite)
{
    COORD	    coord = g_coord;
    DWORD	    written;
    DWORD	    n, cchwritten;
    static DWORD    cells;
    static WCHAR    *unicodebuf = NULL;
    static int	    unibuflen = 0;
    static int	    length;
    int		    cp = enc_utf8 ? CP_UTF8 : enc_codepage;
    static WCHAR    *utf8spbuf = NULL;
    static int	    utf8splength;
    static DWORD    utf8spcells;
    static WCHAR    **utf8usingbuf = &unicodebuf;

    if (cbToWrite != 1 || *pchBuf != ' ' || !enc_utf8)
    {
	utf8usingbuf = &unicodebuf;
	do
	{
	    length = MultiByteToWideChar(cp, 0, (LPCSTR)pchBuf, cbToWrite,
							unicodebuf, unibuflen);
	    if (length && length <= unibuflen)
		break;
	    vim_free(unicodebuf);
	    unicodebuf = length ? LALLOC_MULT(WCHAR, length) : NULL;
	    unibuflen = unibuflen ? 0 : length;
	} while (TRUE);
	cells = mb_string2cells(pchBuf, cbToWrite);
    }
    else // cbToWrite == 1 && *pchBuf == ' ' && enc_utf8
    {
	if (utf8usingbuf != &utf8spbuf)
	{
	    if (utf8spbuf == NULL)
	    {
		cells = mb_string2cells((char_u *)" ", 1);
		length = MultiByteToWideChar(CP_UTF8, 0, " ", 1, NULL, 0);
		utf8spbuf = LALLOC_MULT(WCHAR, length);
		if (utf8spbuf != NULL)
		{
		    MultiByteToWideChar(CP_UTF8, 0, " ", 1, utf8spbuf, length);
		    utf8usingbuf = &utf8spbuf;
		    utf8splength = length;
		    utf8spcells = cells;
		}
	    }
	    else
	    {
		utf8usingbuf = &utf8spbuf;
		length = utf8splength;
		cells = utf8spcells;
	    }
	}
    }

    if (!USE_VTP)
    {
	FillConsoleOutputAttribute(g_hConOut, g_attrCurrent, cells,
				    coord, &written);
	// When writing fails or didn't write a single character, pretend one
	// character was written, otherwise we get stuck.
	if (WriteConsoleOutputCharacterW(g_hConOut, *utf8usingbuf, length,
		    coord, &cchwritten) == 0
		|| cchwritten == 0 || cchwritten == (DWORD)-1)
	    cchwritten = 1;
    }
    else
    {
	if (WriteConsoleW(g_hConOut, *utf8usingbuf, length, &cchwritten,
		    NULL) == 0 || cchwritten == 0)
	    cchwritten = 1;
    }

    if (cchwritten == (DWORD)length)
    {
	written = cbToWrite;
	g_coord.X += (SHORT)cells;
    }
    else
    {
	char_u *p = pchBuf;
	for (n = 0; n < cchwritten; n++)
	    MB_CPTR_ADV(p);
	written = p - pchBuf;
	g_coord.X += (SHORT)mb_string2cells(pchBuf, written);
    }

    while (g_coord.X > g_srScrollRegion.Right)
    {
	g_coord.X -= (SHORT) Columns;
	if (g_coord.Y < g_srScrollRegion.Bottom)
	    g_coord.Y++;
    }

    // Cursor under VTP is always in the correct position, no need to reset.
    if (!USE_VTP)
	gotoxy(g_coord.X + 1, g_coord.Y + 1);

    return written;
}

    static char_u *
get_seq(
    int *args,
    int *count,
    char_u *head)
{
    int argc;
    char_u *p;

    if (head == NULL || *head != '\033')
	return NULL;

    argc = 0;
    p = head;
    ++p;
    do
    {
	++p;
	args[argc] = getdigits(&p);
	argc += (argc < 15) ? 1 : 0;
    } while (*p == ';');
    *count = argc;

    return p;
}

    static char_u *
get_sgr(
    int *args,
    int *count,
    char_u *head)
{
    char_u *p = get_seq(args, count, head);

    return (p && *p == 'm') ? ++p : NULL;
}

/*
 * Pointer to next if SGR (^[[n;2;*;*;*m), NULL otherwise.
 */
    static char_u *
sgrn2(
    char_u *head,
    int	   n)
{
    int argc;
    int args[16];
    char_u *p = get_sgr(args, &argc, head);

    return p && argc == 5 && args[0] == n && args[1] == 2 ? p : NULL;
}

/*
 * Pointer to next if SGR(^[[nm)<space>ESC, NULL otherwise.
 */
    static char_u *
sgrnc(
    char_u *head,
    int	   n)
{
    int argc;
    int args[16];
    char_u *p = get_sgr(args, &argc, head);

    return p && argc == 1 && args[0] == n && (p = skipwhite(p)) && *p == '\033'
								    ? p : NULL;
}

    static char_u *
skipblank(char_u *q)
{
    char_u *p = q;

    while (*p == ' ' || *p == '\t' || *p == '\n' || *p == '\r')
	++p;
    return p;
}

/*
 * Pointer to the next if any whitespace that may follow SGR is ESC, otherwise
 * NULL.
 */
    static char_u *
sgrn2c(
    char_u *head,
    int	   n)
{
    char_u *p = sgrn2(head, n);

    return p && *p != NUL && (p = skipblank(p)) && *p == '\033' ? p : NULL;
}

/*
 * If there is only a newline between the sequence immediately following it,
 * a pointer to the character following the newline is returned.
 * Otherwise NULL.
 */
    static char_u *
sgrn2cn(
    char_u *head,
    int n)
{
    char_u *p = sgrn2(head, n);

    return p && p[0] == 0x0a && p[1] == '\033' ? ++p : NULL;
}

/*
 * mch_write(): write the output buffer to the screen, translating ESC
 * sequences into calls to console output routines.
 */
    void
mch_write(
    char_u  *s,
    int	    len)
{
    char_u  *end = s + len;

# ifdef VIMDLL
    if (gui.in_use)
	return;
# endif

    if (!term_console)
    {
	write(1, s, (unsigned)len);
	return;
    }

    // translate ESC | sequences into faked bios calls
    while (len--)
    {
	int prefix = -1;
	char_u ch;

	// While processing a sequence, on rare occasions it seems that another
	// sequence may be inserted asynchronously.
	if (len < 0)
	{
	    redraw_all_later(UPD_CLEAR);
	    return;
	}

	while (s + ++prefix < end)
	{
	    ch = s[prefix];
	    if (ch <= 0x1e && !(ch != '\n' && ch != '\r' && ch != '\b'
						&& ch != '\a' && ch != '\033'))
		break;
	}

	if (p_wd)
	{
	    WaitForChar(p_wd, FALSE);
	    if (prefix != 0)
		prefix = 1;
	}

	if (prefix != 0)
	{
	    DWORD nWritten;

	    nWritten = write_chars(s, prefix);
# ifdef MCH_WRITE_DUMP
	    if (fdDump)
	    {
		fputc('>', fdDump);
		fwrite(s, sizeof(char_u), nWritten, fdDump);
		fputs("<\n", fdDump);
	    }
# endif
	    len -= (nWritten - 1);
	    s += nWritten;
	}
	else if (s[0] == '\n')
	{
	    // \n, newline: go to the beginning of the next line or scroll
	    if (g_coord.Y == g_srScrollRegion.Bottom)
	    {
		scroll(1);
		gotoxy(g_srScrollRegion.Left + 1, g_srScrollRegion.Bottom + 1);
	    }
	    else
	    {
		gotoxy(g_srScrollRegion.Left + 1, g_coord.Y + 2);
	    }
# ifdef MCH_WRITE_DUMP
	    if (fdDump)
		fputs("\\n\n", fdDump);
# endif
	    s++;
	}
	else if (s[0] == '\r')
	{
	    // \r, carriage return: go to beginning of line
	    gotoxy(g_srScrollRegion.Left+1, g_coord.Y + 1);
# ifdef MCH_WRITE_DUMP
	    if (fdDump)
		fputs("\\r\n", fdDump);
# endif
	    s++;
	}
	else if (s[0] == '\b')
	{
	    // \b, backspace: move cursor one position left
	    if (g_coord.X > g_srScrollRegion.Left)
		g_coord.X--;
	    else if (g_coord.Y > g_srScrollRegion.Top)
	    {
		g_coord.X = g_srScrollRegion.Right;
		g_coord.Y--;
	    }
	    gotoxy(g_coord.X + 1, g_coord.Y + 1);
# ifdef MCH_WRITE_DUMP
	    if (fdDump)
		fputs("\\b\n", fdDump);
# endif
	    s++;
	}
	else if (s[0] == '\a')
	{
	    // \a, bell
	    MessageBeep(0xFFFFFFFF);
# ifdef MCH_WRITE_DUMP
	    if (fdDump)
		fputs("\\a\n", fdDump);
# endif
	    s++;
	}
	else if (s[0] == ESC && len >= 3-1 && s[1] == '|')
	{
# ifdef MCH_WRITE_DUMP
	    char_u  *old_s = s;
# endif
	    char_u  *p;
	    int	    arg1 = 0, arg2 = 0, argc = 0, args[16];
	    char_u  *sp;

	    switch (s[2])
	    {
	    case '0': case '1': case '2': case '3': case '4':
	    case '5': case '6': case '7': case '8': case '9':
		if (*(p = get_seq(args, &argc, s)) != 'm')
		    goto notsgr;

		p = s;

		// Handling frequent optional sequences.  Output to the screen
		// takes too long, so do not output as much as possible.

		// If resetFG,FG,BG,<cr>,BG,FG are connected, the preceding
		// resetFG,FG,BG are omitted.
		if (sgrn2(sgrn2(sgrn2cn(sgrn2(sgrnc(p, 39), 38), 48), 48), 38))
		{
		    p = sgrn2(sgrn2(sgrnc(p, 39), 38), 48);
		    len = len + 1 - (int)(p - s);
		    s = p;
		    break;
		}

		// If FG,BG,BG,FG of SGR are connected, the first FG can be
		// omitted.
		if (sgrn2(sgrn2(sgrn2c((sp = sgrn2(p, 38)), 48), 48), 38))
		    p = sp;

		// If FG,BG,FG,BG of SGR are connected, the first FG can be
		// omitted.
		if (sgrn2(sgrn2(sgrn2c((sp = sgrn2(p, 38)), 48), 38), 48))
		    p = sp;

		// If BG,BG of SGR are connected, the first BG can be omitted.
		if (sgrn2((sp = sgrn2(p, 48)), 48))
		    p = sp;

		// If restoreFG and FG are connected, the restoreFG can be
		// omitted.
		if (sgrn2((sp = sgrnc(p, 39)), 38))
		    p = sp;

		p = get_seq(args, &argc, p);

notsgr:
		arg1 = args[0];
		arg2 = args[1];
		if (*p == 'm')
		{
		    if (argc == 1 && args[0] == 0)
			normvideo();
		    else if (argc == 1)
		    {
			if (USE_VTP)
			    textcolor((WORD)arg1);
			else
			    textattr((WORD)arg1);
		    }
		    else if (vtp_working)
			vtp_sgr_bulks(argc, args);
		}
		else if (argc == 2 && *p == 'H')
		{
		    gotoxy(arg2, arg1);
		}
		else if (argc == 2 && *p == 'r')
		{
		    set_scroll_region(0, arg1 - 1, Columns - 1, arg2 - 1);
		}
		else if (argc == 2 && *p == 'R')
		{
		    set_scroll_region_tb(arg1, arg2);
		}
		else if (argc == 2 && *p == 'V')
		{
		    set_scroll_region_lr(arg1, arg2);
		}
		else if (argc == 1 && *p == 'A')
		{
		    gotoxy(g_coord.X + 1,
			   max(g_srScrollRegion.Top, g_coord.Y - arg1) + 1);
		}
		else if (argc == 1 && *p == 'b')
		{
		    textbackground((WORD) arg1);
		}
		else if (argc == 1 && *p == 'C')
		{
		    gotoxy(min(g_srScrollRegion.Right, g_coord.X + arg1) + 1,
			   g_coord.Y + 1);
		}
		else if (argc == 1 && *p == 'f')
		{
		    textcolor((WORD) arg1);
		}
		else if (argc == 1 && *p == 'H')
		{
		    gotoxy(1, arg1);
		}
		else if (argc == 1 && *p == 'L')
		{
		    insert_lines(arg1);
		}
		else if (argc == 1 && *p == 'M')
		{
		    delete_lines(arg1);
		}

		len -= (int)(p - s);
		s = p + 1;
		break;

	    case 'A':
		gotoxy(g_coord.X + 1,
		       max(g_srScrollRegion.Top, g_coord.Y - 1) + 1);
		goto got3;

	    case 'B':
		visual_bell();
		goto got3;

	    case 'C':
		gotoxy(min(g_srScrollRegion.Right, g_coord.X + 1) + 1,
		       g_coord.Y + 1);
		goto got3;

	    case 'E':
		termcap_mode_end();
		goto got3;

	    case 'F':
		standout();
		goto got3;

	    case 'f':
		standend();
		goto got3;

	    case 'H':
		gotoxy(1, 1);
		goto got3;

	    case 'j':
		clear_to_end_of_display();
		goto got3;

	    case 'J':
		clear_screen();
		goto got3;

	    case 'K':
		clear_to_end_of_line();
		goto got3;

	    case 'L':
		insert_lines(1);
		goto got3;

	    case 'M':
		delete_lines(1);
		goto got3;

	    case 'S':
		termcap_mode_start();
		goto got3;

	    case 'V':
		cursor_visible(TRUE);
		goto got3;

	    case 'v':
		cursor_visible(FALSE);
		goto got3;

	    got3:
		s += 3;
		len -= 2;
	    }

# ifdef MCH_WRITE_DUMP
	    if (fdDump)
	    {
		fputs("ESC | ", fdDump);
		fwrite(old_s + 2, sizeof(char_u), s - old_s - 2, fdDump);
		fputc('\n', fdDump);
	    }
# endif
	}
	else if (s[0] == ESC && len >= 3-1 && s[1] == '[')
	{
	    int l = 2;

	    if (SAFE_isdigit(s[l]))
		l++;
	    if (s[l] == ' ' && s[l + 1] == 'q')
	    {
		// DECSCUSR (cursor style) sequences
		if (vtp_working)
		    vtp_printf("%.*s", l + 2, s);   // Pass through
		s += l + 2;
		len -= l + 1;
	    }
	}
	else
	{
	    // Write a single character
	    DWORD nWritten;

	    nWritten = write_chars(s, 1);
# ifdef MCH_WRITE_DUMP
	    if (fdDump)
	    {
		fputc('>', fdDump);
		fwrite(s, sizeof(char_u), nWritten, fdDump);
		fputs("<\n", fdDump);
	    }
# endif

	    len -= (nWritten - 1);
	    s += nWritten;
	}
    }

# ifdef MCH_WRITE_DUMP
    if (fdDump)
	fflush(fdDump);
# endif
}

#endif // FEAT_GUI_MSWIN


/*
 * Delay for "msec" milliseconds.
 */
    void
mch_delay(
    long    msec,
    int	    flags UNUSED)
{
#if defined(FEAT_GUI_MSWIN) && !defined(VIMDLL)
    Sleep((int)msec);	    // never wait for input
#else // Console
# ifdef VIMDLL
    if (gui.in_use)
    {
	Sleep((int)msec);	    // never wait for input
	return;
    }
# endif
    if (flags & MCH_DELAY_IGNOREINPUT)
# ifdef FEAT_MZSCHEME
	if (mzthreads_allowed() && p_mzq > 0 && msec > p_mzq)
	{
	    int towait = p_mzq;

	    // if msec is large enough, wait by portions in p_mzq
	    while (msec > 0)
	    {
		mzvim_check_threads();
		if (msec < towait)
		    towait = msec;
		Sleep(towait);
		msec -= towait;
	    }
	}
	else
# endif
	    Sleep((int)msec);
    else
	WaitForChar(msec, FALSE);
#endif
}


/*
 * This version of remove is not scared by a readonly (backup) file.
 * This can also remove a symbolic link like Unix.
 * Return 0 for success, -1 for failure.
 */
    int
mch_remove(char_u *name)
{
    WCHAR	*wn;
    int		n;

    /*
     * On Windows, deleting a directory's symbolic link is done by
     * RemoveDirectory(): mch_rmdir.  It seems unnatural, but it is fact.
     */
    if (mch_isdir(name) && mch_is_symbolic_link(name))
	return mch_rmdir(name);

    win32_setattrs(name, FILE_ATTRIBUTE_NORMAL);

    wn = enc_to_utf16(name, NULL);
    if (wn == NULL)
	return -1;

    n = DeleteFileW(wn) ? 0 : -1;
    vim_free(wn);
    return n;
}


/*
 * Check for an "interrupt signal": CTRL-break or CTRL-C.
 */
    void
mch_breakcheck(int force UNUSED)
{
#if !defined(FEAT_GUI_MSWIN) || defined(VIMDLL)
# ifdef VIMDLL
    if (!gui.in_use)
# endif
	if (g_fCtrlCPressed || g_fCBrkPressed)
	{
	    ctrl_break_was_pressed = g_fCBrkPressed;
	    g_fCtrlCPressed = g_fCBrkPressed = FALSE;
	    got_int = TRUE;
	}
#endif
}

// physical RAM to leave for the OS
#define WINNT_RESERVE_BYTES     (256*1024*1024)

/*
 * How much main memory in KiB that can be used by VIM.
 */
    long_u
mch_total_mem(int special UNUSED)
{
    MEMORYSTATUSEX  ms;

    // Need to use GlobalMemoryStatusEx() when there is more memory than
    // what fits in 32 bits.
    ms.dwLength = sizeof(MEMORYSTATUSEX);
    GlobalMemoryStatusEx(&ms);
    if (ms.ullAvailVirtual < ms.ullTotalPhys)
    {
	// Process address space fits in physical RAM, use all of it.
	return (long_u)(ms.ullAvailVirtual / 1024);
    }
    if (ms.ullTotalPhys <= WINNT_RESERVE_BYTES)
    {
	// Catch old NT box or perverse hardware setup.
	return (long_u)((ms.ullTotalPhys / 2) / 1024);
    }
    // Use physical RAM less reserve for OS + data.
    return (long_u)((ms.ullTotalPhys - WINNT_RESERVE_BYTES) / 1024);
}

/*
 * mch_wrename() works around a bug in rename (aka MoveFile) in
 * Windows 95: rename("foo.bar", "foo.bar~") will generate a
 * file whose short file name is "FOO.BAR" (its long file name will
 * be correct: "foo.bar~").  Because a file can be accessed by
 * either its SFN or its LFN, "foo.bar" has effectively been
 * renamed to "foo.bar", which is not at all what was wanted.  This
 * seems to happen only when renaming files with three-character
 * extensions by appending a suffix that does not include ".".
 * Windows NT gets it right, however, with an SFN of "FOO~1.BAR".
 *
 * There is another problem, which isn't really a bug but isn't right either:
 * When renaming "abcdef~1.txt" to "abcdef~1.txt~", the short name can be
 * "abcdef~1.txt" again.  This has been reported on Windows NT 4.0 with
 * service pack 6.  Doesn't seem to happen on Windows 98.
 *
 * Like rename(), returns 0 upon success, non-zero upon failure.
 * Should probably set errno appropriately when errors occur.
 */
    int
mch_wrename(WCHAR *wold, WCHAR *wnew)
{
    WCHAR	*p;
    int		i;
    WCHAR	szTempFile[_MAX_PATH + 1];
    WCHAR	szNewPath[_MAX_PATH + 1];
    HANDLE	hf;

    // No need to play tricks unless the file name contains a "~" as the
    // seventh character.
    p = wold;
    for (i = 0; wold[i] != NUL; ++i)
	if ((wold[i] == '/' || wold[i] == '\\' || wold[i] == ':')
		&& wold[i + 1] != 0)
	    p = wold + i + 1;
    if ((int)(wold + i - p) < 8 || p[6] != '~')
	return (MoveFileW(wold, wnew) == 0);

    // Get base path of new file name.  Undocumented feature: If pszNewFile is
    // a directory, no error is returned and pszFilePart will be NULL.
    if (GetFullPathNameW(wnew, _MAX_PATH, szNewPath, &p) == 0 || p == NULL)
	return -1;
    *p = NUL;

    // Get (and create) a unique temporary file name in directory of new file
    if (GetTempFileNameW(szNewPath, L"VIM", 0, szTempFile) == 0)
	return -2;

    // blow the temp file away
    if (!DeleteFileW(szTempFile))
	return -3;

    // rename old file to the temp file
    if (!MoveFileW(wold, szTempFile))
	return -4;

    // now create an empty file called pszOldFile; this prevents the operating
    // system using pszOldFile as an alias (SFN) if we're renaming within the
    // same directory.  For example, we're editing a file called
    // filename.asc.txt by its SFN, filena~1.txt.  If we rename filena~1.txt
    // to filena~1.txt~ (i.e., we're making a backup while writing it), the
    // SFN for filena~1.txt~ will be filena~1.txt, by default, which will
    // cause all sorts of problems later in buf_write().  So, we create an
    // empty file called filena~1.txt and the system will have to find some
    // other SFN for filena~1.txt~, such as filena~2.txt
    if ((hf = CreateFileW(wold, GENERIC_WRITE, 0, NULL, CREATE_NEW,
		    FILE_ATTRIBUTE_NORMAL, NULL)) == INVALID_HANDLE_VALUE)
	return -5;
    if (!CloseHandle(hf))
	return -6;

    // rename the temp file to the new file
    if (!MoveFileW(szTempFile, wnew))
    {
	// Renaming failed.  Rename the file back to its old name, so that it
	// looks like nothing happened.
	(void)MoveFileW(szTempFile, wold);
	return -7;
    }

    // Seems to be left around on Novell filesystems
    DeleteFileW(szTempFile);

    // finally, remove the empty old file
    if (!DeleteFileW(wold))
	return -8;

    return 0;
}


/*
 * Converts the filenames to UTF-16, then call mch_wrename().
 * Like rename(), returns 0 upon success, non-zero upon failure.
 */
    int
mch_rename(
    const char	*pszOldFile,
    const char	*pszNewFile)
{
    WCHAR	*wold = NULL;
    WCHAR	*wnew = NULL;
    int		retval = -1;

    wold = enc_to_utf16((char_u *)pszOldFile, NULL);
    wnew = enc_to_utf16((char_u *)pszNewFile, NULL);
    if (wold != NULL && wnew != NULL)
	retval = mch_wrename(wold, wnew);
    vim_free(wold);
    vim_free(wnew);
    return retval;
}

/*
 * Get the default shell for the current hardware platform
 */
    char *
default_shell(void)
{
    return "cmd.exe";
}

/*
 * mch_access() extends access() to do more detailed check on network drives.
 * Returns 0 if file "n" has access rights according to "p", -1 otherwise.
 */
    int
mch_access(char *n, int p)
{
    HANDLE	hFile;
    int		retval = -1;	    // default: fail
    WCHAR	*wn;

    wn = enc_to_utf16((char_u *)n, NULL);
    if (wn == NULL)
	return -1;

    if (mch_isdir((char_u *)n))
    {
	WCHAR TempNameW[_MAX_PATH + 16] = L"";

	if (p & R_OK)
	{
	    // Read check is performed by seeing if we can do a find file on
	    // the directory for any file.
	    int			i;
	    WIN32_FIND_DATAW    d;

	    for (i = 0; i < _MAX_PATH && wn[i] != 0; ++i)
		TempNameW[i] = wn[i];
	    if (TempNameW[i - 1] != '\\' && TempNameW[i - 1] != '/')
		TempNameW[i++] = '\\';
	    TempNameW[i++] = '*';
	    TempNameW[i++] = 0;

	    hFile = FindFirstFileW(TempNameW, &d);
	    if (hFile == INVALID_HANDLE_VALUE)
		goto getout;
	    else
		(void)FindClose(hFile);
	}

	if (p & W_OK)
	{
	    // Trying to create a temporary file in the directory should catch
	    // directories on read-only network shares.  However, in
	    // directories whose ACL allows writes but denies deletes will end
	    // up keeping the temporary file :-(.
	    if (!GetTempFileNameW(wn, L"VIM", 0, TempNameW))
		goto getout;
	    else
		DeleteFileW(TempNameW);
	}
    }
    else
    {
	// Don't consider a file read-only if another process has opened it.
	DWORD share_mode = FILE_SHARE_READ | FILE_SHARE_WRITE;

	// Trying to open the file for the required access does ACL, read-only
	// network share, and file attribute checks.
	DWORD access_mode = ((p & W_OK) ? GENERIC_WRITE : 0)
					     | ((p & R_OK) ? GENERIC_READ : 0);

	hFile = CreateFileW(wn, access_mode, share_mode,
					     NULL, OPEN_EXISTING, 0, NULL);
	if (hFile == INVALID_HANDLE_VALUE)
	    goto getout;
	CloseHandle(hFile);
    }

    retval = 0;	    // success
getout:
    vim_free(wn);
    return retval;
}

/*
 * Version of open() that may use UTF-16 file name.
 */
    int
mch_open(const char *name, int flags, int mode)
{
    WCHAR	*wn;
    int		f;

    wn = enc_to_utf16((char_u *)name, NULL);
    if (wn == NULL)
	return -1;

    f = _wopen(wn, flags, mode);
    vim_free(wn);
    return f;
}

/*
 * Version of fopen() that uses UTF-16 file name.
 */
    FILE *
mch_fopen(const char *name, const char *mode)
{
    WCHAR	*wn, *wm;
    FILE	*f = NULL;

#if defined(DEBUG) && _MSC_VER >= 1400
    // Work around an annoying assertion in the Microsoft debug CRT
    // when mode's text/binary setting doesn't match _get_fmode().
    char newMode = mode[strlen(mode) - 1];
    int oldMode = 0;

    _get_fmode(&oldMode);
    if (newMode == 't')
	_set_fmode(_O_TEXT);
    else if (newMode == 'b')
	_set_fmode(_O_BINARY);
#endif
    wn = enc_to_utf16((char_u *)name, NULL);
    wm = enc_to_utf16((char_u *)mode, NULL);
    if (wn != NULL && wm != NULL)
	f = _wfopen(wn, wm);
    vim_free(wn);
    vim_free(wm);

#if defined(DEBUG) && _MSC_VER >= 1400
    _set_fmode(oldMode);
#endif
    return f;
}

/*
 * SUB STREAM (aka info stream) handling:
 *
 * NTFS can have sub streams for each file.  The normal contents of a file is
 * stored in the main stream, and extra contents (author information, title and
 * so on) can be stored in a sub stream.  After Windows 2000, the user can
 * access and store this information in sub streams via an explorer's property
 * menu item in the right click menu.  This information in sub streams was lost
 * when copying only the main stream.  Therefore we have to copy sub streams.
 *
 * Incomplete explanation:
 *	http://msdn.microsoft.com/library/en-us/dnw2k/html/ntfs5.asp
 * More useful info and an example:
 *	http://www.sysinternals.com/ntw2k/source/misc.shtml#streams
 */

/*
 * Copy info stream data "substream".  Read from the file with BackupRead(sh)
 * and write to stream "substream" of file "to".
 * Errors are ignored.
 */
    static void
copy_substream(HANDLE sh, void *context, WCHAR *to, WCHAR *substream, long len)
{
    HANDLE  hTo;
    WCHAR   *to_name;

    to_name = malloc((wcslen(to) + wcslen(substream) + 1) * sizeof(WCHAR));
    wcscpy(to_name, to);
    wcscat(to_name, substream);

    hTo = CreateFileW(to_name, GENERIC_WRITE, 0, NULL, OPEN_ALWAYS,
						 FILE_ATTRIBUTE_NORMAL, NULL);
    if (hTo != INVALID_HANDLE_VALUE)
    {
	long	done;
	DWORD	todo;
	DWORD	readcnt, written;
	char	buf[4096];

	// Copy block of bytes at a time.  Abort when something goes wrong.
	for (done = 0; done < len; done += written)
	{
	    // (size_t) cast for Borland C 5.5
	    todo = (DWORD)((size_t)(len - done) > sizeof(buf) ? sizeof(buf)
						       : (size_t)(len - done));
	    if (!BackupRead(sh, (LPBYTE)buf, todo, &readcnt,
						       FALSE, FALSE, context)
		    || readcnt != todo
		    || !WriteFile(hTo, buf, todo, &written, NULL)
		    || written != todo)
		break;
	}
	CloseHandle(hTo);
    }

    free(to_name);
}

/*
 * Copy info streams from file "from" to file "to".
 */
    static void
copy_infostreams(char_u *from, char_u *to)
{
    WCHAR		*fromw;
    WCHAR		*tow;
    HANDLE		sh;
    WIN32_STREAM_ID	sid;
    int			headersize;
    WCHAR		streamname[_MAX_PATH];
    DWORD		readcount;
    void		*context = NULL;
    DWORD		lo, hi;
    int			len;

    // Convert the file names to wide characters.
    fromw = enc_to_utf16(from, NULL);
    tow = enc_to_utf16(to, NULL);
    if (fromw != NULL && tow != NULL)
    {
	// Open the file for reading.
	sh = CreateFileW(fromw, GENERIC_READ, FILE_SHARE_READ, NULL,
			     OPEN_EXISTING, FILE_FLAG_BACKUP_SEMANTICS, NULL);
	if (sh != INVALID_HANDLE_VALUE)
	{
	    // Use BackupRead() to find the info streams.  Repeat until we
	    // have done them all.
	    for (;;)
	    {
		// Get the header to find the length of the stream name.  If
		// the "readcount" is zero we have done all info streams.
		ZeroMemory(&sid, sizeof(WIN32_STREAM_ID));
		headersize = (int)((char *)&sid.cStreamName - (char *)&sid.dwStreamId);
		if (!BackupRead(sh, (LPBYTE)&sid, headersize,
					   &readcount, FALSE, FALSE, &context)
			|| readcount == 0)
		    break;

		// We only deal with streams that have a name.  The normal
		// file data appears to be without a name, even though docs
		// suggest it is called "::$DATA".
		if (sid.dwStreamNameSize > 0)
		{
		    // Read the stream name.
		    if (!BackupRead(sh, (LPBYTE)streamname,
							 sid.dwStreamNameSize,
					  &readcount, FALSE, FALSE, &context))
			break;

		    // Copy an info stream with a name ":anything:$DATA".
		    // Skip "::$DATA", it has no stream name (examples suggest
		    // it might be used for the normal file contents).
		    // Note that BackupRead() counts bytes, but the name is in
		    // wide characters.
		    len = readcount / sizeof(WCHAR);
		    streamname[len] = 0;
		    if (len > 7 && wcsicmp(streamname + len - 6,
							      L":$DATA") == 0)
		    {
			streamname[len - 6] = 0;
			copy_substream(sh, &context, tow, streamname,
						    (long)sid.Size.u.LowPart);
		    }
		}

		// Advance to the next stream.  We might try seeking too far,
		// but BackupSeek() doesn't skip over stream borders, thus
		// that's OK.
		(void)BackupSeek(sh, sid.Size.u.LowPart, sid.Size.u.HighPart,
							  &lo, &hi, &context);
	    }

	    // Clear the context.
	    (void)BackupRead(sh, NULL, 0, &readcount, TRUE, FALSE, &context);

	    CloseHandle(sh);
	}
    }
    vim_free(fromw);
    vim_free(tow);
}

/*
 * ntdll.dll definitions
 */
#define FileEaInformation   7
#ifndef STATUS_SUCCESS
# define STATUS_SUCCESS	    ((NTSTATUS) 0x00000000L)
#endif

typedef struct _FILE_FULL_EA_INFORMATION_ {
    ULONG  NextEntryOffset;
    UCHAR  Flags;
    UCHAR  EaNameLength;
    USHORT EaValueLength;
    CHAR   EaName[1];
} FILE_FULL_EA_INFORMATION_, *PFILE_FULL_EA_INFORMATION_;

typedef struct _FILE_EA_INFORMATION_ {
    ULONG EaSize;
} FILE_EA_INFORMATION_, *PFILE_EA_INFORMATION_;

#ifndef PROTO
typedef NTSTATUS (NTAPI *PfnNtOpenFile)(
	PHANDLE FileHandle,
	ACCESS_MASK DesiredAccess,
	POBJECT_ATTRIBUTES ObjectAttributes,
	PIO_STATUS_BLOCK IoStatusBlock,
	ULONG ShareAccess,
	ULONG OpenOptions);
typedef NTSTATUS (NTAPI *PfnNtClose)(
	HANDLE Handle);
typedef NTSTATUS (NTAPI *PfnNtSetEaFile)(
	HANDLE		    FileHandle,
	PIO_STATUS_BLOCK    IoStatusBlock,
	PVOID		    Buffer,
	ULONG		    Length);
typedef NTSTATUS (NTAPI *PfnNtQueryEaFile)(
	HANDLE FileHandle,
	PIO_STATUS_BLOCK IoStatusBlock,
	PVOID Buffer,
	ULONG Length,
	BOOLEAN ReturnSingleEntry,
	PVOID EaList,
	ULONG EaListLength,
	PULONG EaIndex,
	BOOLEAN RestartScan);
typedef NTSTATUS (NTAPI *PfnNtQueryInformationFile)(
	HANDLE			FileHandle,
	PIO_STATUS_BLOCK	IoStatusBlock,
	PVOID			FileInformation,
	ULONG			Length,
	FILE_INFORMATION_CLASS FileInformationClass);
typedef VOID (NTAPI *PfnRtlInitUnicodeString)(
	PUNICODE_STRING DestinationString,
	PCWSTR SourceString);

PfnNtOpenFile pNtOpenFile = NULL;
PfnNtClose pNtClose = NULL;
PfnNtSetEaFile pNtSetEaFile = NULL;
PfnNtQueryEaFile pNtQueryEaFile = NULL;
PfnNtQueryInformationFile pNtQueryInformationFile = NULL;
PfnRtlInitUnicodeString pRtlInitUnicodeString = NULL;
#endif

/*
 * Load ntdll.dll functions.
 */
    static BOOL
load_ntdll(void)
{
    static int	loaded = -1;

    if (loaded != -1)
	return (BOOL) loaded;

    HMODULE hNtdll = GetModuleHandle("ntdll.dll");
    if (hNtdll != NULL)
    {
	pNtOpenFile = (PfnNtOpenFile) GetProcAddress(hNtdll, "NtOpenFile");
	pNtClose = (PfnNtClose) GetProcAddress(hNtdll, "NtClose");
	pNtSetEaFile = (PfnNtSetEaFile)
	    GetProcAddress(hNtdll, "NtSetEaFile");
	pNtQueryEaFile = (PfnNtQueryEaFile)
	    GetProcAddress(hNtdll, "NtQueryEaFile");
	pNtQueryInformationFile = (PfnNtQueryInformationFile)
	    GetProcAddress(hNtdll, "NtQueryInformationFile");
	pRtlInitUnicodeString = (PfnRtlInitUnicodeString)
	    GetProcAddress(hNtdll, "RtlInitUnicodeString");
    }
    if (pNtOpenFile == NULL
	    || pNtClose == NULL
	    || pNtSetEaFile == NULL
	    || pNtQueryEaFile == NULL
	    || pNtQueryInformationFile == NULL
	    || pRtlInitUnicodeString == NULL)
	loaded = FALSE;
    else
	loaded = TRUE;
    return (BOOL) loaded;
}

/*
 * Copy extended attributes (EA) from file "from" to file "to".
 */
    static void
copy_extattr(char_u *from, char_u *to)
{
    char_u		    *fromf = NULL;
    char_u		    *tof = NULL;
    WCHAR		    *fromw = NULL;
    WCHAR		    *tow = NULL;
    UNICODE_STRING	    u;
    HANDLE		    h;
    OBJECT_ATTRIBUTES	    oa;
    IO_STATUS_BLOCK	    iosb;
    FILE_EA_INFORMATION_    eainfo = {0};
    void		    *ea = NULL;

    if (!load_ntdll())
	return;

    // Convert the file names to the fully qualified object names.
    fromf = alloc(STRLEN(from) + 5);
    tof = alloc(STRLEN(to) + 5);
    if (fromf == NULL || tof == NULL)
	goto theend;
    STRCPY(fromf, "\\??\\");
    STRCAT(fromf, from);
    STRCPY(tof, "\\??\\");
    STRCAT(tof, to);

    // Convert the names to wide characters.
    fromw = enc_to_utf16(fromf, NULL);
    tow = enc_to_utf16(tof, NULL);
    if (fromw == NULL || tow == NULL)
	goto theend;

    // Get the EA.
    pRtlInitUnicodeString(&u, fromw);
    InitializeObjectAttributes(&oa, &u, 0, NULL, NULL);
    if (pNtOpenFile(&h, FILE_READ_EA, &oa, &iosb, 0,
		FILE_NON_DIRECTORY_FILE) != STATUS_SUCCESS)
	goto theend;
    pNtQueryInformationFile(h, &iosb, &eainfo, sizeof(eainfo),
	    FileEaInformation);
    if (eainfo.EaSize != 0)
    {
	ea = alloc(eainfo.EaSize);
	if (ea != NULL)
	{
	    if (pNtQueryEaFile(h, &iosb, ea, eainfo.EaSize, FALSE,
			NULL, 0, NULL, TRUE) != STATUS_SUCCESS)
	    {
		VIM_CLEAR(ea);
	    }
	}
    }
    pNtClose(h);

    // Set the EA.
    if (ea != NULL)
    {
	pRtlInitUnicodeString(&u, tow);
	InitializeObjectAttributes(&oa, &u, 0, NULL, NULL);
	if (pNtOpenFile(&h, FILE_WRITE_EA, &oa, &iosb, 0,
		    FILE_NON_DIRECTORY_FILE) != STATUS_SUCCESS)
	    goto theend;

	pNtSetEaFile(h, &iosb, ea, eainfo.EaSize);
	pNtClose(h);
    }

theend:
    vim_free(fromf);
    vim_free(tof);
    vim_free(fromw);
    vim_free(tow);
    vim_free(ea);
}

/*
 * Copy file attributes from file "from" to file "to".
 * For Windows NT and later we copy info streams.
 * Always returns zero, errors are ignored.
 */
    int
mch_copy_file_attribute(char_u *from, char_u *to)
{
    // File streams only work on Windows NT and later.
    copy_infostreams(from, to);
    copy_extattr(from, to);
    return 0;
}


/*
 * The command line arguments in UTF-16
 */
static int	nArgsW = 0;
static LPWSTR	*ArglistW = NULL;
static int	global_argc = 0;
static char	**global_argv;

static int	used_file_argc = 0;	// last argument in global_argv[] used
					// for the argument list.
static int	*used_file_indexes = NULL; // indexes in global_argv[] for
					   // command line arguments added to
					   // the argument list
static int	used_file_count = 0;	// nr of entries in used_file_indexes
static int	used_file_literal = FALSE;  // take file names literally
static int	used_file_full_path = FALSE;  // file name was full path
static int	used_file_diff_mode = FALSE;  // file name was with diff mode
static int	used_alist_count = 0;


/*
 * Get the command line arguments.  Unicode version.
 * Returns argc.  Zero when something fails.
 */
    int
get_cmd_argsW(char ***argvp)
{
    char	**argv = NULL;
    int		argc = 0;
    int		i;

    free_cmd_argsW();
    ArglistW = CommandLineToArgvW(GetCommandLineW(), &nArgsW);
    if (ArglistW != NULL)
    {
	argv = malloc((nArgsW + 1) * sizeof(char *));
	if (argv != NULL)
	{
	    argc = nArgsW;
	    argv[argc] = NULL;
	    for (i = 0; i < argc; ++i)
	    {
		int	len;

		// Convert each Unicode argument to UTF-8.
		WideCharToMultiByte_alloc(CP_UTF8, 0,
				ArglistW[i], (int)wcslen(ArglistW[i]) + 1,
				(LPSTR *)&argv[i], &len, 0, 0);
		if (argv[i] == NULL)
		{
		    // Out of memory, clear everything.
		    while (i > 0)
			free(argv[--i]);
		    free(argv);
		    argv = NULL;
		    argc = 0;
		}
	    }
	}
    }

    global_argc = argc;
    global_argv = argv;
    if (argc > 0)
    {
	if (used_file_indexes != NULL)
	    free(used_file_indexes);
	used_file_indexes = malloc(argc * sizeof(int));
    }

    if (argvp != NULL)
	*argvp = argv;
    return argc;
}

    void
free_cmd_argsW(void)
{
    if (ArglistW == NULL)
	return;

    GlobalFree(ArglistW);
    ArglistW = NULL;
}

/*
 * Remember "name" is an argument that was added to the argument list.
 * This avoids that we have to re-parse the argument list when fix_arg_enc()
 * is called.
 */
    void
used_file_arg(char *name, int literal, int full_path, int diff_mode)
{
    int		i;

    if (used_file_indexes == NULL)
	return;
    for (i = used_file_argc + 1; i < global_argc; ++i)
	if (STRCMP(global_argv[i], name) == 0)
	{
	    used_file_argc = i;
	    used_file_indexes[used_file_count++] = i;
	    break;
	}
    used_file_literal = literal;
    used_file_full_path = full_path;
    used_file_diff_mode = diff_mode;
}

/*
 * Remember the length of the argument list as it was.  If it changes then we
 * leave it alone when 'encoding' is set.
 */
    void
set_alist_count(void)
{
    used_alist_count = GARGCOUNT;
}

/*
 * Fix the encoding of the command line arguments.  Invoked when 'encoding'
 * has been changed while starting up.  Use the UTF-16 command line arguments
 * and convert them to 'encoding'.
 */
    void
fix_arg_enc(void)
{
    int		i;
    int		idx;
    char_u	*str;
    int		*fnum_list;

    // Safety checks:
    // - if argument count differs between the wide and non-wide argument
    //   list, something must be wrong.
    // - the file name arguments must have been located.
    // - the length of the argument list wasn't changed by the user.
    if (global_argc != nArgsW
	    || ArglistW == NULL
	    || used_file_indexes == NULL
	    || used_file_count == 0
	    || used_alist_count != GARGCOUNT)
	return;

    // Remember the buffer numbers for the arguments.
    fnum_list = ALLOC_MULT(int, GARGCOUNT);
    if (fnum_list == NULL)
	return;		// out of memory
    for (i = 0; i < GARGCOUNT; ++i)
	fnum_list[i] = GARGLIST[i].ae_fnum;

    // Clear the argument list.  Make room for the new arguments.
    alist_clear(&global_alist);
    if (ga_grow(&global_alist.al_ga, used_file_count) == FAIL)
	return;		// out of memory

    for (i = 0; i < used_file_count; ++i)
    {
	idx = used_file_indexes[i];
	str = utf16_to_enc(ArglistW[idx], NULL);
	if (str != NULL)
	{
	    int literal = used_file_literal;

#ifdef FEAT_DIFF
	    // When using diff mode may need to concatenate file name to
	    // directory name.  Just like it's done in main().
	    if (used_file_diff_mode && mch_isdir(str) && GARGCOUNT > 0
				      && !mch_isdir(alist_name(&GARGLIST[0])))
	    {
		char_u	    *r;

		r = concat_fnames(str, gettail(alist_name(&GARGLIST[0])), TRUE);
		if (r != NULL)
		{
		    vim_free(str);
		    str = r;
		}
	    }
#endif
	    // Re-use the old buffer by renaming it.  When not using literal
	    // names it's done by alist_expand() below.
	    if (used_file_literal)
		buf_set_name(fnum_list[i], str);

	    // Check backtick literal. backtick literal is already expanded in
	    // main.c, so this part add str as literal.
	    if (literal == FALSE)
	    {
		size_t len = STRLEN(str);

		if (len > 2 && *str == '`' && *(str + len - 1) == '`')
		    literal = TRUE;
	    }
	    alist_add(&global_alist, str, literal ? 2 : 0);
	}
    }

    if (!used_file_literal)
    {
	// Now expand wildcards in the arguments.
	// Temporarily add '(' and ')' to 'isfname'.  These are valid
	// filename characters but are excluded from 'isfname' to make
	// "gf" work on a file name in parentheses (e.g.: see vim.h).
	// Also, unset wildignore to not be influenced by this option.
	// The arguments specified in command-line should be kept even if
	// encoding options were changed.
	// Use :legacy so that it also works when in Vim9 script.
	do_cmdline_cmd((char_u *)":legacy let g:SaVe_ISF = &isf|set isf+=(,)");
	do_cmdline_cmd((char_u *)":legacy let g:SaVe_WIG = &wig|set wig=");
	alist_expand(fnum_list, used_alist_count);
	do_cmdline_cmd(
		(char_u *)":legacy let &isf = g:SaVe_ISF|unlet g:SaVe_ISF");
	do_cmdline_cmd(
		(char_u *)":legacy let &wig = g:SaVe_WIG|unlet g:SaVe_WIG");
    }

    // If wildcard expansion failed, we are editing the first file of the
    // arglist and there is no file name: Edit the first argument now.
    if (curwin->w_arg_idx == 0 && curbuf->b_fname == NULL)
    {
	do_cmdline_cmd((char_u *)":rewind");
	if (GARGCOUNT == 1 && used_file_full_path
		&& vim_chdirfile(alist_name(&GARGLIST[0]), "drop") == OK)
	    last_chdir_reason = "drop";
    }

    set_alist_count();
}

    int
mch_setenv(char *var, char *value, int x UNUSED)
{
    char_u	*envbuf;
    WCHAR	*p;

    envbuf = alloc(STRLEN(var) + STRLEN(value) + 2);
    if (envbuf == NULL)
	return -1;

    sprintf((char *)envbuf, "%s=%s", var, value);

    p = enc_to_utf16(envbuf, NULL);

    vim_free(envbuf);
    if (p == NULL)
	return -1;
    _wputenv(p);
#ifdef libintl_wputenv
    libintl_wputenv(p);
#endif
    // Unlike Un*x systems, we can free the string for _wputenv().
    vim_free(p);

    return 0;
}

/*
 * Support for 256 colors and 24-bit colors was added in Windows 10
 * version 1703 (Creators update).
 */
#define VTP_FIRST_SUPPORT_BUILD MAKE_VER(10, 0, 15063)

/*
 * Support for pseudo-console (ConPTY) was added in windows 10
 * version 1809 (October 2018 update).
 */
#define CONPTY_FIRST_SUPPORT_BUILD  MAKE_VER(10, 0, 17763)

/*
 * ConPTY differences between versions, need different logic.
 * version 1903 (May 2019 update).
 */
#define CONPTY_1903_BUILD	    MAKE_VER(10, 0, 18362)

/*
 * version 1909 (November 2019 update).
 */
#define CONPTY_1909_BUILD	    MAKE_VER(10, 0, 18363)

/*
 * Stay ahead of the next update, and when it's done, fix this.
 * version ? (2020 update, temporarily use the build number of insider preview)
 */
#define CONPTY_NEXT_UPDATE_BUILD    MAKE_VER(10, 0, 19587)

/*
 * Confirm until this version.  Also the logic changes.
 * insider preview.
 */
#define CONPTY_INSIDER_BUILD	    MAKE_VER(10, 0, 18995)

/*
 * Not stable now.
 */
#define CONPTY_STABLE_BUILD	    MAKE_VER(10, 0, 32767)  // T.B.D.
// Notes:
// Win 10 22H2 Final is build 19045, it's conpty is widely used.
// Strangely, 19045 is newer but is a lower build number than the 2020 insider
// preview which had a build 19587.  And, not sure how stable that was?
// Win Server 2022 (May 10, 2022) is build 20348, its conpty is widely used.
// Win 11 starts from build 22000, even though the major version says 10!

    static void
vtp_flag_init(void)
{
    DWORD   ver = get_build_number();
#if !defined(FEAT_GUI_MSWIN) || defined(VIMDLL)
    DWORD   mode;
    HANDLE  out;

# ifdef VIMDLL
    if (!gui.in_use)
# endif
    {
	out = GetStdHandle(STD_OUTPUT_HANDLE);

	vtp_working = (ver >= VTP_FIRST_SUPPORT_BUILD) ? 1 : 0;
	GetConsoleMode(out, &mode);
	mode |= (ENABLE_PROCESSED_OUTPUT | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
	if (SetConsoleMode(out, mode) == 0)
	    vtp_working = 0;

	// VTP uses alternate screen buffer.
	// But, not if running in a nested terminal
	use_alternate_screen_buffer = win10_22H2_or_later && p_rs && vtp_working
						&& !mch_getenv("VIM_TERMINAL");
    }
#endif

    if (ver >= CONPTY_FIRST_SUPPORT_BUILD)
	conpty_working = 1;
    if (ver >= CONPTY_STABLE_BUILD)
	conpty_stable = 1;

    if (ver <= CONPTY_INSIDER_BUILD)
	conpty_type = 3;
    if (ver <= CONPTY_1909_BUILD)
	conpty_type = 2;
    if (ver <= CONPTY_1903_BUILD)
	conpty_type = 2;
    if (ver < CONPTY_FIRST_SUPPORT_BUILD)
	conpty_type = 1;

    if (ver >= CONPTY_NEXT_UPDATE_BUILD)
	conpty_fix_type = 1;
}

#if !defined(FEAT_GUI_MSWIN) || defined(VIMDLL) || defined(PROTO)

    static void
vtp_init(void)
{
# ifdef FEAT_TERMGUICOLORS
    CONSOLE_SCREEN_BUFFER_INFOEX csbi;
    csbi.cbSize = sizeof(csbi);
    GetConsoleScreenBufferInfoEx(g_hConOut, &csbi);
    save_console_bg_rgb = (guicolor_T)csbi.ColorTable[g_color_index_bg];
    save_console_fg_rgb = (guicolor_T)csbi.ColorTable[g_color_index_fg];
    store_console_bg_rgb = save_console_bg_rgb;
    store_console_fg_rgb = save_console_fg_rgb;

    COLORREF bg;
    bg = (COLORREF)csbi.ColorTable[g_color_index_bg];
    bg = (GetRValue(bg) << 16) | (GetGValue(bg) << 8) | GetBValue(bg);
    default_console_color_bg = bg;

    COLORREF fg;
    fg = (COLORREF)csbi.ColorTable[g_color_index_fg];
    fg = (GetRValue(fg) << 16) | (GetGValue(fg) << 8) | GetBValue(fg);
    default_console_color_fg = fg;
# endif
    set_console_color_rgb();
}

    static void
vtp_exit(void)
{
    restore_console_color_rgb();
}

    int
vtp_printf(
    char *format,
    ...)
{
    char_u  buf[100];
    va_list list;
    DWORD   result;
    int	    len;

    if (silent_mode)
	return 0;

    va_start(list, format);
    len = vim_vsnprintf((char *)buf, 100, (char *)format, list);
    va_end(list);
    WriteConsoleA(g_hConOut, buf, (DWORD)len, &result, NULL);
    return (int)result;
}

    static void
vtp_sgr_bulk(
    int arg)
{
    int args[1];

    args[0] = arg;
    vtp_sgr_bulks(1, args);
}

# define FAST256(x) \
    if ((*p-- = "0123456789"[(n = x % 10)]) \
	    && x >= 10 && (*p-- = "0123456789"[((m = x % 100) - n) / 10]) \
	    && x >= 100 && (*p-- = "012"[((x & 0xff) - m) / 100]));

# define FAST256CASE(x) \
    case x: \
	FAST256(newargs[x - 1]);

    static void
vtp_sgr_bulks(
    int argc,
    int *args)
{
# define MAXSGR 16
# define SGRBUFSIZE 2 + 4 * MAXSGR + 1 // '\033[' + SGR + 'm'
    char_u  buf[SGRBUFSIZE];
    char_u  *p;
    int	    in, out;
    int	    newargs[16];
    static int sgrfgr = -1, sgrfgg, sgrfgb;
    static int sgrbgr = -1, sgrbgg, sgrbgb;

    if (argc == 0)
    {
	sgrfgr = sgrbgr = -1;
	vtp_printf("\033[m");
	return;
    }

    in = out = 0;
    while (in < argc)
    {
	int s = args[in];
	int copylen = 1;

	if (s == 38)
	{
	    if (argc - in >= 5 && args[in + 1] == 2)
	    {
		if (sgrfgr == args[in + 2] && sgrfgg == args[in + 3]
						     && sgrfgb == args[in + 4])
		{
		    in += 5;
		    copylen = 0;
		}
		else
		{
		    sgrfgr = args[in + 2];
		    sgrfgg = args[in + 3];
		    sgrfgb = args[in + 4];
		    copylen = 5;
		}
	    }
	    else if (argc - in >= 3 && args[in + 1] == 5)
	    {
		sgrfgr = -1;
		copylen = 3;
	    }
	}
	else if (s == 48)
	{
	    if (argc - in >= 5 && args[in + 1] == 2)
	    {
		if (sgrbgr == args[in + 2] && sgrbgg == args[in + 3]
						     && sgrbgb == args[in + 4])
		{
		    in += 5;
		    copylen = 0;
		}
		else
		{
		    sgrbgr = args[in + 2];
		    sgrbgg = args[in + 3];
		    sgrbgb = args[in + 4];
		    copylen = 5;
		}
	    }
	    else if (argc - in >= 3 && args[in + 1] == 5)
	    {
		sgrbgr = -1;
		copylen = 3;
	    }
	}
	else if (30 <= s && s <= 39)
	    sgrfgr = -1;
	else if (90 <= s && s <= 97)
	    sgrfgr = -1;
	else if (40 <= s && s <= 49)
	    sgrbgr = -1;
	else if (100 <= s && s <= 107)
	    sgrbgr = -1;
	else if (s == 0)
	    sgrfgr = sgrbgr = -1;

	while (copylen--)
	    newargs[out++] = args[in++];
    }

    p = &buf[sizeof(buf) - 1];
    *p-- = 'm';

    switch (out)
    {
	int	n, m;
	DWORD	r;

	FAST256CASE(16);
	*p-- = ';';
	FAST256CASE(15);
	*p-- = ';';
	FAST256CASE(14);
	*p-- = ';';
	FAST256CASE(13);
	*p-- = ';';
	FAST256CASE(12);
	*p-- = ';';
	FAST256CASE(11);
	*p-- = ';';
	FAST256CASE(10);
	*p-- = ';';
	FAST256CASE(9);
	*p-- = ';';
	FAST256CASE(8);
	*p-- = ';';
	FAST256CASE(7);
	*p-- = ';';
	FAST256CASE(6);
	*p-- = ';';
	FAST256CASE(5);
	*p-- = ';';
	FAST256CASE(4);
	*p-- = ';';
	FAST256CASE(3);
	*p-- = ';';
	FAST256CASE(2);
	*p-- = ';';
	FAST256CASE(1);
	*p-- = '[';
	*p = '\033';
	WriteConsoleA(g_hConOut, p, (DWORD)(&buf[SGRBUFSIZE] - p), &r, NULL);
    default:
	break;
    }
}

    static void
wt_init(void)
{
    wt_working = mch_getenv("WT_SESSION") != NULL;
}

# ifdef FEAT_TERMGUICOLORS
    static int
ctermtoxterm(
    int cterm)
{
    char_u r, g, b, idx;

    cterm_color2rgb(cterm, &r, &g, &b, &idx);
    return (((int)r << 16) | ((int)g << 8) | (int)b);
}
# endif

    static void
set_console_color_rgb(void)
{
# ifdef FEAT_TERMGUICOLORS
    CONSOLE_SCREEN_BUFFER_INFOEX csbi;
    guicolor_T	fg, bg;
    int		ctermfg, ctermbg;

    if (!vtp_working)
	return;

    get_default_console_color(&ctermfg, &ctermbg, &fg, &bg);

    if (p_tgc || t_colors >= 256)
    {
	term_fg_rgb_color(fg);
	term_bg_rgb_color(bg);
	return;
    }

    if (use_alternate_screen_buffer)
	return;

    fg = (GetRValue(fg) << 16) | (GetGValue(fg) << 8) | GetBValue(fg);
    bg = (GetRValue(bg) << 16) | (GetGValue(bg) << 8) | GetBValue(bg);

    csbi.cbSize = sizeof(csbi);
    GetConsoleScreenBufferInfoEx(g_hConOut, &csbi);

    csbi.cbSize = sizeof(csbi);
    csbi.srWindow.Right += 1;
    csbi.srWindow.Bottom += 1;
    store_console_bg_rgb = csbi.ColorTable[g_color_index_bg];
    store_console_fg_rgb = csbi.ColorTable[g_color_index_fg];
    csbi.ColorTable[g_color_index_bg] = (COLORREF)bg;
    csbi.ColorTable[g_color_index_fg] = (COLORREF)fg;
    SetConsoleScreenBufferInfoEx(g_hConOut, &csbi);
# endif
}

# if defined(FEAT_TERMGUICOLORS) || defined(PROTO)
    void
get_default_console_color(
    int *cterm_fg,
    int *cterm_bg,
    guicolor_T *gui_fg,
    guicolor_T *gui_bg)
{
    int id;
    guicolor_T guifg = INVALCOLOR;
    guicolor_T guibg = INVALCOLOR;
    int ctermfg = 0;
    int ctermbg = 0;
    int dummynull = 0;

    id = syn_name2id((char_u *)"Normal");
    if (id > 0 && p_tgc)
	syn_id2colors(id, &guifg, &guibg);
    if (guifg == INVALCOLOR)
    {
	ctermfg = -1;
	if (id > 0)
	    syn_id2cterm_bg(id, &ctermfg, &dummynull);
	if (ctermfg != -1)
	    guifg = ctermtoxterm(ctermfg);
	else
	    guifg = USE_WT ? INVALCOLOR : default_console_color_fg;
	cterm_normal_fg_gui_color = guifg;
	ctermfg = ctermfg < 0 ? 0 : ctermfg;
    }
    if (guibg == INVALCOLOR)
    {
	ctermbg = -1;
	if (id > 0)
	    syn_id2cterm_bg(id, &dummynull, &ctermbg);
	if (ctermbg != -1)
	    guibg = ctermtoxterm(ctermbg);
	else
	    guibg = USE_WT ? INVALCOLOR : default_console_color_bg;
	cterm_normal_bg_gui_color = guibg;
	ctermbg = ctermbg < 0 ? 0 : ctermbg;
    }

    *cterm_fg = ctermfg;
    *cterm_bg = ctermbg;
    *gui_fg = guifg;
    *gui_bg = guibg;
}
# endif

/*
 * Set the console colors to the original colors or the last set colors.
 */
    static void
reset_console_color_rgb(void)
{
# ifdef FEAT_TERMGUICOLORS
    if (use_alternate_screen_buffer)
	return;

    CONSOLE_SCREEN_BUFFER_INFOEX csbi;

    csbi.cbSize = sizeof(csbi);
    GetConsoleScreenBufferInfoEx(g_hConOut, &csbi);

    csbi.cbSize = sizeof(csbi);
    csbi.srWindow.Right += 1;
    csbi.srWindow.Bottom += 1;
    csbi.ColorTable[g_color_index_bg] = (COLORREF)store_console_bg_rgb;
    csbi.ColorTable[g_color_index_fg] = (COLORREF)store_console_fg_rgb;
    SetConsoleScreenBufferInfoEx(g_hConOut, &csbi);
# endif
}

/*
 * Set the console colors to the original colors.
 */
    static void
restore_console_color_rgb(void)
{
# ifdef FEAT_TERMGUICOLORS
    if (use_alternate_screen_buffer)
	return;

    CONSOLE_SCREEN_BUFFER_INFOEX csbi;

    csbi.cbSize = sizeof(csbi);
    GetConsoleScreenBufferInfoEx(g_hConOut, &csbi);

    csbi.cbSize = sizeof(csbi);
    csbi.srWindow.Right += 1;
    csbi.srWindow.Bottom += 1;
    csbi.ColorTable[g_color_index_bg] = (COLORREF)save_console_bg_rgb;
    csbi.ColorTable[g_color_index_fg] = (COLORREF)save_console_fg_rgb;
    SetConsoleScreenBufferInfoEx(g_hConOut, &csbi);
# endif
}

    void
control_console_color_rgb(void)
{
    if (vtp_working)
	set_console_color_rgb();
    else
	reset_console_color_rgb();
}

    int
use_vtp(void)
{
    return USE_VTP;
}

    int
is_term_win32(void)
{
    return T_NAME != NULL && STRCMP(T_NAME, "win32") == 0;
}

    int
has_vtp_working(void)
{
    return vtp_working;
}

#endif

    int
has_conpty_working(void)
{
    return conpty_working;
}

    int
get_conpty_type(void)
{
    return conpty_type;
}

    int
is_conpty_stable(void)
{
    return conpty_stable;
}

    int
get_conpty_fix_type(void)
{
    return conpty_fix_type;
}

#if !defined(FEAT_GUI_MSWIN) || defined(VIMDLL) || defined(PROTO)
    void
resize_console_buf(void)
{
    if (use_alternate_screen_buffer)
	return;

    CONSOLE_SCREEN_BUFFER_INFO csbi;
    COORD coord;
    SMALL_RECT newsize;

    if (!GetConsoleScreenBufferInfo(g_hConOut, &csbi))
	return;

    coord.X = SRWIDTH(csbi.srWindow);
    coord.Y = SRHEIGHT(csbi.srWindow);
    SetConsoleScreenBufferSize(g_hConOut, coord);

    newsize.Left = 0;
    newsize.Top = 0;
    newsize.Right = coord.X - 1;
    newsize.Bottom = coord.Y - 1;
    SetConsoleWindowInfo(g_hConOut, TRUE, &newsize);

    SetConsoleScreenBufferSize(g_hConOut, coord);
}
#endif

    char *
GetWin32Error(void)
{
    static char *oldmsg = NULL;
    char *msg = NULL;

    FormatMessage(FORMAT_MESSAGE_ALLOCATE_BUFFER|FORMAT_MESSAGE_FROM_SYSTEM,
	    NULL, GetLastError(), 0, (LPSTR)&msg, 0, NULL);
    if (oldmsg != NULL)
	LocalFree(oldmsg);
    if (msg == NULL)
	return NULL;

    // remove trailing \r\n
    char *pcrlf = strstr(msg, "\r\n");
    if (pcrlf != NULL)
	*pcrlf = '\0';
    oldmsg = msg;
    return msg;
}

#if defined(FEAT_RELTIME) || defined(PROTO)
static HANDLE   timer_handle;
static int      timer_active = FALSE;

/*
 * Calls to start_timeout alternate the return value pointer between the two
 * entries in timeout_flags. If the previously active timeout is very close to
 * expiring when start_timeout() is called then a race condition means that the
 * set_flag() function may still be invoked after the previous timer is
 * deleted. Ping-ponging between the two flags prevents this causing 'fake'
 * timeouts.
 */
static sig_atomic_t timeout_flags[2];
static int	    timeout_flag_idx = 0;
static sig_atomic_t *timeout_flag = &timeout_flags[0];


    static void CALLBACK
set_flag(void *param, BOOLEAN unused2 UNUSED)
{
    int *timeout_flag = (int *)param;

    *timeout_flag = TRUE;
}

/*
 * Stop any active timeout.
 */
    void
stop_timeout(void)
{
    if (timer_active)
    {
	BOOL ret = DeleteTimerQueueTimer(NULL, timer_handle, NULL);
	timer_active = FALSE;
	if (!ret && GetLastError() != ERROR_IO_PENDING)
	{
	    semsg(_(e_could_not_clear_timeout_str), GetWin32Error());
	}
    }
    *timeout_flag = FALSE;
}

/*
 * Start the timeout timer.
 *
 * The period is defined in milliseconds.
 *
 * The return value is a pointer to a flag that is initialised to 0.  If the
 * timeout expires, the flag is set to 1. This will only return pointers to
 * static memory; i.e. any pointer returned by this function may always be
 * safely dereferenced.
 *
 * This function is not expected to fail, but if it does it still returns a
 * valid flag pointer; the flag will remain stuck at zero.
 */
    volatile sig_atomic_t *
start_timeout(long msec)
{
    BOOL ret;

    timeout_flag = &timeout_flags[timeout_flag_idx];

    stop_timeout();
    ret = CreateTimerQueueTimer(
	    &timer_handle, NULL, set_flag, timeout_flag,
	    (DWORD)msec, 0, WT_EXECUTEDEFAULT);
    if (!ret)
    {
	semsg(_(e_could_not_set_timeout_str), GetWin32Error());
    }
    else
    {
	timeout_flag_idx = (timeout_flag_idx + 1) % 2;
	timer_active = TRUE;
	*timeout_flag = FALSE;
    }
    return timeout_flag;
}
#endif
