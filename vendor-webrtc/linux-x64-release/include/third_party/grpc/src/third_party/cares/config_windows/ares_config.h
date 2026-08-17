/* Generated from ares_config.h.cmake*/

/* Define if building universal (internal helper macro) */
#undef AC_APPLE_UNIVERSAL_BUILD

/* define this if ares is built for a big endian system */
#undef ARES_BIG_ENDIAN

/* when building as static part of libcurl */
#undef BUILDING_LIBCURL

/* Defined for build that exposes internal static functions for testing. */
#undef CARES_EXPOSE_STATICS

/* Defined for build with symbol hiding. */
#undef CARES_SYMBOL_HIDING

/* Definition to make a library symbol externally visible. */
#undef CARES_SYMBOL_SCOPE_EXTERN

/* Use resolver library to configure cares */
/* #undef CARES_USE_LIBRESOLV */

/* if a /etc/inet dir is being used */
#undef ETC_INET

/* Define to the type of arg 2 for gethostname. */
#define GETHOSTNAME_TYPE_ARG2 int

/* Define to the type qualifier of arg 1 for getnameinfo. */
#define GETNAMEINFO_QUAL_ARG1 

/* Define to the type of arg 1 for getnameinfo. */
#define GETNAMEINFO_TYPE_ARG1 struct sockaddr *

/* Define to the type of arg 2 for getnameinfo. */
#define GETNAMEINFO_TYPE_ARG2 socklen_t

/* Define to the type of args 4 and 6 for getnameinfo. */
#define GETNAMEINFO_TYPE_ARG46 socklen_t

/* Define to the type of arg 7 for getnameinfo. */
#define GETNAMEINFO_TYPE_ARG7 int

/* Specifies the number of arguments to getservbyport_r */
#define GETSERVBYPORT_R_ARGS 

/* Define to 1 if you have AF_INET6. */
#define HAVE_AF_INET6

/* Define to 1 if you have the <arpa/inet.h> header file. */
/* #undef HAVE_ARPA_INET_H */

/* Define to 1 if you have the <arpa/nameser_compat.h> header file. */
/* #undef HAVE_ARPA_NAMESER_COMPAT_H */

/* Define to 1 if you have the <arpa/nameser.h> header file. */
/* #undef HAVE_ARPA_NAMESER_H */

/* Define to 1 if you have the <assert.h> header file. */
#define HAVE_ASSERT_H

/* Define to 1 if you have the `bitncmp' function. */
/* #undef HAVE_BITNCMP */

/* Define to 1 if bool is an available type. */
#define HAVE_BOOL_T

/* Define to 1 if you have the clock_gettime function and monotonic timer. */
/* #undef HAVE_CLOCK_GETTIME_MONOTONIC */

/* Define to 1 if you have the closesocket function. */
#define HAVE_CLOSESOCKET

/* Define to 1 if you have the CloseSocket camel case function. */
/* #undef HAVE_CLOSESOCKET_CAMEL */

/* Define to 1 if you have the connect function. */
#define HAVE_CONNECT

/* define if the compiler supports basic C++11 syntax */
/* #undef HAVE_CXX11 */

/* Define to 1 if you have the <dlfcn.h> header file. */
/* #undef HAVE_DLFCN_H */

/* Define to 1 if you have the <errno.h> header file. */
#define HAVE_ERRNO_H

/* Define to 1 if you have the fcntl function. */
/* #undef HAVE_FCNTL */

/* Define to 1 if you have the <fcntl.h> header file. */
#define HAVE_FCNTL_H

/* Define to 1 if you have a working fcntl O_NONBLOCK function. */
/* #undef HAVE_FCNTL_O_NONBLOCK */

/* Define to 1 if you have the freeaddrinfo function. */
#define HAVE_FREEADDRINFO

/* Define to 1 if you have a working getaddrinfo function. */
#define HAVE_GETADDRINFO

/* Define to 1 if the getaddrinfo function is threadsafe. */
#define HAVE_GETADDRINFO_THREADSAFE

/* Define to 1 if you have the getenv function. */
#define HAVE_GETENV

/* Define to 1 if you have the gethostbyaddr function. */
#define HAVE_GETHOSTBYADDR

/* Define to 1 if you have the gethostbyname function. */
#define HAVE_GETHOSTBYNAME

/* Define to 1 if you have the gethostname function. */
#define HAVE_GETHOSTNAME

/* Define to 1 if you have the getnameinfo function. */
#define HAVE_GETNAMEINFO

/* Define to 1 if you have the getservbyport_r function. */
/* #undef HAVE_GETSERVBYPORT_R */

/* Define to 1 if you have the `gettimeofday' function. */
/* #undef HAVE_GETTIMEOFDAY */

/* Define to 1 if you have the `if_indextoname' function. */
/* #undef HAVE_IF_INDEXTONAME */

/* Define to 1 if you have a IPv6 capable working inet_net_pton function. */
/* #undef HAVE_INET_NET_PTON */

/* Define to 1 if you have a IPv6 capable working inet_ntop function. */
/* #undef HAVE_INET_NTOP */

