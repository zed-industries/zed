#ifndef XML_ERROR_H_PRIVATE__
#define XML_ERROR_H_PRIVATE__

#include <stdarg.h>

#include <libxml/xmlerror.h>
#include <libxml/xmlversion.h>

#define MAX_ERR_MSG_SIZE 64000

struct _xmlNode;

XML_HIDDEN int
xmlIsCatastrophicError(int level, int code);

XML_HIDDEN void
xmlRaiseMemoryError(xmlStructuredErrorFunc schannel, xmlGenericErrorFunc channel,
                    void *data, int domain, xmlError *error);
XML_HIDDEN int
xmlVRaiseError(xmlStructuredErrorFunc schannel, xmlGenericErrorFunc channel,
               void *data, void *ctx, struct _xmlNode *node,
               int domain, int code, xmlErrorLevel level,
               const char *file, int line, const char *str1,
               const char *str2, const char *str3, int int1, int col,
               const char *msg, va_list ap);
XML_HIDDEN int
xmlRaiseError(xmlStructuredErrorFunc schannel, xmlGenericErrorFunc channel,
              void *data, void *ctx, struct _xmlNode *node,
              int domain, int code, xmlErrorLevel level,
              const char *file, int line, const char *str1,
              const char *str2, const char *str3, int int1, int col,
              const char *msg, ...) LIBXML_ATTR_FORMAT(16,17);
XML_HIDDEN void
xmlGenericErrorDefaultFunc(void *ctx, const char *msg,
                           ...) LIBXML_ATTR_FORMAT(2,3);
XML_HIDDEN const char *
xmlErrString(xmlParserErrors code);

XML_HIDDEN void
xmlVPrintErrorMessage(const char *fmt, va_list ap);
XML_HIDDEN void
xmlPrintErrorMessage(const char *fmt, ...);
XML_HIDDEN void
xmlAbort(const char *fmt, ...);

#endif /* XML_ERROR_H_PRIVATE__ */
