#include <fcntl.h>
#include <sanitizer/linux_syscall_hooks.h>
#include <signal.h>
#include <sys/syscall.h>
#include <sys/types.h>
#include <unistd.h>

int myfork() {
  __sanitizer_syscall_pre_fork();
#ifdef SYS_fork
  int res = syscall(SYS_fork);
#else
  int res = syscall(SYS_clone, SIGCHLD, 0);
#endif
  __sanitizer_syscall_post_fork(res);
  return res;
}

int mypipe(int pipefd[2]) {
  __sanitizer_syscall_pre_pipe(pipefd);
  int res = syscall(SYS_pipe2, pipefd, 0);
  __sanitizer_syscall_post_pipe(res, pipefd);
  return res;
}

int myclose(int fd) {
  __sanitizer_syscall_pre_close(fd);
  int res = syscall(SYS_close, fd);
  __sanitizer_syscall_post_close(res, fd);
  return res;
}

ssize_t myread(int fd, void *buf, size_t count) {
  __sanitizer_syscall_pre_read(fd, buf, count);
  ssize_t res = syscall(SYS_read, fd, buf, count);
  __sanitizer_syscall_post_read(res, fd, buf, count);
  return res;
}

ssize_t mywrite(int fd, const void *buf, size_t count) {
  __sanitizer_syscall_pre_write(fd, buf, count);
  ssize_t res = syscall(SYS_write, fd, buf, count);
  __sanitizer_syscall_post_write(res, fd, buf, count);
  return res;
}
