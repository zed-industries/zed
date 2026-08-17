#ifndef XML_PARSER_H_PRIVATE__
#define XML_PARSER_H_PRIVATE__

#include <libxml/parser.h>
#include <libxml/xmlversion.h>

#define XML_INVALID_CHAR 0x200000

#define XML_MAX_URI_LENGTH 2000

/**
 * XML_VCTXT_DTD_VALIDATED:
 *
 * Set after xmlValidateDtdFinal was called.
 */
#define XML_VCTXT_DTD_VALIDATED (1u << 0)
/**
 * XML_VCTXT_USE_PCTXT:
 *
 * Set if the validation context is part of a parser context.
 */
#define XML_VCTXT_USE_PCTXT (1u << 1)

/*
 * TODO: Rename to avoid confusion with xmlParserInputFlags
 */
#define XML_INPUT_HAS_ENCODING      (1u << 0)
#define XML_INPUT_AUTO_ENCODING     (7u << 1)
#define XML_INPUT_AUTO_UTF8         (1u << 1)
#define XML_INPUT_AUTO_UTF16LE      (2u << 1)
#define XML_INPUT_AUTO_UTF16BE      (3u << 1)
#define XML_INPUT_AUTO_OTHER        (4u << 1)
#define XML_INPUT_USES_ENC_DECL     (1u << 4)
#define XML_INPUT_ENCODING_ERROR    (1u << 5)
#define XML_INPUT_PROGRESSIVE       (1u << 6)

#define PARSER_STOPPED(ctxt) ((ctxt)->disableSAX > 1)

#define PARSER_PROGRESSIVE(ctxt) \
    ((ctxt)->input->flags & XML_INPUT_PROGRESSIVE)

#define PARSER_IN_PE(ctxt) \
    (((ctxt)->input->entity != NULL) && \
     (((ctxt)->input->entity->etype == XML_INTERNAL_PARAMETER_ENTITY) || \
      ((ctxt)->input->entity->etype == XML_EXTERNAL_PARAMETER_ENTITY)))

#define PARSER_EXTERNAL(ctxt) \
    (((ctxt)->inSubset == 2) || \
     (((ctxt)->input->entity != NULL) && \
      ((ctxt)->input->entity->etype == XML_EXTERNAL_PARAMETER_ENTITY)))

/**
 * INPUT_CHUNK:
 *
 * The parser tries to always have that amount of input ready.
 * One of the point is providing context when reporting errors.
 */
#define INPUT_CHUNK	250

struct _xmlAttrHashBucket {
    int index;
};

XML_HIDDEN void
xmlCtxtVErr(xmlParserCtxtPtr ctxt, xmlNodePtr node, xmlErrorDomain domain,
            xmlParserErrors code, xmlErrorLevel level,
            const xmlChar *str1, const xmlChar *str2, const xmlChar *str3,
            int int1, const char *msg, va_list ap);
XML_HIDDEN void
xmlCtxtErr(xmlParserCtxtPtr ctxt, xmlNodePtr node, xmlErrorDomain domain,
           xmlParserErrors code, xmlErrorLevel level,
           const xmlChar *str1, const xmlChar *str2, const xmlChar *str3,
           int int1, const char *msg, ...);
XML_HIDDEN void
xmlFatalErr(xmlParserCtxtPtr ctxt, xmlParserErrors error, const char *info);
XML_HIDDEN void LIBXML_ATTR_FORMAT(3,0)
xmlWarningMsg(xmlParserCtxtPtr ctxt, xmlParserErrors error,
              const char *msg, const xmlChar *str1, const xmlChar *str2);
XML_HIDDEN void
xmlCtxtErrIO(xmlParserCtxtPtr ctxt, int code, const char *uri);
XML_HIDDEN int
xmlCtxtIsCatastrophicError(xmlParserCtxtPtr ctxt);

XML_HIDDEN void
xmlHaltParser(xmlParserCtxtPtr ctxt);
XML_HIDDEN int
xmlParserGrow(xmlParserCtxtPtr ctxt);
XML_HIDDEN void
xmlParserShrink(xmlParserCtxtPtr ctxt);

XML_HIDDEN void
xmlDetectEncoding(xmlParserCtxtPtr ctxt);
XML_HIDDEN void
xmlSetDeclaredEncoding(xmlParserCtxtPtr ctxt, xmlChar *encoding);
XML_HIDDEN const xmlChar *
xmlGetActualEncoding(xmlParserCtxtPtr ctxt);

XML_HIDDEN int
nodePush(xmlParserCtxtPtr ctxt, xmlNodePtr value);
XML_HIDDEN xmlNodePtr
nodePop(xmlParserCtxtPtr ctxt);

XML_HIDDEN xmlParserNsData *
xmlParserNsCreate(void);
XML_HIDDEN void
xmlParserNsFree(xmlParserNsData *nsdb);
/*
 * These functions allow SAX handlers to attach extra data to namespaces
 * efficiently and should be made public.
 */
XML_HIDDEN int
xmlParserNsUpdateSax(xmlParserCtxtPtr ctxt, const xmlChar *prefix,
                     void *saxData);
XML_HIDDEN void *
xmlParserNsLookupSax(xmlParserCtxtPtr ctxt, const xmlChar *prefix);

XML_HIDDEN xmlParserInputPtr
xmlLoadResource(xmlParserCtxtPtr ctxt, const char *url, const char *publicId,
                xmlResourceType type);
XML_HIDDEN xmlParserInputPtr
xmlCtxtNewInputFromUrl(xmlParserCtxtPtr ctxt, const char *url,
                       const char *publicId, const char *encoding,
                       xmlParserInputFlags flags);
XML_HIDDEN xmlParserInputPtr
xmlCtxtNewInputFromMemory(xmlParserCtxtPtr ctxt, const char *url,
                          const void *mem, size_t size,
                          const char *encoding,
                          xmlParserInputFlags flags);
XML_HIDDEN xmlParserInputPtr
xmlCtxtNewInputFromString(xmlParserCtxtPtr ctxt, const char *url,
                          const char *str, const char *encoding,
                          xmlParserInputFlags flags);
XML_HIDDEN xmlParserInputPtr
xmlCtxtNewInputFromFd(xmlParserCtxtPtr ctxt, const char *filename, int fd,
                      const char *encoding, xmlParserInputFlags flags);
XML_HIDDEN xmlParserInputPtr
xmlCtxtNewInputFromIO(xmlParserCtxtPtr ctxt, const char *url,
                      xmlInputReadCallback ioRead,
                      xmlInputCloseCallback ioClose,
                      void *ioCtxt,
                      const char *encoding, xmlParserInputFlags flags);
XML_HIDDEN xmlParserInputPtr
xmlNewPushInput(const char *url, const char *chunk, int size);

XML_HIDDEN xmlChar *
xmlExpandEntitiesInAttValue(xmlParserCtxtPtr ctxt, const xmlChar *str,
                            int normalize);

XML_HIDDEN void
xmlParserCheckEOF(xmlParserCtxtPtr ctxt, xmlParserErrors code);

#endif /* XML_PARSER_H_PRIVATE__ */
