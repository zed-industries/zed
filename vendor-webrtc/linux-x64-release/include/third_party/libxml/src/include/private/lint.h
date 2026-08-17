#ifndef XML_LINT_H_PRIVATE__
#define XML_LINT_H_PRIVATE__

#include <stdio.h>

#include <libxml/parser.h>

int
xmllintMain(int argc, const char **argv, FILE *errStream,
            xmlResourceLoader loader);

void
xmllintShell(xmlDocPtr doc, const char *filename, FILE *output);

#endif /* XML_LINT_H_PRIVATE__ */
