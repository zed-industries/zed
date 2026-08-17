#ifndef NASM_IFLAG_H
#define NASM_IFLAG_H

#include "compiler.h"
#include "ilog2.h"


#include "iflaggen.h"

#define IF_GENBIT(bit)          (UINT32_C(1) << ((bit) & 31))

static inline int ifcomp(uint32_t a, uint32_t b)
{
    return (a > b) - (a < b);
}

static inline bool iflag_test(const iflag_t *f, unsigned int bit)
{
    return !!(f->field[bit >> 5] & IF_GENBIT(bit));
}

static inline void iflag_set(iflag_t *f, unsigned int bit)
{
    f->field[bit >> 5] |= IF_GENBIT(bit);
}

static inline void iflag_clear(iflag_t *f, unsigned int bit)
{
    f->field[bit >> 5] &= ~IF_GENBIT(bit);
}

static inline void iflag_clear_all(iflag_t *f)
{
     memset(f, 0, sizeof(*f));
}

static inline void iflag_set_all(iflag_t *f)
{
     memset(f, ~0, sizeof(*f));
}

#define iflag_for_each_field(v) for ((v) = 0; (v) < IF_FIELD_COUNT; (v)++)

static inline int iflag_cmp(const iflag_t *a, const iflag_t *b)
{
    int i;

    /* This is intentionally a reverse loop! */
    for (i = IF_FIELD_COUNT-1; i >= 0; i--) {
        if (a->field[i] == b->field[i])
            continue;

        return ifcomp(a->field[i], b->field[i]);
    }

    return 0;
}

#define IF_GEN_HELPER(name, op)                                         \
    static inline iflag_t iflag_##name(const iflag_t *a, const iflag_t *b) \
    {                                                                   \
        unsigned int i;                                                 \
        iflag_t res;                                                    \
                                                                        \
        iflag_for_each_field(i)                                         \
            res.field[i] = a->field[i] op b->field[i];                  \
                                                                        \
        return res;                                                     \
    }

IF_GEN_HELPER(xor, ^)

/* Some helpers which are to work with predefined masks */
#define IF_SMASK        (IFM_SB|IFM_SW|IFM_SD|IFM_SQ|IFM_SO|IFM_SY|IFM_SZ|IFM_SIZE|IFM_ANYSIZE)
#define IF_ARMASK       (IFM_AR0|IFM_AR1|IFM_AR2|IFM_AR3|IFM_AR4)

#define _itemp_smask(idx)      (insns_flags[(idx)].field[0] & IF_SMASK)
#define _itemp_armask(idx)     (insns_flags[(idx)].field[0] & IF_ARMASK)
#define _itemp_arg(idx)        ((_itemp_armask(idx) >> IF_AR0) - 1)

#define itemp_smask(itemp)      _itemp_smask((itemp)->iflag_idx)
#define itemp_arg(itemp)        _itemp_arg((itemp)->iflag_idx)
#define itemp_armask(itemp)     _itemp_armask((itemp)->iflag_idx)

/*
 * IF_ANY is the highest CPU level by definition
 */
#define IF_CPU_LEVEL_MASK      ((IFM_ANY << 1) - 1)

static inline int iflag_cmp_cpu(const iflag_t *a, const iflag_t *b)
{
    return ifcomp(a->field[IF_CPU_FIELD], b->field[IF_CPU_FIELD]);
}

static inline uint32_t _iflag_cpu_level(const iflag_t *a)
{
    return a->field[IF_CPU_FIELD] & IF_CPU_LEVEL_MASK;
}

static inline int iflag_cmp_cpu_level(const iflag_t *a, const iflag_t *b)
{
    return ifcomp(_iflag_cpu_level(a), _iflag_cpu_level(b));
}

/* Returns true if the CPU level is at least a certain value */
static inline bool iflag_cpu_level_ok(const iflag_t *a, unsigned int bit)
{
    return _iflag_cpu_level(a) >= IF_GENBIT(bit);
}

static inline void iflag_set_all_features(iflag_t *a)
{
    uint32_t *p = &a->field[IF_FEATURE_FIELD];

    memset(p, -1, IF_FEATURE_NFIELDS * sizeof(uint32_t));
}

static inline iflag_t _iflag_pfmask(const iflag_t *a)
{
    iflag_t r;

    iflag_clear_all(&r);

    if (iflag_test(a, IF_CYRIX))
        iflag_set(&r, IF_CYRIX);
    if (iflag_test(a, IF_AMD))
        iflag_set(&r, IF_AMD);

    return r;
}

#define iflag_pfmask(itemp)     _iflag_pfmask(&insns_flags[(itemp)->iflag_idx])

#endif /* NASM_IFLAG_H */
