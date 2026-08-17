#ifndef __TSAN_JAVA_H__
#define __TSAN_JAVA_H__

#include "test.h"

extern "C" {
typedef unsigned long jptr;
void __tsan_java_preinit(const char *libjvm_path);
void __tsan_java_init(jptr heap_begin, jptr heap_size);
int  __tsan_java_fini();
void __tsan_java_alloc(jptr ptr, jptr size);
void __tsan_java_free(jptr ptr, jptr size);
jptr __tsan_java_find(jptr *from_ptr, jptr to);
void __tsan_java_move(jptr src, jptr dst, jptr size);
void __tsan_java_finalize();
void __tsan_java_mutex_lock(jptr addr);
void __tsan_java_mutex_unlock(jptr addr);
void __tsan_java_mutex_read_lock(jptr addr);
void __tsan_java_mutex_read_unlock(jptr addr);
void __tsan_java_mutex_lock_rec(jptr addr, int rec);
int  __tsan_java_mutex_unlock_rec(jptr addr);
int  __tsan_java_acquire(jptr addr);
int  __tsan_java_release(jptr addr);
int  __tsan_java_release_store(jptr addr);

void __tsan_read1_pc(jptr addr, jptr pc);
void __tsan_write1_pc(jptr addr, jptr pc);
void __tsan_func_entry(jptr pc);
void __tsan_func_exit();
}

const jptr kExternalPCBit = 1ULL << 60;

#endif // __TSAN_JAVA_H__