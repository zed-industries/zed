#ifndef _SEPOL_POLICYDB_POLCAPS_H_
#define _SEPOL_POLICYDB_POLCAPS_H_

#ifdef __cplusplus
extern "C" {
#endif

/* Policy capabilities */
enum {
	POLICYDB_CAPABILITY_NETPEER,
	POLICYDB_CAPABILITY_OPENPERM,
	POLICYDB_CAPABILITY_EXTSOCKCLASS,
	POLICYDB_CAPABILITY_ALWAYSNETWORK,
	POLICYDB_CAPABILITY_CGROUPSECLABEL,
	POLICYDB_CAPABILITY_NNP_NOSUID_TRANSITION,
	POLICYDB_CAPABILITY_GENFS_SECLABEL_SYMLINKS,
	__POLICYDB_CAPABILITY_MAX
};
#define POLICYDB_CAPABILITY_MAX (__POLICYDB_CAPABILITY_MAX - 1)

/* Convert a capability name to number. */
extern int sepol_polcap_getnum(const char *name);

/* Convert a capability number to name. */
extern const char *sepol_polcap_getname(unsigned int capnum);

#ifdef __cplusplus
}
#endif

#endif /* _SEPOL_POLICYDB_POLCAPS_H_ */
