#ifndef XML_GLOBALS_H_PRIVATE__
#define XML_GLOBALS_H_PRIVATE__

XML_HIDDEN void
xmlInitGlobalsInternal(void);
XML_HIDDEN void
xmlCleanupGlobalsInternal(void);

XML_HIDDEN xmlError *
xmlGetLastErrorInternal(void);

XML_HIDDEN unsigned *
xmlGetLocalRngState(void);

#endif /* XML_GLOBALS_H_PRIVATE__ */
