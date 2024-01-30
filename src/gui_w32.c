/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved		by Bram Moolenaar
 *				GUI support by Robert Webb
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */
/*
 * Windows GUI.
 *
 * GUI support for Microsoft Windows, aka Win32.  Also for Win64.
 *
 * George V. Reilly <george@reilly.org> wrote the original Win32 GUI.
 * Robert Webb reworked it to use the existing GUI stuff and added menu,
 * scrollbars, etc.
 *
 * Note: Clipboard stuff, for cutting and pasting text to other windows, is in
 * winclip.c.	(It can also be done from the terminal version).
 *
 * TODO: Some of the function signatures ought to be updated for Win64;
 * e.g., replace LONG with LONG_PTR, etc.
 */

#include "vim.h"

#if defined(FEAT_DIRECTX)
# include "gui_dwrite.h"
#endif

// values for "dead_key"
#define DEAD_KEY_OFF			0	// no dead key
#define DEAD_KEY_SET_DEFAULT		1	// dead key pressed
#define DEAD_KEY_TRANSIENT_IN_ON_CHAR	2	// wait for next key press
#define DEAD_KEY_SKIP_ON_CHAR		3	// skip next _OnChar()

#if defined(FEAT_DIRECTX)
static DWriteContext *s_dwc = NULL;
static int s_directx_enabled = 0;
static int s_directx_load_attempted = 0;
# define IS_ENABLE_DIRECTX() (s_directx_enabled && s_dwc != NULL && enc_utf8)
static int directx_enabled(void);
static void directx_binddc(void);
#endif

#ifdef FEAT_MENU
static int gui_mswin_get_menu_height(int fix_window);
#else
# define gui_mswin_get_menu_height(fix_window)	0
#endif

typedef struct keycode_trans_strategy {
    void (*ptr_on_char) (HWND /*hwnd UNUSED*/, UINT /*cch*/, int /*cRepeat UNUSED*/);
    void (*ptr_on_sys_char) (HWND /*hwnd UNUSED*/, UINT /*cch*/, int /*cRepeat UNUSED*/);
    void (*ptr_process_message_usual_key) (UINT /*vk*/, const MSG* /*pmsg*/);
    int  (*ptr_get_active_modifiers)(void);
    int  (*is_experimental)(void);
} keycode_trans_strategy;

// forward declarations for input instance initializer
static void _OnChar_experimental(HWND /*hwnd UNUSED*/, UINT /*cch*/, int /*cRepeat UNUSED*/);
static void _OnSysChar_experimental(HWND /*hwnd UNUSED*/, UINT /*cch*/, int /*cRepeat UNUSED*/);
static void process_message_usual_key_experimental(UINT /*vk*/, const MSG* /*pmsg*/);
static int  get_active_modifiers_experimental(void);
static int  is_experimental_true(void);

keycode_trans_strategy keycode_trans_strategy_experimental = {
      _OnChar_experimental      // ptr_on_char
    , _OnSysChar_experimental // ptr_on_sys_char
    , process_message_usual_key_experimental // ptr_process_message_usual_key
    , get_active_modifiers_experimental
    , is_experimental_true
};

// forward declarations for input instance initializer
static void _OnChar_classic(HWND /*hwnd UNUSED*/, UINT /*cch*/, int /*cRepeat UNUSED*/);
static void _OnSysChar_classic(HWND /*hwnd UNUSED*/, UINT /*cch*/, int /*cRepeat UNUSED*/);
static void process_message_usual_key_classic(UINT /*vk*/, const MSG* /*pmsg*/);
static int  get_active_modifiers_classic(void);
static int  is_experimental_false(void);

keycode_trans_strategy keycode_trans_strategy_classic = {
      _OnChar_classic      // ptr_on_char
    , _OnSysChar_classic // ptr_on_sys_char
    , process_message_usual_key_classic // ptr_process_message_usual_key
    , get_active_modifiers_classic
    , is_experimental_false
};

keycode_trans_strategy *keycode_trans_strategy_used = NULL;

static int is_experimental_true(void)
{
    return 1;
}

static int is_experimental_false(void)
{
    return 0;
}

/*
 * Initialize the keycode translation strategy.
 */
static void keycode_trans_strategy_init(void)
{
    const char *strategy = NULL;

    // set default value as fallback
    keycode_trans_strategy_used = &keycode_trans_strategy_classic;

    strategy = getenv("VIM_KEYCODE_TRANS_STRATEGY");
    if (strategy == NULL)
    {
	return;
    }

    if (STRICMP(strategy, "classic") == 0)
    {
	keycode_trans_strategy_used = &keycode_trans_strategy_classic;
	return;
    }

    if (STRICMP(strategy, "experimental") == 0)
    {
	keycode_trans_strategy_used = &keycode_trans_strategy_experimental;
	return;
    }

}

#if defined(FEAT_RENDER_OPTIONS) || defined(PROTO)
    int
gui_mch_set_rendering_options(char_u *s)
{
# ifdef FEAT_DIRECTX
    char_u  *p, *q;

    int	    dx_enable = 0;
    int	    dx_flags = 0;
    float   dx_gamma = 0.0f;
    float   dx_contrast = 0.0f;
    float   dx_level = 0.0f;
    int	    dx_geom = 0;
    int	    dx_renmode = 0;
    int	    dx_taamode = 0;

    // parse string as rendering options.
    for (p = s; p != NULL && *p != NUL; )
    {
	char_u  item[256];
	char_u  name[128];
	char_u  value[128];

	copy_option_part(&p, item, sizeof(item), ",");
	if (p == NULL)
	    break;
	q = &item[0];
	copy_option_part(&q, name, sizeof(name), ":");
	if (q == NULL)
	    return FAIL;
	copy_option_part(&q, value, sizeof(value), ":");

	if (STRCMP(name, "type") == 0)
	{
	    if (STRCMP(value, "directx") == 0)
		dx_enable = 1;
	    else
		return FAIL;
	}
	else if (STRCMP(name, "gamma") == 0)
	{
	    dx_flags |= 1 << 0;
	    dx_gamma = (float)atof((char *)value);
	}
	else if (STRCMP(name, "contrast") == 0)
	{
	    dx_flags |= 1 << 1;
	    dx_contrast = (float)atof((char *)value);
	}
	else if (STRCMP(name, "level") == 0)
	{
	    dx_flags |= 1 << 2;
	    dx_level = (float)atof((char *)value);
	}
	else if (STRCMP(name, "geom") == 0)
	{
	    dx_flags |= 1 << 3;
	    dx_geom = atoi((char *)value);
	    if (dx_geom < 0 || dx_geom > 2)
		return FAIL;
	}
	else if (STRCMP(name, "renmode") == 0)
	{
	    dx_flags |= 1 << 4;
	    dx_renmode = atoi((char *)value);
	    if (dx_renmode < 0 || dx_renmode > 6)
		return FAIL;
	}
	else if (STRCMP(name, "taamode") == 0)
	{
	    dx_flags |= 1 << 5;
	    dx_taamode = atoi((char *)value);
	    if (dx_taamode < 0 || dx_taamode > 3)
		return FAIL;
	}
	else if (STRCMP(name, "scrlines") == 0)
	{
	    // Deprecated.  Simply ignore it.
	}
	else
	    return FAIL;
    }

    if (!gui.in_use)
	return OK;  // only checking the syntax of the value

    // Enable DirectX/DirectWrite
    if (dx_enable)
    {
	if (!directx_enabled())
	    return FAIL;
	DWriteContext_SetRenderingParams(s_dwc, NULL);
	if (dx_flags)
	{
	    DWriteRenderingParams param;
	    DWriteContext_GetRenderingParams(s_dwc, &param);
	    if (dx_flags & (1 << 0))
		param.gamma = dx_gamma;
	    if (dx_flags & (1 << 1))
		param.enhancedContrast = dx_contrast;
	    if (dx_flags & (1 << 2))
		param.clearTypeLevel = dx_level;
	    if (dx_flags & (1 << 3))
		param.pixelGeometry = dx_geom;
	    if (dx_flags & (1 << 4))
		param.renderingMode = dx_renmode;
	    if (dx_flags & (1 << 5))
		param.textAntialiasMode = dx_taamode;
	    DWriteContext_SetRenderingParams(s_dwc, &param);
	}
    }
    s_directx_enabled = dx_enable;

    return OK;
# else
    return FAIL;
# endif
}
#endif

/*
 * These are new in Windows ME/XP, only defined in recent compilers.
 */
#ifndef HANDLE_WM_XBUTTONUP
# define HANDLE_WM_XBUTTONUP(hwnd, wParam, lParam, fn) \
   ((fn)((hwnd), (int)(short)LOWORD(lParam), (int)(short)HIWORD(lParam), (UINT)(wParam)), 0L)
#endif
#ifndef HANDLE_WM_XBUTTONDOWN
# define HANDLE_WM_XBUTTONDOWN(hwnd, wParam, lParam, fn) \
   ((fn)((hwnd), FALSE, (int)(short)LOWORD(lParam), (int)(short)HIWORD(lParam), (UINT)(wParam)), 0L)
#endif
#ifndef HANDLE_WM_XBUTTONDBLCLK
# define HANDLE_WM_XBUTTONDBLCLK(hwnd, wParam, lParam, fn) \
   ((fn)((hwnd), TRUE, (int)(short)LOWORD(lParam), (int)(short)HIWORD(lParam), (UINT)(wParam)), 0L)
#endif


#include "version.h"	// used by dialog box routine for default title
#ifdef DEBUG
# include <tchar.h>
#endif

// cproto fails on missing include files
#ifndef PROTO

# ifndef __MINGW32__
#  include <shellapi.h>
# endif
# include <commctrl.h>
# include <windowsx.h>

#endif // PROTO

#ifdef FEAT_MENU
# define MENUHINTS		// show menu hints in command line
#endif

// Some parameters for dialog boxes.  All in pixels.
#define DLG_PADDING_X		10
#define DLG_PADDING_Y		10
#define DLG_VERT_PADDING_X	4	// For vertical buttons
#define DLG_VERT_PADDING_Y	4
#define DLG_ICON_WIDTH		34
#define DLG_ICON_HEIGHT		34
#define DLG_MIN_WIDTH		150
#define DLG_FONT_NAME		"MS Shell Dlg"
#define DLG_FONT_POINT_SIZE	8
#define DLG_MIN_MAX_WIDTH	400
#define DLG_MIN_MAX_HEIGHT	400

#define DLG_NONBUTTON_CONTROL	5000	// First ID of non-button controls

#ifndef WM_DPICHANGED
# define WM_DPICHANGED			0x02E0
#endif

#ifndef WM_GETDPISCALEDSIZE
# define WM_GETDPISCALEDSIZE		0x02E4
#endif

#ifndef WM_MOUSEHWHEEL
# define WM_MOUSEHWHEEL			0x020E
#endif

#ifndef SPI_GETWHEELSCROLLCHARS
# define SPI_GETWHEELSCROLLCHARS	0x006C
#endif

#ifndef SPI_SETWHEELSCROLLCHARS
# define SPI_SETWHEELSCROLLCHARS	0x006D
#endif

#ifdef PROTO
/*
 * Define a few things for generating prototypes.  This is just to avoid
 * syntax errors, the defines do not need to be correct.
 */
# define APIENTRY
# define CALLBACK
# define CONST
# define FAR
# define NEAR
# define WINAPI
# undef _cdecl
# define _cdecl
typedef int BOOL;
typedef int BYTE;
typedef int DWORD;
typedef int WCHAR;
typedef int ENUMLOGFONT;
typedef int FINDREPLACE;
typedef int HANDLE;
typedef int HBITMAP;
typedef int HBRUSH;
typedef int HDROP;
typedef int INT;
typedef int LOGFONTW[];
typedef int LPARAM;
typedef int LPCREATESTRUCT;
typedef int LPCSTR;
typedef int LPCTSTR;
typedef int LPRECT;
typedef int LPSTR;
typedef int LPWINDOWPOS;
typedef int LPWORD;
typedef int LRESULT;
typedef int HRESULT;
# undef MSG
typedef int MSG;
typedef int NEWTEXTMETRIC;
typedef int NMHDR;
typedef int OSVERSIONINFO;
typedef int PWORD;
typedef int RECT;
typedef int SIZE;
typedef int UINT;
typedef int WORD;
typedef int WPARAM;
typedef int POINT;
typedef void *HINSTANCE;
typedef void *HMENU;
typedef void *HWND;
typedef void *HDC;
typedef void VOID;
typedef int LPNMHDR;
typedef int LONG;
typedef int WNDPROC;
typedef int UINT_PTR;
typedef int COLORREF;
typedef int HCURSOR;
#endif

static void _OnPaint(HWND hwnd);
static void fill_rect(const RECT *rcp, HBRUSH hbr, COLORREF color);
static void clear_rect(RECT *rcp);

static WORD		s_dlgfntheight;		// height of the dialog font
static WORD		s_dlgfntwidth;		// width of the dialog font

#ifdef FEAT_MENU
static HMENU		s_menuBar = NULL;
#endif
#ifdef FEAT_TEAROFF
static void rebuild_tearoff(vimmenu_T *menu);
static HBITMAP	s_htearbitmap;	    // bitmap used to indicate tearoff
#endif

// Flag that is set while processing a message that must not be interrupted by
// processing another message.
static int		s_busy_processing = FALSE;

static int		destroying = FALSE;	// call DestroyWindow() ourselves

#ifdef MSWIN_FIND_REPLACE
static UINT		s_findrep_msg = 0;
static FINDREPLACEW	s_findrep_struct;
static HWND		s_findrep_hwnd = NULL;
static int		s_findrep_is_find;	// TRUE for find dialog, FALSE
						// for find/replace dialog
#endif

HWND			s_hwnd = NULL;
static HDC		s_hdc = NULL;
static HBRUSH		s_brush = NULL;

#ifdef FEAT_TOOLBAR
static HWND		s_toolbarhwnd = NULL;
static WNDPROC		s_toolbar_wndproc = NULL;
#endif

#ifdef FEAT_GUI_TABLINE
static HWND		s_tabhwnd = NULL;
static WNDPROC		s_tabline_wndproc = NULL;
static int		showing_tabline = 0;
#endif

static WPARAM		s_wParam = 0;
static LPARAM		s_lParam = 0;

static HWND		s_textArea = NULL;
static UINT		s_uMsg = 0;

static char_u		*s_textfield; // Used by dialogs to pass back strings

static int		s_need_activate = FALSE;

// This variable is set when waiting for an event, which is the only moment
// scrollbar dragging can be done directly.  It's not allowed while commands
// are executed, because it may move the cursor and that may cause unexpected
// problems (e.g., while ":s" is working).
static int allow_scrollbar = FALSE;

#ifndef _DPI_AWARENESS_CONTEXTS_
typedef HANDLE DPI_AWARENESS_CONTEXT;

typedef enum DPI_AWARENESS {
    DPI_AWARENESS_INVALID	    = -1,
    DPI_AWARENESS_UNAWARE	    = 0,
    DPI_AWARENESS_SYSTEM_AWARE	    = 1,
    DPI_AWARENESS_PER_MONITOR_AWARE = 2
} DPI_AWARENESS;

# define DPI_AWARENESS_CONTEXT_UNAWARE		    ((DPI_AWARENESS_CONTEXT)-1)
# define DPI_AWARENESS_CONTEXT_SYSTEM_AWARE	    ((DPI_AWARENESS_CONTEXT)-2)
# define DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE    ((DPI_AWARENESS_CONTEXT)-3)
# define DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2 ((DPI_AWARENESS_CONTEXT)-4)
# define DPI_AWARENESS_CONTEXT_UNAWARE_GDISCALED    ((DPI_AWARENESS_CONTEXT)-5)
#endif

#define DEFAULT_DPI	96
static int		s_dpi = DEFAULT_DPI;
static BOOL		s_in_dpichanged = FALSE;
static DPI_AWARENESS	s_process_dpi_aware = DPI_AWARENESS_INVALID;
static RECT		s_suggested_rect;

static UINT (WINAPI *pGetDpiForSystem)(void) = NULL;
static UINT (WINAPI *pGetDpiForWindow)(HWND hwnd) = NULL;
static int (WINAPI *pGetSystemMetricsForDpi)(int, UINT) = NULL;
//static INT (WINAPI *pGetWindowDpiAwarenessContext)(HWND hwnd) = NULL;
static DPI_AWARENESS_CONTEXT (WINAPI *pSetThreadDpiAwarenessContext)(DPI_AWARENESS_CONTEXT dpiContext) = NULL;
static DPI_AWARENESS (WINAPI *pGetAwarenessFromDpiAwarenessContext)(DPI_AWARENESS_CONTEXT) = NULL;

    static int WINAPI
stubGetSystemMetricsForDpi(int nIndex, UINT dpi UNUSED)
{
    return GetSystemMetrics(nIndex);
}

    static int
adjust_fontsize_by_dpi(int size)
{
    return size * s_dpi / (int)pGetDpiForSystem();
}

    static int
adjust_by_system_dpi(int size)
{
    return size * (int)pGetDpiForSystem() / DEFAULT_DPI;
}

#if defined(FEAT_DIRECTX)
    static int
directx_enabled(void)
{
    if (s_dwc != NULL)
	return 1;
    else if (s_directx_load_attempted)
	return 0;
    // load DirectX
    DWrite_Init();
    s_directx_load_attempted = 1;
    s_dwc = DWriteContext_Open();
    directx_binddc();
    return s_dwc != NULL ? 1 : 0;
}

    static void
directx_binddc(void)
{
    if (s_textArea == NULL)
	return;

    RECT	rect;
    GetClientRect(s_textArea, &rect);
    DWriteContext_BindDC(s_dwc, s_hdc, &rect);
}
#endif

extern int current_font_height;	    // this is in os_mswin.c

static struct
{
    UINT    key_sym;
    char_u  vim_code0;
    char_u  vim_code1;
} special_keys[] =
{
    {VK_UP,		'k', 'u'},
    {VK_DOWN,		'k', 'd'},
    {VK_LEFT,		'k', 'l'},
    {VK_RIGHT,		'k', 'r'},

    {VK_F1,		'k', '1'},
    {VK_F2,		'k', '2'},
    {VK_F3,		'k', '3'},
    {VK_F4,		'k', '4'},
    {VK_F5,		'k', '5'},
    {VK_F6,		'k', '6'},
    {VK_F7,		'k', '7'},
    {VK_F8,		'k', '8'},
    {VK_F9,		'k', '9'},
    {VK_F10,		'k', ';'},

    {VK_F11,		'F', '1'},
    {VK_F12,		'F', '2'},
    {VK_F13,		'F', '3'},
    {VK_F14,		'F', '4'},
    {VK_F15,		'F', '5'},
    {VK_F16,		'F', '6'},
    {VK_F17,		'F', '7'},
    {VK_F18,		'F', '8'},
    {VK_F19,		'F', '9'},
    {VK_F20,		'F', 'A'},

    {VK_F21,		'F', 'B'},
#ifdef FEAT_NETBEANS_INTG
    {VK_PAUSE,		'F', 'B'},	// Pause == F21 (see gui_gtk_x11.c)
#endif
    {VK_F22,		'F', 'C'},
    {VK_F23,		'F', 'D'},
    {VK_F24,		'F', 'E'},	// winuser.h defines up to F24

    {VK_HELP,		'%', '1'},
    {VK_BACK,		'k', 'b'},
    {VK_INSERT,		'k', 'I'},
    {VK_DELETE,		'k', 'D'},
    {VK_HOME,		'k', 'h'},
    {VK_END,		'@', '7'},
    {VK_PRIOR,		'k', 'P'},
    {VK_NEXT,		'k', 'N'},
    {VK_PRINT,		'%', '9'},
    {VK_ADD,		'K', '6'},
    {VK_SUBTRACT,	'K', '7'},
    {VK_DIVIDE,		'K', '8'},
    {VK_MULTIPLY,	'K', '9'},
    {VK_SEPARATOR,	'K', 'A'},	// Keypad Enter
    {VK_DECIMAL,	'K', 'B'},

    {VK_NUMPAD0,	'K', 'C'},
    {VK_NUMPAD1,	'K', 'D'},
    {VK_NUMPAD2,	'K', 'E'},
    {VK_NUMPAD3,	'K', 'F'},
    {VK_NUMPAD4,	'K', 'G'},
    {VK_NUMPAD5,	'K', 'H'},
    {VK_NUMPAD6,	'K', 'I'},
    {VK_NUMPAD7,	'K', 'J'},
    {VK_NUMPAD8,	'K', 'K'},
    {VK_NUMPAD9,	'K', 'L'},

    // Keys that we want to be able to use any modifier with:
    {VK_SPACE,		' ', NUL},
    {VK_TAB,		TAB, NUL},
    {VK_ESCAPE,		ESC, NUL},
    {NL,		NL, NUL},
    {CAR,		CAR, NUL},

    // End of list marker:
    {0,			0, 0}
};

// Local variables
static int	s_button_pending = -1;

// s_getting_focus is set when we got focus but didn't see mouse-up event yet,
// so don't reset s_button_pending.
static int	s_getting_focus = FALSE;

static int	s_x_pending;
static int	s_y_pending;
static UINT	s_kFlags_pending;
static UINT_PTR	s_wait_timer = 0;	  // Timer for get char from user
static int	s_timed_out = FALSE;
static int	dead_key = DEAD_KEY_OFF;
static UINT	surrogate_pending_ch = 0; // 0: no surrogate pending,
					  // else a high surrogate

#ifdef FEAT_BEVAL_GUI
// balloon-eval WM_NOTIFY_HANDLER
static void Handle_WM_Notify(HWND hwnd, LPNMHDR pnmh);
static void track_user_activity(UINT uMsg);
#endif

/*
 * For control IME.
 *
 * These LOGFONTW used for IME.
 */
#ifdef FEAT_MBYTE_IME
// holds LOGFONTW for 'guifontwide' if available, otherwise 'guifont'
static LOGFONTW norm_logfont;
// holds LOGFONTW for 'guifont' always.
static LOGFONTW sub_logfont;
#endif

#ifdef FEAT_MBYTE_IME
static LRESULT _OnImeNotify(HWND hWnd, DWORD dwCommand, DWORD dwData);
#endif

#if defined(FEAT_BROWSE)
static char_u *convert_filter(char_u *s);
#endif

#ifdef DEBUG_PRINT_ERROR
/*
 * Print out the last Windows error message
 */
    static void
print_windows_error(void)
{
    LPVOID  lpMsgBuf;

    FormatMessage(FORMAT_MESSAGE_ALLOCATE_BUFFER | FORMAT_MESSAGE_FROM_SYSTEM,
		  NULL, GetLastError(),
		  MAKELANGID(LANG_NEUTRAL, SUBLANG_DEFAULT),
		  (LPTSTR) &lpMsgBuf, 0, NULL);
    TRACE1("Error: %s\n", lpMsgBuf);
    LocalFree(lpMsgBuf);
}
#endif

/*
 * Cursor blink functions.
 *
 * This is a simple state machine:
 * BLINK_NONE	not blinking at all
 * BLINK_OFF	blinking, cursor is not shown
 * BLINK_ON	blinking, cursor is shown
 */

#define BLINK_NONE  0
#define BLINK_OFF   1
#define BLINK_ON    2

static int		blink_state = BLINK_NONE;
static long_u		blink_waittime = 700;
static long_u		blink_ontime = 400;
static long_u		blink_offtime = 250;
static UINT_PTR		blink_timer = 0;

    int
gui_mch_is_blinking(void)
{
    return blink_state != BLINK_NONE;
}

    int
gui_mch_is_blink_off(void)
{
    return blink_state == BLINK_OFF;
}

    void
gui_mch_set_blinking(long wait, long on, long off)
{
    blink_waittime = wait;
    blink_ontime = on;
    blink_offtime = off;
}

    static VOID CALLBACK
_OnBlinkTimer(
    HWND hwnd,
    UINT uMsg UNUSED,
    UINT_PTR idEvent,
    DWORD dwTime UNUSED)
{
    MSG msg;

    /*
    TRACE2("Got timer event, id %d, blink_timer %d\n", idEvent, blink_timer);
    */

    KillTimer(NULL, idEvent);

    // Eat spurious WM_TIMER messages
    while (PeekMessageW(&msg, hwnd, WM_TIMER, WM_TIMER, PM_REMOVE))
	;

    if (blink_state == BLINK_ON)
    {
	gui_undraw_cursor();
	blink_state = BLINK_OFF;
	blink_timer = SetTimer(NULL, 0, (UINT)blink_offtime, _OnBlinkTimer);
    }
    else
    {
	gui_update_cursor(TRUE, FALSE);
	blink_state = BLINK_ON;
	blink_timer = SetTimer(NULL, 0, (UINT)blink_ontime, _OnBlinkTimer);
    }
    gui_mch_flush();
}

    static void
gui_mswin_rm_blink_timer(void)
{
    MSG msg;

    if (blink_timer == 0)
	return;

    KillTimer(NULL, blink_timer);
    // Eat spurious WM_TIMER messages
    while (PeekMessageW(&msg, s_hwnd, WM_TIMER, WM_TIMER, PM_REMOVE))
	;
    blink_timer = 0;
}

/*
 * Stop the cursor blinking.  Show the cursor if it wasn't shown.
 */
    void
gui_mch_stop_blink(int may_call_gui_update_cursor)
{
    gui_mswin_rm_blink_timer();
    if (blink_state == BLINK_OFF && may_call_gui_update_cursor)
    {
	gui_update_cursor(TRUE, FALSE);
	gui_mch_flush();
    }
    blink_state = BLINK_NONE;
}

/*
 * Start the cursor blinking.  If it was already blinking, this restarts the
 * waiting time and shows the cursor.
 */
    void
gui_mch_start_blink(void)
{
    gui_mswin_rm_blink_timer();

    // Only switch blinking on if none of the times is zero
    if (blink_waittime && blink_ontime && blink_offtime && gui.in_focus)
    {
	blink_timer = SetTimer(NULL, 0, (UINT)blink_waittime, _OnBlinkTimer);
	blink_state = BLINK_ON;
	gui_update_cursor(TRUE, FALSE);
	gui_mch_flush();
    }
}

/*
 * Call-back routines.
 */

    static VOID CALLBACK
_OnTimer(
    HWND hwnd,
    UINT uMsg UNUSED,
    UINT_PTR idEvent,
    DWORD dwTime UNUSED)
{
    MSG msg;

    /*
    TRACE2("Got timer event, id %d, s_wait_timer %d\n", idEvent, s_wait_timer);
    */
    KillTimer(NULL, idEvent);
    s_timed_out = TRUE;

    // Eat spurious WM_TIMER messages
    while (PeekMessageW(&msg, hwnd, WM_TIMER, WM_TIMER, PM_REMOVE))
	;
    if (idEvent == s_wait_timer)
	s_wait_timer = 0;
}

    static void
_OnDeadChar(
    HWND hwnd UNUSED,
    UINT ch UNUSED,
    int cRepeat UNUSED)
{
    dead_key = DEAD_KEY_SET_DEFAULT;
}

/*
 * Convert Unicode character "ch" to bytes in "string[slen]".
 * When "had_alt" is TRUE the ALT key was included in "ch".
 * Return the length.
 * Because the Windows API uses UTF-16, we have to deal with surrogate
 * pairs; this is where we choose to deal with them: if "ch" is a high
 * surrogate, it will be stored, and the length returned will be zero; the next
 * char_to_string call will then include the high surrogate, decoding the pair
 * of UTF-16 code units to a single Unicode code point, presuming it is the
 * matching low surrogate.
 */
    static int
char_to_string(int ch, char_u *string, int slen, int had_alt)
{
    int		len;
    int		i;
    WCHAR	wstring[2];
    char_u	*ws = NULL;

    if (surrogate_pending_ch != 0)
    {
	// We don't guarantee ch is a low surrogate to match the high surrogate
	// we already have; it should be, but if it isn't, tough luck.
	wstring[0] = surrogate_pending_ch;
	wstring[1] = ch;
	surrogate_pending_ch = 0;
	len = 2;
    }
    else if (ch >= 0xD800 && ch <= 0xDBFF)	// high surrogate
    {
	// We don't have the entire code point yet, only the first UTF-16 code
	// unit; so just remember it and use it in the next call.
	surrogate_pending_ch = ch;
	return 0;
    }
    else
    {
	wstring[0] = ch;
	len = 1;
    }

    // "ch" is a UTF-16 character.  Convert it to a string of bytes.  When
    // "enc_codepage" is non-zero use the standard Win32 function,
    // otherwise use our own conversion function (e.g., for UTF-8).
    if (enc_codepage > 0)
    {
	len = WideCharToMultiByte(enc_codepage, 0, wstring, len,
		(LPSTR)string, slen, 0, NULL);
	// If we had included the ALT key into the character but now the
	// upper bit is no longer set, that probably means the conversion
	// failed.  Convert the original character and set the upper bit
	// afterwards.
	if (had_alt && len == 1 && ch >= 0x80 && string[0] < 0x80)
	{
	    wstring[0] = ch & 0x7f;
	    len = WideCharToMultiByte(enc_codepage, 0, wstring, len,
		    (LPSTR)string, slen, 0, NULL);
	    if (len == 1) // safety check
		string[0] |= 0x80;
	}
    }
    else
    {
	ws = utf16_to_enc(wstring, &len);
	if (ws == NULL)
	    len = 0;
	else
	{
	    if (len > slen)	// just in case
		len = slen;
	    mch_memmove(string, ws, len);
	    vim_free(ws);
	}
    }

    if (len == 0)
    {
	string[0] = ch;
	len = 1;
    }

    for (i = 0; i < len; ++i)
	if (string[i] == CSI && len <= slen - 2)
	{
	    // Insert CSI as K_CSI.
	    mch_memmove(string + i + 3, string + i + 1, len - i - 1);
	    string[++i] = KS_EXTRA;
	    string[++i] = (int)KE_CSI;
	    len += 2;
	}

    return len;
}

/*
 * Experimental implementation, introduced in v8.2.4807
 * "processing key event in Win32 GUI is not ideal"
 *
 * TODO: since introduction, this experimental function started
 * to be used as well outside of original key press/processing
 * area, and usages not via "get_active_modifiers_via_ptr" should
 * be watched.
 */
    static int
get_active_modifiers_experimental(void)
{
    int modifiers = 0;

    if (GetKeyState(VK_CONTROL) & 0x8000)
	modifiers |= MOD_MASK_CTRL;
    if (GetKeyState(VK_SHIFT) & 0x8000)
	modifiers |= MOD_MASK_SHIFT;
    // Windows handles Ctrl + Alt as AltGr and vice-versa. We can distinguish
    // the two cases by checking whether the left or the right Alt key is
    // pressed.
    if (GetKeyState(VK_LMENU) & 0x8000)
	modifiers |= MOD_MASK_ALT;
    if ((modifiers & MOD_MASK_CTRL) && (GetKeyState(VK_RMENU) & 0x8000))
	modifiers &= ~MOD_MASK_CTRL;
    // Add RightALT only if it is hold alone (without Ctrl), because if AltGr
    // is pressed, Windows claims that Ctrl is hold as well. That way we can
    // recognize Right-ALT alone and be sure that not AltGr is hold.
    if (!(GetKeyState(VK_CONTROL) & 0x8000)
	    &&  (GetKeyState(VK_RMENU) & 0x8000)
	    && !(GetKeyState(VK_LMENU) & 0x8000)) // seems AltGr has both set
	modifiers |= MOD_MASK_ALT;

    return modifiers;
}

/*
 * "Classic" implementation, existing prior to v8.2.4807
 */
    static int
get_active_modifiers_classic(void)
{
    int modifiers = 0;

    if (GetKeyState(VK_SHIFT) & 0x8000)
	modifiers |= MOD_MASK_SHIFT;
    /*
     * Don't use caps-lock as shift, because these are special keys
     * being considered here, and we only want letters to get
     * shifted -- webb
     */
    /*
    if (GetKeyState(VK_CAPITAL) & 0x0001)
	modifiers ^= MOD_MASK_SHIFT;
    */
    if (GetKeyState(VK_CONTROL) & 0x8000)
	modifiers |= MOD_MASK_CTRL;
    if (GetKeyState(VK_MENU) & 0x8000)
	modifiers |= MOD_MASK_ALT;

    return modifiers;
}

    static int
get_active_modifiers(void)
{
    return get_active_modifiers_experimental();
}

    static int
get_active_modifiers_via_ptr(void)
{
    // marshal to corresponding implementation
    return keycode_trans_strategy_used->ptr_get_active_modifiers();
}

/*
 * Key hit, add it to the input buffer.
 */
    static void
_OnChar(
    HWND hwnd UNUSED,
    UINT cch,
    int cRepeat UNUSED)
{
    // marshal to corresponding implementation
    keycode_trans_strategy_used->ptr_on_char(hwnd, cch, cRepeat);
}

/*
 * Experimental implementation, introduced in v8.2.4807
 * "processing key event in Win32 GUI is not ideal"
 */
    static void
_OnChar_experimental(
    HWND hwnd UNUSED,
    UINT cch,
    int cRepeat UNUSED)
{
    char_u	string[40];
    int		len = 0;
    int		modifiers;
    int		ch = cch;   // special keys are negative

    if (dead_key == DEAD_KEY_SKIP_ON_CHAR)
	return;

    //  keep DEAD_KEY_TRANSIENT_IN_ON_CHAR value for later handling in
    //  process_message()
    if (dead_key != DEAD_KEY_TRANSIENT_IN_ON_CHAR)
	dead_key = DEAD_KEY_OFF;

    modifiers = get_active_modifiers_experimental();

    ch = simplify_key(ch, &modifiers);

    // Some keys need adjustment when the Ctrl modifier is used.
    ++no_reduce_keys;
    ch = may_adjust_key_for_ctrl(modifiers, ch);
    --no_reduce_keys;

    // remove the SHIFT modifier for keys where it's already included, e.g.,
    // '(' and '*'
    modifiers = may_remove_shift_modifier(modifiers, ch);

    // Unify modifiers somewhat.  No longer use ALT to set the 8th bit.
    ch = extract_modifiers(ch, &modifiers, FALSE, NULL);
    if (ch == CSI)
	ch = K_CSI;

    if (modifiers)
    {
	string[0] = CSI;
	string[1] = KS_MODIFIER;
	string[2] = modifiers;
	add_to_input_buf(string, 3);
    }

    len = char_to_string(ch, string, 40, FALSE);
    if (len == 1 && string[0] == Ctrl_C && ctrl_c_interrupts)
    {
	trash_input_buf();
	got_int = TRUE;
    }

    add_to_input_buf(string, len);
}

/*
 * "Classic" implementation, existing prior to v8.2.4807
 */
    static void
_OnChar_classic(
    HWND hwnd UNUSED,
    UINT ch,
    int cRepeat UNUSED)
{
    char_u	string[40];
    int		len = 0;

    dead_key = 0;

    len = char_to_string(ch, string, 40, FALSE);
    if (len == 1 && string[0] == Ctrl_C && ctrl_c_interrupts)
    {
	trash_input_buf();
	got_int = TRUE;
    }

    add_to_input_buf(string, len);
}

/*
 * Alt-Key hit, add it to the input buffer.
 */
    static void
_OnSysChar(
    HWND hwnd UNUSED,
    UINT cch,
    int cRepeat UNUSED)
{
    // marshal to corresponding implementation
    keycode_trans_strategy_used->ptr_on_sys_char(hwnd, cch, cRepeat);
}

/*
 * Experimental implementation, introduced in v8.2.4807
 * "processing key event in Win32 GUI is not ideal"
 */
    static void
_OnSysChar_experimental(
    HWND hwnd UNUSED,
    UINT cch,
    int cRepeat UNUSED)
{
    char_u	string[40]; // Enough for multibyte character
    int		len;
    int		modifiers;
    int		ch = cch;   // special keys are negative

    dead_key = DEAD_KEY_OFF;

    // OK, we have a character key (given by ch) which was entered with the
    // ALT key pressed. Eg, if the user presses Alt-A, then ch == 'A'. Note
    // that the system distinguishes Alt-a and Alt-A (Alt-Shift-a unless
    // CAPSLOCK is pressed) at this point.
    modifiers = get_active_modifiers_experimental();
    ch = simplify_key(ch, &modifiers);
    // remove the SHIFT modifier for keys where it's already included, e.g.,
    // '(' and '*'
    modifiers = may_remove_shift_modifier(modifiers, ch);

    // Unify modifiers somewhat.  No longer use ALT to set the 8th bit.
    ch = extract_modifiers(ch, &modifiers, FALSE, NULL);
    if (ch == CSI)
	ch = K_CSI;

    len = 0;
    if (modifiers)
    {
	string[len++] = CSI;
	string[len++] = KS_MODIFIER;
	string[len++] = modifiers;
    }

    if (IS_SPECIAL((int)ch))
    {
	string[len++] = CSI;
	string[len++] = K_SECOND((int)ch);
	string[len++] = K_THIRD((int)ch);
    }
    else
    {
	// Although the documentation isn't clear about it, we assume "ch" is
	// a Unicode character.
	len += char_to_string(ch, string + len, 40 - len, TRUE);
    }

    add_to_input_buf(string, len);
}

