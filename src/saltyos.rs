//! SaltyOS libc bindings
//!
//! Raw FFI type definitions and extern declarations matching the SaltyOS
//! C library's installed headers (`lib/c/include`).

use crate::prelude::*;
use crate::c_schar;

pub type size_t = usize;
pub type ssize_t = isize;
pub type ptrdiff_t = isize;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type off_t = i64;
pub type mode_t = u16;
pub type pid_t = i32;
pub type uid_t = u32;
pub type gid_t = u32;
pub type time_t = i64;
pub type clockid_t = i32;
pub type speed_t = c_uint;
pub type tcflag_t = c_uint;
pub type cc_t = c_uchar;
pub type nfds_t = c_ulong;
pub type ino_t = u64;
pub type dev_t = u64;
pub type nlink_t = u64;
pub type blksize_t = i32;
pub type blkcnt_t = i64;
pub type suseconds_t = i64;
// <bits/pthreadtypes.h>: a thread is the address of the library's record
// for it, carried as an integer.
pub type pthread_t = c_ulong;
pub type pthread_key_t = c_uint;
pub type pthread_once_t = c_int;
pub type pthread_spinlock_t = c_int;
pub type key_t = c_int;
pub type timer_t = *mut c_void;
pub type fsblkcnt_t = c_ulong;
pub type fsfilcnt_t = c_ulong;
pub type reclen_t = c_ushort;
pub type u_char = c_uchar;
pub type u_short = c_ushort;
pub type u_int = c_uint;
pub type u_long = c_ulong;
pub type socklen_t = u32;
pub type sa_family_t = u16;
pub type in_addr_t = u32;
pub type in_port_t = u16;
pub type sighandler_t = usize;
pub type sig_t = sighandler_t;
pub type sig_atomic_t = i32;
// Clang's `wchar_t` is `int` on x86_64 and `unsigned int` on aarch64.
#[cfg(target_arch = "x86_64")]
pub type wchar_t = i32;
#[cfg(target_arch = "aarch64")]
pub type wchar_t = u32;
pub type wint_t = u32;
pub type rlim_t = c_ulong;
pub type clock_t = c_long;
pub type useconds_t = c_uint;
pub type id_t = c_long;
pub type sigset_t = c_ulong;
pub type sigval_t = sigval;
// <sys/wait.h> `idtype_t`: an enumeration with no negative member.
pub type idtype_t = c_uint;
pub type shmatt_t = c_ulong;
pub type msgqnum_t = c_ulong;
pub type msglen_t = c_ulong;
// <bits/locale_t.h>: the locale object handle, a pointer to an opaque object.
pub enum __locale_struct {}
pub type locale_t = *mut __locale_struct;

// FILE is opaque
pub enum FILE {}

// DIR is opaque
pub enum DIR {}

// =========================================================================
// Errno constants (<errno.h> — Linux numbering)
// =========================================================================

pub const EPERM: c_int = 1;
pub const ENOENT: c_int = 2;
pub const ESRCH: c_int = 3;
pub const EINTR: c_int = 4;
pub const EIO: c_int = 5;
pub const ENXIO: c_int = 6;
pub const E2BIG: c_int = 7;
pub const ENOEXEC: c_int = 8;
pub const EBADF: c_int = 9;
pub const ECHILD: c_int = 10;
pub const EAGAIN: c_int = 11;
pub const ENOMEM: c_int = 12;
pub const EACCES: c_int = 13;
pub const EFAULT: c_int = 14;
pub const EBUSY: c_int = 16;
pub const EEXIST: c_int = 17;
pub const EXDEV: c_int = 18;
pub const ENODEV: c_int = 19;
pub const ENOTDIR: c_int = 20;
pub const EISDIR: c_int = 21;
pub const EINVAL: c_int = 22;
pub const ENFILE: c_int = 23;
pub const EMFILE: c_int = 24;
pub const ENOTTY: c_int = 25;
pub const EFBIG: c_int = 27;
pub const ENOSPC: c_int = 28;
pub const ESPIPE: c_int = 29;
pub const EROFS: c_int = 30;
pub const EMLINK: c_int = 31;
pub const EPIPE: c_int = 32;
pub const EDOM: c_int = 33;
pub const ERANGE: c_int = 34;
pub const EDEADLK: c_int = 35;
pub const ENAMETOOLONG: c_int = 36;
pub const ENOLCK: c_int = 37;
pub const ENOSYS: c_int = 38;
pub const ENOTEMPTY: c_int = 39;
pub const ELOOP: c_int = 40;
pub const EWOULDBLOCK: c_int = EAGAIN;
pub const ENODATA: c_int = 61;
pub const ETIME: c_int = 62;
pub const EPROTO: c_int = 71;
pub const EMULTIHOP: c_int = 72;
pub const EBADMSG: c_int = 74;
pub const EOVERFLOW: c_int = 75;
pub const EILSEQ: c_int = 84;
pub const ENOTSOCK: c_int = 88;
pub const EOPNOTSUPP: c_int = 95;
pub const ENOTSUP: c_int = EOPNOTSUPP;
pub const EAFNOSUPPORT: c_int = 97;
pub const EADDRINUSE: c_int = 98;
pub const ECONNRESET: c_int = 104;
pub const ENOBUFS: c_int = 105;
pub const EISCONN: c_int = 106;
pub const ENOTCONN: c_int = 107;
pub const ETIMEDOUT: c_int = 110;
pub const ECONNREFUSED: c_int = 111;
pub const EALREADY: c_int = 114;
pub const EINPROGRESS: c_int = 115;
pub const ECANCELED: c_int = 125;
pub const EOWNERDEAD: c_int = 130;
pub const ENOTRECOVERABLE: c_int = 131;
pub const ECONNABORTED: c_int = 103;
pub const EHOSTUNREACH: c_int = 113;
pub const ENETUNREACH: c_int = 101;
pub const EPROTONOSUPPORT: c_int = 93;
pub const EDESTADDRREQ: c_int = 89;
pub const EMSGSIZE: c_int = 90;
pub const ETXTBSY: c_int = 26;
pub const EADDRNOTAVAIL: c_int = 99;
pub const ENETDOWN: c_int = 100;
pub const ESTALE: c_int = 116;
pub const EDQUOT: c_int = 122;
pub const ENOTBLK: c_int = 15;
pub const ENOMSG: c_int = 42;
pub const EIDRM: c_int = 43;
pub const ENOSTR: c_int = 60;
pub const ENOSR: c_int = 63;
pub const EREMOTE: c_int = 66;
pub const ENOLINK: c_int = 67;
pub const ENOTUNIQ: c_int = 76;
pub const EBADFD: c_int = 77;
pub const EFTYPE: c_int = 79;
pub const EUSERS: c_int = 87;
pub const EPROTOTYPE: c_int = 91;
pub const ENOPROTOOPT: c_int = 92;
pub const ESOCKTNOSUPPORT: c_int = 94;
pub const EPFNOSUPPORT: c_int = 96;
pub const ENETRESET: c_int = 102;
pub const ESHUTDOWN: c_int = 108;
pub const ETOOMANYREFS: c_int = 109;
pub const EHOSTDOWN: c_int = 112;
pub const ENOMEDIUM: c_int = 123;
pub const EMEDIUMTYPE: c_int = 124;
pub const EDEADLOCK: c_int = EDEADLK;

// =========================================================================
// open() flags, seek origins, *at() flags and advice (<fcntl.h>, <unistd.h>)
// =========================================================================

pub const O_RDONLY: c_int = 0x0000;
pub const O_WRONLY: c_int = 0x0001;
pub const O_RDWR: c_int = 0x0002;
pub const O_ACCMODE: c_int = 0x0003;
pub const O_CREAT: c_int = 0x0040;
pub const O_EXCL: c_int = 0x0080;
pub const O_NOCTTY: c_int = 0x0100;
pub const O_TRUNC: c_int = 0x0200;
pub const O_APPEND: c_int = 0x0400;
pub const O_NONBLOCK: c_int = 0x0800;
pub const O_DIRECTORY: c_int = 0x10000;
pub const O_NOFOLLOW: c_int = 0x20000;
pub const O_CLOEXEC: c_int = 0x80000i32;
pub const O_DSYNC: c_int = 0o10000;
pub const O_NOATIME: c_int = 0x40000;
pub const O_SYNC: c_int = 0o4010000;
pub const O_PATH: c_int = 0x200000;
pub const O_EXEC: c_int = O_PATH;
pub const O_SEARCH: c_int = O_PATH;
pub const O_RSYNC: c_int = 0x4000000;
pub const O_CLOFORK: c_int = 0x1000000;
pub const O_TTY_INIT: c_int = 0;
pub const O_NDELAY: c_int = O_NONBLOCK;

pub const SEEK_SET: c_int = 0;
pub const SEEK_CUR: c_int = 1;
pub const SEEK_END: c_int = 2;
pub const SEEK_DATA: c_int = 3;
pub const SEEK_HOLE: c_int = 4;

pub const AT_FDCWD: c_int = -100;
pub const AT_SYMLINK_NOFOLLOW: c_int = 0x100;
pub const AT_REMOVEDIR: c_int = 0x200;
pub const AT_SYMLINK_FOLLOW: c_int = 0x400;
pub const AT_EMPTY_PATH: c_int = 0x1000;
pub const AT_EACCESS: c_int = 0x10000;

pub const POSIX_FADV_NORMAL: c_int = 0;
pub const POSIX_FADV_RANDOM: c_int = 1;
pub const POSIX_FADV_SEQUENTIAL: c_int = 2;
pub const POSIX_FADV_WILLNEED: c_int = 3;
pub const POSIX_FADV_DONTNEED: c_int = 4;
pub const POSIX_FADV_NOREUSE: c_int = 5;

// =========================================================================
// Sockets (<sys/socket.h>, <netinet/in.h>, <netinet/tcp.h>)
// =========================================================================

pub const AF_UNSPEC: c_int = 0;
pub const AF_UNIX: c_int = 1;
pub const AF_LOCAL: c_int = AF_UNIX;
pub const AF_INET: c_int = 2;
pub const AF_INET6: c_int = 10;
pub const AF_NETLINK: c_int = 16;
pub const AF_ROUTE: c_int = 17;
pub const AF_LINK: c_int = 18;
pub const AF_MAX: c_int = 19;
pub const PF_UNSPEC: c_int = AF_UNSPEC;
pub const PF_UNIX: c_int = AF_UNIX;
pub const PF_LOCAL: c_int = AF_LOCAL;
pub const PF_INET: c_int = AF_INET;
pub const PF_INET6: c_int = AF_INET6;
pub const PF_NETLINK: c_int = AF_NETLINK;
pub const PF_ROUTE: c_int = AF_ROUTE;
pub const PF_LINK: c_int = AF_LINK;
pub const PF_MAX: c_int = AF_MAX;

pub const SOCK_STREAM: c_int = 1;
pub const SOCK_DGRAM: c_int = 2;
pub const SOCK_RAW: c_int = 3;
pub const SOCK_SEQPACKET: c_int = 5;
pub const SOCK_NONBLOCK: c_int = 0x800;
pub const SOCK_CLOEXEC: c_int = 0x80000;
pub const SOCK_CLOFORK: c_int = 0x1000000;

pub const SOL_SOCKET: c_int = 1;
pub const SO_DEBUG: c_int = 1;
pub const SO_REUSEADDR: c_int = 2;
pub const SO_TYPE: c_int = 3;
pub const SO_ERROR: c_int = 4;
pub const SO_DONTROUTE: c_int = 5;
pub const SO_BROADCAST: c_int = 6;
pub const SO_SNDBUF: c_int = 7;
pub const SO_RCVBUF: c_int = 8;
pub const SO_KEEPALIVE: c_int = 9;
pub const SO_OOBINLINE: c_int = 10;
pub const SO_LINGER: c_int = 13;
pub const SO_REUSEPORT: c_int = 15;
pub const SO_RCVLOWAT: c_int = 18;
pub const SO_SNDLOWAT: c_int = 19;
pub const SO_RCVTIMEO: c_int = 20;
pub const SO_SNDTIMEO: c_int = 21;
pub const SO_ACCEPTCONN: c_int = 30;
pub const SO_PROTOCOL: c_int = 38;
pub const SO_DOMAIN: c_int = 39;
pub const SO_TIMESTAMP: c_int = 0x400;
pub const SO_TS_CLOCK: c_int = 0x1017;
pub const SO_TS_MONOTONIC: c_int = 3;
pub const SOMAXCONN: c_int = 128;

pub const MSG_OOB: c_int = 0x1;
pub const MSG_PEEK: c_int = 0x2;
pub const MSG_DONTROUTE: c_int = 0x4;
pub const MSG_CTRUNC: c_int = 0x8;
pub const MSG_TRUNC: c_int = 0x20;
pub const MSG_DONTWAIT: c_int = 0x40;
pub const MSG_EOR: c_int = 0x80;
pub const MSG_WAITALL: c_int = 0x100;
pub const MSG_NOSIGNAL: c_int = 0x4000;
pub const MSG_CMSG_CLOEXEC: c_int = 0x40000000;
pub const MSG_CMSG_CLOFORK: c_int = 0x20000000;

pub const SHUT_RD: c_int = 0;
pub const SHUT_WR: c_int = 1;
pub const SHUT_RDWR: c_int = 2;

pub const SCM_RIGHTS: c_int = 1;
pub const SCM_TIMESTAMP: c_int = 2;

// <netinet/in.h>
pub const IPPROTO_IP: c_int = 0;
pub const IPPROTO_ICMP: c_int = 1;
pub const IPPROTO_IGMP: c_int = 2;
pub const IPPROTO_TCP: c_int = 6;
pub const IPPROTO_UDP: c_int = 17;
pub const IPPROTO_IPV6: c_int = 41;
pub const IPPROTO_ICMPV6: c_int = 58;
pub const IPPROTO_RAW: c_int = 255;

pub const IP_TOS: c_int = 1;
pub const IP_TTL: c_int = 2;
pub const IP_HDRINCL: c_int = 3;
pub const IP_OPTIONS: c_int = 4;
pub const IP_RECVTTL: c_int = 12;
pub const IP_MULTICAST_IF: c_int = 32;
pub const IP_MULTICAST_TTL: c_int = 33;
pub const IP_MULTICAST_LOOP: c_int = 34;
pub const IP_ADD_MEMBERSHIP: c_int = 35;
pub const IP_DROP_MEMBERSHIP: c_int = 36;

pub const IPV6_UNICAST_HOPS: c_int = 16;
pub const IPV6_MULTICAST_IF: c_int = 17;
pub const IPV6_MULTICAST_HOPS: c_int = 18;
pub const IPV6_MULTICAST_LOOP: c_int = 19;
pub const IPV6_JOIN_GROUP: c_int = 20;
pub const IPV6_LEAVE_GROUP: c_int = 21;
pub const IPV6_V6ONLY: c_int = 26;
pub const IPV6_ADD_MEMBERSHIP: c_int = IPV6_JOIN_GROUP;
pub const IPV6_DROP_MEMBERSHIP: c_int = IPV6_LEAVE_GROUP;

// Host byte order.
pub const INADDR_ANY: in_addr_t = 0;
pub const INADDR_LOOPBACK: in_addr_t = 0x7f000001;
pub const INADDR_BROADCAST: in_addr_t = 0xffffffff;
pub const INADDR_NONE: in_addr_t = 0xffffffff;

pub const INET_ADDRSTRLEN: usize = 16;
pub const INET6_ADDRSTRLEN: usize = 46;

pub const IN6ADDR_ANY_INIT: in6_addr = in6_addr { s6_addr: [0; 16] };
pub const IN6ADDR_LOOPBACK_INIT: in6_addr = in6_addr {
    s6_addr: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
};

// <netinet/tcp.h>
pub const TCP_NODELAY: c_int = 1;
pub const TCP_MAXSEG: c_int = 2;
pub const TCP_KEEPIDLE: c_int = 4;
pub const TCP_KEEPINTVL: c_int = 5;
pub const TCP_KEEPCNT: c_int = 6;

// =========================================================================
// File modes and flags (<sys/stat.h>)
// =========================================================================

pub const S_IFMT: mode_t = 0o170000;
pub const S_IFSOCK: mode_t = 0o140000;
pub const S_IFLNK: mode_t = 0o120000;
pub const S_IFREG: mode_t = 0o100000;
pub const S_IFBLK: mode_t = 0o060000;
pub const S_IFDIR: mode_t = 0o040000;
pub const S_IFCHR: mode_t = 0o020000;
pub const S_IFIFO: mode_t = 0o010000;
pub const S_IFWHT: mode_t = 0o160000;

pub const S_IRWXU: mode_t = 0o700;
pub const S_IRUSR: mode_t = 0o400;
pub const S_IWUSR: mode_t = 0o200;
pub const S_IXUSR: mode_t = 0o100;
pub const S_IRWXG: mode_t = 0o070;
pub const S_IRGRP: mode_t = 0o040;
pub const S_IWGRP: mode_t = 0o020;
pub const S_IXGRP: mode_t = 0o010;
pub const S_IRWXO: mode_t = 0o007;
pub const S_IROTH: mode_t = 0o004;
pub const S_IWOTH: mode_t = 0o002;
pub const S_IXOTH: mode_t = 0o001;
pub const S_ISUID: mode_t = 0o4000;
pub const S_ISGID: mode_t = 0o2000;
pub const S_ISVTX: mode_t = 0o1000;
pub const DEFFILEMODE: mode_t = S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP | S_IROTH | S_IWOTH;
pub const ACCESSPERMS: mode_t = S_IRWXU | S_IRWXG | S_IRWXO;
pub const ALLPERMS: mode_t = S_ISUID | S_ISGID | S_ISVTX | S_IRWXU | S_IRWXG | S_IRWXO;

// st_flags
pub const UF_SETTABLE: c_uint = 0x0000ffff;
pub const UF_NODUMP: c_uint = 0x00000001;
pub const UF_IMMUTABLE: c_uint = 0x00000002;
pub const UF_APPEND: c_uint = 0x00000004;
pub const UF_OPAQUE: c_uint = 0x00000008;
pub const UF_NOUNLINK: c_uint = 0x00000010;
pub const UF_SYSTEM: c_uint = 0x00000080;
pub const UF_ARCHIVE: c_uint = 0x00000800;
pub const UF_READONLY: c_uint = 0x00001000;
pub const UF_HIDDEN: c_uint = 0x00008000;
pub const SF_SETTABLE: c_uint = 0xffff0000;
pub const SF_ARCHIVED: c_uint = 0x00010000;
pub const SF_IMMUTABLE: c_uint = 0x00020000;
pub const SF_APPEND: c_uint = 0x00040000;
pub const SF_NOUNLINK: c_uint = 0x00100000;

pub const UTIME_NOW: c_long = (1 << 30) - 1;
pub const UTIME_OMIT: c_long = (1 << 30) - 2;

// =========================================================================
// Memory mappings (<sys/mman.h>)
// =========================================================================

pub const PROT_NONE: c_int = 0x0;
pub const PROT_READ: c_int = 0x1;
pub const PROT_WRITE: c_int = 0x2;
pub const PROT_EXEC: c_int = 0x4;

pub const MAP_SHARED: c_int = 0x01;
pub const MAP_PRIVATE: c_int = 0x02;
pub const MAP_FIXED: c_int = 0x10;
pub const MAP_ANONYMOUS: c_int = 0x20;
pub const MAP_ANON: c_int = MAP_ANONYMOUS;
pub const MAP_STACK: c_int = 0x20000;
pub const MAP_FIXED_NOREPLACE: c_int = 0x100000;
pub const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

pub const MS_ASYNC: c_int = 1;
pub const MS_INVALIDATE: c_int = 2;
pub const MS_SYNC: c_int = 4;

pub const MCL_CURRENT: c_int = 1;
pub const MCL_FUTURE: c_int = 2;
pub const MCL_ONFAULT: c_int = 4;

pub const MADV_NORMAL: c_int = 0;
pub const MADV_RANDOM: c_int = 1;
pub const MADV_SEQUENTIAL: c_int = 2;
pub const MADV_WILLNEED: c_int = 3;
pub const MADV_DONTNEED: c_int = 4;
pub const MADV_FREE: c_int = 8;

pub const POSIX_MADV_NORMAL: c_int = 0;
pub const POSIX_MADV_RANDOM: c_int = 1;
pub const POSIX_MADV_SEQUENTIAL: c_int = 2;
pub const POSIX_MADV_WILLNEED: c_int = 3;
pub const POSIX_MADV_DONTNEED: c_int = 4;

// =========================================================================
// Readiness (<poll.h>, <sys/epoll.h>)
// =========================================================================

pub const POLLIN: c_short = 0x001;
pub const POLLPRI: c_short = 0x002;
pub const POLLOUT: c_short = 0x004;
pub const POLLERR: c_short = 0x008;
pub const POLLHUP: c_short = 0x010;
pub const POLLNVAL: c_short = 0x020;
pub const POLLRDNORM: c_short = 0x040;
pub const POLLRDBAND: c_short = 0x080;
pub const POLLWRNORM: c_short = 0x100;
pub const POLLWRBAND: c_short = 0x200;
pub const INFTIM: c_int = -1;

pub const EPOLLIN: c_int = 0x001;
pub const EPOLLPRI: c_int = 0x002;
pub const EPOLLOUT: c_int = 0x004;
pub const EPOLLERR: c_int = 0x008;
pub const EPOLLHUP: c_int = 0x010;
pub const EPOLLRDNORM: c_int = 0x040;
pub const EPOLLRDBAND: c_int = 0x080;
pub const EPOLLWRNORM: c_int = 0x100;
pub const EPOLLWRBAND: c_int = 0x200;
pub const EPOLLRDHUP: c_int = 0x2000;
pub const EPOLLET: c_int = 1 << 31;
pub const EPOLLONESHOT: c_int = 1 << 30;
pub const EPOLL_CTL_ADD: c_int = 1;
pub const EPOLL_CTL_DEL: c_int = 2;
pub const EPOLL_CTL_MOD: c_int = 3;
pub const EPOLL_CLOEXEC: c_int = 0x80000;

// =========================================================================
// Signal constants (<signal.h>)
// =========================================================================

pub const SIGHUP: c_int = 1;
pub const SIGINT: c_int = 2;
pub const SIGQUIT: c_int = 3;
pub const SIGILL: c_int = 4;
pub const SIGTRAP: c_int = 5;
pub const SIGABRT: c_int = 6;
pub const SIGIOT: c_int = SIGABRT;
pub const SIGBUS: c_int = 7;
pub const SIGFPE: c_int = 8;
pub const SIGKILL: c_int = 9;
pub const SIGUSR1: c_int = 10;
pub const SIGSEGV: c_int = 11;
pub const SIGUSR2: c_int = 12;
pub const SIGPIPE: c_int = 13;
pub const SIGALRM: c_int = 14;
pub const SIGTERM: c_int = 15;
pub const SIGSTKFLT: c_int = 16;
pub const SIGCHLD: c_int = 17;
pub const SIGCONT: c_int = 18;
pub const SIGSTOP: c_int = 19;
pub const SIGTSTP: c_int = 20;
pub const SIGTTIN: c_int = 21;
pub const SIGTTOU: c_int = 22;
pub const SIGURG: c_int = 23;
pub const SIGXCPU: c_int = 24;
pub const SIGXFSZ: c_int = 25;
pub const SIGVTALRM: c_int = 26;
pub const SIGPROF: c_int = 27;
pub const SIGWINCH: c_int = 28;
pub const SIGIO: c_int = 29;
pub const SIGPOLL: c_int = SIGIO;
pub const SIGPWR: c_int = 30;
pub const SIGINFO: c_int = SIGPWR;
pub const SIGSYS: c_int = 31;
pub const SIGUNUSED: c_int = SIGSYS;
pub const SIGRTMIN: c_int = 32;
pub const SIGRTMAX: c_int = 63;
pub const NSIG: c_int = 64;
pub const _NSIG: c_int = NSIG;
pub const SIG2STR_MAX: usize = 32;

pub const SI_USER: c_int = 0;
pub const SI_KERNEL: c_int = 0x80;
pub const SI_QUEUE: c_int = -1;
pub const SI_TIMER: c_int = -2;
pub const SI_MESGQ: c_int = -3;
pub const SI_ASYNCIO: c_int = -4;
pub const SI_SIGIO: c_int = -5;
pub const SI_TKILL: c_int = -6;
pub const CLD_EXITED: c_int = 1;
pub const CLD_KILLED: c_int = 2;
pub const CLD_DUMPED: c_int = 3;
pub const CLD_TRAPPED: c_int = 4;
pub const CLD_STOPPED: c_int = 5;
pub const CLD_CONTINUED: c_int = 6;
pub const SEGV_MAPERR: c_int = 1;
pub const SEGV_ACCERR: c_int = 2;
pub const BUS_ADRALN: c_int = 1;
pub const BUS_ADRERR: c_int = 2;
pub const BUS_OBJERR: c_int = 3;
pub const ILL_ILLOPC: c_int = 1;
pub const ILL_ILLOPN: c_int = 2;
pub const ILL_ILLADR: c_int = 3;
pub const ILL_ILLTRP: c_int = 4;
pub const ILL_PRVOPC: c_int = 5;
pub const ILL_PRVREG: c_int = 6;
pub const ILL_COPROC: c_int = 7;
pub const ILL_BADSTK: c_int = 8;
pub const FPE_INTDIV: c_int = 1;
pub const FPE_INTOVF: c_int = 2;
pub const FPE_FLTDIV: c_int = 3;
pub const FPE_FLTOVF: c_int = 4;
pub const FPE_FLTUND: c_int = 5;
pub const FPE_FLTRES: c_int = 6;
pub const FPE_FLTINV: c_int = 7;
pub const FPE_FLTSUB: c_int = 8;
pub const TRAP_BRKPT: c_int = 1;
pub const TRAP_TRACE: c_int = 2;
pub const POLL_IN: c_int = 1;
pub const POLL_OUT: c_int = 2;
pub const POLL_MSG: c_int = 3;
pub const POLL_ERR: c_int = 4;
pub const POLL_PRI: c_int = 5;
pub const POLL_HUP: c_int = 6;

pub const SIGEV_SIGNAL: c_int = 0;
pub const SIGEV_NONE: c_int = 1;
pub const SIGEV_THREAD: c_int = 2;
pub const SIGEV_THREAD_ID: c_int = 4;

pub const SIG_DFL: sighandler_t = 0;
pub const SIG_IGN: sighandler_t = 1;
pub const SIG_ERR: sighandler_t = !0;

// SA_* flags (<signal.h>)
pub const SA_NOCLDSTOP: c_int = 0x00000001;
pub const SA_NOCLDWAIT: c_int = 0x00000002;
pub const SA_SIGINFO: c_int = 0x00000004;
pub const SA_ONSTACK: c_int = 0x08000000;
pub const SA_RESTART: c_int = 0x10000000;
pub const SA_NODEFER: c_int = 0x40000000;
pub const SA_RESETHAND: c_int = 0x80000000u32 as c_int;
pub const SA_NOMASK: c_int = SA_NODEFER;
pub const SA_ONESHOT: c_int = SA_RESETHAND;

pub const SIG_BLOCK: c_int = 0;
pub const SIG_UNBLOCK: c_int = 1;
pub const SIG_SETMASK: c_int = 2;

#[cfg(target_arch = "x86_64")]
pub const MINSIGSTKSZ: usize = 2048;
#[cfg(target_arch = "x86_64")]
pub const SIGSTKSZ: usize = 8192;
#[cfg(target_arch = "aarch64")]
pub const MINSIGSTKSZ: usize = 5120;
#[cfg(target_arch = "aarch64")]
pub const SIGSTKSZ: usize = 16384;
pub const SS_ONSTACK: c_int = 1;
pub const SS_DISABLE: c_int = 2;

// =========================================================================
// Clocks and timers (<time.h>, <sys/time.h>)
// =========================================================================

pub const CLOCK_REALTIME: clockid_t = 0;
pub const CLOCK_MONOTONIC: clockid_t = 1;
pub const CLOCK_PROCESS_CPUTIME_ID: clockid_t = 2;
pub const CLOCK_THREAD_CPUTIME_ID: clockid_t = 3;
pub const TIMER_ABSTIME: c_int = 1;
pub const TIME_UTC: c_int = 1;
pub const TIME_MONOTONIC: c_int = 2;
pub const TIME_ACTIVE: c_int = 3;
pub const TIME_THREAD_ACTIVE: c_int = 4;
pub const CLOCKS_PER_SEC: clock_t = 1_000_000;
pub const CLK_TCK: clock_t = 100;
pub const ITIMER_REAL: c_int = 0;
pub const ITIMER_VIRTUAL: c_int = 1;
pub const ITIMER_PROF: c_int = 2;

// =========================================================================
// fcntl() commands and descriptor flags (<fcntl.h>)
// =========================================================================

pub const F_DUPFD: c_int = 0;
pub const F_GETFD: c_int = 1;
pub const F_SETFD: c_int = 2;
pub const F_GETFL: c_int = 3;
pub const F_SETFL: c_int = 4;
pub const F_GETLK: c_int = 5;
pub const F_SETLK: c_int = 6;
pub const F_SETLKW: c_int = 7;
pub const F_SETOWN: c_int = 8;
pub const F_GETOWN: c_int = 9;
pub const F_OFD_GETLK: c_int = 36;
pub const F_OFD_SETLK: c_int = 37;
pub const F_OFD_SETLKW: c_int = 38;
pub const F_DUPFD_CLOEXEC: c_int = 1030;
pub const F_DUPFD_CLOFORK: c_int = 1036;
pub const FD_CLOEXEC: c_int = 1;
pub const FD_CLOFORK: c_int = 2;

pub const F_RDLCK: c_int = 0;
pub const F_WRLCK: c_int = 1;
pub const F_UNLCK: c_int = 2;

// =========================================================================
// Process constants (<sys/wait.h>, <stdlib.h>, <sched.h>, <spawn.h>)
// =========================================================================

pub const WNOHANG: c_int = 0x00000001;
pub const WUNTRACED: c_int = 0x00000002;
pub const WSTOPPED: c_int = WUNTRACED;
pub const WEXITED: c_int = 0x00000004;
pub const WCONTINUED: c_int = 0x00000008;
pub const WNOWAIT: c_int = 0x01000000;

pub const P_ALL: idtype_t = 0;
pub const P_PID: idtype_t = 1;
pub const P_PGID: idtype_t = 2;

pub const EXIT_SUCCESS: c_int = 0;
pub const EXIT_FAILURE: c_int = 1;

pub const SCHED_OTHER: c_int = 0;
pub const SCHED_FIFO: c_int = 1;
pub const SCHED_RR: c_int = 2;

pub const POSIX_SPAWN_RESETIDS: c_short = 0x01;
pub const POSIX_SPAWN_SETPGROUP: c_short = 0x02;
pub const POSIX_SPAWN_SETSIGDEF: c_short = 0x04;
pub const POSIX_SPAWN_SETSIGMASK: c_short = 0x08;
pub const POSIX_SPAWN_SETSCHEDPARAM: c_short = 0x10;
pub const POSIX_SPAWN_SETSCHEDULER: c_short = 0x20;
pub const POSIX_SPAWN_SETSID: c_short = 0x80;

// =========================================================================
// <unistd.h>: versions, options and the sysconf, pathconf and confstr names
// =========================================================================

// STDIN_FILENO, STDOUT_FILENO, STDERR_FILENO are provided by src/new/saltyos/unistd.rs

pub const _POSIX_VERSION: c_long = 202405;
pub const _POSIX2_VERSION: c_long = 202405;
pub const _XOPEN_VERSION: c_long = 800;
pub const _POSIX_ADVISORY_INFO: c_long = 202405;
pub const _POSIX_ASYNCHRONOUS_IO: c_long = 202405;
pub const _POSIX_BARRIERS: c_long = 202405;
pub const _POSIX_CHOWN_RESTRICTED: c_long = 202405;
pub const _POSIX_CLOCK_SELECTION: c_long = 202405;
pub const _POSIX_CPUTIME: c_long = 202405;
pub const _POSIX_DEVICE_CONTROL: c_long = -1;
pub const _POSIX_FSYNC: c_long = 202405;
pub const _POSIX_IPV6: c_long = 202405;
pub const _POSIX_JOB_CONTROL: c_long = 202405;
pub const _POSIX_MAPPED_FILES: c_long = 202405;
pub const _POSIX_MEMLOCK: c_long = 202405;
pub const _POSIX_MEMLOCK_RANGE: c_long = 202405;
pub const _POSIX_MEMORY_PROTECTION: c_long = 202405;
pub const _POSIX_MESSAGE_PASSING: c_long = -1;
pub const _POSIX_MONOTONIC_CLOCK: c_long = 202405;
pub const _POSIX_NO_TRUNC: c_long = 202405;
pub const _POSIX_PRIORITIZED_IO: c_long = -1;
pub const _POSIX_PRIORITY_SCHEDULING: c_long = 202405;
pub const _POSIX_RAW_SOCKETS: c_long = 202405;
pub const _POSIX_READER_WRITER_LOCKS: c_long = 202405;
pub const _POSIX_REALTIME_SIGNALS: c_long = 202405;
pub const _POSIX_REGEXP: c_long = 202405;
pub const _POSIX_SAVED_IDS: c_long = 202405;
pub const _POSIX_SEMAPHORES: c_long = 202405;
pub const _POSIX_SHARED_MEMORY_OBJECTS: c_long = 202405;
pub const _POSIX_SHELL: c_long = 202405;
pub const _POSIX_SPAWN: c_long = 202405;
pub const _POSIX_SPIN_LOCKS: c_long = 202405;
pub const _POSIX_SPORADIC_SERVER: c_long = -1;
pub const _POSIX_SYNCHRONIZED_IO: c_long = 202405;
pub const _POSIX_THREAD_ATTR_STACKADDR: c_long = 202405;
pub const _POSIX_THREAD_ATTR_STACKSIZE: c_long = 202405;
pub const _POSIX_THREAD_CPUTIME: c_long = 202405;
pub const _POSIX_THREAD_PRIO_INHERIT: c_long = -1;
pub const _POSIX_THREAD_PRIO_PROTECT: c_long = -1;
pub const _POSIX_THREAD_PRIORITY_SCHEDULING: c_long = 202405;
pub const _POSIX_THREAD_PROCESS_SHARED: c_long = 202405;
pub const _POSIX_THREAD_ROBUST_PRIO_INHERIT: c_long = -1;
pub const _POSIX_THREAD_ROBUST_PRIO_PROTECT: c_long = -1;
pub const _POSIX_THREAD_SAFE_FUNCTIONS: c_long = 202405;
pub const _POSIX_THREAD_SPORADIC_SERVER: c_long = -1;
pub const _POSIX_THREADS: c_long = 202405;
pub const _POSIX_TIMEOUTS: c_long = 202405;
pub const _POSIX_TIMERS: c_long = 202405;
pub const _POSIX_TYPED_MEMORY_OBJECTS: c_long = -1;
pub const _POSIX_V8_ILP32_OFF32: c_long = -1;
pub const _POSIX_V8_ILP32_OFFBIG: c_long = -1;
pub const _POSIX_V8_LP64_OFF64: c_long = 202405;
pub const _POSIX_V8_LPBIG_OFFBIG: c_long = -1;
pub const _POSIX_V7_ILP32_OFF32: c_long = -1;
pub const _POSIX_V7_ILP32_OFFBIG: c_long = -1;
pub const _POSIX_V7_LP64_OFF64: c_long = 202405;
pub const _POSIX_V7_LPBIG_OFFBIG: c_long = -1;
pub const _POSIX2_C_BIND: c_long = 202405;
pub const _POSIX2_C_DEV: c_long = -1;
pub const _POSIX2_CHAR_TERM: c_long = -1;
pub const _POSIX2_FORT_DEV: c_long = -1;
pub const _POSIX2_FORT_RUN: c_long = -1;
pub const _POSIX2_LOCALEDEF: c_long = -1;
pub const _POSIX2_SW_DEV: c_long = -1;
pub const _POSIX2_UPE: c_long = -1;
pub const _XOPEN_CRYPT: c_long = 202405;
pub const _XOPEN_ENH_I18N: c_long = 202405;
pub const _XOPEN_REALTIME: c_long = -1;
pub const _XOPEN_REALTIME_THREADS: c_long = -1;
pub const _XOPEN_SHM: c_long = 202405;
pub const _XOPEN_UNIX: c_long = 202405;
pub const _XOPEN_UUCP: c_long = -1;
pub const _POSIX_VDISABLE: cc_t = 0;

// lockf() commands
pub const F_ULOCK: c_int = 0;
pub const F_LOCK: c_int = 1;
pub const F_TLOCK: c_int = 2;
pub const F_TEST: c_int = 3;

pub const POSIX_CLOSE_RESTART: c_int = 0;

