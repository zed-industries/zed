#ifndef Py_PYFPE_H
#define Py_PYFPE_H

/* These macros used to do something when Python was built with --with-fpectl,
 * but support for that was dropped in 3.7. We continue to define them though,
 * to avoid breaking API users.
 */

#define PyFPE_START_PROTECT(err_string, leave_stmt)
#define PyFPE_END_PROTECT(v)

#endif /* !Py_PYFPE_H */
