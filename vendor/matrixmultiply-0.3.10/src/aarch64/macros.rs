macro_rules! is_aarch64_feature_detected_ {
    ($name:tt) => {{
        #[cfg(feature="std")]
        {
            // For testing purposes, we can make sure only one specific feature
            // is enabled by setting MMTEST_FEATURE=featurename (all others
            // disabled). This does not force it to be detected, it must also be.
             compile_env_matches_or_is_empty!("MMTEST_FEATURE", $name) && std::arch::is_aarch64_feature_detected!($name)
        }
        #[cfg(not(feature="std"))]
        {
            // For testing purposes, we can make sure only one specific feature
            // is enabled by setting MMTEST_FEATURE=featurename (all others
            // disabled). This does not force it to be detected, it must also
            // be. In the `no_std` case, the `is_86_feature_detected` macro is
            // not available, so we have to fall back to checking whether the
            // feature is enabled at compile-time.
            compile_env_matches_or_is_empty!("MMTEST_FEATURE", $name) && cfg!(target_feature=$name)
        }
    }};
}
