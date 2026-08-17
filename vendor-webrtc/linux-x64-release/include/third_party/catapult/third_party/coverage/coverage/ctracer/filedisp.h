/* Licensed under the Apache License: http://www.apache.org/licenses/LICENSE-2.0 */
/* For details: https://bitbucket.org/ned/coveragepy/src/default/NOTICE.txt */

#ifndef _COVERAGE_FILEDISP_H
#define _COVERAGE_FILEDISP_H

#include "util.h"
#include "structmember.h"

typedef struct CFileDisposition {
    PyObject_HEAD

    PyObject * original_filename;
    PyObject * canonical_filename;
    PyObject * source_filename;
    PyObject * trace;
    PyObject * reason;
    PyObject * file_tracer;
    PyObject * has_dynamic_filename;
} CFileDisposition;

void CFileDisposition_dealloc(CFileDisposition *self);

extern PyTypeObject CFileDispositionType;

#endif /* _COVERAGE_FILEDISP_H */
