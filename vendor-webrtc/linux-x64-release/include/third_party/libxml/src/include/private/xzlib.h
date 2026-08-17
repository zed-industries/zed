/**
 * xzlib.h: header for the front end for the transparent support of lzma
 *          compression at the I/O layer
 *
 * See Copyright for the status of this software.
 *
 * Anders F Bjorklund <afb@users.sourceforge.net>
 */

#ifndef LIBXML2_XZLIB_H
#define LIBXML2_XZLIB_H

#include <libxml/xmlversion.h>

#ifdef LIBXML_LZMA_ENABLED

typedef void *xzFile;           /* opaque lzma file descriptor */

XML_HIDDEN xzFile
__libxml2_xzopen(const char *path, const char *mode);
XML_HIDDEN xzFile
__libxml2_xzdopen(const char *path, int fd, const char *mode);
XML_HIDDEN int
__libxml2_xzread(xzFile file, void *buf, unsigned len);
XML_HIDDEN int
__libxml2_xzclose(xzFile file);
XML_HIDDEN int
__libxml2_xzcompressed(xzFile f);

#endif /* LIBXML_LZMA_ENABLED */

#endif /* LIBXML2_XZLIB_H */
