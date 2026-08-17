#include <stdlib.h>

#include <sepol/policydb/policydb.h>

int sepol_kernel_policydb_to_cil(FILE *fp, struct policydb *pdb);
