/*
 *
 * © 2016 and later: Unicode, Inc. and others.
 * License & terms of use: http://www.unicode.org/copyright.html
 *
 * (C) Copyright IBM Corp. 1998-2007 - All Rights Reserved
 *
 */

#ifndef __PFLOW_H
#define __PFLOW_H

#include "unicode/utypes.h"
#include "layout/LETypes.h"

#include "layout/plruns.h"
#include "layout/playout.h"

#include "gsupport.h"
#include "rsurface.h"

typedef void pf_flow;

pf_flow *pf_create(const LEUnicode chars[], le_int32 charCount, const pl_fontRuns *fontRuns, LEErrorCode *status);

void pf_close(pf_flow *flow);

le_int32 pf_getAscent(pf_flow *flow);
le_int32 pf_getLineHeight(pf_flow *flow);
le_int32 pf_getLineCount(pf_flow *flow);
void pf_breakLines(pf_flow *flow, le_int32 width, le_int32 height);
void pf_draw(pf_flow *flow, rs_surface *surface, le_int32 firstLine, le_int32 lastLine);

pf_flow *pf_factory(const char *fileName, const le_font *font, gs_guiSupport *guiSupport);

#endif
