/*
 *
 * © 2016 and later: Unicode, Inc. and others.
 * License & terms of use: http://www.unicode.org/copyright.html
 *
 * (C) Copyright IBM Corp. 1998-2007 - All Rights Reserved
 *
 */

#ifndef __GSUPPORT_H
#define __GSUPPORT_H

typedef void gs_guiSupport;

void gs_postErrorMessage(gs_guiSupport *guiSupport, const char *message, const char *title);

#endif
