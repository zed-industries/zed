def rust_cxx_bridge(
        name: str,
        src: str,
        deps: list[str] = []):
    native.export_file(
        name = "%s/header" % name,
        src = ":%s/generated[generated.h]" % name,
        out = src + ".h",
    )

    native.export_file(
        name = "%s/source" % name,
        src = ":%s/generated[generated.cc]" % name,
        out = src + ".cc",
    )

    native.genrule(
        name = "%s/generated" % name,
        srcs = [src],
        outs = {
            "generated.cc": ["generated.cc"],
            "generated.h": ["generated.h"],
        },
        cmd = "$(exe //:codegen) ${SRCS} -o ${OUT}/generated.h -o ${OUT}/generated.cc",
        type = "cxxbridge",
    )

    native.cxx_library(
        name = name,
        srcs = [":%s/source" % name],
        preferred_linkage = "static",
        exported_deps = deps + [":%s/include" % name],
    )

    native.cxx_library(
        name = "%s/include" % name,
        exported_headers = [":%s/header" % name],
    )
