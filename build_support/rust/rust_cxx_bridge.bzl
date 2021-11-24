load("@bazel_skylib//rules:run_binary.bzl", "run_binary")
load("@rules_cc//cc:defs.bzl", "cc_library")

def rust_cxx_bridge(name, src, visibility = None, deps = []):
    native.alias(
        name = "%s/header" % name,
        actual = src + ".h",
    )

    native.alias(
        name = "%s/source" % name,
        actual = src + ".cc",
    )

    run_binary(
        name = "%s/generated" % name,
        srcs = [src],
        outs = [
            src + ".h",
            src + ".cc",
        ],
        args = [
            "$(location %s)" % src,
            "-o",
            "$(location %s.h)" % src,
            "-o",
            "$(location %s.cc)" % src,
        ],
        tool = "@//3rdparty/rust:cargo_bin_cxxbridge",
    )

    cc_library(
        name = name,
        copts = ["-Wno-unused-variable"],
        srcs = [src + ".cc"],
        visibility = visibility,
        deps = deps + [":%s/include" % name],
    )

    cc_library(
        name = "%s/include" % name,
        hdrs = [src + ".h"],
        visibility = visibility,
    )
