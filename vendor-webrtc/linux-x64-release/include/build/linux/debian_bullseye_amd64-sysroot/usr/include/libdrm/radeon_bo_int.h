#ifndef RADEON_BO_INT
#define RADEON_BO_INT

struct radeon_bo_manager {
    const struct radeon_bo_funcs *funcs;
    int                     fd;
};

struct radeon_bo_int {
    void                        *ptr;
    uint32_t                    flags;
    uint32_t                    handle;
    uint32_t                    size;
    /* private members */
    uint32_t                    alignment;
    uint32_t                    domains;
    unsigned                    cref;
    struct radeon_bo_manager    *bom;
    uint32_t                    space_accounted;
    uint32_t                    referenced_in_cs;
};

/* bo functions */
struct radeon_bo_funcs {
    struct radeon_bo *(*bo_open)(struct radeon_bo_manager *bom,
                                 uint32_t handle,
                                 uint32_t size,
                                 uint32_t alignment,
                                 uint32_t domains,
                                 uint32_t flags);
    void (*bo_ref)(struct radeon_bo_int *bo);
    struct radeon_bo *(*bo_unref)(struct radeon_bo_int *bo);
    int (*bo_map)(struct radeon_bo_int *bo, int write);
    int (*bo_unmap)(struct radeon_bo_int *bo);
    int (*bo_wait)(struct radeon_bo_int *bo);
    int (*bo_is_static)(struct radeon_bo_int *bo);
    int (*bo_set_tiling)(struct radeon_bo_int *bo, uint32_t tiling_flags,
                         uint32_t pitch);
    int (*bo_get_tiling)(struct radeon_bo_int *bo, uint32_t *tiling_flags,
                         uint32_t *pitch);
    int (*bo_is_busy)(struct radeon_bo_int *bo, uint32_t *domain);
    int (*bo_is_referenced_by_cs)(struct radeon_bo_int *bo, struct radeon_cs *cs);
};

#endif