pub const _CS_PATH: c_int = 0;
pub const _CS_POSIX_V6_WIDTH_RESTRICTED_ENVS: c_int = 1;
pub const _CS_POSIX_V7_WIDTH_RESTRICTED_ENVS: c_int = 5;
pub const _CS_POSIX_V8_WIDTH_RESTRICTED_ENVS: c_int = 6;
pub const _CS_POSIX_V7_ILP32_OFF32_CFLAGS: c_int = 1132;
pub const _CS_POSIX_V7_ILP32_OFF32_LDFLAGS: c_int = 1133;
pub const _CS_POSIX_V7_ILP32_OFF32_LIBS: c_int = 1134;
pub const _CS_POSIX_V7_ILP32_OFF32_LINTFLAGS: c_int = 1135;
pub const _CS_POSIX_V7_ILP32_OFFBIG_CFLAGS: c_int = 1136;
pub const _CS_POSIX_V7_ILP32_OFFBIG_LDFLAGS: c_int = 1137;
pub const _CS_POSIX_V7_ILP32_OFFBIG_LIBS: c_int = 1138;
pub const _CS_POSIX_V7_ILP32_OFFBIG_LINTFLAGS: c_int = 1139;
pub const _CS_POSIX_V7_LP64_OFF64_CFLAGS: c_int = 1140;
pub const _CS_POSIX_V7_LP64_OFF64_LDFLAGS: c_int = 1141;
pub const _CS_POSIX_V7_LP64_OFF64_LIBS: c_int = 1142;
pub const _CS_POSIX_V7_LP64_OFF64_LINTFLAGS: c_int = 1143;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_CFLAGS: c_int = 1144;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_LDFLAGS: c_int = 1145;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_LIBS: c_int = 1146;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_LINTFLAGS: c_int = 1147;
pub const _CS_V6_ENV: c_int = 1148;
pub const _CS_V7_ENV: c_int = 1149;
pub const _CS_POSIX_V8_ILP32_OFF32_CFLAGS: c_int = 1150;
pub const _CS_POSIX_V8_ILP32_OFF32_LDFLAGS: c_int = 1151;
pub const _CS_POSIX_V8_ILP32_OFF32_LIBS: c_int = 1152;
pub const _CS_POSIX_V8_ILP32_OFFBIG_CFLAGS: c_int = 1153;
pub const _CS_POSIX_V8_ILP32_OFFBIG_LDFLAGS: c_int = 1154;
pub const _CS_POSIX_V8_ILP32_OFFBIG_LIBS: c_int = 1155;
pub const _CS_POSIX_V8_LP64_OFF64_CFLAGS: c_int = 1156;
pub const _CS_POSIX_V8_LP64_OFF64_LDFLAGS: c_int = 1157;
pub const _CS_POSIX_V8_LP64_OFF64_LIBS: c_int = 1158;
pub const _CS_POSIX_V8_LPBIG_OFFBIG_CFLAGS: c_int = 1159;
pub const _CS_POSIX_V8_LPBIG_OFFBIG_LDFLAGS: c_int = 1160;
pub const _CS_POSIX_V8_LPBIG_OFFBIG_LIBS: c_int = 1161;
pub const _CS_POSIX_V8_THREADS_CFLAGS: c_int = 1162;
pub const _CS_POSIX_V8_THREADS_LDFLAGS: c_int = 1163;
pub const _CS_V8_ENV: c_int = 1164;
pub const _CS_POSIX_V7_THREADS_CFLAGS: c_int = 1165;
pub const _CS_POSIX_V7_THREADS_LDFLAGS: c_int = 1166;

pub const _PC_LINK_MAX: c_int = 0;
pub const _PC_MAX_CANON: c_int = 1;
pub const _PC_MAX_INPUT: c_int = 2;
pub const _PC_NAME_MAX: c_int = 3;
pub const _PC_PATH_MAX: c_int = 4;
pub const _PC_PIPE_BUF: c_int = 5;
pub const _PC_CHOWN_RESTRICTED: c_int = 6;
pub const _PC_NO_TRUNC: c_int = 7;
pub const _PC_VDISABLE: c_int = 8;
pub const _PC_SYNC_IO: c_int = 9;
pub const _PC_ASYNC_IO: c_int = 10;
pub const _PC_PRIO_IO: c_int = 11;
pub const _PC_SOCK_MAXBUF: c_int = 12;
pub const _PC_FILESIZEBITS: c_int = 13;
pub const _PC_REC_INCR_XFER_SIZE: c_int = 14;
pub const _PC_REC_MAX_XFER_SIZE: c_int = 15;
pub const _PC_REC_MIN_XFER_SIZE: c_int = 16;
pub const _PC_REC_XFER_ALIGN: c_int = 17;
pub const _PC_ALLOC_SIZE_MIN: c_int = 18;
pub const _PC_SYMLINK_MAX: c_int = 19;
pub const _PC_2_SYMLINKS: c_int = 20;
pub const _PC_TIMESTAMP_RESOLUTION: c_int = 21;
pub const _PC_TEXTDOMAIN_MAX: c_int = 22;
pub const _PC_FALLOC: c_int = 23;

pub const _SC_ARG_MAX: c_int = 0;
pub const _SC_CHILD_MAX: c_int = 1;
pub const _SC_CLK_TCK: c_int = 2;
pub const _SC_NGROUPS_MAX: c_int = 3;
pub const _SC_OPEN_MAX: c_int = 4;
pub const _SC_STREAM_MAX: c_int = 5;
pub const _SC_TZNAME_MAX: c_int = 6;
pub const _SC_JOB_CONTROL: c_int = 7;
pub const _SC_SAVED_IDS: c_int = 8;
pub const _SC_REALTIME_SIGNALS: c_int = 9;
pub const _SC_PRIORITY_SCHEDULING: c_int = 10;
pub const _SC_TIMERS: c_int = 11;
pub const _SC_ASYNCHRONOUS_IO: c_int = 12;
pub const _SC_PRIORITIZED_IO: c_int = 13;
pub const _SC_SYNCHRONIZED_IO: c_int = 14;
pub const _SC_FSYNC: c_int = 15;
pub const _SC_MAPPED_FILES: c_int = 16;
pub const _SC_MEMLOCK: c_int = 17;
pub const _SC_MEMLOCK_RANGE: c_int = 18;
pub const _SC_MEMORY_PROTECTION: c_int = 19;
pub const _SC_MESSAGE_PASSING: c_int = 20;
pub const _SC_SEMAPHORES: c_int = 21;
pub const _SC_SHARED_MEMORY_OBJECTS: c_int = 22;
pub const _SC_AIO_LISTIO_MAX: c_int = 23;
pub const _SC_AIO_MAX: c_int = 24;
pub const _SC_AIO_PRIO_DELTA_MAX: c_int = 25;
pub const _SC_DELAYTIMER_MAX: c_int = 26;
pub const _SC_MQ_OPEN_MAX: c_int = 27;
pub const _SC_MQ_PRIO_MAX: c_int = 28;
pub const _SC_VERSION: c_int = 29;
pub const _SC_PAGESIZE: c_int = 30;
pub const _SC_PAGE_SIZE: c_int = _SC_PAGESIZE;
pub const _SC_RTSIG_MAX: c_int = 31;
pub const _SC_SEM_NSEMS_MAX: c_int = 32;
pub const _SC_SEM_VALUE_MAX: c_int = 33;
pub const _SC_SIGQUEUE_MAX: c_int = 34;
pub const _SC_TIMER_MAX: c_int = 35;
pub const _SC_BC_BASE_MAX: c_int = 36;
pub const _SC_BC_DIM_MAX: c_int = 37;
pub const _SC_BC_SCALE_MAX: c_int = 38;
pub const _SC_BC_STRING_MAX: c_int = 39;
pub const _SC_COLL_WEIGHTS_MAX: c_int = 40;
pub const _SC_EXPR_NEST_MAX: c_int = 42;
pub const _SC_LINE_MAX: c_int = 43;
pub const _SC_RE_DUP_MAX: c_int = 44;
pub const _SC_2_VERSION: c_int = 46;
pub const _SC_2_C_BIND: c_int = 47;
pub const _SC_2_C_DEV: c_int = 48;
pub const _SC_2_FORT_DEV: c_int = 49;
pub const _SC_2_FORT_RUN: c_int = 50;
pub const _SC_2_SW_DEV: c_int = 51;
pub const _SC_2_LOCALEDEF: c_int = 52;
pub const _SC_IOV_MAX: c_int = 60;
pub const _SC_THREADS: c_int = 67;
pub const _SC_THREAD_SAFE_FUNCTIONS: c_int = 68;
pub const _SC_GETGR_R_SIZE_MAX: c_int = 69;
pub const _SC_GETPW_R_SIZE_MAX: c_int = 70;
pub const _SC_LOGIN_NAME_MAX: c_int = 71;
pub const _SC_TTY_NAME_MAX: c_int = 72;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: c_int = 73;
pub const _SC_THREAD_KEYS_MAX: c_int = 74;
pub const _SC_THREAD_STACK_MIN: c_int = 75;
pub const _SC_THREAD_THREADS_MAX: c_int = 76;
pub const _SC_THREAD_ATTR_STACKADDR: c_int = 77;
pub const _SC_THREAD_ATTR_STACKSIZE: c_int = 78;
pub const _SC_THREAD_PRIORITY_SCHEDULING: c_int = 79;
pub const _SC_THREAD_PRIO_INHERIT: c_int = 80;
pub const _SC_THREAD_PRIO_PROTECT: c_int = 81;
pub const _SC_THREAD_PROCESS_SHARED: c_int = 82;
pub const _SC_NPROCESSORS_CONF: c_int = 83;
pub const _SC_NPROCESSORS_ONLN: c_int = 84;
pub const _SC_PHYS_PAGES: c_int = 85;
pub const _SC_AVPHYS_PAGES: c_int = 86;
pub const _SC_ATEXIT_MAX: c_int = 87;
pub const _SC_XOPEN_VERSION: c_int = 89;
pub const _SC_XOPEN_UNIX: c_int = 91;
pub const _SC_XOPEN_CRYPT: c_int = 92;
pub const _SC_XOPEN_ENH_I18N: c_int = 93;
pub const _SC_XOPEN_SHM: c_int = 94;
pub const _SC_2_CHAR_TERM: c_int = 95;
pub const _SC_2_UPE: c_int = 97;
pub const _SC_NZERO: c_int = 109;
pub const _SC_XOPEN_REALTIME: c_int = 130;
pub const _SC_XOPEN_REALTIME_THREADS: c_int = 131;
pub const _SC_ADVISORY_INFO: c_int = 132;
pub const _SC_BARRIERS: c_int = 133;
pub const _SC_CLOCK_SELECTION: c_int = 137;
pub const _SC_CPUTIME: c_int = 138;
pub const _SC_THREAD_CPUTIME: c_int = 139;
pub const _SC_MONOTONIC_CLOCK: c_int = 149;
pub const _SC_READER_WRITER_LOCKS: c_int = 153;
pub const _SC_SPIN_LOCKS: c_int = 154;
pub const _SC_REGEXP: c_int = 155;
pub const _SC_SHELL: c_int = 157;
pub const _SC_SPAWN: c_int = 159;
pub const _SC_SPORADIC_SERVER: c_int = 160;
pub const _SC_THREAD_SPORADIC_SERVER: c_int = 161;
pub const _SC_TIMEOUTS: c_int = 164;
pub const _SC_TYPED_MEMORY_OBJECTS: c_int = 165;
pub const _SC_SYMLOOP_MAX: c_int = 173;
pub const _SC_V6_ILP32_OFF32: c_int = 176;
pub const _SC_V6_ILP32_OFFBIG: c_int = 177;
pub const _SC_V6_LP64_OFF64: c_int = 178;
pub const _SC_V6_LPBIG_OFFBIG: c_int = 179;
pub const _SC_HOST_NAME_MAX: c_int = 180;
pub const _SC_IPV6: c_int = 235;
pub const _SC_RAW_SOCKETS: c_int = 236;
pub const _SC_V7_ILP32_OFF32: c_int = 237;
pub const _SC_V7_ILP32_OFFBIG: c_int = 238;
pub const _SC_V7_LP64_OFF64: c_int = 239;
pub const _SC_V7_LPBIG_OFFBIG: c_int = 240;
pub const _SC_SS_REPL_MAX: c_int = 241;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: c_int = 247;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: c_int = 248;
pub const _SC_MINSIGSTKSZ: c_int = 249;
pub const _SC_SIGSTKSZ: c_int = 250;
pub const _SC_V8_ILP32_OFF32: c_int = 251;
pub const _SC_V8_ILP32_OFFBIG: c_int = 252;
pub const _SC_V8_LP64_OFF64: c_int = 253;
pub const _SC_V8_LPBIG_OFFBIG: c_int = 254;
pub const _SC_NSIG: c_int = 255;
pub const _SC_DEVICE_CONTROL: c_int = 256;
pub const _SC_TEXTDOMAIN_MAX: c_int = 257;
pub const _SC_XOPEN_UUCP: c_int = 258;

// =========================================================================
// Pthread constants (<pthread.h>)
// =========================================================================

pub const PTHREAD_CREATE_JOINABLE: c_int = 0;
pub const PTHREAD_CREATE_DETACHED: c_int = 1;
pub const PTHREAD_INHERIT_SCHED: c_int = 0;
pub const PTHREAD_EXPLICIT_SCHED: c_int = 1;
pub const PTHREAD_SCOPE_SYSTEM: c_int = 0;
pub const PTHREAD_SCOPE_PROCESS: c_int = 1;
pub const PTHREAD_PROCESS_PRIVATE: c_int = 0;
pub const PTHREAD_PROCESS_SHARED: c_int = 1;
pub const PTHREAD_MUTEX_NORMAL: c_int = 0;
pub const PTHREAD_MUTEX_RECURSIVE: c_int = 1;
pub const PTHREAD_MUTEX_ERRORCHECK: c_int = 2;
pub const PTHREAD_MUTEX_DEFAULT: c_int = PTHREAD_MUTEX_NORMAL;
pub const PTHREAD_MUTEX_STALLED: c_int = 0;
pub const PTHREAD_MUTEX_ROBUST: c_int = 1;
pub const PTHREAD_CANCEL_ENABLE: c_int = 0;
pub const PTHREAD_CANCEL_DISABLE: c_int = 1;
pub const PTHREAD_CANCEL_DEFERRED: c_int = 0;
pub const PTHREAD_CANCEL_ASYNCHRONOUS: c_int = 1;
pub const PTHREAD_CANCELED: *mut c_void = !0usize as *mut c_void;
pub const PTHREAD_BARRIER_SERIAL_THREAD: c_int = -1;

// =========================================================================
// Resource limits, usage and priority (<sys/resource.h>)
// =========================================================================

pub const RLIMIT_CPU: c_int = 0;
pub const RLIMIT_FSIZE: c_int = 1;
pub const RLIMIT_DATA: c_int = 2;
pub const RLIMIT_STACK: c_int = 3;
pub const RLIMIT_CORE: c_int = 4;
pub const RLIMIT_RSS: c_int = 5;
pub const RLIMIT_NPROC: c_int = 6;
pub const RLIMIT_NOFILE: c_int = 7;
pub const RLIMIT_MEMLOCK: c_int = 8;
pub const RLIMIT_AS: c_int = 9;
pub const RLIM_NLIMITS: c_int = 10;
pub const RLIM_INFINITY: rlim_t = !0;
pub const RLIM_SAVED_MAX: rlim_t = RLIM_INFINITY;
pub const RLIM_SAVED_CUR: rlim_t = RLIM_INFINITY;

pub const PRIO_PROCESS: c_int = 0;
pub const PRIO_PGRP: c_int = 1;
pub const PRIO_USER: c_int = 2;

pub const RUSAGE_SELF: c_int = 0;
pub const RUSAGE_CHILDREN: c_int = -1;
pub const RUSAGE_THREAD: c_int = 1;

// =========================================================================
// Network database (<netdb.h>)
// =========================================================================

pub const IPPORT_RESERVED: c_int = 1024;

pub const AI_PASSIVE: c_int = 0x1;
pub const AI_CANONNAME: c_int = 0x2;
pub const AI_NUMERICHOST: c_int = 0x4;
pub const AI_V4MAPPED: c_int = 0x8;
pub const AI_ALL: c_int = 0x10;
pub const AI_ADDRCONFIG: c_int = 0x20;
pub const AI_NUMERICSERV: c_int = 0x400;

pub const EAI_AGAIN: c_int = 2;
pub const EAI_BADFLAGS: c_int = 3;
pub const EAI_FAIL: c_int = 4;
pub const EAI_FAMILY: c_int = 5;
pub const EAI_MEMORY: c_int = 6;
pub const EAI_NONAME: c_int = 8;
pub const EAI_SERVICE: c_int = 9;
pub const EAI_SOCKTYPE: c_int = 10;
pub const EAI_SYSTEM: c_int = 11;
pub const EAI_OVERFLOW: c_int = 14;

pub const NI_MAXHOST: usize = 1025;
pub const NI_MAXSERV: usize = 32;
pub const NI_NUMERICHOST: c_int = 0x1;
pub const NI_NUMERICSERV: c_int = 0x2;
pub const NI_NOFQDN: c_int = 0x4;
pub const NI_NAMEREQD: c_int = 0x8;
pub const NI_DGRAM: c_int = 0x10;
pub const NI_NUMERICSCOPE: c_int = 0x100;

pub const HOST_NOT_FOUND: c_int = 1;
pub const TRY_AGAIN: c_int = 2;
pub const NO_RECOVERY: c_int = 3;
pub const NO_DATA: c_int = 4;
pub const NO_ADDRESS: c_int = NO_DATA;

// =========================================================================
// Terminals (<termios.h>)
// =========================================================================

pub const NCCS: usize = 32;

// c_cc indices; a zero character disables its function
pub const VINTR: usize = 0;
pub const VQUIT: usize = 1;
pub const VERASE: usize = 2;
pub const VKILL: usize = 3;
pub const VEOF: usize = 4;
pub const VTIME: usize = 5;
pub const VMIN: usize = 6;
pub const VSTART: usize = 8;
pub const VSTOP: usize = 9;
pub const VSUSP: usize = 10;
pub const VEOL: usize = 11;
pub const VREPRINT: usize = 12;
pub const VDISCARD: usize = 13;
pub const VWERASE: usize = 14;
pub const VLNEXT: usize = 15;

// c_iflag
pub const IGNBRK: tcflag_t = 0o1;
pub const BRKINT: tcflag_t = 0o2;
pub const IGNPAR: tcflag_t = 0o4;
pub const PARMRK: tcflag_t = 0o10;
pub const INPCK: tcflag_t = 0o20;
pub const ISTRIP: tcflag_t = 0o40;
pub const INLCR: tcflag_t = 0o100;
pub const IGNCR: tcflag_t = 0o200;
pub const ICRNL: tcflag_t = 0o400;
pub const IXON: tcflag_t = 0o2000;
pub const IXANY: tcflag_t = 0o4000;
pub const IXOFF: tcflag_t = 0o10000;
pub const IMAXBEL: tcflag_t = 0o20000;

// c_oflag
pub const OPOST: tcflag_t = 0o1;
pub const ONLCR: tcflag_t = 0o4;
pub const OCRNL: tcflag_t = 0o10;
pub const ONOCR: tcflag_t = 0o20;
pub const ONLRET: tcflag_t = 0o40;
pub const OFILL: tcflag_t = 0o100;
pub const OFDEL: tcflag_t = 0o200;
pub const NLDLY: tcflag_t = 0o400;
pub const NL0: tcflag_t = 0;
pub const NL1: tcflag_t = 0o400;
pub const CRDLY: tcflag_t = 0o3000;
pub const CR0: tcflag_t = 0;
pub const CR1: tcflag_t = 0o1000;
pub const CR2: tcflag_t = 0o2000;
pub const CR3: tcflag_t = 0o3000;
pub const TABDLY: tcflag_t = 0o14000;
pub const TAB0: tcflag_t = 0;
pub const TAB1: tcflag_t = 0o4000;
pub const TAB2: tcflag_t = 0o10000;
pub const TAB3: tcflag_t = 0o14000;
pub const BSDLY: tcflag_t = 0o20000;
pub const BS0: tcflag_t = 0;
pub const BS1: tcflag_t = 0o20000;
pub const VTDLY: tcflag_t = 0o40000;
pub const VT0: tcflag_t = 0;
pub const VT1: tcflag_t = 0o40000;
pub const FFDLY: tcflag_t = 0o100000;
pub const FF0: tcflag_t = 0;
pub const FF1: tcflag_t = 0o100000;

// c_cflag
pub const CSIZE: tcflag_t = 0o60;
pub const CS5: tcflag_t = 0;
pub const CS6: tcflag_t = 0o20;
pub const CS7: tcflag_t = 0o40;
pub const CS8: tcflag_t = 0o60;
pub const CSTOPB: tcflag_t = 0o100;
pub const CREAD: tcflag_t = 0o200;
pub const PARENB: tcflag_t = 0o400;
pub const PARODD: tcflag_t = 0o1000;
pub const HUPCL: tcflag_t = 0o2000;
pub const CLOCAL: tcflag_t = 0o4000;

// c_lflag
pub const ISIG: tcflag_t = 0o1;
pub const ICANON: tcflag_t = 0o2;
pub const ECHO: tcflag_t = 0o10;
pub const ECHOE: tcflag_t = 0o20;
pub const ECHOK: tcflag_t = 0o40;
pub const ECHONL: tcflag_t = 0o100;
pub const NOFLSH: tcflag_t = 0o200;
pub const TOSTOP: tcflag_t = 0o400;
pub const ECHOCTL: tcflag_t = 0o1000;
pub const ECHOKE: tcflag_t = 0o4000;
pub const IEXTEN: tcflag_t = 0o100000;

// tcsetattr() actions, tcflush() queues, tcflow() actions
pub const TCSANOW: c_int = 0;
pub const TCSADRAIN: c_int = 1;
pub const TCSAFLUSH: c_int = 2;
pub const TCIFLUSH: c_int = 0;
pub const TCOFLUSH: c_int = 1;
pub const TCIOFLUSH: c_int = 2;
pub const TCOOFF: c_int = 0;
pub const TCOON: c_int = 1;
pub const TCIOFF: c_int = 2;
pub const TCION: c_int = 3;

// Line speeds: each name carries its rate in bits per second
pub const B0: speed_t = 0;
pub const B50: speed_t = 50;
pub const B75: speed_t = 75;
pub const B110: speed_t = 110;
pub const B134: speed_t = 134;
pub const B150: speed_t = 150;
pub const B200: speed_t = 200;
pub const B300: speed_t = 300;
pub const B600: speed_t = 600;
pub const B1200: speed_t = 1200;
pub const B1800: speed_t = 1800;
pub const B2400: speed_t = 2400;
pub const B4800: speed_t = 4800;
pub const B9600: speed_t = 9600;
pub const B19200: speed_t = 19200;
pub const B38400: speed_t = 38400;
pub const B57600: speed_t = 57600;
pub const B115200: speed_t = 115200;
pub const B230400: speed_t = 230400;
pub const B460800: speed_t = 460800;
pub const B500000: speed_t = 500000;
pub const B576000: speed_t = 576000;
pub const B921600: speed_t = 921600;
pub const B1000000: speed_t = 1000000;
pub const B1152000: speed_t = 1152000;
pub const B1500000: speed_t = 1500000;
pub const B2000000: speed_t = 2000000;
pub const B2500000: speed_t = 2500000;
pub const B3000000: speed_t = 3000000;
pub const B3500000: speed_t = 3500000;
pub const B4000000: speed_t = 4000000;

// ioctl requests (<sys/ioctl.h>)
pub const TCGETS: c_ulong = 0x5401;
pub const TCSETS: c_ulong = 0x5402;
pub const TCSETSW: c_ulong = 0x5403;
pub const TCSETSF: c_ulong = 0x5404;
pub const TIOCGETA: c_ulong = TCGETS;
pub const TIOCSETA: c_ulong = TCSETS;
pub const TIOCSETAW: c_ulong = TCSETSW;
pub const TIOCSETAF: c_ulong = TCSETSF;
pub const TCSBRK: c_ulong = 0x5409;
pub const TCXONC: c_ulong = 0x540A;
pub const TCFLSH: c_ulong = 0x540B;
pub const TCSBRKP: c_ulong = 0x5425;
pub const TIOCSCTTY: c_ulong = 0x540E;
pub const TIOCGPGRP: c_ulong = 0x540F;
pub const TIOCSPGRP: c_ulong = 0x5410;
pub const TIOCNOTTY: c_ulong = 0x5422;
pub const TIOCGSID: c_ulong = 0x5429;
pub const TIOCOUTQ: c_ulong = 0x5411;
pub const FIONREAD: c_ulong = 0x541B;
pub const TIOCINQ: c_ulong = FIONREAD;
pub const TIOCGWINSZ: c_ulong = 0x5413;
pub const TIOCSWINSZ: c_ulong = 0x5414;
pub const TIOCSETD: c_ulong = 0x5423;
pub const TIOCGETD: c_ulong = 0x5424;
pub const N_TTY: c_int = 0;
pub const TIOCGPTN: c_ulong = 0x80045430;
pub const TIOCSPTLCK: c_ulong = 0x40045431;
pub const TIOCGDEV: c_ulong = 0x80045432;
pub const FIONBIO: c_ulong = 0x5421;
pub const FIONCLEX: c_ulong = 0x5450;
pub const FIOCLEX: c_ulong = 0x5451;

// Socket and interface controls (<sys/sockio.h>)
pub const SIOCATMARK: c_ulong = 0x8905;
pub const SIOCGIFNAME: c_ulong = 0x8910;
pub const SIOCGIFCONF: c_ulong = 0x8912;
pub const SIOCGIFFLAGS: c_ulong = 0x8913;
pub const SIOCSIFFLAGS: c_ulong = 0x8914;
pub const SIOCGIFADDR: c_ulong = 0x8915;
pub const SIOCGIFBRDADDR: c_ulong = 0x8919;
pub const SIOCGIFNETMASK: c_ulong = 0x891B;
pub const SIOCGIFMTU: c_ulong = 0x8921;
pub const SIOCGIFHWADDR: c_ulong = 0x8927;
pub const SIOCGIFINDEX: c_ulong = 0x8933;

// Interface flags (<net/if.h>)
pub const IFF_UP: c_int = 0x1;
pub const IFF_BROADCAST: c_int = 0x2;
pub const IFF_DEBUG: c_int = 0x4;
pub const IFF_LOOPBACK: c_int = 0x8;
pub const IFF_POINTOPOINT: c_int = 0x10;
pub const IFF_NOTRAILERS: c_int = 0x20;
pub const IFF_RUNNING: c_int = 0x40;
pub const IFF_NOARP: c_int = 0x80;
pub const IFF_PROMISC: c_int = 0x100;
pub const IFF_ALLMULTI: c_int = 0x200;
pub const IFF_MULTICAST: c_int = 0x1000;

// Directory entry types
pub const DT_UNKNOWN: u8 = 0;
pub const DT_FIFO: u8 = 1;
pub const DT_CHR: u8 = 2;
pub const DT_DIR: u8 = 4;
pub const DT_BLK: u8 = 6;
pub const DT_REG: u8 = 8;
pub const DT_LNK: u8 = 10;
pub const DT_SOCK: u8 = 12;

// FD_SETSIZE for select()
pub const FD_SETSIZE: usize = 1024;

// Access mode flags
pub const F_OK: c_int = 0;
pub const R_OK: c_int = 4;
pub const W_OK: c_int = 2;
pub const X_OK: c_int = 1;

// flock(2)
pub const LOCK_SH: c_int = 1;
pub const LOCK_EX: c_int = 2;
pub const LOCK_NB: c_int = 4;
pub const LOCK_UN: c_int = 8;

// syslog(3)
pub const LOG_EMERG: c_int = 0;
pub const LOG_ALERT: c_int = 1;
pub const LOG_CRIT: c_int = 2;
pub const LOG_ERR: c_int = 3;
pub const LOG_WARNING: c_int = 4;
pub const LOG_NOTICE: c_int = 5;
pub const LOG_INFO: c_int = 6;
pub const LOG_DEBUG: c_int = 7;
pub const LOG_KERN: c_int = 0 << 3;
pub const LOG_USER: c_int = 1 << 3;
pub const LOG_MAIL: c_int = 2 << 3;
pub const LOG_DAEMON: c_int = 3 << 3;
pub const LOG_AUTH: c_int = 4 << 3;
pub const LOG_SYSLOG: c_int = 5 << 3;
pub const LOG_LPR: c_int = 6 << 3;
pub const LOG_NEWS: c_int = 7 << 3;
pub const LOG_UUCP: c_int = 8 << 3;
pub const LOG_CRON: c_int = 9 << 3;
pub const LOG_LOCAL0: c_int = 16 << 3;
pub const LOG_LOCAL1: c_int = 17 << 3;
pub const LOG_LOCAL2: c_int = 18 << 3;
pub const LOG_LOCAL3: c_int = 19 << 3;
pub const LOG_LOCAL4: c_int = 20 << 3;
pub const LOG_LOCAL5: c_int = 21 << 3;
pub const LOG_LOCAL6: c_int = 22 << 3;
pub const LOG_LOCAL7: c_int = 23 << 3;
pub const LOG_PID: c_int = 0x01;
pub const LOG_CONS: c_int = 0x02;
pub const LOG_NDELAY: c_int = 0x08;
pub const LOG_NOWAIT: c_int = 0x10;
pub const LOG_PERROR: c_int = 0x20;

// <sys/wait.h> status macros, as functions over the supervisor's status word
#[inline]
pub const fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}
#[inline]
pub const fn WEXITSTATUS(status: c_int) -> c_int {
    (status >> 8) & 0xff
}
#[inline]
pub const fn WIFSIGNALED(status: c_int) -> bool {
    (status & 0x7f) != 0 && !WIFSTOPPED(status) && status != 0xffff
}
#[inline]
pub const fn WTERMSIG(status: c_int) -> c_int {
    status & 0x7f
}
#[inline]
pub const fn WIFSTOPPED(status: c_int) -> bool {
    (status & 0xff) == 0x7f
}
#[inline]
pub const fn WSTOPSIG(status: c_int) -> c_int {
    (status >> 8) & 0xff
}
#[inline]
pub const fn WCOREDUMP(status: c_int) -> bool {
    (status & 0x80) != 0
}
#[inline]
pub const fn WIFCONTINUED(status: c_int) -> bool {
    status == 0xffff
}

// <limits.h>
pub const CHAR_BIT: c_int = 8;
pub const SCHAR_MIN: c_schar = -128;
pub const SCHAR_MAX: c_schar = 127;
#[cfg(target_arch = "x86_64")]
pub const CHAR_MIN: c_char = SCHAR_MIN;
#[cfg(target_arch = "x86_64")]
pub const CHAR_MAX: c_char = SCHAR_MAX;
#[cfg(target_arch = "aarch64")]
pub const CHAR_MIN: c_char = 0;
#[cfg(target_arch = "aarch64")]
pub const CHAR_MAX: c_char = UCHAR_MAX;
pub const UCHAR_MAX: c_uchar = 255;
pub const SHRT_MIN: c_short = -32768;
pub const SHRT_MAX: c_short = 32767;
pub const USHRT_MAX: c_ushort = 65535;
pub const INT_MIN: c_int = -2147483647 - 1;
pub const INT_MAX: c_int = 2147483647;
pub const UINT_MAX: c_uint = 4294967295;
pub const LONG_MIN: c_long = -9223372036854775807 - 1;
pub const LONG_MAX: c_long = 9223372036854775807;
pub const ULONG_MAX: c_ulong = 18446744073709551615;
pub const LLONG_MIN: c_longlong = -9223372036854775807 - 1;
pub const LLONG_MAX: c_longlong = 9223372036854775807;
pub const ULLONG_MAX: c_ulonglong = 18446744073709551615;
pub const SSIZE_MAX: ssize_t = LONG_MAX as ssize_t;
pub const SIZE_MAX: size_t = ULONG_MAX as size_t;
pub const PATH_MAX: usize = 4096;
pub const NAME_MAX: usize = 255;
pub const PIPE_BUF: usize = 4096;
pub const ARG_MAX: c_int = 131072;
pub const LINE_MAX: c_int = 2048;
pub const NGROUPS_MAX: c_int = 16;
pub const LOGIN_NAME_MAX: c_int = 256;
pub const HOST_NAME_MAX: c_int = 255;
pub const MB_LEN_MAX: c_int = 4;
pub const IOV_MAX: usize = 1024;
pub const SYMLOOP_MAX: c_int = 40;
pub const TTY_NAME_MAX: c_int = 32;
pub const PTHREAD_STACK_MIN: size_t = 16384;
pub const PTHREAD_KEYS_MAX: c_int = 128;
pub const PTHREAD_DESTRUCTOR_ITERATIONS: c_int = 4;
pub const SEM_VALUE_MAX: c_int = 2147483647;
pub const SIGQUEUE_MAX: c_int = 64;
pub const RTSIG_MAX: c_int = 32;
pub const DELAYTIMER_MAX: c_int = 2147483647;
pub const AIO_LISTIO_MAX: c_int = 64;
pub const AIO_MAX: c_int = 1024;
pub const AIO_PRIO_DELTA_MAX: c_int = 0;
pub const _POSIX_AIO_LISTIO_MAX: c_int = 2;
pub const _POSIX_AIO_MAX: c_int = 1;
pub const _POSIX_ARG_MAX: c_int = 4096;
pub const _POSIX_CHILD_MAX: c_int = 25;
pub const _POSIX_CLOCKRES_MIN: c_int = 20000000;
pub const _POSIX_DELAYTIMER_MAX: c_int = 32;
pub const _POSIX_HOST_NAME_MAX: c_int = 255;
pub const _POSIX_LINK_MAX: c_int = 8;
pub const _POSIX_LOGIN_NAME_MAX: c_int = 9;
pub const _POSIX_MAX_CANON: c_int = 255;
pub const _POSIX_MAX_INPUT: c_int = 255;
pub const _POSIX_NAME_MAX: c_int = 14;
pub const _POSIX_NGROUPS_MAX: c_int = 8;
pub const _POSIX_OPEN_MAX: c_int = 20;
pub const _POSIX_PATH_MAX: c_int = 256;
pub const _POSIX_PIPE_BUF: c_int = 512;
pub const _POSIX_RE_DUP_MAX: c_int = 255;
pub const _POSIX_RTSIG_MAX: c_int = 8;
pub const _POSIX_SEM_NSEMS_MAX: c_int = 256;
pub const _POSIX_SEM_VALUE_MAX: c_int = 32767;
pub const _POSIX_SIGQUEUE_MAX: c_int = 32;
pub const _POSIX_SSIZE_MAX: c_int = 32767;
pub const _POSIX_SYMLINK_MAX: c_int = 255;
pub const _POSIX_SYMLOOP_MAX: c_int = 8;
pub const _POSIX_THREAD_DESTRUCTOR_ITERATIONS: c_int = 4;
pub const _POSIX_THREAD_KEYS_MAX: c_int = 128;
pub const _POSIX_THREAD_THREADS_MAX: c_int = 64;
pub const _POSIX_TIMER_MAX: c_int = 32;
pub const _POSIX_TTY_NAME_MAX: c_int = 9;
pub const _POSIX_TZNAME_MAX: c_int = 6;
pub const _POSIX2_BC_BASE_MAX: c_int = 99;
pub const _POSIX2_BC_DIM_MAX: c_int = 2048;
pub const _POSIX2_BC_SCALE_MAX: c_int = 99;
pub const _POSIX2_BC_STRING_MAX: c_int = 1000;
pub const _POSIX2_CHARCLASS_NAME_MAX: c_int = 14;
pub const _POSIX2_COLL_WEIGHTS_MAX: c_int = 2;
pub const _POSIX2_EXPR_NEST_MAX: c_int = 32;
pub const _POSIX2_LINE_MAX: c_int = 2048;
pub const _POSIX2_RE_DUP_MAX: c_int = 255;
pub const _XOPEN_IOV_MAX: c_int = 16;
pub const _XOPEN_NAME_MAX: c_int = 255;
pub const _XOPEN_PATH_MAX: c_int = 1024;
pub const RE_DUP_MAX: c_int = _POSIX2_RE_DUP_MAX;
pub const NL_ARGMAX: c_int = 64;
pub const GETENTROPY_MAX: c_int = 256;
pub const NL_LANGMAX: c_int = 14;
pub const NL_MSGMAX: c_int = 32767;
pub const NL_SETMAX: c_int = 255;
pub const NL_TEXTMAX: c_int = _POSIX2_LINE_MAX;
pub const NZERO: c_int = 20;
pub const WORD_BIT: c_int = 32;
pub const LONG_BIT: c_int = 64;

// =========================================================================
// Volumes and mounts (<sys/mount.h>, <sys/statvfs.h>)
// =========================================================================

pub const MFSNAMELEN: usize = 16;
pub const MNAMELEN: usize = 88;
pub const MAXPATHLEN: usize = 4096;

pub const MNT_RDONLY: c_ulong = 0x00000001;
pub const MNT_NOEXEC: c_ulong = 0x00000004;
pub const MNT_NOSUID: c_ulong = 0x00000008;
pub const MNT_LOCAL: c_ulong = 0x00001000;
pub const MNT_NOATIME: c_ulong = 0x10000000;
pub const MNT_WAIT: c_int = 1;
pub const MNT_NOWAIT: c_int = 2;

