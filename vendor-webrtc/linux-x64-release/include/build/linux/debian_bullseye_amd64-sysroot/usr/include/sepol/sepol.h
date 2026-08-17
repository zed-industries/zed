#ifndef _SEPOL_H_
#define _SEPOL_H_

#include <stddef.h>
#include <stdio.h>

#ifdef __cplusplus
extern "C" {
#endif

#include <sepol/user_record.h>
#include <sepol/context_record.h>
#include <sepol/iface_record.h>
#include <sepol/ibpkey_record.h>
#include <sepol/ibendport_record.h>
#include <sepol/port_record.h>
#include <sepol/boolean_record.h>
#include <sepol/node_record.h>

#include <sepol/booleans.h>
#include <sepol/interfaces.h>
#include <sepol/ibpkeys.h>
#include <sepol/ibendports.h>
#include <sepol/ports.h>
#include <sepol/nodes.h>
#include <sepol/users.h>
#include <sepol/handle.h>
#include <sepol/debug.h>
#include <sepol/policydb.h>
#include <sepol/module.h>
#include <sepol/context.h>

/* Set internal policydb from a file for subsequent service calls. */
extern int sepol_set_policydb_from_file(FILE * fp);

#ifdef __cplusplus
}
#endif

#endif
