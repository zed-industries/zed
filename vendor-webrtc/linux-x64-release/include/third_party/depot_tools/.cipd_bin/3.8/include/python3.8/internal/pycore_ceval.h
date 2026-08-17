#ifndef Py_INTERNAL_CEVAL_H
#define Py_INTERNAL_CEVAL_H
#ifdef __cplusplus
extern "C" {
#endif

#ifndef Py_BUILD_CORE
#  error "this header requires Py_BUILD_CORE define"
#endif

#include "pycore_atomic.h"
#include "pycore_pystate.h"
#include "pythread.h"

PyAPI_FUNC(void) _Py_FinishPendingCalls(_PyRuntimeState *runtime);
PyAPI_FUNC(void) _PyEval_Initialize(struct _ceval_runtime_state *);
PyAPI_FUNC(void) _PyEval_FiniThreads(
    struct _ceval_runtime_state *ceval);
PyAPI_FUNC(void) _PyEval_SignalReceived(
    struct _ceval_runtime_state *ceval);
PyAPI_FUNC(int) _PyEval_AddPendingCall(
    PyThreadState *tstate,
    struct _ceval_runtime_state *ceval,
    int (*func)(void *),
    void *arg);
PyAPI_FUNC(void) _PyEval_SignalAsyncExc(
    struct _ceval_runtime_state *ceval);
PyAPI_FUNC(void) _PyEval_ReInitThreads(
    _PyRuntimeState *runtime);

/* Private function */
void _PyEval_Fini(void);

#ifdef __cplusplus
}
#endif
#endif /* !Py_INTERNAL_CEVAL_H */