pub const MS_RDONLY: c_ulong = 0x00000001;
pub const MS_NOSUID: c_ulong = 0x00000002;
pub const MS_NODEV: c_ulong = 0x00000004;
pub const MS_NOEXEC: c_ulong = 0x00000008;
pub const MS_REMOUNT: c_ulong = 0x00000020;
pub const MS_NOATIME: c_ulong = 0x00000400;
pub const MS_BIND: c_ulong = 0x00001000;
pub const MS_MOVE: c_ulong = 0x00002000;
pub const MS_REC: c_ulong = 0x00004000;

// umount2() flags
pub const MNT_FORCE: c_int = 0x00000001;
pub const MNT_DETACH: c_int = 0x00000002;

pub const ST_RDONLY: c_ulong = 0x0001;
pub const ST_NOSUID: c_ulong = 0x0002;
pub const ST_NODEV: c_ulong = 0x0004;
pub const ST_NOEXEC: c_ulong = 0x0008;
pub const ST_SYNCHRONOUS: c_ulong = 0x0010;
pub const ST_MANDLOCK: c_ulong = 0x0040;
pub const ST_WRITE: c_ulong = 0x0080;
pub const ST_APPEND: c_ulong = 0x0100;
pub const ST_IMMUTABLE: c_ulong = 0x0200;
pub const ST_NOATIME: c_ulong = 0x0400;
pub const ST_NODIRATIME: c_ulong = 0x0800;
pub const ST_RELATIME: c_ulong = 0x1000;

// =========================================================================
// File-tree walks (<ftw.h>)
// =========================================================================

pub const FTW_F: c_int = 0;
pub const FTW_D: c_int = 1;
pub const FTW_DNR: c_int = 2;
pub const FTW_NS: c_int = 3;
pub const FTW_SL: c_int = 4;
pub const FTW_DP: c_int = 5;
pub const FTW_SLN: c_int = 6;

pub const FTW_PHYS: c_int = 1;
pub const FTW_MOUNT: c_int = 2;
pub const FTW_CHDIR: c_int = 4;
pub const FTW_DEPTH: c_int = 8;

// =========================================================================
// System V IPC (<sys/ipc.h>, <sys/shm.h>, <sys/sem.h>, <sys/msg.h>)
// =========================================================================

pub const IPC_CREAT: c_int = 0o1000;
pub const IPC_EXCL: c_int = 0o2000;
pub const IPC_NOWAIT: c_int = 0o4000;
pub const IPC_PRIVATE: key_t = 0;
pub const IPC_RMID: c_int = 0;
pub const IPC_SET: c_int = 1;
pub const IPC_STAT: c_int = 2;

pub const SHM_RDONLY: c_int = 0o10000;
pub const SHM_RND: c_int = 0o20000;
pub const SHMLBA: c_int = 4096;

pub const SEM_UNDO: c_int = 0x1000;
pub const GETPID: c_int = 11;
pub const GETVAL: c_int = 12;
pub const GETALL: c_int = 13;
pub const GETNCNT: c_int = 14;
pub const GETZCNT: c_int = 15;
pub const SETVAL: c_int = 16;
pub const SETALL: c_int = 17;

pub const MSG_NOERROR: c_int = 0o10000;
pub const MSG_EXCEPT: c_int = 0o20000;

// =========================================================================
// Semaphores and asynchronous I/O (<semaphore.h>, <aio.h>)
// =========================================================================

pub const SEM_FAILED: *mut sem_t = core::ptr::null_mut();

pub const AIO_CANCELED: c_int = 0;
pub const AIO_NOTCANCELED: c_int = 1;
pub const AIO_ALLDONE: c_int = 2;
pub const LIO_READ: c_int = 0;
pub const LIO_WRITE: c_int = 1;
pub const LIO_NOP: c_int = 2;
pub const LIO_WAIT: c_int = 0;
pub const LIO_NOWAIT: c_int = 1;

// =========================================================================
// ISO C threads (<threads.h>)
// =========================================================================

pub const TSS_DTOR_ITERATIONS: c_int = 4;
pub const thrd_success: c_int = 0;
pub const thrd_busy: c_int = 1;
pub const thrd_error: c_int = 2;
pub const thrd_nomem: c_int = 3;
pub const thrd_timedout: c_int = 4;
pub const mtx_plain: c_int = 0;
pub const mtx_recursive: c_int = 1;
pub const mtx_timed: c_int = 2;
pub const ONCE_FLAG_INIT: once_flag = once_flag { __flag: 0 };

// =========================================================================
// Address wait and wake (<sys/umtx.h>)
// =========================================================================

pub const UMTX_OP_WAIT: c_int = 2;
pub const UMTX_OP_WAKE: c_int = 3;
pub const UMTX_OP_WAIT_UINT: c_int = 11;
pub const UMTX_OP_WAIT_UINT_PRIVATE: c_int = 15;
pub const UMTX_OP_WAKE_PRIVATE: c_int = 16;
pub const UMTX_ABSTIME: c_uint = 0x01;

// =========================================================================
// Auxiliary vector (<sys/auxv.h>)
// =========================================================================

pub const AT_NULL: c_ulong = 0;
pub const AT_IGNORE: c_ulong = 1;
pub const AT_EXECFD: c_ulong = 2;
pub const AT_PHDR: c_ulong = 3;
pub const AT_PHENT: c_ulong = 4;
pub const AT_PHNUM: c_ulong = 5;
pub const AT_PAGESZ: c_ulong = 6;
pub const AT_BASE: c_ulong = 7;
pub const AT_FLAGS: c_ulong = 8;
pub const AT_ENTRY: c_ulong = 9;
pub const AT_NOTELF: c_ulong = 10;
pub const AT_UID: c_ulong = 11;
pub const AT_EUID: c_ulong = 12;
pub const AT_GID: c_ulong = 13;
pub const AT_EGID: c_ulong = 14;
pub const AT_PLATFORM: c_ulong = 15;
pub const AT_HWCAP: c_ulong = 16;
pub const AT_CLKTCK: c_ulong = 17;
pub const AT_SECURE: c_ulong = 23;
pub const AT_BASE_PLATFORM: c_ulong = 24;
pub const AT_RANDOM: c_ulong = 25;
pub const AT_HWCAP2: c_ulong = 26;
pub const AT_EXECFN: c_ulong = 31;
pub const AT_SYSINFO_EHDR: c_ulong = 33;
pub const AT_MINSIGSTKSZ: c_ulong = 51;
pub const AT_EXECPATH: c_ulong = AT_EXECFN;

#[cfg(target_arch = "aarch64")]
pub const HWCAP_FP: c_ulong = 1 << 0;
#[cfg(target_arch = "aarch64")]
pub const HWCAP_ASIMD: c_ulong = 1 << 1;
#[cfg(target_arch = "aarch64")]
pub const HWCAP_AES: c_ulong = 1 << 3;
#[cfg(target_arch = "aarch64")]
pub const HWCAP_PMULL: c_ulong = 1 << 4;
#[cfg(target_arch = "aarch64")]
pub const HWCAP_SHA1: c_ulong = 1 << 5;
#[cfg(target_arch = "aarch64")]
pub const HWCAP_SHA2: c_ulong = 1 << 6;
#[cfg(target_arch = "aarch64")]
pub const HWCAP_CRC32: c_ulong = 1 << 7;
#[cfg(target_arch = "aarch64")]
pub const HWCAP_ATOMICS: c_ulong = 1 << 8;
#[cfg(target_arch = "aarch64")]
pub const HWCAP_FPHP: c_ulong = 1 << 9;
#[cfg(target_arch = "aarch64")]
pub const HWCAP_ASIMDHP: c_ulong = 1 << 10;
#[cfg(target_arch = "aarch64")]
pub const HWCAP_ASIMDRDM: c_ulong = 1 << 12;
#[cfg(target_arch = "aarch64")]
pub const HWCAP_LRCPC: c_ulong = 1 << 15;
#[cfg(target_arch = "aarch64")]
pub const HWCAP_DCPOP: c_ulong = 1 << 16;
#[cfg(target_arch = "aarch64")]
pub const HWCAP_SHA3: c_ulong = 1 << 17;
#[cfg(target_arch = "aarch64")]
pub const HWCAP_ASIMDDP: c_ulong = 1 << 20;
#[cfg(target_arch = "aarch64")]
pub const HWCAP_SHA512: c_ulong = 1 << 21;
#[cfg(target_arch = "aarch64")]
pub const HWCAP_SVE: c_ulong = 1 << 22;
#[cfg(target_arch = "aarch64")]
pub const HWCAP_DIT: c_ulong = 1 << 24;
#[cfg(target_arch = "aarch64")]
pub const HWCAP_FLAGM: c_ulong = 1 << 27;
#[cfg(target_arch = "aarch64")]
pub const HWCAP_SSBS: c_ulong = 1 << 28;
#[cfg(target_arch = "aarch64")]
pub const HWCAP2_SVE2: c_ulong = 1 << 1;
#[cfg(target_arch = "aarch64")]
pub const HWCAP2_I8MM: c_ulong = 1 << 13;
#[cfg(target_arch = "aarch64")]
pub const HWCAP2_BF16: c_ulong = 1 << 14;
#[cfg(target_arch = "aarch64")]
pub const HWCAP2_RNG: c_ulong = 1 << 16;
// x86_64: AT_HWCAP is CPUID leaf 1 EDX, whose bits keep their CPUID names.
#[cfg(target_arch = "x86_64")]
pub const HWCAP2_RING3MWAIT: c_ulong = 1 << 0;
#[cfg(target_arch = "x86_64")]
pub const HWCAP2_FSGSBASE: c_ulong = 1 << 1;

// =========================================================================
// ioctl command encoding (<linux/ioctl.h>)
// =========================================================================

pub const _IOC_NRBITS: u32 = 8;
pub const _IOC_TYPEBITS: u32 = 8;
pub const _IOC_SIZEBITS: u32 = 14;
pub const _IOC_DIRBITS: u32 = 2;
pub const _IOC_NRMASK: u32 = (1 << _IOC_NRBITS) - 1;
pub const _IOC_TYPEMASK: u32 = (1 << _IOC_TYPEBITS) - 1;
pub const _IOC_SIZEMASK: u32 = (1 << _IOC_SIZEBITS) - 1;
pub const _IOC_DIRMASK: u32 = (1 << _IOC_DIRBITS) - 1;
pub const _IOC_NRSHIFT: u32 = 0;
pub const _IOC_TYPESHIFT: u32 = _IOC_NRSHIFT + _IOC_NRBITS;
pub const _IOC_SIZESHIFT: u32 = _IOC_TYPESHIFT + _IOC_TYPEBITS;
pub const _IOC_DIRSHIFT: u32 = _IOC_SIZESHIFT + _IOC_SIZEBITS;
pub const _IOC_NONE: u32 = 0;
pub const _IOC_WRITE: u32 = 1;
pub const _IOC_READ: u32 = 2;

pub const fn _IOC(dir: u32, ty: u32, nr: u32, size: usize) -> c_ulong {
    ((dir << _IOC_DIRSHIFT)
        | (ty << _IOC_TYPESHIFT)
        | (nr << _IOC_NRSHIFT)
        | ((size as u32) << _IOC_SIZESHIFT)) as c_ulong
}

pub const fn _IO(ty: u32, nr: u32) -> c_ulong {
    _IOC(_IOC_NONE, ty, nr, 0)
}

pub const fn _IOR<T>(ty: u32, nr: u32) -> c_ulong {
    _IOC(_IOC_READ, ty, nr, core::mem::size_of::<T>())
}

pub const fn _IOW<T>(ty: u32, nr: u32) -> c_ulong {
    _IOC(_IOC_WRITE, ty, nr, core::mem::size_of::<T>())
}

pub const fn _IOWR<T>(ty: u32, nr: u32) -> c_ulong {
    _IOC(_IOC_READ | _IOC_WRITE, ty, nr, core::mem::size_of::<T>())
}

pub const fn _IOC_DIR(nr: c_ulong) -> u32 {
    ((nr as u32) >> _IOC_DIRSHIFT) & _IOC_DIRMASK
}

pub const fn _IOC_TYPE(nr: c_ulong) -> u32 {
    ((nr as u32) >> _IOC_TYPESHIFT) & _IOC_TYPEMASK
}

pub const fn _IOC_NR(nr: c_ulong) -> u32 {
    ((nr as u32) >> _IOC_NRSHIFT) & _IOC_NRMASK
}

pub const fn _IOC_SIZE(nr: c_ulong) -> u32 {
    ((nr as u32) >> _IOC_SIZESHIFT) & _IOC_SIZEMASK
}

// =========================================================================
// Inode attributes and block devices (<linux/fs.h>)
// =========================================================================

pub const FS_SECRM_FL: c_int = 0x00000001;
pub const FS_UNRM_FL: c_int = 0x00000002;
pub const FS_COMPR_FL: c_int = 0x00000004;
pub const FS_SYNC_FL: c_int = 0x00000008;
pub const FS_IMMUTABLE_FL: c_int = 0x00000010;
pub const FS_APPEND_FL: c_int = 0x00000020;
pub const FS_NODUMP_FL: c_int = 0x00000040;
pub const FS_NOATIME_FL: c_int = 0x00000080;
pub const FS_DIRTY_FL: c_int = 0x00000100;
pub const FS_COMPRBLK_FL: c_int = 0x00000200;
pub const FS_NOCOMP_FL: c_int = 0x00000400;
pub const FS_ENCRYPT_FL: c_int = 0x00000800;
pub const FS_BTREE_FL: c_int = 0x00001000;
pub const FS_INDEX_FL: c_int = 0x00001000;
pub const FS_IMAGIC_FL: c_int = 0x00002000;
pub const FS_JOURNAL_DATA_FL: c_int = 0x00004000;
pub const FS_NOTAIL_FL: c_int = 0x00008000;
pub const FS_DIRSYNC_FL: c_int = 0x00010000;
pub const FS_TOPDIR_FL: c_int = 0x00020000;
pub const FS_HUGE_FILE_FL: c_int = 0x00040000;
pub const FS_EXTENT_FL: c_int = 0x00080000;
pub const FS_VERITY_FL: c_int = 0x00100000;
pub const FS_EA_INODE_FL: c_int = 0x00200000;
pub const FS_EOFBLOCKS_FL: c_int = 0x00400000;
pub const FS_NOCOW_FL: c_int = 0x00800000;
pub const FS_DAX_FL: c_int = 0x02000000;
pub const FS_INLINE_DATA_FL: c_int = 0x10000000;
pub const FS_PROJINHERIT_FL: c_int = 0x20000000;
pub const FS_CASEFOLD_FL: c_int = 0x40000000;
pub const FS_RESERVED_FL: c_int = 0x80000000u32 as c_int;
// Refuses unlink, rmdir and renaming the inode away or over; not a Linux flag.
pub const FS_NATIVE_NOUNLINK_FL: c_int = 0x04000000;
pub const FS_FL_USER_VISIBLE: c_int = 0x0003DFFF;
pub const FS_FL_USER_MODIFIABLE: c_int = 0x000380FF;

pub const FS_XFLAG_REALTIME: u32 = 0x00000001;
pub const FS_XFLAG_PREALLOC: u32 = 0x00000002;
pub const FS_XFLAG_IMMUTABLE: u32 = 0x00000008;
pub const FS_XFLAG_APPEND: u32 = 0x00000010;
pub const FS_XFLAG_SYNC: u32 = 0x00000020;
pub const FS_XFLAG_NOATIME: u32 = 0x00000040;
pub const FS_XFLAG_NODUMP: u32 = 0x00000080;
pub const FS_XFLAG_RTINHERIT: u32 = 0x00000100;
pub const FS_XFLAG_PROJINHERIT: u32 = 0x00000200;
pub const FS_XFLAG_NOSYMLINKS: u32 = 0x00000400;
pub const FS_XFLAG_EXTSIZE: u32 = 0x00000800;
pub const FS_XFLAG_EXTSZINHERIT: u32 = 0x00001000;
pub const FS_XFLAG_NODEFRAG: u32 = 0x00002000;
pub const FS_XFLAG_FILESTREAM: u32 = 0x00004000;
pub const FS_XFLAG_DAX: u32 = 0x00008000;
pub const FS_XFLAG_COWEXTSIZE: u32 = 0x00010000;
pub const FS_XFLAG_HASATTR: u32 = 0x80000000;

pub const FS_IOC_GETFLAGS: c_ulong = _IOR::<c_long>(b'f' as u32, 1);
pub const FS_IOC_SETFLAGS: c_ulong = _IOW::<c_long>(b'f' as u32, 2);
pub const FS_IOC32_GETFLAGS: c_ulong = _IOR::<c_int>(b'f' as u32, 1);
pub const FS_IOC32_SETFLAGS: c_ulong = _IOW::<c_int>(b'f' as u32, 2);
pub const FS_IOC_FSGETXATTR: c_ulong = _IOR::<fsxattr>(b'X' as u32, 31);
pub const FS_IOC_FSSETXATTR: c_ulong = _IOW::<fsxattr>(b'X' as u32, 32);

pub const BLKROGET: c_ulong = _IO(0x12, 94);
pub const BLKGETSIZE: c_ulong = _IO(0x12, 96);
pub const BLKFLSBUF: c_ulong = _IO(0x12, 97);
pub const BLKSSZGET: c_ulong = _IO(0x12, 104);
pub const BLKIOMIN: c_ulong = _IO(0x12, 120);
pub const BLKIOOPT: c_ulong = _IO(0x12, 121);
pub const BLKPBSZGET: c_ulong = _IO(0x12, 123);
pub const BLKGETSIZE64: c_ulong = _IOR::<u64>(0x12, 114);

// =========================================================================
// Project quotas (<sys/quota.h>)
// =========================================================================

pub const SUBCMDMASK: c_int = 0x00ff;
pub const SUBCMDSHIFT: c_int = 8;

pub const fn QCMD(cmd: c_int, ty: c_int) -> c_int {
    (cmd << SUBCMDSHIFT) | (ty & SUBCMDMASK)
}

pub const Q_SYNC: c_int = 0x800001;
pub const Q_QUOTAON: c_int = 0x800002;
pub const Q_QUOTAOFF: c_int = 0x800003;
pub const Q_GETFMT: c_int = 0x800004;
pub const Q_GETINFO: c_int = 0x800005;
pub const Q_SETINFO: c_int = 0x800006;
pub const Q_GETQUOTA: c_int = 0x800007;
pub const Q_SETQUOTA: c_int = 0x800008;
pub const Q_GETNEXTQUOTA: c_int = 0x800009;

pub const USRQUOTA: c_int = 0;
pub const GRPQUOTA: c_int = 1;
pub const PRJQUOTA: c_int = 2;
pub const MAXQUOTAS: c_int = 3;

pub const QIF_DQBLKSIZE_BITS: c_int = 10;
pub const QIF_DQBLKSIZE: c_int = 1 << QIF_DQBLKSIZE_BITS;
pub const QIF_BLIMITS: u32 = 1;
pub const QIF_SPACE: u32 = 2;
pub const QIF_ILIMITS: u32 = 4;
pub const QIF_INODES: u32 = 8;
pub const QIF_BTIME: u32 = 16;
pub const QIF_ITIME: u32 = 32;
pub const QIF_LIMITS: u32 = QIF_BLIMITS | QIF_ILIMITS;
pub const QIF_USAGE: u32 = QIF_SPACE | QIF_INODES;
pub const QIF_TIMES: u32 = QIF_BTIME | QIF_ITIME;
pub const QIF_ALL: u32 = QIF_LIMITS | QIF_USAGE | QIF_TIMES;

pub const IIF_BGRACE: u32 = 1;
pub const IIF_IGRACE: u32 = 2;
pub const IIF_FLAGS: u32 = 4;
pub const IIF_ALL: u32 = IIF_BGRACE | IIF_IGRACE | IIF_FLAGS;

// =========================================================================
// Framebuffer screen information (<linux/fb.h>)
// =========================================================================

pub const FBIOGET_VSCREENINFO: c_ulong = 0x4600;
pub const FBIOGET_FSCREENINFO: c_ulong = 0x4602;
pub const FB_TYPE_PACKED_PIXELS: u32 = 0;
pub const FB_VISUAL_TRUECOLOR: u32 = 2;

// =========================================================================
// Device catalog (<sys/device_catalog.h>)
// =========================================================================

pub const DEVICE_CLASS_MEMORY: u64 = 1;
pub const DEVICE_CLASS_TTY: u64 = 2;
pub const DEVICE_CLASS_PTY: u64 = 3;
pub const DEVICE_CLASS_INPUT: u64 = 4;
pub const DEVICE_CLASS_FRAMEBUFFER: u64 = 5;
pub const DEVICE_CLASS_BLOCK: u64 = 6;
pub const DEVICE_CLASS_NETWORK: u64 = 7;
pub const DEVICE_CLASS_SERIAL: u64 = 8;
pub const DEVICE_CLASS_MISC: u64 = 9;

pub const DEVICE_KIND_NONE: u8 = 0;
pub const DEVICE_KIND_CHARACTER: u8 = 1;
pub const DEVICE_KIND_BLOCK: u8 = 2;

pub const DEVICE_RECORD_ALIAS: u8 = 0x01;
pub const DEVICE_RECORD_FACTORY: u8 = 0x02;

pub const DEVICE_NAME_MAX: usize = 64;

pub const DEVICE_CHANGE_BOUNDARY: u64 = 0;
pub const DEVICE_CHANGE_ADDED: u64 = 1;
pub const DEVICE_CHANGE_CHANGED: u64 = 2;
pub const DEVICE_CHANGE_REMOVED: u64 = 3;
pub const DEVICE_CHANGE_OVERFLOW: u64 = 4;
pub const DEVICE_CHANGE_RESTART: u64 = 5;

pub const DEVICE_ACTION_ADD: u64 = 1;
pub const DEVICE_ACTION_CHANGE: u64 = 2;
pub const DEVICE_ACTION_REMOVE: u64 = 3;
pub const DEVICE_ACTION_BIND: u64 = 4;
pub const DEVICE_ACTION_UNBIND: u64 = 5;
pub const DEVICE_ACTION_OFFLINE: u64 = 6;
pub const DEVICE_ACTION_ONLINE: u64 = 7;

pub const DEVICE_EVENT_PROPERTIES_MAX: usize = 4096;
pub const DEVICE_EVENT_TRUNCATED: u64 = 1;
pub const DEVICE_OBJECTS_PAGE: usize = 58;

// =========================================================================
// User accounting (<utmpx.h>)
// =========================================================================

pub const EMPTY: c_short = 0;
pub const RUN_LVL: c_short = 1;
pub const BOOT_TIME: c_short = 2;
pub const NEW_TIME: c_short = 3;
pub const OLD_TIME: c_short = 4;
pub const INIT_PROCESS: c_short = 5;
pub const LOGIN_PROCESS: c_short = 6;
pub const USER_PROCESS: c_short = 7;
pub const DEAD_PROCESS: c_short = 8;

// =========================================================================
// Exec operands and the authentication broker (<exec_operands.h>,
// <auth_broker.h>)
// =========================================================================

pub const EXEC_OPERANDS_WORDS: usize = 14;
pub const EXEC_OPERANDS_CAPS: usize = 3;

pub const AUTH_BROKER_LOGIN: c_int = 1;
pub const AUTH_BROKER_SWITCH_USER: c_int = 2;
pub const AUTH_BROKER_SUDO: c_int = 3;
pub const AUTH_BROKER_CHANGE_PASSWORD: c_int = 4;
pub const AUTH_BROKER_REPLY_PROMPT: c_int = 1;
pub const AUTH_BROKER_REPLY_RESULT: c_int = 2;
pub const AUTH_BROKER_STYLE_ECHO_OFF: c_int = 1;
pub const AUTH_BROKER_STYLE_ECHO_ON: c_int = 2;
pub const AUTH_BROKER_STYLE_ERROR: c_int = 3;
pub const AUTH_BROKER_STYLE_INFO: c_int = 4;
pub const AUTH_BROKER_GRANTED: c_int = 0;
pub const AUTH_BROKER_DENIED: c_int = 1;
pub const AUTH_BROKER_ACCOUNT_UNAVAILABLE: c_int = 2;
pub const AUTH_BROKER_NOT_PERMITTED: c_int = 3;
pub const AUTH_BROKER_UNKNOWN_USER: c_int = 4;
pub const AUTH_BROKER_FAILED: c_int = 5;
pub const AUTH_BROKER_SHAPE_ADMINISTRATORS: c_int = 2;
pub const AUTH_BROKER_SHAPE_ORDINARY: c_int = 3;
pub const AUTH_BROKER_USER_MAX: usize = 32;
pub const AUTH_BROKER_TTY_MAX: usize = 32;
pub const AUTH_BROKER_TEXT_MAX: usize = 512;
pub const AUTH_BROKER_ANSWER_MAX: usize = 512;

// =========================================================================
// Password database field bits (<pwd.h>)
// =========================================================================

pub const _PASSWORD_LEN: c_int = 128;
pub const _PWF_NAME: c_int = 1 << 0;
pub const _PWF_PASSWD: c_int = 1 << 1;
pub const _PWF_UID: c_int = 1 << 2;
pub const _PWF_GID: c_int = 1 << 3;
pub const _PWF_CHANGE: c_int = 1 << 4;
pub const _PWF_CLASS: c_int = 1 << 5;
pub const _PWF_GECOS: c_int = 1 << 6;
pub const _PWF_DIR: c_int = 1 << 7;
pub const _PWF_SHELL: c_int = 1 << 8;
pub const _PWF_EXPIRE: c_int = 1 << 9;
pub const _PWF_SOURCE: c_int = 0x3000;
pub const _PWF_FILES: c_int = 0x1000;
pub const _PWF_NIS: c_int = 0x2000;

// =========================================================================
// Dynamic linking (<dlfcn.h>, <link.h>)
// =========================================================================

pub const RTLD_LAZY: c_int = 0x0001;
pub const RTLD_NOW: c_int = 0x0002;
pub const RTLD_NOLOAD: c_int = 0x0004;
pub const RTLD_GLOBAL: c_int = 0x0100;
pub const RTLD_LOCAL: c_int = 0x0000;
pub const RTLD_NODELETE: c_int = 0x1000;
pub const RTLD_DEFAULT: *mut c_void = core::ptr::null_mut();
pub const RTLD_NEXT: *mut c_void = -1isize as *mut c_void;

pub const PT_NULL: u32 = 0;
pub const PT_LOAD: u32 = 1;
pub const PT_DYNAMIC: u32 = 2;
pub const PT_INTERP: u32 = 3;
pub const PT_NOTE: u32 = 4;
pub const PT_SHLIB: u32 = 5;
pub const PT_PHDR: u32 = 6;
pub const PT_TLS: u32 = 7;
pub const PT_GNU_EH_FRAME: u32 = 0x6474e550;
pub const PT_GNU_STACK: u32 = 0x6474e551;
pub const PT_GNU_RELRO: u32 = 0x6474e552;

pub const PF_X: u32 = 0x1;
pub const PF_W: u32 = 0x2;
pub const PF_R: u32 = 0x4;

// =========================================================================
// Network interfaces and routing messages (<net/if.h>, <net/route.h>,
// <net/if_arp.h>, <net/if_types.h>)
// =========================================================================

pub const IF_NAMESIZE: usize = 16;
pub const IFNAMSIZ: usize = IF_NAMESIZE;

pub const LINK_STATE_UNKNOWN: u8 = 0;
pub const LINK_STATE_DOWN: u8 = 1;
pub const LINK_STATE_UP: u8 = 2;

pub const ARPHRD_ETHER: u16 = 1;
pub const ARPHRD_LOOPBACK: u16 = 772;
pub const ARPHRD_NONE: u16 = 0xfffe;

pub const IFT_OTHER: u8 = 0x1;
pub const IFT_ETHER: u8 = 0x6;
pub const IFT_LOOP: u8 = 0x18;

pub const RTM_VERSION: c_int = 5;
// The routing-socket message types; <linux/rtnetlink.h> reuses RTM_NEWADDR
// and RTM_DELADDR with other values, which the `rtnetlink` module carries.
pub const RTM_ADD: c_int = 0x1;
pub const RTM_DELETE: c_int = 0x2;
pub const RTM_CHANGE: c_int = 0x3;
pub const RTM_GET: c_int = 0x4;
pub const RTM_NEWADDR: c_int = 0xc;
pub const RTM_DELADDR: c_int = 0xd;
pub const RTM_IFINFO: c_int = 0xe;

pub const RTF_UP: c_int = 0x1;
pub const RTF_GATEWAY: c_int = 0x2;
pub const RTF_HOST: c_int = 0x4;
pub const RTF_REJECT: c_int = 0x8;
pub const RTF_DYNAMIC: c_int = 0x10;
pub const RTF_DONE: c_int = 0x40;
pub const RTF_STATIC: c_int = 0x800;
pub const RTF_BLACKHOLE: c_int = 0x1000;

pub const RTA_DST: c_int = 0x1;
pub const RTA_GATEWAY: c_int = 0x2;
pub const RTA_NETMASK: c_int = 0x4;
pub const RTA_GENMASK: c_int = 0x8;
pub const RTA_IFP: c_int = 0x10;
pub const RTA_IFA: c_int = 0x20;
pub const RTA_AUTHOR: c_int = 0x40;
pub const RTA_BRD: c_int = 0x80;

pub const RTAX_DST: c_int = 0;
pub const RTAX_GATEWAY: c_int = 1;
pub const RTAX_NETMASK: c_int = 2;
pub const RTAX_GENMASK: c_int = 3;
pub const RTAX_IFP: c_int = 4;
pub const RTAX_IFA: c_int = 5;
pub const RTAX_AUTHOR: c_int = 6;
pub const RTAX_BRD: c_int = 7;
pub const RTAX_MAX: c_int = 8;

pub const NET_RT_DUMP: c_int = 1;
pub const NET_RT_FLAGS: c_int = 2;
pub const NET_RT_IFLIST: c_int = 3;

// =========================================================================
// Netlink sockets (<linux/netlink.h>)
// =========================================================================

pub const NETLINK_ROUTE: c_int = 0;

pub const NLM_F_REQUEST: c_int = 0x1;
pub const NLM_F_MULTI: c_int = 0x2;
pub const NLM_F_ACK: c_int = 0x4;
pub const NLM_F_ECHO: c_int = 0x8;
pub const NLM_F_DUMP_INTR: c_int = 0x10;
pub const NLM_F_ROOT: c_int = 0x100;
pub const NLM_F_MATCH: c_int = 0x200;
pub const NLM_F_ATOMIC: c_int = 0x400;
pub const NLM_F_DUMP: c_int = NLM_F_ROOT | NLM_F_MATCH;
pub const NLM_F_REPLACE: c_int = 0x100;
pub const NLM_F_EXCL: c_int = 0x200;
pub const NLM_F_CREATE: c_int = 0x400;
pub const NLM_F_APPEND: c_int = 0x800;

pub const NLMSG_ALIGNTO: u32 = 4;
pub const NLMSG_NOOP: c_int = 0x1;
pub const NLMSG_ERROR: c_int = 0x2;
pub const NLMSG_DONE: c_int = 0x3;
pub const NLMSG_OVERRUN: c_int = 0x4;
pub const NLMSG_MIN_TYPE: c_int = 0x10;

pub const NLA_ALIGNTO: c_int = 4;

// =========================================================================
// Management information base (<sys/sysctl.h>)
// =========================================================================

pub const CTL_MAXNAME: c_int = 24;

pub const CTLTYPE_NODE: c_int = 1;
pub const CTLTYPE_INT: c_int = 2;
pub const CTLTYPE_STRING: c_int = 3;
pub const CTLTYPE_S64: c_int = 4;
pub const CTLTYPE_OPAQUE: c_int = 5;
pub const CTLTYPE_STRUCT: c_int = CTLTYPE_OPAQUE;
pub const CTLTYPE_UINT: c_int = 6;
pub const CTLTYPE_LONG: c_int = 7;
pub const CTLTYPE_ULONG: c_int = 8;
pub const CTLTYPE_U64: c_int = 9;

pub const CTL_KERN: c_int = 1;
pub const CTL_VM: c_int = 2;
pub const CTL_NET: c_int = 4;
pub const CTL_HW: c_int = 6;
pub const CTL_SECURITY: c_int = 9;

pub const KERN_OSTYPE: c_int = 1;
pub const KERN_OSRELEASE: c_int = 2;
pub const KERN_OSREV: c_int = 3;
pub const KERN_VERSION: c_int = 4;
pub const KERN_MAXPROC: c_int = 6;
pub const KERN_HOSTNAME: c_int = 10;
pub const KERN_CLOCKRATE: c_int = 12;
pub const KERN_PROC: c_int = 14;
pub const KERN_BOOTTIME: c_int = 21;
pub const KERN_OSRELDATE: c_int = 24;
pub const KERN_CPTIME: c_int = 40;

pub const KERN_PROC_ALL: c_int = 0;
pub const KERN_PROC_PID: c_int = 1;
pub const KERN_PROC_PGRP: c_int = 2;
pub const KERN_PROC_SESSION: c_int = 3;
pub const KERN_PROC_TTY: c_int = 4;
pub const KERN_PROC_UID: c_int = 5;
pub const KERN_PROC_RUID: c_int = 6;
pub const KERN_PROC_ARGS: c_int = 7;
pub const KERN_PROC_PATHNAME: c_int = 12;
pub const KERN_PROC_INC_THREAD: c_int = 0x10;

pub const VM_LOADAVG: c_int = 2;

pub const HW_MACHINE: c_int = 1;
pub const HW_MODEL: c_int = 2;
pub const HW_NCPU: c_int = 3;
pub const HW_PHYSMEM: c_int = 5;
pub const HW_USERMEM: c_int = 6;
pub const HW_PAGESIZE: c_int = 7;
pub const HW_MACHINE_ARCH: c_int = 11;

// =========================================================================
// Standard I/O, conversions and searching (<stdio.h>, <stdlib.h>,
// <inttypes.h>, <glob.h>, <fnmatch.h>, <regex.h>, <search.h>, <wordexp.h>,
// <fmtmsg.h>, <ndbm.h>, <crypt.h>, <sys/random.h>)
// =========================================================================

pub const EOF: c_int = -1;
pub const BUFSIZ: c_uint = 1024;
pub const _IOFBF: c_int = 0;
pub const _IOLBF: c_int = 1;
pub const _IONBF: c_int = 2;
pub const FILENAME_MAX: c_uint = 4096;
pub const FOPEN_MAX: c_uint = 16;
pub const TMP_MAX: c_uint = 10000;
pub const L_tmpnam: c_uint = 20;
pub const L_ctermid: c_uint = 32;

pub type fpos_t = off_t;
pub type funopen_readfn = Option<unsafe extern "C" fn(*mut c_void, *mut c_char, c_int) -> c_int>;
pub type funopen_writefn = Option<unsafe extern "C" fn(*mut c_void, *const c_char, c_int) -> c_int>;
pub type funopen_seekfn =
    Option<unsafe extern "C" fn(*mut c_void, c_longlong, c_int) -> c_longlong>;
pub type funopen_closefn = Option<unsafe extern "C" fn(*mut c_void) -> c_int>;