/*
 * "Classic" implementation, existing prior to v8.2.4807
 */
    static void
_OnSysChar_classic(
    HWND hwnd UNUSED,
    UINT cch,
    int cRepeat UNUSED)
{
    char_u	string[40]; // Enough for multibyte character
    int		len;
    int		modifiers;
    int		ch = cch;   // special keys are negative

    dead_key = 0;

    // TRACE("OnSysChar(%d, %c)\n", ch, ch);

    // OK, we have a character key (given by ch) which was entered with the
    // ALT key pressed. Eg, if the user presses Alt-A, then ch == 'A'. Note
    // that the system distinguishes Alt-a and Alt-A (Alt-Shift-a unless
    // CAPSLOCK is pressed) at this point.
    modifiers = MOD_MASK_ALT;
    if (GetKeyState(VK_SHIFT) & 0x8000)
	modifiers |= MOD_MASK_SHIFT;
    if (GetKeyState(VK_CONTROL) & 0x8000)
	modifiers |= MOD_MASK_CTRL;

    ch = simplify_key(ch, &modifiers);
    // remove the SHIFT modifier for keys where it's already included, e.g.,
    // '(' and '*'
    modifiers = may_remove_shift_modifier(modifiers, ch);

    // Unify modifiers somewhat.  No longer use ALT to set the 8th bit.
    ch = extract_modifiers(ch, &modifiers, FALSE, NULL);
    if (ch == CSI)
	ch = K_CSI;

    len = 0;
    if (modifiers)
    {
	string[len++] = CSI;
	string[len++] = KS_MODIFIER;
	string[len++] = modifiers;
    }

    if (IS_SPECIAL((int)ch))
    {
	string[len++] = CSI;
	string[len++] = K_SECOND((int)ch);
	string[len++] = K_THIRD((int)ch);
    }
    else
    {
	// Although the documentation isn't clear about it, we assume "ch" is
	// a Unicode character.
	len += char_to_string(ch, string + len, 40 - len, TRUE);
    }

    add_to_input_buf(string, len);
}

    static void
_OnMouseEvent(
    int button,
    int x,
    int y,
    int repeated_click,
    UINT keyFlags)
{
    int vim_modifiers = 0x0;

    s_getting_focus = FALSE;

    if (keyFlags & MK_SHIFT)
	vim_modifiers |= MOUSE_SHIFT;
    if (keyFlags & MK_CONTROL)
	vim_modifiers |= MOUSE_CTRL;
    if (GetKeyState(VK_LMENU) & 0x8000)
	vim_modifiers |= MOUSE_ALT;

    gui_send_mouse_event(button, x, y, repeated_click, vim_modifiers);
}

    static void
_OnMouseButtonDown(
    HWND hwnd UNUSED,
    BOOL fDoubleClick UNUSED,
    int x,
    int y,
    UINT keyFlags)
{
    static LONG	s_prevTime = 0;

    LONG    currentTime = GetMessageTime();
    int	    button = -1;
    int	    repeated_click;

    // Give main window the focus: this is so the cursor isn't hollow.
    (void)SetFocus(s_hwnd);

    if (s_uMsg == WM_LBUTTONDOWN || s_uMsg == WM_LBUTTONDBLCLK)
	button = MOUSE_LEFT;
    else if (s_uMsg == WM_MBUTTONDOWN || s_uMsg == WM_MBUTTONDBLCLK)
	button = MOUSE_MIDDLE;
    else if (s_uMsg == WM_RBUTTONDOWN || s_uMsg == WM_RBUTTONDBLCLK)
	button = MOUSE_RIGHT;
    else if (s_uMsg == WM_XBUTTONDOWN || s_uMsg == WM_XBUTTONDBLCLK)
    {
	button = ((GET_XBUTTON_WPARAM(s_wParam) == 1) ? MOUSE_X1 : MOUSE_X2);
    }
    else if (s_uMsg == WM_CAPTURECHANGED)
    {
	// on W95/NT4, somehow you get in here with an odd Msg
	// if you press one button while holding down the other..
	if (s_button_pending == MOUSE_LEFT)
	    button = MOUSE_RIGHT;
	else
	    button = MOUSE_LEFT;
    }

    if (button < 0)
	return;

    repeated_click = ((int)(currentTime - s_prevTime) < p_mouset);

    /*
     * Holding down the left and right buttons simulates pushing the middle
     * button.
     */
    if (repeated_click
	    && ((button == MOUSE_LEFT && s_button_pending == MOUSE_RIGHT)
		|| (button == MOUSE_RIGHT
		    && s_button_pending == MOUSE_LEFT)))
    {
	/*
	 * Hmm, gui.c will ignore more than one button down at a time, so
	 * pretend we let go of it first.
	 */
	gui_send_mouse_event(MOUSE_RELEASE, x, y, FALSE, 0x0);
	button = MOUSE_MIDDLE;
	repeated_click = FALSE;
	s_button_pending = -1;
	_OnMouseEvent(button, x, y, repeated_click, keyFlags);
    }
    else if ((repeated_click)
	    || (mouse_model_popup() && (button == MOUSE_RIGHT)))
    {
	if (s_button_pending > -1)
	{
	    _OnMouseEvent(s_button_pending, x, y, FALSE, keyFlags);
	    s_button_pending = -1;
	}
	// TRACE("Button down at x %d, y %d\n", x, y);
	_OnMouseEvent(button, x, y, repeated_click, keyFlags);
    }
    else
    {
	/*
	 * If this is the first press (i.e. not a multiple click) don't
	 * action immediately, but store and wait for:
	 * i) button-up
	 * ii) mouse move
	 * iii) another button press
	 * before using it.
	 * This enables us to make left+right simulate middle button,
	 * without left or right being actioned first.  The side-effect is
	 * that if you click and hold the mouse without dragging, the
	 * cursor doesn't move until you release the button. In practice
	 * this is hardly a problem.
	 */
	s_button_pending = button;
	s_x_pending = x;
	s_y_pending = y;
	s_kFlags_pending = keyFlags;
    }

    s_prevTime = currentTime;
}

    static void
_OnMouseMoveOrRelease(
    HWND hwnd UNUSED,
    int x,
    int y,
    UINT keyFlags)
{
    int button;

    s_getting_focus = FALSE;
    if (s_button_pending > -1)
    {
	// Delayed action for mouse down event
	_OnMouseEvent(s_button_pending, s_x_pending,
					s_y_pending, FALSE, s_kFlags_pending);
	s_button_pending = -1;
    }
    if (s_uMsg == WM_MOUSEMOVE)
    {
	/*
	 * It's only a MOUSE_DRAG if one or more mouse buttons are being held
	 * down.
	 */
	if (!(keyFlags & (MK_LBUTTON | MK_MBUTTON | MK_RBUTTON
						| MK_XBUTTON1 | MK_XBUTTON2)))
	{
	    gui_mouse_moved(x, y);
	    return;
	}

	/*
	 * While button is down, keep grabbing mouse move events when
	 * the mouse goes outside the window
	 */
	SetCapture(s_textArea);
	button = MOUSE_DRAG;
	// TRACE("  move at x %d, y %d\n", x, y);
    }
    else
    {
	ReleaseCapture();
	button = MOUSE_RELEASE;
	// TRACE("  up at x %d, y %d\n", x, y);
    }

    _OnMouseEvent(button, x, y, FALSE, keyFlags);
}

    static void
_OnSizeTextArea(
    HWND hwnd UNUSED,
    UINT state UNUSED,
    int cx UNUSED,
    int cy UNUSED)
{
#if defined(FEAT_DIRECTX)
    if (IS_ENABLE_DIRECTX())
	directx_binddc();
#endif
}

#ifdef FEAT_MENU
/*
 * Find the vimmenu_T with the given id
 */
    static vimmenu_T *
gui_mswin_find_menu(
    vimmenu_T	*pMenu,
    int		id)
{
    vimmenu_T	*pChildMenu;

    while (pMenu)
    {
	if (pMenu->id == (UINT)id)
	    break;
	if (pMenu->children != NULL)
	{
	    pChildMenu = gui_mswin_find_menu(pMenu->children, id);
	    if (pChildMenu)
	    {
		pMenu = pChildMenu;
		break;
	    }
	}
	pMenu = pMenu->next;
    }
    return pMenu;
}

    static void
_OnMenu(
    HWND	hwnd UNUSED,
    int		id,
    HWND	hwndCtl UNUSED,
    UINT	codeNotify UNUSED)
{
    vimmenu_T	*pMenu;

    pMenu = gui_mswin_find_menu(root_menu, id);
    if (pMenu)
	gui_menu_cb(pMenu);
}
#endif

#ifdef MSWIN_FIND_REPLACE
/*
 * Handle a Find/Replace window message.
 */
    static void
_OnFindRepl(void)
{
    int	    flags = 0;
    int	    down;

    if (s_findrep_struct.Flags & FR_DIALOGTERM)
	// Give main window the focus back.
	(void)SetFocus(s_hwnd);

    if (s_findrep_struct.Flags & FR_FINDNEXT)
    {
	flags = FRD_FINDNEXT;

	// Give main window the focus back: this is so the cursor isn't
	// hollow.
	(void)SetFocus(s_hwnd);
    }
    else if (s_findrep_struct.Flags & FR_REPLACE)
    {
	flags = FRD_REPLACE;

	// Give main window the focus back: this is so the cursor isn't
	// hollow.
	(void)SetFocus(s_hwnd);
    }
    else if (s_findrep_struct.Flags & FR_REPLACEALL)
    {
	flags = FRD_REPLACEALL;
    }

    if (flags == 0)
	return;

    char_u	*p, *q;

    // Call the generic GUI function to do the actual work.
    if (s_findrep_struct.Flags & FR_WHOLEWORD)
	flags |= FRD_WHOLE_WORD;
    if (s_findrep_struct.Flags & FR_MATCHCASE)
	flags |= FRD_MATCH_CASE;
    down = (s_findrep_struct.Flags & FR_DOWN) != 0;
    p = utf16_to_enc(s_findrep_struct.lpstrFindWhat, NULL);
    q = utf16_to_enc(s_findrep_struct.lpstrReplaceWith, NULL);
    if (p != NULL && q != NULL)
	gui_do_findrepl(flags, p, q, down);
    vim_free(p);
    vim_free(q);
}
#endif

    static void
HandleMouseHide(UINT uMsg, LPARAM lParam)
{
    static LPARAM last_lParam = 0L;

    // We sometimes get a mousemove when the mouse didn't move...
    if (uMsg == WM_MOUSEMOVE || uMsg == WM_NCMOUSEMOVE)
    {
	if (lParam == last_lParam)
	    return;
	last_lParam = lParam;
    }

    // Handle specially, to centralise coding. We need to be sure we catch all
    // possible events which should cause us to restore the cursor (as it is a
    // shared resource, we take full responsibility for it).
    switch (uMsg)
    {
    case WM_KEYUP:
    case WM_CHAR:
	/*
	 * blank out the pointer if necessary
	 */
	if (p_mh)
	    gui_mch_mousehide(TRUE);
	break;

    case WM_SYSKEYUP:	 // show the pointer when a system-key is pressed
    case WM_SYSCHAR:
    case WM_MOUSEMOVE:	 // show the pointer on any mouse action
    case WM_LBUTTONDOWN:
    case WM_LBUTTONUP:
    case WM_MBUTTONDOWN:
    case WM_MBUTTONUP:
    case WM_RBUTTONDOWN:
    case WM_RBUTTONUP:
    case WM_XBUTTONDOWN:
    case WM_XBUTTONUP:
    case WM_NCMOUSEMOVE:
    case WM_NCLBUTTONDOWN:
    case WM_NCLBUTTONUP:
    case WM_NCMBUTTONDOWN:
    case WM_NCMBUTTONUP:
    case WM_NCRBUTTONDOWN:
    case WM_NCRBUTTONUP:
    case WM_KILLFOCUS:
	/*
	 * if the pointer is currently hidden, then we should show it.
	 */
	gui_mch_mousehide(FALSE);
	break;
    }
}

    static LRESULT CALLBACK
_TextAreaWndProc(
    HWND hwnd,
    UINT uMsg,
    WPARAM wParam,
    LPARAM lParam)
{
    /*
    TRACE("TextAreaWndProc: hwnd = %08x, msg = %x, wParam = %x, lParam = %x\n",
	  hwnd, uMsg, wParam, lParam);
    */

    HandleMouseHide(uMsg, lParam);

    s_uMsg = uMsg;
    s_wParam = wParam;
    s_lParam = lParam;

#ifdef FEAT_BEVAL_GUI
    track_user_activity(uMsg);
#endif

    switch (uMsg)
    {
	HANDLE_MSG(hwnd, WM_LBUTTONDBLCLK,_OnMouseButtonDown);
	HANDLE_MSG(hwnd, WM_LBUTTONDOWN,_OnMouseButtonDown);
	HANDLE_MSG(hwnd, WM_LBUTTONUP,	_OnMouseMoveOrRelease);
	HANDLE_MSG(hwnd, WM_MBUTTONDBLCLK,_OnMouseButtonDown);
	HANDLE_MSG(hwnd, WM_MBUTTONDOWN,_OnMouseButtonDown);
	HANDLE_MSG(hwnd, WM_MBUTTONUP,	_OnMouseMoveOrRelease);
	HANDLE_MSG(hwnd, WM_MOUSEMOVE,	_OnMouseMoveOrRelease);
	HANDLE_MSG(hwnd, WM_PAINT,	_OnPaint);
	HANDLE_MSG(hwnd, WM_RBUTTONDBLCLK,_OnMouseButtonDown);
	HANDLE_MSG(hwnd, WM_RBUTTONDOWN,_OnMouseButtonDown);
	HANDLE_MSG(hwnd, WM_RBUTTONUP,	_OnMouseMoveOrRelease);
	HANDLE_MSG(hwnd, WM_XBUTTONDBLCLK,_OnMouseButtonDown);
	HANDLE_MSG(hwnd, WM_XBUTTONDOWN,_OnMouseButtonDown);
	HANDLE_MSG(hwnd, WM_XBUTTONUP,	_OnMouseMoveOrRelease);
	HANDLE_MSG(hwnd, WM_SIZE,	_OnSizeTextArea);

#ifdef FEAT_BEVAL_GUI
	case WM_NOTIFY: Handle_WM_Notify(hwnd, (LPNMHDR)lParam);
	    return TRUE;
#endif
	default:
	    return DefWindowProcW(hwnd, uMsg, wParam, lParam);
    }
}

/*
 * Called when the foreground or background color has been changed.
 */
    void
gui_mch_new_colors(void)
{
    HBRUSH prevBrush;

    s_brush = CreateSolidBrush(gui.back_pixel);
    prevBrush = (HBRUSH)SetClassLongPtr(
				s_hwnd, GCLP_HBRBACKGROUND, (LONG_PTR)s_brush);
    InvalidateRect(s_hwnd, NULL, TRUE);
    DeleteObject(prevBrush);
}

/*
 * Set the colors to their default values.
 */
    void
gui_mch_def_colors(void)
{
    gui.norm_pixel = GetSysColor(COLOR_WINDOWTEXT);
    gui.back_pixel = GetSysColor(COLOR_WINDOW);
    gui.def_norm_pixel = gui.norm_pixel;
    gui.def_back_pixel = gui.back_pixel;
}

/*
 * Open the GUI window which was created by a call to gui_mch_init().
 */
    int
gui_mch_open(void)
{
    // Actually open the window, if not already visible
    // (may be done already in gui_mch_set_shellsize)
    if (!IsWindowVisible(s_hwnd))
	ShowWindow(s_hwnd, SW_SHOWDEFAULT);

#ifdef MSWIN_FIND_REPLACE
    // Init replace string here, so that we keep it when re-opening the
    // dialog.
    s_findrep_struct.lpstrReplaceWith[0] = NUL;
#endif

    return OK;
}

/*
 * Get the position of the top left corner of the window.
 */
    int
gui_mch_get_winpos(int *x, int *y)
{
    RECT    rect;

    GetWindowRect(s_hwnd, &rect);
    *x = rect.left;
    *y = rect.top;
    return OK;
}

/*
 * Set the position of the top left corner of the window to the given
 * coordinates.
 */
    void
gui_mch_set_winpos(int x, int y)
{
    SetWindowPos(s_hwnd, NULL, x, y, 0, 0,
		 SWP_NOZORDER | SWP_NOSIZE | SWP_NOACTIVATE);
}
    void
gui_mch_set_text_area_pos(int x, int y, int w, int h)
{
    static int oldx = 0;
    static int oldy = 0;

    SetWindowPos(s_textArea, NULL, x, y, w, h, SWP_NOZORDER | SWP_NOACTIVATE);

#ifdef FEAT_TOOLBAR
    if (vim_strchr(p_go, GO_TOOLBAR) != NULL)
	SendMessage(s_toolbarhwnd, WM_SIZE,
	      (WPARAM)0, MAKELPARAM(w, gui.toolbar_height));
#endif
#if defined(FEAT_GUI_TABLINE)
    if (showing_tabline)
    {
	int	top = 0;
	RECT	rect;

# ifdef FEAT_TOOLBAR
	if (vim_strchr(p_go, GO_TOOLBAR) != NULL)
	    top = gui.toolbar_height;
# endif
	GetClientRect(s_hwnd, &rect);
	MoveWindow(s_tabhwnd, 0, top, rect.right, gui.tabline_height, TRUE);
    }
#endif

    // When side scroll bar is unshown, the size of window will change.
    // then, the text area move left or right. thus client rect should be
    // forcedly redrawn. (Yasuhiro Matsumoto)
    if (oldx != x || oldy != y)
    {
	InvalidateRect(s_hwnd, NULL, FALSE);
	oldx = x;
	oldy = y;
    }
}


/*
 * Scrollbar stuff:
 */

    void
gui_mch_enable_scrollbar(
    scrollbar_T     *sb,
    int		    flag)
{
    ShowScrollBar(sb->id, SB_CTL, flag);

    // TODO: When the window is maximized, the size of the window stays the
    // same, thus the size of the text area changes.  On Win98 it's OK, on Win
    // NT 4.0 it's not...
}

    void
gui_mch_set_scrollbar_pos(
    scrollbar_T *sb,
    int		x,
    int		y,
    int		w,
    int		h)
{
    SetWindowPos(sb->id, NULL, x, y, w, h,
			      SWP_NOZORDER | SWP_NOACTIVATE | SWP_SHOWWINDOW);
}

    int
gui_mch_get_scrollbar_xpadding(void)
{
    RECT    rcTxt, rcWnd;
    int	    xpad;

    GetWindowRect(s_textArea, &rcTxt);
    GetWindowRect(s_hwnd, &rcWnd);
    xpad = rcWnd.right - rcTxt.right - gui.scrollbar_width
	- pGetSystemMetricsForDpi(SM_CXFRAME, s_dpi)
	- pGetSystemMetricsForDpi(SM_CXPADDEDBORDER, s_dpi);
    return (xpad < 0) ? 0 : xpad;
}

    int
gui_mch_get_scrollbar_ypadding(void)
{
    RECT    rcTxt, rcWnd;
    int	    ypad;

    GetWindowRect(s_textArea, &rcTxt);
    GetWindowRect(s_hwnd, &rcWnd);
    ypad = rcWnd.bottom - rcTxt.bottom - gui.scrollbar_height
	- pGetSystemMetricsForDpi(SM_CYFRAME, s_dpi)
	- pGetSystemMetricsForDpi(SM_CXPADDEDBORDER, s_dpi);
    return (ypad < 0) ? 0 : ypad;
}

    void
gui_mch_create_scrollbar(
    scrollbar_T *sb,
    int		orient)	// SBAR_VERT or SBAR_HORIZ
{
    sb->id = CreateWindow(
	"SCROLLBAR", "Scrollbar",
	WS_CHILD | ((orient == SBAR_VERT) ? SBS_VERT : SBS_HORZ), 0, 0,
	10,				// Any value will do for now
	10,				// Any value will do for now
	s_hwnd, NULL,
	g_hinst, NULL);
}

/*
 * Find the scrollbar with the given hwnd.
 */
    static scrollbar_T *
gui_mswin_find_scrollbar(HWND hwnd)
{
    win_T	*wp;

    if (gui.bottom_sbar.id == hwnd)
	return &gui.bottom_sbar;
    FOR_ALL_WINDOWS(wp)
    {
	if (wp->w_scrollbars[SBAR_LEFT].id == hwnd)
	    return &wp->w_scrollbars[SBAR_LEFT];
	if (wp->w_scrollbars[SBAR_RIGHT].id == hwnd)
	    return &wp->w_scrollbars[SBAR_RIGHT];
    }
    return NULL;
}

    static void
update_scrollbar_size(void)
{
    gui.scrollbar_width = pGetSystemMetricsForDpi(SM_CXVSCROLL, s_dpi);
    gui.scrollbar_height = pGetSystemMetricsForDpi(SM_CYHSCROLL, s_dpi);
}

/*
 * Get the average character size of a font.
 */
    static void
GetAverageFontSize(HDC hdc, SIZE *size)
{
    // GetTextMetrics() may not return the right value in tmAveCharWidth
    // for some fonts.  Do our own average computation.
    GetTextExtentPoint(hdc,
	    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
	    52, size);
    size->cx = (size->cx / 26 + 1) / 2;
}

/*
 * Get the character size of a font.
 */
    static void
GetFontSize(GuiFont font, int *char_width, int *char_height)
{
    HWND    hwnd = GetDesktopWindow();
    HDC	    hdc = GetWindowDC(hwnd);
    HFONT   hfntOld = SelectFont(hdc, (HFONT)font);
    SIZE    size;
    TEXTMETRIC tm;

    GetTextMetrics(hdc, &tm);
    GetAverageFontSize(hdc, &size);

    if (char_width)
	*char_width = size.cx + tm.tmOverhang;
    if (char_height)
	*char_height = tm.tmHeight + p_linespace;

    SelectFont(hdc, hfntOld);

    ReleaseDC(hwnd, hdc);
}

/*
 * Update the character size in "gui" structure with the specified font.
 */
    static void
UpdateFontSize(GuiFont font)
{
    GetFontSize(font, &gui.char_width, &gui.char_height);
}

/*
 * Adjust gui.char_height (after 'linespace' was changed).
 */
    int
gui_mch_adjust_charheight(void)
{
    UpdateFontSize(gui.norm_font);
    return OK;
}

    static GuiFont
get_font_handle(LOGFONTW *lf)
{
    HFONT   font = NULL;

    // Load the font
    font = CreateFontIndirectW(lf);

    if (font == NULL)
	return NOFONT;

    return (GuiFont)font;
}

    static int
pixels_to_points(int pixels, int vertical)
{
    int		points;
    HWND	hwnd;
    HDC		hdc;

    hwnd = GetDesktopWindow();
    hdc = GetWindowDC(hwnd);

    points = MulDiv(pixels, 72,
		    GetDeviceCaps(hdc, vertical ? LOGPIXELSY : LOGPIXELSX));

    ReleaseDC(hwnd, hdc);

    return points;
}

    GuiFont
gui_mch_get_font(
    char_u	*name,
    int		giveErrorIfMissing)
{
    LOGFONTW	lf;
    GuiFont	font = NOFONT;

    if (get_logfont(&lf, name, NULL, giveErrorIfMissing) == OK)
    {
	lf.lfHeight = adjust_fontsize_by_dpi(lf.lfHeight);
	font = get_font_handle(&lf);
    }
    if (font == NOFONT && giveErrorIfMissing)
	semsg(_(e_unknown_font_str), name);
    return font;
}

#if defined(FEAT_EVAL) || defined(PROTO)
/*
 * Return the name of font "font" in allocated memory.
 * Don't know how to get the actual name, thus use the provided name.
 */
    char_u *
gui_mch_get_fontname(GuiFont font UNUSED, char_u *name)
{
    if (name == NULL)
	return NULL;
    return vim_strsave(name);
}
#endif

    void
gui_mch_free_font(GuiFont font)
{
    if (font)
	DeleteObject((HFONT)font);
}

/*
 * Return the Pixel value (color) for the given color name.
 * Return INVALCOLOR for error.
 */
    guicolor_T
gui_mch_get_color(char_u *name)
{
    int i;

    typedef struct SysColorTable
    {
	char	    *name;
	int	    color;
    } SysColorTable;

    static SysColorTable sys_table[] =
    {
	{"SYS_3DDKSHADOW", COLOR_3DDKSHADOW},
	{"SYS_3DHILIGHT", COLOR_3DHILIGHT},
#ifdef COLOR_3DHIGHLIGHT
	{"SYS_3DHIGHLIGHT", COLOR_3DHIGHLIGHT},
#endif
	{"SYS_BTNHILIGHT", COLOR_BTNHILIGHT},
	{"SYS_BTNHIGHLIGHT", COLOR_BTNHIGHLIGHT},
	{"SYS_3DLIGHT", COLOR_3DLIGHT},
	{"SYS_3DSHADOW", COLOR_3DSHADOW},
	{"SYS_DESKTOP", COLOR_DESKTOP},
	{"SYS_INFOBK", COLOR_INFOBK},
	{"SYS_INFOTEXT", COLOR_INFOTEXT},
	{"SYS_3DFACE", COLOR_3DFACE},
	{"SYS_BTNFACE", COLOR_BTNFACE},
	{"SYS_BTNSHADOW", COLOR_BTNSHADOW},
	{"SYS_ACTIVEBORDER", COLOR_ACTIVEBORDER},
	{"SYS_ACTIVECAPTION", COLOR_ACTIVECAPTION},
	{"SYS_APPWORKSPACE", COLOR_APPWORKSPACE},
	{"SYS_BACKGROUND", COLOR_BACKGROUND},
	{"SYS_BTNTEXT", COLOR_BTNTEXT},
	{"SYS_CAPTIONTEXT", COLOR_CAPTIONTEXT},
	{"SYS_GRAYTEXT", COLOR_GRAYTEXT},
	{"SYS_HIGHLIGHT", COLOR_HIGHLIGHT},
	{"SYS_HIGHLIGHTTEXT", COLOR_HIGHLIGHTTEXT},
	{"SYS_INACTIVEBORDER", COLOR_INACTIVEBORDER},
	{"SYS_INACTIVECAPTION", COLOR_INACTIVECAPTION},
	{"SYS_INACTIVECAPTIONTEXT", COLOR_INACTIVECAPTIONTEXT},
	{"SYS_MENU", COLOR_MENU},
	{"SYS_MENUTEXT", COLOR_MENUTEXT},
	{"SYS_SCROLLBAR", COLOR_SCROLLBAR},
	{"SYS_WINDOW", COLOR_WINDOW},
	{"SYS_WINDOWFRAME", COLOR_WINDOWFRAME},
	{"SYS_WINDOWTEXT", COLOR_WINDOWTEXT}
    };

    /*
     * Try to look up a system colour.
     */
    for (i = 0; i < (int)ARRAY_LENGTH(sys_table); i++)
	if (STRICMP(name, sys_table[i].name) == 0)
	    return GetSysColor(sys_table[i].color);

    return gui_get_color_cmn(name);
}

    guicolor_T
gui_mch_get_rgb_color(int r, int g, int b)
{
    return gui_get_rgb_color_cmn(r, g, b);
}

/*
 * Return OK if the key with the termcap name "name" is supported.
 */
    int
gui_mch_haskey(char_u *name)
{
    int i;

    for (i = 0; special_keys[i].vim_code1 != NUL; i++)
	if (name[0] == special_keys[i].vim_code0
				       && name[1] == special_keys[i].vim_code1)
	    return OK;
    return FAIL;
}

    void
gui_mch_beep(void)
{
    MessageBeep((UINT)-1);
}
/*
 * Invert a rectangle from row r, column c, for nr rows and nc columns.
 */
    void
gui_mch_invert_rectangle(
    int	    r,
    int	    c,
    int	    nr,
    int	    nc)
{
    RECT    rc;

#if defined(FEAT_DIRECTX)
    if (IS_ENABLE_DIRECTX())
	DWriteContext_Flush(s_dwc);
#endif

    /*
     * Note: InvertRect() excludes right and bottom of rectangle.
     */
    rc.left = FILL_X(c);
    rc.top = FILL_Y(r);
    rc.right = rc.left + nc * gui.char_width;
    rc.bottom = rc.top + nr * gui.char_height;
    InvertRect(s_hdc, &rc);
}

/*
 * Iconify the GUI window.
 */
    void
gui_mch_iconify(void)
{
    ShowWindow(s_hwnd, SW_MINIMIZE);
}

/*
 * Draw a cursor without focus.
 */
    void
gui_mch_draw_hollow_cursor(guicolor_T color)
{
    HBRUSH  hbr;
    RECT    rc;

#if defined(FEAT_DIRECTX)
    if (IS_ENABLE_DIRECTX())
	DWriteContext_Flush(s_dwc);
#endif

    /*
     * Note: FrameRect() excludes right and bottom of rectangle.
     */
    rc.left = FILL_X(gui.col);
    rc.top = FILL_Y(gui.row);
    rc.right = rc.left + gui.char_width;
    if (mb_lefthalve(gui.row, gui.col))
	rc.right += gui.char_width;
    rc.bottom = rc.top + gui.char_height;
    hbr = CreateSolidBrush(color);
    FrameRect(s_hdc, &rc, hbr);
    DeleteBrush(hbr);
}
/*
 * Draw part of a cursor, "w" pixels wide, and "h" pixels high, using
 * color "color".
 */
    void
gui_mch_draw_part_cursor(
    int		w,
    int		h,
    guicolor_T	color)
{
    RECT	rc;

    /*
     * Note: FillRect() excludes right and bottom of rectangle.
     */
    rc.left =
#ifdef FEAT_RIGHTLEFT
		// vertical line should be on the right of current point
		CURSOR_BAR_RIGHT ? FILL_X(gui.col + 1) - w :
#endif
		    FILL_X(gui.col);
    rc.top = FILL_Y(gui.row) + gui.char_height - h;
    rc.right = rc.left + w;
    rc.bottom = rc.top + h;

    fill_rect(&rc, NULL, color);
}


/*
 * Generates a VK_SPACE when the internal dead_key flag is set to output the
 * dead key's nominal character and re-post the original message.
 */
    static void
outputDeadKey_rePost_Ex(MSG originalMsg, int dead_key2set)
{
    static MSG deadCharExpel;

    if (dead_key == DEAD_KEY_OFF)
	return;

    dead_key = dead_key2set;

    // Make Windows generate the dead key's character
    deadCharExpel.message = originalMsg.message;
    deadCharExpel.hwnd    = originalMsg.hwnd;
    deadCharExpel.wParam  = VK_SPACE;

    TranslateMessage(&deadCharExpel);

    // re-generate the current character free of the dead char influence
    PostMessage(originalMsg.hwnd, originalMsg.message, originalMsg.wParam,
							  originalMsg.lParam);
}

/*
 * Wrapper for outputDeadKey_rePost_Ex which always reset dead_key value.
 */
    static void
outputDeadKey_rePost(MSG originalMsg)
{
    outputDeadKey_rePost_Ex(originalMsg, DEAD_KEY_OFF);
}

/*
 * Refactored out part of process_message(), responsible for
 * handling the case of "not a special key"
 */
static void process_message_usual_key(UINT vk, const MSG *pmsg)
{
    // marshal to corresponding implementation
    keycode_trans_strategy_used->ptr_process_message_usual_key(vk, pmsg);
}

/*
 * Experimental implementation, introduced in v8.2.4807
 * "processing key event in Win32 GUI is not ideal"
 */
static void process_message_usual_key_experimental(UINT vk, const MSG *pmsg)
{
    WCHAR	ch[8];
    int		len;
    int		i;
    UINT	scan_code;
    BYTE	keyboard_state[256];

    // Construct the state table with only a few modifiers, we don't
    // really care about the presence of Ctrl/Alt as those modifiers are
    // handled by Vim separately.
    memset(keyboard_state, 0, 256);
    if (GetKeyState(VK_SHIFT) & 0x8000)
	keyboard_state[VK_SHIFT] = 0x80;
    if (GetKeyState(VK_CAPITAL) & 0x0001)
	keyboard_state[VK_CAPITAL] = 0x01;
    // Alt-Gr is synthesized as Alt + Ctrl.
    if ((GetKeyState(VK_RMENU) & 0x8000)
				 && (GetKeyState(VK_CONTROL) & 0x8000))
    {
	keyboard_state[VK_MENU] = 0x80;
	keyboard_state[VK_CONTROL] = 0x80;
    }

    // Translate the virtual key according to the current keyboard
    // layout.
    scan_code = MapVirtualKey(vk, MAPVK_VK_TO_VSC);
    // Convert the scan-code into a sequence of zero or more unicode
    // codepoints.
    // If this is a dead key ToUnicode returns a negative value.
    len = ToUnicode(vk, scan_code, keyboard_state, ch, ARRAY_LENGTH(ch),
	    0);
    if (len < 0)
	dead_key = DEAD_KEY_SET_DEFAULT;

    if (len <= 0)
    {
	int wm_char = NUL;

	if (dead_key == DEAD_KEY_SET_DEFAULT
		&& (GetKeyState(VK_CONTROL) & 0x8000))
	{
	    if (   // AZERTY CTRL+dead_circumflex
		   (vk == 221 && scan_code == 26)
		   // QWERTZ CTRL+dead_circumflex
		|| (vk == 220 && scan_code == 41))
		wm_char = '[';
	    if (   // QWERTZ CTRL+dead_two-overdots
		   (vk == 192 && scan_code == 27))
		wm_char = ']';
	}
	if (wm_char != NUL)
	{
	    // post WM_CHAR='[' - which will be interpreted with CTRL
	    // still hold as ESC
	    PostMessageW(pmsg->hwnd, WM_CHAR, wm_char, pmsg->lParam);
	    // ask _OnChar() to not touch this state, wait for next key
	    // press and maintain knowledge that we are "poisoned" with
	    // "dead state"
	    dead_key = DEAD_KEY_TRANSIENT_IN_ON_CHAR;
	}
	return;
    }

    // Post the message as TranslateMessage would do.
    if (pmsg->message == WM_KEYDOWN)
    {
	for (i = 0; i < len; i++)
	    PostMessageW(pmsg->hwnd, WM_CHAR, ch[i], pmsg->lParam);
    }
    else
    {
	for (i = 0; i < len; i++)
	    PostMessageW(pmsg->hwnd, WM_SYSCHAR, ch[i], pmsg->lParam);
    }
}

/*
 * "Classic" implementation, existing prior to v8.2.4807
 */
static void process_message_usual_key_classic(UINT vk, const MSG *pmsg)
{
    char_u	string[40];

    // Some keys need C-S- where they should only need C-.
    // Ignore 0xff, Windows XP sends it when NUMLOCK has changed since
    // system startup (Helmut Stiegler, 2003 Oct 3).
    if (vk != 0xff
	    && (GetKeyState(VK_CONTROL) & 0x8000)
	    && !(GetKeyState(VK_SHIFT) & 0x8000)
	    && !(GetKeyState(VK_MENU) & 0x8000))
    {
	// CTRL-6 is '^'; Japanese keyboard maps '^' to vk == 0xDE
	if (vk == '6' || MapVirtualKey(vk, 2) == (UINT)'^')
	{
	    string[0] = Ctrl_HAT;
	    add_to_input_buf(string, 1);
	}
	// vk == 0xBD AZERTY for CTRL-'-', but CTRL-[ for * QWERTY!
	else if (vk == 0xBD)	// QWERTY for CTRL-'-'
	{
	    string[0] = Ctrl__;
	    add_to_input_buf(string, 1);
	}
	// CTRL-2 is '@'; Japanese keyboard maps '@' to vk == 0xC0
	else if (vk == '2' || MapVirtualKey(vk, 2) == (UINT)'@')
	{
	    string[0] = Ctrl_AT;
	    add_to_input_buf(string, 1);
	}
	else
	    TranslateMessage(pmsg);
    }
    else
	TranslateMessage(pmsg);
}

/*
 * Process a single Windows message.
 * If one is not available we hang until one is.
 */
    static void