/* Define to 1 if you have a IPv6 capable working inet_pton function. */
/* #undef HAVE_INET_PTON */

/* Define to 1 if you have the <inttypes.h> header file. */
#define HAVE_INTTYPES_H

/* Define to 1 if you have the ioctl function. */
/* #undef HAVE_IOCTL */

/* Define to 1 if you have the ioctlsocket function. */
#define HAVE_IOCTLSOCKET

/* Define to 1 if you have the IoctlSocket camel case function. */
/* #undef HAVE_IOCTLSOCKET_CAMEL */

/* Define to 1 if you have a working IoctlSocket camel case FIONBIO function.
   */
/* #undef HAVE_IOCTLSOCKET_CAMEL_FIONBIO */

/* Define to 1 if you have a working ioctlsocket FIONBIO function. */
#define HAVE_IOCTLSOCKET_FIONBIO

/* Define to 1 if you have a working ioctl FIONBIO function. */
/* #undef HAVE_IOCTL_FIONBIO */

/* Define to 1 if you have a working ioctl SIOCGIFADDR function. */
/* #undef HAVE_IOCTL_SIOCGIFADDR */

/* Define to 1 if you have the `resolve' library (-lresolve). */
/* #undef HAVE_LIBRESOLV */

/* Define to 1 if you have the <limits.h> header file. */
#define HAVE_LIMITS_H

/* if your compiler supports LL */
#define HAVE_LL

/* Define to 1 if the compiler supports the 'long long' data type. */
#define HAVE_LONGLONG

/* Define to 1 if you have the malloc.h header file. */
#define HAVE_MALLOC_H

/* Define to 1 if you have the memory.h header file. */
#define HAVE_MEMORY_H

/* Define to 1 if you have the MSG_NOSIGNAL flag. */
/* #undef HAVE_MSG_NOSIGNAL */

/* Define to 1 if you have the <netdb.h> header file. */
/* #undef HAVE_NETDB_H */

/* Define to 1 if you have the <netinet/in.h> header file. */
/* #undef HAVE_NETINET_IN_H */

/* Define to 1 if you have the <netinet/tcp.h> header file. */
/* #undef HAVE_NETINET_TCP_H */

/* Define to 1 if you have the <net/if.h> header file. */
/* #undef HAVE_NET_IF_H */

/* Define to 1 if you have PF_INET6. */
#define HAVE_PF_INET6

/* Define to 1 if you have the recv function. */
#define HAVE_RECV

/* Define to 1 if you have the recvfrom function. */
#define HAVE_RECVFROM

/* Define to 1 if you have the send function. */
#define HAVE_SEND

/* Define to 1 if you have the setsockopt function. */
#define HAVE_SETSOCKOPT

/* Define to 1 if you have a working setsockopt SO_NONBLOCK function. */
/* #undef HAVE_SETSOCKOPT_SO_NONBLOCK */

/* Define to 1 if you have the <signal.h> header file. */
#define HAVE_SIGNAL_H

/* Define to 1 if sig_atomic_t is an available typedef. */
#define HAVE_SIG_ATOMIC_T

/* Define to 1 if sig_atomic_t is already defined as volatile. */
/* #undef HAVE_SIG_ATOMIC_T_VOLATILE */

/* Define to 1 if your struct sockaddr_in6 has sin6_scope_id. */
#define HAVE_SOCKADDR_IN6_SIN6_SCOPE_ID

/* Define to 1 if you have the socket function. */
#define HAVE_SOCKET

/* Define to 1 if you have the <socket.h> header file. */
/* #undef HAVE_SOCKET_H */

/* Define to 1 if you have the <stdbool.h> header file. */
#define HAVE_STDBOOL_H

/* Define to 1 if you have the <stdint.h> header file. */
#define HAVE_STDINT_H

/* Define to 1 if you have the <stdlib.h> header file. */
#define HAVE_STDLIB_H

/* Define to 1 if you have the strcasecmp function. */
/* #undef HAVE_STRCASECMP */

/* Define to 1 if you have the strcmpi function. */
#define HAVE_STRCMPI

/* Define to 1 if you have the strdup function. */
#define HAVE_STRDUP

/* Define to 1 if you have the stricmp function. */
#define HAVE_STRICMP

/* Define to 1 if you have the <strings.h> header file. */
/* #undef HAVE_STRINGS_H */

/* Define to 1 if you have the <string.h> header file. */
#define HAVE_STRING_H

/* Define to 1 if you have the strncasecmp function. */
/* #undef HAVE_STRNCASECMP */

/* Define to 1 if you have the strncmpi function. */
/* #undef HAVE_STRNCMPI */

/* Define to 1 if you have the strnicmp function. */
#define HAVE_STRNICMP

/* Define to 1 if you have the <stropts.h> header file. */
/* #undef HAVE_STROPTS_H */

/* Define to 1 if you have struct addrinfo. */
#define HAVE_STRUCT_ADDRINFO

/* Define to 1 if you have struct in6_addr. */
#define HAVE_STRUCT_IN6_ADDR

/* Define to 1 if you have struct sockaddr_in6. */
#define HAVE_STRUCT_SOCKADDR_IN6

