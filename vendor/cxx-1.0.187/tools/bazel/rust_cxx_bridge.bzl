"""CXX Bridge rules."""

load("@bazel_skylib//rules:run_binary.bzl", "run_binary")
load("@rules_cc//cc:defs.bzl", "cc_library")

def rust_cxx_bridge(name, src, deps = [], linkstatic = True, **kwargs):
    """A macro defining a cxx bridge library

    Args:
        name (string): The name of the new target
        src (string): The rust source file to generate a bridge for
        deps (list, optional): A list of dependencies for the underlying cc_library. Defaults to [].
        **kwargs: Common arguments to pass through to underlying rules.
    """
    native.alias(
        name = "%s/header" % name,
        actual = src + ".h",
        **kwargs
    )

    native.alias(
        name = "%s/source" % name,
        actual = src + ".cc",
        **kwargs
    )

    run_binary(
        name = "%s/generated" % name,
        srcs = [src],
        outs = [
            src + ".h",
            src + ".cc",
        ],
        args = [
            "$(execpath %s)" % src,
            "-o",
            "$(execpath %s.h)" % src,
            "-o",
            "$(execpath %s.cc)" % src,
        ],
        tool = "@cxx.rs//:codegen",
        **kwargs
    )

    cc_library(
        name = name,
        srcs = [src + ".cc"],
        deps = deps + [":%s/include" % name],
        linkstatic = linkstatic,
        **kwargs
    )

    cc_library(
        name = "%s/include" % name,
        hdrs = [src + ".h"],
        **kwargs
    )