process_message(void)
{
    MSG		msg;
    UINT	vk = 0;		// Virtual key
    char_u	string[40];
    int		i;
    int		modifiers = 0;
    int		key;
#ifdef FEAT_MENU
    static char_u k10[] = {K_SPECIAL, 'k', ';', 0};
#endif
    static int  keycode_trans_strategy_initialized = 0;

    // lazy initialize - first time only
    if (!keycode_trans_strategy_initialized)
    {
	keycode_trans_strategy_initialized = 1;
	keycode_trans_strategy_init();
    }

    GetMessageW(&msg, NULL, 0, 0);

#ifdef FEAT_OLE
    // Look after OLE Automation commands
    if (msg.message == WM_OLE)
    {
	char_u *str = (char_u *)msg.lParam;
	if (str == NULL || *str == NUL)
	{
	    // Message can't be ours, forward it.  Fixes problem with Ultramon
	    // 3.0.4
	    DispatchMessageW(&msg);
	}
	else
	{
	    add_to_input_buf(str, (int)STRLEN(str));
	    vim_free(str);  // was allocated in CVim::SendKeys()
	}
	return;
    }
#endif

#ifdef MSWIN_FIND_REPLACE
    // Don't process messages used by the dialog
    if (s_findrep_hwnd != NULL && IsDialogMessageW(s_findrep_hwnd, &msg))
    {
	HandleMouseHide(msg.message, msg.lParam);
	return;
    }
#endif

    /*
     * Check if it's a special key that we recognise.  If not, call
     * TranslateMessage().
     */
    if (msg.message == WM_KEYDOWN || msg.message == WM_SYSKEYDOWN)
    {
	vk = (int) msg.wParam;

	/*
	 * Handle dead keys in special conditions in other cases we let Windows
	 * handle them and do not interfere.
	 *
	 * The dead_key flag must be reset on several occasions:
	 * - in _OnChar() (or _OnSysChar()) as any dead key was necessarily
	 *   consumed at that point (This is when we let Windows combine the
	 *   dead character on its own)
	 *
	 * - Before doing something special such as regenerating keypresses to
	 *   expel the dead character as this could trigger an infinite loop if
	 *   for some reason TranslateMessage() do not trigger a call
	 *   immediately to _OnChar() (or _OnSysChar()).
	 */

	/*
	 * We are at the moment after WM_CHAR with DEAD_KEY_SKIP_ON_CHAR event
	 * was handled by _WndProc, this keypress we want to process normally
	 */
	if (keycode_trans_strategy_used->is_experimental()
		&& dead_key == DEAD_KEY_SKIP_ON_CHAR)
	{
	    dead_key = DEAD_KEY_OFF;
	}

	if (dead_key != DEAD_KEY_OFF)
	{
	    /*
	     * Expel the dead key pressed with Ctrl in a special way.
	     *
	     * After dead key was pressed with Ctrl in some cases, ESC was
	     * artificially injected and handled by _OnChar(), now we are
	     * dealing with completely new key press from the user. If we don't
	     * do anything, ToUnicode() call will interpret this vk+scan_code
	     * under influence of "dead-modifier". To prevent this we translate
	     * this message replacing current char from user with VK_SPACE,
	     * which will cause WM_CHAR with dead_key's character itself. Using
	     * DEAD_KEY_SKIP_ON_CHAR value of dead_char we force _OnChar() to
	     * ignore this one WM_CHAR event completely. Afterwards (due to
	     * usage of PostMessage), this procedure is scheduled to be called
	     * again with user char and on next entry we will clean
	     * DEAD_KEY_SKIP_ON_CHAR. We cannot use original
	     * outputDeadKey_rePost() since we do not wish to reset dead_key
	     * value.
	     */
	    if (keycode_trans_strategy_used->is_experimental() &&
		    dead_key == DEAD_KEY_TRANSIENT_IN_ON_CHAR)
	    {
		outputDeadKey_rePost_Ex(msg,
				       /*dead_key2set=*/DEAD_KEY_SKIP_ON_CHAR);
		return;
	    }

	    if (dead_key != DEAD_KEY_SET_DEFAULT)
	    {
		// should never happen - is there a way to make ASSERT here?
		return;
	    }

	    /*
	     * If a dead key was pressed and the user presses VK_SPACE,
	     * VK_BACK, or VK_ESCAPE it means that he actually wants to deal
	     * with the dead char now, so do nothing special and let Windows
	     * handle it.
	     *
	     * Note that VK_SPACE combines with the dead_key's character and
	     * only one WM_CHAR will be generated by TranslateMessage(), in
	     * the two other cases two WM_CHAR will be generated: the dead
	     * char and VK_BACK or VK_ESCAPE. That is most likely what the
	     * user expects.
	     */
	    if ((vk == VK_SPACE || vk == VK_BACK || vk == VK_ESCAPE))
	    {
		dead_key = DEAD_KEY_OFF;
		TranslateMessage(&msg);
		return;
	    }
	    // In modes where we are not typing, dead keys should behave
	    // normally
	    else if ((get_real_state()
			    & (MODE_INSERT | MODE_CMDLINE | MODE_SELECT)) == 0)
	    {
		outputDeadKey_rePost(msg);
		return;
	    }
	}

	// Check for CTRL-BREAK
	if (vk == VK_CANCEL)
	{
	    trash_input_buf();
	    got_int = TRUE;
	    ctrl_break_was_pressed = TRUE;
	    string[0] = Ctrl_C;
	    add_to_input_buf(string, 1);
	}

	// This is an IME event or a synthetic keystroke, let Windows handle it.
	if (vk == VK_PROCESSKEY || vk == VK_PACKET)
	{
	    TranslateMessage(&msg);
	    return;
	}

	for (i = 0; special_keys[i].key_sym != 0; i++)
	{
	    // ignore VK_SPACE when ALT key pressed: system menu
	    if (special_keys[i].key_sym == vk
		    && (vk != VK_SPACE || !(GetKeyState(VK_MENU) & 0x8000)))
	    {
		/*
		 * Behave as expected if we have a dead key and the special key
		 * is a key that would normally trigger the dead key nominal
		 * character output (such as a NUMPAD printable character or
		 * the TAB key, etc...).
		 */
		if (dead_key == DEAD_KEY_SET_DEFAULT
			&& (special_keys[i].vim_code0 == 'K'
						|| vk == VK_TAB || vk == CAR))
		{
		    outputDeadKey_rePost(msg);
		    return;
		}

#ifdef FEAT_MENU
		// Check for <F10>: Windows selects the menu.  When <F10> is
		// mapped we want to use the mapping instead.
		if (vk == VK_F10
			&& gui.menu_is_active
			&& check_map(k10, State, FALSE, TRUE, FALSE,
							  NULL, NULL) == NULL)
		    break;
#endif
		modifiers = get_active_modifiers_via_ptr();

		if (special_keys[i].vim_code1 == NUL)
		    key = special_keys[i].vim_code0;
		else
		    key = TO_SPECIAL(special_keys[i].vim_code0,
						   special_keys[i].vim_code1);
		key = simplify_key(key, &modifiers);
		if (key == CSI)
		    key = K_CSI;

		if (modifiers)
		{
		    string[0] = CSI;
		    string[1] = KS_MODIFIER;
		    string[2] = modifiers;
		    add_to_input_buf(string, 3);
		}

		if (IS_SPECIAL(key))
		{
		    string[0] = CSI;
		    string[1] = K_SECOND(key);
		    string[2] = K_THIRD(key);
		    add_to_input_buf(string, 3);
		}
		else
		{
		    int	len;

		    // Handle "key" as a Unicode character.
		    len = char_to_string(key, string, 40, FALSE);
		    add_to_input_buf(string, len);
		}
		break;
	    }
	}

	// Not a special key.
	if (special_keys[i].key_sym == 0)
	{
	    process_message_usual_key(vk, &msg);
	}
    }
#ifdef FEAT_MBYTE_IME
    else if (msg.message == WM_IME_NOTIFY)
	_OnImeNotify(msg.hwnd, (DWORD)msg.wParam, (DWORD)msg.lParam);
    else if (msg.message == WM_KEYUP && im_get_status())
	// added for non-MS IME (Yasuhiro Matsumoto)
	TranslateMessage(&msg);
#endif

#ifdef FEAT_MENU
    // Check for <F10>: Default effect is to select the menu.  When <F10> is
    // mapped we need to stop it here to avoid strange effects (e.g., for the
    // key-up event)
    if (vk != VK_F10 || check_map(k10, State, FALSE, TRUE, FALSE,
							  NULL, NULL) == NULL)
#endif
	DispatchMessageW(&msg);
}

/*
 * Catch up with any queued events.  This may put keyboard input into the
 * input buffer, call resize call-backs, trigger timers etc.  If there is
 * nothing in the event queue (& no timers pending), then we return
 * immediately.
 */
    void
gui_mch_update(void)
{
    MSG	    msg;

    if (!s_busy_processing)
	while (PeekMessageW(&msg, NULL, 0, 0, PM_NOREMOVE)
						  && !vim_is_input_buf_full())
	    process_message();
}

    static void
remove_any_timer(void)
{
    MSG		msg;

    if (s_wait_timer != 0 && !s_timed_out)
    {
	KillTimer(NULL, s_wait_timer);

	// Eat spurious WM_TIMER messages
	while (PeekMessageW(&msg, s_hwnd, WM_TIMER, WM_TIMER, PM_REMOVE))
	    ;
	s_wait_timer = 0;
    }
}

/*
 * GUI input routine called by gui_wait_for_chars().  Waits for a character
 * from the keyboard.
 *  wtime == -1	    Wait forever.
 *  wtime == 0	    This should never happen.
 *  wtime > 0	    Wait wtime milliseconds for a character.
 * Returns OK if a character was found to be available within the given time,
 * or FAIL otherwise.
 */
    int
gui_mch_wait_for_chars(int wtime)
{
    int		focus;

    s_timed_out = FALSE;

    if (wtime >= 0)
    {
	// Don't do anything while processing a (scroll) message.
	if (s_busy_processing)
	    return FAIL;

	// When called with "wtime" zero, just want one msec.
	s_wait_timer = SetTimer(NULL, 0, (UINT)(wtime == 0 ? 1 : wtime),
								_OnTimer);
    }

    allow_scrollbar = TRUE;

    focus = gui.in_focus;
    while (!s_timed_out)
    {
	// Stop or start blinking when focus changes
	if (gui.in_focus != focus)
	{
	    if (gui.in_focus)
		gui_mch_start_blink();
	    else
		gui_mch_stop_blink(TRUE);
	    focus = gui.in_focus;
	}

	if (s_need_activate)
	{
	    (void)SetForegroundWindow(s_hwnd);
	    s_need_activate = FALSE;
	}

#ifdef FEAT_TIMERS
	did_add_timer = FALSE;
#endif
#ifdef MESSAGE_QUEUE
	// Check channel I/O while waiting for a message.
	for (;;)
	{
	    MSG msg;

	    parse_queued_messages();
# ifdef FEAT_TIMERS
	    if (did_add_timer)
		break;
# endif
	    if (PeekMessageW(&msg, NULL, 0, 0, PM_NOREMOVE))
	    {
		process_message();
		break;
	    }
	    else if (input_available()
		    // TODO: The 10 msec is a compromise between laggy response
		    // and consuming more CPU time.  Better would be to handle
		    // channel messages when they arrive.
		    || MsgWaitForMultipleObjects(0, NULL, FALSE, 10,
						  QS_ALLINPUT) != WAIT_TIMEOUT)
		break;
	}
#else
	// Don't use gui_mch_update() because then we will spin-lock until a
	// char arrives, instead we use GetMessage() to hang until an
	// event arrives.  No need to check for input_buf_full because we are
	// returning as soon as it contains a single char -- webb
	process_message();
#endif

	if (input_available())
	{
	    remove_any_timer();
	    allow_scrollbar = FALSE;

	    // Clear pending mouse button, the release event may have been
	    // taken by the dialog window.  But don't do this when getting
	    // focus, we need the mouse-up event then.
	    if (!s_getting_focus)
		s_button_pending = -1;

	    return OK;
	}

#ifdef FEAT_TIMERS
	if (did_add_timer)
	{
	    // Need to recompute the waiting time.
	    remove_any_timer();
	    break;
	}
#endif
    }
    allow_scrollbar = FALSE;
    return FAIL;
}

/*
 * Clear a rectangular region of the screen from text pos (row1, col1) to
 * (row2, col2) inclusive.
 */
    void
gui_mch_clear_block(
    int		row1,
    int		col1,
    int		row2,
    int		col2)
{
    RECT	rc;

    /*
     * Clear one extra pixel at the far right, for when bold characters have
     * spilled over to the window border.
     * Note: FillRect() excludes right and bottom of rectangle.
     */
    rc.left = FILL_X(col1);
    rc.top = FILL_Y(row1);
    rc.right = FILL_X(col2 + 1) + (col2 == Columns - 1);
    rc.bottom = FILL_Y(row2 + 1);
    clear_rect(&rc);
}

/*
 * Clear the whole text window.
 */
    void
gui_mch_clear_all(void)
{
    RECT    rc;

    rc.left = 0;
    rc.top = 0;
    rc.right = Columns * gui.char_width + 2 * gui.border_width;
    rc.bottom = Rows * gui.char_height + 2 * gui.border_width;
    clear_rect(&rc);
}
/*
 * Menu stuff.
 */

    void
gui_mch_enable_menu(int flag)
{
#ifdef FEAT_MENU
    SetMenu(s_hwnd, flag ? s_menuBar : NULL);
#endif
}

    void
gui_mch_set_menu_pos(
    int	    x UNUSED,
    int	    y UNUSED,
    int	    w UNUSED,
    int	    h UNUSED)
{
    // It will be in the right place anyway
}

#if defined(FEAT_MENU) || defined(PROTO)
/*
 * Make menu item hidden or not hidden
 */
    void
gui_mch_menu_hidden(
    vimmenu_T	*menu,
    int		hidden)
{
    /*
     * This doesn't do what we want.  Hmm, just grey the menu items for now.
     */
    /*
    if (hidden)
	EnableMenuItem(s_menuBar, menu->id, MF_BYCOMMAND | MF_DISABLED);
    else
	EnableMenuItem(s_menuBar, menu->id, MF_BYCOMMAND | MF_ENABLED);
    */
    gui_mch_menu_grey(menu, hidden);
}

/*
 * This is called after setting all the menus to grey/hidden or not.
 */
    void
gui_mch_draw_menubar(void)
{
    DrawMenuBar(s_hwnd);
}
#endif // FEAT_MENU

/*
 * Return the RGB value of a pixel as a long.
 */
    guicolor_T
gui_mch_get_rgb(guicolor_T pixel)
{
    return (guicolor_T)((GetRValue(pixel) << 16) + (GetGValue(pixel) << 8)
							   + GetBValue(pixel));
}

#if defined(FEAT_GUI_DIALOG) || defined(PROTO)
/*
 * Convert pixels in X to dialog units
 */
    static WORD
PixelToDialogX(int numPixels)
{
    return (WORD)((numPixels * 4) / s_dlgfntwidth);
}

/*
 * Convert pixels in Y to dialog units
 */
    static WORD
PixelToDialogY(int numPixels)
{
    return (WORD)((numPixels * 8) / s_dlgfntheight);
}

/*
 * Return the width in pixels of the given text in the given DC.
 */
    static int
GetTextWidth(HDC hdc, char_u *str, int len)
{
    SIZE    size;

    GetTextExtentPoint(hdc, (LPCSTR)str, len, &size);
    return size.cx;
}

/*
 * Return the width in pixels of the given text in the given DC, taking care
 * of 'encoding' to active codepage conversion.
 */
    static int
GetTextWidthEnc(HDC hdc, char_u *str, int len)
{
    SIZE	size;
    WCHAR	*wstr;
    int		n;
    int		wlen = len;

    wstr = enc_to_utf16(str, &wlen);
    if (wstr == NULL)
	return 0;

    n = GetTextExtentPointW(hdc, wstr, wlen, &size);
    vim_free(wstr);
    if (n)
	return size.cx;
    return 0;
}

static void get_work_area(RECT *spi_rect);

/*
 * A quick little routine that will center one window over another, handy for
 * dialog boxes.  Taken from the Win32SDK samples and modified for multiple
 * monitors.
 */
    static BOOL
CenterWindow(
    HWND hwndChild,
    HWND hwndParent)
{
    HMONITOR	    mon;
    MONITORINFO	    moninfo;
    RECT	    rChild, rParent, rScreen;
    int		    wChild, hChild, wParent, hParent;
    int		    xNew, yNew;
    HDC		    hdc;

    GetWindowRect(hwndChild, &rChild);
    wChild = rChild.right - rChild.left;
    hChild = rChild.bottom - rChild.top;

    // If Vim is minimized put the window in the middle of the screen.
    if (hwndParent == NULL || IsMinimized(hwndParent))
	get_work_area(&rParent);
    else
	GetWindowRect(hwndParent, &rParent);
    wParent = rParent.right - rParent.left;
    hParent = rParent.bottom - rParent.top;

    moninfo.cbSize = sizeof(MONITORINFO);
    mon = MonitorFromWindow(hwndChild, MONITOR_DEFAULTTOPRIMARY);
    if (mon != NULL && GetMonitorInfo(mon, &moninfo))
    {
	rScreen = moninfo.rcWork;
    }
    else
    {
	hdc = GetDC(hwndChild);
	rScreen.left = 0;
	rScreen.top = 0;
	rScreen.right = GetDeviceCaps(hdc, HORZRES);
	rScreen.bottom = GetDeviceCaps(hdc, VERTRES);
	ReleaseDC(hwndChild, hdc);
    }

    xNew = rParent.left + ((wParent - wChild) / 2);
    if (xNew < rScreen.left)
	xNew = rScreen.left;
    else if ((xNew + wChild) > rScreen.right)
	xNew = rScreen.right - wChild;

    yNew = rParent.top + ((hParent - hChild) / 2);
    if (yNew < rScreen.top)
	yNew = rScreen.top;
    else if ((yNew + hChild) > rScreen.bottom)
	yNew = rScreen.bottom - hChild;

    return SetWindowPos(hwndChild, NULL, xNew, yNew, 0, 0,
						   SWP_NOSIZE | SWP_NOZORDER);
}
#endif // FEAT_GUI_DIALOG

#if defined(FEAT_TOOLBAR) || defined(PROTO)
    void
gui_mch_show_toolbar(int showit)
{
    if (s_toolbarhwnd == NULL)
	return;

    if (showit)
    {
	// Enable unicode support
	SendMessage(s_toolbarhwnd, TB_SETUNICODEFORMAT, (WPARAM)TRUE,
								(LPARAM)0);
	ShowWindow(s_toolbarhwnd, SW_SHOW);
    }
    else
	ShowWindow(s_toolbarhwnd, SW_HIDE);
}

// The number of bitmaps is fixed.  Exit is missing!
# define TOOLBAR_BITMAP_COUNT 31

#endif

#if defined(FEAT_GUI_TABLINE) || defined(PROTO)
    static void
add_tabline_popup_menu_entry(HMENU pmenu, UINT item_id, char_u *item_text)
{
    WCHAR	    *wn;
    MENUITEMINFOW   infow;

    wn = enc_to_utf16(item_text, NULL);
    if (wn == NULL)
	return;

    infow.cbSize = sizeof(infow);
    infow.fMask = MIIM_TYPE | MIIM_ID;
    infow.wID = item_id;
    infow.fType = MFT_STRING;
    infow.dwTypeData = wn;
    infow.cch = (UINT)wcslen(wn);
    InsertMenuItemW(pmenu, item_id, FALSE, &infow);
    vim_free(wn);
}

    static void
show_tabline_popup_menu(void)
{
    HMENU	    tab_pmenu;
    long	    rval;
    POINT	    pt;

    // When ignoring events don't show the menu.
    if (hold_gui_events || cmdwin_type != 0)
	return;

    tab_pmenu = CreatePopupMenu();
    if (tab_pmenu == NULL)
	return;

    if (first_tabpage->tp_next != NULL)
	add_tabline_popup_menu_entry(tab_pmenu,
				TABLINE_MENU_CLOSE, (char_u *)_("Close tab"));
    add_tabline_popup_menu_entry(tab_pmenu,
				TABLINE_MENU_NEW, (char_u *)_("New tab"));
    add_tabline_popup_menu_entry(tab_pmenu,
				TABLINE_MENU_OPEN, (char_u *)_("Open tab..."));

    GetCursorPos(&pt);
    rval = TrackPopupMenuEx(tab_pmenu, TPM_RETURNCMD, pt.x, pt.y, s_tabhwnd,
									NULL);

    DestroyMenu(tab_pmenu);

    // Add the string cmd into input buffer
    if (rval > 0)
    {
	TCHITTESTINFO htinfo;
	int idx;

	if (ScreenToClient(s_tabhwnd, &pt) == 0)
	    return;

	htinfo.pt.x = pt.x;
	htinfo.pt.y = pt.y;
	idx = TabCtrl_HitTest(s_tabhwnd, &htinfo);
	if (idx == -1)
	    idx = 0;
	else
	    idx += 1;

	send_tabline_menu_event(idx, (int)rval);
    }
}

/*
 * Show or hide the tabline.
 */
    void
gui_mch_show_tabline(int showit)
{
    if (s_tabhwnd == NULL)
	return;

    if (!showit != !showing_tabline)
    {
	if (showit)
	    ShowWindow(s_tabhwnd, SW_SHOW);
	else
	    ShowWindow(s_tabhwnd, SW_HIDE);
	showing_tabline = showit;
    }
}

/*
 * Return TRUE when tabline is displayed.
 */
    int
gui_mch_showing_tabline(void)
{
    return s_tabhwnd != NULL && showing_tabline;
}

/*
 * Update the labels of the tabline.
 */
    void
gui_mch_update_tabline(void)
{
    tabpage_T	*tp;
    TCITEM	tie;
    int		nr = 0;
    int		curtabidx = 0;
    int		tabadded = 0;
    WCHAR	*wstr = NULL;

    if (s_tabhwnd == NULL)
	return;

    // Enable unicode support
    SendMessage(s_tabhwnd, CCM_SETUNICODEFORMAT, (WPARAM)TRUE, (LPARAM)0);

    tie.mask = TCIF_TEXT;
    tie.iImage = -1;

    // Disable redraw for tab updates to eliminate O(N^2) draws.
    SendMessage(s_tabhwnd, WM_SETREDRAW, (WPARAM)FALSE, 0);

    // Add a label for each tab page.  They all contain the same text area.
    for (tp = first_tabpage; tp != NULL; tp = tp->tp_next, ++nr)
    {
	if (tp == curtab)
	    curtabidx = nr;

	if (nr >= TabCtrl_GetItemCount(s_tabhwnd))
	{
	    // Add the tab
	    tie.pszText = "-Empty-";
	    TabCtrl_InsertItem(s_tabhwnd, nr, &tie);
	    tabadded = 1;
	}

	get_tabline_label(tp, FALSE);
	tie.pszText = (LPSTR)NameBuff;

	wstr = enc_to_utf16(NameBuff, NULL);
	if (wstr != NULL)
	{
	    TCITEMW		tiw;

	    tiw.mask = TCIF_TEXT;
	    tiw.iImage = -1;
	    tiw.pszText = wstr;
	    SendMessage(s_tabhwnd, TCM_SETITEMW, (WPARAM)nr, (LPARAM)&tiw);
	    vim_free(wstr);
	}
    }

    // Remove any old labels.
    while (nr < TabCtrl_GetItemCount(s_tabhwnd))
	TabCtrl_DeleteItem(s_tabhwnd, nr);

    if (!tabadded && TabCtrl_GetCurSel(s_tabhwnd) != curtabidx)
	TabCtrl_SetCurSel(s_tabhwnd, curtabidx);

    // Re-enable redraw and redraw.
    SendMessage(s_tabhwnd, WM_SETREDRAW, (WPARAM)TRUE, 0);
    RedrawWindow(s_tabhwnd, NULL, NULL,
		    RDW_ERASE | RDW_FRAME | RDW_INVALIDATE | RDW_ALLCHILDREN);

    if (tabadded && TabCtrl_GetCurSel(s_tabhwnd) != curtabidx)
	TabCtrl_SetCurSel(s_tabhwnd, curtabidx);
}

/*
 * Set the current tab to "nr".  First tab is 1.
 */
    void
gui_mch_set_curtab(int nr)
{
    if (s_tabhwnd == NULL)
	return;

    if (TabCtrl_GetCurSel(s_tabhwnd) != nr - 1)
	TabCtrl_SetCurSel(s_tabhwnd, nr - 1);
}

#endif

/*
 * ":simalt" command.
 */
    void
ex_simalt(exarg_T *eap)
{
    char_u	*keys = eap->arg;
    int		fill_typebuf = FALSE;
    char_u	key_name[4];

    PostMessage(s_hwnd, WM_SYSCOMMAND, (WPARAM)SC_KEYMENU, (LPARAM)0);
    while (*keys)
    {
	if (*keys == '~')
	    *keys = ' ';	    // for showing system menu
	PostMessage(s_hwnd, WM_CHAR, (WPARAM)*keys, (LPARAM)0);
	keys++;
	fill_typebuf = TRUE;
    }
    if (fill_typebuf)
    {
	// Put a NOP in the typeahead buffer so that the message will get
	// processed.
	key_name[0] = K_SPECIAL;
	key_name[1] = KS_EXTRA;
	key_name[2] = KE_NOP;
	key_name[3] = NUL;
#if defined(FEAT_CLIENTSERVER) || defined(FEAT_EVAL)
	typebuf_was_filled = TRUE;
#endif
	(void)ins_typebuf(key_name, REMAP_NONE, 0, TRUE, FALSE);
    }
}

/*
 * Create the find & replace dialogs.
 * You can't have both at once: ":find" when replace is showing, destroys
 * the replace dialog first, and the other way around.
 */
#ifdef MSWIN_FIND_REPLACE
    static void
initialise_findrep(char_u *initial_string)
{
    int		wword = FALSE;
    int		mcase = !p_ic;
    char_u	*entry_text;

    // Get the search string to use.
    entry_text = get_find_dialog_text(initial_string, &wword, &mcase);

    s_findrep_struct.hwndOwner = s_hwnd;
    s_findrep_struct.Flags = FR_DOWN;
    if (mcase)
	s_findrep_struct.Flags |= FR_MATCHCASE;
    if (wword)
	s_findrep_struct.Flags |= FR_WHOLEWORD;
    if (entry_text != NULL && *entry_text != NUL)
    {
	WCHAR *p = enc_to_utf16(entry_text, NULL);
	if (p != NULL)
	{
	    int len = s_findrep_struct.wFindWhatLen - 1;

	    wcsncpy(s_findrep_struct.lpstrFindWhat, p, len);
	    s_findrep_struct.lpstrFindWhat[len] = NUL;
	    vim_free(p);
	}
    }
    vim_free(entry_text);
}
#endif

    static void
set_window_title(HWND hwnd, char *title)
{
    if (title != NULL)
    {
	WCHAR	*wbuf;

	// Convert the title from 'encoding' to UTF-16.
	wbuf = (WCHAR *)enc_to_utf16((char_u *)title, NULL);
	if (wbuf != NULL)
	{
	    SetWindowTextW(hwnd, wbuf);
	    vim_free(wbuf);
	}
    }
    else
	(void)SetWindowTextW(hwnd, NULL);
}

    void
gui_mch_find_dialog(exarg_T *eap)
{
#ifdef MSWIN_FIND_REPLACE
    if (s_findrep_msg != 0)
    {
	if (IsWindow(s_findrep_hwnd) && !s_findrep_is_find)
	    DestroyWindow(s_findrep_hwnd);

	if (!IsWindow(s_findrep_hwnd))
	{
	    initialise_findrep(eap->arg);
	    s_findrep_hwnd = FindTextW(&s_findrep_struct);
	}

	set_window_title(s_findrep_hwnd, _("Find string"));
	(void)SetFocus(s_findrep_hwnd);

	s_findrep_is_find = TRUE;
    }
#endif
}


    void
gui_mch_replace_dialog(exarg_T *eap)
{
#ifdef MSWIN_FIND_REPLACE
    if (s_findrep_msg != 0)
    {
	if (IsWindow(s_findrep_hwnd) && s_findrep_is_find)
	    DestroyWindow(s_findrep_hwnd);

	if (!IsWindow(s_findrep_hwnd))
	{
	    initialise_findrep(eap->arg);
	    s_findrep_hwnd = ReplaceTextW(&s_findrep_struct);
	}

	set_window_title(s_findrep_hwnd, _("Find & Replace"));
	(void)SetFocus(s_findrep_hwnd);

	s_findrep_is_find = FALSE;
    }
#endif
}


/*
 * Set visibility of the pointer.
 */
    void
gui_mch_mousehide(int hide)
{
    if (hide == gui.pointer_hidden)
	return;

    ShowCursor(!hide);
    gui.pointer_hidden = hide;
}

#ifdef FEAT_MENU
    static void
gui_mch_show_popupmenu_at(vimmenu_T *menu, int x, int y)
{
    // Unhide the mouse, we don't get move events here.
    gui_mch_mousehide(FALSE);

    (void)TrackPopupMenu(
	(HMENU)menu->submenu_id,
	TPM_LEFTALIGN | TPM_LEFTBUTTON,
	x, y,
	(int)0,	    //reserved param
	s_hwnd,
	NULL);
    /*
     * NOTE: The pop-up menu can eat the mouse up event.
     * We deal with this in normal.c.
     */
}
#endif

/*
 * Got a message when the system will go down.
 */
    static void
_OnEndSession(void)
{
    getout_preserve_modified(1);
}

/*
 * Get this message when the user clicks on the cross in the top right corner
 * of a Windows95 window.
 */
    static void
_OnClose(HWND hwnd UNUSED)
{
    gui_shell_closed();
}

/*
 * Get a message when the window is being destroyed.
 */
    static void
_OnDestroy(HWND hwnd)
{
    if (!destroying)
	_OnClose(hwnd);
}

    static void
_OnPaint(
    HWND hwnd)
{
    if (IsMinimized(hwnd))
	return;

    PAINTSTRUCT ps;

    out_flush();	    // make sure all output has been processed
    (void)BeginPaint(hwnd, &ps);

    // prevent multi-byte characters from misprinting on an invalid
    // rectangle
    if (has_mbyte)
    {
	RECT rect;

	GetClientRect(hwnd, &rect);
	ps.rcPaint.left = rect.left;
	ps.rcPaint.right = rect.right;
    }

    if (!IsRectEmpty(&ps.rcPaint))
    {
	gui_redraw(ps.rcPaint.left, ps.rcPaint.top,
		ps.rcPaint.right - ps.rcPaint.left + 1,
		ps.rcPaint.bottom - ps.rcPaint.top + 1);
    }

    EndPaint(hwnd, &ps);
}

    static void
_OnSize(
    HWND hwnd,
    UINT state UNUSED,
    int cx,
    int cy)
{
    if (!IsMinimized(hwnd) && !s_in_dpichanged)
    {
	gui_resize_shell(cx, cy);

	// Menu bar may wrap differently now
	gui_mswin_get_menu_height(TRUE);
    }
}

    static void
_OnSetFocus(
    HWND hwnd,
    HWND hwndOldFocus)
{
    gui_focus_change(TRUE);
    s_getting_focus = TRUE;
    (void)DefWindowProcW(hwnd, WM_SETFOCUS, (WPARAM)hwndOldFocus, 0);
}

    static void
_OnKillFocus(
    HWND hwnd,
    HWND hwndNewFocus)
{
    if (destroying)
	return;
    gui_focus_change(FALSE);
    s_getting_focus = FALSE;
    (void)DefWindowProcW(hwnd, WM_KILLFOCUS, (WPARAM)hwndNewFocus, 0);
}

/*
 * Get a message when the user switches back to vim
 */
    static LRESULT
_OnActivateApp(
    HWND hwnd,
    BOOL fActivate,
    DWORD dwThreadId)
{
    // we call gui_focus_change() in _OnSetFocus()
    // gui_focus_change((int)fActivate);
    return DefWindowProcW(hwnd, WM_ACTIVATEAPP, fActivate, (DWORD)dwThreadId);
}

    void
gui_mch_destroy_scrollbar(scrollbar_T *sb)
{
    DestroyWindow(sb->id);
}

/*
 * Get current mouse coordinates in text window.
 */
    void
gui_mch_getmouse(int *x, int *y)
{
    RECT rct;
    POINT mp;

    (void)GetWindowRect(s_textArea, &rct);
    (void)GetCursorPos(&mp);
    *x = (int)(mp.x - rct.left);
    *y = (int)(mp.y - rct.top);
}

/*
 * Move mouse pointer to character at (x, y).
 */
    void
gui_mch_setmouse(int x, int y)
{
    RECT rct;

    (void)GetWindowRect(s_textArea, &rct);
    (void)SetCursorPos(x + gui.border_offset + rct.left,
		       y + gui.border_offset + rct.top);
}

    static void
gui_mswin_get_valid_dimensions(
    int w,
    int h,
    int *valid_w,
    int *valid_h,
    int *cols,
    int *rows)
{
    int	    base_width, base_height;

    base_width = gui_get_base_width()
	+ (pGetSystemMetricsForDpi(SM_CXFRAME, s_dpi) +
	   pGetSystemMetricsForDpi(SM_CXPADDEDBORDER, s_dpi)) * 2;
    base_height = gui_get_base_height()
	+ (pGetSystemMetricsForDpi(SM_CYFRAME, s_dpi) +
	   pGetSystemMetricsForDpi(SM_CXPADDEDBORDER, s_dpi)) * 2
	+ pGetSystemMetricsForDpi(SM_CYCAPTION, s_dpi)
	+ gui_mswin_get_menu_height(FALSE);
    *cols = (w - base_width) / gui.char_width;
    *rows = (h - base_height) / gui.char_height;
    *valid_w = base_width + *cols * gui.char_width;
    *valid_h = base_height + *rows * gui.char_height;
}

    void
gui_mch_flash(int msec)
{
    RECT    rc;

#if defined(FEAT_DIRECTX)
    if (IS_ENABLE_DIRECTX())
	DWriteContext_Flush(s_dwc);
#endif

    /*
     * Note: InvertRect() excludes right and bottom of rectangle.
     */
    rc.left = 0;
    rc.top = 0;
    rc.right = gui.num_cols * gui.char_width;
    rc.bottom = gui.num_rows * gui.char_height;
    InvertRect(s_hdc, &rc);
    gui_mch_flush();			// make sure it's displayed

    ui_delay((long)msec, TRUE);	// wait for a few msec

    InvertRect(s_hdc, &rc);
}

/*
 * Check if the specified point is on-screen. (multi-monitor aware)
 */
    static BOOL
is_point_onscreen(int x, int y)
{
    POINT   pt = {x, y};

    return MonitorFromPoint(pt, MONITOR_DEFAULTTONULL) != NULL;
}

/*
 * Check if the whole client area of the specified window is on-screen.
 *
 * Note about DirectX: Windows 10 1809 or above no longer maintains image of
 * the window portion that is off-screen.  Scrolling by DWriteContext_Scroll()
 * only works when the whole window is on-screen.
 */
    static BOOL
is_window_onscreen(HWND hwnd)
{
    RECT    rc;
    POINT   p1, p2;

    GetClientRect(hwnd, &rc);
    p1.x = rc.left;
    p1.y = rc.top;
    p2.x = rc.right - 1;
    p2.y = rc.bottom - 1;
    ClientToScreen(hwnd, &p1);
    ClientToScreen(hwnd, &p2);

    if (!is_point_onscreen(p1.x, p1.y))
	return FALSE;
    if (!is_point_onscreen(p1.x, p2.y))
	return FALSE;
    if (!is_point_onscreen(p2.x, p1.y))
	return FALSE;
    if (!is_point_onscreen(p2.x, p2.y))
	return FALSE;
    return TRUE;
}

/*
 * Return flags used for scrolling.
 * The SW_INVALIDATE is required when part of the window is covered or
 * off-screen. Refer to MS KB Q75236.
 */
    static int
get_scroll_flags(void)
{
    HWND	hwnd;
    RECT	rcVim, rcOther, rcDest;

    // Check if the window is (partly) off-screen.
    if (!is_window_onscreen(s_hwnd))
	return SW_INVALIDATE;

    // Check if there is a window (partly) on top of us.
    GetWindowRect(s_hwnd, &rcVim);
    for (hwnd = s_hwnd; (hwnd = GetWindow(hwnd, GW_HWNDPREV)) != (HWND)0; )
	if (IsWindowVisible(hwnd))
	{
	    GetWindowRect(hwnd, &rcOther);
	    if (IntersectRect(&rcDest, &rcVim, &rcOther))
		return SW_INVALIDATE;
	}
    return 0;
}

/*
 * On some Intel GPUs, the regions drawn just prior to ScrollWindowEx()
 * may not be scrolled out properly.
 * For gVim, when _OnScroll() is repeated, the character at the
 * previous cursor position may be left drawn after scroll.
 * The problem can be avoided by calling GetPixel() to get a pixel in
 * the region before ScrollWindowEx().
 */
    static void
intel_gpu_workaround(void)
{
    GetPixel(s_hdc, FILL_X(gui.col), FILL_Y(gui.row));
}

