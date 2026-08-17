// For baremetal platforms, we don't really initialise '__aarch64_cpu_features',
// with exception of FEAT_SME that we can get from '__aarch64_sme_accessible'.

#if defined(COMPILER_RT_SHARED_LIB)
__attribute__((weak))
#endif
extern _Bool
__aarch64_sme_accessible(void);

static _Bool has_sme(void) {
#if defined(COMPILER_RT_SHARED_LIB)
  if (!__aarch64_sme_accessible)
    return 0;
#endif
  return __aarch64_sme_accessible();
}

void __init_cpu_features_resolver(unsigned long hwcap,
                                  const __ifunc_arg_t *arg) {}

void CONSTRUCTOR_ATTRIBUTE __init_cpu_features(void) {
  // CPU features already initialized.
  if (__atomic_load_n(&__aarch64_cpu_features.features, __ATOMIC_RELAXED))
    return;

  unsigned long long feat = 0;
  if (has_sme())
    feat |= 1ULL << FEAT_SME;

  __atomic_store_n(&__aarch64_cpu_features.features, feat, __ATOMIC_RELAXED);
}
