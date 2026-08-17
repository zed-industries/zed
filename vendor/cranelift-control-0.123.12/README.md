This crate contains the control plane for "chaos mode". It can be used to
inject pseudo-random perturbations into specific sections in the code while
fuzzing. Its compilation is feature-gated to prevent any performance
impact on release builds.
