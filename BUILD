load("@rules_cc//cc:defs.bzl", "cc_library")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library", "rust_static_library")
load("//build_support/rust:rust_cxx_bridge.bzl", "rust_cxx_bridge")

cc_library(
    name = "cards_cpp",
    srcs = ["cards.cpp"],
    hdrs = ["cards.hpp"],
)

rust_cxx_bridge(
    name = "cards_bridge",
    src = "cards.rs",
    deps = [":cards_cpp"],
)

rust_library(
    name = "cards_rs",
    srcs = ["cards.rs"],
    deps = [":cards_bridge", ":cards_cpp", "//3rdparty/rust:cxx"],
)

rust_binary(
    name = "cards_cli",
    srcs = ["main.rs"],
    deps = [":cards_rs", "//3rdparty/rust:structopt"],
)