pub const RAND_MAX: c_int = 0x7FFFFFFF;
pub const MB_CUR_MAX: size_t = 4;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct div_t {
    pub quot: c_int,
    pub rem: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ldiv_t {
    pub quot: c_long,
    pub rem: c_long,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct lldiv_t {
    pub quot: c_longlong,
    pub rem: c_longlong,
}

pub type intmax_t = c_long;
pub type uintmax_t = c_ulong;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct imaxdiv_t {
    pub quot: intmax_t,
    pub rem: intmax_t,
}

pub const GLOB_ERR: c_int = 1 << 0;
pub const GLOB_MARK: c_int = 1 << 1;
pub const GLOB_NOSORT: c_int = 1 << 2;
pub const GLOB_DOOFFS: c_int = 1 << 3;
pub const GLOB_NOCHECK: c_int = 1 << 4;
pub const GLOB_APPEND: c_int = 1 << 5;
pub const GLOB_NOESCAPE: c_int = 1 << 6;
pub const GLOB_PERIOD: c_int = 1 << 7;
pub const GLOB_MAGCHAR: c_int = 1 << 8;
pub const GLOB_ALTDIRFUNC: c_int = 1 << 9;
pub const GLOB_BRACE: c_int = 1 << 10;
pub const GLOB_NOMAGIC: c_int = 1 << 11;
pub const GLOB_TILDE: c_int = 1 << 12;
pub const GLOB_ONLYDIR: c_int = 1 << 13;
pub const GLOB_TILDE_CHECK: c_int = 1 << 14;
pub const GLOB_NOSPACE: c_int = 1;
pub const GLOB_ABORTED: c_int = 2;
pub const GLOB_NOMATCH: c_int = 3;

pub const FNM_NOMATCH: c_int = 1;
pub const FNM_NOESCAPE: c_int = 1 << 1;
pub const FNM_PATHNAME: c_int = 1 << 2;
pub const FNM_PERIOD: c_int = 1 << 3;
pub const FNM_CASEFOLD: c_int = 1 << 4;

pub const REG_EXTENDED: c_int = 1;
pub const REG_ICASE: c_int = 2;
pub const REG_NOSUB: c_int = 4;
pub const REG_NEWLINE: c_int = 8;
pub const REG_NOMATCH: c_int = 1;
pub const REG_BADPAT: c_int = 2;
pub const REG_ECOLLATE: c_int = 3;
pub const REG_ECTYPE: c_int = 4;
pub const REG_EESCAPE: c_int = 5;
pub const REG_ESUBREG: c_int = 6;
pub const REG_EBRACK: c_int = 7;
pub const REG_EPAREN: c_int = 8;
pub const REG_EBRACE: c_int = 9;
pub const REG_BADBR: c_int = 10;
pub const REG_ERANGE: c_int = 11;
pub const REG_ESPACE: c_int = 12;
pub const REG_BADRPT: c_int = 13;
pub const REG_NOTBOL: c_int = 1;
pub const REG_NOTEOL: c_int = 2;

pub type regoff_t = ssize_t;

#[repr(C)]
pub struct regex_t {
    pub re_nsub: size_t,
    pub re_program: *mut c_void,
    pub re_cflags: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct regmatch_t {
    pub rm_so: regoff_t,
    pub rm_eo: regoff_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ENTRY {
    pub key: *mut c_char,
    pub data: *mut c_void,
}

// <search.h> enumerations: no negative member.
pub type ACTION = c_uint;
pub const FIND: ACTION = 0;
pub const ENTER: ACTION = 1;
pub type VISIT = c_uint;
pub const preorder: VISIT = 0;
pub const postorder: VISIT = 1;
pub const endorder: VISIT = 2;
pub const leaf: VISIT = 3;
pub type posix_tnode = c_void;

#[repr(C)]
pub struct wordexp_t {
    pub we_wordc: size_t,
    pub we_wordv: *mut *mut c_char,
    pub we_offs: size_t,
}

pub const WRDE_DOOFFS: c_int = 1 << 0;
pub const WRDE_APPEND: c_int = 1 << 1;
pub const WRDE_NOCMD: c_int = 1 << 2;
pub const WRDE_REUSE: c_int = 1 << 3;
pub const WRDE_SHOWERR: c_int = 1 << 4;
pub const WRDE_UNDEF: c_int = 1 << 5;
pub const WRDE_NOSPACE: c_int = 1;
pub const WRDE_BADCHAR: c_int = 2;
pub const WRDE_BADVAL: c_int = 3;
pub const WRDE_CMDSUB: c_int = 4;
pub const WRDE_SYNTAX: c_int = 5;

pub const MM_HARD: c_long = 0x001;
pub const MM_SOFT: c_long = 0x002;
pub const MM_FIRM: c_long = 0x004;
pub const MM_APPL: c_long = 0x008;
pub const MM_UTIL: c_long = 0x010;
pub const MM_OPSYS: c_long = 0x020;
pub const MM_RECOVER: c_long = 0x040;
pub const MM_NRECOV: c_long = 0x080;
pub const MM_PRINT: c_long = 0x100;
pub const MM_CONSOLE: c_long = 0x200;
pub const MM_NOSEV: c_int = 0;
pub const MM_HALT: c_int = 1;
pub const MM_ERROR: c_int = 2;
pub const MM_WARNING: c_int = 3;
pub const MM_INFO: c_int = 4;
pub const MM_NULLLBL: *const c_char = core::ptr::null();
pub const MM_NULLSEV: c_int = 0;
pub const MM_NULLMC: c_long = 0;
pub const MM_NULLTXT: *const c_char = core::ptr::null();
pub const MM_NULLACT: *const c_char = core::ptr::null();
pub const MM_NULLTAG: *const c_char = core::ptr::null();
pub const MM_OK: c_int = 0;
pub const MM_NOTOK: c_int = -1;
pub const MM_NOMSG: c_int = 1;
pub const MM_NOCON: c_int = 4;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct datum {
    pub dptr: *mut c_void,
    pub dsize: size_t,
}

pub enum DBM {}
pub const DBM_INSERT: c_int = 0;
pub const DBM_REPLACE: c_int = 1;

#[repr(C)]
pub struct crypt_data {
    pub output: [c_char; 256],
    pub internal: [c_char; 256],
}

pub const GRND_NONBLOCK: c_uint = 0x0001;
pub const GRND_RANDOM: c_uint = 0x0002;
pub const GRND_INSECURE: c_uint = 0x0004;

// =========================================================================
// Locales, wide characters, message catalogs, conversions and mathematics
// (<locale.h>, <langinfo.h>, <nl_types.h>, <wchar.h>, <wctype.h>,
// <uchar.h>, <iconv.h>, <math.h>, <complex.h>, <fenv.h>)
// =========================================================================

pub const LC_CTYPE: c_int = 0;
pub const LC_NUMERIC: c_int = 1;
pub const LC_TIME: c_int = 2;
pub const LC_COLLATE: c_int = 3;
pub const LC_MONETARY: c_int = 4;
pub const LC_MESSAGES: c_int = 5;
pub const LC_ALL: c_int = 6;
pub const LC_CTYPE_MASK: c_int = 1 << LC_CTYPE;
pub const LC_NUMERIC_MASK: c_int = 1 << LC_NUMERIC;
pub const LC_TIME_MASK: c_int = 1 << LC_TIME;
pub const LC_COLLATE_MASK: c_int = 1 << LC_COLLATE;
pub const LC_MONETARY_MASK: c_int = 1 << LC_MONETARY;
pub const LC_MESSAGES_MASK: c_int = 1 << LC_MESSAGES;
pub const LC_ALL_MASK: c_int = LC_CTYPE_MASK
    | LC_NUMERIC_MASK
    | LC_TIME_MASK
    | LC_COLLATE_MASK
    | LC_MONETARY_MASK
    | LC_MESSAGES_MASK;
pub const LC_GLOBAL_LOCALE: locale_t = -1isize as locale_t;

#[repr(C)]
pub struct lconv {
    pub decimal_point: *mut c_char,
    pub thousands_sep: *mut c_char,
    pub grouping: *mut c_char,
    pub int_curr_symbol: *mut c_char,
    pub currency_symbol: *mut c_char,
    pub mon_decimal_point: *mut c_char,
    pub mon_thousands_sep: *mut c_char,
    pub mon_grouping: *mut c_char,
    pub positive_sign: *mut c_char,
    pub negative_sign: *mut c_char,
    pub int_frac_digits: c_char,
    pub frac_digits: c_char,
    pub p_cs_precedes: c_char,
    pub p_sep_by_space: c_char,
    pub n_cs_precedes: c_char,
    pub n_sep_by_space: c_char,
    pub p_sign_posn: c_char,
    pub n_sign_posn: c_char,
    pub int_p_cs_precedes: c_char,
    pub int_p_sep_by_space: c_char,
    pub int_n_cs_precedes: c_char,
    pub int_n_sep_by_space: c_char,
    pub int_p_sign_posn: c_char,
    pub int_n_sign_posn: c_char,
}

pub type nl_item = c_int;
pub type nl_catd = *mut c_void;
pub const NL_SETD: c_int = 1;
pub const NL_CAT_LOCALE: c_int = 1;

// Item numbers of nl_langinfo() (<langinfo.h>)
pub const D_T_FMT: nl_item = 0;
pub const D_FMT: nl_item = 1;
pub const T_FMT: nl_item = 2;
pub const T_FMT_AMPM: nl_item = 3;
pub const AM_STR: nl_item = 4;
pub const PM_STR: nl_item = 5;
pub const DAY_1: nl_item = 6;
pub const DAY_2: nl_item = 7;
pub const DAY_3: nl_item = 8;
pub const DAY_4: nl_item = 9;
pub const DAY_5: nl_item = 10;
pub const DAY_6: nl_item = 11;
pub const DAY_7: nl_item = 12;
pub const ABDAY_1: nl_item = 13;
pub const ABDAY_2: nl_item = 14;
pub const ABDAY_3: nl_item = 15;
pub const ABDAY_4: nl_item = 16;
pub const ABDAY_5: nl_item = 17;
pub const ABDAY_6: nl_item = 18;
pub const ABDAY_7: nl_item = 19;
pub const MON_1: nl_item = 20;
pub const MON_2: nl_item = 21;
pub const MON_3: nl_item = 22;
pub const MON_4: nl_item = 23;
pub const MON_5: nl_item = 24;
pub const MON_6: nl_item = 25;
pub const MON_7: nl_item = 26;
pub const MON_8: nl_item = 27;
pub const MON_9: nl_item = 28;
pub const MON_10: nl_item = 29;
pub const MON_11: nl_item = 30;
pub const MON_12: nl_item = 31;
pub const ALTMON_1: nl_item = 32;
pub const ALTMON_2: nl_item = 33;
pub const ALTMON_3: nl_item = 34;
pub const ALTMON_4: nl_item = 35;
pub const ALTMON_5: nl_item = 36;
pub const ALTMON_6: nl_item = 37;
pub const ALTMON_7: nl_item = 38;
pub const ALTMON_8: nl_item = 39;
pub const ALTMON_9: nl_item = 40;
pub const ALTMON_10: nl_item = 41;
pub const ALTMON_11: nl_item = 42;
pub const ALTMON_12: nl_item = 43;
pub const ABMON_1: nl_item = 44;
pub const ABMON_2: nl_item = 45;
pub const ABMON_3: nl_item = 46;
pub const ABMON_4: nl_item = 47;
pub const ABMON_5: nl_item = 48;
pub const ABMON_6: nl_item = 49;
pub const ABMON_7: nl_item = 50;
pub const ABMON_8: nl_item = 51;
pub const ABMON_9: nl_item = 52;
pub const ABMON_10: nl_item = 53;
pub const ABMON_11: nl_item = 54;
pub const ABMON_12: nl_item = 55;
pub const RADIXCHAR: nl_item = 56;
pub const THOUSEP: nl_item = 57;
pub const YESEXPR: nl_item = 58;
pub const NOEXPR: nl_item = 59;
pub const CRNCYSTR: nl_item = 60;
pub const ERA: nl_item = 61;
pub const D_MD_ORDER: nl_item = 62;
pub const CODESET: nl_item = 63;
pub const ABALTMON_1: nl_item = 64;
pub const ABALTMON_2: nl_item = 65;
pub const ABALTMON_3: nl_item = 66;
pub const ABALTMON_4: nl_item = 67;
pub const ABALTMON_5: nl_item = 68;
pub const ABALTMON_6: nl_item = 69;
pub const ABALTMON_7: nl_item = 70;
pub const ABALTMON_8: nl_item = 71;
pub const ABALTMON_9: nl_item = 72;
pub const ABALTMON_10: nl_item = 73;
pub const ABALTMON_11: nl_item = 74;
pub const ABALTMON_12: nl_item = 75;
pub const ERA_D_FMT: nl_item = 76;
pub const ALT_DIGITS: nl_item = 77;
pub const ERA_D_T_FMT: nl_item = 78;
pub const ERA_T_FMT: nl_item = 79;

// <bits/mbstate_t.h>: a zero-filled object is the initial state.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mbstate_t {
    __state: c_uint,
    __value: c_uint,
}

pub type wctype_t = c_ulong;
pub type wctrans_t = c_ulong;
pub type char16_t = c_ushort;
pub type char32_t = c_uint;
pub type iconv_t = *mut c_void;

pub const WEOF: wint_t = 0xFFFFFFFF;
pub const WINT_MIN: wint_t = 0;
pub const WINT_MAX: wint_t = 0xFFFFFFFF;
#[cfg(target_arch = "x86_64")]
pub const WCHAR_MIN: wchar_t = -2147483647 - 1;
#[cfg(target_arch = "x86_64")]
pub const WCHAR_MAX: wchar_t = 2147483647;
#[cfg(target_arch = "aarch64")]
pub const WCHAR_MIN: wchar_t = 0;
#[cfg(target_arch = "aarch64")]
pub const WCHAR_MAX: wchar_t = 4294967295;

pub const FP_NAN: c_int = 0;
pub const FP_INFINITE: c_int = 1;
pub const FP_ZERO: c_int = 2;
pub const FP_SUBNORMAL: c_int = 3;
pub const FP_NORMAL: c_int = 4;
pub const FP_ILOGB0: c_int = -2147483647 - 1;
pub const FP_ILOGBNAN: c_int = -2147483647 - 1;
pub const MATH_ERRNO: c_int = 1;
pub const MATH_ERREXCEPT: c_int = 2;
pub const math_errhandling: c_int = MATH_ERRNO | MATH_ERREXCEPT;
#[cfg(target_arch = "aarch64")]
pub const FP_FAST_FMA: c_int = 1;
#[cfg(target_arch = "aarch64")]
pub const FP_FAST_FMAF: c_int = 1;
pub type float_t = c_float;
pub type double_t = c_double;
pub const MAXFLOAT: c_float = 3.40282346638528859812e+38;
pub const M_E: c_double = 2.7182818284590452354;
pub const M_EGAMMA: c_double = 0.57721566490153286061;
pub const M_LOG2E: c_double = 1.4426950408889634074;
pub const M_LOG10E: c_double = 0.43429448190325182765;
pub const M_LN2: c_double = 0.69314718055994530942;
pub const M_LN10: c_double = 2.30258509299404568402;
pub const M_PHI: c_double = 1.61803398874989484820;
pub const M_PI: c_double = 3.14159265358979323846;
pub const M_PI_2: c_double = 1.57079632679489661923;
pub const M_PI_4: c_double = 0.78539816339744830962;
pub const M_1_PI: c_double = 0.31830988618379067154;
pub const M_1_SQRTPI: c_double = 0.56418958354775628695;
pub const M_2_PI: c_double = 0.63661977236758134308;
pub const M_2_SQRTPI: c_double = 1.12837916709551257390;
pub const M_SQRT2: c_double = 1.41421356237309504880;
pub const M_SQRT3: c_double = 1.73205080756887729353;
pub const M_SQRT1_2: c_double = 0.70710678118654752440;
pub const M_SQRT1_3: c_double = 0.57735026918962576451;
pub const M_Ef: c_float = 2.7182818284590452354;
pub const M_EGAMMAf: c_float = 0.57721566490153286061;
pub const M_LOG2Ef: c_float = 1.4426950408889634074;
pub const M_LOG10Ef: c_float = 0.43429448190325182765;
pub const M_LN2f: c_float = 0.69314718055994530942;
pub const M_LN10f: c_float = 2.30258509299404568402;
pub const M_PHIf: c_float = 1.61803398874989484820;
pub const M_PIf: c_float = 3.14159265358979323846;
pub const M_PI_2f: c_float = 1.57079632679489661923;
pub const M_PI_4f: c_float = 0.78539816339744830962;
pub const M_1_PIf: c_float = 0.31830988618379067154;
pub const M_1_SQRTPIf: c_float = 0.56418958354775628695;
pub const M_2_PIf: c_float = 0.63661977236758134308;
pub const M_2_SQRTPIf: c_float = 1.12837916709551257390;
pub const M_SQRT2f: c_float = 1.41421356237309504880;
pub const M_SQRT3f: c_float = 1.73205080756887729353;
pub const M_SQRT1_2f: c_float = 0.70710678118654752440;
pub const M_SQRT1_3f: c_float = 0.57735026918962576451;

// The <complex.h> types, which C passes and returns as a pair of real and
// imaginary parts in the floating-point registers of both targets.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct c_double_complex {
    pub re: c_double,
    pub im: c_double,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct c_float_complex {
    pub re: c_float,
    pub im: c_float,
}

#[cfg(target_arch = "x86_64")]
pub const FE_INVALID: c_int = 0x01;
#[cfg(target_arch = "x86_64")]
pub const FE_DIVBYZERO: c_int = 0x04;
#[cfg(target_arch = "x86_64")]
pub const FE_OVERFLOW: c_int = 0x08;
#[cfg(target_arch = "x86_64")]
pub const FE_UNDERFLOW: c_int = 0x10;
#[cfg(target_arch = "x86_64")]
pub const FE_INEXACT: c_int = 0x20;
#[cfg(target_arch = "x86_64")]
pub const FE_TONEAREST: c_int = 0x000;
#[cfg(target_arch = "x86_64")]
pub const FE_DOWNWARD: c_int = 0x400;
#[cfg(target_arch = "x86_64")]
pub const FE_UPWARD: c_int = 0x800;
#[cfg(target_arch = "x86_64")]
pub const FE_TOWARDZERO: c_int = 0xc00;
#[cfg(target_arch = "x86_64")]
pub type fexcept_t = c_ushort;

// The x87 environment (28 bytes) followed by MXCSR.
#[cfg(target_arch = "x86_64")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct fenv_t {
    pub __control_word: c_ushort,
    __reserved1: c_ushort,
    pub __status_word: c_ushort,
    __reserved2: c_ushort,
    pub __tags: c_ushort,
    __reserved3: c_ushort,
    pub __instruction_offset: c_uint,
    pub __instruction_selector: c_ushort,
    pub __opcode: c_ushort,
    pub __data_offset: c_uint,
    pub __data_selector: c_ushort,
    __reserved4: c_ushort,
    pub __mxcsr: c_uint,
}

#[cfg(target_arch = "aarch64")]
pub const FE_INVALID: c_int = 0x01;
#[cfg(target_arch = "aarch64")]
pub const FE_DIVBYZERO: c_int = 0x02;
#[cfg(target_arch = "aarch64")]
pub const FE_OVERFLOW: c_int = 0x04;
#[cfg(target_arch = "aarch64")]
pub const FE_UNDERFLOW: c_int = 0x08;
#[cfg(target_arch = "aarch64")]
pub const FE_INEXACT: c_int = 0x10;
#[cfg(target_arch = "aarch64")]
pub const FE_TONEAREST: c_int = 0x000000;
#[cfg(target_arch = "aarch64")]
pub const FE_UPWARD: c_int = 0x400000;
#[cfg(target_arch = "aarch64")]
pub const FE_DOWNWARD: c_int = 0x800000;
#[cfg(target_arch = "aarch64")]
pub const FE_TOWARDZERO: c_int = 0xc00000;
#[cfg(target_arch = "aarch64")]
pub type fexcept_t = c_uint;

// FPCR and FPSR.
#[cfg(target_arch = "aarch64")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct fenv_t {
    pub __fpcr: c_uint,
    pub __fpsr: c_uint,
}

pub const FE_ALL_EXCEPT: c_int =
    FE_INVALID | FE_DIVBYZERO | FE_OVERFLOW | FE_UNDERFLOW | FE_INEXACT;
// The environment in effect at program start.
pub const FE_DFL_ENV: *const fenv_t = -1isize as *const fenv_t;

// =========================================================================
// Struct definitions
// =========================================================================

// timespec — <time.h>: tv_sec: i64, tv_nsec: i64
#[repr(C)]
#[derive(Clone, Copy)]
pub struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: c_long,
}

// timeval — <sys/time.h>: tv_sec: i64, tv_usec: i64
#[repr(C)]
#[derive(Clone, Copy)]
pub struct timeval {
    pub tv_sec: time_t,
    pub tv_usec: suseconds_t,
}

// <time.h>
#[repr(C)]
#[derive(Clone, Copy)]
pub struct itimerspec {
    pub it_interval: timespec,
    pub it_value: timespec,
}

// <sys/time.h>
#[repr(C)]
#[derive(Clone, Copy)]
pub struct itimerval {
    pub it_interval: timeval,
    pub it_value: timeval,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct timezone {
    pub tz_minuteswest: c_int,
    pub tz_dsttime: c_int,
}

// <sys/times.h>: CLK_TCK ticks
#[repr(C)]
#[derive(Clone, Copy)]
pub struct tms {
    pub tms_utime: clock_t,
    pub tms_stime: clock_t,
    pub tms_cutime: clock_t,
    pub tms_cstime: clock_t,
}

// stat — <sys/stat.h>, 224 bytes on both targets. The four timespec members
// (st_atim, st_mtim, st_ctim, st_birthtim) appear as their seconds and
// nanoseconds, the names the header's macros give them.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct stat {
    pub st_dev: dev_t,
    pub st_ino: ino_t,
    pub st_nlink: nlink_t,
    pub st_mode: u16,
    __st_padding0: i16,
    pub st_uid: uid_t,
    pub st_gid: gid_t,
    __st_padding1: i32,
    pub st_rdev: dev_t,
    pub st_atime: time_t,
    pub st_atime_nsec: c_long,
    pub st_mtime: time_t,
    pub st_mtime_nsec: c_long,
    pub st_ctime: time_t,
    pub st_ctime_nsec: c_long,
    pub st_birthtime: time_t,
    pub st_birthtime_nsec: c_long,
    pub st_size: off_t,
    pub st_blocks: blkcnt_t,
    pub st_blksize: i32,
    pub st_flags: u32,
    pub st_gen: u64,
    __st_spare: [u64; 10],
}

// dirent — <dirent.h>
// Fields: d_ino(u64), d_off(i64), d_reclen(u16), d_type(u8),
//   d_pad0(u8), d_namlen(u16), d_pad1(u16), d_name([u8;256])
#[repr(C)]
pub struct dirent {
    pub d_ino: ino_t,
    pub d_off: i64,
    pub d_reclen: u16,
    pub d_type: u8,
    __d_pad0: u8,
    pub d_namlen: u16,
    __d_pad1: u16,
    pub d_name: [c_char; 256],
}

// posix_dent — <dirent.h>: the record posix_getdents writes, d_reclen bytes
// long with a NUL-terminated name.
#[repr(C)]
pub struct posix_dent {
    pub d_ino: ino_t,
    pub d_reclen: reclen_t,
    pub d_type: c_uchar,
    pub d_name: [c_char; 0],
}

// sockaddr
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [c_char; 14],
}

// sockaddr_un — <sys/un.h>: sun_family(u16), sun_path([char;108])
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [c_char; 108],
}

// in_addr
#[repr(C)]
#[derive(Clone, Copy)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}

// sockaddr_in
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}

// <netinet/in.h>
#[repr(C)]
#[derive(Clone, Copy)]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: u32,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: u32,
}

// sockaddr_storage — <sys/socket.h>, 128 bytes
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    __ss_pad1: [c_char; 6],
    __ss_align: c_longlong,
    __ss_pad2: [c_char; 112],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct if_nameindex {
    pub if_index: c_uint,
    pub if_name: *mut c_char,
}

// ifreq — <net/if.h>, 32 bytes. ifr_ifindex and ifr_metric name ifru_ivalue.
#[repr(C)]
#[derive(Clone, Copy)]
pub union __c_anonymous_ifr_ifru {
    pub ifru_addr: sockaddr,
    pub ifru_dstaddr: sockaddr,
    pub ifru_broadaddr: sockaddr,
    pub ifru_netmask: sockaddr,
    pub ifru_hwaddr: sockaddr,
    pub ifru_flags: c_short,
    pub ifru_ivalue: c_int,
    pub ifru_mtu: c_int,
    pub ifru_newname: [c_char; IFNAMSIZ],
    pub ifru_data: *mut c_char,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ifreq {
    pub ifr_name: [c_char; IFNAMSIZ],
    pub ifr_ifru: __c_anonymous_ifr_ifru,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union __c_anonymous_ifc_ifcu {
    pub ifcu_buf: *mut c_char,
    pub ifcu_req: *mut ifreq,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ifconf {
    pub ifc_len: c_int,
    pub ifc_ifcu: __c_anonymous_ifc_ifcu,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union __c_anonymous_ifa_ifu {
    pub ifu_broadaddr: *mut sockaddr,
    pub ifu_dstaddr: *mut sockaddr,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ifaddrs {
    pub ifa_next: *mut ifaddrs,
    pub ifa_name: *mut c_char,
    pub ifa_flags: c_uint,
    pub ifa_addr: *mut sockaddr,
    pub ifa_netmask: *mut sockaddr,
    pub ifa_ifu: __c_anonymous_ifa_ifu,
    pub ifa_data: *mut c_void,
}

// if_data — <net/if.h>: link data and counters, 152 bytes
#[repr(C)]
#[derive(Clone, Copy)]
pub struct if_data {
    pub ifi_type: u8,
    pub ifi_physical: u8,
    pub ifi_hdrlen: u8,
    pub ifi_link_state: u8,
    pub ifi_vhid: u8,
    pub _ifi_spare0: u8,
    pub ifi_addrlen: u8,
    pub _ifi_spare1: u8,
    pub ifi_baudrate: u64,
    pub ifi_mtu: u32,
    pub ifi_metric: u32,
    pub ifi_ipackets: u64,
    pub ifi_ierrors: u64,
    pub ifi_opackets: u64,
    pub ifi_oerrors: u64,
    pub ifi_collisions: u64,
    pub ifi_ibytes: u64,
    pub ifi_obytes: u64,
    pub ifi_imcasts: u64,
    pub ifi_omcasts: u64,
    pub ifi_iqdrops: u64,
    pub ifi_oqdrops: u64,
    pub ifi_noproto: u64,
    pub ifi_hwassist: u64,
    pub ifi_epoch: u64,
    pub ifi_lastchange: timeval,
}

// if_msghdr — <net/if.h>: RTM_IFINFO, 176 bytes
#[repr(C)]
#[derive(Clone, Copy)]
pub struct if_msghdr {
    pub ifm_msglen: c_ushort,
    pub ifm_version: c_uchar,
    pub ifm_type: c_uchar,
    pub ifm_addrs: c_int,
    pub ifm_flags: c_int,
    pub ifm_index: c_ushort,
    pub _ifm_spare1: c_ushort,
    pub _ifm_spare2: u64,
    pub ifm_data: if_data,
}

// ifa_msghdr — <net/if.h>: RTM_NEWADDR, 24 bytes
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ifa_msghdr {
    pub ifam_msglen: c_ushort,
    pub ifam_version: c_uchar,
    pub ifam_type: c_uchar,
    pub ifam_addrs: c_int,
    pub ifam_flags: c_int,
    pub ifam_index: c_ushort,
    pub _ifam_spare1: c_ushort,
    pub ifam_metric: c_int,
    pub _ifam_spare2: c_int,
}

// <net/route.h>
#[repr(C)]
#[derive(Clone, Copy)]
pub struct rt_metrics {
    pub rmx_locks: c_ulong,
    pub rmx_mtu: c_ulong,
    pub rmx_hopcount: c_ulong,
    pub rmx_expire: c_ulong,
    pub rmx_recvpipe: c_ulong,
    pub rmx_sendpipe: c_ulong,
    pub rmx_ssthresh: c_ulong,
    pub rmx_rtt: c_ulong,
    pub rmx_rttvar: c_ulong,
    pub rmx_pksent: c_ulong,
    pub rmx_weight: c_ulong,
    pub rmx_nhidx: c_ulong,
    pub rmx_filler: [c_ulong; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct rt_msghdr {
    pub rtm_msglen: c_ushort,
    pub rtm_version: c_uchar,
    pub rtm_type: c_uchar,
    pub rtm_index: c_ushort,
    pub _rtm_spare1: c_ushort,
    pub rtm_flags: c_int,
    pub rtm_addrs: c_int,
    pub rtm_pid: pid_t,
    pub rtm_seq: c_int,
    pub rtm_errno: c_int,
    pub rtm_fmask: c_int,
    pub rtm_inits: c_ulong,
    pub rtm_rmx: rt_metrics,
}

// sockaddr_dl — <net/if_dl.h>: the routing-message layout (length byte,
// family byte), not the sa_family_t-first layout of <sys/socket.h>
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sockaddr_dl {
    pub sdl_len: c_uchar,
    pub sdl_family: c_uchar,
    pub sdl_index: c_ushort,
    pub sdl_type: c_uchar,
    pub sdl_nlen: c_uchar,
    pub sdl_alen: c_uchar,
    pub sdl_slen: c_uchar,
    pub sdl_data: [c_char; 46],
}

// <linux/netlink.h>
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sockaddr_nl {
    pub nl_family: sa_family_t,
    pub nl_pad: c_ushort,
    pub nl_pid: u32,
    pub nl_groups: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct nlmsghdr {
    pub nlmsg_len: u32,
    pub nlmsg_type: u16,
    pub nlmsg_flags: u16,
    pub nlmsg_seq: u32,
    pub nlmsg_pid: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct nlmsgerr {
    pub error: c_int,
    pub msg: nlmsghdr,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct nlattr {
    pub nla_len: u16,
    pub nla_type: u16,
}

// pollfd — <poll.h>: fd(i32), events(i16), revents(i16)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct pollfd {
    pub fd: c_int,
    pub events: c_short,
    pub revents: c_short,
}

// epoll_event — natural C layout: the 64-bit data word is 8-byte aligned,
// matching the C library's record (16 bytes).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct epoll_event {
    pub events: u32,
    pub u64: u64,
}

// addrinfo
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: c_int,
    pub ai_family: c_int,
    pub ai_socktype: c_int,
    pub ai_protocol: c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut c_char,
    pub ai_next: *mut addrinfo,
}

#[repr(C)]
pub struct hostent {
    pub h_name: *mut c_char,
    pub h_aliases: *mut *mut c_char,
    pub h_addrtype: c_int,
    pub h_length: c_int,
    pub h_addr_list: *mut *mut c_char,
}

#[repr(C)]
pub struct netent {
    pub n_name: *mut c_char,
    pub n_aliases: *mut *mut c_char,
    pub n_addrtype: c_int,
    pub n_net: u32,
}

#[repr(C)]
pub struct protoent {
    pub p_name: *mut c_char,
    pub p_aliases: *mut *mut c_char,
    pub p_proto: c_int,
}

#[repr(C)]
pub struct servent {
    pub s_name: *mut c_char,
    pub s_aliases: *mut *mut c_char,
    pub s_port: c_int,
    pub s_proto: *mut c_char,
}

// sigaction — <signal.h>: handler, 64-bit mask, flags, restorer
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sigaction {
    pub sa_sigaction: sighandler_t,
    pub sa_mask: sigset_t,
    pub sa_flags: c_int,
    pub sa_restorer: Option<unsafe extern "C" fn()>,
}

// siginfo_t — <signal.h>: 128 bytes, the generic Linux member offsets. The
// union of per-cause members starts at byte 16; the accessors read the
// member the header's macro names. Both targets are little-endian, so the
// low half of a word is the first 32-bit member at its offset.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct siginfo_t {
    pub si_signo: c_int,
    pub si_errno: c_int,
    pub si_code: c_int,
    _pad0: c_int,
    _fields: [u64; 14],
}

impl siginfo_t {
    pub unsafe fn si_addr(&self) -> *mut c_void {
        self._fields[0] as *mut c_void
    }
    pub unsafe fn si_addr_lsb(&self) -> c_short {
        self._fields[1] as u16 as c_short
    }
    pub unsafe fn si_pid(&self) -> pid_t {
        self._fields[0] as u32 as pid_t
    }
    pub unsafe fn si_uid(&self) -> uid_t {
        (self._fields[0] >> 32) as uid_t
    }
    pub unsafe fn si_status(&self) -> c_int {
        self._fields[1] as u32 as c_int
    }
    pub unsafe fn si_utime(&self) -> clock_t {
        self._fields[2] as clock_t
    }
    pub unsafe fn si_stime(&self) -> clock_t {
        self._fields[3] as clock_t
    }
    pub unsafe fn si_value(&self) -> sigval {
        sigval { sival_ptr: self._fields[1] as *mut c_void }
    }
    pub unsafe fn si_band(&self) -> c_long {
        self._fields[0] as c_long
    }
    pub unsafe fn si_fd(&self) -> c_int {
        self._fields[1] as u32 as c_int
    }
    pub unsafe fn si_timerid(&self) -> c_int {
        self._fields[0] as u32 as c_int
    }
    pub unsafe fn si_overrun(&self) -> c_int {
        (self._fields[0] >> 32) as u32 as c_int
    }
}

// struct sigevent — <signal.h>, 64 bytes. The thread id of SIGEV_THREAD_ID
// (sigev_notify_thread_id) shares its word with sigev_notify_function.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sigevent {
    pub sigev_value: sigval,
    pub sigev_signo: c_int,
    pub sigev_notify: c_int,
    pub sigev_notify_function: Option<extern "C" fn(sigval)>,
    pub sigev_notify_attributes: *mut pthread_attr_t,
    _pad: [c_int; 8],
}

// union sigval
#[repr(C)]
#[derive(Clone, Copy)]
pub union sigval {
    pub sival_int: c_int,
    pub sival_ptr: *mut c_void,
}

// stack_t for sigaltstack
#[repr(C)]
#[derive(Clone, Copy)]
pub struct stack_t {
    pub ss_sp: *mut c_void,
    pub ss_flags: c_int,
    pub ss_size: size_t,
}

// The interrupted context a handler receives (<sys/ucontext.h>): the Linux C
// library's member layout of each architecture, with this library's 64-bit
// signal mask.
#[cfg(target_arch = "x86_64")]
pub type greg_t = c_longlong;
#[cfg(target_arch = "x86_64")]
pub const NGREG: usize = 23;
#[cfg(target_arch = "x86_64")]
pub type gregset_t = [greg_t; NGREG];

#[cfg(target_arch = "x86_64")]
pub const REG_R8: c_int = 0;
#[cfg(target_arch = "x86_64")]
pub const REG_R9: c_int = 1;
#[cfg(target_arch = "x86_64")]
pub const REG_R10: c_int = 2;
#[cfg(target_arch = "x86_64")]
pub const REG_R11: c_int = 3;
#[cfg(target_arch = "x86_64")]
pub const REG_R12: c_int = 4;
#[cfg(target_arch = "x86_64")]
pub const REG_R13: c_int = 5;
#[cfg(target_arch = "x86_64")]
pub const REG_R14: c_int = 6;
#[cfg(target_arch = "x86_64")]
pub const REG_R15: c_int = 7;
#[cfg(target_arch = "x86_64")]
pub const REG_RDI: c_int = 8;
#[cfg(target_arch = "x86_64")]
pub const REG_RSI: c_int = 9;
#[cfg(target_arch = "x86_64")]
pub const REG_RBP: c_int = 10;
#[cfg(target_arch = "x86_64")]
pub const REG_RBX: c_int = 11;
#[cfg(target_arch = "x86_64")]
pub const REG_RDX: c_int = 12;
#[cfg(target_arch = "x86_64")]
pub const REG_RAX: c_int = 13;
#[cfg(target_arch = "x86_64")]
pub const REG_RCX: c_int = 14;
#[cfg(target_arch = "x86_64")]
pub const REG_RSP: c_int = 15;
#[cfg(target_arch = "x86_64")]
pub const REG_RIP: c_int = 16;
#[cfg(target_arch = "x86_64")]
pub const REG_EFL: c_int = 17;
#[cfg(target_arch = "x86_64")]
pub const REG_CSGSFS: c_int = 18;
#[cfg(target_arch = "x86_64")]
pub const REG_ERR: c_int = 19;
#[cfg(target_arch = "x86_64")]
pub const REG_TRAPNO: c_int = 20;
#[cfg(target_arch = "x86_64")]
pub const REG_OLDMASK: c_int = 21;
#[cfg(target_arch = "x86_64")]
pub const REG_CR2: c_int = 22;

#[cfg(target_arch = "x86_64")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _libc_fpxreg {
    pub significand: [c_ushort; 4],
    pub exponent: c_ushort,
    __reserved: [c_ushort; 3],
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _libc_xmmreg {
    pub element: [u32; 4],
}

// The FXSAVE image, 512 bytes.
#[cfg(target_arch = "x86_64")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _libc_fpstate {
    pub cwd: u16,
    pub swd: u16,
    pub ftw: u16,
    pub fop: u16,
    pub rip: u64,
    pub rdp: u64,
    pub mxcsr: u32,
    pub mxcr_mask: u32,
    pub _st: [_libc_fpxreg; 8],
    pub _xmm: [_libc_xmmreg; 16],
    __reserved: [u32; 24],
}

#[cfg(target_arch = "x86_64")]
pub type fpregset_t = *mut _libc_fpstate;

// 256 bytes.
#[cfg(target_arch = "x86_64")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mcontext_t {
    pub gregs: gregset_t,
    pub fpregs: fpregset_t,
    __reserved: [c_ulonglong; 8],
}

// 816 bytes; __fpregs_mem is 16-byte aligned, at offset 304.
#[cfg(target_arch = "x86_64")]
#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub struct ucontext_t {
    pub uc_flags: c_ulong,
    pub uc_link: *mut ucontext_t,
    pub uc_stack: stack_t,
    pub uc_mcontext: mcontext_t,
    pub uc_sigmask: sigset_t,
    __fpregs_mem: _libc_fpstate,
}

// A record in mcontext_t's reserved area: the FP/SIMD record first, then a
// terminating record with magic and size zero.
#[cfg(target_arch = "aarch64")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _aarch64_ctx {
    pub magic: u32,
    pub size: u32,
}

#[cfg(target_arch = "aarch64")]
pub const FPSIMD_MAGIC: u32 = 0x46508001;

#[cfg(target_arch = "aarch64")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct fpsimd_context {
    pub head: _aarch64_ctx,
    pub fpsr: u32,
    pub fpcr: u32,
    pub vregs: [u128; 32],
}

