#include <stdlib.h>

#include <sepol/module.h>
#include <sepol/policydb/policydb.h>

int sepol_module_policydb_to_cil(FILE *fp, struct policydb *pdb, int linked);
int sepol_module_package_to_cil(FILE *fp, struct sepol_module_package *mod_pkg);
int sepol_ppfile_to_module_package(FILE *fp, struct sepol_module_package **mod_pkg);