/*
 * Delete the given number of lines from the given row, scrolling up any
 * text further down within the scroll region.
 */
    void
gui_mch_delete_lines(
    int	    row,
    int	    num_lines)
{
    RECT	rc;

    rc.left = FILL_X(gui.scroll_region_left);
    rc.right = FILL_X(gui.scroll_region_right + 1);
    rc.top = FILL_Y(row);
    rc.bottom = FILL_Y(gui.scroll_region_bot + 1);

#if defined(FEAT_DIRECTX)
    if (IS_ENABLE_DIRECTX() && is_window_onscreen(s_hwnd))
    {
	DWriteContext_Scroll(s_dwc, 0, -num_lines * gui.char_height, &rc);
    }
    else
#endif
    {
#if defined(FEAT_DIRECTX)
	if (IS_ENABLE_DIRECTX())
	    DWriteContext_Flush(s_dwc);
#endif
	intel_gpu_workaround();
	ScrollWindowEx(s_textArea, 0, -num_lines * gui.char_height,
				    &rc, &rc, NULL, NULL, get_scroll_flags());
	UpdateWindow(s_textArea);
    }

    // This seems to be required to avoid the cursor disappearing when
    // scrolling such that the cursor ends up in the top-left character on
    // the screen...   But why?  (Webb)
    // It's probably fixed by disabling drawing the cursor while scrolling.
    // gui.cursor_is_valid = FALSE;

    gui_clear_block(gui.scroll_region_bot - num_lines + 1,
						       gui.scroll_region_left,
	gui.scroll_region_bot, gui.scroll_region_right);
}

/*
 * Insert the given number of lines before the given row, scrolling down any
 * following text within the scroll region.
 */
    void
gui_mch_insert_lines(
    int		row,
    int		num_lines)
{
    RECT	rc;

    rc.left = FILL_X(gui.scroll_region_left);
    rc.right = FILL_X(gui.scroll_region_right + 1);
    rc.top = FILL_Y(row);
    rc.bottom = FILL_Y(gui.scroll_region_bot + 1);

#if defined(FEAT_DIRECTX)
    if (IS_ENABLE_DIRECTX() && is_window_onscreen(s_hwnd))
    {
	DWriteContext_Scroll(s_dwc, 0, num_lines * gui.char_height, &rc);
    }
    else
#endif
    {
#if defined(FEAT_DIRECTX)
	if (IS_ENABLE_DIRECTX())
	    DWriteContext_Flush(s_dwc);
#endif
	intel_gpu_workaround();
	// The SW_INVALIDATE is required when part of the window is covered or
	// off-screen.  How do we avoid it when it's not needed?
	ScrollWindowEx(s_textArea, 0, num_lines * gui.char_height,
				    &rc, &rc, NULL, NULL, get_scroll_flags());
	UpdateWindow(s_textArea);
    }

    gui_clear_block(row, gui.scroll_region_left,
				row + num_lines - 1, gui.scroll_region_right);
}


    void
gui_mch_exit(int rc UNUSED)
{
#if defined(FEAT_DIRECTX)
    DWriteContext_Close(s_dwc);
    DWrite_Final();
    s_dwc = NULL;
#endif

    ReleaseDC(s_textArea, s_hdc);
    DeleteObject(s_brush);

#ifdef FEAT_TEAROFF
    // Unload the tearoff bitmap
    (void)DeleteObject((HGDIOBJ)s_htearbitmap);
#endif

    // Destroy our window (if we have one).
    if (s_hwnd != NULL)
    {
	destroying = TRUE;	// ignore WM_DESTROY message now
	DestroyWindow(s_hwnd);
    }
}

    static char_u *
logfont2name(LOGFONTW lf)
{
    char	*p;
    char	*res;
    char	*charset_name;
    char	*quality_name;
    char	*font_name;
    int		points;

    font_name = (char *)utf16_to_enc(lf.lfFaceName, NULL);
    if (font_name == NULL)
	return NULL;
    charset_name = charset_id2name((int)lf.lfCharSet);
    quality_name = quality_id2name((int)lf.lfQuality);

    res = alloc(strlen(font_name) + 30
		    + (charset_name == NULL ? 0 : strlen(charset_name) + 2)
		    + (quality_name == NULL ? 0 : strlen(quality_name) + 2));
    if (res != NULL)
    {
	p = res;
	// make a normal font string out of the lf thing:
	points = pixels_to_points(
			 lf.lfHeight < 0 ? -lf.lfHeight : lf.lfHeight, TRUE);
	if (lf.lfWeight == FW_NORMAL || lf.lfWeight == FW_BOLD)
	    sprintf((char *)p, "%s:h%d", font_name, points);
	else
	    sprintf((char *)p, "%s:h%d:W%ld", font_name, points, lf.lfWeight);
	while (*p)
	{
	    if (*p == ' ')
		*p = '_';
	    ++p;
	}
	if (lf.lfItalic)
	    STRCAT(p, ":i");
	if (lf.lfWeight == FW_BOLD)
	    STRCAT(p, ":b");
	if (lf.lfUnderline)
	    STRCAT(p, ":u");
	if (lf.lfStrikeOut)
	    STRCAT(p, ":s");
	if (charset_name != NULL)
	{
	    STRCAT(p, ":c");
	    STRCAT(p, charset_name);
	}
	if (quality_name != NULL)
	{
	    STRCAT(p, ":q");
	    STRCAT(p, quality_name);
	}
    }

    vim_free(font_name);
    return (char_u *)res;
}


#ifdef FEAT_MBYTE_IME
/*
 * Set correct LOGFONTW to IME.  Use 'guifontwide' if available, otherwise use
 * 'guifont'.
 */
    static void
update_im_font(void)
{
    LOGFONTW	lf_wide, lf;

    if (p_guifontwide != NULL && *p_guifontwide != NUL
	    && gui.wide_font != NOFONT
	    && GetObjectW((HFONT)gui.wide_font, sizeof(lf_wide), &lf_wide))
	norm_logfont = lf_wide;
    else
	norm_logfont = sub_logfont;

    lf = norm_logfont;
    if (s_process_dpi_aware == DPI_AWARENESS_UNAWARE)
	// Work around when PerMonitorV2 is not enabled in the process level.
	lf.lfHeight = lf.lfHeight * DEFAULT_DPI / s_dpi;
    im_set_font(&lf);
}
#endif

/*
 * Handler of gui.wide_font (p_guifontwide) changed notification.
 */
    void
gui_mch_wide_font_changed(void)
{
    LOGFONTW lf;

#ifdef FEAT_MBYTE_IME
    update_im_font();
#endif

    gui_mch_free_font(gui.wide_ital_font);
    gui.wide_ital_font = NOFONT;
    gui_mch_free_font(gui.wide_bold_font);
    gui.wide_bold_font = NOFONT;
    gui_mch_free_font(gui.wide_boldital_font);
    gui.wide_boldital_font = NOFONT;

    if (gui.wide_font
	&& GetObjectW((HFONT)gui.wide_font, sizeof(lf), &lf))
    {
	if (!lf.lfItalic)
	{
	    lf.lfItalic = TRUE;
	    gui.wide_ital_font = get_font_handle(&lf);
	    lf.lfItalic = FALSE;
	}
	if (lf.lfWeight < FW_BOLD)
	{
	    lf.lfWeight = FW_BOLD;
	    gui.wide_bold_font = get_font_handle(&lf);
	    if (!lf.lfItalic)
	    {
		lf.lfItalic = TRUE;
		gui.wide_boldital_font = get_font_handle(&lf);
	    }
	}
    }
}

/*
 * Initialise vim to use the font with the given name.
 * Return FAIL if the font could not be loaded, OK otherwise.
 */
    int
gui_mch_init_font(char_u *font_name, int fontset UNUSED)
{
    LOGFONTW	lf, lfOrig;
    GuiFont	font = NOFONT;
    char_u	*p;

    // Load the font
    if (get_logfont(&lf, font_name, NULL, TRUE) == OK)
    {
	lfOrig = lf;
	lf.lfHeight = adjust_fontsize_by_dpi(lf.lfHeight);
	font = get_font_handle(&lf);
    }
    if (font == NOFONT)
	return FAIL;

    if (font_name == NULL)
	font_name = (char_u *)"";
#ifdef FEAT_MBYTE_IME
    norm_logfont = lf;
    sub_logfont = lf;
    if (!s_in_dpichanged)
	update_im_font();
#endif
    gui_mch_free_font(gui.norm_font);
    gui.norm_font = font;
    current_font_height = lfOrig.lfHeight;
    UpdateFontSize(font);

    p = logfont2name(lfOrig);
    if (p != NULL)
    {
	hl_set_font_name(p);

	// When setting 'guifont' to "*" replace it with the actual font name.
	if (STRCMP(font_name, "*") == 0 && STRCMP(p_guifont, "*") == 0)
	{
	    vim_free(p_guifont);
	    p_guifont = p;
	}
	else
	    vim_free(p);
    }

    gui_mch_free_font(gui.ital_font);
    gui.ital_font = NOFONT;
    gui_mch_free_font(gui.bold_font);
    gui.bold_font = NOFONT;
    gui_mch_free_font(gui.boldital_font);
    gui.boldital_font = NOFONT;

    if (!lf.lfItalic)
    {
	lf.lfItalic = TRUE;
	gui.ital_font = get_font_handle(&lf);
	lf.lfItalic = FALSE;
    }
    if (lf.lfWeight < FW_BOLD)
    {
	lf.lfWeight = FW_BOLD;
	gui.bold_font = get_font_handle(&lf);
	if (!lf.lfItalic)
	{
	    lf.lfItalic = TRUE;
	    gui.boldital_font = get_font_handle(&lf);
	}
    }

    return OK;
}

/*
 * Return TRUE if the GUI window is maximized, filling the whole screen.
 * Also return TRUE if the window is snapped.
 */
    int
gui_mch_maximized(void)
{
    WINDOWPLACEMENT wp;
    RECT	    rc;

    wp.length = sizeof(WINDOWPLACEMENT);
    if (GetWindowPlacement(s_hwnd, &wp))
    {
	if (wp.showCmd == SW_SHOWMAXIMIZED
	    || (wp.showCmd == SW_SHOWMINIMIZED
		    && wp.flags == WPF_RESTORETOMAXIMIZED))
	    return TRUE;
	if (wp.showCmd == SW_SHOWMINIMIZED)
	    return FALSE;

	// Assume the window is snapped when the sizes from two APIs differ.
	GetWindowRect(s_hwnd, &rc);
	if ((rc.right - rc.left !=
		    wp.rcNormalPosition.right - wp.rcNormalPosition.left)
		|| (rc.bottom - rc.top !=
		    wp.rcNormalPosition.bottom - wp.rcNormalPosition.top))
	    return TRUE;
    }
    return FALSE;
}

/*
 * Called when the font changed while the window is maximized or GO_KEEPWINSIZE
 * is set.  Compute the new Rows and Columns.  This is like resizing the
 * window.
 */
    void
gui_mch_newfont(void)
{
    RECT	rect;

    GetWindowRect(s_hwnd, &rect);
    if (win_socket_id == 0)
    {
	gui_resize_shell(rect.right - rect.left
	    - (pGetSystemMetricsForDpi(SM_CXFRAME, s_dpi) +
	       pGetSystemMetricsForDpi(SM_CXPADDEDBORDER, s_dpi)) * 2,
	    rect.bottom - rect.top
	    - (pGetSystemMetricsForDpi(SM_CYFRAME, s_dpi) +
	       pGetSystemMetricsForDpi(SM_CXPADDEDBORDER, s_dpi)) * 2
	    - pGetSystemMetricsForDpi(SM_CYCAPTION, s_dpi)
	    - gui_mswin_get_menu_height(FALSE));
    }
    else
    {
	// Inside another window, don't use the frame and border.
	gui_resize_shell(rect.right - rect.left,
	    rect.bottom - rect.top - gui_mswin_get_menu_height(FALSE));
    }
}

/*
 * Set the window title
 */
    void
gui_mch_settitle(
    char_u  *title,
    char_u  *icon UNUSED)
{
    set_window_title(s_hwnd, (title == NULL ? "VIM" : (char *)title));
}

#if defined(FEAT_MOUSESHAPE) || defined(PROTO)
// Table for shape IDCs.  Keep in sync with the mshape_names[] table in
// misc2.c!
static LPCSTR mshape_idcs[] =
{
    IDC_ARROW,			// arrow
    MAKEINTRESOURCE(0),		// blank
    IDC_IBEAM,			// beam
    IDC_SIZENS,			// updown
    IDC_SIZENS,			// udsizing
    IDC_SIZEWE,			// leftright
    IDC_SIZEWE,			// lrsizing
    IDC_WAIT,			// busy
    IDC_NO,			// no
    IDC_ARROW,			// crosshair
    IDC_ARROW,			// hand1
    IDC_ARROW,			// hand2
    IDC_ARROW,			// pencil
    IDC_ARROW,			// question
    IDC_ARROW,			// right-arrow
    IDC_UPARROW,		// up-arrow
    IDC_ARROW			// last one
};

    void
mch_set_mouse_shape(int shape)
{
    LPCSTR idc;

    if (shape == MSHAPE_HIDE)
	ShowCursor(FALSE);
    else
    {
	if (shape >= MSHAPE_NUMBERED)
	    idc = IDC_ARROW;
	else
	    idc = mshape_idcs[shape];
	SetClassLongPtr(s_textArea, GCLP_HCURSOR, (LONG_PTR)LoadCursor(NULL, idc));
	if (!p_mh)
	{
	    POINT mp;

	    // Set the position to make it redrawn with the new shape.
	    (void)GetCursorPos(&mp);
	    (void)SetCursorPos(mp.x, mp.y);
	    ShowCursor(TRUE);
	}
    }
}
#endif

#if defined(FEAT_BROWSE) || defined(PROTO)
/*
 * Wide version of convert_filter().
 */
    static WCHAR *
convert_filterW(char_u *s)
{
    char_u *tmp;
    int len;
    WCHAR *res;

    tmp = convert_filter(s);
    if (tmp == NULL)
	return NULL;
    len = (int)STRLEN(s) + 3;
    res = enc_to_utf16(tmp, &len);
    vim_free(tmp);
    return res;
}

/*
 * Pop open a file browser and return the file selected, in allocated memory,
 * or NULL if Cancel is hit.
 *  saving  - TRUE if the file will be saved to, FALSE if it will be opened.
 *  title   - Title message for the file browser dialog.
 *  dflt    - Default name of file.
 *  ext     - Default extension to be added to files without extensions.
 *  initdir - directory in which to open the browser (NULL = current dir)
 *  filter  - Filter for matched files to choose from.
 */
    char_u *
gui_mch_browse(
	int saving,
	char_u *title,
	char_u *dflt,
	char_u *ext,
	char_u *initdir,
	char_u *filter)
{
    // We always use the wide function.  This means enc_to_utf16() must work,
    // otherwise it fails miserably!
    OPENFILENAMEW	fileStruct;
    WCHAR		fileBuf[MAXPATHL];
    WCHAR		*wp;
    int			i;
    WCHAR		*titlep = NULL;
    WCHAR		*extp = NULL;
    WCHAR		*initdirp = NULL;
    WCHAR		*filterp;
    char_u		*p, *q;
    BOOL		ret;

    if (dflt == NULL)
	fileBuf[0] = NUL;
    else
    {
	wp = enc_to_utf16(dflt, NULL);
	if (wp == NULL)
	    fileBuf[0] = NUL;
	else
	{
	    for (i = 0; wp[i] != NUL && i < MAXPATHL - 1; ++i)
		fileBuf[i] = wp[i];
	    fileBuf[i] = NUL;
	    vim_free(wp);
	}
    }

    // Convert the filter to Windows format.
    filterp = convert_filterW(filter);

    CLEAR_FIELD(fileStruct);
# ifdef OPENFILENAME_SIZE_VERSION_400W
    // be compatible with Windows NT 4.0
    fileStruct.lStructSize = OPENFILENAME_SIZE_VERSION_400W;
# else
    fileStruct.lStructSize = sizeof(fileStruct);
# endif

    if (title != NULL)
	titlep = enc_to_utf16(title, NULL);
    fileStruct.lpstrTitle = titlep;

    if (ext != NULL)
	extp = enc_to_utf16(ext, NULL);
    fileStruct.lpstrDefExt = extp;

    fileStruct.lpstrFile = fileBuf;
    fileStruct.nMaxFile = MAXPATHL;
    fileStruct.lpstrFilter = filterp;
    fileStruct.hwndOwner = s_hwnd;		// main Vim window is owner
    // has an initial dir been specified?
    if (initdir != NULL && *initdir != NUL)
    {
	// Must have backslashes here, no matter what 'shellslash' says
	initdirp = enc_to_utf16(initdir, NULL);
	if (initdirp != NULL)
	{
	    for (wp = initdirp; *wp != NUL; ++wp)
		if (*wp == '/')
		    *wp = '\\';
	}
	fileStruct.lpstrInitialDir = initdirp;
    }

    /*
     * TODO: Allow selection of multiple files.  Needs another arg to this
     * function to ask for it, and need to use OFN_ALLOWMULTISELECT below.
     * Also, should we use OFN_FILEMUSTEXIST when opening?  Vim can edit on
     * files that don't exist yet, so I haven't put it in.  What about
     * OFN_PATHMUSTEXIST?
     * Don't use OFN_OVERWRITEPROMPT, Vim has its own ":confirm" dialog.
     */
    fileStruct.Flags = (OFN_NOCHANGEDIR | OFN_PATHMUSTEXIST | OFN_HIDEREADONLY);
# ifdef FEAT_SHORTCUT
    if (curbuf->b_p_bin)
	fileStruct.Flags |= OFN_NODEREFERENCELINKS;
# endif
    if (saving)
	ret = GetSaveFileNameW(&fileStruct);
    else
	ret = GetOpenFileNameW(&fileStruct);

    vim_free(filterp);
    vim_free(initdirp);
    vim_free(titlep);
    vim_free(extp);

    if (!ret)
	return NULL;

    // Convert from UTF-16 to 'encoding'.
    p = utf16_to_enc(fileBuf, NULL);
    if (p == NULL)
	return NULL;

    // Give focus back to main window (when using MDI).
    SetFocus(s_hwnd);

    // Shorten the file name if possible
    q = vim_strsave(shorten_fname1(p));
    vim_free(p);
    return q;
}


/*
 * Convert the string s to the proper format for a filter string by replacing
 * the \t and \n delimiters with \0.
 * Returns the converted string in allocated memory.
 *
 * Keep in sync with convert_filterW() above!
 */
    static char_u *
convert_filter(char_u *s)
{
    char_u	*res;
    unsigned	s_len = (unsigned)STRLEN(s);
    unsigned	i;

    res = alloc(s_len + 3);
    if (res != NULL)
    {
	for (i = 0; i < s_len; ++i)
	    if (s[i] == '\t' || s[i] == '\n')
		res[i] = '\0';
	    else
		res[i] = s[i];
	res[s_len] = NUL;
	// Add two extra NULs to make sure it's properly terminated.
	res[s_len + 1] = NUL;
	res[s_len + 2] = NUL;
    }
    return res;
}

/*
 * Select a directory.
 */
    char_u *
gui_mch_browsedir(char_u *title, char_u *initdir)
{
    // We fake this: Use a filter that doesn't select anything and a default
    // file name that won't be used.
    return gui_mch_browse(0, title, (char_u *)_("Not Used"), NULL,
			      initdir, (char_u *)_("Directory\t*.nothing\n"));
}
#endif // FEAT_BROWSE

    static void
_OnDropFiles(
    HWND hwnd UNUSED,
    HDROP hDrop)
{
#define BUFPATHLEN _MAX_PATH
#define DRAGQVAL 0xFFFFFFFF
    WCHAR   wszFile[BUFPATHLEN];
    char    szFile[BUFPATHLEN];
    UINT    cFiles = DragQueryFile(hDrop, DRAGQVAL, NULL, 0);
    UINT    i;
    char_u  **fnames;
    POINT   pt;
    int_u   modifiers = 0;

    // Obtain dropped position
    DragQueryPoint(hDrop, &pt);
    MapWindowPoints(s_hwnd, s_textArea, &pt, 1);

    reset_VIsual();

    fnames = ALLOC_MULT(char_u *, cFiles);

    if (fnames != NULL)
	for (i = 0; i < cFiles; ++i)
	{
	    if (DragQueryFileW(hDrop, i, wszFile, BUFPATHLEN) > 0)
		fnames[i] = utf16_to_enc(wszFile, NULL);
	    else
	    {
		DragQueryFile(hDrop, i, szFile, BUFPATHLEN);
		fnames[i] = vim_strsave((char_u *)szFile);
	    }
	}

    DragFinish(hDrop);

    if (fnames == NULL)
	return;

    int kbd_modifiers = get_active_modifiers();

    if ((kbd_modifiers & MOD_MASK_SHIFT) != 0)
	modifiers |= MOUSE_SHIFT;
    if ((kbd_modifiers & MOD_MASK_CTRL) != 0)
	modifiers |= MOUSE_CTRL;
    if ((kbd_modifiers & MOD_MASK_ALT) != 0)
	modifiers |= MOUSE_ALT;

    gui_handle_drop(pt.x, pt.y, modifiers, fnames, cFiles);

    s_need_activate = TRUE;
}

    static int
_OnScroll(
    HWND hwnd UNUSED,
    HWND hwndCtl,
    UINT code,
    int pos)
{
    static UINT	prev_code = 0;   // code of previous call
    scrollbar_T *sb, *sb_info;
    long	val;
    int		dragging = FALSE;
    int		dont_scroll_save = dont_scroll;
    SCROLLINFO	si;

    si.cbSize = sizeof(si);
    si.fMask = SIF_POS;

    sb = gui_mswin_find_scrollbar(hwndCtl);
    if (sb == NULL)
	return 0;

    if (sb->wp != NULL)		// Left or right scrollbar
    {
	/*
	 * Careful: need to get scrollbar info out of first (left) scrollbar
	 * for window, but keep real scrollbar too because we must pass it to
	 * gui_drag_scrollbar().
	 */
	sb_info = &sb->wp->w_scrollbars[0];
    }
    else	    // Bottom scrollbar
	sb_info = sb;
    val = sb_info->value;

    switch (code)
    {
	case SB_THUMBTRACK:
	    val = pos;
	    dragging = TRUE;
	    if (sb->scroll_shift > 0)
		val <<= sb->scroll_shift;
	    break;
	case SB_LINEDOWN:
	    val++;
	    break;
	case SB_LINEUP:
	    val--;
	    break;
	case SB_PAGEDOWN:
	    val += (sb_info->size > 2 ? sb_info->size - 2 : 1);
	    break;
	case SB_PAGEUP:
	    val -= (sb_info->size > 2 ? sb_info->size - 2 : 1);
	    break;
	case SB_TOP:
	    val = 0;
	    break;
	case SB_BOTTOM:
	    val = sb_info->max;
	    break;
	case SB_ENDSCROLL:
	    if (prev_code == SB_THUMBTRACK)
	    {
		/*
		 * "pos" only gives us 16-bit data.  In case of large file,
		 * use GetScrollPos() which returns 32-bit.  Unfortunately it
		 * is not valid while the scrollbar is being dragged.
		 */
		val = GetScrollPos(hwndCtl, SB_CTL);
		if (sb->scroll_shift > 0)
		    val <<= sb->scroll_shift;
	    }
	    break;

	default:
	    // TRACE("Unknown scrollbar event %d\n", code);
	    return 0;
    }
    prev_code = code;

    si.nPos = (sb->scroll_shift > 0) ? val >> sb->scroll_shift : val;
    SetScrollInfo(hwndCtl, SB_CTL, &si, TRUE);

    /*
     * When moving a vertical scrollbar, move the other vertical scrollbar too.
     */
    if (sb->wp != NULL)
    {
	scrollbar_T *sba = sb->wp->w_scrollbars;
	HWND    id = sba[ (sb == sba + SBAR_LEFT) ? SBAR_RIGHT : SBAR_LEFT].id;

	SetScrollInfo(id, SB_CTL, &si, TRUE);
    }

    // Don't let us be interrupted here by another message.
    s_busy_processing = TRUE;

    // When "allow_scrollbar" is FALSE still need to remember the new
    // position, but don't actually scroll by setting "dont_scroll".
    dont_scroll = !allow_scrollbar;

    mch_disable_flush();
    gui_drag_scrollbar(sb, val, dragging);
    mch_enable_flush();
    gui_may_flush();

    s_busy_processing = FALSE;
    dont_scroll = dont_scroll_save;

    return 0;
}


#ifdef FEAT_XPM_W32
# include "xpm_w32.h"
#endif


// Some parameters for tearoff menus.  All in pixels.
#define TEAROFF_PADDING_X	2
#define TEAROFF_BUTTON_PAD_X	8
#define TEAROFF_MIN_WIDTH	200
#define TEAROFF_SUBMENU_LABEL	">>"
#define TEAROFF_COLUMN_PADDING	3	// # spaces to pad column with.


#ifdef FEAT_BEVAL_GUI
# define ID_BEVAL_TOOLTIP   200
# define BEVAL_TEXT_LEN	    MAXPATHL

static BalloonEval  *cur_beval = NULL;
static UINT_PTR	    beval_timer_id = 0;
static DWORD	    last_user_activity = 0;
#endif // defined(FEAT_BEVAL_GUI)


// Local variables:

#ifdef FEAT_MENU
static UINT	s_menu_id = 100;
#endif

/*
 * Use the system font for dialogs and tear-off menus.  Remove this line to
 * use DLG_FONT_NAME.
 */
#define USE_SYSMENU_FONT

#define VIM_NAME	"vim"
#define VIM_CLASSW	L"Vim"

// Initial size for the dialog template.  For gui_mch_dialog() it's fixed,
// thus there should be room for every dialog.  For tearoffs it's made bigger
// when needed.
#define DLG_ALLOC_SIZE 16 * 1024

/*
 * stuff for dialogs, menus, tearoffs etc.
 */
static PWORD
add_dialog_element(
	PWORD p,
	DWORD lStyle,
	WORD x,
	WORD y,
	WORD w,
	WORD h,
	WORD Id,
	WORD clss,
	const char *caption);
static LPWORD lpwAlign(LPWORD);
static int nCopyAnsiToWideChar(LPWORD, LPSTR, BOOL);
#if defined(FEAT_MENU) && defined(FEAT_TEAROFF)
static void gui_mch_tearoff(char_u *title, vimmenu_T *menu, int initX, int initY);
#endif
static void get_dialog_font_metrics(void);

static int dialog_default_button = -1;

#ifdef FEAT_TOOLBAR
static void initialise_toolbar(void);
static void update_toolbar_size(void);
static LRESULT CALLBACK toolbar_wndproc(HWND hwnd, UINT uMsg, WPARAM wParam, LPARAM lParam);
static int get_toolbar_bitmap(vimmenu_T *menu);
#else
# define update_toolbar_size()
#endif

#ifdef FEAT_GUI_TABLINE
static void initialise_tabline(void);
static LRESULT CALLBACK tabline_wndproc(HWND hwnd, UINT uMsg, WPARAM wParam, LPARAM lParam);
#endif

#ifdef FEAT_MBYTE_IME
static LRESULT _OnImeComposition(HWND hwnd, WPARAM dbcs, LPARAM param);
static char_u *GetResultStr(HWND hwnd, int GCS, int *lenp);
#endif
#if defined(FEAT_MBYTE_IME) && defined(DYNAMIC_IME)
# ifdef NOIME
typedef struct tagCOMPOSITIONFORM {
    DWORD dwStyle;
    POINT ptCurrentPos;
    RECT  rcArea;
} COMPOSITIONFORM, *PCOMPOSITIONFORM, NEAR *NPCOMPOSITIONFORM, FAR *LPCOMPOSITIONFORM;
typedef HANDLE HIMC;
# endif

static HINSTANCE hLibImm = NULL;
static LONG (WINAPI *pImmGetCompositionStringW)(HIMC, DWORD, LPVOID, DWORD);
static HIMC (WINAPI *pImmGetContext)(HWND);
static HIMC (WINAPI *pImmAssociateContext)(HWND, HIMC);
static BOOL (WINAPI *pImmReleaseContext)(HWND, HIMC);
static BOOL (WINAPI *pImmGetOpenStatus)(HIMC);
static BOOL (WINAPI *pImmSetOpenStatus)(HIMC, BOOL);
static BOOL (WINAPI *pImmGetCompositionFontW)(HIMC, LPLOGFONTW);
static BOOL (WINAPI *pImmSetCompositionFontW)(HIMC, LPLOGFONTW);
static BOOL (WINAPI *pImmSetCompositionWindow)(HIMC, LPCOMPOSITIONFORM);
static BOOL (WINAPI *pImmGetConversionStatus)(HIMC, LPDWORD, LPDWORD);
static BOOL (WINAPI *pImmSetConversionStatus)(HIMC, DWORD, DWORD);
static void dyn_imm_load(void);
#else
# define pImmGetCompositionStringW ImmGetCompositionStringW
# define pImmGetContext		  ImmGetContext
# define pImmAssociateContext	  ImmAssociateContext
# define pImmReleaseContext	  ImmReleaseContext
# define pImmGetOpenStatus	  ImmGetOpenStatus
# define pImmSetOpenStatus	  ImmSetOpenStatus
# define pImmGetCompositionFontW  ImmGetCompositionFontW
# define pImmSetCompositionFontW  ImmSetCompositionFontW
# define pImmSetCompositionWindow ImmSetCompositionWindow
# define pImmGetConversionStatus  ImmGetConversionStatus
# define pImmSetConversionStatus  ImmSetConversionStatus
#endif

#ifdef FEAT_MENU
/*
 * Figure out how high the menu bar is at the moment.
 */
    static int
gui_mswin_get_menu_height(
    int	    fix_window)	    // If TRUE, resize window if menu height changed
{
    static int	old_menu_height = -1;

    RECT    rc1, rc2;
    int	    num;
    int	    menu_height;

    if (gui.menu_is_active)
	num = GetMenuItemCount(s_menuBar);
    else
	num = 0;

    if (num == 0)
	menu_height = 0;
    else if (IsMinimized(s_hwnd))
    {
	// The height of the menu cannot be determined while the window is
	// minimized.  Take the previous height if the menu is changed in that
	// state, to avoid that Vim's vertical window size accidentally
	// increases due to the unaccounted-for menu height.
	menu_height = old_menu_height == -1 ? 0 : old_menu_height;
    }
    else
    {
	/*
	 * In case 'lines' is set in _vimrc/_gvimrc window width doesn't
	 * seem to have been set yet, so menu wraps in default window
	 * width which is very narrow.  Instead just return height of a
	 * single menu item.  Will still be wrong when the menu really
	 * should wrap over more than one line.
	 */
	GetMenuItemRect(s_hwnd, s_menuBar, 0, &rc1);
	if (gui.starting)
	    menu_height = rc1.bottom - rc1.top + 1;
	else
	{
	    GetMenuItemRect(s_hwnd, s_menuBar, num - 1, &rc2);
	    menu_height = rc2.bottom - rc1.top + 1;
	}
    }

    if (fix_window && menu_height != old_menu_height)
	gui_set_shellsize(FALSE, FALSE, RESIZE_VERT);
    old_menu_height = menu_height;

    return menu_height;
}
#endif // FEAT_MENU


/*
 * Setup for the Intellimouse
 */
    static long
mouse_vertical_scroll_step(void)
{
    UINT val;
    if (SystemParametersInfo(SPI_GETWHEELSCROLLLINES, 0, &val, 0))
	return (val != WHEEL_PAGESCROLL) ? (long)val : -1;
    return 3; // Safe default;
}

    static long
mouse_horizontal_scroll_step(void)
{
    UINT val;
    if (SystemParametersInfo(SPI_GETWHEELSCROLLCHARS, 0, &val, 0))
	return (long)val;
    return 3; // Safe default;
}

    static void
init_mouse_wheel(void)
{
    // Get the default values for the horizontal and vertical scroll steps from
    // the system.
    mouse_set_vert_scroll_step(mouse_vertical_scroll_step());
    mouse_set_hor_scroll_step(mouse_horizontal_scroll_step());
}

/*
 * Mouse scroll event handler.
 */
    static void
_OnMouseWheel(HWND hwnd UNUSED, WPARAM wParam, LPARAM lParam, int horizontal)
{
    int		button;
    win_T	*wp;
    int		modifiers = 0;
    int		kbd_modifiers;
    int		zDelta = GET_WHEEL_DELTA_WPARAM(wParam);
    POINT	pt;

    wp = gui_mouse_window(FIND_POPUP);

#ifdef FEAT_PROP_POPUP
    if (wp != NULL && popup_is_popup(wp))
    {
	cmdarg_T cap;
	oparg_T	oa;

	// Mouse hovers over popup window, scroll it if possible.
	mouse_row = wp->w_winrow;
	mouse_col = wp->w_wincol;
	CLEAR_FIELD(cap);
	if (horizontal)
	{
	    cap.arg = zDelta < 0 ? MSCR_LEFT : MSCR_RIGHT;
	    cap.cmdchar = zDelta < 0 ? K_MOUSELEFT : K_MOUSERIGHT;
	}
	else
	{
	    cap.arg = zDelta < 0 ? MSCR_UP : MSCR_DOWN;
	    cap.cmdchar = zDelta < 0 ? K_MOUSEUP : K_MOUSEDOWN;
	}
	clear_oparg(&oa);
	cap.oap = &oa;
	nv_mousescroll(&cap);
	update_screen(0);
	setcursor();
	out_flush();
	return;
    }
#endif

    if (wp == NULL || !p_scf)
	wp = curwin;

    // Translate the scroll event into an event that Vim can process so that
    // the user has a chance to map the scrollwheel buttons.
    if (horizontal)
	button = zDelta >= 0 ? MOUSE_6 : MOUSE_7;
    else
	button = zDelta >= 0 ? MOUSE_4 : MOUSE_5;

    kbd_modifiers = get_active_modifiers();

    if ((kbd_modifiers & MOD_MASK_SHIFT) != 0)
	modifiers |= MOUSE_SHIFT;
    if ((kbd_modifiers & MOD_MASK_CTRL) != 0)
	modifiers |= MOUSE_CTRL;
    if ((kbd_modifiers & MOD_MASK_ALT) != 0)
	modifiers |= MOUSE_ALT;

    // The cursor position is relative to the upper-left corner of the screen.
    pt.x = GET_X_LPARAM(lParam);
    pt.y = GET_Y_LPARAM(lParam);
    ScreenToClient(s_textArea, &pt);

    gui_send_mouse_event(button, pt.x, pt.y, FALSE, modifiers);
}

#ifdef USE_SYSMENU_FONT
/*
 * Get Menu Font.
 * Return OK or FAIL.
 */
    static int
gui_w32_get_menu_font(LOGFONTW *lf)
{
    NONCLIENTMETRICSW nm;

    nm.cbSize = sizeof(NONCLIENTMETRICSW);
    if (!SystemParametersInfoW(
	    SPI_GETNONCLIENTMETRICS,
	    sizeof(NONCLIENTMETRICSW),
	    &nm,
	    0))
	return FAIL;
    *lf = nm.lfMenuFont;
    return OK;
}
#endif


#if defined(FEAT_GUI_TABLINE) && defined(USE_SYSMENU_FONT)
/*
 * Set the GUI tabline font to the system menu font
 */
    static void
set_tabline_font(void)
{
    LOGFONTW	lfSysmenu;
    HFONT	font;
    HWND	hwnd;
    HDC		hdc;
    HFONT	hfntOld;
    TEXTMETRIC	tm;

    if (gui_w32_get_menu_font(&lfSysmenu) != OK)
	return;

    lfSysmenu.lfHeight = adjust_fontsize_by_dpi(lfSysmenu.lfHeight);
    font = CreateFontIndirectW(&lfSysmenu);

    SendMessage(s_tabhwnd, WM_SETFONT, (WPARAM)font, TRUE);

    /*
     * Compute the height of the font used for the tab text
     */
    hwnd = GetDesktopWindow();
    hdc = GetWindowDC(hwnd);
    hfntOld = SelectFont(hdc, font);

    GetTextMetrics(hdc, &tm);

    SelectFont(hdc, hfntOld);
    ReleaseDC(hwnd, hdc);

    /*
     * The space used by the tab border and the space between the tab label
     * and the tab border is included as 7.
     */
    gui.tabline_height = tm.tmHeight + tm.tmInternalLeading + 7;
}
#else
# define set_tabline_font()
#endif

/*
 * Invoked when a setting was changed.
 */
    static LRESULT CALLBACK