// 4384 bytes; the reserved area is 16-byte aligned, at offset 288.
#[cfg(target_arch = "aarch64")]
#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub struct mcontext_t {
    pub fault_address: c_ulonglong,
    pub regs: [c_ulonglong; 31],
    pub sp: c_ulonglong,
    pub pc: c_ulonglong,
    pub pstate: c_ulonglong,
    __pad0: c_ulonglong,
    pub __reserved: [c_uchar; 4096],
}

// 4432 bytes.
#[cfg(target_arch = "aarch64")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ucontext_t {
    pub uc_flags: c_ulong,
    pub uc_link: *mut ucontext_t,
    pub uc_stack: stack_t,
    pub uc_sigmask: sigset_t,
    pub uc_mcontext: mcontext_t,
}

// The pthread object types of <bits/pthreadtypes.h>: the header's sizes and
// alignments (the same on both targets); the library owns the contents.
#[repr(C, align(8))]
pub struct pthread_attr_t {
    __size: [u8; 56],
}

#[repr(C, align(8))]
pub struct pthread_mutex_t {
    __size: [u8; 40],
}

#[repr(C, align(4))]
pub struct pthread_mutexattr_t {
    __size: [u8; 4],
}

#[repr(C, align(8))]
pub struct pthread_cond_t {
    __size: [u8; 48],
}

#[repr(C, align(4))]
pub struct pthread_condattr_t {
    __size: [u8; 4],
}

#[repr(C, align(8))]
pub struct pthread_rwlock_t {
    __size: [u8; 56],
}

#[repr(C, align(8))]
pub struct pthread_rwlockattr_t {
    __size: [u8; 8],
}

#[repr(C, align(8))]
pub struct pthread_barrier_t {
    __size: [u8; 32],
}

#[repr(C, align(4))]
pub struct pthread_barrierattr_t {
    __size: [u8; 4],
}

// The <pthread.h> initializers: zero bytes, except the mutex kind word
// (bytes 8..12, little-endian on both targets) of the recursive and
// error-checking forms.
const fn mutex_initializer(kind: c_int) -> pthread_mutex_t {
    let mut size = [0u8; 40];
    size[8] = kind as u8;
    pthread_mutex_t { __size: size }
}

pub const PTHREAD_ONCE_INIT: pthread_once_t = 0;
pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = pthread_mutex_t { __size: [0; 40] };
pub const PTHREAD_RECURSIVE_MUTEX_INITIALIZER_NP: pthread_mutex_t =
    mutex_initializer(PTHREAD_MUTEX_RECURSIVE);
pub const PTHREAD_ERRORCHECK_MUTEX_INITIALIZER_NP: pthread_mutex_t =
    mutex_initializer(PTHREAD_MUTEX_ERRORCHECK);
pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t { __size: [0; 48] };
pub const PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t = pthread_rwlock_t { __size: [0; 56] };

// sem_t — <semaphore.h>, 32 bytes
#[repr(C, align(8))]
pub struct sem_t {
    __size: [u8; 32],
}

// <threads.h>
pub type thrd_t = c_ulong;
pub type thrd_start_t = Option<unsafe extern "C" fn(*mut c_void) -> c_int>;
pub type tss_t = c_uint;
pub type tss_dtor_t = Option<unsafe extern "C" fn(*mut c_void)>;

#[repr(C, align(8))]
pub struct mtx_t {
    __size: [u8; 40],
}

#[repr(C, align(8))]
pub struct cnd_t {
    __size: [u8; 48],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct once_flag {
    __flag: c_int,
}

// sched_param — <sched.h>
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sched_param {
    pub sched_priority: c_int,
}

// aiocb — <aio.h>, 168 bytes; the private members hold the library's
// request state while the request is in flight.
#[repr(C)]
pub struct aiocb {
    pub aio_fildes: c_int,
    pub aio_lio_opcode: c_int,
    pub aio_reqprio: c_int,
    __reserved0: c_int,
    pub aio_buf: *mut c_void,
    pub aio_nbytes: size_t,
    pub aio_sigevent: sigevent,
    __request: *mut c_void,
    __state: c_int,
    __reserved1: c_int,
    __error_code: c_int,
    __reserved2: c_int,
    __return_value: ssize_t,
    pub aio_offset: off_t,
    __reserved3: [c_char; 32],
}

// _umtx_time — <sys/umtx.h>
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _umtx_time {
    pub _timeout: timespec,
    pub _flags: c_uint,
    pub _clockid: c_uint,
}

// rlimit
#[repr(C)]
#[derive(Clone, Copy)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}

// dl_phdr_info for dl_iterate_phdr
#[repr(C)]
pub struct dl_phdr_info {
    pub dlpi_addr: u64,
    pub dlpi_name: *const c_char,
    pub dlpi_phdr: *const Elf64_Phdr,
    pub dlpi_phnum: u16,
    pub dlpi_adds: u64,
    pub dlpi_subs: u64,
    pub dlpi_tls_modid: size_t,
    pub dlpi_tls_data: *mut c_void,
}

// iovec — <sys/uio.h>
#[repr(C)]
#[derive(Clone, Copy)]
pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: size_t,
}

// msghdr
#[repr(C)]
pub struct msghdr {
    pub msg_name: *mut c_void,
    pub msg_namelen: socklen_t,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: c_int,
    pub msg_control: *mut c_void,
    pub msg_controllen: socklen_t,
    pub msg_flags: c_int,
}

// cmsghdr
#[repr(C)]
pub struct cmsghdr {
    pub cmsg_len: socklen_t,
    pub cmsg_level: c_int,
    pub cmsg_type: c_int,
}

// fd_set — <sys/select.h>: fds_bits([u64; 16])
#[repr(C)]
#[derive(Clone, Copy)]
pub struct fd_set {
    pub fds_bits: [c_ulong; FD_SETSIZE / 64],
}

// termios — <termios.h>
#[repr(C)]
#[derive(Clone, Copy)]
pub struct termios {
    pub c_iflag: u32,
    pub c_oflag: u32,
    pub c_cflag: u32,
    pub c_lflag: u32,
    pub c_line: u8,
    pub c_cc: [u8; NCCS],
    pub c_ispeed: u32,
    pub c_ospeed: u32,
}

// winsize
#[repr(C)]
#[derive(Clone, Copy)]
pub struct winsize {
    pub ws_row: c_ushort,
    pub ws_col: c_ushort,
    pub ws_xpixel: c_ushort,
    pub ws_ypixel: c_ushort,
}

// linger
#[repr(C)]
#[derive(Clone, Copy)]
pub struct linger {
    pub l_onoff: c_int,
    pub l_linger: c_int,
}

// ip_mreq
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ip_mreq {
    pub imr_multiaddr: in_addr,
    pub imr_interface: in_addr,
}

// ipv6_mreq
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ipv6_mreq {
    pub ipv6mr_multiaddr: in6_addr,
    pub ipv6mr_interface: c_uint,
}

// rusage — <sys/resource.h>, 144 bytes
#[repr(C)]
#[derive(Clone, Copy)]
pub struct rusage {
    pub ru_utime: timeval,
    pub ru_stime: timeval,
    pub ru_maxrss: c_long,
    pub ru_ixrss: c_long,
    pub ru_idrss: c_long,
    pub ru_isrss: c_long,
    pub ru_minflt: c_long,
    pub ru_majflt: c_long,
    pub ru_nswap: c_long,
    pub ru_inblock: c_long,
    pub ru_oublock: c_long,
    pub ru_msgsnd: c_long,
    pub ru_msgrcv: c_long,
    pub ru_nsignals: c_long,
    pub ru_nvcsw: c_long,
    pub ru_nivcsw: c_long,
}

// flock
#[repr(C)]
#[derive(Clone, Copy)]
pub struct flock {
    pub l_type: c_short,
    pub l_whence: c_short,
    pub l_start: off_t,
    pub l_len: off_t,
    pub l_pid: pid_t,
}

// passwd
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut c_char,
    pub pw_passwd: *mut c_char,
    pub pw_uid: uid_t,
    pub pw_gid: gid_t,
    pub pw_change: time_t,
    pub pw_class: *mut c_char,
    pub pw_gecos: *mut c_char,
    pub pw_dir: *mut c_char,
    pub pw_shell: *mut c_char,
    pub pw_expire: time_t,
    pub pw_fields: c_int,
}

// group
#[repr(C)]
pub struct group {
    pub gr_name: *mut c_char,
    pub gr_passwd: *mut c_char,
    pub gr_gid: gid_t,
    pub gr_mem: *mut *mut c_char,
}

// shadow password (spwd)
#[repr(C)]
pub struct spwd {
    pub sp_namp: *mut c_char,
    pub sp_pwdp: *mut c_char,
    pub sp_lstchg: c_long,
    pub sp_min: c_long,
    pub sp_max: c_long,
    pub sp_warn: c_long,
    pub sp_inact: c_long,
    pub sp_expire: c_long,
    pub sp_flag: c_ulong,
}

// glob_t — <glob.h>: glob() writes gl_flags, so a Rust-allocated glob_t must
// carry every member.
#[repr(C)]
pub struct glob_t {
    pub gl_pathc: size_t,
    pub gl_pathv: *mut *mut c_char,
    pub gl_offs: size_t,
    pub gl_flags: c_int,
    pub gl_closedir: Option<unsafe extern "C" fn(*mut c_void)>,
    pub gl_readdir: Option<unsafe extern "C" fn(*mut c_void) -> *mut dirent>,
    pub gl_opendir: Option<unsafe extern "C" fn(*const c_char) -> *mut c_void>,
    pub gl_lstat: Option<unsafe extern "C" fn(*const c_char, *mut stat) -> c_int>,
    pub gl_stat: Option<unsafe extern "C" fn(*const c_char, *mut stat) -> c_int>,
}

// utsname
#[repr(C)]
pub struct utsname {
    pub sysname: [c_char; 65],
    pub nodename: [c_char; 65],
    pub release: [c_char; 65],
    pub version: [c_char; 65],
    pub machine: [c_char; 65],
}

// tm (broken-down time) — <time.h>
#[repr(C)]
#[derive(Clone, Copy)]
pub struct tm {
    pub tm_sec: c_int,
    pub tm_min: c_int,
    pub tm_hour: c_int,
    pub tm_mday: c_int,
    pub tm_mon: c_int,
    pub tm_year: c_int,
    pub tm_wday: c_int,
    pub tm_yday: c_int,
    pub tm_isdst: c_int,
    pub tm_gmtoff: c_long,
    pub tm_zone: *const c_char,
}

// Elf64 types of <link.h>
pub type Elf64_Half = u16;
pub type Elf64_Word = u32;
pub type Elf64_Sword = i32;
pub type Elf64_Xword = u64;
pub type Elf64_Sxword = i64;
pub type Elf64_Addr = u64;
pub type Elf64_Off = u64;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Elf64_Phdr {
    pub p_type: Elf64_Word,
    pub p_flags: Elf64_Word,
    pub p_offset: Elf64_Off,
    pub p_vaddr: Elf64_Addr,
    pub p_paddr: Elf64_Addr,
    pub p_filesz: Elf64_Xword,
    pub p_memsz: Elf64_Xword,
    pub p_align: Elf64_Xword,
}

pub type dlfunc_t = Option<unsafe extern "C" fn()>;

// Dl_info — <dlfcn.h>
#[repr(C)]
pub struct Dl_info {
    pub dli_fname: *const c_char,
    pub dli_fbase: *mut c_void,
    pub dli_sname: *const c_char,
    pub dli_saddr: *mut c_void,
}

// The <spawn.h> objects: opaque, 64 bytes each on both targets.
#[repr(C, align(8))]
pub struct posix_spawnattr_t {
    __size: [u8; 64],
}

#[repr(C, align(8))]
pub struct posix_spawn_file_actions_t {
    __size: [u8; 64],
}

// statvfs — <sys/statvfs.h>
#[repr(C)]
#[derive(Clone, Copy)]
pub struct statvfs {
    pub f_bsize: c_ulong,
    pub f_frsize: c_ulong,
    pub f_blocks: fsblkcnt_t,
    pub f_bfree: fsblkcnt_t,
    pub f_bavail: fsblkcnt_t,
    pub f_files: fsfilcnt_t,
    pub f_ffree: fsfilcnt_t,
    pub f_favail: fsfilcnt_t,
    pub f_fsid: c_ulong,
    pub f_flag: c_ulong,
    pub f_namemax: c_ulong,
}

// statfs — <sys/mount.h>
#[repr(C)]
#[derive(Clone, Copy)]
pub struct statfs {
    pub f_version: c_uint,
    pub f_type: c_uint,
    pub f_flags: c_ulong,
    pub f_bsize: c_ulong,
    pub f_iosize: c_ulong,
    pub f_blocks: c_ulong,
    pub f_bfree: c_ulong,
    pub f_bavail: c_long,
    pub f_files: c_ulong,
    pub f_ffree: c_long,
    pub f_syncwrites: c_ulong,
    pub f_asyncwrites: c_ulong,
    pub f_syncreads: c_ulong,
    pub f_asyncreads: c_ulong,
    pub f_namemax: c_ulong,
    pub f_owner: uid_t,
    pub f_fsid: c_int,
    pub f_fstypename: [c_char; MFSNAMELEN],
    pub f_mntfromname: [c_char; MNAMELEN],
    pub f_mntonname: [c_char; MNAMELEN],
}

// FTW — <ftw.h>
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FTW {
    pub base: c_int,
    pub level: c_int,
}