/* if struct sockaddr_storage is defined */
#define HAVE_STRUCT_SOCKADDR_STORAGE

/* Define to 1 if you have the timeval struct. */
#define HAVE_STRUCT_TIMEVAL

/* Define to 1 if you have the <sys/ioctl.h> header file. */
/* #undef HAVE_SYS_IOCTL_H */

/* Define to 1 if you have the <sys/param.h> header file. */
/* #undef HAVE_SYS_PARAM_H */

/* Define to 1 if you have the <sys/select.h> header file. */
/* #undef HAVE_SYS_SELECT_H */

/* Define to 1 if you have the <sys/socket.h> header file. */
/* #undef HAVE_SYS_SOCKET_H */

/* Define to 1 if you have the <sys/stat.h> header file. */
#define HAVE_SYS_STAT_H

/* Define to 1 if you have the <sys/time.h> header file. */
/* #undef HAVE_SYS_TIME_H */

/* Define to 1 if you have the <sys/types.h> header file. */
#define HAVE_SYS_TYPES_H

/* Define to 1 if you have the <sys/uio.h> header file. */
/* #undef HAVE_SYS_UIO_H */

/* Define to 1 if you have the <time.h> header file. */
#define HAVE_TIME_H

/* Define to 1 if you have the <unistd.h> header file. */
/* #undef HAVE_UNISTD_H */

/* Define to 1 if you have the windows.h header file. */
#define HAVE_WINDOWS_H

/* Define to 1 if you have the winsock2.h header file. */
#define HAVE_WINSOCK2_H

/* Define to 1 if you have the winsock.h header file. */
#define HAVE_WINSOCK_H

/* Define to 1 if you have the writev function. */
/* #undef HAVE_WRITEV */

/* Define to 1 if you have the ws2tcpip.h header file. */
#define HAVE_WS2TCPIP_H

/* Define if __system_property_get exists. */
/* #undef HAVE___SYSTEM_PROPERTY_GET */

/* Define to 1 if you need the malloc.h header file even with stdlib.h */
/* #undef NEED_MALLOC_H */

/* Define to 1 if you need the memory.h header file even with stdlib.h */
/* #undef NEED_MEMORY_H */

/* a suitable file/device to read random data from */
/* #undef RANDOM_FILE */

/* Define to the type qualifier pointed by arg 5 for recvfrom. */
#define RECVFROM_QUAL_ARG5 

/* Define to the type of arg 1 for recvfrom. */
#define RECVFROM_TYPE_ARG1 SOCKET

/* Define to the type pointed by arg 2 for recvfrom. */
#define RECVFROM_TYPE_ARG2 void *

/* Define to 1 if the type pointed by arg 2 for recvfrom is void. */
#define RECVFROM_TYPE_ARG2_IS_VOID 0

/* Define to the type of arg 3 for recvfrom. */
#define RECVFROM_TYPE_ARG3 int

/* Define to the type of arg 4 for recvfrom. */
#define RECVFROM_TYPE_ARG4 int

/* Define to the type pointed by arg 5 for recvfrom. */
#define RECVFROM_TYPE_ARG5 struct sockaddr *

/* Define to 1 if the type pointed by arg 5 for recvfrom is void. */
#define RECVFROM_TYPE_ARG5_IS_VOID 0

/* Define to the type pointed by arg 6 for recvfrom. */
#define RECVFROM_TYPE_ARG6 socklen_t *

/* Define to 1 if the type pointed by arg 6 for recvfrom is void. */
#define RECVFROM_TYPE_ARG6_IS_VOID 0

/* Define to the function return type for recvfrom. */
#define RECVFROM_TYPE_RETV int

/* Define to the type of arg 1 for recv. */
#define RECV_TYPE_ARG1 SOCKET

/* Define to the type of arg 2 for recv. */
#define RECV_TYPE_ARG2 void *

/* Define to the type of arg 3 for recv. */
#define RECV_TYPE_ARG3 int

/* Define to the type of arg 4 for recv. */
#define RECV_TYPE_ARG4 int

/* Define to the function return type for recv. */
#define RECV_TYPE_RETV int

/* Define as the return type of signal handlers (`int' or `void'). */
#define RETSIGTYPE 

/* Define to the type qualifier of arg 2 for send. */
#define SEND_QUAL_ARG2 

/* Define to the type of arg 1 for send. */
#define SEND_TYPE_ARG1 SOCKET

/* Define to the type of arg 2 for send. */
#define SEND_TYPE_ARG2 void *

/* Define to the type of arg 3 for send. */
#define SEND_TYPE_ARG3 int

/* Define to the type of arg 4 for send. */
#define SEND_TYPE_ARG4 int

/* Define to the function return type for send. */
#define SEND_TYPE_RETV int

/* Define to 1 if you can safely include both <sys/time.h> and <time.h>. */
/* #undef TIME_WITH_SYS_TIME */

/* Define to disable non-blocking sockets. */
#undef USE_BLOCKING_SOCKETS

/* Define to avoid automatic inclusion of winsock.h */
#undef WIN32_LEAN_AND_MEAN

/* Type to use in place of in_addr_t when system does not provide it. */
#undef in_addr_t