_OnSettingChange(UINT param)
{
    switch (param)
    {
	case SPI_SETWHEELSCROLLLINES:
	    mouse_set_vert_scroll_step(mouse_vertical_scroll_step());
	    break;
	case SPI_SETWHEELSCROLLCHARS:
	    mouse_set_hor_scroll_step(mouse_horizontal_scroll_step());
	    break;
	case SPI_SETNONCLIENTMETRICS:
	    set_tabline_font();
	    break;
	default:
	    break;
    }
    return 0;
}

#ifdef FEAT_NETBEANS_INTG
    static void
_OnWindowPosChanged(
    HWND hwnd,
    const LPWINDOWPOS lpwpos)
{
    static int x = 0, y = 0, cx = 0, cy = 0;
    extern int WSInitialized;

    if (WSInitialized && (lpwpos->x != x || lpwpos->y != y
				     || lpwpos->cx != cx || lpwpos->cy != cy))
    {
	x = lpwpos->x;
	y = lpwpos->y;
	cx = lpwpos->cx;
	cy = lpwpos->cy;
	netbeans_frame_moved(x, y);
    }
    // Allow to send WM_SIZE and WM_MOVE
    FORWARD_WM_WINDOWPOSCHANGED(hwnd, lpwpos, DefWindowProcW);
}
#endif


static HWND hwndTip = NULL;

    static void
show_sizing_tip(int cols, int rows)
{
    TOOLINFOA ti;
    char buf[32];

    ti.cbSize = sizeof(ti);
    ti.hwnd = s_hwnd;
    ti.uId = (UINT_PTR)s_hwnd;
    ti.uFlags = TTF_SUBCLASS | TTF_IDISHWND;
    ti.lpszText = buf;
    sprintf(buf, "%dx%d", cols, rows);
    if (hwndTip == NULL)
    {
	hwndTip = CreateWindowExA(0, TOOLTIPS_CLASSA, NULL,
		WS_POPUP | TTS_ALWAYSTIP | TTS_NOPREFIX,
		CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT,
		s_hwnd, NULL, GetModuleHandle(NULL), NULL);
	SendMessage(hwndTip, TTM_ADDTOOL, 0, (LPARAM)&ti);
	SendMessage(hwndTip, TTM_TRACKACTIVATE, TRUE, (LPARAM)&ti);
    }
    else
    {
	SendMessage(hwndTip, TTM_UPDATETIPTEXT, 0, (LPARAM)&ti);
    }
    SendMessage(hwndTip, TTM_POPUP, 0, 0);
}

    static void
destroy_sizing_tip(void)
{
    if (hwndTip == NULL)
	return;

    DestroyWindow(hwndTip);
    hwndTip = NULL;
}

    static int
_DuringSizing(
    UINT fwSide,
    LPRECT lprc)
{
    int	    w, h;
    int	    valid_w, valid_h;
    int	    w_offset, h_offset;
    int	    cols, rows;

    w = lprc->right - lprc->left;
    h = lprc->bottom - lprc->top;
    gui_mswin_get_valid_dimensions(w, h, &valid_w, &valid_h, &cols, &rows);
    w_offset = w - valid_w;
    h_offset = h - valid_h;

    if (fwSide == WMSZ_LEFT || fwSide == WMSZ_TOPLEFT
			    || fwSide == WMSZ_BOTTOMLEFT)
	lprc->left += w_offset;
    else if (fwSide == WMSZ_RIGHT || fwSide == WMSZ_TOPRIGHT
			    || fwSide == WMSZ_BOTTOMRIGHT)
	lprc->right -= w_offset;

    if (fwSide == WMSZ_TOP || fwSide == WMSZ_TOPLEFT
			    || fwSide == WMSZ_TOPRIGHT)
	lprc->top += h_offset;
    else if (fwSide == WMSZ_BOTTOM || fwSide == WMSZ_BOTTOMLEFT
			    || fwSide == WMSZ_BOTTOMRIGHT)
	lprc->bottom -= h_offset;

    show_sizing_tip(cols, rows);
    return TRUE;
}

#ifdef FEAT_GUI_TABLINE
    static void
_OnRButtonUp(HWND hwnd, int x, int y, UINT keyFlags)
{
    if (gui_mch_showing_tabline())
    {
	POINT pt;
	RECT rect;

	/*
	 * If the cursor is on the tabline, display the tab menu
	 */
	GetCursorPos(&pt);
	GetWindowRect(s_textArea, &rect);
	if (pt.y < rect.top)
	{
	    show_tabline_popup_menu();
	    return;
	}
    }
    FORWARD_WM_RBUTTONUP(hwnd, x, y, keyFlags, DefWindowProcW);
}

    static void
_OnLButtonDown(HWND hwnd, BOOL fDoubleClick, int x, int y, UINT keyFlags)
{
    /*
     * If the user double clicked the tabline, create a new tab
     */
    if (gui_mch_showing_tabline())
    {
	POINT pt;
	RECT rect;

	GetCursorPos(&pt);
	GetWindowRect(s_textArea, &rect);
	if (pt.y < rect.top)
	    send_tabline_menu_event(0, TABLINE_MENU_NEW);
    }
    FORWARD_WM_LBUTTONDOWN(hwnd, fDoubleClick, x, y, keyFlags, DefWindowProcW);
}
#endif

    static UINT
_OnNCHitTest(HWND hwnd, int xPos, int yPos)
{
    UINT	result;
    int		x, y;

    result = FORWARD_WM_NCHITTEST(hwnd, xPos, yPos, DefWindowProcW);
    if (result != HTCLIENT)
	return result;

#ifdef FEAT_GUI_TABLINE
    if (gui_mch_showing_tabline())
    {
	RECT rct;

	// If the cursor is on the GUI tabline, don't process this event
	GetWindowRect(s_textArea, &rct);
	if (yPos < rct.top)
	    return result;
    }
#endif
    (void)gui_mch_get_winpos(&x, &y);
    xPos -= x;

    if (xPos < 48) // <VN> TODO should use system metric?
	return HTBOTTOMLEFT;
    else
	return HTBOTTOMRIGHT;
}

#if defined(FEAT_TOOLBAR) || defined(FEAT_GUI_TABLINE)
    static LRESULT
_OnNotify(HWND hwnd, UINT id, NMHDR *hdr)
{
    switch (hdr->code)
    {
	case TTN_GETDISPINFOW:
	case TTN_GETDISPINFO:
	    {
		char_u		*str = NULL;
		static void	*tt_text = NULL;

		VIM_CLEAR(tt_text);

# ifdef FEAT_GUI_TABLINE
		if (gui_mch_showing_tabline()
			&& hdr->hwndFrom == TabCtrl_GetToolTips(s_tabhwnd))
		{
		    POINT	pt;
		    /*
		     * Mouse is over the GUI tabline. Display the
		     * tooltip for the tab under the cursor
		     *
		     * Get the cursor position within the tab control
		     */
		    GetCursorPos(&pt);
		    if (ScreenToClient(s_tabhwnd, &pt) != 0)
		    {
			TCHITTESTINFO htinfo;
			int idx;

			/*
			 * Get the tab under the cursor
			 */
			htinfo.pt.x = pt.x;
			htinfo.pt.y = pt.y;
			idx = TabCtrl_HitTest(s_tabhwnd, &htinfo);
			if (idx != -1)
			{
			    tabpage_T *tp;

			    tp = find_tabpage(idx + 1);
			    if (tp != NULL)
			    {
				get_tabline_label(tp, TRUE);
				str = NameBuff;
			    }
			}
		    }
		}
# endif
# ifdef FEAT_TOOLBAR
#  ifdef FEAT_GUI_TABLINE
		else
#  endif
		{
		    UINT	idButton;
		    vimmenu_T	*pMenu;

		    idButton = (UINT) hdr->idFrom;
		    pMenu = gui_mswin_find_menu(root_menu, idButton);
		    if (pMenu)
			str = pMenu->strings[MENU_INDEX_TIP];
		}
# endif
		if (str == NULL)
		    break;

		// Set the maximum width, this also enables using \n for
		// line break.
		SendMessage(hdr->hwndFrom, TTM_SETMAXTIPWIDTH, 0, 500);

		if (hdr->code == TTN_GETDISPINFOW)
		{
		    LPNMTTDISPINFOW	lpdi = (LPNMTTDISPINFOW)hdr;

		    tt_text = enc_to_utf16(str, NULL);
		    lpdi->lpszText = tt_text;
		    // can't show tooltip if failed
		}
		else
		{
		    LPNMTTDISPINFO	lpdi = (LPNMTTDISPINFO)hdr;

		    if (STRLEN(str) < sizeof(lpdi->szText)
			    || ((tt_text = vim_strsave(str)) == NULL))
			vim_strncpy((char_u *)lpdi->szText, str,
				sizeof(lpdi->szText) - 1);
		    else
			lpdi->lpszText = tt_text;
		}
	    }
	    break;

# ifdef FEAT_GUI_TABLINE
	case TCN_SELCHANGE:
	    if (gui_mch_showing_tabline() && (hdr->hwndFrom == s_tabhwnd))
	    {
		send_tabline_event(TabCtrl_GetCurSel(s_tabhwnd) + 1);
		return 0L;
	    }
	    break;

	case NM_RCLICK:
	    if (gui_mch_showing_tabline() && (hdr->hwndFrom == s_tabhwnd))
	    {
		show_tabline_popup_menu();
		return 0L;
	    }
	    break;
# endif

	default:
	    break;
    }
    return DefWindowProcW(hwnd, WM_NOTIFY, (WPARAM)id, (LPARAM)hdr);
}
#endif

#if defined(MENUHINTS) && defined(FEAT_MENU)
    static LRESULT
_OnMenuSelect(HWND hwnd, WPARAM wParam, LPARAM lParam)
{
    if (((UINT) HIWORD(wParam)
		& (0xffff ^ (MF_MOUSESELECT + MF_BITMAP + MF_POPUP)))
	    == MF_HILITE
	    && (State & MODE_CMDLINE) == 0)
    {
	UINT	    idButton;
	vimmenu_T   *pMenu;
	static int  did_menu_tip = FALSE;

	if (did_menu_tip)
	{
	    msg_clr_cmdline();
	    setcursor();
	    out_flush();
	    did_menu_tip = FALSE;
	}

	idButton = (UINT)LOWORD(wParam);
	pMenu = gui_mswin_find_menu(root_menu, idButton);
	if (pMenu != NULL && pMenu->strings[MENU_INDEX_TIP] != 0
		&& GetMenuState(s_menuBar, pMenu->id, MF_BYCOMMAND) != -1)
	{
	    ++msg_hist_off;
	    msg((char *)pMenu->strings[MENU_INDEX_TIP]);
	    --msg_hist_off;
	    setcursor();
	    out_flush();
	    did_menu_tip = TRUE;
	}
	return 0L;
    }
    return DefWindowProcW(hwnd, WM_MENUSELECT, wParam, lParam);
}
#endif

    static BOOL
_OnGetDpiScaledSize(HWND hwnd, UINT dpi, SIZE *size)
{
    int		old_width, old_height;
    int		new_width, new_height;
    LOGFONTW	lf;
    HFONT	font;

    //TRACE("DPI: %d, SIZE=(%d,%d), s_dpi: %d", dpi, size->cx, size->cy, s_dpi);

    // Calculate new approximate size.
    GetFontSize(gui.norm_font, &old_width, &old_height);    // Current size
    GetObjectW((HFONT)gui.norm_font, sizeof(lf), &lf);
    lf.lfHeight = lf.lfHeight * (int)dpi / s_dpi;
    font = CreateFontIndirectW(&lf);
    if (font)
    {
	GetFontSize((GuiFont)font, &new_width, &new_height);	// New size
	DeleteFont(font);
    }
    else
    {
	new_width = old_width;
	new_height = old_height;
    }
    size->cx = size->cx * new_width / old_width;
    size->cy = size->cy * new_height / old_height;
    //TRACE("New approx. SIZE=(%d,%d)", size->cx, size->cy);

    return TRUE;
}

    static LRESULT
_OnDpiChanged(HWND hwnd, UINT xdpi UNUSED, UINT ydpi, RECT *rc)
{
    s_dpi = ydpi;
    s_in_dpichanged = TRUE;
    //TRACE("DPI: %d", ydpi);

    s_suggested_rect = *rc;
    //TRACE("Suggested pos&size: %d,%d %d,%d", rc->left, rc->top,
    //		rc->right - rc->left, rc->bottom - rc->top);

    update_scrollbar_size();
    update_toolbar_size();
    set_tabline_font();

    gui_init_font(*p_guifont == NUL ? hl_get_font_name() : p_guifont, FALSE);
    gui_get_wide_font();
    gui_mswin_get_menu_height(FALSE);
#ifdef FEAT_MBYTE_IME
    im_set_position(gui.row, gui.col);
#endif
    InvalidateRect(hwnd, NULL, TRUE);

    s_in_dpichanged = FALSE;
    return 0L;
}


    static LRESULT CALLBACK
_WndProc(
    HWND hwnd,
    UINT uMsg,
    WPARAM wParam,
    LPARAM lParam)
{
    // ch_log(NULL, "WndProc: hwnd = %08x, msg = %x, wParam = %x, lParam = %x",
	    // hwnd, uMsg, wParam, lParam);

    HandleMouseHide(uMsg, lParam);

    s_uMsg = uMsg;
    s_wParam = wParam;
    s_lParam = lParam;

    switch (uMsg)
    {
	HANDLE_MSG(hwnd, WM_DEADCHAR,	_OnDeadChar);
	HANDLE_MSG(hwnd, WM_SYSDEADCHAR, _OnDeadChar);
	// HANDLE_MSG(hwnd, WM_ACTIVATE,    _OnActivate);
	HANDLE_MSG(hwnd, WM_CLOSE,	_OnClose);
	// HANDLE_MSG(hwnd, WM_COMMAND,	_OnCommand);
	HANDLE_MSG(hwnd, WM_DESTROY,	_OnDestroy);
	HANDLE_MSG(hwnd, WM_DROPFILES,	_OnDropFiles);
	HANDLE_MSG(hwnd, WM_HSCROLL,	_OnScroll);
	HANDLE_MSG(hwnd, WM_KILLFOCUS,	_OnKillFocus);
#ifdef FEAT_MENU
	HANDLE_MSG(hwnd, WM_COMMAND,	_OnMenu);
#endif
	// HANDLE_MSG(hwnd, WM_MOVE,	    _OnMove);
	// HANDLE_MSG(hwnd, WM_NCACTIVATE,  _OnNCActivate);
	HANDLE_MSG(hwnd, WM_SETFOCUS,	_OnSetFocus);
	HANDLE_MSG(hwnd, WM_SIZE,	_OnSize);
	// HANDLE_MSG(hwnd, WM_SYSCOMMAND,  _OnSysCommand);
	// HANDLE_MSG(hwnd, WM_SYSKEYDOWN,  _OnAltKey);
	HANDLE_MSG(hwnd, WM_VSCROLL,	_OnScroll);
	// HANDLE_MSG(hwnd, WM_WINDOWPOSCHANGING,	_OnWindowPosChanging);
	HANDLE_MSG(hwnd, WM_ACTIVATEAPP, _OnActivateApp);
#ifdef FEAT_NETBEANS_INTG
	HANDLE_MSG(hwnd, WM_WINDOWPOSCHANGED, _OnWindowPosChanged);
#endif
#ifdef FEAT_GUI_TABLINE
	HANDLE_MSG(hwnd, WM_RBUTTONUP,	_OnRButtonUp);
	HANDLE_MSG(hwnd, WM_LBUTTONDBLCLK,  _OnLButtonDown);
#endif
	HANDLE_MSG(hwnd, WM_NCHITTEST,	_OnNCHitTest);

    case WM_QUERYENDSESSION:	// System wants to go down.
	gui_shell_closed();	// Will exit when no changed buffers.
	return FALSE;		// Do NOT allow system to go down.

    case WM_ENDSESSION:
	if (wParam)	// system only really goes down when wParam is TRUE
	{
	    _OnEndSession();
	    return 0L;
	}
	break;

    case WM_CHAR:
	// Don't use HANDLE_MSG() for WM_CHAR, it truncates wParam to a single
	// byte while we want the UTF-16 character value.
	_OnChar(hwnd, (UINT)wParam, (int)(short)LOWORD(lParam));
	return 0L;

    case WM_SYSCHAR:
	/*
	 * if 'winaltkeys' is "no", or it's "menu" and it's not a menu
	 * shortcut key, handle like a typed ALT key, otherwise call Windows
	 * ALT key handling.
	 */
#ifdef FEAT_MENU
	if (	!gui.menu_is_active
		|| p_wak[0] == 'n'
		|| (p_wak[0] == 'm' && !gui_is_menu_shortcut((int)wParam))
		)
#endif
	{
	    _OnSysChar(hwnd, (UINT)wParam, (int)(short)LOWORD(lParam));
	    return 0L;
	}
#ifdef FEAT_MENU
	else
	    return DefWindowProcW(hwnd, uMsg, wParam, lParam);
#endif

    case WM_SYSKEYUP:
#ifdef FEAT_MENU
	// This used to be done only when menu is active: ALT key is used for
	// that.  But that caused problems when menu is disabled and using
	// Alt-Tab-Esc: get into a strange state where no mouse-moved events
	// are received, mouse pointer remains hidden.
	return DefWindowProcW(hwnd, uMsg, wParam, lParam);
#else
	return 0L;
#endif

    case WM_EXITSIZEMOVE:
	destroy_sizing_tip();
	break;

    case WM_SIZING:	// HANDLE_MSG doesn't seem to handle this one
	return _DuringSizing((UINT)wParam, (LPRECT)lParam);

    case WM_MOUSEWHEEL:
    case WM_MOUSEHWHEEL:
	_OnMouseWheel(hwnd, wParam, lParam, uMsg == WM_MOUSEHWHEEL);
	return 0L;

	// Notification for change in SystemParametersInfo()
    case WM_SETTINGCHANGE:
	return _OnSettingChange((UINT)wParam);

#if defined(FEAT_TOOLBAR) || defined(FEAT_GUI_TABLINE)
    case WM_NOTIFY:
	return _OnNotify(hwnd, (UINT)wParam, (NMHDR*)lParam);
#endif

#if defined(MENUHINTS) && defined(FEAT_MENU)
    case WM_MENUSELECT:
	return _OnMenuSelect(hwnd, wParam, lParam);
#endif

#ifdef FEAT_MBYTE_IME
    case WM_IME_NOTIFY:
	if (!_OnImeNotify(hwnd, (DWORD)wParam, (DWORD)lParam))
	    return DefWindowProcW(hwnd, uMsg, wParam, lParam);
	return 1L;

    case WM_IME_COMPOSITION:
	if (!_OnImeComposition(hwnd, wParam, lParam))
	    return DefWindowProcW(hwnd, uMsg, wParam, lParam);
	return 1L;
#endif
    case WM_GETDPISCALEDSIZE:
	return _OnGetDpiScaledSize(hwnd, (UINT)wParam, (SIZE *)lParam);
    case WM_DPICHANGED:
	return _OnDpiChanged(hwnd, (UINT)LOWORD(wParam), (UINT)HIWORD(wParam),
		(RECT*)lParam);

    default:
#ifdef MSWIN_FIND_REPLACE
	if (uMsg == s_findrep_msg && s_findrep_msg != 0)
	    _OnFindRepl();
#endif
	break;
    }

    return DefWindowProcW(hwnd, uMsg, wParam, lParam);
}

/*
 * End of call-back routines
 */

// parent window, if specified with -P
HWND vim_parent_hwnd = NULL;

    static BOOL CALLBACK
FindWindowTitle(HWND hwnd, LPARAM lParam)
{
    char	buf[2048];
    char	*title = (char *)lParam;

    if (GetWindowText(hwnd, buf, sizeof(buf)))
    {
	if (strstr(buf, title) != NULL)
	{
	    // Found it.  Store the window ref. and quit searching if MDI
	    // works.
	    vim_parent_hwnd = FindWindowEx(hwnd, NULL, "MDIClient", NULL);
	    if (vim_parent_hwnd != NULL)
		return FALSE;
	}
    }
    return TRUE;	// continue searching
}

/*
 * Invoked for '-P "title"' argument: search for parent application to open
 * our window in.
 */
    void
gui_mch_set_parent(char *title)
{
    EnumWindows(FindWindowTitle, (LPARAM)title);
    if (vim_parent_hwnd == NULL)
    {
	semsg(_(e_cannot_find_window_title_str), title);
	mch_exit(2);
    }
}

#ifndef FEAT_OLE
    static void
ole_error(char *arg)
{
    char buf[IOSIZE];

# ifdef VIMDLL
    gui.in_use = mch_is_gui_executable();
# endif

    // Can't use emsg() here, we have not finished initialisation yet.
    vim_snprintf(buf, IOSIZE,
			 _(e_argument_not_supported_str_use_ole_version), arg);
    mch_errmsg(buf);
}
#endif

#if defined(GUI_MAY_SPAWN) || defined(PROTO)
    static char *
gvim_error(void)
{
    char *msg = _(e_gui_cannot_be_used_cannot_execute_gvim_exe);

    if (starting)
    {
	mch_errmsg(msg);
	mch_errmsg("\n");
	mch_exit(2);
    }
    return msg;
}

    char *
gui_mch_do_spawn(char_u *arg)
{
    int			len;
# if defined(FEAT_SESSION) && defined(EXPERIMENTAL_GUI_CMD)
    char_u		*session = NULL;
    LPWSTR		tofree1 = NULL;
# endif
    WCHAR		name[MAX_PATH];
    LPWSTR		cmd, newcmd = NULL, p, warg, tofree2 = NULL;
    STARTUPINFOW	si = {sizeof(si)};
    PROCESS_INFORMATION pi;

    if (!GetModuleFileNameW(g_hinst, name, MAX_PATH))
	goto error;
    p = wcsrchr(name, L'\\');
    if (p == NULL)
	goto error;
    // Replace the executable name from vim(d).exe to gvim(d).exe.
# ifdef DEBUG
    wcscpy(p + 1, L"gvimd.exe");
# else
    wcscpy(p + 1, L"gvim.exe");
# endif

# if defined(FEAT_SESSION) && defined(EXPERIMENTAL_GUI_CMD)
    if (starting)
# endif
    {
	// Pass the command line to the new process.
	p = GetCommandLineW();
	// Skip 1st argument.
	while (*p && *p != L' ' && *p != L'\t')
	{
	    if (*p == L'"')
	    {
		while (*p && *p != L'"')
		    ++p;
		if (*p)
		    ++p;
	    }
	    else
		++p;
	}
	cmd = p;
    }
# if defined(FEAT_SESSION) && defined(EXPERIMENTAL_GUI_CMD)
    else
    {
	// Create a session file and pass it to the new process.
	LPWSTR	wsession;
	char_u	*savebg;
	int	ret;

	session = vim_tempname('s', FALSE);
	if (session == NULL)
	    goto error;
	savebg = p_bg;
	p_bg = vim_strsave((char_u *)"light");	// Set 'bg' to "light".
	ret = write_session_file(session);
	vim_free(p_bg);
	p_bg = savebg;
	if (!ret)
	    goto error;
	wsession = enc_to_utf16(session, NULL);
	if (wsession == NULL)
	    goto error;
	len = (int)wcslen(wsession) * 2 + 27 + 1;
	cmd = ALLOC_MULT(WCHAR, len);
	if (cmd == NULL)
	{
	    vim_free(wsession);
	    goto error;
	}
	tofree1 = cmd;
	_snwprintf(cmd, len, L" -S \"%s\" -c \"call delete('%s')\"",
		wsession, wsession);
	vim_free(wsession);
    }
# endif

    // Check additional arguments to the `:gui` command.
    if (arg != NULL)
    {
	warg = enc_to_utf16(arg, NULL);
	if (warg == NULL)
	    goto error;
	tofree2 = warg;
    }
    else
	warg = L"";

    // Set up the new command line.
    len = (int)wcslen(name) + (int)wcslen(cmd) + (int)wcslen(warg) + 4;
    newcmd = ALLOC_MULT(WCHAR, len);
    if (newcmd == NULL)
	goto error;
    _snwprintf(newcmd, len, L"\"%s\"%s %s", name, cmd, warg);

    // Spawn a new GUI process.
    if (!CreateProcessW(NULL, newcmd, NULL, NULL, TRUE, 0,
		NULL, NULL, &si, &pi))
	goto error;
    CloseHandle(pi.hProcess);
    CloseHandle(pi.hThread);
    mch_exit(0);

error:
# if defined(FEAT_SESSION) && defined(EXPERIMENTAL_GUI_CMD)
    if (session)
	mch_remove(session);
    vim_free(session);
    vim_free(tofree1);
# endif
    vim_free(newcmd);
    vim_free(tofree2);
    return gvim_error();
}
#endif

/*
 * Parse the GUI related command-line arguments.  Any arguments used are
 * deleted from argv, and *argc is decremented accordingly.  This is called
 * when Vim is started, whether or not the GUI has been started.
 */
    void
gui_mch_prepare(int *argc, char **argv)
{
    int		silent = FALSE;
    int		idx;

    // Check for special OLE command line parameters
    if ((*argc == 2 || *argc == 3) && (argv[1][0] == '-' || argv[1][0] == '/'))
    {
	// Check for a "-silent" argument first.
	if (*argc == 3 && STRICMP(argv[1] + 1, "silent") == 0
		&& (argv[2][0] == '-' || argv[2][0] == '/'))
	{
	    silent = TRUE;
	    idx = 2;
	}
	else
	    idx = 1;

	// Register Vim as an OLE Automation server
	if (STRICMP(argv[idx] + 1, "register") == 0)
	{
#ifdef FEAT_OLE
	    RegisterMe(silent);
	    mch_exit(0);
#else
	    if (!silent)
		ole_error("register");
	    mch_exit(2);
#endif
	}

	// Unregister Vim as an OLE Automation server
	if (STRICMP(argv[idx] + 1, "unregister") == 0)
	{
#ifdef FEAT_OLE
	    UnregisterMe(!silent);
	    mch_exit(0);
#else
	    if (!silent)
		ole_error("unregister");
	    mch_exit(2);
#endif
	}

	// Ignore an -embedding argument. It is only relevant if the
	// application wants to treat the case when it is started manually
	// differently from the case where it is started via automation (and
	// we don't).
	if (STRICMP(argv[idx] + 1, "embedding") == 0)
	{
#ifdef FEAT_OLE
	    *argc = 1;
#else
	    ole_error("embedding");
	    mch_exit(2);
#endif
	}
    }

#ifdef FEAT_OLE
    {
	int	bDoRestart = FALSE;

	InitOLE(&bDoRestart);
	// automatically exit after registering
	if (bDoRestart)
	    mch_exit(0);
    }
#endif

#ifdef FEAT_NETBEANS_INTG
    {
	// stolen from gui_x11.c
	int arg;

	for (arg = 1; arg < *argc; arg++)
	    if (strncmp("-nb", argv[arg], 3) == 0)
	    {
		netbeansArg = argv[arg];
		mch_memmove(&argv[arg], &argv[arg + 1],
					    (--*argc - arg) * sizeof(char *));
		argv[*argc] = NULL;
		break;	// enough?
	    }
    }
#endif
}

    static void
