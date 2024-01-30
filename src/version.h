/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved		by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 */

/*
 * Define the version number, name, etc.
 * The patchlevel is in included_patches[], in version.c.
 */

// Trick to turn a number into a string.
#define VIM_TOSTR_(a)			#a
#define VIM_TOSTR(a)			VIM_TOSTR_(a)

// Values that change for a new release.
#define VIM_VERSION_MAJOR		9
#define VIM_VERSION_MINOR		1
#define VIM_VERSION_BUILD		285
#define VIM_VERSION_BUILD_BCD		0x11d
#define VIM_VERSION_DATE_ONLY		"2024 Jan 02"

// Values based on the above
#define VIM_VERSION_MAJOR_STR		VIM_TOSTR(VIM_VERSION_MAJOR)
#define VIM_VERSION_MINOR_STR		VIM_TOSTR(VIM_VERSION_MINOR)
#define VIM_VERSION_100	    (VIM_VERSION_MAJOR * 100 + VIM_VERSION_MINOR)

#define VIM_VERSION_BUILD_STR		VIM_TOSTR(VIM_VERSION_BUILD)
#ifndef VIM_VERSION_PATCHLEVEL
# define VIM_VERSION_PATCHLEVEL		0
#endif
#define VIM_VERSION_PATCHLEVEL_STR	VIM_TOSTR(VIM_VERSION_PATCHLEVEL)
// Used by MacOS port; should be one of: development, alpha, beta, final
#define VIM_VERSION_RELEASE		final

/*
 * VIM_VERSION_NODOT is used for the runtime directory name.
 * VIM_VERSION_SHORT is copied into the swap file (max. length is 6 chars).
 * VIM_VERSION_MEDIUM is used for the startup-screen.
 * VIM_VERSION_LONG is used for the ":version" command and "Vim -h".
 */
#define VIM_VERSION_NODOT     "vim" VIM_VERSION_MAJOR_STR VIM_VERSION_MINOR_STR
#define VIM_VERSION_SHORT     VIM_VERSION_MAJOR_STR "." VIM_VERSION_MINOR_STR
#define VIM_VERSION_MEDIUM    VIM_VERSION_SHORT
#define VIM_VERSION_LONG_ONLY "VIM - Vi IMproved " VIM_VERSION_MEDIUM
#define VIM_VERSION_LONG_HEAD VIM_VERSION_LONG_ONLY " (" VIM_VERSION_DATE_ONLY
#define VIM_VERSION_LONG      VIM_VERSION_LONG_HEAD ")"
#define VIM_VERSION_LONG_DATE VIM_VERSION_LONG_HEAD ", compiled "
