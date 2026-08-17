#ifndef __res_state_defined
#define __res_state_defined 1

#include <sys/types.h>
#include <netinet/in.h>

/* res_state: the global state used by the resolver stub.  */
#define MAXNS			3	/* max # name servers we'll track */
#define MAXDFLSRCH		3	/* # default domain levels to try */
#define MAXDNSRCH		6	/* max # domains in search path */
#define MAXRESOLVSORT		10	/* number of net to sort on */

struct __res_state {
	int	retrans;		/* retransmition time interval */
	int	retry;			/* number of times to retransmit */
	unsigned long options;		/* option flags - see below. */
	int	nscount;		/* number of name servers */
	struct sockaddr_in
		nsaddr_list[MAXNS];	/* address of name server */
	unsigned short id;		/* current message id */
	/* 2 byte hole here.  */
	char	*dnsrch[MAXDNSRCH+1];	/* components of domain to search */
	char	defdname[256];		/* default domain (deprecated) */
	unsigned long pfcode;		/* RES_PRF_ flags - see below. */
	unsigned ndots:4;		/* threshold for initial abs. query */
	unsigned nsort:4;		/* number of elements in sort_list[] */
	unsigned ipv6_unavail:1;	/* connecting to IPv6 server failed */
	unsigned unused:23;
	struct {
		struct in_addr	addr;
		uint32_t	mask;
	} sort_list[MAXRESOLVSORT];
	/* 4 byte hole here on 64-bit architectures.  */
	void * __glibc_unused_qhook;
	void * __glibc_unused_rhook;
	int	res_h_errno;		/* last one set for this context */
	int	_vcsock;		/* PRIVATE: for res_send VC i/o */
	unsigned int _flags;		/* PRIVATE: see below */
	/* 4 byte hole here on 64-bit architectures.  */
	union {
		char	pad[52];	/* On an i386 this means 512b total. */
		struct {
			uint16_t		nscount;
			uint16_t		nsmap[MAXNS];
			int			nssocks[MAXNS];
			uint16_t		nscount6;
			uint16_t		nsinit;
			struct sockaddr_in6	*nsaddrs[MAXNS];
#ifdef _LIBC
			unsigned long long int __glibc_extension_index
			  __attribute__((packed));
#else
			unsigned int		__glibc_reserved[2];
#endif
		} _ext;
	} _u;
};

typedef struct __res_state *res_state;

#endif /* __res_state_defined */
