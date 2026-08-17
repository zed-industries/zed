load("@bazel_skylib//:bzl_library.bzl", "bzl_library")

bzl_library(
    name = "bzl_srcs",
    srcs = glob(["**/*.bzl"]),
    visibility = ["//visibility:public"],
)

xcode_version(
    name = "github_actions_xcode_14_2_0",
    default_macos_sdk_version = "13.1",
    version = "14.2",
)

xcode_config(
    name = "github_actions_xcodes",
    default = ":github_actions_xcode_14_2_0",
    versions = [":github_actions_xcode_14_2_0"],
)