load_dpi_func(void)
{
    HMODULE	hUser32;

    hUser32 = GetModuleHandle("user32.dll");
    if (hUser32 == NULL)
	goto fail;

    pGetDpiForSystem = (UINT (WINAPI *)(void))GetProcAddress(hUser32, "GetDpiForSystem");
    pGetDpiForWindow = (UINT (WINAPI *)(HWND))GetProcAddress(hUser32, "GetDpiForWindow");
    pGetSystemMetricsForDpi = (int (WINAPI *)(int, UINT))GetProcAddress(hUser32, "GetSystemMetricsForDpi");
    //pGetWindowDpiAwarenessContext = (void*)GetProcAddress(hUser32, "GetWindowDpiAwarenessContext");
    pSetThreadDpiAwarenessContext = (DPI_AWARENESS_CONTEXT (WINAPI *)(DPI_AWARENESS_CONTEXT))GetProcAddress(hUser32, "SetThreadDpiAwarenessContext");
    pGetAwarenessFromDpiAwarenessContext = (DPI_AWARENESS (WINAPI *)(DPI_AWARENESS_CONTEXT))GetProcAddress(hUser32, "GetAwarenessFromDpiAwarenessContext");

    if (pSetThreadDpiAwarenessContext != NULL)
    {
	DPI_AWARENESS_CONTEXT oldctx = pSetThreadDpiAwarenessContext(
		DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
	if (oldctx != NULL)
	{
	    TRACE("DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2 enabled");
	    s_process_dpi_aware = pGetAwarenessFromDpiAwarenessContext(oldctx);
#ifdef DEBUG
	    if (s_process_dpi_aware == DPI_AWARENESS_UNAWARE)
	    {
		TRACE("WARNING: PerMonitorV2 is not enabled in the process level for some reasons. IME window may not shown correctly.");
	    }
#endif
	    return;
	}
    }

fail:
    // Disable PerMonitorV2 APIs.
    pGetDpiForSystem = vimGetDpiForSystem;
    pGetDpiForWindow = NULL;
    pGetSystemMetricsForDpi = stubGetSystemMetricsForDpi;
    pSetThreadDpiAwarenessContext = NULL;
    pGetAwarenessFromDpiAwarenessContext = NULL;
}

/*
 * Initialise the GUI.	Create all the windows, set up all the call-backs
 * etc.
 */
    int
gui_mch_init(void)
{
    const WCHAR szVimWndClassW[] = VIM_CLASSW;
    const WCHAR szTextAreaClassW[] = L"VimTextArea";
    WNDCLASSW wndclassw;

    // Return here if the window was already opened (happens when
    // gui_mch_dialog() is called early).
    if (s_hwnd != NULL)
	goto theend;

    /*
     * Load the tearoff bitmap
     */
#ifdef FEAT_TEAROFF
    s_htearbitmap = LoadBitmap(g_hinst, "IDB_TEAROFF");
#endif

    load_dpi_func();

    s_dpi = pGetDpiForSystem();
    update_scrollbar_size();

#ifdef FEAT_MENU
    gui.menu_height = 0;	// Windows takes care of this
#endif
    gui.border_width = 0;
#ifdef FEAT_TOOLBAR
    gui.toolbar_height = TOOLBAR_BUTTON_HEIGHT + TOOLBAR_BORDER_HEIGHT;
#endif

    s_brush = CreateSolidBrush(GetSysColor(COLOR_BTNFACE));

    // First try using the wide version, so that we can use any title.
    // Otherwise only characters in the active codepage will work.
    if (GetClassInfoW(g_hinst, szVimWndClassW, &wndclassw) == 0)
    {
	wndclassw.style = CS_DBLCLKS;
	wndclassw.lpfnWndProc = _WndProc;
	wndclassw.cbClsExtra = 0;
	wndclassw.cbWndExtra = 0;
	wndclassw.hInstance = g_hinst;
	wndclassw.hIcon = LoadIcon(wndclassw.hInstance, "IDR_VIM");
	wndclassw.hCursor = LoadCursor(NULL, IDC_ARROW);
	wndclassw.hbrBackground = s_brush;
	wndclassw.lpszMenuName = NULL;
	wndclassw.lpszClassName = szVimWndClassW;

	if (RegisterClassW(&wndclassw) == 0)
	    return FAIL;
    }

    if (vim_parent_hwnd != NULL)
    {
#ifdef HAVE_TRY_EXCEPT
	__try
	{
#endif
	    // Open inside the specified parent window.
	    // TODO: last argument should point to a CLIENTCREATESTRUCT
	    // structure.
	    s_hwnd = CreateWindowExW(
		WS_EX_MDICHILD,
		szVimWndClassW, L"Vim MSWindows GUI",
		WS_OVERLAPPEDWINDOW | WS_CHILD
				 | WS_CLIPSIBLINGS | WS_CLIPCHILDREN | 0xC000,
		gui_win_x == -1 ? CW_USEDEFAULT : gui_win_x,
		gui_win_y == -1 ? CW_USEDEFAULT : gui_win_y,
		100,				// Any value will do
		100,				// Any value will do
		vim_parent_hwnd, NULL,
		g_hinst, NULL);
#ifdef HAVE_TRY_EXCEPT
	}
	__except(EXCEPTION_EXECUTE_HANDLER)
	{
	    // NOP
	}
#endif
	if (s_hwnd == NULL)
	{
	    emsg(_(e_unable_to_open_window_inside_mdi_application));
	    mch_exit(2);
	}
    }
    else
    {
	// If the provided windowid is not valid reset it to zero, so that it
	// is ignored and we open our own window.
	if (IsWindow((HWND)win_socket_id) <= 0)
	    win_socket_id = 0;

	// Create a window.  If win_socket_id is not zero without border and
	// titlebar, it will be reparented below.
	s_hwnd = CreateWindowW(
		szVimWndClassW, L"Vim MSWindows GUI",
		(win_socket_id == 0 ? WS_OVERLAPPEDWINDOW : WS_POPUP)
					  | WS_CLIPSIBLINGS | WS_CLIPCHILDREN,
		gui_win_x == -1 ? CW_USEDEFAULT : gui_win_x,
		gui_win_y == -1 ? CW_USEDEFAULT : gui_win_y,
		100,				// Any value will do
		100,				// Any value will do
		NULL, NULL,
		g_hinst, NULL);
	if (s_hwnd != NULL && win_socket_id != 0)
	{
	    SetParent(s_hwnd, (HWND)win_socket_id);
	    ShowWindow(s_hwnd, SW_SHOWMAXIMIZED);
	}
    }

    if (s_hwnd == NULL)
	return FAIL;

    if (pGetDpiForWindow != NULL)
    {
	s_dpi = pGetDpiForWindow(s_hwnd);
	update_scrollbar_size();
	//TRACE("System DPI: %d, DPI: %d", pGetDpiForSystem(), s_dpi);
    }

#if defined(FEAT_MBYTE_IME) && defined(DYNAMIC_IME)
    dyn_imm_load();
#endif

    // Create the text area window
    if (GetClassInfoW(g_hinst, szTextAreaClassW, &wndclassw) == 0)
    {
	wndclassw.style = CS_OWNDC;
	wndclassw.lpfnWndProc = _TextAreaWndProc;
	wndclassw.cbClsExtra = 0;
	wndclassw.cbWndExtra = 0;
	wndclassw.hInstance = g_hinst;
	wndclassw.hIcon = NULL;
	wndclassw.hCursor = LoadCursor(NULL, IDC_ARROW);
	wndclassw.hbrBackground = NULL;
	wndclassw.lpszMenuName = NULL;
	wndclassw.lpszClassName = szTextAreaClassW;

	if (RegisterClassW(&wndclassw) == 0)
	    return FAIL;
    }

    s_textArea = CreateWindowExW(
	0,
	szTextAreaClassW, L"Vim text area",
	WS_CHILD | WS_VISIBLE, 0, 0,
	100,				// Any value will do for now
	100,				// Any value will do for now
	s_hwnd, NULL,
	g_hinst, NULL);

    if (s_textArea == NULL)
	return FAIL;

#ifdef FEAT_LIBCALL
    // Try loading an icon from $RUNTIMEPATH/bitmaps/vim.ico.
    {
	HANDLE	hIcon = NULL;

	if (mch_icon_load(&hIcon) == OK && hIcon != NULL)
	    SendMessage(s_hwnd, WM_SETICON, ICON_SMALL, (LPARAM)hIcon);
    }
#endif

#ifdef FEAT_MENU
    s_menuBar = CreateMenu();
#endif
    s_hdc = GetDC(s_textArea);

    DragAcceptFiles(s_hwnd, TRUE);

    // Do we need to bother with this?
    // m_fMouseAvail = pGetSystemMetricsForDpi(SM_MOUSEPRESENT, s_dpi);

    // Get background/foreground colors from the system
    gui_mch_def_colors();

    // Get the colors from the "Normal" group (set in syntax.c or in a vimrc
    // file)
    set_normal_colors();

    /*
     * Check that none of the colors are the same as the background color.
     * Then store the current values as the defaults.
     */
    gui_check_colors();
    gui.def_norm_pixel = gui.norm_pixel;
    gui.def_back_pixel = gui.back_pixel;

    // Get the colors for the highlight groups (gui_check_colors() might have
    // changed them)
    highlight_gui_started();

    /*
     * Start out by adding the configured border width into the border offset.
     */
    gui.border_offset = gui.border_width;

    /*
     * Set up for Intellimouse processing
     */
    init_mouse_wheel();

    /*
     * compute a couple of metrics used for the dialogs
     */
    get_dialog_font_metrics();
#ifdef FEAT_TOOLBAR
    /*
     * Create the toolbar
     */
    initialise_toolbar();
#endif
#ifdef FEAT_GUI_TABLINE
    /*
     * Create the tabline
     */
    initialise_tabline();
#endif
#ifdef MSWIN_FIND_REPLACE
    /*
     * Initialise the dialog box stuff
     */
    s_findrep_msg = RegisterWindowMessage(FINDMSGSTRING);

    // Initialise the struct
    s_findrep_struct.lStructSize = sizeof(s_findrep_struct);
    s_findrep_struct.lpstrFindWhat = ALLOC_MULT(WCHAR, MSWIN_FR_BUFSIZE);
    s_findrep_struct.lpstrFindWhat[0] = NUL;
    s_findrep_struct.lpstrReplaceWith = ALLOC_MULT(WCHAR, MSWIN_FR_BUFSIZE);
    s_findrep_struct.lpstrReplaceWith[0] = NUL;
    s_findrep_struct.wFindWhatLen = MSWIN_FR_BUFSIZE;
    s_findrep_struct.wReplaceWithLen = MSWIN_FR_BUFSIZE;
#endif

#ifdef FEAT_EVAL
    // set the v:windowid variable
    set_vim_var_nr(VV_WINDOWID, HandleToLong(s_hwnd));
#endif

#ifdef FEAT_RENDER_OPTIONS
    if (p_rop)
	(void)gui_mch_set_rendering_options(p_rop);
#endif

theend:
    // Display any pending error messages
    display_errors();

    return OK;
}

/*
 * Get the size of the screen, taking position on multiple monitors into
 * account (if supported).
 */
    static void
get_work_area(RECT *spi_rect)
{
    HMONITOR	    mon;
    MONITORINFO	    moninfo;

    // work out which monitor the window is on, and get *its* work area
    mon = MonitorFromWindow(s_hwnd, MONITOR_DEFAULTTOPRIMARY);
    if (mon != NULL)
    {
	moninfo.cbSize = sizeof(MONITORINFO);
	if (GetMonitorInfo(mon, &moninfo))
	{
	    *spi_rect = moninfo.rcWork;
	    return;
	}
    }
    // this is the old method...
    SystemParametersInfo(SPI_GETWORKAREA, 0, spi_rect, 0);
}

/*
 * Set the size of the window to the given width and height in pixels.
 */
    void
gui_mch_set_shellsize(
	int width,
	int height,
	int min_width UNUSED,
	int min_height UNUSED,
	int base_width UNUSED,
	int base_height UNUSED,
	int direction)
{
    RECT	workarea_rect;
    RECT	window_rect;
    int		win_width, win_height;

    // Try to keep window completely on screen.
    // Get position of the screen work area.  This is the part that is not
    // used by the taskbar or appbars.
    get_work_area(&workarea_rect);

    // Resizing a maximized window looks very strange, unzoom it first.
    // But don't do it when still starting up, it may have been requested in
    // the shortcut.
    if (IsZoomed(s_hwnd) && starting == 0)
	ShowWindow(s_hwnd, SW_SHOWNORMAL);

    if (s_in_dpichanged)
	// Use the suggested position when in WM_DPICHANGED.
	window_rect = s_suggested_rect;
    else
	// Use current position.
	GetWindowRect(s_hwnd, &window_rect);

    // compute the size of the outside of the window
    win_width = width + (pGetSystemMetricsForDpi(SM_CXFRAME, s_dpi) +
		     pGetSystemMetricsForDpi(SM_CXPADDEDBORDER, s_dpi)) * 2;
    win_height = height + (pGetSystemMetricsForDpi(SM_CYFRAME, s_dpi) +
		       pGetSystemMetricsForDpi(SM_CXPADDEDBORDER, s_dpi)) * 2
			+ pGetSystemMetricsForDpi(SM_CYCAPTION, s_dpi)
			+ gui_mswin_get_menu_height(FALSE);

    // The following should take care of keeping Vim on the same monitor, no
    // matter if the secondary monitor is left or right of the primary
    // monitor.
    window_rect.right = window_rect.left + win_width;
    window_rect.bottom = window_rect.top + win_height;

    // If the window is going off the screen, move it on to the screen.
    // Don't adjust the position when in WM_DPICHANGED.
    if (!s_in_dpichanged)
    {
	if ((direction & RESIZE_HOR)
		&& window_rect.right > workarea_rect.right)
	    OffsetRect(&window_rect,
		    workarea_rect.right - window_rect.right, 0);

	if ((direction & RESIZE_HOR)
		&& window_rect.left < workarea_rect.left)
	    OffsetRect(&window_rect,
		    workarea_rect.left - window_rect.left, 0);

	if ((direction & RESIZE_VERT)
		&& window_rect.bottom > workarea_rect.bottom)
	    OffsetRect(&window_rect,
		    0, workarea_rect.bottom - window_rect.bottom);

	if ((direction & RESIZE_VERT)
		&& window_rect.top < workarea_rect.top)
	    OffsetRect(&window_rect,
		    0, workarea_rect.top - window_rect.top);
    }

    MoveWindow(s_hwnd, window_rect.left, window_rect.top,
						win_width, win_height, TRUE);

    //TRACE("New pos: %d,%d  New size: %d,%d",
    //	    window_rect.left, window_rect.top, win_width, win_height);

    SetActiveWindow(s_hwnd);
    SetFocus(s_hwnd);

    // Menu may wrap differently now
    gui_mswin_get_menu_height(!gui.starting);
}


    void
gui_mch_set_scrollbar_thumb(
    scrollbar_T *sb,
    long	val,
    long	size,
    long	max)
{
    SCROLLINFO	info;

    sb->scroll_shift = 0;
    while (max > 32767)
    {
	max = (max + 1) >> 1;
	val  >>= 1;
	size >>= 1;
	++sb->scroll_shift;
    }

    if (sb->scroll_shift > 0)
	++size;

    info.cbSize = sizeof(info);
    info.fMask = SIF_POS | SIF_RANGE | SIF_PAGE;
    info.nPos = val;
    info.nMin = 0;
    info.nMax = max;
    info.nPage = size;
    SetScrollInfo(sb->id, SB_CTL, &info, TRUE);
}


/*
 * Set the current text font.
 */
    void
gui_mch_set_font(GuiFont font)
{
    gui.currFont = font;
}


/*
 * Set the current text foreground color.
 */
    void
gui_mch_set_fg_color(guicolor_T color)
{
    gui.currFgColor = color;
}

/*
 * Set the current text background color.
 */
    void
gui_mch_set_bg_color(guicolor_T color)
{
    gui.currBgColor = color;
}

/*
 * Set the current text special color.
 */
    void
gui_mch_set_sp_color(guicolor_T color)
{
    gui.currSpColor = color;
}

#ifdef FEAT_MBYTE_IME
/*
 * Multi-byte handling, originally by Sung-Hoon Baek.
 * First static functions (no prototypes generated).
 */
# ifdef _MSC_VER
#  include <ime.h>   // Apparently not needed for Cygwin or MinGW.
# endif
# include <imm.h>

/*
 * handle WM_IME_NOTIFY message
 */
    static LRESULT
_OnImeNotify(HWND hWnd, DWORD dwCommand, DWORD dwData UNUSED)
{
    LRESULT lResult = 0;
    HIMC hImc;

    if (!pImmGetContext || (hImc = pImmGetContext(hWnd)) == (HIMC)0)
	return lResult;
    switch (dwCommand)
    {
	case IMN_SETOPENSTATUS:
	    if (pImmGetOpenStatus(hImc))
	    {
		LOGFONTW lf = norm_logfont;
		if (s_process_dpi_aware == DPI_AWARENESS_UNAWARE)
		    // Work around when PerMonitorV2 is not enabled in the process level.
		    lf.lfHeight = lf.lfHeight * DEFAULT_DPI / s_dpi;
		pImmSetCompositionFontW(hImc, &lf);
		im_set_position(gui.row, gui.col);

		// Disable langmap
		State &= ~MODE_LANGMAP;
		if (State & MODE_INSERT)
		{
# if defined(FEAT_KEYMAP)
		    // Unshown 'keymap' in status lines
		    if (curbuf->b_p_iminsert == B_IMODE_LMAP)
		    {
			// Save cursor position
			int old_row = gui.row;
			int old_col = gui.col;

			// This must be called here before
			// status_redraw_curbuf(), otherwise the mode
			// message may appear in the wrong position.
			showmode();
			status_redraw_curbuf();
			update_screen(0);
			// Restore cursor position
			gui.row = old_row;
			gui.col = old_col;
		    }
# endif
		}
	    }
	    gui_update_cursor(TRUE, FALSE);
	    gui_mch_flush();
	    lResult = 0;
	    break;
    }
    pImmReleaseContext(hWnd, hImc);
    return lResult;
}

    static LRESULT
_OnImeComposition(HWND hwnd, WPARAM dbcs UNUSED, LPARAM param)
{
    char_u	*ret;
    int		len;

    if ((param & GCS_RESULTSTR) == 0) // Composition unfinished.
	return 0;

    ret = GetResultStr(hwnd, GCS_RESULTSTR, &len);
    if (ret != NULL)
    {
	add_to_input_buf_csi(ret, len);
	vim_free(ret);
	return 1;
    }
    return 0;
}

/*
 * void GetResultStr()
 *
 * This handles WM_IME_COMPOSITION with GCS_RESULTSTR flag on.
 * get complete composition string
 */
    static char_u *
GetResultStr(HWND hwnd, int GCS, int *lenp)
{
    HIMC	hIMC;		// Input context handle.
    LONG	ret;
    WCHAR	*buf = NULL;
    char_u	*convbuf = NULL;

    if (!pImmGetContext || (hIMC = pImmGetContext(hwnd)) == (HIMC)0)
	return NULL;

    // Get the length of the composition string.
    ret = pImmGetCompositionStringW(hIMC, GCS, NULL, 0);
    if (ret <= 0)
	return NULL;

    // Allocate the requested buffer plus space for the NUL character.
    buf = alloc(ret + sizeof(WCHAR));
    if (buf == NULL)
	return NULL;

    // Reads in the composition string.
    pImmGetCompositionStringW(hIMC, GCS, buf, ret);
    *lenp = ret / sizeof(WCHAR);

    convbuf = utf16_to_enc(buf, lenp);
    pImmReleaseContext(hwnd, hIMC);
    vim_free(buf);
    return convbuf;
}
#endif

// For global functions we need prototypes.
#if defined(FEAT_MBYTE_IME) || defined(PROTO)

/*
 * set font to IM.
 */
    void
im_set_font(LOGFONTW *lf)
{
    HIMC hImc;

    if (pImmGetContext && (hImc = pImmGetContext(s_hwnd)) != (HIMC)0)
    {
	pImmSetCompositionFontW(hImc, lf);
	pImmReleaseContext(s_hwnd, hImc);
    }
}

/*
 * Notify cursor position to IM.
 */
    void
im_set_position(int row, int col)
{
    HIMC hImc;

    if (pImmGetContext && (hImc = pImmGetContext(s_hwnd)) != (HIMC)0)
    {
	COMPOSITIONFORM	cfs;

	cfs.dwStyle = CFS_POINT;
	cfs.ptCurrentPos.x = FILL_X(col);
	cfs.ptCurrentPos.y = FILL_Y(row);
	MapWindowPoints(s_textArea, s_hwnd, &cfs.ptCurrentPos, 1);
	if (s_process_dpi_aware == DPI_AWARENESS_UNAWARE)
	{
	    // Work around when PerMonitorV2 is not enabled in the process level.
	    cfs.ptCurrentPos.x = cfs.ptCurrentPos.x * DEFAULT_DPI / s_dpi;
	    cfs.ptCurrentPos.y = cfs.ptCurrentPos.y * DEFAULT_DPI / s_dpi;
	}
	pImmSetCompositionWindow(hImc, &cfs);

	pImmReleaseContext(s_hwnd, hImc);
    }
}

/*
 * Set IM status on ("active" is TRUE) or off ("active" is FALSE).
 */
    void
im_set_active(int active)
{
    HIMC	hImc;
    static HIMC	hImcOld = (HIMC)0;

# ifdef VIMDLL
    if (!gui.in_use && !gui.starting)
    {
	mbyte_im_set_active(active);
	return;
    }
# endif

    if (!pImmGetContext)    // if NULL imm32.dll wasn't loaded (yet)
	return;

    if (p_imdisable)
    {
	if (hImcOld == (HIMC)0)
	{
	    hImcOld = pImmGetContext(s_hwnd);
	    if (hImcOld)
		pImmAssociateContext(s_hwnd, (HIMC)0);
	}
	active = FALSE;
    }
    else if (hImcOld != (HIMC)0)
    {
	pImmAssociateContext(s_hwnd, hImcOld);
	hImcOld = (HIMC)0;
    }

    hImc = pImmGetContext(s_hwnd);
    if (!hImc)
	return;

    /*
     * for Korean ime
     */
    HKL hKL = GetKeyboardLayout(0);

    if (LOWORD(hKL) == MAKELANGID(LANG_KOREAN, SUBLANG_KOREAN))
    {
	static DWORD dwConversionSaved = 0, dwSentenceSaved = 0;
	static BOOL bSaved = FALSE;

	if (active)
	{
	    // if we have a saved conversion status, restore it
	    if (bSaved)
		pImmSetConversionStatus(hImc, dwConversionSaved,
			dwSentenceSaved);
	    bSaved = FALSE;
	}
	else
	{
	    // save conversion status and disable korean
	    if (pImmGetConversionStatus(hImc, &dwConversionSaved,
			&dwSentenceSaved))
	    {
		bSaved = TRUE;
		pImmSetConversionStatus(hImc,
			dwConversionSaved & ~(IME_CMODE_NATIVE
			    | IME_CMODE_FULLSHAPE),
			dwSentenceSaved);
	    }
	}
    }

    pImmSetOpenStatus(hImc, active);
    pImmReleaseContext(s_hwnd, hImc);
}

/*
 * Get IM status.  When IM is on, return not 0.  Else return 0.
 */
    int
im_get_status(void)
{
    int		status = 0;
    HIMC	hImc;

# ifdef VIMDLL
    if (!gui.in_use && !gui.starting)
	return mbyte_im_get_status();
# endif

    if (pImmGetContext && (hImc = pImmGetContext(s_hwnd)) != (HIMC)0)
    {
	status = pImmGetOpenStatus(hImc) ? 1 : 0;
	pImmReleaseContext(s_hwnd, hImc);
    }
    return status;
}

#endif // FEAT_MBYTE_IME


/*
 * Convert latin9 text "text[len]" to ucs-2 in "unicodebuf".
 */
    static void
latin9_to_ucs(char_u *text, int len, WCHAR *unicodebuf)
{
    int		c;

    while (--len >= 0)
    {
	c = *text++;
	switch (c)
	{
	    case 0xa4: c = 0x20ac; break;   // euro
	    case 0xa6: c = 0x0160; break;   // S hat
	    case 0xa8: c = 0x0161; break;   // S -hat
	    case 0xb4: c = 0x017d; break;   // Z hat
	    case 0xb8: c = 0x017e; break;   // Z -hat
	    case 0xbc: c = 0x0152; break;   // OE
	    case 0xbd: c = 0x0153; break;   // oe
	    case 0xbe: c = 0x0178; break;   // Y
	}
	*unicodebuf++ = c;
    }
}

#ifdef FEAT_RIGHTLEFT
/*
 * What is this for?  In the case where you are using Win98 or Win2K or later,
 * and you are using a Hebrew font (or Arabic!), Windows does you a favor and
 * reverses the string sent to the TextOut... family.  This sucks, because we
 * go to a lot of effort to do the right thing, and there doesn't seem to be a
 * way to tell Windblows not to do this!
 *
 * The short of it is that this 'RevOut' only gets called if you are running
 * one of the new, "improved" MS OSes, and only if you are running in
 * 'rightleft' mode.  It makes display take *slightly* longer, but not
 * noticeably so.
 */
    static void
RevOut( HDC hdc,
	int col,
	int row,
	UINT foptions,
	CONST RECT *pcliprect,
	LPCTSTR text,
	UINT len,
	CONST INT *padding)
{
    int		ix;

    for (ix = 0; ix < (int)len; ++ix)
	ExtTextOut(hdc, col + TEXT_X(ix), row, foptions,
					pcliprect, text + ix, 1, padding);
}
#endif

    static void
draw_line(
    int		x1,
    int		y1,
    int		x2,
    int		y2,
    COLORREF	color)
{
#if defined(FEAT_DIRECTX)
    if (IS_ENABLE_DIRECTX())
	DWriteContext_DrawLine(s_dwc, x1, y1, x2, y2, color);
    else
#endif
    {
	HPEN	hpen = CreatePen(PS_SOLID, 1, color);
	HPEN	old_pen = SelectObject(s_hdc, hpen);
	MoveToEx(s_hdc, x1, y1, NULL);
	// Note: LineTo() excludes the last pixel in the line.
	LineTo(s_hdc, x2, y2);
	DeleteObject(SelectObject(s_hdc, old_pen));
    }
}

    static void
set_pixel(
    int		x,
    int		y,
    COLORREF	color)
{
#if defined(FEAT_DIRECTX)
    if (IS_ENABLE_DIRECTX())
	DWriteContext_SetPixel(s_dwc, x, y, color);
    else
#endif
	SetPixel(s_hdc, x, y, color);
}

    static void
fill_rect(
    const RECT	*rcp,
    HBRUSH	hbr,
    COLORREF	color)
{
#if defined(FEAT_DIRECTX)
    if (IS_ENABLE_DIRECTX())
	DWriteContext_FillRect(s_dwc, rcp, color);
    else
#endif
    {
	HBRUSH	hbr2;

	if (hbr == NULL)
	    hbr2 = CreateSolidBrush(color);
	else
	    hbr2 = hbr;
	FillRect(s_hdc, rcp, hbr2);
	if (hbr == NULL)
	    DeleteBrush(hbr2);
    }
}

    void
gui_mch_draw_string(
    int		row,
    int		col,
    char_u	*text,
    int		len,
    int		flags)
{
    static int	*padding = NULL;
    static int	pad_size = 0;
    const RECT	*pcliprect = NULL;
    UINT	foptions = 0;
    static WCHAR *unicodebuf = NULL;
    static int   *unicodepdy = NULL;
    static int	unibuflen = 0;
    int		n = 0;
    int		y;

    /*
     * Italic and bold text seems to have an extra row of pixels at the bottom
     * (below where the bottom of the character should be).  If we draw the
     * characters with a solid background, the top row of pixels in the
     * character below will be overwritten.  We can fix this by filling in the
     * background ourselves, to the correct character proportions, and then
     * writing the character in transparent mode.  Still have a problem when
     * the character is "_", which gets written on to the character below.
     * New fix: set gui.char_ascent to -1.  This shifts all characters up one
     * pixel in their slots, which fixes the problem with the bottom row of
     * pixels.	We still need this code because otherwise the top row of pixels
     * becomes a problem. - webb.
     */
    static HBRUSH	hbr_cache[2] = {NULL, NULL};
    static guicolor_T	brush_color[2] = {INVALCOLOR, INVALCOLOR};
    static int		brush_lru = 0;
    HBRUSH		hbr;
    RECT		rc;

    if (!(flags & DRAW_TRANSP))
    {
	/*
	 * Clear background first.
	 * Note: FillRect() excludes right and bottom of rectangle.
	 */
	rc.left = FILL_X(col);
	rc.top = FILL_Y(row);
	if (has_mbyte)
	{
	    // Compute the length in display cells.
	    rc.right = FILL_X(col + mb_string2cells(text, len));
	}
	else
	    rc.right = FILL_X(col + len);
	rc.bottom = FILL_Y(row + 1);

	// Cache the created brush, that saves a lot of time.  We need two:
	// one for cursor background and one for the normal background.
	if (gui.currBgColor == brush_color[0])
	{
	    hbr = hbr_cache[0];
	    brush_lru = 1;
	}
	else if (gui.currBgColor == brush_color[1])
	{
	    hbr = hbr_cache[1];
	    brush_lru = 0;
	}
	else
	{
	    if (hbr_cache[brush_lru] != NULL)
		DeleteBrush(hbr_cache[brush_lru]);
	    hbr_cache[brush_lru] = CreateSolidBrush(gui.currBgColor);
	    brush_color[brush_lru] = gui.currBgColor;
	    hbr = hbr_cache[brush_lru];
	    brush_lru = !brush_lru;
	}

	fill_rect(&rc, hbr, gui.currBgColor);

	SetBkMode(s_hdc, TRANSPARENT);

	/*
	 * When drawing block cursor, prevent inverted character spilling
	 * over character cell (can happen with bold/italic)
	 */
	if (flags & DRAW_CURSOR)
	{
	    pcliprect = &rc;
	    foptions = ETO_CLIPPED;
	}
    }
    SetTextColor(s_hdc, gui.currFgColor);
    SelectFont(s_hdc, gui.currFont);

#ifdef FEAT_DIRECTX
    if (IS_ENABLE_DIRECTX())
	DWriteContext_SetFont(s_dwc, (HFONT)gui.currFont);
#endif

    if (pad_size != Columns || padding == NULL || padding[0] != gui.char_width)
    {
	int	i;

	vim_free(padding);
	pad_size = Columns;

	// Don't give an out-of-memory message here, it would call us
	// recursively.
	padding = LALLOC_MULT(int, pad_size);
	if (padding != NULL)
	    for (i = 0; i < pad_size; i++)
		padding[i] = gui.char_width;
    }

    /*
     * We have to provide the padding argument because italic and bold versions
     * of fixed-width fonts are often one pixel or so wider than their normal
     * versions.
     * No check for DRAW_BOLD, Windows will have done it already.
     */

    // Check if there are any UTF-8 characters.  If not, use normal text
    // output to speed up output.
    if (enc_utf8)
	for (n = 0; n < len; ++n)
	    if (text[n] >= 0x80)
		break;

#if defined(FEAT_DIRECTX)
    // Quick hack to enable DirectWrite.  To use DirectWrite (antialias), it is
    // required that unicode drawing routine, currently.  So this forces it
    // enabled.
    if (IS_ENABLE_DIRECTX())
	n = 0; // Keep n < len, to enter block for unicode.
#endif

    // Check if the Unicode buffer exists and is big enough.  Create it
    // with the same length as the multi-byte string, the number of wide
    // characters is always equal or smaller.
    if ((enc_utf8
		|| (enc_codepage > 0 && (int)GetACP() != enc_codepage)
		|| enc_latin9)
	    && (unicodebuf == NULL || len > unibuflen))
    {
	vim_free(unicodebuf);
	unicodebuf = LALLOC_MULT(WCHAR, len);

	vim_free(unicodepdy);
	unicodepdy = LALLOC_MULT(int, len);

	unibuflen = len;
    }

    if (enc_utf8 && n < len && unicodebuf != NULL)
    {
	// Output UTF-8 characters.  Composing characters should be
	// handled here.
	int		i;
	int		wlen;	// string length in words
	int		cells;	// cell width of string up to composing char
	int		cw;	// width of current cell
	int		c;

	wlen = 0;
	cells = 0;
	for (i = 0; i < len; )
	{
	    c = utf_ptr2char(text + i);
	    if (c >= 0x10000)
	    {
		// Turn into UTF-16 encoding.
		unicodebuf[wlen++] = ((c - 0x10000) >> 10) + 0xD800;
		unicodebuf[wlen++] = ((c - 0x10000) & 0x3ff) + 0xDC00;
	    }
	    else
	    {
		unicodebuf[wlen++] = c;
	    }

	    if (utf_iscomposing(c))
		cw = 0;
	    else
	    {
		cw = utf_char2cells(c);
		if (cw > 2)		// don't use 4 for unprintable char
		    cw = 1;
	    }

	    if (unicodepdy != NULL)
	    {
		// Use unicodepdy to make characters fit as we expect, even
		// when the font uses different widths (e.g., bold character
		// is wider).
		if (c >= 0x10000)
		{
		    unicodepdy[wlen - 2] = cw * gui.char_width;
		    unicodepdy[wlen - 1] = 0;
		}
		else
		    unicodepdy[wlen - 1] = cw * gui.char_width;
	    }
	    cells += cw;
	    i += utf_ptr2len_len(text + i, len - i);
	}
#if defined(FEAT_DIRECTX)
	if (IS_ENABLE_DIRECTX())
	{
	    // Add one to "cells" for italics.
	    DWriteContext_DrawText(s_dwc, unicodebuf, wlen,
		    TEXT_X(col), TEXT_Y(row),
		    FILL_X(cells + 1), FILL_Y(1) - p_linespace,
		    gui.char_width, gui.currFgColor,
		    foptions, pcliprect, unicodepdy);
	}
	else
#endif
	    ExtTextOutW(s_hdc, TEXT_X(col), TEXT_Y(row),
		    foptions, pcliprect, unicodebuf, wlen, unicodepdy);
	len = cells;	// used for underlining
    }
    else if ((enc_codepage > 0 && (int)GetACP() != enc_codepage) || enc_latin9)
    {
	// If we want to display codepage data, and the current CP is not the
	// ANSI one, we need to go via Unicode.
	if (unicodebuf != NULL)
	{
	    if (enc_latin9)
		latin9_to_ucs(text, len, unicodebuf);
	    else
		len = MultiByteToWideChar(enc_codepage,
			MB_PRECOMPOSED,
			(char *)text, len,
			(LPWSTR)unicodebuf, unibuflen);
	    if (len != 0)
	    {
		// Use unicodepdy to make characters fit as we expect, even
		// when the font uses different widths (e.g., bold character
		// is wider).
		if (unicodepdy != NULL)
		{
		    int i;
		    int cw;

		    for (i = 0; i < len; ++i)
		    {
			cw = utf_char2cells(unicodebuf[i]);
			if (cw > 2)
			    cw = 1;
			unicodepdy[i] = cw * gui.char_width;
		    }
		}
		ExtTextOutW(s_hdc, TEXT_X(col), TEXT_Y(row),
			    foptions, pcliprect, unicodebuf, len, unicodepdy);
	    }
	}
    }
    else
    {
#ifdef FEAT_RIGHTLEFT
	// Windows will mess up RL text, so we have to draw it character by
	// character.  Only do this if RL is on, since it's slow.
	if (curwin->w_p_rl)
	    RevOut(s_hdc, TEXT_X(col), TEXT_Y(row),
			 foptions, pcliprect, (char *)text, len, padding);
	else
#endif
	    ExtTextOut(s_hdc, TEXT_X(col), TEXT_Y(row),
			 foptions, pcliprect, (char *)text, len, padding);
    }

    // Underline
    if (flags & DRAW_UNDERL)
    {
	// When p_linespace is 0, overwrite the bottom row of pixels.
	// Otherwise put the line just below the character.
	y = FILL_Y(row + 1) - 1;
	if (p_linespace > 1)
	    y -= p_linespace - 1;
	draw_line(FILL_X(col), y, FILL_X(col + len), y, gui.currFgColor);
    }

    // Strikethrough
    if (flags & DRAW_STRIKE)
    {
	y = FILL_Y(row + 1) - gui.char_height/2;
	draw_line(FILL_X(col), y, FILL_X(col + len), y, gui.currSpColor);
    }

    // Undercurl
    if (flags & DRAW_UNDERC)
    {
	int			x;
	int			offset;
	static const int	val[8] = {1, 0, 0, 0, 1, 2, 2, 2 };

	y = FILL_Y(row + 1) - 1;
	for (x = FILL_X(col); x < FILL_X(col + len); ++x)
	{
	    offset = val[x % 8];
	    set_pixel(x, y - offset, gui.currSpColor);
	}
    }
}


/*
 * Output routines.
 */

/*
 * Flush any output to the screen
 */
    void
gui_mch_flush(void)
{
#if defined(FEAT_DIRECTX)
    if (IS_ENABLE_DIRECTX())
	DWriteContext_Flush(s_dwc);
#endif

    GdiFlush();
}

    static void
clear_rect(RECT *rcp)
{
    fill_rect(rcp, NULL, gui.back_pixel);
}


    void
gui_mch_get_screen_dimensions(int *screen_w, int *screen_h)
{
    RECT	workarea_rect;

    get_work_area(&workarea_rect);

    *screen_w = workarea_rect.right - workarea_rect.left
		- (pGetSystemMetricsForDpi(SM_CXFRAME, s_dpi) +
		   pGetSystemMetricsForDpi(SM_CXPADDEDBORDER, s_dpi)) * 2;

    // FIXME: dirty trick: Because the gui_get_base_height() doesn't include
    // the menubar for MSwin, we subtract it from the screen height, so that
    // the window size can be made to fit on the screen.
    *screen_h = workarea_rect.bottom - workarea_rect.top
		- (pGetSystemMetricsForDpi(SM_CYFRAME, s_dpi) +
		   pGetSystemMetricsForDpi(SM_CXPADDEDBORDER, s_dpi)) * 2
		- pGetSystemMetricsForDpi(SM_CYCAPTION, s_dpi)
		- gui_mswin_get_menu_height(FALSE);
}


#if defined(FEAT_MENU) || defined(PROTO)
/*
 * Add a sub menu to the menu bar.
 */
    void
gui_mch_add_menu(
    vimmenu_T	*menu,
    int		pos)
{
    vimmenu_T	*parent = menu->parent;

    menu->submenu_id = CreatePopupMenu();
    menu->id = s_menu_id++;

    if (menu_is_menubar(menu->name))
    {
	WCHAR	*wn;
	MENUITEMINFOW	infow;

	wn = enc_to_utf16(menu->name, NULL);
	if (wn == NULL)
	    return;

	infow.cbSize = sizeof(infow);
	infow.fMask = MIIM_DATA | MIIM_TYPE | MIIM_ID
	    | MIIM_SUBMENU;
	infow.dwItemData = (long_u)menu;
	infow.wID = menu->id;
	infow.fType = MFT_STRING;
	infow.dwTypeData = wn;
	infow.cch = (UINT)wcslen(wn);
	infow.hSubMenu = menu->submenu_id;
	InsertMenuItemW((parent == NULL)
		? s_menuBar : parent->submenu_id,
		(UINT)pos, TRUE, &infow);
	vim_free(wn);
    }

    // Fix window size if menu may have wrapped
    if (parent == NULL)
	gui_mswin_get_menu_height(!gui.starting);
# ifdef FEAT_TEAROFF
    else if (IsWindow(parent->tearoff_handle))
	rebuild_tearoff(parent);
# endif
}

    void
gui_mch_show_popupmenu(vimmenu_T *menu)
{
    POINT mp;

    (void)GetCursorPos(&mp);
    gui_mch_show_popupmenu_at(menu, (int)mp.x, (int)mp.y);
}

    void
gui_make_popup(char_u *path_name, int mouse_pos)
{
    vimmenu_T	*menu = gui_find_menu(path_name);

    if (menu == NULL)
	return;

    POINT	p;

    // Find the position of the current cursor
    GetDCOrgEx(s_hdc, &p);
    if (mouse_pos)
    {
	int	mx, my;

	gui_mch_getmouse(&mx, &my);
	p.x += mx;
	p.y += my;
    }
    else if (curwin != NULL)
    {
	p.x += TEXT_X(curwin->w_wincol + curwin->w_wcol + 1);
	p.y += TEXT_Y(W_WINROW(curwin) + curwin->w_wrow + 1);
    }
    msg_scroll = FALSE;
    gui_mch_show_popupmenu_at(menu, (int)p.x, (int)p.y);
}

# if defined(FEAT_TEAROFF) || defined(PROTO)
/*
 * Given a menu descriptor, e.g. "File.New", find it in the menu hierarchy and
 * create it as a pseudo-"tearoff menu".
 */
    void
gui_make_tearoff(char_u *path_name)
{
    vimmenu_T	*menu = gui_find_menu(path_name);

    // Found the menu, so tear it off.
    if (menu != NULL)
	gui_mch_tearoff(menu->dname, menu, 0xffffL, 0xffffL);
}
# endif

/*
 * Add a menu item to a menu
 */
    void
gui_mch_add_menu_item(
    vimmenu_T	*menu,
    int		idx)
{
    vimmenu_T	*parent = menu->parent;

    menu->id = s_menu_id++;
    menu->submenu_id = NULL;

# ifdef FEAT_TEAROFF
    if (STRNCMP(menu->name, TEAR_STRING, TEAR_LEN) == 0)
    {
	InsertMenu(parent->submenu_id, (UINT)idx, MF_BITMAP|MF_BYPOSITION,
		(UINT)menu->id, (LPCTSTR) s_htearbitmap);
    }
    else
# endif
# ifdef FEAT_TOOLBAR
    if (menu_is_toolbar(parent->name))
    {
	TBBUTTON newtb;

	CLEAR_FIELD(newtb);
	if (menu_is_separator(menu->name))
	{
	    newtb.iBitmap = 0;
	    newtb.fsStyle = TBSTYLE_SEP;
	}
	else
	{
	    newtb.iBitmap = get_toolbar_bitmap(menu);
	    newtb.fsStyle = TBSTYLE_BUTTON;
	}
	newtb.idCommand = menu->id;
	newtb.fsState = TBSTATE_ENABLED;
	newtb.iString = 0;
	SendMessage(s_toolbarhwnd, TB_INSERTBUTTON, (WPARAM)idx,
							     (LPARAM)&newtb);
	menu->submenu_id = (HMENU)-1;
    }
    else
# endif
    {
	WCHAR	*wn;

	wn = enc_to_utf16(menu->name, NULL);
	if (wn != NULL)
	{
	    InsertMenuW(parent->submenu_id, (UINT)idx,
		    (menu_is_separator(menu->name)
		     ? MF_SEPARATOR : MF_STRING) | MF_BYPOSITION,
		    (UINT)menu->id, wn);
	    vim_free(wn);
	}
# ifdef FEAT_TEAROFF
	if (IsWindow(parent->tearoff_handle))
	    rebuild_tearoff(parent);
# endif
    }
}

/*
 * Destroy the machine specific menu widget.
 */
    void
gui_mch_destroy_menu(vimmenu_T *menu)
{
# ifdef FEAT_TOOLBAR
    /*
     * is this a toolbar button?
     */
    if (menu->submenu_id == (HMENU)-1)
    {
	int iButton;

	iButton = (int)SendMessage(s_toolbarhwnd, TB_COMMANDTOINDEX,
							 (WPARAM)menu->id, 0);
	SendMessage(s_toolbarhwnd, TB_DELETEBUTTON, (WPARAM)iButton, 0);
    }
    else
# endif
    {
	if (menu->parent != NULL
		&& menu_is_popup(menu->parent->dname)
		&& menu->parent->submenu_id != NULL)
	    RemoveMenu(menu->parent->submenu_id, menu->id, MF_BYCOMMAND);
	else
	    RemoveMenu(s_menuBar, menu->id, MF_BYCOMMAND);
	if (menu->submenu_id != NULL)
	    DestroyMenu(menu->submenu_id);
# ifdef FEAT_TEAROFF
	if (IsWindow(menu->tearoff_handle))
	    DestroyWindow(menu->tearoff_handle);
	if (menu->parent != NULL
		&& menu->parent->children != NULL
		&& IsWindow(menu->parent->tearoff_handle))
	{
	    // This menu must not show up when rebuilding the tearoff window.
	    menu->modes = 0;
	    rebuild_tearoff(menu->parent);
	}
# endif
    }
}

# ifdef FEAT_TEAROFF
    static void
rebuild_tearoff(vimmenu_T *menu)
{
    //hackish
    char_u	tbuf[128];
    RECT	trect;
    RECT	rct;
    RECT	roct;
    int		x, y;

    HWND thwnd = menu->tearoff_handle;

    GetWindowText(thwnd, (LPSTR)tbuf, 127);
    if (GetWindowRect(thwnd, &trect)
	    && GetWindowRect(s_hwnd, &rct)
	    && GetClientRect(s_hwnd, &roct))
    {
	x = trect.left - rct.left;
	y = (trect.top -  rct.bottom  + roct.bottom);
    }
    else
    {
	x = y = 0xffffL;
    }
    DestroyWindow(thwnd);
    if (menu->children != NULL)
    {
	gui_mch_tearoff(tbuf, menu, x, y);
	if (IsWindow(menu->tearoff_handle))
	    (void) SetWindowPos(menu->tearoff_handle,
				NULL,
				(int)trect.left,
				(int)trect.top,
				0, 0,
				SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE);
    }
}
# endif // FEAT_TEAROFF

/*
 * Make a menu either grey or not grey.
 */
    void
gui_mch_menu_grey(
    vimmenu_T	*menu,
    int	    grey)
{
# ifdef FEAT_TOOLBAR
    /*
     * is this a toolbar button?
     */
    if (menu->submenu_id == (HMENU)-1)
    {
	SendMessage(s_toolbarhwnd, TB_ENABLEBUTTON,
	    (WPARAM)menu->id, (LPARAM) MAKELONG((grey ? FALSE : TRUE), 0));
    }
    else
# endif
    (void)EnableMenuItem(menu->parent ? menu->parent->submenu_id : s_menuBar,
		    menu->id, MF_BYCOMMAND | (grey ? MF_GRAYED : MF_ENABLED));

# ifdef FEAT_TEAROFF
    if ((menu->parent != NULL) && (IsWindow(menu->parent->tearoff_handle)))
    {
	WORD menuID;
	HWND menuHandle;

	/*
	 * A tearoff button has changed state.
	 */
	if (menu->children == NULL)
	    menuID = (WORD)(menu->id);
	else
	    menuID = (WORD)((long_u)(menu->submenu_id) | (DWORD)0x8000);
	menuHandle = GetDlgItem(menu->parent->tearoff_handle, menuID);
	if (menuHandle)
	    EnableWindow(menuHandle, !grey);

    }
# endif
}

#endif // FEAT_MENU


// define some macros used to make the dialogue creation more readable

#define add_word(x)		*p++ = (x)
#define add_long(x)		dwp = (DWORD *)p; *dwp++ = (x); p = (WORD *)dwp

#if defined(FEAT_GUI_DIALOG) || defined(PROTO)
/*
 * stuff for dialogs
 */

/*
 * The callback routine used by all the dialogs.  Very simple.  First,
 * acknowledges the INITDIALOG message so that Windows knows to do standard
 * dialog stuff (Return = default, Esc = cancel....) Second, if a button is
 * pressed, return that button's ID - IDCANCEL (2), which is the button's
 * number.
 */
    static LRESULT CALLBACK
dialog_callback(
    HWND hwnd,
    UINT message,
    WPARAM wParam,
    LPARAM lParam UNUSED)
{
    if (message == WM_INITDIALOG)
    {
	CenterWindow(hwnd, GetWindow(hwnd, GW_OWNER));
	// Set focus to the dialog.  Set the default button, if specified.
	(void)SetFocus(hwnd);
	if (dialog_default_button > IDCANCEL)
	    (void)SetFocus(GetDlgItem(hwnd, dialog_default_button));
	else
	    // We don't have a default, set focus on another element of the
	    // dialog window, probably the icon
	    (void)SetFocus(GetDlgItem(hwnd, DLG_NONBUTTON_CONTROL));
	return FALSE;
    }

    if (message == WM_COMMAND)
    {
	int	button = LOWORD(wParam);

	// Don't end the dialog if something was selected that was
	// not a button.
	if (button >= DLG_NONBUTTON_CONTROL)
	    return TRUE;

	// If the edit box exists, copy the string.
	if (s_textfield != NULL)
	{
	    WCHAR  *wp = ALLOC_MULT(WCHAR, IOSIZE);
	    char_u *p;

	    GetDlgItemTextW(hwnd, DLG_NONBUTTON_CONTROL + 2, wp, IOSIZE);
	    p = utf16_to_enc(wp, NULL);
	    vim_strncpy(s_textfield, p, IOSIZE);
	    vim_free(p);
	    vim_free(wp);
	}

	/*
	 * Need to check for IDOK because if the user just hits Return to
	 * accept the default value, some reason this is what we get.
	 */
	if (button == IDOK)
	{
	    if (dialog_default_button > IDCANCEL)
		EndDialog(hwnd, dialog_default_button);
	}
	else
	    EndDialog(hwnd, button - IDCANCEL);
	return TRUE;
    }

    if ((message == WM_SYSCOMMAND) && (wParam == SC_CLOSE))
    {
	EndDialog(hwnd, 0);
	return TRUE;
    }
    return FALSE;
}

/*
 * Create a dialog dynamically from the parameter strings.
 * type		= type of dialog (question, alert, etc.)
 * title	= dialog title. may be NULL for default title.
 * message	= text to display. Dialog sizes to accommodate it.
 * buttons	= '\n' separated list of button captions, default first.
 * dfltbutton	= number of default button.
 *
 * This routine returns 1 if the first button is pressed,
 *			2 for the second, etc.
 *
 *			0 indicates Esc was pressed.
 *			-1 for unexpected error
 *
 * If stubbing out this fn, return 1.
 */

static const char *dlg_icons[] = // must match names in resource file
{
    "IDR_VIM",
    "IDR_VIM_ERROR",
    "IDR_VIM_ALERT",
    "IDR_VIM_INFO",
    "IDR_VIM_QUESTION"
};

    int
gui_mch_dialog(
    int		 type,
    char_u	*title,
    char_u	*message,
    char_u	*buttons,
    int		 dfltbutton,
    char_u	*textfield,
    int		ex_cmd UNUSED)
{
    WORD	*p, *pdlgtemplate, *pnumitems;
    DWORD	*dwp;
    int		numButtons;
    int		*buttonWidths, *buttonPositions;
    int		buttonYpos;
    int		nchar, i;
    DWORD	lStyle;
    int		dlgwidth = 0;
    int		dlgheight;
    int		editboxheight;
    int		horizWidth = 0;
    int		msgheight;
    char_u	*pstart;
    char_u	*pend;
    char_u	*last_white;
    char_u	*tbuffer;
    RECT	rect;
    HWND	hwnd;
    HDC		hdc;
    HFONT	font, oldFont;
    TEXTMETRIC	fontInfo;
    int		fontHeight;
    int		textWidth, minButtonWidth, messageWidth;
    int		maxDialogWidth;
    int		maxDialogHeight;
    int		scroll_flag = 0;
    int		vertical;
    int		dlgPaddingX;
    int		dlgPaddingY;
# ifdef USE_SYSMENU_FONT
    LOGFONTW	lfSysmenu;
    int		use_lfSysmenu = FALSE;
# endif
    garray_T	ga;
    int		l;
    int		dlg_icon_width;
    int		dlg_icon_height;
    int		dpi;

# ifndef NO_CONSOLE
    // Don't output anything in silent mode ("ex -s")
#  ifdef VIMDLL
    if (!(gui.in_use || gui.starting))
#  endif
	if (silent_mode)
	    return dfltbutton;   // return default option
# endif

    if (s_hwnd == NULL)
    {
	load_dpi_func();
	s_dpi = dpi = pGetDpiForSystem();
	get_dialog_font_metrics();
    }
    else
	dpi = pGetDpiForSystem();

    if ((type < 0) || (type > VIM_LAST_TYPE))
	type = 0;

    // allocate some memory for dialog template
    // TODO should compute this really
    pdlgtemplate = p = (PWORD)LocalAlloc(LPTR,
					DLG_ALLOC_SIZE + STRLEN(message) * 2);

    if (p == NULL)
	return -1;

    /*
     * make a copy of 'buttons' to fiddle with it.  compiler grizzles because
     * vim_strsave() doesn't take a const arg (why not?), so cast away the
     * const.
     */
    tbuffer = vim_strsave(buttons);
    if (tbuffer == NULL)
	return -1;

    --dfltbutton;   // Change from one-based to zero-based

    // Count buttons
    numButtons = 1;
    for (i = 0; tbuffer[i] != '\0'; i++)
    {
	if (tbuffer[i] == DLG_BUTTON_SEP)
	    numButtons++;
    }
    if (dfltbutton >= numButtons)
	dfltbutton = -1;

    // Allocate array to hold the width of each button
    buttonWidths = ALLOC_MULT(int, numButtons);
    if (buttonWidths == NULL)
	return -1;

    // Allocate array to hold the X position of each button
    buttonPositions = ALLOC_MULT(int, numButtons);
    if (buttonPositions == NULL)
	return -1;

    /*
     * Calculate how big the dialog must be.
     */
    hwnd = GetDesktopWindow();
    hdc = GetWindowDC(hwnd);
# ifdef USE_SYSMENU_FONT
    if (gui_w32_get_menu_font(&lfSysmenu) == OK)
    {
	font = CreateFontIndirectW(&lfSysmenu);
	use_lfSysmenu = TRUE;
    }
    else
# endif
	font = CreateFont(-DLG_FONT_POINT_SIZE, 0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, VARIABLE_PITCH, DLG_FONT_NAME);

    oldFont = SelectFont(hdc, font);
    dlgPaddingX = DLG_PADDING_X;
    dlgPaddingY = DLG_PADDING_Y;

    GetTextMetrics(hdc, &fontInfo);
    fontHeight = fontInfo.tmHeight;

    // Minimum width for horizontal button
    minButtonWidth = GetTextWidth(hdc, (char_u *)"Cancel", 6);

    // Maximum width of a dialog, if possible
    if (s_hwnd == NULL)
    {
	RECT	workarea_rect;

	// We don't have a window, use the desktop area.
	get_work_area(&workarea_rect);
	maxDialogWidth = workarea_rect.right - workarea_rect.left - 100;
	if (maxDialogWidth > adjust_by_system_dpi(600))
	    maxDialogWidth = adjust_by_system_dpi(600);
	// Leave some room for the taskbar.
	maxDialogHeight = workarea_rect.bottom - workarea_rect.top - 150;
    }
    else
    {
	// Use our own window for the size, unless it's very small.
	GetWindowRect(s_hwnd, &rect);
	maxDialogWidth = rect.right - rect.left
		       - (pGetSystemMetricsForDpi(SM_CXFRAME, dpi) +
			  pGetSystemMetricsForDpi(SM_CXPADDEDBORDER, dpi)) * 2;
	if (maxDialogWidth < adjust_by_system_dpi(DLG_MIN_MAX_WIDTH))
	    maxDialogWidth = adjust_by_system_dpi(DLG_MIN_MAX_WIDTH);

	maxDialogHeight = rect.bottom - rect.top
		       - (pGetSystemMetricsForDpi(SM_CYFRAME, dpi) +
			  pGetSystemMetricsForDpi(SM_CXPADDEDBORDER, dpi)) * 4
		       - pGetSystemMetricsForDpi(SM_CYCAPTION, dpi);
	if (maxDialogHeight < adjust_by_system_dpi(DLG_MIN_MAX_HEIGHT))
	    maxDialogHeight = adjust_by_system_dpi(DLG_MIN_MAX_HEIGHT);
    }

    // Set dlgwidth to width of message.
    // Copy the message into "ga", changing NL to CR-NL and inserting line
    // breaks where needed.
    pstart = message;
    messageWidth = 0;
    msgheight = 0;
    ga_init2(&ga, sizeof(char), 500);
    do
    {
	msgheight += fontHeight;    // at least one line

	// Need to figure out where to break the string.  The system does it
	// at a word boundary, which would mean we can't compute the number of
	// wrapped lines.
	textWidth = 0;
	last_white = NULL;
	for (pend = pstart; *pend != NUL && *pend != '\n'; )
	{
	    l = (*mb_ptr2len)(pend);
	    if (l == 1 && VIM_ISWHITE(*pend)
					&& textWidth > maxDialogWidth * 3 / 4)
		last_white = pend;
	    textWidth += GetTextWidthEnc(hdc, pend, l);
	    if (textWidth >= maxDialogWidth)
	    {
		// Line will wrap.
		messageWidth = maxDialogWidth;
		msgheight += fontHeight;
		textWidth = 0;

		if (last_white != NULL)
		{
		    // break the line just after a space
		    if (pend > last_white)
			ga.ga_len -= (int)(pend - (last_white + 1));
		    pend = last_white + 1;
		    last_white = NULL;
		}
		ga_append(&ga, '\r');
		ga_append(&ga, '\n');
		continue;
	    }

	    while (--l >= 0)
		ga_append(&ga, *pend++);
	}
	if (textWidth > messageWidth)
	    messageWidth = textWidth;

	ga_append(&ga, '\r');
	ga_append(&ga, '\n');
	pstart = pend + 1;
    } while (*pend != NUL);

    if (ga.ga_data != NULL)
	message = ga.ga_data;

    messageWidth += 10;		// roundoff space

    dlg_icon_width = adjust_by_system_dpi(DLG_ICON_WIDTH);
    dlg_icon_height = adjust_by_system_dpi(DLG_ICON_HEIGHT);

    // Add width of icon to dlgwidth, and some space
    dlgwidth = messageWidth + dlg_icon_width + 3 * dlgPaddingX
			     + pGetSystemMetricsForDpi(SM_CXVSCROLL, dpi);

    if (msgheight < dlg_icon_height)
	msgheight = dlg_icon_height;

    /*
     * Check button names.  A long one will make the dialog wider.
     * When called early (-register error message) p_go isn't initialized.
     */
    vertical = (p_go != NULL && vim_strchr(p_go, GO_VERTICAL) != NULL);
    if (!vertical)
    {
	// Place buttons horizontally if they fit.
	horizWidth = dlgPaddingX;
	pstart = tbuffer;
	i = 0;
	do
	{
	    pend = vim_strchr(pstart, DLG_BUTTON_SEP);
	    if (pend == NULL)
		pend = pstart + STRLEN(pstart);	// Last button name.
	    textWidth = GetTextWidthEnc(hdc, pstart, (int)(pend - pstart));
	    if (textWidth < minButtonWidth)
		textWidth = minButtonWidth;
	    textWidth += dlgPaddingX;	    // Padding within button
	    buttonWidths[i] = textWidth;
	    buttonPositions[i++] = horizWidth;
	    horizWidth += textWidth + dlgPaddingX; // Pad between buttons
	    pstart = pend + 1;
	} while (*pend != NUL);

	if (horizWidth > maxDialogWidth)
	    vertical = TRUE;	// Too wide to fit on the screen.
	else if (horizWidth > dlgwidth)
	    dlgwidth = horizWidth;
    }

    if (vertical)
    {
	// Stack buttons vertically.
	pstart = tbuffer;
	do
	{
	    pend = vim_strchr(pstart, DLG_BUTTON_SEP);
	    if (pend == NULL)
		pend = pstart + STRLEN(pstart);	// Last button name.
	    textWidth = GetTextWidthEnc(hdc, pstart, (int)(pend - pstart));
	    textWidth += dlgPaddingX;		// Padding within button
	    textWidth += DLG_VERT_PADDING_X * 2; // Padding around button
	    if (textWidth > dlgwidth)
		dlgwidth = textWidth;
	    pstart = pend + 1;
	} while (*pend != NUL);
    }

    if (dlgwidth < DLG_MIN_WIDTH)
	dlgwidth = DLG_MIN_WIDTH;	// Don't allow a really thin dialog!

    // start to fill in the dlgtemplate information.  addressing by WORDs
    lStyle = DS_MODALFRAME | WS_CAPTION | DS_3DLOOK | WS_VISIBLE | DS_SETFONT;

    add_long(lStyle);
    add_long(0);	// (lExtendedStyle)
    pnumitems = p;	//save where the number of items must be stored
    add_word(0);	// NumberOfItems(will change later)
    add_word(10);	// x
    add_word(10);	// y
    add_word(PixelToDialogX(dlgwidth));	// cx

    // Dialog height.
    if (vertical)
	dlgheight = msgheight + 2 * dlgPaddingY
			   + DLG_VERT_PADDING_Y + 2 * fontHeight * numButtons;
    else
	dlgheight = msgheight + 3 * dlgPaddingY + 2 * fontHeight;

    // Dialog needs to be taller if contains an edit box.
    editboxheight = fontHeight + dlgPaddingY + 4 * DLG_VERT_PADDING_Y;
    if (textfield != NULL)
	dlgheight += editboxheight;

    // Restrict the size to a maximum.  Causes a scrollbar to show up.
    if (dlgheight > maxDialogHeight)
    {
	msgheight = msgheight - (dlgheight - maxDialogHeight);
	dlgheight = maxDialogHeight;
	scroll_flag = WS_VSCROLL;
	// Make sure scrollbar doesn't appear in the middle of the dialog
	messageWidth = dlgwidth - dlg_icon_width - 3 * dlgPaddingX;
    }

    add_word(PixelToDialogY(dlgheight));

    add_word(0);	// Menu
    add_word(0);	// Class

    // copy the title of the dialog
    nchar = nCopyAnsiToWideChar(p, (title ? (LPSTR)title
				   : (LPSTR)("Vim "VIM_VERSION_MEDIUM)), TRUE);
    p += nchar;

    // do the font, since DS_3DLOOK doesn't work properly
# ifdef USE_SYSMENU_FONT
    if (use_lfSysmenu)
    {
	// point size
	*p++ = -MulDiv(lfSysmenu.lfHeight, 72,
		GetDeviceCaps(hdc, LOGPIXELSY));
	wcscpy(p, lfSysmenu.lfFaceName);
	nchar = (int)wcslen(lfSysmenu.lfFaceName) + 1;
    }
    else
# endif
    {
	*p++ = DLG_FONT_POINT_SIZE;		// point size
	nchar = nCopyAnsiToWideChar(p, DLG_FONT_NAME, FALSE);
    }
    p += nchar;

    buttonYpos = msgheight + 2 * dlgPaddingY;

    if (textfield != NULL)
	buttonYpos += editboxheight;

    pstart = tbuffer;
    if (!vertical)
	horizWidth = (dlgwidth - horizWidth) / 2;	// Now it's X offset
    for (i = 0; i < numButtons; i++)
    {
	// get end of this button.
	for (	pend = pstart;
		*pend && (*pend != DLG_BUTTON_SEP);
		pend++)
	    ;

	if (*pend)
	    *pend = '\0';

	/*
	 * old NOTE:
	 * setting the BS_DEFPUSHBUTTON style doesn't work because Windows sets
	 * the focus to the first tab-able button and in so doing makes that
	 * the default!! Grrr.  Workaround: Make the default button the only
	 * one with WS_TABSTOP style. Means user can't tab between buttons, but
	 * he/she can use arrow keys.
	 *
	 * new NOTE: BS_DEFPUSHBUTTON is required to be able to select the
	 * right button when hitting <Enter>.  E.g., for the ":confirm quit"
	 * dialog.  Also needed for when the textfield is the default control.
	 * It appears to work now (perhaps not on Win95?).
	 */
	if (vertical)
	{
	    p = add_dialog_element(p,
		    (i == dfltbutton
			    ? BS_DEFPUSHBUTTON : BS_PUSHBUTTON) | WS_TABSTOP,
		    PixelToDialogX(DLG_VERT_PADDING_X),
		    PixelToDialogY(buttonYpos // TBK
				   + 2 * fontHeight * i),
		    PixelToDialogX(dlgwidth - 2 * DLG_VERT_PADDING_X),
		    (WORD)(PixelToDialogY(2 * fontHeight) - 1),
		    (WORD)(IDCANCEL + 1 + i), (WORD)0x0080, (char *)pstart);
	}
	else
	{
	    p = add_dialog_element(p,
		    (i == dfltbutton
			    ? BS_DEFPUSHBUTTON : BS_PUSHBUTTON) | WS_TABSTOP,
		    PixelToDialogX(horizWidth + buttonPositions[i]),
		    PixelToDialogY(buttonYpos), // TBK
		    PixelToDialogX(buttonWidths[i]),
		    (WORD)(PixelToDialogY(2 * fontHeight) - 1),
		    (WORD)(IDCANCEL + 1 + i), (WORD)0x0080, (char *)pstart);
	}
	pstart = pend + 1;	//next button
    }
    *pnumitems += numButtons;

    // Vim icon
    p = add_dialog_element(p, SS_ICON,
	    PixelToDialogX(dlgPaddingX),
	    PixelToDialogY(dlgPaddingY),
	    PixelToDialogX(dlg_icon_width),
	    PixelToDialogY(dlg_icon_height),
	    DLG_NONBUTTON_CONTROL + 0, (WORD)0x0082,
	    dlg_icons[type]);

    // Dialog message
    p = add_dialog_element(p, ES_LEFT|scroll_flag|ES_MULTILINE|ES_READONLY,
	    PixelToDialogX(2 * dlgPaddingX + dlg_icon_width),
	    PixelToDialogY(dlgPaddingY),
	    (WORD)(PixelToDialogX(messageWidth) + 1),
	    PixelToDialogY(msgheight),
	    DLG_NONBUTTON_CONTROL + 1, (WORD)0x0081, (char *)message);

    // Edit box
    if (textfield != NULL)
    {
	p = add_dialog_element(p, ES_LEFT|ES_AUTOHSCROLL|WS_TABSTOP|WS_BORDER,
		PixelToDialogX(2 * dlgPaddingX),
		PixelToDialogY(2 * dlgPaddingY + msgheight),
		PixelToDialogX(dlgwidth - 4 * dlgPaddingX),
		PixelToDialogY(fontHeight + dlgPaddingY),
		DLG_NONBUTTON_CONTROL + 2, (WORD)0x0081, (char *)textfield);
	*pnumitems += 1;
    }

    *pnumitems += 2;

    SelectFont(hdc, oldFont);
    DeleteObject(font);
    ReleaseDC(hwnd, hdc);

    // Let the dialog_callback() function know which button to make default
    // If we have an edit box, make that the default. We also need to tell
    // dialog_callback() if this dialog contains an edit box or not. We do
    // this by setting s_textfield if it does.
    if (textfield != NULL)
    {
	dialog_default_button = DLG_NONBUTTON_CONTROL + 2;
	s_textfield = textfield;
    }
    else
    {
	dialog_default_button = IDCANCEL + 1 + dfltbutton;
	s_textfield = NULL;
    }

    // show the dialog box modally and get a return value
    nchar = (int)DialogBoxIndirect(
	    g_hinst,
	    (LPDLGTEMPLATE)pdlgtemplate,
	    s_hwnd,
	    (DLGPROC)dialog_callback);

    LocalFree(LocalHandle(pdlgtemplate));
    vim_free(tbuffer);
    vim_free(buttonWidths);
    vim_free(buttonPositions);
    vim_free(ga.ga_data);

    // Focus back to our window (for when MDI is used).
    (void)SetFocus(s_hwnd);

    return nchar;
}

#endif // FEAT_GUI_DIALOG

/*
 * Put a simple element (basic class) onto a dialog template in memory.
 * return a pointer to where the next item should be added.
 *
 * parameters:
 *  lStyle = additional style flags
 *		(be careful, NT3.51 & Win32s will ignore the new ones)
 *  x,y = x & y positions IN DIALOG UNITS
 *  w,h = width and height IN DIALOG UNITS
 *  Id  = ID used in messages
 *  clss  = class ID, e.g 0x0080 for a button, 0x0082 for a static
 *  caption = usually text or resource name
 *
 *  TODO: use the length information noted here to enable the dialog creation
 *  routines to work out more exactly how much memory they need to alloc.
 */
    static PWORD
add_dialog_element(
    PWORD p,
    DWORD lStyle,
    WORD x,
    WORD y,
    WORD w,
    WORD h,
    WORD Id,
    WORD clss,
    const char *caption)
{
    int nchar;

    p = lpwAlign(p);	// Align to dword boundary
    lStyle = lStyle | WS_VISIBLE | WS_CHILD;
    *p++ = LOWORD(lStyle);
    *p++ = HIWORD(lStyle);
    *p++ = 0;		// LOWORD (lExtendedStyle)
    *p++ = 0;		// HIWORD (lExtendedStyle)
    *p++ = x;
    *p++ = y;
    *p++ = w;
    *p++ = h;
    *p++ = Id;		//9 or 10 words in all

    *p++ = (WORD)0xffff;
    *p++ = clss;			//2 more here

    nchar = nCopyAnsiToWideChar(p, (LPSTR)caption, TRUE); //strlen(caption)+1
    p += nchar;

    *p++ = 0;  // advance pointer over nExtraStuff WORD   - 2 more

    return p;	// total = 15 + strlen(caption) words
		// bytes read = 2 * total
}


/*
 * Helper routine.  Take an input pointer, return closest pointer that is
 * aligned on a DWORD (4 byte) boundary.  Taken from the Win32SDK samples.
 */
    static LPWORD
lpwAlign(
    LPWORD lpIn)
{
    long_u ul;

    ul = (long_u)lpIn;
    ul += 3;
    ul >>= 2;
    ul <<= 2;
    return (LPWORD)ul;
}

/*
 * Helper routine.  Takes second parameter as Ansi string, copies it to first
 * parameter as wide character (16-bits / char) string, and returns integer
 * number of wide characters (words) in string (including the trailing wide
 * char NULL).  Partly taken from the Win32SDK samples.
 * If "use_enc" is TRUE, 'encoding' is used for "lpAnsiIn". If FALSE, current
 * ACP is used for "lpAnsiIn". */
    static int
nCopyAnsiToWideChar(
    LPWORD lpWCStr,
    LPSTR lpAnsiIn,
    BOOL use_enc)
{
    int		nChar = 0;
    int		len = lstrlen(lpAnsiIn) + 1;	// include NUL character
    int		i;
    WCHAR	*wn;

    if (use_enc && enc_codepage >= 0 && (int)GetACP() != enc_codepage)
    {
	// Not a codepage, use our own conversion function.
	wn = enc_to_utf16((char_u *)lpAnsiIn, NULL);
	if (wn != NULL)
	{
	    wcscpy(lpWCStr, wn);
	    nChar = (int)wcslen(wn) + 1;
	    vim_free(wn);
	}
    }
    if (nChar == 0)
	// Use Win32 conversion function.
	nChar = MultiByteToWideChar(
		enc_codepage > 0 ? enc_codepage : CP_ACP,
		MB_PRECOMPOSED,
		lpAnsiIn, len,
		lpWCStr, len);
    for (i = 0; i < nChar; ++i)
	if (lpWCStr[i] == (WORD)'\t')	// replace tabs with spaces
	    lpWCStr[i] = (WORD)' ';

    return nChar;
}


#ifdef FEAT_TEAROFF
/*
 * Lookup menu handle from "menu_id".
 */
    static HMENU
tearoff_lookup_menuhandle(
    vimmenu_T *menu,
    WORD menu_id)
{
    for ( ; menu != NULL; menu = menu->next)
    {
	if (menu->modes == 0)	// this menu has just been deleted
	    continue;
	if (menu_is_separator(menu->dname))
	    continue;
	if ((WORD)((long_u)(menu->submenu_id) | (DWORD)0x8000) == menu_id)
	    return menu->submenu_id;
    }
    return NULL;
}

/*
 * The callback function for all the modeless dialogs that make up the
 * "tearoff menus" Very simple - forward button presses (to fool Vim into
 * thinking its menus have been clicked), and go away when closed.
 */
    static LRESULT CALLBACK
tearoff_callback(
    HWND hwnd,
    UINT message,
    WPARAM wParam,
    LPARAM lParam)
{
    if (message == WM_INITDIALOG)
    {
	SetWindowLongPtr(hwnd, DWLP_USER, (LONG_PTR)lParam);
	return (TRUE);
    }

    // May show the mouse pointer again.
    HandleMouseHide(message, lParam);

    if (message == WM_COMMAND)
    {
	if ((WORD)(LOWORD(wParam)) & 0x8000)
	{
	    POINT   mp;
	    RECT    rect;

	    if (GetCursorPos(&mp) && GetWindowRect(hwnd, &rect))
	    {
		vimmenu_T *menu;

		menu = (vimmenu_T*)GetWindowLongPtr(hwnd, DWLP_USER);
		(void)TrackPopupMenu(
			 tearoff_lookup_menuhandle(menu, LOWORD(wParam)),
			 TPM_LEFTALIGN | TPM_LEFTBUTTON,
			 (int)rect.right - 8,
			 (int)mp.y,
			 (int)0,	    // reserved param
			 s_hwnd,
			 NULL);
		/*
		 * NOTE: The pop-up menu can eat the mouse up event.
		 * We deal with this in normal.c.
		 */
	    }
	}
	else
	    // Pass on messages to the main Vim window
	    PostMessage(s_hwnd, WM_COMMAND, LOWORD(wParam), 0);
	/*
	 * Give main window the focus back: this is so after
	 * choosing a tearoff button you can start typing again
	 * straight away.
	 */
	(void)SetFocus(s_hwnd);
	return TRUE;
    }
    if ((message == WM_SYSCOMMAND) && (wParam == SC_CLOSE))
    {
	DestroyWindow(hwnd);
	return TRUE;
    }

    // When moved around, give main window the focus back.
    if (message == WM_EXITSIZEMOVE)
	(void)SetActiveWindow(s_hwnd);

    return FALSE;
}
#endif


/*
 * Computes the dialog base units based on the current dialog font.
 * We don't use the GetDialogBaseUnits() API, because we don't use the
 * (old-style) system font.
 */
    static void
get_dialog_font_metrics(void)
{
    HDC		    hdc;
    HFONT	    hfontTools = 0;
    SIZE	    size;
#ifdef USE_SYSMENU_FONT
    LOGFONTW	    lfSysmenu;
#endif

#ifdef USE_SYSMENU_FONT
    if (gui_w32_get_menu_font(&lfSysmenu) == OK)
	hfontTools = CreateFontIndirectW(&lfSysmenu);
    else
#endif
	hfontTools = CreateFont(-DLG_FONT_POINT_SIZE, 0, 0, 0, 0, 0, 0, 0,
				0, 0, 0, 0, VARIABLE_PITCH, DLG_FONT_NAME);

    hdc = GetDC(s_hwnd);
    SelectObject(hdc, hfontTools);
    GetAverageFontSize(hdc, &size);
    ReleaseDC(s_hwnd, hdc);

    s_dlgfntwidth = (WORD)size.cx;
    s_dlgfntheight = (WORD)size.cy;
}

#if defined(FEAT_MENU) && defined(FEAT_TEAROFF)
/*
 * Create a pseudo-"tearoff menu" based on the child
 * items of a given menu pointer.
 */
    static void
gui_mch_tearoff(
    char_u	*title,
    vimmenu_T	*menu,
    int		initX,
    int		initY)
{
    WORD	*p, *pdlgtemplate, *pnumitems, *ptrueheight;
    int		template_len;
    int		nchar, textWidth, submenuWidth;
    DWORD	lStyle;
    DWORD	lExtendedStyle;
    WORD	dlgwidth;
    WORD	menuID;
    vimmenu_T	*pmenu;
    vimmenu_T	*top_menu;
    vimmenu_T	*the_menu = menu;
    HWND	hwnd;
    HDC		hdc;
    HFONT	font, oldFont;
    int		col, spaceWidth, len;
    int		columnWidths[2];
    char_u	*label, *text;
    int		acLen = 0;
    int		nameLen;
    int		padding0, padding1, padding2 = 0;
    int		sepPadding=0;
    int		x;
    int		y;
# ifdef USE_SYSMENU_FONT
    LOGFONTW	lfSysmenu;
    int		use_lfSysmenu = FALSE;
# endif

    /*
     * If this menu is already torn off, move it to the mouse position.
     */
    if (IsWindow(menu->tearoff_handle))
    {
	POINT mp;
	if (GetCursorPos(&mp))
	{
	    SetWindowPos(menu->tearoff_handle, NULL, mp.x, mp.y, 0, 0,
		    SWP_NOACTIVATE | SWP_NOSIZE | SWP_NOZORDER);
	}
	return;
    }

    /*
     * Create a new tearoff.
     */
    if (*title == MNU_HIDDEN_CHAR)
	title++;

    // Allocate memory to store the dialog template.  It's made bigger when
    // needed.
    template_len = DLG_ALLOC_SIZE;
    pdlgtemplate = p = (WORD *)LocalAlloc(LPTR, template_len);
    if (p == NULL)
	return;

    hwnd = GetDesktopWindow();
    hdc = GetWindowDC(hwnd);
# ifdef USE_SYSMENU_FONT
    if (gui_w32_get_menu_font(&lfSysmenu) == OK)
    {
	font = CreateFontIndirectW(&lfSysmenu);
	use_lfSysmenu = TRUE;
    }
    else
# endif
	font = CreateFont(-DLG_FONT_POINT_SIZE, 0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, VARIABLE_PITCH, DLG_FONT_NAME);

    oldFont = SelectFont(hdc, font);

    // Calculate width of a single space.  Used for padding columns to the
    // right width.
    spaceWidth = GetTextWidth(hdc, (char_u *)" ", 1);

    // Figure out max width of the text column, the accelerator column and the
    // optional submenu column.
    submenuWidth = 0;
    for (col = 0; col < 2; col++)
    {
	columnWidths[col] = 0;
	FOR_ALL_CHILD_MENUS(menu, pmenu)
	{
	    // Use "dname" here to compute the width of the visible text.
	    text = (col == 0) ? pmenu->dname : pmenu->actext;
	    if (text != NULL && *text != NUL)
	    {
		textWidth = GetTextWidthEnc(hdc, text, (int)STRLEN(text));
		if (textWidth > columnWidths[col])
		    columnWidths[col] = textWidth;
	    }
	    if (pmenu->children != NULL)
		submenuWidth = TEAROFF_COLUMN_PADDING * spaceWidth;
	}
    }
    if (columnWidths[1] == 0)
    {
	// no accelerators
	if (submenuWidth != 0)
	    columnWidths[0] += submenuWidth;
	else
	    columnWidths[0] += spaceWidth;
    }
    else
    {
	// there is an accelerator column
	columnWidths[0] += TEAROFF_COLUMN_PADDING * spaceWidth;
	columnWidths[1] += submenuWidth;
    }

    /*
     * Now find the total width of our 'menu'.
     */
    textWidth = columnWidths[0] + columnWidths[1];
    if (submenuWidth != 0)
    {
	submenuWidth = GetTextWidth(hdc, (char_u *)TEAROFF_SUBMENU_LABEL,
					  (int)STRLEN(TEAROFF_SUBMENU_LABEL));
	textWidth += submenuWidth;
    }
    dlgwidth = GetTextWidthEnc(hdc, title, (int)STRLEN(title));
    if (textWidth > dlgwidth)
	dlgwidth = textWidth;
    dlgwidth += 2 * TEAROFF_PADDING_X + TEAROFF_BUTTON_PAD_X;

    // start to fill in the dlgtemplate information.  addressing by WORDs
    lStyle = DS_MODALFRAME | WS_CAPTION | WS_SYSMENU | DS_SETFONT | WS_VISIBLE;

    lExtendedStyle = WS_EX_TOOLWINDOW|WS_EX_STATICEDGE;
    *p++ = LOWORD(lStyle);
    *p++ = HIWORD(lStyle);
    *p++ = LOWORD(lExtendedStyle);
    *p++ = HIWORD(lExtendedStyle);
    pnumitems = p;	// save where the number of items must be stored
    *p++ = 0;		// NumberOfItems(will change later)
    gui_mch_getmouse(&x, &y);
    if (initX == 0xffffL)
	*p++ = PixelToDialogX(x); // x
    else
	*p++ = PixelToDialogX(initX); // x
    if (initY == 0xffffL)
	*p++ = PixelToDialogY(y); // y
    else
	*p++ = PixelToDialogY(initY); // y
    *p++ = PixelToDialogX(dlgwidth);    // cx
    ptrueheight = p;
    *p++ = 0;		// dialog height: changed later anyway
    *p++ = 0;		// Menu
    *p++ = 0;		// Class

    // copy the title of the dialog
    nchar = nCopyAnsiToWideChar(p, ((*title)
			    ? (LPSTR)title
			    : (LPSTR)("Vim "VIM_VERSION_MEDIUM)), TRUE);
    p += nchar;

    // do the font, since DS_3DLOOK doesn't work properly
# ifdef USE_SYSMENU_FONT
    if (use_lfSysmenu)
    {
	// point size
	*p++ = -MulDiv(lfSysmenu.lfHeight, 72,
		GetDeviceCaps(hdc, LOGPIXELSY));
	wcscpy(p, lfSysmenu.lfFaceName);
	nchar = (int)wcslen(lfSysmenu.lfFaceName) + 1;
    }
    else
# endif
    {
	*p++ = DLG_FONT_POINT_SIZE;		// point size
	nchar = nCopyAnsiToWideChar(p, DLG_FONT_NAME, FALSE);
    }
    p += nchar;

    /*
     * Loop over all the items in the menu.
     * But skip over the tearbar.
     */
    if (STRCMP(menu->children->name, TEAR_STRING) == 0)
	menu = menu->children->next;
    else
	menu = menu->children;
    top_menu = menu;
    for ( ; menu != NULL; menu = menu->next)
    {
	if (menu->modes == 0)	// this menu has just been deleted
	    continue;
	if (menu_is_separator(menu->dname))
	{
	    sepPadding += 3;
	    continue;
	}

	// Check if there still is plenty of room in the template.  Make it
	// larger when needed.
	if (((char *)p - (char *)pdlgtemplate) + 1000 > template_len)
	{
	    WORD    *newp;

	    newp = (WORD *)LocalAlloc(LPTR, template_len + 4096);
	    if (newp != NULL)
	    {
		template_len += 4096;
		mch_memmove(newp, pdlgtemplate,
					    (char *)p - (char *)pdlgtemplate);
		p = newp + (p - pdlgtemplate);
		pnumitems = newp + (pnumitems - pdlgtemplate);
		ptrueheight = newp + (ptrueheight - pdlgtemplate);
		LocalFree(LocalHandle(pdlgtemplate));
		pdlgtemplate = newp;
	    }
	}

	// Figure out minimal length of this menu label.  Use "name" for the
	// actual text, "dname" for estimating the displayed size.  "name"
	// has "&a" for mnemonic and includes the accelerator.
	len = nameLen = (int)STRLEN(menu->name);
	padding0 = (columnWidths[0] - GetTextWidthEnc(hdc, menu->dname,
				      (int)STRLEN(menu->dname))) / spaceWidth;
	len += padding0;

	if (menu->actext != NULL)
	{
	    acLen = (int)STRLEN(menu->actext);
	    len += acLen;
	    textWidth = GetTextWidthEnc(hdc, menu->actext, acLen);
	}
	else
	    textWidth = 0;
	padding1 = (columnWidths[1] - textWidth) / spaceWidth;
	len += padding1;

	if (menu->children == NULL)
	{
	    padding2 = submenuWidth / spaceWidth;
	    len += padding2;
	    menuID = (WORD)(menu->id);
	}
	else
	{
	    len += (int)STRLEN(TEAROFF_SUBMENU_LABEL);
	    menuID = (WORD)((long_u)(menu->submenu_id) | (DWORD)0x8000);
	}

	// Allocate menu label and fill it in
	text = label = alloc(len + 1);
	if (label == NULL)
	    break;

	vim_strncpy(text, menu->name, nameLen);
	text = vim_strchr(text, TAB);	    // stop at TAB before actext
	if (text == NULL)
	    text = label + nameLen;	    // no actext, use whole name
	while (padding0-- > 0)
	    *text++ = ' ';
	if (menu->actext != NULL)
	{
	    STRNCPY(text, menu->actext, acLen);
	    text += acLen;
	}
	while (padding1-- > 0)
	    *text++ = ' ';
	if (menu->children != NULL)
	{
	    STRCPY(text, TEAROFF_SUBMENU_LABEL);
	    text += STRLEN(TEAROFF_SUBMENU_LABEL);
	}
	else
	{
	    while (padding2-- > 0)
		*text++ = ' ';
	}
	*text = NUL;

	/*
	 * BS_LEFT will just be ignored on Win32s/NT3.5x - on
	 * W95/NT4 it makes the tear-off look more like a menu.
	 */
	p = add_dialog_element(p,
		BS_PUSHBUTTON|BS_LEFT,
		(WORD)PixelToDialogX(TEAROFF_PADDING_X),
		(WORD)(sepPadding + 1 + 13 * (*pnumitems)),
		(WORD)PixelToDialogX(dlgwidth - 2 * TEAROFF_PADDING_X),
		(WORD)12,
		menuID, (WORD)0x0080, (char *)label);
	vim_free(label);
	(*pnumitems)++;
    }

    *ptrueheight = (WORD)(sepPadding + 1 + 13 * (*pnumitems));


    // show modelessly
    the_menu->tearoff_handle = CreateDialogIndirectParam(
	    g_hinst,
	    (LPDLGTEMPLATE)pdlgtemplate,
	    s_hwnd,
	    (DLGPROC)tearoff_callback,
	    (LPARAM)top_menu);

    LocalFree(LocalHandle(pdlgtemplate));
    SelectFont(hdc, oldFont);
    DeleteObject(font);
    ReleaseDC(hwnd, hdc);

    /*
     * Reassert ourselves as the active window.  This is so that after creating
     * a tearoff, the user doesn't have to click with the mouse just to start
     * typing again!
     */
    (void)SetActiveWindow(s_hwnd);

    // make sure the right buttons are enabled
    force_menu_update = TRUE;
}
#endif

#if defined(FEAT_TOOLBAR) || defined(PROTO)
# include "gui_w32_rc.h"

/*
 * Create the toolbar, initially unpopulated.
 *  (just like the menu, there are no defaults, it's all
 *  set up through menu.vim)
 */
    static void
initialise_toolbar(void)
{
    InitCommonControls();
    s_toolbarhwnd = CreateToolbarEx(
		    s_hwnd,
		    WS_CHILD | TBSTYLE_TOOLTIPS | TBSTYLE_FLAT,
		    4000,		//any old big number
		    31,			//number of images in initial bitmap
		    g_hinst,
		    IDR_TOOLBAR1,	// id of initial bitmap
		    NULL,
		    0,			// initial number of buttons
		    TOOLBAR_BUTTON_WIDTH, //api guide is wrong!
		    TOOLBAR_BUTTON_HEIGHT,
		    TOOLBAR_BUTTON_WIDTH,
		    TOOLBAR_BUTTON_HEIGHT,
		    sizeof(TBBUTTON)
		    );

    // Remove transparency from the toolbar to prevent the main window
    // background colour showing through
    SendMessage(s_toolbarhwnd, TB_SETSTYLE, 0,
	SendMessage(s_toolbarhwnd, TB_GETSTYLE, 0, 0) & ~TBSTYLE_TRANSPARENT);

    s_toolbar_wndproc = SubclassWindow(s_toolbarhwnd, toolbar_wndproc);

    gui_mch_show_toolbar(vim_strchr(p_go, GO_TOOLBAR) != NULL);

    update_toolbar_size();
}

    static void
update_toolbar_size(void)
{
    int		w, h;
    TBMETRICS	tbm;

    tbm.cbSize = sizeof(TBMETRICS);
    tbm.dwMask = TBMF_PAD | TBMF_BUTTONSPACING;
    SendMessage(s_toolbarhwnd, TB_GETMETRICS, 0, (LPARAM)&tbm);
    //TRACE("Pad: %d, %d", tbm.cxPad, tbm.cyPad);
    //TRACE("ButtonSpacing: %d, %d", tbm.cxButtonSpacing, tbm.cyButtonSpacing);

    w = (TOOLBAR_BUTTON_WIDTH + tbm.cxPad) * s_dpi / DEFAULT_DPI;
    h = (TOOLBAR_BUTTON_HEIGHT + tbm.cyPad) * s_dpi / DEFAULT_DPI;
    //TRACE("button size: %d, %d", w, h);
    SendMessage(s_toolbarhwnd, TB_SETBUTTONSIZE, 0, MAKELPARAM(w, h));
    gui.toolbar_height = h + 6;

    //DWORD s = SendMessage(s_toolbarhwnd, TB_GETBUTTONSIZE, 0, 0);
    //TRACE("actual button size: %d, %d", LOWORD(s), HIWORD(s));

    // TODO:
    // Currently, this function only updates the size of toolbar buttons.
    // It would be nice if the toolbar images are resized based on DPI.
}

    static LRESULT CALLBACK
toolbar_wndproc(
    HWND hwnd,
    UINT uMsg,
    WPARAM wParam,
    LPARAM lParam)
{
    HandleMouseHide(uMsg, lParam);
    return CallWindowProc(s_toolbar_wndproc, hwnd, uMsg, wParam, lParam);
}

    static int
get_toolbar_bitmap(vimmenu_T *menu)
{
    int i = -1;

    /*
     * Check user bitmaps first, unless builtin is specified.
     */
    if (!menu->icon_builtin)
    {
	char_u fname[MAXPATHL];
	HANDLE hbitmap = NULL;

	if (menu->iconfile != NULL)
	{
	    gui_find_iconfile(menu->iconfile, fname, "bmp");
	    hbitmap = LoadImage(
			NULL,
			(LPCSTR)fname,
			IMAGE_BITMAP,
			TOOLBAR_BUTTON_WIDTH,
			TOOLBAR_BUTTON_HEIGHT,
			LR_LOADFROMFILE |
			LR_LOADMAP3DCOLORS
			);
	}

	/*
	 * If the LoadImage call failed, or the "icon=" file
	 * didn't exist or wasn't specified, try the menu name
	 */
	if (hbitmap == NULL
		&& (gui_find_bitmap(
# ifdef FEAT_MULTI_LANG
			    menu->en_dname != NULL ? menu->en_dname :
# endif
					menu->dname, fname, "bmp") == OK))
	    hbitmap = LoadImage(
		    NULL,
		    (LPCSTR)fname,
		    IMAGE_BITMAP,
		    TOOLBAR_BUTTON_WIDTH,
		    TOOLBAR_BUTTON_HEIGHT,
		    LR_LOADFROMFILE |
		    LR_LOADMAP3DCOLORS
		);

	if (hbitmap != NULL)
	{
	    TBADDBITMAP tbAddBitmap;

	    tbAddBitmap.hInst = NULL;
	    tbAddBitmap.nID = (long_u)hbitmap;

	    i = (int)SendMessage(s_toolbarhwnd, TB_ADDBITMAP,
			    (WPARAM)1, (LPARAM)&tbAddBitmap);
	    // i will be set to -1 if it fails
	}
    }
    if (i == -1 && menu->iconidx >= 0 && menu->iconidx < TOOLBAR_BITMAP_COUNT)
	i = menu->iconidx;

    return i;
}
#endif

#if defined(FEAT_GUI_TABLINE) || defined(PROTO)
    static void
initialise_tabline(void)
{
    InitCommonControls();

    s_tabhwnd = CreateWindow(WC_TABCONTROL, "Vim tabline",
	    WS_CHILD|TCS_FOCUSNEVER|TCS_TOOLTIPS,
	    CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT,
	    CW_USEDEFAULT, s_hwnd, NULL, g_hinst, NULL);
    s_tabline_wndproc = SubclassWindow(s_tabhwnd, tabline_wndproc);

    gui.tabline_height = TABLINE_HEIGHT;

    set_tabline_font();
}

/*
 * Get tabpage_T from POINT.
 */
    static tabpage_T *
GetTabFromPoint(
    HWND    hWnd,
    POINT   pt)
{
    tabpage_T	*ptp = NULL;

    if (gui_mch_showing_tabline())
    {
	TCHITTESTINFO htinfo;
	htinfo.pt = pt;
	// ignore if a window under cursor is not tabcontrol.
	if (s_tabhwnd == hWnd)
	{
	    int idx = TabCtrl_HitTest(s_tabhwnd, &htinfo);
	    if (idx != -1)
		ptp = find_tabpage(idx + 1);
	}
    }
    return ptp;
}

static POINT	    s_pt = {0, 0};
static HCURSOR      s_hCursor = NULL;

    static LRESULT CALLBACK
tabline_wndproc(
    HWND hwnd,
    UINT uMsg,
    WPARAM wParam,
    LPARAM lParam)
{
    POINT	pt;
    tabpage_T	*tp;
    RECT	rect;
    int		nCenter;
    int		idx0;
    int		idx1;

    HandleMouseHide(uMsg, lParam);

    switch (uMsg)
    {
	case WM_LBUTTONDOWN:
	    {
		s_pt.x = GET_X_LPARAM(lParam);
		s_pt.y = GET_Y_LPARAM(lParam);
		SetCapture(hwnd);
		s_hCursor = GetCursor(); // backup default cursor
		break;
	    }
	case WM_MOUSEMOVE:
	    if (GetCapture() == hwnd
		    && ((wParam & MK_LBUTTON)) != 0)
	    {
		pt.x = GET_X_LPARAM(lParam);
		pt.y = s_pt.y;
		if (abs(pt.x - s_pt.x) >
			pGetSystemMetricsForDpi(SM_CXDRAG, s_dpi))
		{
		    SetCursor(LoadCursor(NULL, IDC_SIZEWE));

		    tp = GetTabFromPoint(hwnd, pt);
		    if (tp != NULL)
		    {
			idx0 = tabpage_index(curtab) - 1;
			idx1 = tabpage_index(tp) - 1;

			TabCtrl_GetItemRect(hwnd, idx1, &rect);
			nCenter = rect.left + (rect.right - rect.left) / 2;

			// Check if the mouse cursor goes over the center of
			// the next tab to prevent "flickering".
			if ((idx0 < idx1) && (nCenter < pt.x))
			{
			    tabpage_move(idx1 + 1);
			    update_screen(0);
			}
			else if ((idx1 < idx0) && (pt.x < nCenter))
			{
			    tabpage_move(idx1);
			    update_screen(0);
			}
		    }
		}
	    }
	    break;
	case WM_LBUTTONUP:
	    {
		if (GetCapture() == hwnd)
		{
		    SetCursor(s_hCursor);
		    ReleaseCapture();
		}
		break;
	    }
	case WM_MBUTTONUP:
	    {
		TCHITTESTINFO htinfo;

		htinfo.pt.x = GET_X_LPARAM(lParam);
		htinfo.pt.y = GET_Y_LPARAM(lParam);
		idx0 = TabCtrl_HitTest(hwnd, &htinfo);
		if (idx0 != -1)
		{
		    idx0 += 1;
		    send_tabline_menu_event(idx0, TABLINE_MENU_CLOSE);
		}
		break;
	    }
	default:
	    break;
    }

    return CallWindowProc(s_tabline_wndproc, hwnd, uMsg, wParam, lParam);
}
#endif

#if defined(FEAT_OLE) || defined(FEAT_EVAL) || defined(PROTO)
/*
 * Make the GUI window come to the foreground.
 */
    void
gui_mch_set_foreground(void)
{
    if (IsIconic(s_hwnd))
	 SendMessage(s_hwnd, WM_SYSCOMMAND, SC_RESTORE, 0);
    SetForegroundWindow(s_hwnd);
}
#endif

#if defined(FEAT_MBYTE_IME) && defined(DYNAMIC_IME)
    static void
dyn_imm_load(void)
{
    hLibImm = vimLoadLib("imm32.dll");
    if (hLibImm == NULL)
	return;

    pImmGetCompositionStringW
	    = (LONG (WINAPI *)(HIMC, DWORD, LPVOID, DWORD))GetProcAddress(hLibImm, "ImmGetCompositionStringW");
    pImmGetContext
	    = (HIMC (WINAPI *)(HWND))GetProcAddress(hLibImm, "ImmGetContext");
    pImmAssociateContext
	    = (HIMC (WINAPI *)(HWND, HIMC))GetProcAddress(hLibImm, "ImmAssociateContext");
    pImmReleaseContext
	    = (BOOL (WINAPI *)(HWND, HIMC))GetProcAddress(hLibImm, "ImmReleaseContext");
    pImmGetOpenStatus
	    = (BOOL (WINAPI *)(HIMC))GetProcAddress(hLibImm, "ImmGetOpenStatus");
    pImmSetOpenStatus
	    = (BOOL (WINAPI *)(HIMC, BOOL))GetProcAddress(hLibImm, "ImmSetOpenStatus");
    pImmGetCompositionFontW
	    = (BOOL (WINAPI *)(HIMC, LPLOGFONTW))GetProcAddress(hLibImm, "ImmGetCompositionFontW");
    pImmSetCompositionFontW
	    = (BOOL (WINAPI *)(HIMC, LPLOGFONTW))GetProcAddress(hLibImm, "ImmSetCompositionFontW");
    pImmSetCompositionWindow
	    = (BOOL (WINAPI *)(HIMC, LPCOMPOSITIONFORM))GetProcAddress(hLibImm, "ImmSetCompositionWindow");
    pImmGetConversionStatus
	    = (BOOL (WINAPI *)(HIMC, LPDWORD, LPDWORD))GetProcAddress(hLibImm, "ImmGetConversionStatus");
    pImmSetConversionStatus
	    = (BOOL (WINAPI *)(HIMC, DWORD, DWORD))GetProcAddress(hLibImm, "ImmSetConversionStatus");

    if (       pImmGetCompositionStringW == NULL
	    || pImmGetContext == NULL
	    || pImmAssociateContext == NULL
	    || pImmReleaseContext == NULL
	    || pImmGetOpenStatus == NULL
	    || pImmSetOpenStatus == NULL
	    || pImmGetCompositionFontW == NULL
	    || pImmSetCompositionFontW == NULL
	    || pImmSetCompositionWindow == NULL
	    || pImmGetConversionStatus == NULL
	    || pImmSetConversionStatus == NULL)
    {
	FreeLibrary(hLibImm);
	hLibImm = NULL;
	pImmGetContext = NULL;
	return;
    }

    return;
}

#endif

#if defined(FEAT_SIGN_ICONS) || defined(PROTO)

# ifdef FEAT_XPM_W32
#  define IMAGE_XPM   100
# endif

typedef struct _signicon_t
{
    HANDLE	hImage;
    UINT	uType;
# ifdef FEAT_XPM_W32
    HANDLE	hShape;	// Mask bitmap handle
# endif
} signicon_t;

    void
gui_mch_drawsign(int row, int col, int typenr)
{
    signicon_t *sign;
    int x, y, w, h;

    if (!gui.in_use || (sign = (signicon_t *)sign_get_image(typenr)) == NULL)
	return;

# if defined(FEAT_DIRECTX)
    if (IS_ENABLE_DIRECTX())
	DWriteContext_Flush(s_dwc);
# endif

    x = TEXT_X(col);
    y = TEXT_Y(row);
    w = gui.char_width * 2;
    h = gui.char_height;
    switch (sign->uType)
    {
	case IMAGE_BITMAP:
	    {
		HDC hdcMem;
		HBITMAP hbmpOld;

		hdcMem = CreateCompatibleDC(s_hdc);
		hbmpOld = (HBITMAP)SelectObject(hdcMem, sign->hImage);
		BitBlt(s_hdc, x, y, w, h, hdcMem, 0, 0, SRCCOPY);
		SelectObject(hdcMem, hbmpOld);
		DeleteDC(hdcMem);
	    }
	    break;
	case IMAGE_ICON:
	case IMAGE_CURSOR:
	    DrawIconEx(s_hdc, x, y, (HICON)sign->hImage, w, h, 0, NULL, DI_NORMAL);
	    break;
# ifdef FEAT_XPM_W32
	case IMAGE_XPM:
	    {
		HDC hdcMem;
		HBITMAP hbmpOld;

		hdcMem = CreateCompatibleDC(s_hdc);
		hbmpOld = (HBITMAP)SelectObject(hdcMem, sign->hShape);
		// Make hole
		BitBlt(s_hdc, x, y, w, h, hdcMem, 0, 0, SRCAND);

		SelectObject(hdcMem, sign->hImage);
		// Paint sign
		BitBlt(s_hdc, x, y, w, h, hdcMem, 0, 0, SRCPAINT);
		SelectObject(hdcMem, hbmpOld);
		DeleteDC(hdcMem);
	    }
	    break;
# endif
    }
}

    static void
close_signicon_image(signicon_t *sign)
{
    if (sign == NULL)
	return;

    switch (sign->uType)
    {
	case IMAGE_BITMAP:
	    DeleteObject((HGDIOBJ)sign->hImage);
	    break;
	case IMAGE_CURSOR:
	    DestroyCursor((HCURSOR)sign->hImage);
	    break;
	case IMAGE_ICON:
	    DestroyIcon((HICON)sign->hImage);
	    break;
# ifdef FEAT_XPM_W32
	case IMAGE_XPM:
	    DeleteObject((HBITMAP)sign->hImage);
	    DeleteObject((HBITMAP)sign->hShape);
	    break;
# endif
    }
}

    void *
gui_mch_register_sign(char_u *signfile)
{
    signicon_t	sign, *psign;
    char_u	*ext;

    sign.hImage = NULL;
    ext = signfile + STRLEN(signfile) - 4; // get extension
    if (ext > signfile)
    {
	int do_load = 1;

	if (!STRICMP(ext, ".bmp"))
	    sign.uType =  IMAGE_BITMAP;
	else if (!STRICMP(ext, ".ico"))
	    sign.uType =  IMAGE_ICON;
	else if (!STRICMP(ext, ".cur") || !STRICMP(ext, ".ani"))
	    sign.uType =  IMAGE_CURSOR;
	else
	    do_load = 0;

	if (do_load)
	    sign.hImage = (HANDLE)LoadImage(NULL, (LPCSTR)signfile, sign.uType,
		    gui.char_width * 2, gui.char_height,
		    LR_LOADFROMFILE | LR_CREATEDIBSECTION);
# ifdef FEAT_XPM_W32
	if (!STRICMP(ext, ".xpm"))
	{
	    sign.uType = IMAGE_XPM;
	    LoadXpmImage((char *)signfile, (HBITMAP *)&sign.hImage,
		    (HBITMAP *)&sign.hShape);
	}
# endif
    }

    psign = NULL;
    if (sign.hImage && (psign = ALLOC_ONE(signicon_t)) != NULL)
	*psign = sign;

    if (!psign)
    {
	if (sign.hImage)
	    close_signicon_image(&sign);
	emsg(_(e_couldnt_read_in_sign_data));
    }
    return (void *)psign;

}

    void
gui_mch_destroy_sign(void *sign)
{
    if (sign == NULL)
	return;

    close_signicon_image((signicon_t *)sign);
    vim_free(sign);
}
#endif

#if defined(FEAT_BEVAL_GUI) || defined(PROTO)

/*
 * BALLOON-EVAL IMPLEMENTATION FOR WINDOWS.
 *  Added by Sergey Khorev <sergey.khorev@gmail.com>
 *
 * The only reused thing is beval.h and get_beval_info()
 * from gui_beval.c (note it uses x and y of the BalloonEval struct
 * to get current mouse position).
 *
 * Trying to use as more Windows services as possible, and as less
 * IE version as possible :)).
 *
 * 1) Don't create ToolTip in gui_mch_create_beval_area, only initialize
 * BalloonEval struct.
 * 2) Enable/Disable simply create/kill BalloonEval Timer
 * 3) When there was enough inactivity, timer procedure posts
 * async request to debugger
 * 4) gui_mch_post_balloon (invoked from netbeans.c) creates tooltip control
 * and performs some actions to show it ASAP
 * 5) WM_NOTIFY:TTN_POP destroys created tooltip
 */

    static void
make_tooltip(BalloonEval *beval, char *text, POINT pt)
{
    TOOLINFOW	*pti;
    RECT	rect;

    pti = ALLOC_ONE(TOOLINFOW);
    if (pti == NULL)
	return;

    beval->balloon = CreateWindowExW(WS_EX_TOPMOST, TOOLTIPS_CLASSW,
	    NULL, WS_POPUP | TTS_NOPREFIX | TTS_ALWAYSTIP,
	    CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT,
	    beval->target, NULL, g_hinst, NULL);

    SetWindowPos(beval->balloon, HWND_TOPMOST, 0, 0, 0, 0,
	    SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE);

    pti->cbSize = sizeof(TOOLINFOW);
    pti->uFlags = TTF_SUBCLASS;
    pti->hwnd = beval->target;
    pti->hinst = 0; // Don't use string resources
    pti->uId = ID_BEVAL_TOOLTIP;

    pti->lpszText = LPSTR_TEXTCALLBACKW;
    beval->tofree = enc_to_utf16((char_u*)text, NULL);
    pti->lParam = (LPARAM)beval->tofree;
    // switch multiline tooltips on
    if (GetClientRect(s_textArea, &rect))
	SendMessageW(beval->balloon, TTM_SETMAXTIPWIDTH, 0,
		(LPARAM)rect.right);

    // Limit ballooneval bounding rect to CursorPos neighbourhood.
    pti->rect.left = pt.x - 3;
    pti->rect.top = pt.y - 3;
    pti->rect.right = pt.x + 3;
    pti->rect.bottom = pt.y + 3;

    SendMessageW(beval->balloon, TTM_ADDTOOLW, 0, (LPARAM)pti);
    // Make tooltip appear sooner.
    SendMessageW(beval->balloon, TTM_SETDELAYTIME, TTDT_INITIAL, 10);
    // I've performed some tests and it seems the longest possible life time
    // of tooltip is 30 seconds.
    SendMessageW(beval->balloon, TTM_SETDELAYTIME, TTDT_AUTOPOP, 30000);
    /*
     * HACK: force tooltip to appear, because it'll not appear until
     * first mouse move. D*mn M$
     * Amazingly moving (2, 2) and then (-1, -1) the mouse doesn't move.
     */
    mouse_event(MOUSEEVENTF_MOVE, 2, 2, 0, 0);
    mouse_event(MOUSEEVENTF_MOVE, (DWORD)-1, (DWORD)-1, 0, 0);
    vim_free(pti);
}

    static void
delete_tooltip(BalloonEval *beval)
{
    PostMessage(beval->balloon, WM_CLOSE, 0, 0);
}

    static VOID CALLBACK
beval_timer_proc(
    HWND	hwnd UNUSED,
    UINT	uMsg UNUSED,
    UINT_PTR	idEvent UNUSED,
    DWORD	dwTime)
{
    POINT	pt;
    RECT	rect;

    if (cur_beval == NULL || cur_beval->showState == ShS_SHOWING || !p_beval)
	return;

    GetCursorPos(&pt);
    if (WindowFromPoint(pt) != s_textArea)
	return;

    ScreenToClient(s_textArea, &pt);
    GetClientRect(s_textArea, &rect);
    if (!PtInRect(&rect, pt))
	return;

    if (last_user_activity > 0
	    && (dwTime - last_user_activity) >= (DWORD)p_bdlay
	    && (cur_beval->showState != ShS_PENDING
		|| abs(cur_beval->x - pt.x) > 3
		|| abs(cur_beval->y - pt.y) > 3))
    {
	// Pointer resting in one place long enough, it's time to show
	// the tooltip.
	cur_beval->showState = ShS_PENDING;
	cur_beval->x = pt.x;
	cur_beval->y = pt.y;

	if (cur_beval->msgCB != NULL)
	    (*cur_beval->msgCB)(cur_beval, 0);
    }
}

    void
gui_mch_disable_beval_area(BalloonEval *beval UNUSED)
{
    KillTimer(s_textArea, beval_timer_id);
}

    void
gui_mch_enable_beval_area(BalloonEval *beval)
{
    if (beval == NULL)
	return;
    beval_timer_id = SetTimer(s_textArea, 0, (UINT)(p_bdlay / 2),
							     beval_timer_proc);
}

    void
gui_mch_post_balloon(BalloonEval *beval, char_u *mesg)
{
    POINT   pt;

    vim_free(beval->msg);
    beval->msg = mesg == NULL ? NULL : vim_strsave(mesg);
    if (beval->msg == NULL)
    {
	delete_tooltip(beval);
	beval->showState = ShS_NEUTRAL;
	return;
    }

    if (beval->showState == ShS_SHOWING)
	return;
    GetCursorPos(&pt);
    ScreenToClient(s_textArea, &pt);

    if (abs(beval->x - pt.x) < 3 && abs(beval->y - pt.y) < 3)
    {
	// cursor is still here
	gui_mch_disable_beval_area(cur_beval);
	beval->showState = ShS_SHOWING;
	make_tooltip(beval, (char *)mesg, pt);
    }
}

    BalloonEval *
gui_mch_create_beval_area(
    void	*target UNUSED,	// ignored, always use s_textArea
    char_u	*mesg,
    void	(*mesgCB)(BalloonEval *, int),
    void	*clientData)
{
    // partially stolen from gui_beval.c
    BalloonEval	*beval;

    if (mesg != NULL && mesgCB != NULL)
    {
	iemsg(e_cannot_create_ballooneval_with_both_message_and_callback);
	return NULL;
    }

    beval = ALLOC_CLEAR_ONE(BalloonEval);
    if (beval != NULL)
    {
	beval->target = s_textArea;

	beval->showState = ShS_NEUTRAL;
	beval->msg = mesg;
	beval->msgCB = mesgCB;
	beval->clientData = clientData;

	InitCommonControls();
	cur_beval = beval;

	if (p_beval)
	    gui_mch_enable_beval_area(beval);
    }
    return beval;
}

    static void
Handle_WM_Notify(HWND hwnd UNUSED, LPNMHDR pnmh)
{
    if (pnmh->idFrom != ID_BEVAL_TOOLTIP) // it is not our tooltip
	return;

    if (cur_beval == NULL)
	return;

    switch (pnmh->code)
    {
	case TTN_SHOW:
	    break;
	case TTN_POP: // Before tooltip disappear
	    delete_tooltip(cur_beval);
	    gui_mch_enable_beval_area(cur_beval);

	    cur_beval->showState = ShS_NEUTRAL;
	    break;
	case TTN_GETDISPINFO:
	    {
		// if you get there then we have new common controls
		NMTTDISPINFO *info = (NMTTDISPINFO *)pnmh;
		info->lpszText = (LPSTR)info->lParam;
		info->uFlags |= TTF_DI_SETITEM;
	    }
	    break;
	case TTN_GETDISPINFOW:
	    {
		// if we get here then we have new common controls
		NMTTDISPINFOW *info = (NMTTDISPINFOW *)pnmh;
		info->lpszText = (LPWSTR)info->lParam;
		info->uFlags |= TTF_DI_SETITEM;
	    }
	    break;
    }
}

    static void
track_user_activity(UINT uMsg)
{
    if ((uMsg >= WM_MOUSEFIRST && uMsg <= WM_MOUSELAST)
	    || (uMsg >= WM_KEYFIRST && uMsg <= WM_KEYLAST))
	last_user_activity = GetTickCount();
}

    void
gui_mch_destroy_beval_area(BalloonEval *beval)
{
# ifdef FEAT_VARTABS
    vim_free(beval->vts);
# endif
    vim_free(beval->tofree);
    vim_free(beval);
}
#endif // FEAT_BEVAL_GUI

#if defined(FEAT_NETBEANS_INTG) || defined(PROTO)
/*
 * We have multiple signs to draw at the same location. Draw the
 * multi-sign indicator (down-arrow) instead. This is the Win32 version.
 */
    void
netbeans_draw_multisign_indicator(int row)
{
    int i;
    int y;
    int x;

    if (!netbeans_active())
	return;

    x = 0;
    y = TEXT_Y(row);

# if defined(FEAT_DIRECTX)
    if (IS_ENABLE_DIRECTX())
	DWriteContext_Flush(s_dwc);
# endif

    for (i = 0; i < gui.char_height - 3; i++)
	SetPixel(s_hdc, x+2, y++, gui.currFgColor);

    SetPixel(s_hdc, x+0, y, gui.currFgColor);
    SetPixel(s_hdc, x+2, y, gui.currFgColor);
    SetPixel(s_hdc, x+4, y++, gui.currFgColor);
    SetPixel(s_hdc, x+1, y, gui.currFgColor);
    SetPixel(s_hdc, x+2, y, gui.currFgColor);
    SetPixel(s_hdc, x+3, y++, gui.currFgColor);
    SetPixel(s_hdc, x+2, y, gui.currFgColor);
}
#endif

#if defined(FEAT_EVAL) || defined(PROTO)

// TODO: at the moment, this is just a copy of test_gui_mouse_event.
// But, we could instead generate actual Win32 mouse event messages,
// ie. to make it consistent with test_gui_w32_sendevent_keyboard.
    static int
test_gui_w32_sendevent_mouse(dict_T *args)
{
    if (!dict_has_key(args, "row") || !dict_has_key(args, "col"))
	return FALSE;

    // Note: "move" is optional, requires fewer arguments
    int move = (int)dict_get_bool(args, "move", FALSE);

    if (!move && (!dict_has_key(args, "button")
	    || !dict_has_key(args, "multiclick")
	    || !dict_has_key(args, "modifiers")))
	return FALSE;

    int row = (int)dict_get_number(args, "row");
    int col = (int)dict_get_number(args, "col");

    if (move)
    {
	// the "move" argument expects row and col coordnates to be in pixels,
	// unless "cell" is specified and is TRUE.
	if (dict_get_bool(args, "cell", FALSE))
	{
	    // calculate the middle of the character cell
	    // Note: Cell coordinates are 1-based from vimscript
	    int pY = (row - 1) * gui.char_height + gui.char_height / 2;
	    int pX = (col - 1) * gui.char_width + gui.char_width / 2;
	    gui_mouse_moved(pX, pY);
	}
	else
	    gui_mouse_moved(col, row);
    }
    else
    {
	int button = (int)dict_get_number(args, "button");
	int repeated_click = (int)dict_get_number(args, "multiclick");
	int_u mods = (int)dict_get_number(args, "modifiers");

	// Reset the scroll values to known values.
	// XXX: Remove this when/if the scroll step is made configurable.
	mouse_set_hor_scroll_step(6);
	mouse_set_vert_scroll_step(3);

	gui_send_mouse_event(button, TEXT_X(col - 1), TEXT_Y(row - 1),
							repeated_click, mods);
    }
    return TRUE;
}

    static int
test_gui_w32_sendevent_keyboard(dict_T *args)
{
    INPUT inputs[1];
    INPUT modkeys[3];
    SecureZeroMemory(inputs, sizeof(INPUT));
    SecureZeroMemory(modkeys, 3 * sizeof(INPUT));

    char_u *event = dict_get_string(args, "event", TRUE);

    if (event && (STRICMP(event, "keydown") == 0
				      || STRICMP(event, "keyup") == 0))
    {
	WORD vkCode = dict_get_number_def(args, "keycode", 0);
	if (vkCode <= 0 || vkCode >= 0xFF)
	{
	    semsg(_(e_invalid_argument_nr), (long)vkCode);
	    return FALSE;
	}

	BOOL isModKey = (vkCode == VK_SHIFT || vkCode == VK_CONTROL
	    || vkCode == VK_MENU || vkCode == VK_LSHIFT || vkCode == VK_RSHIFT
	    || vkCode == VK_LCONTROL || vkCode == VK_RCONTROL
	    || vkCode == VK_LMENU || vkCode == VK_RMENU );

	BOOL unwrapMods = FALSE;
	int mods = (int)dict_get_number(args, "modifiers");

	// If there are modifiers in the args, and it is not a keyup event and
	// vkCode is not a modifier key, then we generate virtual modifier key
	// messages before sending the actual key message.
	if (mods && STRICMP(event, "keydown") == 0 && !isModKey)
	{
	    int n = 0;
	    if (mods & MOD_MASK_SHIFT)
	    {
		modkeys[n].type = INPUT_KEYBOARD;
		modkeys[n].ki.wVk = VK_LSHIFT;
		n++;
	    }
	    if (mods & MOD_MASK_CTRL)
	    {
		modkeys[n].type = INPUT_KEYBOARD;
		modkeys[n].ki.wVk = VK_LCONTROL;
		n++;
	    }
	    if (mods & MOD_MASK_ALT)
	    {
		modkeys[n].type = INPUT_KEYBOARD;
		modkeys[n].ki.wVk = VK_LMENU;
		n++;
	    }
	    if (n)
	    {
		(void)SetForegroundWindow(s_hwnd);
		SendInput(n, modkeys, sizeof(INPUT));
	    }
	}

	inputs[0].type = INPUT_KEYBOARD;
	inputs[0].ki.wVk = vkCode;
	if (STRICMP(event, "keyup") == 0)
	{
	    inputs[0].ki.dwFlags = KEYEVENTF_KEYUP;
	    if (!isModKey)
		unwrapMods = TRUE;
	}

	(void)SetForegroundWindow(s_hwnd);
	SendInput(ARRAYSIZE(inputs), inputs, sizeof(INPUT));
	vim_free(event);

	if (unwrapMods)
	{
	    modkeys[0].type = INPUT_KEYBOARD;
	    modkeys[0].ki.wVk = VK_LSHIFT;
	    modkeys[0].ki.dwFlags = KEYEVENTF_KEYUP;

	    modkeys[1].type = INPUT_KEYBOARD;
	    modkeys[1].ki.wVk = VK_LCONTROL;
	    modkeys[1].ki.dwFlags = KEYEVENTF_KEYUP;

	    modkeys[2].type = INPUT_KEYBOARD;
	    modkeys[2].ki.wVk = VK_LMENU;
	    modkeys[2].ki.dwFlags = KEYEVENTF_KEYUP;

	    (void)SetForegroundWindow(s_hwnd);
	    SendInput(3, modkeys, sizeof(INPUT));
	}
    }
    else
    {
	if (event == NULL)
	{
	    semsg(_(e_missing_argument_str), "event");
	}
	else
	{
	    semsg(_(e_invalid_value_for_argument_str_str), "event", event);
	    vim_free(event);
	}
	return FALSE;
    }
    return TRUE;
}

    static int
test_gui_w32_sendevent_set_keycode_trans_strategy(dict_T *args)
{
    int handled = 0;
    char_u *strategy = dict_get_string(args, "strategy", TRUE);

    if (strategy)
    {
	if (STRICMP(strategy, "classic") == 0)
	{
	    handled = 1;
	    keycode_trans_strategy_used = &keycode_trans_strategy_classic;
	}
	else if (STRICMP(strategy, "experimental") == 0)
	{
	    handled = 1;
	    keycode_trans_strategy_used = &keycode_trans_strategy_experimental;
	}
    }

    if (!handled)
    {
	if (strategy == NULL)
	{
	    semsg(_(e_missing_argument_str), "strategy");
	}
	else
	{
	    semsg(_(e_invalid_value_for_argument_str_str), "strategy", strategy);
	    vim_free(strategy);
	}
	return FALSE;
    }
    return TRUE;
}


    int
test_gui_w32_sendevent(char_u *event, dict_T *args)
{
    if (STRICMP(event, "key") == 0)
	return test_gui_w32_sendevent_keyboard(args);
    else if (STRICMP(event, "mouse") == 0)
	return test_gui_w32_sendevent_mouse(args);
    else if (STRICMP(event, "set_keycode_trans_strategy") == 0)
	return test_gui_w32_sendevent_set_keycode_trans_strategy(args);
    else
    {
	semsg(_(e_invalid_value_for_argument_str_str), "event", event);
	return FALSE;
    }
}
#endif
