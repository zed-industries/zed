
#ifndef _RADEON_CS_INT_H_
#define _RADEON_CS_INT_H_

struct radeon_cs_space_check {
    struct radeon_bo_int *bo;
    uint32_t read_domains;
    uint32_t write_domain;
    uint32_t new_accounted;
};

struct radeon_cs_int {
    /* keep first two in same place */
    uint32_t                    *packets;    
    unsigned                    cdw;
    unsigned                    ndw;
    unsigned                    section_ndw;
    unsigned                    section_cdw;
    /* private members */
    struct radeon_cs_manager    *csm;
    void                        *relocs;
    unsigned                    crelocs;
    unsigned                    relocs_total_size;
    const char                  *section_file;
    const char                  *section_func;
    int                         section_line;
    struct radeon_cs_space_check bos[MAX_SPACE_BOS];
    int                         bo_count;
    void                        (*space_flush_fn)(void *);
    void                        *space_flush_data;
    uint32_t                    id;
};

/* cs functions */
struct radeon_cs_funcs {
    struct radeon_cs_int *(*cs_create)(struct radeon_cs_manager *csm,
                                   uint32_t ndw);
    int (*cs_write_reloc)(struct radeon_cs_int *cs,
                          struct radeon_bo *bo,
                          uint32_t read_domain,
                          uint32_t write_domain,
                          uint32_t flags);
    int (*cs_begin)(struct radeon_cs_int *cs,
                    uint32_t ndw,
		    const char *file,
		    const char *func,
		    int line);
    int (*cs_end)(struct radeon_cs_int *cs,
		  const char *file, const char *func,
		  int line);


    int (*cs_emit)(struct radeon_cs_int *cs);
    int (*cs_destroy)(struct radeon_cs_int *cs);
    int (*cs_erase)(struct radeon_cs_int *cs);
    int (*cs_need_flush)(struct radeon_cs_int *cs);
    void (*cs_print)(struct radeon_cs_int *cs, FILE *file);
};

struct radeon_cs_manager {
    const struct radeon_cs_funcs  *funcs;
    int                     fd;
    int32_t vram_limit, gart_limit;
    int32_t vram_write_used, gart_write_used;
    int32_t read_used;
};
#endif
