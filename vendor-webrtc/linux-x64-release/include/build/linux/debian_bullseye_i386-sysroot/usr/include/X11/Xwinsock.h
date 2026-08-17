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
 * This header file has for sole purpose to allow to include winsock.h
 * without getting any name conflicts with our code.
 * Conflicts come from the fact that including winsock.h actually pulls
 * in the whole Windows API...
 */

#undef _XFree86Server
#ifdef XFree86Server
# define _XFree86Server
# undef XFree86Server
#endif

/*
 * mingw-w64 headers define BOOL as a typedef, protecting against macros
 * mingw.org headers define BOOL in terms of WINBOOL
 * ... so try to come up with something which works with both :-)
 */
#define _NO_BOOL_TYPEDEF
#define BOOL WINBOOL
#define INT32 wINT32
#undef Status
#define Status wStatus
#define ATOM wATOM
#define BYTE wBYTE
#define FreeResource wFreeResource
#include <winsock2.h>
#undef Status
#define Status int
#undef BYTE
#undef BOOL
#undef INT32
#undef ATOM
#undef FreeResource
#undef CreateWindowA
#undef RT_FONT
#undef RT_CURSOR

/*
 * Older version of this header used to name the windows API bool type wBOOL,
 * rather than more standard name WINBOOL
 */
#define wBOOL WINBOOL

#ifdef _XFree86Server
# define XFree86Server
# undef _XFree86Server
#endif

