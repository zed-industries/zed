#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <processthreadsapi.h>
#include <stdint.h>

#ifndef PF_ARM_V82_DP_INSTRUCTIONS_AVAILABLE
#define PF_ARM_V82_DP_INSTRUCTIONS_AVAILABLE 43 
#endif
#ifndef PF_ARM_V83_JSCVT_INSTRUCTIONS_AVAILABLE
#define PF_ARM_V83_JSCVT_INSTRUCTIONS_AVAILABLE 44
#endif
#ifndef PF_ARM_V83_LRCPC_INSTRUCTIONS_AVAILABLE
#define PF_ARM_V83_LRCPC_INSTRUCTIONS_AVAILABLE 45
#endif
#ifndef PF_ARM_SVE_INSTRUCTIONS_AVAILABLE
#define PF_ARM_SVE_INSTRUCTIONS_AVAILABLE 46
#endif
#ifndef PF_ARM_SVE2_INSTRUCTIONS_AVAILABLE
#define PF_ARM_SVE2_INSTRUCTIONS_AVAILABLE 47
#endif
#ifndef PF_ARM_SVE_PMULL128_INSTRUCTIONS_AVAILABLE
#define PF_ARM_SVE_PMULL128_INSTRUCTIONS_AVAILABLE 50
#endif
#ifndef PF_ARM_SVE_SHA3_INSTRUCTIONS_AVAILABLE
#define PF_ARM_SVE_SHA3_INSTRUCTIONS_AVAILABLE 55
#endif
#ifndef PF_ARM_SVE_SM4_INSTRUCTIONS_AVAILABLE
#define PF_ARM_SVE_SM4_INSTRUCTIONS_AVAILABLE 56
#endif
#ifndef PF_ARM_SVE_I8MM_INSTRUCTIONS_AVAILABLE
#define PF_ARM_SVE_I8MM_INSTRUCTIONS_AVAILABLE 57
#endif
#ifndef PF_ARM_SVE_F32MM_INSTRUCTIONS_AVAILABLE
#define PF_ARM_SVE_F32MM_INSTRUCTIONS_AVAILABLE 58
#endif
#ifndef PF_ARM_SVE_F64MM_INSTRUCTIONS_AVAILABLE
#define PF_ARM_SVE_F64MM_INSTRUCTIONS_AVAILABLE 59
#endif

void __init_cpu_features_resolver(unsigned long hwcap,
                                  const __ifunc_arg_t *arg) {}

void CONSTRUCTOR_ATTRIBUTE __init_cpu_features(void) {
  if (__atomic_load_n(&__aarch64_cpu_features.features, __ATOMIC_RELAXED))
    return;

#define setCPUFeature(F) features |= 1ULL << F

  uint64_t features = 0;

  setCPUFeature(FEAT_INIT);
  setCPUFeature(FEAT_FP);

  // https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-isprocessorfeaturepresent
  if (IsProcessorFeaturePresent(PF_ARM_V8_CRYPTO_INSTRUCTIONS_AVAILABLE)) {
    setCPUFeature(FEAT_SHA2);
    setCPUFeature(FEAT_PMULL);
  }

  static const struct ProcessFeatureToFeatMap_t {
    int WinApiFeature;
    enum CPUFeatures CPUFeature;
  } FeatMap[] = {
      {PF_ARM_V8_CRC32_INSTRUCTIONS_AVAILABLE, FEAT_CRC},
      {PF_ARM_V81_ATOMIC_INSTRUCTIONS_AVAILABLE, FEAT_LSE},
      {PF_ARM_V82_DP_INSTRUCTIONS_AVAILABLE, FEAT_DOTPROD},
      {PF_ARM_V83_JSCVT_INSTRUCTIONS_AVAILABLE, FEAT_JSCVT},
      {PF_ARM_V83_LRCPC_INSTRUCTIONS_AVAILABLE, FEAT_RCPC},
      {PF_ARM_SVE_INSTRUCTIONS_AVAILABLE, FEAT_SVE},
      {PF_ARM_SVE2_INSTRUCTIONS_AVAILABLE, FEAT_SVE2},
      {PF_ARM_SVE_PMULL128_INSTRUCTIONS_AVAILABLE, FEAT_SVE_PMULL128},
      {PF_ARM_SVE_SHA3_INSTRUCTIONS_AVAILABLE, FEAT_SVE_SHA3},
      {PF_ARM_SVE_SM4_INSTRUCTIONS_AVAILABLE, FEAT_SVE_SM4},
      {PF_ARM_SVE_F32MM_INSTRUCTIONS_AVAILABLE, FEAT_SVE_F32MM},
      {PF_ARM_SVE_F64MM_INSTRUCTIONS_AVAILABLE, FEAT_SVE_F64MM},
      // There is no I8MM flag, but when SVE_I8MM is available, I8MM is too.
      {PF_ARM_SVE_I8MM_INSTRUCTIONS_AVAILABLE, FEAT_I8MM},
  };

  for (size_t I = 0, E = sizeof(FeatMap) / sizeof(FeatMap[0]); I != E; ++I)
    if (IsProcessorFeaturePresent(FeatMap[I].WinApiFeature))
      setCPUFeature(FeatMap[I].CPUFeature);

  __atomic_store(&__aarch64_cpu_features.features, &features, __ATOMIC_RELAXED);
}
