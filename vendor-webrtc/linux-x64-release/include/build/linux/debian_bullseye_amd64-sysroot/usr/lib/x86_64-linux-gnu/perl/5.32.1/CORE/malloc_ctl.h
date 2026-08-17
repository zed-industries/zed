#ifndef PERL_MALLOC_CTL_H_
#  define PERL_MALLOC_CTL_H_

struct perl_mstats {
    UV *nfree;
    UV *ntotal;
    IV topbucket, topbucket_ev, topbucket_odd, totfree, total, total_chain;
    IV total_sbrk, sbrks, sbrk_good, sbrk_slack, start_slack, sbrked_remains;
    IV minbucket;
    /* Level 1 info */
    UV *bucket_mem_size;
    UV *bucket_available_size;
    UV nbuckets;
};
typedef struct perl_mstats perl_mstats_t;

PERL_CALLCONV Malloc_t Perl_malloc (MEM_SIZE nbytes);
PERL_CALLCONV Malloc_t Perl_calloc (MEM_SIZE elements, MEM_SIZE size);
PERL_CALLCONV Malloc_t Perl_realloc (Malloc_t where, MEM_SIZE nbytes);
/* 'mfree' rather than 'free', since there is already a 'perl_free'
 * that causes clashes with case-insensitive linkers */
PERL_CALLCONV Free_t   Perl_mfree (Malloc_t where);

#ifndef NO_MALLOC_DYNAMIC_CFG

/* IV configuration data */
enum {
  MallocCfg_FIRST_SBRK,
  MallocCfg_MIN_SBRK,
  MallocCfg_MIN_SBRK_FRAC1000,
  MallocCfg_SBRK_ALLOW_FAILURES,
  MallocCfg_SBRK_FAILURE_PRICE,
  MallocCfg_sbrk_goodness,

  MallocCfg_filldead,
  MallocCfg_fillalive,
  MallocCfg_fillcheck,

  MallocCfg_skip_cfg_env,
  MallocCfg_cfg_env_read,

  MallocCfg_emergency_buffer_size,
  MallocCfg_emergency_buffer_last_req,

  MallocCfg_emergency_buffer_prepared_size,

  MallocCfg_last
};
/* char* configuration data */
enum {
  MallocCfgP_emergency_buffer,
  MallocCfgP_emergency_buffer_prepared,
  MallocCfgP_last
};
START_EXTERN_C
extern IV *MallocCfg_ptr;
extern char **MallocCfgP_ptr;
END_EXTERN_C

#endif

#endif
