"""CXX bzlmod extensions"""

load("@bazel_features//:features.bzl", "bazel_features")
load("//third-party/bazel:crates.bzl", _crate_repositories = "crate_repositories")

def _crates_vendor_remote_repository_impl(repository_ctx):
    repository_ctx.symlink(repository_ctx.attr.build_file, "BUILD.bazel")

_crates_vendor_remote_repository = repository_rule(
    implementation = _crates_vendor_remote_repository_impl,
    attrs = {
        "build_file": attr.label(mandatory = True),
    },
)

def _crate_repositories_impl(module_ctx):
    _crate_repositories()
    _crates_vendor_remote_repository(
        name = "crates.io",
        build_file = "//third-party/bazel:BUILD.bazel",
    )

    metadata_kwargs = {}
    if bazel_features.external_deps.extension_metadata_has_reproducible:
        metadata_kwargs["reproducible"] = True
    return module_ctx.extension_metadata(**metadata_kwargs)

crate_repositories = module_extension(
    implementation = _crate_repositories_impl,
)
