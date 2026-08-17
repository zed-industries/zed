
#ifndef _X11_IMUTIL_H_
#define _X11_IMUTIL_H_

extern int
_XGetScanlinePad(
    Display *dpy,
    int depth);

extern int
_XGetBitsPerPixel(
 Display *dpy,
 int depth);

extern int
_XSetImage(
    XImage *srcimg,
    XImage *dstimg,
    int x,
    int y);

extern int
_XReverse_Bytes(
    unsigned char *bpt,
    int nb);
extern void
_XInitImageFuncPtrs(
    XImage *image);

#endif /* _X11_IMUTIL_H_ */