// System V IPC — <sys/ipc.h>, <sys/shm.h>, <sys/sem.h>, <sys/msg.h>: the
// Linux LP64 layouts.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ipc_perm {
    pub __key: key_t,
    pub uid: uid_t,
    pub gid: gid_t,
    pub cuid: uid_t,
    pub cgid: gid_t,
    pub mode: mode_t,
    __pad1: c_ushort,
    pub __seq: c_ushort,
    __pad2: c_ushort,
    __unused1: c_ulong,
    __unused2: c_ulong,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct shmid_ds {
    pub shm_perm: ipc_perm,
    pub shm_segsz: size_t,
    pub shm_atime: time_t,
    pub shm_dtime: time_t,
    pub shm_ctime: time_t,
    pub shm_cpid: pid_t,
    pub shm_lpid: pid_t,
    pub shm_nattch: shmatt_t,
    __unused5: c_ulong,
    __unused6: c_ulong,
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct semid_ds {
    pub sem_perm: ipc_perm,
    pub sem_otime: time_t,
    __unused1: c_ulong,
    pub sem_ctime: time_t,
    __unused2: c_ulong,
    pub sem_nsems: c_ulong,
    __unused3: c_ulong,
    __unused4: c_ulong,
}

#[cfg(target_arch = "aarch64")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct semid_ds {
    pub sem_perm: ipc_perm,
    pub sem_otime: time_t,
    pub sem_ctime: time_t,
    pub sem_nsems: c_ulong,
    __unused3: c_ulong,
    __unused4: c_ulong,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct sembuf {
    pub sem_num: c_ushort,
    pub sem_op: c_short,
    pub sem_flg: c_short,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct msqid_ds {
    pub msg_perm: ipc_perm,
    pub msg_stime: time_t,
    pub msg_rtime: time_t,
    pub msg_ctime: time_t,
    pub __msg_cbytes: c_ulong,
    pub msg_qnum: msgqnum_t,
    pub msg_qbytes: msglen_t,
    pub msg_lspid: pid_t,
    pub msg_lrpid: pid_t,
    __unused4: c_ulong,
    __unused5: c_ulong,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct msgbuf {
    pub mtype: c_long,
    pub mtext: [c_char; 1],
}

// fsxattr — <linux/fs.h>, 28 bytes
#[repr(C)]
#[derive(Clone, Copy)]
pub struct fsxattr {
    pub fsx_xflags: u32,
    pub fsx_extsize: u32,
    pub fsx_nextents: u32,
    pub fsx_projid: u32,
    pub fsx_cowextsize: u32,
    pub fsx_pad: [c_uchar; 8],
}

// <sys/quota.h>
#[repr(C)]
#[derive(Clone, Copy)]
pub struct dqblk {
    pub dqb_bhardlimit: u64,
    pub dqb_bsoftlimit: u64,
    pub dqb_curspace: u64,
    pub dqb_ihardlimit: u64,
    pub dqb_isoftlimit: u64,
    pub dqb_curinodes: u64,
    pub dqb_btime: u64,
    pub dqb_itime: u64,
    pub dqb_valid: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct if_nextdqblk {
    pub dqb_bhardlimit: u64,
    pub dqb_bsoftlimit: u64,
    pub dqb_curspace: u64,
    pub dqb_ihardlimit: u64,
    pub dqb_isoftlimit: u64,
    pub dqb_curinodes: u64,
    pub dqb_btime: u64,
    pub dqb_itime: u64,
    pub dqb_valid: u32,
    pub dqb_id: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct dqinfo {
    pub dqi_bgrace: u64,
    pub dqi_igrace: u64,
    pub dqi_flags: u32,
    pub dqi_valid: u32,
}

// <linux/fb.h>
#[repr(C)]
#[derive(Clone, Copy)]
pub struct fb_bitfield {
    pub offset: u32,
    pub length: u32,
    pub msb_right: u32,
}

// 80 bytes
#[repr(C)]
#[derive(Clone, Copy)]
pub struct fb_fix_screeninfo {
    pub id: [c_char; 16],
    pub smem_start: c_ulong,
    pub smem_len: u32,
    pub type_: u32,
    pub type_aux: u32,
    pub visual: u32,
    pub xpanstep: u16,
    pub ypanstep: u16,
    pub ywrapstep: u16,
    pub line_length: u32,
    pub mmio_start: c_ulong,
    pub mmio_len: u32,
    pub accel: u32,
    pub capabilities: u16,
    pub reserved: [u16; 2],
}

// 160 bytes
#[repr(C)]
#[derive(Clone, Copy)]
pub struct fb_var_screeninfo {
    pub xres: u32,
    pub yres: u32,
    pub xres_virtual: u32,
    pub yres_virtual: u32,
    pub xoffset: u32,
    pub yoffset: u32,
    pub bits_per_pixel: u32,
    pub grayscale: u32,
    pub red: fb_bitfield,
    pub green: fb_bitfield,
    pub blue: fb_bitfield,
    pub transp: fb_bitfield,
    pub nonstd: u32,
    pub activate: u32,
    pub height: u32,
    pub width: u32,
    pub accel_flags: u32,
    pub pixclock: u32,
    pub left_margin: u32,
    pub right_margin: u32,
    pub upper_margin: u32,
    pub lower_margin: u32,
    pub hsync_len: u32,
    pub vsync_len: u32,
    pub sync: u32,
    pub vmode: u32,
    pub rotate: u32,
    pub colorspace: u32,
    pub reserved: [u32; 4],
}

// <sys/device_catalog.h>: one published device node, 144 bytes. name is
// relative to /dev and not NUL-terminated.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct device_record {
    pub device_id: u64,
    pub incarnation: u64,
    pub device_class: u64,
    pub dev: dev_t,
    pub mode: u32,
    pub kind: u8,
    pub flags: u8,
    pub name_len: u8,
    pub reserved0: u8,
    pub owner: u64,
    pub group: u64,
    pub alias_of: u64,
    pub name: [c_char; DEVICE_NAME_MAX],
    pub devtype: u16,
    pub partition: u16,
    pub reserved1: [u8; 4],
    pub object_id: u64,
}

// 160 bytes
#[repr(C)]
#[derive(Clone, Copy)]
pub struct device_change {
    pub kind: u64,
    pub generation: u64,
    pub record: device_record,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct device_event {
    pub action: u64,
    pub object_id: u64,
    pub incarnation: u64,
    pub sequence: u64,
    pub drops: u64,
    pub flags: u64,
    pub properties_length: size_t,
    pub properties: [c_char; DEVICE_EVENT_PROPERTIES_MAX],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct device_object {
    pub object_id: u64,
    pub incarnation: u64,
}

// <utmpx.h>: 400 bytes
#[repr(C)]
#[derive(Clone, Copy)]
pub struct __exit_status {
    pub e_termination: c_short,
    pub e_exit: c_short,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct utmpx {
    pub ut_type: c_short,
    __ut_reserved0: c_short,
    pub ut_pid: pid_t,
    pub ut_line: [c_char; 32],
    pub ut_id: [c_char; 4],
    pub ut_user: [c_char; 32],
    pub ut_host: [c_char; 256],
    pub ut_exit: __exit_status,
    pub ut_session: i32,
    __ut_reserved1: i32,
    pub ut_tv: timeval,
    pub ut_addr_v6: [u32; 4],
    __ut_reserved2: [c_char; 20],
}

// The capability reference of <kernite/uapi/invoke.h>, which <exec_operands.h>
// embeds. Keep in step with that header.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct kernite_cnode_ref {
    pub cnode_addr: u64,
    pub cnode_depth: u8,
    pub reserved0: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct kernite_cap_ref {
    pub cnode: kernite_cnode_ref,
    pub index: u64,
    pub depth: u8,
    pub reserved0: [u8; 3],
    pub rights: u32,
}

// exec_operands — <exec_operands.h>, 224 bytes
#[repr(C)]
#[derive(Clone, Copy)]
pub struct exec_operands {
    pub words: [u64; EXEC_OPERANDS_WORDS],
    pub caps: [kernite_cap_ref; EXEC_OPERANDS_CAPS],
    pub cap_count: u32,
    pub reserved: u32,
    pub handle: u64,
}

// auth_broker_reply — <auth_broker.h>
#[repr(C)]
#[derive(Clone, Copy)]
pub struct auth_broker_reply {
    pub kind: c_int,
    pub style: c_int,
    pub outcome: c_int,
    pub shape: c_int,
    pub uid: uid_t,
    pub gid: gid_t,
    pub text_len: size_t,
    pub text: [c_char; AUTH_BROKER_TEXT_MAX + 1],
}

// =========================================================================
// Extern "C" function declarations
// =========================================================================

// Programs using these declarations link the C library itself; the
// target's default link arguments name only libsystem.
#[link(name = "c")]
unsafe extern "C" {
    // --- errno ---
    pub fn __errno_location() -> *mut c_int;

    // --- Memory ---
    pub fn malloc(size: size_t) -> *mut c_void;
    pub fn calloc(nobj: size_t, size: size_t) -> *mut c_void;
    pub fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    pub fn free(ptr: *mut c_void);
    pub fn posix_memalign(memptr: *mut *mut c_void, align: size_t, size: size_t) -> c_int;
    pub fn aligned_alloc(align: size_t, size: size_t) -> *mut c_void;

    // --- String ---
    pub fn strlen(s: *const c_char) -> size_t;
    pub fn strnlen(s: *const c_char, maxlen: size_t) -> size_t;
    pub fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    pub fn strncpy(dst: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    pub fn strcat(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    pub fn strncat(dst: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    pub fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    pub fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    pub fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    pub fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    pub fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    pub fn strtok_r(
        s: *mut c_char,
        delim: *const c_char,
        saveptr: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn strtol(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_long;
    pub fn strtoul(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_ulong;
    pub fn strtoll(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_longlong;
    pub fn strtoull(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_ulonglong;
    pub fn strtof(s: *const c_char, endp: *mut *mut c_char) -> c_float;
    pub fn strtod(s: *const c_char, endp: *mut *mut c_char) -> c_double;
    pub fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    pub fn memmove(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    pub fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    pub fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    pub fn memchr(s: *const c_void, c: c_int, n: size_t) -> *mut c_void;
    pub fn strdup(s: *const c_char) -> *mut c_char;
    pub fn strndup(s: *const c_char, n: size_t) -> *mut c_char;
    pub fn strerror(errnum: c_int) -> *mut c_char;
    pub fn strerror_r(errnum: c_int, buf: *mut c_char, buflen: size_t) -> c_int;
    pub fn strsignal(sig: c_int) -> *mut c_char;
    pub fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
    pub fn strncasecmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    pub fn stpcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    pub fn stpncpy(dst: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    pub fn strspn(s: *const c_char, accept: *const c_char) -> size_t;
    pub fn strcspn(s: *const c_char, reject: *const c_char) -> size_t;
    pub fn strpbrk(s: *const c_char, accept: *const c_char) -> *mut c_char;
    pub fn strtoimax(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> intmax_t;
    pub fn strtoumax(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> uintmax_t;
    pub fn memrchr(s: *const c_void, c: c_int, n: size_t) -> *mut c_void;
    pub fn explicit_bzero(s: *mut c_void, n: size_t);
    pub fn bcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;

    // --- I/O ---
    pub fn open(path: *const c_char, oflag: c_int, ...) -> c_int;
    pub fn openat(dirfd: c_int, path: *const c_char, oflag: c_int, ...) -> c_int;
    pub fn close(fd: c_int) -> c_int;
    pub fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    pub fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    pub fn pread(fd: c_int, buf: *mut c_void, count: size_t, offset: off_t) -> ssize_t;
    pub fn pwrite(fd: c_int, buf: *const c_void, count: size_t, offset: off_t) -> ssize_t;
    pub fn readv(fd: c_int, iov: *const iovec, iovcnt: c_int) -> ssize_t;
    pub fn writev(fd: c_int, iov: *const iovec, iovcnt: c_int) -> ssize_t;
    pub fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t;
    pub fn dup(oldfd: c_int) -> c_int;
    pub fn dup2(oldfd: c_int, newfd: c_int) -> c_int;
    pub fn dup3(oldfd: c_int, newfd: c_int, flags: c_int) -> c_int;
    pub fn pipe(fds: *mut c_int) -> c_int;
    pub fn pipe2(fds: *mut c_int, flags: c_int) -> c_int;
    pub fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    pub fn stat(path: *const c_char, buf: *mut stat) -> c_int;
    pub fn fstat(fd: c_int, buf: *mut stat) -> c_int;
    pub fn lstat(path: *const c_char, buf: *mut stat) -> c_int;
    pub fn fstatat(dirfd: c_int, path: *const c_char, buf: *mut stat, flag: c_int) -> c_int;
    pub fn unlink(path: *const c_char) -> c_int;
    pub fn unlinkat(dirfd: c_int, path: *const c_char, flag: c_int) -> c_int;
    pub fn rmdir(path: *const c_char) -> c_int;
    pub fn mkdir(path: *const c_char, mode: mode_t) -> c_int;
    pub fn mkdirat(dirfd: c_int, path: *const c_char, mode: mode_t) -> c_int;
    pub fn rename(old: *const c_char, new: *const c_char) -> c_int;
    pub fn renameat(
        olddirfd: c_int,
        old: *const c_char,
        newdirfd: c_int,
        new: *const c_char,
    ) -> c_int;
    pub fn access(path: *const c_char, amode: c_int) -> c_int;
    pub fn faccessat(dirfd: c_int, path: *const c_char, amode: c_int, flag: c_int) -> c_int;
    pub fn chdir(path: *const c_char) -> c_int;
    pub fn fchdir(fd: c_int) -> c_int;
    pub fn getcwd(buf: *mut c_char, size: size_t) -> *mut c_char;
    pub fn symlink(target: *const c_char, linkpath: *const c_char) -> c_int;
    pub fn symlinkat(target: *const c_char, newdirfd: c_int, linkpath: *const c_char) -> c_int;
    pub fn readlink(path: *const c_char, buf: *mut c_char, bufsiz: size_t) -> ssize_t;
    pub fn readlinkat(
        dirfd: c_int,
        path: *const c_char,
        buf: *mut c_char,
        bufsiz: size_t,
    ) -> ssize_t;
    pub fn link(old: *const c_char, new: *const c_char) -> c_int;
    pub fn linkat(
        olddirfd: c_int,
        old: *const c_char,
        newdirfd: c_int,
        new: *const c_char,
        flags: c_int,
    ) -> c_int;
    pub fn opendir(name: *const c_char) -> *mut DIR;
    pub fn readdir(dirp: *mut DIR) -> *mut dirent;
    pub fn closedir(dirp: *mut DIR) -> c_int;
    pub fn dirfd(dirp: *mut DIR) -> c_int;
    pub fn rewinddir(dirp: *mut DIR);
    pub fn isatty(fd: c_int) -> c_int;
    pub fn ttyname(fd: c_int) -> *mut c_char;
    pub fn ttyname_r(fd: c_int, buf: *mut c_char, buflen: size_t) -> c_int;
    pub fn ftruncate(fd: c_int, length: off_t) -> c_int;
    pub fn truncate(path: *const c_char, length: off_t) -> c_int;
    pub fn chmod(path: *const c_char, mode: mode_t) -> c_int;
    pub fn fchmod(fd: c_int, mode: mode_t) -> c_int;
    pub fn fchmodat(dirfd: c_int, path: *const c_char, mode: mode_t, flag: c_int) -> c_int;
    pub fn chown(path: *const c_char, owner: uid_t, group: gid_t) -> c_int;
    pub fn fchown(fd: c_int, owner: uid_t, group: gid_t) -> c_int;
    pub fn lchown(path: *const c_char, owner: uid_t, group: gid_t) -> c_int;
    pub fn fchownat(
        dirfd: c_int,
        path: *const c_char,
        owner: uid_t,
        group: gid_t,
        flag: c_int,
    ) -> c_int;
    pub fn umask(mask: mode_t) -> mode_t;
    pub fn mkfifo(path: *const c_char, mode: mode_t) -> c_int;
    pub fn realpath(path: *const c_char, resolved_path: *mut c_char) -> *mut c_char;
    pub fn utimensat(
        dirfd: c_int,
        path: *const c_char,
        times: *const timespec,
        flag: c_int,
    ) -> c_int;
    pub fn futimens(fd: c_int, times: *const timespec) -> c_int;
    pub fn pathconf(path: *const c_char, name: c_int) -> c_long;
    pub fn fpathconf(fd: c_int, name: c_int) -> c_long;
    pub fn confstr(name: c_int, buf: *mut c_char, len: size_t) -> size_t;
    pub fn flock(fd: c_int, operation: c_int) -> c_int;
    pub fn mkdtemp(template: *mut c_char) -> *mut c_char;

    // --- Mmap ---
    pub fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: off_t,
    ) -> *mut c_void;
    pub fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    pub fn mprotect(addr: *mut c_void, length: size_t, prot: c_int) -> c_int;
    pub fn madvise(addr: *mut c_void, length: size_t, advice: c_int) -> c_int;

    // --- Process ---
    pub fn fork() -> pid_t;
    pub fn execve(
        path: *const c_char,
        argv: *const *const c_char,
        envp: *const *const c_char,
    ) -> c_int;
    pub fn execvp(file: *const c_char, argv: *const *const c_char) -> c_int;
    pub fn _exit(status: c_int) -> !;
    pub fn getpid() -> pid_t;
    pub fn getppid() -> pid_t;
    pub fn kill(pid: pid_t, sig: c_int) -> c_int;
    pub fn killpg(pgrp: pid_t, sig: c_int) -> c_int;
    pub fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    pub fn abort() -> !;
    pub fn exit(status: c_int) -> !;
    pub fn _Exit(status: c_int) -> !;
    pub fn setsid() -> pid_t;
    pub fn setpgid(pid: pid_t, pgid: pid_t) -> c_int;
    pub fn getpgid(pid: pid_t) -> pid_t;
    pub fn getpgrp() -> pid_t;
    pub fn getsid(pid: pid_t) -> pid_t;
    pub fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;

    // --- Env ---
    pub fn getenv(name: *const c_char) -> *mut c_char;
    pub fn setenv(name: *const c_char, value: *const c_char, overwrite: c_int) -> c_int;
    pub fn unsetenv(name: *const c_char) -> c_int;
    pub fn putenv(string: *mut c_char) -> c_int;

    // --- Socket ---
    pub fn socket(domain: c_int, ty: c_int, protocol: c_int) -> c_int;
    pub fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    pub fn listen(sockfd: c_int, backlog: c_int) -> c_int;
    pub fn accept(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    pub fn connect(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    pub fn shutdown(sockfd: c_int, how: c_int) -> c_int;
    pub fn socketpair(domain: c_int, ty: c_int, protocol: c_int, sv: *mut c_int) -> c_int;
    pub fn send(sockfd: c_int, buf: *const c_void, len: size_t, flags: c_int) -> ssize_t;
    pub fn recv(sockfd: c_int, buf: *mut c_void, len: size_t, flags: c_int) -> ssize_t;
    pub fn sendto(
        sockfd: c_int,
        buf: *const c_void,
        len: size_t,
        flags: c_int,
        dest_addr: *const sockaddr,
        addrlen: socklen_t,
    ) -> ssize_t;
    pub fn recvfrom(
        sockfd: c_int,
        buf: *mut c_void,
        len: size_t,
        flags: c_int,
        src_addr: *mut sockaddr,
        addrlen: *mut socklen_t,
    ) -> ssize_t;
    pub fn sendmsg(sockfd: c_int, msg: *const msghdr, flags: c_int) -> ssize_t;
    pub fn recvmsg(sockfd: c_int, msg: *mut msghdr, flags: c_int) -> ssize_t;
    pub fn setsockopt(
        sockfd: c_int,
        level: c_int,
        optname: c_int,
        optval: *const c_void,
        optlen: socklen_t,
    ) -> c_int;
    pub fn getsockopt(
        sockfd: c_int,
        level: c_int,
        optname: c_int,
        optval: *mut c_void,
        optlen: *mut socklen_t,
    ) -> c_int;
    pub fn getsockname(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    pub fn getpeername(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    pub fn getifaddrs(ifap: *mut *mut ifaddrs) -> c_int;
    pub fn freeifaddrs(ifa: *mut ifaddrs);
    pub fn if_nameindex() -> *mut if_nameindex;
    pub fn if_freenameindex(ptr: *mut if_nameindex);
    pub fn if_nametoindex(ifname: *const c_char) -> c_uint;
    pub fn if_indextoname(ifindex: c_uint, ifname: *mut c_char) -> *mut c_char;
    pub fn getaddrinfo(
        node: *const c_char,
        service: *const c_char,
        hints: *const addrinfo,
        res: *mut *mut addrinfo,
    ) -> c_int;
    pub fn freeaddrinfo(res: *mut addrinfo);
    pub fn gai_strerror(ecode: c_int) -> *const c_char;
    pub fn getnameinfo(
        sa: *const sockaddr,
        salen: socklen_t,
        host: *mut c_char,
        hostlen: socklen_t,
        serv: *mut c_char,
        servlen: socklen_t,
        flags: c_int,
    ) -> c_int;
    pub fn poll(fds: *mut pollfd, nfds: nfds_t, timeout: c_int) -> c_int;
    pub fn select(
        nfds: c_int,
        readfds: *mut fd_set,
        writefds: *mut fd_set,
        exceptfds: *mut fd_set,
        timeout: *mut timeval,
    ) -> c_int;
    pub fn epoll_create1(flags: c_int) -> c_int;
    pub fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut epoll_event) -> c_int;
    pub fn epoll_wait(
        epfd: c_int,
        events: *mut epoll_event,
        maxevents: c_int,
        timeout: c_int,
    ) -> c_int;
    pub fn inet_ntop(
        af: c_int,
        src: *const c_void,
        dst: *mut c_char,
        size: socklen_t,
    ) -> *const c_char;
    pub fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
    pub fn htons(hostshort: u16) -> u16;
    pub fn ntohs(netshort: u16) -> u16;
    pub fn htonl(hostlong: u32) -> u32;
    pub fn ntohl(netlong: u32) -> u32;
    pub fn inet_addr(cp: *const c_char) -> in_addr_t;

    // --- Pthread ---
    pub fn pthread_create(
        thread: *mut pthread_t,
        attr: *const pthread_attr_t,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    pub fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    pub fn pthread_exit(retval: *mut c_void) -> !;
    pub fn pthread_self() -> pthread_t;
    pub fn pthread_detach(thread: pthread_t) -> c_int;
    pub fn pthread_cancel(thread: pthread_t) -> c_int;
    pub fn pthread_equal(t1: pthread_t, t2: pthread_t) -> c_int;

    pub fn pthread_mutex_init(
        mutex: *mut pthread_mutex_t,
        attr: *const pthread_mutexattr_t,
    ) -> c_int;
    pub fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> c_int;
    pub fn pthread_mutex_trylock(mutex: *mut pthread_mutex_t) -> c_int;
    pub fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> c_int;
    pub fn pthread_mutex_destroy(mutex: *mut pthread_mutex_t) -> c_int;
    pub fn pthread_mutex_timedlock(
        mutex: *mut pthread_mutex_t,
        abstime: *const timespec,
    ) -> c_int;

    pub fn pthread_mutexattr_init(attr: *mut pthread_mutexattr_t) -> c_int;
    pub fn pthread_mutexattr_destroy(attr: *mut pthread_mutexattr_t) -> c_int;
    pub fn pthread_mutexattr_settype(attr: *mut pthread_mutexattr_t, kind: c_int) -> c_int;
    pub fn pthread_mutexattr_gettype(attr: *const pthread_mutexattr_t, kind: *mut c_int) -> c_int;

    pub fn pthread_cond_init(cond: *mut pthread_cond_t, attr: *const pthread_condattr_t) -> c_int;
    pub fn pthread_cond_signal(cond: *mut pthread_cond_t) -> c_int;
    pub fn pthread_cond_broadcast(cond: *mut pthread_cond_t) -> c_int;
    pub fn pthread_cond_wait(cond: *mut pthread_cond_t, mutex: *mut pthread_mutex_t) -> c_int;
    pub fn pthread_cond_timedwait(
        cond: *mut pthread_cond_t,
        mutex: *mut pthread_mutex_t,
        abstime: *const timespec,
    ) -> c_int;
    pub fn pthread_cond_destroy(cond: *mut pthread_cond_t) -> c_int;

    pub fn pthread_condattr_init(attr: *mut pthread_condattr_t) -> c_int;
    pub fn pthread_condattr_destroy(attr: *mut pthread_condattr_t) -> c_int;

    pub fn pthread_rwlock_init(
        rwlock: *mut pthread_rwlock_t,
        attr: *const pthread_rwlockattr_t,
    ) -> c_int;
    pub fn pthread_rwlock_rdlock(rwlock: *mut pthread_rwlock_t) -> c_int;
    pub fn pthread_rwlock_wrlock(rwlock: *mut pthread_rwlock_t) -> c_int;
    pub fn pthread_rwlock_tryrdlock(rwlock: *mut pthread_rwlock_t) -> c_int;
    pub fn pthread_rwlock_trywrlock(rwlock: *mut pthread_rwlock_t) -> c_int;
    pub fn pthread_rwlock_unlock(rwlock: *mut pthread_rwlock_t) -> c_int;
    pub fn pthread_rwlock_destroy(rwlock: *mut pthread_rwlock_t) -> c_int;

    pub fn pthread_key_create(key: *mut pthread_key_t, dtor: Option<unsafe extern "C" fn(*mut c_void)>) -> c_int;
    pub fn pthread_key_delete(key: pthread_key_t) -> c_int;
    pub fn pthread_setspecific(key: pthread_key_t, value: *const c_void) -> c_int;
    pub fn pthread_getspecific(key: pthread_key_t) -> *mut c_void;

    pub fn pthread_once(
        once_control: *mut pthread_once_t,
        init_routine: unsafe extern "C" fn(),
    ) -> c_int;

    pub fn pthread_attr_init(attr: *mut pthread_attr_t) -> c_int;
    pub fn pthread_attr_destroy(attr: *mut pthread_attr_t) -> c_int;
    pub fn pthread_attr_setstacksize(attr: *mut pthread_attr_t, stacksize: size_t) -> c_int;
    pub fn pthread_attr_getstacksize(attr: *const pthread_attr_t, stacksize: *mut size_t) -> c_int;
    pub fn pthread_attr_setdetachstate(attr: *mut pthread_attr_t, detachstate: c_int) -> c_int;
    pub fn pthread_attr_getdetachstate(
        attr: *const pthread_attr_t,
        detachstate: *mut c_int,
    ) -> c_int;

    pub fn pthread_sigmask(
        how: c_int,
        set: *const sigset_t,
        oldset: *mut sigset_t,
    ) -> c_int;

    pub fn pthread_setcancelstate(state: c_int, oldstate: *mut c_int) -> c_int;
    pub fn pthread_setcanceltype(ctype: c_int, oldtype: *mut c_int) -> c_int;
    pub fn pthread_testcancel();

    pub fn pthread_barrier_init(
        barrier: *mut pthread_barrier_t,
        attr: *const pthread_barrierattr_t,
        count: c_uint,
    ) -> c_int;
    pub fn pthread_barrier_wait(barrier: *mut pthread_barrier_t) -> c_int;
    pub fn pthread_barrier_destroy(barrier: *mut pthread_barrier_t) -> c_int;

    // --- Signal ---
    pub fn signal(signum: c_int, handler: sighandler_t) -> sighandler_t;
    pub fn sigaction(
        signum: c_int,
        act: *const sigaction,
        oldact: *mut sigaction,
    ) -> c_int;
    pub fn sigprocmask(how: c_int, set: *const sigset_t, oldset: *mut sigset_t) -> c_int;
    pub fn sigsuspend(mask: *const sigset_t) -> c_int;
    pub fn sigpending(set: *mut sigset_t) -> c_int;
    pub fn sigemptyset(set: *mut sigset_t) -> c_int;
    pub fn sigfillset(set: *mut sigset_t) -> c_int;
    pub fn sigaddset(set: *mut sigset_t, signum: c_int) -> c_int;
    pub fn sigdelset(set: *mut sigset_t, signum: c_int) -> c_int;
    pub fn sigismember(set: *const sigset_t, signum: c_int) -> c_int;
    pub fn raise(sig: c_int) -> c_int;
    pub fn sigaltstack(ss: *const stack_t, old_ss: *mut stack_t) -> c_int;
    pub fn sigwait(set: *const sigset_t, sig: *mut c_int) -> c_int;
    pub fn sigwaitinfo(set: *const sigset_t, info: *mut siginfo_t) -> c_int;
    pub fn sigtimedwait(set: *const sigset_t, info: *mut siginfo_t, timeout: *const timespec) -> c_int;
    pub fn sigqueue(pid: pid_t, sig: c_int, value: sigval) -> c_int;
    pub fn pthread_kill(thread: pthread_t, sig: c_int) -> c_int;
    pub fn psignal(sig: c_int, s: *const c_char);
    pub fn psiginfo(info: *const siginfo_t, s: *const c_char);
    pub fn sig2str(sig: c_int, str: *mut c_char) -> c_int;
    pub fn str2sig(str: *const c_char, sig: *mut c_int) -> c_int;
    pub fn sigisemptyset(set: *const sigset_t) -> c_int;
    pub fn sigandset(dest: *mut sigset_t, left: *const sigset_t, right: *const sigset_t) -> c_int;
    pub fn sigorset(dest: *mut sigset_t, left: *const sigset_t, right: *const sigset_t) -> c_int;

    // --- Time ---
    pub fn time(t: *mut time_t) -> time_t;
    pub fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;
    pub fn clock_gettime(clock_id: clockid_t, tp: *mut timespec) -> c_int;
    pub fn clock_getres(clock_id: clockid_t, tp: *mut timespec) -> c_int;
    pub fn nanosleep(req: *const timespec, rem: *mut timespec) -> c_int;
    pub fn sleep(seconds: c_uint) -> c_uint;
    pub fn usleep(usec: c_uint) -> c_int;
    pub fn strftime(
        buf: *mut c_char,
        maxsize: size_t,
        format: *const c_char,
        tm: *const tm,
    ) -> size_t;
    pub fn gmtime_r(timer: *const time_t, result: *mut tm) -> *mut tm;
    pub fn gmtime(timer: *const time_t) -> *mut tm;
    pub fn localtime_r(timer: *const time_t, result: *mut tm) -> *mut tm;
    pub fn localtime(timer: *const time_t) -> *mut tm;
    pub fn mktime(tm: *mut tm) -> time_t;
    pub fn asctime_r(tm: *const tm, buf: *mut c_char) -> *mut c_char;
    pub fn asctime(tm: *const tm) -> *mut c_char;
    pub fn ctime_r(timer: *const time_t, buf: *mut c_char) -> *mut c_char;
    pub fn ctime(timer: *const time_t) -> *mut c_char;
    pub fn difftime(time1: time_t, time0: time_t) -> c_double;
    pub fn timegm(tm: *mut tm) -> time_t;
    pub fn strptime(buf: *const c_char, fmt: *const c_char, tm: *mut tm) -> *mut c_char;
    pub fn clock() -> clock_t;

    // --- Misc ---
    pub fn sysconf(name: c_int) -> c_long;
    pub fn getrlimit(resource: c_int, rlim: *mut rlimit) -> c_int;
    pub fn setrlimit(resource: c_int, rlim: *const rlimit) -> c_int;
    pub fn dl_iterate_phdr(
        callback: unsafe extern "C" fn(
            info: *mut dl_phdr_info,
            size: size_t,
            data: *mut c_void,
        ) -> c_int,
        data: *mut c_void,
    ) -> c_int;
    pub fn atoi(s: *const c_char) -> c_int;
    pub fn atol(s: *const c_char) -> c_long;
    pub fn atoll(s: *const c_char) -> c_longlong;
    pub fn atof(s: *const c_char) -> c_double;
    pub fn rand() -> c_int;
    pub fn srand(seed: c_uint);
    pub fn abs(i: c_int) -> c_int;
    pub fn labs(i: c_long) -> c_long;
    pub fn llabs(i: c_longlong) -> c_longlong;
    pub fn qsort(
        base: *mut c_void,
        nmemb: size_t,
        size: size_t,
        compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
    );
    pub fn bsearch(
        key: *const c_void,
        base: *const c_void,
        nmemb: size_t,
        size: size_t,
        compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
    ) -> *mut c_void;
    pub fn getuid() -> uid_t;
    pub fn geteuid() -> uid_t;
    pub fn getgid() -> gid_t;
    pub fn getegid() -> gid_t;
    pub fn getgroups(gidsetsize: c_int, grouplist: *mut gid_t) -> c_int;
    pub fn getgrouplist(
        user: *const c_char,
        group: gid_t,
        groups: *mut gid_t,
        ngroups: *mut c_int,
    ) -> c_int;
    pub fn getpwuid(uid: uid_t) -> *mut passwd;
    pub fn getpwnam(name: *const c_char) -> *mut passwd;
    pub fn getpwuid_r(
        uid: uid_t,
        pwd: *mut passwd,
        buf: *mut c_char,
        buflen: size_t,
        result: *mut *mut passwd,
    ) -> c_int;
    pub fn getpwnam_r(
        name: *const c_char,
        pwd: *mut passwd,
        buf: *mut c_char,
        buflen: size_t,
        result: *mut *mut passwd,
    ) -> c_int;
    pub fn getrandom(buf: *mut c_void, buflen: size_t, flags: c_uint) -> ssize_t;
    pub fn uname(buf: *mut utsname) -> c_int;
    pub fn fsync(fd: c_int) -> c_int;
    pub fn chroot(path: *const c_char) -> c_int;
    pub fn setuid(uid: uid_t) -> c_int;
    pub fn setgid(gid: gid_t) -> c_int;
    pub fn seteuid(uid: uid_t) -> c_int;
    pub fn setegid(gid: gid_t) -> c_int;
    pub fn setreuid(ruid: uid_t, euid: uid_t) -> c_int;
    pub fn setregid(rgid: gid_t, egid: gid_t) -> c_int;
    pub fn setresuid(ruid: uid_t, euid: uid_t, suid: uid_t) -> c_int;
    pub fn setresgid(rgid: gid_t, egid: gid_t, sgid: gid_t) -> c_int;
    pub fn getresuid(ruid: *mut uid_t, euid: *mut uid_t, suid: *mut uid_t) -> c_int;
    pub fn getresgid(rgid: *mut gid_t, egid: *mut gid_t, sgid: *mut gid_t) -> c_int;
    pub fn setgroups(ngroups: c_int, grouplist: *const gid_t) -> c_int;
    pub fn initgroups(user: *const c_char, group: gid_t) -> c_int;
    pub fn getgrnam(name: *const c_char) -> *mut group;
    pub fn getgrgid(gid: gid_t) -> *mut group;
    pub fn getgrnam_r(
        name: *const c_char,
        grp: *mut group,
        buf: *mut c_char,
        buflen: size_t,
        result: *mut *mut group,
    ) -> c_int;
    pub fn getgrgid_r(
        gid: gid_t,
        grp: *mut group,
        buf: *mut c_char,
        buflen: size_t,
        result: *mut *mut group,
    ) -> c_int;
    pub fn getspnam(name: *const c_char) -> *mut spwd;
    pub fn getpriority(which: c_int, who: id_t) -> c_int;
    pub fn setpriority(which: c_int, who: id_t, prio: c_int) -> c_int;
    pub fn glob(
        pattern: *const c_char,
        flags: c_int,
        errfunc: Option<unsafe extern "C" fn(*const c_char, c_int) -> c_int>,
        pglob: *mut glob_t,
    ) -> c_int;
    pub fn globfree(pglob: *mut glob_t);
    pub fn sched_yield() -> c_int;
    pub fn gethostname(name: *mut c_char, len: size_t) -> c_int;

    // --- stdio ---
    pub fn printf(format: *const c_char, ...) -> c_int;
    pub fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    pub fn sprintf(buf: *mut c_char, format: *const c_char, ...) -> c_int;
    pub fn snprintf(buf: *mut c_char, size: size_t, format: *const c_char, ...) -> c_int;
    pub fn perror(s: *const c_char);
    pub fn puts(s: *const c_char) -> c_int;
    pub fn fputs(s: *const c_char, stream: *mut FILE) -> c_int;
    pub fn fgets(buf: *mut c_char, n: c_int, stream: *mut FILE) -> *mut c_char;
    pub fn fputc(c: c_int, stream: *mut FILE) -> c_int;
    pub fn fgetc(stream: *mut FILE) -> c_int;
    pub fn getc(stream: *mut FILE) -> c_int;
    pub fn putc(c: c_int, stream: *mut FILE) -> c_int;
    pub fn putchar(c: c_int) -> c_int;
    pub fn getchar() -> c_int;
    pub fn ungetc(c: c_int, stream: *mut FILE) -> c_int;
    pub fn fflush(stream: *mut FILE) -> c_int;
    pub fn feof(stream: *mut FILE) -> c_int;
    pub fn ferror(stream: *mut FILE) -> c_int;
    pub fn clearerr(stream: *mut FILE);
    pub fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    pub fn fclose(stream: *mut FILE) -> c_int;
    pub fn fread(ptr: *mut c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
    pub fn fwrite(ptr: *const c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
    pub fn fseek(stream: *mut FILE, offset: c_long, whence: c_int) -> c_int;
    pub fn ftell(stream: *mut FILE) -> c_long;
    pub fn rewind(stream: *mut FILE);
    pub fn fileno(stream: *mut FILE) -> c_int;
    pub fn setbuf(stream: *mut FILE, buf: *mut c_char);
    pub fn setvbuf(stream: *mut FILE, buf: *mut c_char, mode: c_int, size: size_t) -> c_int;
    pub fn freopen(path: *const c_char, mode: *const c_char, stream: *mut FILE) -> *mut FILE;
    pub fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    pub fn openlog(ident: *const c_char, option: c_int, facility: c_int);
    pub fn syslog(priority: c_int, format: *const c_char, ...);
    pub fn closelog();
    pub fn setlogmask(mask: c_int) -> c_int;

    // --- termios ---
    pub fn tcgetattr(fd: c_int, termios_p: *mut termios) -> c_int;
    pub fn tcsetattr(fd: c_int, optional_actions: c_int, termios_p: *const termios) -> c_int;
    pub fn cfgetispeed(termios_p: *const termios) -> u32;
    pub fn cfgetospeed(termios_p: *const termios) -> u32;
    pub fn cfsetispeed(termios_p: *mut termios, speed: u32) -> c_int;
    pub fn cfsetospeed(termios_p: *mut termios, speed: u32) -> c_int;
    pub fn cfmakeraw(termios_p: *mut termios);
    pub fn tcgetpgrp(fd: c_int) -> pid_t;
    pub fn tcsetpgrp(fd: c_int, pgrp: pid_t) -> c_int;
    pub fn tcgetsid(fd: c_int) -> pid_t;
    pub fn grantpt(fd: c_int) -> c_int;
    pub fn unlockpt(fd: c_int) -> c_int;
    pub fn ptsname(fd: c_int) -> *mut c_char;
    pub fn ptsname_r(fd: c_int, buf: *mut c_char, buflen: size_t) -> c_int;
    pub fn openpty(
        amaster: *mut c_int,
        aslave: *mut c_int,
        name: *mut c_char,
        termp: *const termios,
        winp: *const winsize,
    ) -> c_int;

    // --- Locale ---
    pub fn setlocale(category: c_int, locale: *const c_char) -> *mut c_char;
    pub fn localeconv() -> *mut lconv;

    // --- ctype ---
    pub fn isalpha(c: c_int) -> c_int;
    pub fn isdigit(c: c_int) -> c_int;
    pub fn isalnum(c: c_int) -> c_int;
    pub fn isspace(c: c_int) -> c_int;
    pub fn isupper(c: c_int) -> c_int;
    pub fn islower(c: c_int) -> c_int;
    pub fn isprint(c: c_int) -> c_int;
    pub fn iscntrl(c: c_int) -> c_int;
    pub fn ispunct(c: c_int) -> c_int;
    pub fn isxdigit(c: c_int) -> c_int;
    pub fn isgraph(c: c_int) -> c_int;
    pub fn isblank(c: c_int) -> c_int;
    pub fn toupper(c: c_int) -> c_int;
    pub fn tolower(c: c_int) -> c_int;

    // --- Wide characters ---
    pub fn wcscmp(s1: *const wchar_t, s2: *const wchar_t) -> c_int;
    pub fn wcslen(s: *const wchar_t) -> size_t;
    pub fn wcschr(s: *const wchar_t, c: wchar_t) -> *mut wchar_t;
    pub fn wcsrchr(s: *const wchar_t, c: wchar_t) -> *mut wchar_t;
    pub fn wmemchr(s: *const wchar_t, c: wchar_t, n: size_t) -> *mut wchar_t;
    pub fn wmemcmp(s1: *const wchar_t, s2: *const wchar_t, n: size_t) -> c_int;
    pub fn wmemcpy(dst: *mut wchar_t, src: *const wchar_t, n: size_t) -> *mut wchar_t;
    pub fn wmemset(s: *mut wchar_t, c: wchar_t, n: size_t) -> *mut wchar_t;
    pub fn mbrtowc(
        pwc: *mut wchar_t,
        s: *const c_char,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    pub fn wcrtomb(s: *mut c_char, wc: wchar_t, ps: *mut mbstate_t) -> size_t;

    // --- <signal.h> ---
    pub fn siginterrupt(sig: c_int, flag: c_int) -> c_int;

    // --- <fcntl.h> ---
    pub fn creat(pathname: *const c_char, mode: mode_t) -> c_int;
    pub fn posix_fadvise(fd: c_int, offset: off_t, len: off_t, advice: c_int) -> c_int;
    pub fn posix_fallocate(fd: c_int, offset: off_t, len: off_t) -> c_int;

    // --- <unistd.h> ---
    pub fn posix_close(fd: c_int, flag: c_int) -> c_int;
    pub fn fdatasync(fd: c_int) -> c_int;
    pub fn sync();
    pub fn syncfs(fd: c_int) -> c_int;
    pub fn setproctitle(fmt: *const c_char, ...);
    pub fn lockf(fd: c_int, cmd: c_int, len: off_t) -> c_int;
    pub fn lpathconf(path: *const c_char, name: c_int) -> c_long;
    pub fn _Fork() -> pid_t;
    pub fn execl(pathname: *const c_char, arg: *const c_char, ...) -> c_int;
    pub fn execle(pathname: *const c_char, arg: *const c_char, ...) -> c_int;
    pub fn execlp(file: *const c_char, arg: *const c_char, ...) -> c_int;
    pub fn execv(pathname: *const c_char, argv: *const *const c_char) -> c_int;
    pub fn fexecve(fd: c_int, argv: *const *const c_char, envp: *const *const c_char) -> c_int;
    pub fn nice(incr: c_int) -> c_int;
    pub fn alarm(seconds: c_uint) -> c_uint;
    pub fn pause() -> c_int;
    pub fn getlogin() -> *mut c_char;
    pub fn getlogin_r(name: *mut c_char, namesize: size_t) -> c_int;
    pub fn gethostid() -> c_long;
    pub fn getentropy(buf: *mut c_void, buflen: size_t) -> c_int;
    pub fn crypt(key: *const c_char, salt: *const c_char) -> *mut c_char;
    pub fn encrypt(block: *mut c_char, edflag: c_int);
    pub fn swab(src: *const c_void, dest: *mut c_void, nbytes: ssize_t);
    pub fn getopt(argc: c_int, argv: *const *const c_char, optstring: *const c_char) -> c_int;
    pub fn vfork() -> pid_t;
    pub fn setpgrp() -> c_int;
    pub fn execvpe(
        file: *const c_char,
        argv: *const *const c_char,
        envp: *const *const c_char,
    ) -> c_int;
    pub fn eaccess(pathname: *const c_char, mode: c_int) -> c_int;
    pub fn getdtablesize() -> c_int;
    pub fn getpagesize() -> c_int;
    pub fn sethostname(name: *const c_char, len: size_t) -> c_int;
    pub fn getdomainname(name: *mut c_char, len: size_t) -> c_int;
    pub fn setdomainname(name: *const c_char, len: size_t) -> c_int;
    pub fn issetugid() -> c_int;
    pub fn user_from_uid(uid: uid_t, noname: c_int) -> *const c_char;
    pub fn group_from_gid(gid: gid_t, noname: c_int) -> *const c_char;
    pub fn getusershell() -> *mut c_char;
    pub fn setusershell();
    pub fn endusershell();
    pub fn chflags(path: *const c_char, flags: c_ulong) -> c_int;
    pub fn lchflags(path: *const c_char, flags: c_ulong) -> c_int;
    pub fn fchflags(fd: c_int, flags: c_ulong) -> c_int;
    pub fn undelete(path: *const c_char) -> c_int;
    pub fn pledge(promises: *const c_char, execpromises: *const c_char) -> c_int;
    pub fn unveil(path: *const c_char, permissions: *const c_char) -> c_int;
    pub fn copy_file_range(
        fd_in: c_int,
        off_in: *mut off_t,
        fd_out: c_int,
        off_out: *mut off_t,
        len: size_t,
        flags: c_uint,
    ) -> ssize_t;

    // --- <sys/stat.h> ---
    pub fn lchmod(pathname: *const c_char, mode: mode_t) -> c_int;
    pub fn mkfifoat(dirfd: c_int, pathname: *const c_char, mode: mode_t) -> c_int;
    pub fn mknod(pathname: *const c_char, mode: mode_t, dev: dev_t) -> c_int;
    pub fn mknodat(dirfd: c_int, pathname: *const c_char, mode: mode_t, dev: dev_t) -> c_int;

    // --- <dirent.h> ---
    pub fn fdopendir(fd: c_int) -> *mut DIR;
    pub fn readdir_r(dirp: *mut DIR, entry: *mut dirent, result: *mut *mut dirent) -> c_int;
    pub fn telldir(dirp: *mut DIR) -> c_long;
    pub fn seekdir(dirp: *mut DIR, loc: c_long);
    pub fn posix_getdents(fd: c_int, buf: *mut c_void, nbyte: size_t, flags: c_int) -> ssize_t;
    pub fn scandir(
        dirname: *const c_char,
        namelist: *mut *mut *mut dirent,
        filter: Option<unsafe extern "C" fn(*const dirent) -> c_int>,
        compar: Option<unsafe extern "C" fn(*mut *const dirent, *mut *const dirent) -> c_int>,
    ) -> c_int;
    pub fn alphasort(a: *mut *const dirent, b: *mut *const dirent) -> c_int;

    // --- <poll.h> ---
    pub fn ppoll(
        fds: *mut pollfd,
        nfds: nfds_t,
        timeout: *const timespec,
        sigmask: *const sigset_t,
    ) -> c_int;

    // --- <sys/select.h> ---
    pub fn pselect(
        nfds: c_int,
        readfds: *mut fd_set,
        writefds: *mut fd_set,
        exceptfds: *mut fd_set,
        timeout: *const timespec,
        sigmask: *const sigset_t,
    ) -> c_int;

    // --- <sys/uio.h> ---
    pub fn preadv(fd: c_int, iov: *const iovec, iovcnt: c_int, offset: off_t) -> ssize_t;
    pub fn pwritev(fd: c_int, iov: *const iovec, iovcnt: c_int, offset: off_t) -> ssize_t;

    // --- <sys/statvfs.h> ---
    pub fn statvfs(path: *const c_char, buf: *mut statvfs) -> c_int;
    pub fn fstatvfs(fd: c_int, buf: *mut statvfs) -> c_int;

    // --- <sys/mount.h> ---
    pub fn statfs(path: *const c_char, buf: *mut statfs) -> c_int;
    pub fn fstatfs(fd: c_int, buf: *mut statfs) -> c_int;
    pub fn getmntinfo(mntbufp: *mut *mut statfs, mode: c_int) -> c_int;
    pub fn getfsstat(buf: *mut statfs, bufsize: c_long, mode: c_int) -> c_int;
    pub fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: c_ulong,
        data: *const c_void,
    ) -> c_int;
    pub fn pivot_root(new_root: *const c_char, put_old: *const c_char) -> c_int;
    pub fn umount(target: *const c_char) -> c_int;
    pub fn umount2(target: *const c_char, flags: c_int) -> c_int;

    // --- <ftw.h> ---
    pub fn ftw(
        path: *const c_char,
        fn_: Option<unsafe extern "C" fn(*const c_char, *const stat, c_int) -> c_int>,
        fd_limit: c_int,
    ) -> c_int;
    pub fn nftw(
        path: *const c_char,
        fn_: Option<unsafe extern "C" fn(*const c_char, *const stat, c_int, *mut FTW) -> c_int>,
        fd_limit: c_int,
        flags: c_int,
    ) -> c_int;

    // --- <sys/epoll.h> ---
    pub fn epoll_create(size: c_int) -> c_int;
    pub fn epoll_pwait(
        epfd: c_int,
        events: *mut epoll_event,
        maxevents: c_int,
        timeout: c_int,
        sigmask: *const sigset_t,
    ) -> c_int;

    // --- <sys/mman.h> ---
    pub fn msync(addr: *mut c_void, length: size_t, flags: c_int) -> c_int;
    pub fn mlock(addr: *const c_void, len: size_t) -> c_int;
    pub fn munlock(addr: *const c_void, len: size_t) -> c_int;
    pub fn mlockall(flags: c_int) -> c_int;
    pub fn munlockall() -> c_int;
    pub fn posix_madvise(addr: *mut c_void, length: size_t, advice: c_int) -> c_int;
    pub fn mseal(addr: *mut c_void, length: size_t, flags: c_ulong) -> c_int;
    pub fn shm_open(name: *const c_char, oflag: c_int, mode: mode_t) -> c_int;
    pub fn shm_unlink(name: *const c_char) -> c_int;

    // --- <sys/sysmacros.h> ---
    pub fn gnu_dev_major(dev: c_ulong) -> c_uint;
    pub fn gnu_dev_minor(dev: c_ulong) -> c_uint;
    pub fn gnu_dev_makedev(major: c_uint, minor: c_uint) -> c_ulong;

    // --- <sys/auxv.h> ---
    pub fn getauxval(type_: c_ulong) -> c_ulong;
    pub fn elf_aux_info(aux: c_int, buf: *mut c_void, buflen: c_int) -> c_int;

    // --- <sys/quota.h> ---
    pub fn quotactl(cmd: c_int, special: *const c_char, id: c_int, addr: *mut c_char) -> c_int;
    pub fn quotactl_fd(fd: c_uint, cmd: c_uint, id: c_int, addr: *mut c_void) -> c_int;

    // --- <sys/device_catalog.h> ---
    pub fn device_catalog_snapshot(
        cursor: u64,
        records: *mut device_record,
        capacity: size_t,
        written: *mut size_t,
        next_cursor: *mut u64,
        generation: *mut u64,
    ) -> c_int;
    pub fn device_catalog_watch(generation: u64, flags: c_int) -> c_int;
    pub fn device_catalog_read(fd: c_int, change: *mut device_change) -> c_int;
    pub fn device_events_monitor(flags: c_int, generation: *mut u64) -> c_int;
    pub fn device_events_read(fd: c_int, event: *mut device_event) -> c_int;
    pub fn device_properties(
        object_id: u64,
        buffer: *mut c_char,
        capacity: size_t,
        length: *mut size_t,
        incarnation: *mut u64,
    ) -> c_int;
    pub fn device_objects(
        cursor: u64,
        generation: u64,
        objects: *mut device_object,
        capacity: size_t,
        count: *mut size_t,
        next_cursor: *mut u64,
        out_generation: *mut u64,
    ) -> c_int;

    // --- <sys/wait.h> ---
    pub fn wait(wstatus: *mut c_int) -> pid_t;
    pub fn waitid(idtype: idtype_t, id: id_t, infop: *mut siginfo_t, options: c_int) -> c_int;
    pub fn wait3(wstatus: *mut c_int, options: c_int, rusage: *mut rusage) -> pid_t;
    pub fn wait4(pid: pid_t, wstatus: *mut c_int, options: c_int, rusage: *mut rusage) -> pid_t;

    // --- <spawn.h> ---
    pub fn posix_spawn(
        pid: *mut pid_t,
        path: *const c_char,
        file_actions: *const posix_spawn_file_actions_t,
        attrp: *const posix_spawnattr_t,
        argv: *const *const c_char,
        envp: *const *const c_char,
    ) -> c_int;
    pub fn posix_spawnp(
        pid: *mut pid_t,
        file: *const c_char,
        file_actions: *const posix_spawn_file_actions_t,
        attrp: *const posix_spawnattr_t,
        argv: *const *const c_char,
        envp: *const *const c_char,
    ) -> c_int;
    pub fn posix_spawnattr_init(attr: *mut posix_spawnattr_t) -> c_int;
    pub fn posix_spawnattr_destroy(attr: *mut posix_spawnattr_t) -> c_int;
    pub fn posix_spawnattr_getflags(attr: *const posix_spawnattr_t, flags: *mut c_short) -> c_int;
    pub fn posix_spawnattr_setflags(attr: *mut posix_spawnattr_t, flags: c_short) -> c_int;
    pub fn posix_spawnattr_getpgroup(attr: *const posix_spawnattr_t, pgroup: *mut pid_t) -> c_int;
    pub fn posix_spawnattr_setpgroup(attr: *mut posix_spawnattr_t, pgroup: pid_t) -> c_int;
    pub fn posix_spawnattr_getschedparam(
        attr: *const posix_spawnattr_t,
        param: *mut sched_param,
    ) -> c_int;
    pub fn posix_spawnattr_setschedparam(
        attr: *mut posix_spawnattr_t,
        param: *const sched_param,
    ) -> c_int;
    pub fn posix_spawnattr_getschedpolicy(
        attr: *const posix_spawnattr_t,
        policy: *mut c_int,
    ) -> c_int;
    pub fn posix_spawnattr_setschedpolicy(attr: *mut posix_spawnattr_t, policy: c_int) -> c_int;
    pub fn posix_spawnattr_getsigdefault(
        attr: *const posix_spawnattr_t,
        sigdefault: *mut sigset_t,
    ) -> c_int;
    pub fn posix_spawnattr_setsigdefault(
        attr: *mut posix_spawnattr_t,
        sigdefault: *const sigset_t,
    ) -> c_int;
    pub fn posix_spawnattr_getsigmask(
        attr: *const posix_spawnattr_t,
        sigmask: *mut sigset_t,
    ) -> c_int;
    pub fn posix_spawnattr_setsigmask(
        attr: *mut posix_spawnattr_t,
        sigmask: *const sigset_t,
    ) -> c_int;
    pub fn posix_spawn_file_actions_init(file_actions: *mut posix_spawn_file_actions_t) -> c_int;
    pub fn posix_spawn_file_actions_destroy(file_actions: *mut posix_spawn_file_actions_t)
        -> c_int;
    pub fn posix_spawn_file_actions_addclose(
        file_actions: *mut posix_spawn_file_actions_t,
        fildes: c_int,
    ) -> c_int;
    pub fn posix_spawn_file_actions_adddup2(
        file_actions: *mut posix_spawn_file_actions_t,
        fildes: c_int,
        newfildes: c_int,
    ) -> c_int;
    pub fn posix_spawn_file_actions_addopen(
        file_actions: *mut posix_spawn_file_actions_t,
        fildes: c_int,
        path: *const c_char,
        oflag: c_int,
        mode: mode_t,
    ) -> c_int;
    pub fn posix_spawn_file_actions_addchdir(
        file_actions: *mut posix_spawn_file_actions_t,
        path: *const c_char,
    ) -> c_int;
    pub fn posix_spawn_file_actions_addfchdir(
        file_actions: *mut posix_spawn_file_actions_t,
        fildes: c_int,
    ) -> c_int;

    // --- <sched.h> ---
    pub fn sched_get_priority_max(policy: c_int) -> c_int;
    pub fn sched_get_priority_min(policy: c_int) -> c_int;
    pub fn sched_getparam(pid: pid_t, param: *mut sched_param) -> c_int;
    pub fn sched_getscheduler(pid: pid_t) -> c_int;
    pub fn sched_rr_get_interval(pid: pid_t, interval: *mut timespec) -> c_int;
    pub fn sched_setparam(pid: pid_t, param: *const sched_param) -> c_int;
    pub fn sched_setscheduler(pid: pid_t, policy: c_int, param: *const sched_param) -> c_int;

    // --- <pwd.h> ---
    pub fn getpwent() -> *mut passwd;
    pub fn setpwent();
    pub fn endpwent();

    // --- <grp.h> ---
    pub fn getgrent() -> *mut group;
    pub fn setgrent();
    pub fn endgrent();

    // --- <shadow.h> ---
    pub fn getspent() -> *mut spwd;
    pub fn setspent();
    pub fn endspent();

    // --- <sys/resource.h> ---
    pub fn getrusage(who: c_int, usage: *mut rusage) -> c_int;

    // --- <utmpx.h> ---
    pub fn endutxent();
    pub fn getutxent() -> *mut utmpx;
    pub fn getutxid(id: *const utmpx) -> *mut utmpx;
    pub fn getutxline(line: *const utmpx) -> *mut utmpx;
    pub fn pututxline(utmpx: *const utmpx) -> *mut utmpx;
    pub fn setutxent();

    // --- <exec_operands.h> ---
    pub fn exec_operands_prepare(
        path: *const c_char,
        argv: *const *const c_char,
        envp: *const *const c_char,
        out: *mut exec_operands,
    ) -> c_int;
    pub fn exec_operands_inherit(ops: *mut exec_operands, clone_nonce: u64) -> c_int;
    pub fn exec_operands_release(ops: *mut exec_operands);

    // --- <auth_broker.h> ---
    pub fn auth_broker_begin(
        purpose: c_int,
        user: *const c_char,
        tty: *const c_char,
        reply: *mut auth_broker_reply,
    ) -> c_int;
    pub fn auth_broker_answer(
        text: *const c_char,
        len: size_t,
        reply: *mut auth_broker_reply,
    ) -> c_int;
    pub fn auth_broker_cancel() -> c_int;
    pub fn auth_broker_launch(
        ops: *const exec_operands,
        pid: *mut pid_t,
        clone_nonce: *mut u64,
    ) -> c_int;

    // --- <termios.h> ---
    pub fn cfsetspeed(termios_p: *mut termios, speed: speed_t) -> c_int;
    pub fn tcdrain(fd: c_int) -> c_int;
    pub fn tcflow(fd: c_int, action: c_int) -> c_int;
    pub fn tcflush(fd: c_int, queue_selector: c_int) -> c_int;
    pub fn tcgetwinsize(fd: c_int, winsize_p: *mut winsize) -> c_int;
    pub fn tcsendbreak(fd: c_int, duration: c_int) -> c_int;
    pub fn tcsetwinsize(fd: c_int, winsize_p: *const winsize) -> c_int;

    // --- <pty.h> ---
    pub fn forkpty(
        amaster: *mut c_int,
        name: *mut c_char,
        termp: *const termios,
        winp: *const winsize,
    ) -> pid_t;
    pub fn login_tty(fd: c_int) -> c_int;

    // --- <time.h> ---
    pub fn clock_settime(clock_id: clockid_t, tp: *const timespec) -> c_int;
    pub fn clock_getcpuclockid(pid: pid_t, clock_id: *mut clockid_t) -> c_int;
    pub fn clock_nanosleep(
        clock_id: clockid_t,
        flags: c_int,
        request: *const timespec,
        remain: *mut timespec,
    ) -> c_int;
    pub fn timespec_get(ts: *mut timespec, base: c_int) -> c_int;
    pub fn timespec_getres(ts: *mut timespec, base: c_int) -> c_int;
    pub fn timer_create(clock_id: clockid_t, event: *mut sigevent, timerid: *mut timer_t) -> c_int;
    pub fn timer_delete(timerid: timer_t) -> c_int;
    pub fn timer_settime(
        timerid: timer_t,
        flags: c_int,
        value: *const itimerspec,
        ovalue: *mut itimerspec,
    ) -> c_int;
    pub fn timer_gettime(timerid: timer_t, value: *mut itimerspec) -> c_int;
    pub fn timer_getoverrun(timerid: timer_t) -> c_int;
    pub fn strftime_l(
        s: *mut c_char,
        max: size_t,
        format: *const c_char,
        tm: *const tm,
        locale: locale_t,
    ) -> size_t;
    pub fn getdate(string: *const c_char) -> *mut tm;
    pub fn tzset();

    // --- <sys/time.h> ---
    pub fn getitimer(which: c_int, curr_value: *mut itimerval) -> c_int;
    pub fn setitimer(which: c_int, new_value: *const itimerval, old_value: *mut itimerval)
        -> c_int;
    pub fn utimes(filename: *const c_char, times: *const timeval) -> c_int;
    pub fn futimes(fd: c_int, times: *const timeval) -> c_int;
    pub fn lutimes(filename: *const c_char, times: *const timeval) -> c_int;

    // --- <sys/times.h> ---
    pub fn times(buf: *mut tms) -> clock_t;

    // --- <sys/ipc.h> ---
    pub fn ftok(path: *const c_char, id: c_int) -> key_t;

    // --- <sys/shm.h> ---
    pub fn shmget(key: key_t, size: size_t, shmflg: c_int) -> c_int;
    pub fn shmat(shmid: c_int, shmaddr: *const c_void, shmflg: c_int) -> *mut c_void;
    pub fn shmdt(shmaddr: *const c_void) -> c_int;
    pub fn shmctl(shmid: c_int, cmd: c_int, buf: *mut shmid_ds) -> c_int;

    // --- <sys/sem.h> ---
    pub fn semget(key: key_t, nsems: c_int, semflg: c_int) -> c_int;
    pub fn semop(semid: c_int, sops: *mut sembuf, nsops: size_t) -> c_int;
    pub fn semctl(semid: c_int, semnum: c_int, cmd: c_int, ...) -> c_int;
    pub fn semtimedop(
        semid: c_int,
        sops: *mut sembuf,
        nsops: size_t,
        timeout: *const timespec,
    ) -> c_int;

    // --- <sys/msg.h> ---
    pub fn msgget(key: key_t, msgflg: c_int) -> c_int;
    pub fn msgsnd(msqid: c_int, msgp: *const c_void, msgsz: size_t, msgflg: c_int) -> c_int;
    pub fn msgrcv(
        msqid: c_int,
        msgp: *mut c_void,
        msgsz: size_t,
        msgtyp: c_long,
        msgflg: c_int,
    ) -> ssize_t;
    pub fn msgctl(msqid: c_int, cmd: c_int, buf: *mut msqid_ds) -> c_int;

    // --- <pthread.h> ---
    pub fn pthread_atfork(
        prepare: Option<unsafe extern "C" fn()>,
        parent: Option<unsafe extern "C" fn()>,
        child: Option<unsafe extern "C" fn()>,
    ) -> c_int;
    pub fn pthread_attr_setstack(
        attr: *mut pthread_attr_t,
        stackaddr: *mut c_void,
        stacksize: size_t,
    ) -> c_int;
    pub fn pthread_attr_getstack(
        attr: *const pthread_attr_t,
        stackaddr: *mut *mut c_void,
        stacksize: *mut size_t,
    ) -> c_int;
    pub fn pthread_attr_setguardsize(attr: *mut pthread_attr_t, guardsize: size_t) -> c_int;
    pub fn pthread_attr_getguardsize(attr: *const pthread_attr_t, guardsize: *mut size_t) -> c_int;
    pub fn pthread_attr_setinheritsched(attr: *mut pthread_attr_t, inheritsched: c_int) -> c_int;
    pub fn pthread_attr_getinheritsched(
        attr: *const pthread_attr_t,
        inheritsched: *mut c_int,
    ) -> c_int;
    pub fn pthread_attr_setschedpolicy(attr: *mut pthread_attr_t, policy: c_int) -> c_int;
    pub fn pthread_attr_getschedpolicy(attr: *const pthread_attr_t, policy: *mut c_int) -> c_int;
    pub fn pthread_attr_setschedparam(
        attr: *mut pthread_attr_t,
        param: *const sched_param,
    ) -> c_int;
    pub fn pthread_attr_getschedparam(
        attr: *const pthread_attr_t,
        param: *mut sched_param,
    ) -> c_int;
    pub fn pthread_attr_setscope(attr: *mut pthread_attr_t, scope: c_int) -> c_int;
    pub fn pthread_attr_getscope(attr: *const pthread_attr_t, scope: *mut c_int) -> c_int;
    pub fn pthread_setschedparam(
        thread: pthread_t,
        policy: c_int,
        param: *const sched_param,
    ) -> c_int;
    pub fn pthread_getschedparam(
        thread: pthread_t,
        policy: *mut c_int,
        param: *mut sched_param,
    ) -> c_int;
    pub fn pthread_setschedprio(thread: pthread_t, prio: c_int) -> c_int;
    pub fn pthread_getcpuclockid(thread: pthread_t, clock_id: *mut clockid_t) -> c_int;
    pub fn pthread_setconcurrency(new_level: c_int) -> c_int;
    pub fn pthread_getconcurrency() -> c_int;
    pub fn pthread_mutexattr_setpshared(attr: *mut pthread_mutexattr_t, pshared: c_int) -> c_int;
    pub fn pthread_mutexattr_getpshared(
        attr: *const pthread_mutexattr_t,
        pshared: *mut c_int,
    ) -> c_int;
    pub fn pthread_mutexattr_setrobust(attr: *mut pthread_mutexattr_t, robust: c_int) -> c_int;
    pub fn pthread_mutexattr_getrobust(
        attr: *const pthread_mutexattr_t,
        robust: *mut c_int,
    ) -> c_int;
    pub fn pthread_mutex_clocklock(
        mutex: *mut pthread_mutex_t,
        clock_id: clockid_t,
        abstime: *const timespec,
    ) -> c_int;
    pub fn pthread_mutex_consistent(mutex: *mut pthread_mutex_t) -> c_int;
    pub fn pthread_condattr_setclock(attr: *mut pthread_condattr_t, clock_id: clockid_t) -> c_int;
    pub fn pthread_condattr_getclock(
        attr: *const pthread_condattr_t,
        clock_id: *mut clockid_t,
    ) -> c_int;
    pub fn pthread_condattr_setpshared(attr: *mut pthread_condattr_t, pshared: c_int) -> c_int;
    pub fn pthread_condattr_getpshared(
        attr: *const pthread_condattr_t,
        pshared: *mut c_int,
    ) -> c_int;
    pub fn pthread_cond_clockwait(
        cond: *mut pthread_cond_t,
        mutex: *mut pthread_mutex_t,
        clock_id: clockid_t,
        abstime: *const timespec,
    ) -> c_int;
    pub fn pthread_rwlockattr_init(attr: *mut pthread_rwlockattr_t) -> c_int;
    pub fn pthread_rwlockattr_destroy(attr: *mut pthread_rwlockattr_t) -> c_int;
    pub fn pthread_rwlockattr_setpshared(attr: *mut pthread_rwlockattr_t, pshared: c_int) -> c_int;
    pub fn pthread_rwlockattr_getpshared(
        attr: *const pthread_rwlockattr_t,
        pshared: *mut c_int,
    ) -> c_int;
    pub fn pthread_rwlock_timedrdlock(
        rwlock: *mut pthread_rwlock_t,
        abstime: *const timespec,
    ) -> c_int;
    pub fn pthread_rwlock_timedwrlock(
        rwlock: *mut pthread_rwlock_t,
        abstime: *const timespec,
    ) -> c_int;
    pub fn pthread_rwlock_clockrdlock(
        rwlock: *mut pthread_rwlock_t,
        clock_id: clockid_t,
        abstime: *const timespec,
    ) -> c_int;
    pub fn pthread_rwlock_clockwrlock(
        rwlock: *mut pthread_rwlock_t,
        clock_id: clockid_t,
        abstime: *const timespec,
    ) -> c_int;
    pub fn pthread_barrierattr_init(attr: *mut pthread_barrierattr_t) -> c_int;
    pub fn pthread_barrierattr_destroy(attr: *mut pthread_barrierattr_t) -> c_int;
    pub fn pthread_barrierattr_setpshared(
        attr: *mut pthread_barrierattr_t,
        pshared: c_int,
    ) -> c_int;
    pub fn pthread_barrierattr_getpshared(
        attr: *const pthread_barrierattr_t,
        pshared: *mut c_int,
    ) -> c_int;
    pub fn pthread_spin_init(lock: *mut pthread_spinlock_t, pshared: c_int) -> c_int;
    pub fn pthread_spin_destroy(lock: *mut pthread_spinlock_t) -> c_int;
    pub fn pthread_spin_lock(lock: *mut pthread_spinlock_t) -> c_int;
    pub fn pthread_spin_trylock(lock: *mut pthread_spinlock_t) -> c_int;
    pub fn pthread_spin_unlock(lock: *mut pthread_spinlock_t) -> c_int;
    pub fn pthread_setname_np(thread: pthread_t, name: *const c_char) -> c_int;
    pub fn pthread_getname_np(thread: pthread_t, name: *mut c_char, len: size_t) -> c_int;

    // --- <pthread_np.h> ---
    pub fn pthread_getthreadid_np() -> c_int;
    pub fn pthread_set_name_np(thread: pthread_t, name: *const c_char);
    pub fn pthread_get_name_np(thread: pthread_t, name: *mut c_char, len: size_t);

    // --- <sys/umtx.h> ---
    pub fn _umtx_op(
        obj: *mut c_void,
        op: c_int,
        val: c_ulong,
        uaddr: *mut c_void,
        uaddr2: *mut c_void,
    ) -> c_int;

    // --- <semaphore.h> ---
    pub fn sem_init(sem: *mut sem_t, pshared: c_int, value: c_uint) -> c_int;
    pub fn sem_destroy(sem: *mut sem_t) -> c_int;
    pub fn sem_open(name: *const c_char, oflag: c_int, ...) -> *mut sem_t;
    pub fn sem_close(sem: *mut sem_t) -> c_int;
    pub fn sem_unlink(name: *const c_char) -> c_int;
    pub fn sem_wait(sem: *mut sem_t) -> c_int;
    pub fn sem_trywait(sem: *mut sem_t) -> c_int;
    pub fn sem_timedwait(sem: *mut sem_t, abstime: *const timespec) -> c_int;
    pub fn sem_clockwait(sem: *mut sem_t, clock_id: clockid_t, abstime: *const timespec) -> c_int;
    pub fn sem_post(sem: *mut sem_t) -> c_int;
    pub fn sem_getvalue(sem: *mut sem_t, sval: *mut c_int) -> c_int;

    // --- <threads.h> ---
    pub fn thrd_create(thr: *mut thrd_t, func: thrd_start_t, arg: *mut c_void) -> c_int;
    pub fn thrd_current() -> thrd_t;
    pub fn thrd_detach(thr: thrd_t) -> c_int;
    pub fn thrd_equal(thr0: thrd_t, thr1: thrd_t) -> c_int;
    pub fn thrd_exit(res: c_int) -> !;
    pub fn thrd_join(thr: thrd_t, res: *mut c_int) -> c_int;
    pub fn thrd_sleep(duration: *const timespec, remaining: *mut timespec) -> c_int;
    pub fn thrd_yield();
    pub fn mtx_init(mtx: *mut mtx_t, type_: c_int) -> c_int;
    pub fn mtx_destroy(mtx: *mut mtx_t);
    pub fn mtx_lock(mtx: *mut mtx_t) -> c_int;
    pub fn mtx_timedlock(mtx: *mut mtx_t, ts: *const timespec) -> c_int;
    pub fn mtx_trylock(mtx: *mut mtx_t) -> c_int;
    pub fn mtx_unlock(mtx: *mut mtx_t) -> c_int;
    pub fn cnd_init(cond: *mut cnd_t) -> c_int;
    pub fn cnd_destroy(cond: *mut cnd_t);
    pub fn cnd_signal(cond: *mut cnd_t) -> c_int;
    pub fn cnd_broadcast(cond: *mut cnd_t) -> c_int;
    pub fn cnd_wait(cond: *mut cnd_t, mtx: *mut mtx_t) -> c_int;
    pub fn cnd_timedwait(cond: *mut cnd_t, mtx: *mut mtx_t, ts: *const timespec) -> c_int;
    pub fn tss_create(key: *mut tss_t, dtor: tss_dtor_t) -> c_int;
    pub fn tss_delete(key: tss_t);
    pub fn tss_get(key: tss_t) -> *mut c_void;
    pub fn tss_set(key: tss_t, val: *mut c_void) -> c_int;
    pub fn call_once(flag: *mut once_flag, func: Option<unsafe extern "C" fn()>);

    // --- <aio.h> ---
    pub fn aio_read(aiocbp: *mut aiocb) -> c_int;
    pub fn aio_write(aiocbp: *mut aiocb) -> c_int;
    pub fn aio_fsync(op: c_int, aiocbp: *mut aiocb) -> c_int;
    pub fn aio_error(aiocbp: *const aiocb) -> c_int;
    pub fn aio_return(aiocbp: *mut aiocb) -> ssize_t;
    pub fn aio_cancel(fildes: c_int, aiocbp: *mut aiocb) -> c_int;
    pub fn aio_suspend(list: *const *const aiocb, nent: c_int, timeout: *const timespec) -> c_int;
    pub fn lio_listio(
        mode: c_int,
        list: *const *mut aiocb,
        nent: c_int,
        sig: *mut sigevent,
    ) -> c_int;

    // --- <sys/socket.h> ---
    pub fn accept4(
        sockfd: c_int,
        addr: *mut sockaddr,
        addrlen: *mut socklen_t,
        flags: c_int,
    ) -> c_int;
    pub fn sockatmark(sockfd: c_int) -> c_int;

    // --- <arpa/inet.h> ---
    pub fn inet_aton(cp: *const c_char, inp: *mut in_addr) -> c_int;
    pub fn inet_ntoa(in_: in_addr) -> *mut c_char;

    // --- <netdb.h> ---
    pub fn hstrerror(err: c_int) -> *const c_char;
    pub fn gethostbyname(name: *const c_char) -> *mut hostent;
    pub fn gethostbyname2(name: *const c_char, af: c_int) -> *mut hostent;
    pub fn gethostbyaddr(addr: *const c_void, len: socklen_t, type_: c_int) -> *mut hostent;
    pub fn gethostent() -> *mut hostent;
    pub fn sethostent(stayopen: c_int);
    pub fn endhostent();
    pub fn getnetbyname(name: *const c_char) -> *mut netent;
    pub fn getnetbyaddr(net: u32, type_: c_int) -> *mut netent;
    pub fn getnetent() -> *mut netent;
    pub fn setnetent(stayopen: c_int);
    pub fn endnetent();
    pub fn getprotobyname(name: *const c_char) -> *mut protoent;
    pub fn getprotobynumber(proto: c_int) -> *mut protoent;
    pub fn getprotoent() -> *mut protoent;
    pub fn setprotoent(stayopen: c_int);
    pub fn endprotoent();
    pub fn getservbyname(name: *const c_char, proto: *const c_char) -> *mut servent;
    pub fn getservbyport(port: c_int, proto: *const c_char) -> *mut servent;
    pub fn getservent() -> *mut servent;
    pub fn setservent(stayopen: c_int);
    pub fn endservent();
    pub fn innetgr(
        netgroup: *const c_char,
        host: *const c_char,
        user: *const c_char,
        domain: *const c_char,
    ) -> c_int;

    // --- <sys/sysctl.h> ---
    pub fn sysctl(
        name: *const c_int,
        namelen: c_uint,
        oldp: *mut c_void,
        oldlenp: *mut size_t,
        newp: *const c_void,
        newlen: size_t,
    ) -> c_int;
    pub fn sysctlbyname(
        name: *const c_char,
        oldp: *mut c_void,
        oldlenp: *mut size_t,
        newp: *const c_void,
        newlen: size_t,
    ) -> c_int;
    pub fn sysctlnametomib(name: *const c_char, mibp: *mut c_int, sizep: *mut size_t) -> c_int;

    // --- <dlfcn.h> ---
    pub fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    pub fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    pub fn dlclose(handle: *mut c_void) -> c_int;
    pub fn dlerror() -> *mut c_char;
    pub fn dladdr(addr: *const c_void, info: *mut Dl_info) -> c_int;
    pub fn dlfunc(handle: *mut c_void, symbol: *const c_char) -> dlfunc_t;

    // --- <stdio.h> ---
    pub fn fdopen(fd: c_int, mode: *const c_char) -> *mut FILE;
    pub fn fmemopen(buf: *mut c_void, size: size_t, mode: *const c_char) -> *mut FILE;
    pub fn open_memstream(ptr: *mut *mut c_char, sizeloc: *mut size_t) -> *mut FILE;
    pub fn fseeko(stream: *mut FILE, offset: off_t, whence: c_int) -> c_int;
    pub fn ftello(stream: *mut FILE) -> off_t;
    pub fn fgetpos(stream: *mut FILE, pos: *mut fpos_t) -> c_int;
    pub fn fsetpos(stream: *mut FILE, pos: *const fpos_t) -> c_int;
    pub fn flockfile(stream: *mut FILE);
    pub fn funlockfile(stream: *mut FILE);
    pub fn ftrylockfile(stream: *mut FILE) -> c_int;
    pub fn setbuffer(stream: *mut FILE, buf: *mut c_char, size: size_t);
    pub fn setlinebuf(stream: *mut FILE);
    pub fn dprintf(fd: c_int, format: *const c_char, ...) -> c_int;
    pub fn asprintf(strp: *mut *mut c_char, format: *const c_char, ...) -> c_int;
    pub fn scanf(format: *const c_char, ...) -> c_int;
    pub fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    pub fn remove(pathname: *const c_char) -> c_int;
    pub fn tmpfile() -> *mut FILE;
    pub fn tmpnam(s: *mut c_char) -> *mut c_char;
    pub fn tempnam(dir: *const c_char, pfx: *const c_char) -> *mut c_char;
    pub fn ctermid(s: *mut c_char) -> *mut c_char;
    pub fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> ssize_t;
    pub fn getdelim(
        lineptr: *mut *mut c_char,
        n: *mut size_t,
        delim: c_int,
        stream: *mut FILE,
    ) -> ssize_t;
    pub fn popen(command: *const c_char, type_: *const c_char) -> *mut FILE;
    pub fn pclose(stream: *mut FILE) -> c_int;
    pub fn funopen(
        cookie: *const c_void,
        readfn: funopen_readfn,
        writefn: funopen_writefn,
        seekfn: funopen_seekfn,
        closefn: funopen_closefn,
    ) -> *mut FILE;
    pub fn getc_unlocked(stream: *mut FILE) -> c_int;
    pub fn getchar_unlocked() -> c_int;
    pub fn putc_unlocked(c: c_int, stream: *mut FILE) -> c_int;
    pub fn putchar_unlocked(c: c_int) -> c_int;
    pub fn fgetc_unlocked(stream: *mut FILE) -> c_int;
    pub fn fputc_unlocked(c: c_int, stream: *mut FILE) -> c_int;
    pub fn fgets_unlocked(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    pub fn fputs_unlocked(s: *const c_char, stream: *mut FILE) -> c_int;
    pub fn fread_unlocked(
        ptr: *mut c_void,
        size: size_t,
        nmemb: size_t,
        stream: *mut FILE,
    ) -> size_t;
    pub fn fwrite_unlocked(
        ptr: *const c_void,
        size: size_t,
        nmemb: size_t,
        stream: *mut FILE,
    ) -> size_t;
    pub fn clearerr_unlocked(stream: *mut FILE);
    pub fn feof_unlocked(stream: *mut FILE) -> c_int;
    pub fn ferror_unlocked(stream: *mut FILE) -> c_int;
    pub fn fileno_unlocked(stream: *mut FILE) -> c_int;
    pub fn fflush_unlocked(stream: *mut FILE) -> c_int;
    pub fn fpurge(stream: *mut FILE) -> c_int;

    // --- <stdlib.h> ---
    pub fn reallocarray(ptr: *mut c_void, nmemb: size_t, size: size_t) -> *mut c_void;
    pub fn quick_exit(status: c_int) -> !;
    pub fn atexit(function: Option<unsafe extern "C" fn()>) -> c_int;
    pub fn at_quick_exit(function: Option<unsafe extern "C" fn()>) -> c_int;
    pub fn div(numer: c_int, denom: c_int) -> div_t;
    pub fn ldiv(numer: c_long, denom: c_long) -> ldiv_t;
    pub fn lldiv(numer: c_longlong, denom: c_longlong) -> lldiv_t;
    pub fn rand_r(seedp: *mut c_uint) -> c_int;
    pub fn random() -> c_long;
    pub fn srandom(seed: c_uint);
    pub fn initstate(seed: c_uint, state: *mut c_char, size: size_t) -> *mut c_char;
    pub fn setstate(state: *mut c_char) -> *mut c_char;
    pub fn drand48() -> c_double;
    pub fn erand48(xsubi: *mut c_ushort) -> c_double;
    pub fn lrand48() -> c_long;
    pub fn nrand48(xsubi: *mut c_ushort) -> c_long;
    pub fn mrand48() -> c_long;
    pub fn jrand48(xsubi: *mut c_ushort) -> c_long;
    pub fn srand48(seedval: c_long);
    pub fn seed48(seed16v: *mut c_ushort) -> *mut c_ushort;
    pub fn lcong48(param: *mut c_ushort);
    pub fn arc4random() -> u32;
    pub fn arc4random_buf(buf: *mut c_void, n: size_t);
    pub fn arc4random_uniform(upper_bound: u32) -> u32;
    pub fn a64l(s: *const c_char) -> c_long;
    pub fn l64a(value: c_long) -> *mut c_char;
    pub fn secure_getenv(name: *const c_char) -> *mut c_char;
    pub fn clearenv() -> c_int;
    pub fn getsubopt(
        optionp: *mut *mut c_char,
        keylistp: *const *mut c_char,
        valuep: *mut *mut c_char,
    ) -> c_int;
    pub fn qsort_r(
        base: *mut c_void,
        nmemb: size_t,
        size: size_t,
        compar: Option<unsafe extern "C" fn(*const c_void, *const c_void, *mut c_void) -> c_int>,
        arg: *mut c_void,
    );
    pub fn mkstemp(tmpl: *mut c_char) -> c_int;
    pub fn mkostemp(tmpl: *mut c_char, flags: c_int) -> c_int;
    pub fn mkstemps(tmpl: *mut c_char, suffixlen: c_int) -> c_int;
    pub fn mkostemps(tmpl: *mut c_char, suffixlen: c_int, flags: c_int) -> c_int;
    pub fn mktemp(tmpl: *mut c_char) -> *mut c_char;
    pub fn mblen(s: *const c_char, n: size_t) -> c_int;
    pub fn mbstowcs(dst: *mut wchar_t, src: *const c_char, n: size_t) -> size_t;
    pub fn wcstombs(dst: *mut c_char, src: *const wchar_t, n: size_t) -> size_t;
    pub fn mbtowc(pwc: *mut wchar_t, s: *const c_char, n: size_t) -> c_int;
    pub fn wctomb(s: *mut c_char, wc: wchar_t) -> c_int;
    pub fn posix_openpt(oflag: c_int) -> c_int;
    pub fn system(command: *const c_char) -> c_int;
    pub fn setkey(key: *const c_char);
    pub fn getprogname() -> *const c_char;
    pub fn setprogname(name: *const c_char);
    pub fn getprogpath(buf: *mut c_char, argv0: *const c_char) -> *mut c_char;
    pub fn strmode(mode: c_int, bp: *mut c_char);
    pub fn setmode(mode_str: *const c_char) -> *mut c_void;
    pub fn getmode(set: *const c_void, omode: mode_t) -> mode_t;
    pub fn strtonum(
        numstr: *const c_char,
        minval: c_longlong,
        maxval: c_longlong,
        errstrp: *mut *const c_char,
    ) -> c_longlong;
    pub fn strtoq(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_longlong;
    pub fn strtouq(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulonglong;
    pub fn getbsize(headerlenp: *mut c_int, blocksizep: *mut c_long) -> *mut c_char;
    pub fn reallocf(ptr: *mut c_void, size: size_t) -> *mut c_void;
    pub fn rpmatch(response: *const c_char) -> c_int;
    pub fn heapsort(
        base: *mut c_void,
        nmemb: size_t,
        size: size_t,
        compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
    ) -> c_int;
    pub fn mergesort(
        base: *mut c_void,
        nmemb: size_t,
        size: size_t,
        compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
    ) -> c_int;
    pub fn fflagstostr(flags: c_ulong) -> *mut c_char;
    pub fn getloadavg(loadavg: *mut c_double, nelem: c_int) -> c_int;

    // --- <malloc.h> ---
    pub fn malloc_usable_size(pointer: *mut c_void) -> size_t;

    // --- <string.h> ---
    pub fn memccpy(dest: *mut c_void, src: *const c_void, c: c_int, n: size_t) -> *mut c_void;
    pub fn memmem(
        haystack: *const c_void,
        haystacklen: size_t,
        needle: *const c_void,
        needlelen: size_t,
    ) -> *mut c_void;
    pub fn mempcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    pub fn strcoll(s1: *const c_char, s2: *const c_char) -> c_int;
    pub fn strcoll_l(s1: *const c_char, s2: *const c_char, locale: locale_t) -> c_int;
    pub fn strxfrm(dest: *mut c_char, src: *const c_char, n: size_t) -> size_t;
    pub fn strxfrm_l(dest: *mut c_char, src: *const c_char, n: size_t, locale: locale_t) -> size_t;
    pub fn strchrnul(s: *const c_char, c: c_int) -> *mut c_char;
    pub fn strcasestr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    pub fn strnstr(haystack: *const c_char, needle: *const c_char, len: size_t) -> *mut c_char;
    pub fn strtok(str_: *mut c_char, delim: *const c_char) -> *mut c_char;
    pub fn strerror_l(errnum: c_int, locale: locale_t) -> *mut c_char;
    pub fn bzero(s: *mut c_void, n: size_t);
    pub fn bcopy(src: *const c_void, dest: *mut c_void, n: size_t);
    pub fn strlcpy(dst: *mut c_char, src: *const c_char, dstsize: size_t) -> size_t;
    pub fn strlcat(dst: *mut c_char, src: *const c_char, dstsize: size_t) -> size_t;
    pub fn strsep(stringp: *mut *mut c_char, delim: *const c_char) -> *mut c_char;
    pub fn strverscmp(s1: *const c_char, s2: *const c_char) -> c_int;

    // --- <strings.h> ---
    pub fn ffs(i: c_int) -> c_int;
    pub fn ffsl(i: c_long) -> c_int;
    pub fn ffsll(i: c_longlong) -> c_int;
    pub fn strcasecmp_l(s1: *const c_char, s2: *const c_char, locale: locale_t) -> c_int;
    pub fn strncasecmp_l(
        s1: *const c_char,
        s2: *const c_char,
        n: size_t,
        locale: locale_t,
    ) -> c_int;

    // --- <ctype.h> ---
    pub fn isalnum_l(c: c_int, locale: locale_t) -> c_int;
    pub fn isalpha_l(c: c_int, locale: locale_t) -> c_int;
    pub fn isblank_l(c: c_int, locale: locale_t) -> c_int;
    pub fn iscntrl_l(c: c_int, locale: locale_t) -> c_int;
    pub fn isdigit_l(c: c_int, locale: locale_t) -> c_int;
    pub fn isgraph_l(c: c_int, locale: locale_t) -> c_int;
    pub fn islower_l(c: c_int, locale: locale_t) -> c_int;
    pub fn isprint_l(c: c_int, locale: locale_t) -> c_int;
    pub fn ispunct_l(c: c_int, locale: locale_t) -> c_int;
    pub fn isspace_l(c: c_int, locale: locale_t) -> c_int;
    pub fn isupper_l(c: c_int, locale: locale_t) -> c_int;
    pub fn isxdigit_l(c: c_int, locale: locale_t) -> c_int;
    pub fn tolower_l(c: c_int, locale: locale_t) -> c_int;
    pub fn toupper_l(c: c_int, locale: locale_t) -> c_int;
    pub fn isascii(c: c_int) -> c_int;
    pub fn toascii(c: c_int) -> c_int;

    // --- <fnmatch.h> ---
    pub fn fnmatch(pattern: *const c_char, string: *const c_char, flags: c_int) -> c_int;

    // --- <regex.h> ---
    pub fn regcomp(preg: *mut regex_t, pattern: *const c_char, cflags: c_int) -> c_int;
    pub fn regexec(
        preg: *const regex_t,
        string: *const c_char,
        nmatch: size_t,
        pmatch: *mut regmatch_t,
        eflags: c_int,
    ) -> c_int;
    pub fn regfree(preg: *mut regex_t);
    pub fn regerror(
        errcode: c_int,
        preg: *const regex_t,
        errbuf: *mut c_char,
        errbuf_size: size_t,
    ) -> size_t;

    // --- <search.h> ---
    pub fn hcreate(nel: size_t) -> c_int;
    pub fn hdestroy();
    pub fn hsearch(item: ENTRY, action: ACTION) -> *mut ENTRY;
    pub fn insque(element: *mut c_void, pred: *mut c_void);
    pub fn remque(element: *mut c_void);
    pub fn lfind(
        key: *const c_void,
        base: *const c_void,
        nelp: *mut size_t,
        width: size_t,
        compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
    ) -> *mut c_void;
    pub fn lsearch(
        key: *const c_void,
        base: *mut c_void,
        nelp: *mut size_t,
        width: size_t,
        compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
    ) -> *mut c_void;
    pub fn tsearch(
        key: *const c_void,
        rootp: *mut *mut posix_tnode,
        compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
    ) -> *mut posix_tnode;
    pub fn tfind(
        key: *const c_void,
        rootp: *const *mut posix_tnode,
        compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
    ) -> *mut posix_tnode;
    pub fn tdelete(
        key: *const c_void,
        rootp: *mut *mut posix_tnode,
        compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
    ) -> *mut c_void;
    pub fn twalk(
        root: *const posix_tnode,
        action: Option<unsafe extern "C" fn(*const posix_tnode, VISIT, c_int)>,
    );

    // --- <inttypes.h> ---
    pub fn imaxabs(j: intmax_t) -> intmax_t;
    pub fn imaxdiv(numer: intmax_t, denom: intmax_t) -> imaxdiv_t;
    pub fn wcstoimax(nptr: *const wchar_t, endptr: *mut *mut wchar_t, base: c_int) -> intmax_t;
    pub fn wcstoumax(nptr: *const wchar_t, endptr: *mut *mut wchar_t, base: c_int) -> uintmax_t;

    // --- <wordexp.h> ---
    pub fn wordexp(words: *const c_char, pwordexp: *mut wordexp_t, flags: c_int) -> c_int;
    pub fn wordfree(pwordexp: *mut wordexp_t);

    // --- <fmtmsg.h> ---
    pub fn fmtmsg(
        classification: c_long,
        label: *const c_char,
        severity: c_int,
        text: *const c_char,
        action: *const c_char,
        tag: *const c_char,
    ) -> c_int;

    // --- <ndbm.h> ---
    pub fn dbm_open(file: *const c_char, open_flags: c_int, file_mode: mode_t) -> *mut DBM;
    pub fn dbm_close(db: *mut DBM);
    pub fn dbm_fetch(db: *mut DBM, key: datum) -> datum;
    pub fn dbm_store(db: *mut DBM, key: datum, content: datum, store_mode: c_int) -> c_int;
    pub fn dbm_delete(db: *mut DBM, key: datum) -> c_int;
    pub fn dbm_firstkey(db: *mut DBM) -> datum;
    pub fn dbm_nextkey(db: *mut DBM) -> datum;
    pub fn dbm_error(db: *mut DBM) -> c_int;
    pub fn dbm_clearerr(db: *mut DBM) -> c_int;

    // --- <crypt.h> ---
    pub fn crypt_r(key: *const c_char, salt: *const c_char, data: *mut crypt_data) -> *mut c_char;

    // --- <locale.h> ---
    pub fn newlocale(category_mask: c_int, locale: *const c_char, base: locale_t) -> locale_t;
    pub fn duplocale(locobj: locale_t) -> locale_t;
    pub fn freelocale(locobj: locale_t);
    pub fn uselocale(newloc: locale_t) -> locale_t;
    pub fn getlocalename_l(category: c_int, locobj: locale_t) -> *const c_char;

    // --- <xlocale.h> ---
    pub fn localeconv_l(locale: locale_t) -> *mut lconv;
    pub fn btowc_l(c: c_int, locale: locale_t) -> wint_t;
    pub fn wctob_l(c: wint_t, locale: locale_t) -> c_int;
    pub fn mbtowc_l(pwc: *mut wchar_t, s: *const c_char, n: size_t, locale: locale_t) -> c_int;
    pub fn mbrlen_l(s: *const c_char, n: size_t, ps: *mut mbstate_t, locale: locale_t) -> size_t;
    pub fn mbrtowc_l(
        pwc: *mut wchar_t,
        s: *const c_char,
        n: size_t,
        ps: *mut mbstate_t,
        locale: locale_t,
    ) -> size_t;
    pub fn wcrtomb_l(s: *mut c_char, wc: wchar_t, ps: *mut mbstate_t, locale: locale_t) -> size_t;
    pub fn mbsrtowcs_l(
        dst: *mut wchar_t,
        src: *mut *const c_char,
        len: size_t,
        ps: *mut mbstate_t,
        locale: locale_t,
    ) -> size_t;
    pub fn mbsnrtowcs_l(
        dst: *mut wchar_t,
        src: *mut *const c_char,
        nms: size_t,
        len: size_t,
        ps: *mut mbstate_t,
        locale: locale_t,
    ) -> size_t;
    pub fn wcsnrtombs_l(
        dst: *mut c_char,
        src: *mut *const wchar_t,
        nwc: size_t,
        len: size_t,
        ps: *mut mbstate_t,
        locale: locale_t,
    ) -> size_t;

    // --- <langinfo.h> ---
    pub fn nl_langinfo(item: nl_item) -> *mut c_char;
    pub fn nl_langinfo_l(item: nl_item, locale: locale_t) -> *mut c_char;

    // --- <nl_types.h> ---
    pub fn catopen(name: *const c_char, oflag: c_int) -> nl_catd;
    pub fn catgets(catd: nl_catd, set_id: c_int, msg_id: c_int, s: *const c_char) -> *mut c_char;
    pub fn catclose(catd: nl_catd) -> c_int;

    // --- <wchar.h> ---
    pub fn btowc(c: c_int) -> wint_t;
    pub fn wctob(c: wint_t) -> c_int;
    pub fn mbsinit(ps: *const mbstate_t) -> c_int;
    pub fn mbrlen(s: *const c_char, n: size_t, ps: *mut mbstate_t) -> size_t;
    pub fn mbsrtowcs(
        dst: *mut wchar_t,
        src: *mut *const c_char,
        len: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    pub fn mbsnrtowcs(
        dst: *mut wchar_t,
        src: *mut *const c_char,
        nms: size_t,
        len: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    pub fn wcsrtombs(
        dst: *mut c_char,
        src: *mut *const wchar_t,
        len: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    pub fn wcsnrtombs(
        dst: *mut c_char,
        src: *mut *const wchar_t,
        nwc: size_t,
        len: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    pub fn wcpcpy(dst: *mut wchar_t, src: *const wchar_t) -> *mut wchar_t;
    pub fn wcpncpy(dst: *mut wchar_t, src: *const wchar_t, n: size_t) -> *mut wchar_t;
    pub fn wcscasecmp(s1: *const wchar_t, s2: *const wchar_t) -> c_int;
    pub fn wcscasecmp_l(s1: *const wchar_t, s2: *const wchar_t, locale: locale_t) -> c_int;
    pub fn wcscat(dst: *mut wchar_t, src: *const wchar_t) -> *mut wchar_t;
    pub fn wcscoll(s1: *const wchar_t, s2: *const wchar_t) -> c_int;
    pub fn wcscoll_l(s1: *const wchar_t, s2: *const wchar_t, locale: locale_t) -> c_int;
    pub fn wcscpy(dst: *mut wchar_t, src: *const wchar_t) -> *mut wchar_t;
    pub fn wcscspn(ws: *const wchar_t, reject: *const wchar_t) -> size_t;
    pub fn wcsdup(src: *const wchar_t) -> *mut wchar_t;
    pub fn wcsftime(
        s: *mut wchar_t,
        maxsize: size_t,
        format: *const wchar_t,
        tm: *const tm,
    ) -> size_t;
    pub fn wcslcat(dst: *mut wchar_t, src: *const wchar_t, dstsize: size_t) -> size_t;
    pub fn wcslcpy(dst: *mut wchar_t, src: *const wchar_t, dstsize: size_t) -> size_t;
    pub fn wcsncasecmp(s1: *const wchar_t, s2: *const wchar_t, n: size_t) -> c_int;
    pub fn wcsncasecmp_l(
        s1: *const wchar_t,
        s2: *const wchar_t,
        n: size_t,
        locale: locale_t,
    ) -> c_int;
    pub fn wcsncat(dst: *mut wchar_t, src: *const wchar_t, n: size_t) -> *mut wchar_t;
    pub fn wcsncmp(s1: *const wchar_t, s2: *const wchar_t, n: size_t) -> c_int;
    pub fn wcsncpy(dst: *mut wchar_t, src: *const wchar_t, n: size_t) -> *mut wchar_t;
    pub fn wcsnlen(ws: *const wchar_t, maxlen: size_t) -> size_t;
    pub fn wcspbrk(ws: *const wchar_t, accept: *const wchar_t) -> *mut wchar_t;
    pub fn wcsspn(ws: *const wchar_t, accept: *const wchar_t) -> size_t;
    pub fn wcsstr(haystack: *const wchar_t, needle: *const wchar_t) -> *mut wchar_t;
    pub fn wcstok(
        ws: *mut wchar_t,
        delim: *const wchar_t,
        saveptr: *mut *mut wchar_t,
    ) -> *mut wchar_t;
    pub fn wcswidth(ws: *const wchar_t, n: size_t) -> c_int;
    pub fn wcsxfrm(dst: *mut wchar_t, src: *const wchar_t, n: size_t) -> size_t;
    pub fn wcsxfrm_l(dst: *mut wchar_t, src: *const wchar_t, n: size_t, locale: locale_t)
        -> size_t;
    pub fn wcwidth(wc: wchar_t) -> c_int;
    pub fn wmemmove(dst: *mut wchar_t, src: *const wchar_t, n: size_t) -> *mut wchar_t;
    pub fn wcstod(nptr: *const wchar_t, endptr: *mut *mut wchar_t) -> c_double;
    pub fn wcstof(nptr: *const wchar_t, endptr: *mut *mut wchar_t) -> c_float;
    pub fn wcstol(nptr: *const wchar_t, endptr: *mut *mut wchar_t, base: c_int) -> c_long;
    pub fn wcstoll(nptr: *const wchar_t, endptr: *mut *mut wchar_t, base: c_int) -> c_longlong;
    pub fn wcstoul(nptr: *const wchar_t, endptr: *mut *mut wchar_t, base: c_int) -> c_ulong;
    pub fn wcstoull(nptr: *const wchar_t, endptr: *mut *mut wchar_t, base: c_int) -> c_ulonglong;
    pub fn fgetwc(stream: *mut FILE) -> wint_t;
    pub fn fgetws(ws: *mut wchar_t, n: c_int, stream: *mut FILE) -> *mut wchar_t;
    pub fn fputwc(wc: wchar_t, stream: *mut FILE) -> wint_t;
    pub fn fputws(ws: *const wchar_t, stream: *mut FILE) -> c_int;
    pub fn fwide(stream: *mut FILE, mode: c_int) -> c_int;
    pub fn getwc(stream: *mut FILE) -> wint_t;
    pub fn getwchar() -> wint_t;
    pub fn putwc(wc: wchar_t, stream: *mut FILE) -> wint_t;
    pub fn putwchar(wc: wchar_t) -> wint_t;
    pub fn ungetwc(wc: wint_t, stream: *mut FILE) -> wint_t;
    pub fn open_wmemstream(bufp: *mut *mut wchar_t, sizep: *mut size_t) -> *mut FILE;
    pub fn fwprintf(stream: *mut FILE, format: *const wchar_t, ...) -> c_int;
    pub fn swprintf(s: *mut wchar_t, n: size_t, format: *const wchar_t, ...) -> c_int;
    pub fn wprintf(format: *const wchar_t, ...) -> c_int;
    pub fn fwscanf(stream: *mut FILE, format: *const wchar_t, ...) -> c_int;
    pub fn swscanf(s: *const wchar_t, format: *const wchar_t, ...) -> c_int;
    pub fn wscanf(format: *const wchar_t, ...) -> c_int;

    // --- <wctype.h> ---
    pub fn iswalnum(wc: wint_t) -> c_int;
    pub fn iswalpha(wc: wint_t) -> c_int;
    pub fn iswblank(wc: wint_t) -> c_int;
    pub fn iswcntrl(wc: wint_t) -> c_int;
    pub fn iswdigit(wc: wint_t) -> c_int;
    pub fn iswgraph(wc: wint_t) -> c_int;
    pub fn iswlower(wc: wint_t) -> c_int;
    pub fn iswprint(wc: wint_t) -> c_int;
    pub fn iswpunct(wc: wint_t) -> c_int;
    pub fn iswspace(wc: wint_t) -> c_int;
    pub fn iswupper(wc: wint_t) -> c_int;
    pub fn iswxdigit(wc: wint_t) -> c_int;
    pub fn iswctype(wc: wint_t, desc: wctype_t) -> c_int;
    pub fn towlower(wc: wint_t) -> wint_t;
    pub fn towupper(wc: wint_t) -> wint_t;
    pub fn towctrans(wc: wint_t, desc: wctrans_t) -> wint_t;
    pub fn wctype(property: *const c_char) -> wctype_t;
    pub fn wctrans(property: *const c_char) -> wctrans_t;
    pub fn iswalnum_l(wc: wint_t, locale: locale_t) -> c_int;
    pub fn iswalpha_l(wc: wint_t, locale: locale_t) -> c_int;
    pub fn iswblank_l(wc: wint_t, locale: locale_t) -> c_int;
    pub fn iswcntrl_l(wc: wint_t, locale: locale_t) -> c_int;
    pub fn iswdigit_l(wc: wint_t, locale: locale_t) -> c_int;
    pub fn iswgraph_l(wc: wint_t, locale: locale_t) -> c_int;
    pub fn iswlower_l(wc: wint_t, locale: locale_t) -> c_int;
    pub fn iswprint_l(wc: wint_t, locale: locale_t) -> c_int;
    pub fn iswpunct_l(wc: wint_t, locale: locale_t) -> c_int;
    pub fn iswspace_l(wc: wint_t, locale: locale_t) -> c_int;
    pub fn iswupper_l(wc: wint_t, locale: locale_t) -> c_int;
    pub fn iswxdigit_l(wc: wint_t, locale: locale_t) -> c_int;
    pub fn iswctype_l(wc: wint_t, desc: wctype_t, locale: locale_t) -> c_int;
    pub fn towlower_l(wc: wint_t, locale: locale_t) -> wint_t;
    pub fn towupper_l(wc: wint_t, locale: locale_t) -> wint_t;
    pub fn towctrans_l(wc: wint_t, desc: wctrans_t, locale: locale_t) -> wint_t;
    pub fn wctype_l(property: *const c_char, locale: locale_t) -> wctype_t;
    pub fn wctrans_l(property: *const c_char, locale: locale_t) -> wctrans_t;
    pub fn nextwctype(wc: wint_t, desc: wctype_t) -> wint_t;

    // --- <uchar.h> ---
    pub fn mbrtoc16(pc16: *mut char16_t, s: *const c_char, n: size_t, ps: *mut mbstate_t)
        -> size_t;
    pub fn c16rtomb(s: *mut c_char, c16: char16_t, ps: *mut mbstate_t) -> size_t;
    pub fn mbrtoc32(pc32: *mut char32_t, s: *const c_char, n: size_t, ps: *mut mbstate_t)
        -> size_t;
    pub fn c32rtomb(s: *mut c_char, c32: char32_t, ps: *mut mbstate_t) -> size_t;

    // --- <monetary.h> ---
    pub fn strfmon(s: *mut c_char, maxsize: size_t, format: *const c_char, ...) -> ssize_t;
    pub fn strfmon_l(
        s: *mut c_char,
        maxsize: size_t,
        locale: locale_t,
        format: *const c_char,
        ...
    ) -> ssize_t;

    // --- <libintl.h> ---
    pub fn gettext(msgid: *const c_char) -> *mut c_char;
    pub fn dgettext(domainname: *const c_char, msgid: *const c_char) -> *mut c_char;
    pub fn dcgettext(
        domainname: *const c_char,
        msgid: *const c_char,
        category: c_int,
    ) -> *mut c_char;
    pub fn ngettext(msgid1: *const c_char, msgid2: *const c_char, n: c_ulong) -> *mut c_char;
    pub fn dngettext(
        domainname: *const c_char,
        msgid1: *const c_char,
        msgid2: *const c_char,
        n: c_ulong,
    ) -> *mut c_char;
    pub fn dcngettext(
        domainname: *const c_char,
        msgid1: *const c_char,
        msgid2: *const c_char,
        n: c_ulong,
        category: c_int,
    ) -> *mut c_char;
    pub fn textdomain(domainname: *const c_char) -> *mut c_char;
    pub fn bindtextdomain(domainname: *const c_char, dirname: *const c_char) -> *mut c_char;
    pub fn bind_textdomain_codeset(
        domainname: *const c_char,
        codeset: *const c_char,
    ) -> *mut c_char;

    // --- <iconv.h> ---
    pub fn iconv_open(tocode: *const c_char, fromcode: *const c_char) -> iconv_t;
    pub fn iconv(
        cd: iconv_t,
        inbuf: *mut *mut c_char,
        inbytesleft: *mut size_t,
        outbuf: *mut *mut c_char,
        outbytesleft: *mut size_t,
    ) -> size_t;
    pub fn iconv_close(cd: iconv_t) -> c_int;

    // --- <math.h> ---
    pub fn acos(x: c_double) -> c_double;
    pub fn acosf(x: c_float) -> c_float;
    pub fn asin(x: c_double) -> c_double;
    pub fn asinf(x: c_float) -> c_float;
    pub fn atan(x: c_double) -> c_double;
    pub fn atanf(x: c_float) -> c_float;
    pub fn atan2(y: c_double, x: c_double) -> c_double;
    pub fn atan2f(y: c_float, x: c_float) -> c_float;
    pub fn cos(x: c_double) -> c_double;
    pub fn cosf(x: c_float) -> c_float;
    pub fn sin(x: c_double) -> c_double;
    pub fn sinf(x: c_float) -> c_float;
    pub fn tan(x: c_double) -> c_double;
    pub fn tanf(x: c_float) -> c_float;
    pub fn acosh(x: c_double) -> c_double;
    pub fn acoshf(x: c_float) -> c_float;
    pub fn asinh(x: c_double) -> c_double;
    pub fn asinhf(x: c_float) -> c_float;
    pub fn atanh(x: c_double) -> c_double;
    pub fn atanhf(x: c_float) -> c_float;
    pub fn cosh(x: c_double) -> c_double;
    pub fn coshf(x: c_float) -> c_float;
    pub fn sinh(x: c_double) -> c_double;
    pub fn sinhf(x: c_float) -> c_float;
    pub fn tanh(x: c_double) -> c_double;
    pub fn tanhf(x: c_float) -> c_float;
    pub fn exp(x: c_double) -> c_double;
    pub fn expf(x: c_float) -> c_float;
    pub fn exp2(x: c_double) -> c_double;
    pub fn exp2f(x: c_float) -> c_float;
    pub fn expm1(x: c_double) -> c_double;
    pub fn expm1f(x: c_float) -> c_float;
    pub fn frexp(x: c_double, exp: *mut c_int) -> c_double;
    pub fn frexpf(x: c_float, exp: *mut c_int) -> c_float;
    pub fn ilogb(x: c_double) -> c_int;
    pub fn ilogbf(x: c_float) -> c_int;
    pub fn ldexp(x: c_double, exp: c_int) -> c_double;
    pub fn ldexpf(x: c_float, exp: c_int) -> c_float;
    pub fn log(x: c_double) -> c_double;
    pub fn logf(x: c_float) -> c_float;
    pub fn log10(x: c_double) -> c_double;
    pub fn log10f(x: c_float) -> c_float;
    pub fn log1p(x: c_double) -> c_double;
    pub fn log1pf(x: c_float) -> c_float;
    pub fn log2(x: c_double) -> c_double;
    pub fn log2f(x: c_float) -> c_float;
    pub fn logb(x: c_double) -> c_double;
    pub fn logbf(x: c_float) -> c_float;
    pub fn modf(x: c_double, iptr: *mut c_double) -> c_double;
    pub fn modff(x: c_float, iptr: *mut c_float) -> c_float;
    pub fn scalbn(x: c_double, n: c_int) -> c_double;
    pub fn scalbnf(x: c_float, n: c_int) -> c_float;
    pub fn scalbln(x: c_double, n: c_long) -> c_double;
    pub fn scalblnf(x: c_float, n: c_long) -> c_float;
    pub fn cbrt(x: c_double) -> c_double;
    pub fn cbrtf(x: c_float) -> c_float;
    pub fn fabs(x: c_double) -> c_double;
    pub fn fabsf(x: c_float) -> c_float;
    pub fn hypot(x: c_double, y: c_double) -> c_double;
    pub fn hypotf(x: c_float, y: c_float) -> c_float;
    pub fn pow(x: c_double, y: c_double) -> c_double;
    pub fn powf(x: c_float, y: c_float) -> c_float;
    pub fn sqrt(x: c_double) -> c_double;
    pub fn sqrtf(x: c_float) -> c_float;
    pub fn erf(x: c_double) -> c_double;
    pub fn erff(x: c_float) -> c_float;
    pub fn erfc(x: c_double) -> c_double;
    pub fn erfcf(x: c_float) -> c_float;
    pub fn lgamma(x: c_double) -> c_double;
    pub fn lgammaf(x: c_float) -> c_float;
    pub fn tgamma(x: c_double) -> c_double;
    pub fn tgammaf(x: c_float) -> c_float;
    pub fn lgamma_r(x: c_double, signp: *mut c_int) -> c_double;
    pub fn lgammaf_r(x: c_float, signp: *mut c_int) -> c_float;
    pub fn ceil(x: c_double) -> c_double;
    pub fn ceilf(x: c_float) -> c_float;
    pub fn floor(x: c_double) -> c_double;
    pub fn floorf(x: c_float) -> c_float;
    pub fn nearbyint(x: c_double) -> c_double;
    pub fn nearbyintf(x: c_float) -> c_float;
    pub fn rint(x: c_double) -> c_double;
    pub fn rintf(x: c_float) -> c_float;
    pub fn lrint(x: c_double) -> c_long;
    pub fn lrintf(x: c_float) -> c_long;
    pub fn llrint(x: c_double) -> c_longlong;
    pub fn llrintf(x: c_float) -> c_longlong;
    pub fn round(x: c_double) -> c_double;
    pub fn roundf(x: c_float) -> c_float;
    pub fn lround(x: c_double) -> c_long;
    pub fn lroundf(x: c_float) -> c_long;
    pub fn llround(x: c_double) -> c_longlong;
    pub fn llroundf(x: c_float) -> c_longlong;
    pub fn trunc(x: c_double) -> c_double;
    pub fn truncf(x: c_float) -> c_float;
    pub fn fmod(x: c_double, y: c_double) -> c_double;
    pub fn fmodf(x: c_float, y: c_float) -> c_float;
    pub fn remainder(x: c_double, y: c_double) -> c_double;
    pub fn remainderf(x: c_float, y: c_float) -> c_float;
    pub fn remquo(x: c_double, y: c_double, quo: *mut c_int) -> c_double;
    pub fn remquof(x: c_float, y: c_float, quo: *mut c_int) -> c_float;
    pub fn copysign(x: c_double, y: c_double) -> c_double;
    pub fn copysignf(x: c_float, y: c_float) -> c_float;
    pub fn nan(tagp: *const c_char) -> c_double;
    pub fn nanf(tagp: *const c_char) -> c_float;
    pub fn nextafter(x: c_double, y: c_double) -> c_double;
    pub fn nextafterf(x: c_float, y: c_float) -> c_float;
    pub fn fdim(x: c_double, y: c_double) -> c_double;
    pub fn fdimf(x: c_float, y: c_float) -> c_float;
    pub fn fmax(x: c_double, y: c_double) -> c_double;
    pub fn fmaxf(x: c_float, y: c_float) -> c_float;
    pub fn fmin(x: c_double, y: c_double) -> c_double;
    pub fn fminf(x: c_float, y: c_float) -> c_float;
    pub fn fma(x: c_double, y: c_double, z: c_double) -> c_double;
    pub fn fmaf(x: c_float, y: c_float, z: c_float) -> c_float;
    pub fn j0(x: c_double) -> c_double;
    pub fn j1(x: c_double) -> c_double;
    pub fn jn(n: c_int, x: c_double) -> c_double;
    pub fn y0(x: c_double) -> c_double;
    pub fn y1(x: c_double) -> c_double;
    pub fn yn(n: c_int, x: c_double) -> c_double;

    // --- <complex.h> ---
    pub fn cacos(z: c_double_complex) -> c_double_complex;
    pub fn cacosf(z: c_float_complex) -> c_float_complex;
    pub fn casin(z: c_double_complex) -> c_double_complex;
    pub fn casinf(z: c_float_complex) -> c_float_complex;
    pub fn catan(z: c_double_complex) -> c_double_complex;
    pub fn catanf(z: c_float_complex) -> c_float_complex;
    pub fn ccos(z: c_double_complex) -> c_double_complex;
    pub fn ccosf(z: c_float_complex) -> c_float_complex;
    pub fn csin(z: c_double_complex) -> c_double_complex;
    pub fn csinf(z: c_float_complex) -> c_float_complex;
    pub fn ctan(z: c_double_complex) -> c_double_complex;
    pub fn ctanf(z: c_float_complex) -> c_float_complex;
    pub fn cacosh(z: c_double_complex) -> c_double_complex;
    pub fn cacoshf(z: c_float_complex) -> c_float_complex;
    pub fn casinh(z: c_double_complex) -> c_double_complex;
    pub fn casinhf(z: c_float_complex) -> c_float_complex;
    pub fn catanh(z: c_double_complex) -> c_double_complex;
    pub fn catanhf(z: c_float_complex) -> c_float_complex;
    pub fn ccosh(z: c_double_complex) -> c_double_complex;
    pub fn ccoshf(z: c_float_complex) -> c_float_complex;
    pub fn csinh(z: c_double_complex) -> c_double_complex;
    pub fn csinhf(z: c_float_complex) -> c_float_complex;
    pub fn ctanh(z: c_double_complex) -> c_double_complex;
    pub fn ctanhf(z: c_float_complex) -> c_float_complex;
    pub fn cexp(z: c_double_complex) -> c_double_complex;
    pub fn cexpf(z: c_float_complex) -> c_float_complex;
    pub fn clog(z: c_double_complex) -> c_double_complex;
    pub fn clogf(z: c_float_complex) -> c_float_complex;
    pub fn cabs(z: c_double_complex) -> c_double;
    pub fn cabsf(z: c_float_complex) -> c_float;
    pub fn cpow(x: c_double_complex, y: c_double_complex) -> c_double_complex;
    pub fn cpowf(x: c_float_complex, y: c_float_complex) -> c_float_complex;
    pub fn csqrt(z: c_double_complex) -> c_double_complex;
    pub fn csqrtf(z: c_float_complex) -> c_float_complex;
    pub fn carg(z: c_double_complex) -> c_double;
    pub fn cargf(z: c_float_complex) -> c_float;
    pub fn cimag(z: c_double_complex) -> c_double;
    pub fn cimagf(z: c_float_complex) -> c_float;
    pub fn conj(z: c_double_complex) -> c_double_complex;
    pub fn conjf(z: c_float_complex) -> c_float_complex;
    pub fn cproj(z: c_double_complex) -> c_double_complex;
    pub fn cprojf(z: c_float_complex) -> c_float_complex;
    pub fn creal(z: c_double_complex) -> c_double;
    pub fn crealf(z: c_float_complex) -> c_float;

    // --- <fenv.h> ---
    pub fn feclearexcept(excepts: c_int) -> c_int;
    pub fn fegetexceptflag(flagp: *mut fexcept_t, excepts: c_int) -> c_int;
    pub fn feraiseexcept(excepts: c_int) -> c_int;
    pub fn fesetexceptflag(flagp: *const fexcept_t, excepts: c_int) -> c_int;
    pub fn fetestexcept(excepts: c_int) -> c_int;
    pub fn fegetround() -> c_int;
    pub fn fesetround(round: c_int) -> c_int;
    pub fn fegetenv(envp: *mut fenv_t) -> c_int;
    pub fn feholdexcept(envp: *mut fenv_t) -> c_int;
    pub fn fesetenv(envp: *const fenv_t) -> c_int;
    pub fn feupdateenv(envp: *const fenv_t) -> c_int;

    // --- Variables ---
    pub static mut optarg: *mut c_char;
    pub static mut optind: c_int;
    pub static mut opterr: c_int;
    pub static mut optopt: c_int;
    pub static mut getdate_err: c_int;
    pub static mut tzname: [*mut c_char; 2];
    pub static mut timezone: c_long;
    pub static mut daylight: c_int;
    pub static mut h_errno: c_int;
    pub static mut signgam: c_int;
    pub static in6addr_any: in6_addr;
    pub static in6addr_loopback: in6_addr;
    pub static sys_signame: [*const c_char; NSIG as usize];
    pub static sys_siglist: [*const c_char; NSIG as usize];

    // --- Standard streams ---
    pub static stdin: *mut FILE;
    pub static stdout: *mut FILE;
    pub static stderr: *mut FILE;
    pub static mut environ: *mut *mut c_char;
}

// <sys/socket.h> ancillary-data macros as functions; alignment is to
// sizeof(long).
#[inline]
pub const fn CMSG_ALIGN(len: size_t) -> size_t {
    (len + core::mem::size_of::<c_long>() - 1) & !(core::mem::size_of::<c_long>() - 1)
}

#[inline]
pub unsafe fn CMSG_FIRSTHDR(mhdr: *const msghdr) -> *mut cmsghdr {
    unsafe {
        if (*mhdr).msg_controllen as size_t >= core::mem::size_of::<cmsghdr>() {
            (*mhdr).msg_control as *mut cmsghdr
        } else {
            core::ptr::null_mut()
        }
    }
}

// The next header, or null when it would not fit in the control buffer.
#[inline]
pub unsafe fn CMSG_NXTHDR(mhdr: *const msghdr, cmsg: *const cmsghdr) -> *mut cmsghdr {
    unsafe {
        let len = (*cmsg).cmsg_len as size_t;
        if len < core::mem::size_of::<cmsghdr>() {
            return core::ptr::null_mut();
        }
        let next = (cmsg as *const u8).wrapping_add(CMSG_ALIGN(len));
        let used = (next as usize)
            .wrapping_add(core::mem::size_of::<cmsghdr>())
            .wrapping_sub((*mhdr).msg_control as usize);
        if used > (*mhdr).msg_controllen as size_t {
            core::ptr::null_mut()
        } else {
            next as *mut cmsghdr
        }
    }
}

#[inline]
pub unsafe fn CMSG_DATA(cmsg: *const cmsghdr) -> *mut c_uchar {
    unsafe { (cmsg as *mut c_uchar).add(CMSG_ALIGN(core::mem::size_of::<cmsghdr>())) }
}

#[inline]
pub const fn CMSG_SPACE(length: size_t) -> size_t {
    CMSG_ALIGN(core::mem::size_of::<cmsghdr>()) + CMSG_ALIGN(length)
}

#[inline]
pub const fn CMSG_LEN(length: size_t) -> size_t {
    CMSG_ALIGN(core::mem::size_of::<cmsghdr>()) + length
}

// <sys/un.h>: the address length of a sockaddr_un holding a NUL-terminated
// path.
#[inline]
pub unsafe fn SUN_LEN(su: *const sockaddr_un) -> size_t {
    unsafe { core::mem::size_of::<sa_family_t>() + strlen((*su).sun_path.as_ptr()) }
}

// <net/if_dl.h>: the hardware address, which follows the name in sdl_data.
#[inline]
pub unsafe fn LLADDR(s: *const sockaddr_dl) -> *mut c_char {
    unsafe { ((*s).sdl_data.as_ptr() as *mut c_char).add((*s).sdl_nlen as usize) }
}

// <net/route.h>: the padded size of a socket address inside a routing
// message, from its length byte.
#[inline]
pub unsafe fn SA_SIZE(sa: *const c_void) -> size_t {
    unsafe {
        let len = *(sa as *const c_uchar) as size_t;
        if len == 0 {
            core::mem::size_of::<c_long>()
        } else {
            1 + ((len - 1) | (core::mem::size_of::<c_long>() - 1))
        }
    }
}

// <sys/sysmacros.h>: the 64-bit device number. The major number's low 12
// bits are bits 19:8 and its high 20 bits bits 63:44; the minor number's low
// 8 bits are bits 7:0 and its high 24 bits bits 43:20 (gnu_dev_major,
// gnu_dev_minor and gnu_dev_makedev compute the same).
#[inline]
pub const fn major(dev: dev_t) -> c_uint {
    (((dev >> 32) & 0xfffff000) | ((dev >> 8) & 0xfff)) as c_uint
}

#[inline]
pub const fn minor(dev: dev_t) -> c_uint {
    (((dev >> 12) & 0xffffff00) | (dev & 0xff)) as c_uint
}

#[inline]
pub const fn makedev(major: c_uint, minor: c_uint) -> dev_t {
    let major = major as dev_t;
    let minor = minor as dev_t;
    ((major & 0xfffff000) << 32)
        | ((major & 0xfff) << 8)
        | ((minor & 0xffffff00) << 12)
        | (minor & 0xff)
}

// <linux/netlink.h> alignment and message macros as functions.
#[inline]
pub const fn NLMSG_ALIGN(len: u32) -> u32 {
    (len + NLMSG_ALIGNTO - 1) & !(NLMSG_ALIGNTO - 1)
}

pub const NLMSG_HDRLEN: c_int = NLMSG_ALIGN(core::mem::size_of::<nlmsghdr>() as u32) as c_int;

#[inline]
pub const fn NLMSG_LENGTH(len: u32) -> u32 {
    len + NLMSG_HDRLEN as u32
}

#[inline]
pub const fn NLMSG_SPACE(len: u32) -> u32 {
    NLMSG_ALIGN(NLMSG_LENGTH(len))
}

#[inline]
pub unsafe fn NLMSG_DATA(nlh: *const nlmsghdr) -> *mut c_void {
    unsafe { (nlh as *mut c_char).add(NLMSG_HDRLEN as usize) as *mut c_void }
}

#[inline]
pub unsafe fn NLMSG_NEXT(nlh: *const nlmsghdr, len: &mut c_int) -> *mut nlmsghdr {
    unsafe {
        let step = NLMSG_ALIGN((*nlh).nlmsg_len);
        *len -= step as c_int;
        (nlh as *mut c_char).add(step as usize) as *mut nlmsghdr
    }
}

#[inline]
pub unsafe fn NLMSG_OK(nlh: *const nlmsghdr, len: c_int) -> bool {
    unsafe {
        len >= core::mem::size_of::<nlmsghdr>() as c_int
            && (*nlh).nlmsg_len >= core::mem::size_of::<nlmsghdr>() as u32
            && (*nlh).nlmsg_len <= len as c_uint
    }
}

#[inline]
pub unsafe fn NLMSG_PAYLOAD(nlh: *const nlmsghdr, len: u32) -> u32 {
    unsafe { (*nlh).nlmsg_len - NLMSG_SPACE(len) }
}

#[inline]
pub const fn NLA_ALIGN(len: c_int) -> c_int {
    (len + NLA_ALIGNTO - 1) & !(NLA_ALIGNTO - 1)
}

pub const NLA_HDRLEN: c_int = NLA_ALIGN(core::mem::size_of::<nlattr>() as c_int);

// FD_SET/FD_CLR/FD_ISSET/FD_ZERO helper functions
#[inline]
pub unsafe fn FD_SET(fd: c_int, set: *mut fd_set) {
    unsafe {
        let fd = fd as usize;
        (*set).fds_bits[fd / 64] |= 1u64 << (fd % 64);
    }
}

#[inline]
pub unsafe fn FD_CLR(fd: c_int, set: *mut fd_set) {
    unsafe {
        let fd = fd as usize;
        (*set).fds_bits[fd / 64] &= !(1u64 << (fd % 64));
    }
}

#[inline]
pub unsafe fn FD_ISSET(fd: c_int, set: *const fd_set) -> bool {
    unsafe {
        let fd = fd as usize;
        ((*set).fds_bits[fd / 64] & (1u64 << (fd % 64))) != 0
    }
}

#[inline]
pub unsafe fn FD_ZERO(set: *mut fd_set) {
    unsafe {
        let mut i = 0;
        while i < FD_SETSIZE / 64 {
            (*set).fds_bits[i] = 0;
            i += 1;
        }
    }
}

/// Route netlink (<linux/rtnetlink.h>): the NETLINK_ROUTE message types, the
/// fixed message headers, the attribute header and macros, and the IFLA_*,
/// IFA_* and RTA_* attribute types. They are kept in this module because
/// RTM_NEWADDR, RTM_DELADDR and RTA_GATEWAY name other values in the
/// routing-socket interface of <net/route.h>, which the crate root carries.
pub mod rtnetlink {
    use crate::nlmsghdr;
    use crate::prelude::*;
    use crate::NLMSG_ALIGN;

    pub const RTM_NEWLINK: c_int = 16;
    pub const RTM_DELLINK: c_int = 17;
    pub const RTM_GETLINK: c_int = 18;
    pub const RTM_SETLINK: c_int = 19;
    pub const RTM_NEWADDR: c_int = 20;
    pub const RTM_DELADDR: c_int = 21;
    pub const RTM_GETADDR: c_int = 22;
    pub const RTM_NEWROUTE: c_int = 24;
    pub const RTM_DELROUTE: c_int = 25;
    pub const RTM_GETROUTE: c_int = 26;

    pub const RTMGRP_LINK: c_uint = 0x1;
    pub const RTMGRP_IPV4_IFADDR: c_uint = 0x10;
    pub const RTMGRP_IPV4_ROUTE: c_uint = 0x40;
    pub const RTMGRP_IPV6_IFADDR: c_uint = 0x100;
    pub const RTMGRP_IPV6_ROUTE: c_uint = 0x400;

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct rtattr {
        pub rta_len: c_ushort,
        pub rta_type: c_ushort,
    }

    pub const RTA_ALIGNTO: c_uint = 4;

    #[inline]
    pub const fn RTA_ALIGN(len: c_uint) -> c_uint {
        (len + RTA_ALIGNTO - 1) & !(RTA_ALIGNTO - 1)
    }

    #[inline]
    pub const fn RTA_LENGTH(len: c_uint) -> c_uint {
        RTA_ALIGN(core::mem::size_of::<rtattr>() as c_uint) + len
    }

    #[inline]
    pub const fn RTA_SPACE(len: c_uint) -> c_uint {
        RTA_ALIGN(RTA_LENGTH(len))
    }

    #[inline]
    pub unsafe fn RTA_OK(rta: *const rtattr, len: c_int) -> bool {
        unsafe {
            len >= core::mem::size_of::<rtattr>() as c_int
                && (*rta).rta_len as usize >= core::mem::size_of::<rtattr>()
                && (*rta).rta_len as c_int <= len
        }
    }

    #[inline]
    pub unsafe fn RTA_NEXT(rta: *const rtattr, attrlen: &mut c_int) -> *mut rtattr {
        unsafe {
            let step = RTA_ALIGN((*rta).rta_len as c_uint);
            *attrlen -= step as c_int;
            (rta as *mut c_char).add(step as usize) as *mut rtattr
        }
    }

    #[inline]
    pub unsafe fn RTA_DATA(rta: *const rtattr) -> *mut c_void {
        unsafe { (rta as *mut c_char).add(RTA_LENGTH(0) as usize) as *mut c_void }
    }

    #[inline]
    pub unsafe fn RTA_PAYLOAD(rta: *const rtattr) -> c_int {
        unsafe { (*rta).rta_len as c_int - RTA_LENGTH(0) as c_int }
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct rtmsg {
        pub rtm_family: c_uchar,
        pub rtm_dst_len: c_uchar,
        pub rtm_src_len: c_uchar,
        pub rtm_tos: c_uchar,
        pub rtm_table: c_uchar,
        pub rtm_protocol: c_uchar,
        pub rtm_scope: c_uchar,
        pub rtm_type: c_uchar,
        pub rtm_flags: c_uint,
    }

    pub const RTN_UNSPEC: c_uchar = 0;
    pub const RTN_UNICAST: c_uchar = 1;
    pub const RTN_LOCAL: c_uchar = 2;
    pub const RTN_BROADCAST: c_uchar = 3;
    pub const RTN_ANYCAST: c_uchar = 4;
    pub const RTN_MULTICAST: c_uchar = 5;
    pub const RTN_BLACKHOLE: c_uchar = 6;
    pub const RTN_UNREACHABLE: c_uchar = 7;
    pub const RTN_PROHIBIT: c_uchar = 8;

    pub const RTPROT_UNSPEC: c_uchar = 0;
    pub const RTPROT_REDIRECT: c_uchar = 1;
    pub const RTPROT_KERNEL: c_uchar = 2;
    pub const RTPROT_BOOT: c_uchar = 3;
    pub const RTPROT_STATIC: c_uchar = 4;

    pub const RT_SCOPE_UNIVERSE: c_uchar = 0;
    pub const RT_SCOPE_SITE: c_uchar = 200;
    pub const RT_SCOPE_LINK: c_uchar = 253;
    pub const RT_SCOPE_HOST: c_uchar = 254;
    pub const RT_SCOPE_NOWHERE: c_uchar = 255;

    pub const RT_TABLE_UNSPEC: c_uchar = 0;
    pub const RT_TABLE_DEFAULT: c_uchar = 253;
    pub const RT_TABLE_MAIN: c_uchar = 254;
    pub const RT_TABLE_LOCAL: c_uchar = 255;

    pub const RTA_UNSPEC: c_ushort = 0;
    pub const RTA_DST: c_ushort = 1;
    pub const RTA_SRC: c_ushort = 2;
    pub const RTA_IIF: c_ushort = 3;
    pub const RTA_OIF: c_ushort = 4;
    pub const RTA_GATEWAY: c_ushort = 5;
    pub const RTA_PRIORITY: c_ushort = 6;
    pub const RTA_PREFSRC: c_ushort = 7;
    pub const RTA_METRICS: c_ushort = 8;
    pub const RTA_TABLE: c_ushort = 15;

    #[inline]
    pub unsafe fn RTM_RTA(r: *const rtmsg) -> *mut rtattr {
        unsafe {
            (r as *mut c_char).add(NLMSG_ALIGN(core::mem::size_of::<rtmsg>() as u32) as usize)
                as *mut rtattr
        }
    }

    #[inline]
    pub unsafe fn RTM_PAYLOAD(n: *const nlmsghdr) -> u32 {
        unsafe { crate::NLMSG_PAYLOAD(n, core::mem::size_of::<rtmsg>() as u32) }
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct ifinfomsg {
        pub ifi_family: c_uchar,
        pub __ifi_pad: c_uchar,
        pub ifi_type: c_ushort,
        pub ifi_index: c_int,
        pub ifi_flags: c_uint,
        pub ifi_change: c_uint,
    }

    pub const IFLA_UNSPEC: c_ushort = 0;
    pub const IFLA_ADDRESS: c_ushort = 1;
    pub const IFLA_BROADCAST: c_ushort = 2;
    pub const IFLA_IFNAME: c_ushort = 3;
    pub const IFLA_MTU: c_ushort = 4;
    pub const IFLA_LINK: c_ushort = 5;
    pub const IFLA_STATS: c_ushort = 7;
    pub const IFLA_OPERSTATE: c_ushort = 16;
    pub const IFLA_STATS64: c_ushort = 23;

    #[inline]
    pub unsafe fn IFLA_RTA(r: *const ifinfomsg) -> *mut rtattr {
        unsafe {
            (r as *mut c_char).add(NLMSG_ALIGN(core::mem::size_of::<ifinfomsg>() as u32) as usize)
                as *mut rtattr
        }
    }

    #[inline]
    pub unsafe fn IFLA_PAYLOAD(n: *const nlmsghdr) -> u32 {
        unsafe { crate::NLMSG_PAYLOAD(n, core::mem::size_of::<ifinfomsg>() as u32) }
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct ifaddrmsg {
        pub ifa_family: c_uchar,
        pub ifa_prefixlen: c_uchar,
        pub ifa_flags: c_uchar,
        pub ifa_scope: c_uchar,
        pub ifa_index: c_uint,
    }

    pub const IFA_UNSPEC: c_ushort = 0;
    pub const IFA_ADDRESS: c_ushort = 1;
    pub const IFA_LOCAL: c_ushort = 2;
    pub const IFA_LABEL: c_ushort = 3;
    pub const IFA_BROADCAST: c_ushort = 4;
    pub const IFA_ANYCAST: c_ushort = 5;
    pub const IFA_CACHEINFO: c_ushort = 6;
    pub const IFA_FLAGS: c_ushort = 8;

    pub const IFA_F_SECONDARY: c_uchar = 0x01;
    pub const IFA_F_DADFAILED: c_uchar = 0x08;
    pub const IFA_F_TENTATIVE: c_uchar = 0x40;
    pub const IFA_F_PERMANENT: c_uchar = 0x80;

    #[inline]
    pub unsafe fn IFA_RTA(r: *const ifaddrmsg) -> *mut rtattr {
        unsafe {
            (r as *mut c_char).add(NLMSG_ALIGN(core::mem::size_of::<ifaddrmsg>() as u32) as usize)
                as *mut rtattr
        }
    }

    #[inline]
    pub unsafe fn IFA_PAYLOAD(n: *const nlmsghdr) -> u32 {
        unsafe { crate::NLMSG_PAYLOAD(n, core::mem::size_of::<ifaddrmsg>() as u32) }
    }
}
