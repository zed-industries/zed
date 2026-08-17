/*
 *  version.h
 */

#define SND_LIB_MAJOR		1 /**< major number of library version */
#define SND_LIB_MINOR		2 /**< minor number of library version */
#define SND_LIB_SUBMINOR	4 /**< subminor number of library version */
#define SND_LIB_EXTRAVER	1000000 /**< extra version number, used mainly for betas */
/** library version */
#define SND_LIB_VERSION		((SND_LIB_MAJOR<<16)|\
				 (SND_LIB_MINOR<<8)|\
				  SND_LIB_SUBMINOR)
/** library version (string) */
#define SND_LIB_VERSION_STR	"1.2.4"

