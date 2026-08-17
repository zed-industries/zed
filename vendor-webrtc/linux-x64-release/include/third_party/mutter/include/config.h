/* The prefix for our gettext translation domains. */
#define GETTEXT_PACKAGE "mutter"

/* Version number of package */
#define VERSION "48.alpha"

/* Name of package */
#define PACKAGE_NAME "mutter"

/* Version number of package */
#define PACKAGE_VERSION "48.alpha"

/* Search path for plugins */
#define MUTTER_PLUGIN_DIR "."

/* */
#define MUTTER_LOCALEDIR "/usr/share/locale"

/* */
#define MUTTER_LIBEXECDIR "/usr/libexec"

/* */
#define MUTTER_PKGDATADIR "/usr/share/mutter-16"

/* Defined if EGL support is enabled */
#define HAVE_EGL

/* Defined if EGLDevice support is enabled */
#undef HAVE_EGL_DEVICE

/* Have GLX for rendering */
#undef HAVE_GLX

#undef HAVE_EGL_PLATFORM_XLIB

/* Have GL for rendering */
#define HAVE_GL

/* Have GLES 2.0 for rendering */
#define HAVE_GLES2

/* Defined if EGLStream support is enabled */
#undef HAVE_WAYLAND_EGLSTREAM

/* Building with gudev for device type detection */
#define HAVE_LIBGUDEV

/* Building with libwacom for advanced tablet management */
#undef HAVE_LIBWACOM

/* libwacom has get_num_rings() */
#undef HAVE_LIBWACOM_GET_NUM_RINGS

/* Building with libsystemd */
#define HAVE_LIBSYSTEMD

/* Define if you want to enable the native (KMS) backend based on systemd */
#define HAVE_NATIVE_BACKEND

/* Define if you want to enable Wayland support */
#define HAVE_WAYLAND

/* Define if you want to enable XWayland support */
#undef HAVE_XWAYLAND

/* Define if you want to enable X11 backend support */
#undef HAVE_X11

/* Define if either XWayland or X11 backend are enabled */
#undef HAVE_X11_CLIENT

/* Defined if screen cast and remote desktop support is enabled */
#undef HAVE_REMOTE_DESKTOP

/* Defined if gnome-desktop is enabled */
#undef HAVE_GNOME_DESKTOP

/* Defined if sound player is enabled */
#undef HAVE_SOUND_PLAYER

/* Building with SM support */
#undef HAVE_SM

/* Building with startup notification support */
#undef HAVE_STARTUP_NOTIFICATION

/* Building with Sysprof profiling support */
#undef HAVE_PROFILER

/* Path to Xwayland executable */
/* #undef XWAYLAND_PATH */

/* Xwayland applications allowed to issue keyboard grabs */
#define XWAYLAND_GRAB_DEFAULT_ACCESS_RULES                                \
  "gnome-boxes,remote-viewer,virt-viewer,virt-manager,vinagre,vncviewer," \
  "Xephyr"

/* XKB base prefix */
/* #undef XKB_BASE */

/* Whether <sys/prctl.h> exists and it defines prctl() */
#define HAVE_SYS_PRCTL 1

/* Either <sys/random.h> or <linux/random.h> */
/* #undef HAVE_SYS_RANDOM */
/* #undef HAVE_LINUX_RANDOM */

/* Whether Xwayland has -initfd option */
/* #undef HAVE_XWAYLAND_INITFD */

/* Whether Xwayland has -listenfd option */
/* #undef HAVE_XWAYLAND_LISTENFD */

/* Whether the mkostemp function exists */
#define HAVE_MKOSTEMP 1

/* Whether the posix_fallocate function exists */
#define HAVE_POSIX_FALLOCATE 1

/* Whether the memfd_create function exists */
#define HAVE_MEMFD_CREATE 1

/* Whether the Xwayland -terminate supports a delay */
/* #undef HAVE_XWAYLAND_TERMINATE_DELAY */

/* Whether the Xwayland supports +/-byteswappedclients */
/* #undef HAVE_XWAYLAND_BYTE_SWAPPED_CLIENTS */

/* Whether the Xwayland has -enable-ei-portal option */
/* #undef HAVE_XWAYLAND_ENABLE_EI_PORTAL */

/* Supports timerfd_create/timerfd_settime */
#define HAVE_TIMERFD

/* Supports malloc_trim */
#define HAVE_MALLOC_TRIM

/* Supports eventfd */
#define HAVE_EVENTFD

/* libdrm defines struct drm_plane_size_hint */
#define HAVE_DRM_PLANE_SIZE_HINT

/* Building with font rendering integration support */
#undef HAVE_FONTS
