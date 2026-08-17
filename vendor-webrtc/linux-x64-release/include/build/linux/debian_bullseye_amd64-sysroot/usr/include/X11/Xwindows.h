/*

Copyright 1996, 1998  The Open Group

Permission to use, copy, modify, distribute, and sell this software and its
documentation for any purpose is hereby granted without fee, provided that
the above copyright notice appear in all copies and that both that
copyright notice and this permission notice appear in supporting
documentation.

The above copyright notice and this permission notice shall be included
in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABIL-
ITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT
SHALL THE OPEN GROUP BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABIL-
ITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
IN THE SOFTWARE.

Except as contained in this notice, the name of The Open Group shall
not be used in advertising or otherwise to promote the sale, use or
other dealings in this Software without prior written authorization from
The Open Group.

*/

/*
 * This header file has the sole purpose of allowing the inclusion of
 * windows.h without getting any name conflicts with X headers code, by
 * renaming or disabling the conflicting definitions from windows.h
 */

/*
 * Mingw.org versions of the Windows API headers actually avoid
 * making the conflicting definitions if XFree86Server is defined, so we
 * need to remember if that was defined and undefine it during including
 * windows.h (so the conflicting definitions get wrapped correctly), and
 * then redefine it afterwards. (This was never the correct thing to
 * do as it's no help at all to X11 clients which also need to use the
 * Win32 API)
 */
#undef _XFree86Server
#ifdef XFree86Server
# define _XFree86Server
# undef XFree86Server
#endif

/*
 * There doesn't seem to be a good way to wrap the min/max macros from
 * windows.h, so we simply avoid defining them completely, allowing any
 * pre-existing definition to stand.
 *
 */
#define NOMINMAX

/*
 * mingw-w64 headers define BOOL as a typedef, protecting against macros
 * mingw.org headers define BOOL in terms of WINBOOL
 * ... so try to come up with something which works with both :-)
 */
#define _NO_BOOL_TYPEDEF
#define BOOL WINBOOL
#define INT32 wINT32
#ifdef __x86_64__
#define INT64 wINT64
#define LONG64 wLONG64
#endif
#undef Status
#define Status wStatus
#define ATOM wATOM
#define BYTE wBYTE
#define FreeResource wFreeResource
#include <windows.h>
#undef NOMINMAX
#undef Status
#define Status int
#undef BYTE
#undef BOOL
#undef INT32
#undef INT64
#undef LONG64
#undef ATOM
#undef FreeResource
#undef CreateWindowA

/*
 * Older version of this header used to name the windows API bool type wBOOL,
 * rather than more standard name WINBOOL
 */
#define wBOOL WINBOOL

#ifdef RESOURCE_H
# undef RT_FONT
# undef RT_CURSOR
# define RT_FONT         ((RESTYPE)4)
# define RT_CURSOR       ((RESTYPE)5)
#endif

#ifndef __CYGWIN__
#define sleep(x) Sleep((x) * 1000)
#endif

#if defined(WIN32) && (!defined(PATH_MAX) || PATH_MAX < 1024)
# undef PATH_MAX
# define PATH_MAX 1024
#endif

#ifdef _XFree86Server
# define XFree86Server
# undef _XFree86Server
#endif

